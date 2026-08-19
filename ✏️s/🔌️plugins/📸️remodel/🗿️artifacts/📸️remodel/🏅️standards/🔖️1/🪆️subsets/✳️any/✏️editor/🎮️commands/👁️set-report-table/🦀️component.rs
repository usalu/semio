//! 👁️ 👁️ Remodel play app commands command — `set-report-table`.

use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "report-table")]
pub struct SetReportTable {
    pub table: String,
}

pub async fn handle(payload: &SetReportTable, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::config(vec![RemodelConfigMutation::SetReportTable { table: payload.table.clone() }]))
}
