use crate::environment::Environment;
use crate::environment_stack::EnvironmentStack;
use crate::native_function::{NativeFunction, NativeFunctionContext, NativeFunctionTrait};
use crate::test_event::TestEvent;
use crate::value::{InterpreterValue, ValueKind};
use effy_ast::expression::{
    BinaryExpression, BinaryOperator, CallExpression, Expression, ExpressionNode,
    LiteralExpression, VarUseExpression,
};
use effy_ast::function_definition::FunctionDefinition;
use effy_ast::script::ScriptNode;
use effy_ast::statement::{Statement, StatementNode};
use effy_base::error::{EffyResult, bail};
use effy_base::source_file::SourceFile;
use effy_base::source_message::SourceMessage;
use effy_base::value::Value;
use effy_parser::parser::parse_script;

pub struct Interpreter {
    environment_stack: EnvironmentStack,
    source_file: SourceFile,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let environment_stack = EnvironmentStack::new();
        Interpreter {
            environment_stack,
            source_file: SourceFile::new("<uninitialized>", ""),
        }
    }

    fn add_binding(&mut self, name: impl Into<String>, value: impl Into<InterpreterValue>) {
        self.current_environment().add(name, value);
    }

    fn current_environment(&mut self) -> &mut Environment {
        self.environment_stack.top()
    }

    fn get_binding(&mut self, name: impl AsRef<str>) -> Option<InterpreterValue> {
        self.current_environment().get(name)
    }

    pub fn add_native_function(
        &mut self,
        name: impl Into<String>,
        function: impl NativeFunctionTrait,
    ) {
        let name = name.into();
        self.add_binding(
            name.clone(),
            InterpreterValue::native_function(NativeFunction::new(name, function)),
        );
    }

    pub fn add_native_function_fn(
        &mut self,
        name: impl Into<String>,
        function: impl Fn(&mut NativeFunctionContext) -> EffyResult<InterpreterValue>
        + Send
        + Sync
        + 'static,
    ) {
        self.add_native_function(name, function);
    }

    pub fn run_script(&mut self, source_file: SourceFile) -> EffyResult<InterpreterValue> {
        self.source_file = source_file;
        let script = parse_script(&self.source_file)?;
        self.eval_script(&script)
    }

    pub fn run_tests(
        &mut self,
        source_file: SourceFile,
        callback: &mut dyn FnMut(TestEvent),
    ) -> EffyResult<()> {
        self.source_file = source_file;
        let script = parse_script(&self.source_file)?;
        self.eval_tests(&script, callback)
    }

    pub fn eval_script(&mut self, script: &ScriptNode) -> EffyResult<InterpreterValue> {
        self.eval_statements(script.statements.as_slice())
    }

    pub fn eval_tests(
        &mut self,
        script: &ScriptNode,
        callback: &mut dyn FnMut(TestEvent),
    ) -> EffyResult<()> {
        for statement in &script.statements {
            if let Statement::FunctionDefinition(function_definition) = &statement.data {
                if !function_definition
                    .function_definition
                    .annotations()
                    .iter()
                    .any(|annotation| annotation.name == "Test")
                {
                    continue;
                }
                let test_name = function_definition
                    .function_definition
                    .name()
                    .data
                    .name
                    .clone();
                callback(TestEvent::TestStarted {
                    test_name: test_name.clone(),
                });
                match self.eval_test(&function_definition.function_definition) {
                    Ok(_) => {
                        callback(TestEvent::TestSuccess {
                            test_name: test_name.clone(),
                        });
                    }
                    Err(error) => {
                        callback(TestEvent::TestFailed {
                            test_name: test_name.clone(),
                            error,
                        });
                    }
                };
            }
        }
        Ok(())
    }

    pub fn eval_test(&mut self, test_definition: &FunctionDefinition) -> EffyResult<()> {
        self.eval_statements(test_definition.statements().as_slice())?;
        Ok(())
    }

    fn eval_statements(&mut self, statements: &[StatementNode]) -> EffyResult<InterpreterValue> {
        let mut result = InterpreterValue::unit();
        for statement in statements {
            result = self.eval_statement(statement)?;
        }
        Ok(result)
    }

    pub fn eval_statement(&mut self, statement: &StatementNode) -> EffyResult<InterpreterValue> {
        Ok(match &statement.data {
            Statement::Expression(expression) => self.eval_expression(&expression.expression)?,
            Statement::FunctionDefinition(function_definition) => {
                let function_value = InterpreterValue::coded_function(
                    function_definition.function_definition.clone(),
                    self.environment_stack.top().clone(),
                );
                self.add_binding(
                    function_definition
                        .function_definition
                        .name()
                        .data
                        .name
                        .clone(),
                    function_value.clone(),
                );
                function_value
            }
        })
    }

    pub fn eval_expression(&mut self, expression: &ExpressionNode) -> EffyResult<InterpreterValue> {
        Ok(match &expression.data {
            Expression::Literal(literal) => self.eval_literal(literal)?,
            Expression::Call(call) => self.eval_call(call)?,
            Expression::VarUse(var_use) => self.eval_var_use(var_use)?,
            Expression::Binary(binary_expression) => {
                self.eval_binary_expression(binary_expression)?
            }
        })
    }

    pub fn eval_binary_expression(
        &mut self,
        binary_expression: &BinaryExpression,
    ) -> EffyResult<InterpreterValue> {
        let left = self.eval_expression(binary_expression.left())?;
        let right = self.eval_expression(binary_expression.right())?;
        let result = match (left.value_kind(), right.value_kind()) {
            (
                ValueKind::PrimitiveValue(Value::Int(left)),
                ValueKind::PrimitiveValue(Value::Int(right)),
            ) => match binary_expression.operator() {
                BinaryOperator::Add => InterpreterValue::int(left + right),
                BinaryOperator::Multiply => InterpreterValue::int(left * right),
                BinaryOperator::Equals => InterpreterValue::bool(left == right),
                _ => bail!(
                    "Unsupported binary expression for ints: '{}'",
                    binary_expression.operator()
                ),
            },
            _ => bail!("Could not eval binary expression"),
        };
        Ok(result)
    }

    pub fn eval_literal(&mut self, literal: &LiteralExpression) -> EffyResult<InterpreterValue> {
        Ok(InterpreterValue::primitive(literal.value().clone()))
    }

    pub fn eval_call(&mut self, call: &CallExpression) -> EffyResult<InterpreterValue> {
        let arguments = call
            .arguments()
            .iter()
            .map(|arg| self.eval_expression(arg))
            .collect::<EffyResult<Vec<InterpreterValue>>>()?;
        let argument_spans = call
            .arguments()
            .iter()
            .map(|arg| arg.span.clone())
            .collect::<Vec<_>>();
        let callee = self.eval_expression(call.callee())?;
        match callee.value_kind() {
            ValueKind::NativeFunction(native_function) => {
                native_function.invoke(&mut NativeFunctionContext {
                    arguments,
                    argument_spans,
                })
            }
            ValueKind::CodedFunction(coded_function) => {
                self.push_environment(coded_function.environment().new_child());
                let result = self.eval_statements(
                    coded_function.function_definition().statements().as_slice(),
                )?;
                self.pop_environment();
                Ok(result)
            }
            _ => {
                bail!("Expression value is not callable: {}", callee);
            }
        }
    }

    pub fn push_environment(&mut self, environment: Environment) {
        self.environment_stack.push(environment);
    }

    pub fn pop_environment(&mut self) {
        self.environment_stack.pop();
    }

    pub fn eval_var_use(&mut self, var_use: &VarUseExpression) -> EffyResult<InterpreterValue> {
        let value = self.get_binding(&var_use.name().name).ok_or_else(|| {
            SourceMessage::error_builder(
                &self.source_file,
                format!("Could not resolve binding '{}'", var_use.name().name),
            )
            .label(var_use.name().span.clone(), "name could not be found")
            .build_error()
        })?;
        Ok(value)
    }
}

