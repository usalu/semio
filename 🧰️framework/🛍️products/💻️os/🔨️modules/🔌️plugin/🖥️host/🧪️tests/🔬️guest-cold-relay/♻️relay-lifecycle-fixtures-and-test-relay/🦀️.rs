/// ♻️ Relay lifecycle fixtures and test relay: the neutral relay lifecycle fixture, the production machines it drives, the retained pool future retry, the mounted stack authority and the test relay admission and step helpers.

fn relay_lifecycle_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/♻️relay-lifecycle.json")).expect("relay lifecycle fixture")
}

#[test]
fn neutral_relay_lifecycle_traces_drive_production_machines() {
    let _replay_authority = semio_framework_plugin_host_shard::replay_test_authority();
    for trace in relay_lifecycle_fixture()["traces"].as_array().expect("relay lifecycle traces") {
        let actual: serde_json::Value = serde_json::from_str(&exercise_relay_lifecycle_trace(&trace.to_string(),test_relay_wake_authority()).expect("production lifecycle trace")).expect("production lifecycle projection");
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
    let (completion_sender, completion_receiver) = semio_framework_async_sync::oneshot::channel();
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
    let (failure_sender, failure_receiver) = semio_framework_async_sync::oneshot::channel();
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
    let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool,test_relay_wake_authority()));
    let (index, generation) = registry.reserve().expect("pre-failure mounted slot");
    registry.mount(index, generation, GuestRelayMountedOwner::Empty);
    registry.detach(index, generation);
    assert!(registry.reaper_failed.load(std::sync::atomic::Ordering::Acquire));
    assert!(matches!(&*registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Empty));
    assert!(registry.reserve().is_none(), "a scheduler-failed registry must reject future mounts");

    let mock = Arc::new(MockGuestRuntime::new().await);
    let handle = mounted_handle(Arc::clone(&mock), RuntimeActorId(8_017)).await;
    let oversized = vec![0; semio_framework_job_payload::JOB_PAYLOAD_OPERATION_BYTES + 1];
    let error = handle.infer(&oversized, &semio_framework_job::root_cancel_token()).await.expect_err("oversized relay input");
    assert!(error.to_string().contains("input exceeds the retained operation byte limit"));
    assert_eq!(mock.start_admissions(), 0);
    let mut maximum_rejected_job = GuestColdRelayJob::new(
        Arc::clone(&handle.runtime),
        Arc::clone(&handle.instance),
        Arc::clone(&handle.instance_gate),
        registry.pool.clone(),
        semio_framework_async::CancelToken::root_now(),
        1,
        ("semio.infer".to_string(), vec![0; semio_framework_job_payload::JOB_PAYLOAD_OPERATION_BYTES]),
    );
    maximum_rejected_job.begin_close();
    for _ in 0..semio_framework_job_payload::JOB_PAYLOAD_OPERATION_PAGES.saturating_add(3) {
        if matches!(semio_framework_job::close_with_self_funded_context(test_relay_drive_policy(), |cx| maximum_rejected_job.close_step(cx)).expect("original fixture close turn"), semio_framework_job::InteractiveJobCloseStep::Complete{..}) {
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
    assert_eq!(GuestRelayMountedRegistry::new(test_relay_wake_authority()).slots.len(), GUEST_RELAY_MOUNTED_SLOTS);
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

fn admit_test_relay(relay: GuestColdRelayJob, params: semio_framework_job_worker_authority::BatchJobParams) -> semio_framework_job_worker::WorkerJobSession<GuestColdRelayJob> {
    match admit_guest_relay_session(relay, params, test_relay_drive_policy()) {
        Ok(session) => session,
        Err(mut rejected) => {
            rejected.begin_close();
            while !rejected.terminal_is_empty() {
                let _ = semio_framework_job::close_with_self_funded_context(test_relay_drive_policy(), |cx| rejected.close_step(cx));
            }
            panic!("relay test session admission");
        }
    }
}

async fn test_relay_step(session: &mut semio_framework_job_worker::WorkerJobSession<GuestColdRelayJob>, _pool: &WorkerPool) -> TestRelayOutcome {
    let grant = test_relay_drive_policy();
    if matches!(session.poll(), semio_framework_job_worker::WorkerJobPoll::Closing | semio_framework_job_worker::WorkerJobPoll::TerminalEmpty) {
        let _ = semio_framework_job::close_with_self_funded_context(grant, |cx| session.close_step(cx)).expect("original fixture close turn");
        return TestRelayOutcome::Yield;
    }
    let GuestRelayStep::Checked(mut checked, _receipt) = step_guest_relay_owner(session, grant) else { return TestRelayOutcome::Yield };
    let mut bytes = Vec::new();
    let mut page = 0;
    let kind = loop {
        let (payload, kind) = read_guest_relay_outcome_page(&checked, page).expect("relay test outcome page");
        match payload {
            Some(payload) => {
                bytes.extend_from_slice(payload);
                page += 1;
            }
            None => break kind,
        }
    };
    let result = match kind {
        GuestRelayOutcomeKind::Continues => TestRelayOutcome::Yield,
        GuestRelayOutcomeKind::Complete => TestRelayOutcome::Complete(bytes),
        GuestRelayOutcomeKind::Cancelled => TestRelayOutcome::Cancelled,
        GuestRelayOutcomeKind::Fault => TestRelayOutcome::Fault(bytes),
    };
    for _ in 0..256 {
        if matches!(checked.acknowledge_outcome(grant), semio_framework_value::RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    if kind == GuestRelayOutcomeKind::Continues {
        if let Err(checked) = checked.resume() {
            checked.begin_close();
        }
    } else {
        checked.begin_close();
    }
    result
}
