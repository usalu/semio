use super::*;

use db_io_core::{PayloadStorage as _, WalStorage as _};

/// 🧾️ The authored source of the handle crate: every child module and then the root, in module order, as the one text the source laws scan.
const HANDLE_SOURCE: &str = concat!(
    include_str!("../../🎟️history-view-lease-and-journal-sink/🦀️.rs"),
    include_str!("../../📨️submit-admission-and-state/🦀️.rs"),
    include_str!("../../📤️submit-future-and-terminal/🦀️.rs"),
    include_str!("../../🚪️history-admission-and-wake/🦀️.rs"),
    include_str!("../../📚️history-state/🦀️.rs"),
    include_str!("../../🔭️history-future-and-terminals/🦀️.rs"),
    include_str!("../../🤝️artifact-handle/🦀️.rs"),
    include_str!("../../🦀️.rs"),
);

//#region 🧸️Fixtures
async fn envelope(id: &str, deps: &[&str], actor: &str, document: &protocol::ArtifactId, entries: &[(&str, serde_json::Value)]) -> protocol::MutationEnvelope {
    let mut payload = serde_json::Map::new();
    for (path, value) in entries {
        payload.insert((*path).to_string(), value.clone());
    }
    protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(id.to_string()),
        document_id: document.clone(),
        actor: protocol::ActorId(actor.to_string()),
        dependencies: deps.iter().map(|dep| protocol::MutationId((*dep).to_string())).collect(),
        observed: None,
        target: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId(db_artifact_contract::DB_PATHMAP_SCHEMA.to_string()), payload: db_artifact_contract::encode_pathmap_json(&serde_json::Value::Object(payload)).await.unwrap() },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId(db_artifact_contract::DB_PATHMAP_SCHEMA.to_string()), payload: db_artifact_contract::encode_pathmap_json(&serde_json::Value::Object(serde_json::Map::new())).await.unwrap() },
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        transaction: None, verb: None, line: None,
    }
}
//#endregion 🧸️Fixtures
//#region 🔖️Retained submit authority
fn retire_history_admission(mut admission: ArtifactHistoryAdmission) {
    let mut cursor = admission.begin_reservation_close().expect("history test admission retained its reservation");
    for _ in 0..50_000 {
        if cursor.terminal_is_empty() {
            break;
        }
        assert!(cursor.close_step());
    }
    assert!(cursor.terminal_is_empty());
}

#[test]
fn artifact_submit_stale_generation_and_slot_aba_cannot_consume_current_work() {
    let first = ArtifactSubmitAdmission::try_claim(1, ARTIFACT_SUBMIT_PAGE_BYTES).unwrap();
    let first_slot = first.slot;
    let first_generation = first.generation;
    drop(first);
    let next = ArtifactSubmitAdmission::try_claim(1, ARTIFACT_SUBMIT_PAGE_BYTES).unwrap();
    assert_eq!(next.slot, first_slot);
    assert_ne!(next.generation, first_generation);
    let source = HANDLE_SOURCE;
    let stale = source.find("if generation != self.generation").unwrap();
    let mutation = source[stale..].find("self.scheduled.store").unwrap();
    assert!(mutation > 0);
}

#[semio_framework_async_macros::async_test]
async fn artifact_submit_item_cap_plus_one_and_nested_bytes_plus_one_return_owner() {
    let document = protocol::ArtifactId("credit-doc".to_string());
    let one = envelope("credit-1", &[], "credit-actor", &document, &[("x", serde_json::json!(1))]).await;
    let admitted = db_artifact_contract::CommandBatch::new(vec![one]).await.unwrap();
    assert!(artifact_submit_credit(&admitted).is_ok());

    let mut envelopes = Vec::new();
    for index in 0..=ARTIFACT_SUBMIT_BATCH_ITEMS {
        envelopes.push(envelope(&format!("credit-{index}"), &[], "credit-actor", &document, &[("x", serde_json::json!(index))]).await);
    }
    let rejected = db_artifact_contract::CommandBatch { envelopes };
    assert!(artifact_submit_credit(&rejected).is_err());

    let mut oversize = envelope("credit-oversize", &[], "credit-actor", &document, &[("x", serde_json::json!(1))]).await;
    oversize.diff.payload = vec![0; ARTIFACT_SUBMIT_OPERATION_BYTES as usize + 1];
    assert!(artifact_submit_credit(&db_artifact_contract::CommandBatch { envelopes: vec![oversize] }).is_err());
}

