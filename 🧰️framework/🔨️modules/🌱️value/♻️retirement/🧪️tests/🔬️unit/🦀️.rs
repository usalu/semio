use super::*;

fn drain(mut retirement: Box<dyn ErasedSnapshotRetirement>, items: usize, bytes: usize) -> usize {
    let mut released = 0;
    for _ in 0..100_000 {
        match retirement.close_step(items, bytes).unwrap() {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= items);
                assert!(released_bytes <= bytes);
                released += released_bytes;
            }
            SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                return released;
            }
            SnapshotRetirementStep::Blocked => panic!("unshared fixture unexpectedly blocked"),
        }
    }
    panic!("bounded fixture retirement did not finish");
}

#[test]
fn owned_retirement_matches_neutral_exact_byte_grants() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(fixture["cases"].as_array().unwrap().len(), 11);
    for row in fixture["cases"].as_array().unwrap() {
        for budget in fixture["budgets"].as_array().unwrap() {
            let retirement = match row["kind"].as_str().unwrap() {
                "string" => owned_retirement(row["value"].as_str().unwrap().to_owned()),
                "strings" => owned_retirement(serde_json::from_value::<Vec<String>>(row["value"].clone()).unwrap()),
                "optionalString" => owned_retirement(serde_json::from_value::<Option<String>>(row["value"].clone()).unwrap()),
                "pair" => owned_retirement(serde_json::from_value::<(String, String)>(row["value"].clone()).unwrap()),
                "stringMap" => owned_retirement(serde_json::from_value::<std::collections::BTreeMap<String, String>>(row["value"].clone()).unwrap()),
                "value" => owned_retirement(crate::DslValue::from(&row["value"])),
                "bytes" => owned_retirement(serde_json::from_value::<Vec<u8>>(row["value"].clone()).unwrap()),
                "words" => owned_retirement(serde_json::from_value::<Vec<u32>>(row["value"].clone()).unwrap()),
                _ => panic!("unknown neutral case"),
            };
            assert_eq!(drain(retirement, budget["items"].as_u64().unwrap() as usize, budget["bytes"].as_u64().unwrap() as usize), row["bytes"].as_u64().unwrap() as usize, "{}", row["id"]);
        }
    }
}

/// ♻️ A collection without drop glue retires a page per step: a 2 MiB byte buffer under one item and 64 KiB per step releases
/// exactly its bytes in at most 36 steps (32 pages, the push, the pop, the root), a grant narrower than one element still
/// completes, and a list of strings still retires string by string.
#[test]
fn a_byte_buffer_retires_page_by_page_and_owned_elements_one_by_one() {
    let mut steps = 0usize;
    let mut released = 0usize;
    let mut retirement = owned_retirement(vec![7u8; 2 * 1024 * 1024]);
    loop {
        steps += 1;
        assert!(steps <= 36, "a 2 MiB buffer needs at most 36 steps of 64 KiB");
        match retirement.close_step(1, 64 * 1024).unwrap() {
            SnapshotRetirementStep::Pending { released_bytes, .. } => released += released_bytes,
            SnapshotRetirementStep::Complete => break,
            SnapshotRetirementStep::Blocked => panic!("an owned buffer never blocks"),
        }
    }
    assert_eq!(released, 2 * 1024 * 1024);
    assert!(retirement.terminal_is_empty());
    assert_eq!(drain(owned_retirement(vec![1u32, 2, 3]), 1, 4), 12);
    assert_eq!(drain(owned_retirement(vec![1u32, 2, 3]), 1, 3), 12, "a grant narrower than one element retires it element by element");
    assert_eq!(drain(owned_retirement(vec!["ab".to_string(), "c".to_string()]), 1, 1), 3);
}

