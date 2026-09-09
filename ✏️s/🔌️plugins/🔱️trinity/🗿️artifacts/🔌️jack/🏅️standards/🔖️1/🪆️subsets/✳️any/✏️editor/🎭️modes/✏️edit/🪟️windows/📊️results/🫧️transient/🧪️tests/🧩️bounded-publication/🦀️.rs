//! 🧪️ Large query output moves once and retires through exact tracked ownership.

use super::*;
use semio_framework_plugin::WindowTransientOwner;

fn close_publication(publication: &mut store::ArtifactEphemeralOneItemPublication<JackResultsWindowTransient, JackResultsWindowTransientMutation>, grant: store::ArtifactStoreOneItemGrant) {
    for _ in 0..65_536 {
        match publication.close_step(grant).expect("Jack results publication close") {
            store::SnapshotRetirementStep::Complete => break,
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= grant.maximum_items && released_bytes <= grant.maximum_bytes);
            }
            store::SnapshotRetirementStep::Blocked => panic!("publication must return tracked aliases without waiting for their payload"),
        }
    }
    assert!(publication.terminal_is_empty());
}

#[test]
fn results_window_large_output_preserves_alias_cancel_and_bounded_disposal() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧩️bounded-publication/🔣️.json")).unwrap();
    let payload = fixture["payload"]["text"].as_str().unwrap().repeat(fixture["payload"]["repeat"].as_u64().unwrap() as usize);
    assert!(payload.len() > fixture["payload"]["minimumUtf8Bytes"].as_u64().unwrap() as usize);
    let payload_pointer = payload.as_ptr();
    let owners = JackResultsWindowTransientOwner::build_owners();
    let mut state = store::TransientStore::<JackResultsWindowTransient, JackResultsWindowTransientMutation>::new(Default::default());
    let held_base = state.current_read().unwrap();
    let mutation = JackResultsWindowTransientMutation::ReplaceQueryResult(ReplaceQueryResult {
        execution_id: Some(fixture["published"]["queryExecutionId"].as_str().unwrap().into()),
        result: Some(crate::ast::QueryResult::table(vec![fixture["published"]["column"].as_str().unwrap().into()], vec![vec![crate::PropertyValue::String(payload)]])),
        error: None,
    });
    let mut publication =
        state.begin_publish_one_leased(semio_framework_job::OperationId(1), 0, mutation, owners.preparation.as_ref(), owners.state_retirement.clone()).unwrap_or_else(|rejected| panic!("large Jack result admission: {}", rejected.reason));
    let zero = store::ArtifactStoreOneItemGrant { maximum_items: fixture["grants"]["blocked"]["maximumItems"].as_u64().unwrap() as usize, maximum_bytes: fixture["grants"]["blocked"]["maximumBytes"].as_u64().unwrap() as usize };
    assert!(matches!(state.advance_publish_one(&mut publication, zero).unwrap(), store::ArtifactStoreOneItemAdvance::Blocked));
    assert_eq!(publication.progress().completed_items, 0);
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: fixture["grants"]["advance"]["maximumItems"].as_u64().unwrap() as usize, maximum_bytes: fixture["grants"]["advance"]["maximumBytes"].as_u64().unwrap() as usize };
    for _ in 0..16 {
        if matches!(state.advance_publish_one(&mut publication, grant).unwrap(), store::ArtifactStoreOneItemAdvance::Published(_)) {
            assert!(publication.acknowledge());
            break;
        }
    }
    let published = state.current_root();
    let result = published.result.as_ref().expect("published query result");
    let crate::PropertyValue::String(actual) = &result.rows[0][0] else { panic!("published string cell") };
    assert_eq!(actual.as_ptr(), payload_pointer, "publication must move the large allocation exactly once");
    assert_eq!(published.query_execution_id.as_deref(), fixture["published"]["queryExecutionId"].as_str());
    let kind: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(&result.kind)).unwrap();
    assert_eq!(kind, fixture["published"]["kind"]);
    close_publication(&mut publication, grant);
    assert_eq!(held_base.query_execution_id, None);
    assert!(matches!(state.maintenance_returned_reads_step(&owners.state_retirement, 1, 1).unwrap(), store::SnapshotRetirementStep::Complete));
    assert!(!state.returned_reads_terminal_is_empty(), "the live tracked alias must remain visible to terminal ownership");
    drop(held_base);
    for _ in 0..128 {
        if matches!(state.maintenance_returned_reads_step(&owners.state_retirement, 1, 1).unwrap(), store::SnapshotRetirementStep::Complete) {
            break;
        }
    }
    assert!(state.returned_reads_terminal_is_empty());

    let before_cancel = state.current_root();
    let cancelled = JackResultsWindowTransientMutation::ReplaceQueryResult(ReplaceQueryResult { execution_id: Some("cancelled".into()), result: None, error: Some("cancelled output".repeat(512)) });
    let mut cancellation =
        state.begin_publish_one_leased(semio_framework_job::OperationId(2), 1, cancelled, owners.preparation.as_ref(), owners.state_retirement.clone()).unwrap_or_else(|rejected| panic!("cancellation admission: {}", rejected.reason));
    assert!(state.cancel_publish_one(&mut cancellation));
    assert!(matches!(state.advance_publish_one(&mut cancellation, grant).unwrap(), store::ArtifactStoreOneItemAdvance::Blocked));
    close_publication(&mut cancellation, grant);
    assert!(std::sync::Arc::ptr_eq(&before_cancel, &state.current_root()));
    drop(before_cancel);
    drop(published);

    let mut retirement = state.begin_retirement(Default::default(), owners.state_retirement.clone());
    for _ in 0..65_536 {
        match retirement.close_step(1, 1).unwrap() {
            store::SnapshotRetirementStep::Complete => break,
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 1),
            store::SnapshotRetirementStep::Blocked => panic!("all Jack results aliases were returned"),
        }
    }
    assert!(retirement.terminal_is_empty());
    eprintln!("[DEBUG] Jack results transient: >4KiB serde output moved once, tracked alias returned, cancellation preserved state, and all owners retired with one-item/one-byte grants");
}
