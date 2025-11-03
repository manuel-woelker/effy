use effy_base::FilePath;
use effy_base::error::{EffyError, EffyResult, bail};
use effy_base::source_file::SourceFile;
use effy_base::value::Value;
use effy_interpreter::interpreter::Interpreter;
use effy_interpreter::test_event::TestEvent;
use effy_interpreter::value::{InterpreterValue, ValueKind};
use effy_pal::PalHandle;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Default)]
pub struct TestResult {
    pub total_tests: u64,
    pub assertions: u64,
    pub test_failures: Vec<(String, EffyError)>,
}

pub fn run_tests(pal: PalHandle) -> EffyResult<()> {
    let files = pal.walk_directory(&FilePath::from("."), &["*.effy".to_string()])?;
    for file in files {
        let file = file?;
        pal.print(&format!("Testing file: '{}'\n", file));
        let source = pal.read_file_to_string(&file)?;
        let source_file = SourceFile::new(file, source);

        let mut interpreter = Interpreter::new();
        let assertions = Arc::new(AtomicU64::new(0));
        let assertions_clone = assertions.clone();
        interpreter.add_native_function_fn("assert", move |context| {
            let first_argument = &context.arguments()[0];
            assertions_clone.fetch_add(1, Ordering::SeqCst);
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
        let mut test_result = TestResult::default();
        interpreter.run_tests(source_file, &mut |event| match event {
            TestEvent::TestStarted { test_name } => {
                test_result.total_tests += 1;
                pal.print(&format!(" ⌛ {}", test_name));
            }
            TestEvent::TestSuccess { test_name } => {
                pal.print(&format!("\r ✅  {}\n", test_name));
            }
            TestEvent::TestFailed { test_name, error } => {
                pal.print(&format!("\r ❌  {}\n", test_name));
                test_result.test_failures.push((test_name, error));
            }
        })?;
        test_result.assertions = assertions.load(Ordering::SeqCst);
        if !test_result.test_failures.is_empty() {
            pal.print(&format!(
                "\n 🔴  {} failed tests\n\n",
                test_result.test_failures.len()
            ));
            for (test_name, test_failure) in &test_result.test_failures {
                pal.print(&format!(" ❌  Test '{test_name}'\n{test_failure}\n\n"))
            }
        }
        let summary_symbol = if test_result.test_failures.is_empty() {
            "✅"
        } else {
            "❌"
        };
        pal.print(&format!("\n {summary_symbol}  Test Summary:\n"));
        pal.print(&format!(
            "    Test failures: {:7}\n",
            test_result.test_failures.len()
        ));
        pal.print(&format!(
            "    Total tests:   {:7}\n",
            test_result.total_tests
        ));
        pal.print(&format!(
            "    Assertions:    {:7}\n",
            test_result.assertions
        ));
    }
    Ok(())
}
