use effy_base::error::EffyResult;
use effy_base::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use effy_pal::{FileChangeCallback, FilePath, Pal};
use expect_test::Expect;
use indent::indent_all_with;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct PalMock {
    inner: Arc<RwLock<PalMockInner>>,
}

#[derive(Default)]
struct PalMockInner {
    args: Vec<String>,
    effects_string: String,
}

impl PalMock {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(PalMockInner {
                args: vec!["./effy.exe".to_string()],
                effects_string: String::new(),
            })),
        }
    }

    fn read(&self) -> RwLockReadGuard<'_, PalMockInner> {
        self.inner.read()
    }

    fn write(&self) -> RwLockWriteGuard<'_, PalMockInner> {
        self.inner.write()
    }

    pub fn log_effect(&self, effect: impl AsRef<str>) {
        self.write().effects_string.push_str(effect.as_ref());
        self.write().effects_string.push('\n');
    }

    pub fn verify_effects(&self, expected: Expect) {
        expected.assert_eq(&self.read().effects_string);
        self.write().effects_string.clear();
    }

    #[allow(dead_code)]
    pub fn get_effects(&self) -> String {
        self.read().effects_string.clone()
    }

    pub fn clear_effects(&self) {
        self.write().effects_string.clear();
    }

    pub fn set_args(&self, args: &[&str]) {
        let mut all_args = vec!["./effy.exe".to_string()];
        all_args.extend(args.iter().map(|s| s.to_string()));
        self.write().args = all_args;
    }
}

impl Pal for PalMock {
    fn args(&self) -> Vec<String> {
        self.read().args.clone()
    }

    fn print(&self, message: &str) {
        self.log_effect(format!("PRINT:\n{}", indent_all_with("\t", message)));
    }

    fn exit(&self, exit_code: i32) {
        self.log_effect(format!("EXIT: {}", exit_code));
    }

    fn read_file(&self, _path: &FilePath) -> EffyResult<Box<dyn Read + 'static>> {
        todo!()
    }

    fn create_file(&self, _path: &FilePath) -> EffyResult<Box<dyn Write>> {
        todo!()
    }

    fn create_directory_all(&self, _path: &FilePath) -> EffyResult<()> {
        todo!()
    }

    fn remove_directory_all(&self, _path: &FilePath) -> EffyResult<()> {
        todo!()
    }

    fn walk_directory(
        &self,
        _path: &FilePath,
        _globs: &[String],
    ) -> EffyResult<Box<dyn Iterator<Item = EffyResult<FilePath>> + '_>> {
        todo!()
    }

    fn watch_directory(
        &self,
        _directory: &FilePath,
        _globs: &[String],
        _callback: FileChangeCallback,
    ) -> EffyResult<()> {
        todo!()
    }
}

impl Debug for PalMock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PalMock").finish()
    }
}
