//! 🎟️ Original JSON authority is conserved across repeated turns, cancellation and paid physical closure.
use super::*;
use semio_framework_value::{DslValue,NativeDecodeControl,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};

fn law()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn grant(value:&serde_json::Value)->RetainedCloneGrant{serde_json::from_value(value.clone()).unwrap()}
fn policy(law:&serde_json::Value)->JsonReadPolicy{let limits=&law["limits"];JsonReadPolicy{limits:JsonReadLimits{maximum_bytes:limits["maximumBytes"].as_u64().unwrap(),maximum_allocation_bytes:limits["maximumAllocationBytes"].as_u64().unwrap()as usize,maximum_depth:limits["maximumDepth"].as_u64().unwrap()as usize,maximum_items:limits["maximumItems"].as_u64().unwrap()},normal:grant(&law["normalAuthority"]),retirement:grant(&law["retirementAuthority"])}}
fn close<S:JsonReadSource+Copy>(operation:&mut JsonReadOperation<'_,'_,S,DslValue>,law:&serde_json::Value){
    let grant=grant(&law["closeTurn"]);let mut turns=0;
    while !operation.terminal_is_empty(){
        let(result,born,released)=test_allocation::observe_backing(||operation.close_step(grant));let step=result.unwrap();
        assert!(step.progress().fits(grant));assert_eq!((born,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        turns+=1;assert!(turns<law["maximumCloseTurns"].as_u64().unwrap());
    }
    println!("[DEBUG] original JSON operation physically closed turns={turns} cumulative={:?}",operation.retirement_receipt());
}

#[test]
fn original_json_operation_never_refills_cumulative_authority_from_repeated_turns(){
    let law=law();let source=law["source"].as_str().unwrap();let source_pointer=source.as_ptr();let mut accepted=|_|true;let mut control=NativeDecodeControl::new_retained(&mut accepted);let control_pointer=std::ptr::addr_of!(control);
    let(result,born,released)=test_allocation::observe_backing(||JsonReadOperation::<_,DslValue>::new(source,JsonMemberPolicy::Reject,policy(&law),&mut control));let mut operation=result.unwrap();assert_eq!((born,released),(0,0));assert_eq!(operation.source().as_ptr(),source_pointer);assert_eq!(std::ptr::addr_of!(*operation.control),control_pointer);
    let incoming=grant(&law["turn"]);let(result,born,released)=test_allocation::observe_backing(||operation.step(incoming));assert!(result.unwrap().is_none());assert_eq!((born,released),(0,0));assert_eq!(operation.normal_receipt(),RetainedCloneProgress{copied_items:1,..Default::default()});
    let position=operation.position();for _ in 0..32{let(result,born,released)=test_allocation::observe_backing(||operation.step(incoming));assert!(result.unwrap().is_none());assert_eq!((born,released),(0,0));assert_eq!(operation.position(),position);assert_eq!(operation.normal_receipt().copied_items,1);assert_eq!(operation.source().as_ptr(),source_pointer);}
    assert_eq!(serde_json::from_str::<String>(source).unwrap(),"😀");
    let denied=RetainedCloneGrant{maximum_capacity_bytes:0,..grant(&law["closeTurn"])};let(result,born,released)=test_allocation::observe_backing(||operation.close_step(denied));assert!(result.is_err());assert_eq!((born,released),(0,0));assert_eq!(operation.source().as_ptr(),source_pointer);assert_eq!(operation.retirement_receipt(),Default::default());assert!(operation.step(incoming).unwrap().is_none());
    close(&mut operation,&law);let((),born,released)=test_allocation::observe_backing(||drop(operation));assert_eq!((born,released),(0,0));
}

#[test]
fn original_json_operation_debits_post_effect_cancellation_and_keeps_its_cleanup_wallet(){
    let law=law();let source=law["source"].as_str().unwrap();let source_pointer=source.as_ptr();let mut accepted=|event:semio_framework_value::native_decoding::NativeDecodeProgress|event.completed!=1;let mut control=NativeDecodeControl::new_retained(&mut accepted);let mut incoming_policy=policy(&law);incoming_policy.normal.maximum_items=128;
    let mut operation=JsonReadOperation::<_,DslValue>::new(source,JsonMemberPolicy::Reject,incoming_policy,&mut control).unwrap();let(result,born,released)=test_allocation::observe_backing(||operation.step(grant(&law["turn"])));let error=result.unwrap_err();let JsonError::Native(error)=error else{unreachable!()};assert_eq!(error.kind,ValueRefusalKind::Canceled);assert_eq!((born,released),(0,0));assert_eq!(error.retained_progress(),operation.normal_receipt());assert_eq!(operation.normal_receipt().copied_items,1);assert_eq!(operation.source().as_ptr(),source_pointer);assert_eq!(operation.retirement_receipt(),Default::default());close(&mut operation,&law);
}
