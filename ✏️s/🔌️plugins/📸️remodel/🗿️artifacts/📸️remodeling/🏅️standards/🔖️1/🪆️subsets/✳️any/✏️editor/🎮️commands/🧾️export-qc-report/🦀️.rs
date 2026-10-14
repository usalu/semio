//! 🐚️ 🐚️ Remodeling play app commands command — `export-qc-report`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "export-qc-report")]
pub struct ExportQcReport {}

pub fn handle(_payload: &ExportQcReport, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    let qc = doc.snapshot.results.qc.as_ref().ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("remodeling.qc-report.missing"), "Run the quality check before exporting its report."))?;
    let data = serde_json::to_string_pretty(qc).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("remodeling.qc-report.encode"), format!("The quality report could not be written: {error}")))?;
    Ok(Emit::effect(Effect::DownloadMediaExport { filename: "remodeling-qc-report.json".into(), mime_type: "application/json".into(), data, encoding: None }))
}
