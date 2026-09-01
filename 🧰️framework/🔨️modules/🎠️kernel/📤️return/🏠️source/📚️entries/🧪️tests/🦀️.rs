//#region 🧪️ReturnSourceEntries
use super::return_source_entries::{ReturnSourceEntries, ReturnSourceEntry};

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧪️fixture/🔣️.json")).unwrap() }

fn release_entry<T>(entry: &mut Option<ReturnSourceEntry<T>>) -> Option<T> {
    let mut value = None;
    let owner = entry.as_mut().unwrap();
    owner.take_value_into(&mut value, usize::MAX).unwrap();
    assert!(owner.close_empty_step(1, usize::MAX).unwrap().complete);
    drop(entry.take());
    value
}

#[test]
fn return_source_entries_reserve_before_placement_and_preserve_original_allocation() {
    let mut entries = ReturnSourceEntries::<Vec<u8>>::new(1);
    let bytes = ReturnSourceEntries::<Vec<u8>>::required_allocation_bytes();
    assert_eq!(entries.allocated_bytes(), 0);
    assert!(!entries.reserve_step(bytes - 1).unwrap().ready);
    assert_eq!(entries.allocated_bytes(), 0);
    let reserved = entries.reserve_step(bytes).unwrap();
    assert!(reserved.ready && reserved.allocated_bytes >= bytes);
    assert_eq!(entries.reserve_step(bytes).unwrap().allocated_bytes, 0);
    let mut source = Some(vec![17; 8193]);
    let pointer = source.as_ref().unwrap().as_ptr();
    assert_eq!(entries.try_push_reserved(&mut source, 0).unwrap(), 0);
    assert_eq!(source.as_ref().unwrap().as_ptr(), pointer);
    assert!(entries.try_push_reserved(&mut source, ReturnSourceEntries::<Vec<u8>>::required_placement_bytes()).unwrap() > 0);
    assert!(source.is_none());
    assert!(entries.reserve_step(usize::MAX).is_err());
    while !entries.freeze_step(1, 4096).unwrap().complete {}
    let mut entry = None;
    assert!(entries.take_front_into(&mut entry, 4096).unwrap());
    assert_eq!(entry.as_ref().unwrap().value().unwrap().as_ptr(), pointer);
    assert!(!entry.as_mut().unwrap().close_empty_step(1, 4096).unwrap().complete);
    let value = release_entry(&mut entry).unwrap();
    assert_eq!(value.as_ptr(), pointer);
    assert_eq!(value.len(), 8193);
    assert!(entries.terminal_is_empty());
}

#[test]
fn return_source_entries_incremental_freeze_preserves_exact_fifo_and_handoff() {
    let fixture = fixture();
    let expected: Vec<u64> = serde_json::from_value(fixture["fifo"].clone()).unwrap();
    let mut entries = ReturnSourceEntries::<u64>::new(fixture["maximumEntries"].as_u64().unwrap() as usize);
    for value in &expected {
        entries.reserve_step(ReturnSourceEntries::<u64>::required_allocation_bytes()).unwrap();
        entries.try_push_reserved(&mut Some(*value), ReturnSourceEntries::<u64>::required_placement_bytes()).unwrap();
    }
    let zero = entries.freeze_step(0, 4096).unwrap();
    assert_eq!((zero.advanced_items, zero.copied_bytes), (0, 0));
    let zero = entries.freeze_step(1, 0).unwrap();
    assert_eq!((zero.advanced_items, zero.copied_bytes), (0, 0));
    for _ in 0..expected.len() + 1 {
        let step = entries.freeze_step(1, 4096).unwrap();
        assert!(step.advanced_items <= 1 && step.copied_bytes <= 4096);
        if step.complete { break; }
    }
    let mut actual = Vec::new();
    let mut entry = None;
    while entries.take_front_into(&mut entry, 4096).unwrap() {
        assert!(entries.take_front_into(&mut entry, 4096).is_err());
        actual.push(release_entry(&mut entry).unwrap());
    }
    assert_eq!(actual, expected);
    assert!(entries.terminal_is_empty());
}

