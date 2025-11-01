use crate::environment::Environment;
use crate::native_function::{NativeFunction, NativeFunctionContext, NativeFunctionTrait};
use crate::value::{InterpreterValue, ValueKind};
use effy_ast::expression::{
    CallExpression, Expression, ExpressionNode, LiteralExpression, VarUseExpression,
};
use effy_ast::script::ScriptNode;
use effy_ast::statement::{Statement, StatementNode};
use effy_base::error::{EffyResult, bail};
use effy_base::source_file::SourceFile;
use effy_parser::parser::parse_script;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let environment = Environment::new();
        Interpreter { environment }
    }

    pub fn add_native_function(
        &mut self,
        name: impl Into<String>,
        function: impl NativeFunctionTrait,
    ) {
        let name = name.into();
        self.environment.add(
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
        let script = parse_script(&source_file)?;
        self.eval_script(&script)
    }

    pub fn eval_script(&mut self, script: &ScriptNode) -> EffyResult<InterpreterValue> {
        let mut result = InterpreterValue::unit();
        for statement in &script.statements {
            result = self.eval_statement(statement)?;
        }
        Ok(result)
    }

    pub fn eval_statement(&mut self, statement: &StatementNode) -> EffyResult<InterpreterValue> {
        Ok(match &statement.data {
            Statement::Expression(expression) => self.eval_expression(&expression.expression)?,
            Statement::FunctionDefinition(_) => {
                todo!()
            }
        })
    }

    pub fn eval_expression(&mut self, expression: &ExpressionNode) -> EffyResult<InterpreterValue> {
        Ok(match &expression.data {
            Expression::Literal(literal) => self.eval_literal(literal)?,
            Expression::Call(call) => self.eval_call(call)?,
            Expression::VarUse(var_use) => self.eval_var_use(var_use)?,
        })
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
        let callee = self.eval_expression(call.callee())?;
        match callee.value_kind() {
            ValueKind::NativeFunction(native_function) => {
                native_function.invoke(&mut NativeFunctionContext { arguments })
            }
            _ => {
                bail!("Expression value is not callable: {}", callee);
            }
        }
    }

    pub fn eval_var_use(&mut self, var_use: &VarUseExpression) -> EffyResult<InterpreterValue> {
        let value = self.environment.get(&var_use.name().name)?;
        Ok(value)
    }
}

#[cfg(test)]
mod test {
    use crate::interpreter::Interpreter;
    use crate::shared_string_buffer::SharedStringBuffer;
    use crate::value::InterpreterValue;
    use effy_base::error::EffyResult;
    use effy_base::source_file::SourceFile;
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
}
