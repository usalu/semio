
use super::*;

fn every_mutation() -> Vec<SourcingMutation> {
    vec![
        SourcingMutation::CreateCuratedItem(create_curated_item::CreateCuratedItem { item: CuratedItem { object_id: "beam-glulam-gl24h".into(), count: 3 } }),
        SourcingMutation::DeleteCuratedItem(delete_curated_item::DeleteCuratedItem { object_id: "beam-glulam-gl24h".into() }),
        SourcingMutation::ChangeCuratedItemCount(change_curated_item_count::ChangeCuratedItemCount { object_id: "beam-glulam-gl24h".into(), new_count: 5 }),
    ]
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_create_curated_item() {
    store::os_store::test_support::assert_op_line_round_trip(&SourcingMutation::CreateCuratedItem(create_curated_item::CreateCuratedItem { item: CuratedItem { object_id: "beam-glulam-gl24h".into(), count: 3 } }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_delete_curated_item() {
    store::os_store::test_support::assert_op_line_round_trip(&SourcingMutation::DeleteCuratedItem(delete_curated_item::DeleteCuratedItem { object_id: "beam-glulam-gl24h".into() }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_curated_item_count() {
    store::os_store::test_support::assert_op_line_round_trip(&SourcingMutation::ChangeCuratedItemCount(change_curated_item_count::ChangeCuratedItemCount { object_id: "beam-glulam-gl24h".into(), new_count: 5 }));
}

/// ⚖️ Every variant, not just the three hand-picked above — full-coverage `OpText` round trip
/// over the closed vocabulary, one sample value per field.
#[semio_framework_async_macros::async_test]
async fn every_variant_op_text_round_trips() {
    for mutation in every_mutation() {
        store::os_store::test_support::assert_op_line_round_trip(&mutation);
    }
}
