//! 🧬️ SvgSnapshot schema (1.1/🔬️tiny) — reuses the ✳️any subset's `SvgSnapshot` verbatim (the
//! SAME Rust type, same `s.stdio.svg` schema id). SVG Tiny 1.1 (W3C Mobile SVG Profiles,
//! REC-SVGMobile-20030114 §SVG Tiny 1.1) is a validation-gated dialect STAMP on top of that
//! existing schema, not a new one -- D4's Tier-1 "same snapshot type, subset moves" semantics
//! (`ArtifactCommand::MigrateDialect`). This leaf exists so `🪆️subsets/🔬️tiny/🧬️schema/` is
//! present per `🔣️taxonomy.json`'s `subsetChildDirs`, without duplicating the schema definition.

pub use crate::standards::v1_1::subsets::base::schema::*;

//#region 🧬️Mutations
/// 🧬️ THIS subset's own mutation vocabulary — `SvgTinyMutation`, not the `✳️any` subset's
/// `SvgMutation` the glob re-export above would otherwise supply. Declared here rather than in the
/// crate's module glue so the vocabulary lives with the subset that owns it; the explicit item wins
/// over the glob import, which is exactly the intent.
#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
#[cfg(test)]
pub use mutations::{apply_svg_tiny_mutation};
pub use mutations::{SvgTinyMutation, KINDS as TINY_MUTATION_KINDS};
//#endregion 🧬️Mutations
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

#[path="🏅️conformance/🦀️.rs"]
pub mod conformance;
