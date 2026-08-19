//! 🗣️ 🗣️ Raster play app commands command — `set-locale`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "locale")]
pub struct SetLocale {
    pub value: String,
}

pub async fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    Ok(Emit::config(vec![RasterConfigMutation::SetLocale { value: payload.value.clone() }]))
}
