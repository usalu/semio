//! 🧬️ Ifc2x3Snapshot schema (2x3/🤝️cv20) — reuses the ✳️base subset's `Ifc2x3Snapshot` verbatim
//! (same Rust type, same `s.stdio.ifc.2x3` schema id). Coordination View 2.0 is a validation-gated
//! dialect STAMP on top of that existing schema, not a new one -- a subset is a conformance
//! marker, never a fork of the snapshot type (see `🪆️subsets/✳️base/🧬️schema`).

pub use crate::standards::v2x3::subsets::base::schema::*;

//#region 🧬️Mutations
/// 🧬️ This subset's OWN mutation vocabulary — one kind per Coordination View 2.0 conformance rule,
/// not a copy of the `✳️base` subset's generic Part-21 graph editing. The module re-exports `✳️base`'s
/// `Ifc2x3Mutation`/`apply_ifc2x3_mutation` as well, since this explicit declaration shadows the
/// glob re-export those names used to arrive through.
#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
//#endregion 🧬️Mutations
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
