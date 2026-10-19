use super::*;

use db_io_core::{DB_IO_LEDGER_OWNER, DB_IO_OPERATION_PAGES, DB_IO_PAGE_BYTES, DbIoCredit, DbIoLedgerOwner, DbIoPagePhase, DbIoPageWriter, DbIoPages, PayloadStorage as _, db_io_ledger_census};

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

fn drain_pages(mut pages: DbIoPages) {
    while pages.close_step().unwrap().is_some() {}
    while db_io_page_maintenance_step().unwrap().is_some() {}
}

#[test]
fn db_io_page_writer_seal_memory_sqlite_neo_state_wal_index_max_cancel_fault_drop_is_one_opportunity() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let waker = std::task::Waker::noop();
    let context = &mut std::task::Context::from_waker(waker);

    let mut writer = DbIoPageWriter::try_reserve(2).unwrap();
    assert_eq!(writer.write_fragment(&vec![0x41; DB_IO_PAGE_BYTES + 1]).unwrap(), DB_IO_PAGE_BYTES);
    assert_eq!(writer.write_fragment(&[0x42]).unwrap(), 1);
    let mut seal = Box::pin(writer.seal_retained());
    assert!(matches!(Future::poll(seal.as_mut(), context), std::task::Poll::Pending));
    drop(seal);
    for _ in 0..3 * (DB_IO_LOST_OWNER_SLOTS + 2 * DB_IO_LOST_OWNER_OVERFLOW_SLOTS) {
        if !db_io_lost_owner_close_step().unwrap() { break; }
    }
    assert!(!db_io_lost_owner_close_step().unwrap());
    assert_eq!(ledger_witness(), before);

    let mut writer = DbIoPageWriter::try_reserve(DB_IO_OPERATION_PAGES).unwrap();
    assert_eq!(writer.write_fragment(&[0x43]).unwrap(), 1);
    let mut seal = Box::pin(writer.seal_retained());
    let mut pending = 0usize;
    let pages = loop {
        match Future::poll(seal.as_mut(), context) {
            std::task::Poll::Pending => pending += 1,
            std::task::Poll::Ready(Ok(pages)) => break pages,
            std::task::Poll::Ready(Err(rejected)) => panic!("retained seal faulted: {}", rejected.error()),
        }
    };
    assert!(pending > DB_IO_OPERATION_PAGES);
    drain_pages(pages);
    assert!(DbIoPageWriter::try_reserve(DB_IO_OPERATION_PAGES + 1).is_err());

    let writer = DbIoPageWriter::try_reserve(1).unwrap();
    writer.transition(DbIoPagePhase::CheckedOutWriter, DbIoPagePhase::Queued).unwrap();
    let mut seal = Box::pin(writer.seal_retained());
    let rejected = match Future::poll(seal.as_mut(), context) {
        std::task::Poll::Ready(Err(rejected)) => rejected,
        _ => panic!("invalid-phase seal did not retain its typed fault owner"),
    };
    let mut writer = rejected.into_writer().unwrap();
    while writer.close_step().unwrap().is_some() {}
    assert_eq!(ledger_witness(), before);
}

#[test]
fn db_io_process_and_operation_ledger_return_to_exact_prior_witness() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let owner = pages(&[0x41; DB_IO_PAGE_BYTES + 1]);
    let during = ledger_witness();
    assert_eq!(during.0.pages, before.0.pages + 2);
    assert_eq!(during.0.bytes, before.0.bytes + (2 * DB_IO_PAGE_BYTES) as u64);
    assert_eq!(during.0.items, before.0.items + 3);
    assert_eq!(during.0.controls, before.0.controls + 1);
    drain_pages(owner);
    assert_eq!(ledger_witness(), before);
}

#[test]
fn db_io_range_moves_the_same_page_leases_without_suffix_copy() {
    let _owner = fixture_owner();
    let bytes = vec![0x33; DB_IO_PAGE_BYTES + 3];
    let owner = pages(&bytes);
    let operation = owner.operation();
    let suffix = owner.try_range(DB_IO_PAGE_BYTES).unwrap();
    assert_eq!(suffix.operation(), operation);
    assert_eq!(suffix, [0x33; 3]);
    drain_pages(suffix);
}

#[test]
fn db_io_maintenance_rotates_ready_and_faulted_classes_without_starvation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🔐️writer/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let row = &fixture["maintenanceFairness"];
    assert_eq!(DB_IO_MAINTENANCE_CLASSES, row["classes"].as_array().unwrap().len());
    for (name, faults) in [("continuouslyReady", false), ("firstClassFaults", true)] {
        let cursor = std::sync::atomic::AtomicUsize::new(0);
        let mut trace = Vec::new();
        for _ in row[name].as_array().unwrap() {
            let mut attempted = 0;
            let result = db_io_maintenance_turn(&cursor, |class| {
                attempted += 1;
                trace.push(class);
                if faults && class == 0 {
                    return Err(DbError::Unavailable("retained maintenance fixture fault".to_string()));
                }
                Ok(true)
            });
            assert_eq!(attempted, 1);
            assert_eq!(result.is_err(), faults && trace.last() == Some(&0));
        }
        assert_eq!(serde_json::to_value(trace).unwrap(), row[name]);
    }
    let cursor = std::sync::atomic::AtomicUsize::new(0);
    let mut attempted = 0;
    assert!(!db_io_maintenance_turn(&cursor, |_| {
        attempted += 1;
        Ok(false)
    })
    .unwrap());
    assert_eq!(attempted, DB_IO_MAINTENANCE_CLASSES);
}
