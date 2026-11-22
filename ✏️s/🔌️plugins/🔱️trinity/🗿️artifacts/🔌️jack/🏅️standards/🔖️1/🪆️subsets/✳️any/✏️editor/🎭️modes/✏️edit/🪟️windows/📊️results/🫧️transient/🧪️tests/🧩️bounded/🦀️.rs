//! 🧪️ Large query output moves once and retires through exact tracked ownership.

use crate::editor::jack::transient::results_window::*;
use semio_plugin::WindowTransientOwner;
use semio_framework_job::{OriginalRetirementPump, OriginalRetirementTurn};
use semio_framework_value::{OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneStep, RetainedWorkPoll};

/// 🎟️ One five-currency one-item grant whose byte currencies all equal `bytes`.
fn one_item_grant(items: usize, bytes: usize) -> store::os_store::one_item::ArtifactStoreOneItemGrant {
    store::os_store::one_item::ArtifactStoreOneItemGrant { maximum_items: items, maximum_copy_bytes: bytes, maximum_capacity_bytes: bytes, maximum_release_bytes: bytes, maximum_depth: 64 }
}

/// 🎟️ The generous wallet a maintenance turn spends when the store publishes no quote of its own.
fn maintenance_grant(bytes: usize) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    one_item_grant(1, bytes).retained_grant()
}

fn close_publication(turn: &mut OriginalRetirementTurn, publication: &mut store::ArtifactEphemeralOneItemPublication<JackResultsWindowTransient, JackResultsWindowTransientMutation>) {
    publication.begin_close();
    let body = semio_plugin::PLUGIN_SELF_FUNDED_BODY_BYTES;
    for _ in 0..65_536 {
        if publication.terminal_is_empty() {
            return;
        }
        let demand = publication.retirement_demands(body).expect("Jack results publication close quote");
        let grant = semio_plugin::plugin_self_funded_grant(demand, semio_plugin::PLUGIN_SELF_FUNDED_BODY_BYTES);
        assert!(retirement_turn(turn, grant, |effect| publication.close_step(body, effect)).progress().fits(grant));
    }
    assert!(publication.terminal_is_empty());
}

/// 💳️ Funds one publication retirement action from an exact test loan.
fn retirement_turn(turn: &mut OriginalRetirementTurn, grant: RetainedCloneGrant, close: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll) -> RetainedCloneStep {
    let (poll, _) = turn.with_effect(|effect| close(&mut effect.nested(grant))).unwrap().expect("results retirement admitted");
    match poll {
        RetainedWorkPoll::Complete(turn) => turn.result.expect("results retirement step"),
        RetainedWorkPoll::Pending { .. } => RetainedCloneStep::Progress(Default::default()),
    }
}

