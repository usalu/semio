//! 🗣️ 🗣️ Sourcing curate app commands command — `set-locale`.

use crate::editor::sourcing::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::CurateSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "locale")]
pub struct SetLocale {
    pub value: String,
}

pub async fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
    Ok(Emit::config(vec![SourcingCurateConfigMutation::SetLocale { value: payload.value.clone() }]))
}
