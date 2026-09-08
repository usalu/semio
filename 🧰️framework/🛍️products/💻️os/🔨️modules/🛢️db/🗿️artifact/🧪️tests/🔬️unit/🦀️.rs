
use super::*;
use std::sync::Arc as StdArc;

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
        close_fault_once: std::sync::atomic::AtomicBool::new(false),
        close_polls: std::sync::atomic::AtomicUsize::new(0),
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
        close_fault_once: std::sync::atomic::AtomicBool::new(false),
        close_polls: std::sync::atomic::AtomicUsize::new(0),
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
    for _ in 0..10_000 {
        if authority.handoff.driver.load(std::sync::atomic::Ordering::Acquire) == ArtifactRunnerDriver::RunnableIdle as u8 {
            break;
        }
        semio_framework_async::yield_once().await;
    }
    assert_eq!(authority.handoff.driver.load(std::sync::atomic::Ordering::Acquire), ArtifactRunnerDriver::RunnableIdle as u8, "artifact authority readiness did not complete its internal driver handoff");
    authority.handoff.driver.store(ArtifactRunnerDriver::Parked as u8, std::sync::atomic::Ordering::Release);
    *authority.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) =
        Some((semio_framework_async::WorkerSubmitErrorKind::Saturated, Box::new(|| panic!("parked terminal job must be retired, not executed after authority Drop"))));
    let done = authority._done.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().expect("artifact retirement fixture owns its exact terminal acknowledgement");
    drop(authority);
    done.await.expect("registered authority retirement did not produce its terminal acknowledgement");
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while ARTIFACT_RUNNER_RETIREMENT_GENERATIONS.iter().any(|generation| generation.load(std::sync::atomic::Ordering::Acquire) != 0) {
        assert!(std::time::Instant::now() < deadline, "registered authority retirement did not release its exact callback slot");
        semio_framework_async::yield_once().await;
    }
    assert_eq!(pool.shutdown(), Ok(()));
    drop(storage);
    assert!(ARTIFACT_RUNNER_RETIREMENT_GENERATIONS.iter().all(|generation| generation.load(std::sync::atomic::Ordering::Acquire) == 0));
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
        close_fault_once: std::sync::atomic::AtomicBool::new(false),
        close_polls: std::sync::atomic::AtomicUsize::new(0),
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

#[semio_framework_async_macros::async_test]
async fn artifact_engine_close_fault_retains_exact_runner_until_explicit_maintenance_retry() {
    let (authority, storage, pool) = journal_authority().await;
    let handoff = authority.handoff.clone();
    let retirement = authority.retirement.as_ref().expect("artifact engine close-fault fixture owns one retirement reservation");
    let index = retirement.index;
    let generation = retirement.generation;
    let done = authority._done.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().expect("artifact engine close-fault fixture owns its terminal acknowledgement");
    handoff.close_fault_once.store(true, std::sync::atomic::Ordering::Release);
    drop(authority);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let retained = ARTIFACT_RUNNER_RETIREMENTS[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(|owner| owner.generation == generation && Arc::ptr_eq(&owner.handoff, &handoff));
        if retained && matches!(&*handoff.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner), Some(DbError::Io(detail)) if detail == "injected artifact engine close fault") {
            break;
        }
        assert!(std::time::Instant::now() < deadline, "artifact engine close fault did not retain its exact runner cursor");
        semio_framework_async::yield_once().await;
    }
    let retained_polls = handoff.close_polls.load(std::sync::atomic::Ordering::Acquire);
    let (barrier_tx, barrier_rx) = std::sync::mpsc::sync_channel(1);
    pool.submit_at(pool.now_ms(), semio_framework_async::Lane::UserVisible, Box::new(move || barrier_tx.send(()).unwrap()));
    barrier_rx.recv().unwrap();
    assert_eq!(handoff.close_polls.load(std::sync::atomic::Ordering::Acquire), retained_polls, "faulted artifact engine close was repolled without explicit maintenance admission");
    assert_eq!(ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(std::sync::atomic::Ordering::Acquire), generation);
    assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    handoff.request_retirement_maintenance();
    done.await.expect("explicit artifact engine close retry did not produce one terminal acknowledgement");
    while ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(std::sync::atomic::Ordering::Acquire) == generation {
        assert!(std::time::Instant::now() < deadline, "explicit artifact engine close retry did not release its retirement slot");
        semio_framework_async::yield_once().await;
    }
    assert!(handoff.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    assert!(handoff.close_polls.load(std::sync::atomic::Ordering::Acquire) > retained_polls);
    assert_eq!(pool.shutdown(), Ok(()));
    drop(storage);
}

#[semio_framework_async_macros::async_test]
async fn artifact_authority_drop_reuses_registered_retirement_slot_beyond_capacity() {
    fn idle(_: [u64; 2]) -> semio_framework_async::WorkerMaintenanceStep {
        semio_framework_async::WorkerMaintenanceStep::Idle
    }
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let storage = storage().await;
    let document = protocol::ArtifactId(String::from("retirement-reuse"));
    let core = to_core_document_id(&document).await;
    for ordinal in 0..=ARTIFACT_RUNNER_RETIREMENT_SLOTS {
        let engine_storage = storage.clone();
        let engine_document = document.clone();
        let authority = ArtifactAuthority::spawn(
            pool.clone(),
            move || async move {
                if ordinal == 0 {
                    ArtifactEngine::create_retained(engine_document, engine_storage, ArtifactEngineConfig::default(), 0).await
                } else {
                    ArtifactEngine::open_retained(engine_document, engine_storage, ArtifactEngineConfig::default(), 0).await.map(|(engine, _)| engine)
                }
            },
            MailboxCapacities::uniform(4),
        )
        .await
        .unwrap();
        drop(authority);
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while ARTIFACT_RUNNER_RETIREMENT_GENERATIONS.iter().any(|generation| generation.load(std::sync::atomic::Ordering::Acquire) != 0) {
            assert!(std::time::Instant::now() < deadline, "artifact authority retirement did not release its callback slot");
            semio_framework_async::yield_once().await;
        }
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
    eprintln!("[DEBUG] artifact authority Drop reused its self-retiring maintenance slot beyond the fixed 64-owner capacity");
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
        close_fault_once: std::sync::atomic::AtomicBool::new(false),
        close_polls: std::sync::atomic::AtomicUsize::new(0),
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
        close_fault_once: std::sync::atomic::AtomicBool::new(false),
        close_polls: std::sync::atomic::AtomicUsize::new(0),
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

#[semio_framework_async_macros::async_test]
async fn artifact_open_ignores_neutral_aborted_command_snapshot_and_cas() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../📝️wal/🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == "aborted-commands-snapshot-cas-have-no-effects").unwrap();
    let document = ArtifactId::from("committed-artifact");
    let memory = db_wal::tests::committed_fixture_storage(row, &document).await;
    let storage = StdArc::new(db_storage::DbBackend::Memory(memory));
    let (mut engine, report) = ArtifactEngine::open_retained(protocol::ArtifactId(document.0.clone()), storage, ArtifactEngineConfig::default(), 1).await.unwrap();
    assert_eq!(report.commands_replayed, 0);
    assert!(!report.from_snapshot);
    assert_eq!(engine.frontier.head_seq, 0);
    assert!(engine.state.values.is_empty());
    assert!(engine.applied.is_empty());
    while engine.wal.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    while engine.state.values.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_engine_create_rejection_propagates_exact_wal_release_owner() {
    let inner = StdArc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
    let fault = crate::db_testkit::FaultStorage::new(inner).await;
    fault.set_script(crate::db_testkit::FaultScript { fail_nth_write: Some(1), ..crate::db_testkit::FaultScript::default() }).await;
    let storage = StdArc::new(db_storage::DbBackend::Fault(Box::new(fault)));
    let document = document_id().await;
    let core_document = to_core_document_id(&document).await;
    let rejected = match ArtifactEngine::create_retained(document, storage.clone(), ArtifactEngineConfig::default(), 0).await {
        Err(rejected) => rejected,
        Ok(mut engine) => {
            engine.wal.close().await.unwrap();
            panic!("faulted engine create was admitted");
        }
    };
    assert!(matches!(rejected.error(), DbError::Io(_)));
    assert!(rejected.has_retained_writer());
    assert!(matches!(storage.wal().await.acquire_writer(&core_document).await, Err(DbError::Conflict(_))));
    assert!(matches!(rejected_engine_open_error(rejected).await, DbError::Io(_)));
    storage.wal().await.acquire_writer(&core_document).await.unwrap().release().await.unwrap();
    eprintln!("[DEBUG] engine construction propagated the exact rejected WAL release owner until terminal close");
}

fn history_construction_test_lock() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[semio_framework_async_macros::async_test]
async fn artifact_history_replay_uses_neutral_committed_inventory_and_retires_every_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../📝️wal/🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    for (name, compacted, hole) in [
        ("high-water-across-segments", false, false),
        ("high-water-across-segments", true, false),
        ("high-water-across-segments", false, true),
        ("aborted-commands-snapshot-cas-have-no-effects", false, false),
        ("commit-outside-transaction", false, false),
        ("active-incomplete-needs-durable-abort", false, false),
    ] {
        let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == name).unwrap();
        let document = ArtifactId::from("committed-history");
        let memory = db_wal::tests::committed_fixture_storage(row, &document).await;
        if compacted || hole {
            let writer = db_storage::WalStorage::acquire_writer(&memory, &document).await.unwrap();
            if compacted {
                db_storage::WalStorage::delete_segment(&memory, &writer, 0).await.unwrap();
            }
            if hole {
                db_storage::WalStorage::create_segment(&memory, &writer, 3).await.unwrap();
            }
            writer.release().await.unwrap();
        }
        let storage = StdArc::new(db_storage::DbBackend::Memory(memory));
        let mut replay = HistoryReplayFuture::new(storage, document, 1, StdArc::new(std::sync::atomic::AtomicBool::new(false)), HistoryReplayReservation::try_new().unwrap());
        let result = (&mut replay).await;
        assert!(replay.terminal_is_empty(), "{name}");
        if !hole && row["expected"]["accepted"] == true && row["expected"]["recoverAbort"].is_null() {
            let mut view = result.unwrap();
            assert!(view.entries.is_empty(), "{name}");
            assert!(view.operation_ids.is_empty(), "{name}");
            assert_eq!(view.result_len, 0, "{name}");
            while view.close_step() {}
        } else {
            assert!(matches!(result, Err(DbError::Corrupt(_))), "{name}");
        }
        eprintln!("[DEBUG] history committed neutral law retired source, inventory and result owners: {name}, compacted={compacted}, hole={hole}");
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_history_replay_projects_real_committed_batch_and_cancels_owned_sources() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../📝️wal/🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    let projection = &fixture["historyProjection"];
    let mut engine = ArtifactEngine::create_retained(document_id().await, storage().await, ArtifactEngineConfig::default(), 0).await.unwrap();
    let mut commands = Vec::new();
    for row in projection["commands"].as_array().unwrap() {
        commands.push(envelope(row["id"].as_str().unwrap(), &[], "history-author", &[(row["path"].as_str().unwrap(), row["value"].clone())]).await);
    }
    let receipt = engine.submit(CommandBatch::new(commands).await.unwrap(), SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, 1).await.unwrap();
    let mut replay = engine.history_replay(1, StdArc::new(std::sync::atomic::AtomicBool::new(false)), HistoryReplayReservation::try_new().unwrap());
    let mut view = (&mut replay).await.unwrap();
    assert!(replay.terminal_is_empty());
    assert_eq!(view.entries.len() as u64, projection["expected"]["entries"].as_u64().unwrap());
    let entry = view.entries[0];
    assert_eq!(entry.operation_count, 2);
    for (index, id) in projection["expected"]["operationIds"].as_array().unwrap().iter().enumerate() {
        assert!(view.operation_id_eq(0, index, id.as_str().unwrap()));
    }
    assert_eq!(entry.head_seq, projection["expected"]["headSeq"].as_u64().unwrap());
    assert_eq!(entry.commit_seq, projection["expected"]["commitSeq"].as_u64().unwrap());
    assert_eq!((entry.head_seq, entry.commit_seq, entry.chain_hash, entry.epoch), (receipt.frontier.head_seq, receipt.frontier.commit_seq, receipt.frontier.chain_hash, receipt.frontier.epoch));
    while view.close_step() {}
    assert!(view.terminal_is_empty());
    for checkpoint in ["verify", "committed", "copied", "published"] {
        let cancelled = StdArc::new(std::sync::atomic::AtomicBool::new(false));
        let mut replay = engine.history_replay(1, cancelled.clone(), HistoryReplayReservation::try_new().unwrap());
        let mut turns = 0;
        let reached = std::future::poll_fn(|context| {
            let target = match checkpoint {
                "verify" => matches!(replay.phase.as_ref(), Some(HistoryReplayPhase::Verify { .. })),
                "committed" => matches!(replay.phase.as_ref(), Some(HistoryReplayPhase::CommittedBody { .. })),
                "copied" => replay.result_len > 0 && replay.reservation.as_ref().is_some_and(|owner| owner.operation_ids.len() == 1),
                "published" => replay.reservation.as_ref().is_some_and(|owner| owner.entries.len() == 1),
                _ => unreachable!(),
            };
            if target {
                return std::task::Poll::Ready(true);
            }
            turns += 1;
            assert!(turns < 100_000, "history cancellation checkpoint stopped progressing");
            match Pin::new(&mut replay).poll(context) {
                std::task::Poll::Pending => std::task::Poll::Pending,
                std::task::Poll::Ready(result) => {
                    if let Ok(mut view) = result {
                        while view.close_step() {}
                    }
                    std::task::Poll::Ready(false)
                }
            }
        })
        .await;
        assert!(reached);
        assert!(replay.authenticated.is_some());
        assert_eq!(replay.reservation.as_ref().unwrap().entries.len(), usize::from(checkpoint == "published"));
        cancelled.store(true, std::sync::atomic::Ordering::Release);
        assert!(matches!((&mut replay).await, Err(DbError::Closed)));
        assert!(replay.terminal_is_empty());
        eprintln!("[DEBUG] history cancelled and retired authenticated source at {checkpoint}");
    }
    while engine.wal.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    while engine.state.values.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    eprintln!("[DEBUG] history projected one committed two-operation entry with the exact submitted frontier");
}

