//! 🔁️ Direct interaction-state replacement payload, semantics and source-owned metadata.

use crate::app::InteractionConfigMutation;
use protocol::InteractionState;

//#region 🔖️Payload
// 🌱️ `InteractionState` (defined in `📡️replication/📡️wire/🦀️.rs`) carries only the hand-written
// `ToValue`/`FromValue` codec — its `serde` derive is gone, so this wrapper cannot derive `serde`
// either. The transparent passthrough it used to get from `#[serde(transparent)]` is supplied by
// the hand-written impls below, which forward straight to the inner state.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, serde::Serialize, serde::Deserialize)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetInteractionState { pub state: InteractionState }

impl protocol::ToValue for SetInteractionState {
    fn to_value(&self) -> protocol::DslValue {
        protocol::ToValue::to_value(&self.state)
    }
}
impl protocol::FromValue for SetInteractionState {
    fn from_value(value: protocol::DslValue) -> Result<Self, protocol::ValueError> {
        Ok(Self { state: protocol::FromValue::from_value(value)? })
    }
}
//#endregion 🔖️Payload

//#region ⚙️ColdSemantics
impl SetInteractionState {
    /// 🧊️ Ordinary mutation evaluation; retained publication supplies its exact prebuilt root separately.
    pub fn apply(&self) -> protocol::MutationApplyResult<InteractionState> { Ok(self.state.clone()) }
    pub fn diff(&self) -> protocol::MutationOutcome<InteractionConfigMutation> { protocol::MutationOutcome::new(InteractionConfigMutation::SetState(self.clone())) }
    pub fn inverse(&self, base: &InteractionState) -> Vec<InteractionConfigMutation> { vec![InteractionConfigMutation::set_state(base.clone())] }
}
//#endregion ⚙️ColdSemantics

#[cfg(test)]
#[path = "🧪️.rs"]
mod tests;
