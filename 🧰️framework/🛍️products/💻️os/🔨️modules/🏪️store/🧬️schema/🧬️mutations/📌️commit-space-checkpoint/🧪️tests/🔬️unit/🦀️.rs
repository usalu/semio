use super::*;
use semio_framework_os_spr::{MutationKind, MutationLeaf};
#[test]
fn metadata_and_diff_are_leaf_owned() {
    assert_eq!(<CommitSpaceCheckpoint as MutationLeaf>::DESCRIPTOR.payload_schema, "🧬️schema/🔣️.json");
    assert!(<CommitSpaceCheckpoint as MutationLeaf>::PROVENANCE.owner.ends_with("/📌️commit-space-checkpoint"));
    let payload = CommitSpaceCheckpoint { checkpoint: SpaceCheckpoint { id: "cp".into(), parent_id: None, message: String::new(), authors: Vec::new(), timestamp: semio_framework_os_spr::HybridLogicalTimestamp::new(0, 0), members: Vec::new() } };
    assert!(matches!(payload.diff(&SpaceHistorySnapshot::default()).diff().steps.as_slice(), [crate::space_history_document::SpaceHistoryStep::AddCheckpoint(value)] if value.id == "cp"));
}
