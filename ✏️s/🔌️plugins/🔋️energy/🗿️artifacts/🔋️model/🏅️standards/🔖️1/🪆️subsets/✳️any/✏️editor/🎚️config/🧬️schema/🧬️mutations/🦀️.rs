//! 🧬️ Semantic energy model config mutation vocabulary and codecs.

use super::EnergyModelConfig;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[path = "⏱️change-simulation/🦀️.rs"]
mod change_simulation_settings;
pub use change_simulation_settings::ChangeSimulationSettings;

#[path = "🎨️change-result/🦀️.rs"]
mod change_result_field;
pub use change_result_field::ChangeResultField;

/// 🧬️ `EnergyModelEditor::ConfigMutation`.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", content = "payload", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", content = "payload", rename_all = "camelCase"))]
#[mutations(snapshot = EnergyModelConfig, diff = EnergyModelConfig, schema = "energy.model.config")]
pub enum EnergyModelConfigMutation {
    ChangeSimulationSettings(ChangeSimulationSettings),
    ChangeResultField(ChangeResultField),
}

impl protocol::OpText for EnergyModelConfigMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_dsl_record::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        semio_framework_dsl_record::variants_text::print_op(self)
    }
}

impl protocol::OpBinary for EnergyModelConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

//#region 🌉️TestBridge
/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `EnergyModelConfig`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn energy_model_config_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<EnergyModelConfig, EnergyModelConfigMutation>(base_json, mutation_json, after_json)
}
//#endregion 🌉️TestBridge
