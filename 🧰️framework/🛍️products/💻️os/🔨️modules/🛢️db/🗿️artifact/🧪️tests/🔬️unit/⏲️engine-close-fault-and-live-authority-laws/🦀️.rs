/// ⏲️ Engine close fault and live authority laws: the bounded timer backoff, the exhausted budget, the cancelled timer, the live authorities over the pool maintenance hooks and the runner terminal close and closing poll.

#[semio_framework_async_macros::async_test]
async fn artifact_engine_close_fault_retries_on_bounded_timer_backoff_until_terminal() {
    let (authority, storage, pool) = journal_authority().await;
    let handoff = authority.handoff.clone();
    let retirement = authority.retirement.as_ref().expect("artifact engine close-fault fixture owns one retirement reservation");
    let (index, generation) = (retirement.index, retirement.generation);
    let done = authority.take_terminal_signal().expect("artifact engine close-fault fixture owns its terminal acknowledgement");
    handoff.close_faults.store(3, std::sync::atomic::Ordering::Release);
    drop(authority);
    done.await.expect("the pool timer did not drive the faulted close to its terminal acknowledgement");
    await_retirement_slot_release(&handoff, index, generation, std::time::Instant::now() + std::time::Duration::from_secs(10)).await;
    assert!(handoff.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    let retry = handoff.close_retry.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    assert_eq!((retry.state, retry.attempts), (ArtifactCloseRetryState::Clear, 0), "a healthy close step clears the retry owner");
    drop(retry);
    assert_eq!(handoff.close_faults.load(std::sync::atomic::Ordering::Acquire), 0);
    assert_eq!(pool.shutdown(), Ok(()));
    drop(storage);
}

#[semio_framework_async_macros::async_test]
async fn artifact_engine_close_fault_exhausts_its_budget_then_polls_only_on_readmission() {
    let (authority, storage, pool) = journal_authority().await;
    let handoff = authority.handoff.clone();
    let retirement = authority.retirement.as_ref().expect("artifact engine close-fault fixture owns one retirement reservation");
    let (index, generation) = (retirement.index, retirement.generation);
    let done = authority.take_terminal_signal().expect("artifact engine close-fault fixture owns its terminal acknowledgement");
    handoff.close_faults.store(usize::MAX, std::sync::atomic::Ordering::Release);
    drop(authority);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    while artifact_close_retry_progress(&pool).is_none_or(|progress| !progress.is_blocked()) {
        assert!(std::time::Instant::now() < deadline, "faulted close never exhausted its bounded retry budget: {:?}", handoff.close_retry_progress());
        semio_framework_async::yield_once().await;
    }
    let exhausted = artifact_close_retry_progress(&pool).unwrap();
    assert_eq!(exhausted, ArtifactCloseRetryProgress { state: ArtifactCloseRetryState::Exhausted, attempts: ARTIFACT_CLOSE_RETRY_LIMIT, limit: ARTIFACT_CLOSE_RETRY_LIMIT });
    let polls = handoff.close_polls.load(std::sync::atomic::Ordering::Acquire);
    assert_eq!(polls, 1 + usize::from(ARTIFACT_CLOSE_RETRY_LIMIT), "one initial poll plus exactly one poll per armed retry");
    handoff.request_retirement_maintenance();
    let (barrier_tx, barrier_rx) = std::sync::mpsc::sync_channel(1);
    pool.submit_at(pool.now_ms() + ARTIFACT_CLOSE_RETRY_CAP_MS, semio_framework_async::Lane::UserVisible, Box::new(move || barrier_tx.send(()).unwrap()));
    barrier_rx.recv().unwrap();
    assert_eq!(handoff.close_polls.load(std::sync::atomic::Ordering::Acquire), polls, "an exhausted faulted close was re-polled without re-admission");
    assert_eq!(ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(std::sync::atomic::Ordering::Acquire), generation);
    assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    handoff.close_faults.store(0, std::sync::atomic::Ordering::Release);
    assert_eq!(artifact_close_retry_readmit(&pool), 1);
    done.await.expect("re-admitted close did not reach its terminal acknowledgement");
    await_retirement_slot_release(&handoff, index, generation, deadline).await;
    assert!(artifact_close_retry_progress(&pool).is_none());
    assert_eq!(pool.shutdown(), Ok(()));
    drop(storage);
}

#[semio_framework_async_macros::async_test]
async fn artifact_engine_close_fault_cancel_stops_the_timer_until_readmission() {
    let (authority, storage, pool) = journal_authority().await;
    let handoff = authority.handoff.clone();
    let retirement = authority.retirement.as_ref().expect("artifact engine close-fault fixture owns one retirement reservation");
    let (index, generation) = (retirement.index, retirement.generation);
    let done = authority.take_terminal_signal().expect("artifact engine close-fault fixture owns its terminal acknowledgement");
    handoff.close_faults.store(usize::MAX, std::sync::atomic::Ordering::Release);
    drop(authority);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    while artifact_close_retry_cancel(&pool) == 0 {
        assert!(std::time::Instant::now() < deadline, "faulted close never scheduled its first retry");
        semio_framework_async::yield_once().await;
    }
    let cancelled = handoff.close_retry_progress().expect("cancelled close keeps its fault");
    assert_eq!(cancelled.state, ArtifactCloseRetryState::Cancelled);
    let polls = handoff.close_polls.load(std::sync::atomic::Ordering::Acquire);
    let (barrier_tx, barrier_rx) = std::sync::mpsc::sync_channel(1);
    pool.submit_at(pool.now_ms() + ARTIFACT_CLOSE_RETRY_CAP_MS, semio_framework_async::Lane::UserVisible, Box::new(move || barrier_tx.send(()).unwrap()));
    barrier_rx.recv().unwrap();
    assert_eq!(handoff.close_polls.load(std::sync::atomic::Ordering::Acquire), polls, "a cancelled retry timer re-polled the faulted close");
    assert_eq!(ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(std::sync::atomic::Ordering::Acquire), generation);
    handoff.close_faults.store(0, std::sync::atomic::Ordering::Release);
    assert_eq!(artifact_close_retry_readmit(&pool), 1);
    done.await.expect("re-admitted close did not reach its terminal acknowledgement");
    await_retirement_slot_release(&handoff, index, generation, deadline).await;
    assert_eq!(pool.shutdown(), Ok(()));
    drop(storage);
}

/// 🪝️ Live authorities are not maintenance hooks: one pool holds more open authorities at once than it has maintenance
/// hooks in all (spread over two fixed-capacity memory backends), each dropped authority still retires within its turn
/// budget and releases its WAL writer, and afterwards the pool's whole hook capacity is free again — the shared retirement
/// hook retired with the last slot.
#[semio_framework_async_macros::async_test]
async fn more_live_authorities_than_pool_maintenance_hooks_retire_through_one_shared_hook() {
    const ARTIFACT_RETIREMENT_LIVENESS_WATCHDOG: std::time::Duration = std::time::Duration::from_secs(60);
    const ARTIFACT_RETIREMENT_EVENT_TURN_BUDGET: usize = 96;
    fn idle(_: [u64; 2]) -> semio_framework_async::WorkerMaintenanceStep {
        semio_framework_async::WorkerMaintenanceStep::Idle
    }
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let storages = [storage().await, storage().await];
    let mut authorities = Vec::new();
    for ordinal in 0..=semio_framework_async::WORKER_MAINTENANCE_CAPACITY {
        let storage = storages[ordinal % storages.len()].clone();
        let engine_storage = storage.clone();
        let document = protocol::ArtifactId(format!("retirement-shared-{ordinal}"));
        let engine_document = document.clone();
        let authority = ArtifactAuthority::spawn(
            pool.clone(),
            move || async move { ArtifactEngine::create_retained(engine_document, engine_storage, ArtifactEngineConfig::default(), 0).await.map(Box::new) },
            MailboxCapacities::uniform(4),
        )
        .await
        .unwrap_or_else(|rejected| panic!("authority {ordinal} spawns beside the others: {:?}", rejected.error()));
        authorities.push((storage, to_core_document_id(&document).await, authority));
    }
    for (storage, core, authority) in authorities {
        let handoff = authority.handoff.clone();
        let retirement = authority.retirement.as_ref().expect("spawned authority owns its retirement reservation");
        let (index, generation) = (retirement.index, retirement.generation);
        drop(authority);
        let deadline = std::time::Instant::now() + ARTIFACT_RETIREMENT_LIVENESS_WATCHDOG;
        await_retirement_slot_release(&handoff, index, generation, deadline).await;
        let turns = handoff.retirement_turns.load(std::sync::atomic::Ordering::Acquire);
        assert!(turns <= ARTIFACT_RETIREMENT_EVENT_TURN_BUDGET, "dropped authority retirement took {turns} hook turns: the hook re-requested itself instead of waking on progress");
        let wal = storage.wal().await;
        let writer = loop {
            match wal.acquire_writer(&core).await {
                Ok(writer) => break writer,
                Err(DbError::Conflict(_)) => {
                    assert!(std::time::Instant::now() < deadline, "artifact authority retirement did not release its WAL writer");
                    semio_framework_async::yield_once().await;
                }
                Err(error) => panic!("artifact authority retirement writer probe failed: {error}"),
            }
        };
        writer.release().await.unwrap();
    }
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let mut probes = Vec::with_capacity(semio_framework_async::WORKER_MAINTENANCE_CAPACITY);
        let mut complete = true;
        for _ in 0..semio_framework_async::WORKER_MAINTENANCE_CAPACITY {
            match pool.install_maintenance_hook(semio_framework_async::Lane::Io, idle, [0; 2]) {
                Ok(ticket) => probes.push(ticket),
                Err(semio_framework_async::WorkerMaintenanceError::Capacity) => {
                    complete = false;
                    break;
                }
                Err(error) => panic!("artifact retirement capacity probe failed: {error:?}"),
            }
        }
        for ticket in probes {
            assert!(pool.remove_maintenance_hook(ticket).unwrap());
        }
        if complete {
            break;
        }
        assert!(std::time::Instant::now() < deadline, "artifact authority retirement leaked a maintenance callback slot");
        semio_framework_async::yield_once().await;
    }
    assert_eq!(pool.shutdown(), Ok(()));
}

