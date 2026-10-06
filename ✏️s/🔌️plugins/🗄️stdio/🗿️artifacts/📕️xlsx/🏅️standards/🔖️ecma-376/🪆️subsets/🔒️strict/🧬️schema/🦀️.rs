//! 🧬️ XlsxSnapshot schema (ecma-376/🔒️strict) — reuses the 🧱️base subset's `XlsxSnapshot`
//! verbatim (the SAME Rust type, same `s.stdio.xlsx` schema id). ISO/IEC 29500-1 Strict
//! conformance is a validation-gated dialect STAMP on top of that existing schema, not a new one
//! (D4's Tier-1 "same snapshot type, subset moves" semantics — `ArtifactCommand::MigrateDialect`).
//! This leaf exists so `🪆️subsets/🔒️strict/🧬️schema/` is present per `🔣️taxonomy.json`'s
//! `subsetChildDirs`, without duplicating the schema definition.

pub use crate::standards::v_ecma_376::subsets::base::schema::*;
//#region 🧬️Mutations
// 🧬️ This subset's OWN conformance-class vocabulary, mounted here rather than in the crate's shared
// `🦀️.rs`: that file is one wiring file for every stdio artifact at once, and the rationale the
// 🧱️base subset already records for its own test mount — leave the shared file alone, let an artifact
// own the subtree it owns — applies to a production leaf of this subset just as well. `#[path]` on a
// non-inline module resolves against this file's own directory. The explicit declaration shadows the
// glob re-export of 🧱️base's `mutations` above, which is what puts this subset's own vocabulary at
// `subsets::strict::schema::mutations` while 🧱️base's document vocabulary stays reachable at its own
// address.
#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
//#endregion 🧬️Mutations

//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
