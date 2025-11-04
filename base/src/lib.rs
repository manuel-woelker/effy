pub mod error;
pub mod indent;
pub mod logging;
pub mod source_error;
pub mod source_file;
pub mod source_message;
pub mod source_snippet;
pub mod source_span;
pub mod test_print;
pub mod value;

pub type FilePath = relative_path::RelativePathBuf;
pub type SourceString = String;
pub use annotate_snippets;
pub use yansi::Paint;

pub fn unansi(string: &str) -> String {
    strip_ansi_escapes::strip_str(string)
}
