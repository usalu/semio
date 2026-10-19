use super::*;

use db_io_core::{PayloadStorage as _, WalStorage as _};

#[test]
fn interrupted_query_stream_drop_retains_one_resumable_close_owner() {
    if !db_io_driver::process_isolated_law("tests::interrupted_query_stream_drop_retains_one_resumable_close_owner") {
        return;
    }
    while engine_query_maintenance_step().unwrap() {}
    let mut stream = QueryStream::new();
    stream.push(QueryResultEntry { path: db_io_core::DbIoText::try_from_str("retained-path").unwrap(), value: None }).unwrap();
    drop(stream);
    assert!(engine_query_maintenance_step().unwrap());
    {
        let retired = ENGINE_QUERY_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(retired.iter().flatten().any(|owner| !owner.terminal_is_empty()));
    }
    while engine_query_maintenance_step().unwrap() {}

    ENGINE_QUERY_RETIREMENT_PRESSURE_FAULT.store(false, std::sync::atomic::Ordering::Release);
    {
        let mut retired = ENGINE_QUERY_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in retired.iter_mut() {
            *slot = Some(QueryStream::new());
        }
    }
    {
        let mut overflow = ENGINE_QUERY_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in overflow.iter_mut() {
            *slot = Some(QueryStream::new());
        }
    }
    let mut exact = QueryStream::new();
    exact.push(QueryResultEntry { path: db_io_core::DbIoText::try_from_str("exact-overflow-stream").unwrap(), value: None }).unwrap();
    let mut second = QueryStream::new();
    second.push(QueryResultEntry { path: db_io_core::DbIoText::try_from_str("second-overflow-stream").unwrap(), value: None }).unwrap();
    assert_eq!(exact.retirement.map(|reservation| reservation.tier), Some(2));
    assert_eq!(second.retirement.map(|reservation| reservation.tier), Some(2));
    assert!(retire_engine_query_stream(exact).is_ok());
    assert!(retire_engine_query_stream(second).is_ok());
    assert!(ENGINE_QUERY_RETIREMENT_PRESSURE_FAULT.load(std::sync::atomic::Ordering::Acquire));
    {
        let quarantine = ENGINE_QUERY_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(quarantine.iter().flatten().find_map(|stream| stream.get(0).map(QueryResultEntry::path)), Some("exact-overflow-stream"));
        assert!(quarantine.iter().flatten().any(|stream| stream.get(0).map(QueryResultEntry::path) == Some("second-overflow-stream")));
    }
    {
        let mut retired = ENGINE_QUERY_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in retired.iter_mut() {
            *slot = None;
        }
    }
    for _ in 0..ENGINE_RETIRED_QUERY_STREAMS * 2 {
        assert!(engine_query_maintenance_step().unwrap());
    }
    assert!(engine_query_maintenance_step().unwrap());
    {
        let retired = ENGINE_QUERY_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(retired.iter().flatten().find_map(|stream| stream.get(0).map(QueryResultEntry::path)), Some("exact-overflow-stream"));
    }
    while engine_query_maintenance_step().unwrap() {}

    for tier in [&ENGINE_QUERY_RETIREMENT, &ENGINE_QUERY_RETIREMENT_OVERFLOW, &ENGINE_QUERY_RETIREMENT_QUARANTINE] {
        let mut owners = tier.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in owners.iter_mut() {
            *slot = Some(QueryStream::new());
        }
    }
    let exact_refusal = QueryResultEntry { path: db_io_core::DbIoText::try_from_str("exact-all-tier-stream-refusal").unwrap(), value: None };
    let mut refused = QueryStream::new();
    let mut exact_refusal = refused.push(exact_refusal).unwrap_err();
    assert_eq!(exact_refusal.path(), "exact-all-tier-stream-refusal");
    assert!(exact_refusal.close_step().unwrap());
    for tier in [&ENGINE_QUERY_RETIREMENT, &ENGINE_QUERY_RETIREMENT_OVERFLOW, &ENGINE_QUERY_RETIREMENT_QUARANTINE] {
        let mut owners = tier.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in owners.iter_mut() {
            *slot = None;
        }
    }
    assert!(refused.terminal_is_empty());
}
//#region 🔖️Database open/catalog
#[test]
fn database_document_mount_hard_scheduler_fault_retains_nonrunnable_job_without_retry_timer() {
    let pool = test_worker_pool();
    assert_eq!(pool.shutdown(), Ok(()));
    let registry = Arc::new(Mutex::new(DatabaseDocumentMountRegistry::new()));
    let owner = DatabaseDocumentMountOwner::new(
        pool,
        Arc::downgrade(&registry),
        String::from("non-runnable-mount"),
        1,
        Box::pin(async { std::future::pending::<Result<DatabaseDocumentMountReply, DatabaseDocumentMountFailure>>().await }),
        Arc::new(Mutex::new(None)),
        Arc::new(Mutex::new(None)),
    );
    owner.request_drive(false);
    assert_eq!(owner.driver.load(std::sync::atomic::Ordering::Acquire), DatabaseDocumentMountDriver::NonRunnable as u8);
    assert_eq!(owner.scheduler_fault(), Some(semio_framework_async::WorkerSubmitErrorKind::Shutdown));
    assert!(owner.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
    assert!(!owner.retry_armed.load(std::sync::atomic::Ordering::Acquire));
    drop(owner.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take());
    owner.terminal.store(true, std::sync::atomic::Ordering::Release);
    owner.driver.store(DatabaseDocumentMountDriver::Terminal as u8, std::sync::atomic::Ordering::Release);
}
//#endregion 🔖️Database open/catalog
//#region 🔖️Round trip
#[test]
fn query_stream_max_plus_one_hands_back_exact_owner_and_close_is_terminal() {
    if !db_io_driver::process_isolated_law("tests::query_stream_max_plus_one_hands_back_exact_owner_and_close_is_terminal") {
        return;
    }
    let mut stream = QueryStream::new();
    for index in 0..64 {
        stream.push(QueryResultEntry { path: db_io_core::DbIoText::try_from_str(&format!("path-{index:02}")).unwrap(), value: None }).unwrap();
    }
    let rejected = stream.push(QueryResultEntry { path: db_io_core::DbIoText::try_from_str("overflow-owner").unwrap(), value: None }).unwrap_err();
    assert_eq!(rejected.path(), "overflow-owner");
    let mut rejected = rejected;
    while rejected.close_step().unwrap() {}
    while stream.close_step().unwrap() {}
    assert!(stream.terminal_is_empty());
}
//#endregion 🔖️Round trip

//#region 🔖️Source laws
#[test]
fn retained_retry_owners_release_their_guard_before_resubmission() {
    for (name, source) in [("db_engine_base", include_str!("../../🦀️.rs"))] {
        let lines: Vec<&str> = source.lines().collect();
        for (index, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start().trim_start_matches("} else ");
            let Some((_, scrutinee)) = trimmed.strip_prefix("if let ").or_else(|| trimmed.strip_prefix("while let ")).and_then(|rest| rest.split_once(" = ")) else { continue };
            if !(scrutinee.contains("retry_job.lock()") && !scrutinee.starts_with("{ ") && scrutinee.trim_end().ends_with('{')) {
                continue;
            }
            let mut depth = 1i64;
            for body in &lines[index + 1..] {
                assert!(!body.contains("retry_job.lock()") && !body.contains("submit_exact("), "{name}:{} holds the retry-job guard across a body that relocks or resubmits it: {line}", index + 1);
                depth += body.matches('{').count() as i64 - body.matches('}').count() as i64;
                if depth <= 0 {
                    break;
                }
            }
        }
    }
}
//#endregion 🔖️Source laws
