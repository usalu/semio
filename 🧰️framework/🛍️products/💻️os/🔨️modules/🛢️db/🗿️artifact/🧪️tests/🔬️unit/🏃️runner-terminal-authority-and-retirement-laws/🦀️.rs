/// 🏃️ Runner terminal authority and retirement laws: the hash member store producer, the runner terminal authority latch, the terminal resume refusal, the authority drop transfer, the retirement panic and the retirement idle wake.

#[derive(Clone,Copy)]
pub struct HashMemberStoreProducer;

async fn rejected_engine_open_error(mut rejected: ArtifactEngineOpenRejected) -> DbError {
    loop {
        match rejected.retry_close().await {
            Ok(cause) => return cause,
            Err(retained) => rejected = retained,
        }
    }
}

#[test]
fn artifact_runner_terminal_authority_latch_preserves_external_job_and_one_resume() {
    use std::sync::atomic::Ordering;
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let pool_use = pool.acquire_use().unwrap();
    let handoff = Arc::new(ArtifactRunnerHandoff {
        pool: pool.clone(),
        pool_use: std::sync::Mutex::new(Some(pool_use.clone())),
        terminal_job: std::sync::Mutex::new(None),
        close_runner: std::sync::Mutex::new(None),
        retirement_maintenance: std::sync::Mutex::new(None),
        close_error: std::sync::Mutex::new(None),
        close_retry: std::sync::Mutex::new(ArtifactCloseRetry::clear()),
        close_faults: std::sync::atomic::AtomicUsize::new(0),
        close_polls: std::sync::atomic::AtomicUsize::new(0),
        retirement_turns: std::sync::atomic::AtomicUsize::new(0),
        active_history: std::sync::atomic::AtomicBool::new(false),
        driver: std::sync::atomic::AtomicU8::new(ArtifactRunnerDriver::Parked as u8),
        terminal: std::sync::atomic::AtomicBool::new(false),
    });
    let turns = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let job_handoff = handoff.clone();
    let job_turns = turns.clone();
    *handoff.terminal_job.lock().unwrap() = Some((
        semio_framework_async::WorkerSubmitErrorKind::Saturated,
        Box::new(move || {
            assert_eq!(job_handoff.driver.compare_exchange(ArtifactRunnerDriver::Queued as u8, ArtifactRunnerDriver::Polling as u8, Ordering::AcqRel, Ordering::Acquire,), Ok(ArtifactRunnerDriver::Queued as u8));
            job_turns.fetch_add(1, Ordering::AcqRel);
            job_handoff.driver.store(ArtifactRunnerDriver::RunnableIdle as u8, Ordering::Release);
        }),
    ));
    let external = handoff.terminal_job.lock().unwrap().take().map(|owner| ArtifactRunnerTerminalJob { handoff: handoff.clone(), owner: Some(owner) }).unwrap();
    assert_eq!(handoff.driver.load(Ordering::Acquire), ArtifactRunnerDriver::Parked as u8);
    assert!(handoff.driver.compare_exchange(ArtifactRunnerDriver::RunnableIdle as u8, ArtifactRunnerDriver::Queued as u8, Ordering::AcqRel, Ordering::Acquire).is_err());
    drop(external);
    assert!(handoff.terminal_job.lock().unwrap().is_some());
    let resumed = handoff.terminal_job.lock().unwrap().take().map(|owner| ArtifactRunnerTerminalJob { handoff: handoff.clone(), owner: Some(owner) }).unwrap();
    let mut retained = Some(resumed);
    for _ in 0..1_024 {
        match retained.take().expect("terminal resume fixture lost its exact cursor").resume() {
            Ok(()) => break,
            Err(cursor) => {
                retained = Some(cursor);
                std::thread::yield_now();
            }
        }
    }
    assert!(retained.is_none(), "terminal resume fixture did not admit its exact cursor");
    let (done_tx, done_rx) = std::sync::mpsc::sync_channel(1);
    pool.submit_at(pool.now_ms(), semio_framework_async::Lane::Maintenance, Box::new(move || done_tx.send(()).unwrap()));
    done_rx.recv().unwrap();
    assert_eq!(turns.load(Ordering::Acquire), 1);
    assert_eq!(handoff.driver.load(Ordering::Acquire), ArtifactRunnerDriver::RunnableIdle as u8);
    assert!(handoff.terminal_job.lock().unwrap().is_none());
    drop(handoff);
    drop(pool_use);
    assert_eq!(pool.shutdown(), Ok(()));
}

