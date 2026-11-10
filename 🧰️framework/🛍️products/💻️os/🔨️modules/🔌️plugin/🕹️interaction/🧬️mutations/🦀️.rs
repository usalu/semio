//! 🕹️ The framework interaction mutation aggregate: one direct whole-state leaf over `protocol::InteractionState`.

use protocol::InteractionState;

//#region 🧬️Diff
#[path = "../🔺️diff/🦀️.rs"]
mod diff;
pub use diff::{DomainEdit, InteractionStateDiff};
//#endregion 🧬️Diff

//#region 🧬️Leaves
#[path = "🔁️set-state/🦀️.rs"]
mod set_state;
pub use set_state::SetInteractionState;
//#endregion 🧬️Leaves

//#region 🧬️Aggregate
/// 🕹️ Framework interaction mutation aggregate; its direct leaf owns metadata and ordinary semantics. Externally tagged:
/// `{"setInteractionState": <InteractionState>}` (`🔣️.json` beside this file).
/// 🧊️ Whole-state cold codecs/evaluation do not certify a retained restore or reserved interaction route.
#[derive(semio_framework_value::RetireOwned,Clone, Debug, PartialEq, dsl::Mutations, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value_derive::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = InteractionState, diff = InteractionStateDiff, schema = "framework.interaction", reference = InteractionState, reference_diff = InteractionStateDiff)]
pub enum InteractionConfigMutation {
    SetInteractionState(SetInteractionState),
}
impl InteractionConfigMutation {
    pub fn set_state(state: InteractionState) -> Self {
        Self::SetInteractionState(SetInteractionState { state })
    }
}

impl ::protocol::OpText for InteractionConfigMutation {
    fn print_op(&self) -> String {
        let InteractionConfigMutation::SetInteractionState(state) = self;
        format!("set-interaction-state {}", semio_framework_pack_json::to_json_string(state))
    }
    fn parse_op(line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
        let body = line
            .strip_prefix("set-interaction-state ")
            .ok_or_else(|| ::semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown interaction config op '{line}'"), ::semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        let state: InteractionState = semio_framework_pack_json::from_json_str(body, semio_framework_pack_json::JsonMemberPolicy::Reject)
            .map_err(|error| ::semio_framework_diagnostic::TextError::from_value_error(error, ::semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Ok(InteractionConfigMutation::set_state(state))
    }
}

impl ::protocol::OpBinary for InteractionConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        Ok(semio_os_pack::pack_rt::encode_pack_value(&semio_framework_value::ToValue::to_value(self)))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        let malformed = |detail: String| ::protocol::ProtocolError::Malformed { what: "interaction-config-mutation", offset: 0, detail };
        let value = semio_os_pack::pack_rt::decode_pack_value(bytes).map_err(|error| malformed(error.to_string()))?;
        semio_framework_value::FromValue::from_value(semio_os_pack::pack_rt::renormalize_whole_number_floats(value)).map_err(|error| malformed(error.to_string()))
    }
}
semio_os_dsl::original_pack_value_operation_receiving!(InteractionConfigMutation);
//#endregion 🧬️Aggregate
