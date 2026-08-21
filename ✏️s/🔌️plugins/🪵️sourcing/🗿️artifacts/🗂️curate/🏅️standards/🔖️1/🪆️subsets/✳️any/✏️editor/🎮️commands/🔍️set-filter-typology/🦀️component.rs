//! 🔍️ 🔍️ Sourcing curate app commands command — `set-filter-typology`.

use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::CurateSnapshot;
use crate::editor::sourcing::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "filter-typology")]
pub struct SetFilterTypology {
    pub path: String,
}

pub async fn handle(payload: &SetFilterTypology, _doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
    let path = if payload.path.is_empty() { Vec::new() } else { payload.path.split('/').map(String::from).collect() };
    Ok(Emit::config(vec![SourcingCurateConfigMutation::SetFilterTypology { path }]))
}
