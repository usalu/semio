//! 🥣️ Companion tests of tool-machine: the transaction, scrub, typing, node-graph-edit and gesture-drive laws against the shared fixtures.
//!
//! 🧭️ The tests read the tool machines through their public API and the `testkit` hooks; every fixture is read at run time from the tool-machine module directory, so no document is compiled into the test closure.
#![cfg(test)]

use semio_framework_test_fixtures::fixture_path;
use std::{collections::HashMap, path::{Path, PathBuf}, sync::{Mutex, OnceLock}};

pub use semio_framework_tool_machine::*;

pub fn module_directory() -> &'static Path {
    static DIRECTORY: OnceLock<PathBuf> = OnceLock::new();
    DIRECTORY.get_or_init(|| fixture_path!("../../../.."))
}

fn leaked(relative: &str) -> &'static [u8] {
    static READ: OnceLock<Mutex<HashMap<String, &'static [u8]>>> = OnceLock::new();
    let mut read = READ.get_or_init(Default::default).lock().unwrap();
    read.entry(relative.to_owned()).or_insert_with(|| {
        let path = module_directory().join(relative);
        Box::leak(std::fs::read(&path).unwrap_or_else(|error| panic!("tool-machine test input {}: {error}", path.display())).into_boxed_slice())
    })
}

pub fn fixture_text(relative: &str) -> &'static str {
    std::str::from_utf8(leaked(relative)).unwrap_or_else(|error| panic!("tool-machine test input {relative} is not UTF-8: {error}"))
}

pub fn fixture_bytes(relative: &str) -> &'static [u8] {
    leaked(relative)
}

#[path = "../🔬️unit/🦀️.rs"]
mod tests;

#[path = "../🧪️node-graph-edit-rows/🦀️.rs"]
mod node_graph_edit_rows_tests;

#[path = "../🧪️gesture-drive-law/🦀️.rs"]
mod gesture_drive_law_tests;