#[test]
fn artifact_runner_terminal_close_returns_exact_cursor_until_retained_wake() {
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
        active_history: std::sync::atomic::AtomicBool::new(true),
        driver: std::sync::atomic::AtomicU8::new(ArtifactRunnerDriver::Parked as u8),
        terminal: std::sync::atomic::AtomicBool::new(false),
    });
    let polls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let close_handoff = handoff.clone();
    let close_polls = polls.clone();
    *handoff.close_runner.lock().unwrap() = Some(Arc::new(move || {
        assert_eq!(close_handoff.driver.compare_exchange(ArtifactRunnerDriver::ClosingReady as u8, ArtifactRunnerDriver::ClosingPolling as u8, Ordering::AcqRel, Ordering::Acquire,), Ok(ArtifactRunnerDriver::ClosingReady as u8));
        if close_polls.fetch_add(1, Ordering::AcqRel) == 0 {
            close_handoff.driver.store(ArtifactRunnerDriver::ClosingParked as u8, Ordering::Release);
            return false;
        }
        close_handoff.active_history.store(false, Ordering::Release);
        close_handoff.pool_use.lock().unwrap().take();
        close_handoff.terminal.store(true, Ordering::Release);
        true
    }));
    *handoff.terminal_job.lock().unwrap() = Some((semio_framework_async::WorkerSubmitErrorKind::Saturated, Box::new(|| {})));
    let cursor = handoff.terminal_job.lock().unwrap().take().map(|owner| ArtifactRunnerTerminalJob { handoff: handoff.clone(), owner: Some(owner) }).unwrap();
    let cursor = cursor.close().unwrap_err();
    assert_eq!(polls.load(Ordering::Acquire), 1);
    assert_eq!(handoff.driver.load(Ordering::Acquire), ArtifactRunnerDriver::ClosingParked as u8);
    assert!(handoff.terminal_job.lock().unwrap().is_none());
    let cursor = cursor.close().unwrap_err();
    assert_eq!(polls.load(Ordering::Acquire), 1, "close cursor must not repoll before a retained wake");
    assert_eq!(handoff.driver.compare_exchange(ArtifactRunnerDriver::ClosingParked as u8, ArtifactRunnerDriver::ClosingReady as u8, Ordering::AcqRel, Ordering::Acquire,), Ok(ArtifactRunnerDriver::ClosingParked as u8));
    assert!(cursor.close().is_ok());
    assert_eq!(polls.load(Ordering::Acquire), 2);
    assert_eq!(handoff.driver.load(Ordering::Acquire), ArtifactRunnerDriver::Terminal as u8);
    drop(handoff);
    drop(pool_use);
    assert_eq!(pool.shutdown(), Ok(()));
}

