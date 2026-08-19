//! 🔧️ 🔧️ Imperative play app commands command — `add-step-at`.

use crate::editor::imperative::config::{ImperativeConfig, ImperativeConfigMutation};
use crate::artifacts::imperative::mutations::{create_step, ImperativeMutation};
use crate::artifacts::imperative::{Dictionary, ImperativeSnapshot, PathRef, Step};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Helpers
/// 🆔️ Allocates a fresh `step-N` id one past the highest suffix used anywhere in the document
/// (including nested `control.*` bodies), deterministically from pre-state — no mutable counter.
/// Reads through the `flow` working scene (`ImperativeSnapshot` no longer carries `path` inline —
/// ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`).
async fn next_step_id(document: &ImperativeSnapshot) -> String {
    async fn max_suffix(steps: &[Step]) -> u64 {
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
async fn path_ref_from(owner: Option<&str>, slot: Option<&str>, document: &ImperativeSnapshot) -> PathRef {
    let path = crate::artifacts::imperative::imperative_working_scene(document).path;
    match (owner, slot) {
        (Some(owner), Some(slot)) if path.steps.iter().any(|step| step.id == owner) => PathRef { owner: Some(owner.to_string()), slot: Some(slot.to_string()) },
        _ => PathRef::default(),
    }
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
#[dsl(keyword = "add-step-at")]
pub struct AddStepAt {
    pub kind: String,
    pub index: Option<usize>,
    pub owner: Option<String>,
    pub slot: Option<String>,
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: no auto-select of the new step
// anymore — selection is the framework-owned `steps` interaction domain, reachable only through the
// injected `interactionSelect` verb, not an ordinary command's `config_mutations`.
pub async fn handle(payload: &AddStepAt, doc: &ArtifactView<'_, ImperativeSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeMutation, ImperativeConfigMutation>, Fault> {
    let document = doc.snapshot;
    let path_ref = path_ref_from(payload.owner.as_deref(), payload.slot.as_deref(), document);
    let id = next_step_id(document);
    let step = Step { id, kind: payload.kind.clone(), params: Dictionary::new(), bodies: BTreeMap::new() };
    Ok(Emit::mutations(vec![create_step(path_ref, step)]))
}
