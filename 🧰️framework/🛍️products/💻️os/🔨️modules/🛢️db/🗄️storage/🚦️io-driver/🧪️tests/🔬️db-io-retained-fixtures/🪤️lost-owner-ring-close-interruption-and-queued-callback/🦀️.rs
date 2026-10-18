/// 🪤️ Lost owner ring, close interruption and queued callback: the lost owner ring, the interrupted storage close, the one-owner-per-grant close and the queued callback slot reuse.

#[test]
fn db_io_lost_owner_fixed_ring_max_plus_one_returns_the_exact_candidate() {
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_lost_owner_fixed_ring_max_plus_one_returns_the_exact_candidate") {
        return;
    }
    let _owner = fixture_owner();
    while db_io_lost_owner_close_step().unwrap() {}
    DB_IO_RETIREMENT_PRESSURE_FAULT.store(false, std::sync::atomic::Ordering::Release);
    for _ in 0..DB_IO_LOST_OWNER_SLOTS {
        let owner = DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "retained-ring-owner"));
        assert!(db_io_try_park_lost_owner(owner).is_ok());
    }
    {
        let mut overflow = DB_IO_LOST_OWNER_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in overflow.iter_mut() {
            *slot = Some(DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "retained-overflow-owner")));
        }
    }
    let owner = DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "exact-plus-one-candidate"));
    assert!(db_io_park_lost_owner(owner).is_ok());
    let second = DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "exact-plus-two-candidate"));
    assert!(db_io_park_lost_owner(second).is_ok());
    assert!(DB_IO_RETIREMENT_PRESSURE_FAULT.load(std::sync::atomic::Ordering::Acquire));
    {
        let quarantine = DB_IO_LOST_OWNER_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let exact = quarantine.iter().flatten().find_map(|owner| match owner {
            DbIoLostOwner::Fault(candidate) => Some(candidate.detail.as_str()),
            _ => None,
        });
        assert_eq!(exact, Some("exact-plus-one-candidate"));
        assert!(quarantine.iter().flatten().any(|owner| matches!(owner, DbIoLostOwner::Fault(candidate) if candidate.detail.as_str() == "exact-plus-two-candidate")));
    }
    for _ in 0..DB_IO_LOST_OWNER_SLOTS {
        assert!(db_io_lost_owner_close_step().unwrap());
    }
    for _ in 0..DB_IO_LOST_OWNER_OVERFLOW_SLOTS * 2 {
        assert!(db_io_lost_owner_close_step().unwrap());
    }
    assert!(db_io_lost_owner_close_step().unwrap());
    {
        let owners = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(owners.iter().flatten().any(|owner| matches!(owner, DbIoLostOwner::Fault(candidate) if candidate.detail.as_str() == "exact-plus-one-candidate")));
    }
    assert!(db_io_lost_owner_close_step().unwrap());
    assert!(db_io_lost_owner_close_step().unwrap());
    {
        let owners = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(owners.iter().flatten().any(|owner| matches!(owner, DbIoLostOwner::Fault(candidate) if candidate.detail.as_str() == "exact-plus-two-candidate")));
    }
    assert!(db_io_lost_owner_close_step().unwrap());
    assert!(!db_io_lost_owner_close_step().unwrap());

    for _ in 0..DB_IO_LOST_OWNER_SLOTS {
        assert!(db_io_try_park_lost_owner(DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "full-primary-owner"))).is_ok());
    }
    {
        let mut overflow = DB_IO_LOST_OWNER_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in overflow.iter_mut() {
            *slot = Some(DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "full-overflow-owner")));
        }
    }
    {
        let mut quarantine = DB_IO_LOST_OWNER_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in quarantine.iter_mut() {
            *slot = Some(DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "full-quarantine-owner")));
        }
    }
    let refused = match db_io_park_lost_owner(DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "exact-all-tier-refusal"))) {
        Err(owner) => owner,
        Ok(()) => panic!("all-tier retirement saturation accepted an unreserved owner"),
    };
    assert!(matches!(&refused, DbIoLostOwner::Fault(candidate) if candidate.detail.as_str() == "exact-all-tier-refusal"));
    assert!(db_io_lost_owner_close_step().unwrap());
    assert!(db_io_park_lost_owner(refused).is_ok());
    while db_io_lost_owner_close_step().unwrap() {}
}

