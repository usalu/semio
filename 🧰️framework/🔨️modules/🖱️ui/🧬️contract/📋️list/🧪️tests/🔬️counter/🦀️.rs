use super::*;

#[test]
fn retained_fixed_list_pages_counter_refuses_unaddressable_ownership_before_allocation() {
    let mut list = PagedList::<u64, 1>::default();
    list.allocated = isize::MAX as usize;
    let result = list.reserve_one(4096);
    let allocated = list.root.capacity() * size_of::<Page<u64>>();
    list.allocated = allocated;
    while !list.terminal_is_empty() {
        list.release_empty_page().unwrap();
    }
    assert!(result.is_err());
    assert_eq!(allocated, 0, "counter rejection must precede a new physical allocation");
}

struct Overallocated;
impl PageAllocation for Overallocated {
    fn reserve<T>(owner: &mut Vec<T>, slots: usize) -> Result<(), std::collections::TryReserveError> {
        owner.try_reserve_exact(slots * 2)
    }
}

#[test]
fn retained_fixed_list_pages_counter_keeps_actual_failed_allocation_until_release() {
    let data: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let mut list = PagedList::<u64, 1>::default();
    let requested = list.next_allocation_bytes().unwrap();
    let error = list.reserve_page_using::<Overallocated>(requested).unwrap_err();
    assert_eq!(error.allocated_bytes, requested * data["counter"]["allocatorMultiplier"].as_u64().unwrap() as usize);
    assert_eq!(list.allocated_bytes(), error.allocated_bytes);
    assert!(!list.terminal_is_empty());
    let released = list.release_empty_page().unwrap();
    assert_eq!(released.released_allocation_bytes, error.allocated_bytes);
    assert!(list.terminal_is_empty());
    let mut list = PagedList::<u64, 1>::default();
    list.reserve_one(requested).unwrap();
    let before = list.allocated_bytes();
    let error = list.reserve_page_using::<Overallocated>(size_of::<u64>()).unwrap_err();
    assert_eq!(list.allocated_bytes() - before, error.allocated_bytes);
    assert_eq!(error.allocated_bytes, 2 * size_of::<u64>());
    assert!(list.has_reserved_slot());
    let step = list.release_empty_page().unwrap();
    assert_eq!(step.released_allocation_bytes, error.allocated_bytes);
    assert_eq!(list.allocated_bytes(), before);
    list.release_empty_page().unwrap();
    assert!(list.terminal_is_empty());
    eprintln!("[DEBUG] fixed-list-allocation-error metadata-and-payload actual-capacity-retained=true released-exact=true");
}
