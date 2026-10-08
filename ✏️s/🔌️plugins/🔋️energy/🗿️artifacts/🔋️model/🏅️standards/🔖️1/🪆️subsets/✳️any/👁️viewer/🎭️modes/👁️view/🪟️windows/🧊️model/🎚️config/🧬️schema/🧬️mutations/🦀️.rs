//! 🧬️ Energy model 3d window-config mutations — one verb, `set-camera`.

use super::{EnergyModelViewerCameraPose, EnergyModelViewerWindowConfig, EnergyModelViewerWindowConfigDiff};
#[path = "🎥️set-camera/🦀️.rs"]
mod set_camera;
pub use set_camera::SetCamera;

#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "kind", rename_all = "kebab-case")]
#[mutations(snapshot = EnergyModelViewerWindowConfig, diff = EnergyModelViewerWindowConfigDiff, schema = "energy.model3dviewerwindowconfig")]
pub enum EnergyModelViewerWindowConfigMutation {
    #[dsl(key = "set-camera")]
    SetCamera(SetCamera),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
