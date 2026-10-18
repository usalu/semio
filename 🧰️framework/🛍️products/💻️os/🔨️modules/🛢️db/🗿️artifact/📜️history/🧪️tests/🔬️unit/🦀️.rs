use super::*;

use std::sync::Arc as StdArc;

/// 🧾️ The authored source of the history crate: every child module and then the root, in module order, as the one text the source laws scan.
const HISTORY_SOURCE: &str = concat!(
    include_str!("../../🧭️replay-limits-view-and-reservation/🦀️.rs"),
    include_str!("../../🛠️reservation-construction/🦀️.rs"),
    include_str!("../../🔖️page-set-and-cursors/🦀️.rs"),
    include_str!("../../⏳️replay-future/🦀️.rs"),
    include_str!("../../🦀️.rs"),
);

fn history_construction_test_lock() -> std::sync::MutexGuard<'static, ()> {
    history_capacity_test_lock()
}

#[semio_framework_async_macros::async_test]
async fn artifact_history_replay_uses_neutral_committed_inventory_and_retires_every_owner() {
    if !db_io_driver::process_isolated_law("tests::artifact_history_replay_uses_neutral_committed_inventory_and_retires_every_owner") {
        return;
    }
    let _history_capacity = history_construction_test_lock();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../📝️wal/🧪️tests/🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
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
        let memory = db_wal::testkit::committed_fixture_storage(row, &document).await;
        if compacted || hole {
            let writer = db_io_core::WalStorage::acquire_writer(&memory, &document).await.unwrap();
            if compacted {
                db_io_core::WalStorage::delete_segment(&memory, &writer, 0).await.unwrap();
            }
            if hole {
                db_io_core::WalStorage::create_segment(&memory, &writer, 3).await.unwrap();
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
    }
}

async fn storage() -> StdArc<db_storage::DbBackend> {
    StdArc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()))
}
//#region 🔖️Actor
#[test]
fn artifact_history_backend_token_crc_fault_retire_1024_pages_one_grant_each() {
    if !db_io_driver::process_isolated_law("tests::artifact_history_backend_token_crc_fault_retire_1024_pages_one_grant_each") {
        return;
    }
    let _history_capacity = history_construction_test_lock();
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
    let replay = HISTORY_SOURCE;
    for fault in ["history backend page ownership", "history envelope has trailing bytes", "history source finished without authentication"] {
        assert!(replay.contains(fault), "missing retained fault source {fault}");
    }
    assert!(replay.contains("HistoryReplayTransition::FaultRetire"));
}

