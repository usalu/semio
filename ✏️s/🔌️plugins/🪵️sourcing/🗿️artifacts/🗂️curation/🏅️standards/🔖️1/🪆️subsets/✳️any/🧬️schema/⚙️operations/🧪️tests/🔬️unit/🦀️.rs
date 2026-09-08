
use super::*;
use crate::mutations::{ChangeCuratedItemCount, CreateCuratedItem, DeleteCuratedItem};

use crate::CuratedItem;
use protocol::Mutation;

/// 🏷️ The three declarations of this vocabulary — the enum, [`KINDS`] and the committed catalog
/// — must agree, in spelling AND in order. The framework never parses Rust, so without this test
/// `KINDS` could drift from the enum and the catalog could keep measuring `🗂️mutate-curation-1`
/// against a vocabulary the artifact no longer has.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    use protocol::SemanticMutation;
    let descriptors = SourcingMutation::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
    assert!(!manifest.contains("\"set-snapshot\"") && !manifest.contains("\"create-stock-item\""), "stock population and whole-document replace are not mutations here — the catalog must not smuggle either back in");
}

/// 🧪️ A base with one pre-existing curated entry (`beam-glulam-gl24h`) so `delete`/`change`
/// mutations have a real target, and `beam-kvh-c24` left uncurated so `create` has a real
/// not-yet-existing target — mirrors `din16798`'s `sample_snapshot()` fixture shape.
fn sample_snapshot() -> CurationSnapshot {
    crate::curation_snapshot_from_stock(&crate::schema::demo_stock(), vec![CuratedItem { object_id: "beam-glulam-gl24h".into(), count: 2 }])
}

/// ⚖️ One value per `SourcingMutation` variant — the closed set the semantics/round-trip tests
/// iterate, mirroring `din16798`'s own `every_mutation()` fixture.
fn every_mutation() -> Vec<SourcingMutation> {
    vec![
        SourcingMutation::CreateCuratedItem(CreateCuratedItem { item: CuratedItem { object_id: "beam-kvh-c24".into(), count: 3 } }),
        SourcingMutation::DeleteCuratedItem(DeleteCuratedItem { object_id: "beam-glulam-gl24h".into() }),
        SourcingMutation::ChangeCuratedItemCount(ChangeCuratedItemCount { object_id: "beam-glulam-gl24h".into(), new_count: 5 }),
    ]
}

fn round_trip(base: &CurationSnapshot, mutation: &SourcingMutation) -> CurationSnapshot {
    let (forward, _messages) = vcs::apply_mutation(base, mutation).expect("valid mutation");
    let mut restored = forward.clone();
    for back in mutation.inverse(base) {
        let (next, _messages) = vcs::apply_mutation(&restored, &back).expect("valid inverse mutation");
        restored = next;
    }
    assert_eq!(&restored, base, "inverse(base) must restore the pre-mutation document");
    forward
}

