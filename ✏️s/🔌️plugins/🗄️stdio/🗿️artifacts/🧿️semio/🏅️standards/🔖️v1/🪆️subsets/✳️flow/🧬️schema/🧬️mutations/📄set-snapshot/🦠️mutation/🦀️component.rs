use crate::artifacts::semio::standards::v1::subsets::flow::schema::mutations::{apply_semio_flow_mutation, SemioFlowMutation};
use crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut SemioFlowSnapshot, mutation: &SemioFlowMutation) {
    let _ = apply_semio_flow_mutation(projection, mutation);
}
