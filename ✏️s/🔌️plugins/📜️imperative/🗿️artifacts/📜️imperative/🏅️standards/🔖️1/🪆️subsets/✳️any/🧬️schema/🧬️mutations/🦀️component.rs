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
use crate::artifacts::imperative::{ImperativeSnapshot, Path, PathRef, Step};
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

/// 🏷️ Kebab-case spelling of every [`ImperativeMutation`] variant, in declaration order — the
/// vocabulary the `imperative-1-any` mutation catalog (`../../🧪️oracle/🔣️component.json`) declares
/// and `mutate-imperative-1`'s exhaustive case measures itself against. Four kinds, and every one of
/// them addresses a `PathRef` rather than a bare id, because a step list is NESTED: `{}` is the root
/// program and `{"owner": "step-3", "slot": "then"}` is a branch body inside it, and the same step id
/// in the two scopes is two different targets. There is no `set-snapshot` and no `edit-step-kind`:
/// whole-document replace reaches the store through its non-history path, and a step's `kind`
/// determines which bodies it may carry, so retyping one is a create/delete pair rather than an edit.
/// [`kinds_match_the_enum_and_the_catalog`] keeps this list honest against the enum, since the
/// framework never parses Rust.
pub const KINDS: &[&str] = &["create-step", "delete-step", "reorder-steps", "edit-step-params"];

//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes this facet's internally-tagged (`{"mutation": "createStep", …}`, camelCase payload
/// fields) JSON projection — exactly the shape the committed
/// `<slug>/🧪️tests/<fixture>/🦠️mutation/🔣️component.json` specification vectors and
/// `mutate-imperative-1`'s own `Examples` payloads carry — into a real [`ImperativeMutation`]. The
/// test adapter cannot reach `serde_json` (the generated host links only `semio-repo-test-host` and
/// this crate) and cannot name this crate's private `protocol`/`store` extern-crate aliases either,
/// so the bridge belongs here rather than there.
pub fn decode_imperative_mutation_json(text: &str) -> Result<ImperativeMutation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// 🌱 Resolves `snapshot`'s composed `s.stdio.semio.flow` child to the program in `program_json`
/// (a `{"steps": [...]}` `Path`). An imperative document persists only a content-addressed HANDLE,
/// and the working scene is a thread-local scratch cache, so a decoded `⬅️before` stands for no
/// program at all until something caches one — which is exactly what each triad leaf's own
/// `cached_program()` does inside this crate. `mutate-imperative-1` needs the same seeding from
/// outside, where neither `Path` nor `cache_imperative_flow`'s `Dictionary`/`Value` argument types
/// can be constructed, so the program travels as JSON and is decoded here.
pub fn seed_imperative_flow_json(snapshot: &ImperativeSnapshot, program_json: &str) -> Result<(), String> {
    let path: Path = serde_json::from_str(program_json).map_err(|error| error.to_string())?;
    crate::artifacts::imperative::cache_imperative_flow(&snapshot.flow.child_id, &path);
    Ok(())
}

/// ▶️ Applies `mutation` in place and returns every diagnostic it raised as `(code, severity)`
/// pairs. All four committed vectors leave the document byte-identical — two refusals and two
/// `Warning`-level no-ops — so the pair is the evidence rather than a side channel.
pub fn apply_imperative_mutation_reporting(snapshot: &mut ImperativeSnapshot, mutation: &ImperativeMutation) -> Vec<(String, String)> {
    let outcome = <ImperativeMutation as protocol::Mutation<ImperativeSnapshot>>::diff(mutation, snapshot).apply_to(snapshot);
    outcome.messages().iter().map(|message| (message.code.0.clone(), format!("{:?}", message.level))).collect()
}

/// ↩️ The mutation's OWN computed undo steps, which is what an `inverse-<kind>` scenario has to
/// apply for the metamorphic law to mean anything.
pub fn inverse_imperative_mutation_steps(mutation: &ImperativeMutation, base: &ImperativeSnapshot) -> Vec<ImperativeMutation> {
    <ImperativeMutation as protocol::Mutation<ImperativeSnapshot>>::inverse(mutation, base)
}

