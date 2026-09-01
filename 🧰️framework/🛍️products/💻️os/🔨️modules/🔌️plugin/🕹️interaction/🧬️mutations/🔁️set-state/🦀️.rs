//! 🔁️ Direct interaction-state replacement payload, semantics and source-owned metadata.

use crate::app::InteractionConfigMutation;
use protocol::InteractionState;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
// 🌱️ RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS (26/09/01): `InteractionState`
// (defined in `📡️replication/📡️wire/🦀️.rs`) now hand-implements `ToValue`/`FromValue` alongside
// its existing `serde` derive (the composed `BTreeMap<String, DomainSelection/DomainHover/
// SelectionMode>` chain converted too), so `SetInteractionState`/`InteractionConfigMutation` no
// longer need to stay serde-only. `#[serde(transparent)]` has no `#[derive(ToValue, FromValue)]`
// equivalent (see the fan-out playbook's "not supported" list) — hand-written passthrough below.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(transparent)]
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
