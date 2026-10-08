//! 🕹️ The framework interaction mutation aggregate: one direct whole-state leaf over `protocol::InteractionState`.

use protocol::InteractionState;

//#region 🧬️Diff
#[path = "../🔺️diff/🦀️.rs"]
pub(crate) mod diff;
pub use diff::{DomainEdit, InteractionStateDiff};
//#endregion 🧬️Diff

//#region 🧬️Leaves
#[path = "🔁️set-state/🦀️.rs"]
pub(crate) mod set_state;
pub use set_state::SetInteractionState;
//#endregion 🧬️Leaves

//#region 🧬️Aggregate
/// 🕹️ Framework interaction mutation aggregate; its direct leaf owns metadata and ordinary semantics. Externally tagged:
/// `{"setInteractionState": <InteractionState>}` (`🔣️.json` beside this file).
/// 🧊️ Whole-state cold codecs/evaluation do not certify a retained restore or reserved interaction route.
#[derive(Clone, Debug, PartialEq, dsl::Mutations, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = InteractionState, diff = InteractionStateDiff, schema = "framework.interaction")]
pub enum InteractionConfigMutation {
    SetInteractionState(SetInteractionState),
}
//#endregion 🧬️Aggregate
