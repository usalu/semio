//! 🔒️ Per-thread isolation of the process-wide cells of the wgpu renderer.
//!
//! 🧭️ Native renderer state (scene state, worker cells) is one process-wide instance shared by the shared-pool workers. A test build asks for
//! one instance per test thread instead, so no law meets another law's scenes. The ask is the cargo feature `isolated` of this crate: a test
//! target enables it through a dev-dependency on this crate, which has no dependency of its own, so enabling it never pulls a heavy crate
//! into the test closure. [`ISOLATED`] reports the ask and [`isolated_cell`] holds the per-thread instances.
//!
//! @see `semio-framework-os-renderer-wgpu-scene-state` — `SceneCell`
//! @see `semio-framework-os-renderer-wgpu-contracts` — `test_worker_cell`

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;

/// 🧪️ Whether a build asked for one instance of every process-wide cell per thread.
pub const ISOLATED: bool = cfg!(feature = "isolated");

thread_local! {
    /// 🧪️ This thread's state of each cell, by the cell's address; leaked for the thread's (one law's) life.
    static CELLS: RefCell<HashMap<usize, &'static dyn Any>> = RefCell::new(HashMap::new());
}

/// 🧪️ The calling thread's own instance of the state at `key`, built by `initialize` on first use.
pub fn isolated_cell<T: 'static>(key: usize, initialize: impl FnOnce() -> T) -> &'static Mutex<T> {
    let state = CELLS.with(|cells| *cells.borrow_mut().entry(key).or_insert_with(|| Box::leak(Box::new(Mutex::new(initialize()))) as &'static dyn Any));
    state.downcast_ref::<Mutex<T>>().expect("one cell holds one state type")
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
