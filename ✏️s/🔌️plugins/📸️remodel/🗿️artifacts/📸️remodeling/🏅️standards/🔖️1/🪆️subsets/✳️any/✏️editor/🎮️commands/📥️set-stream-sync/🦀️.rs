//! 📥️ 📥️ Remodeling play app commands command — `set-stream-sync`.

use crate::artifacts::remodeling::mutations::change_stream_sync;
use crate::artifacts::remodeling::op::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;
use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "stream-sync")]
pub struct SetStreamSync {
    pub stream_id: String,
    pub sync_offset_ms: f64,
}

pub async fn handle(payload: &SetStreamSync, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    if !doc.snapshot.streams.iter().any(|stream| stream.id == payload.stream_id) {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![change_stream_sync(payload.stream_id.clone(), payload.sync_offset_ms)]))
}
