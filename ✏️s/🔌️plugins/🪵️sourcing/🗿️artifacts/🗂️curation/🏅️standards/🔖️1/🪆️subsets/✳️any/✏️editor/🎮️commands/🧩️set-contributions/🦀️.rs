//! 🧩️ 🧩️ Sourcing curation app commands command — `set-contributions`.

use crate::artifacts::curation::{op::SourcingMutation, CurationSnapshot};
use crate::editor::sourcing::config::{SourcingCurationConfig, SourcingCurationConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "contributions")]
pub struct SetContributions {
    pub json: String,
}

pub fn handle(payload: &SetContributions, _doc: &ArtifactView<'_, CurationSnapshot>, _cfg: &ConfigView<'_, SourcingCurationConfig>) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation>, Fault> {
    Ok(Emit::config(vec![SourcingCurationConfigMutation::SetContributions { json: payload.json.clone() }]))
}
