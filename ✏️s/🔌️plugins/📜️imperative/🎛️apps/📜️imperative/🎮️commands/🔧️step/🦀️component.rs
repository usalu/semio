//! 🔧️ Imperative play app commands — step CRUD: document-mutating edits dispatched as VCS operations
//! with a true inverse. The `*At` variants address a nested `control.*` body via `owner`/`slot` (drag-
//! and-drop into blocks).

use crate::apps::imperative::config::{ImperativeConfig, ImperativeConfigMutation};
use crate::artifacts::imperative::dsl::ValueDsl;
use crate::artifacts::imperative::mutations::ImperativeMutation;
use crate::artifacts::imperative::{Dictionary, ImperativeDocument, PathRef, Step};
use protocol::CollectionMutation;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Helpers
/// 🆔️ Allocates a fresh `step-N` id one past the highest suffix used anywhere in the document
/// (including nested `control.*` bodies), deterministically from pre-state — no mutable counter.
fn next_step_id(document: &ImperativeDocument) -> String {
    fn max_suffix(steps: &[Step]) -> u64 {
        steps.iter().fold(0, |acc, step| {
            let own = step.id.strip_prefix("step-").and_then(|rest| rest.parse::<u64>().ok()).unwrap_or(0);
            let nested = step.bodies.values().map(|path| max_suffix(&path.steps)).max().unwrap_or(0);
            acc.max(own).max(nested)
        })
    }
    format!("step-{}", max_suffix(&document.path.steps) + 1)
}

/// 📍️ Resolves `owner`/`slot` command fields into a [`PathRef`] so nested control-step bodies (e.g.
/// `control.if` then/else) resolve correctly; falls back to the root path unless both are present and
/// `owner` names a real top-level step, avoiding an unresolvable or unknown reference that would
/// otherwise address nothing.
fn path_ref_from(owner: Option<&str>, slot: Option<&str>, document: &ImperativeDocument) -> PathRef {
    match (owner, slot) {
        (Some(owner), Some(slot)) if document.path.steps.iter().any(|step| step.id == owner) => PathRef { owner: Some(owner.to_string()), slot: Some(slot.to_string()) },
        _ => PathRef::default(),
    }
}

/// 🔎️ Resolves the step list a `PathRef` addresses — the root path, or a nested `control.*` step's slot
/// (an unmaterialized slot reads as empty).
fn steps_at<'a>(document: &'a ImperativeDocument, path_ref: &PathRef) -> &'a [Step] {
    match (&path_ref.owner, &path_ref.slot) {
        (Some(owner), Some(slot)) => document.path.steps.iter().find(|step| &step.id == owner).and_then(|step| step.bodies.get(slot)).map_or(&[], |path| path.steps.as_slice()),
        _ => document.path.steps.as_slice(),
    }
}

/// 🔎️ True when the step `id` exists in the list the `owner`/`slot` command fields address — the
/// pre-state guard the operation arms share so a stale id never emits a no-operation edit into history.
fn resolve_contains(document: &ImperativeDocument, owner: Option<&str>, slot: Option<&str>, id: &str) -> bool {
    let path_ref = path_ref_from(owner, slot, document);
    steps_at(document, &path_ref).iter().any(|step| step.id == id)
}
//#endregion 🔖️Helpers

