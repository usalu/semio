//! 🧬️ playbook artifact — semantic document mutation dispatch enum. Every variant is a
//! single-field tuple wrapping a handcrafted `protocol::MutationKind` payload (see the
//! `🧬️mutations/<slug>/` triad leaves); `#[derive(dsl::Mutations)]` generates
//! `impl protocol::Mutation<PlaybookSnapshot>` and `impl protocol::SemanticMutation<PlaybookSnapshot>`
//! from those payloads — no hand-written apply/diff/inverse dispatch here.
//!
//! Moved from the framework kernel module
//! (`🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️component.rs`) by ticket
//! `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`'s playbook design decision: the dispatch enum cannot stay
//! in the framework and wrap plugin-local payload structs (crate dependency direction — the
//! framework cannot depend on a plugin), so it moves here, matching the other 106 mutation facets.
//! Domain types (`PlaybookStep`/`PlaybookBlock`/`PlaybookExpr`), validation, `generation_forms`, and
//! `builder_kit`'s rendering half stay in the framework kernel (`crate::playbook::*`) — only the
//! mutation vocabulary moved.

use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 🧮️ Semantic playbook document mutation vocabulary: id-keyed step/block add/remove/move, a
/// whole-block replace, a step-header update, and the playbook's own title scalar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = PlaybookSnapshot, diff = PlaybookDiff, schema = "playbook.playbook")]
pub enum PlaybookMutation {
    AddStep(AddStep),
    RemoveStep(RemoveStep),
    MoveStep(MoveStep),
    AddBlock(AddBlock),
    RemoveBlock(RemoveBlock),
    MoveBlock(MoveBlock),
    ReplaceBlock(ReplaceBlock),
    UpdateStep(UpdateStep),
    ChangeTitle(ChangeTitle),
}
//#endregion 🔖️Mutations

pub use super::add_block::{add_block_operation, AddBlock};
pub use super::add_step::{add_step_operation, AddStep};
pub use super::change_title::{change_title_operation, ChangeTitle};
pub use super::move_block::{move_block_operation, MoveBlock};
pub use super::move_step::{move_step_operation, MoveStep};
pub use super::remove_block::{remove_block_operation, RemoveBlock};
pub use super::remove_step::{remove_step_operation, RemoveStep};
pub use super::replace_block::{replace_block_operation, ReplaceBlock};
pub use super::update_step::{update_step_operation, UpdateStep};

/// ▶️ Applies `mutation` via its diff. External call site: `derived_construction`'s
/// `ArtifactBuilder::mutate` (`../🦀️component.rs`).
pub fn apply_playbook_mutation(snapshot: &PlaybookSnapshot, mutation: &PlaybookMutation) -> protocol::MutationApplyResult<PlaybookSnapshot> {
    protocol::MutationDiff::apply(protocol::Mutation::diff(mutation, snapshot).diff(), snapshot)
}

/// ↩️ Computes `mutation`'s inverse from the pre-state `snapshot`.
pub fn inverse_playbook_mutation(snapshot: &PlaybookSnapshot, mutation: &PlaybookMutation) -> Vec<PlaybookMutation> {
    protocol::Mutation::inverse(mutation, snapshot)
}

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every [`PlaybookMutation`] variant, in declaration order — the vocabulary the
/// `playbook-1-any` mutation catalog (`../../🧪️oracle/🔣️.json`) declares and the
/// exhaustive `mutate-*` case measures itself against (3 step kinds, 4 block kinds, one step-header patch and the document title). The framework never
/// parses Rust, so `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest
/// against both the enum and the committed catalog.
pub const KINDS: &[&str] = &["add-step", "remove-step", "move-step", "add-block", "remove-block", "move-block", "replace-block", "update-step", "change-title"];

