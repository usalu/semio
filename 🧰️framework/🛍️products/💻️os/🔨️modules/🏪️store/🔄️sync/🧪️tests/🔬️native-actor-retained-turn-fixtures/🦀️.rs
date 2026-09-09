pub(super) mod retained_turn_fixtures {
    use super::*;

    fn runner_with(pool: Arc<semio_framework_async::WorkerPool>, owner: Option<ActorTurnOwner>, mailbox: ArtifactMailboxClose) -> Arc<ActorRunner> {
        Arc::new(ActorRunner {
            pool,
            io_reactor: None,
            generation: 1,
            turn: std::sync::Mutex::new(owner),
            terminal_turn: std::sync::Mutex::new(None),
            mailbox,
            scheduled: std::sync::atomic::AtomicBool::new(false),
            wake_requested: std::sync::atomic::AtomicBool::new(false),
            turn_generation: std::sync::atomic::AtomicU64::new(1),
            deadline_armed: std::sync::atomic::AtomicBool::new(false),
            deadline_generation: std::sync::atomic::AtomicU64::new(1),
            deadline_at_ms: std::sync::atomic::AtomicU64::new(u64::MAX),
            retry_armed: std::sync::atomic::AtomicBool::new(false),
            retry_generation: std::sync::atomic::AtomicU64::new(1),
            retry_job: std::sync::Mutex::new(None),
            terminal_job: std::sync::Mutex::new(None),
            terminal_reason: std::sync::Mutex::new(None),
            terminal_empty_callback: std::sync::Mutex::new(None),
            self_retained: std::sync::Mutex::new(None),
            external_tickets: std::sync::atomic::AtomicUsize::new(0),
            close_requested: std::sync::atomic::AtomicBool::new(false),
            terminal: std::sync::atomic::AtomicBool::new(false),
            complete: std::sync::atomic::AtomicBool::new(false),
        })
    }

    pub(in super::super) fn fixture_runner_handle(pool: Arc<semio_framework_async::WorkerPool>, generation: u64, mailbox: ArtifactMailboxClose) -> ArtifactActorRunnerHandle {
        let runner = runner_with(pool, None, mailbox);
        let runner = Arc::new(ActorRunner { generation, ..Arc::try_unwrap(runner).ok().expect("fixture runner has one owner") });
        *runner.self_retained.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(runner.clone());
        ArtifactActorRunnerHandle { generation, runner }
    }

    #[test]
    fn stale_generation_wake_cannot_schedule_or_mutate_current_turn() {
        let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        pool.shutdown().expect("fixture pool shuts down without retained uses");
        let (_, receiver) = artifact_mailbox_pair();
        let runner = runner_with(pool, None, receiver.close_handle());
        runner.request_wake(0);
        assert!(!runner.scheduled.load(std::sync::atomic::Ordering::Acquire));
        assert!(!runner.terminal.load(std::sync::atomic::Ordering::Acquire));
        runner.request_wake(1);
        assert_eq!(*runner.terminal_reason.lock().unwrap_or_else(std::sync::PoisonError::into_inner), Some(ArtifactActorTerminalReason::Pool(semio_framework_async::WorkerSubmitErrorKind::Shutdown)));
        assert!(runner.take_terminal_job().is_some(), "shutdown returns the exact rejected closure to observable terminal ownership");
    }

    #[test]
    fn retained_readiness_wake_after_turn_release_is_observed_once() {
        let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        let worker_entered = Arc::new(std::sync::Barrier::new(2));
        let worker_release = Arc::new(std::sync::Barrier::new(2));
        let worker_entered_job = worker_entered.clone();
        let worker_release_job = worker_release.clone();
        pool.submit(
            semio_framework_async::Lane::UserVisible,
            Box::new(move || {
                worker_entered_job.wait();
                worker_release_job.wait();
            }),
        );
        worker_entered.wait();
        let polls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let future_polls = polls.clone();
        let future: ActorTurnFuture = Box::pin(std::future::poll_fn(move |_| {
            future_polls.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            std::task::Poll::Pending
        }));
        let (_, receiver) = artifact_mailbox_pair();
        let runner = runner_with(pool.clone(), Some(ActorTurnOwner::Future(future)), receiver.close_handle());
        runner.scheduled.store(true, std::sync::atomic::Ordering::Release);
        let observed = runner.release_scheduled_after_turn_with(|| {
            assert!(!runner.scheduled.load(std::sync::atomic::Ordering::Acquire));
            runner.request_wake(runner.turn_generation.load(std::sync::atomic::Ordering::Acquire));
        });
        assert!(observed);
        assert!(!runner.wake_requested.load(std::sync::atomic::Ordering::Acquire));
        assert!(runner.scheduled.load(std::sync::atomic::Ordering::Acquire));
        worker_release.wait();
        let deadline = std::time::Instant::now() + Duration::from_secs(1);
        while runner.scheduled.load(std::sync::atomic::Ordering::Acquire) && std::time::Instant::now() < deadline {
            std::thread::yield_now();
        }
        assert_eq!(polls.load(std::sync::atomic::Ordering::Acquire), 1);
        assert!(!runner.scheduled.load(std::sync::atomic::Ordering::Acquire));
        runner.cancel();
        let cancel_deadline = std::time::Instant::now() + Duration::from_secs(1);
        while !runner.complete.load(std::sync::atomic::Ordering::Acquire) && std::time::Instant::now() < cancel_deadline {
            std::thread::yield_now();
        }
        assert!(runner.complete.load(std::sync::atomic::Ordering::Acquire));
        pool.shutdown().expect("fixture pool shuts down after exact readiness successor");
    }

    #[test]
    fn turn_fault_and_cancel_retain_then_close_one_owner_per_grant() {
        let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        pool.shutdown().expect("fixture pool shuts down without retained uses");
        let (sender, receiver) = artifact_mailbox_pair();
        sender.send(ArtifactActorMsg::ExternalChanged).expect("first terminal mailbox owner");
        sender.send(ArtifactActorMsg::Detach).expect("second terminal mailbox owner");
        let fault: ActorTurnFuture = Box::pin(async { panic!("fixture turn fault") });
        let runner = runner_with(pool, Some(ActorTurnOwner::Future(fault)), receiver.close_handle());
        runner.scheduled.store(true, std::sync::atomic::Ordering::Release);
        runner.clone().run_job();
        assert_eq!(*runner.terminal_reason.lock().unwrap_or_else(std::sync::PoisonError::into_inner), Some(ArtifactActorTerminalReason::TurnFault));
        assert!(runner.close_one_terminal_owner());
        assert_eq!(runner.mailbox.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 1);
        assert!(runner.close_one_terminal_owner());
        assert_eq!(runner.mailbox.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 0);
        assert!(runner.close_one_terminal_owner(), "retained fault future is a distinct terminal owner");

        let (_, receiver) = artifact_mailbox_pair();
        let cancelled: ActorTurnFuture = Box::pin(async { std::future::pending::<(Box<ArtifactActor>, ArtifactDrive)>().await });
        let cancelled_runner = runner_with(runner.pool.clone(), Some(ActorTurnOwner::Future(cancelled)), receiver.close_handle());
        cancelled_runner.cancel();
        assert_eq!(*cancelled_runner.terminal_reason.lock().unwrap_or_else(std::sync::PoisonError::into_inner), Some(ArtifactActorTerminalReason::Cancelled));
    }

    #[test]
    fn cancellation_cannot_complete_before_an_inflight_turn_returns_its_exact_owner() {
        let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        let worker_entered = Arc::new(std::sync::Barrier::new(2));
        let worker_release = Arc::new(std::sync::Barrier::new(2));
        let worker_entered_job = worker_entered.clone();
        let worker_release_job = worker_release.clone();
        pool.submit(
            semio_framework_async::Lane::UserVisible,
            Box::new(move || {
                worker_entered_job.wait();
                worker_release_job.wait();
            }),
        );
        worker_entered.wait();

        let turn_entered = Arc::new(std::sync::Barrier::new(2));
        let turn_release = Arc::new(std::sync::Barrier::new(2));
        let turn_entered_future = turn_entered.clone();
        let turn_release_future = turn_release.clone();
        let future: ActorTurnFuture = Box::pin(std::future::poll_fn(move |_| {
            turn_entered_future.wait();
            turn_release_future.wait();
            std::task::Poll::Pending
        }));
        let (_, receiver) = artifact_mailbox_pair();
        let runner = runner_with(pool.clone(), Some(ActorTurnOwner::Future(future)), receiver.close_handle());
        runner.scheduled.store(true, std::sync::atomic::Ordering::Release);
        let active = runner.clone();
        let active_turn = std::thread::spawn(move || active.run_job());
        turn_entered.wait();
        runner.cancel();
        assert!(runner.scheduled.load(std::sync::atomic::Ordering::Acquire));
        assert!(!runner.complete.load(std::sync::atomic::Ordering::Acquire));
        assert!(runner.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
        turn_release.wait();
        active_turn.join().expect("inflight turn returns");
        assert!(runner.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
        assert!(!runner.complete.load(std::sync::atomic::Ordering::Acquire));
        worker_release.wait();
        while !runner.complete.load(std::sync::atomic::Ordering::Acquire) {
            std::thread::yield_now();
        }
        pool.shutdown().expect("fixture pool closes after exact terminal owner");
    }

    #[test]
    fn quiet_pool_saturation_retains_exact_successor_for_timer_wheel_retry() {
        let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        let started = Arc::new(std::sync::Barrier::new(2));
        let release = Arc::new(std::sync::Barrier::new(2));
        let worker_started = started.clone();
        let worker_release = release.clone();
        pool.submit(
            semio_framework_async::Lane::UserVisible,
            Box::new(move || {
                worker_started.wait();
                worker_release.wait();
            }),
        );
        started.wait();
        let mut rejected = None;
        for _ in 0..semio_framework_async::WORKER_JOBS_PER_LANE {
            if let Err(error) = pool.try_submit(semio_framework_async::Lane::UserVisible, Box::new(|| {})) {
                rejected = Some(error);
                break;
            }
        }
        if let Some(error) = rejected {
            let kind = error.kind();
            let job = error.into_job();
            release.wait();
            pool.shutdown().expect("fixture pool shuts down without retained uses");
            job();
            panic!("fill exact quiet queue slot: {kind:?}");
        }
        let (_, receiver) = artifact_mailbox_pair();
        let runner = runner_with(pool, None, receiver.close_handle());
        runner.enqueue(false);
        assert!(runner.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
        assert!(runner.retry_armed.load(std::sync::atomic::Ordering::Acquire));
        release.wait();
    }

    #[test]
    fn idle_runner_is_strongly_retained_and_quiet_late_wake_schedules_once() {
        let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        pool.shutdown().expect("fixture pool shuts down without retained uses");
        let (_, receiver) = artifact_mailbox_pair();
        let handle = fixture_runner_handle(pool, 41, receiver.close_handle());
        let weak = Arc::downgrade(&handle.runner);
        let runner = handle.runner.clone();
        drop(handle);
        drop(runner);
        let retained = weak.upgrade().expect("idle runner must retain itself until terminal close");
        for _ in 0..8 {
            retained.request_wake(retained.turn_generation.load(std::sync::atomic::Ordering::Acquire));
        }
        assert!(matches!(*retained.terminal_reason.lock().unwrap_or_else(std::sync::PoisonError::into_inner), Some(ArtifactActorTerminalReason::Pool(semio_framework_async::WorkerSubmitErrorKind::Shutdown))));
        assert!(retained.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(), "wake storm retains one exact rejected job");
        let cleanup = ArtifactActorRunnerHandle { generation: 41, runner: retained };
        while cleanup.close_step() {}
        assert!(cleanup.terminal_is_empty());
    }

    #[test]
    fn external_ticket_held_across_close_delays_completion_until_exact_return() {
        let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        pool.shutdown().expect("fixture pool shuts down without retained uses");
        let (_, receiver) = artifact_mailbox_pair();
        let handle = fixture_runner_handle(pool, 73, receiver.close_handle());
        let host = Arc::new(std::sync::Mutex::new(ArtifactHostState::new()));
        let ticket = handle.issue_ticket(host);
        handle.cancel();
        while handle.close_step() {}
        assert!(!handle.terminal_is_empty(), "external ticket is a retained close authority");
        assert!(!handle.runner.complete.load(std::sync::atomic::Ordering::Acquire));
        ticket.return_to_host();
        assert!(handle.terminal_is_empty());
        assert!(handle.runner.complete.load(std::sync::atomic::Ordering::Acquire));
    }

    #[test]
    fn external_ticket_dropped_before_close_and_generation_aba_are_exact() {
        let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        pool.shutdown().expect("fixture pool shuts down without retained uses");
        let (_, old_receiver) = artifact_mailbox_pair();
        let old = fixture_runner_handle(pool.clone(), 91, old_receiver.close_handle());
        let host = Arc::new(std::sync::Mutex::new(ArtifactHostState::new()));
        let old_ticket = old.issue_ticket(host.clone());
        assert_eq!(old_ticket.generation(), 91);
        drop(old_ticket);
        old.cancel();
        while old.close_step() {}
        assert!(old.terminal_is_empty(), "returned-before-close ticket cannot delay retirement");

        let (_, current_receiver) = artifact_mailbox_pair();
        let current = fixture_runner_handle(pool, 92, current_receiver.close_handle());
        let current_ticket = current.issue_ticket(host);
        assert_eq!(current.runner.external_tickets.load(std::sync::atomic::Ordering::Acquire), 1);
        assert_eq!(old.runner.external_tickets.load(std::sync::atomic::Ordering::Acquire), 0, "old ticket return cannot decrement the reused generation");
        drop(current_ticket);
        current.cancel();
        while current.close_step() {}
        assert!(current.terminal_is_empty());
    }

    #[test]
    fn terminal_job_take_resume_and_close_preserve_exact_owner() {
        let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        pool.shutdown().expect("fixture pool shuts down without retained uses");
        let (_, receiver) = artifact_mailbox_pair();
        let handle = fixture_runner_handle(pool, 101, receiver.close_handle());
        handle.cancel();
        let job = handle.take_terminal_job().expect("host retrieves exact rejected terminal job");
        assert!(matches!(job.reason(), ArtifactActorTerminalReason::Pool(semio_framework_async::WorkerSubmitErrorKind::Shutdown)));
        job.resume();
        let resumed = handle.take_terminal_job().expect("failed resume hands the same job back to terminal ownership");
        resumed.close();
        while handle.close_step() {}
        assert!(handle.terminal_is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn idle_then_late_send_upgrades_the_host_retained_runner_once() {
        let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        let host = ArtifactHost::new(pool);
        let channels = host.open(ArtifactActorConfig { document_id: "quiet".into(), schema: "fixture/v1".into(), bindings: Vec::new(), watch_external: false, actor: "fixture".into() }).await;
        let runner = channels.runner.runner.upgrade().expect("host retains the quiet runner");
        for _ in 0..10_000 {
            if !runner.scheduled.load(std::sync::atomic::Ordering::Acquire) {
                break;
            }
            std::thread::yield_now();
        }
        assert!(!runner.scheduled.load(std::sync::atomic::Ordering::Acquire), "runner reaches wake-driven idle without polling");
        channels.cmd_tx.send(ArtifactActorMsg::ExternalChanged).expect("late owner admitted");
        for _ in 0..10_000 {
            if !runner.mailbox.has_pending() {
                break;
            }
            std::thread::yield_now();
        }
        assert!(!runner.mailbox.has_pending(), "late send wakes and consumes exactly its mailbox owner");
        let generation = host.close("quiet").expect("host transfers the runner into closing ownership");
        channels.runner.return_to_host();
        assert_eq!(generation, runner.generation);
    }

    fn fixture_execution_target_lease(surface_id: &str) -> crate::os_directory::DocumentExecutionTargetLeaseFieldsV1 {
        crate::os_directory::DocumentExecutionTargetLeaseFieldsV1 {
            schema: "semio.os.document-execution-target-lease/v1".into(),
            version: 1,
            scope: crate::os_directory::DocumentScope::new("space-a", "shared-document"),
            descriptor_digest_v1: "11".repeat(32),
            catalog: crate::os_directory::DocumentOpenCatalogV1 { generation_id: "22".repeat(32) },
            package: crate::os_directory::DocumentOpenPackageV1 {
                plugin_id: "fixture.plugin".into(),
                package_id: "fixture.package".into(),
                version: "1.0.0".into(),
                component_sha256: "33".repeat(32),
                component_blake3: "44".repeat(32),
                descriptor_byte_sha256: "55".repeat(32),
                execution_protocol: crate::os_directory::DocumentExecutionProtocolV1 { app_channel_version: crate::os_spr::CHANNEL_VERSION },
            },
            component: crate::os_directory::DocumentExecutionTargetComponentV1 { sha256: "33".repeat(32), blake3: "44".repeat(32), byte_length: 1024 },
            descriptor: crate::os_directory::DocumentExecutionTargetDescriptorV1 { sha256: "55".repeat(32), byte_length: 512 },
            browser_actor: crate::os_directory::schema::DocumentExecutionTargetBrowserActorV1::None,
            artifact: crate::os_directory::DocumentOpenArtifactV1 { kind: "fixture".into(), schema: "fixture/v1".into(), pack_schema_hash: "66".repeat(32) },
            parent_dialect: crate::os_directory::DocumentOpenParentDialectV1 { artifact_kind: "fixture".into(), standard: "1".into(), subset: "*".into() },
            surface: crate::os_directory::DocumentOpenSurfaceV1 {
                surface_id: surface_id.into(),
                app_id: "fixture.app".into(),
                window_kind_id: "fixture.window".into(),
                role: crate::os_directory::DocumentOpenSurfaceRoleV1::Editor,
                renderer_target: crate::os_directory::DocumentOpenRendererTargetV1::Wgpu,
            },
            grant: crate::os_directory::DocumentOpenGrantV1 { read: true, write: true, observe: true },
            checkpoint: crate::os_directory::DocumentOpenCheckpointV1 {
                checkpoint_id: "77".repeat(32),
                descriptor_digest_v1: "11".repeat(32),
                baseline_frontier: crate::os_directory::ArtifactFrontier { document_id: "shared-document".into(), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: crate::os_directory::ArtifactHash::new([0; 32]) },
                aggregate_sha256: "88".repeat(32),
            },
            revalidation: crate::os_directory::DocumentOpenRevalidationV1 { directory_revision: 7, membership_generation: 7, session_generation: Some(3), share_generation: None },
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn hub_document_actor_and_surface_authority_are_isolated_by_full_scope() {
        let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        pool.shutdown().expect("fixture pool shuts down without retained uses");
        let host = ArtifactHost::new(pool);
        let space_a = ArtifactDocumentKey::hub("space-a", "shared-document");
        let space_b = ArtifactDocumentKey::hub("space-b", "shared-document");
        let surface = fixture_execution_target_lease("fixture.editor");
        assert!(host.set_document_execution_target_lease(&space_a, surface.clone()));
        assert!(host.set_document_execution_target_lease(&space_b, surface));
        assert!(!host.set_document_execution_target_lease(&ArtifactDocumentKey::local("shared-document"), fixture_execution_target_lease("fixture.editor")));
        let channels_a = host
            .open(ArtifactActorConfig {
                document_id: "shared-document".into(),
                schema: "fixture/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: "http://127.0.0.1:1".into(), space_id: "space-a".into(), surface: Some("fixture.editor".into()) }],
                watch_external: false,
                actor: "fixture-a".into(),
            })
            .await;
        let channels_b = host
            .open(ArtifactActorConfig {
                document_id: "shared-document".into(),
                schema: "fixture/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: "http://127.0.0.1:1".into(), space_id: "space-b".into(), surface: Some("fixture.editor".into()) }],
                watch_external: false,
                actor: "fixture-b".into(),
            })
            .await;
        assert_eq!(channels_a.document_key, space_a);
        assert_eq!(channels_b.document_key, space_b);
        {
            let state = host.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            assert_eq!(state.documents.len(), 2);
            assert!(state.documents.contains_key(&space_a));
            assert!(state.documents.contains_key(&space_b));
            assert!(state.document_execution_target_leases.is_empty());
        }
        assert!(host.close_key(&space_a).is_some());
        assert!(!host.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).documents.contains_key(&space_a));
        assert!(host.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).documents.contains_key(&space_b));
        assert!(host.close_key(&space_b).is_some());
    }

    #[semio_framework_async_macros::async_test]
    async fn host_close_registry_survives_external_ticket_until_return() {
        let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        pool.shutdown().expect("fixture pool shuts down without retained uses");
        let host = ArtifactHost::new(pool);
        let channels = host.open(ArtifactActorConfig { document_id: "held".into(), schema: "fixture/v1".into(), bindings: Vec::new(), watch_external: false, actor: "fixture".into() }).await;
        let generation = channels.runner.generation();
        assert_eq!(host.close("held"), Some(generation));
        let control = host.closing_runner(generation).expect("closing registry owns the runner before cancellation progression");
        while control.close_step() {}
        assert!(!control.terminal_is_empty(), "held external ticket delays terminal-empty callback");
        assert!(host.closing_runner(generation).is_some());
        channels.runner.return_to_host();
        assert!(control.terminal_is_empty());
        assert!(host.closing_runner(generation).is_none(), "ticket return clears exactly the matching generation");
    }

    #[semio_framework_async_macros::async_test]
    async fn ticket_return_before_host_close_allows_immediate_retirement() {
        let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        pool.shutdown().expect("fixture pool shuts down without retained uses");
        let host = ArtifactHost::new(pool);
        let channels = host.open(ArtifactActorConfig { document_id: "returned".into(), schema: "fixture/v1".into(), bindings: Vec::new(), watch_external: false, actor: "fixture".into() }).await;
        let generation = channels.runner.generation();
        channels.runner.return_to_host();
        assert_eq!(host.close("returned"), Some(generation));
        let control = host.closing_runner(generation).expect("host retains terminal control");
        while control.close_step() {}
        assert!(control.terminal_is_empty());
        assert!(host.closing_runner(generation).is_none());
    }

    #[test]
    fn detach_while_pending_retains_future_then_cancel_closes_one_owner() {
        let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        pool.shutdown().expect("fixture pool shuts down without retained uses");
        let (_, receiver) = artifact_mailbox_pair();
        let pending: ActorTurnFuture = Box::pin(async { std::future::pending::<(Box<ArtifactActor>, ArtifactDrive)>().await });
        let runner = runner_with(pool, Some(ActorTurnOwner::Future(pending)), receiver.close_handle());
        *runner.self_retained.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(runner.clone());
        runner.request_close();
        assert!(runner.close_requested.load(std::sync::atomic::Ordering::Acquire));
        assert!(runner.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(), "fatal close scheduling retains the pending future");
        let handle = ArtifactActorRunnerHandle { generation: 1, runner };
        while handle.close_step() {}
        assert!(handle.terminal_is_empty());
    }
}
