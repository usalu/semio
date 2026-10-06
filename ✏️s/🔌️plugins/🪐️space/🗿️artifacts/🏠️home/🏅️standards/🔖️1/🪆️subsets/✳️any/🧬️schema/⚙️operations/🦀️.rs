//! ⚙️ S Home mutation codec bridge, catalog identity, and behavior tests. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/`
//! direct leaves); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<SHomeSnapshot>`
//! and `impl protocol::SemanticMutation<SHomeSnapshot>` from that payload — no hand-written
//! apply/diff/inverse dispatch here. Whole-document replace (the old `SetSnapshot`) is banned; it
//! goes through `ArtifactStore::reset` (non-history), never through this enum.

use crate::standards::v1::subsets::any::schema::mutations::SHomeMutation;
#[cfg(test)]
use crate::standards::v1::subsets::any::schema::mutations::{change_catalog_generation, register_s_home_mutation_descriptors};
use crate::SHomeSnapshot;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `SHomeMutation` variant, in declaration order — the vocabulary the `s-home-1-any` mutation catalog
/// (`../../🔣️oracle.json`) declares and the `🌵️mutate-s-home-1` exhaustive test case measures
/// itself against. The framework never parses Rust, so `kinds_match_the_enum_and_the_catalog` below is
/// what keeps this list honest in both directions.
pub const KINDS: &[&str] = &["change-catalog-generation"];
//#endregion 🔖️Kinds

//#region 🌉️TestBridge

//#endregion 🌉️TestBridge

//#region 🧪️KindsConformance
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-conformance/🦀️.rs"]
mod kinds_conformance;
//#endregion 🧪️KindsConformance
