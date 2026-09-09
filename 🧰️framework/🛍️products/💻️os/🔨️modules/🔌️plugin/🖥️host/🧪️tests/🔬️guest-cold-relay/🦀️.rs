use super::*;
use semio_framework_job::InteractiveJob;

fn relay_lifecycle_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/♻️relay-lifecycle.json")).expect("relay lifecycle fixture")
}

#[test]
fn neutral_relay_lifecycle_traces_drive_production_machines() {
    let _replay_authority = shard::replay_test_authority();
    for trace in relay_lifecycle_fixture()["traces"].as_array().expect("relay lifecycle traces") {
        let actual: serde_json::Value = serde_json::from_str(&exercise_relay_lifecycle_trace(&trace.to_string()).expect("production lifecycle trace")).expect("production lifecycle projection");
        assert_eq!(actual, trace["expected"], "{} production projection", trace["🪪️id"]);
    }
}

#[semio_framework_async_macros::async_test]
async fn retained_pool_future_retries_saturation_once_and_terminalizes_shutdown() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let (entered_sender, entered_receiver) = std::sync::mpsc::sync_channel(0);
    let (release_sender, release_receiver) = std::sync::mpsc::sync_channel(0);
    pool.submit(
        Lane::UserVisible,
        Box::new(move || {
            entered_sender.send(()).expect("blocked worker announces entry");
            release_receiver.recv().expect("blocked worker release");
        }),
    );
    entered_receiver.recv().expect("worker enters the blocking closure");
    for _ in 0..semio_framework_async::WORKER_JOBS_PER_LANE {
        pool.submit(Lane::UserVisible, Box::new(|| {}));
    }
    let (completion_sender, completion_receiver) = semio_framework_async::oneshot::channel();
    let failures = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let failure_count = Arc::clone(&failures);
    GuestRelayPoolFuture::spawn_recoverable(
        pool.clone(),
        Lane::UserVisible,
        async move {
            let _ = completion_sender.send(b"retried".to_vec());
        },
        move |_| {
            failure_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        },
    );
    release_sender.send(()).expect("release saturated worker");
    assert_eq!(completion_receiver.await.expect("retained future completion after saturation"), b"retried");
    assert_eq!(failures.load(std::sync::atomic::Ordering::SeqCst), 0);
    pool.shutdown().expect("worker shutdown");

    let ran = Arc::new(AtomicBool::new(false));
    let future_ran = Arc::clone(&ran);
    let (failure_sender, failure_receiver) = semio_framework_async::oneshot::channel();
    GuestRelayPoolFuture::spawn_recoverable(
        pool.clone(),
        Lane::UserVisible,
        async move {
            future_ran.store(true, std::sync::atomic::Ordering::SeqCst);
        },
        move |failure| {
            let _ = failure_sender.send(failure);
        },
    );
    assert!(matches!(failure_receiver.await.expect("shutdown terminal failure"), GuestRelayPoolFailure::Admission(semio_framework_async::WorkerSubmitErrorKind::Shutdown)));
    assert!(!ran.load(std::sync::atomic::Ordering::SeqCst));
    let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool));
    let (index, generation) = registry.reserve().expect("pre-failure mounted slot");
    registry.mount(index, generation, GuestRelayMountedOwner::Empty);
    registry.detach(index, generation);
    assert!(registry.reaper_failed.load(std::sync::atomic::Ordering::Acquire));
    assert!(matches!(&*registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Empty));
    assert!(registry.reserve().is_none(), "a scheduler-failed registry must reject future mounts");

    let mock = Arc::new(MockGuestRuntime::new().await);
    let handle = mounted_handle(Arc::clone(&mock), RuntimeActorId(8_017)).await;
    let oversized = vec![0; semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES + 1];
    let error = handle.infer(&oversized).await.expect_err("oversized relay input");
    assert!(error.to_string().contains("input exceeds the retained operation byte limit"));
    assert_eq!(mock.start_admissions(), 0);
    let mut maximum_rejected_job = GuestColdRelayJob::new(
        Arc::clone(&handle.runtime),
        Arc::clone(&handle.instance),
        Arc::clone(&handle.instance_gate),
        registry.pool.clone(),
        semio_framework_async::CancelToken::root_now(),
        1,
        ("semio.infer".to_string(), vec![0; semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES]),
    );
    maximum_rejected_job.begin_close();
    for _ in 0..semio_framework_job::JOB_PAYLOAD_OPERATION_PAGES.saturating_add(3) {
        if matches!(maximum_rejected_job.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {
            break;
        }
    }
    assert!(maximum_rejected_job.terminal_is_empty(), "maximum admitted pre-start relay closes within the fatal rejection bound");
}

/// 🧵️ Keeps fixed mounted capacity on the heap while every one-opportunity owner remains small
/// enough for the native test stack on every supported architecture.
#[test]
fn mounted_relay_stack_authority_matches_the_neutral_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧱️stack-authority.json")).expect("stack authority fixture");
    let maximum = fixture["maximumInlineBytes"].as_u64().expect("maximum inline bytes") as usize;
    let registry = fixture["registries"].as_array().expect("registry rows").iter().find(|row| row["id"] == "guest-relay-mounted").expect("mounted relay row");
    assert_eq!(fixture["schemaVersion"], 1);
    assert_eq!(registry["capacity"].as_u64(), Some(GUEST_RELAY_MOUNTED_SLOTS as u64));
    assert_eq!(registry["storage"], "heap");
    assert_eq!(GuestRelayMountedRegistry::new().slots.len(), GUEST_RELAY_MOUNTED_SLOTS);
    assert!(size_of::<GuestRelayMountedSlot>() <= maximum);
    assert!(size_of::<GuestRelayMountedRegistry>() <= maximum);
}

#[derive(Debug, PartialEq, Eq)]
enum TestRelayOutcome {
    Yield,
    Complete(Vec<u8>),
    Cancelled,
    Fault(Vec<u8>),
}

