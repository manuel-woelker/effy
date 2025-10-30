use crate::error::EffyResult;
use std::fmt::Write;

pub fn indent(write: &mut dyn Write, indent: usize) -> EffyResult<()> {
    write!(write, "{:indent$}", "", indent = indent)?;
    Ok(())
}
