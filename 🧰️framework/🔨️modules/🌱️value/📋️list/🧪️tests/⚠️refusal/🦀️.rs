use super::*;

struct RefusedAllocation;
unsafe impl PageAllocation for RefusedAllocation {
    fn reserve<T>(_: &mut Vec<T>, _: usize, actual:Option<&mut Option<usize>>) -> Result<(), std::collections::TryReserveError> {
        if let Some(marker)=actual{*marker=None}Err(Vec::<u8>::new().try_reserve_exact(usize::MAX).unwrap_err())
    }
}
struct ExcessAllocation;
unsafe impl PageAllocation for ExcessAllocation {
    fn reserve<T>(owner: &mut Vec<T>, slots: usize, actual:Option<&mut Option<usize>>) -> Result<(), std::collections::TryReserveError> {
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
        let refusal = if operation == "extraRelease" { list.release_empty_page_granted(usize::MAX).unwrap_err() } else { list.next_release_allocation_bytes().unwrap_err() };
        assert_eq!(list.pop(), Some(7));
        while !list.terminal_is_empty() { let bytes = list.next_release_allocation_bytes().unwrap(); assert!(list.release_empty_page_granted(bytes).unwrap().progressed); }
        return refusal;
    }
    if operation.ends_with("Payload") { list.reserve_one_granted(list.next_allocation_bytes().unwrap(),&mut None).unwrap(); }
    let requested = list.next_allocation_bytes().unwrap();
    let physical = list.allocated_bytes();
    let mut actual=None;let error = match operation {
        "counter" => { list.allocated = isize::MAX as usize; list.reserve_one_granted(requested,&mut None).unwrap_err() },
        "failMetadata" | "failPayload" => list.reserve_one_using_granted::<RefusedAllocation>(requested,&mut actual).unwrap_err(),
        "extraMetadata" | "extraPayload" => list.reserve_one_using_granted::<ExcessAllocation>(requested,&mut actual).unwrap_err(),
        "signedMetadata" | "signedPayload" => { list.allocated = isize::MAX as usize - requested; list.reserve_one_using_granted::<ExcessAllocation>(requested * 2,&mut actual).unwrap_err() },
        _ => panic!("closed paged refusal operation"),
    };
    if operation.starts_with("fail"){assert_eq!(actual,None)}else if operation!="counter"{assert_eq!(actual,Some(error.allocated_bytes))}list.allocated = physical + error.allocated_bytes;
    let refusal = error.refusal();
    while !list.terminal_is_empty() {
        let bytes = list.next_release_allocation_bytes().unwrap();
        assert!(list.release_empty_page_granted(bytes).unwrap().progressed);
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


fn test_reserve<T>(owner:&mut Vec<T>,slots:usize,mut actual:Option<&mut Option<usize>>)->Result<(),std::collections::TryReserveError>{let allocating=std::mem::size_of::<T>()!=0&&owner.len().saturating_add(slots)>owner.capacity();if allocating{if let Some(marker)=actual.as_deref_mut(){*marker=Some(0)}}let result=owner.try_reserve_exact(slots);if allocating&&result.is_ok(){if let Some(marker)=actual.as_deref_mut(){*marker=Some(owner.capacity()*std::mem::size_of::<T>())}}result}
