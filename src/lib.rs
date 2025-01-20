#![doc = include_str!("../README.md")]

use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    fs::read,
    mem::take,
    ops::Deref,
    path::PathBuf,
    sync::{OnceLock, RwLock, RwLockReadGuard},
};

/// A hot-reloaded module.
#[derive(Debug)]
pub struct Module {
    path: &'static str,
    current: RwLock<Vec<u8>>,
    next: RwLock<Option<Vec<u8>>>,
    watcher: OnceLock<RecommendedWatcher>,
}

/// A read guard of a module.
#[derive(Debug)]
pub struct Guard(RwLockReadGuard<'static, Vec<u8>>);

impl Deref for Guard {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Module {
    /// Creates a module.
    pub const fn new(path: &'static str) -> Self {
        Self {
            path,
            current: RwLock::new(Vec::new()),
            next: RwLock::new(None),
            watcher: OnceLock::new(),
        }
    }

    /// Loads a content.
    pub fn load(&'static self) -> Guard {
        self.watcher.get_or_init(|| {
            let mut lock = self.current.write().unwrap();
            *lock = read(self.path).unwrap();

            let mut watcher = RecommendedWatcher::new(
                |result| {
                    if let Ok(Event { kind, .. }) = result {
                        if kind.is_modify() {
                            if let Ok(mut content) = self.next.write() {
                                *content = Some(read(self.path).unwrap());
                            }
                        }
                    }
                },
                Default::default(),
            )
            .unwrap();
            watcher
                .watch(&PathBuf::from(self.path), RecursiveMode::NonRecursive)
                .unwrap();
            watcher
        });

        // All lock functions used here must be asynchronous.
        if let Ok(mut content) = self.next.try_write() {
            if let Some(content) = take(&mut *content) {
                if let Ok(mut current) = self.current.try_write() {
                    *current = content;
                }
            }
        }

        Guard(self.current.read().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_file() {
        static FOO: Module = Module::new("test/foo.txt");

        assert_eq!(&*FOO.load(), "foo\n".as_bytes());
    }
}
