//! 🧬️ PdfSnapshot schema (1.4/✳️a) — reuses the ✳️any subset's `PdfSnapshot` verbatim (the SAME
//! Rust type, same `s.stdio.pdf` schema id). A subset is a validation-gated dialect STAMP on top
//! of that existing schema, not a new one -- see D4's Tier-1 "same snapshot type, subset moves"
//! semantics (`ArtifactCommand::MigrateDialect`). This leaf exists so `🪆️subsets/✳️a/🧬️schema/`
//! is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without duplicating the schema definition.
//!
//! Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES: PDF 1.4's `PdfSnapshot` is a bare
//! `PageDoc{width,height,text}` -- no retained object graph -- so `🧐️analyzer` here implements
//! only what's honestly checkable from those fields, plus a SOFT schema-gap diagnostic. See
//! `🧐️analyzer` for the full honesty accounting.

pub use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::*;
