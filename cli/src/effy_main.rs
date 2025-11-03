use effy_base::FilePath;
use effy_base::error::{EffyResult, bail};
use effy_base::logging::error;
use effy_base::source_file::SourceFile;
use effy_base::value::Value;
use effy_interpreter::interpreter::Interpreter;
use effy_interpreter::test_event::TestEvent;
use effy_interpreter::value::{InterpreterValue, ValueKind};
use effy_pal::PalHandle;
use effy_pal_real::PalReal;
use std::process::ExitCode;
use std::thread::sleep;
use std::time::Duration;

fn main() -> ExitCode {
    effy_base::logging::init_logging();
    color_eyre::install().unwrap();
    let pal = PalHandle::new(PalReal::new());
    let result = main_cli(pal);
    if let Err(err) = result {
        error!("Effy failed with error: {}", err);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn main_cli(pal: PalHandle) -> EffyResult<()> {
    let args = pal.args();
    if args.len() >= 2 {
        let command = &args[1];
        match command.as_str() {
            "version" => pal.print(&format!("Effy version v{}\n", env!("CARGO_PKG_VERSION"))),
            "test" => run_tests(pal)?,
            _ => {
                pal.print(&format!("Unknown command: '{}'", command));
                pal.exit(44);
            }
        }
    }
    Ok(())
}

fn run_tests(pal: PalHandle) -> EffyResult<()> {
    let files = pal.walk_directory(&FilePath::from("."), &["*.effy".to_string()])?;
    for file in files {
        let file = file?;
        pal.print(&format!("Testing file: '{}'\n", file));
        let source = pal.read_file_to_string(&file)?;
        let source_file = SourceFile::new(file, source);

        let mut interpreter = Interpreter::new();
        interpreter.add_native_function_fn("assert", move |context| {
            let first_argument = &context.arguments()[0];
            // TODO: count assertions?
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

        interpreter.run_tests(source_file, &mut |event| match event {
            TestEvent::TestStarted { test_name } => {
                pal.print(&format!("   {}", test_name));
                sleep(Duration::from_millis(200));
            }
            TestEvent::TestSuccess { test_name } => {
                pal.print(&format!("\r ✅ {}\n", test_name));
            }
            TestEvent::TestFailed { test_name, error } => {
                pal.print(&format!("\r ❌ {} Test failed\n{}\n", test_name, error));
            }
        })?;
    }
    Ok(())
}