#[test]
fn artifact_runner_terminal_resume_refusal_returns_exact_cursor_for_close() {
    use std::sync::atomic::Ordering;
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    assert_eq!(pool.shutdown(), Ok(()));
    let handoff = Arc::new(ArtifactRunnerHandoff {
        pool,
        pool_use: std::sync::Mutex::new(None),
        terminal_job: std::sync::Mutex::new(None),
        close_runner: std::sync::Mutex::new(None),
        retirement_maintenance: std::sync::Mutex::new(None),
        close_error: std::sync::Mutex::new(None),
        close_retry: std::sync::Mutex::new(ArtifactCloseRetry::clear()),
        close_faults: std::sync::atomic::AtomicUsize::new(0),
        close_polls: std::sync::atomic::AtomicUsize::new(0),
        retirement_turns: std::sync::atomic::AtomicUsize::new(0),
        active_history: std::sync::atomic::AtomicBool::new(false),
        driver: std::sync::atomic::AtomicU8::new(ArtifactRunnerDriver::Parked as u8),
        terminal: std::sync::atomic::AtomicBool::new(false),
    });
    let turns = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let job_turns = turns.clone();
    let cursor = ArtifactRunnerTerminalJob {
        handoff: handoff.clone(),
        owner: Some((
            semio_framework_async::WorkerSubmitErrorKind::Saturated,
            Box::new(move || {
                job_turns.fetch_add(1, Ordering::AcqRel);
            }),
        )),
    };
    let close_handoff = handoff.clone();
    *handoff.close_runner.lock().unwrap() = Some(Arc::new(move || {
        assert_eq!(close_handoff.driver.load(Ordering::Acquire), ArtifactRunnerDriver::ClosingReady as u8);
        close_handoff.terminal.store(true, Ordering::Release);
        true
    }));
    let cursor = cursor.resume().unwrap_err();
    assert_eq!(turns.load(Ordering::Acquire), 0);
    assert_eq!(handoff.driver.load(Ordering::Acquire), ArtifactRunnerDriver::Parked as u8);
    assert!(handoff.terminal_job.lock().unwrap().is_none());
    assert!(cursor.close().is_ok());
    assert_eq!(handoff.driver.load(Ordering::Acquire), ArtifactRunnerDriver::Terminal as u8);
}

#[semio_framework_async_macros::async_test]
async fn artifact_authority_drop_transfers_parked_terminal_job_to_registered_close_owner() {
    let (authority, storage, pool) = journal_authority().await;
    let ready_deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while authority.handoff.driver.load(std::sync::atomic::Ordering::Acquire) != ArtifactRunnerDriver::RunnableIdle as u8 && std::time::Instant::now() < ready_deadline {
        semio_framework_async::yield_once().await;
    }
    assert_eq!(authority.handoff.driver.load(std::sync::atomic::Ordering::Acquire), ArtifactRunnerDriver::RunnableIdle as u8, "artifact authority readiness did not complete its internal driver handoff");
    authority.handoff.driver.store(ArtifactRunnerDriver::Parked as u8, std::sync::atomic::Ordering::Release);
    *authority.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) =
        Some((semio_framework_async::WorkerSubmitErrorKind::Saturated, Box::new(|| panic!("parked terminal job must be retired, not executed after authority Drop"))));
    let done = authority.take_terminal_signal().expect("artifact retirement fixture owns its exact terminal acknowledgement");
    let retirement = authority.retirement.as_ref().expect("artifact retirement fixture owns its reservation");
    let (index, generation) = (retirement.index, retirement.generation);
    let handoff = authority.handoff.clone();
    drop(authority);
    done.await.expect("registered authority retirement did not produce its terminal acknowledgement");
    await_retirement_slot_release(&handoff, index, generation, std::time::Instant::now() + std::time::Duration::from_secs(10)).await;
    assert_eq!(pool.shutdown(), Ok(()));
    drop(storage);
    assert_ne!(ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(std::sync::atomic::Ordering::Acquire), generation);
}

