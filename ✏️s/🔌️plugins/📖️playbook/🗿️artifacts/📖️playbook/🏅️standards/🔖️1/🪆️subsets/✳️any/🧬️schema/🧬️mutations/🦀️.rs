//! 🧬️ playbook artifact — semantic document mutation dispatch enum. Every variant is a
//! single-field tuple wrapping a handcrafted `protocol::MutationKind` payload (see the
//! `🧬️mutations/<slug>/` triad leaves); `#[derive(dsl::Mutations)]` generates
//! `impl protocol::Mutation<PlaybookSnapshot>` and `impl protocol::SemanticMutation<PlaybookSnapshot>`
//! from those payloads — no hand-written apply/diff/inverse dispatch here.
//!
//! Moved from the framework kernel module
//! (`🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️.rs`) by ticket
//! `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`'s playbook design decision: the dispatch enum cannot stay
//! in the framework and wrap plugin-local payload structs (crate dependency direction — the
//! framework cannot depend on a plugin), so it moves here, matching the other 106 mutation facets.
//! Domain types (`PlaybookStep`/`PlaybookBlock`/`PlaybookExpr`), validation, `generation_forms`, and
//! `builder_kit`'s rendering half stay in the framework kernel (`crate::playbook::*`) — only the
//! mutation vocabulary moved.

use crate::{PlaybookDiff, PlaybookSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
// 🔬️ `Serialize`/`Deserialize` survive ONLY as a `#[cfg(test)]` differential oracle — committed
// `🧪️tests/<fixture>/🦀️.rs` fixture vectors decode/re-encode through them — never a production
// dependency of this crate.
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 🧮️ Semantic playbook document mutation vocabulary: id-keyed step/block add/remove/move, a
/// whole-block replace, a step-header update, and the playbook's own title scalar.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslEnum, dsl::Mutations)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
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
/// `ArtifactBuilder::mutate` (`../🦀️.rs`).
pub fn apply_playbook_mutation(snapshot: &PlaybookSnapshot, mutation: &PlaybookMutation) -> protocol::MutationApplyResult<PlaybookSnapshot> {
    protocol::MutationDiff::apply(protocol::Mutation::diff(mutation, snapshot).diff(), snapshot)
}

/// ↩️ Computes `mutation`'s inverse from the pre-state `snapshot`.
pub fn inverse_playbook_mutation(snapshot: &PlaybookSnapshot, mutation: &PlaybookMutation) -> Vec<PlaybookMutation> {
    protocol::Mutation::inverse(mutation, snapshot)
}

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every [`PlaybookMutation`] variant, in declaration order — the vocabulary the
/// `playbook-1-any` mutation catalog (`../../🔣️oracle.json`) declares and the
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
/// `protocol` extern-crate alias is private to `🦀️.rs`.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn inverse_playbook_mutation_steps(mutation: &PlaybookMutation, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
    <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::inverse(mutation, base)
}

/// 📥️ Decodes the internally-tagged (`{"mutation": "<camelCaseVariant>", …}`) projection the
/// committed `<slug>/🧪️tests/<fixture>/🦠️mutation/🔣️.json` vectors carry.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn decode_playbook_mutation_json(text: &str) -> Result<PlaybookMutation, String> {
    protocol::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📥️ Decodes a committed `📸️snapshot/{⬅️before,➡️after}/🔣️.json` vector.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn decode_playbook_snapshot_json(text: &str) -> Result<PlaybookSnapshot, String> {
    protocol::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📤️ The snapshot as the same canonical JSON the committed vectors are written in — the
/// projection an external test host compares through.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn encode_playbook_snapshot_json(snapshot: &PlaybookSnapshot) -> String {
    protocol::json::to_json_string(snapshot)
}
/// 🌱 Attaches the working scene to this snapshot's exact composed `flow` child handle from a
/// committed `[PlaybookStep]` JSON document, and hands back what it decoded.
///
/// This subset's persisted snapshot holds only the child HANDLE; the live rows behind it are an
/// ephemeral, session-side scene that a fresh process has never populated. A committed
/// `📸️snapshot/⬅️before/🔣️.json` vector is therefore only HALF of a before-state, and the
/// other half lives today in each leaf's own `🧪️tests/<fixture>/🦀️.rs` as a Rust literal.
/// An external conformance host cannot reach that, so this bridge lets the scene half travel as
/// DATA — the exhaustive `🌾️mutate-playbook-1` case carries it in its own `Examples` table, with the leaf
/// it was read from cited there. The right long-term fix is to commit the scene beside the snapshot
/// as a fixture file of its own; until then this is the seam that makes the vectors runnable.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn seed_playbook_scene_json(snapshot: &mut PlaybookSnapshot, steps_json: &str) -> Result<Vec<crate::PlaybookStep>, String> {
    let steps: Vec<crate::PlaybookStep> = protocol::json::from_json_str(steps_json).map_err(|error| error.to_string())?;
    crate::attach_playbook_steps(&mut snapshot.flow, steps.clone());
    Ok(steps)
}
//#endregion 🔖️Kinds

//#region 🧪️KindsCatalog
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
//#endregion 🧪️KindsCatalog

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
