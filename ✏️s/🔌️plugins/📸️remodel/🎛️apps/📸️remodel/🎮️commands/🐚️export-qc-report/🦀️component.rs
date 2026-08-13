//! 🐚️ 🐚️ Remodel play app commands command — `export-qc-report`.

use crate::apps::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault, HostEffect};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "export-qc-report")]
pub struct ExportQcReport {}

pub fn handle(_payload: &ExportQcReport, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    match &doc.snapshot.results.qc {
        Some(qc) => Ok(Emit::effect(HostEffect::DownloadMediaExport { filename: "remodel-qc-report.ops".into(), mime_type: "text/plain".into(), data: serde_json::to_string_pretty(qc).unwrap_or_default(), encoding: None })),
        None => Ok(Emit::default()),
    }
}
