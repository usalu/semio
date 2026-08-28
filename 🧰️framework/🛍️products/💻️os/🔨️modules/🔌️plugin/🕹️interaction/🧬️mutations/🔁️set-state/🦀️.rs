//! 🔁️ Direct interaction-state replacement payload, semantics and source-owned metadata.

use crate::app::InteractionConfigMutation;
use protocol::InteractionState;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(transparent)]
pub struct SetInteractionState { pub state: InteractionState }
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
