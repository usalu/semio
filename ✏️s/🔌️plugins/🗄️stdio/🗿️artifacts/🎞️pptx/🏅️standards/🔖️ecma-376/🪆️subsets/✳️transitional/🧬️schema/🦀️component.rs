//! 🧬️ PptxSnapshot schema (ecma-376/✳️transitional) — reuses the ✳️any subset's `PptxSnapshot`
//! verbatim (the SAME Rust type, same `s.stdio.pptx` schema id). ISO/IEC 29500-4:2016
//! Transitional is a validation-gated dialect STAMP on top of that existing schema, not a new
//! one -- see D4's Tier-1 "same snapshot type, subset moves" semantics
//! (`ArtifactCommand::MigrateDialect`). This leaf exists so `🪆️subsets/✳️transitional/🧬️schema/`
//! is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without duplicating the schema
//! definition.
//!
//! Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES: real ISO/IEC 29500-4 Transitional
//! conformance-class subset, same shared pattern as `📜️docx`/`📕️xlsx` ecma-376 ✳️transitional.

pub use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::*;
