//! 📥️ 📥️ Remodel play app commands command — `add-stream`.

use crate::artifacts::remodel::mutations::create_stream;
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::schema::next_remodel_id;
use crate::artifacts::remodel::{MediaKind, MediaStream, RemodelSnapshot};
use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-stream")]
pub struct AddStream {
    pub name: String,
    pub kind: String,
    pub camera_id: String,
}

pub async fn handle(payload: &AddStream, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    let kind = if payload.kind == "video" { MediaKind::Video } else { MediaKind::ImageSequence };
    let camera_id = if payload.camera_id.is_empty() { None } else { Some(payload.camera_id.clone()) };
    let id = next_remodel_id("stream");
    let stream = MediaStream { id, name: payload.name.clone(), kind, camera_id, sync_offset_ms: 0.0, fps_hint: 30.0, frames: Vec::new(), source: None };
    Ok(Emit::mutations(vec![create_stream(stream)]))
}
