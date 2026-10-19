/// 🧯️ Pressure slots and lost backend laws: the rejected backend and lost owner pressure slots, the lost backend and page handle owners and the typed lane lease.

struct RejectedBackendPressureLawSlots;

impl RejectedBackendPressureLawSlots {
    fn reserve() -> Self {
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(registry.slots.iter().all(|slot| slot.generation == 0));
        assert_ne!(registry.next_generation, u64::MAX);
        for slot in &mut registry.slots {
            slot.generation = u64::MAX;
            slot.reserved = true;
        }
        Self
    }
}

impl Drop for RejectedBackendPressureLawSlots {
    fn drop(&mut self) {
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in &mut registry.slots {
            if slot.generation == u64::MAX && slot.reserved && slot.executor.is_none() && slot.pool.is_none() {
                *slot = DbIoRejectedBackendSlot::empty();
            }
        }
    }
}

struct LostOwnerPressureLawSlots;

fn retire_lost_owner_pressure_slots<const N: usize>(slots: &std::sync::Mutex<[Option<DbIoLostOwner>; N]>) {
    let mut slots = slots.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    for slot in slots.iter_mut() {
        let Some(DbIoLostOwner::Fault(owner)) = slot.as_mut() else { panic!("lost-owner pressure row changed its exact fault sentinel") };
        while owner.close_step() {}
        *slot = None;
    }
}

impl LostOwnerPressureLawSlots {
    fn reserve() -> Self {
        while db_io_lost_owner_close_step().unwrap() {}
        for slot in DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter_mut() {
            *slot = Some(DbIoLostOwner::Fault(db_io_literal_fault(db_io_fault_custody_reserve().expect("fixture prebirth custody"), DbIoFaultKind::Backend, DbIoFaultCause::Internal, "registration-primary-sentinel")));
        }
        for slot in DB_IO_LOST_OWNER_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter_mut() {
            *slot = Some(DbIoLostOwner::Fault(db_io_literal_fault(db_io_fault_custody_reserve().expect("fixture prebirth custody"), DbIoFaultKind::Backend, DbIoFaultCause::Internal, "registration-overflow-sentinel")));
        }
        for slot in DB_IO_LOST_OWNER_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter_mut() {
            *slot = Some(DbIoLostOwner::Fault(db_io_literal_fault(db_io_fault_custody_reserve().expect("fixture prebirth custody"), DbIoFaultKind::Backend, DbIoFaultCause::Internal, "registration-quarantine-sentinel")));
        }
        Self
    }
}

impl Drop for LostOwnerPressureLawSlots {
    fn drop(&mut self) {
        retire_lost_owner_pressure_slots(&DB_IO_LOST_OWNERS);
        retire_lost_owner_pressure_slots(&DB_IO_LOST_OWNER_OVERFLOW);
        retire_lost_owner_pressure_slots(&DB_IO_LOST_OWNER_QUARANTINE);
    }
}

#[semio_framework_async_macros::async_test]
async fn db_io_lost_backend_retains_exact_owner_under_rejected_registry_pressure() {
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_lost_backend_retains_exact_owner_under_rejected_registry_pressure") {
        return;
    }
    let _owner = fixture_owner();
    while db_io_lost_owner_close_step().unwrap() {}
    let before = ledger_witness();
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../🔐️writer/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let row = &fixture["backendPressure"];
    assert_eq!(DB_IO_BACKEND_CONTROLS, row["capacity"].as_u64().unwrap() as usize);
    let pressure = DB_IO_RETIREMENT_PRESSURE_FAULT.swap(false, std::sync::atomic::Ordering::AcqRel);
    let sentinels = RejectedBackendPressureLawSlots::reserve();
    let credit = DbIoCredit { pages: 0, bytes: size_of::<BlockingCompleteLawExecutor>() as u64, items: 1, controls: 1 };
    let operation = db_io_backend_owner_reserve(credit).unwrap();
    let pool = db_io_test_pool();
    let pool_use = pool.acquire_use().unwrap();
    assert!(db_io_try_park_lost_owner(DbIoLostOwner::Backend { owner: Some(Box::new(BlockingCompleteLawExecutor { terminal: false })), operation, credit, pool: Some(pool.clone()), pool_use: Some(pool_use.clone()) }).is_ok());
    assert!(db_io_lost_owner_close_step().unwrap());
    assert!(DB_IO_RETIREMENT_PRESSURE_FAULT.load(std::sync::atomic::Ordering::Acquire));
    let retained = {
        let owners = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        matches!(&owners[0], Some(DbIoLostOwner::Backend { owner: Some(owner), operation: actual_operation, credit: actual_credit, pool: Some(actual_pool), pool_use: Some(actual_pool_use) }) if !owner.backend_terminal_is_empty() && *actual_operation == operation && *actual_credit == credit && Arc::ptr_eq(actual_pool, &pool) && Arc::ptr_eq(actual_pool_use, &pool_use))
    };
    assert_eq!(retained, row["retainedWhenFull"].as_bool().unwrap());
    assert_eq!(ledger_witness().0, before.0.checked_add(credit).unwrap());
    drop(sentinels);
    assert!(db_io_lost_owner_close_step().unwrap());
    assert!(!db_io_lost_owner_close_step().unwrap());
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let terminal = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots.iter().all(|slot| slot.generation == 0);
        if terminal {
            assert_eq!(terminal, row["terminalAfterCapacityReturns"].as_bool().unwrap());
            break;
        }
        assert!(std::time::Instant::now() < deadline, "retained rejected backend did not finish after capacity returned");
        db_io_rejected_backend_maintenance_step().unwrap();
        semio_framework_async::yield_once().await;
    }
    assert_eq!(ledger_witness(), before);
    DB_IO_RETIREMENT_PRESSURE_FAULT.store(pressure, std::sync::atomic::Ordering::Release);
}

