//! 🕸️ The cooperative wasm worker pool laws, run natively through the `wasm-pool-host` feature.
pub use semio_framework_async::*;
use semio_framework_async::wasm_internals::{CooperativePoolSnapshot, WorkerPool, WorkerPoolUse};
use std::{sync::{Arc, Mutex, atomic::{AtomicU32, Ordering}}, task::Waker};

include!("../../🔬️wasm-pool-cooperative/🦀️.rs");

