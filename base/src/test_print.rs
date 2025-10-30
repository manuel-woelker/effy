use crate::error::EffyResult;
use crate::indent;
use std::fmt::Write;

pub trait TestPrint {
    fn test_print(&self, write: &mut dyn Write, indent: usize) -> EffyResult<()>;
    fn test_print_to_string(&self, indent: usize) -> EffyResult<String> {
        let mut string = String::new();
        self.test_print(&mut string, indent)?;
        Ok(string)
    }
    fn indent(&self, write: &mut dyn Write, indent: usize) -> EffyResult<()> {
        indent::indent(write, indent)?;
        Ok(())
    }
}