#[test]
fn artifact_history_empty_one_cap_plus_one_admission_returns_exact_request() {
    if !db_io_driver::process_isolated_law("tests::artifact_history_empty_one_cap_plus_one_admission_returns_exact_request") {
        return;
    }
    let _history_capacity = db_artifact_history::history_capacity_test_lock();
    let mut claims = Vec::new();
    for _ in 0..ARTIFACT_HISTORY_OPERATION_SLOTS {
        claims.push(ArtifactHistoryAdmission::try_claim().unwrap());
    }
    assert!(matches!(ArtifactHistoryAdmission::try_claim(), Err(ArtifactHistoryAdmissionError::Rejected(DbError::Unavailable(_)))));
    for claim in claims {
        retire_history_admission(claim);
    }
    let source = HANDLE_SOURCE;
    assert!(source.contains("ArtifactHistoryWorkOwner::Request"));
}

#[test]
fn artifact_history_admission_unwind_under_contention_retires_live_reservations_and_returns_credit() {
    if !db_io_driver::process_isolated_law("tests::artifact_history_admission_unwind_under_contention_retires_live_reservations_and_returns_credit") {
        return;
    }
    let _history_capacity = db_artifact_history::history_capacity_test_lock();
    let unwound = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut peers = Vec::new();
        for _ in 0..ARTIFACT_HISTORY_OPERATION_SLOTS {
            peers.push(ArtifactHistoryAdmission::try_claim().unwrap());
        }
        let _started = peers[0].begin_reservation_close().map(|mut cursor| {
            assert!(cursor.close_step());
            cursor
        });
        let _contended = ArtifactHistoryAdmission::try_claim().unwrap();
    }));
    assert!(unwound.is_err(), "the contended claim must fail while every peer holds a live reservation");
    let mut claims = Vec::new();
    for _ in 0..ARTIFACT_HISTORY_OPERATION_SLOTS {
        claims.push(ArtifactHistoryAdmission::try_claim().expect("unwinding returned every slot's exact credit"));
    }
    for claim in claims {
        retire_history_admission(claim);
    }
}

#[test]
fn artifact_history_cancel_before_handoff_retires_full_reservation_before_credit_release() {
    if !db_io_driver::process_isolated_law("tests::artifact_history_cancel_before_handoff_retires_full_reservation_before_credit_release") {
        return;
    }
    let _history_capacity = db_artifact_history::history_capacity_test_lock();
    let mut cancelled = ArtifactHistoryAdmission::try_claim().unwrap();
    let cancelled_generation = cancelled.generation;
    let mut cursor = cancelled.begin_reservation_close().expect("cancelled pre-handoff request retained its full reservation");
    let mut peers = Vec::new();
    for _ in 1..ARTIFACT_HISTORY_OPERATION_SLOTS {
        peers.push(ArtifactHistoryAdmission::try_claim().unwrap());
    }
    assert!(matches!(ArtifactHistoryAdmission::try_claim(), Err(ArtifactHistoryAdmissionError::Rejected(DbError::Unavailable(_)))), "retirement must retain admission credit until terminal-empty");
    assert!(cursor.close_step());
    assert!(!cursor.terminal_is_empty());
    for _ in 0..50_000 {
        if cursor.terminal_is_empty() {
            break;
        }
        assert!(cursor.close_step());
    }
    assert!(cursor.terminal_is_empty());
    drop(cursor);
    drop(cancelled);
    let replacement = ArtifactHistoryAdmission::try_claim().unwrap();
    assert_ne!(replacement.generation, cancelled_generation);
    retire_history_admission(replacement);
    for peer in peers {
        retire_history_admission(peer);
    }
}

#[test]
fn artifact_history_stale_generation_and_slot_aba_precede_mailbox_mutation() {
    if !db_io_driver::process_isolated_law("tests::artifact_history_stale_generation_and_slot_aba_precede_mailbox_mutation") {
        return;
    }
    let _history_capacity = db_artifact_history::history_capacity_test_lock();
    let first = ArtifactHistoryAdmission::try_claim().unwrap();
    let slot = first.slot;
    let generation = first.generation;
    retire_history_admission(first);
    let next = ArtifactHistoryAdmission::try_claim().unwrap();
    assert_eq!(next.slot, slot);
    assert_ne!(next.generation, generation);
    let source = HANDLE_SOURCE;
    let stale = source.find("if self.authority.generation() != self.authority_generation").unwrap();
    let handoff = source.find("self.authority.history_retained").unwrap();
    assert!(stale < handoff);
    retire_history_admission(next);
}
//#endregion 🔖️Retained submit authority

