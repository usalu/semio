//! 🧬️ XmlSnapshot schema (1.0/✳️valid) — reuses the ✳️any subset's `XmlSnapshot` verbatim (the
//! SAME Rust type, same `s.stdio.xml` schema id). W3C XML 1.0 Fifth Edition §5.1 validity is a
//! validation-gated dialect STAMP on top of that existing schema, not a new one -- see D4's
//! Tier-1 "same snapshot type, subset moves" semantics (`ArtifactCommand::MigrateDialect`). This
//! leaf exists so `🪆️subsets/✳️valid/🧬️schema/` is present per `🔣️taxonomy.json`'s
//! `subsetChildDirs`, without duplicating the schema definition.

pub use crate::artifacts::xml::standards::v1_0::subsets::any::schema::*;