#[semio_framework_async_macros::async_test]
async fn artifact_history_and_opener_reject_neutral_inner_documents_and_frontier_order() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../📝️wal/🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    for row in fixture["historyProjection"]["rejections"].as_array().unwrap() {
        let backing = storage().await;
        let mut engine = ArtifactEngine::create_retained(document_id().await, backing.clone(), ArtifactEngineConfig::default(), 0).await.unwrap();
        let mut records = db_wal::WalRecordBatch::new();
        for record in row["records"].as_array().unwrap() {
            let document = if record["document"] == "foreign" { "foreign-history-document".to_string() } else { engine.document.0.clone() };
            let record = match record["kind"].as_str().unwrap() {
                "command" => {
                    let mut command = envelope("hostile-history", &[], "history-author", &[("entry", serde_json::json!("value"))]).await;
                    command.document_id = protocol::ArtifactId(document);
                    db_wal::WalRecord::Command(retained_wal_envelope(&command).await)
                }
                "frontier" => db_wal::WalRecord::Frontier(Frontier { document: ArtifactId(document), head_seq: 1, commit_seq: 1, chain_hash: [7; 32], epoch: 3 }),
                _ => unreachable!(),
            };
            records.push(record).unwrap_or_else(|mut record| {
                while record.close_step().unwrap() {}
                panic!("hostile history fixture exceeded record capacity");
            });
        }
        let facet = backing.wal().await;
        engine.wal.submit(&facet, &records, DurabilityClass::Fsync, 1).await.unwrap();
        while records.close_step().unwrap() {
            semio_framework_async::yield_once().await;
        }
        drop(records);
        drop(facet);
        let mut replay = engine.history_replay(1, StdArc::new(std::sync::atomic::AtomicBool::new(false)), HistoryReplayReservation::try_new().unwrap());
        let rejected = match (&mut replay).await {
            Err(DbError::Corrupt(_)) => true,
            Ok(mut view) => {
                while view.close_step() {}
                false
            }
            Err(_) => false,
        };
        assert!(replay.terminal_is_empty());
        while engine.wal.close_step().unwrap() {
            semio_framework_async::yield_once().await;
        }
        while engine.state.values.close_step().unwrap() {
            semio_framework_async::yield_once().await;
        }
        drop(engine);
        assert!(rejected, "history admitted {}", row["name"]);
        let rejected = match ArtifactEngine::open_retained(document_id().await, backing, ArtifactEngineConfig::default(), 2).await {
            Err(rejected) => matches!(rejected_engine_open_error(rejected).await, DbError::Corrupt(_)),
            Ok((mut engine, _)) => {
                while engine.wal.close_step().unwrap() {
                    semio_framework_async::yield_once().await;
                }
                while engine.state.values.close_step().unwrap() {
                    semio_framework_async::yield_once().await;
                }
                false
            }
        };
        assert!(rejected, "opener admitted {}", row["name"]);
        eprintln!("[DEBUG] history and opener rejected authenticated committed projection and retired owners: {}", row["name"]);
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_staging_retirement_success_refusal_cancel_stale_fault_drop_interrupted_close_and_max_plus_one_are_lossless() {
    while artifact_state_retirement_maintenance_step().unwrap() {}
    let accepted_cancel = StdArc::new(std::sync::atomic::AtomicBool::new(false));
    let mut accepted_control = db_state::StateCursorControl::new(accepted_cancel, std::time::Instant::now() + std::time::Duration::from_secs(30), 8).unwrap();
    let mut accepted = db_state::StateEntry::try_admit("accepted", vec![0x41], 1, &mut accepted_control).await.unwrap();
    assert!(accepted.close_step().unwrap());
    assert!(accepted.close_step().unwrap());
    assert!(accepted.close_step().unwrap());
    assert!(!accepted.close_step().unwrap());
    assert!(accepted.terminal_is_empty());

    let cancelled = StdArc::new(std::sync::atomic::AtomicBool::new(true));
    let mut control = db_state::StateCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 8).unwrap();
    let rejected = db_state::StateEntry::try_admit("path", vec![0x42], 1, &mut control).await.unwrap_err();
    assert!(matches!(rejected.error(), DbError::Unavailable(message) if message == "state cursor cancelled"));
    let source = rejected.source().expect("exact refused source").as_ptr();
    let mut second_control = db_state::StateCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(true)), std::time::Instant::now() + std::time::Duration::from_secs(30), 8).unwrap();
    let second_rejected = db_state::StateEntry::try_admit("second-path", vec![0x44], 1, &mut second_control).await.unwrap_err();
    let second_source = second_rejected.source().expect("second exact refused source").as_ptr();
    let initial_retirement = reserve_artifact_state_retirement().expect("artifact retirement preflight");
    let mut cursor = ArtifactStateRetirementCursor::rejected(initial_retirement, rejected, std::array::from_fn(|_| None));
    assert!(cursor.close_step().unwrap());
    assert_eq!(cursor.rejected.as_ref().and_then(db_state::StateEntryRejected::source).map(Vec::as_ptr), Some(source));
    release_artifact_state_retirement(&mut cursor.retirement);
    {
        let mut retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in retired.iter_mut() {
            *slot = Some(ArtifactStateRetirementCursor::empty());
        }
    }
    {
        let mut overflow = ARTIFACT_STATE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in overflow.iter_mut() {
            *slot = Some(ArtifactStateRetirementCursor::empty());
        }
    }
    ARTIFACT_STATE_RETIREMENT_PRESSURE_FAULT.store(false, std::sync::atomic::Ordering::Release);
    cursor.retirement = Some(reserve_artifact_state_retirement().expect("artifact quarantine preflight"));
    let second_retirement = reserve_artifact_state_retirement().expect("second artifact quarantine preflight");
    assert!(retire_artifact_state_owner(cursor).is_ok());
    assert!(retire_artifact_state_owner(ArtifactStateRetirementCursor::rejected(second_retirement, second_rejected, std::array::from_fn(|_| None))).is_ok());
    assert!(ARTIFACT_STATE_RETIREMENT_PRESSURE_FAULT.load(std::sync::atomic::Ordering::Acquire));
    {
        let quarantine = ARTIFACT_STATE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(quarantine.iter().flatten().find_map(|owner| owner.rejected.as_ref()?.source().map(Vec::as_ptr)), Some(source));
        assert!(quarantine.iter().flatten().any(|owner| owner.rejected.as_ref().and_then(db_state::StateEntryRejected::source).map(Vec::as_ptr) == Some(second_source)));
    }
    {
        let mut retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in retired.iter_mut() {
            *slot = None;
        }
    }
    for _ in 0..ARTIFACT_STATE_RETIREMENT_SLOTS * 2 {
        assert!(artifact_state_retirement_maintenance_step().unwrap());
    }
    assert!(artifact_state_retirement_maintenance_step().unwrap());
    {
        let retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(retired.iter().flatten().find_map(|owner| owner.rejected.as_ref()?.source().map(Vec::as_ptr)), Some(source));
    }
    while artifact_state_retirement_maintenance_step().unwrap() {}

    let mut deadline_control = db_state::StateCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now(), 1).unwrap();
    let deadline = db_state::StateEntry::try_admit("deadline", vec![0x43], 1, &mut deadline_control).await.unwrap_err();
    assert!(matches!(deadline.error(), DbError::Unavailable(message) if message == "state cursor deadline reached"));
    let deadline_retirement = reserve_artifact_state_retirement().expect("deadline artifact retirement preflight");
    assert!(retire_artifact_state_owner(ArtifactStateRetirementCursor::rejected(deadline_retirement, deadline, std::array::from_fn(|_| None))).is_ok());
    while artifact_state_retirement_maintenance_step().unwrap() {}

    for tier in [&ARTIFACT_STATE_RETIREMENT, &ARTIFACT_STATE_RETIREMENT_OVERFLOW, &ARTIFACT_STATE_RETIREMENT_QUARANTINE] {
        let mut owners = tier.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in owners.iter_mut() {
            *slot = Some(ArtifactStateRetirementCursor::empty());
        }
    }
    let mut state = DocumentState::new();
    let entries = [("exact-all-tier-state-refusal".to_string(), None)];
    let refusal = match state.apply_entries(&protocol::MutationId("exact-all-tier-state-refusal".to_string()), &[], &entries).await {
        Err(error) => error,
        Ok(_) => panic!("all-tier artifact retirement saturation admitted a state mutation"),
    };
    assert!(matches!(refusal, DbError::Unavailable(message) if message == "artifact state retirement pressure refused admission"));
    for tier in [&ARTIFACT_STATE_RETIREMENT, &ARTIFACT_STATE_RETIREMENT_OVERFLOW, &ARTIFACT_STATE_RETIREMENT_QUARANTINE] {
        let mut owners = tier.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in owners.iter_mut() {
            *slot = None;
        }
    }
}

