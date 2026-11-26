//! 🔒️ Wasmtime cold compilation admission across processes sharing one cache.

use std::fs::{File, OpenOptions};
use std::io::{self, Read};
use std::path::Path;

#[path = "🔋️memory/🦀️.rs"]
mod memory;

pub use memory::require_compile_memory;

pub(super) fn native_compile_lock(cache_root: &Path) -> io::Result<File> {
    std::fs::create_dir_all(cache_root)?;
    OpenOptions::new().read(true).write(true).create(true).truncate(false).open(cache_root.join("cold-compile.lock"))
}

/// 🎟️ Holds the exclusive cold compiler slot until the returned file is dropped. A busy slot returns `WouldBlock`.
pub fn try_native_compile(cache_root: &Path) -> io::Result<File> {
    let file = native_compile_lock(cache_root)?;
    file.try_lock().map_err(io::Error::from)?;
    Ok(file)
}

pub(super) fn read_compile_input(input: impl Read, expected: usize) -> io::Result<Vec<u8>> {
    let limit = (expected as u64).checked_add(1).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "component length exceeds the worker input range"))?;
    let mut bytes = Vec::new();
    input.take(limit).read_to_end(&mut bytes)?;
    if bytes.len() != expected {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "component stream length differs from the admitted byte count"));
    }
    Ok(bytes)
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
