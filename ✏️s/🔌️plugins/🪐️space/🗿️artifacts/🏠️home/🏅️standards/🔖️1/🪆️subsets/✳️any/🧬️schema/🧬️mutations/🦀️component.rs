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

    #[semio_framework_async_macros::async_test]
    async fn home_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&change_catalog_generation(7));
    }

    #[semio_framework_async_macros::async_test]
    async fn dispatch_registers_semantic_descriptors() {
        register_s_home_mutation_descriptors();
        for kind in <SHomeMutation as protocol::SemanticMutation<SHomeSnapshot>>::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(<SHomeMutation as protocol::SemanticMutation<SHomeSnapshot>>::kinds().len(), 1);
    }

    //#region 🔖️MutationLaws
    #[semio_framework_async_macros::async_test]
    async fn change_catalog_generation_inverse_law() {
        let base = SHomeSnapshot::default();
        protocol::testkit::assert_mutation_inverse_law(&base, &change_catalog_generation(7));
    }

    #[semio_framework_async_macros::async_test]
    async fn change_catalog_generation_diff_absorb_law() {
        use protocol::Mutation;
        let base = SHomeSnapshot::default();
        let d1 = change_catalog_generation(3).diff(&base).diff().clone();
        let mid = protocol::MutationDiff::apply(&d1, &base).expect("valid mutation diff");
        let d2 = change_catalog_generation(9).diff(&mid).diff().clone();
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🔖️MutationLaws

    // 🧪️OutcomeLaws — no `assert_missing_target_is_error`/`assert_fatal_never_applies` case applies:
    // this facet's one mutation kind (`change-catalog-generation`) is a root scalar counter setter
    // with no addressable target and no domain invariant to violate — it can only succeed or be a
    // `mutation.no-op` (see the leaf's own `🔺️diff` for that check). `assert_outcome_policy_matrix`
    // is also not yet landed in `📡️spr/🧪️testkit` — TODO(1-D testkit laws pending) once it lands.
}
//#endregion 🧪️Tests