/// 🧮️ Applies `mutation` to `base` and hands back the whole `protocol::MutationOutcome`, the
/// diagnostics included — the shape an external conformance host needs, since a committed
/// `🎯️outcome` vector declares a status AND its diagnostic codes, and the plain apply wrapper
/// beside this one answers `Result<_, _>` and drops the messages.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn apply_playbook_mutation_outcome(snapshot: &mut PlaybookSnapshot, mutation: &PlaybookMutation) -> protocol::MutationOutcome<PlaybookDiff> {
    let outcome = <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::diff(mutation, snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ `mutation`'s own inverse against `base`, as the step LIST `protocol::Mutation::inverse`
/// returns. Reachable from outside this crate, which `protocol::Mutation` itself is not — the
/// `protocol` extern-crate alias is private to `📦️glue.rs`.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn inverse_playbook_mutation_steps(mutation: &PlaybookMutation, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
    <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::inverse(mutation, base)
}

/// 📥️ Decodes the internally-tagged (`{"mutation": "<camelCaseVariant>", …}`) projection the
/// committed `<slug>/🧪️tests/<fixture>/🦠️mutation/🔣️component.json` vectors carry.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn decode_playbook_mutation_json(text: &str) -> Result<PlaybookMutation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// 📥️ Decodes a committed `📸️snapshot/{⬅️before,➡️after}/🔣️component.json` vector.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn decode_playbook_snapshot_json(text: &str) -> Result<PlaybookSnapshot, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// 📤️ The snapshot as the same canonical JSON the committed vectors are written in — the
/// projection an external test host compares through.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn encode_playbook_snapshot_json(snapshot: &PlaybookSnapshot) -> String {
    serde_json::to_string(snapshot).expect("a PlaybookSnapshot is always serializable")
}
/// 🌱 Attaches the working scene to this snapshot's exact composed `flow` child handle from a
/// committed `[PlaybookStep]` JSON document, and hands back what it decoded.
///
/// This subset's persisted snapshot holds only the child HANDLE; the live rows behind it are an
/// ephemeral, session-side scene that a fresh process has never populated. A committed
/// `📸️snapshot/⬅️before/🔣️component.json` vector is therefore only HALF of a before-state, and the
/// other half lives today in each leaf's own `🧪️tests/<fixture>/🦀️component.rs` as a Rust literal.
/// An external conformance host cannot reach that, so this bridge lets the scene half travel as
/// DATA — the exhaustive `mutate-playbook-1` case carries it in its own `Examples` table, with the leaf
/// it was read from cited there. The right long-term fix is to commit the scene beside the snapshot
/// as a fixture file of its own; until then this is the seam that makes the vectors runnable.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn seed_playbook_scene_json(snapshot: &mut PlaybookSnapshot, steps_json: &str) -> Result<Vec<crate::artifacts::playbook::PlaybookStep>, String> {
    let steps: Vec<crate::artifacts::playbook::PlaybookStep> = serde_json::from_str(steps_json).map_err(|error| error.to_string())?;
    crate::artifacts::playbook::attach_playbook_steps(&mut snapshot.flow, steps.clone());
    Ok(steps)
}
//#endregion 🔖️Kinds

//#region 🧪️KindsCatalog
#[cfg(test)]
mod kinds_catalog {
    use super::*;