/// 🔎️ The program the document's composed flow child currently resolves to, rendered as nested
/// `id:kind` entries in list order — the readable half of a divergence message, so a failing
/// scenario names WHICH step moved rather than only that two content digests differ.
pub fn imperative_program_summary(snapshot: &ImperativeSnapshot) -> String {
    fn render(path: &Path) -> String {
        path.steps
            .iter()
            .map(|step| {
                let bodies = step.bodies.iter().map(|(slot, body)| format!("{slot}{{{}}}", render(body))).collect::<Vec<_>>().join(" ");
                if bodies.is_empty() {
                    format!("{}:{}", step.id, step.kind)
                } else {
                    format!("{}:{}[{bodies}]", step.id, step.kind)
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
    render(&crate::artifacts::imperative::imperative_working_scene(snapshot).path)
}
//#endregion 🌉️ExternalCodecBridge

/// 🔎️ Resolves the step list a `PathRef` addresses (read from the live `flow` working scene, since
/// `ImperativeSnapshot` no longer carries `path` directly — ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`); a not-yet-materialized nested slot reads as
/// empty. Owned `Vec` (not a borrow) since the working scene is a cache lookup, not a live borrow of
/// `snapshot` itself. Shared by every triad's `🔺️diff`/`↩️inverse` leaf so base-state lookups agree.
pub fn resolve_steps(snapshot: &ImperativeSnapshot, path_ref: &PathRef) -> Vec<Step> {
    let path = crate::artifacts::imperative::imperative_working_scene(snapshot).path;
    resolve_steps_in_path(&path, path_ref)
}

fn resolve_steps_in_path(path: &Path, path_ref: &PathRef) -> Vec<Step> {
    if path_ref.owner.is_none() && path_ref.slot.is_none() {
        return path.steps.clone();
    }
    let (Some(owner), Some(slot)) = (&path_ref.owner, &path_ref.slot) else { return Vec::new() };
    let Some(owner_step) = path.steps.iter().find(|step| &step.id == owner) else { return Vec::new() };
    owner_step.bodies.get(slot).map(|body| body.steps.clone()).unwrap_or_default()
}

/// 🔧 Resolves the MUTABLE step list at `path_ref` within a live working-scene `Path` — the
/// mutation-side counterpart of `resolve_steps`, used by every triad's `🔺️diff` builder to edit a
/// full copy of the current path before re-minting a whole `flow` handle (composed children are
/// opaque; a diff never edits a sub-slice, only mints a whole replacement — see
/// `crate::artifacts::imperative::diff_replace_flow`).
pub fn resolve_path_mut<'a>(path: &'a mut Path, path_ref: &PathRef) -> Option<&'a mut Vec<Step>> {
    if path_ref.owner.is_none() && path_ref.slot.is_none() {
        return Some(&mut path.steps);
    }
    let owner = path_ref.owner.clone()?;
    let slot = path_ref.slot.clone()?;
    let owner_step = path.steps.iter_mut().find(|step| step.id == owner)?;
    Some(&mut owner_step.bodies.entry(slot).or_insert_with(Path::new).steps)
}

/// 🧹 Removes a now-empty nested body slot after a delete — mirrors the pre-migration snapshot-
/// level `prune_empty_slot` this same helper set used to own.
pub fn prune_empty_slot(path: &mut Path, path_ref: &PathRef) {
    let (Some(owner), Some(slot)) = (&path_ref.owner, &path_ref.slot) else { return };
    if let Some(owner_step) = path.steps.iter_mut().find(|step| &step.id == owner) {
        if owner_step.bodies.get(slot).is_some_and(|body| body.steps.is_empty()) {
            owner_step.bodies.remove(slot);
        }
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::imperative::schema::default_snapshot;
    use crate::artifacts::imperative::Dictionary;
    use neural_engine::{Atom, Value};
    use protocol::os_spr::testkit::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
    use protocol::SemanticMutation;

    /// 🏷️ The three declarations of this vocabulary — the enum, [`KINDS`] and the committed catalog
    /// — must agree, in spelling AND in order. The framework never parses Rust, so without this test
    /// `KINDS` could drift from the enum and the catalog could keep measuring `mutate-imperative-1`
    /// against a vocabulary the artifact no longer has.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = ImperativeMutation::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️component.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
        assert!(!manifest.contains("\"set-snapshot\"") && !manifest.contains("\"edit-step-kind\""), "neither whole-document replace nor a step-kind edit is a mutation here — the catalog must not smuggle either in");
    }
    use std::collections::BTreeMap;

    fn step(id: &str, kind: &str) -> Step {
        Step { id: id.into(), kind: kind.into(), params: Dictionary::new(), bodies: BTreeMap::new() }
    }

    //#region 🔖️MutationLaws
    #[semio_framework_async_macros::async_test]
    async fn create_step_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &create_step(PathRef::default(), step("step-99", "log.print")));
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_step_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &delete_step(PathRef::default(), "step-1".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_step_missing_target_is_error() {
        let base = default_snapshot();
        protocol::os_spr::testkit::assert_missing_target_is_error(&base, &delete_step(PathRef::default(), "step-missing".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn reorder_steps_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &reorder_steps(PathRef::default(), "step-2".into(), 0));
    }

    #[semio_framework_async_macros::async_test]
    async fn reorder_steps_missing_target_is_error() {
        let base = default_snapshot();
        protocol::os_spr::testkit::assert_missing_target_is_error(&base, &reorder_steps(PathRef::default(), "step-missing".into(), 0));
    }

    #[semio_framework_async_macros::async_test]
    async fn edit_step_params_inverse_law() {
        let base = default_snapshot();
        let params = Dictionary::new().insert("message", Value::Atom(Atom::String("hi".into())));
        assert_mutation_inverse_law(&base, &edit_step_params(PathRef::default(), "step-2".into(), params));
    }

    #[semio_framework_async_macros::async_test]
    async fn edit_step_params_missing_target_is_error() {
        let base = default_snapshot();
        protocol::os_spr::testkit::assert_missing_target_is_error(&base, &edit_step_params(PathRef::default(), "step-missing".into(), Dictionary::new()));
    }

    #[semio_framework_async_macros::async_test]
    async fn create_step_duplicate_id_fatal_never_applies() {
        let base = default_snapshot();
        let mutation = create_step(PathRef::default(), step("step-1", "log.print"));
        protocol::os_spr::testkit::assert_fatal_never_applies(&protocol::Mutation::diff(&mutation, &base));
    }

    #[semio_framework_async_macros::async_test]
    async fn create_step_diff_absorb_law() {
        use protocol::Mutation;
        let base = default_snapshot();
        let d1 = create_step(PathRef::default(), step("step-97", "log.print")).diff(&base).into_parts().0;
        let mid = protocol::MutationDiff::apply(&d1, &base).expect("valid mutation diff");
        let d2 = create_step(PathRef::default(), step("step-98", "log.print")).diff(&mid).into_parts().0;
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[semio_framework_async_macros::async_test]
    async fn dispatch_registers_semantic_descriptors() {
        register_imperative_mutation_descriptors();
        for kind in ImperativeMutation::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(ImperativeMutation::kinds().len(), 4);
    }
    //#endregion 🔖️MutationLaws
}
//#endregion 🧪️Tests
