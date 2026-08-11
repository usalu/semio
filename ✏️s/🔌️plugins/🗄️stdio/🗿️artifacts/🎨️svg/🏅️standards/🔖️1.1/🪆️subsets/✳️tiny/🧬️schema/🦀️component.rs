//! 🧬️ SvgSnapshot schema (1.1/✳️tiny) — reuses the ✳️any subset's `SvgSnapshot` verbatim (the
//! SAME Rust type, same `s.stdio.svg` schema id). SVG Tiny 1.1 (W3C Mobile SVG Profiles,
//! REC-SVGMobile-20030114 §SVG Tiny 1.1) is a validation-gated dialect STAMP on top of that
//! existing schema, not a new one -- D4's Tier-1 "same snapshot type, subset moves" semantics
//! (`ArtifactCommand::MigrateDialect`). This leaf exists so `🪆️subsets/✳️tiny/🧬️schema/` is
//! present per `🔣️taxonomy.json`'s `subsetChildDirs`, without duplicating the schema definition.

pub use crate::artifacts::svg::standards::v1_1::subsets::any::schema::*;
