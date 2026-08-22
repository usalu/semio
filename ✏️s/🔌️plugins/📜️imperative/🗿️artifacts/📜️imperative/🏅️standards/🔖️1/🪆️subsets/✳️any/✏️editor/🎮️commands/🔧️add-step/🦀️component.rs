//! 🔧️ 🔧️ Imperative play app commands command — `add-step`.

use crate::artifacts::imperative::mutations::{create_step, ImperativeMutation};
use crate::artifacts::imperative::{Dictionary, ImperativeSnapshot, PathRef, Step};
use crate::editor::imperative::config::{ImperativeConfig, ImperativeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
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
