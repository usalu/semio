use super::*;

struct RefusedAllocation;
unsafe impl PageAllocation for RefusedAllocation {
    fn reserve<T>(_: &mut Vec<T>, _: usize, actual:&mut Option<usize>) -> Result<(), PagedListError> {
        *actual=None;Err(PagedListError{kind:PagedListRefusalKind::AllocationFailed,reason:"synthetic allocation refusal"})
    }
}
struct ExcessAllocation;
unsafe impl PageAllocation for ExcessAllocation {
    fn reserve<T>(owner: &mut Vec<T>, slots: usize, actual:&mut Option<usize>) -> Result<(), PagedListError> {
        test_reserve(owner,slots * 2,actual)
    }
}
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixtures/⚠️refusal/🔣️.json")).unwrap() }
fn run(operation: &str) -> PagedListError {
    if operation == "nextEmpty" { return PagedList::<u64, 0>::default().next_allocation_bytes().unwrap_err(); }
    let mut list = PagedList::<u64, 1>::default();
    if operation == "capacity" { return list.next_capacity_allocation_bytes(2).unwrap_err(); }
    if operation == "extraRetireEmpty" { return list.truncate_retired_last().unwrap_err(); }
    if operation == "extraRelease" || operation == "extraReleaseNext" {
        list.reserve_full().unwrap(); list.push_reserved(7).unwrap();
        let refusal = if operation == "extraRelease" { list.release_empty_page(usize::MAX).unwrap_err() } else { list.next_release_allocation_bytes().unwrap_err() };
        assert_eq!(list.pop(), Some(7));
        while !list.terminal_is_empty() { let bytes = list.next_release_allocation_bytes().unwrap(); assert!(list.release_empty_page(bytes).unwrap().progressed); }
        return refusal;
    }
    if operation.ends_with("Payload") { list.reserve_one(list.next_allocation_bytes().unwrap(),&mut None).unwrap(); }
    let requested = list.next_allocation_bytes().unwrap();
    let physical = list.allocated_bytes();
    let mut actual=None;let error = match operation {
        "counter" => { list.allocated = isize::MAX as usize; list.reserve_one(requested,&mut None).unwrap_err() },
        "failMetadata" | "failPayload" => list.reserve_page_using::<RefusedAllocation>(requested,&mut actual).unwrap_err(),
        "extraMetadata" | "extraPayload" => list.reserve_page_using::<ExcessAllocation>(requested,&mut actual).unwrap_err(),
        "signedMetadata" | "signedPayload" => { list.allocated = isize::MAX as usize - requested; list.reserve_page_using::<ExcessAllocation>(requested * 2,&mut actual).unwrap_err() },
        _ => panic!("closed paged refusal operation"),
    };
    if operation.starts_with("fail"){assert_eq!(actual,None)}else if operation!="counter"{assert_eq!(actual,Some(error.allocated_bytes))}list.allocated = physical + error.allocated_bytes;
    let refusal = error.refusal();
    while !list.terminal_is_empty() {
        let bytes = list.next_release_allocation_bytes().unwrap();
        assert!(list.release_empty_page(bytes).unwrap().progressed);
    }
    assert_eq!(list.allocated_bytes(), 0);
    refusal
}

#[test]
fn controlled_value_paged_refusal_actual_allocation_kind_corpus() {
    for row in fixture()["cases"].as_array().unwrap() {
        let refusal = run(row["operation"].as_str().unwrap());
        assert_eq!(refusal.kind.as_str(), row["expected"]["kind"].as_str().unwrap(), "{}", row["id"]);
        assert_eq!(refusal.reason, row["expected"]["reason"].as_str().unwrap(), "{}", row["id"]);
    }
}


fn test_reserve<T>(owner:&mut Vec<T>,slots:usize,actual:&mut Option<usize>)->Result<(),PagedListError>{*actual=None;let needed=owner.len().checked_add(slots).ok_or(PagedListError{kind:PagedListRefusalKind::OwnershipLimit,reason:"test reservation overflow"})?;let allocating=std::mem::size_of::<T>()!=0&&needed>owner.capacity();if allocating{*actual=Some(0)}owner.try_reserve_exact(slots).map_err(|_|PagedListError{kind:PagedListRefusalKind::AllocationFailed,reason:"test allocation failed"})?;if allocating{*actual=Some(owner.capacity()*std::mem::size_of::<T>())}Ok(())}
