//! 🧬️ playbook artifact — semantic parent-lane mutation dispatch enum. Every variant is a
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
/// 🧮️ Semantic playbook PARENT-lane mutation vocabulary: the playbook's own title scalar. Steps and blocks are composed content
/// of the `flow` child and are edited only on that child's lane (stdio flow leaves, design §20.15 of ticket
/// 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING; see the artifact root's `🔖️ChildLane`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = PlaybookSnapshot, diff = PlaybookDiff, schema = "playbook.playbook")]
pub enum PlaybookMutation {
    ChangeTitle(ChangeTitle),
}
//#endregion 🔖️Mutations

pub use super::change_title::{change_title_operation, ChangeTitle};


/// ↩️ Computes `mutation`'s inverse from the pre-state `snapshot`.
pub fn inverse_playbook_mutation(snapshot: &PlaybookSnapshot, mutation: &PlaybookMutation) -> Result<Vec<PlaybookMutation>, semio_framework_value::ValueError> {
    Ok({
    protocol::Mutation::inverse(mutation, snapshot)?

    })
}

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every [`PlaybookMutation`] variant, in declaration order — the vocabulary the
/// `playbook-1-any` mutation catalog (`../../🔮️oracles/🔣️.json`) declares and the exhaustive `mutate-*` case measures itself
/// against. The framework never parses Rust, so `kinds_match_the_enum_and_the_catalog` keeps this list honest against both the
/// enum and the committed catalog.
pub const KINDS: &[&str] = &["change-title"];


/// ↩️ `mutation`'s own inverse against `base`, as the step LIST `protocol::Mutation::inverse`
/// returns. Reachable from outside this crate, which `protocol::Mutation` itself is not — the
/// `protocol` extern-crate alias is private to `🦀️.rs`.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn inverse_playbook_mutation_steps(mutation: &PlaybookMutation, base: &PlaybookSnapshot) -> Result<Vec<PlaybookMutation>, semio_framework_value::ValueError> {
    Ok({
    <PlaybookMutation as protocol::Mutation<PlaybookSnapshot>>::inverse(mutation, base)?

    })
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