#[cfg(test)]
mod test {
    use crate::interpreter::Interpreter;
    use crate::shared_string_buffer::SharedStringBuffer;
    use crate::test_event::TestEvent;
    use crate::value::{InterpreterValue, ValueKind};
    use effy_base::error::{EffyResult, bail};
    use effy_base::source_file::SourceFile;
    use effy_base::unansi;
    use effy_base::value::Value;
    use expect_test::{Expect, expect};

    fn test_eval(source: &str, expected: Expect) -> EffyResult<()> {
        let source_file = SourceFile::new("script.effy", source);
        let mut interpreter = Interpreter::new();
        let result_string_buffer = SharedStringBuffer::new();
        let result_string_clone = result_string_buffer.clone();
        interpreter.add_native_function_fn("println", move |context| {
            write!(result_string_clone, "PRINTLN {}\n", context.arguments()[0]);
            Ok(InterpreterValue::unit())
        });
        let result = interpreter.run_script(source_file)?;
        let mut result_string = result_string_buffer.to_string();
        result_string.push_str("RESULT: ");
        result_string.push_str(&result.to_string());
        expected.assert_eq(&result_string);
        Ok(())
    }

    macro_rules! test_eval {
        ($name:ident, $source:literal, $expected:expr) => {
            #[test]
            fn $name() -> EffyResult<()> {
                test_eval($source, $expected)
            }
        };
    }

    test_eval!(empty, "", expect!["RESULT: unit"]);

    test_eval!(integer, "42;", expect!["RESULT: 42i64"]);

