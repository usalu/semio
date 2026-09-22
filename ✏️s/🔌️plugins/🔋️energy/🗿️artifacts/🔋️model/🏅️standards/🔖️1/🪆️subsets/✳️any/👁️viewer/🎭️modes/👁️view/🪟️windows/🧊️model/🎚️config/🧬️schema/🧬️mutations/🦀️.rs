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
