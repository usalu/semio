//! 🧬️ TiffSnapshot schema (6.0/🧱️baseline) — reuses the ✳️any subset's `TiffSnapshot` verbatim
//! (the SAME Rust type, same `s.stdio.tiff` schema id). A subset is a validation-gated dialect
//! STAMP on top of that existing schema, not a new one -- see D4's Tier-1 "same snapshot type,
//! subset moves" semantics (`ArtifactCommand::MigrateDialect`). This leaf exists so
//! `🪆️subsets/🧱️baseline/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without
//! duplicating the schema definition.
//!
//! Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: `TiffSnapshot`
//! now retains the REAL IFD (tag/type/count/value entries) — `Compression`/
//! `PhotometricInterpretation`/`BitsPerSample`/`StripOffsets`/`Tile*` are all genuinely present
//! and checkable. `🧐️analyzer` here now implements real Baseline TIFF conformance checks
//! against those fields (superseding the earlier ticket 26/08/11's schema-gap-only revision).
//! See `🧐️analyzer` for the full accounting.

pub use crate::standards::v6_0::subsets::document::schema::*;
//#region 🧬️Mutations
// 🧬️ This subset's OWN conformance-class vocabulary, mounted here rather than in the crate's shared
// `🦀️.rs` — the same placement, and the same rationale, the ✳️strict/✳️transitional OOXML
// subsets already use for theirs: that file is one wiring file for every stdio artifact at once,
// and an artifact owns the subtree it owns. `#[path]` on a non-inline module resolves against this
// file's own directory. The explicit declaration shadows the glob re-export of ✳️any's `mutations`
// above, which is what puts this subset's own vocabulary at
// `subsets::baseline::schema::mutations` while ✳️any's document vocabulary stays reachable at its
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