#[test]
fn db_io_lost_page_handle_resumes_the_same_retirement_cursor() {
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_lost_page_handle_resumes_the_same_retirement_cursor") {
        return;
    }
    let _owner = fixture_owner();
    while db_io_lost_owner_close_step().unwrap() {}
    let before = ledger_witness();
    let owner = pages(&[0x77; DB_IO_PAGE_BYTES + 1]);
    let operation = owner.operation();
    assert_ne!(operation, 0);
    drop(owner);
    assert_eq!(db_io_page_maintenance_step().unwrap(), None, "a dropped page set parks as one lost owner, never as loose page handles");
    let mut opportunities = 0;
    while db_io_lost_owner_close_step().unwrap() {
        opportunities += 1;
        let parked = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().flatten().count();
        assert!(parked <= 1, "the lost page set resumes one retirement cursor");
    }
    assert_eq!(opportunities, 4, "two pages, the shell credit, then the terminal slot release: one owner per opportunity");
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_async_native_lost_backend_uses_typed_lane_lease_and_mounted_terminal_witness() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let pool = db_io_test_pool();
    let control = register_db_io_backend(DbIoBackendKind::Postgres, Box::new(AsyncNativeLawExecutor { terminal: false }), pool.clone()).unwrap();
    let mut operation = submit_db_io_task(DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://async-native").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
    let lease = operation.take_async_native().await.unwrap();
    lease.complete(Ok(DbIoResult::Unit)).unwrap();
    let terminal = match operation.await {
        Ok(terminal) => terminal,
        Err(_) => panic!("async-native law task faulted"),
    };
    assert!(matches!(terminal.into_result().unwrap(), DbIoResult::Unit));
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_cancellation_before_during_and_receiver_drop_retain_exact_terminal_owners() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let pool = db_io_test_pool();
    let control = register_db_io_backend(DbIoBackendKind::Neo4j, Box::new(AsyncNativeLawExecutor { terminal: false }), pool.clone()).unwrap();

    let mut before_execution = submit_db_io_task(DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://cancel-before").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
    before_execution.cancel().unwrap();
    let mut fault = match before_execution.await {
        Err(fault) => fault,
        Ok(_) => panic!("cancel-before fixture returned a result"),
    };
    assert_eq!(fault.kind, DbIoFaultKind::Cancelled);
    assert_eq!(fault.cause, DbIoFaultCause::Closed);
    while fault.close_step() {}
    drain_control_tasks(control).await;

    let mut during_execution = submit_db_io_task(DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://cancel-during").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
    let lease = during_execution.take_async_native().await.unwrap();
    during_execution.cancel().unwrap();
    db_io_maintenance_step().unwrap();
    {
        let owner = DB_IO_TASK_SLOTS[during_execution.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(owner.async_detached);
        assert!(owner.task.is_none());
        assert!(owner.close_enqueued);
        assert!(owner.terminal.is_none());
    }
    let mut output = DbIoPageWriter::try_reserve_for_operation(lease.operation(), 1).unwrap();
    assert_eq!(output.write_fragment(&[0xA5]).unwrap(), 1);
    lease.complete(Ok(DbIoResult::Pages(output.finish().unwrap()))).unwrap();
    let mut fault = loop {
        match during_execution.take().unwrap() {
            Some(Err(fault)) => break fault,
            Some(Ok(_)) => panic!("cancel-during fixture published its retained result"),
            None => {
                db_io_maintenance_step().unwrap();
                semio_framework_async::yield_once().await;
            }
        }
    };
    assert_eq!(fault.kind, DbIoFaultKind::Cancelled);
    assert_eq!(fault.cause, DbIoFaultCause::Closed);
    while fault.close_step() {}
    drain_control_tasks(control).await;

    let mut receiver_drop = submit_db_io_task(DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://receiver-drop").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
    let lease = receiver_drop.take_async_native().await.unwrap();
    drop(receiver_drop);
    drop(lease);
    drain_control_tasks(control).await;
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_panic_backend_fault_and_shutdown_close_reach_exact_prior_witness() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let pool = db_io_test_pool();
    for (panics, expected) in [(false, DbIoFaultKind::Backend), (true, DbIoFaultKind::Panic)] {
        let control = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(BlockingFaultLawExecutor { panic: panics, terminal: false }), pool.clone()).unwrap();
        let operation = submit_db_io_task(DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://hostile-blocking").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
        let mut fault = match operation.await {
            Err(fault) => fault,
            Ok(_) => panic!("hostile blocking fixture returned a result"),
        };
        assert_eq!(fault.kind, expected);
        assert_eq!(fault.cause, DbIoFaultCause::Internal);
        let expected_detail = if panics { "DB I/O backend panicked" } else { "hostile DB I/O fixture backend fault" };
        assert_eq!(fault.detail.as_str(), expected_detail);
        while fault.close_step() {}
        drain_control_tasks(control).await;
        retire_db_io_backend(control).unwrap();
        close_db_io_backend(control).await.unwrap();
    }
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_registered_backend_use_blocks_pool_shutdown_until_terminal_close() {
    let _owner = fixture_owner();
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/🔐️backend-pool-use/🔣️.json")).unwrap();
    assert_eq!(fixture["version"], 1);
    assert_eq!(fixture["cases"].as_array().unwrap().len(), 9);
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let control = register_db_io_backend(DbIoBackendKind::Memory, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone()).unwrap();
    assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(pool.shutdown(), Ok(()));
}
