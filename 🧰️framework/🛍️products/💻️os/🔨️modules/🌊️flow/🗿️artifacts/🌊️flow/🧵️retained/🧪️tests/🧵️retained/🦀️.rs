//! 🧪️ Original Flow semantic, byte, transfer, ingress and capacity laws with independent grants.

use super::*;
use super::field_tests::observe;
use std::mem::size_of;

#[derive(Default)]
struct Receipt {copied:usize,born:usize,released:usize,last_release:usize}
fn grant(owner:&FlowRetirement,copy:usize)->RetainedCloneGrant {RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()}}
fn drain(mut owner:FlowRetirement,copy:usize)->Receipt {
    let mut receipt=Receipt::default();
    for turn in 0..200_000 {
        if owner.terminal_is_empty(){return receipt;}
        let paid=grant(&owner,copy);
        let (step,born,freed)=observe(||owner.step(paid).unwrap());let progress=step.progress();
        assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(born,freed));assert!(progress.copied_items<=1&&progress.copied_bytes<=copy);assert!(born<=paid.maximum_capacity_bytes&&freed<=paid.maximum_release_bytes);
        receipt.copied+=progress.copied_bytes;receipt.born+=born;receipt.released+=freed;if freed!=0{receipt.last_release=freed;}
        assert!(progress.copied_items!=0,"exact Flow grant blocked on turn {turn}");
    }
    panic!("Flow retirement did not reach terminal-empty")
}
#[test]
fn flow_retirement_typed_serde_oracle_and_exact_bytes_survive_worker_transfer() {
    let fixture=semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for copy in [1,4096] {
        let value:FlowHostSnapshot=semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(fixture.get("hostSnapshot").unwrap())).unwrap();
        let oracle:FlowHostSnapshot=semio_framework_value::FromValue::from_value(semio_framework_value::ToValue::to_value(&value)).unwrap();assert_eq!(value,oracle);
        let expected=drain(FlowRetirement::from_owner(FlowOwner::HostSnapshot(oracle)),copy).copied;
        let mut retirement=FlowRetirement::from_owner(FlowOwner::HostSnapshot(value));let paid=grant(&retirement,copy);
        assert_eq!(retirement.step(RetainedCloneGrant {maximum_items:0,..paid}).unwrap().progress().copied_items,0);
        assert_eq!(retirement.step(RetainedCloneGrant {maximum_capacity_bytes:0,maximum_copy_bytes:0,maximum_release_bytes:0,..paid}).unwrap().progress().copied_items,0);
        let observed=std::thread::spawn(move||drain(retirement,copy)).join().unwrap();assert_eq!(observed.copied,expected);assert!(expected>=fixture.get("expected").unwrap().get("releasedBytes").unwrap().as_u64().unwrap()as usize);
    }
}
#[test]
fn flow_retirement_populated_drop_is_guarded_and_unwind_does_not_double_panic() {
    let retirement=FlowRetirement::from_owner(FlowOwner::Bytes(vec![0;8192]));assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||drop(retirement))).is_err());
    assert!(std::thread::spawn(||{let _retirement=FlowRetirement::from_owner(FlowOwner::Bytes(vec![0;8192]));panic!("primary Flow retirement fault");}).join().is_err());
}
fn oversized_empty<T>(minimum_bytes:usize)->Vec<T> {Vec::with_capacity(minimum_bytes.div_ceil(size_of::<T>()).max(1))}
fn exact_direct_backing(value:FlowOwner,original:usize,payload:usize) {
    assert!(original>4096);let mut owner=FlowRetirement::from_owner(value);let (mut copied,mut born,mut freed,mut physical_backing_releases)=(0,0,0,0);
    for turn in 0..payload+100_000 {
        if owner.terminal_is_empty(){break;}
        let paid=grant(&owner,1);let (zero,a,r)=observe(||owner.step(RetainedCloneGrant {maximum_items:0,..paid}).unwrap());assert_eq!(zero.progress().copied_items,0);assert_eq!((a,r),(0,0));
        if paid.maximum_capacity_bytes!=0{let (denied,a,r)=observe(||owner.step(RetainedCloneGrant {maximum_capacity_bytes:paid.maximum_capacity_bytes-1,..paid}).unwrap());assert_eq!(denied.progress().copied_items,0);assert_eq!((a,r),(0,0));}
        if paid.maximum_release_bytes!=0{let (denied,a,r)=observe(||owner.step(RetainedCloneGrant {maximum_release_bytes:paid.maximum_release_bytes-1,..paid}).unwrap());assert_eq!(denied.progress().copied_items,0);assert_eq!((a,r),(0,0));}
        let (step,a,r)=observe(||owner.step(paid).unwrap());let progress=step.progress();assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(a,r));assert!(progress.copied_bytes<=1);copied+=progress.copied_bytes;born+=a;freed+=r;if r==original{physical_backing_releases+=1;}assert!(progress.copied_items!=0,"direct Flow ownership blocked on turn {turn}");
    }
    assert!(owner.terminal_is_empty());assert_eq!(copied,payload);assert_eq!(freed,original+born);assert_eq!(physical_backing_releases,1,"actual original capacity leaves once and only under its release grant");assert!(matches!(owner.step(RetainedCloneGrant {maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0}).unwrap(),RetainedCloneStep::Complete(_)));
}
#[test]
fn flow_physical_retirement_every_direct_string_and_vec_releases_actual_capacity_once() {
    let mut bytes=Vec::with_capacity(8193);bytes.resize(4097,7);let capacity=bytes.capacity();exact_direct_backing(FlowOwner::Bytes(bytes),capacity,4097);
    let values=oversized_empty::<String>(8193);let capacity=values.capacity()*size_of::<String>();exact_direct_backing(FlowOwner::Strings(values),capacity,0);
    let values=oversized_empty::<Widget>(8193);let capacity=values.capacity()*size_of::<Widget>();exact_direct_backing(FlowOwner::Widgets(values),capacity,0);
    let values=oversized_empty::<SynapseSpec>(8193);let capacity=values.capacity()*size_of::<SynapseSpec>();exact_direct_backing(FlowOwner::Specs(values),capacity,0);
    let values=oversized_empty::<neural::Neuron>(8193);let capacity=values.capacity()*size_of::<neural::Neuron>();exact_direct_backing(FlowOwner::Neurons(values),capacity,0);
    let values=oversized_empty::<neural::Synapse>(8193);let capacity=values.capacity()*size_of::<neural::Synapse>();exact_direct_backing(FlowOwner::Synapses(values),capacity,0);
    let values=oversized_empty::<FlowPreviewGui>(8193);let capacity=values.capacity()*size_of::<FlowPreviewGui>();exact_direct_backing(FlowOwner::Previews(values),capacity,0);
    let values=oversized_empty::<FlowLayoutEntry>(8193);let capacity=values.capacity()*size_of::<FlowLayoutEntry>();exact_direct_backing(FlowOwner::Layout(values),capacity,0);
}
#[test]
fn flow_physical_retirement_frontier_requires_exact_admission_and_releases_metadata_last() {
    let fixture=semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let (snapshot,born,freed)=observe(||semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(fixture.get("hostSnapshot").unwrap())).unwrap());let original=born-freed;let mut owner=FlowRetirement::from_owner(FlowOwner::HostSnapshot(snapshot));let demand=owner.next_capacity_byte_demand(1).unwrap();assert!(demand>0);
    let paid=grant(&owner,1);for capacity in [0,demand-1]{let (step,a,r)=observe(||owner.step(RetainedCloneGrant {maximum_capacity_bytes:capacity,..paid}).unwrap());assert_eq!(step.progress().copied_items,0);assert_eq!((a,r),(0,0));}
    let receipt=drain(owner,1);assert_eq!(receipt.released,original+receipt.born);assert!(receipt.last_release>0,"the cursor frontier metadata has a final exact physical release");
}
#[test]
fn flow_physical_retirement_multi_root_ingress_records_fault_without_admission_then_admits_exactly() {
    let fixture=semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let contract=fixture.get("physicalRetirement").unwrap().get("multiRootIngress").unwrap();let first_capacity=contract.get("firstCapacityBytes").unwrap().as_u64().unwrap()as usize;let second_capacity=contract.get("secondCapacityBytes").unwrap().as_u64().unwrap()as usize;
    let mut first=Vec::with_capacity(first_capacity);first.push(1);let mut second=Vec::with_capacity(second_capacity);second.push(2);let pointer=second.as_ptr();let original=first.capacity()+second.capacity();let mut owner=FlowRetirement::from_owner(FlowOwner::Bytes(first));let (second,a,r)=observe(||owner.push(FlowOwner::Bytes(second)).err().unwrap());assert_eq!((a,r),(0,0));let FlowOwner::Bytes(second)=second else{unreachable!()};assert_eq!(second.as_ptr(),pointer);
    let capacity=owner.next_push_capacity_byte_demand().unwrap();for maximum in [0,capacity-1]{let (step,a,r)=observe(||owner.reserve_push(RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:maximum,maximum_release_bytes:0,maximum_depth:1}).unwrap());assert_eq!(step.copied_items,0);assert_eq!((a,r),(0,0));}
    let (reservation,a,r)=observe(||owner.reserve_push(RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:capacity,maximum_release_bytes:0,maximum_depth:1}).unwrap());assert_eq!((reservation.retained_capacity_bytes,reservation.released_bytes),(a,r));let admitted=a;
    let bytes=owner.next_push_capacity_byte_demand().unwrap();let (denied,a,r)=observe(||owner.admit_owner(FlowOwner::Bytes(second),RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:bytes-1,maximum_release_bytes:0,maximum_depth:1}));assert_eq!((a,r),(0,0));let (_,second)=denied.err().unwrap();let FlowOwner::Bytes(second)=second else{unreachable!()};assert_eq!(second.as_ptr(),pointer);
    let (frame,a,r)=observe(||owner.admit_owner(FlowOwner::Bytes(second),RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:bytes,maximum_release_bytes:0,maximum_depth:1}).unwrap_or_else(|(error,_)|panic!("Flow frame refused: {error}")));assert_eq!((frame.retained_capacity_bytes,frame.released_bytes),(a,r));let receipt=drain(owner,1);assert_eq!(receipt.copied,2);assert_eq!(receipt.released,original+admitted+a+receipt.born);
}
#[test]
fn flow_physical_retirement_charges_payload_bytes_not_machine_width_capacity() {
    let mut scene=Vec::with_capacity(32*1024);scene.extend_from_slice(b"7 bytes");let drained=oversized_empty::<String>(8193);let original=scene.capacity()+drained.capacity()*size_of::<String>();assert!(original>30_000);let mut owner=FlowRetirement::from_owner(FlowOwner::Bytes(scene));let (_,born,freed)=observe(||owner.push_cold(FlowOwner::Strings(drained)));assert_eq!(freed,0);let receipt=drain(owner,1);assert_eq!(receipt.copied,7,"portable scene payload is separate from physical Vec capacity");assert_eq!(receipt.released,original+born+receipt.born);
}
