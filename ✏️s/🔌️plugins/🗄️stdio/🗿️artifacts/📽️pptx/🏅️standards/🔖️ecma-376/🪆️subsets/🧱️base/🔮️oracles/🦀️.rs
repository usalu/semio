//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed
//! independently of this repository's own codec so the subject has something real to be compared
//! against instead of being checked against its own reading.
//!
//! Reference: `zip` (unzip/rezip the OPC container) composed with `quick-xml` (parse/edit/
//! re-serialize `ppt/presentation.xml` + every `ppt/slides/slideN.xml`) — a PPTX is a ZIP of XML
//! parts, and both crates are already linked and genuinely independent of this repository's own
//! codec. This is the same composition the 📰️xml subset's oracle uses for its `quick-xml` half and
//! the 🎒zip subset's for its archive half, written fresh here (PPTX has no shared family module to
//! reach either through).
//!
//! **Design**: every slide and shape kind is a pure edit of one XML part's own parsed tree, addressed by the wire
//! address's part path and child-index node path exactly as the vocabulary defines it: an entry or shape inserted at a
//! vacancy among the `p:sldId` (or shape) siblings, one removed, the slide list reordered, every `a:t` of a shape
//! rewritten, or its first `a:xfrm` repositioned, and `replace-xml-node` swaps one addressed XML node for the wire's replacement. Revisions are the subject's guard and are not
//! re-derived here; a stale row shows as a refused subject result that the comparison then fails. Only the edited parts
//! are re-serialized; the identity round trip still regenerates every slide part from the typed slide/shape list.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared family modules rather than by copying it.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`PptxMutation`'s
//! 9 variants).

use semio_repo_test_host::Json;

//#region 🔖️Vocabulary
/// 🧾️ Kebab-case spelling of every variant this subset's `PptxMutation` declares, in declaration
/// order. The `pptx-ecma-376-base` catalog is measured against this exact list, and the
/// production-side `kinds_matches_enum_variants_and_manifest` proves enum, constant and manifest
/// never drift apart. Declared here rather than in the case adapter so the adapter, this module's
/// own law tests and the manifest all read ONE list.
pub const KINDS: &[&str] = &["insert-slide", "remove-slide", "move-slide", "insert-shape", "remove-shape", "set-shape-text", "set-shape-position", "replace-xml-node", "set-relationship", "remove-relationship", "set-content-type", "remove-content-type"];
//#endregion 🔖️Vocabulary

//#region 🔖️Oracles
#[path = "🧰️reference-implementation/🦀️.rs"]
mod oracles;
//#endregion 🔖️Oracles

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if kind.is_empty() {
        return Err("mutation spec carries no `kind`".to_string());
    }
    let params = spec.get("params").cloned().unwrap_or(Json::Null);
    oracles::apply_mutation(input, &kind, &params)
}

/// ↩️ Applies one declared mutation kind and then its own computed inverse, in sequence, proving
/// the same `apply(inverse(m, base), apply(m, base)) == base` law `PptxMutation::inverse` proves
/// at the Rust-model level, here against the registered reference implementation instead.
pub fn oracle_apply_mutation_inverse(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if kind.is_empty() {
        return Err("mutation spec carries no `kind`".to_string());
    }
    let params = spec.get("params").cloned().unwrap_or(Json::Null);
    oracles::apply_mutation_inverse(input, &kind, &params)
}

/// 🔁️ The identity round trip: decode with the independent reader, re-encode with the reference
/// writer, no mutation applied.
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    oracles::round_trip(input)
}

/// 👁️ This subset's own semantic projection. @see [`oracles::project`].
pub fn project_pptx_mutation(bytes: &[u8]) -> Result<Json, String> {
    oracles::project(bytes)
}

/// 🪢️ The package plumbing (`[Content_Types].xml` entries and every owner's relationships, in the order the package states them). @see [`oracles::project_plumbing`].
pub fn project_pptx_plumbing(bytes: &[u8]) -> Result<Json, String> {
    oracles::project_plumbing(bytes)
}

//#endregion 🔖️Dispatch

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
