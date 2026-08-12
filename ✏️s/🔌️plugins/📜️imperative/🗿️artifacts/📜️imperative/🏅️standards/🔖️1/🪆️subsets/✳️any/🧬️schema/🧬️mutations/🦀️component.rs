//! 🧬️ imperative artifact — document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/`
//! triad leaves); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<ImperativeSnapshot>`
//! and `impl protocol::SemanticMutation<ImperativeSnapshot>` from those payloads — no hand-written
//! apply/diff/inverse dispatch here.
//!
//! Deviation from the fan-out brief's literal derive list: this enum does NOT also derive
//! `dsl::DslEnum`. `Step`/`Dictionary` are foreign kernel types (`imperative_engine`/
//! `neural_engine`) with no `dsl::DslRecord`/`dsl::DslField` support and `Step.bodies` recurses
//! (unlike the sibling `🎬️sequence` plugin's local `SequenceStep`/`StepParams`, which do derive
//! DSL support), so `ImperativeMutation`'s payload structs cannot derive `dsl::DslRecord` either.
//! The text/binary wire codec for this enum stays hand-written in the sibling `💾️binary` leaf
//! (as it already was for the old struct), converting through the existing `StepNodeDsl`/
//! `ValueDsl` mirrors — see that file's doc comment.

use crate::artifacts::imperative::diff::ImperativeDiff;
use crate::artifacts::imperative::{ImperativeSnapshot, PathRef, Step};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 🧮️ Semantic imperative document mutation vocabulary: id-keyed step create/delete/reorder/
/// edit-params at a `PathRef` — the root path, or a nested `control.*` step's body slot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = ImperativeSnapshot, diff = ImperativeDiff, schema = "imperative.imperative")]
pub enum ImperativeMutation {
    CreateStep(CreateStep),
    DeleteStep(DeleteStep),
    ReorderSteps(ReorderSteps),
    EditStepParams(EditStepParams),
}
//#endregion 🔖️Mutations

pub use super::create_step::mutation::{create_step, CreateStep};
pub use super::delete_step::mutation::{delete_step, DeleteStep};
pub use super::edit_step_params::mutation::{edit_step_params, EditStepParams};
pub use super::reorder_steps::mutation::{reorder_steps, ReorderSteps};

/// 🔎️ Resolves the step list a `PathRef` addresses; a not-yet-materialized nested slot reads as
/// empty. Shared by every triad's `🔺️diff`/`↩️inverse` leaf so base-state lookups agree.
pub fn resolve_steps<'a>(snapshot: &'a ImperativeSnapshot, path_ref: &PathRef) -> Option<&'a [Step]> {
    if path_ref.owner.is_none() && path_ref.slot.is_none() {
        return Some(&snapshot.path.steps);
    }
    let owner = path_ref.owner.as_ref()?;
    let slot = path_ref.slot.as_ref()?;
    let owner_step = snapshot.path.steps.iter().find(|step| &step.id == owner)?;
    Some(owner_step.bodies.get(slot).map_or(&[] as &[Step], |path| path.steps.as_slice()))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::imperative::engine::default_snapshot;
    use crate::artifacts::imperative::Dictionary;
    use neural_engine::{Atom, Value};
    use protocol::testkit::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
    use protocol::SemanticMutation;
    use std::collections::BTreeMap;

    fn step(id: &str, kind: &str) -> Step {
        Step { id: id.into(), kind: kind.into(), params: Dictionary::new(), bodies: BTreeMap::new() }
    }

    //#region 🔖️MutationLaws
    #[test]
    fn create_step_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &create_step(PathRef::default(), step("step-99", "log.print")));
    }

    #[test]
    fn delete_step_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &delete_step(PathRef::default(), "step-1".into()));
    }

    #[test]
    fn delete_step_missing_target_is_a_noop_inverse() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &delete_step(PathRef::default(), "step-missing".into()));
    }

    #[test]
    fn reorder_steps_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &reorder_steps(PathRef::default(), "step-2".into(), 0));
    }

    #[test]
    fn reorder_steps_missing_target_is_a_noop_inverse() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &reorder_steps(PathRef::default(), "step-missing".into(), 0));
    }

    #[test]
    fn edit_step_params_inverse_law() {
        let base = default_snapshot();
        let params = Dictionary::new().insert("message", Value::Atom(Atom::String("hi".into())));
        assert_mutation_inverse_law(&base, &edit_step_params(PathRef::default(), "step-2".into(), params));
    }

    #[test]
    fn edit_step_params_missing_target_is_a_noop_inverse() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &edit_step_params(PathRef::default(), "step-missing".into(), Dictionary::new()));
    }

    #[test]
    fn create_step_diff_absorb_law() {
        use protocol::Mutation;
        let base = default_snapshot();
        let d1 = create_step(PathRef::default(), step("step-97", "log.print")).diff(&base);
        let mid = protocol::MutationDiff::apply(&d1, &base);
        let d2 = create_step(PathRef::default(), step("step-98", "log.print")).diff(&mid);
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn dispatch_registers_semantic_descriptors() {
        register_imperative_mutation_descriptors();
        for kind in ImperativeMutation::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(ImperativeMutation::kinds().len(), 4);
    }
    //#endregion 🔖️MutationLaws
}
//#endregion 🧪️Tests
