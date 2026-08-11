//! 🧬️ DocxSnapshot schema (ecma-376/✳️strict) — reuses the ✳️any subset's `DocxSnapshot`
//! verbatim (the SAME Rust type, same `s.stdio.docx` schema id). ISO/IEC 29500-1:2016 Strict is a
//! validation-gated dialect STAMP on top of that existing schema, not a new one -- see D4's Tier-1
//! "same snapshot type, subset moves" semantics (`ArtifactCommand::MigrateDialect`). This leaf
//! exists so `🪆️subsets/✳️strict/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`,
//! without duplicating the schema definition.

pub use crate::artifacts::docx::standards::v_ecma_376::subsets::any::schema::*;