struct HistoryReplayTestWake;

impl std::task::Wake for HistoryReplayTestWake {
    fn wake(self: StdArc<Self>) {}
}

async fn storage() -> StdArc<db_storage::DbBackend> {
    StdArc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()))
}

async fn document_id() -> protocol::ArtifactId {
    protocol::ArtifactId("doc-1".to_string())
}

async fn stored_json(mut bytes: db_query::QueryBytes) -> serde_json::Value {
    let mut raw = Vec::with_capacity(bytes.len());
    for fragment in bytes.fragments() {
        raw.extend_from_slice(fragment);
    }
    while bytes.close_step().unwrap().is_some() {}
    decode_pathmap_json(&raw).await.expect("stored json value")
}

async fn envelope(id: &str, deps: &[&str], actor: &str, entries: &[(&str, serde_json::Value)]) -> protocol::MutationEnvelope {
    let object: Vec<(String, DslValue)> = entries.iter().map(|(path, value)| (path.to_string(), DslValue::from(value))).collect();
    protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(id.to_string()),
        document_id: document_id().await,
        actor: protocol::ActorId(actor.to_string()),
        dependencies: deps.iter().map(|dep| protocol::MutationId((*dep).to_string())).collect(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&DslValue::Object(object)).await },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&DslValue::Object(vec![])).await },
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
    }
}

async fn retained_wal_envelope(envelope: &protocol::MutationEnvelope) -> db_wal::WalBytes {
    let mut encoded = Vec::new();
    protocol::encode_envelope(envelope, &mut encoded);
    encoded.shrink_to_fit();
    let mut control = db_wal::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap();
    db_wal::WalBytes::try_admit(encoded, (db_storage::DB_IO_OPERATION_PAGES * db_storage::DB_IO_PAGE_BYTES) as u64, &mut control).await.unwrap()
}

fn poll_artifact_wal_decode(future: &mut ArtifactWalEnvelopeDecode<'_, '_>) -> std::task::Poll<Result<ArtifactWalRetainedEnvelope, DbError>> {
    let waker = std::task::Waker::from(StdArc::new(HistoryReplayTestWake));
    let mut context = std::task::Context::from_waker(&waker);
    Pin::new(future).poll(&mut context)
}

fn poll_artifact_wal_adapter(future: &mut ArtifactWalEnvelopeAdapter<'_>) -> std::task::Poll<Result<protocol::MutationEnvelope, DbError>> {
    let waker = std::task::Waker::from(StdArc::new(HistoryReplayTestWake));
    let mut context = std::task::Context::from_waker(&waker);
    Pin::new(future).poll(&mut context)
}

//#region 🔖️Command
#[semio_framework_async_macros::async_test]
async fn command_batch_rejects_empty_and_mixed_documents() {
    assert!(CommandBatch::new(Vec::new()).await.is_err());
    let mut mismatched = envelope("op-2", &[], "alice", &[("x", serde_json::json!(1))]).await;
    mismatched.document_id = protocol::ArtifactId("other-doc".to_string());
    let batch = CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await, mismatched]).await;
    assert!(batch.is_err());
}
//#endregion 🔖️Command

//#region 🔖️Bridge
mod bridge {
    use super::*;

    #[derive(Clone, serde::Serialize, serde::Deserialize, store::ToValue, store::FromValue)]
    struct Counter {
        value: i64,
    }

    #[derive(Clone, Default, serde::Serialize, serde::Deserialize, store::ToValue, store::FromValue)]
    struct AddDiff {
        amount: i64,
    }

    impl protocol::MutationDiff<Counter> for AddDiff {
        fn apply(&self, base: &Counter) -> protocol::MutationApplyResult<Counter> {
            Ok(Counter { value: base.value + self.amount })
        }
        fn absorb(&mut self, other: Self) {
            self.amount += other.amount;
        }
    }

    #[derive(Clone, serde::Serialize, serde::Deserialize, store::ToValue, store::FromValue)]
    struct Add {
        amount: i64,
    }

    const ADD_DESCRIPTOR: protocol::MutationLeafDescriptor = protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "framework/os/db/artifact/test-add",
        semantic_kind: "add",
        display_name: "Add",
        emoji: "➕️",
        aggregate_variant: "Add",
        payload_schema: "db.artifact.test-add/v1",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    };

    impl protocol::Mutation<Counter> for Add {
        type Diff = AddDiff;
        const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[ADD_DESCRIPTOR];
        fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
            &ADD_DESCRIPTOR
        }
        fn diff(&self, _base: &Counter) -> protocol::MutationOutcome<AddDiff> {
            protocol::MutationOutcome::new(AddDiff { amount: self.amount })
        }
        fn inverse(&self, _base: &Counter) -> Vec<Self> {
            vec![Add { amount: -self.amount }]
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn envelope_from_operation_uses_operation_and_diff_traits() {
        let base = Counter { value: 10 };
        let op = Add { amount: 5 };
        let envelope = envelope_from_operation(document_id().await, "counter", &op, &base, protocol::ActorId("alice".to_string()), protocol::MutationId("op-add-1".to_string()), protocol::HybridLogicalTimestamp::new(1, 0)).await.unwrap();
        let entries = diff_entries(&envelope.diff).await.unwrap();
        assert_eq!(entries.len(), 1);
        let (path, value) = &entries[0];
        assert_eq!(path, "counter");
        let new_value: Counter = dsl::from_dsl_value(value.clone().unwrap()).unwrap();
        assert_eq!(new_value.value, 15);
    }
}
//#endregion 🔖️Bridge

//#region 🔖️Engine submit + materialize + WAL replay
#[semio_framework_async_macros::async_test]
async fn retained_wal_decoder_covers_pending_cancel_deadline_corrupt_max_and_max_plus_one() {
    let mut expected = envelope("wal-real-path", &["dependency-a", "dependency-b"], "worker", &[]).await;
    expected.diff.payload = vec![0x4d; db_storage::DB_IO_PAGE_BYTES + 17];
    expected.inverse.payload = vec![0x2a; db_storage::DB_IO_PAGE_BYTES + 1];
    let mut bytes = retained_wal_envelope(&expected).await;
    let cancelled = StdArc::new(std::sync::atomic::AtomicBool::new(false));
    let mut control = db_wal::WalCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap();
    let mut decode = decode_retained_envelope(&bytes, &mut control);
    assert!(poll_artifact_wal_decode(&mut decode).is_pending());
    assert_eq!(decode.phase, 0);
    let retained = decode.await.unwrap();
    let mut adapter_control = db_wal::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap();
    let mut adapter = adapt_retained_envelope(retained, &mut adapter_control);
    assert!(poll_artifact_wal_adapter(&mut adapter).is_pending());
    let actual = adapter.await.unwrap();
    assert_eq!(actual.mutation_id, expected.mutation_id);
    assert_eq!(actual.dependencies, expected.dependencies);
    assert_eq!(actual.diff.payload, expected.diff.payload);
    assert_eq!(actual.inverse.payload, expected.inverse.payload);
    while bytes.close_step().unwrap().is_some() {}

    let mut cancelled_bytes = retained_wal_envelope(&expected).await;
    let cancellation = StdArc::new(std::sync::atomic::AtomicBool::new(false));
    let mut cancelled_control = db_wal::WalCursorControl::new(cancellation.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap();
    let mut interrupted = decode_retained_envelope(&cancelled_bytes, &mut cancelled_control);
    while interrupted.phase < 6 {
        assert!(poll_artifact_wal_decode(&mut interrupted).is_pending());
    }
    assert!(poll_artifact_wal_decode(&mut interrupted).is_pending());
    cancellation.store(true, std::sync::atomic::Ordering::Release);
    assert!(matches!(poll_artifact_wal_decode(&mut interrupted), std::task::Poll::Ready(Err(DbError::Unavailable(message))) if message == "wal cursor cancelled"));
    drop(interrupted);
    while db_storage::db_io_maintenance_step().unwrap() {}
    while cancelled_bytes.close_step().unwrap().is_some() {}

    let mut deadline_bytes = retained_wal_envelope(&expected).await;
    let mut deadline_control = db_wal::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now(), 1_000_000).unwrap();
    assert!(matches!(decode_retained_envelope(&deadline_bytes, &mut deadline_control).await, Err(DbError::Unavailable(message)) if message == "wal cursor deadline reached"));
    while deadline_bytes.close_step().unwrap().is_some() {}

    let mut corrupt_encoded = Vec::new();
    protocol::encode_envelope(&expected, &mut corrupt_encoded);
    corrupt_encoded.push(0xff);
    corrupt_encoded.shrink_to_fit();
    let mut corrupt_admission = db_wal::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap();
    let mut corrupt = db_wal::WalBytes::try_admit(corrupt_encoded, (db_storage::DB_IO_OPERATION_PAGES * db_storage::DB_IO_PAGE_BYTES) as u64, &mut corrupt_admission).await.unwrap();
    let mut corrupt_control = db_wal::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap();
    assert!(matches!(decode_retained_envelope(&corrupt, &mut corrupt_control).await, Err(DbError::Corrupt(message)) if message == "WAL command envelope has trailing bytes"));
    while db_storage::db_io_maintenance_step().unwrap() {}
    while corrupt.close_step().unwrap().is_some() {}

    let max_dependencies: Vec<protocol::MutationId> = (0..ARTIFACT_WAL_DEPENDENCIES).map(|index| protocol::MutationId(format!("dependency-{index}"))).collect();
    let mut maximum = expected.clone();
    maximum.dependencies = max_dependencies;
    maximum.diff.payload = vec![0x6d; ARTIFACT_WAL_FIELD_BYTES];
    maximum.inverse.payload.clear();
    let mut maximum_bytes = retained_wal_envelope(&maximum).await;
    let mut maximum_control = db_wal::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap();
    let maximum_retained = decode_retained_envelope(&maximum_bytes, &mut maximum_control).await.unwrap();
    let mut maximum_adapter_control = db_wal::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap();
    assert_eq!(adapt_retained_envelope(maximum_retained, &mut maximum_adapter_control).await.unwrap().diff.payload.len(), ARTIFACT_WAL_FIELD_BYTES);
    while maximum_bytes.close_step().unwrap().is_some() {}

    let mut dependency_refusal = maximum.clone();
    dependency_refusal.dependencies.push(protocol::MutationId("dependency-max-plus-one".to_string()));
    dependency_refusal.diff.payload.clear();
    let mut dependency_refusal_bytes = retained_wal_envelope(&dependency_refusal).await;
    let mut dependency_refusal_control = db_wal::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap();
    assert!(matches!(decode_retained_envelope(&dependency_refusal_bytes, &mut dependency_refusal_control).await, Err(DbError::LimitExceeded("artifact WAL envelope dependencies"))));
    while dependency_refusal_bytes.close_step().unwrap().is_some() {}

    let mut page_refusal = expected;
    page_refusal.diff.payload = vec![0x7d; ARTIFACT_WAL_FIELD_BYTES + 1];
    page_refusal.inverse.payload.clear();
    let mut page_refusal_bytes = retained_wal_envelope(&page_refusal).await;
    let mut page_refusal_control = db_wal::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap();
    assert!(matches!(decode_retained_envelope(&page_refusal_bytes, &mut page_refusal_control).await, Err(DbError::LimitExceeded("wal retained field"))));
    while page_refusal_bytes.close_step().unwrap().is_some() {}
}

