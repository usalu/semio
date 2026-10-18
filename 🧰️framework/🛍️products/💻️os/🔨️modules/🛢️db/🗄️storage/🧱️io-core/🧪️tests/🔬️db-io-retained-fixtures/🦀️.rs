use db_io_core::*;
use db_io_core::testkit::*;
use db_ids::{DbError, GenerationId};

fn fixture_owner() -> DbIoLedgerOwner {
    DbIoLedgerOwner::enter()
}

fn ledger_witness() -> (DbIoCredit, usize) {
    db_io_ledger_census(DB_IO_LEDGER_OWNER.with(std::cell::Cell::get))
}

fn pages(bytes: &[u8]) -> DbIoPages {
    let mut writer = DbIoPageWriter::try_reserve(bytes.len().div_ceil(DB_IO_PAGE_BYTES)).expect("fixture pages admitted");
    for fragment in bytes.chunks(DB_IO_PAGE_BYTES) {
        assert_eq!(writer.write_fragment(fragment).unwrap(), fragment.len());
    }
    writer.finish().unwrap()
}

#[test]
fn db_io_executing_output_seal_keeps_every_page_executing_until_atomic_publication() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let bytes: Vec<u8> = (0..DB_IO_PAGE_BYTES + 1).map(|index| (index % 251) as u8).collect();
    let mut writer = DbIoPageWriter::try_reserve(2).unwrap();
    assert_eq!(writer.write_fragment(&bytes).unwrap(), DB_IO_PAGE_BYTES);
    assert_eq!(writer.write_fragment(&bytes[DB_IO_PAGE_BYTES..]).unwrap(), 1);
    writer.transition(DbIoPagePhase::CheckedOutWriter, DbIoPagePhase::Queued).unwrap();
    writer.transition(DbIoPagePhase::Queued, DbIoPagePhase::Executing).unwrap();

    let mut yields = 0;
    let mut published = loop {
        match writer.seal_retained_step().unwrap() {
            Some(pages) => break pages,
            None => {
                yields += 1;
                let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                assert!(writer.pages.iter().take(writer.reserved as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::Executing));
                drop(arena);
                writer.transition(DbIoPagePhase::Executing, DbIoPagePhase::Queued).unwrap();
                writer.transition(DbIoPagePhase::Queued, DbIoPagePhase::Executing).unwrap();
            }
        }
    };

    assert!(yields > 1);
    assert_eq!(published, bytes);
    {
        let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(published.pages.iter().take(published.retained as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::TerminalResult));
    }
    while published.close_step().unwrap().is_some() {}
    assert_eq!(ledger_witness(), before);
}

#[test]
fn db_io_page_identity_rejects_generation_operation_and_phase_mismatches_exactly() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let mut writer = DbIoPageWriter::try_reserve(1).unwrap();
    let page = writer.pages[0].as_ref().unwrap();
    let (slot_index, generation, operation) = (page.slot as usize, page.generation, page.operation);

    {
        let mut arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        arena.slots[slot_index].generation = generation + 1;
    }
    let generation_error = writer.transition(DbIoPagePhase::CheckedOutWriter, DbIoPagePhase::Queued).unwrap_err();
    {
        let mut arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        arena.slots[slot_index].generation = generation;
    }
    assert_eq!(generation_error, DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(generation + 1) });

    {
        let mut arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        arena.slots[slot_index].operation = operation + 1;
    }
    let operation_error = writer.transition(DbIoPagePhase::CheckedOutWriter, DbIoPagePhase::Queued).unwrap_err();
    {
        let mut arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        arena.slots[slot_index].operation = operation;
    }
    assert!(matches!(operation_error, DbError::Internal(message) if message.contains("page operation mismatch") && message.contains(&operation.to_string())));

    let phase_error = writer.transition(DbIoPagePhase::Executing, DbIoPagePhase::Queued).unwrap_err();
    assert!(matches!(phase_error, DbError::Internal(message) if message.contains("page phase mismatch") && message.contains("CheckedOutWriter")));
    while writer.close_step().unwrap().is_some() {}
    assert_eq!(ledger_witness(), before);
}

