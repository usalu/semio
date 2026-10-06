//! 🧬️ Ifc2x3Snapshot schema (2x3/🏢️cobie) — reuses the ✳️base subset's `Ifc2x3Snapshot` verbatim.
//! Basic FM Handover (carries COBie 2.4) is a validation-gated dialect STAMP, not a new type.

pub use crate::standards::v2x3::subsets::base::schema::*;

//#region 🧬️Mutations
/// 🧬️ This subset's OWN mutation vocabulary — one kind per Basic FM Handover conformance rule, not a copy of
/// the `✳️base` subset's generic Part-21 graph editing. The module re-exports `✳️base`'s
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