#[semio_framework_async_macros::async_test]
async fn every_variant_registers_an_approved_semantic_descriptor() {
    for mutation in every_mutation() {
        let descriptor = protocol::SemanticMutation::semantics(&mutation);
        assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {mutation:?}", descriptor.verb);
    }
    assert_eq!(<SourcingMutation as protocol::SemanticMutation<CurationSnapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
}

#[semio_framework_async_macros::async_test]
async fn every_variant_round_trips_via_inverse() {
    let base = sample_snapshot();
    for mutation in every_mutation() {
        round_trip(&base, &mutation);
    }
}

//#region 🧪️MutationLaws
/// ⚖️ Shared law helpers from
/// `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️test/🦀️kit.rs` (reachable here as
/// `protocol::os_spr::testkit`), exercised against all three variants.
#[semio_framework_async_macros::async_test]
async fn create_curated_item_satisfies_the_inverse_and_absorb_laws() {
    let base = sample_snapshot();
    let mutation = SourcingMutation::CreateCuratedItem(CreateCuratedItem { item: CuratedItem { object_id: "beam-kvh-c24".into(), count: 2 } });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).into_parts().0;
    let mid = protocol::MutationDiff::apply(&d1, &base).expect("valid mutation diff");
    let d2 = SourcingMutation::ChangeCuratedItemCount(ChangeCuratedItemCount { object_id: "beam-kvh-c24".into(), new_count: 5 }).diff(&mid).into_parts().0;
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_curated_item_satisfies_the_inverse_and_absorb_laws() {
    let mut base = sample_snapshot();
    base.curated.push(CuratedItem { object_id: "beam-steel-ipe200".into(), count: 4 });
    let mutation = SourcingMutation::DeleteCuratedItem(DeleteCuratedItem { object_id: "beam-steel-ipe200".into() });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).into_parts().0;
    let d2 = SourcingMutation::CreateCuratedItem(CreateCuratedItem { item: CuratedItem { object_id: "beam-steel-hea160".into(), count: 1 } }).diff(&base).into_parts().0;
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn change_curated_item_count_satisfies_the_inverse_and_absorb_laws() {
    let base = sample_snapshot();
    let mutation = SourcingMutation::ChangeCuratedItemCount(ChangeCuratedItemCount { object_id: "beam-glulam-gl24h".into(), new_count: 6 });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).into_parts().0;
    let d2 = SourcingMutation::DeleteCuratedItem(DeleteCuratedItem { object_id: "beam-glulam-gl24h".into() }).diff(&base).into_parts().0;
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
//#endregion 🧪️MutationLaws

//#region 🔖️OutcomeLaws
/// ✅️ 26/08/16 MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS §C2 laws — one per
/// verb family this facet has (`delete`/`change`, `create`): `assert_missing_target_is_error`/
/// `assert_fatal_never_applies` below, `assert_outcome_policy_matrix` cases further down.
#[semio_framework_async_macros::async_test]
async fn delete_curated_item_missing_target_is_an_error() {
    let base = sample_snapshot();
    let mutation = SourcingMutation::DeleteCuratedItem(DeleteCuratedItem { object_id: "beam-kvh-c24".into() });
    protocol::os_spr::testkit::assert_missing_target_is_error(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn change_curated_item_count_missing_target_is_an_error() {
    let base = sample_snapshot();
    let mutation = SourcingMutation::ChangeCuratedItemCount(ChangeCuratedItemCount { object_id: "beam-kvh-c24".into(), new_count: 9 });
    protocol::os_spr::testkit::assert_missing_target_is_error(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn create_curated_item_duplicate_id_is_fatal_and_never_applies() {
    let base = sample_snapshot();
    let mutation = SourcingMutation::CreateCuratedItem(CreateCuratedItem { item: CuratedItem { object_id: "beam-glulam-gl24h".into(), count: 1 } });
    let outcome = mutation.diff(&base);
    assert_eq!(outcome.worst_level(), Some(protocol::os_dsl::Severity::Fatal));
    protocol::os_spr::testkit::assert_fatal_never_applies(&outcome).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_curated_item_outcome_obeys_the_policy_matrix() {
    let base = sample_snapshot();
    let mutation = SourcingMutation::DeleteCuratedItem(DeleteCuratedItem { object_id: "beam-glulam-gl24h".into() });
    protocol::os_spr::testkit::assert_outcome_policy_matrix(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn change_curated_item_count_outcome_obeys_the_policy_matrix() {
    let base = sample_snapshot();
    let mutation = SourcingMutation::ChangeCuratedItemCount(ChangeCuratedItemCount { object_id: "beam-glulam-gl24h".into(), new_count: 6 });
    protocol::os_spr::testkit::assert_outcome_policy_matrix(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn create_curated_item_outcome_obeys_the_policy_matrix() {
    let base = sample_snapshot();
    let mutation = SourcingMutation::CreateCuratedItem(CreateCuratedItem { item: CuratedItem { object_id: "beam-kvh-c24".into(), count: 3 } });
    protocol::os_spr::testkit::assert_outcome_policy_matrix(&base, &mutation).await;
}
//#endregion 🔖️OutcomeLaws