#[test]
fn return_source_entries_cancel_keeps_empty_reservation_and_both_freeze_roots() {
    let fixture = fixture();
    let values: Vec<u64> = serde_json::from_value(fixture["fifo"].clone()).unwrap();
    for reverse_count in [0, fixture["cancelAfterFreezeItems"].as_u64().unwrap() as usize] {
        type Payload = (u64, Vec<u8>);
        let mut entries = ReturnSourceEntries::<Payload>::new(values.len() + 1);
        let mut expected = Vec::new();
        for value in &values[..values.len() - 1] {
            let bytes = vec![*value as u8; 64];
            expected.push((*value, bytes.as_ptr() as usize, bytes.len()));
            entries.reserve_step(ReturnSourceEntries::<Payload>::required_allocation_bytes()).unwrap();
            entries.try_push_reserved(&mut Some((*value, bytes)), ReturnSourceEntries::<Payload>::required_placement_bytes()).unwrap();
        }
        let bytes = vec![23; fixture["unwindPayloadBytes"].as_u64().unwrap() as usize];
        let value = *values.last().unwrap();
        expected.push((value, bytes.as_ptr() as usize, bytes.len()));
        let mut source = Some((value, bytes));
        entries.reserve_step(ReturnSourceEntries::<Payload>::required_allocation_bytes()).unwrap();
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            assert!(entries.try_push_reserved(&mut source, ReturnSourceEntries::<Payload>::required_placement_bytes()).unwrap() > 0);
            if reverse_count == 0 { entries.reserve_step(ReturnSourceEntries::<Payload>::required_allocation_bytes()).unwrap(); }
            for _ in 0..reverse_count { entries.freeze_step(1, 4096).unwrap(); }
            panic!("fixture producer failed after owned placement");
        }));
        assert!(caught.is_err());
        assert!(source.is_none());
        entries.begin_close();
        let mut actual = Vec::new();
        let mut entry = None;
        while entries.take_close_entry_into(&mut entry, 4096).unwrap() {
            if let Some((value, bytes)) = release_entry(&mut entry) { actual.push((value, bytes.as_ptr() as usize, bytes.len())); }
        }
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
        assert!(entries.terminal_is_empty());
        assert_eq!(entries.allocated_bytes(), 0);
    }
}

#[test]
fn return_source_entries_over_admission_reports_and_retains_exact_empty_backing() {
    let mut entries = ReturnSourceEntries::<u64>::new(1);
    let required = ReturnSourceEntries::<u64>::required_allocation_bytes();
    let error = entries.reserve_step_with_capacity_for_test(required, 2).unwrap_err();
    assert!(error.allocated_bytes > required);
    assert_eq!(entries.allocated_bytes(), error.allocated_bytes as u128);
    let mut source = Some(7);
    assert!(entries.try_push_reserved(&mut source, usize::MAX).is_err());
    assert_eq!(source, Some(7));
    entries.begin_close();
    let mut entry = None;
    assert!(entries.take_close_entry_into(&mut entry, 4096).unwrap());
    assert_eq!(entries.allocated_bytes(), 0);
    assert_eq!(entry.as_ref().unwrap().allocated_bytes(), error.allocated_bytes);
    assert!(entry.as_mut().unwrap().close_empty_step(0, usize::MAX).unwrap().complete == false);
    assert!(entry.as_mut().unwrap().close_empty_step(1, error.allocated_bytes - 1).unwrap().complete == false);
    let step = entry.as_mut().unwrap().close_empty_step(1, error.allocated_bytes).unwrap();
    assert!(step.complete);
    assert_eq!(step.released_bytes, error.allocated_bytes);
    drop(entry.take());
    assert!(entries.terminal_is_empty());
}
//#endregion 🧪️ReturnSourceEntries