fn admit_test_relay(relay: GuestColdRelayJob, params: semio_framework_job::BatchJobParams) -> semio_framework_job::WorkerJobSession<GuestColdRelayJob> {
    match semio_framework_job::WorkerJobSession::try_new(relay, params) {
        Ok(session) => session,
        Err(mut rejected) => {
            rejected.begin_close();
            while !rejected.terminal_is_empty() {
                let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
            panic!("relay test session admission");
        }
    }
}

fn test_payload_bytes(payload: &semio_framework_job::RetainedJobPayload) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(payload.len());
    for page in 0..payload.page_count() {
        if let Some(page) = payload.page(page) {
            bytes.extend_from_slice(page);
        }
    }
    bytes
}

async fn test_relay_step(session: &semio_framework_job::WorkerJobSession<GuestColdRelayJob>, _pool: &WorkerPool) -> TestRelayOutcome {
    if matches!(session.poll(), semio_framework_job::WorkerJobPoll::Closing | semio_framework_job::WorkerJobPoll::TerminalEmpty) {
        let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        return TestRelayOutcome::Yield;
    }
    let (ticket, poll) = session.try_step_on_caller().expect("relay test caller opportunity");
    let mut owner = match poll {
        semio_framework_job::WorkerJobPoll::Outcome => session.take_outcome(ticket).expect("relay test outcome"),
        semio_framework_job::WorkerJobPoll::Terminal => session.take_terminal().expect("relay test terminal"),
        _ => return TestRelayOutcome::Yield,
    };
    let mut outcome = owner.take_outcome();
    let result = match &outcome {
        semio_framework_job::StepOutcome::Complete(candidate) => TestRelayOutcome::Complete(test_payload_bytes(&candidate.output)),
        semio_framework_job::StepOutcome::Cancelled => TestRelayOutcome::Cancelled,
        semio_framework_job::StepOutcome::Fault(fault) => TestRelayOutcome::Fault(test_payload_bytes(&fault.detail)),
        semio_framework_job::StepOutcome::Yield | semio_framework_job::StepOutcome::PreviewReady(_) | semio_framework_job::StepOutcome::CheckpointReady(_) => TestRelayOutcome::Yield,
    };
    while !outcome.terminal_is_empty() {
        let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
    if outcome.is_terminal() {
        owner.begin_close();
    } else if let Err(owner) = owner.resume() {
        owner.begin_close();
    }
    result
}

#[test]
fn guest_cold_relay_registry_max_plus_one_generation_and_zero_pump_are_exact() {
    let registry = GuestRelayMountedRegistry::new();
    let mut generations = Vec::new();
    for _ in 0..GUEST_RELAY_MOUNTED_SLOTS {
        let (index, generation) = registry.reserve().expect("exact mounted relay capacity");
        let output_identity = {
            let slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let GuestRelayMountedSlot::Reserved { output, .. } = &*slot else { panic!("reserved output owner") };
            output.storage.as_ptr()
        };
        generations.push((index, generation, output_identity));
    }
    assert!(registry.reserve().is_none());
    assert!(generations.windows(2).all(|pair| pair[0].1 < pair[1].1));
    assert!(!registry.has_detached());
    for (index, generation, output_identity) in generations {
        let mut slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let GuestRelayMountedSlot::Reserved { generation: actual, output } = &*slot else { panic!("reserved output remains mounted") };
        assert_eq!(*actual, generation);
        assert_eq!(output.storage.as_ptr(), output_identity);
        *slot = GuestRelayMountedSlot::Empty;
    }
    let exhausted = GuestRelayMountedRegistry::new();
    exhausted.next_generation.store(u64::MAX, std::sync::atomic::Ordering::Release);
    let (index, generation) = exhausted.reserve().expect("last generation remains admissible");
    assert_eq!(generation, u64::MAX);
    *exhausted.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestRelayMountedSlot::Empty;
    assert!(exhausted.reserve().is_none());
}

fn mounted_test_session(generation: u64, lifecycle: GuestRelayMountedLifecycle, output: &[u8]) -> GuestRelayMountedSession {
    let mut mounted_output = GuestRelayMountedOutput::new();
    mounted_output.write_page(output).expect("mounted test output");
    GuestRelayMountedSession {
        generation,
        owner: GuestRelayMountedOwner::Empty,
        checked_out: None,
        lifecycle_probe_checked_out: None,
        outcome: None,
        outcome_page: 0,
        output: mounted_output,
        terminal: Some(GuestRelayMountedTerminal::Complete),
        lifecycle,
    }
}

#[test]
fn detached_reaper_reclaims_one_slot_per_opportunity_round_robin_and_refuses_stale_generation() {
    let fixture = relay_lifecycle_fixture();
    assert_eq!(fixture["schemaVersion"], 1);
    assert_eq!(fixture["capacities"]["mountedRelaySlots"].as_u64(), Some(GUEST_RELAY_MOUNTED_SLOTS as u64));
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool.clone()));
    let mut generations = Vec::new();
    for _ in 0..GUEST_RELAY_MOUNTED_SLOTS {
        let (index, generation) = registry.reserve().expect("full detached fixture capacity");
        registry.mount(index, generation, GuestRelayMountedOwner::Empty);
        let mut slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let GuestRelayMountedSlot::Mounted(session) = &mut *slot else { panic!("mounted fixture") };
        session.lifecycle = GuestRelayMountedLifecycle::DetachedForReap;
        generations.push((index, generation));
    }
    for released in 1..=GUEST_RELAY_MOUNTED_SLOTS {
        assert!(matches!(registry.reap_one(std::task::Waker::noop()), GuestRelayMountedReap::Progress));
        let empty = registry.slots.iter().filter(|slot| matches!(&*slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Empty)).count();
        assert_eq!(empty, released, "one reaper opportunity releases one rotating slot");
    }
    assert!(matches!(registry.reap_one(std::task::Waker::noop()), GuestRelayMountedReap::Idle));
    let (index, generation) = registry.reserve().expect("reclaimed capacity is reusable");
    registry.mount(index, generation, GuestRelayMountedOwner::Empty);
    registry.detach(index, generations[0].1);
    let slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    assert!(matches!(&*slot, GuestRelayMountedSlot::Mounted(session) if session.generation == generation && session.lifecycle == GuestRelayMountedLifecycle::Running));
    drop(slot);
    *registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestRelayMountedSlot::Empty;
    pool.shutdown().expect("worker shutdown");
}

