use std::sync::atomic::Ordering;
use super::*;
use semio_framework_value::retained_clone::RetainedCloneSource;

#[test]
fn captured_store_source_preserves_original_read_on_refusal_and_returns_root_to_its_original_registry(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🎟️source-authority/🔣️.json")).unwrap();
    let registry=crate::os_store::SnapshotReadRegistryHandle::new();let mut value=String::with_capacity(law["ownerCapacity"].as_u64().unwrap()as usize);value.push_str(law["owner"].as_str().unwrap());let owner=Arc::new(value);let pointer=Arc::as_ptr(&owner);let lease=registry.try_issue(Arc::clone(&owner)).unwrap_or_else(|_|panic!("original Store read issue"));let(index,generation)=(lease.index,lease.generation);let mut read=SnapshotRead::new(owner,lease);
    let(bytes,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||RetainedCloneSource::<String>::constructor_capacity_bytes::<SnapshotRead<String>>());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let grant=RetainedCloneGrant{maximum_items:law["birth"]["items"].as_u64().unwrap()as usize,maximum_capacity_bytes:bytes,maximum_depth:law["birth"]["depth"].as_u64().unwrap()as usize,..Default::default()};
    for currency in law["refusals"].as_array().unwrap(){let denied=match currency.as_str().unwrap(){"items"=>RetainedCloneGrant{maximum_items:0,..grant},"capacity"=>RetainedCloneGrant{maximum_capacity_bytes:bytes-1,..grant},"depth"=>RetainedCloneGrant{maximum_depth:0,..grant},_=>unreachable!()};let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||read.admit_retained_clone_source(denied));let(_,returned)=result.err().unwrap();read=returned;assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(read.get()as*const String,pointer);assert_eq!(read.lease.as_ref().unwrap().index,index);assert_eq!(read.lease.as_ref().unwrap().generation,generation);assert!(registry.contains(index,generation));}
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||read.admit_retained_clone_source(grant));let(mut source,receipt)=result.map_err(|(error,_)|error).unwrap();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,0));let(mut born,mut released)=(heap.requested_bytes,0usize);assert_eq!(source.borrow().get()as*const String,pointer);
    for _ in 0..law["maximumTurns"].as_u64().unwrap(){if source.terminal_is_empty(){break;}let grant=crate::os_store::component::presence_test_retirement::CLOSE_GRANT;assert!(source.next_close_copy_byte_demand().unwrap()<=grant.maximum_copy_bytes);assert!(source.next_close_capacity_byte_demand(grant.maximum_copy_bytes).unwrap()<=grant.maximum_capacity_bytes);assert!(source.next_close_release_byte_demand().unwrap()<=grant.maximum_release_bytes);assert!(source.next_close_depth_demand().unwrap()<=grant.maximum_depth);let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||source.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;}
    assert!(source.terminal_is_empty());assert_eq!(released,born);assert!(registry.contains(index,generation));assert_eq!(registry.returned.load(Ordering::Acquire),law["afterSourceClose"]["returned"].as_u64().unwrap()as usize);assert_eq!(semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(source)).1.released_bytes,0);
    let mut original=None;
    for _ in 0..crate::os_store::component::SNAPSHOT_READ_LEASE_CAPACITY {
        let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||registry.try_admit_one_returned::<String,_>(crate::os_store::component::presence_test_retirement::CLOSE_GRANT,|original,grant|{assert_eq!(Arc::as_ptr(&original),pointer);assert_eq!(original.as_str(),law["owner"].as_str().unwrap());semio_framework_value::retirement::shared::admit_shared_retirement(original,grant,false)}));
        let (owner,receipt)=result.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));original=owner;if original.is_some(){break;}
    }
    assert!(original.is_some());crate::os_store::component::presence_test_retirement::finish_box(&mut original);assert!(registry.terminal_is_empty());crate::os_store::component::presence_test_retirement::finish_registry(registry);
    eprintln!("[DEBUG] original Store read source pointer/index/generation retained on denied birth; source born={born} release={released}; original root returned to original pump registry");
}


#[test]
fn store_read_terminal_registry_receipt_preserves_original_system_release() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../👥️presence/🧫️fixtures/🧹️retirement.json")).unwrap();
    let grant = crate::os_store::component::presence_test_retirement::CLOSE_GRANT;
    let registry = crate::os_store::SnapshotReadRegistryHandle::new();
    let mut cursor = SnapshotReadRetirement::<String> { read: ManuallyDrop::new(None), alias: ManuallyDrop::new(None), registry: ManuallyDrop::new(Some(registry)), active_returned: ManuallyDrop::new(None) };
    let mut final_release = 0;
    for _ in 0..16 {
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| cursor.close_step(grant));
        let receipt = match step { RetirementStep::Progress(receipt) => receipt, RetirementStep::Complete => RetainedCloneProgress::default(), RetirementStep::Failure(error) => panic!("{error}"), _ => panic!("original registry cursor has no alternate physical authority") };
        assert!(receipt.fits(grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
        if cursor.terminal_is_empty() { final_release = receipt.released_bytes; break; }
    }
    assert_eq!(cursor.terminal_is_empty(), law["terminalRegistry"]["expectedTerminal"].as_bool().unwrap());
    assert_eq!(final_release != 0, law["terminalRegistry"]["finalReleaseMustBeReported"].as_bool().unwrap());
    eprintln!("[DEBUG] original Store read final registry frame release={final_release} preserved through RetirementCursor adapter and System observer");
}
