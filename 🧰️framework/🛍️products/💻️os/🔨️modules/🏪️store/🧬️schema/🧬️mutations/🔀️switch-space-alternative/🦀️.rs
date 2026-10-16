//! 🔀️ Direct active space-alternative switch mutation.
use crate::space_history_mutations::set_active_space_alternative::SetActiveSpaceAlternative;
use crate::space_history_mutations::SpaceHistoryMutation;
use crate::space_history_document::{SpaceHistoryDiff, SpaceHistorySnapshot, SpaceHistoryStep};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
/// 🔀️ serde stays TEST-ONLY: feeds `SpaceHistoryMutation`'s own `cfg_attr(test)` oracle
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
impl semio_framework_os_spr::MutationKind<SpaceHistorySnapshot, SpaceHistoryMutation> for SwitchSpaceAlternative {
    const SEMANTICS: semio_framework_os_spr::SemanticDescriptor = semio_framework_os_spr::SemanticDescriptor { verb: "switch", entity: "space-alternative", kind: "switch-space-alternative", record: "SwitchedSpaceAlternative" };
    fn diff(&self, _base: &SpaceHistorySnapshot) -> semio_framework_os_spr::MutationOutcome<SpaceHistoryDiff> {
        semio_framework_os_spr::MutationOutcome::new(SpaceHistoryDiff::step(SpaceHistoryStep::SetActive { alternative_id: Some(self.alternative_id.clone()) }))
    }
    fn inverse(&self, base: &SpaceHistorySnapshot) -> Result<Vec<SpaceHistoryMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![SpaceHistoryMutation::SetActiveSpaceAlternative(SetActiveSpaceAlternative { alternative_id: base.active_alternative_id.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Switch space alternative {}", self.alternative_id), &format!("Zu Space-Alternative {} wechseln", self.alternative_id))
    }
    fn target(&self) -> Vec<String> {
        vec!["activeAlternativeId".into()]
    }
}
//#endregion ⚙️Semantics

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