#[semio_framework_async_macros::async_test]
async fn artifact_runner_retirement_panic_retains_exact_cursor_until_explicit_retry() {
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let pool_use = pool.acquire_use().unwrap();
    let handoff = Arc::new(ArtifactRunnerHandoff {
        pool: pool.clone(),
        pool_use: std::sync::Mutex::new(Some(pool_use.clone())),
        terminal_job: std::sync::Mutex::new(None),
        close_runner: std::sync::Mutex::new(None),
        retirement_maintenance: std::sync::Mutex::new(None),
        close_error: std::sync::Mutex::new(None),
        close_retry: std::sync::Mutex::new(ArtifactCloseRetry::clear()),
        close_faults: std::sync::atomic::AtomicUsize::new(0),
        close_polls: std::sync::atomic::AtomicUsize::new(0),
        retirement_turns: std::sync::atomic::AtomicUsize::new(0),
        active_history: std::sync::atomic::AtomicBool::new(false),
        driver: std::sync::atomic::AtomicU8::new(ArtifactRunnerDriver::ClosingReady as u8),
        terminal: std::sync::atomic::AtomicBool::new(false),
    });
    let reservation = ArtifactRunnerRetirementReservation::try_reserve(pool.clone()).unwrap();
    let index = reservation.index;
    let generation = reservation.generation;
    let attempts = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let close_attempts = attempts.clone();
    let close_handoff = handoff.clone();
    let close: Arc<dyn Fn() -> bool + Send + Sync> = Arc::new(move || {
        if close_attempts.fetch_add(1, std::sync::atomic::Ordering::AcqRel) == 0 {
            panic!("injected artifact retirement close panic");
        }
        close_handoff.pool_use.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        close_handoff.terminal.store(true, std::sync::atomic::Ordering::Release);
        true
    });
    reservation.commit(close, handoff.clone(), pool_use.clone());
    drop(pool_use);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let restored =
            ARTIFACT_RUNNER_RETIREMENTS[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(|owner| owner.generation == generation && Arc::ptr_eq(&owner.handoff, &handoff) && Arc::ptr_eq(&owner.pool, &pool));
        if attempts.load(std::sync::atomic::Ordering::Acquire) != 0 && restored {
            break;
        }
        assert!(std::time::Instant::now() < deadline, "injected retirement panic did not restore its exact cursor");
        semio_framework_async::yield_once().await;
    }
    assert_eq!(attempts.load(std::sync::atomic::Ordering::Acquire), 1);
    assert_eq!(ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(std::sync::atomic::Ordering::Acquire), generation);
    assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    handoff.request_retirement_maintenance();
    while ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(std::sync::atomic::Ordering::Acquire) == generation {
        assert!(std::time::Instant::now() < deadline, "explicit retirement retry did not reach terminal acknowledgement");
        semio_framework_async::yield_once().await;
    }
    assert_eq!(attempts.load(std::sync::atomic::Ordering::Acquire), 2);
    assert_eq!(pool.shutdown(), Ok(()));
}

