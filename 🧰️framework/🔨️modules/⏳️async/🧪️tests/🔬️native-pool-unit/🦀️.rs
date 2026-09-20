mod tests {
    use super::*;

    struct DeferredWakeProbe {
        wakes: AtomicUsize,
        external: Arc<Mutex<()>>,
        observed_locked: std::sync::atomic::AtomicBool,
    }

    impl std::task::Wake for DeferredWakeProbe {
        fn wake(self: Arc<Self>) {
            if self.external.try_lock().is_err() {
                self.observed_locked.store(true, Ordering::SeqCst);
            }
            self.wakes.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn worker_pool_use_native_busy_keeps_executor_running_until_final_release() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let retained = pool.acquire_use().unwrap();
        let clone = retained.clone();
        assert_eq!(pool.shutdown(), Err(WorkerPoolShutdownError::Busy { retained_uses: 1 }));
        assert!(!pool.is_shutdown());
        let (ran_tx, ran_rx) = std::sync::mpsc::sync_channel(1);
        pool.submit(Lane::UserVisible, Box::new(move || ran_tx.send(()).unwrap()));
        ran_rx.recv_timeout(Duration::from_secs(5)).unwrap();
        drop(retained);
        assert_eq!(pool.shutdown(), Err(WorkerPoolShutdownError::Busy { retained_uses: 1 }));
        drop(clone);
        assert_eq!(pool.shutdown(), Ok(()));
        assert_eq!(pool.shutdown(), Ok(()));
    }

    /// ⚖️ A mandatory admission waits for its selected queue's ownership and preserves the exact job;
    /// the explicitly nonblocking `try_submit` remains the API that reports transient contention.
    #[test]
    fn mandatory_submit_linearizes_after_queue_ownership_is_released() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let queue = pool.inner.workers[0].queues[Lane::Io.index()].lock().expect("test owns the selected queue");
        let refusal = pool.try_submit(Lane::Io, Box::new(|| {})).expect_err("try_submit refuses concurrent queue ownership");
        assert_eq!(refusal.kind(), WorkerSubmitErrorKind::Contended);

        let submit_pool = pool.clone();
        let (admitted_tx, admitted_rx) = std::sync::mpsc::channel();
        let (ran_tx, ran_rx) = std::sync::mpsc::channel();
        let submitting = thread::spawn(move || {
            submit_pool.submit(Lane::Io, Box::new(move || ran_tx.send(()).expect("job receiver remains live")));
            admitted_tx.send(()).expect("admission receiver remains live");
        });
        assert_eq!(admitted_rx.try_recv(), Err(std::sync::mpsc::TryRecvError::Empty), "mandatory submit cannot report admission before it owns the queue");
        drop(queue);
        admitted_rx.recv_timeout(Duration::from_secs(5)).expect("mandatory submit acquires the released queue");
        ran_rx.recv_timeout(Duration::from_secs(5)).expect("the exact submitted job runs");
        submitting.join().expect("submitting thread remains live");
        assert_eq!(pool.shutdown(), Ok(()));
    }

    #[test]
    fn worker_pool_use_acquire_and_shutdown_linearize_exactly_once() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let barrier = Arc::new(std::sync::Barrier::new(3));
        let acquire_pool = pool.clone();
        let acquire_barrier = barrier.clone();
        let acquire = thread::spawn(move || {
            acquire_barrier.wait();
            acquire_pool.acquire_use()
        });
        let shutdown_pool = pool.clone();
        let shutdown_barrier = barrier.clone();
        let shutdown = thread::spawn(move || {
            shutdown_barrier.wait();
            shutdown_pool.shutdown()
        });
        barrier.wait();
        let acquired = acquire.join().unwrap();
        let closed = shutdown.join().unwrap();
        match (acquired, closed) {
            (Ok(retained), Err(WorkerPoolShutdownError::Busy { retained_uses: 1 })) => {
                drop(retained);
                assert_eq!(pool.shutdown(), Ok(()));
            }
            (Err(WorkerPoolUseError::Closing | WorkerPoolUseError::Stopped), Ok(())) => {}
            _ => panic!("pool use and shutdown did not linearize"),
        }
        assert!(pool.is_shutdown());
        assert_eq!(pool.shutdown(), Ok(()));
    }

    #[test]
    fn worker_deferred_wake_native_never_runs_inline_and_shutdown_drains_accepted_owner() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let (started_tx, started_rx) = std::sync::mpsc::channel();
        let (release_tx, release_rx) = std::sync::mpsc::channel();
        pool.submit(
            Lane::Interactive,
            Box::new(move || {
                started_tx.send(()).expect("blocking worker start");
                release_rx.recv().expect("blocking worker release");
            }),
        );
        started_rx.recv_timeout(Duration::from_secs(5)).expect("only worker occupied");
        let ticket = pool.install_deferred_wake_partition().expect("fixed partition admission");
        let external = Arc::new(Mutex::new(()));
        let probe = Arc::new(DeferredWakeProbe { wakes: AtomicUsize::new(0), external: external.clone(), observed_locked: std::sync::atomic::AtomicBool::new(false) });
        let guard = external.lock().expect("application mutex");
        pool.defer_wake(ticket, 0, Waker::from(probe.clone())).expect("exact deferred slot");
        assert_eq!(probe.wakes.load(Ordering::SeqCst), 0);
        let closing = pool.clone();
        let shutdown = thread::spawn(move || closing.shutdown());
        drop(guard);
        release_tx.send(()).expect("release worker");
        shutdown.join().expect("shutdown drains accepted waker");
        assert_eq!(probe.wakes.load(Ordering::SeqCst), 1);
        assert!(!probe.observed_locked.load(Ordering::SeqCst));
        assert!(pool.remove_deferred_wake_partition(ticket).expect("drained partition remains generation-qualified"));
    }

    #[test]
    fn worker_maintenance_native_idle_wake_uses_no_queued_job() {
        static STEPS: AtomicUsize = AtomicUsize::new(0);
        fn step(context: [u64; 2]) -> WorkerMaintenanceStep {
            assert_eq!(context, [31, 41]);
            if STEPS.fetch_add(1, Ordering::SeqCst) == 0 { WorkerMaintenanceStep::More } else { WorkerMaintenanceStep::Idle }
        }
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔔️maintenance/🧫️fixtures/🔣️.json")).unwrap();
        STEPS.store(0, Ordering::SeqCst);
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let ticket = pool.install_maintenance_hook(Lane::Io, step, [31, 41]).unwrap();
        assert_eq!(pool.request_maintenance(ticket), Ok(WorkerMaintenanceRequest::Requested));
        let deadline = Instant::now() + Duration::from_secs(5);
        while STEPS.load(Ordering::SeqCst) != fixture["idleWake"]["steps"].as_u64().unwrap() as usize {
            assert!(Instant::now() < deadline, "idle native worker did not service maintenance without ingress");
            thread::yield_now();
        }
        while !pool.remove_maintenance_hook(ticket).unwrap() {
            assert!(Instant::now() < deadline, "native hook did not retain then retire its running callback");
            thread::yield_now();
        }
        assert_eq!(pool.request_maintenance(ticket), Err(WorkerMaintenanceError::Stale));
        let queued = pool.inner.workers.iter().flat_map(|worker| &worker.queues).map(|queue| queue.lock().unwrap().len()).sum::<usize>();
        assert_eq!(queued, fixture["idleWake"]["queuedJobs"].as_u64().unwrap() as usize);
        pool.shutdown();
        assert_eq!(pool.occupancy(), 0);
        assert_eq!(pool.install_maintenance_hook(Lane::Io, step, [31, 41]), Err(WorkerMaintenanceError::Shutdown));
        eprintln!("[DEBUG] native idle worker serviced two preallocated Io maintenance turns without any queued job or subsequent task ingress");
    }

    #[test]
    fn worker_maintenance_native_self_retire_reuses_all_fixed_slots() {
        fn retire(_: [u64; 2]) -> WorkerMaintenanceStep {
            WorkerMaintenanceStep::Retire
        }
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔔️maintenance/🧫️fixtures/🔣️.json")).unwrap();
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        for _ in 0..fixture["selfRetire"]["cycles"].as_u64().unwrap() {
            let ticket = pool.install_maintenance_hook(Lane::Io, retire, [0; 2]).unwrap();
            assert_eq!(pool.request_maintenance(ticket), Ok(WorkerMaintenanceRequest::Requested));
            let deadline = Instant::now() + Duration::from_secs(5);
            loop {
                match pool.request_maintenance(ticket) {
                    Err(WorkerMaintenanceError::Stale) => break,
                    Ok(WorkerMaintenanceRequest::Requested | WorkerMaintenanceRequest::Coalesced) => thread::yield_now(),
                    other => panic!("self-retiring maintenance hook lost exact generation: {other:?}"),
                }
                assert!(Instant::now() < deadline, "self-retiring maintenance callback did not release its exact slot");
            }
        }
        let tickets: Vec<_> = (0..WORKER_MAINTENANCE_CAPACITY).map(|_| pool.install_maintenance_hook(Lane::Io, retire, [0; 2]).unwrap()).collect();
        assert_eq!(pool.install_maintenance_hook(Lane::Io, retire, [0; 2]), Err(WorkerMaintenanceError::Capacity));
        for ticket in tickets {
            assert!(pool.remove_maintenance_hook(ticket).unwrap());
        }
        assert_eq!(pool.shutdown(), Ok(()));
        eprintln!("[DEBUG] running maintenance callbacks retired themselves after return and reused every fixed slot across 64 generations");
    }

    #[test]
    fn worker_maintenance_native_running_close_and_shutdown_keep_exact_invocation() {
        static ENTERED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        static RELEASE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        fn blocked(_: [u64; 2]) -> WorkerMaintenanceStep {
            ENTERED.store(true, Ordering::SeqCst);
            let deadline = Instant::now() + Duration::from_secs(5);
            while !RELEASE.load(Ordering::SeqCst) {
                assert!(Instant::now() < deadline);
                thread::yield_now();
            }
            WorkerMaintenanceStep::More
        }
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔔️maintenance/🧫️fixtures/🔣️.json")).unwrap();
        for shutdown in [false, true] {
            ENTERED.store(false, Ordering::SeqCst);
            RELEASE.store(false, Ordering::SeqCst);
            let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
            let ticket = pool.install_maintenance_hook(Lane::Io, blocked, [0; 2]).unwrap();
            pool.request_maintenance(ticket).unwrap();
            let deadline = Instant::now() + Duration::from_secs(5);
            while !ENTERED.load(Ordering::SeqCst) {
                assert!(Instant::now() < deadline);
                thread::yield_now();
            }
            if shutdown {
                assert_eq!(pool.request_maintenance(ticket), Ok(WorkerMaintenanceRequest::Requested));
                let closing = pool.clone();
                let thread = thread::spawn(move || closing.shutdown());
                while !pool.is_shutdown() {
                    assert!(Instant::now() < deadline);
                    thread::yield_now();
                }
                RELEASE.store(true, Ordering::SeqCst);
                thread.join().unwrap();
                assert_eq!(pool.inner.maintenance.has_pending(None), fixture["runningClose"]["shutdownRearms"].as_bool().unwrap());
                assert_eq!(pool.remove_maintenance_hook(ticket).unwrap(), fixture["runningClose"]["secondRemoveTerminal"].as_bool().unwrap());
            } else {
                assert_eq!(pool.remove_maintenance_hook(ticket).unwrap(), fixture["runningClose"]["firstRemoveTerminal"].as_bool().unwrap());
                assert_eq!(pool.request_maintenance(ticket), Err(WorkerMaintenanceError::Closed));
                RELEASE.store(true, Ordering::SeqCst);
                while !pool.remove_maintenance_hook(ticket).unwrap() {
                    assert!(Instant::now() < deadline);
                    thread::yield_now();
                }
                assert_eq!(pool.request_maintenance(ticket), Err(WorkerMaintenanceError::Stale));
                pool.shutdown();
            }
            assert_eq!(pool.occupancy(), 0);
        }
        eprintln!("[DEBUG] native running hook retained the exact second-remove witness and shutdown prevented More or concurrent wake from rearming");
    }

    #[test]
    fn worker_maintenance_native_interleaves_io_jobs_and_rotating_hooks() {
        static ORDER: Mutex<Vec<u64>> = Mutex::new(Vec::new());
        fn hook(context: [u64; 2]) -> WorkerMaintenanceStep {
            ORDER.lock().unwrap().push(context[0]);
            WorkerMaintenanceStep::Idle
        }
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔔️maintenance/🧫️fixtures/🔣️.json")).unwrap();
        ORDER.lock().unwrap().clear();
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let (started_tx, started_rx) = std::sync::mpsc::channel();
        let (release_tx, release_rx) = std::sync::mpsc::channel();
        pool.submit(
            Lane::Interactive,
            Box::new(move || {
                started_tx.send(()).unwrap();
                release_rx.recv_timeout(Duration::from_secs(5)).unwrap();
            }),
        );
        started_rx.recv_timeout(Duration::from_secs(5)).unwrap();
        let mut tickets = Vec::new();
        for value in fixture["competingWork"]["hooks"].as_array().unwrap() {
            let ticket = pool.install_maintenance_hook(Lane::Io, hook, [value.as_u64().unwrap(), 0]).unwrap();
            pool.request_maintenance(ticket).unwrap();
            tickets.push(ticket);
        }
        for value in fixture["competingWork"]["jobs"].as_array().unwrap() {
            let value = value.as_u64().unwrap();
            pool.submit(Lane::Io, Box::new(move || ORDER.lock().unwrap().push(value)));
        }
        release_tx.send(()).unwrap();
        let deadline = Instant::now() + Duration::from_secs(5);
        while ORDER.lock().unwrap().len() < 4 {
            assert!(Instant::now() < deadline);
            thread::yield_now();
        }
        for ticket in tickets {
            while !pool.remove_maintenance_hook(ticket).unwrap() {
                assert!(Instant::now() < deadline);
                thread::yield_now();
            }
        }
        pool.shutdown();
        assert_eq!(serde_json::to_value(&*ORDER.lock().unwrap()).unwrap(), fixture["competingWork"]["order"]);
        assert_eq!(pool.occupancy(), 0);
        eprintln!("[DEBUG] native Io DRR alternated actual queued jobs with two rotating fixed hooks after interactive work released the worker");
    }

    #[test]
    fn native_drr_finishes_eligible_deficit_frontier_before_idle() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🤝️cooperative/🧫️fixtures/🔣️.json")).unwrap();
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let (started_tx, started_rx) = std::sync::mpsc::channel();
        let (release_tx, release_rx) = std::sync::mpsc::channel();
        pool.submit(
            Lane::Interactive,
            Box::new(move || {
                started_tx.send(()).unwrap();
                release_rx.recv_timeout(Duration::from_secs(5)).unwrap();
            }),
        );
        started_rx.recv_timeout(Duration::from_secs(2)).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let lane: Lane = serde_json::from_value(case["lane"].clone()).unwrap();
            assert_eq!(u64::from(lane.weight()), case["weight"].as_u64().unwrap());
            pool.try_submit(lane, Box::new(|| {})).ok().expect("isolated lane admission");
            let mut cursor = 0;
            let mut deficits = [0; LANE_COUNT];
            let (selected, job, permit) = select_and_pop(&pool.inner, 0, &mut cursor, &mut deficits).expect("eligible queued work must accrue deficit before idle parking");
            assert_eq!(selected, lane);
            assert_eq!(deficits[lane.index()], case["deficits"].as_array().unwrap().last().unwrap().as_i64().unwrap());
            job.run(&pool.inner.maintenance);
            drop(permit);
            assert!(select_and_pop(&pool.inner, 0, &mut cursor, &mut deficits).is_none());
        }
        release_tx.send(()).unwrap();
        pool.shutdown();
    }

    /// 🪜️ A pool worker owns [`WORKER_STACK_BYTES`], not `std::thread`'s default.
    ///
    /// This is the one regression that no ordinary assertion can catch: every repo test runner
    /// floors `RUST_MIN_STACK` between 32 and 256 MiB (`⏳️async/📦️packages/🦀️rust/📜️script.ts`,
    /// `🔌️plugin/📦️packages/🦀️rust/📜️script.ts`), so a thread spawned with NO `stack_size` looks
    /// enormous in every gate and is 2 MiB in the shipped binary — which is exactly how the hub came
    /// to abort on its first document socket (ticket 26/09/18 slice HS1: the artifact-engine mount
    /// chain reserves 8,506,832 B). `Builder::stack_size` overrides `RUST_MIN_STACK` in both
    /// directions, so this law sees the production number whatever the runner exports.
    ///
    /// It consumes 24 MiB inside a real pool job — past both the 2 MiB `std::thread` default and the
    /// 21,520,944 B peak HS1 measured on the artifact-engine turn chain, and comfortably inside the
    /// stated budget — and the consumption is genuine: each level keeps a 64 KiB array alive across
    /// its recursive call and writes through `black_box`, so nothing is elided or merged. If the
    /// `stack_size` call is ever dropped the process ABORTS here with `has overflowed its stack`
    /// rather than reporting a soft failure; that is the intended signal, and it is the same fatal
    /// runtime error the hub printed in production.
    #[test]
    fn native_pool_workers_own_the_stated_worker_stack() {
        #[inline(never)]
        fn consume(level: u32) -> u64 {
            let mut page = [0u64; 8 * 1024];
            page[0] = u64::from(level);
            page[page.len() - 1] = u64::from(level);
            let deeper = if level == 0 { 0 } else { consume(level - 1) };
            std::hint::black_box(&page);
            page[0] ^ page[page.len() - 1] ^ deeper
        }

        const LEVEL_BYTES: usize = 8 * 1024 * core::mem::size_of::<u64>();
        const PROBED_BYTES: usize = 24 * 1024 * 1024;
        assert!(PROBED_BYTES < WORKER_STACK_BYTES, "🪜️ the probe must stay inside the worker budget it is proving");
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let (done_tx, done_rx) = std::sync::mpsc::sync_channel(1);
        pool.submit(
            Lane::UserVisible,
            Box::new(move || {
                let name = std::thread::current().name().map(str::to_string);
                let reached = consume((PROBED_BYTES / LEVEL_BYTES) as u32);
                done_tx.send((name, reached)).unwrap();
            }),
        );
        let (name, reached) = done_rx.recv_timeout(Duration::from_secs(30)).expect("🪜️ the probing job must return from a worker that owns the stated stack");
        assert_eq!(reached, 0, "🪜️ every probed level must fold to zero, proving each frame was really written");
        assert!(name.is_some_and(|name| name.starts_with("semio-pool-worker-")), "🪜️ the probe must have run on a pool worker, not inline");
        let _ = pool.shutdown();
    }
}
