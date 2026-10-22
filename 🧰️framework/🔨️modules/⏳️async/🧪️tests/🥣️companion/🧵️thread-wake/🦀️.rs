//! 🧵️ The native thread wake custody laws.
pub use semio_framework_async::original_thread_wake::*;
use semio_framework_value::ValueRefusalKind;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retirement::{RetireOwned, RetirementCursor, RetirementStep};
use std::mem::size_of;
use std::sync::Arc;
use std::task::Waker;
pub use semio_framework_async::original_thread_wake::internals::*;

#[path = "../../../🔔️wake/🧵️thread/🧪️tests/🦀️.rs"]
mod tests;