//#region 🔖️AddStep
pub mod add_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-step")]
    pub struct AddStep {
        pub kind: String,
        pub index: Option<usize>,
    }

    pub fn handle(payload: &AddStep, doc: &DocumentView<'_, ImperativeDocument>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeMutation, ImperativeConfigMutation>, Fault> {
        let document = doc.projection;
        let id = next_step_id(document);
        let step = Step { id: id.clone(), kind: payload.kind.clone(), params: Dictionary::new(), bodies: BTreeMap::new() };
        Ok(Emit {
            document_mutations: vec![ImperativeMutation { path_ref: PathRef::default(), collection: CollectionMutation::Add { index: payload.index.unwrap_or(usize::MAX), item: step } }],
            config_mutations: vec![ImperativeConfigMutation::SetSelectedSteps { ids: vec![id] }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️AddStep

//#region 🔖️AddStepAt
pub mod add_step_at {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-step-at")]
    pub struct AddStepAt {
        pub kind: String,
        pub index: Option<usize>,
        pub owner: Option<String>,
        pub slot: Option<String>,
    }

    pub fn handle(payload: &AddStepAt, doc: &DocumentView<'_, ImperativeDocument>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeMutation, ImperativeConfigMutation>, Fault> {
        let document = doc.projection;
        let path_ref = path_ref_from(payload.owner.as_deref(), payload.slot.as_deref(), document);
        let id = next_step_id(document);
        let step = Step { id: id.clone(), kind: payload.kind.clone(), params: Dictionary::new(), bodies: BTreeMap::new() };
        Ok(Emit {
            document_mutations: vec![ImperativeMutation { path_ref, collection: CollectionMutation::Add { index: payload.index.unwrap_or(usize::MAX), item: step } }],
            config_mutations: vec![ImperativeConfigMutation::SetSelectedSteps { ids: vec![id] }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️AddStepAt

//#region 🔖️RemoveStep
pub mod remove_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-step")]
    pub struct RemoveStep {
        pub id: String,
    }

    pub fn handle(payload: &RemoveStep, doc: &DocumentView<'_, ImperativeDocument>, cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeMutation, ImperativeConfigMutation>, Fault> {
        let document = doc.projection;
        let config = cfg.projection;
        if resolve_contains(document, None, None, &payload.id) {
            let mut ids = config.selected_step_ids.clone();
            ids.retain(|step_id| step_id != &payload.id);
            Ok(Emit {
                document_mutations: vec![ImperativeMutation { path_ref: PathRef::default(), collection: CollectionMutation::Remove { id: payload.id.clone() } }],
                config_mutations: vec![ImperativeConfigMutation::SetSelectedSteps { ids }],
                ..Default::default()
            })
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️RemoveStep

//#region 🔖️RemoveStepAt
pub mod remove_step_at {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-step-at")]
    pub struct RemoveStepAt {
        pub id: String,
        pub owner: Option<String>,
        pub slot: Option<String>,
    }

    pub fn handle(payload: &RemoveStepAt, doc: &DocumentView<'_, ImperativeDocument>, cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeMutation, ImperativeConfigMutation>, Fault> {
        let document = doc.projection;
        let config = cfg.projection;
        if resolve_contains(document, payload.owner.as_deref(), payload.slot.as_deref(), &payload.id) {
            let path_ref = path_ref_from(payload.owner.as_deref(), payload.slot.as_deref(), document);
            let mut ids = config.selected_step_ids.clone();
            ids.retain(|step_id| step_id != &payload.id);
            Ok(Emit {
                document_mutations: vec![ImperativeMutation { path_ref, collection: CollectionMutation::Remove { id: payload.id.clone() } }],
                config_mutations: vec![ImperativeConfigMutation::SetSelectedSteps { ids }],
                ..Default::default()
            })
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️RemoveStepAt

//#region 🔖️MoveStep
pub mod move_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "move-step")]
    pub struct MoveStep {
        pub id: String,
        pub index: usize,
    }

    pub fn handle(payload: &MoveStep, doc: &DocumentView<'_, ImperativeDocument>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeMutation, ImperativeConfigMutation>, Fault> {
        let document = doc.projection;
        if resolve_contains(document, None, None, &payload.id) {
            Ok(Emit::mutations(vec![ImperativeMutation { path_ref: PathRef::default(), collection: CollectionMutation::Move { id: payload.id.clone(), to_index: payload.index } }]))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️MoveStep

//#region 🔖️MoveStepAt
pub mod move_step_at {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "move-step-at")]
    pub struct MoveStepAt {
        pub id: String,
        pub index: usize,
        pub owner: Option<String>,
        pub slot: Option<String>,
    }

    pub fn handle(payload: &MoveStepAt, doc: &DocumentView<'_, ImperativeDocument>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeMutation, ImperativeConfigMutation>, Fault> {
        let document = doc.projection;
        if resolve_contains(document, payload.owner.as_deref(), payload.slot.as_deref(), &payload.id) {
            let path_ref = path_ref_from(payload.owner.as_deref(), payload.slot.as_deref(), document);
            Ok(Emit::mutations(vec![ImperativeMutation { path_ref, collection: CollectionMutation::Move { id: payload.id.clone(), to_index: payload.index } }]))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️MoveStepAt

//#region 🔖️SetStepParams
pub mod set_step_params {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-step-params")]
    pub struct SetStepParams {
        pub id: String,
        pub params: BTreeMap<String, ValueDsl>,
    }

    pub fn handle(payload: &SetStepParams, doc: &DocumentView<'_, ImperativeDocument>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeMutation, ImperativeConfigMutation>, Fault> {
        let document = doc.projection;
        if resolve_contains(document, None, None, &payload.id) {
            Ok(Emit::mutations(vec![ImperativeMutation { path_ref: PathRef::default(), collection: CollectionMutation::Patch { id: payload.id.clone(), patch: crate::artifacts::imperative::dsl::value_dsl_map_to_dictionary(&payload.params) } }]))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️SetStepParams

//#region 🔖️SetStepParamsAt
pub mod set_step_params_at {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-step-params-at")]
    pub struct SetStepParamsAt {
        pub id: String,
        pub owner: Option<String>,
        pub slot: Option<String>,
        pub params: BTreeMap<String, ValueDsl>,
    }

    pub fn handle(payload: &SetStepParamsAt, doc: &DocumentView<'_, ImperativeDocument>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ImperativeMutation, ImperativeConfigMutation>, Fault> {
        let document = doc.projection;
        if resolve_contains(document, payload.owner.as_deref(), payload.slot.as_deref(), &payload.id) {
            let path_ref = path_ref_from(payload.owner.as_deref(), payload.slot.as_deref(), document);
            Ok(Emit::mutations(vec![ImperativeMutation { path_ref, collection: CollectionMutation::Patch { id: payload.id.clone(), patch: crate::artifacts::imperative::dsl::value_dsl_map_to_dictionary(&payload.params) } }]))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️SetStepParamsAt