#[test]
fn artifact_runner_closing_poll_waits_for_retained_wake_before_next_turn() {
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
        active_history: std::sync::atomic::AtomicBool::new(true),
        driver: std::sync::atomic::AtomicU8::new(ArtifactRunnerDriver::ClosingPolling as u8),
        terminal: std::sync::atomic::AtomicBool::new(false),
    });
    drop(ArtifactRunnerClosePoll { handoff: handoff.clone() });
    assert_eq!(handoff.driver.load(Ordering::Acquire), ArtifactRunnerDriver::ClosingParked as u8);
    assert!(handoff.driver.compare_exchange(ArtifactRunnerDriver::ClosingReady as u8, ArtifactRunnerDriver::ClosingPolling as u8, Ordering::AcqRel, Ordering::Acquire).is_err());
    assert_eq!(handoff.driver.compare_exchange(ArtifactRunnerDriver::ClosingParked as u8, ArtifactRunnerDriver::ClosingReady as u8, Ordering::AcqRel, Ordering::Acquire,), Ok(ArtifactRunnerDriver::ClosingParked as u8));
    assert_eq!(handoff.driver.compare_exchange(ArtifactRunnerDriver::ClosingReady as u8, ArtifactRunnerDriver::ClosingPolling as u8, Ordering::AcqRel, Ordering::Acquire,), Ok(ArtifactRunnerDriver::ClosingReady as u8));
    handoff.driver.store(ArtifactRunnerDriver::Terminal as u8, Ordering::Release);
    handoff.pool_use.lock().unwrap().take();
    drop(handoff);
    drop(pool_use);
    assert_eq!(pool.shutdown(), Ok(()));
}