    test_eval!(string_empty, "\"\";", expect![[r#"RESULT: """#]]);

    test_eval!(string_simple, "\"foo\";", expect![[r#"RESULT: "foo""#]]);

    test_eval!(
        var_use_println,
        "println;",
        expect!["RESULT: <native function 'println'>"]
    );

    test_eval!(
        call_println,
        "println(\"hello world\");",
        expect![[r#"
            PRINTLN "hello world"
            RESULT: unit"#]]
    );

    test_eval!(
        add,
        "println(1+2);",
        expect![[r#"
            PRINTLN 3i64
            RESULT: unit"#]]
    );

    test_eval!(
        mul,
        "println(2*3);",
        expect![[r#"
            PRINTLN 6i64
            RESULT: unit"#]]
    );

    test_eval!(
        equals,
        "println(1==2);println(2==2);",
        expect![[r#"
            PRINTLN #false
            PRINTLN #true
            RESULT: unit"#]]
    );

    test_eval!(
        fun_call,
        "fun foo() {println(1);} println(0);foo();println(2);",
        expect![[r#"
            PRINTLN 0i64
            PRINTLN 1i64
            PRINTLN 2i64
            RESULT: unit"#]]
    );

    fn run_test(source: &str, expected: Expect) -> EffyResult<()> {
        let source_file = SourceFile::new("script.effy", source);
        let mut interpreter = Interpreter::new();
        let result_string_buffer = SharedStringBuffer::new();
        let result_string_clone = result_string_buffer.clone();
        interpreter.add_native_function_fn("println", move |context| {
            write!(result_string_clone, "PRINTLN {}\n", context.arguments()[0]);
            Ok(InterpreterValue::unit())
        });
        let result_string_clone = result_string_buffer.clone();
        interpreter.add_native_function_fn("assert", move |context| {
            let first_argument = &context.arguments()[0];
            write!(result_string_clone, "ASSERT {}\n", first_argument);
            // check if first_argument is a boolean
            if let ValueKind::PrimitiveValue(Value::Boolean(value)) = first_argument.value_kind() {
                if !value {
                    bail!("assertion failed");
                }
                Ok(InterpreterValue::unit())
            } else {
                bail!(
                    "argument to assert() call must evaluate boolean, but instead found: '{}'",
                    first_argument
                );
            }
        });
        let result_string_clone = result_string_buffer.clone();
        interpreter.run_tests(source_file, &mut |event| match event {
            TestEvent::TestStarted { test_name } => {
                writeln!(result_string_clone, "TEST STARTED: {}", test_name);
            }
            TestEvent::TestSuccess { test_name } => {
                writeln!(result_string_clone, "TEST SUCCESS: {}", test_name);
            }
            TestEvent::TestFailed { test_name, error } => {
                writeln!(
                    result_string_clone,
                    "TEST FAILED: {}\nERROR: {}",
                    test_name, error
                );
            }
        })?;
        let result_string = result_string_buffer.to_string();
        expected.assert_eq(&unansi(&result_string));
        Ok(())
    }

    macro_rules! run_test {
        ($name:ident, $source:literal, $expected:expr) => {
            #[test]
            fn $name() -> EffyResult<()> {
                run_test($source, $expected)
            }
        };
    }

    run_test!(test_nothing, "", expect![""]);

    run_test!(test_empty_fun_unannotated, "fun foo() {}", expect![""]);

    run_test!(
        test_empty_test,
        "@Test fun foo() {}",
        expect![[r#"
        TEST STARTED: foo
        TEST SUCCESS: foo
    "#]]
    );

    run_test!(
        test_fail,
        "@Test fun foo() {bar;}",
        expect![[r#"
            TEST STARTED: foo
            TEST FAILED: foo
            ERROR: error: Could not resolve binding 'bar'
              ╭▸ script.effy:1:18
              │
            1 │ @Test fun foo() {bar;}
              ╰╴                 ━━━ name could not be found
        "#]]
    );

    run_test!(
        test_assert_true,
        "@Test fun foo() {assert(true);}",
        expect![[r#"
            TEST STARTED: foo
            ASSERT #true
            TEST SUCCESS: foo
        "#]]
    );

    run_test!(
        test_assert_false,
        "@Test fun foo() {assert(false);}",
        expect![[r#"
            TEST STARTED: foo
            ASSERT #false
            TEST FAILED: foo
            ERROR: assertion failed
        "#]]
    );

    run_test!(
        test_assert_string,
        "@Test fun foo() {assert(\"bar\");}",
        expect![[r#"
            TEST STARTED: foo
            ASSERT "bar"
            TEST FAILED: foo
            ERROR: argument to assert() call must evaluate boolean, but instead found: '"bar"'
        "#]]
    );
}
