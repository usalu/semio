//! 📥️ 📥️ Remodel play app commands command — `add-stream`.

use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::editor::remodel::engine::{describe_video_probe, images as remodel_image, video as remodel_video, video_codec_to_artifact};
use crate::editor::remodel::{decode_still_image, payload_from_data_url};
use crate::artifacts::remodel::mutations::{add_stream_frame, change_stream_sync, create_asset, create_stream, delete_stream, replace_stream_source};
use crate::artifacts::remodel::schema::{next_remodel_id, video_codec_from_label};
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::{FrameRef, ImageAsset, MediaKind, MediaStream, RemodelSnapshot, VideoSource};
use base64::Engine as _;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault, HostEffect};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-stream")]
pub struct AddStream {
    pub name: String,
    pub kind: String,
    pub camera_id: String,
}

pub fn handle(payload: &AddStream, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    let kind = if payload.kind == "video" { MediaKind::Video } else { MediaKind::ImageSequence };
    let camera_id = if payload.camera_id.is_empty() { None } else { Some(payload.camera_id.clone()) };
    let id = next_remodel_id("stream");
    let stream = MediaStream { id, name: payload.name.clone(), kind, camera_id, sync_offset_ms: 0.0, fps_hint: 30.0, frames: Vec::new(), source: None };
    Ok(Emit::mutations(vec![create_stream(stream)]))
}
