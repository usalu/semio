use super::*;

//#region 🧪️PagedStorageLaws
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🧪️fixture.json")).unwrap() }

#[test]
fn retained_fixed_list_pages_preserve_order_without_all_n_allocation() {
    let data = fixture();
    let grant = data["maximumPageBytes"].as_u64().unwrap() as usize;
    let mut values = UiFixedList::<u64, 600>::default();
    let mut allocations = 0;
    for index in 0..data["ordered"]["count"].as_u64().unwrap() {
        let mut source = Some(index);
        for _ in 0..100 {
            if values.has_reserved_slot() {
                let step = values.try_place_reserved(&mut source, grant).unwrap();
                assert_eq!(step.placed_bytes, 8);
                assert!(source.is_none());
                break;
            }
            let before = values.allocated_bytes();
            let step = values.try_reserve_one(grant).unwrap();
            assert!(step.progressed);
            assert_eq!(values.allocated_bytes() - before, step.allocated_bytes);
            assert!(step.allocated_bytes <= grant);
            allocations += usize::from(step.allocated_bytes != 0);
        }
        assert!(source.is_none());
    }
    assert_eq!(values.iter().copied().sum::<u64>(), data["ordered"]["sum"].as_u64().unwrap());
    let expected: Vec<u64> = (0..600).collect();
    assert_eq!(serde_json::to_value(&values).unwrap(), serde_json::to_value(&expected).unwrap());
    assert_eq!(values.get(599), Some(&599));
    assert!(values.get(600).is_none());
    assert_eq!(values.iter().rev().next(), Some(&599));
    assert!(values.release_empty_page().is_err());
    while values.pop().is_some() {}
    let mut released = 0;
    while !values.terminal_is_empty() {
        let before = values.allocated_bytes();
        let step = values.release_empty_page().unwrap();
        assert!(step.progressed);
        assert_eq!(before - values.allocated_bytes(), step.released_allocation_bytes);
        released += usize::from(step.released_allocation_bytes != 0);
    }
    assert_eq!(released, allocations);
    eprintln!("[DEBUG] fixed-list-pages items=600 allocations={allocations} releases={released} maximum-allocation={grant} wire-order=true");
}

#[test]
fn retained_fixed_list_pages_refuse_oversized_payload_without_losing_owner() {
    let mut values = UiFixedList::<[u8; 32769], 2>::default();
    for _ in 0..10 {
        if values.next_allocation_bytes().unwrap() > 32768 { break; }
        values.try_reserve_one(32768).unwrap();
    }
    let before = values.allocated_bytes();
    let step = values.try_reserve_one(32768).unwrap();
    assert!(!step.progressed);
    assert_eq!(values.allocated_bytes(), before);
    let mut source = Some([7; 32769]);
    assert!(!values.try_place_reserved(&mut source, 32768).unwrap().progressed);
    assert_eq!(source.as_ref().unwrap()[32768], 7);
    assert!(values.is_empty());
    while !values.terminal_is_empty() { values.release_empty_page().unwrap(); }
    eprintln!("[DEBUG] fixed-list-oversized source-retained=true allocated-after-refusal={before} placed=false");
}

