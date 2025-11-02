pub use color_eyre::Report as EffyError;
use std::fmt::Write;

pub type EffyResult<T> = Result<T, EffyError>;

use crate::unansi;
pub use color_eyre::eyre::Context;
pub use color_eyre::eyre::anyhow as err;
pub use color_eyre::eyre::bail;
pub use color_eyre::eyre::ensure;

pub fn to_test_string(err: &EffyError) -> String {
    let mut test_string = String::new();
    writeln!(test_string, "{}", err).unwrap();
    unansi(&test_string)
}
