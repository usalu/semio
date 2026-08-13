//! 📥️ 📥️ Remodel play app commands command — `import-video-done`.

use crate::apps::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::apps::remodel::engine::{describe_video_probe, images as remodel_image, video as remodel_video, video_codec_to_artifact};
use crate::apps::remodel::{decode_still_image, payload_from_data_url};
use crate::artifacts::remodel::mutations::{add_stream_frame, change_stream_sync, create_asset, create_stream, delete_stream, replace_stream_source};
use crate::artifacts::remodel::schema::{next_remodel_id, video_codec_from_label};
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{FrameRef, ImageAsset, MediaKind, MediaStream, RemodelSnapshot, VideoSource};
use base64::Engine as _;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault, HostEffect};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

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
pub fn handle(payload: &ImportVideoDone, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    let scene = doc.snapshot;
    let Some(stream_id) = scene.streams.last().map(|stream| stream.id.clone()) else { return Ok(Emit::default()) };
    let codec_value = video_codec_from_label(&payload.codec);
    let source = VideoSource { name: payload.name.clone(), container: "unknown".into(), codec: codec_value, duration_ms: payload.duration_ms, frame_count: payload.frame_count, width: payload.width, height: payload.height };
    Ok(Emit::amend(vec![replace_stream_source(stream_id.clone(), Some(source))], format!("remodel-import:{stream_id}")))
}
