//! ⚙️ ⚙️ Remodeling play app commands command — `set-ingest-params`.

use crate::mutations::update_ingest_params;
use crate::op::RemodelingMutation;
use crate::{IngestParams, RemodelingSnapshot};
use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "ingest-params")]
pub struct SetIngestParams {
    pub frame_sample_stride: u32,
    pub max_frames: u32,
    pub downscale_long_edge_px: u32,
    pub min_sharpness: f32,
}

pub fn handle(payload: &SetIngestParams, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![update_ingest_params(IngestParams { frame_sample_stride: payload.frame_sample_stride, max_frames: payload.max_frames, downscale_long_edge_px: payload.downscale_long_edge_px, min_sharpness: payload.min_sharpness })]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
