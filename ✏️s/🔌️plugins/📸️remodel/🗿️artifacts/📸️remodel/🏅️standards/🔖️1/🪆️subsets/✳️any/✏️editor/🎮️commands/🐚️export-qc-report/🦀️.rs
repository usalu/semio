//! 🐚️ 🐚️ Remodel play app commands command — `export-qc-report`.

use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "export-qc-report")]
pub struct ExportQcReport {}

pub async fn handle(_payload: &ExportQcReport, doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    match &doc.snapshot.results.qc {
        Some(qc) => Ok(Emit::effect(Effect::DownloadMediaExport { filename: "remodel-qc-report.ops".into(), mime_type: "text/plain".into(), data: serde_json::to_string_pretty(qc).unwrap_or_default(), encoding: None })),
        None => Ok(Emit::default()),
    }
}
