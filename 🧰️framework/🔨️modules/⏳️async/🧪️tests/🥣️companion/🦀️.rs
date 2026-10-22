//! 🥣️ Companion tests of async: cancellation, scopes, combinators, publication, the native worker pool and its parking, maintenance and deferred wake registries.
//!
//! 🧭️ The root laws are mounted flat; the laws of private modules sit below wrappers that restate the module's imports and use the crate's `internals` hooks.
#![cfg(test)]

#![allow(async_fn_in_trait)]
use semio_framework_test_fixtures::fixture_path;

pub use semio_framework_async::*;
pub use semio_framework_async_slots::*;
pub use semio_framework_async_sync::*;
pub use semio_framework_async_timer::*;
pub use semio_framework_async::internals::LANE_COUNT;
pub use semio_framework_async::internals::CancelNode;
use std::collections::VecDeque;
use std::{future::Future, pin::Pin, sync::{Arc, atomic::{AtomicU32, Ordering}}, task::{Context, Poll, Waker}};

use std::{collections::HashMap, path::{Path, PathBuf}, sync::{Mutex, OnceLock}};

pub fn product_directory() -> &'static Path {
    static DIRECTORY: OnceLock<PathBuf> = OnceLock::new();
    DIRECTORY.get_or_init(|| fixture_path!("../../../.."))
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

#[global_allocator]
static ASYNC_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

//#region 🧬️Tests
#[path = "../🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧬️Tests

#[path = "../../🔐️publication/🧪️tests/🦀️.rs"]
mod publication_tests;

#[path = "../../🛑️cancel/🎟️admission/🧪️tests/🦀️.rs"]
mod original_cancel_admission_tests;

#[path = "📬️publication-claim/🦀️.rs"]
pub mod publication_claim;

#[path = "🔂️poll/🦀️.rs"]
pub mod poll;

#[path = "🧵️thread-wake/🦀️.rs"]
pub mod original_thread_wake;

#[path = "🔔️deferred-wake/🦀️.rs"]
pub mod deferred_wake;

#[path = "🛠️maintenance/🦀️.rs"]
pub mod maintenance;

#[path = "🛌️worker-parking/🦀️.rs"]
pub mod worker_parking;

#[path = "♻️cancellation-retirement/🦀️.rs"]
pub mod cancellation_retirement;

#[path = "🖥️native-pool/🦀️.rs"]
pub mod native_pool;
