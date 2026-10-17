use super::*;

#[test]
fn retained_paged_list_owner_page_ceiling_preserves_order_and_exact_retirement() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📏️owner-page-ceiling.json")).unwrap();
    let ceiling = fixture["ownerPayloadPageBytes"].as_u64().unwrap() as usize;
    for invalid in fixture["invalidPayloadPageBytes"].as_array().unwrap() {
        assert_eq!(PagedList::<u64, 200>::with_payload_page_bytes(invalid.as_u64().unwrap() as usize).err().unwrap().kind, PagedListRefusalKind::OwnershipLimit);
    }
    for count in fixture["boundaryCounts"].as_array().unwrap() {
        let count = count.as_u64().unwrap() as usize;
        let mut owner = PagedList::<u64, 200>::with_payload_page_bytes(ceiling).unwrap();
        let mut admitted = 0;
        for value in 0..count as u64 {
            while !owner.has_reserved_slot() {
                let bytes = owner.next_allocation_bytes().unwrap();
                assert!(bytes <= ceiling);
                let before = (owner.capacity(), owner.allocated_bytes(), owner.len());
                assert!(!owner.reserve_one(bytes - 1).unwrap().progressed);
                assert_eq!((owner.capacity(), owner.allocated_bytes(), owner.len()), before);
                let step = owner.reserve_one(bytes).unwrap();
                assert!(step.progressed);
                assert_eq!(step.allocated_bytes, bytes);
                admitted += bytes;
            }
            owner.push_reserved(value).unwrap();
        }
        assert_eq!(owner.iter().copied().collect::<Vec<_>>(), (0..count as u64).collect::<Vec<_>>());
        while owner.pop().is_some() {}
        let mut returned = 0;
        while !owner.terminal_is_empty() {
            let bytes = owner.next_release_allocation_bytes().unwrap();
            assert!(!owner.release_empty_page(bytes - 1).unwrap().progressed);
            let step = owner.release_empty_page(bytes).unwrap();
            assert!(step.progressed);
            returned += step.released_allocation_bytes;
        }
        assert_eq!(returned, admitted);
    }
    let mut owner = PagedList::<u64, 200>::with_payload_page_bytes(ceiling).unwrap();
    for value in fixture["input"].as_array().unwrap().iter().chain(fixture["append"].as_array().unwrap()) {
        while !owner.has_reserved_slot() { assert!(owner.reserve_one(ceiling).unwrap().progressed); }
        owner.push_reserved(value.as_u64().unwrap()).unwrap();
    }
    assert_eq!(serde_json::to_value(owner.iter().copied().collect::<Vec<_>>()).unwrap(), fixture["expected"]);
    assert_eq!(owner.iter().sum::<u64>(), fixture["sum"].as_u64().unwrap());
    let mut default = PagedList::<u64, 600>::default();
    assert!(default.reserve_one(ceiling).unwrap().progressed);
    assert_eq!(default.next_allocation_bytes().unwrap(), fixture["defaultPayloadPageBytes"].as_u64().unwrap() as usize);
}

#[test]
fn retained_paged_list_neutral_order_capacity_and_close() {
    let data: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
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
}

