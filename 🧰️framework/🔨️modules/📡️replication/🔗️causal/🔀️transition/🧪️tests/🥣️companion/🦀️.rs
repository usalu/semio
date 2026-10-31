//! 🥣️ Companion tests of history transitions: bounded wire cursors, input drafts, fold birth and retirement, supersession folds and the transition codec.
//!
//! 🧭️ Flat laws resolve `super::*` to the public vocabulary of the history crate plus its `testkit` forwarders of crate-private items.
#![cfg(test)]

pub use semio_framework_replication_history::*;
pub use semio_framework_replication_ids::*;
pub use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,admit_owned_retirement,owned_retirement_birth_bytes,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor}};
pub use std::sync::atomic::{AtomicUsize,AtomicU64,Ordering};
pub use std::{future::Future,pin::Pin,task::{Context,Poll}};
pub use semio_framework_value::ErasedSnapshotRetirement;

use semio_framework_value_testing_allocator as _;

#[path = "📤️wire/🦀️.rs"]
mod wire;
#[path = "📝️drafts/🦀️.rs"]
mod drafts;
#[path = "🏗️birth/🦀️.rs"]
mod birth;
#[path = "📏️retirement/🦀️.rs"]
mod retirement;
#[path = "🔬️unit/🦀️.rs"]
mod unit;
#[path = "🔬️supersede-fold/🦀️.rs"]
mod supersede_fold;
