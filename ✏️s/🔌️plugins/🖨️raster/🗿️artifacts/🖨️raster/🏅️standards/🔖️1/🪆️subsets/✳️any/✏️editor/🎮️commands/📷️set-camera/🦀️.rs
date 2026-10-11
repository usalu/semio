//! 🎥️ 🎥️ Raster play app commands command — `set-camera`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation, SetCameraEdit};
use crate::op::RasterMutation;
use crate::{RasterCamera, RasterSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "camera")]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: RasterCamera,
}

pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    Ok(Emit::config(vec![RasterConfigMutation::SetCamera(SetCameraEdit { camera: payload.camera.clone() })]))
}