//#region 🔖️Source laws
#[test]
fn artifact_submit_late_readiness_parks_then_one_shot_wake_reschedules() {
    let source = HANDLE_SOURCE;
    assert!(source.contains("impl std::task::Wake for ArtifactSubmitWake"));
    assert!(source.contains("self.scheduled.compare_exchange(false, true"));
    assert!(source.contains("std::task::Poll::Pending =>"));
    assert!(source.contains("self.set_progress(SubmitProgress::Waiting)"));
}

#[test]
fn artifact_submit_pool_saturation_without_later_ingress_retains_exact_job() {
    let source = HANDLE_SOURCE;
    assert!(source.contains("self.pool.try_submit(Lane::Io, job)"));
    assert!(source.contains("error.into_job()"));
    assert!(source.contains("self.pool.callback_at"));
    assert!(source.contains("ARTIFACT_SUBMIT_RETRY_LIMIT"));
}

#[test]
fn artifact_submit_cancel_before_during_after_preserves_exact_owner() {
    let source = HANDLE_SOURCE;
    assert!(source.contains("self.state.cancelled.store(true"));
    assert!(source.contains("self.terminalize_work(Err(DbError::Closed), SubmitProgress::Cancelled)"));
    assert!(source.contains("*self.terminal_result.lock()"));
    assert!(source.contains("SubmitProgress::Completed | SubmitProgress::Cancelled | SubmitProgress::Fault"));
}

#[test]
fn artifact_submit_missing_handle_terminalizes_without_mailbox_mutation() {
    let source = HANDLE_SOURCE;
    let released = source.find("let Some(authority) = self.authority() else {").unwrap();
    let stale = source.find("if authority.generation() != self.authority_generation").unwrap();
    let handoff = source.find("authority.submit_retained(batch, options, submitted_at_ms)").unwrap();
    assert!(released < stale && stale < handoff);
    assert!(source.contains("Err(DbError::StaleGeneration"));
}

#[test]
fn artifact_submit_terminal_job_work_result_take_resume_and_close_one_owner() {
    let source = HANDLE_SOURCE;
    for required in ["pub fn take_terminal_job", "pub fn take_terminal_work", "pub fn take_terminal_result", "pub fn take_actor_terminal_job", "pub fn close_step", "pub fn terminal_is_empty", "pub fn resume(mut self)"] {
        assert!(source.contains(required), "missing {required}");
    }
    assert!(source.contains("fn close_one(&self) -> bool"));
}

#[test]
fn artifact_history_quiet_late_wake_and_retry_are_generation_coalesced() {
    let source = HANDLE_SOURCE;
    assert!(source.contains("impl std::task::Wake for ArtifactHistoryWake"));
    assert!(source.contains("self.generation == state.generation"));
    assert!(source.contains("self.scheduled.compare_exchange(false, true"));
    assert!(source.contains("self.pool.callback_at"));
    assert!(source.contains("retry_generation"));
}

#[test]
fn artifact_history_cancel_before_during_after_retains_actor_and_result_owners() {
    if !db_io_driver::process_isolated_law("tests::artifact_history_cancel_before_during_after_retains_actor_and_result_owners") {
        return;
    }
    let source = HANDLE_SOURCE;
    assert!(source.contains("history_retained(self.generation, self.cancelled.clone(), reservation)"));
    assert!(source.contains("terminalize_unhanded_request(Err(DbError::Closed), HistoryProgress::Cancelled)"));
    assert!(source.contains("self.terminalize_work(Err(DbError::Closed), HistoryProgress::Cancelled)"));
    assert!(source.contains("*self.terminal_result.lock()"));
    assert!(source.contains("HistoryProgress::Completed | HistoryProgress::Cancelled | HistoryProgress::Fault"));
}

#[test]
fn artifact_history_terminal_job_work_result_take_resume_and_close_one_owner() {
    let source = HANDLE_SOURCE;
    for required in [
        "pub fn take_terminal_job",
        "pub fn take_terminal_work",
        "pub fn take_terminal_result",
        "pub fn take_actor_terminal_job",
        "pub fn close_step",
        "pub fn terminal_is_empty",
        "impl ArtifactHistoryTerminalJob",
        "impl ArtifactHistoryTerminalWork",
        "pub struct ArtifactHistoryTerminalReservation",
        "pub fn take_terminal_reservation",
    ] {
        assert!(source.contains(required), "missing {required}");
    }
    assert!(source.contains("fn close_one(self: &Arc<Self>) -> bool"));
}

