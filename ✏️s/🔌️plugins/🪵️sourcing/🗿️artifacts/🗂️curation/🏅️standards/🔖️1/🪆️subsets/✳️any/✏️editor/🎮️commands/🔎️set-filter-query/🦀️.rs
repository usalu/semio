//! 🔍️ 🔍️ Sourcing curation app commands command — `set-filter-query`.

use crate::standards::v1::subsets::any::schema::mutations::SourcingMutation;
use crate::CurationSnapshot;
use crate::editor::sourcing::config::{SourcingCurationConfig, SourcingCurationConfigMutation, SetFilterQueryEdit};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "filter-query")]
pub struct SetFilterQuery {
    pub value: String,
}

pub fn handle(payload: &SetFilterQuery, _doc: &ArtifactView<'_, CurationSnapshot>, _cfg: &ConfigView<'_, SourcingCurationConfig>) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation>, Fault> {
    Ok(Emit::config(vec![SourcingCurationConfigMutation::SetFilterQuery(SetFilterQueryEdit { value: payload.value.clone() })]))
}