#[semio_framework_async_macros::async_test]
async fn submit_persists_to_wal_and_updates_materialized_state_and_frontier() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();

    let batch = CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("name", serde_json::json!("hello"))]).await]).await.unwrap();
    let receipt = engine.submit(batch, SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, 1).await.unwrap();

    assert_eq!(receipt.command_id, protocol::MutationId("op-1".to_string()));
    assert_eq!(receipt.frontier.head_seq, 1);
    assert_eq!(receipt.frontier.commit_seq, 1);
    assert!(receipt.conflicts.is_empty());
    assert!(receipt.state_hash.is_some());

    let stored = engine.get("name").await.unwrap().unwrap();
    let value: serde_json::Value = stored_json(stored).await;
    assert_eq!(value, serde_json::json!("hello"));
    assert_eq!(engine.frontier().await.head_seq, 1);
    assert_eq!(engine.checkpoint_publication_snapshot().await.head_edit_id, Some(protocol::MutationId("op-1".to_string())));
}

#[semio_framework_async_macros::async_test]
async fn open_replays_the_wal_and_reconstructs_state_and_frontier_identically() {
    let storage = storage().await;
    let (before_count, before_frontier) = {
        let mut engine = ArtifactEngine::create(document_id().await, storage.clone(), ArtifactEngineConfig::default(), 0).unwrap();
        let batch1 = CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("name", serde_json::json!("hello"))]).await]).await.unwrap();
        engine.submit(batch1, SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, 1).await.unwrap();
        let batch2 = CommandBatch::new(vec![envelope("op-2", &["op-1"], "alice", &[("count", serde_json::json!(2))]).await]).await.unwrap();
        engine.submit(batch2, SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, 2).await.unwrap();
        let count = stored_json(engine.get("count").await.unwrap().unwrap()).await;
        assert!(store::pack_rt::json_values_equal(&count, &serde_json::json!(2)));
        let frontier = engine.frontier().await;
        while engine.wal.close_step().unwrap() {
            semio_framework_async::yield_once().await;
        }
        while engine.state.values.close_step().unwrap() {
            semio_framework_async::yield_once().await;
        }
        (count, frontier)
    };

    let (mut reopened, report) = ArtifactEngine::open(document_id().await, &storage, ArtifactEngineConfig::default(), 3).unwrap();
    assert_eq!(report.torn_tail_bytes, 0);
    assert_eq!(reopened.frontier().await.head_seq, 2);
    assert_eq!(reopened.frontier().await.commit_seq, 2);
    assert_eq!(reopened.frontier().await, before_frontier);
    assert_eq!(reopened.checkpoint_publication_snapshot().await.head_edit_id, Some(protocol::MutationId("op-2".to_string())));

    let name: serde_json::Value = stored_json(reopened.get("name").await.unwrap().unwrap()).await;
    assert_eq!(name, serde_json::json!("hello"));
    let count: serde_json::Value = stored_json(reopened.get("count").await.unwrap().unwrap()).await;
    assert_eq!(count, before_count);
    while reopened.wal.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    while reopened.state.values.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    eprintln!("[DEBUG] replay preserved the complete live frontier and exact decoded numeric representation after explicit owner retirement");
}

#[semio_framework_async_macros::async_test]
async fn materialize_from_snapshot_plus_wal_suffix_matches_full_replay() {
    let storage = storage().await;
    {
        let mut engine = ArtifactEngine::create(document_id().await, storage.clone(), ArtifactEngineConfig::default(), 0).unwrap();
        for i in 0..3 {
            let key = format!("path-{i}");
            let value = format!("value-{i}");
            let batch = CommandBatch::new(vec![envelope(&format!("op-{i}"), &[], "alice", &[(&key, serde_json::json!(value))]).await]).await.unwrap();
            engine.submit(batch, SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, i).await.unwrap();
        }
        engine.snapshot_now(10).await.unwrap();
        for i in 3..6 {
            let key = format!("path-{i}");
            let value = format!("value-{i}");
            let batch = CommandBatch::new(vec![envelope(&format!("op-{i}"), &[], "alice", &[(&key, serde_json::json!(value))]).await]).await.unwrap();
            engine.submit(batch, SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, i).await.unwrap();
        }
    }

    let (reopened, report) = ArtifactEngine::open(document_id().await, &storage, ArtifactEngineConfig::default(), 20).unwrap();
    assert!(report.from_snapshot);
    assert_eq!(report.commands_replayed, 3);
    assert_eq!(reopened.frontier().await.head_seq, 6);
    for i in 0..6 {
        let value: serde_json::Value = stored_json(reopened.get(&format!("path-{i}")).await.unwrap().unwrap()).await;
        assert_eq!(value, serde_json::json!(format!("value-{i}")));
    }
}

#[semio_framework_async_macros::async_test]
async fn deletion_via_json_null_tombstones_a_path() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();
    assert!(engine.get("x").await.unwrap().is_some());
    engine.submit(CommandBatch::new(vec![envelope("op-2", &["op-1"], "alice", &[("x", serde_json::Value::Null)]).await]).await.unwrap(), SubmitOptions::default(), 1).await.unwrap();
    assert!(engine.get("x").await.unwrap().is_none());
}
//#endregion 🔖️Engine submit + materialize + WAL replay

//#region 🔖️Deps + Dedupe
#[semio_framework_async_macros::async_test]
async fn submit_rejects_an_envelope_whose_dependency_was_never_applied() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    let batch = CommandBatch::new(vec![envelope("op-2", &["op-1-never-applied"], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap();
    let result = engine.submit(batch, SubmitOptions::default(), 0);
    assert!(matches!(result.await, Err(DbError::InvalidArgument(_))));
    assert!(engine.get("x").await.unwrap().is_none(), "a rejected batch must not have partially applied");
}

#[semio_framework_async_macros::async_test]
async fn resubmitting_the_same_batch_returns_the_cached_receipt_without_advancing_the_frontier() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    let batch = || db_actor::block_on(CommandBatch::new(vec![db_actor::block_on(envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]))])).unwrap();

    let first = engine.submit(batch(), SubmitOptions::default(), 0).await.unwrap();
    let frontier_after_first = engine.frontier().await;
    let second = engine.submit(batch(), SubmitOptions::default(), 1).await.unwrap();

    assert_eq!(first, second);
    assert_eq!(engine.frontier().await, frontier_after_first, "a deduped resubmit must not move the frontier");
}
//#endregion 🔖️Deps + Dedupe

//#region 🔖️Conflict
#[semio_framework_async_macros::async_test]
async fn concurrent_write_to_the_same_path_without_a_dependency_is_recorded_as_a_conflict() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();

    // op-2 writes the same path but does NOT declare op-1 as a dependency: a real concurrent
    // write from `op-2`'s author's point of view.
    let receipt = engine.submit(CommandBatch::new(vec![envelope("op-2", &[], "bob", &[("x", serde_json::json!(2))]).await]).await.unwrap(), SubmitOptions::default(), 1).await.unwrap();
    assert_eq!(receipt.conflicts.len(), 1);
    assert_eq!(receipt.conflicts[0].conflicting_with, protocol::MutationId("op-1".to_string()));
    assert_eq!(receipt.conflicts[0].path, "x");
    // Last-writer-wins: the conflicting write still applies.
    let x: serde_json::Value = stored_json(engine.get("x").await.unwrap().unwrap()).await;
    assert_eq!(x, serde_json::json!(2));
}

#[semio_framework_async_macros::async_test]
async fn declaring_the_prior_writer_as_a_dependency_avoids_the_conflict() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();
    let receipt = engine.submit(CommandBatch::new(vec![envelope("op-2", &["op-1"], "bob", &[("x", serde_json::json!(2))]).await]).await.unwrap(), SubmitOptions::default(), 1).await.unwrap();
    assert!(receipt.conflicts.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn preview_conflicts_uses_real_db_conflict_detector_against_recent_history() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();

    let probe = CommandBatch::new(vec![envelope("op-2", &[], "bob", &[("x", serde_json::json!(2))]).await]).await.unwrap();
    let conflicts = engine.preview_conflicts(&probe).await.unwrap();
    assert!(!conflicts.is_empty(), "db_conflict must detect the same-path intersection against recent history");
}
//#endregion 🔖️Conflict

