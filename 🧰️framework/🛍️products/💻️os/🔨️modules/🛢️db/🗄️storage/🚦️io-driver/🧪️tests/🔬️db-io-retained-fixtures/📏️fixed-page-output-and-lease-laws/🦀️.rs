/// 📏️ Fixed page, output and lease laws: the page maximum, the artifact rejection, the async native fault, the output task, the result owners, the process page preflight and the platform ring.

#[test]
fn db_io_fixed_page_max_plus_one_and_zero_are_exact() {
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_fixed_page_max_plus_one_and_zero_are_exact") {
        return;
    }
    let _owner = fixture_owner();
    let empty = pages(&[]);
    assert!(empty.is_empty());
    drain_pages(empty);
    let max = vec![0x5a; DB_IO_PAGE_BYTES * DB_IO_OPERATION_PAGES];
    let retained = pages(&max);
    assert_eq!(retained.len(), max.len());
    assert_eq!(usize::from(retained.page_count()), DB_IO_OPERATION_PAGES);
    assert!(DbIoPageWriter::try_reserve(DB_IO_OPERATION_PAGES + 1).is_err());
    drain_pages(retained);
}

#[test]
fn db_io_artifact_rejection_is_an_internal_executor_boundary_violation() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let fault = db_io_task_fault(DbIoFaultKind::Backend, &DbError::Rejected { policy: protocol::MergePolicy::Normal, worst: semio_framework_diagnostic::Severity::Error, messages: Vec::new() });
    assert_eq!(fault.cause, DbIoFaultCause::Internal);
    assert_eq!(fault.into_db_error(), DbError::Internal("DB I/O backend returned an artifact-layer rejection".to_string()));
    while db_io_maintenance_step().unwrap() {}
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_async_native_fault_preserves_exact_category_scalars_and_retires() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let fixture: PageLifecycleFixture = serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/🧬️page-lifecycle/🔣️.json")).unwrap();
    let pool = db_io_test_pool();
    let control = register_db_io_backend(DbIoBackendKind::Postgres, Box::new(AsyncFaultTaxonomyLawExecutor { terminal: false }), pool.clone()).unwrap();

    for (index, category) in fixture.fault_categories.into_iter().enumerate() {
        let expected_error = fault_fixture_error(category);
        let expected_category = fault_category_oracle(&expected_error);
        let output = DbIoPageWriter::try_reserve(1).unwrap();
        let mut operation = submit_db_io_task(DbIoTask::PayloadGet { backend: control, hash: ContentHash([u8::try_from(index).unwrap(); 32]), output }).unwrap_or_else(|(error, _)| panic!("async fault taxonomy admission failed: {error}"));
        let handle = operation.handle;
        operation.start_async_native_on_lane_io().await.unwrap();
        let fault = match operation.await {
            Err(fault) => fault,
            Ok(_) => panic!("async fault taxonomy fixture returned a result"),
        };
        assert_eq!(fault.kind, DbIoFaultKind::Backend);
        {
            let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if db_io_slot_matches(&owner, handle) {
                if let Some(DbIoTask::PayloadGet { output, .. }) = owner.task.as_ref() {
                    let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    assert!(output.pages.iter().take(output.reserved as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::TerminalResult), "async fault taxonomy output left TerminalResult before its task retired");
                }
            }
        }
        let actual_error = fault.into_db_error();
        assert_eq!(actual_error, expected_error);
        assert_eq!(fault_category_oracle(&actual_error), expected_category);
        drain_control_tasks(control).await;
    }

    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_output_task_yield_cancel_abandon_and_close_retire_exactly_once() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let pool = db_io_test_pool();
    let success_steps = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let cancel_steps = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let abandon_steps = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let control =
        register_db_io_backend(DbIoBackendKind::Memory, Box::new(BlockingOutputLifecycleLawExecutor { terminal: false, success_steps: success_steps.clone(), cancel_steps: cancel_steps.clone(), abandon_steps: abandon_steps.clone() }), pool.clone())
            .unwrap();
    let expected: Vec<u8> = (0..DB_IO_PAGE_BYTES + 1).map(|index| (index % 251) as u8).collect();

    let output = DbIoPageWriter::try_reserve(2).unwrap();
    let operation = submit_db_io_task(DbIoTask::PayloadGet { backend: control, hash: ContentHash([0x71; 32]), output }).unwrap_or_else(|(error, _)| panic!("output lifecycle task admission failed: {error}"));
    let mut result = match exact_fixture_result(operation.await, "output lifecycle") {
        DbIoResult::Pages(pages) => pages,
        _ => panic!("output lifecycle returned the wrong typed result"),
    };
    assert!(success_steps.load(std::sync::atomic::Ordering::Acquire) > 4);
    assert_eq!(result, expected);
    while result.close_step().unwrap().is_some() {}
    drain_control_tasks(control).await;

    let output = DbIoPageWriter::try_reserve(2).unwrap();
    let operation = submit_db_io_task(DbIoTask::PayloadGet { backend: control, hash: ContentHash([0x72; 32]), output }).unwrap_or_else(|(error, _)| panic!("cancel lifecycle task admission failed: {error}"));
    let steps_deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while cancel_steps.load(std::sync::atomic::Ordering::Acquire) < 3 && std::time::Instant::now() < steps_deadline {
        std::thread::yield_now();
    }
    assert!(cancel_steps.load(std::sync::atomic::Ordering::Acquire) >= 3);
    let handle = operation.handle;
    operation.cancel().unwrap();
    let mut fault = match operation.await {
        Err(fault) => fault,
        Ok(_) => panic!("cancel lifecycle task published a result"),
    };
    {
        let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if db_io_slot_matches(&owner, handle) {
            if let Some(DbIoTask::PayloadGet { output, .. }) = owner.task.as_ref() {
                let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                assert!(output.pages.iter().take(output.reserved as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::TerminalResult));
            }
        }
    }
    assert_eq!(fault.kind, DbIoFaultKind::Cancelled);
    while fault.close_step() {}
    drain_control_tasks(control).await;

    let output = DbIoPageWriter::try_reserve(2).unwrap();
    let operation = submit_db_io_task(DbIoTask::PayloadGet { backend: control, hash: ContentHash([0x73; 32]), output }).unwrap_or_else(|(error, _)| panic!("abandon lifecycle task admission failed: {error}"));
    let steps_deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while abandon_steps.load(std::sync::atomic::Ordering::Acquire) < 3 && std::time::Instant::now() < steps_deadline {
        std::thread::yield_now();
    }
    assert!(abandon_steps.load(std::sync::atomic::Ordering::Acquire) >= 3);
    let handle = operation.handle;
    drop(operation);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, handle) || matches!(owner.phase, DbIoTaskPhase::Cancelled | DbIoTaskPhase::Closing) {
            break;
        }
        let witness = format!("phase={:?} cancelled={} abandoned={} task={} retry={:?} submit_kind={:?} steps={}", owner.phase, owner.cancelled, owner.abandoned, owner.task.is_some(), owner.retry_attempt, owner.terminal_submit_kind, abandon_steps.load(std::sync::atomic::Ordering::Acquire));
        drop(owner);
        assert!(std::time::Instant::now() < deadline, "abandoned lifecycle task never observed its cancellation: {witness}");
        std::thread::yield_now();
    }
    {
        let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if db_io_slot_matches(&owner, handle) {
            assert!(matches!(owner.phase, DbIoTaskPhase::Cancelled | DbIoTaskPhase::Closing));
            if let Some(DbIoTask::PayloadGet { output, .. }) = owner.task.as_ref() {
                let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                assert!(output.pages.iter().take(output.reserved as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::TerminalResult));
            }
        }
    }
    drain_control_tasks(control).await;

    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(ledger_witness(), before);
}

