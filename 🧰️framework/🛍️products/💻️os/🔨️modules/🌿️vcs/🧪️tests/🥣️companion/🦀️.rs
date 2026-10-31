//! 🥣️ Companion tests of the version-graph algebra: history ledger, group visibility, read custody, checkpoint and entity identities, run against the public surface of `semio-framework-os-vcs`.
//!
//! 🧭️ The test sources stay beside their owners and are mounted here with `#[path]` below wrappers that restate the imports of the owning module; every fixture is read at run time from the vcs folder.
#![cfg(test)]

pub use protocol::{ColdMutation, Edit, Mutation, MutationApplyError, MutationDiff};
pub use semio_framework_os_vcs::*;
pub use semio_framework_value::{DslValue, FromValue, ToValue, ValueError};
pub(crate) use semio_framework_trace_memory_testing as test_allocation;

#[global_allocator]
static REQUESTED_ALLOCATOR: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;

#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[path = "../../🧪️tests/🧪️fault-params/🦀️.rs"]
mod fault_params_tests;
#[path = "🗃️history-ledger/🦀️.rs"]
mod history_ledger;
#[path = "🪟️group-visibility/🦀️.rs"]
mod group_visibility;
#[path = "🪪️entity-identity/🦀️.rs"]
mod entity_identity;
#[path = "🪪️checkpoint-identity/🦀️.rs"]
mod checkpoint_identity;
