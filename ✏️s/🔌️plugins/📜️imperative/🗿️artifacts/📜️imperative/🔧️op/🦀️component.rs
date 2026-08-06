//! 🔧️ Imperative artifact — operation enum + laws (constitutional: op).

use crate::artifacts::imperative::{Dictionary, ImperativeDocument, PathRef, Step};

//#region 🔖️Operation
/// @emoji ✂️ A step-collection edit at a `PathRef` — root path or a nested `control.*` step's slot.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImperativeOperation {
    pub path_ref: PathRef,
    pub collection: protocol::CollectionOperation<String, Step, Dictionary>,
}

impl protocol::Operation<ImperativeDocument> for ImperativeOperation {
    type Diff = crate::artifacts::imperative::diff::ImperativeDiff;

    fn diff(&self, _projection: &ImperativeDocument) -> Self::Diff {
        crate::artifacts::imperative::diff::ImperativeDiff(Some(self.clone()))
    }

    fn backwards(&self, projection: &ImperativeDocument) -> Vec<Self> {
        match resolve_steps(projection, &self.path_ref) {
            Some(steps) => vec![ImperativeOperation { path_ref: self.path_ref.clone(), collection: protocol::invert_collection_operation(steps, &self.collection) }],
            None => Vec::new(),
        }
    }
}

/// 🔎️ Resolves the step list a `PathRef` addresses; a not-yet-materialized nested slot reads as empty.
pub(crate) fn resolve_steps<'a>(document: &'a ImperativeDocument, path_ref: &PathRef) -> Option<&'a [Step]> {
    if path_ref.owner.is_none() && path_ref.slot.is_none() {
        return Some(&document.path.steps);
    }
    let owner = path_ref.owner.as_ref()?;
    let slot = path_ref.slot.as_ref()?;
    let owner_step = document.path.steps.iter().find(|step| &step.id == owner)?;
    Some(owner_step.bodies.get(slot).map_or(&[] as &[Step], |path| path.steps.as_slice()))
}
//#endregion 🔖️Operation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn step(id: &str, kind: &str) -> Step {
        Step { id: id.into(), kind: kind.into(), params: Dictionary::new(), bodies: BTreeMap::new() }
    }

    #[test]
    fn add_step_op_round_trips() {
        let document = crate::artifacts::imperative::engine::default_document();
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: protocol::CollectionOperation::Add { index: 0, item: step("step-x", "log.print") } };
        store::test_support::assert_operation_round_trip(&document, operation.clone());
        store::test_support::assert_op_line_round_trip(&operation);
        store::test_support::assert_store_roundtrip(document, operation);
    }

    #[test]
    fn remove_step_op_round_trips() {
        let document = crate::artifacts::imperative::engine::default_document();
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: protocol::CollectionOperation::Remove { id: "step-1".into() } };
        store::test_support::assert_operation_round_trip(&document, operation.clone());
        store::test_support::assert_op_line_round_trip(&operation);
        store::test_support::assert_store_roundtrip(document, operation);
    }

    #[test]
    fn move_step_op_round_trips() {
        let document = crate::artifacts::imperative::engine::default_document();
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: protocol::CollectionOperation::Move { id: "step-1".into(), to_index: 1 } };
        store::test_support::assert_operation_round_trip(&document, operation.clone());
        store::test_support::assert_op_line_round_trip(&operation);
        store::test_support::assert_store_roundtrip(document, operation);
    }

    #[test]
    fn patch_step_params_op_round_trips() {
        use neural_engine::{Atom, Value};
        let document = crate::artifacts::imperative::engine::default_document();
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: protocol::CollectionOperation::Patch { id: "step-1".into(), patch: Dictionary::new().insert("key", Value::Atom(Atom::String("renamed".into()))) } };
        store::test_support::assert_operation_round_trip(&document, operation.clone());
        store::test_support::assert_op_line_round_trip(&operation);
        store::test_support::assert_store_roundtrip(document, operation);
    }

    #[test]
    fn add_step_into_nested_control_body_round_trips() {
        let mut document = crate::artifacts::imperative::engine::default_document();
        document.path.steps.push(step("step-if", "control.if"));
        let path_ref = PathRef { owner: Some("step-if".into()), slot: Some("then".into()) };
        let operation = ImperativeOperation { path_ref, collection: protocol::CollectionOperation::Add { index: 0, item: step("step-nested", "log.print") } };
        store::test_support::assert_operation_round_trip(&document, operation.clone());
        store::test_support::assert_op_line_round_trip(&operation);
        let post = vcs::apply_operation(&document, &operation);
        let owner_step = post.path.steps.iter().find(|entry| entry.id == "step-if").expect("owner step");
        assert_eq!(owner_step.bodies.get("then").map(|body| body.steps.len()), Some(1));
        store::test_support::assert_store_roundtrip(document, operation);
    }

    #[test]
    fn resolve_steps_root_returns_document_steps() {
        let document = crate::artifacts::imperative::engine::default_document();
        let steps = resolve_steps(&document, &PathRef::default()).expect("root always resolves");
        assert_eq!(steps.len(), document.path.steps.len());
    }

    #[test]
    fn resolve_steps_unknown_owner_is_none() {
        let document = crate::artifacts::imperative::engine::default_document();
        let path_ref = PathRef { owner: Some("missing".into()), slot: Some("then".into()) };
        assert!(resolve_steps(&document, &path_ref).is_none());
    }

    #[test]
    fn resolve_steps_missing_owner_or_slot_is_none() {
        let document = crate::artifacts::imperative::engine::default_document();
        assert!(resolve_steps(&document, &PathRef { owner: Some("step-1".into()), slot: None }).is_none());
        assert!(resolve_steps(&document, &PathRef { owner: None, slot: Some("then".into()) }).is_none());
    }

    #[test]
    fn resolve_steps_unmaterialized_slot_reads_empty() {
        let mut document = crate::artifacts::imperative::engine::default_document();
        document.path.steps.push(step("step-if", "control.if"));
        let path_ref = PathRef { owner: Some("step-if".into()), slot: Some("then".into()) };
        assert_eq!(resolve_steps(&document, &path_ref), Some(&[][..]));
    }

    #[test]
    fn operation_backwards_on_unresolvable_path_ref_is_empty() {
        let document = crate::artifacts::imperative::engine::default_document();
        let operation = ImperativeOperation { path_ref: PathRef { owner: Some("missing".into()), slot: Some("then".into()) }, collection: protocol::CollectionOperation::Remove { id: "step-x".into() } };
        assert!(protocol::Operation::backwards(&operation, &document).is_empty());
    }
}
//#endregion 🧪️Tests
