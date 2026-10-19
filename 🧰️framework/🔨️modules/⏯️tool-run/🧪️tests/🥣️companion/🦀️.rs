//! 🥣️ Companion tests of tool-run: the lifecycle, trace and tick laws against the shared fixtures, and the original native writer, receiving and retirement laws.
//!
//! 🧭️ The tests read the run contract through its public API and the `testkit` hooks; every fixture is read at run time from the tool-run module directory, so no document is compiled into the test closure.
#![cfg(test)]

use semio_framework_test_fixtures::fixture_path;
use std::{collections::HashMap, path::{Path, PathBuf}, sync::{Mutex, OnceLock}};

pub use semio_framework_tool_run::testkit::*;
pub use semio_framework_tool_run::*;
pub use semio_framework_tool_run_tick_wire::testkit::*;
pub use semio_framework_tool_run_tick_wire::*;

#[global_allocator]
static TOOL_RUN_ORIGINAL_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

pub fn module_directory() -> &'static Path {
    static DIRECTORY: OnceLock<PathBuf> = OnceLock::new();
    DIRECTORY.get_or_init(|| fixture_path!("../../../.."))
}

fn leaked(relative: &str) -> &'static [u8] {
    static READ: OnceLock<Mutex<HashMap<String, &'static [u8]>>> = OnceLock::new();
    let mut read = READ.get_or_init(Default::default).lock().unwrap();
    read.entry(relative.to_owned()).or_insert_with(|| {
        let path = module_directory().join(relative);
        Box::leak(std::fs::read(&path).unwrap_or_else(|error| panic!("tool-run test input {}: {error}", path.display())).into_boxed_slice())
    })
}

pub fn fixture_text(relative: &str) -> &'static str {
    std::str::from_utf8(leaked(relative)).unwrap_or_else(|error| panic!("tool-run test input {relative} is not UTF-8: {error}"))
}

pub fn fixture_bytes(relative: &str) -> &'static [u8] {
    leaked(relative)
}

#[path = "../🔬️unit/🦀️.rs"]
mod tests;

#[path = "../../♻️retirement/🧪️tests/🦀️.rs"]
mod original_retirement_tests;

#[path = "../../🎞️tick/⏭️successor/🧪️tests/🦀️.rs"]
mod original_tick_successor_tests;

#[path = "../../🎞️tick/🎟️admission/🧪️tests/🦀️.rs"]
mod original_tick_admission_tests;

#[path = "../../🎞️tick/🛫️encode/🧪️tests/🦀️.rs"]
mod original_tick_encode_tests;

#[path = "../../🎞️tick/🛬️receive/🎟️limits/🧪️tests/🦀️.rs"]
mod original_receiving_limits_tests;

#[path = "../../🎞️tick/🛬️receive/🏗️native/🧪️tests/🦀️.rs"]
mod original_native_tick_tests;

#[path = "../../🎞️tick/🛬️receive/📊️progress/🧪️tests/🦀️.rs"]
mod original_progress_receiving_tests;

#[path = "../../🎞️tick/🛬️receive/📝️step/🧪️tests/🦀️.rs"]
mod original_step_receiving_tests;

#[path = "../../🎞️tick/🛬️receive/🔢️entities/🧪️tests/🦀️.rs"]
mod original_entity_column_tests;

#[path = "../../🎞️tick/🛬️receive/🪪️identity/🧪️tests/🦀️.rs"]
mod original_identity_receiving_tests;
