//! 🧬️ Energy model 3d window-config mutations — one verb, `set-camera`.

use super::{EnergyModelViewerCameraPose, EnergyModelViewerWindowConfig};
#[path = "🎥️set-camera/🦀️.rs"]
mod set_camera;
pub use set_camera::SetCamera;

#[derive(Clone, Copy, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslOps, dsl::Mutations)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "kind", rename_all = "kebab-case")]
#[mutations(snapshot = EnergyModelViewerWindowConfig, diff = EnergyModelViewerWindowConfig, schema = "energy.model3dviewerwindowconfig")]
pub enum EnergyModelViewerWindowConfigMutation {
    #[dsl(key = "set-camera")]
    SetCamera(SetCamera),
}

impl protocol::OpText for EnergyModelViewerWindowConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        dsl::variants_text::print_op(self)
    }
}

impl protocol::OpBinary for EnergyModelViewerWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

//#region 🌉️TestBridge
/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `EnergyModelViewerWindowConfig`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn energy_model_viewer_window_config_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<EnergyModelViewerWindowConfig, EnergyModelViewerWindowConfigMutation>(base_json, mutation_json, after_json)
}
//#endregion 🌉️TestBridge
