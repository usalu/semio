//! 🧬️ Energy model 3d window-config mutations — one verb, `set-camera`.

use super::{EnergyModelCameraPose, EnergyModelWindowConfig};
#[path = "🎥️set-camera/🦀️.rs"]
mod set_camera;
pub use set_camera::SetCamera;

#[derive(Clone, Copy, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslOps, dsl::Mutations)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "kind", rename_all = "kebab-case")]
#[mutations(snapshot = EnergyModelWindowConfig, diff = EnergyModelWindowConfig, schema = "energy.model3dwindowconfig")]
pub enum EnergyModelWindowConfigMutation {
    #[dsl(key = "set-camera")]
    SetCamera(SetCamera),
}

impl protocol::OpText for EnergyModelWindowConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        dsl::variants_text::print_op(self)
    }
}

impl protocol::OpBinary for EnergyModelWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

//#region 🌉️TestBridge
/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `EnergyModelWindowConfig`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn energy_model_window_config_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<EnergyModelWindowConfig, EnergyModelWindowConfigMutation>(base_json, mutation_json, after_json)
}
//#endregion 🌉️TestBridge