#[test]
fn detached_reaper_never_steals_a_live_draining_callers_exact_output() {
    let fixture = relay_lifecycle_fixture();
    let expected = fixture["traces"].as_array().expect("fixture traces").iter().find(|trace| trace["🪪️id"] == "relay-live-terminal-caller-output").and_then(|trace| trace["expected"]["callerOutput"].as_str()).expect("caller output trace");
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool.clone()));
    let (detached_index, detached_generation) = registry.reserve().expect("detached slot");
    *registry.slots[detached_index].lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestRelayMountedSlot::Mounted(mounted_test_session(detached_generation, GuestRelayMountedLifecycle::DetachedForReap, &[]));
    let (live_index, live_generation) = registry.reserve().expect("live slot");
    *registry.slots[live_index].lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestRelayMountedSlot::Mounted(mounted_test_session(live_generation, GuestRelayMountedLifecycle::DrainingForCaller, expected.as_bytes()));

    assert!(matches!(registry.reap_one(std::task::Waker::noop()), GuestRelayMountedReap::Progress));
    assert!(matches!(&*registry.slots[detached_index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Empty));
    assert!(matches!(&*registry.slots[live_index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Mounted(session) if session.lifecycle == GuestRelayMountedLifecycle::DrainingForCaller));
    assert!(matches!(registry.pump(live_index, live_generation, std::task::Waker::noop()), std::task::Poll::Ready(Ok(bytes)) if bytes == expected.as_bytes()));
    pool.shutdown().expect("worker shutdown");
}

#[test]
fn guest_cold_relay_publication_max_plus_one_selects_retained_fault_without_losing_source() {
    let mut maximum = GuestRelayPublication::new(GuestRelayPublicationKind::Commit, vec![7; semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES]);
    assert!(!maximum.oversized);
    assert_eq!(maximum.source.len(), semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES);
    let mut plus_one = GuestRelayPublication::new(GuestRelayPublicationKind::Commit, vec![9; semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES + 1]);
    assert!(plus_one.oversized);
    assert_eq!(plus_one.source.len(), semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES + 1);
    assert!(matches!(plus_one.kind, GuestRelayPublicationKind::Fault));
    maximum.begin_close();
    plus_one.begin_close();
    while !maximum.terminal_is_empty() {
        let _ = maximum.close_step(1, semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES + 1);
    }
    while !plus_one.terminal_is_empty() {
        let _ = plus_one.close_step(1, semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES + 1);
    }
}

async fn relay_session_with_tokens(
    mock: Arc<MockGuestRuntime>,
    actor: RuntimeActorId,
    pool: WorkerPool,
    relay_cancel: semio_framework_async::CancelToken,
    session_cancel: semio_framework_async::CancelToken,
    step: JobStep,
) -> (semio_framework_job::WorkerJobSession<GuestColdRelayJob>, Arc<MockJobStepGate>, Arc<Mutex<GuestInstanceSlot>>) {
    let compiled = mock.compile(&PackageRef { package: PackageId("relay-test".to_string()), hash: PackageHash([41; 32]) }, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4_096, max_frames: 1 }).await.expect("mock instantiate");
    let instance = Arc::new(Mutex::new(GuestInstanceSlot::Available(instance)));
    let gate = mock.script_pending_job_step(actor, step).await;
    let relay = GuestColdRelayJob::new(Arc::new(GuestRuntimes::Mock(mock)), Arc::clone(&instance), Arc::new(semio_framework_async::Semaphore::new(1)), pool, relay_cancel, 1, ("semio.infer".to_string(), b"request".to_vec()));
    let params = semio_framework_job::BatchJobParams {
        operation: semio_framework_job::OperationId(actor.0),
        generation: semio_framework_job::Generation(1),
        cancel: session_cancel,
        config: semio_framework_job::BatchDriveConfig {
            site: "test.plugin-host.guest-cold-relay",
            stage: semio_framework_job::InteractiveStage::UserVisibleSimStep,
            fuel_per_step: semio_framework_job::USER_VISIBLE_LANE_FUEL,
            step_budget_us: semio_framework_job::USER_VISIBLE_LANE_WALL_US,
        },
        now_us: semio_framework_job::default_now_us,
    };
    (admit_test_relay(relay, params), gate, instance)
}

async fn relay_session(
    mock: Arc<MockGuestRuntime>,
    actor: RuntimeActorId,
    pool: WorkerPool,
    cancel: semio_framework_async::CancelToken,
    step: JobStep,
) -> (semio_framework_job::WorkerJobSession<GuestColdRelayJob>, Arc<MockJobStepGate>, Arc<Mutex<GuestInstanceSlot>>) {
    relay_session_with_tokens(mock, actor, pool, cancel.clone(), cancel, step).await
}

async fn mounted_handle(mock: Arc<MockGuestRuntime>, actor: RuntimeActorId) -> PluginInstanceHandle {
    let compiled = mock.compile(&PackageRef { package: PackageId("mounted-relay-test".to_string()), hash: PackageHash([43; 32]) }, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4_096, max_frames: 1 }).await.expect("mock instantiate");
    PluginInstanceHandle::new(actor, Arc::new(GuestRuntimes::Mock(mock)), instance).await
}

