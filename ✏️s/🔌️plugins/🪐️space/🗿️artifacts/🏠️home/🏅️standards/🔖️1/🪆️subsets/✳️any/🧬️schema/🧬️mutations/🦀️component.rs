//! 🧬️ S Home artifact — semantic document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/`
//! triad leaves); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<SHomeSnapshot>`
//! and `impl protocol::SemanticMutation<SHomeSnapshot>` from that payload — no hand-written
//! apply/diff/inverse dispatch here. Whole-document replace (the old `SetSnapshot`) is banned; it
//! goes through `ArtifactStore::reset` (non-history), never through this enum.

use crate::artifacts::home::diff::SHomeDiff;
use crate::artifacts::home::SHomeSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 🧮️ Semantic S Home launcher mutation vocabulary: the single root scalar mutable field
/// (`catalog_generation`, the counter that forces a studio-list re-materialize).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = SHomeSnapshot, diff = SHomeDiff, schema = "s.space.home")]
pub enum SHomeMutation {
    ChangeCatalogGeneration(ChangeCatalogGeneration),
}
//#endregion 🔖️Mutations

pub use super::change_catalog_generation::mutation::{change_catalog_generation, ChangeCatalogGeneration};

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&change_catalog_generation(7));
    }

    #[test]
    fn dispatch_registers_semantic_descriptors() {
        register_s_home_mutation_descriptors();
        for kind in <SHomeMutation as protocol::SemanticMutation<SHomeSnapshot>>::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(<SHomeMutation as protocol::SemanticMutation<SHomeSnapshot>>::kinds().len(), 1);
    }
}
//#endregion 🧪️Tests
