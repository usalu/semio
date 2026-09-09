//! 👁️ 👁️ Remodeling play app commands command — `set-report-table`.

use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use crate::op::RemodelingMutation;
use crate::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "report-table")]
pub struct SetReportTable {
    pub table: String,
}

pub fn handle(payload: &SetReportTable, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    Ok(Emit::config(vec![RemodelingConfigMutation::SetReportTable(crate::editor::remodeling::config::SetReportTable { table: payload.table.clone() })]))
}
