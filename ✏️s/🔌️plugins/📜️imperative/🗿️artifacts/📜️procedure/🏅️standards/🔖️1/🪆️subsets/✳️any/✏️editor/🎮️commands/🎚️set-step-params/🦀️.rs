//! 🔧️ 🔧️ Imperative play app commands command — `set-step-params`.

use crate::artifacts::procedure::dsl::ValueDsl;
use crate::artifacts::procedure::mutations::{edit_step_params, ProcedureMutation};
use crate::artifacts::procedure::{ProcedureSnapshot, PathRef, Step};
use crate::editor::procedure::config::{ImperativeConfig, ImperativeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use std::collections::BTreeMap;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Helpers
/// 📍️ Resolves `owner`/`slot` command fields into a [`PathRef`] so nested control-step bodies (e.g.
/// `control.if` then/else) resolve correctly; falls back to the root path unless both are present and
/// `owner` names a real top-level step, avoiding an unresolvable or unknown reference that would
/// otherwise address nothing.
fn path_ref_from(owner: Option<&str>, slot: Option<&str>, document: &ProcedureSnapshot) -> PathRef {
    let path = crate::artifacts::procedure::procedure_working_scene(document).path;
    match (owner, slot) {
        (Some(owner), Some(slot)) if path.steps.iter().any(|step| step.id == owner) => PathRef { owner: Some(owner.to_string()), slot: Some(slot.to_string()) },
        _ => PathRef::default(),
    }
}

/// 🔎️ Resolves the step list a `PathRef` addresses — the root path, or a nested `control.*` step's slot
/// (an unmaterialized slot reads as empty).
fn steps_at(document: &ProcedureSnapshot, path_ref: &PathRef) -> Vec<Step> {
    let path = crate::artifacts::procedure::procedure_working_scene(document).path;
    match (&path_ref.owner, &path_ref.slot) {
        (Some(owner), Some(slot)) => path.steps.iter().find(|step| &step.id == owner).and_then(|step| step.bodies.get(slot)).map(|body| body.steps.clone()).unwrap_or_default(),
        _ => path.steps,
    }
}

/// 🔎️ True when the step `id` exists in the list the `owner`/`slot` command fields address — the
/// pre-state guard the operation arms share so a stale id never emits a no-operation edit into history.
fn resolve_contains(document: &ProcedureSnapshot, owner: Option<&str>, slot: Option<&str>, id: &str) -> bool {
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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-step-params")]
pub struct SetStepParams {
    pub id: String,
    pub params: BTreeMap<String, ValueDsl>,
}

pub fn handle(payload: &SetStepParams, doc: &ArtifactView<'_, ProcedureSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ProcedureMutation, ImperativeConfigMutation>, Fault> {
    let document = doc.snapshot;
    if resolve_contains(document, None, None, &payload.id) {
        Ok(Emit::mutations(vec![edit_step_params(PathRef::default(), payload.id.clone(), crate::artifacts::procedure::dsl::value_dsl_map_to_dictionary(&payload.params))]))
    } else {
        Ok(Emit::default())
    }
}