#[test]
fn db_io_one_byte_high_capacity_candidate_is_rejected_with_exact_owner() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let operation = db_io_operation_reserve(DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();
    let mut reservation = DbIoDriverReservation::try_reserve(operation, DB_IO_OPERATION_BYTES as usize).unwrap();
    let mut candidate = Vec::with_capacity(DB_IO_OPERATION_BYTES as usize + 1);
    candidate.push(0x5a);
    let identity = candidate.as_ptr();
    let capacity = candidate.capacity();
    let error = reservation.observe_capacity(candidate.capacity()).unwrap_err();
    assert!(matches!(error, DbError::LimitExceeded("DB I/O external driver allocation capacity")));
    assert_eq!(candidate.as_ptr(), identity);
    assert_eq!(candidate.capacity(), capacity);
    assert_eq!(candidate, [0x5a]);
    drop(candidate);
    reservation.close_step().unwrap();
    db_io_operation_return(operation, DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();
    assert_eq!(ledger_witness(), before);
}

#[test]
fn db_io_list_capacity_plus_one_does_not_mutate_the_fixed_owner() {
    let _owner = fixture_owner();
    let mut list = DbIoU64List::new();
    for value in 0..DB_IO_LIST_ITEMS as u64 {
        list.push(value).unwrap();
    }
    assert!(list.push(DB_IO_LIST_ITEMS as u64).is_err());
    assert_eq!(list.len(), DB_IO_LIST_ITEMS);
    assert_eq!(list.as_slice().last(), Some(&((DB_IO_LIST_ITEMS - 1) as u64)));
    while list.close_step() {}
    assert!(list.terminal_is_empty());
}

#[test]
fn db_io_list_keeps_exact_capacity_off_worker_stacks_and_in_the_ledger() {
    let _owner = fixture_owner();
    let mut task = DbIoTask::WalList { backend: DbIoBackendControl::Memory { slot: 0, generation: 1 }, document: DbIoText::try_from_str("budget-witness").unwrap(), output: DbIoU64List::new() };
    assert!(matches!(&task, DbIoTask::WalList { output, .. } if output.values.is_none()));
    let credit = task.aggregate_credit();
    assert_eq!(credit.bytes, DB_IO_TASK_SLOT_BYTES + DB_IO_LIST_TRANSIENT_BYTES);
    assert_eq!(credit.items, DB_IO_LIST_ITEMS * 2 + 1);
    assert!(db_io_credit_within_limits(credit, false));
    assert!(db_io_credit_within_limits(DbIoCredit { pages: DB_IO_OPERATION_PAGES, bytes: DB_IO_OPERATION_BYTES, items: DB_IO_OPERATION_ITEM_CREDIT, controls: DB_IO_OPERATION_CONTROL_CREDIT }, false,));
    assert!(!db_io_credit_within_limits(DbIoCredit { pages: DB_IO_OPERATION_PAGES, bytes: DB_IO_OPERATION_BYTES + 1, items: DB_IO_OPERATION_ITEM_CREDIT, controls: DB_IO_OPERATION_CONTROL_CREDIT }, false,));
    let before = ledger_witness();
    let operation = db_io_operation_reserve(credit).unwrap();
    let during = ledger_witness();
    assert_eq!(during.0, before.0.checked_add(credit).unwrap());
    assert_eq!(during.1, before.1 + 1);
    task.admit_list_backing().unwrap();
    assert!(matches!(&task, DbIoTask::WalList { output, .. } if output.values.as_deref().map(<[u64]>::len) == Some(DB_IO_LIST_ITEMS)));
    task.release_unstarted_list_backing();
    assert!(matches!(&task, DbIoTask::WalList { output, .. } if output.values.is_none()));
    task.admit_list_backing().unwrap();
    let mut source = DbIoU64List::new();
    source.push(1).unwrap();
    assert_eq!(source.values.as_deref().map(<[u64]>::len), Some(DB_IO_LIST_ITEMS));
    assert_eq!(DB_IO_LIST_TRANSIENT_BYTES, 2 * DB_IO_LIST_ITEMS as u64 * size_of::<u64>() as u64);
    while source.close_step() {}
    db_io_operation_return(operation, credit).unwrap();
    assert_eq!(ledger_witness(), before);
    assert!(size_of::<DbIoU64List>() <= size_of::<Option<Box<[u64]>>>() + size_of::<Option<DbIoResultHandback>>() + size_of::<u64>(), "list capacity stays boxed off the worker stack");
    assert!(size_of::<DbIoTask>() <= 4 * 1024);
    assert!(size_of::<DbIoResult>() <= 4 * 1024);
    assert!(size_of::<DbIoTaskSlot>() <= 8 * 1024);
    assert!(size_of::<DbIoLostOwner>() <= 4 * 1024);
}

#[test]
fn db_io_result_page_reservation_plus_one_returns_the_writer() {
    let _owner = fixture_owner();
    let mut writer = DbIoPageWriter::try_reserve(1).unwrap();
    assert_eq!(writer.write_fragment(&[0x44; DB_IO_PAGE_BYTES]).unwrap(), DB_IO_PAGE_BYTES);
    assert!(matches!(writer.write_fragment(&[0x55]), Err(DbError::LimitExceeded(_))));
    assert_eq!(writer.len(), DB_IO_PAGE_BYTES);
    assert_eq!(writer.close_step().unwrap(), Some(DB_IO_PAGE_BYTES));
    assert!(!writer.terminal_is_empty());
    assert_eq!(writer.close_step().unwrap(), Some(0));
    assert!(writer.terminal_is_empty());
    assert_eq!(writer.close_step().unwrap(), None);
}
