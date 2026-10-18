//! 🔬️ Companion white-box laws of the job core: the private operation ledger, the writer's staged and rejected custody and the payload retirement ladder.
#![cfg(test)]

use semio_framework_job::RetainedCloneGrant;

pub(crate) const TEST_RETAINED_POLICY:RetainedCloneGrant=RetainedCloneGrant{maximum_items:16,maximum_copy_bytes:32768,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128};

#[global_allocator]
static JOB_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[path = "../🔬️retained-ownership/🦀️.rs"]
mod retained_ownership_tests;

#[path = "../../♻️retirement/📄️payload/🧪️tests/🦀️.rs"]
mod payload_retirement_tests;
