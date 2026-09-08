//! 🔍️ 🔍️ Sourcing curation app commands command — `set-filter-query`.

use crate::op::SourcingMutation;
use crate::CurationSnapshot;
use crate::editor::sourcing::config::{SourcingCurationConfig, SourcingCurationConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "filter-query")]
pub struct SetFilterQuery {
    pub value: String,
}

pub fn handle(payload: &SetFilterQuery, _doc: &ArtifactView<'_, CurationSnapshot>, _cfg: &ConfigView<'_, SourcingCurationConfig>) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation>, Fault> {
    Ok(Emit::config(vec![SourcingCurationConfigMutation::SetFilterQuery { value: payload.value.clone() }]))
}
