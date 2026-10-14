//! 🌿️ Direct space-alternative creation mutation.
use super::super::{RemoveSpaceAlternative, SetActiveSpaceAlternative, SpaceHistoryMutation};
use super::super::{SpaceAlternative, SpaceHistoryDiff, SpaceHistorySnapshot, SpaceHistoryStep};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
/// 🌿️ serde stays TEST-ONLY: feeds `SpaceHistoryMutation`'s own `cfg_attr(test)` oracle
/// derive (its sibling `serde_json` differential test). Production never serializes through serde.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct CreateSpaceAlternative {
    pub alternative: SpaceAlternative,
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl crate::os_spr::MutationKind<SpaceHistorySnapshot, SpaceHistoryMutation> for CreateSpaceAlternative {
    const SEMANTICS: crate::os_spr::SemanticDescriptor = crate::os_spr::SemanticDescriptor { verb: "create", entity: "space-alternative", kind: "create-space-alternative", record: "CreatedSpaceAlternative" };
    fn diff(&self, _base: &SpaceHistorySnapshot) -> crate::os_spr::MutationOutcome<SpaceHistoryDiff> {
        crate::os_spr::MutationOutcome::new(SpaceHistoryDiff { steps: vec![SpaceHistoryStep::AddAlternative(self.alternative.clone()), SpaceHistoryStep::SetActive { alternative_id: Some(self.alternative.id.clone()) }] })
    }
    fn inverse(&self, base: &SpaceHistorySnapshot) -> Result<Vec<SpaceHistoryMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![
            SpaceHistoryMutation::RemoveSpaceAlternative(RemoveSpaceAlternative { alternative_id: self.alternative.id.clone() }),
            SpaceHistoryMutation::SetActiveSpaceAlternative(SetActiveSpaceAlternative { alternative_id: base.active_alternative_id.clone() }),
        ]
    
    })())
}
    fn label(&self) -> crate::LocalizedLabel {
        crate::LocalizedLabel::native(&format!("Create space alternative {}", self.alternative.name), &format!("Space-Alternative {} erstellen", self.alternative.name))
    }
    fn target(&self) -> Vec<String> {
        vec!["alternatives".into(), self.alternative.id.clone()]
    }
}
//#endregion ⚙️Semantics

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
