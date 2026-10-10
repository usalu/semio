use super::*;

#[test]
fn retained_fixed_list_pages_counter_refuses_unaddressable_ownership_before_allocation() {
    let mut list = PagedList::<u64, 1>::default();
    list.allocated = isize::MAX as usize;
    let result = list.reserve_one(4096);
    let allocated = list.root.capacity() * size_of::<Page<u64>>();
    list.allocated = allocated;
    while !list.terminal_is_empty() {
        let exact = list.next_release_allocation_bytes().unwrap();
        list.release_empty_page(exact).unwrap();
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
fn retained_fixed_list_pages_counter_rejects_actual_signed_limit_and_preserves_release_owner() {
    let data: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let multiplier = data["counter"]["allocatorMultiplier"].as_u64().unwrap() as usize;
    for payload in [false, true] {
        let mut list = PagedList::<u64, 1>::default();
        if payload {
            list.reserve_one(list.next_allocation_bytes().unwrap()).unwrap();
        }
        let physical_before = list.allocated_bytes();
        let requested = list.next_allocation_bytes().unwrap();
        let seeded_before = isize::MAX as usize - requested;
        list.allocated = seeded_before;
        let error = list.reserve_page_using::<Overallocated>(requested * multiplier).unwrap_err();
        assert_eq!(error.reason, "fixed list actual allocation exceeds addressable ownership; owner retained");
        assert_eq!(error.allocated_bytes, requested * multiplier);
        assert_eq!(list.allocated_bytes(), seeded_before + error.allocated_bytes);
        let pointer = if payload { list.backing_ptr(0).unwrap().cast::<u8>() } else { list.root.as_ptr().cast::<u8>() };
        assert!(!list.release_empty_page(error.allocated_bytes - 1).unwrap().progressed);
        assert_eq!(list.allocated_bytes(), seeded_before + error.allocated_bytes);
        let retained = if payload { list.backing_ptr(0).unwrap().cast::<u8>() } else { list.root.as_ptr().cast::<u8>() };
        assert_eq!(pointer, retained);
        let released = list.release_empty_page(error.allocated_bytes).unwrap();
        assert_eq!(released.released_allocation_bytes, error.allocated_bytes);
        assert_eq!(list.allocated_bytes(), seeded_before);
        list.allocated = physical_before;
        while !list.terminal_is_empty() {
            let exact = list.next_release_allocation_bytes().unwrap();
            assert!(list.release_empty_page(exact).unwrap().progressed);
        }
        assert_eq!((list.root.capacity(), list.len(), list.capacity(), list.allocated_bytes()), (0, 0, 0, 0));
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
    let exact = list.next_release_allocation_bytes().unwrap();
    let released = list.release_empty_page(exact).unwrap();
    assert_eq!(released.released_allocation_bytes, error.allocated_bytes);
    assert!(list.terminal_is_empty());
    assert_eq!((list.root.capacity(), list.len(), list.capacity(), list.allocated_bytes()), (0, 0, 0, 0));
    let mut list = PagedList::<u64, 1>::default();
    list.reserve_one(requested).unwrap();
    let before = list.allocated_bytes();
    let error = list.reserve_page_using::<Overallocated>(size_of::<u64>()).unwrap_err();
    assert_eq!(list.allocated_bytes() - before, error.allocated_bytes);
    assert_eq!(error.allocated_bytes, 2 * size_of::<u64>());
    assert!(list.has_reserved_slot());
    let exact = list.next_release_allocation_bytes().unwrap();
    let step = list.release_empty_page(exact).unwrap();
    assert_eq!(step.released_allocation_bytes, error.allocated_bytes);
    assert_eq!(list.allocated_bytes(), before);
    let exact = list.next_release_allocation_bytes().unwrap();
    list.release_empty_page(exact).unwrap();
    assert!(list.terminal_is_empty());
    assert_eq!((list.root.capacity(), list.len(), list.capacity(), list.allocated_bytes()), (0, 0, 0, 0));
}

#[test]
fn retained_fixed_list_depth_quotes_original_tail_and_whole_release_frontiers() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let count=fixture["ordered"]["count"].as_u64().unwrap()as usize;
    let tail=fixture["ordered"]["emptyTailRelease"]["tailItems"].as_u64().unwrap()as usize;
    let earlier=fixture["ordered"]["emptyTailRelease"]["earlierLiveItems"].as_u64().unwrap()as usize;
    assert_eq!(tail+earlier,count);
    let mut list=PagedList::<u64,600>::empty();
    let mut oracle=std::collections::VecDeque::new();
    for value in 0..count as u64{
        while !list.has_reserved_slot(){let bytes=list.next_allocation_bytes().unwrap();list.reserve_one(bytes).unwrap();}
        list.push_reserved(value).unwrap();oracle.push_back(value);
    }
    while !oracle.is_empty(){
        let pointer=list.backing_ptr(list.len()-1).unwrap();
        assert_eq!(list.next_pop_depth_demand().unwrap(),list.height()+3);
        assert_eq!(list.next_release_depth_demand().is_err(),oracle.len()>earlier);
        assert_eq!(list.backing_ptr(list.len()-1).unwrap(),pointer);
        assert_eq!(list.pop(),oracle.pop_back());
    }
    assert_eq!(list.next_pop_depth_demand().unwrap(),0);
    while !list.terminal_is_empty(){
        let bytes=list.next_release_allocation_bytes().unwrap();let depth=list.next_release_depth_demand().unwrap();
        assert!(depth>=2);let before=list.allocated_bytes();
        if bytes>0{let denied=list.release_empty_page(bytes-1).unwrap();assert!(!denied.progressed);assert_eq!(list.allocated_bytes(),before);assert_eq!(list.next_release_depth_demand().unwrap(),depth);}
        let released=list.release_empty_page(bytes).unwrap();assert!(released.progressed);assert_eq!(released.released_allocation_bytes,bytes);
    }
    assert_eq!(list.next_release_depth_demand().unwrap(),0);
    println!("[DEBUG] original600 PagedList tail order equals VecDeque; actual retained branch/leaf depth and separate whole backing refusals preserved");
}

#[test]
fn retained_fixed_list_reservation_copy_quotes_actual_original_metadata_and_heap(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️reservation.json")).unwrap();let count=fixture["count"].as_u64().unwrap()as usize;let layout=fixture["layouts"].as_array().unwrap().iter().find(|row|row["pointerBits"].as_u64().unwrap()==usize::BITS as u64).unwrap();assert_eq!(size_of::<Page<u64>>(),layout["pageBytes"].as_u64().unwrap()as usize);assert_eq!(size_of::<Vec<Page<u64>>>(),layout["headerBytes"].as_u64().unwrap()as usize);
 let mut owner=PagedList::<u64,600>::with_payload_page_bytes(fixture["payloadBytes"].as_u64().unwrap()as usize).unwrap();let mut oracle=std::collections::VecDeque::new();let(mut born,mut promotions,mut metadata,mut payload)=(0,0,0,0);
 for value in 0..count as u64{while !owner.has_reserved_slot(){let copy=owner.next_reserve_copy_byte_demand().unwrap();let capacity=owner.next_allocation_bytes().unwrap();let before=(owner.len(),owner.capacity(),owner.allocated_bytes());let before_pointer=owner.root.as_ptr();let grant=crate::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:0,maximum_depth:1};for axis in fixture["deniedAxes"].as_array().unwrap(){let mut denied=grant;match axis.as_str().unwrap(){"items"=>denied.maximum_items=0,"copyBytes"=>denied.maximum_copy_bytes-=1,"capacityBytes"=>denied.maximum_capacity_bytes-=1,_=>denied.maximum_depth=0};let(result,physical)=crate::observe_retirement_allocations(||owner.reserve_one_funded(denied).unwrap());assert_eq!(result,Default::default());assert_eq!(physical,(0,0));assert_eq!((owner.len(),owner.capacity(),owner.allocated_bytes()),before);assert_eq!(owner.root.as_ptr(),before_pointer);}let(result,physical)=crate::observe_retirement_allocations(||owner.reserve_one_funded(grant).unwrap());assert_eq!(physical,(capacity,0));assert_eq!(result.retained_capacity_bytes,capacity);assert_eq!(result.copied_bytes,copy);assert!(result.fits(grant));born+=physical.0;if copy==size_of::<Vec<u64>>()+3*size_of::<usize>(){payload+=1;}else if copy==size_of::<Page<u64>>()+size_of::<Vec<Page<u64>>>()+size_of::<usize>(){metadata+=1;}else{assert_eq!(copy,size_of::<Page<u64>>()+3*size_of::<Vec<Page<u64>>>()+size_of::<usize>());promotions+=1;}assert_eq!(result.copied_items,1);}
  assert_eq!(owner.next_reserve_copy_byte_demand().unwrap(),0);owner.push_reserved(value).unwrap();oracle.push_back(value);
 }
 assert!(metadata>0&&payload>0);assert_eq!(promotions,fixture["promotionPages"].as_array().unwrap().len());assert_eq!(owner.iter().copied().collect::<Vec<_>>(),oracle.iter().copied().collect::<Vec<_>>());while !oracle.is_empty(){assert_eq!(owner.pop(),oracle.pop_back());}let mut released=0;while !owner.terminal_is_empty(){let exact=owner.next_release_allocation_bytes().unwrap();let(result,physical)=crate::observe_retirement_allocations(||owner.release_empty_page(exact).unwrap());assert_eq!(physical,(0,result.released_allocation_bytes));released+=physical.1;}assert_eq!(born,released);
 println!("[DEBUG] PagedList original reservation metadata/payload/root promotions {metadata}/{payload}/{promotions}; allocator capacity/release exact; ordered values matched VecDeque");
}
