//! 🧬️ ZipSnapshot schema (2.0/🌐️iso21320) — reuses the 🧱️base subset's `ZipSnapshot` verbatim
//! (the SAME Rust type, same `s.stdio.zip` schema id). ISO/IEC 21320-1:2015 (Document Container
//! File, Part 1: Core) is a validation-gated dialect STAMP on top of that existing schema, not a
//! new one -- see D4's Tier-1 "same snapshot type, subset moves" semantics
//! (`ArtifactCommand::MigrateDialect`). This leaf exists so `🪆️subsets/🌐️iso21320/🧬️schema/` is
//! present per `🔣️taxonomy.json`'s `subsetChildDirs`, without duplicating the schema definition.

pub use crate::standards::v2_0::subsets::base::schema::*;

//#region 🧬️Mutations
/// 🧬️ THIS subset's own mutation vocabulary — `ZipIso21320Mutation`, not the `🧱️base` subset's
/// `ZipMutation` the glob re-export above would otherwise supply. Declared here rather than in the
/// crate's module glue so the vocabulary lives with the subset that owns it; the explicit item wins
/// over the glob import, which is exactly the intent.
#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use mutations::{ZipIso21320Method, ZipIso21320Mutation, KINDS as ISO21320_MUTATION_KINDS};
//#endregion 🧬️Mutations
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
