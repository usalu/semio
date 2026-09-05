//! 🐚️ 🐚️ Remodeling play app commands command — `export-qc-report`.

use crate::artifacts::remodeling::op::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;
use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "export-qc-report")]
pub struct ExportQcReport {}

pub async fn handle(_payload: &ExportQcReport, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    match &doc.snapshot.results.qc {
        Some(qc) => Ok(Emit::effect(Effect::DownloadMediaExport { filename: "remodeling-qc-report.ops".into(), mime_type: "text/plain".into(), data: serde_json::to_string_pretty(qc).unwrap_or_default(), encoding: None })),
        None => Ok(Emit::default()),
    }
}