async fn relay_session_for_handle(
    handle: &PluginInstanceHandle,
    mock: &MockGuestRuntime,
    pool: WorkerPool,
    relay_cancel: semio_framework_async::CancelToken,
    session_cancel: semio_framework_async::CancelToken,
    step: JobStep,
) -> (semio_framework_job::WorkerJobSession<GuestColdRelayJob>, Arc<MockJobStepGate>) {
    let gate = mock.script_pending_job_step(handle.actor, step).await;
    let relay = GuestColdRelayJob::new(Arc::clone(&handle.runtime), Arc::clone(&handle.instance), Arc::clone(&handle.instance_gate), pool, relay_cancel, 77, ("semio.infer".to_string(), b"request".to_vec()));
    let params = semio_framework_job::BatchJobParams {
        operation: semio_framework_job::OperationId(handle.actor.0),
        generation: semio_framework_job::Generation(77),
        cancel: session_cancel,
        config: semio_framework_job::BatchDriveConfig {
            site: "test.plugin-host.guest-cold-relay.mounted",
            stage: semio_framework_job::InteractiveStage::UserVisibleSimStep,
            fuel_per_step: semio_framework_job::USER_VISIBLE_LANE_FUEL,
            step_budget_us: semio_framework_job::USER_VISIBLE_LANE_WALL_US,
        },
        now_us: semio_framework_job::default_now_us,
    };
    (admit_test_relay(relay, params), gate)
}

async fn drive_until_step_admitted(session: &semio_framework_job::WorkerJobSession<GuestColdRelayJob>, pool: &WorkerPool, mock: &MockGuestRuntime) {
    for _ in 0..64 {
        assert_eq!(test_relay_step(session, pool).await, TestRelayOutcome::Yield);
        if mock.step_admissions() == 1 {
            return;
        }
        pool_timer_barrier(pool).await;
    }
    panic!("pending guest step was not admitted");
}

async fn wait_for_cancel_admission(pool: &WorkerPool, mock: &MockGuestRuntime) {
    for _ in 0..64 {
        let _ = semio_framework_job::pump_worker_job_retirements(1, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        if mock.cancel_admissions() == 1 {
            return;
        }
        let (sender, receiver) = semio_framework_async::oneshot::channel();
        pool.submit_at(
            pool.now_ms().saturating_add(1),
            Lane::UserVisible,
            Box::new(move || {
                let _ = sender.send(());
            }),
        );
        receiver.await.expect("timer barrier");
    }
    panic!("guest cancellation was not admitted");
}

async fn wait_for_quarantine(pool: &WorkerPool, instance: &Arc<Mutex<GuestInstanceSlot>>) {
    for _ in 0..64 {
        if instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_quarantined() {
            return;
        }
        let (sender, receiver) = semio_framework_async::oneshot::channel();
        pool.submit_at(
            pool.now_ms().saturating_add(1),
            Lane::UserVisible,
            Box::new(move || {
                let _ = sender.send(());
            }),
        );
        receiver.await.expect("quarantine timer barrier");
    }
    panic!("guest instance was not quarantined");
}

async fn wait_for_available_instance(pool: &WorkerPool, instance: &Arc<Mutex<GuestInstanceSlot>>) {
    for _ in 0..64 {
        if instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_available() {
            return;
        }
        let (sender, receiver) = semio_framework_async::oneshot::channel();
        pool.submit_at(
            pool.now_ms().saturating_add(1),
            Lane::UserVisible,
            Box::new(move || {
                let _ = sender.send(());
            }),
        );
        receiver.await.expect("instance restoration timer barrier");
    }
    panic!("guest instance was not restored");
}

async fn wait_for_release_barrier(pool: &WorkerPool, barrier: &MockJobStepGate) {
    for _ in 0..64 {
        if barrier.is_waiting() {
            return;
        }
        let (sender, receiver) = semio_framework_async::oneshot::channel();
        pool.submit_at(
            pool.now_ms().saturating_add(1),
            Lane::UserVisible,
            Box::new(move || {
                let _ = sender.send(());
            }),
        );
        receiver.await.expect("release barrier timer");
    }
    panic!("guest relay did not reach its post-release barrier");
}

fn spawn_mounted_infer(pool: WorkerPool, handle: Arc<PluginInstanceHandle>, request: &'static [u8]) -> semio_framework_async::oneshot::Receiver<Result<Vec<u8>, PluginHostError>> {
    let (sender, receiver) = semio_framework_async::oneshot::channel();
    GuestRelayPoolFuture::spawn_recoverable(
        pool,
        Lane::UserVisible,
        async move {
            let _ = sender.send(handle.infer(request).await);
        },
        |_| {},
    );
    receiver
}

async fn pool_timer_barrier(pool: &WorkerPool) {
    let (sender, receiver) = semio_framework_async::oneshot::channel();
    pool.submit_at(
        pool.now_ms().saturating_add(1),
        Lane::Maintenance,
        Box::new(move || {
            let _ = sender.send(());
        }),
    );
    receiver.await.expect("maintenance timer barrier");
}

#[semio_framework_async_macros::async_test]
async fn dropping_a_pending_mounted_future_reaps_without_a_second_foreground_poll() {
    let fixture = relay_lifecycle_fixture();
    let expected_cancels = fixture["traces"].as_array().expect("fixture traces").iter().find(|trace| trace["🪪️id"] == "relay-abandoned-blocked-wake").and_then(|trace| trace["expected"]["cancelAdmissions"].as_u64()).expect("cancel trace");
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(8_016);
    let cancel = semio_framework_async::CancelToken::root_now();
    let (session, gate, instance) = relay_session(Arc::clone(&mock), actor, pool.clone(), cancel.clone(), JobStep::Done { output: b"abandoned".to_vec() }).await;
    let cancel_gate = mock.script_pending_cancel_job();
    let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool.clone()));
    let (index, generation) = registry.reserve().expect("mounted relay slot");
    registry.mount(index, generation, GuestRelayMountedOwner::Session(session));
    let mut future = Box::pin(GuestRelayMountedFuture { registry: Arc::clone(&registry), index, generation, complete: false });
    let mut admitted = false;
    for _ in 0..64 {
        let poll = std::future::poll_fn(|context| std::task::Poll::Ready(std::future::Future::poll(future.as_mut(), context))).await;
        assert!(poll.is_pending(), "gated mounted relay remains pending");
        if mock.step_admissions() == 1 && gate.is_waiting() {
            admitted = true;
            break;
        }
        pool_timer_barrier(&pool).await;
    }
    assert!(admitted, "the pending guest step must be owned before detachment");
    drop(future);
    assert!(matches!(&*registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Mounted(session) if session.lifecycle == GuestRelayMountedLifecycle::DetachedForReap));
    for _ in 0..64 {
        if registry.reaper_wake_waits.load(std::sync::atomic::Ordering::SeqCst) == 1 {
            break;
        }
        pool_timer_barrier(&pool).await;
    }
    assert_eq!(registry.reaper_wake_waits.load(std::sync::atomic::Ordering::SeqCst), 1, "the detached session parks once on its relay-completion waker");
    assert_eq!(registry.reaper_timers.load(std::sync::atomic::Ordering::SeqCst), 0, "a wake-capable detached session needs no timer fallback");
    let blocked_polls = registry.reaper_polls.load(std::sync::atomic::Ordering::SeqCst);
    for _ in 0..8 {
        pool_timer_barrier(&pool).await;
    }
    assert_eq!(registry.reaper_polls.load(std::sync::atomic::Ordering::SeqCst), blocked_polls, "an unwoken detached session performs zero close rechecks");
    assert_eq!(registry.reaper_timers.load(std::sync::atomic::Ordering::SeqCst), 0, "an unwoken detached session creates zero timers");
    cancel_gate.release();
    let mut reaped = false;
    for _ in 0..256 {
        if matches!(&*registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Empty) {
            reaped = true;
            break;
        }
        pool_timer_barrier(&pool).await;
    }
    assert!(reaped, "the registry-owned reaper must release the detached slot without foreground pump");
    let post_wake_polls = registry.reaper_polls.load(std::sync::atomic::Ordering::SeqCst) - blocked_polls;
    assert!((1..=64).contains(&post_wake_polls), "one actual wake permits only a finite close sequence, got {post_wake_polls}");
    assert_eq!(registry.reaper_timers.load(std::sync::atomic::Ordering::SeqCst), 0, "wake-driven close remains timer-free through terminal release");
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_available_instance(&pool, &instance).await;
    assert!(cancel.is_cancelled_now());
    assert_eq!(mock.cancel_admissions(), expected_cancels as usize);
    assert_eq!(mock.step_admissions(), 1);
    pool.shutdown().expect("worker shutdown");
}

