//! 🔍️ 🔍️ Sourcing curation app commands command — `set-filter-typology`.

use crate::artifacts::curation::op::SourcingMutation;
use crate::artifacts::curation::CurationSnapshot;
use crate::editor::sourcing::config::{SourcingCurationConfig, SourcingCurationConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "filter-typology")]
pub struct SetFilterTypology {
    pub path: String,
}

pub fn handle(payload: &SetFilterTypology, _doc: &ArtifactView<'_, CurationSnapshot>, _cfg: &ConfigView<'_, SourcingCurationConfig>) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation>, Fault> {
    let path = if payload.path.is_empty() { Vec::new() } else { payload.path.split('/').map(String::from).collect() };
    Ok(Emit::config(vec![SourcingCurationConfigMutation::SetFilterTypology { path }]))
}
