//! 🧬️ StepSnapshot schema (ap214/2️⃣cc2) — reuses the 🧱️base subset's `StepSnapshot` verbatim
//! (the SAME Rust type, same `stdio.step` schema id). ISO 10303-214 CC2 (bounded wireframe/basic surfaces) is a validation-gated
//! dialect STAMP on top of that existing schema, not a new one — see D4's Tier-1 "same snapshot
//! type, subset moves" semantics (`ArtifactCommand::MigrateDialect`). This leaf exists so
//! `🪆️subsets/2️⃣cc2/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without
//! duplicating the schema definition.

pub use crate::standards::v_ap214::subsets::base::schema::*;

//#region 🧬️Mutations
/// 🧬️ This subset's OWN mutation vocabulary — one kind per ISO 10303-214 CC2 (bounded wireframe/basic surfaces) conformance
/// rule, derived from `check_cc2_conformance` below rather than copied from a sibling class, and
/// NOT the `🧱️base` subset's generic ISO 10303-21 graph editing. The module re-exports `🧱️base`'s
/// `StepMutation`/`apply_step_mutation` as well, since this explicit declaration shadows the glob
/// re-export those names used to arrive through.
#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
//#endregion 🧬️Mutations
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