#[semio_framework_async_macros::async_test]
async fn wake_incapable_close_uses_one_coalesced_bounded_fallback() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool.clone()));
    registry.park_blocked_reaper(std::task::Waker::noop(), true);
    assert_eq!(registry.reaper_timers.load(std::sync::atomic::Ordering::SeqCst), 0, "wake-capable owners never enter fallback timing");
    registry.park_blocked_reaper(std::task::Waker::noop(), false);
    registry.park_blocked_reaper(std::task::Waker::noop(), false);
    assert_eq!(registry.reaper_timers.load(std::sync::atomic::Ordering::SeqCst), 1, "a wake-incapable owner receives one coalesced fallback");
    assert!(registry.reaper_timer_pending.load(std::sync::atomic::Ordering::Acquire));
    for _ in 0..64 {
        if !registry.reaper_timer_pending.load(std::sync::atomic::Ordering::Acquire) {
            break;
        }
        pool_timer_barrier(&pool).await;
    }
    assert!(!registry.reaper_timer_pending.load(std::sync::atomic::Ordering::Acquire), "the bounded fallback relinquishes its pending ownership");
    assert_eq!(registry.reaper_timers.load(std::sync::atomic::Ordering::SeqCst), 1, "the fallback never duplicates itself while coalesced");
    pool.shutdown().expect("worker shutdown");
}

#[semio_framework_async_macros::async_test]
async fn pending_guest_releases_the_only_worker_and_admits_no_duplicate_step() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let mock = Arc::new(MockGuestRuntime::new().await);
    let cancel = semio_framework_async::CancelToken::root_now();
    let (session, gate, _) = relay_session(Arc::clone(&mock), RuntimeActorId(8_001), pool.clone(), cancel, JobStep::Done { output: b"done".to_vec() }).await;
    drive_until_step_admitted(&session, &pool, &mock).await;

    for _ in 0..8 {
        assert_eq!(test_relay_step(&session, &pool).await, TestRelayOutcome::Yield);
    }
    assert_eq!(mock.start_admissions(), 1);
    assert_eq!(mock.step_admissions(), 1, "pending caller turns must only try-receive the admitted guest future");

    let (sender, receiver) = semio_framework_async::oneshot::channel();
    pool.submit(
        Lane::UserVisible,
        Box::new(move || {
            let _ = sender.send(b"competitor-progress".to_vec());
        }),
    );
    assert_eq!(receiver.await.expect("competing user-visible job"), b"competitor-progress", "a genuinely pending guest must not park the single worker");

    gate.release();
    let terminal = loop {
        let outcome = test_relay_step(&session, &pool).await;
        if !matches!(outcome, TestRelayOutcome::Yield) {
            break outcome;
        }
    };
    assert_eq!(terminal, TestRelayOutcome::Complete(b"done".to_vec()));
    assert_eq!(test_relay_step(&session, &pool).await, TestRelayOutcome::Yield);
    assert_eq!(mock.step_admissions(), 1, "neither pending polls nor terminal replay may duplicate guest admission");
    pool.shutdown().expect("worker shutdown");
}