#[test]
fn results_window_large_output_preserves_alias_cancel_and_bounded_disposal() {
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧪️tests/🧫️fixtures/🧩️bounded-publication/🔣️.json")).unwrap();
    let payload = fixture["payload"]["text"].as_str().unwrap().repeat(fixture["payload"]["repeat"].as_u64().unwrap() as usize);
    assert!(payload.len() > fixture["payload"]["minimumUtf8Bytes"].as_u64().unwrap() as usize);
    let payload_pointer = payload.as_ptr();
    let payload_bytes = payload.len();
    let mut accepted = |_| true;
    let mut receiving = semio_framework_value::NativeDecodeControl::new(1 << 20, &mut accepted);
    let owners = JackResultsWindowTransientOwner::build_owners();
    let mut pump = OriginalRetirementPump::new().turn(1 << 16, u64::MAX, || Some(0), maintenance_grant(65_536)).unwrap();
    let mut state = store::TransientStore::<JackResultsWindowTransient, JackResultsWindowTransientMutation>::new(Default::default());
    let held_base = state.current_read().unwrap();
    let mutation = JackResultsWindowTransientMutation::ReplaceQueryResult(ReplaceQueryResult {
        execution_id: Some(fixture["published"]["queryExecutionId"].as_str().unwrap().into()),
        result: Some(crate::standards::v1::subsets::any::schema::ast::QueryResult::table(vec![fixture["published"]["column"].as_str().unwrap().into()], vec![vec![semio_framework_graph_manifest::PropertyValue::String(payload)]])),
        error: None,
    });
    let mut publication =
        state.begin_publish_one_leased(semio_framework_job::OperationId(1), 0, mutation, owners.preparation.as_ref(), owners.state_retirement.clone()).unwrap_or_else(|rejected| panic!("large Jack result admission: {}", rejected.reason));
    use store::os_store::one_item_publication::ArtifactStoreOneItemAdvance;
    let fixture_grant = |name: &str| {
        let grant = &fixture["grants"][name];
        one_item_grant(grant["maximumItems"].as_u64().unwrap() as usize, grant["maximumBytes"].as_u64().unwrap() as usize)
    };
    for name in ["blocked", "undersized"] {
        assert!(matches!(state.advance_publish_one(&mut publication, fixture_grant(name), &mut receiving).unwrap(), ArtifactStoreOneItemAdvance::Blocked));
        assert_eq!(publication.progress().completed_items, 0);
    }
    let grant = fixture_grant("advance");
    assert!(grant.maximum_copy_bytes < payload_bytes);
    for _ in 0..16 {
        match state.advance_publish_one(&mut publication, grant, &mut receiving).unwrap() {
            ArtifactStoreOneItemAdvance::Published(..) => {
                assert!(publication.acknowledge());
                break;
            }
            ArtifactStoreOneItemAdvance::PreparationProgress(..) | ArtifactStoreOneItemAdvance::Progress(..) => {}
            _ => panic!("results publication stopped in {:?}: {:?}", publication.phase(), publication.fault()),
        }
    }
    let published = state.current_root();
    let result = published.result.as_ref().expect("published query result");
    let semio_framework_graph_manifest::PropertyValue::String(actual) = &result.rows[0][0] else { panic!("published string cell") };
    assert_eq!(actual.as_ptr(), payload_pointer, "publication must move the large allocation exactly once");
    assert_eq!(published.query_execution_id.as_deref(), fixture["published"]["queryExecutionId"].as_str());
    let kind: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&result.kind)).unwrap();
    assert_eq!(kind, fixture["published"]["kind"]);
    close_publication(&mut pump, &mut publication);
    assert_eq!(held_base.query_execution_id, None);
    assert!(matches!(retirement_turn(&mut pump, maintenance_grant(65_536), |effect| state.maintenance_returned_reads_step(&owners.state_retirement, effect)), semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)));
    assert!(!state.returned_reads_terminal_is_empty(), "the live tracked alias must remain visible to terminal ownership");
    drop(held_base);
    for _ in 0..128 {
        if matches!(retirement_turn(&mut pump, maintenance_grant(65_536), |effect| state.maintenance_returned_reads_step(&owners.state_retirement, effect)), semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    assert!(state.returned_reads_terminal_is_empty());

    let before_cancel = state.current_root();
    let cancelled = JackResultsWindowTransientMutation::ReplaceQueryResult(ReplaceQueryResult { execution_id: Some("cancelled".into()), result: None, error: Some("cancelled output".repeat(512)) });
    let mut cancellation =
        state.begin_publish_one_leased(semio_framework_job::OperationId(2), 1, cancelled, owners.preparation.as_ref(), owners.state_retirement.clone()).unwrap_or_else(|rejected| panic!("cancellation admission: {}", rejected.reason));
    assert!(state.cancel_publish_one(&mut cancellation));
    assert!(matches!(state.advance_publish_one(&mut cancellation, grant, &mut receiving).unwrap(), store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Blocked));
    close_publication(&mut pump, &mut cancellation);
    assert!(std::sync::Arc::ptr_eq(&before_cancel, &state.current_root()));
    drop(before_cancel);
    drop(published);

    let mut retirement = state.begin_retirement(Default::default(), owners.state_retirement.clone());
    let body = semio_plugin::PLUGIN_SELF_FUNDED_BODY_BYTES;
    for _ in 0..65_536 {
        if retirement.terminal_is_empty() {
            break;
        }
        let demand = retirement.retirement_demands(body).expect("all Jack results aliases were returned");
        let grant = semio_plugin::plugin_self_funded_grant(demand, semio_plugin::PLUGIN_SELF_FUNDED_BODY_BYTES);
        assert!(retirement_turn(&mut pump, grant, |effect| retirement.close_step(body, effect)).progress().fits(grant));
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

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
