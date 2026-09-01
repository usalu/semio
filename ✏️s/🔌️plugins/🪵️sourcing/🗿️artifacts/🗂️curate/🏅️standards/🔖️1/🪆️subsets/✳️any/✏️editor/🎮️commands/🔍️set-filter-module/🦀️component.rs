//! 🔍️ 🔍️ Sourcing curate app commands command — `set-filter-module`.

use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::CurateSnapshot;
use crate::editor::sourcing::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "filter-module")]
pub struct SetFilterModule {
    pub module_id: String,
    pub enabled: bool,
}

pub fn handle(payload: &SetFilterModule, _doc: &ArtifactView<'_, CurateSnapshot>, cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
    let mut module_ids = cfg.snapshot.filters.module_ids.clone();
    if payload.enabled {
        if !module_ids.iter().any(|id| id == &payload.module_id) {
            module_ids.push(payload.module_id.clone());
        }
    } else {
        module_ids.retain(|id| id != &payload.module_id);
    }
    Ok(Emit::config(vec![SourcingCurateConfigMutation::SetFilterModules { module_ids }]))
}
