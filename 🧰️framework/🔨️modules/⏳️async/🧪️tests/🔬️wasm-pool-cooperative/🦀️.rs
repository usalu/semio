mod cooperative_tests {
    use super::*;

    struct DeferredWakeCount(std::sync::atomic::AtomicUsize);

    impl std::task::Wake for DeferredWakeCount {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn worker_pool_use_cooperative_busy_keeps_executor_running_until_final_release() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let retained = pool.acquire_use().unwrap();
        assert_eq!(pool.shutdown(), Err(WorkerPoolShutdownError::Busy { retained_uses: 1 }));
        assert!(!pool.is_shutdown());
        let ran = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let ran_job = ran.clone();
        pool.submit(Lane::UserVisible, Box::new(move || ran_job.store(true, Ordering::SeqCst)));
        pool.pump(1);
        assert!(ran.load(Ordering::SeqCst));
        drop(retained);
        assert_eq!(pool.shutdown(), Ok(()));
        assert_eq!(pool.shutdown(), Ok(()));
    }

    #[test]
    fn worker_deferred_wake_cooperative_shutdown_requires_later_pump_to_drain() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let ticket = pool.install_deferred_wake_partition().expect("fixed cooperative partition");
        let count = Arc::new(DeferredWakeCount(std::sync::atomic::AtomicUsize::new(0)));
        pool.defer_wake(ticket, 0, Waker::from(count.clone())).expect("exact cooperative deferred slot");
        assert_eq!(count.0.load(Ordering::SeqCst), 0);
        pool.shutdown();
        assert!(pool.has_pending_work());
        assert!(!pool.pump(1));
        assert_eq!(count.0.load(Ordering::SeqCst), 1);
        assert!(pool.remove_deferred_wake_partition(ticket).expect("drained cooperative partition"));
    }

    #[test]
    fn worker_maintenance_cooperative_wake_obeys_pump_and_drr() {
        use super::*;
        static STEPS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        fn step(context: [u64; 2]) -> WorkerMaintenanceStep {
            assert_eq!(context, [51, 61]);
            if STEPS.fetch_add(1, Ordering::SeqCst) == 0 { WorkerMaintenanceStep::More } else { WorkerMaintenanceStep::Idle }
        }
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔔️maintenance/🧫️fixtures/🔣️.json")).unwrap();
        STEPS.store(0, Ordering::SeqCst);
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let ticket = pool.install_maintenance_hook(Lane::Io, step, [51, 61]).unwrap();
        assert_eq!(pool.request_maintenance(ticket), Ok(WorkerMaintenanceRequest::Requested));
        assert_eq!(pool.request_maintenance(ticket), Ok(WorkerMaintenanceRequest::Coalesced));
        assert_eq!(STEPS.load(Ordering::SeqCst), 0);
        for now in 0..fixture["idleWake"]["cooperativePumpsAtMost"].as_u64().unwrap() {
            let before = STEPS.load(Ordering::SeqCst);
            if !pool.pump(now) {
                break;
            }
            assert!(STEPS.load(Ordering::SeqCst) <= before + 1);
        }
        assert_eq!(STEPS.load(Ordering::SeqCst), fixture["idleWake"]["steps"].as_u64().unwrap() as usize);
        let snapshot = pool.try_cooperative_snapshot().unwrap();
        assert_eq!(snapshot.queued_by_lane.iter().sum::<usize>(), 0);
        assert_eq!(snapshot.selected_by_lane[Lane::Io.index()], 2);
        assert!(!pool.has_pending_work());
        assert!(pool.remove_maintenance_hook(ticket).unwrap());
        pool.shutdown();
        assert_eq!(pool.occupancy(), 0);
        assert_eq!(pool.request_maintenance(ticket), Err(WorkerMaintenanceError::Shutdown));
        eprintln!("[DEBUG] cooperative maintenance waited for host pumps, used Io DRR and one work permit, coalesced duplicate wakes, and consumed no queued closure");
    }

    #[test]
    fn worker_maintenance_cooperative_interleaves_io_jobs_and_rotating_hooks() {
        use super::*;
        static ORDER: Mutex<Vec<u64>> = Mutex::new(Vec::new());
        fn hook(context: [u64; 2]) -> WorkerMaintenanceStep {
            ORDER.lock().unwrap().push(context[0]);
            WorkerMaintenanceStep::Idle
        }
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔔️maintenance/🧫️fixtures/🔣️.json")).unwrap();
        ORDER.lock().unwrap().clear();
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
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
        for now in 0..fixture["idleWake"]["cooperativePumpsAtMost"].as_u64().unwrap() {
            if !pool.pump(now) {
                break;
            }
        }
        assert!(!pool.has_pending_work());
        assert_eq!(serde_json::to_value(&*ORDER.lock().unwrap()).unwrap(), fixture["competingWork"]["order"]);
        for ticket in tickets {
            assert!(pool.remove_maintenance_hook(ticket).unwrap());
        }
        pool.shutdown();
        assert_eq!(pool.occupancy(), 0);
        eprintln!("[DEBUG] cooperative Io DRR matched the same alternating job/hook order and returned every worker permit");
    }

    include!("../../🤝️cooperative/🦀️.rs");
}
