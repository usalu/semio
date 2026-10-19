//! 🌿️ Direct space-alternative creation mutation.
use crate::space_history_mutations::remove_space_alternative::RemoveSpaceAlternative;
use crate::space_history_mutations::set_active_space_alternative::SetActiveSpaceAlternative;
use crate::space_history_mutations::SpaceHistoryMutation;
use crate::space_history_document::{SpaceAlternative, SpaceHistoryDiff, SpaceHistorySnapshot, SpaceHistoryStep};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
/// 🌿️ serde stays TEST-ONLY: feeds `SpaceHistoryMutation`'s own `cfg_attr(test)` oracle
/// derive (its sibling `serde_json` differential test). Production never serializes through serde.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_dsl::MutationLeaf)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct CreateSpaceAlternative {
    pub alternative: SpaceAlternative,
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl semio_framework_os_spr::MutationKind<SpaceHistorySnapshot, SpaceHistoryMutation> for CreateSpaceAlternative {
    const SEMANTICS: semio_framework_os_spr::SemanticDescriptor = semio_framework_os_spr::SemanticDescriptor { verb: "create", entity: "space-alternative", kind: "create-space-alternative", record: "CreatedSpaceAlternative" };
    fn diff(&self, _base: &SpaceHistorySnapshot) -> semio_framework_os_spr::MutationOutcome<SpaceHistoryDiff> {
        semio_framework_os_spr::MutationOutcome::new(SpaceHistoryDiff { steps: vec![SpaceHistoryStep::AddAlternative(self.alternative.clone()), SpaceHistoryStep::SetActive { alternative_id: Some(self.alternative.id.clone()) }] })
    }
    fn inverse(&self, base: &SpaceHistorySnapshot) -> Result<Vec<SpaceHistoryMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![
            SpaceHistoryMutation::RemoveSpaceAlternative(RemoveSpaceAlternative { alternative_id: self.alternative.id.clone() }),
            SpaceHistoryMutation::SetActiveSpaceAlternative(SetActiveSpaceAlternative { alternative_id: base.active_alternative_id.clone() }),
        ]
    
    })())
}
}

impl crate::os_spr::ColdMutationKind<SpaceHistorySnapshot, SpaceHistoryMutation> for CreateSpaceAlternative {
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create space alternative {}", self.alternative.name), &format!("Space-Alternative {} erstellen", self.alternative.name))
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
