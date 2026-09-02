//! 🗣️ 🗣️ Sourcing curation app commands command — `set-locale`.

use crate::artifacts::curation::op::SourcingMutation;
use crate::artifacts::curation::CurationSnapshot;
use crate::editor::sourcing::config::{SourcingCurationConfig, SourcingCurationConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "locale")]
pub struct SetLocale {
    pub value: String,
}

pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, CurationSnapshot>, _cfg: &ConfigView<'_, SourcingCurationConfig>) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation>, Fault> {
    Ok(Emit::config(vec![SourcingCurationConfigMutation::SetLocale { value: payload.value.clone() }]))
}
