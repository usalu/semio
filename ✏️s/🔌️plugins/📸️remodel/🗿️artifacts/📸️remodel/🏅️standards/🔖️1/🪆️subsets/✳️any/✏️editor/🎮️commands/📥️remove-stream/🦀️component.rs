//! 📥️ 📥️ Remodel play app commands command — `remove-stream`.

use crate::artifacts::remodel::mutations::delete_stream;
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "remove-stream")]
pub struct RemoveStream {
    pub stream_id: String,
}

pub async fn handle(payload: &RemoveStream, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![delete_stream(payload.stream_id.clone())]))
}
