//! 🔍️ 🔍️ Sourcing curation app commands command — `set-filter-module`.

use crate::artifacts::curation::op::SourcingMutation;
use crate::artifacts::curation::CurationSnapshot;
use crate::editor::sourcing::config::{SourcingCurationConfig, SourcingCurationConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "filter-module")]
pub struct SetFilterModule {
    pub module_id: String,
    pub enabled: bool,
}

pub fn handle(payload: &SetFilterModule, _doc: &ArtifactView<'_, CurationSnapshot>, cfg: &ConfigView<'_, SourcingCurationConfig>) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation>, Fault> {
    let mut module_ids = cfg.snapshot.filters.module_ids.clone();
    if payload.enabled {
        if !module_ids.iter().any(|id| id == &payload.module_id) {
            module_ids.push(payload.module_id.clone());
        }
    } else {
        module_ids.retain(|id| id != &payload.module_id);
    }
    Ok(Emit::config(vec![SourcingCurationConfigMutation::SetFilterModules { module_ids }]))
}
