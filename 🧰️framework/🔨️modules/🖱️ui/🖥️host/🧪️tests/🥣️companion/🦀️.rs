//! 🥣️ Companion tests of ui-host: platform event normalization, the event queue admission and the private input byte buffer.
//!
//! 🧭️ The laws read the host through its public vocabulary and the `testkit` hooks; the allocator that observes allocation is the companion's own copy of the host's test allocator.
#![cfg(test)]

pub use semio_framework_ui_host::*;
use semio_framework_abi::AbiEvent;
use ui_render::{DispatchEvent, ImeEvent, PointerButton, PointerKind};

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

#[path = "🔎️observed-allocator/🦀️.rs"]
mod observed_allocator;

mod input_root_tests {
    pub(crate) use crate::observed_allocator::{allocations_end, allocations_start};
}

#[path = "../../📥️input/🎟️admission/✍️writer/🦀️.rs"]
mod input_writer;

#[path = "../../📥️input/🎟️admission/✍️writer/🧪️tests/✍️writer/🦀️.rs"]
mod input_writer_tests;

//#region Tests
#[path = "../🔬️event-unit/🦀️.rs"]
mod tests;
//#endregion Tests

#[path = "../🔬️enqueue-unit/🦀️.rs"]
mod enqueue_tests;

#[path = "🪟️window/🦀️.rs"]
mod window;

pub mod event {
    pub use semio_framework_ui_host::*;
}
