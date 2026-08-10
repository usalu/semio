//! 🧬️ PdfSnapshot schema (1.7/✳️a-2b) — reuses the ✳️any subset's `PdfSnapshot` verbatim (the
//! SAME Rust type, same `s.stdio.pdf.1.7` schema id). PDF/A-2b is a validation-gated dialect
//! STAMP on top of that existing schema, not a new one -- see D4's Tier-1 "same snapshot type,
//! subset moves" semantics (`ArtifactCommand::MigrateDialect`). This leaf exists so
//! `🪆️subsets/✳️a-2b/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without
//! duplicating the schema definition.

pub use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::*;
