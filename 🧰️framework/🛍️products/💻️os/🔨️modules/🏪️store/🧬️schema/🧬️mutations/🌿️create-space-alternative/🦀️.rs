//! 🌿️ Direct space-alternative creation mutation.
use super::super::{RestoreActiveSpaceAlternative, RemoveSpaceAlternative, SpaceHistoryMutation};
use super::super::{SpaceAlternative, SpaceHistoryDiff, SpaceHistorySnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
// 🔀️ No ToValue/FromValue here: SpaceAlternative embeds foreign types that cannot derive them
// (orphan rule), and the aggregate's ToValue/FromValue is hand-written via the serde bridge
// (`../../🦀️component.rs`), not per-leaf — see that impl's docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CreateSpaceAlternative { pub alternative: SpaceAlternative }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl crate::os_spr::MutationKind<SpaceHistorySnapshot, SpaceHistoryMutation> for CreateSpaceAlternative {
    const SEMANTICS: crate::os_spr::SemanticDescriptor = crate::os_spr::SemanticDescriptor { verb: "create", entity: "space-alternative", kind: "create-space-alternative", record: "CreatedSpaceAlternative" };
    fn diff(&self, _base: &SpaceHistorySnapshot) -> crate::os_spr::MutationOutcome<SpaceHistoryDiff> { crate::os_spr::MutationOutcome::new(SpaceHistoryDiff { add_alternative: Some(self.alternative.clone()), set_active_alternative_id: Some(Some(self.alternative.id.clone())), ..Default::default() }) }
    fn inverse(&self, base: &SpaceHistorySnapshot) -> Vec<SpaceHistoryMutation> { vec![SpaceHistoryMutation::RestoreActiveSpaceAlternative(RestoreActiveSpaceAlternative { alternative_id: base.active_alternative_id.clone() }), SpaceHistoryMutation::RemoveSpaceAlternative(RemoveSpaceAlternative { alternative_id: self.alternative.id.clone() })] }
    fn label(&self) -> String { format!("Create space alternative {}", self.alternative.name) }
    fn target(&self) -> Vec<String> { vec!["alternatives".into(), self.alternative.id.clone()] }
}
//#endregion ⚙️Semantics

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::os_spr::{MutationKind, MutationLeaf};
    #[test]
    fn metadata_and_activation_diff_are_leaf_owned() {
        assert_eq!(<CreateSpaceAlternative as MutationLeaf>::DESCRIPTOR.payload_schema, "🧬️schema/🔣️.json");
        assert!(<CreateSpaceAlternative as MutationLeaf>::PROVENANCE.owner.ends_with("/🌿️create-space-alternative"));
        let payload = CreateSpaceAlternative { alternative: SpaceAlternative { id: "alt".into(), name: "alt".into(), checkpoint_ids: Vec::new() } };
        assert_eq!(payload.diff(&SpaceHistorySnapshot::default()).diff().set_active_alternative_id, Some(Some("alt".into())));
    }
}
//#endregion 🧪️Tests
