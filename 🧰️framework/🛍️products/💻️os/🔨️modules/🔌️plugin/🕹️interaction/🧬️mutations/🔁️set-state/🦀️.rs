//! 🔁️ Direct interaction-state replacement payload, semantics and source-owned metadata.

use super::{DomainEdit, InteractionConfigMutation, InteractionStateDiff};
use protocol::InteractionState;

//#region 🔖️Payload
// 🌱️ The wire of this leaf is its inner `InteractionState` alone (`#[value(transparent)]`): its plain, controlled and canonical-tree codecs are derived and forward to the state.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value_derive::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(transparent)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetInteractionState {
    pub state: InteractionState,
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
