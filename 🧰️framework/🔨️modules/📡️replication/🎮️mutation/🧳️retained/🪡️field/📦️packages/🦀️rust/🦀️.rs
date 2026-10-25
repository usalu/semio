//! 🪡️ Retained field changes glue — wiring only. Domain lives at the owner `🦀️.rs` files.

use semio_framework_value::RetirementDemand;
use semio_framework_value::retained_clone::RetainedCloneGrant;

pub use semio_framework_mutation_diff::ApplyCapability;

/// 📹️ Sparse and record field changes under central grant admission.
#[path = "../../🦀️.rs"]
pub mod field;
pub use field::{RetainedFieldPreparation, RetainedFieldChange, RetainedRecordChange, RetainedRecordChangeField, RetainedRecordFieldChange, RetainedRecordChangeOwner, boxed_record_field};

/// 🎟️ Independent five-axis admission precedes every action.
pub fn retained_mutation_demand_fits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}

#[cfg(test)]
pub(crate) use semio_framework_trace_memory_testing as test_allocation;
#[cfg(test)]
#[global_allocator]
static REQUESTED_ALLOCATOR: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;
