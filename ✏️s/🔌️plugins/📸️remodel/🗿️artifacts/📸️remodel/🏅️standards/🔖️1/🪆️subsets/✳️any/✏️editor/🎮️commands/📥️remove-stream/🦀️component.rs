//! 📥️ 📥️ Remodel play app commands command — `remove-stream`.

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
#[dsl(keyword = "remove-stream")]
pub struct RemoveStream {
    pub stream_id: String,
}

pub fn handle(payload: &RemoveStream, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![delete_stream(payload.stream_id.clone())]))
}
