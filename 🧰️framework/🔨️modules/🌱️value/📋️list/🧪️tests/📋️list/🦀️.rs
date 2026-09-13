use super::*;

#[test]
fn retained_paged_list_neutral_order_capacity_and_close() {
    let data: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let grant = data["maximumPageBytes"].as_u64().unwrap() as usize;
    let mut list = PagedList::<u64, 600>::default();
    let mut admitted = 0;
    for index in 0..data["ordered"]["count"].as_u64().unwrap() {
        while !list.has_reserved_slot() {
            let step = list.reserve_one(grant).unwrap();
            assert!(step.progressed);
            assert!(step.allocated_bytes <= grant);
            admitted += step.allocated_bytes;
        }
        assert_eq!(list.push_reserved(index), Ok(()));
        assert_eq!(list.initialized_len(), list.len());
    }
    let expected: Vec<u64> = (0..600).collect();
    assert_eq!(serde_json::to_value(list.iter().copied().collect::<Vec<_>>()).unwrap(), serde_json::to_value(expected).unwrap());
    assert_eq!(list.iter().copied().sum::<u64>(), data["ordered"]["sum"].as_u64().unwrap());
    assert_eq!(list.push_reserved(data["capacity"]["rejected"].as_u64().unwrap()), Err(601));
    let before = list.allocated_bytes();
    assert!(list.release_empty_page(grant).is_err());
    assert_eq!(list.allocated_bytes(), before);
    while list.pop().is_some() { assert_eq!(list.initialized_len(), list.len()); }
    assert!(!list.release_empty_page(0).unwrap().progressed);
    assert_eq!(list.allocated_bytes(), before);
    let mut released = 0;
    while !list.terminal_is_empty() {
        let exact = list.next_release_allocation_bytes().unwrap();
        let step = list.release_empty_page(exact).unwrap();
        assert!(step.progressed);
        assert_eq!(step.released_allocation_bytes, exact);
        released += step.released_allocation_bytes;
    }
    assert_eq!(released, admitted);
    assert_eq!(list.len(), 0);
    assert_eq!(list.capacity(), 0);
    assert_eq!(list.allocated_bytes(), 0);
    assert_eq!(list.root.capacity(), 0);
    eprintln!("[DEBUG] Neutral paged list matched Serde order, exact capacity refusal and all allocated backing releases");
}

#[test]
fn retained_paged_list_capacity_admission_and_exact_release_grants() {
    let mut list = PagedList::<u64, 600>::default();
    while list.capacity() < 600 {
        let exact = list.next_capacity_allocation_bytes(600).unwrap().expect("target still needs backing");
        assert!(exact > 0, "a target beyond current capacity must request its next real backing even while the current leaf has unused slots");
        let before = (list.capacity(), list.allocated_bytes());
        assert!(!list.reserve_capacity_one(600, exact - 1).unwrap().progressed);
        assert_eq!((list.capacity(), list.allocated_bytes()), before);
        let step = list.reserve_capacity_one(600, exact).unwrap();
        assert!(step.progressed);
        assert_eq!(step.allocated_bytes, exact);
    }
    assert_eq!(list.next_capacity_allocation_bytes(600), Ok(None));
    assert_eq!(list.len(), 0);
    assert_eq!(list.initialized_len(), 0);
    assert!(list.reserve_capacity_one(601, 4096).is_err());
    assert_eq!(list.capacity(), 600);
    let pointer = list.backing_ptr(512).unwrap();
    let exact = list.next_release_allocation_bytes().unwrap();
    assert_eq!(exact, 88 * size_of::<u64>());
    assert!(!list.release_empty_page(exact - 1).unwrap().progressed);
    assert_eq!(list.backing_ptr(512).unwrap(), pointer);
    let released = list.release_empty_page(88 * size_of::<u64>()).unwrap();
    assert_eq!(released.released_allocation_bytes, 88 * size_of::<u64>());
    assert_eq!(list.capacity(), 512);
    while !list.terminal_is_empty() {
        let exact = list.next_release_allocation_bytes().unwrap();
        list.release_empty_page(exact).unwrap();
    }
    assert_eq!((list.root.capacity(), list.len(), list.capacity(), list.allocated_bytes()), (0, 0, 0, 0));
    eprintln!("[DEBUG] Neutral paged list pre-admitted multiple backing pages and retained exact tail below its grant");
}
