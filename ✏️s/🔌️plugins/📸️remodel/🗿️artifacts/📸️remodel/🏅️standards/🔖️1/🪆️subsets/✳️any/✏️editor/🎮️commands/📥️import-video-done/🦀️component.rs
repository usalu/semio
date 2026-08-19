//! 📥️ 📥️ Remodel play app commands command — `import-video-done`.

use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::mutations::replace_stream_source;
use crate::artifacts::remodel::schema::video_codec_from_label;
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{RemodelSnapshot, VideoSource};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "import-video-done")]
pub struct ImportVideoDone {
    pub name: String,
    pub duration_ms: f64,
    pub frame_count: u32,
    pub width: u32,
    pub height: u32,
    pub codec: String,
}

/// ✅️ Host-decoded video import finished: writes `VideoSource` provenance on the just-imported
/// stream (`scene.streams.last()` — the stream this batch's ticks just built). Uses the SAME
/// coalesce key as every preceding `ImportVideoFramePayload` tick, so the whole import (every
/// accepted frame plus this final metadata write) collapses into one undo step.
pub async fn handle(payload: &ImportVideoDone, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    let scene = doc.snapshot;
    let Some(stream_id) = scene.streams.last().map(|stream| stream.id.clone()) else { return Ok(Emit::default()) };
    let codec_value = video_codec_from_label(&payload.codec);
    let source = VideoSource { name: payload.name.clone(), container: "unknown".into(), codec: codec_value, duration_ms: payload.duration_ms, frame_count: payload.frame_count, width: payload.width, height: payload.height };
    Ok(Emit::amend(vec![replace_stream_source(stream_id.clone(), Some(source))], format!("remodel-import:{stream_id}")))
}
