
use super::*;
use crate::os_spr::{MutationKind, MutationLeaf};
#[test]
fn metadata_and_diff_are_leaf_owned() {
    assert_eq!(<CommitSpaceCheckpoint as MutationLeaf>::DESCRIPTOR.payload_schema, "🧬️schema/🔣️.json");
    assert!(<CommitSpaceCheckpoint as MutationLeaf>::PROVENANCE.owner.ends_with("/📌️commit-space-checkpoint"));
    let payload = CommitSpaceCheckpoint { checkpoint: SpaceCheckpoint { id: "cp".into(), parent_id: None, message: String::new(), authors: Vec::new(), timestamp: crate::os_spr::HybridLogicalTimestamp::new(0, 0), members: Vec::new() } };
    assert_eq!(payload.diff(&SpaceHistorySnapshot::default()).diff().add_checkpoint.as_ref().map(|value| value.id.as_str()), Some("cp"));
}