/// ⏰️ A maintenance turn that arrives before its reservation committed the retirement cursor keeps
/// the hook (`Idle`) instead of retiring it: retiring there stranded the cursor, its slot and its pool
/// use for the life of the process once the commit's own request found the hook gone. Every reservation
/// on one pool shares that pool's one hook, which retires with the pool's last slot.
#[test]
fn retirement_turn_before_its_commit_keeps_the_hook_for_the_committed_cursor() {
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let reservation = ArtifactRunnerRetirementReservation::try_reserve(pool.clone()).unwrap();
    let second = ArtifactRunnerRetirementReservation::try_reserve(pool.clone()).unwrap();
    let (index, generation) = (reservation.index, reservation.generation);
    let hook = reservation.hook.unwrap();
    let shared = second.hook.unwrap();
    assert_eq!((shared.row, shared.generation), (hook.row, hook.generation), "reservations on one pool share its one hook");
    let context = [hook.row as u64, hook.generation];
    assert_eq!(artifact_runner_retirement_step(context), semio_framework_async::WorkerMaintenanceStep::Idle);
    assert_eq!(ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(std::sync::atomic::Ordering::Acquire), generation, "an early turn keeps the reservation's slot");
    drop(reservation);
    assert_eq!(ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(std::sync::atomic::Ordering::Acquire), 0);
    assert_eq!(artifact_runner_retirement_step(context), semio_framework_async::WorkerMaintenanceStep::Idle, "the pool's other slot keeps the shared hook");
    drop(second);
    assert_eq!(artifact_runner_retirement_step(context), semio_framework_async::WorkerMaintenanceStep::Retire, "the hook of a pool without slots retires");
    assert_eq!(pool.shutdown(), Ok(()));
}

async fn await_retirement_slot_release(handoff: &ArtifactRunnerHandoff, index: usize, generation: u64, deadline: std::time::Instant) {
    while ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(std::sync::atomic::Ordering::Acquire) == generation {
        if std::time::Instant::now() < deadline {
            semio_framework_async::yield_once().await;
            continue;
        }
        let close_error = handoff.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        let close_retry = handoff.close_retry_progress();
        panic!(
            "artifact retirement did not release its slot: driver={} terminal={} close_error={:?} retry={:?} turns={} polls={} maintenance={}",
            handoff.driver.load(std::sync::atomic::Ordering::Acquire),
            handoff.terminal.load(std::sync::atomic::Ordering::Acquire),
            close_error,
            close_retry,
            handoff.retirement_turns.load(std::sync::atomic::Ordering::Acquire),
            handoff.close_polls.load(std::sync::atomic::Ordering::Acquire),
            handoff.retirement_maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(),
        );
    }
}

#[semio_framework_async_macros::async_test]
async fn a_runner_that_turns_terminal_after_its_retirement_went_idle_wakes_the_retirement() {
    use std::sync::atomic::Ordering;
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let pool_use = pool.acquire_use().unwrap();
    let handoff = Arc::new(ArtifactRunnerHandoff {
        pool: pool.clone(),
        pool_use: std::sync::Mutex::new(None),
        terminal_job: std::sync::Mutex::new(None),
        close_runner: std::sync::Mutex::new(None),
        retirement_maintenance: std::sync::Mutex::new(None),
        close_error: std::sync::Mutex::new(None),
        close_retry: std::sync::Mutex::new(ArtifactCloseRetry::clear()),
        close_faults: std::sync::atomic::AtomicUsize::new(0),
        close_polls: std::sync::atomic::AtomicUsize::new(0),
        retirement_turns: std::sync::atomic::AtomicUsize::new(0),
        active_history: std::sync::atomic::AtomicBool::new(false),
        driver: std::sync::atomic::AtomicU8::new(ArtifactRunnerDriver::Polling as u8),
        terminal: std::sync::atomic::AtomicBool::new(false),
    });
    let reservation = ArtifactRunnerRetirementReservation::try_reserve(pool.clone()).unwrap();
    let (index, generation) = (reservation.index, reservation.generation);
    let observed = Arc::downgrade(&handoff);
    reservation.commit(Arc::new(move || observed.upgrade().is_some_and(|handoff| handoff.terminal.load(Ordering::Acquire))), handoff.clone(), pool_use.clone());
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while handoff.retirement_turns.load(Ordering::Acquire) == 0 || ARTIFACT_RUNNER_RETIREMENTS[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
        assert!(std::time::Instant::now() < deadline, "the committed retirement never took its first turn");
        semio_framework_async::yield_once().await;
    }
    assert_eq!(ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(Ordering::Acquire), generation, "a runner still polling keeps its retirement cursor");
    handoff.enter_terminal();
    await_retirement_slot_release(&handoff, index, generation, deadline).await;
    assert_eq!(Arc::strong_count(&pool_use), 1, "the retired cursor released its WorkerPoolUse");
    drop(pool_use);
    assert_eq!(pool.shutdown(), Ok(()));
}