#[test]
fn retained_fixed_list_pages_admit_binding_sized_payloads_and_safe_mutable_iteration() {
    let data = fixture();
    let mut values = UiFixedList::<[u8; 2072], 32>::default();
    for index in 0..data["binding"]["count"].as_u64().unwrap() {
        while !values.has_reserved_slot() {
            let step = values.try_reserve_one(4096).unwrap();
            assert!(step.progressed);
            assert!(step.allocated_bytes <= 4096);
        }
        let mut source = Some([index as u8; 2072]);
        assert!(!values.try_place_reserved(&mut source, data["binding"]["smallGrantBytes"].as_u64().unwrap() as usize).unwrap().progressed);
        assert_eq!(source.as_ref().unwrap()[2071], index as u8);
        assert_eq!(values.try_place_reserved(&mut source, 4096).unwrap().placed_bytes, 2072);
    }
    let mut iterator = values.iter_mut();
    let first = iterator.next().unwrap();
    let second = iterator.next().unwrap();
    first[0] = 101;
    second[0] = 102;
    assert_eq!(iterator.count(), 30);
    assert_eq!(values.get(0).unwrap()[0], 101);
    assert_eq!(values.get(1).unwrap()[0], 102);
    let allocated = values.allocated_bytes();
    assert!(allocated > 32 * 2072, "metadata backing is not erased from the resident count");
    while values.pop().is_some() {}
    while !values.terminal_is_empty() { values.release_empty_page().unwrap(); }
    eprintln!("[DEBUG] fixed-list-binding-pages items=32 bytes-per-placement=2072 total-with-metadata={allocated} safe-mutable-iteration=true");
}
#[test]
fn retained_fixed_list_pages_zero_zst_and_empty_tail_reuse_preserve_exact_storage() {
    let data = fixture();
    let data = &data["edgeCases"];
    let mut zero = UiFixedList::<u8, 0>::default();
    assert_eq!(zero.capacity(), data["zeroCapacity"].as_u64().unwrap() as usize);
    assert!(zero.next_allocation_bytes().is_err());
    assert!(zero.try_reserve_one(4096).is_err());
    let mut byte = Some(9);
    assert!(!zero.try_place_reserved(&mut byte, 4096).unwrap().progressed);
    assert_eq!(byte, Some(9));
    assert!(zero.terminal_is_empty());
    let mut zst = UiFixedList::<(), 7>::default();
    while !zst.has_reserved_slot() { assert!(zst.try_reserve_one(4096).unwrap().progressed); }
    for _ in 0..data["zeroSizedCount"].as_u64().unwrap() {
        let mut owner = Some(());
        let step = zst.try_place_reserved(&mut owner, 0).unwrap();
        assert!(step.progressed && owner.is_none());
        assert_eq!(step.placed_bytes, 0);
    }
    assert!(zst.release_empty_page().is_err());
    while zst.pop().is_some() {}
    while !zst.terminal_is_empty() { assert!(zst.release_empty_page().unwrap().progressed); }
    let mut values = UiFixedList::<u64, 600>::default();
    for index in 0..data["tailCount"].as_u64().unwrap() {
        while !values.has_reserved_slot() { values.try_reserve_one(4096).unwrap(); }
        values.try_place_reserved(&mut Some(index), 4096).unwrap();
    }
    let retained = data["retainedPrefix"].as_u64().unwrap() as usize;
    while values.len() > retained { values.pop(); }
    let first_pointer = values.get(0).unwrap() as *const u64;
    let before = values.allocated_bytes();
    let payload_release = values.release_empty_page().unwrap();
    assert_eq!(payload_release.released_allocation_bytes, (600 - retained) * size_of::<u64>());
    assert_eq!(before - values.allocated_bytes(), payload_release.released_allocation_bytes);
    let metadata_release = values.release_empty_page().unwrap();
    assert!(metadata_release.progressed && metadata_release.released_allocation_bytes != 0);
    assert!(values.release_empty_page().is_err());
    assert_eq!(values.get(0).unwrap() as *const u64, first_pointer);
    while !values.has_reserved_slot() { values.try_reserve_one(4096).unwrap(); }
    let mut source = Some(data["reuseValue"].as_u64().unwrap());
    values.try_place_reserved(&mut source, 4096).unwrap();
    assert_eq!(values.get(retained), Some(&900));
    assert_eq!(values.get(0).unwrap() as *const u64, first_pointer);
    while values.pop().is_some() {}
    while !values.terminal_is_empty() { values.release_empty_page().unwrap(); }
    eprintln!("[DEBUG] fixed-list-edges zero=true zst=7 retained-prefix=512 empty-tail-released=true reuse=true");
}
//#endregion 🧪️PagedStorageLaws
