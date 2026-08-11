//! 🧬️ ZipSnapshot schema (2.0/✳️iso21320) — reuses the ✳️any subset's `ZipSnapshot` verbatim
//! (the SAME Rust type, same `s.stdio.zip` schema id). ISO/IEC 21320-1:2015 (Document Container
//! File, Part 1: Core) is a validation-gated dialect STAMP on top of that existing schema, not a
//! new one -- see D4's Tier-1 "same snapshot type, subset moves" semantics
//! (`ArtifactCommand::MigrateDialect`). This leaf exists so `🪆️subsets/✳️iso21320/🧬️schema/` is
//! present per `🔣️taxonomy.json`'s `subsetChildDirs`, without duplicating the schema definition.

pub use crate::artifacts::zip::standards::v2_0::subsets::any::schema::*;
