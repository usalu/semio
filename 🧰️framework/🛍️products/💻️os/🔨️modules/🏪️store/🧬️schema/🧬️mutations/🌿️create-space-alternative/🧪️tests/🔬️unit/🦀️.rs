use super::*;
use crate::os_spr::{MutationKind, MutationLeaf};
#[test]
fn metadata_and_activation_diff_are_leaf_owned() {
    assert_eq!(<CreateSpaceAlternative as MutationLeaf>::DESCRIPTOR.payload_schema, "🧬️schema/🔣️.json");
    assert!(<CreateSpaceAlternative as MutationLeaf>::PROVENANCE.owner.ends_with("/🌿️create-space-alternative"));
    let payload = CreateSpaceAlternative { alternative: SpaceAlternative { id: "alt".into(), name: "alt".into(), checkpoint_ids: Vec::new() } };
    assert_eq!(payload.diff(&SpaceHistorySnapshot::default()).diff().steps.last(), Some(&super::super::super::SpaceHistoryStep::SetActive { alternative_id: Some("alt".into()) }));
}
