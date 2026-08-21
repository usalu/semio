//! 📥️ 📥️ Remodel play app commands command — `set-stream-sync`.

use crate::artifacts::remodel::mutations::change_stream_sync;
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "stream-sync")]
pub struct SetStreamSync {
    pub stream_id: String,
    pub sync_offset_ms: f64,
}

pub async fn handle(payload: &SetStreamSync, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    if !doc.snapshot.streams.iter().any(|stream| stream.id == payload.stream_id) {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![change_stream_sync(payload.stream_id.clone(), payload.sync_offset_ms)]))
}