#[semio_framework_async_macros::async_test]
async fn db_io_storage_ready_and_pending_close_interruption_recover_the_same_owner_and_ledger() {
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_storage_ready_and_pending_close_interruption_recover_the_same_owner_and_ledger") {
        return;
    }
    let _owner = fixture_owner();
    let before = ledger_witness();
    let source = pages(&[0x61; DB_IO_PAGE_BYTES + 1]);
    let platform = db_io_prepare_platform(&source).unwrap().await.unwrap();
    let mut close = db_io_close_platform(platform);
    let waker = std::task::Waker::noop();
    let context = &mut std::task::Context::from_waker(waker);
    assert!(matches!(Pin::new(&mut close).poll(context), std::task::Poll::Pending));
    drop(close);
    assert!(db_io_platform_maintenance_step().unwrap());
    while db_io_lost_owner_close_step().unwrap() {}
    drain_pages(source);

    let empty_source = pages(&[]);
    let empty_platform = db_io_prepare_platform(&empty_source).unwrap().await.unwrap();
    let mut ready_close = db_io_close_platform(empty_platform);
    assert!(matches!(Pin::new(&mut ready_close).poll(context), std::task::Poll::Pending));
    assert!(matches!(Pin::new(&mut ready_close).poll(context), std::task::Poll::Pending));
    assert!(matches!(Pin::new(&mut ready_close).poll(context), std::task::Poll::Ready(Ok(()))));
    drop(ready_close);
    drain_pages(empty_source);

    let operation = db_io_operation_reserve(DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();
    let reservation = DbIoDriverReservation::try_reserve(operation, DB_IO_PAGE_BYTES).unwrap();
    let mut writer = DbIoPageWriter::try_reserve(1).unwrap();
    let mut copy = db_io_write_observed_bytes(reservation, vec![0x62; DB_IO_PAGE_BYTES], &mut writer);
    assert!(matches!(Pin::new(&mut copy).poll(context), std::task::Poll::Pending));
    assert!(matches!(Pin::new(&mut copy).poll(context), std::task::Poll::Pending));
    drop(copy);
    while db_io_lost_owner_close_step().unwrap() {}
    while writer.close_step().unwrap().is_some() {}
    db_io_operation_return(operation, DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();

    let fault_operation = db_io_operation_reserve(DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();
    let reservation = DbIoDriverReservation::try_reserve(fault_operation, 1).unwrap();
    let mut writer = DbIoPageWriter::try_reserve(1).unwrap();
    let mut fault = db_io_write_observed_bytes(reservation, Vec::with_capacity(DB_IO_PAGE_BYTES), &mut writer);
    assert!(matches!(Pin::new(&mut fault).poll(context), std::task::Poll::Ready(Err(DbError::LimitExceeded("DB I/O external driver allocation capacity")))));
    drop(fault);
    while db_io_lost_owner_close_step().unwrap() {}
    while writer.close_step().unwrap().is_some() {}
    db_io_operation_return(fault_operation, DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();
    assert_eq!(ledger_witness(), before);
}

#[test]
fn db_io_interrupted_close_retires_one_page_or_owner_per_grant() {
    let _owner = fixture_owner();
    let input = pages(&[0x66; DB_IO_PAGE_BYTES + 1]);
    let backend = DbIoBackendControl::Memory { slot: 0, generation: 1 };
    let document = DbIoText::try_from_str("close-fixture").unwrap();
    let mut writers = crate::WalWriterTable::new(backend);
    let permit = writers.acquire(&document, ()).unwrap();
    let mut task = DbIoTask::WalAppend { backend, document: document.clone(), writer: permit.key(), index: 0, input };
    assert_eq!(task.close_step().unwrap(), Some(DB_IO_PAGE_BYTES));
    assert!(!task.terminal_is_empty());
    assert_eq!(task.close_step().unwrap(), Some(DB_IO_PAGE_BYTES));
    assert!(!task.terminal_is_empty());
    assert_eq!(task.close_step().unwrap(), Some(0));
    assert!(!task.terminal_is_empty());
    assert_eq!(task.close_step().unwrap(), Some(0));
    assert!(task.terminal_is_empty());
    assert_eq!(task.close_step().unwrap(), None);
    assert!(!writers.release_step(permit.key(), backend, &document).unwrap());
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn db_io_real_queued_callback_rejects_a_reused_task_slot_aba() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let control = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone()).unwrap();
    let (started_tx, started_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    admit_fixture_job(
        &pool,
        Lane::Io,
        Box::new(move || {
            let _ = started_tx.send(());
            let _ = release_rx.recv();
        }),
    );
    started_rx.recv_timeout(std::time::Duration::from_secs(5)).unwrap();

    let task = DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://aba-old").unwrap() };
    let old = db_io_allocate_task(task).unwrap_or_else(|(error, _)| panic!("{error}"));
    let (callback_tx, callback_rx) = std::sync::mpsc::channel();
    admit_fixture_job(
        &pool,
        Lane::Io,
        Box::new(move || {
            db_io_drive_one(old);
            let _ = callback_tx.send(());
        }),
    );
    db_io_drive_one(old);
    let terminal = match (DbIoTaskOperation { handle: old, resolved: false }).await {
        Ok(lease) => match lease.into_result() {
            Ok(result) => result,
            Err(error) => panic!("old ABA result handback failed: {error}"),
        },
        Err(_) => panic!("old ABA task faulted"),
    };
    assert!(matches!(terminal, DbIoResult::Unit));
    drain_control_tasks(control).await;

    let reused = loop {
        let task = DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://aba-reused").unwrap() };
        let handle = db_io_allocate_task(task).unwrap_or_else(|(error, _)| panic!("{error}"));
        if handle.slot == old.slot {
            break handle;
        }
        db_io_drive_one(handle);
        let terminal = match (DbIoTaskOperation { handle, resolved: false }).await {
            Ok(lease) => match lease.into_result() {
                Ok(result) => result,
                Err(error) => panic!("ABA cycle result handback failed: {error}"),
            },
            Err(_) => panic!("ABA cycle task faulted"),
        };
        assert!(matches!(terminal, DbIoResult::Unit));
        drain_control_tasks(control).await;
    };
    let before_callback = {
        let owner = DB_IO_TASK_SLOTS[reused.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        (owner.generation, owner.operation, owner.phase)
    };
    release_tx.send(()).unwrap();
    callback_rx.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
    let after_callback = {
        let owner = DB_IO_TASK_SLOTS[reused.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        (owner.generation, owner.operation, owner.phase)
    };
    assert_eq!(before_callback, after_callback);
    assert!(db_io_slot_matches(&DB_IO_TASK_SLOTS[reused.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner), reused));
    db_io_drive_one(reused);
    let terminal = match (DbIoTaskOperation { handle: reused, resolved: false }).await {
        Ok(lease) => match lease.into_result() {
            Ok(result) => result,
            Err(error) => panic!("reused ABA result handback failed: {error}"),
        },
        Err(_) => panic!("reused ABA task faulted"),
    };
    assert!(matches!(terminal, DbIoResult::Unit));
    drain_control_tasks(control).await;
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    pool.shutdown();
    loop {
        match db_io_maintenance_step() {
            Ok(true) => {}
            Ok(false) => break,
            Err(error) => panic!("ABA maintenance failed: {error}"),
        }
    }
    assert_eq!(ledger_witness(), before);
}