    /// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
    /// `#[derive(dsl::Mutations)]` assigns, and every one of those spellings must also appear in the
    /// committed `playbook-1-any` catalog. The framework reads the catalog and never the enum, so
    /// this is the only thing standing between a renamed variant and a mutation catalog that
    /// silently measures a vocabulary the code no longer has.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = <PlaybookMutation as protocol::SemanticMutation<PlaybookSnapshot>>::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared PlaybookMutation variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
}
//#endregion 🧪️KindsCatalog

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::playbook::{PlaybookBlock, PlaybookStep};
    use protocol::testkit::{assert_missing_target_is_error, assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
    use protocol::MutationKind;
    use protocol::SemanticMutation;

    fn sample_block(id: &str, kind: &str, label: &str) -> PlaybookBlock {
        PlaybookBlock {
            id: id.into(),
            label: label.into(),
            kind: kind.into(),
            description: None,
            required: None,
            placeholder: None,
            default: None,
            min: None,
            max: None,
            step: None,
            unit: None,
            text: None,
            options: None,
            fields: None,
            schema: None,
            src: None,
            accept: None,
            fixture_slug: None,
            params: None,
            condition: None,
        }
    }

    fn sample_snapshot() -> PlaybookSnapshot {
        let base = PlaybookSnapshot::default();
        let mut steps = base.steps();
        steps.push(PlaybookStep { id: "s2".into(), title: "Review".into(), description: None, blocks: vec![sample_block("b1", "number", "Team size")] });
        crate::artifacts::playbook::playbook_snapshot_with_steps(&base.schema, &base.id, &base.version, base.title.clone(), steps)
    }

    //#region 🔖️MutationLaws
    #[semio_framework_async_macros::async_test]
    async fn add_step_inverse_law() {
        let base = sample_snapshot();
        let step = PlaybookStep { id: "s3".into(), title: "New".into(), description: None, blocks: Vec::new() };
        assert_mutation_inverse_law(&base, &PlaybookMutation::AddStep(AddStep { step, index: None }));
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_step_inverse_law() {
        let base = sample_snapshot();
        assert_mutation_inverse_law(&base, &PlaybookMutation::RemoveStep(RemoveStep { step_id: "s2".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_step_inverse_law() {
        let base = sample_snapshot();
        assert_mutation_inverse_law(&base, &PlaybookMutation::MoveStep(MoveStep { step_id: "s2".into(), index: 0 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn add_block_inverse_law() {
        let base = sample_snapshot();
        let block = sample_block("b2", "text", "New");
        assert_mutation_inverse_law(&base, &PlaybookMutation::AddBlock(AddBlock { step_id: "s2".into(), block, index: None }));
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_block_inverse_law() {
        let base = sample_snapshot();
        assert_mutation_inverse_law(&base, &PlaybookMutation::RemoveBlock(RemoveBlock { step_id: "s2".into(), block_id: "b1".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_block_same_step_inverse_law() {
        let base = sample_snapshot();
        let mut steps = base.steps();
        steps[1].blocks.push(sample_block("b2", "text", "Other"));
        let base = crate::artifacts::playbook::playbook_snapshot_with_steps(&base.schema, &base.id, &base.version, base.title.clone(), steps);
        assert_mutation_inverse_law(&base, &PlaybookMutation::MoveBlock(MoveBlock { block_id: "b1".into(), from_step_id: "s2".into(), to_step_id: "s2".into(), index: 1 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_block_cross_step_inverse_law() {
        let base = sample_snapshot();
        assert_mutation_inverse_law(&base, &PlaybookMutation::MoveBlock(MoveBlock { block_id: "b1".into(), from_step_id: "s2".into(), to_step_id: "s".into(), index: 0 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn replace_block_inverse_law() {
        let base = sample_snapshot();
        let mut block = sample_block("b1", "number", "Team size (people)");
        block.required = Some(true);
        block.min = Some(1.0);
        block.max = Some(80.0);
        block.unit = Some("people".into());
        assert_mutation_inverse_law(&base, &PlaybookMutation::ReplaceBlock(ReplaceBlock { step_id: "s2".into(), block }));
    }

    #[semio_framework_async_macros::async_test]
    async fn update_step_inverse_law() {
        let base = sample_snapshot();
        assert_mutation_inverse_law(&base, &PlaybookMutation::UpdateStep(UpdateStep { step_id: "s2".into(), title: "Review carefully".into(), description: Some("d".into()) }));
    }

    #[semio_framework_async_macros::async_test]
    async fn change_title_inverse_law() {
        let base = sample_snapshot();
        assert_mutation_inverse_law(&base, &PlaybookMutation::ChangeTitle(ChangeTitle { new_title: Some("Recipe".into()) }));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_step_diff_absorb_law() {
        let base = sample_snapshot();
        let d1 = MoveStep { step_id: "s2".into(), index: 0 }.diff(&base).into_parts().0;
        let mid = protocol::MutationDiff::apply(&d1, &base).expect("valid mutation diff");
        let d2 = MoveStep { step_id: "s".into(), index: 0 }.diff(&mid).into_parts().0;
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[semio_framework_async_macros::async_test]
    async fn move_block_cross_step_diff_never_falls_back_to_a_whole_artifact_replacement() {
        let base = sample_snapshot();
        let diff = MoveBlock { block_id: "b1".into(), from_step_id: "s2".into(), to_step_id: "s".into(), index: 0 }.diff(&base).into_parts().0;
        assert!(diff.artifact.is_none(), "cross-step MoveBlock diff must be a real per-field replacement, not the old whole-artifact fallback");
        let after = protocol::MutationDiff::apply(&diff, &base).expect("valid mutation diff");
        let after_steps = after.steps();
        assert!(after_steps[0].blocks.iter().any(|block| block.id == "b1"));
        assert!(!after_steps[1].blocks.iter().any(|block| block.id == "b1"));
    }

    #[semio_framework_async_macros::async_test]
    async fn dispatch_registers_semantic_descriptors() {
        register_playbook_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
        for kind in PlaybookMutation::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(PlaybookMutation::kinds().len(), 9);
    }
    //#endregion 🔖️MutationLaws

    //#region 🔖️OutcomeLaws
    // 26/08/16 MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS — one law test per verb
    // family present in this facet, calling `assert_missing_target_is_error` (landed in
    // `📡️spr/🧪️testkit`). No family in this facet reaches Fatal (playbook's only duplicate-prone
    // family, `add`, treats a duplicate id as Warning `mutation.no-op`, never Fatal), so
    // `assert_fatal_never_applies` has nothing meaningful to exercise here.
    // `assert_outcome_policy_matrix` is NOT landed under that name (only the generic closure-based
    // `assert_policy_matrix` exists) — see this ticket's report.
    #[semio_framework_async_macros::async_test]
    async fn add_family_missing_target_is_error() {
        let base = sample_snapshot();
        assert_missing_target_is_error(&base, &PlaybookMutation::AddBlock(AddBlock { step_id: "missing".into(), block: sample_block("b1", "text", "New"), index: None }));
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_family_missing_target_is_error() {
        let base = sample_snapshot();
        assert_missing_target_is_error(&base, &PlaybookMutation::RemoveStep(RemoveStep { step_id: "missing".into() }));
        assert_missing_target_is_error(&base, &PlaybookMutation::RemoveBlock(RemoveBlock { step_id: "missing".into(), block_id: "b1".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_family_missing_target_is_error() {
        let base = sample_snapshot();
        assert_missing_target_is_error(&base, &PlaybookMutation::MoveStep(MoveStep { step_id: "missing".into(), index: 0 }));
        assert_missing_target_is_error(&base, &PlaybookMutation::MoveBlock(MoveBlock { block_id: "b1".into(), from_step_id: "missing".into(), to_step_id: "s2".into(), index: 0 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn replace_family_missing_target_is_error() {
        let base = sample_snapshot();
        assert_missing_target_is_error(&base, &PlaybookMutation::ReplaceBlock(ReplaceBlock { step_id: "missing".into(), block: sample_block("b1", "text", "New") }));
    }

    #[semio_framework_async_macros::async_test]
    async fn update_family_missing_target_is_error() {
        let base = sample_snapshot();
        assert_missing_target_is_error(&base, &PlaybookMutation::UpdateStep(UpdateStep { step_id: "missing".into(), title: "x".into(), description: None }));
    }
    //#endregion 🔖️OutcomeLaws
}
//#endregion 🧪️Tests
