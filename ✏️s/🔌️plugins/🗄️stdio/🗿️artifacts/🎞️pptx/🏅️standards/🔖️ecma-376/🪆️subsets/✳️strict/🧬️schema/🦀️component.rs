//! 🧬️ PptxSnapshot schema (ecma-376/✳️strict) — reuses the ✳️any subset's `PptxSnapshot` verbatim
//! (the SAME Rust type, same `s.stdio.pptx` schema id). ISO/IEC 29500-1:2016 Strict is a
//! validation-gated dialect STAMP on top of that existing schema, not a new one -- see D4's
//! Tier-1 "same snapshot type, subset moves" semantics (`ArtifactCommand::MigrateDialect`). This
//! leaf exists so `🪆️subsets/✳️strict/🧬️schema/` is present per `🔣️taxonomy.json`'s
//! `subsetChildDirs`, without duplicating the schema definition.
//!
//! Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES: real ISO/IEC 29500-1 Strict
//! conformance-class subset, same shared pattern as `📜️docx`/`📕️xlsx` ecma-376 ✳️strict.

pub use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::*;
