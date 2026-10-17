//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed
//! independently of this repository's own codec so the subject has something real to be compared
//! against instead of being checked against its own reading.
//!
//! A DOCX is a ZIP of XML parts (ECMA-376 Part 2, Open Packaging Conventions), and this module
//! composes two genuinely independent third-party crates to read and write one for real: `zip`
//! unzips/rezips the OPC container, `quick-xml` parses/edits/re-serializes every OOXML part
//! (`word/document.xml`, `word/styles.xml`, `[Content_Types].xml`, every `*.rels`). This file never
//! imports `semio-s-plugin-stdio` — the OPC layer and the WordprocessingML block-tree layer are
//! reimplemented here, independently, against the two reference crates directly, mirroring the
//! shape `📰️xml/…/🔖️1.0/…/🦀️oracle.rs` (the quick-xml half) and
//! `🎒️zip/…/🔖️2.0/…/🦀️oracle.rs` (the zip half) each already establish on their own.
//!
//! The vocabulary is per SUBSET, not per artifact: `📕️xlsx`/`🎞️pptx` are also ECMA-376 ZIP+XML
//! packages but declare their own mutation vocabularies and own their own oracle modules —
//! untouched here.
//!
//! Two entry points: [`oracle_apply_mutation`] performs the FORWARD mutation (the `mutate-<kind>`
//! scenarios), [`oracle_apply_mutation_inverse`] performs the forward mutation and then its computed
//! inverse in sequence (the `inverse-<kind>` scenarios) — the same "apply, then apply the inverse,
//! land back on the start" law `DocxMutation::inverse` proves at the Rust-model level, proven here
//! independently against the two registered reference libraries. [`project_docx_ecma_376`] is the
//! shared independent-reader projection both this module's own handlers AND the case's subject
//! handlers read their results back through before comparison.
//!
//! `quick-xml` 0.42 splits every `&entity;`/`&#NNN;` reference out of `Text` into its own
//! `Event::GeneralRef`, so a text run is accumulated across `Text`/`GeneralRef` events rather than
//! read as one event — see [`oracles::resolve_general_ref`], same technique the XML 1.0 oracle uses.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`DocxMutation::KINDS`).

use semio_repo_test_host::Json;

//#region 🔖️Vocabulary
/// 🧾️ Kebab-case spelling of every variant this subset's `DocxMutation` declares, in declaration
/// order. The `docx-ecma-376-any` catalog is measured against this exact list, and the
/// production-side `kinds_const_matches_enum_variants_in_declaration_order` proves enum, constant
/// and manifest never drift apart. Declared here rather than in the case adapter so the adapter,
/// this module's own law tests and the manifest all read ONE list.
pub const KINDS: &[&str] = &["insert-block", "remove-block", "set-block-content", "set-run-text", "replace-xml-node", "set-run-formatting", "insert-style", "remove-style", "set-style-name", "set-style-based-on", "set-part", "remove-part", "set-relationship", "remove-relationship", "set-content-type", "remove-content-type"];
//#endregion 🔖️Vocabulary

//#region 🔖️Oracles
#[path = "."]
mod oracles {
    use quick_xml::escape::resolve_xml_entity;
    use quick_xml::events::{BytesDecl, BytesEnd, BytesRef, BytesStart, BytesText, Event};
    use quick_xml::reader::Reader;
    use quick_xml::writer::Writer;
    use quick_xml::XmlVersion;
    use semio_repo_test_host::{digest, Json};
    use std::collections::HashMap;
    use std::io::{Cursor, Read, Write};

    #[path = "🌳️xml-tree-and-opc/🦀️.rs"]
    mod xml_tree_and_opc;
    use xml_tree_and_opc::*;

    #[path = "📝️wordprocessing-and-package/🦀️.rs"]
    mod wordprocessing_and_package;
    use wordprocessing_and_package::*;

    #[path = "🧮️json-values-and-addressing/🦀️.rs"]
    mod json_values_and_addressing;
    use json_values_and_addressing::*;

    #[path = "🔁️forward-inverse-and-routing/🦀️.rs"]
    mod forward_inverse_and_routing;
    pub use forward_inverse_and_routing::*;
}
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

/// ↩️ Applies one declared mutation kind and then its own computed inverse, in sequence, proving the
/// same `apply(inverse(m, base), apply(m, base)) == base` law `DocxMutation::inverse` proves at the
/// Rust-model level, here against the registered reference libraries instead.
pub fn oracle_apply_mutation_inverse(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if kind.is_empty() {
        return Err("mutation spec carries no `kind`".to_string());
    }
    let params = spec.get("params").cloned().unwrap_or(Json::Null);
    oracles::apply_mutation_inverse(input, &kind, &params)
}

/// 👁️ This subset's own semantic projection. @see [`oracles::project`].
pub fn project_docx_ecma_376(bytes: &[u8]) -> Result<Json, String> {
    oracles::project(bytes)
}

/// 🔄️ The package read and re-written by the reference alone — its side of the identity law.
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    oracles::round_trip(input)
}

//#endregion 🔖️Dispatch

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
