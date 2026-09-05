//#region 📡️RegisteredQueryDispatch
use super::*;

//#region 🚪️ExactRuntimeCloseLease
/// ⏱️ Mirrors the close worker's clock contract so a fixture clock row pins WHICH cleanup-fault
/// variant the turn must publish, not merely that some fault was published.
fn expected_clock_cause(samples: &[Option<u64>]) -> super::super::RuntimeCleanupFault {
    use super::super::RuntimeCleanupFault;
    let (Some(Some(start)), Some(Some(preflight))) = (samples.first().copied(), samples.get(1).copied()) else { return RuntimeCleanupFault::Clock };
    if preflight < start { return RuntimeCleanupFault::ClockRegression; }
    let Some(Some(finished)) = samples.get(2).copied() else { return RuntimeCleanupFault::Clock };
    if finished < preflight { return RuntimeCleanupFault::ClockRegression; }
    RuntimeCleanupFault::InteractiveCeiling
}

fn terminal_close_state(complete: bool, faulted: bool, blocked: bool) -> std::sync::Arc<super::super::RuntimeCloseWorkerState<TestRuntimeApps>> {
    use super::super::*;
    let job = RuntimeCloseCleanupJob { instance_id: 7, state: None, progress: None, contended: false, closing: true };
    let params = semio_framework_job::BatchJobParams {
        operation: semio_framework_job::OperationId(711), generation: semio_framework_job::Generation(1),
        cancel: semio_framework_job::CancelToken::root_now(),
        config: semio_framework_job::BatchDriveConfig { site: "exact-close-terminal-law", stage: semio_framework_job::InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 500 },
        now_us: semio_framework_job::default_now_us,
    };
    let mut session = match semio_framework_job::BatchJobSession::try_new(job, params) { Ok(session) => session, Err(_) => panic!("fixed terminal fixture admission") };
    session.begin_close();
    for _ in 0..1_000 { if session.terminal_is_empty() { break; } let _ = session.close_step(1, 4096); }
    assert!(session.terminal_is_empty());
    let mut pump = RuntimeCloseCleanupPump::new();
    pump.session = Some(session); pump.terminal = true; pump.complete = complete; pump.faulted = faulted; pump.blocked = blocked;
    std::sync::Arc::new(RuntimeCloseWorkerState {
        instance_id: 7, generation: semio_framework_job::Generation(1),
        cell: std::sync::Mutex::new(std::mem::ManuallyDrop::new(None)), pump: std::sync::Mutex::new(pump),
        status: std::sync::atomic::AtomicU8::new(RuntimeCloseStatus::Queued.repr()), stalled_steps: std::sync::atomic::AtomicU8::new(0),
        preview_sequence: std::sync::atomic::AtomicU64::new(0), last_callback_elapsed_us: std::sync::atomic::AtomicU64::new(0),
        last_fault: std::sync::Mutex::new([0; 256]), last_fault_origin: std::sync::atomic::AtomicU8::new(0),
        callback_phase_started_us: std::sync::atomic::AtomicU64::new(0), callback_phase_us: std::array::from_fn(|_| std::sync::atomic::AtomicU64::new(0)),
    })
}

#[test]
fn instance_lifetime_close_does_not_publish_terminal_before_watchdog() {
    use super::super::*;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🚨️fault.fixture.json")).unwrap();
    let state = terminal_close_state(true, false, false);
    let _ = run_runtime_close_turn_inner(&state);
    assert_eq!(RuntimeCloseStatus::from_repr(state.status.load(std::sync::atomic::Ordering::SeqCst)) == RuntimeCloseStatus::Complete, fixture["owners"]["terminalVisibleBeforeWatchdog"].as_bool().unwrap());
    for row in fixture["callbacks"].as_array().unwrap() {
        let named = |name: &str| match name { "ready" => RuntimeCloseStatus::Ready, "complete" => RuntimeCloseStatus::Complete, "external-wait" => RuntimeCloseStatus::ExternalWait, "fault" => RuntimeCloseStatus::Fault(RuntimeCleanupFault::PriorOutcome), _ => unreachable!() };
        let elapsed_us = row["elapsedUs"].as_u64().unwrap();
        state.status.store(RuntimeCloseStatus::Running.repr(), std::sync::atomic::Ordering::SeqCst);
        runtime_close_publish_turn(&state, named(row["candidate"].as_str().unwrap()), elapsed_us);
        let expected = if elapsed_us >= semio_framework_trace::INTERACTIVE_STEP_CEILING_US { RuntimeCloseStatus::Fault(RuntimeCleanupFault::InteractiveCeiling) } else { named(row["published"].as_str().unwrap()) };
        assert_eq!(RuntimeCloseStatus::from_repr(state.status.load(std::sync::atomic::Ordering::SeqCst)), expected, "{row}");
        assert_eq!(state.last_callback_elapsed_us.load(std::sync::atomic::Ordering::SeqCst), elapsed_us);
    }
}

