//! 🧬️ PdfSnapshot schema (1.7/✳️a) — reuses the ✳️any subset's `PdfSnapshot` verbatim (the SAME
//! Rust type, same `s.stdio.pdf.1.7` schema id). PDF/A (ISO 19005 parts 2 and 3) is a
//! validation-gated dialect STAMP on top of that existing schema, not a new one -- see D4's
//! Tier-1 "same snapshot type, subset moves" semantics (`ArtifactCommand::MigrateDialect`). This
//! leaf exists so `🪆️subsets/✳️a/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`,
//! without duplicating the schema definition.
//!
//! W2 restructure (ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES): this subset was
//! previously `✳️a-2b`, conflating the PDF/A *family* ("a") with a specific conformance *level*
//! ("2b"). The subset id is now just `a`; the level (2b/2u/3b/3u) is analyzer-DETECTED DATA
//! reported as a diagnostic (`stdio.pdf.a.level`), never part of the dialect id -- see
//! `🧐️analyzer`'s `detect_pdfa_level`.

pub use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::*;
