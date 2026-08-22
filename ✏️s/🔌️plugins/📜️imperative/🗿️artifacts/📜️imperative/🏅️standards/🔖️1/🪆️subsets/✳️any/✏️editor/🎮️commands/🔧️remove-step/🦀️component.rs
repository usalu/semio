//! 🔧️ 🔧️ Imperative play app commands command — `remove-step`.

use crate::artifacts::imperative::mutations::{delete_step, ImperativeMutation};
use crate::artifacts::imperative::{ImperativeSnapshot, PathRef, Step};
use crate::editor::imperative::config::{ImperativeConfig, ImperativeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Helpers
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
#[dsl(keyword = "remove-step")]
pub struct RemoveStep {
    pub id: String,
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: no selection pruning here anymore —
// the removed step's id is auto-pruned from the `steps` interaction domain's selection by the
// framework (`validate_state` against `ImperativePlayApp::interaction_topology`) right after this
// document mutation lands.
pub fn handle(payload: &RemoveStep, doc: &ArtifactView<'_, ImperativeSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeMutation, ImperativeConfigMutation>, Fault> {
    let document = doc.snapshot;
    if resolve_contains(document, None, None, &payload.id) {
        Ok(Emit::mutations(vec![delete_step(PathRef::default(), payload.id.clone())]))
    } else {
        Ok(Emit::default())
    }
}
