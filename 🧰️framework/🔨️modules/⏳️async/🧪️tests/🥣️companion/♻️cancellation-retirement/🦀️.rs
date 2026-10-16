//! ♻️ The cancellation token retirement laws.
pub use semio_framework_async::*;
use crate::{CancelNode, CancelToken};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, ValueRefusalKind, shared_retirement_allocation_bytes};
use semio_framework_value::retirement::{RetireOwned, RetirementStep};
use std::mem::size_of;
use std::sync::Arc;
use std::task::Waker;

#[path = "../../../🛑️cancel/♻️retirement/🧪️tests/🦀️.rs"]
mod tests;
