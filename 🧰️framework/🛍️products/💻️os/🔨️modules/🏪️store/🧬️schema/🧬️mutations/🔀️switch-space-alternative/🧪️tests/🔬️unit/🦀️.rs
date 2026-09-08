use super::*;
use crate::os_spr::{MutationKind, MutationLeaf};
#[test]
fn metadata_and_target_subregion_are_leaf_owned() {
    let payload = SwitchSpaceAlternative { alternative_id: "alt".into() };
    assert_eq!(<SwitchSpaceAlternative as MutationLeaf>::DESCRIPTOR.payload_schema, "🧬️schema/🔣️.json");
    assert!(<SwitchSpaceAlternative as MutationLeaf>::PROVENANCE.owner.ends_with("/🔀️switch-space-alternative"));
    assert_eq!(payload.target(), vec!["activeAlternativeId"]);
}