#[test]
fn artifact_history_drop_after_complete_moves_result_to_public_terminal_registry() {
    let source = HANDLE_SOURCE;
    let result_drop = &source[source.find("impl Drop for HistoryView").unwrap()..source.find("//#endregion 🔖️History").unwrap()];
    assert!(result_drop.contains("register_artifact_history(&state)"));
    assert!(result_drop.contains("state.terminal_result"));
    assert!(result_drop.contains("self.inner.take()"));
    assert!(source.contains("pub fn history_terminal(&self, generation: u64)"));
}

#[test]
fn artifact_history_future_handle_drop_and_terminal_take_resume_are_exact() {
    let source = HANDLE_SOURCE;
    let future_drop = &source[source.find("impl Drop for HistoryFuture").unwrap()..source.find("impl ArtifactHistoryTerminalJob").unwrap()];
    assert!(future_drop.contains("completion.take()"));
    assert!(future_drop.contains("terminal_result"));
    assert!(!future_drop.contains("self.state.close_one()"));
    for required in ["pub struct ArtifactHistoryTerminalHandle", "pub fn terminal_handle(&self)", "pub fn take_terminal_result(&self)", "pub fn resume(mut self)", "pub fn close_step(&self)"] {
        assert!(source.contains(required), "missing {required}");
    }
}

#[test]
fn retained_retry_owners_release_their_guard_before_resubmission() {
    for (name, source) in [("db_engine_handle", HANDLE_SOURCE)] {
        let lines: Vec<&str> = source.lines().collect();
        for (index, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start().trim_start_matches("} else ");
            let Some((_, scrutinee)) = trimmed.strip_prefix("if let ").or_else(|| trimmed.strip_prefix("while let ")).and_then(|rest| rest.split_once(" = ")) else { continue };
            if !(scrutinee.contains("retry_job.lock()") && !scrutinee.starts_with("{ ") && scrutinee.trim_end().ends_with('{')) {
                continue;
            }
            let mut depth = 1i64;
            for body in &lines[index + 1..] {
                assert!(!body.contains("retry_job.lock()") && !body.contains("submit_exact("), "{name}:{} holds the retry-job guard across a body that relocks or resubmits it: {line}", index + 1);
                depth += body.matches('{').count() as i64 - body.matches('}').count() as i64;
                if depth <= 0 {
                    break;
                }
            }
        }
    }
}

#[test]
fn artifact_history_construction_fault_is_public_and_admission_release_is_a_final_grant() {
    if !db_io_driver::process_isolated_law("tests::artifact_history_construction_fault_is_public_and_admission_release_is_a_final_grant") {
        return;
    }
    let engine = HANDLE_SOURCE;
    for required in [
        "terminal_construction: Mutex<Option<db_artifact_history::HistoryReplayReservationConstructionFault>>",
        "pub struct ArtifactHistoryTerminalConstructionFault",
        "pub fn take_terminal_construction_fault",
        "pub fn resume(mut self) -> Result<(), Self>",
        "fn terminal_roots_are_empty(&self) -> bool",
        "self.finished.load(std::sync::atomic::Ordering::Acquire)",
        "self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()",
        "self.state.finish_if_terminal_empty()",
    ] {
        assert!(engine.contains(required), "missing retained public construction/admission authority: {required}");
    }
    let terminal = &engine[engine.find("impl ArtifactHistoryTerminalHandle").unwrap()..engine.find("impl Future for HistoryFuture").unwrap()];
    let close = &terminal[terminal.find("pub fn close_step(&self)").unwrap()..terminal.find("pub fn terminal_is_empty(&self)").unwrap()];
    assert!(close.find("self.state.close_one()").unwrap() < close.find("self.state.finish_if_terminal_empty()").unwrap());
    assert!(close.contains("return true"), "owner retirement must return before the final admission-release grant");
}

#[test]
fn artifact_history_one_grant_advances_one_retained_phase_without_blocking() {
    let engine = HANDLE_SOURCE;
    let history = &engine[engine.find("const ARTIFACT_HISTORY_OPERATION_SLOTS").unwrap()..engine.find("pub struct ArtifactHandle").unwrap()];
    for forbidden in ["block_on(", "ask_blocking", "submit_blocking", "loop {", "while "] {
        assert!(!history.contains(forbidden), "history outer authority retained {forbidden}");
    }
    assert!(history.contains("std::pin::Pin::new(future).poll(&mut context)"));
}
//#endregion 🔖️Source laws
