//! 🔍️ 🔍️ Sourcing curate app commands command — `sort-table`.

use crate::editor::sourcing::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::{CurateSnapshot, SortDirection, TableSort};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "sort-table")]
pub struct SortTable {
    pub column_id: String,
    pub direction: String,
}

pub async fn handle(payload: &SortTable, _doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
    let sort = TableSort { column_id: payload.column_id.clone(), direction: if payload.direction == "desc" { SortDirection::Desc } else { SortDirection::Asc } };
    Ok(Emit::config(vec![SourcingCurateConfigMutation::SetSort { sort: Some(sort) }]))
}
