//! 🔍️ 🔍️ Sourcing curate app commands command — `set-filter-query`.

use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::CurateSnapshot;
use crate::editor::sourcing::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "filter-query")]
pub struct SetFilterQuery {
    pub value: String,
}

pub fn handle(payload: &SetFilterQuery, _doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
    Ok(Emit::config(vec![SourcingCurateConfigMutation::SetFilterQuery { value: payload.value.clone() }]))
}
