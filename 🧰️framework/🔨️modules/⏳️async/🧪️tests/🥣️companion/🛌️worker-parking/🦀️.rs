//! 🛌️ The idle worker parking laws.
pub use semio_framework_async::*;
use std::sync::PoisonError;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
pub use semio_framework_async::internals::worker_parking::*;

#[path = "../../../🔔️worker-parking/🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