//#region 🔖️Undo
#[semio_framework_async_macros::async_test]
async fn undo_applies_the_recorded_inverse_and_produces_a_fresh_commit() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    let original = protocol::MutationEnvelope {
        mutation_id: protocol::MutationId("op-1".to_string()),
        document_id: document_id().await,
        actor: protocol::ActorId("alice".to_string()),
        dependencies: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&DslValue::from(&serde_json::json!({ "x": 1 }))).await },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&DslValue::from(&serde_json::json!({ "x": null }))).await },
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
    };
    engine.submit(CommandBatch::new(vec![original]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();
    assert!(engine.get("x").await.unwrap().is_some());

    let receipt = engine.undo(&protocol::MutationId("op-1".to_string()), protocol::MutationId("op-1-undo".to_string()), protocol::ActorId("alice".to_string()), 1).await.unwrap();
    assert_eq!(receipt.frontier.head_seq, 2);
    assert!(engine.get("x").await.unwrap().is_none(), "undo must have applied the recorded inverse (delete x)");
}

#[semio_framework_async_macros::async_test]
async fn undo_of_an_unknown_operation_errs_not_found() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    let never_applied = protocol::MutationId("never-applied".to_string());
    let result = engine.undo(&never_applied, protocol::MutationId("undo-1".to_string()), protocol::ActorId("alice".to_string()), 0);
    assert!(matches!(result.await, Err(DbError::NotFound(_))));
}
//#endregion 🔖️Undo

//#region 🔖️Preview
#[semio_framework_async_macros::async_test]
async fn preview_is_never_durable_and_a_conflicting_commit_supersedes_it() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();

    let preview_id = engine.publish_preview(&[("y".to_string(), Some(serde_json::json!("preview-value")))], 0).await.unwrap();
    assert_eq!(engine.preview_status(&preview_id).await.unwrap(), db_preview::PreviewState::Active);
    let preview_value: serde_json::Value = stored_json(engine.preview_get(&preview_id, "y").await.unwrap().unwrap()).await;
    assert_eq!(preview_value, serde_json::json!("preview-value"));
    assert!(engine.get("y").await.unwrap().is_none(), "a preview must never be visible in committed state");

    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "bob", &[("y", serde_json::json!("committed-value"))]).await]).await.unwrap(), SubmitOptions::default(), 1).await.unwrap();
    assert_eq!(engine.preview_status(&preview_id).await.unwrap(), db_preview::PreviewState::Superseded, "an intersecting real commit must supersede the preview");

    let committed: serde_json::Value = stored_json(engine.get("y").await.unwrap().unwrap()).await;
    assert_eq!(committed, serde_json::json!("committed-value"));
}

#[semio_framework_async_macros::async_test]
async fn preview_withdraw_and_expire_transitions() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();

    let withdrawn_id = engine.publish_preview(&[("a".to_string(), Some(serde_json::json!(1)))], 0).await.unwrap();
    engine.withdraw_preview(&withdrawn_id).await.unwrap();
    assert_eq!(engine.preview_status(&withdrawn_id).await.unwrap(), db_preview::PreviewState::Withdrawn);

    let dummy_id = db_preview::PreviewId("does-not-exist".to_string());
    assert!(matches!(engine.preview_status(&dummy_id).await, Err(DbError::NotFound(_))));
}
//#endregion 🔖️Preview

//#region 🔖️Security
#[semio_framework_async_macros::async_test]
async fn security_gate_rejects_a_principal_denied_by_its_policy() {
    // An empty `RoleBasedPolicy` (no grants at all) denies every action, per its own doc — a
    // default-deny policy, matching `db_engine`'s own equivalent test of the same gate.
    let security = db_security::SecurityGate::new(db_security::RoleBasedPolicy::new(), db_security::ReplayGuard::new(60_000, 1_024), db_security::BudgetRegistry::new(100_000, 100_000), Arc::new(NullEmit));
    let storage = storage().await;
    let config = ArtifactEngineConfig { security, ..ArtifactEngineConfig::default() };
    let mut engine = ArtifactEngine::create(document_id().await, storage, config, 0).unwrap();
    let result = engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "bob", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0);
    assert!(matches!(result.await, Err(DbError::Unauthorized(_))));
    assert!(engine.get("x").await.unwrap().is_none());
}
//#endregion 🔖️Security

//#region 🔖️Query
#[semio_framework_async_macros::async_test]
async fn query_finds_a_committed_row_by_path() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("greeting", serde_json::json!("hello"))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();

    let query = db_query::Query::new().filter(db_query::Predicate::Eq(db_query::Path::empty().push_field("path"), db_query::Value::Text("greeting".to_string())));
    let result = engine.query(query, db_query::Consistency::Canonical).await.unwrap();
    assert_eq!(result.rows.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn live_query_refresh_reports_no_further_diff_right_after_submit_already_refreshed_it() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    let id = engine.subscribe(db_query::LiveQuerySpec { query: db_query::Query::new(), consistency: db_query::Consistency::Canonical }).await;
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();
    let diffs = engine.refresh_live_queries().await;
    assert!(diffs.is_empty());
    engine.unsubscribe(id).await;
}
//#endregion 🔖️Query

//#region 🔖️Outbox + CommitLog
#[semio_framework_async_macros::async_test]
async fn outbox_and_commit_log_accumulate_and_outbox_drains() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();

    assert_eq!(engine.commit_log().await.len(), 1);
    assert_eq!(engine.commit_log().await[0].operation_ids, vec![protocol::MutationId("op-1".to_string())]);

    let drained = engine.drain_outbox().await;
    assert_eq!(drained.len(), 1);
    assert_eq!(drained[0].mutation_id, protocol::MutationId("op-1".to_string()));
    assert!(engine.drain_outbox().await.is_empty(), "drain must clear the outbox");
}
//#endregion 🔖️Outbox + CommitLog

//#region 🔖️Actor
async fn journal_authority() -> (ArtifactAuthority, StdArc<db_storage::DbBackend>, StdArc<semio_framework_async::WorkerPool>) {
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 3)));
    let storage = storage().await;
    let engine_storage = storage.clone();
    let authority = ArtifactAuthority::spawn(pool.clone(), move || ArtifactEngine::create_retained(protocol::ArtifactId("map-a".to_string()), engine_storage, ArtifactEngineConfig::default(), 0), MailboxCapacities::uniform(16)).await.unwrap();
    (authority, storage, pool)
}

fn journal_grant() -> store::ArtifactStoreOneItemGrant {
    store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES }
}

#[cfg(feature = "vcs")]
async fn committed_recovery_hash_store(id: &str, dialect: store::os_io::ArtifactDialect, owner: Option<store::OwnerRef>) -> store::ArtifactStore<crate::db_engine::vcs_integration::HashProjection, crate::db_engine::vcs_integration::HashMutation> {
    use store::MemberStoreOwner as _;
    let mut envelope = store::create_document_envelope::<crate::db_engine::vcs_integration::HashProjection, crate::db_engine::vcs_integration::HashMutation>("db.hash/v1", id, crate::db_engine::vcs_integration::HashProjection::default(), None);
    envelope.dialect = Some(dialect);
    envelope.owner = owner;
    let mut store = store::ArtifactStore::new(envelope).await.expect("committed recovery fixture creates one exact Store");
    store.install_member_store_owners_exact(crate::db_engine::vcs_integration::HashProjection::member_store_owners());
    store
}

#[cfg(feature = "vcs")]
fn close_committed_recovery_hash_store(store: &mut store::ArtifactStore<crate::db_engine::vcs_integration::HashProjection, crate::db_engine::vcs_integration::HashMutation>) {
    for _ in 0..4_096 {
        if store::SpaceMember::close_owned_step(store, 1, 4_096).expect("committed recovery fixture Store closes") == store::SnapshotRetirementStep::Complete {
            assert!(store::SpaceMember::close_owned_terminal_is_empty(store));
            return;
        }
    }
    panic!("committed recovery fixture Store did not reach terminal handoff");
}

fn close_journal_commit(commit: &mut dyn store::durable_group::DurableOwnedGroupJournalCommitV1) {
    commit.begin_close();
    for _ in 0..8 {
        if commit.close_step(journal_grant()).unwrap() == store::SnapshotRetirementStep::Complete {
            assert!(commit.terminal_is_empty());
            return;
        }
    }
    panic!("journal commit did not reach its terminal-empty witness");
}

async fn shutdown_journal_authority(authority: &ArtifactAuthority, pool: &semio_framework_async::WorkerPool) {
    let mut terminal_job = None;
    for _ in 0..10_000 {
        if authority.shutdown_step() {
            pool.shutdown();
            return;
        }
        if terminal_job.is_none() {
            terminal_job = authority.take_terminal_job();
        }
        if let Some(job) = terminal_job.take() {
            if let Err(retained) = job.close() {
                terminal_job = Some(retained);
            }
        }
        semio_framework_async::yield_once().await;
    }
    panic!("journal authority did not terminate: {}", authority.shutdown_debug_witness());
}

async fn durable_group_witness_batch(kinds: &[serde_json::Value], canonical_pack: &[u8]) -> db_wal::WalRecordBatch {
    let mut control = db_wal::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap();
    let mut records = db_wal::WalRecordBatch::new();
    for kind in kinds {
        let bytes = admit_wal_bytes(canonical_pack.to_vec(), db_storage::DB_IO_MAX_READ_BYTES, &mut control).await.unwrap();
        let record = match kind.as_str().unwrap() {
            "event" => db_wal::WalRecord::Event(bytes),
            "command" => db_wal::WalRecord::Command(bytes),
            other => panic!("unknown durable witness record kind {other}"),
        };
        push_wal_record(&mut records, record, &mut control).await.unwrap();
    }
    records
}

