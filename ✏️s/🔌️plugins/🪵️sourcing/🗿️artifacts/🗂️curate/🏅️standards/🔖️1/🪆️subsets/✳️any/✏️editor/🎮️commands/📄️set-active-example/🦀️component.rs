//! 📄️ 📄️ Sourcing curate app commands command — `set-active-example`.

use crate::editor::sourcing::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use crate::editor::sourcing::{reset_document_effect, EMPTY_EXAMPLE_ID};
use crate::artifacts::curate::schema::{available_modules, default_document, empty_document};
use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::CurateSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, CurateSnapshot>, _cfg: &ConfigView<'_, SourcingCurateConfig>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation>, Fault> {
    let next = if payload.example_id.is_empty() || payload.example_id == EMPTY_EXAMPLE_ID { empty_document() } else { default_document() };
    Ok(Emit { effects: vec![reset_document_effect(&next)], ..Default::default() })
}
