//! 🔍️ 🔍️ Sourcing curate app commands command — `sort-table`.

use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::{CurateSnapshot, SortDirection, TableSort};
use crate::editor::sourcing::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "sort-table")]
pub struct SortTable {
    pub column_id: String,
    pub direction: String,
}

pub fn handle(payload: &SortTable, _doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
    let sort = TableSort { column_id: payload.column_id.clone(), direction: if payload.direction == "desc" { SortDirection::Desc } else { SortDirection::Asc } };
    Ok(Emit::config(vec![SourcingCurateConfigMutation::SetSort { sort: Some(sort) }]))
}
