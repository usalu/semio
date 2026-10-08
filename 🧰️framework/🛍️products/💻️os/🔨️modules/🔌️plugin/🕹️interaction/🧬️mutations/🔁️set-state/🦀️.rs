//! 🔁️ Direct interaction-state replacement payload, semantics and source-owned metadata.

use super::{DomainEdit, InteractionConfigMutation, InteractionStateDiff};
use protocol::InteractionState;

//#region 🔖️Payload
// 🌱️ `InteractionState` (defined in `📡️replication/📡️wire/🦀️.rs`) carries only the hand-written
// `ToValue`/`FromValue` codec — its `serde` derive is gone, so this wrapper cannot derive `serde`
// either. The transparent passthrough it used to get from `#[serde(transparent)]` is supplied by
// the hand-written impls below, which forward straight to the inner state.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, serde::Serialize, serde::Deserialize)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetInteractionState {
    pub state: InteractionState,
}

impl semio_framework_value::ToValue for SetInteractionState {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::ToValue::to_value(&self.state)
    }
}
impl semio_framework_value::FromValue for SetInteractionState {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Ok(Self { state: semio_framework_value::FromValue::from_value(value)? })
    }
}
//#endregion 🔖️Payload

//#region ⚙️ColdSemantics
impl protocol::MutationKind<InteractionState, InteractionConfigMutation> for SetInteractionState {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "interaction-state", kind: "set-interaction-state", record: "SetInteractionState" };
    fn diff(&self, base: &InteractionState) -> protocol::MutationOutcome<InteractionStateDiff> {
        protocol::MutationOutcome::new(InteractionStateDiff {
            selection: DomainEdit::changed(&base.selection, &self.state.selection),
            hover: DomainEdit::changed(&base.hover, &self.state.hover),
            active_mode: DomainEdit::changed(&base.active_mode, &self.state.active_mode),
            active_granularity: DomainEdit::changed(&base.active_granularity, &self.state.active_granularity),
        })
    }
    fn inverse(&self, base: &InteractionState) -> Result<Vec<InteractionConfigMutation>, semio_framework_value::ValueError> {
        Ok(vec![InteractionConfigMutation::SetInteractionState(Self { state: base.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set interaction state", "Interaktionszustand setzen")
    }
}
//#endregion ⚙️ColdSemantics

#[cfg(test)]
#[path = "🧪️tests/🧪️set-state/🦀️.rs"]
mod tests;
