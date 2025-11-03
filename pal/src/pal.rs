use effy_base::error::EffyResult;
use relative_path::RelativePathBuf;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::sync::Arc;

pub type FilePath = RelativePathBuf;

/// Platform abstraction layer used to decouple logic from the underlying platform
pub trait Pal: Debug + Sync + Send + 'static {
    /// Get the command line arguments, the first one being the path to the binary
    fn args(&self) -> Vec<String>;

    /// Print a message to stderr
    fn print(&self, message: &str);

    /// Exit the process with the given exit code
    fn exit(&self, exit_code: i32);

    /// Read a file, the path is relative to the base directory
    fn read_file(&self, path: &FilePath) -> EffyResult<Box<dyn Read + 'static>>;

    /// Read a file to a string, the path is relative to the base directory
    fn read_file_to_string(&self, path: &FilePath) -> EffyResult<String> {
        let mut string = String::new();
        self.read_file(path)?.read_to_string(&mut string)?;
        Ok(string)
    }

    /// Create a file to a string, the path is relative to the base directory
    fn create_file(&self, path: &FilePath) -> EffyResult<Box<dyn Write>>;

    /// Create a directory (including parent directories), the path is relative to the base directory
    fn create_directory_all(&self, path: &FilePath) -> EffyResult<()>;

    /// Remove a directory (including _all_ content), the path is relative to the base directory
    fn remove_directory_all(&self, path: &FilePath) -> EffyResult<()>;

    /// walk directory using the supplied globs
    fn walk_directory(
        &self,
        path: &FilePath,
        globs: &[String],
    ) -> EffyResult<Box<dyn Iterator<Item = EffyResult<FilePath>> + '_>>;

    /// Register a callback to be called when a file changes
    fn watch_directory(
        &self,
        directory: &FilePath,
        globs: &[String],
        callback: FileChangeCallback,
    ) -> EffyResult<()>;
}

#[derive(Debug, Clone)]
pub struct PalHandle(Arc<dyn Pal>);

impl PalHandle {
    pub fn new(pal: impl Pal + 'static) -> Self {
        Self(Arc::new(pal))
    }
}

// Implement Deref for convenience
impl std::ops::Deref for PalHandle {
    type Target = dyn Pal;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

pub struct FileChangeEvent {}

pub type FileChangeCallback = Box<dyn Fn(FileChangeEvent) + Send + Sync>;