#[semio_framework_async_macros::async_test]
async fn document_authority_durable_group_journal_commits_one_exact_fsync_event() {
    let (authority, storage, pool) = journal_authority().await;
    let record = store::durable_group::durable_owned_group_journal_test_record();
    let canonical_pack = record.canonical_pack().to_vec();
    let decision_sha256 = record.decision_sha256().to_string();
    let anchor_sha256 = record.anchor_sha256().to_string();
    let durable_head_edit_id = protocol::MutationId(record.parent_edit_id().to_string());
    let durable_chain_hash = record.parent_post_revision();
    let mut sink = authority.durable_group_journal_sink(7);
    let mut commit = sink.begin_commit(canonical_pack.clone(), decision_sha256.clone());
    let receipt = loop {
        match commit.advance(journal_grant()).unwrap() {
            store::durable_group::DurableOwnedGroupJournalAdvanceV1::Pending => semio_framework_async::yield_once().await,
            store::durable_group::DurableOwnedGroupJournalAdvanceV1::Committed(receipt) => break receipt,
            other => panic!("exact durable journal was not committed: {other:?}"),
        }
    };
    assert_eq!(receipt.anchor_sha256, anchor_sha256);
    assert_eq!(receipt.decision_sha256, decision_sha256);
    let durable_snapshot = authority.checkpoint_publication_snapshot().await.unwrap();
    assert_eq!(durable_snapshot.frontier.head_seq, 1);
    assert_eq!(durable_snapshot.frontier.commit_seq, 1);
    assert_eq!(durable_snapshot.frontier.chain_hash, durable_chain_hash);
    assert_eq!(durable_snapshot.head_edit_id, Some(durable_head_edit_id.clone()));
    close_journal_commit(commit.as_mut());
    drop(commit);
    drop(sink);
    let mut ordinary = envelope("ordinary-after-durable-decision", &[], "map-owner", &[("/ordinary", serde_json::json!(true))]).await;
    ordinary.document_id = protocol::ArtifactId("map-a".to_string());
    authority.submit(CommandBatch::new(vec![ordinary]).await.unwrap(), SubmitOptions { durability: DurabilityClass::Fsync, ..SubmitOptions::default() }, 8).await.unwrap();
    let ordinary_snapshot = authority.checkpoint_publication_snapshot().await.unwrap();
    assert_eq!(ordinary_snapshot.frontier.head_seq, 2);
    assert_eq!(ordinary_snapshot.frontier.commit_seq, 2);
    assert_eq!(ordinary_snapshot.head_edit_id, Some(protocol::MutationId("ordinary-after-durable-decision".to_string())));
    shutdown_journal_authority(&authority, &pool).await;

    let (mut reopened, report) = ArtifactEngine::open_retained(protocol::ArtifactId("map-a".to_string()), storage.clone(), ArtifactEngineConfig::default(), 9).await.unwrap();
    assert_eq!(report.commands_replayed, 1);
    let reopened_snapshot = reopened.checkpoint_publication_snapshot().await;
    assert_eq!(reopened_snapshot, ordinary_snapshot, "reopen must reconstruct the exact Event-plus-Command publication frontier");
    while reopened.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    drop(reopened);

    let wal_facet = storage.wal().await;
    let mut replay = db_wal::replay_committed_document(
        &wal_facet,
        &ArtifactId::from("map-a"),
        db_wal::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap(),
    )
    .await
    .unwrap();
    let replay_document = ArtifactId::from("map-a");
    let mut witnesses = Vec::new();
    let mut committed_transactions = 0;
    loop {
        let transaction = match replay.next_transaction_step().await.unwrap() {
            db_wal::WalCommittedStep::Transaction(transaction) => transaction,
            db_wal::WalCommittedStep::Yield => continue,
            db_wal::WalCommittedStep::Done => break,
        };
        committed_transactions += 1;
        if let Some(witness) = committed_durable_group_decision_from_transaction(&replay_document, transaction).await.unwrap() {
            witnesses.push(witness);
        }
    }
    while replay.close_owner_step().unwrap() {}
    assert_eq!(committed_transactions, 2);
    assert_eq!(witnesses.len(), 1);
    let witness = witnesses.pop().unwrap();
    assert_eq!(witness.document(), &replay_document);
    assert_eq!(witness.transaction_id(), receipt.transaction_id);
    assert_eq!(witness.segment_index(), receipt.segment_index);
    assert_eq!(witness.receipt(), receipt);
    assert_eq!(witness.record.canonical_pack(), canonical_pack);
    assert_eq!(witness.record.decision_sha256(), decision_sha256);
    assert_eq!(receipt.transaction_id, 1);
    let source = include_str!("../../🦀️.rs");
    let witness_api = &source[source.find("pub struct ArtifactCommittedDurableGroupDecisionV1").unwrap()..source.find("pub(crate) fn committed_durable_group_decision_from_transaction").unwrap()];
    assert!(witness_api.contains("into_store_owned_recovery") && witness_api.contains("take_rejected_terminal"));
    assert!(!witness_api.contains("fn record(&self)") && !witness_api.contains("fn into_record("));
    assert!(!witness_api.contains("fn cancel("));
    eprintln!("[DEBUG] typed authority journal committed one exact canonical Store decision, projected its actor frontier before acknowledgement, and reconstructed that exact frontier on reopen");
}

#[semio_framework_async_macros::async_test]
async fn committed_durable_group_decision_accepts_only_one_exact_event_transaction() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📓️durable-group-journal/🔣️.json")).unwrap();
    let document = ArtifactId::from(fixture["record"]["document"].as_str().unwrap());
    let record = store::durable_group::durable_owned_group_journal_test_record();
    let canonical_pack = record.canonical_pack().to_vec();
    let backing = storage().await;
    let wal_storage = backing.wal().await;
    let mut wal = db_wal::ArtifactWal::create(&wal_storage, document.clone(), db_wal::GroupCommitPolicy::default(), 0).await.unwrap();
    for (ordinal, row) in fixture["committedDecisionWitnessCases"].as_array().unwrap().iter().filter(|row| row["transaction"] == "committed").enumerate() {
        let mut records = durable_group_witness_batch(row["recordKinds"].as_array().unwrap(), &canonical_pack).await;
        let receipt = wal.submit(&wal_storage, &records, DurabilityClass::Fsync, ordinal as u64 + 1).await.unwrap();
        assert!(receipt.committed);
        close_wal_record_batch(&mut records).await.unwrap();
    }
    wal.close().await.unwrap();

    let mut replay =
        db_wal::replay_committed_document(&wal_storage, &document, db_wal::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap())
            .await
            .unwrap();
    for row in fixture["committedDecisionWitnessCases"].as_array().unwrap().iter().filter(|row| row["transaction"] == "committed") {
        let transaction = loop {
            match replay.next_transaction_step().await.unwrap() {
                db_wal::WalCommittedStep::Transaction(transaction) => break transaction,
                db_wal::WalCommittedStep::Yield => continue,
                db_wal::WalCommittedStep::Done => panic!("durable witness fixture lost a committed transaction"),
            }
        };
        let replay_document = if row["replayDocument"] == "foreign" { ArtifactId::from("foreign-map") } else { document.clone() };
        let outcome = committed_durable_group_decision_from_transaction(&replay_document, transaction).await;
        match row["expected"].as_str().unwrap() {
            "witness" => {
                let witness = outcome.unwrap().expect("one exact Event must produce a committed decision witness");
                assert_eq!(witness.document(), &replay_document);
                assert_eq!(witness.record.canonical_pack(), canonical_pack);
            }
            "ignored" => assert!(outcome.unwrap().is_none()),
            "rejected" => assert!(matches!(outcome, Err(DbError::Corrupt(_)))),
            other => panic!("unknown durable witness expectation {other}"),
        }
    }
    loop {
        match replay.next_transaction_step().await.unwrap() {
            db_wal::WalCommittedStep::Yield => continue,
            db_wal::WalCommittedStep::Done => break,
            db_wal::WalCommittedStep::Transaction(_) => panic!("durable witness fixture replayed an unexpected committed transaction"),
        }
    }
    while replay.close_owner_step().unwrap() {}
    assert!(replay.terminal_is_empty());

    let aborted = db_wal::tests::aborted_event_fixture_storage(&document, &canonical_pack).await;
    let mut replay =
        db_wal::replay_committed_document(&aborted, &document, db_wal::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap()).await.unwrap();
    loop {
        match replay.next_transaction_step().await.unwrap() {
            db_wal::WalCommittedStep::Yield => continue,
            db_wal::WalCommittedStep::Done => break,
            db_wal::WalCommittedStep::Transaction(_) => panic!("aborted durable witness fixture exposed a committed transaction"),
        }
    }
    while replay.close_owner_step().unwrap() {}
    assert!(replay.terminal_is_empty());
    eprintln!("[DEBUG] committed decision witness admitted one sole canonical Event, ignored Command or aborted Event, and rejected mixed, duplicate, or foreign Event transactions");
}

#[cfg(feature = "vcs")]
#[semio_framework_async_macros::async_test]
async fn committed_durable_group_recovery_consumes_wal_witness_and_returns_exact_three_stores_on_pre_mutation_rejection() {
    let document = ArtifactId::from("map-a");
    let record = store::durable_group::durable_owned_group_journal_test_record();
    let anchor_sha256 = record.anchor_sha256().to_string();
    let decision_sha256 = record.decision_sha256().to_string();
    let canonical_pack = record.into_canonical_pack();
    let backing = storage().await;
    let wal_storage = backing.wal().await;
    let mut wal = db_wal::ArtifactWal::create(&wal_storage, document.clone(), db_wal::GroupCommitPolicy::default(), 0).await.unwrap();
    let mut records = durable_group_witness_batch(&[serde_json::Value::String("event".to_string())], &canonical_pack).await;
    wal.submit(&wal_storage, &records, DurabilityClass::Fsync, 1).await.unwrap();
    close_wal_record_batch(&mut records).await.unwrap();
    wal.close().await.unwrap();
    let mut replay =
        db_wal::replay_committed_document(&wal_storage, &document, db_wal::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap())
            .await
            .unwrap();
    let transaction = loop {
        match replay.next_transaction_step().await.unwrap() {
            db_wal::WalCommittedStep::Transaction(transaction) => break transaction,
            db_wal::WalCommittedStep::Yield => continue,
            db_wal::WalCommittedStep::Done => panic!("committed recovery fixture lost its decision transaction"),
        }
    };
    let witness = committed_durable_group_decision_from_transaction(&document, transaction).await.unwrap().expect("one committed Event produces one opaque DB witness");
    while replay.close_owner_step().unwrap() {}

    let parent_dialect = store::os_io::ArtifactDialect { artifact_kind: "s.gis.gismap".into(), standard: "1".into(), subset: "*".into() };
    let child_dialect = |subset: &str| store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() };
    let parent_reference = store::os_io::ArtifactRef { artifact_id: "map-a".into(), dialect: parent_dialect.clone() };
    let parent = committed_recovery_hash_store("map-a", parent_dialect, None).await;
    let drawing = committed_recovery_hash_store("gismap-drawing", child_dialect("drawing"), Some(store::OwnerRef { parent: parent_reference.clone(), slot: "drawing".into(), child_id: "gismap-drawing".into() })).await;
    let value = committed_recovery_hash_store("gismap-value", child_dialect("value"), Some(store::OwnerRef { parent: parent_reference, slot: "value".into(), child_id: "gismap-value".into() })).await;
    let parent_before = (parent.snapshot_pack().await.unwrap(), store::SpaceMember::artifact_ref(&parent), store::SpaceMember::owner_ref(&parent));
    let drawing_before = (drawing.snapshot_pack().await.unwrap(), store::SpaceMember::artifact_ref(&drawing), store::SpaceMember::owner_ref(&drawing));
    let value_before = (value.snapshot_pack().await.unwrap(), store::SpaceMember::artifact_ref(&value), store::SpaceMember::owner_ref(&value));
    let admission = store::durable_group::DurableOwnedMapRecoveryAdmissionV1::new(parent, drawing, value);
    let mut recovery = witness.into_store_owned_recovery(admission);
    assert_eq!(recovery.advance(journal_grant()), ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault(store::durable_group::DurableOwnedGroupDecisionError::InvalidFrontier));
    let mut terminal = recovery.take_rejected_terminal().expect("pre-mutation rejection consumes the witness and returns all three exact Store owners");
    assert!(recovery.terminal_is_empty());
    assert_eq!(terminal.document, document);
    assert_eq!(terminal.receipt.transaction_id, 1);
    assert_eq!(terminal.receipt.anchor_sha256, anchor_sha256);
    assert_eq!(terminal.receipt.decision_sha256, decision_sha256);
    assert_eq!(terminal.error, store::durable_group::DurableOwnedGroupDecisionError::InvalidFrontier);
    assert_eq!([terminal.owners.parent.generation(), terminal.owners.drawing.generation(), terminal.owners.value.generation()], [0, 0, 0]);
    let parent_after = terminal.owners.parent.snapshot_pack().await.unwrap();
    let drawing_after = terminal.owners.drawing.snapshot_pack().await.unwrap();
    let value_after = terminal.owners.value.snapshot_pack().await.unwrap();
    assert_eq!((parent_after.pack, parent_after.spr, store::SpaceMember::artifact_ref(&terminal.owners.parent), store::SpaceMember::owner_ref(&terminal.owners.parent)), (parent_before.0.pack, parent_before.0.spr, parent_before.1, parent_before.2));
    assert_eq!(
        (drawing_after.pack, drawing_after.spr, store::SpaceMember::artifact_ref(&terminal.owners.drawing), store::SpaceMember::owner_ref(&terminal.owners.drawing)),
        (drawing_before.0.pack, drawing_before.0.spr, drawing_before.1, drawing_before.2)
    );
    assert_eq!((value_after.pack, value_after.spr, store::SpaceMember::artifact_ref(&terminal.owners.value), store::SpaceMember::owner_ref(&terminal.owners.value)), (value_before.0.pack, value_before.0.spr, value_before.1, value_before.2));
    close_committed_recovery_hash_store(&mut terminal.owners.value);
    close_committed_recovery_hash_store(&mut terminal.owners.drawing);
    close_committed_recovery_hash_store(&mut terminal.owners.parent);
    eprintln!("[DEBUG] a sole committed WAL Event witness denied a foreign Store frontier before mutation and returned all exact owners without exposing the witness");
}