#[semio_framework_async_macros::async_test]
async fn cancellation_race_admits_one_guest_cancel_and_one_terminal_outcome() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let mock = Arc::new(MockGuestRuntime::new().await);
    let cancel = semio_framework_async::CancelToken::root_now();
    let (session, _gate, _) = relay_session(Arc::clone(&mock), RuntimeActorId(8_002), pool.clone(), cancel.clone(), JobStep::Done { output: b"raced".to_vec() }).await;
    drive_until_step_admitted(&session, &pool, &mock).await;

    cancel.cancel_now();
    let terminal = loop {
        let outcome = test_relay_step(&session, &pool).await;
        if !matches!(outcome, TestRelayOutcome::Yield) {
            break outcome;
        }
    };
    assert_eq!(terminal, TestRelayOutcome::Cancelled);
    assert_eq!(test_relay_step(&session, &pool).await, TestRelayOutcome::Yield, "the terminal cancellation must not be delivered twice");
    drop(session);
    wait_for_cancel_admission(&pool, &mock).await;
    assert_eq!(mock.step_admissions(), 1);
    assert_eq!(mock.cancel_admissions(), 1, "the completion/cancellation race must admit guest cancellation exactly once");
    pool.shutdown().expect("worker shutdown");
}

#[semio_framework_async_macros::async_test]
async fn dropping_a_live_nonterminal_relay_cancels_the_guest_exactly_once() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let mock = Arc::new(MockGuestRuntime::new().await);
    let cancel = semio_framework_async::CancelToken::root_now();
    let (session, _gate, _) = relay_session(Arc::clone(&mock), RuntimeActorId(8_003), pool.clone(), cancel.clone(), JobStep::Done { output: b"unreachable".to_vec() }).await;
    drive_until_step_admitted(&session, &pool, &mock).await;

    drop(session);
    wait_for_cancel_admission(&pool, &mock).await;
    assert!(cancel.is_cancelled_now(), "dropping an admitted nonterminal relay must cancel its owned scope before scheduling guest cleanup");
    assert_eq!(mock.step_admissions(), 1);
    assert_eq!(mock.cancel_admissions(), 1, "the pending-step and Drop cleanup paths share one cancellation admission bit");
    pool.shutdown().expect("worker shutdown");
}

#[semio_framework_async_macros::async_test]
async fn mounted_start_panic_restores_the_instance_and_the_next_route_progresses() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(8_004);
    let handle = mounted_handle(Arc::clone(&mock), actor).await;
    mock.script_job_step(actor, JobStep::Done { output: b"mounted-after-start-panic".to_vec() }).await;
    mock.panic_next_start();

    let error = handle.infer(b"first").await.expect_err("start panic must surface as a typed host fault");
    assert!(error.to_string().contains("plugin guest relay panicked"));
    let pool = plugin_host_worker_pool();
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_available_instance(&pool, &handle.instance).await;
    assert!(handle.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_available(), "the unwind lease must restore the taken instance");

    let (sender, receiver) = semio_framework_async::oneshot::channel();
    pool.submit(
        Lane::UserVisible,
        Box::new(move || {
            let _ = sender.send(());
        }),
    );
    receiver.await.expect("the process worker pool must survive the retained-future panic");
    assert_eq!(handle.infer(b"second").await.expect("the next mounted route must acquire the restored instance"), b"mounted-after-start-panic");
    assert_eq!(mock.cancel_admissions(), 1);
}

#[semio_framework_async_macros::async_test]
async fn mounted_step_panic_restores_the_instance_and_terminalizes_once() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(8_005);
    let handle = mounted_handle(Arc::clone(&mock), actor).await;
    mock.script_job_step(actor, JobStep::Done { output: b"mounted-after-step-panic".to_vec() }).await;
    mock.panic_next_step();

    let error = handle.infer(b"first").await.expect_err("step panic must surface as a typed host fault");
    assert!(error.to_string().contains("plugin guest relay panicked"));
    let pool = plugin_host_worker_pool();
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_available_instance(&pool, &handle.instance).await;
    assert!(handle.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_available());
    assert_eq!(handle.infer(b"second").await.expect("a mounted route after step panic must not deadlock"), b"mounted-after-step-panic");
    assert_eq!(mock.cancel_admissions(), 1);
}

#[semio_framework_async_macros::async_test]
async fn cancel_panic_quarantines_instance_releases_permit_and_faults_once_on_one_worker() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let mock = Arc::new(MockGuestRuntime::new().await);
    let handle = mounted_handle(Arc::clone(&mock), RuntimeActorId(8_006)).await;
    let relay_cancel = semio_framework_async::CancelToken::root_now();
    let session_cancel = semio_framework_async::CancelToken::root_now();
    let (session, _gate) = relay_session_for_handle(&handle, &mock, pool.clone(), relay_cancel.clone(), session_cancel, JobStep::Done { output: b"unreachable".to_vec() }).await;
    drive_until_step_admitted(&session, &pool, &mock).await;
    mock.panic_next_cancel();
    relay_cancel.cancel_now();

    let terminal = loop {
        let outcome = test_relay_step(&session, &pool).await;
        if !matches!(outcome, TestRelayOutcome::Yield) {
            break outcome;
        }
    };
    assert_eq!(terminal, TestRelayOutcome::Fault(b"plugin instance quarantined after cancel-job panic".to_vec()));
    assert_eq!(test_relay_step(&session, &pool).await, TestRelayOutcome::Yield);
    wait_for_cancel_admission(&pool, &mock).await;
    assert_eq!(mock.cancel_admissions(), 1, "a panicking cancel-job admission must never be retried");
    assert!(handle.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_quarantined(), "cancel unwind must retain but quarantine the uncertain instance before fault delivery");

    let (sender, receiver) = semio_framework_async::oneshot::channel();
    pool.submit(
        Lane::UserVisible,
        Box::new(move || {
            let _ = sender.send(());
        }),
    );
    receiver.await.expect("the sole worker and semaphore permit must survive cancel panic");
    pool.shutdown().expect("worker shutdown");
    let next = handle.infer(b"next-route").await.expect_err("a mounted route must reject rather than reuse a quarantined guest");
    assert!(next.to_string().contains("quarantined after cancel-job panic"));
    assert_eq!(mock.cancel_admissions(), 1);
}

