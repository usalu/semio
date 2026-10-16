//! 🖥️ The native worker pool laws.
pub use semio_framework_async::*;
use std::thread;
use std::sync::atomic::AtomicUsize;
use std::time::{Duration, Instant};
pub use semio_framework_async::internals::native_pool::*;
use semio_framework_async::internals::LANE_COUNT;
use std::{sync::{Arc, Mutex, atomic::Ordering}, task::Waker};

include!("../../🔬️native-pool-unit/🦀️.rs");

