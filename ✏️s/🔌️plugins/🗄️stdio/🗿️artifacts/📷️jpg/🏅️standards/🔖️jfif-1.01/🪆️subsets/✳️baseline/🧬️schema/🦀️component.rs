//! 🧬️ JpgSnapshot schema (jfif-1.01/✳️baseline) — reuses the ✳️any subset's `JpgSnapshot`
//! verbatim (the SAME Rust type, same `s.stdio.jpg` schema id). ITU-T T.81/ISO 10918-1 baseline
//! sequential DCT conformance (in a JFIF 1.01 container) is a validation-gated dialect STAMP on
//! top of that existing schema, not a new one -- see D4's Tier-1 "same snapshot type, subset
//! moves" semantics (`ArtifactCommand::MigrateDialect`). This leaf exists so
//! `🪆️subsets/✳️baseline/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without
//! duplicating the schema definition.

pub use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::schema::*;