#[test]
fn owned_retirement_rejects_false_terminal_and_preserves_shared_roots() {
    let root = Arc::new("owned".to_string());
    let mut shared = shared_retirement(Arc::clone(&root));
    assert!(matches!(shared.close_step(0, 0).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
    assert!(matches!(shared.close_step(1, 1).unwrap(), SnapshotRetirementStep::Blocked));
    assert_eq!(Arc::strong_count(&root), 2);
    drop(root);
    assert_eq!(drain(shared, 1, 1), 5);
    let mut value = owned_retirement("zero".to_string());
    for _ in 0..4 {
        assert!(matches!(value.close_step(1, 0).unwrap(), SnapshotRetirementStep::Pending { released_bytes: 0, .. }));
    }
    assert!(!value.terminal_is_empty());
    assert_eq!(drain(value, 1, 2), 4);
    struct Hostile {
        mode: u8,
    }
    impl RetirementCursor for Hostile {
        fn close_step(&mut self, maximum_bytes: usize) -> RetirementStep {
            match self.mode {
                1 => RetirementStep::Bytes(maximum_bytes + 1),
                _ => RetirementStep::Complete,
            }
        }
        fn terminal_is_empty(&self) -> bool {
            self.mode == 2
        }
    }
    for mode in [0, 1] {
        let mut stack = CursorStack(ManuallyDrop::new(vec![Box::new(Hostile { mode })]));
        assert!(stack.step(1, 1).is_err());
        assert_eq!(stack.0.len(), 1);
        stack.0.pop();
        assert!(matches!(stack.step(1, 1).unwrap(), SnapshotRetirementStep::Complete));
    }
}

#[test]
fn shared_source_leases_release_all_orders_with_one_bounded_owner() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["leases"];let text=law["text"].as_str().unwrap();let oracle:serde_json::Value=serde_json::from_str(&serde_json::to_string(text).unwrap()).unwrap();assert_eq!(oracle.as_str().unwrap().len(),text.len());
    for canceled in law["canceled"].as_array().unwrap() {for order in law["orders"].as_array().unwrap() {
        let source=Arc::new(text.to_owned());let mut leases=vec![Some(Arc::clone(&source)),Some(Arc::clone(&source)),Some(source)];let mut released=0;
        for (position,index) in order.as_array().unwrap().iter().enumerate() {
            let alias=leases[index.as_u64().unwrap()as usize].take().unwrap();let mut retirement=shared_lease_retirement(alias);
            assert!(matches!(retirement.close_step(1,0).unwrap(),SnapshotRetirementStep::Pending {released_items:0,released_bytes:0}));assert!(!retirement.terminal_is_empty());
            assert!(matches!(retirement.close_step(0,3).unwrap(),SnapshotRetirementStep::Pending {released_items:0,released_bytes:0}));
            let bytes=drain(retirement,law["items"].as_u64().unwrap()as usize,law["bytes"].as_u64().unwrap()as usize);assert_eq!(bytes,if position==2{text.len()}else{0},"cancel={canceled} order={order}");released+=bytes;
        }assert_eq!(released,text.len());assert!(leases.iter().all(Option::is_none));
    }}
    eprintln!("[DEBUG] source lease six release orders; canceled/completed; one bounded source release");
}

#[test]
fn native_controls_resume_same_cumulative_admission_and_cancel_before_more_work() {
    use crate::{NativeDecodeControl,NativeEncodeControl};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["continuation"];let maximum=law["maximumBytes"].as_u64().unwrap()as usize;
    let mut accepted=|_|true;let mut encode=NativeEncodeControl::new(maximum,&mut accepted);encode.begin_stage(9).unwrap();encode.charge(5).unwrap();encode.advance(4).unwrap();let receipt=encode.pause().unwrap();
    let event=std::cell::Cell::new(None);let mut resumed=|value|{event.set(Some(value));true};let mut encode=NativeEncodeControl::resume(receipt,&mut resumed).unwrap();encode.checkpoint().unwrap();assert_eq!(event.get().unwrap().completed,4);assert_eq!(event.get().unwrap().total,9);assert_eq!(encode.owned_bytes(),5);encode.charge(8).unwrap();encode.advance(5).unwrap();assert_eq!(encode.owned_bytes(),maximum);assert_eq!(encode.charge(1).unwrap_err().kind,crate::ValueRefusalKind::OwnershipLimit);let receipt=encode.pause().unwrap();let mut canceled=|_|false;let mut encode=NativeEncodeControl::resume(receipt,&mut canceled).unwrap();assert_eq!(encode.step().unwrap_err().kind,crate::ValueRefusalKind::Canceled);assert_eq!(encode.owned_bytes(),maximum);
    let mut accepted=|_|true;let mut decode=NativeDecodeControl::new(maximum,&mut accepted);decode.begin_stage(9).unwrap();decode.charge(5).unwrap();decode.advance(4).unwrap();let receipt=decode.pause().unwrap();
    let event=std::cell::Cell::new(None);let mut resumed=|value|{event.set(Some(value));true};let mut decode=NativeDecodeControl::resume(receipt,&mut resumed).unwrap();decode.checkpoint().unwrap();assert_eq!(event.get().unwrap().completed,4);assert_eq!(event.get().unwrap().total,9);assert_eq!(decode.owned_bytes(),5);decode.charge(8).unwrap();decode.advance(5).unwrap();assert_eq!(decode.charge(1).unwrap_err().kind,crate::ValueRefusalKind::OwnershipLimit);let receipt=decode.pause().unwrap();let mut canceled=|_|false;let mut decode=NativeDecodeControl::resume(receipt,&mut canceled).unwrap();assert_eq!(decode.step().unwrap_err().kind,crate::ValueRefusalKind::Canceled);assert_eq!(decode.owned_bytes(),maximum);
    eprintln!("[DEBUG] encoding/decoding resume own receipt; cumulative13bytes; fresh callback cancellation");
}

#[test]
fn native_encoding_capacity_admission_consumes_the_same_counter_owner(){
    use crate::{NativeEncodeControl,ValueRefusalKind};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["capacityAdmission"];let prior=&fixture["continuation"];
    let source=Arc::new(law["source"].as_str().unwrap().to_owned());let pointer=source.as_ptr();let reference:serde_json::Value=serde_json::from_str(&serde_json::to_string(&*source).unwrap()).unwrap();assert_eq!(reference.as_str().unwrap().len(),law["sourceBytes"].as_u64().unwrap()as usize);
    let event=std::cell::Cell::new(None);let mut accepted=|value|{assert_eq!(source.as_ptr(),pointer);event.set(Some(value));true};let mut control=NativeEncodeControl::new(prior["maximumBytes"].as_u64().unwrap()as usize,&mut accepted);control.begin_stage(prior["total"].as_u64().unwrap()as usize).unwrap();control.charge(prior["initialBytes"].as_u64().unwrap()as usize).unwrap();control.advance(prior["initialUnits"].as_u64().unwrap()as usize).unwrap();control.checkpoint().unwrap();let before=event.get().unwrap();
    let (mut control,error)=match control.admit_capacity(law["refusedBytes"].as_u64().unwrap()as usize,1,0){Err(rejection)=>rejection,Ok(_)=>panic!("existing ownership cannot fit refused capacity")};assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(control.maximum_bytes(),prior["maximumBytes"].as_u64().unwrap()as usize);control.checkpoint().unwrap();assert_eq!(event.get().unwrap(),before);
    let mut control=match control.admit_capacity(law["sourceBytes"].as_u64().unwrap()as usize,law["sourceMultiples"].as_u64().unwrap()as usize,law["scaffoldBytes"].as_u64().unwrap()as usize){Ok(control)=>control,Err(_)=>panic!("source policy capacity admitted")};assert_eq!(control.maximum_bytes(),law["maximumBytes"].as_u64().unwrap()as usize);control.checkpoint().unwrap();assert_eq!(event.get().unwrap(),before);
    let maximum=control.maximum_bytes();control.charge(maximum-control.owned_bytes()).unwrap();control.advance(prior["finalUnits"].as_u64().unwrap()as usize).unwrap();assert_eq!(control.charge(law["overrunBytes"].as_u64().unwrap()as usize).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);control.checkpoint().unwrap();let complete=event.get().unwrap();assert_eq!(complete.completed,prior["total"].as_u64().unwrap()as usize);assert_eq!(complete.owned_bytes,maximum);let receipt=control.pause().unwrap();
    let canceled_event=std::cell::Cell::new(None);let mut canceled=|value|{assert_eq!(source.as_ptr(),pointer);canceled_event.set(Some(value));false};let mut control=NativeEncodeControl::resume(receipt,&mut canceled).unwrap();assert_eq!(control.step().unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(canceled_event.get().unwrap(),complete);drop(control);assert_eq!(source.as_ptr(),pointer);assert_eq!(drain(shared_lease_retirement(source),1,3),law["sourceBytes"].as_u64().unwrap()as usize);
    eprintln!("[DEBUG] consuming source capacity admission kept exact owned/completed/total counters and source lease identity; refusal preserved owner; one-byte overrun and resumed cancellation");
}

#[test]
fn native_capacity_closed_vectors_preserve_the_consumed_admission_owner(){
    use crate::{NativeEncodeControl,ValueRefusalKind};
    fn number(value:&serde_json::Value)->usize{match value.as_str().unwrap(){"usizeMax"=>usize::MAX,"isizeMax"=>isize::MAX as usize,decimal=>decimal.parse().unwrap()}}
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let cases=fixture["capacityAdmission"]["cases"].as_array().unwrap();assert_eq!(cases.len(),12);
    for case in cases {
        let calls=std::cell::Cell::new(0);let event=std::cell::Cell::new(None);
        let mut callback=|value|{calls.set(calls.get()+1);event.set(Some(value));true};
        let mut control=NativeEncodeControl::new(13,&mut callback);control.begin_stage(9).unwrap();control.charge(number(&case["ownedBytes"])).unwrap();control.advance(4).unwrap();control.checkpoint().unwrap();
        let before=event.get().unwrap();let before_calls=calls.get();
        let result=control.admit_capacity(number(&case["sourceBytes"]),number(&case["multiples"]),number(&case["scaffoldBytes"]));
        assert_eq!(calls.get(),before_calls,"{}",case["id"]);
        let mut control=if case["accepted"].as_bool().unwrap(){let control=match result{Ok(control)=>control,Err(_)=>panic!("capacity unexpectedly refused: {}",case["id"])};assert_eq!(control.maximum_bytes(),number(&case["maximumBytes"]));control}else{let(control,error)=match result{Err(refusal)=>refusal,Ok(_)=>panic!("capacity unexpectedly admitted: {}",case["id"])};assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(control.maximum_bytes(),13);control};
        assert_eq!(control.owned_bytes(),before.owned_bytes);control.checkpoint().unwrap();assert_eq!(event.get().unwrap(),before);control.advance(5).unwrap();let complete=event.get().unwrap();assert_eq!(complete.completed,9);assert_eq!(complete.total,9);assert_eq!(complete.owned_bytes,before.owned_bytes);
        let receipt=control.pause().unwrap();let canceled_event=std::cell::Cell::new(None);let mut canceled=|value|{canceled_event.set(Some(value));false};let mut control=NativeEncodeControl::resume(receipt,&mut canceled).unwrap();assert_eq!(control.step().unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(canceled_event.get().unwrap(),complete);assert_eq!(control.owned_bytes(),before.owned_bytes);
    }
    eprintln!("[DEBUG] native source capacity twelve closed vectors preserve owner/counters/callback; checked overflow, signed ceiling, refusal and resumed cancellation");
}

use crate::{ValueRefusalKind,retirement::allocation_return::{ParentAllocationReturn,AllocationReturnStep}};
#[test]
fn parent_return_fixture_demands_actual_full_allocation_release(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../📦️allocation-return/🧫️fixtures/📦️return.json")).unwrap();
    let maximum=fixture["maximumAllocationBytes"].as_u64().unwrap()as usize;let total=fixture["maximumTotalBytes"].as_u64().unwrap()as usize;
    let mut parent=ParentAllocationReturn::<2>::try_new(maximum,total).unwrap();let mut text=fixture["text"].as_str().unwrap().to_owned();let text_pointer=text.as_ptr();let text_capacity=text.capacity();assert_eq!(text_capacity,fixture["expectedTextBytes"].as_u64().unwrap()as usize);
    assert!(!parent.return_text(&mut text,0).unwrap());assert_eq!(text.as_ptr(),text_pointer);assert_eq!(text,fixture["text"].as_str().unwrap());assert!(parent.terminal_is_empty());
    assert!(parent.return_text(&mut text,1).unwrap());assert!(text.is_empty());assert_eq!(text.capacity(),0);assert!(!parent.terminal_is_empty());assert_eq!(parent.retained_bytes(),text_capacity);
    let mut values=Vec::<u32>::with_capacity(fixture["emptyVectorCapacity"].as_u64().unwrap()as usize);let vector_capacity=values.capacity();assert_eq!(vector_capacity*std::mem::size_of::<u32>(),fixture["expectedVectorBytes"].as_u64().unwrap()as usize);let vector_pointer=values.as_ptr().cast::<u8>();values.push(9);
    assert_eq!(parent.return_empty_vec(&mut values,1).unwrap_err().kind,ValueRefusalKind::InvariantViolated);assert_eq!(values,[9]);assert_eq!(values.as_ptr().cast::<u8>(),vector_pointer);values.clear();assert!(parent.return_empty_vec(&mut values,1).unwrap());assert_eq!(values.capacity(),0);
    let mut occupied="member".to_owned();let occupied_pointer=occupied.as_ptr();assert!(!parent.return_text(&mut occupied,1).unwrap());assert_eq!(occupied.as_ptr(),occupied_pointer);assert_eq!(occupied,"member");
    let owned=parent.retained_bytes();assert_eq!(parent.close_step(0,maximum),AllocationReturnStep::Pending{released_items:0,released_bytes:0});assert_eq!(parent.close_step(1,fixture["childBytes"].as_u64().unwrap()as usize),AllocationReturnStep::Pending{released_items:0,released_bytes:0});assert_eq!(parent.retained_bytes(),owned);assert!(!parent.terminal_is_empty());
    let mut released=0;for _ in 0..3{match parent.close_step(1,maximum){AllocationReturnStep::Complete=>break,AllocationReturnStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released_bytes<=maximum);released+=released_bytes;}}}assert_eq!(released,fixture["expectedTextBytes"].as_u64().unwrap()as usize+fixture["expectedVectorBytes"].as_u64().unwrap()as usize);assert_eq!(parent.retained_bytes(),0);assert!(parent.terminal_is_empty());
    let mut oversized="x".repeat(fixture["refusedPayloadBytes"].as_u64().unwrap()as usize);let pointer=oversized.as_ptr();let capacity=oversized.capacity();assert_eq!(parent.return_text(&mut oversized,1).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(oversized.as_ptr(),pointer);assert_eq!(oversized.capacity(),capacity);assert_eq!(oversized.len(),fixture["refusedPayloadBytes"].as_u64().unwrap()as usize);assert!(parent.terminal_is_empty());
    assert_eq!(serde_json::to_string(&oversized).unwrap(),serde_json::to_string(&"x".repeat(fixture["refusedPayloadBytes"].as_u64().unwrap()as usize)).unwrap());
    eprintln!("[DEBUG] genuine parent flat-allocation tokens preserve actual source pointers/layouts, zero/full-slot/4byte refusal and terminal=false until physical full-grant deallocation; original8194 exceeds4096 authority unchanged");
}