#[semio_framework_async_macros::async_test]
async fn background_cleanup_cancel_panic_quarantines_before_the_next_mounted_route() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let handle = mounted_handle(Arc::clone(&mock), RuntimeActorId(8_008)).await;
    mock.panic_next_start();
    mock.panic_next_cancel();

    let first = handle.infer(b"first").await.expect_err("start panic must fault");
    assert!(first.to_string().contains("plugin guest relay panicked"));
    let pool = plugin_host_worker_pool();
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_quarantine(&pool, &handle.instance).await;
    let next = handle.infer(b"next-route").await.expect_err("the mounted route must reject background-cancel quarantine without deadlock");
    assert!(next.to_string().contains("quarantined after cleanup cancel-job panic"));
    assert_eq!(mock.cancel_admissions(), 1);
}

#[semio_framework_async_macros::async_test]
async fn context_cancellation_failure_faults_once_quarantines_and_releases_one_worker() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let mock = Arc::new(MockGuestRuntime::new().await);
    let handle = mounted_handle(Arc::clone(&mock), RuntimeActorId(8_009)).await;
    let relay_cancel = semio_framework_async::CancelToken::root_now();
    let session_cancel = semio_framework_async::CancelToken::root_now();
    let (session, _gate) = relay_session_for_handle(&handle, &mock, pool.clone(), relay_cancel.clone(), session_cancel, JobStep::Done { output: b"unreachable".to_vec() }).await;
    drive_until_step_admitted(&session, &pool, &mock).await;
    mock.fail_next_cancel();
    relay_cancel.cancel_now();

    let terminal = loop {
        let outcome = test_relay_step(&session, &pool).await;
        if !matches!(outcome, TestRelayOutcome::Yield) {
            break outcome;
        }
    };
    assert!(matches!(
        terminal,
        TestRelayOutcome::Fault(fault)
            if fault == b"plugin instance quarantined after cancel-job failure: guest trapped: scripted cancel-job failure"
    ));
    assert_eq!(test_relay_step(&session, &pool).await, TestRelayOutcome::Yield);
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_quarantine(&pool, &handle.instance).await;
    assert_eq!(mock.cancel_admissions(), 1, "ordinary cancel failure must consume the sole admission without retry");

    let (sender, receiver) = semio_framework_async::oneshot::channel();
    pool.submit(
        Lane::UserVisible,
        Box::new(move || {
            let _ = sender.send(b"worker-survived".to_vec());
        }),
    );
    assert_eq!(receiver.await.expect("one-worker competitor after cancel failure"), b"worker-survived");
    pool.shutdown().expect("worker shutdown");

    let start_admissions = mock.start_admissions();
    let next = handle.infer(b"next-route").await.expect_err("quarantined mounted route must reject without entering the guest");
    assert!(next.to_string().contains("quarantined after cancel-job failure: guest trapped: scripted cancel-job failure"));
    assert_eq!(mock.start_admissions(), start_admissions, "quarantine rejection must not enter start-job");
    assert_eq!(mock.cancel_admissions(), 1);
}

#[semio_framework_async_macros::async_test]
async fn drop_cleanup_cancel_failure_quarantines_before_the_next_mounted_route() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let mock = Arc::new(MockGuestRuntime::new().await);
    let handle = mounted_handle(Arc::clone(&mock), RuntimeActorId(8_010)).await;
    let cancel = semio_framework_async::CancelToken::root_now();
    let (session, _gate) = relay_session_for_handle(&handle, &mock, pool.clone(), cancel.clone(), cancel, JobStep::Done { output: b"unreachable".to_vec() }).await;
    drive_until_step_admitted(&session, &pool, &mock).await;
    mock.fail_next_cancel();

    drop(session);
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_quarantine(&pool, &handle.instance).await;
    assert_eq!(mock.cancel_admissions(), 1, "Drop cleanup must not retry an ordinary cancel failure");
    let (sender, receiver) = semio_framework_async::oneshot::channel();
    pool.submit(
        Lane::UserVisible,
        Box::new(move || {
            let _ = sender.send(());
        }),
    );
    receiver.await.expect("one-worker permit and worker survive Drop cleanup failure");
    pool.shutdown().expect("worker shutdown");

    let start_admissions = mock.start_admissions();
    let next = handle.infer(b"next-route").await.expect_err("stored Drop cleanup quarantine must reject promptly");
    assert!(next.to_string().contains("quarantined after cancel-job failure: guest trapped: scripted cancel-job failure"));
    assert_eq!(mock.start_admissions(), start_admissions);
    assert_eq!(mock.cancel_admissions(), 1);
}

#[semio_framework_async_macros::async_test]
async fn start_failure_cleanup_cancel_failure_is_stored_for_the_next_route() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let handle = mounted_handle(Arc::clone(&mock), RuntimeActorId(8_011)).await;
    mock.fail_next_start();
    mock.fail_next_cancel();

    let first = handle.infer(b"first").await.expect_err("ordinary start failure must fault");
    assert!(first.to_string().contains("guest trapped: scripted start-job failure"));
    let pool = plugin_host_worker_pool();
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_quarantine(&pool, &handle.instance).await;
    let start_admissions = mock.start_admissions();
    let next = handle.infer(b"next-route").await.expect_err("start-failure cleanup quarantine must reject promptly");
    assert!(next.to_string().contains("quarantined after cancel-job failure: guest trapped: scripted cancel-job failure"));
    assert_eq!(mock.start_admissions(), start_admissions);
    assert_eq!(mock.cancel_admissions(), 1);
}

#[semio_framework_async_macros::async_test]
async fn step_failure_cleanup_cancel_failure_is_stored_for_the_next_route() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(8_012);
    let handle = mounted_handle(Arc::clone(&mock), actor).await;
    mock.script_fault(actor, "scripted step-job failure").await;
    mock.fail_next_cancel();

    let first = handle.infer(b"first").await.expect_err("ordinary step failure must fault");
    assert!(first.to_string().contains("guest trapped: scripted step-job failure"));
    let pool = plugin_host_worker_pool();
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_quarantine(&pool, &handle.instance).await;
    let start_admissions = mock.start_admissions();
    let next = handle.infer(b"next-route").await.expect_err("step-failure cleanup quarantine must reject promptly");
    assert!(next.to_string().contains("quarantined after cancel-job failure: guest trapped: scripted cancel-job failure"));
    assert_eq!(mock.start_admissions(), start_admissions);
    assert_eq!(mock.step_admissions(), 1);
    assert_eq!(mock.cancel_admissions(), 1);
}

