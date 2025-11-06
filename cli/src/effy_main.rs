use crate::run_tests::{print_test_result, run_tests};
use effy_base::error::EffyResult;
use effy_pal::PalHandle;
use effy_pal_real::PalReal;
use std::process::ExitCode;

pub mod run_tests;

fn main() -> ExitCode {
    // Enable ANSI color support on Windows
    if let Err(e) = enable_ansi_support::enable_ansi_support() {
        eprintln!("Warning: Failed to enable ANSI color support: {}", e);
    }
    effy_base::logging::init_logging();
    color_eyre::install().unwrap();
    let pal = PalHandle::new(PalReal::new());
    let result = main_cli(&pal);
    if let Err(err) = result {
        pal.print(&format!("Effy failed with error\n{}\n\n", err));
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn main_cli(pal: &PalHandle) -> EffyResult<()> {
    let args = pal.args();
    if args.len() >= 2 {
        let command = &args[1];
        match command.as_str() {
            "version" => pal.print(&format!("Effy version v{}\n", env!("CARGO_PKG_VERSION"))),
            "test" => {
                let test_result = run_tests(pal)?;
                print_test_result(pal, &test_result);
                if !test_result.test_failures.is_empty() {
                    pal.exit(11);
                }
            }
            _ => {
                pal.print(&format!("Unknown command: '{}'\n", command));
                pal.exit(44);
            }
        }
    }
    Ok(())
}
