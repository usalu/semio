//! 🧬️ DAG play app command — `set-active-example`.
//!
//! 🧬️ Whole-document replace has no in-history mutation (the whole-snapshot variant is banned
//! outright — `📓️taxonomy.md`'s forbidden vocabulary), so loading a named example emits
//! `crate::editor::dag::reset_dag_document_effect` (an `Effect::LoadDocument`, outside undo history)
//! rather than an `artifact_mutations` entry. The react/wgpu shells dispatch this verb at boot for
//! every app whose subset registers examples; dag declared no such verb at all, so every boot and
//! every navbar pick was answered `undeclared-action` and the picked example never reached the canvas.

use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use crate::op::DagMutation;
use crate::DagSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
/// 🧬️ The subset registers exactly one example (`crate::examples::demo`, `ID = "demo"`), whose asset
/// IS this app's canonical default document; every other id loads the empty graph.
pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let next = if payload.example_id.as_str() == crate::examples::demo::ID { crate::default_snapshot() } else { crate::empty_snapshot() };
    Ok(Emit { effects: vec![crate::editor::dag::reset_dag_document_effect(&next)], ..Default::default() })
}
//#endregion 🔖️Handler