#[semio_framework_async_macros::async_test]
async fn document_authority_durable_group_journal_cancellation_before_handoff_is_absent() {
    let (authority, _storage, pool) = journal_authority().await;
    let record = store::durable_group::durable_owned_group_journal_test_record();
    let mut sink = authority.durable_group_journal_sink(9);
    let mut commit = sink.begin_commit(record.canonical_pack().to_vec(), record.decision_sha256().to_string());
    commit.cancel();
    assert_eq!(commit.advance(journal_grant()).unwrap(), store::durable_group::DurableOwnedGroupJournalAdvanceV1::Absent);
    close_journal_commit(commit.as_mut());
    drop(commit);
    drop(sink);
    shutdown_journal_authority(&authority, &pool).await;
    eprintln!("[DEBUG] cancellation before typed mailbox handoff proved absence and closed the retained journal owner");
}

#[semio_framework_async_macros::async_test]
async fn document_authority_durable_group_journal_rejects_hash_before_mailbox() {
    let (authority, _storage, pool) = journal_authority().await;
    let record = store::durable_group::durable_owned_group_journal_test_record();
    let mut sink = authority.durable_group_journal_sink(11);
    let mut commit = sink.begin_commit(record.canonical_pack().to_vec(), "0".repeat(64));
    assert!(matches!(commit.advance(journal_grant()).unwrap(), store::durable_group::DurableOwnedGroupJournalAdvanceV1::Rejected(_)));
    close_journal_commit(commit.as_mut());
    drop(commit);
    drop(sink);
    shutdown_journal_authority(&authority, &pool).await;
    eprintln!("[DEBUG] Store canonical hash admission rejected a forged decision before typed mailbox or WAL handoff");
}

#[semio_framework_async_macros::async_test]
async fn document_authority_submits_and_queries_over_finite_pool_turns() {
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 3)));
    let storage = storage().await;
    let document = document_id().await;
    let authority = ArtifactAuthority::spawn(pool.clone(), move || ArtifactEngine::create_retained(document, storage, ArtifactEngineConfig::default(), 0), MailboxCapacities::uniform(16)).await.unwrap();

    let batch = CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("name", serde_json::json!("hi"))]).await]).await.unwrap();
    let receipt = authority.submit(batch, SubmitOptions::default(), 0).await.unwrap();
    assert_eq!(receipt.frontier.head_seq, 1);

    let queried: serde_json::Value = stored_json(authority.query("name").await.unwrap().unwrap()).await;
    assert_eq!(queried, serde_json::json!("hi"));

    let frontier = authority.frontier().await.unwrap();
    assert_eq!(frontier.head_seq, 1);

    let generation = authority.snapshot_now(1).await.unwrap();
    assert_eq!(generation, 0);

    while !authority.shutdown_step() {
        semio_framework_async::yield_once().await;
    }
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn document_authority_spawn_propagates_a_build_failure_synchronously() {
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 3)));
    let result = ArtifactAuthority::spawn(
        pool.clone(),
        || async { Err::<ArtifactEngine<AllowAll, NullVersionGraph>, ArtifactEngineOpenRejected>(ArtifactEngineOpenRejected::BeforeWal(DbError::InvalidArgument("boom".to_string()))) },
        MailboxCapacities::uniform(4),
    );
    let rejected = match result.await {
        Err(rejected) => rejected,
        Ok(_) => panic!("artifact authority admitted a rejected builder"),
    };
    assert!(matches!(rejected_engine_open_error(rejected).await, DbError::InvalidArgument(_)));
    pool.shutdown();
}

#[test]
fn artifact_history_backend_token_crc_fault_retire_1024_pages_one_grant_each() {
    let mut reservation = HistoryReplayReservation::try_new().unwrap();
    for _ in 0..HISTORY_REPLAY_SEGMENT_PAGES {
        reservation.retain_source_page(vec![0]).unwrap();
    }
    let mut cursor = HistoryReplayReservationCloseCursor::new(reservation);
    for remaining in (0..HISTORY_REPLAY_SEGMENT_PAGES as usize).rev() {
        assert!(cursor.close_step());
        assert_eq!(cursor.source_page_count, remaining);
    }
    assert!(cursor.close_step(), "fault retirement continues with one result page/range/scalar owner");
    for _ in 0..HISTORY_REPLAY_RESULT_PAGES + 8 {
        if cursor.terminal_is_empty() {
            break;
        }
        assert!(cursor.close_step());
    }
    assert!(cursor.terminal_is_empty());
    let replay = include_str!("../../🦀️.rs");
    for fault in ["history backend page ownership", "history frame CRC mismatch", "history envelope has trailing bytes"] {
        assert!(replay.contains(fault), "missing retained fault source {fault}");
    }
    assert!(replay.contains("HistoryReplayTransition::FaultRetire"));
}

#[test]
fn artifact_history_scratch_result_boundary_plus_one_preserves_exact_owner() {
    let reservation = HistoryReplayReservation::try_new().unwrap();
    let first_result_owner = reservation.result_pages[0].as_ref().unwrap().as_ptr();
    assert_eq!(reservation.source_pages.len(), HISTORY_REPLAY_SEGMENT_PAGES as usize);
    assert_eq!(reservation.result_pages.len(), HISTORY_REPLAY_RESULT_PAGES);
    assert_eq!(reservation.operation_ids.capacity(), HISTORY_REPLAY_MAX_OPERATION_IDS);
    assert_eq!(reservation.entries.capacity(), HISTORY_REPLAY_MAX_ENTRIES);
    assert_eq!(reservation.scratch.as_ref().unwrap().len(), HISTORY_REPLAY_MAX_FIELD_BYTES);
    assert_eq!(reservation.result_pages[0].as_ref().unwrap().as_ptr(), first_result_owner);
    assert_eq!(reservation.preflight_result_range(HISTORY_REPLAY_RESULT_BYTES - 1, 1).unwrap(), HISTORY_REPLAY_RESULT_BYTES);
    assert!(matches!(reservation.preflight_result_range(HISTORY_REPLAY_RESULT_BYTES, 1), Err(DbError::LimitExceeded("history result byte credit"))));
    assert_eq!(reservation.result_pages[0].as_ref().unwrap().as_ptr(), first_result_owner, "cap+1 rejection must return the exact preadmitted page owner");
    let mut cursor = HistoryReplayReservationCloseCursor::new(reservation);
    for _ in 0..HISTORY_REPLAY_RESULT_PAGES + 8 {
        if cursor.terminal_is_empty() {
            break;
        }
        assert!(cursor.close_step());
    }
    assert!(cursor.terminal_is_empty());
}

#[test]
fn artifact_history_reservation_construction_fault_cap_plus_one_and_each_page_retire_one_owner() {
    let _guard = history_construction_test_lock();
    for failure_after in [0, 1, HISTORY_REPLAY_RESULT_PAGES / 2, HISTORY_REPLAY_RESULT_PAGES - 1, HISTORY_REPLAY_RESULT_PAGES] {
        let mut fault = HistoryReplayReservation::try_new_with_result_page_failure(failure_after).unwrap_err();
        let error = fault.take_error().expect("exact construction error");
        assert!(matches!(error, DbError::Unavailable(_)));
        let retained_pages = fault.retained_result_page_count();
        assert_eq!(retained_pages, failure_after, "failure must retain every page allocated before its exact boundary");
        let mut previous_pages = retained_pages;
        while !fault.terminal_is_empty() {
            assert!(fault.close_step());
            let current_pages = fault.retained_result_page_count();
            assert!(previous_pages.saturating_sub(current_pages) <= 1, "one construction-fault grant may retire at most one result page");
            previous_pages = current_pages;
        }
    }

    let reservation = HistoryReplayReservation::try_new_with_result_page_failure(HISTORY_REPLAY_RESULT_PAGES + 1).expect("a failure boundary beyond the fixed page cap cannot fabricate an extra owner");
    assert_eq!(reservation.result_pages.len(), HISTORY_REPLAY_RESULT_PAGES);
    let mut cursor = HistoryReplayReservationCloseCursor::new(reservation);
    while !cursor.terminal_is_empty() {
        assert!(cursor.close_step());
    }

    for page_count in 0..=HISTORY_REPLAY_RESULT_PAGES {
        let result_pages = (0..page_count).map(|_| Some(Vec::new())).collect();
        let token = claim_history_replay_reservation_construction().expect("fixed construction slot");
        let close = HistoryReplayReservationCloseCursor {
            source_pages: Some(Vec::new()),
            source_page_count: 0,
            result_pages: Some(result_pages),
            operation_ids: Some(Vec::new()),
            entries: Some(Vec::new()),
            scratch: None,
            retained_operation_bytes: 0,
            retained_result_bytes: 0,
            started: true,
        };
        {
            let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let slot = &mut registry.slots[token.slot];
            slot.cursor = Some(close);
        }
        let mut fault = HistoryReplayReservationConstructionFault { token: Some(token), unregistered_error: None };
        let mut retired_pages = 0;
        while fault.retained_result_page_count() != 0 {
            let before = fault.retained_result_page_count();
            assert!(fault.close_step());
            let after = fault.retained_result_page_count();
            assert_eq!(before - after, 1, "failure after page {page_count} must retire exactly one allocated page per grant");
            retired_pages += 1;
        }
        assert_eq!(retired_pages, page_count);
        while !fault.terminal_is_empty() {
            assert!(fault.close_step());
        }
    }
}

