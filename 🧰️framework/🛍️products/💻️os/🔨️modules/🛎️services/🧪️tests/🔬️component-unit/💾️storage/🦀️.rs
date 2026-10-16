//! 💾️ Tests of storage: the fixed file page against the system oracle, retained fixed-file documents and the storage scheduler.

use semio_framework_actor::PackageId;
use semio_framework_async::{HostAsyncRuntime, ScopeOwner, select2};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use super::compute_pool::Yield;
use super::super::fixed_file_storage::{FixedFileDocumentWriteToken, STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES, STORAGE_FIXED_FILE_DOCUMENT_MAX_WRITE_STEPS, STORAGE_FIXED_FILE_PAGE_BYTES};
use super::super::host_runtime::TokioHostRuntime;
use super::super::storage_scheduler::{StorageError, StorageScheduler};
use super::support::{sleep_ms, test_ctx, test_pool};
#[cfg(not(target_arch = "wasm32"))]
use super::super::fixed_file_storage::{RetainedFixedFileDocuments, storage_worker_read_fixed_file_page, storage_worker_write_fixed_file_page};

//#region 📄️FixedFilePageTests
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn fixed_file_page_exact_max_plus_one_matches_system_oracle_and_preserves_last_valid_page() {
    let directory = std::env::temp_dir().join(format!("semio-fixed-file-page-{}-{:?}", std::process::id(), std::thread::current().id()));
    let path = directory.join("page.bin");
    let exact = vec![0x5au8; STORAGE_FIXED_FILE_PAGE_BYTES];
    storage_worker_write_fixed_file_page(&path, &exact, STORAGE_FIXED_FILE_PAGE_BYTES).expect("exact page must be admitted");
    let retained = storage_worker_read_fixed_file_page(&path, STORAGE_FIXED_FILE_PAGE_BYTES).expect("exact page must be readable");
    let oracle = std::fs::read(&path).expect("system oracle must read the same published page");
    assert_eq!(retained, oracle);
    assert_eq!(retained, exact);

    let plus_one = vec![0xa5u8; STORAGE_FIXED_FILE_PAGE_BYTES + 1];
    assert_eq!(storage_worker_write_fixed_file_page(&path, &plus_one, STORAGE_FIXED_FILE_PAGE_BYTES).expect_err("max plus one must fail before truncation").kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(std::fs::read(&path).expect("rejected write must preserve the last valid page"), exact);

    std::fs::write(&path, &plus_one).expect("system oracle must publish hostile oversized input");
    assert_eq!(storage_worker_read_fixed_file_page(&path, STORAGE_FIXED_FILE_PAGE_BYTES).expect_err("oversized hostile page must fail closed").kind(), std::io::ErrorKind::InvalidData);
    let _ = std::fs::remove_dir_all(&directory);
}
//#endregion 📄️FixedFilePageTests
//#region 📚️RetainedFixedFileDocumentTests
#[cfg(not(target_arch = "wasm32"))]
fn finish_fixed_file_document_write(documents: &mut RetainedFixedFileDocuments, token: FixedFileDocumentWriteToken, expected_steps: usize) {
    let mut steps = 0;
    loop {
        let outcome = documents.write_step(token).expect("the exact retained owner advances one bounded step");
        steps += 1;
        assert!(outcome.written_bytes <= STORAGE_FIXED_FILE_PAGE_BYTES);
        if outcome.ready_to_publish {
            break;
        }
    }
    assert_eq!(steps, expected_steps);
    documents.publish(token).expect("the complete exact owner publishes atomically");
}

#[cfg(not(target_arch = "wasm32"))]
fn fixed_file_document_temporary(directory: &std::path::Path, destination: &std::path::Path) -> std::path::PathBuf {
    std::fs::read_dir(directory).expect("document directory").filter_map(Result::ok).map(|entry| entry.path()).find(|path| path != destination).expect("one inactive document")
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn retained_fixed_file_document_preserves_exact_owner_cancellation_supersession_and_atomic_publication() {
    assert_eq!(STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES, 64 * 1024);
    assert_eq!(STORAGE_FIXED_FILE_DOCUMENT_MAX_WRITE_STEPS, 4);
    assert_eq!(STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES, STORAGE_FIXED_FILE_PAGE_BYTES * STORAGE_FIXED_FILE_DOCUMENT_MAX_WRITE_STEPS);

    let directory = std::env::temp_dir().join(format!("semio-fixed-file-document-{}-{:?}", std::process::id(), std::thread::current().id()));
    let path = directory.join("semio.os.config.field");
    let mut documents = RetainedFixedFileDocuments::default();
    let prior = br#"{"version":1,"preferences":{},"namedLayouts":{},"dockLayouts":{"apps":{}},"dockUi":{"apps":{}},"windowPanes":{"apps":{}}}"#.to_vec();
    let prior_token = documents.begin_write(&path, 7, 1, prior.clone(), STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES).expect("the prior document is admitted");
    finish_fixed_file_document_write(&mut documents, prior_token, 1);
    assert_eq!(documents.read(&path, STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES).expect("the prior document is readable"), prior);

    let exact = vec![0x5au8; STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES];
    let exact_token = documents.begin_write(&path, 7, 2, exact.clone(), STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES).expect("the exact document is admitted");
    for _ in 0..STORAGE_FIXED_FILE_DOCUMENT_MAX_WRITE_STEPS {
        let outcome = documents.write_step(exact_token).expect("the exact owner advances");
        assert_eq!(outcome.written_bytes, STORAGE_FIXED_FILE_PAGE_BYTES);
        assert_eq!(documents.read(&path, STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES).expect("an inactive write keeps the committed document visible"), prior);
    }
    documents.publish(exact_token).expect("four complete steps publish");
    assert_eq!(documents.read(&path, STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES).expect("the exact document is readable"), exact);
    assert_eq!(std::fs::read(&path).expect("the system-file oracle reads the same single value"), exact);
    assert!(documents.begin_write(&path, 8, 2, vec![0x29u8; STORAGE_FIXED_FILE_PAGE_BYTES], STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES).is_err(), "the published generation remains stale for every owner");

    let oversized = vec![0xa5u8; STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES + 1];
    assert!(documents.begin_write(&path, 7, 3, oversized, STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES).is_err());
    assert_eq!(documents.read(&path, STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES).expect("plus one preserves the committed document"), exact);

    let cancelled = documents.begin_write(&path, 7, 4, vec![0xc3u8; STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES], STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES).expect("the cancellable document is admitted");
    for _ in 0..2 {
        documents.write_step(cancelled).expect("the exact cancellable owner advances");
    }
    documents.cancel(cancelled).expect("the exact owner cancels");
    assert!(documents.publish(cancelled).is_err());
    assert!(documents.cancel_step(cancelled).expect("bounded cancellation drains"));
    assert!(documents.write_step(cancelled).is_err(), "a terminal token stays stale");
    assert!(documents.begin_write(&path, 7, 4, vec![0x47u8; STORAGE_FIXED_FILE_PAGE_BYTES], STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES).is_err(), "the cancelled generation remains stale for its owner");
    assert!(documents.begin_write(&path, 8, 4, vec![0x48u8; STORAGE_FIXED_FILE_PAGE_BYTES], STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES).is_err(), "the cancelled generation remains stale for every owner");
    assert_eq!(documents.read(&path, STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES).expect("cancellation preserves the committed document"), exact);

    let stale = documents.begin_write(&path, 7, 5, vec![0xd4u8; STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES], STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES).expect("the first writer is admitted");
    documents.write_step(stale).expect("the first writer advances once");
    assert!(documents.begin_write(&path, 8, 5, vec![0xd5u8; STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES], STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES).is_err(), "a different owner cannot reuse the current generation");
    let replacement_bytes = vec![0xe5u8; STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES];
    let replacement = documents.begin_write(&path, 7, 6, replacement_bytes.clone(), STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES).expect("a newer generation supersedes the prior writer");
    assert!(documents.write_step(stale).is_err());
    assert!(documents.publish(stale).is_err());
    finish_fixed_file_document_write(&mut documents, replacement, STORAGE_FIXED_FILE_DOCUMENT_MAX_WRITE_STEPS);
    assert_eq!(documents.read(&path, STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES).expect("only the replacement is visible"), replacement_bytes);

    for (generation, trailing_bytes) in [(7, 0u64), (8, 1u64)] {
        let hostile = documents.begin_write(&path, 7, generation, vec![0xf6u8; STORAGE_FIXED_FILE_PAGE_BYTES], STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES).expect("hostile owner admitted");
        documents.write_step(hostile).expect("hostile owner completes its page");
        let temporary = fixed_file_document_temporary(&directory, &path);
        std::fs::OpenOptions::new()
            .write(true)
            .open(&temporary)
            .expect("hostile temporary")
            .set_len(if trailing_bytes == 0 { (STORAGE_FIXED_FILE_PAGE_BYTES - 1) as u64 } else { STORAGE_FIXED_FILE_PAGE_BYTES as u64 + trailing_bytes })
            .expect("mutate temporary length");
        assert!(documents.publish(hostile).is_err(), "a truncated or trailing temporary refuses publication");
        documents.cancel(hostile).expect("hostile owner cancels");
        assert!(documents.cancel_step(hostile).expect("hostile temporary drains"));
        assert_eq!(documents.read(&path, STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES).expect("integrity refusal preserves prior"), replacement_bytes);
    }

    assert!(documents.publish(FixedFileDocumentWriteToken::new(9, 1)).is_err(), "a non-owner token cannot publish");
    let _ = std::fs::remove_dir_all(directory);
}
//#endregion 📚️RetainedFixedFileDocumentTests
//#region 💾️StorageSchedulerTests
/// 💾️ `ManualRuntime`'s dispatch would run synchronously, in-line, so nothing can ever be
/// genuinely QUEUED behind it — a real `TokioHostRuntime` is required here to make the priority
/// ordering observable at all: one job occupies the single in-flight slot on its own worker
/// while the two real submissions below queue behind it.
#[semio_framework_async_macros::async_test]
async fn storage_scheduler_dispatches_highest_priority_lane_first_despite_submit_order() {
    let runtime = Arc::new(TokioHostRuntime::with_pool(test_pool(4)));
    let scope = runtime.open_scope(ScopeOwner::Service("storage-priority-test"), None).await;
    let scheduler = StorageScheduler::new(runtime.clone(), scope.clone(), 1, 1_000_000).await;
    let order = Arc::new(Mutex::new(Vec::<&'static str>::new()));

    let (occupy_tx, occupy_rx) = std::sync::mpsc::channel::<()>();
    let occupy_rx = Mutex::new(occupy_rx);
    let occupy_ctx = test_ctx(0, scope.cancel.clone()).await;
    let occupy_ticket = scheduler
        .submit(&occupy_ctx, PackageId("occupy".to_string()), 1, move || {
            occupy_rx.lock().unwrap().recv().ok();
            Ok(Vec::new())
        })
        .unwrap();

    let mut low_ctx = test_ctx(0, scope.cancel.clone()).await;
    low_ctx.lane = 200;
    let order_low = order.clone();
    let low_ticket = scheduler
        .submit(&low_ctx, PackageId("p".to_string()), 1, move || {
            order_low.lock().unwrap().push("low");
            Ok(Vec::new())
        })
        .unwrap();

    let mut high_ctx = test_ctx(0, scope.cancel).await;
    high_ctx.lane = 1;
    let order_high = order.clone();
    let high_ticket = scheduler
        .submit(&high_ctx, PackageId("p".to_string()), 1, move || {
            order_high.lock().unwrap().push("high");
            Ok(Vec::new())
        })
        .unwrap();

    runtime.block_on(async {
        occupy_tx.send(()).expect("occupying job must still be waiting to receive");
        let _ = occupy_ticket.await_result().await;
        let _ = low_ticket.await_result().await;
        let _ = high_ticket.await_result().await;
    });
    assert_eq!(*order.lock().unwrap(), vec!["high", "low"], "the higher-priority lane (lower ctx.lane) must dispatch before the lower-priority one despite submitting second");
}

#[semio_framework_async_macros::async_test]
async fn storage_scheduler_never_exceeds_max_in_flight() {
    let runtime = Arc::new(TokioHostRuntime::with_pool(test_pool(4)));
    let scope = runtime.open_scope(ScopeOwner::Service("storage-cap-test"), None).await;
    let scheduler = StorageScheduler::new(runtime.clone(), scope.clone(), 2, 1_000_000).await;
    let mut tickets = Vec::new();
    for i in 0..8u32 {
        let ctx = test_ctx(0, scope.cancel.clone()).await;
        let ticket = scheduler
            .submit(&ctx, PackageId(format!("p{i}")), 1, move || {
                std::thread::sleep(Duration::from_millis(15));
                Ok(Vec::new())
            })
            .unwrap();
        tickets.push(ticket);
    }
    let cap_violated = Arc::new(AtomicBool::new(false));
    runtime.block_on(async {
        let scheduler_ref = &scheduler;
        let cap_violated_ref = &cap_violated;
        let sampler = async {
            loop {
                if scheduler_ref.in_flight().await > 2 {
                    cap_violated_ref.store(true, Ordering::SeqCst);
                }
                Yield(false).await;
            }
        };
        let drain = async {
            for ticket in tickets {
                let _ = ticket.await_result().await;
            }
        };
        select2(sampler, drain).await;
    });
    assert!(!cap_violated.load(Ordering::SeqCst), "in-flight count must never exceed max_in_flight");
}

/// 💾️ A real `TokioHostRuntime` is required here too (same reasoning as the priority test
/// above): the first job must still be genuinely IN FLIGHT — holding its 60-byte reservation —
/// when the second `submit` is checked, so it is blocked on a channel rather than left to
/// `ManualRuntime`'s synchronous, immediately-releasing execution.
#[semio_framework_async_macros::async_test]
async fn storage_scheduler_rejects_over_budget_submit_with_a_typed_error_and_untouched_usage() {
    let runtime = Arc::new(TokioHostRuntime::with_pool(test_pool(4)));
    let scope = runtime.open_scope(ScopeOwner::Service("storage-budget-test"), None).await;
    let scheduler = StorageScheduler::new(runtime.clone(), scope.clone(), 4, 100).await;
    let ctx = test_ctx(0, scope.cancel).await;
    let plugin = PackageId("p".to_string());
    let (hold_tx, hold_rx) = std::sync::mpsc::channel::<()>();
    let hold_rx = Mutex::new(hold_rx);
    let first_ticket = scheduler
        .submit(&ctx, plugin.clone(), 60, move || {
            hold_rx.lock().unwrap().recv().ok();
            Ok(Vec::new())
        })
        .unwrap();
    let result = scheduler.submit(&ctx, plugin.clone(), 60, || Ok(Vec::new()));
    let error = result.err().expect("submitting past the byte quota must be rejected while the first job still holds its reservation");
    assert_eq!(error, StorageError::BytesQuotaExceeded { plugin, limit: 100 });
    let _ = hold_tx.send(());
    runtime.block_on(async {
        let _ = first_ticket.await_result().await;
    });
}

/// ⏰️ Occupies the scheduler's ONE in-flight slot with a job blocked on a channel, then submits
/// a second job with a short `ctx.deadline_ms` — since the slot never frees during that window,
/// the second job stays queued, so `await_result` must lose the race against its own deadline
/// (never actually run `ran`), and once the occupier finally completes, `storage_try_dispatch`
/// must skip the now-cancelled job and release ITS byte reservation too (proved by a follow-up
/// submit that would otherwise not fit the tight per-plugin quota) — the 50ms sleep after the
/// occupier's own result gives that skip a chance to happen, since it runs on the completion
/// closure's own thread, which may still be unwinding when this task's waker fires.
#[semio_framework_async_macros::async_test]
async fn storage_scheduler_races_a_queued_job_against_its_deadline_and_frees_its_reservation_when_lost() {
    let runtime = Arc::new(TokioHostRuntime::with_pool(test_pool(4)));
    let scope = runtime.open_scope(ScopeOwner::Service("storage-deadline-test"), None).await;
    let scheduler = StorageScheduler::new(runtime.clone(), scope.clone(), 1, 50).await;

    let (occupy_tx, occupy_rx) = std::sync::mpsc::channel::<()>();
    let occupy_rx = Mutex::new(occupy_rx);
    let occupy_ctx = test_ctx(0, scope.cancel.clone()).await;
    let occupy_ticket = scheduler
        .submit(&occupy_ctx, PackageId("occupy".to_string()), 1, move || {
            occupy_rx.lock().unwrap().recv().ok();
            Ok(Vec::new())
        })
        .unwrap();

    let plugin = PackageId("p".to_string());
    let ran = Arc::new(AtomicBool::new(false));
    let ran_clone = ran.clone();
    let outcome = runtime.block_on(async {
        let now = runtime.now_ms().await;
        let mut deadline_ctx = test_ctx(0, scope.cancel.clone()).await;
        deadline_ctx.deadline_ms = Some(now + 30);
        let deadline_ticket = scheduler.submit(&deadline_ctx, plugin.clone(), 42, move || {
            ran_clone.store(true, Ordering::SeqCst);
            Ok(Vec::new())
        });
        deadline_ticket.expect("submit itself must succeed; only the eventual run races the deadline").await_result().await
    });
    assert_eq!(outcome, Err(StorageError::DeadlineExceeded), "a job stuck behind a full in-flight slot must lose the race against its own deadline");
    assert!(!ran.load(Ordering::SeqCst), "the queued job must never have actually run once its deadline had already fired");

    let _ = occupy_tx.send(());
    runtime.block_on(async {
        let _ = occupy_ticket.await_result().await;
        sleep_ms(runtime.as_ref(), 50).await;
    });
    let verify_ticket = scheduler.submit(&test_ctx(0, scope.cancel).await, plugin.clone(), 45, || Ok(Vec::new()));
    let verify_ticket = verify_ticket.expect("the deadline-lost job's 42-byte reservation must have been released, leaving room for 45 more under the 50-byte quota");
    runtime.block_on(async {
        let _ = verify_ticket.await_result().await;
    });
}
//#endregion 💾️StorageSchedulerTests
