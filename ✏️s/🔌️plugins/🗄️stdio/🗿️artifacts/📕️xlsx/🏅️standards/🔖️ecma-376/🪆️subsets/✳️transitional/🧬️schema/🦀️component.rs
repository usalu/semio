//! 🧬️ XlsxSnapshot schema (ecma-376/✳️transitional) — reuses the ✳️any subset's `XlsxSnapshot`
//! verbatim (the SAME Rust type, same `s.stdio.xlsx` schema id). ISO/IEC 29500-4 Transitional
//! conformance is a validation-gated dialect STAMP on top of that existing schema, not a new one
//! (D4's Tier-1 "same snapshot type, subset moves" semantics — `ArtifactCommand::MigrateDialect`).
//! This leaf exists so `🪆️subsets/✳️transitional/🧬️schema/` is present per `🔣️taxonomy.json`'s
//! `subsetChildDirs`, without duplicating the schema definition.

pub use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::*;