#[test]
fn artifact_history_unchecked_construction_error_and_checked_out_drop_hand_back_exact_pages() {
    let _guard = history_construction_test_lock();
    let generation = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).next_generation;
    let dropped = std::panic::catch_unwind(|| {
        let _ = HistoryReplayReservation::try_new_with_result_page_failure(3);
    });
    assert!(dropped.is_ok(), "unchecked construction error Drop must not panic");
    let fault = take_history_replay_reservation_construction_fault(generation).expect("unchecked error registered exact partial owner");
    assert_eq!(fault.retained_result_page_count(), 3);
    drop(fault);
    let mut resumed = take_history_replay_reservation_construction_fault(generation).expect("checked-out Drop returned exact partial owner");
    assert!(matches!(resumed.take_error(), Some(DbError::Unavailable(_))));
    let mut previous_pages = 3usize;
    while !resumed.terminal_is_empty() {
        assert!(resumed.close_step());
        let pages = resumed.retained_result_page_count();
        assert!(previous_pages.saturating_sub(pages) <= 1);
        previous_pages = pages;
    }
    assert!(take_history_replay_reservation_construction_fault(generation).is_none());
}

#[test]
fn artifact_history_construction_unwind_hands_partial_owner_to_registry_without_bulk_drop() {
    let _guard = history_construction_test_lock();
    let generation = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).next_generation;
    let unwind = std::panic::catch_unwind(|| {
        let mut builder = HistoryReplayReservationConstructionBuilder::new().expect("fixed construction authority");
        let mut page = Vec::new();
        page.try_reserve_exact(HISTORY_REPLAY_PAGE_BYTES as usize).expect("fixture page");
        page.resize(HISTORY_REPLAY_PAGE_BYTES as usize, 0);
        assert!(
            builder
                .edit_cursor(|cursor| cursor.result_pages.as_mut().is_some_and(|owners| {
                    owners.push(Some(page));
                    true
                }))
                .unwrap_or(false)
        );
        panic!("injected construction unwind");
    });
    assert!(unwind.is_err());
    let mut fault = take_history_replay_reservation_construction_fault(generation).expect("builder Drop registered partial owner");
    assert_eq!(fault.retained_result_page_count(), 1);
    assert!(matches!(fault.take_error(), Some(DbError::Unavailable(_))));
    let before = fault.retained_result_page_count();
    assert!(fault.close_step());
    let after = fault.retained_result_page_count();
    assert_eq!(before - after, 1);
    while !fault.terminal_is_empty() {
        assert!(fault.close_step());
    }
}

#[test]
fn artifact_history_construction_registry_saturation_rejects_before_partial_owner_and_reuses_with_fresh_generation() {
    let _guard = history_construction_test_lock();
    let mut faults = Vec::with_capacity(HISTORY_REPLAY_CONSTRUCTION_SLOTS);
    for _ in 0..HISTORY_REPLAY_CONSTRUCTION_SLOTS {
        faults.push(HistoryReplayReservation::try_new_with_result_page_failure(0).unwrap_err());
    }
    let mut rejected = HistoryReplayReservation::try_new_with_result_page_failure(0).unwrap_err();
    assert!(rejected.generation().is_none());
    assert!(matches!(rejected.take_error(), Some(DbError::Unavailable(_))));
    let released_generation = faults[0].generation().expect("registered generation");
    let mut released = faults.remove(0);
    while !released.terminal_is_empty() {
        assert!(released.close_step());
    }
    let replacement = HistoryReplayReservation::try_new_with_result_page_failure(0).unwrap_err();
    assert_ne!(replacement.generation(), Some(released_generation));
    faults.push(replacement);
    for mut fault in faults {
        while !fault.terminal_is_empty() {
            assert!(fault.close_step());
        }
    }
}

#[test]
fn artifact_history_construction_handback_rejects_stale_duplicate_and_aba_without_owner_overwrite() {
    let _guard = history_construction_test_lock();
    let mut first = HistoryReplayReservation::try_new_with_result_page_failure(1).unwrap_err();
    let first_token = first.token.as_ref().map(|token| (token.slot, token.generation)).expect("first linear construction token");
    while !first.terminal_is_empty() {
        assert!(first.close_step());
    }

    let replacement = HistoryReplayReservation::try_new_with_result_page_failure(1).unwrap_err();
    let replacement_token = replacement.token.as_ref().map(|token| (token.slot, token.generation)).expect("replacement linear construction token");
    assert_eq!(replacement_token.0, first_token.0);
    assert_ne!(replacement_token.1, first_token.1);
    let page_pointer = replacement.retained_result_page_pointer(0).expect("replacement page owner");
    let error_pointer = replacement.retained_error_pointer().expect("replacement error owner");

    let stale = HistoryReplayReservationConstructionToken { slot: first_token.0, generation: first_token.1 };
    assert_eq!(handback_history_replay_reservation_construction(&stale), Err(HistoryReplayReservationConstructionHandbackRejection { slot: first_token.0, generation: first_token.1 }));
    assert_eq!(replacement.retained_result_page_pointer(0), Some(page_pointer));
    assert_eq!(replacement.retained_error_pointer(), Some(error_pointer));

    let out_of_bounds = HistoryReplayReservationConstructionToken { slot: HISTORY_REPLAY_CONSTRUCTION_SLOTS, generation: replacement_token.1 };
    assert_eq!(handback_history_replay_reservation_construction(&out_of_bounds), Err(HistoryReplayReservationConstructionHandbackRejection { slot: HISTORY_REPLAY_CONSTRUCTION_SLOTS, generation: replacement_token.1 }));
    assert_eq!(replacement.retained_result_page_pointer(0), Some(page_pointer));
    assert_eq!(replacement.retained_error_pointer(), Some(error_pointer));

    drop(replacement);
    let duplicate = HistoryReplayReservationConstructionToken { slot: replacement_token.0, generation: replacement_token.1 };
    assert_eq!(handback_history_replay_reservation_construction(&duplicate), Err(HistoryReplayReservationConstructionHandbackRejection { slot: replacement_token.0, generation: replacement_token.1 }));
    let mut resumed = take_history_replay_reservation_construction_fault(replacement_token.1).expect("current generation remained resumable");
    assert_eq!(resumed.retained_result_page_pointer(0), Some(page_pointer));
    assert_eq!(resumed.retained_error_pointer(), Some(error_pointer));
    while !resumed.terminal_is_empty() {
        assert!(resumed.close_step());
    }
}

#[test]
fn artifact_history_fixed_owner_accounting_has_no_capacity_scan() {
    let source = include_str!("../../🦀️.rs");
    let replay = &source[source.find("//#region 🔖️HistoryReplay").unwrap()..source.find("//#endregion 🔖️HistoryReplay").unwrap()];
    for forbidden in [".rposition(", "result_pages.iter()", "source_pages.iter().all", "pages.iter().all", "pages.iter().filter"] {
        assert!(!replay.contains(forbidden), "retained history accounting scanned fixed capacity through {forbidden}");
    }
    for retained in ["source_page_count", "retained_operation_bytes", "retained_result_bytes"] {
        assert!(replay.contains(retained), "retained history accounting omitted {retained}");
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_history_panic_at_each_phase_transition_retains_then_fault_retires() {
    let mut engine = ArtifactEngine::create_retained(document_id().await, storage().await, ArtifactEngineConfig::default(), 0).await.unwrap();
    let phases = Vec::from([
        HistoryReplayPhase::Probe,
        HistoryReplayPhase::SegmentLen { index: 0, future: Box::pin(async { Err(DbError::NotFound("phase fixture".to_string())) }) },
        HistoryReplayPhase::PageStart { index: 0, len: 1, offset: 0 },
        HistoryReplayPhase::PageRead { index: 0, len: 1, offset: 0, requested: 1, future: Box::pin(async { Ok(vec![0]) }) },
        HistoryReplayPhase::Inventory { future: Box::pin(async { Ok(db_storage::DbIoU64List::new()) }) },
        HistoryReplayPhase::Verify { index: 0 },
        HistoryReplayPhase::Frame { index: 0 },
        HistoryReplayPhase::CommittedBody { index: 0 },
        HistoryReplayPhase::Envelope { index: 0, cursor: HistoryEnvelopeCursor { pos: 0, end: 0, field: HistoryEnvelopeField::MutationId, dependencies: 0, mutation_id: None } },
        HistoryReplayPhase::CopyMutation { index: 0, range: 0..0, copied: 0, result_start: 0 },
        HistoryReplayPhase::Frontier { index: 0, cursor: HistoryFrontierCursor { pos: 0, end: 0, field: HistoryFrontierField::Document, head_seq: 0, commit_seq: 0, chain_hash: [0; 32] } },
        HistoryReplayPhase::Publish { index: 0, head_seq: 0, commit_seq: 0, chain_hash: [0; 32], epoch: 0 },
        HistoryReplayPhase::Retire,
        HistoryReplayPhase::InventoryClose,
        HistoryReplayPhase::FinalizeSuccess,
    ]);
    let waker = std::task::Waker::from(StdArc::new(HistoryReplayTestWake));
    let mut context = std::task::Context::from_waker(&waker);
    for phase in phases {
        let mut replay = engine.history_replay(1, StdArc::new(std::sync::atomic::AtomicBool::new(false)), HistoryReplayReservation::try_new().unwrap());
        replay.phase = Some(phase);
        replay.transition = HistoryReplayTransition::InProgress;
        replay.panic_before_transition_commit = true;
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| Pin::new(&mut replay).poll(&mut context))).is_err());
        assert!(replay.phase.is_some(), "panic must retain the exact active phase owner");
        assert!(matches!(replay.transition, HistoryReplayTransition::InProgress));
        replay.request_close(DbError::Internal("phase panic fixture".to_string()));
        let mut terminal = false;
        for _ in 0..50_000 {
            if Pin::new(&mut replay).poll(&mut context).is_ready() {
                terminal = true;
                break;
            }
        }
        assert!(terminal);
        assert!(replay.terminal_is_empty());
    }
    while engine.wal.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    while engine.state.values.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
}
//#endregion 🔖️Actor
