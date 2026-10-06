//! 🧬️ XmlSnapshot schema (1.0/✳️valid) — reuses the ✳️any subset's `XmlSnapshot` verbatim (the
//! SAME Rust type, same `s.stdio.xml` schema id). W3C XML 1.0 Fifth Edition §5.1 validity is a
//! validation-gated dialect STAMP on top of that existing schema, not a new one -- see D4's
//! Tier-1 "same snapshot type, subset moves" semantics (`ArtifactCommand::MigrateDialect`). This
//! leaf exists so `🪆️subsets/✅️valid/🧬️schema/` is present per `🔣️taxonomy.json`'s
//! `subsetChildDirs`, without duplicating the schema definition.

pub use crate::standards::v1_0::subsets::base::schema::*;

//#region 🧬️Mutations
/// 🧬️ THIS subset's own mutation vocabulary — `XmlValidMutation`, not the `✳️any` subset's
/// `XmlMutation` the glob re-export above would otherwise supply. Declared here rather than in the
/// crate's module glue so the vocabulary lives with the subset that owns it; the explicit item wins
/// over the glob import, which is exactly the intent. Its own gate (a `SetSnapshot` that would land
/// a hard §2.8 violation is refused outright) is tested inside that module;
/// `derived_construction`'s tests below cover the SECOND, independent layer — `build()`, which
/// catches a snapshot that arrived around the vocabulary entirely.
#[path = "🧬️mutations/🦀️.rs"]
pub mod valid_mutations;
pub use valid_mutations::{apply_xml_valid_mutation, XmlValidMutation, KINDS as VALID_MUTATION_KINDS};

/// 🆕️ A new valid xml document: `<!DOCTYPE root><root/>` as the real parser reads it — XML 1.0 §5.1 validity needs a DOCTYPE
/// whose Name is the document element's; the empty document has neither, and this subset refuses every edit (and every
/// undo) that would land on an invalid document, so an empty new document could never be edited.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn blank_valid_xml_snapshot() -> crate::XmlSnapshot {
    let doc = crate::schema::snapshot::xml_document_from_text("<!DOCTYPE root><root/>").expect("blank_valid_xml_snapshot: the minimal valid document parses");
    crate::XmlSnapshot { doc, ..crate::XmlSnapshot::default() }
}
//#endregion 🧬️Mutations

//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
