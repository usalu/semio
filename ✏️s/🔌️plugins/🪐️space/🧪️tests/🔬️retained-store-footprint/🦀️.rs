//! 🧮 The one law under every durable `🪐️space` gesture: the footprint a retained one-item
//! preparation DECLARES must cover the staged rows the fold then CHECKS.
//!
//! `ArtifactStore::fold_batch_item` compares `edit.forwards.len() + edit.inverse.len()` against the
//! declared `work_items` and refuses the whole gesture with `batched item candidate failed its exact
//! fixed fold contract` when the declaration is short. Every space mutation is point-invertible, so
//! one item stages TWO rows — and `admit_space_retained_mutation` declared `work_items: 1`, which
//! fail-closed every durable home, studio and space-index gesture in the product. `createStudio`,
//! the local studio path that needs no hub at all, was refused with exactly that sentence on a
//! signed-in shell (ticket 26/09/18 S10 §1). The framework's own generic factory
//! (`bounded_config_store_one_item_preparation_factory`) has always declared
//! `for_one_invertible_item`; this pins the space copy to the same law.
use super::*;
use semio_s_artifact_space_home::{SHomeMutation, SHomeSnapshot};

/// 🧮 The declared footprint of a real home mutation covers the forward row AND the inverse row.
#[test]
fn space_retained_preflight_declares_both_staged_rows_of_a_point_invertible_item() {
    let factory = space_retained_store_preparation::<SHomeSnapshot, SHomeMutation>("space-home-artifact-retained-law", 128 * 1024).expect("space lanes always supply a preparation factory");
    let mutation = semio_s_artifact_space_home::standards::v1::subsets::any::schema::mutations::change_catalog_generation::change_catalog_generation(7);
    let footprint = factory.preflight(&mutation, None, store::HistoryLane::Document).expect("a bounded home mutation is admissible");
    let base = SHomeSnapshot::default();
    let inverse_rows = ::protocol::Mutation::inverse(&mutation, &base).len();
    assert_eq!(inverse_rows, 1, "the home catalog-generation mutation is point-invertible");
    assert!(
        footprint.work_items >= inverse_rows + 1,
        "a declaration of {} work items cannot admit the {} staged rows fold_batch_item counts, so every durable space gesture would be refused 'batched item candidate failed its exact fixed fold contract'",
        footprint.work_items,
        inverse_rows + 1
    );
    assert_eq!(footprint.work_items, store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS, "the space lane declares exactly the framework's own point-invertible budget");
}