#[test]
fn instance_lifetime_close_fault_outcome_dominates_complete_progress() {
    use super::super::*;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🚨️fault.fixture.json")).unwrap();
    for row in fixture["terminalPump"].as_array().unwrap() {
        let state = terminal_close_state(row["complete"].as_bool().unwrap(), row["faulted"].as_bool().unwrap(), row["blocked"].as_bool().unwrap());
        let actual = runtime_close_cleanup_pump_one(&state, &mut state.pump.lock().unwrap());
        let expected = match row["status"].as_str().unwrap() { "complete" => RuntimeCloseStatus::Complete, "fault" => RuntimeCloseStatus::Fault(RuntimeCleanupFault::PriorOutcome), "external-wait" => RuntimeCloseStatus::ExternalWait, _ => unreachable!() };
        assert_eq!(actual, expected, "exact terminal outcome {row}");
    }
}

#[test]
fn instance_lifetime_close_optional_monotonic_clock_rejects_missing_and_backward_authority() {
    use super::super::*;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🚨️fault.fixture.json")).unwrap();
    for row in fixture["clocks"].as_array().unwrap() {
        let state = terminal_close_state(true, false, false);
        let readings: Vec<Option<u64>> = row["samples"].as_array().unwrap().iter().map(serde_json::Value::as_u64).collect();
        let mut samples = readings.clone().into_iter();
        run_runtime_close_turn_with_clock(&state, || samples.next().flatten());
        let expected = if row["published"] == "complete" { RuntimeCloseStatus::Complete } else { RuntimeCloseStatus::Fault(expected_clock_cause(&readings)) };
        assert_eq!(RuntimeCloseStatus::from_repr(state.status.load(std::sync::atomic::Ordering::SeqCst)), expected, "{row}");
        assert_eq!(state.pump.lock().unwrap().session.is_none(), row["workEntered"].as_bool().unwrap(), "{row}");
        if !row["workEntered"].as_bool().unwrap() {
            let mut pump = state.pump.lock().unwrap();
            assert!(pump.session.as_ref().unwrap().terminal_is_empty());
            pump.session = None;
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn instance_lifetime_close_preflight_and_shared_restore_preserve_exact_owner() {
    use super::super::*;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🚨️fault.fixture.json")).unwrap();
    let cell = std::sync::Arc::new(RuntimeAppCell::new(crate::app::AppInstance { id: 7, app: TestRuntimeApps::from(query_app().await) }));
    let state = terminal_close_state(true, false, false);
    **state.cell.lock().unwrap() = Some(cell.clone());
    assert_eq!(runtime_close_retire_cell(&state), RuntimeCloseStatus::Fault(RuntimeCleanupFault::InstanceNotDrained));
    assert_eq!(std::sync::Arc::ptr_eq(state.cell.lock().unwrap().as_ref().unwrap(), &cell), fixture["owners"]["nonterminalPreflightKeepsExactAllocation"].as_bool().unwrap());
    {
        let mut instance = cell.instance.lock().unwrap();
        for _ in 0..200_000 { if instance.app.close_terminal_is_empty() { break; } let _ = instance.app.close_step(1, 4096).unwrap(); }
        assert!(instance.app.close_terminal_is_empty());
    }
    let _ = cell.maintenance_pump.lock().unwrap().close_step(1, 4096);
    assert_eq!(runtime_close_retire_cell(&state), RuntimeCloseStatus::ExternalWait);
    assert_eq!(std::sync::Arc::ptr_eq(state.cell.lock().unwrap().as_ref().unwrap(), &cell), fixture["owners"]["sharedRestoreKeepsExactAllocation"].as_bool().unwrap());
    drop(cell);
    assert_eq!(runtime_close_retire_cell(&state), RuntimeCloseStatus::Complete);
    assert!(state.cell.lock().unwrap().is_none());
}

#[test]
fn instance_lifetime_close_contended_pump_keeps_exact_outcome_source() {
    use super::super::*;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🚨️fault.fixture.json")).unwrap();
    let state = terminal_close_state(true, false, false);
    let job = RuntimeCloseCleanupJob { instance_id: 7, state: Some(std::sync::Arc::downgrade(&state)), progress: None, contended: false, closing: false };
    let params = semio_framework_job::BatchJobParams {
        operation: semio_framework_job::OperationId(719), generation: semio_framework_job::Generation(1), cancel: semio_framework_job::CancelToken::root_now(),
        config: semio_framework_job::BatchDriveConfig { site: "exact-close-source-law", stage: semio_framework_job::InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 500 },
        now_us: || Some(0),
    };
    let mut session = match semio_framework_job::BatchJobSession::try_new(job, params) { Ok(session) => session, Err(_) => panic!("fixed source fixture admission") };
    session.step().unwrap();
    let outcome = session.take_outcome().unwrap();
    let identity = match &outcome { semio_framework_job::StepOutcome::Complete(candidate) => candidate.state.single_page().unwrap().as_ptr() as usize, _ => panic!("empty app close produces its exact instance receipt") };
    {
        let mut pump = state.pump.lock().unwrap();
        pump.outcome = Some(outcome);
        pump.session = Some(session);
    }
    let (ready_tx, ready_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    let held = state.clone();
    let holder = std::thread::spawn(move || {
        let _guard = held.pump.lock().unwrap();
        ready_tx.send(()).unwrap();
        let _ = release_rx.recv_timeout(std::time::Duration::from_millis(200));
    });
    ready_rx.recv().unwrap();
    run_runtime_close_turn_with_clock(&state, || Some(10));
    let _ = release_tx.send(());
    holder.join().unwrap();
    let status = RuntimeCloseStatus::from_repr(state.status.load(std::sync::atomic::Ordering::SeqCst));
    let mut pump = state.pump.lock().unwrap();
    let session_preserved = pump.session.is_some();
    let source_preserved = matches!(pump.outcome.as_ref(), Some(semio_framework_job::StepOutcome::Complete(candidate)) if candidate.state.single_page().is_some_and(|bytes| bytes.as_ptr() as usize == identity && bytes == [7, 0, 0, 0]));
    if let Some(outcome) = pump.outcome.as_mut() { while !outcome.terminal_is_empty() { let _ = outcome.close_step(1, 4096); } }
    pump.outcome = None;
    if let Some(session) = pump.session.as_mut() {
        session.begin_close();
        for _ in 0..1_000 { if session.terminal_is_empty() { break; } let _ = session.close_step(1, 4096); }
        assert!(session.terminal_is_empty());
    }
    pump.session = None;
    pump.terminal = false;
    assert_eq!(status, RuntimeCloseStatus::Ready);
    assert_eq!(session_preserved, fixture["owners"]["contendedPumpPreservesSession"].as_bool().unwrap());
    assert_eq!(source_preserved, fixture["owners"]["contendedOutcomePreservesSource"].as_bool().unwrap());
}

async fn close_lease_app(runtime: &crate::plugin_runtime::PluginRuntime<TestRuntimeApps>) {
    let cell = std::sync::Arc::new(super::super::RuntimeAppCell::new(crate::app::AppInstance { id: 7, app: TestRuntimeApps::from(query_app().await) }));
    runtime.instances.borrow_mut().insert_admitted(7, cell);
}

fn drive_close_lease(runtime: &crate::plugin_runtime::PluginRuntime<TestRuntimeApps>, lease: &crate::plugin_runtime::PluginInstanceCloseLease<TestRuntimeApps>) {
    for _ in 0..200_000 {
        if let Err(error) = crate::plugin_runtime::plugin_step_close_cleanup(runtime) {
            let entries = runtime.close_quarantine.borrow();
            let state = &entries.get(7).unwrap().state;
            let pump = state.pump.lock().unwrap();
            let fault = state.last_fault.lock().unwrap();
            let length = fault.iter().position(|byte| *byte == 0).unwrap_or(fault.len());
            let phases = state.callback_phase_us.each_ref().map(|phase| phase.load(std::sync::atomic::Ordering::SeqCst));
            panic!("[DEBUG] exact close {error:?}: generation={} origin={} elapsed={} phases={phases:?} stalled={} terminal={} complete={} blocked={} faulted={} pending={:?} detail={}", state.generation.0, state.last_fault_origin.load(std::sync::atomic::Ordering::SeqCst), state.last_callback_elapsed_us.load(std::sync::atomic::Ordering::SeqCst), state.stalled_steps.load(std::sync::atomic::Ordering::SeqCst), pump.terminal, pump.complete, pump.blocked, pump.faulted, pump.pending_status, String::from_utf8_lossy(&fault[..length]));
        }
        if lease.is_retired().unwrap() && runtime.close_quarantine.borrow().get(7).is_none() { return; }
        std::thread::yield_now();
    }
    panic!("exact app close witness did not reach terminal emptiness");
}

#[semio_framework_async_macros::async_test]
async fn instance_lifetime_close_witness_survives_quarantine_removal_and_reused_id() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fixture/🔣️.json")).unwrap();
    let runtime = crate::plugin_runtime::PluginRuntime::<TestRuntimeApps>::new();
    close_lease_app(&runtime).await;
    let mut lease = crate::plugin_runtime::plugin_capture_instance_close(&runtime, 7).unwrap();
    assert!(!lease.is_retired().unwrap());
    lease.begin_close(&runtime).unwrap();
    assert_eq!(lease.close_generation(), Some(1));
    drive_close_lease(&runtime, &lease);
    assert_eq!(lease.is_retired().unwrap(), fixture["nativeCases"]["quarantineRemovalPreservesTerminalWitness"].as_bool().unwrap());
    close_lease_app(&runtime).await;
    lease.begin_close(&runtime).unwrap();
    assert_eq!(runtime.instances.borrow().get(7).is_some(), fixture["nativeCases"]["repeatCloseKeepsReplacement"].as_bool().unwrap());
    let mut fresh = crate::plugin_runtime::plugin_capture_instance_close(&runtime, 7).unwrap();
    fresh.begin_close(&runtime).unwrap();
    assert_eq!(fresh.close_generation(), Some(2));
    drive_close_lease(&runtime, &fresh);
}

#[semio_framework_async_macros::async_test]
async fn instance_lifetime_close_constructs_worker_shell_before_exact_live_detachment() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🚪️lifetime/🏗️construction.json")).unwrap();
    let runtime = crate::plugin_runtime::PluginRuntime::<TestRuntimeApps>::new();
    close_lease_app(&runtime).await;
    let mut lease = crate::plugin_runtime::plugin_capture_instance_close(&runtime, 7).unwrap();
    super::super::RUNTIME_CLOSE_CONSTRUCTION_LIVE.with(|probe| probe.set(None));
    lease.begin_close(&runtime).unwrap();
    let constructed_while_live = super::super::RUNTIME_CLOSE_CONSTRUCTION_LIVE.with(|probe| probe.take());
    let live_after = runtime.instances.borrow().get(7).is_some();
    let quarantined_after = runtime.close_quarantine.borrow().get(7).is_some();
    drive_close_lease(&runtime, &lease);
    assert_eq!(constructed_while_live, fixture["atWorkerShellExactLiveAllocation"].as_bool());
    assert_eq!(live_after, fixture["states"][2]["liveOwner"].as_bool().unwrap());
    assert_eq!(quarantined_after, fixture["states"][2]["quarantineOwner"].as_bool().unwrap());
}

#[semio_framework_async_macros::async_test]
async fn instance_lifetime_close_rejects_foreign_root_and_exhaustion_before_detach() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fixture/🔣️.json")).unwrap();
    let runtime = crate::plugin_runtime::PluginRuntime::<TestRuntimeApps>::new();
    close_lease_app(&runtime).await;
    let mut old = crate::plugin_runtime::plugin_capture_instance_close(&runtime, 7).unwrap();
    let displaced = runtime.instances.borrow_mut().take(7).unwrap();
    close_lease_app(&runtime).await;
    assert_eq!(old.begin_close(&runtime).is_err(), fixture["nativeCases"]["sameIdReplacementRejected"].as_bool().unwrap());
    assert!(runtime.instances.borrow().get(7).is_some());
    assert!(runtime.close_quarantine.borrow().is_empty());
    let mut fresh = crate::plugin_runtime::plugin_capture_instance_close(&runtime, 7).unwrap();
    runtime.close_generation.set(u64::MAX);
    assert!(fresh.begin_close(&runtime).is_err());
    assert_eq!(runtime.instances.borrow().get(7).is_some(), fixture["nativeCases"]["overflowPreservesLiveOwner"].as_bool().unwrap());
    runtime.close_generation.set(0);
    fresh.begin_close(&runtime).unwrap();
    drive_close_lease(&runtime, &fresh);
    runtime.instances.borrow_mut().insert_admitted(7, displaced);
    old.begin_close(&runtime).unwrap();
    drive_close_lease(&runtime, &old);
}

#[semio_framework_async_macros::async_test]
async fn instance_lifetime_close_construction_failure_preserves_original_live_root() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🚪️lifetime/🏗️construction.json")).unwrap();
    let law = &fixture["constructionFailure"];
    let runtime = crate::plugin_runtime::PluginRuntime::<TestRuntimeApps>::new();
    close_lease_app(&runtime).await;
    let rescue = runtime.instances.borrow().get(7).unwrap().clone();
    let generation = runtime.close_generation.get();
    let mut lease = crate::plugin_runtime::plugin_capture_instance_close(&runtime, 7).unwrap();
    super::super::RUNTIME_CLOSE_CONSTRUCTION_FAIL.with(|fail| fail.set(true));
    let fault = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| lease.begin_close(&runtime)));
    let live = runtime.instances.borrow().get(7).is_some();
    let same = runtime.instances.borrow().get(7).is_some_and(|owner| std::sync::Arc::ptr_eq(owner, &rescue));
    let generation_unchanged = runtime.close_generation.get() == generation;
    let quarantine_empty = runtime.close_quarantine.borrow().is_empty();
    if !live { runtime.instances.borrow_mut().insert_admitted(7, rescue.clone()); }
    drop(rescue);
    lease.begin_close(&runtime).unwrap();
    drive_close_lease(&runtime, &lease);
    assert!(fault.is_err());
    assert!(quarantine_empty);
    assert_eq!(live, law["liveOwner"].as_bool().unwrap());
    assert_eq!(same, law["sameAllocation"].as_bool().unwrap());
    assert_eq!(generation_unchanged, law["generationUnchanged"].as_bool().unwrap());
}
//#endregion 🚪️ExactRuntimeCloseLease

