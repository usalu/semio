/// 🧰️ Relay session helpers and waits: the relay sessions with tokens and handles, the step, cancel, quarantine, instance and release waits, the mounted infer spawn and the pool timer barrier.

async fn relay_session_with_tokens(
    mock: Arc<MockGuestRuntime>,
    actor: RuntimeActorId,
    pool: WorkerPool,
    relay_cancel: semio_framework_async::CancelToken,
    session_cancel: semio_framework_async::CancelToken,
    step: JobStep,
) -> (semio_framework_job_worker::WorkerJobSession<GuestColdRelayJob>, Arc<MockJobStepGate>, Arc<Mutex<GuestInstanceSlot>>) {
    let compiled = mock.compile(&PackageRef { package: PackageId("relay-test".to_string()), hash: PackageHash([41; 32]) }, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget::original(semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }, 1_000, semio_framework_job::default_now_us().unwrap_or(0), 4, 8, 4_096, 1)).await.expect("mock instantiate");
    let instance = Arc::new(Mutex::new(GuestInstanceSlot::Available(instance)));
    let gate = mock.script_pending_job_step(actor, step).await;
    let relay = GuestColdRelayJob::new(Arc::new(GuestRuntimes::Mock(mock)), Arc::clone(&instance), Arc::new(semio_framework_async_sync::Semaphore::new(1)), pool, relay_cancel, 1, ("semio.infer".to_string(), b"request".to_vec()));
    let params = semio_framework_job_worker_authority::BatchJobParams {
        operation: semio_framework_job::OperationId(actor.0),
        generation: semio_framework_job::Generation(1),
        cancel: session_cancel,
        config: semio_framework_job_worker_authority::BatchDriveConfig {
            retained:test_relay_drive_policy(),
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
) -> (semio_framework_job_worker::WorkerJobSession<GuestColdRelayJob>, Arc<MockJobStepGate>, Arc<Mutex<GuestInstanceSlot>>) {
    relay_session_with_tokens(mock, actor, pool, cancel.clone(), cancel, step).await
}

async fn mounted_handle(mock: Arc<MockGuestRuntime>, actor: RuntimeActorId) -> PluginInstanceHandle {
    let compiled = mock.compile(&PackageRef { package: PackageId("mounted-relay-test".to_string()), hash: PackageHash([43; 32]) }, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget::original(semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }, 1_000, semio_framework_job::default_now_us().unwrap_or(0), 4, 8, 4_096, 1)).await.expect("mock instantiate");
    PluginInstanceHandle::new(actor, Arc::new(GuestRuntimes::Mock(mock)), instance,test_relay_original_input(),test_relay_wake_authority()).await
}

async fn relay_session_for_handle(
    handle: &PluginInstanceHandle,
    mock: &MockGuestRuntime,
    pool: WorkerPool,
    relay_cancel: semio_framework_async::CancelToken,
    session_cancel: semio_framework_async::CancelToken,
    step: JobStep,
) -> (semio_framework_job_worker::WorkerJobSession<GuestColdRelayJob>, Arc<MockJobStepGate>) {
    let gate = mock.script_pending_job_step(handle.actor, step).await;
    let relay = GuestColdRelayJob::new(Arc::clone(&handle.runtime), Arc::clone(&handle.instance), Arc::clone(&handle.instance_gate), pool, relay_cancel, 77, ("semio.infer".to_string(), b"request".to_vec()));
    let params = semio_framework_job_worker_authority::BatchJobParams {
        operation: semio_framework_job::OperationId(handle.actor.0),
        generation: semio_framework_job::Generation(77),
        cancel: session_cancel,
        config: semio_framework_job_worker_authority::BatchDriveConfig {
            retained:test_relay_drive_policy(),
            site: "test.plugin-host.guest-cold-relay.mounted",
            stage: semio_framework_job::InteractiveStage::UserVisibleSimStep,
            fuel_per_step: semio_framework_job::USER_VISIBLE_LANE_FUEL,
            step_budget_us: semio_framework_job::USER_VISIBLE_LANE_WALL_US,
        },
        now_us: semio_framework_job::default_now_us,
    };
    (admit_test_relay(relay, params), gate)
}

async fn drive_until_step_admitted(session: &mut semio_framework_job_worker::WorkerJobSession<GuestColdRelayJob>, pool: &WorkerPool, mock: &MockGuestRuntime) {
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
        let _ = semio_framework_job::close_with_self_funded_context(test_relay_drive_policy(), |cx| semio_framework_job_worker::pump_worker_job_retirements(1, cx));
        if mock.cancel_admissions() == 1 {
            return;
        }
        let (sender, receiver) = semio_framework_async_sync::oneshot::channel();
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
        let (sender, receiver) = semio_framework_async_sync::oneshot::channel();
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
        let (sender, receiver) = semio_framework_async_sync::oneshot::channel();
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
        let (sender, receiver) = semio_framework_async_sync::oneshot::channel();
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

fn spawn_mounted_infer(pool: WorkerPool, handle: Arc<PluginInstanceHandle>, request: &'static [u8]) -> semio_framework_async_sync::oneshot::Receiver<Result<Vec<u8>, PluginHostError>> {
    let (sender, receiver) = semio_framework_async_sync::oneshot::channel();
    GuestRelayPoolFuture::spawn_recoverable(
        pool,
        Lane::UserVisible,
        async move {
            let _ = sender.send(handle.infer(request, &semio_framework_job::root_cancel_token()).await);
        },
        |_| {},
    );
    receiver
}

async fn pool_timer_barrier(pool: &WorkerPool) {
    let (sender, receiver) = semio_framework_async_sync::oneshot::channel();
    pool.submit_at(
        pool.now_ms().saturating_add(1),
        Lane::Maintenance,
        Box::new(move || {
            let _ = sender.send(());
        }),
    );
    receiver.await.expect("maintenance timer barrier");
}