#[semio_framework_async_macros::async_test]
async fn concurrent_route_rejects_start_failure_cleanup_pending_then_reuses_only_after_cleanup_success() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(8_013);
    let handle = Arc::new(mounted_handle(Arc::clone(&mock), actor).await);
    let release = mock.pause_after_next_start_failure_release();
    mock.fail_next_start();
    let pool = plugin_host_worker_pool();
    let first = spawn_mounted_infer(pool.clone(), Arc::clone(&handle), b"first");

    wait_for_release_barrier(&pool, &release).await;
    assert!(handle.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_cleanup_pending());
    assert_eq!(mock.cancel_admissions(), 0, "the barrier must precede cleanup scheduling");
    let second = spawn_mounted_infer(pool.clone(), Arc::clone(&handle), b"concurrent");
    let second_error = second.await.expect("concurrent mounted result").expect_err("cleanup-pending route must reject promptly");
    assert!(second_error.to_string().contains("cleanup pending after start-job failure"));
    assert_eq!(mock.start_admissions(), 1, "cleanup-pending rejection must not re-enter start-job");

    release.release();
    let first_error = first.await.expect("failed producer result").expect_err("ordinary start fault");
    assert!(first_error.to_string().contains("scripted start-job failure"));
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_available_instance(&pool, &handle.instance).await;
    mock.script_job_step(actor, JobStep::Done { output: b"clean-reuse".to_vec() }).await;
    assert_eq!(handle.infer(b"after-cleanup").await.expect("cleanup success restores mounted availability"), b"clean-reuse");
    assert_eq!(mock.cancel_admissions(), 1);
}

#[semio_framework_async_macros::async_test]
async fn concurrent_route_rejects_step_failure_cleanup_pending_then_quarantine_is_stable() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(8_014);
    let handle = Arc::new(mounted_handle(Arc::clone(&mock), actor).await);
    let release = mock.pause_after_next_step_failure_release();
    mock.script_fault(actor, "scripted concurrent step-job failure").await;
    mock.fail_next_cancel();
    let pool = plugin_host_worker_pool();
    let first = spawn_mounted_infer(pool.clone(), Arc::clone(&handle), b"first");

    wait_for_release_barrier(&pool, &release).await;
    assert!(handle.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_cleanup_pending());
    let second = spawn_mounted_infer(pool.clone(), Arc::clone(&handle), b"concurrent");
    let second_error = second.await.expect("concurrent mounted result").expect_err("cleanup-pending route must not enter the guest");
    assert!(second_error.to_string().contains("cleanup pending after step-job failure"));
    assert_eq!(mock.start_admissions(), 1);
    assert_eq!(mock.step_admissions(), 1);

    release.release();
    let first_error = first.await.expect("failed producer result").expect_err("ordinary step fault");
    assert!(first_error.to_string().contains("scripted concurrent step-job failure"));
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_quarantine(&pool, &handle.instance).await;
    let third_error = handle.infer(b"after-failed-cleanup").await.expect_err("cancel failure must leave a stable quarantine");
    assert!(third_error.to_string().contains("quarantined after cancel-job failure"));
    assert_eq!(mock.start_admissions(), 1);
    assert_eq!(mock.step_admissions(), 1);
    assert_eq!(mock.cancel_admissions(), 1);
}

#[semio_framework_async_macros::async_test]
async fn concurrent_route_rejects_retained_start_panic_cleanup_pending_before_recovery() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(8_015);
    let handle = Arc::new(mounted_handle(Arc::clone(&mock), actor).await);
    let release = mock.pause_after_next_start_failure_release();
    mock.panic_next_start();
    let pool = plugin_host_worker_pool();
    let first = spawn_mounted_infer(pool.clone(), Arc::clone(&handle), b"first");

    wait_for_release_barrier(&pool, &release).await;
    assert!(handle.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_cleanup_pending());
    let second = spawn_mounted_infer(pool.clone(), Arc::clone(&handle), b"concurrent");
    let second_error = second.await.expect("concurrent mounted result").expect_err("panic cleanup-pending route must reject promptly");
    assert!(second_error.to_string().contains("cleanup pending after guest relay panic"));
    assert_eq!(mock.start_admissions(), 1, "panic cleanup-pending rejection must not re-enter start-job");

    release.release();
    let first_error = first.await.expect("panicking producer result").expect_err("retained start panic must fault");
    assert!(first_error.to_string().contains("plugin guest relay panicked"));
    wait_for_cancel_admission(&pool, &mock).await;
    wait_for_available_instance(&pool, &handle.instance).await;
    mock.script_job_step(actor, JobStep::Done { output: b"panic-clean-reuse".to_vec() }).await;
    assert_eq!(handle.infer(b"after-panic-cleanup").await.expect("panic cleanup success restores availability"), b"panic-clean-reuse");
    assert_eq!(mock.cancel_admissions(), 1);
}

#[semio_framework_async_macros::async_test]
async fn poisoned_instance_slot_recovers_without_losing_the_mounted_route() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(8_007);
    let handle = mounted_handle(Arc::clone(&mock), actor).await;
    mock.script_job_step(actor, JobStep::Done { output: b"poison-recovered".to_vec() }).await;
    let instance = Arc::clone(&handle.instance);
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _guard = instance.lock().expect("unpoisoned fixture slot");
        panic!("scripted instance-slot poison");
    }));
    assert!(handle.instance.is_poisoned());
    assert_eq!(handle.infer(b"request").await.expect("poison recovery must preserve the resident instance"), b"poison-recovered");
    assert!(handle.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_available());
}
