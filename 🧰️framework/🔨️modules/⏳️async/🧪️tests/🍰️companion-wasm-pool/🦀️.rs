//! 🍰️ Companion tests of the cooperative wasm worker pool of async, run natively through the `wasm-pool-host` feature.
//!
//! 🧭️ The laws sit below a wrapper that stands for the `wasm_pool` module: it restates the module's imports and uses the crate's `wasm_internals` hooks.
#![cfg(test)]


use std::{collections::HashMap, path::{Path, PathBuf}, sync::{Mutex, OnceLock}};

pub fn product_directory() -> &'static Path {
    static DIRECTORY: OnceLock<PathBuf> = OnceLock::new();
    DIRECTORY.get_or_init(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.."))
}

fn leaked(relative: &str) -> &'static [u8] {
    static READ: OnceLock<Mutex<HashMap<String, &'static [u8]>>> = OnceLock::new();
    let mut read = READ.get_or_init(Default::default).lock().unwrap();
    read.entry(relative.to_owned()).or_insert_with(|| {
        let path = product_directory().join(relative);
        Box::leak(std::fs::read(&path).unwrap_or_else(|error| panic!("test input {}: {error}", path.display())).into_boxed_slice())
    })
}

pub fn text(relative: &str) -> &'static str {
    std::str::from_utf8(leaked(relative)).unwrap_or_else(|error| panic!("test input {relative} is not UTF-8: {error}"))
}

pub fn bytes(relative: &str) -> &'static [u8] {
    leaked(relative)
}

#[path = "🕸️wasm-pool/🦀️.rs"]
pub mod wasm_pool;
