//! 🧩️ 🧩️ Sourcing curate app commands command — `set-contributions`.

use crate::artifacts::curate::{op::SourcingMutation, CurateSnapshot};
use crate::editor::sourcing::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "contributions")]
pub struct SetContributions {
    pub json: String,
}

pub fn handle(payload: &SetContributions, _doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
    Ok(Emit::config(vec![SourcingCurateConfigMutation::SetContributions { json: payload.json.clone() }]))
}
