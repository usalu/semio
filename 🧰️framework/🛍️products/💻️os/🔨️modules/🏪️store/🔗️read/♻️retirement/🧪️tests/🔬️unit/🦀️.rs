use std::sync::Arc;
use std::sync::atomic::Ordering;
use super::*;
use semio_framework_value::retained_clone::RetainedCloneSource;

#[test]
fn captured_store_source_preserves_original_read_on_refusal_and_returns_root_to_its_original_registry(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🎟️source-authority/🔣️.json")).unwrap();
    let registry=crate::SnapshotReadRegistryHandle::new();let mut value=String::with_capacity(law["ownerCapacity"].as_u64().unwrap()as usize);value.push_str(law["owner"].as_str().unwrap());let owner=Arc::new(value);let pointer=Arc::as_ptr(&owner);let lease=registry.try_issue(Arc::clone(&owner)).unwrap_or_else(|_|panic!("original Store read issue"));let(index,generation)=(lease.index,lease.generation);let read=SnapshotRead::new(owner,lease);
    let(bytes,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||RetainedCloneSource::<String,SnapshotRead<String>>::borrowed_constructor_capacity_bytes::<SnapshotRead<String>>());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let grant=RetainedCloneGrant{maximum_items:law["birth"]["items"].as_u64().unwrap()as usize,maximum_copy_bytes:RetainedCloneSource::<String,SnapshotRead<String>>::borrowed_constructor_copy_bytes::<SnapshotRead<String>>(),maximum_capacity_bytes:bytes,maximum_depth:law["birth"]["depth"].as_u64().unwrap()as usize,..Default::default()};
    let mut admission=read.source_admission().unwrap_or_else(|(error,_)|panic!("{error}"));
    for currency in law["refusals"].as_array().unwrap(){let denied=match currency.as_str().unwrap(){"items"=>RetainedCloneGrant{maximum_items:0,..grant},"capacity"=>RetainedCloneGrant{maximum_capacity_bytes:0,..grant},"depth"=>RetainedCloneGrant{maximum_depth:0,..grant},_=>unreachable!()};let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||admission.advance(denied));assert_eq!(turn.result.unwrap().progress(),RetainedCloneProgress::default());assert_eq!(turn.work,0);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let read=admission.original_authority().unwrap();assert_eq!(read.get()as*const String,pointer);assert_eq!(read.lease.as_ref().unwrap().index,index);assert_eq!(read.lease.as_ref().unwrap().generation,generation);assert!(registry.contains(index,generation));}
    let(mut born,mut released)=(0usize,0usize);for _ in 0..16{if admission.phase()=="ready"{break;}let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||admission.advance(grant));let receipt=turn.result.unwrap().progress();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;}assert_eq!(admission.phase(),"ready");
    let mut output=None;let turn=admission.take_output(&mut output,grant);turn.result.unwrap();assert!(admission.terminal_is_empty());drop(admission);let mut source=output.expect("paid source output remains in its actual caller slot");assert_eq!(source.borrow().get()as*const String,pointer);
    for _ in 0..law["maximumTurns"].as_u64().unwrap(){if source.terminal_is_empty(){break;}let grant=crate::presence_test_retirement::CLOSE_GRANT;assert!(source.next_close_copy_byte_demand().unwrap()<=grant.maximum_copy_bytes);assert!(source.next_close_capacity_byte_demand(grant.maximum_copy_bytes).unwrap()<=grant.maximum_capacity_bytes);assert!(source.next_close_release_byte_demand().unwrap()<=grant.maximum_release_bytes);assert!(source.next_close_depth_demand().unwrap()<=grant.maximum_depth);let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||source.close_step(grant).result.unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;}
    assert!(source.terminal_is_empty());assert_eq!(released,born);assert!(registry.contains(index,generation));assert_eq!(registry.returned.load(Ordering::Acquire),law["afterSourceClose"]["returned"].as_u64().unwrap()as usize);assert_eq!(semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(source)).1.released_bytes,0);
    let mut original=None;
    for _ in 0..crate::SNAPSHOT_READ_LEASE_CAPACITY {
        let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||registry.try_admit_one_returned::<String,_>(crate::presence_test_retirement::CLOSE_GRANT,|original,grant|{assert_eq!(Arc::as_ptr(&original),pointer);assert_eq!(original.as_str(),law["owner"].as_str().unwrap());semio_framework_value::retirement::arc_custody::admit_shared_retirement(original,grant,false)}).result);
        let (owner,receipt)=result.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));original=owner;if original.is_some(){break;}
    }
    assert!(original.is_some());crate::presence_test_retirement::finish_box(&mut original);assert!(registry.terminal_is_empty());crate::presence_test_retirement::finish_registry(registry);
}


#[test]
fn store_read_terminal_registry_receipt_preserves_original_system_release() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../👥️presence/🧪️tests/🧫️fixtures/🧹️retirement.json")).unwrap();
    let grant = crate::presence_test_retirement::CLOSE_GRANT;
    let registry = crate::SnapshotReadRegistryHandle::new();
    let mut cursor = SnapshotReadRetirement::<String> { read: ManuallyDrop::new(None), alias: ManuallyDrop::new(None), registry: ManuallyDrop::new(Some(crate::SnapshotReadRegistryAliasRetirement::new(registry))), active_returned: ManuallyDrop::new(None) };
    let mut final_release = 0;
    let mut receipts = std::collections::VecDeque::new();
    for _ in 0..1024 {
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.close_step(grant));
        let receipt = match step { RetirementStep::Progress(receipt, _) => receipt, RetirementStep::Complete(_) => RetainedCloneProgress::default(), RetirementStep::Failure(error, _) => panic!("{error}"), _ => panic!("original registry cursor has no alternate physical authority") };
        assert!(receipt.fits(grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
        if receipts.len() == 4 { receipts.pop_front(); }
        receipts.push_back(receipt);
        final_release += receipt.released_bytes;
        if cursor.terminal_is_empty() { break; }
        assert!(receipt != RetainedCloneProgress::default(), "an unfunded original registry frontier must not stall under its full close grant");
    }
    assert_eq!(cursor.terminal_is_empty(), law["terminalRegistry"]["expectedTerminal"].as_bool().unwrap(), "registry cursor did not reach terminal; last receipts {:?}", receipts);
    assert_eq!(final_release != 0, law["terminalRegistry"]["finalReleaseMustBeReported"].as_bool().unwrap());
}