#[test]
fn paged_child_source_returns_genuine8194_backing_to_parent_without_physical_credit(){
    use semio_framework_value_allocation_return::{ParentAllocationReturn,AllocationReturnStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🏠️parent-return.json")).unwrap();
    let maximum=fixture["maximumBytes"].as_u64().unwrap()as usize;
    let exact=fixture["wireBytes"].as_u64().unwrap()as usize;
    let mut source=PagedList::<u8,8194>::empty();
    for byte in fixture["wire"].as_str().unwrap().bytes().chain(std::iter::repeat_n(fixture["paddingByte"].as_u64().unwrap()as u8,fixture["paddingBytes"].as_u64().unwrap()as usize)){
        while !source.has_reserved_slot(){let progress=source.reserve_one(maximum).unwrap();assert!(progress.progressed&&progress.allocated_bytes<=maximum);}
        source.push_reserved(byte).unwrap();
    }
    assert_eq!(source.len(),exact);
    let pointer=source.backing_ptr(0).unwrap();
    let initial=source.allocated_bytes();
    let mut parent=ParentAllocationReturn::<1>::try_new(maximum,fixture["parentTotalBytes"].as_u64().unwrap()as usize).unwrap();
    assert!(!source.return_empty_page(&mut parent,0).unwrap().progressed);
    assert_eq!(source.backing_ptr(0),Some(pointer));assert_eq!(source.len(),exact);assert!(parent.terminal_is_empty());
    assert_eq!(source.return_empty_page(&mut parent,1).unwrap_err().kind,crate::ValueRefusalKind::InvariantViolated);
    assert_eq!(source.backing_ptr(0),Some(pointer));assert_eq!(source.len(),exact);assert!(parent.terminal_is_empty());
    while source.pop().is_some(){}
    let expected=source.next_release_allocation_bytes().unwrap();
    let mut blocker=Vec::with_capacity(1);blocker.push(9u8);assert!(parent.return_bytes(&mut blocker,1).unwrap());
    let before=source.allocated_bytes();
    assert!(!source.return_empty_page(&mut parent,1).unwrap().progressed);
    assert_eq!(source.allocated_bytes(),before);assert_eq!(parent.retained_bytes(),1);
    assert_eq!(parent.close_step(1,maximum),AllocationReturnStep::Pending{released_items:1,released_bytes:1});assert!(parent.terminal_is_empty());
    let mut insufficient=ParentAllocationReturn::<1>::try_new(expected-1,expected-1).unwrap();
    assert_eq!(source.return_empty_page(&mut insufficient,1).unwrap_err().kind,crate::ValueRefusalKind::OwnershipLimit);
    assert_eq!(source.allocated_bytes(),before);assert!(insufficient.terminal_is_empty());
    let mut returned=0;let mut released=0;
    for _ in 0..fixture["lawTurns"].as_u64().unwrap(){
        if source.terminal_is_empty()&&parent.terminal_is_empty(){break;}
        if !parent.terminal_is_empty(){
            let exact=parent.next_close_byte_demand();
            assert_eq!(parent.close_step(1,exact-1),AllocationReturnStep::Pending{released_items:0,released_bytes:0});
            assert!(!parent.terminal_is_empty());
            match parent.close_step(1,maximum){AllocationReturnStep::Pending{released_items,released_bytes}=>{assert_eq!(released_items,1);assert_eq!(released_bytes,exact);assert!(released_bytes<=maximum);released+=released_bytes;},AllocationReturnStep::Complete=>panic!("actual parent owner disappeared")}
        }else{
            let before=source.allocated_bytes();let progress=source.return_empty_page(&mut parent,1).unwrap();
            assert!(progress.progressed);assert_eq!(before-source.allocated_bytes(),progress.returned_allocation_bytes);assert_eq!(parent.retained_bytes(),progress.returned_allocation_bytes);assert!(!parent.terminal_is_empty());returned+=progress.returned_allocation_bytes;
        }
    }
    assert!(source.terminal_is_empty()&&parent.terminal_is_empty());assert_eq!(returned,initial);assert_eq!(released,initial);
}


#[test]
fn paged_actual_height_preserves_maximum_authority_pointer_and_parent_handoff(){
    use semio_framework_value_allocation_return::{ParentAllocationReturn,AllocationReturnStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🌳️actual-height.json")).unwrap();assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);assert_eq!(fixture["ownerWords"],7);assert_eq!(size_of::<PagedList<u64,{isize::MAX as usize}>>(),7*size_of::<usize>());
    let mut scalar=PagedList::<u64,{isize::MAX as usize}>::empty();let mut allocations=0;while !scalar.has_reserved_slot(){let required=scalar.next_allocation_bytes().unwrap();assert!(required<=4096);let step=scalar.reserve_one(4096).unwrap();assert!(step.progressed);assert_eq!(step.allocated_bytes,required);allocations+=1;}assert!(scalar.push_reserved(fixture["scalar"].as_u64().unwrap()).is_ok());assert!(allocations<=2,"one scalar preallocated maximum-height scaffolding");assert!(scalar.allocated_bytes()<=8192);assert_eq!(scalar[0],7);scalar.pop();while !scalar.terminal_is_empty(){assert!(scalar.release_empty_page(4096).unwrap().progressed)}
    for bytes in fixture["growthPayloadBytes"].as_array().unwrap(){
        let length=bytes.as_u64().unwrap()as usize;let mut owner=PagedList::<u8,{isize::MAX as usize}>::empty();let mut first=None;assert_eq!(owner.next_push_depth_demand().unwrap_err().kind,PagedListRefusalKind::OwnershipLimit);let mut depths=Vec::new();
        for index in 0..length{
            while !owner.has_reserved_slot(){let required=owner.next_allocation_bytes().unwrap();assert!(required<=4096);let retained=owner.allocated_bytes();let pointer=owner.backing_ptr(0);assert!(!owner.reserve_one(required.saturating_sub(1)).unwrap().progressed);assert_eq!(owner.allocated_bytes(),retained);assert_eq!(owner.backing_ptr(0),pointer);let step=owner.reserve_one(4096).unwrap();assert!(step.progressed);assert_eq!(step.allocated_bytes,required);}
            if let Some(row)=fixture["reservedPushDepth"].as_array().unwrap().iter().find(|row|row["index"].as_u64().unwrap()as usize==index){let before=(owner.len(),owner.allocated_bytes(),owner.backing_ptr(0));let depth=owner.next_push_depth_demand().unwrap();assert_eq!(depth,row["depth"].as_u64().unwrap()as usize);assert_eq!((owner.len(),owner.allocated_bytes(),owner.backing_ptr(0)),before);depths.push(serde_json::json!({"index":index,"depth":depth}));}
            assert!(owner.push_reserved((index%251)as u8).is_ok());if index==0{first=owner.backing_ptr(0);}assert_eq!(owner.backing_ptr(0),first);
        }
        assert_eq!(serde_json::to_value(depths).unwrap(),serde_json::Value::Array(fixture["reservedPushDepth"].as_array().unwrap().iter().filter(|row|row["index"].as_u64().unwrap()<length as u64).cloned().collect()));
        assert_eq!(owner.len(),length);for(index,byte)in owner.iter().enumerate(){assert_eq!(*byte,(index%251)as u8)}let allocated=owner.allocated_bytes();while owner.pop().is_some(){}let mut parent=ParentAllocationReturn::<512>::try_new(4096,allocated).unwrap();let mut returned=0;
        while !owner.terminal_is_empty(){let before=owner.allocated_bytes();let step=owner.return_empty_page(&mut parent,1).unwrap();assert!(step.progressed);assert_eq!(before-owner.allocated_bytes(),step.returned_allocation_bytes);returned+=step.returned_allocation_bytes;}assert_eq!(returned,allocated);assert_eq!(owner.allocated_bytes(),0);assert_eq!(parent.retained_bytes(),allocated);let mut physical=0;for _ in 0..512+128{match parent.close_step(1,4096){AllocationReturnStep::Complete=>break,AllocationReturnStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1);assert!(released_bytes<=4096);physical+=released_bytes;}}}assert!(parent.terminal_is_empty());assert_eq!(physical,allocated);
    }
}

