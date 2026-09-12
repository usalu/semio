//! 📥️ 📥️ Remodeling play app commands command — `add-stream`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::mutations::create_stream;
use crate::op::RemodelingMutation;
use crate::schema::next_remodeling_id;
use crate::{MediaKind, MediaStream, RemodelingSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-stream")]
pub struct AddStream {
    pub name: String,
    pub kind: String,
    pub camera_id: String,
}

pub fn handle(payload: &AddStream, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    let kind = if payload.kind == "video" { MediaKind::Video } else { MediaKind::ImageSequence };
    let camera_id = if payload.camera_id.is_empty() { None } else { Some(payload.camera_id.clone()) };
    let id = next_remodeling_id("stream");
    let stream = MediaStream { id, name: payload.name.clone(), kind, camera_id, sync_offset_ms: 0.0, fps_hint: 30.0, frames: Vec::new(), source: None };
    Ok(Emit::mutations(vec![create_stream(stream)]))
}