async fn query_command(runtime: &crate::plugin_runtime::PluginRuntime<TestRuntimeApps>, seq: u64, command: protocol::LocalInteractionQueryCommand) -> Vec<protocol::AppFrame> {
    wire_command(runtime, seq, protocol::AppCommand::LocalInteractionQuery { seq, command }).await
}

async fn wire_command(runtime: &crate::plugin_runtime::PluginRuntime<TestRuntimeApps>, seq: u64, command: protocol::AppCommand) -> Vec<protocol::AppFrame> {
    let encoded = protocol::encode_app_command(&command).await.unwrap();
    run_ingress(runtime, seq, crate::plugin_runtime::PluginCommandIngress::Encoded(encoded)).await
}

async fn cold_decoded_command(runtime: &crate::plugin_runtime::PluginRuntime<TestRuntimeApps>, seq: u64, command: protocol::AppCommand) -> Vec<protocol::AppFrame> {
    run_ingress(runtime, seq, crate::plugin_runtime::PluginCommandIngress::Decoded(protocol::DecodedAppCommandOwner::new(command))).await
}

async fn run_ingress(runtime: &crate::plugin_runtime::PluginRuntime<TestRuntimeApps>, seq: u64, ingress: crate::plugin_runtime::PluginCommandIngress) -> Vec<protocol::AppFrame> {
    let mut input = Some((seq, ingress));
    let mut frames = Vec::new();
    for _ in 0..100 {
        let output = crate::plugin_runtime::plugin_exchange(runtime, 7, input).await.unwrap();
        for bytes in output.frames { frames.push(protocol::decode_app_frame(&bytes).await.unwrap()); }
        input = output.retry_command;
        if input.is_none() { return frames; }
    }
    panic!("fixed local query command did not leave ingress");
}

