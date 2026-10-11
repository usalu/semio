//! 🔍️ 🔍️ Sourcing curation app commands command — `set-filter-typology`.

use crate::standards::v1::subsets::any::schema::mutations::SourcingMutation;
use crate::CurationSnapshot;
use crate::editor::sourcing::config::{SourcingCurationConfig, SourcingCurationConfigMutation, SetFilterTypologyEdit};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "filter-typology")]
pub struct SetFilterTypology {
    pub path: String,
}

pub fn handle(payload: &SetFilterTypology, _doc: &ArtifactView<'_, CurationSnapshot>, _cfg: &ConfigView<'_, SourcingCurationConfig>) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation>, Fault> {
    let path = if payload.path.is_empty() { Vec::new() } else { payload.path.split('/').map(String::from).collect() };
    Ok(Emit::config(vec![SourcingCurationConfigMutation::SetFilterTypology(SetFilterTypologyEdit { path })]))
}
