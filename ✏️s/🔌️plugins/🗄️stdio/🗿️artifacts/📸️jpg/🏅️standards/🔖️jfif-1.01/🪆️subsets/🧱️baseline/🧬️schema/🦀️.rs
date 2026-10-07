//! 🧬️ JpgSnapshot schema (jfif-1.01/🧱️baseline) — reuses the 🧾️document subset's `JpgSnapshot`
//! verbatim (the SAME Rust type, same `s.stdio.jpg` schema id). ITU-T T.81/ISO 10918-1 baseline
//! sequential DCT conformance (in a JFIF 1.01 container) is a validation-gated dialect STAMP on
//! top of that existing schema, not a new one -- see D4's Tier-1 "same snapshot type, subset
//! moves" semantics (`ArtifactCommand::MigrateDialect`). This leaf exists so
//! `🪆️subsets/🧱️baseline/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without
//! duplicating the schema definition.

pub use crate::standards::v_jfif_1_01::subsets::document::schema::*;
//#region 🧬️Mutations
// 🧬️ This subset's OWN conformance-class vocabulary, mounted here rather than in the crate's shared
// `🦀️.rs` — the same placement, and the same rationale, the ✳️strict/✳️transitional OOXML
// subsets already use for theirs: that file is one wiring file for every stdio artifact at once,
// and an artifact owns the subtree it owns. `#[path]` on a non-inline module resolves against this
// file's own directory. The explicit declaration shadows the glob re-export of 🧾️document's `mutations`
// above, which is what puts this subset's own vocabulary at
// `subsets::baseline::schema::mutations` while 🧾️document's document vocabulary stays reachable at its
// own address.
#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
//#endregion 🧬️Mutations
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

#[path="🏅️conformance/🦀️.rs"]
pub mod conformance;
pub use conformance::*;