#[test]
fn artifact_history_scratch_result_boundary_plus_one_preserves_exact_owner() {
    if !db_io_driver::process_isolated_law("tests::artifact_history_scratch_result_boundary_plus_one_preserves_exact_owner") {
        return;
    }
    let _history_capacity = history_construction_test_lock();
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
    if !db_io_driver::process_isolated_law("tests::artifact_history_reservation_construction_fault_cap_plus_one_and_each_page_retire_one_owner") {
        return;
    }
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
            let mut claimed = slot.cursor.replace(close).expect("construction claim installs its exact empty owner");
            while claimed.close_step() {}
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
    if !db_io_driver::process_isolated_law("tests::artifact_history_unchecked_construction_error_and_checked_out_drop_hand_back_exact_pages") {
        return;
    }
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
    if !db_io_driver::process_isolated_law("tests::artifact_history_construction_unwind_hands_partial_owner_to_registry_without_bulk_drop") {
        return;
    }
    let _guard = history_construction_test_lock();
    let generation = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).next_generation;
    let unwind = std::panic::catch_unwind(|| {
        let mut builder = HistoryReplayReservationConstructionBuilder::new().expect("fixed construction authority");
        let mut page = Vec::new();
        page.try_reserve_exact(HISTORY_REPLAY_PAGE_BYTES as usize).expect("fixture page");
        page.resize(HISTORY_REPLAY_PAGE_BYTES as usize, 0);
        assert!(builder
            .edit_cursor(|cursor| cursor.result_pages.as_mut().is_some_and(|owners| {
                owners.push(Some(page));
                true
            }))
            .unwrap_or(false));
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
    if !db_io_driver::process_isolated_law("tests::artifact_history_construction_registry_saturation_rejects_before_partial_owner_and_reuses_with_fresh_generation") {
        return;
    }
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
    if !db_io_driver::process_isolated_law("tests::artifact_history_construction_handback_rejects_stale_duplicate_and_aba_without_owner_overwrite") {
        return;
    }
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
//#endregion 🔖️Actor

//#region 🔖️Source laws
#[test]
fn artifact_history_fixed_owner_accounting_has_no_capacity_scan() {
    let source = HISTORY_SOURCE;
    let replay = &source[source.find("//#region 🔖️HistoryReplay").unwrap()..source.rfind("//#endregion 🔖️HistoryReplay").unwrap()];
    for forbidden in [".rposition(", "result_pages.iter()", "source_pages.iter().all", "pages.iter().all", "pages.iter().filter"] {
        assert!(!replay.contains(forbidden), "retained history accounting scanned fixed capacity through {forbidden}");
    }
    for retained in ["source_page_count", "retained_operation_bytes", "retained_result_bytes"] {
        assert!(replay.contains(retained), "retained history accounting omitted {retained}");
    }
}

#[test]
fn artifact_history_nested_derived_item_and_byte_caps_precede_materialization() {
    let artifact = HISTORY_SOURCE;
    for required in ["HISTORY_REPLAY_RESULT_BYTES", "HISTORY_REPLAY_OPERATION_BYTES", "HISTORY_REPLAY_MAX_ENTRIES", "HISTORY_REPLAY_MAX_OPERATION_IDS", "history dependency item credit", "history result byte credit"] {
        assert!(artifact.contains(required), "missing {required}");
    }
    assert!(artifact.contains("if self.operation_ids.len() >= HISTORY_REPLAY_MAX_OPERATION_IDS"));
    let preflight = artifact.find("owner.preflight_result_range(this.result_len, len)").unwrap();
    let publish = artifact.find("reservation.operation_ids.push(HistoryTextRange").unwrap();
    assert!(preflight < publish);
}

#[test]
fn artifact_history_segment_cap_plus_one_reads_only_one_admitted_page() {
    if !db_io_driver::process_isolated_law("tests::artifact_history_segment_cap_plus_one_reads_only_one_admitted_page") {
        return;
    }
    let artifact = HISTORY_SOURCE;
    assert!(artifact.contains("HISTORY_REPLAY_SEGMENT_PAGES: u64 = 1_024"));
    assert!(artifact.contains(".min(HISTORY_REPLAY_PAGE_BYTES)"));
    assert!(artifact.contains("pack::ByteRange { offset, len: requested }"));
    assert!(artifact.contains("page.capacity() as u64 > HISTORY_REPLAY_PAGE_BYTES"));
    assert!(!artifact.contains("pack::ByteRange { offset: 0, len }"));
}

#[test]
fn artifact_history_cancel_retires_one_page_or_nested_owner_per_actor_grant() {
    let artifact = HISTORY_SOURCE;
    assert!(artifact.contains("HistoryReplayTransition::FaultRetire"));
    assert!(artifact.contains("self.page_count -= 1"));
    assert!(artifact.contains("self.operation_ids.pop().is_some()"));
    assert!(artifact.contains("self.result_pages.pop()"));
    assert!(!artifact.contains("pages.clear()"));
}

#[test]
fn artifact_history_replay_ordering_is_segment_frame_then_result_fifo() {
    let artifact = HISTORY_SOURCE;
    let probe = artifact.find("HistoryReplayPhase::Probe => {").unwrap();
    let frame = artifact.find("HistoryReplayPhase::Frame { index } => {").unwrap();
    let publish = artifact.find("reservation.entries.push(ArtifactHistoryEntry").unwrap();
    assert!(probe < frame && frame < publish);
    assert!(artifact.contains("self.segment_ordinal += 1") || artifact.contains("this.segment_ordinal += 1"));
    assert!(!HISTORY_SOURCE.contains("ArtifactHistoryWorkOwner::Map"));
}

#[test]
fn artifact_history_crc_and_frame_tokenizer_advance_one_page_per_grant() {
    let artifact = HISTORY_SOURCE;
    assert!(artifact.contains("db_wal_log::WalAuthenticatedSource::new(std::mem::take(&mut this.pages), gate, *index, this.previous_tip)"));
    assert!(artifact.contains(".verify_step(&this.document, &mut control)"));
    assert!(artifact.contains("remaining.min(HISTORY_REPLAY_PAGE_BYTES)"));
    assert!(!artifact.contains("protocol::codec::crc32c("));
    assert!(!artifact.contains("decode_history_token"));
}

#[test]
fn retained_retry_owners_release_their_guard_before_resubmission() {
    for (name, source) in [("db_artifact_history", HISTORY_SOURCE)] {
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

#[test]
fn artifact_history_replay_construction_fault_authority_is_retained() {
    let artifact = HISTORY_SOURCE;
    assert!(artifact.contains("const HISTORY_REPLAY_CONSTRUCTION_SLOTS: usize = 64"));
    assert!(artifact.contains("pub fn try_new()"));
    assert!(artifact.contains("pub struct HistoryReplayReservationConstructionFault"));
    assert!(artifact.contains("impl Drop for HistoryReplayReservationConstructionBuilder"));
    assert!(artifact.contains("impl Drop for HistoryReplayReservationConstructionFault"));
    assert!(artifact.contains("take_history_replay_reservation_construction_fault"));
    assert!(!artifact.contains("pub fn into_parts"));
    assert!(!artifact.contains("pub struct HistoryReplayReservationConstructionFaultCursor"));
    assert!(artifact.contains("try_new_with_result_page_failure"));
}

#[test]
fn artifact_history_replay_cursor_one_grant_has_no_blocking_wait() {
    let artifact = HISTORY_SOURCE;
    let replay = &artifact[artifact.find("//#region 🔖️HistoryReplay").unwrap()..artifact.rfind("//#endregion 🔖️HistoryReplay").unwrap()];
    for forbidden in ["block_on(", "ask_blocking", "submit_blocking", "loop {", "while "] {
        assert!(!replay.contains(forbidden), "history replay cursor retained {forbidden}");
    }
}
//#endregion 🔖️Source laws