#[test]
fn db_io_artifact_and_lease_result_owners_retain_exact_incremental_handback() {
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_artifact_and_lease_result_owners_retain_exact_incremental_handback") {
        return;
    }
    let _owner = fixture_owner();
    let before = ledger_witness();
    let operation = db_io_operation_reserve(DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();
    let text = DbIoText::try_from_str("post-admission-artifact").unwrap();
    let mut artifact = DbIoArtifactId::try_from_text(operation, &text).unwrap();
    assert_eq!(artifact.as_str(), "post-admission-artifact");
    assert_eq!(artifact.as_artifact().unwrap().0, "post-admission-artifact");
    assert!(artifact.close_step().unwrap());
    drop(artifact);
    while db_io_lost_owner_close_step().unwrap() {}
    db_io_operation_return(operation, DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();

    let mut lease = DbIoLeaseResult::new(DbIoText::try_from_str("resource").unwrap(), DbIoText::try_from_str("holder").unwrap(), EpochFence::INITIAL, 10);
    assert!(lease.close_step());
    assert!(!lease.terminal_is_empty());
    drop(lease);
    assert!(db_io_lost_owner_close_step().unwrap());
    assert_eq!(ledger_witness(), before);
}

#[test]
fn db_io_process_page_max_plus_one_preflight_is_atomic() {
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_process_page_max_plus_one_preflight_is_atomic") {
        return;
    }
    let mut state = DbIoPageArenaState::new();
    for _ in 0..DB_IO_TOTAL_PAGES / DB_IO_OPERATION_PAGES {
        db_io_preflight_page_checkout(&state, DB_IO_OPERATION_PAGES).unwrap();
        state.free_len -= DB_IO_OPERATION_PAGES;
    }
    let before = (state.free_read, state.free_len, state.next_generation);
    assert!(db_io_preflight_page_checkout(&state, 1).is_err());
    assert_eq!((state.free_read, state.free_len, state.next_generation), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_platform_fixed_ring_max_plus_one_returns_exact_capacity() {
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_platform_fixed_ring_max_plus_one_returns_exact_capacity") {
        return;
    }
    let _owner = fixture_owner();
    let before = ledger_witness();
    let sources: [DbIoPages; DB_IO_PLATFORM_BUFFERS + 1] = std::array::from_fn(|_| pages(&[0x39]));
    let mut owners: [Option<DbIoPlatformBuffer>; DB_IO_PLATFORM_BUFFERS] = std::array::from_fn(|_| None);
    for (owner, source) in owners.iter_mut().zip(&sources) {
        let copy = match db_io_prepare_platform(source) {
            Ok(copy) => copy,
            Err(error) => panic!("platform max fixture reservation failed: {error}"),
        };
        *owner = Some(match copy.await {
            Ok(owner) => owner,
            Err(error) => panic!("platform max fixture copy failed: {error}"),
        });
    }
    let credit_before_refusal = ledger_witness();
    assert!(matches!(db_io_prepare_platform(&sources[DB_IO_PLATFORM_BUFFERS]), Err(DbError::Unavailable(detail)) if detail == "DB I/O prepared platform capacity exhausted"));
    assert_eq!(ledger_witness(), credit_before_refusal, "the ring max+1 refusal returns its exact capacity");
    for owner in &mut owners {
        let Some(owner) = owner.take() else { panic!("platform max fixture lost an admitted owner") };
        if let Err(error) = db_io_close_platform(owner).await {
            panic!("platform max fixture close failed: {error}");
        }
    }
    for source in sources {
        drain_pages(source);
    }
    assert_eq!(ledger_witness(), before);
}
