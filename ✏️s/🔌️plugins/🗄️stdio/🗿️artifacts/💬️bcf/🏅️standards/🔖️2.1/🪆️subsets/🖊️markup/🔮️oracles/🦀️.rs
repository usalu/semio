//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed
//! independently of this repository's own codec so the subject has something real to be compared
//! against instead of being checked against its own reading.
//!
//! Reference: `zip` 6 (the bcfzip container itself) composed with `quick-xml` 0.42 (every XML part
//! inside it: `bcf.version`, `markup.bcf`, `.bcfv`). A BCF file IS a ZIP of XML markup, viewpoint
//! and snapshot files — no standalone BCF crate exists in the Rust ecosystem (BCF support only
//! appears bundled inside much larger MPL-licensed IFC toolkits), so composing two already-linked,
//! genuinely independent crates over the real BCF-XML 2.1 `markup.xsd`/`visinfo.xsd` shapes IS the
//! right oracle here, not a gap to fill with a weaker substitute. Every type, parser and writer
//! below is a fresh, independent implementation of that shape — it never imports
//! `semio-s-plugin-stdio`'s own `BcfSnapshot`/`XmlNode`/`ZipEntry` types (see this crate's own
//! purity gate).
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared family modules rather than by copying it. BCF has no shared family helper (unlike
//! `document`/`raster`/`archive`/...): the `zip`+`quick-xml` composition it needs is specific to
//! this one subset's container shape, so it lives here rather than in a shared module.
//!
//! Two entry points mirror the `📰️xml`/`🎨️svg` precedent: [`oracle_apply_mutation`] performs the
//! FORWARD mutation (the `mutate-<kind>` scenarios), [`oracle_apply_mutation_inverse`] performs the
//! forward mutation and then its computed inverse in sequence (the `inverse-<kind>` scenarios) —
//! the same "apply, then apply the inverse, land back on the start" law `BcfMutation::inverse`
//! proves at the Rust-model level, proven here independently against the registered reference
//! libraries. [`project_bcf_2_1`] is the shared independent-reader projection both this module's own
//! handlers AND the case's subject handlers read their results back through before comparison.
//!
//! Mutation params are the leaf wire payload (design §11): binary payloads (a viewpoint's PNG snapshot, a raw retained
//! part's `data`) travel as byte arrays, a whole document as the `BcfSnapshot` wire.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`BcfMutation::KINDS`).

use semio_repo_test_host::Json;

//#region 🔖️Oracles
#[path = "🧰️engine/🦀️.rs"]
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
    oracles::apply_mutation(input, &kind, &spec.get("params").cloned().unwrap_or(Json::Null))
}

/// ↩️ Applies one declared mutation kind and then its own computed inverse, in sequence, proving the
/// same `apply(inverse(m, base), apply(m, base)) == base` law `BcfMutation::inverse` proves at the
/// Rust-model level, here against the registered `zip`+`quick-xml` reference composition instead.
pub fn oracle_apply_mutation_inverse(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if kind.is_empty() {
        return Err("mutation spec carries no `kind`".to_string());
    }
    oracles::apply_mutation_inverse(input, &kind, &spec.get("params").cloned().unwrap_or(Json::Null))
}

/// 👁️ This subset's own semantic projection. @see [`oracles::project`].
pub fn project_bcf_2_1(bytes: &[u8]) -> Result<Json, String> {
    oracles::project(bytes)
}

/// 🔁️ @see [`oracles::round_trip`].
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    oracles::round_trip(input)
}

//#endregion 🔖️Dispatch
