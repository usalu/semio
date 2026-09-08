//! ⚙️ Sourcing curation mutation codec bridge, catalog identity, and behavior tests.
//!
//! Derived from `CurationSnapshot`'s shape per `📓️derivation-rules.md` rule 2: `curated` is the one
//! genuinely id-keyed, user-editable collection (the curation selection — which objects, how many
//! units of each) addressed by `object_id`. `stock` is deliberately NOT represented in this enum:
//! it is a bulk-populated reference catalogue (seeded from
//! `crate::schema::sourcing_modules("[]")`/hot-installed `sourcing.module`
//! contributions), never hand-authored item-by-item by a user — whole-catalogue population goes
//! through `store::ArtifactStore::reset` (see `crate::apps::curation::reset_document_effect`), same
//! non-history path as whole-document replace, never through this mutation enum. `CuratedItem` has
//! no name/key field beyond `object_id` (no `rename`) and no `Vec` member fields (no
//! `add-`/`remove-curated-item-*`), so the closed vocabulary is exactly the three id-keyed
//! collection verbs the schema supports: `create`, `delete`, `change` (count). The pre-migration
//! whole-document-replace variant (the former whole-snapshot-replace enum case) is gone with NO replacement per
//! `📓️taxonomy.md`/`📓️derivation-rules.md` rule 6.
//!
//! The three semantic payloads are mounted from their direct mutation leaves in `🦀️.rs`.

use crate::schema::mutations::SourcingMutation;
use crate::CurationSnapshot;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("../🧬️mutations/📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

/// 🏷️ Kebab-case spelling of every [`SourcingMutation`] variant, in declaration order — the
/// vocabulary the `curation-1-any` mutation catalog (`../../🔣️oracle.json`) declares and
/// `🗂️mutate-curation-1`'s exhaustive case measures itself against. Three kinds and no more: `stock` is
/// a bulk-populated reference catalogue that reaches the document through
/// `ArtifactStore::reset`, `CuratedItem` carries no name and no nested collection, and
/// whole-document replace was removed with no replacement — so `create`/`delete`/`change` over the
/// one id-keyed collection is the entire closed vocabulary this schema supports.
/// [`kinds_match_the_enum_and_the_catalog`] keeps this list honest against the enum, since the
/// framework never parses Rust.
pub const KINDS: &[&str] = &["create-curated-item", "delete-curated-item", "change-curated-item-count"];

//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes this facet's internally-tagged (`{"mutation": "createCuratedItem", …}`, camelCase
/// payload fields) JSON projection — exactly the shape the committed
/// `<slug>/🧪️tests/<fixture>/🦠️mutation/🔣️.json` specification vectors carry — into a real
/// [`SourcingMutation`]. The test adapter cannot reach `serde_json` (the generated host links only
/// `semio-repo-test-host` and this crate) and cannot name this crate's private `protocol`/`store`
/// extern-crate aliases either, so the bridge belongs here rather than there.
pub fn decode_sourcing_mutation_json(text: &str) -> Result<SourcingMutation, String> {
    dsl::json::from_json_str(text).map_err(|error| error.to_string())
}

/// ▶️ Applies `mutation` in place and returns every diagnostic it raised as `(code, severity)`
/// pairs, so the committed `🎯️outcome/🔣️.json`'s claim is checkable from outside this
/// crate rather than only inside its own leaf tests.
pub fn apply_sourcing_mutation_reporting(snapshot: &mut CurationSnapshot, mutation: &SourcingMutation) -> Vec<(String, String)> {
    let outcome = <SourcingMutation as protocol::Mutation<CurationSnapshot>>::diff(mutation, snapshot).apply_to(snapshot);
    outcome.messages().iter().map(|message| (message.code.0.clone(), format!("{:?}", message.level))).collect()
}

/// ↩️ The mutation's OWN computed undo steps, which is what an `inverse-<kind>` scenario has to
/// apply for the metamorphic law to mean anything.
pub fn inverse_sourcing_mutation_steps(mutation: &SourcingMutation, base: &CurationSnapshot) -> Vec<SourcingMutation> {
    <SourcingMutation as protocol::Mutation<CurationSnapshot>>::inverse(mutation, base)
}
//#endregion 🌉️ExternalCodecBridge

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
