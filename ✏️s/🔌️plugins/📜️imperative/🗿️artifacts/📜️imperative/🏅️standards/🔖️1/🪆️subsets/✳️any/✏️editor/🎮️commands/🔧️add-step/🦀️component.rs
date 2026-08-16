//! 🔧️ 🔧️ Imperative play app commands command — `add-step`.

use crate::editor::imperative::config::{ImperativeConfig, ImperativeConfigMutation};
use crate::artifacts::imperative::dsl::ValueDsl;
use crate::artifacts::imperative::mutations::{create_step, delete_step, edit_step_params, reorder_steps, ImperativeMutation};
use crate::artifacts::imperative::{Dictionary, ImperativeSnapshot, PathRef, Step};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Helpers
/// 🆔️ Allocates a fresh `step-N` id one past the highest suffix used anywhere in the document
/// (including nested `control.*` bodies), deterministically from pre-state — no mutable counter.
/// Reads through the `flow` working scene (`ImperativeSnapshot` no longer carries `path` inline —
/// ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`).
fn next_step_id(document: &ImperativeSnapshot) -> String {
    fn max_suffix(steps: &[Step]) -> u64 {
        steps.iter().fold(0, |acc, step| {
            let own = step.id.strip_prefix("step-").and_then(|rest| rest.parse::<u64>().ok()).unwrap_or(0);
            let nested = step.bodies.values().map(|path| max_suffix(&path.steps)).max().unwrap_or(0);
            acc.max(own).max(nested)
        })
    }
    let path = crate::artifacts::imperative::imperative_working_scene(document).path;
    format!("step-{}", max_suffix(&path.steps) + 1)
}

/// 📍️ Resolves `owner`/`slot` command fields into a [`PathRef`] so nested control-step bodies (e.g.
/// `control.if` then/else) resolve correctly; falls back to the root path unless both are present and
/// `owner` names a real top-level step, avoiding an unresolvable or unknown reference that would
/// otherwise address nothing.
fn path_ref_from(owner: Option<&str>, slot: Option<&str>, document: &ImperativeSnapshot) -> PathRef {
    let path = crate::artifacts::imperative::imperative_working_scene(document).path;
    match (owner, slot) {
        (Some(owner), Some(slot)) if path.steps.iter().any(|step| step.id == owner) => PathRef { owner: Some(owner.to_string()), slot: Some(slot.to_string()) },
        _ => PathRef::default(),
    }
}

/// 🔎️ Resolves the step list a `PathRef` addresses — the root path, or a nested `control.*` step's slot
/// (an unmaterialized slot reads as empty).
fn steps_at(document: &ImperativeSnapshot, path_ref: &PathRef) -> Vec<Step> {
    let path = crate::artifacts::imperative::imperative_working_scene(document).path;
    match (&path_ref.owner, &path_ref.slot) {
        (Some(owner), Some(slot)) => path.steps.iter().find(|step| &step.id == owner).and_then(|step| step.bodies.get(slot)).map(|body| body.steps.clone()).unwrap_or_default(),
        _ => path.steps,
    }
}

/// 🔎️ True when the step `id` exists in the list the `owner`/`slot` command fields address — the
/// pre-state guard the operation arms share so a stale id never emits a no-operation edit into history.
fn resolve_contains(document: &ImperativeSnapshot, owner: Option<&str>, slot: Option<&str>, id: &str) -> bool {
    let path_ref = path_ref_from(owner, slot, document);
    steps_at(document, &path_ref).iter().any(|step| step.id == id)
}
//#endregion 🔖️Helpers

//#region 🔖️AddStep
//#endregion 🔖️AddStep

//#region 🔖️AddStepAt
//#endregion 🔖️AddStepAt

//#region 🔖️RemoveStep
//#endregion 🔖️RemoveStep

//#region 🔖️RemoveStepAt
//#endregion 🔖️RemoveStepAt

//#region 🔖️MoveStep
//#endregion 🔖️MoveStep

//#region 🔖️MoveStepAt
//#endregion 🔖️MoveStepAt

//#region 🔖️SetStepParams
//#endregion 🔖️SetStepParams

//#region 🔖️SetStepParamsAt
//#endregion 🔖️SetStepParamsAt

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-step")]
pub struct AddStep {
    pub kind: String,
    pub index: Option<usize>,
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: no auto-select of the new step
// anymore — selection is the framework-owned `steps` interaction domain, reachable only through the
// injected `interactionSelect` verb, not an ordinary command's `config_mutations`.
pub fn handle(payload: &AddStep, doc: &ArtifactView<'_, ImperativeSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeMutation, ImperativeConfigMutation>, Fault> {
    let document = doc.snapshot;
    let id = next_step_id(document);
    let step = Step { id, kind: payload.kind.clone(), params: Dictionary::new(), bodies: BTreeMap::new() };
    // 🪆️ `create-step` is append-only (no index field, matching `apply_steps_delta`'s `added`
    // handling, which already ignored the old `CollectionMutation::Add`'s index the same way).
    Ok(Emit::mutations(vec![create_step(PathRef::default(), step)]))
}
