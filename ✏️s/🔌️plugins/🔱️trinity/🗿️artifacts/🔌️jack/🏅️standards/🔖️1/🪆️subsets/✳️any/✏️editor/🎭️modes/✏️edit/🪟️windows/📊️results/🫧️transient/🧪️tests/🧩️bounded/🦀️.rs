//! 🧪️ Large query output moves once and retires through exact tracked ownership.

use super::*;
use semio_framework_plugin::WindowTransientOwner;

/// 🎟️ One five-currency one-item grant whose byte currencies all equal `bytes`.
fn one_item_grant(items: usize, bytes: usize) -> store::ArtifactStoreOneItemGrant {
    store::ArtifactStoreOneItemGrant { maximum_items: items, maximum_copy_bytes: bytes, maximum_capacity_bytes: bytes, maximum_release_bytes: bytes, maximum_depth: 64 }
}

/// 🎟️ The generous wallet a maintenance turn spends when the store publishes no quote of its own.
fn maintenance_grant(bytes: usize) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    one_item_grant(1, bytes).retained_grant()
}

fn close_publication(publication: &mut store::ArtifactEphemeralOneItemPublication<JackResultsWindowTransient, JackResultsWindowTransientMutation>) {
    publication.begin_close();
    for _ in 0..65_536 {
        if publication.terminal_is_empty() {
            return;
        }
        let demand = publication.retirement_demands(crate::JACK_SELF_FUNDED_BODY_BYTES).expect("Jack results publication close quote");
        let funded = crate::jack_self_funded_grant(demand);
        let grant = store::ArtifactStoreOneItemGrant { maximum_items: funded.maximum_items, maximum_copy_bytes: funded.maximum_copy_bytes, maximum_capacity_bytes: funded.maximum_capacity_bytes, maximum_release_bytes: funded.maximum_release_bytes, maximum_depth: funded.maximum_depth };
        let step = publication.close_step(grant).expect("Jack results publication close");
        assert!(step.progress().fits(funded));
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
    let zero = one_item_grant(fixture["grants"]["blocked"]["maximumItems"].as_u64().unwrap() as usize, fixture["grants"]["blocked"]["maximumBytes"].as_u64().unwrap() as usize);
    assert!(matches!(state.advance_publish_one(&mut publication, zero).unwrap(), store::ArtifactStoreOneItemAdvance::Blocked));
    assert_eq!(publication.progress().completed_items, 0);
    let grant = one_item_grant(fixture["grants"]["advance"]["maximumItems"].as_u64().unwrap() as usize, fixture["grants"]["advance"]["maximumBytes"].as_u64().unwrap() as usize);
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
    let kind: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&result.kind)).unwrap();
    assert_eq!(kind, fixture["published"]["kind"]);
    close_publication(&mut publication);
    assert_eq!(held_base.query_execution_id, None);
    assert!(matches!(state.maintenance_returned_reads_step(&owners.state_retirement, maintenance_grant(65_536)).unwrap(), semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)));
    assert!(!state.returned_reads_terminal_is_empty(), "the live tracked alias must remain visible to terminal ownership");
    drop(held_base);
    for _ in 0..128 {
        if matches!(state.maintenance_returned_reads_step(&owners.state_retirement, maintenance_grant(65_536)).unwrap(), semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) {
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
    close_publication(&mut cancellation);
    assert!(std::sync::Arc::ptr_eq(&before_cancel, &state.current_root()));
    drop(before_cancel);
    drop(published);

    let mut retirement = state.begin_retirement(Default::default(), owners.state_retirement.clone());
    for _ in 0..65_536 {
        if retirement.terminal_is_empty() {
            break;
        }
        let demand = retirement.retirement_demands(crate::JACK_SELF_FUNDED_BODY_BYTES).expect("all Jack results aliases were returned");
        let grant = crate::jack_self_funded_grant(demand);
        assert!(retirement.close_step(grant).unwrap().progress().fits(grant));
    }
    assert!(retirement.terminal_is_empty());
}

/// ➕️ `replace-query-result`'s concrete inverse row sums to exactly the negative of its sparse diff (law L3).
#[semio_framework_async_macros::async_test]
async fn replace_query_result_inverse_sums_to_the_negative_diff() {
    let base = JackResultsWindowTransient { query_execution_id: Some("run-1".into()), result: None, query_error: Some("old".into()) };
    let mutation: JackResultsWindowTransientMutation = ReplaceQueryResult { execution_id: Some("run-2".into()), result: None, error: None }.into();
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
}
