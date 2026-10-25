//! 🧲️ Run-time test fixtures: one reader that resolves a path below the manifest directory of the crate that calls it, so no test file keeps its own copy of the read.
//!
//! Every macro expands `env!("CARGO_MANIFEST_DIR")` at the call site, which is the crate being tested, not this crate. Add the crate as a `[dev-dependencies]` entry and `use semio_framework_test_fixtures::fixture_text;` (or `fixture_bytes`, `fixture_path`, `manifest_dir`, `repo_root`). Texts and bytes are leaked so they live as long as the test process, the lifetime the former `include_str!` and `include_bytes!` statics had.
//!
//! @see <https://doc.rust-lang.org/std/macro.env.html>
//! @see <https://doc.rust-lang.org/std/macro.include_str.html>

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock, PoisonError};

/// 🧭️ Path of `relative` below `manifest_dir`, without touching the file system.
#[must_use]
pub fn path_in(manifest_dir: &str, relative: impl AsRef<Path>) -> PathBuf {
    Path::new(manifest_dir).join(relative)
}

/// 📖️ UTF-8 text of `relative` below `manifest_dir`, leaked for the rest of the process.
///
/// # Panics
/// When the file is missing or is not UTF-8; the message names the resolved path and both inputs.
#[must_use]
#[track_caller]
pub fn text_in(manifest_dir: &str, relative: impl AsRef<Path>) -> &'static str {
    let relative = relative.as_ref();
    let path = path_in(manifest_dir, relative);
    match std::fs::read_to_string(&path) {
        Ok(text) => Box::leak(text.into_boxed_str()),
        Err(error) => refuse(&path, manifest_dir, relative, &error),
    }
}

/// 💾️ Bytes of `relative` below `manifest_dir`, leaked for the rest of the process.
///
/// # Panics
/// When the file is missing or unreadable; the message names the resolved path and both inputs.
#[must_use]
#[track_caller]
pub fn bytes_in(manifest_dir: &str, relative: impl AsRef<Path>) -> &'static [u8] {
    let relative = relative.as_ref();
    let path = path_in(manifest_dir, relative);
    match std::fs::read(&path) {
        Ok(bytes) => Box::leak(bytes.into_boxed_slice()),
        Err(error) => refuse(&path, manifest_dir, relative, &error),
    }
}

/// 🏠️ Nearest ancestor of `manifest_dir` that is the declared package workspace with build authority, the repository root.
///
/// # Panics
/// When no ancestor is a workspace root, or a `package.json` beside a build marker is malformed.
#[must_use]
#[track_caller]
pub fn repo_root_in(manifest_dir: &str) -> PathBuf {
    match semio_framework_process_routing_workspace::find_workspace_root(Path::new(manifest_dir)) {
        Ok(Some(root)) => root,
        Ok(None) => panic!("repository root: no ancestor of CARGO_MANIFEST_DIR '{manifest_dir}' is a workspace root"),
        Err(error) => panic!("repository root: {error} (below CARGO_MANIFEST_DIR '{manifest_dir}')"),
    }
}

/// 📚️ Bytes of `relative` below the repository root above `manifest_dir`, read once per process and shared by every later call.
///
/// # Panics
/// When no ancestor is a workspace root, or the file is missing or unreadable.
#[must_use]
#[track_caller]
pub fn repo_bytes_in(manifest_dir: &str, relative: impl AsRef<Path>) -> &'static [u8] {
    static BYTES: OnceLock<Mutex<HashMap<PathBuf, &'static [u8]>>> = OnceLock::new();
    let relative = relative.as_ref();
    let path = repo_root_in(manifest_dir).join(relative);
    let mut cache = guard(BYTES.get_or_init(Mutex::default));
    if let Some(bytes) = cache.get(&path) {
        return bytes;
    }
    let bytes: &'static [u8] = match std::fs::read(&path) {
        Ok(bytes) => Box::leak(bytes.into_boxed_slice()),
        Err(error) => refuse(&path, manifest_dir, relative, &error),
    };
    cache.insert(path, bytes);
    bytes
}

/// 🦑️ UTF-8 text of `relative` below the repository root above `manifest_dir`, read once per process and shared by every later call.
///
/// # Panics
/// When no ancestor is a workspace root, or the file is missing, unreadable or not UTF-8.
#[must_use]
#[track_caller]
pub fn repo_text_in(manifest_dir: &str, relative: impl AsRef<Path>) -> &'static str {
    let relative = relative.as_ref();
    match std::str::from_utf8(repo_bytes_in(manifest_dir, relative)) {
        Ok(text) => text,
        Err(error) => refuse(&repo_root_in(manifest_dir).join(relative), manifest_dir, relative, &std::io::Error::new(std::io::ErrorKind::InvalidData, error)),
    }
}

fn guard<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

#[track_caller]
fn refuse(path: &Path, manifest_dir: &str, relative: &Path, error: &std::io::Error) -> ! {
    panic!("fixture {}: {error} (relative path '{}' below CARGO_MANIFEST_DIR '{manifest_dir}')", path.display(), relative.display())
}

/// 🏗️ Manifest directory of the calling crate.
#[macro_export]
macro_rules! manifest_dir {
    () => {
        ::std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    };
}

/// 📂️ Path of a fixture relative to the manifest directory of the calling crate, for readers that need a path (`read_dir`, `File::open`, `fs::write` of a blessed fixture).
#[macro_export]
macro_rules! fixture_path {
    ($relative:expr) => {
        $crate::path_in(env!("CARGO_MANIFEST_DIR"), $relative)
    };
}

/// 🧫️ UTF-8 text of a fixture relative to the manifest directory of the calling crate, as a `&'static str`.
#[macro_export]
macro_rules! fixture_text {
    ($relative:expr) => {
        $crate::text_in(env!("CARGO_MANIFEST_DIR"), $relative)
    };
}

/// 📁️ Bytes of a fixture relative to the manifest directory of the calling crate, as a `&'static [u8]`.
#[macro_export]
macro_rules! fixture_bytes {
    ($relative:expr) => {
        $crate::bytes_in(env!("CARGO_MANIFEST_DIR"), $relative)
    };
}

/// ☎️ Repository root above the calling crate, found by its declared package workspace.
#[macro_export]
macro_rules! repo_root {
    () => {
        $crate::repo_root_in(env!("CARGO_MANIFEST_DIR"))
    };
}

/// 📄️ UTF-8 text of a repository file, given relative to the repository root above the calling crate; read once per process.
#[macro_export]
macro_rules! repo_text {
    ($relative:expr) => {
        $crate::repo_text_in(env!("CARGO_MANIFEST_DIR"), $relative)
    };
}

/// 🗂️ Bytes of a repository file, given relative to the repository root above the calling crate; read once per process.
#[macro_export]
macro_rules! repo_bytes {
    ($relative:expr) => {
        $crate::repo_bytes_in(env!("CARGO_MANIFEST_DIR"), $relative)
    };
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
