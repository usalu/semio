//! 🗂️ 🗂️ Sourcing curate app commands command — `world-select`.

use crate::apps::curate::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::CurateSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "world-select")]
pub struct WorldSelect {
    pub ids: Vec<String>,
}

/// 🖱️ `worldSelect` keeps only the LAST id as the single selection (matches the pool/curated tables'
/// single-select semantics — sourcing has no multi-select surface).
pub fn handle(payload: &WorldSelect, _doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
    match payload.ids.last() {
        Some(id) => Ok(Emit::config(vec![SourcingCurateConfigMutation::SetSelectedObject { object_id: Some(id.clone()) }])),
        None => Ok(Emit::default()),
    }
}
