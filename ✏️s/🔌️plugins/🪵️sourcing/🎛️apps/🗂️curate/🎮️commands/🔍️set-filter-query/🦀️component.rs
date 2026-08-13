//! 🔍️ 🔍️ Sourcing curate app commands command — `set-filter-query`.

use crate::apps::curate::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::{CurateSnapshot, SortDirection, TableSort};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "filter-query")]
pub struct SetFilterQuery {
    pub value: String,
}

pub fn handle(payload: &SetFilterQuery, _doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
    Ok(Emit::config(vec![SourcingCurateConfigMutation::SetFilterQuery { value: payload.value.clone() }]))
}
