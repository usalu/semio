//! 🔺️ Imperative artifact — diff structs + `OperationDiff` impl (constitutional: diff).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::imperative::op::ImperativeOperation;
use crate::artifacts::imperative::{ImperativeDocument, Path, PathRef};

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ImperativeDiff(pub Option<ImperativeOperation>);

impl protocol::OperationDiff<ImperativeDocument> for ImperativeDiff {
    fn apply(&self, projection: &ImperativeDocument) -> ImperativeDocument {
        let mut next = projection.clone();
        if let Some(operation) = &self.0 {
            if let Some(steps) = resolve_steps_mut(&mut next, &operation.path_ref) {
                protocol::apply_collection_operation(steps, &operation.collection);
            }
            prune_empty_slot(&mut next, &operation.path_ref);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.0.is_some() {
            self.0 = other.0;
        }
    }
}

fn resolve_steps_mut<'a>(document: &'a mut ImperativeDocument, path_ref: &PathRef) -> Option<&'a mut Vec<crate::artifacts::imperative::Step>> {
    if path_ref.owner.is_none() && path_ref.slot.is_none() {
        return Some(&mut document.path.steps);
    }
    let owner = path_ref.owner.clone()?;
    let slot = path_ref.slot.clone()?;
    let owner_step = document.path.steps.iter_mut().find(|step| step.id == owner)?;
    Some(&mut owner_step.bodies.entry(slot).or_insert_with(Path::new).steps)
}

/// 🧹️ Drops a nested slot's `bodies` entry once it's empty, so an emptied slot is bit-identical to
/// a never-touched one — required for `Add` then `Remove` to be a true, exact inverse pair.
fn prune_empty_slot(document: &mut ImperativeDocument, path_ref: &PathRef) {
    let (Some(owner), Some(slot)) = (&path_ref.owner, &path_ref.slot) else {
        return;
    };
    if let Some(owner_step) = document.path.steps.iter_mut().find(|step| &step.id == owner) {
        if owner_step.bodies.get(slot).is_some_and(|path| path.steps.is_empty()) {
            owner_step.bodies.remove(slot);
        }
    }
}
//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::imperative::{Dictionary, Step};
    use std::collections::BTreeMap;

    fn step(id: &str, kind: &str) -> Step {
        Step { id: id.into(), kind: kind.into(), params: Dictionary::new(), bodies: BTreeMap::new() }
    }

    #[test]
    fn resolve_steps_mut_unknown_owner_is_none() {
        let mut document = crate::artifacts::imperative::engine::default_document();
        let path_ref = PathRef { owner: Some("missing".into()), slot: Some("then".into()) };
        assert!(resolve_steps_mut(&mut document, &path_ref).is_none());
    }

    #[test]
    fn prune_empty_slot_removes_emptied_bodies_entry() {
        let mut document = crate::artifacts::imperative::engine::default_document();
        document.path.steps.push(step("step-if", "control.if"));
        let path_ref = PathRef { owner: Some("step-if".into()), slot: Some("then".into()) };
        resolve_steps_mut(&mut document, &path_ref).expect("materializes slot").push(step("step-nested", "log.print"));
        let owner_step = document.path.steps.iter().find(|s| s.id == "step-if").expect("owner");
        assert!(owner_step.bodies.contains_key("then"));
        resolve_steps_mut(&mut document, &path_ref).expect("slot exists").clear();
        prune_empty_slot(&mut document, &path_ref);
        let owner_step = document.path.steps.iter().find(|s| s.id == "step-if").expect("owner");
        assert!(!owner_step.bodies.contains_key("then"));
    }

    #[test]
    fn prune_empty_slot_noop_without_owner_or_slot() {
        let mut document = crate::artifacts::imperative::engine::default_document();
        prune_empty_slot(&mut document, &PathRef::default());
    }

    #[test]
    fn imperative_diff_absorb_keeps_latest_some_and_ignores_none() {
        use protocol::OperationDiff;
        let first = ImperativeOperation { path_ref: PathRef::default(), collection: protocol::CollectionOperation::Remove { id: "step-1".into() } };
        let second = ImperativeOperation { path_ref: PathRef::default(), collection: protocol::CollectionOperation::Remove { id: "step-2".into() } };
        let mut diff = ImperativeDiff(Some(first));
        diff.absorb(ImperativeDiff(None));
        assert!(matches!(&diff.0, Some(op) if matches!(&op.collection, protocol::CollectionOperation::Remove { id } if id == "step-1")));
        diff.absorb(ImperativeDiff(Some(second)));
        assert!(matches!(&diff.0, Some(op) if matches!(&op.collection, protocol::CollectionOperation::Remove { id } if id == "step-2")));
    }
}
//#endregion 🧪️Tests
