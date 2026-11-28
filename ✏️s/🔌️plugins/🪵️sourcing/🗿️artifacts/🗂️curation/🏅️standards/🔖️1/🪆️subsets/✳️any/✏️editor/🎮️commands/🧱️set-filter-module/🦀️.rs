//! 🔍️ 🔍️ Sourcing curation app commands command — `set-filter-module`.

use crate::standards::v1::subsets::any::schema::mutations::SourcingMutation;
use crate::CurationSnapshot;
use crate::editor::sourcing::config::{SourcingCurationConfig, SourcingCurationConfigMutation, ToggleFilterModuleEdit};
use semio_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "filter-module")]
pub struct SetFilterModule {
    pub module_id: String,
    pub enabled: bool,
}

pub fn handle(payload: &SetFilterModule, _doc: &ArtifactView<'_, CurationSnapshot>, _cfg: &ConfigView<'_, SourcingCurationConfig>) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation>, Fault> {
    Ok(Emit::config(vec![SourcingCurationConfigMutation::ToggleFilterModule(ToggleFilterModuleEdit { module_id: payload.module_id.clone(), enabled: payload.enabled })]))
}
