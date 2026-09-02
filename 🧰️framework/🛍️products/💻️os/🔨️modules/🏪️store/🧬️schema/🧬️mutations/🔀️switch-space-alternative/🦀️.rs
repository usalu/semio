//! 🔀️ Direct active space-alternative switch mutation.
use super::super::{RestoreActiveSpaceAlternative, SpaceHistoryMutation};
use super::super::{SpaceHistoryDiff, SpaceHistorySnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
/// @emoji 🔀️ serde stays TEST-ONLY: feeds `SpaceHistoryMutation`'s own `cfg_attr(test)` oracle
/// derive (its sibling `serde_json` differential test). Production never serializes through serde.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SwitchSpaceAlternative {
    pub alternative_id: String,
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl crate::os_spr::MutationKind<SpaceHistorySnapshot, SpaceHistoryMutation> for SwitchSpaceAlternative {
    const SEMANTICS: crate::os_spr::SemanticDescriptor = crate::os_spr::SemanticDescriptor { verb: "switch", entity: "space-alternative", kind: "switch-space-alternative", record: "SwitchedSpaceAlternative" };
    fn diff(&self, _base: &SpaceHistorySnapshot) -> crate::os_spr::MutationOutcome<SpaceHistoryDiff> {
        crate::os_spr::MutationOutcome::new(SpaceHistoryDiff { set_active_alternative_id: Some(Some(self.alternative_id.clone())), ..Default::default() })
    }
    fn inverse(&self, base: &SpaceHistorySnapshot) -> Vec<SpaceHistoryMutation> {
        vec![SpaceHistoryMutation::RestoreActiveSpaceAlternative(RestoreActiveSpaceAlternative { alternative_id: base.active_alternative_id.clone() })]
    }
    fn label(&self) -> String {
        format!("Switch space alternative {}", self.alternative_id)
    }
    fn target(&self) -> Vec<String> {
        vec!["activeAlternativeId".into()]
    }
}
//#endregion ⚙️Semantics

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::os_spr::{MutationKind, MutationLeaf};
    #[test]
    fn metadata_and_target_subregion_are_leaf_owned() {
        let payload = SwitchSpaceAlternative { alternative_id: "alt".into() };
        assert_eq!(<SwitchSpaceAlternative as MutationLeaf>::DESCRIPTOR.payload_schema, "🧬️schema/🔣️.json");
        assert!(<SwitchSpaceAlternative as MutationLeaf>::PROVENANCE.owner.ends_with("/🔀️switch-space-alternative"));
        assert_eq!(payload.target(), vec!["activeAlternativeId"]);
    }
}
//#endregion 🧪️Tests
