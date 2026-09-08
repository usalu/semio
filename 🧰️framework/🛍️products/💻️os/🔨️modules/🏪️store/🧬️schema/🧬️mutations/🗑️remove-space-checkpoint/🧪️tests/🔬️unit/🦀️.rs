
use super::*;
use crate::os_spr::{MutationKind, MutationLeaf};
#[test]
fn metadata_and_target_subregion_are_leaf_owned() {
    let payload = RemoveSpaceCheckpoint { checkpoint_id: "cp".into() };
    assert_eq!(<RemoveSpaceCheckpoint as MutationLeaf>::DESCRIPTOR.payload_schema, "🧬️schema/🔣️.json");
    assert!(<RemoveSpaceCheckpoint as MutationLeaf>::PROVENANCE.owner.ends_with("/🗑️remove-space-checkpoint"));
    assert_eq!(payload.target(), vec!["checkpoints", "cp"]);
}
