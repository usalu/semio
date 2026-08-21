//! 🎯️ 🎯️ Remodel play app commands command — `remove-gcp`.

use crate::artifacts::remodel::mutations::delete_gcp;
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "remove-gcp")]
pub struct RemoveGcp {
    pub gcp_id: String,
}

pub async fn handle(payload: &RemoveGcp, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![delete_gcp(payload.gcp_id.clone())]))
}