#[semio_framework_async_macros::async_test]
async fn local_interaction_cold_transaction_receipts_and_encoded_route_rejection() {
    let runtime = crate::plugin_runtime::PluginRuntime::<TestRuntimeApps>::new();
    let cell = std::sync::Arc::new(super::super::RuntimeAppCell::new(crate::app::AppInstance { id: 7, app: TestRuntimeApps::from(query_app().await) }));
    runtime.instances.borrow_mut().insert_admitted(7, cell.clone());
    let denied = wire_command(&runtime, 0, protocol::AppCommand::TransactionPrepare { seq: 0, txn_id: "denied".into(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: Vec::new(), label: String::new(), origin: Vec::new() }).await;
    let fault = denied.iter().find_map(|frame| match frame { protocol::AppFrame::Error { in_reply_to: Some(0), fault, .. } => Some(fault), _ => None }).expect("encoded transaction route must remain explicitly unadmitted");
    let fault: Fault = super::super::decode_wire_serialized(fault).await.unwrap();
    assert_eq!(fault.code.0, "plugin.command-route-state-machine-required");
    assert!(!denied.iter().any(|frame| matches!(frame, protocol::AppFrame::Done { .. })));
    for (prepare_seq, finish_seq, txn_id, commit) in [(1, 2, "receipt-commit", true), (3, 4, "receipt-rollback", false)] {
        let operation = <TestMutation as protocol::OpBinary>::encode_op(&TestMutation::SetCount(SetCount { value: prepare_seq as i32 })).unwrap();
        let prepared = cold_decoded_command(&runtime, prepare_seq, protocol::AppCommand::TransactionPrepare { seq: prepare_seq, txn_id: txn_id.into(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: vec![operation], label: "receipt fixture".into(), origin: Vec::new() }).await;
        assert_eq!(prepared.iter().filter(|frame| matches!(frame, protocol::AppFrame::Done { in_reply_to } if *in_reply_to == prepare_seq)).count(), 1, "[DEBUG] transaction prepare seq={prepare_seq} frames={prepared:?}");
        assert!(prepared.iter().any(|frame| matches!(frame, protocol::AppFrame::TransactionPrepared { txn_id: actual, rejection, .. } if actual == txn_id && rejection.is_empty())));
        let command = if commit { protocol::AppCommand::TransactionCommit { seq: finish_seq, txn_id: txn_id.into() } } else { protocol::AppCommand::TransactionRollback { seq: finish_seq, txn_id: txn_id.into() } };
        let finished = cold_decoded_command(&runtime, finish_seq, command).await;
        assert_eq!(finished.iter().filter(|frame| matches!(frame, protocol::AppFrame::Done { in_reply_to } if *in_reply_to == finish_seq)).count(), 1, "[DEBUG] transaction finish seq={finish_seq} frames={finished:?}");
        assert!(finished.iter().any(|frame| match frame {
            protocol::AppFrame::TransactionCommitted { txn_id: actual, edit_id } => commit && actual == txn_id && !edit_id.is_empty(),
            protocol::AppFrame::TransactionRolledBack { txn_id: actual } => !commit && actual == txn_id,
            _ => false,
        }));
    }
    let rejected = cold_decoded_command(&runtime, 5, protocol::AppCommand::TransactionCommit { seq: 5, txn_id: "absent".into() }).await;
    assert!(rejected.iter().any(|frame| matches!(frame, protocol::AppFrame::Error { in_reply_to: Some(5), .. })));
    assert!(!rejected.iter().any(|frame| matches!(frame, protocol::AppFrame::Done { .. })));
    drop(cell);
    crate::plugin_runtime::plugin_destroy_app(&runtime, 7).await.unwrap();
    for _ in 0..200_000 {
        crate::plugin_runtime::plugin_step_close_cleanup(&runtime).unwrap();
        if runtime.close_quarantine.borrow().get(7).is_none() { break; }
        std::thread::yield_now();
    }
    assert!(runtime.close_quarantine.borrow().get(7).is_none());
}

async fn query_app() -> VcsArtifactApp<TestApp> {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧫️fixtures/🏠️local-interaction/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["id"] == "semantic-unicode-over-page").unwrap();
    let mut state = row["expected"].clone(); state["hover"] = serde_json::json!({});
    let state: protocol::InteractionState = serde_json::from_value(state).unwrap();
    let envelope = store::create_document_envelope::<protocol::InteractionState, crate::app::InteractionConfigMutation>("framework.interaction", "query-dispatch", state, None);
    let mut interaction = store::ArtifactStore::new(envelope).await.unwrap();
    interaction.install_member_store_owners_exact(crate::local_interaction::retirement::interaction_store_owners());
    let mut app = interaction_app_under_test().await;
    let mut previous = std::mem::replace(&mut app.interaction_store, interaction);
    for _ in 0..10_000 { if previous.close_owned_step(1, 4096).unwrap() == store::SnapshotRetirementStep::Complete { break; } }
    assert!(previous.close_owned_terminal_is_empty());
    app.bind_instance_id(7).await;
    app
}

#[semio_framework_async_macros::async_test]
async fn local_interaction_registered_query_channel_continuation_ack_and_close() {
    let runtime = crate::plugin_runtime::PluginRuntime::<TestRuntimeApps>::new();
    let cell = std::sync::Arc::new(super::super::RuntimeAppCell::new(crate::app::AppInstance { id: 7, app: TestRuntimeApps::from(query_app().await) }));
    runtime.instances.borrow_mut().insert_admitted(7, cell.clone());
    let mut pending = std::collections::VecDeque::from(query_command(&runtime, 1, protocol::LocalInteractionQueryCommand::Read { request_id: 13 }).await);
    let mut expected_identity = None;
    let mut output = Vec::new();
    let mut closed = false;
    let mut seq = 1;
    let mut pages = 0;
    let mut received_receipt = 0;
    let mut ephemeral = 0;
    for _ in 0..200_000 {
        if pending.is_empty() {
            let (next, _) = crate::plugin_runtime::plugin_continue_typed_operations(&runtime).await.unwrap();
            if let Some((instance, batch)) = next {
                assert_eq!(instance, 7);
                for bytes in batch.frames { pending.push_back(protocol::decode_app_frame(&bytes).await.unwrap()); }
            }
        }
        if let Some(frame) = pending.pop_front() {
                match frame {
                    protocol::AppFrame::Done { in_reply_to } => { received_receipt += 1; assert_eq!(in_reply_to, received_receipt); },
                    protocol::AppFrame::Ephemeral { presence, presence_generation, transient_generation, interaction } => {
                        assert!(presence.is_empty()); assert!(interaction.is_empty());
                        assert_eq!(presence_generation, 0); assert_eq!(transient_generation, 0);
                        ephemeral += 1;
                    },
                    protocol::AppFrame::LocalInteractionQuery { reply: protocol::LocalInteractionQueryReply::Started { token } } => { assert!(expected_identity.replace(token.identity).is_none()); },
                    protocol::AppFrame::LocalInteractionQuery { reply: protocol::LocalInteractionQueryReply::Page { page } } => {
                        assert_eq!(Some(&page.identity), expected_identity.as_ref());
                        assert!(page.bytes.len() <= 256);
                        output.extend_from_slice(&page.bytes);
                        pages += 1;
                        let token = protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity, ordinal: page.ordinal };
                        seq += 1;
                        pending.extend(query_command(&runtime, seq, protocol::LocalInteractionQueryCommand::Acknowledge { token }).await);
                    },
                    protocol::AppFrame::LocalInteractionQuery { reply: protocol::LocalInteractionQueryReply::Closed { cancelled: false, .. } } => closed = true,
                    frame => panic!("unexpected registered query frame {frame:?}"),
                }
        }
        if closed { break; }
    }
    assert!(closed);
    assert!(pending.is_empty());
    assert!(ephemeral > 0);
    assert_eq!(received_receipt, seq);
    assert!(pages > 16 && output.len() > 4096);
    let capture: protocol::LocalInteractionCapture = protocol::json::from_json_str(std::str::from_utf8(&output).unwrap()).unwrap();
    assert_eq!(Some(capture.identity), expected_identity);
    assert!(!cell.instance.lock().unwrap().app.has_pending_typed_operations());
    drop(cell);
    crate::plugin_runtime::plugin_destroy_app(&runtime, 7).await.unwrap();
    for _ in 0..200_000 {
        crate::plugin_runtime::plugin_step_close_cleanup(&runtime).unwrap();
        if runtime.close_quarantine.borrow().get(7).is_none() { break; }
        std::thread::yield_now();
    }
    assert!(runtime.close_quarantine.borrow().get(7).is_none());
}
//#endregion 📡️RegisteredQueryDispatch
