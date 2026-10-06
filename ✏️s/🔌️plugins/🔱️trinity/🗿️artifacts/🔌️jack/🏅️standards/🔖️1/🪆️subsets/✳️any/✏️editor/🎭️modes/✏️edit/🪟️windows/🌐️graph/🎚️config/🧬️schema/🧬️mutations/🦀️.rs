//! 🧬️ Mutations of one Jack graph-window configuration.

use super::JackGraphWindowConfig;
#[path = "🎥️set-camera/🦀️.rs"]
mod set_camera;
pub use set_camera::SetCamera;
#[path = "🔍️set-lod-mode/🦀️.rs"]
mod set_lod_mode;
pub use set_lod_mode::SetLodMode;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[value(tag = "kind", rename_all = "kebab-case")]
#[mutations(snapshot = JackGraphWindowConfig, diff = JackGraphWindowConfig, schema = "trinity.jackgraphwindowcfg")]
pub enum JackGraphWindowConfigMutation {
    #[dsl(key = "set-camera")]
    SetCamera(SetCamera),
    #[dsl(key = "set-lod-mode")]
    SetLodMode(SetLodMode),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
