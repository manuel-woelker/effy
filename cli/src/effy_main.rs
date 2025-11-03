use crate::run_tests::run_tests;
use effy_base::error::EffyResult;
use effy_base::logging::error;
use effy_pal::PalHandle;
use effy_pal_real::PalReal;
use std::process::ExitCode;

pub mod run_tests;

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
