//! 📏️ Empty nested containers are refused before birth under the original semantic depth policy.
use super::*;

#[test]
fn original_json_operation_enforces_semantic_container_depth_before_empty_container_birth(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let normal=serde_json::from_value(law["normalAuthority"].clone()).unwrap();let retirement=serde_json::from_value(law["retirementAuthority"].clone()).unwrap();let turn:RetainedCloneGrant=serde_json::from_value(law["turn"].clone()).unwrap();let close_turn:RetainedCloneGrant=serde_json::from_value(law["closeTurn"].clone()).unwrap();let maximum_turns=law["maximumTurns"].as_u64().unwrap();
    for row in law["cases"].as_array().unwrap(){
        let source=row["source"].as_str().unwrap();let expected:serde_json::Value=serde_json::from_str(source).unwrap();let source_pointer=source.as_ptr();let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new_retained(&mut accepted);
        let policy=JsonReadPolicy{limits:JsonReadLimits{maximum_bytes:law["maximumBytes"].as_u64().unwrap(),maximum_allocation_bytes:law["maximumAllocationBytes"].as_u64().unwrap()as usize,maximum_depth:row["maximumDepth"].as_u64().unwrap()as usize,maximum_items:law["maximumItems"].as_u64().unwrap()},normal,retirement};
        let mut operation=JsonReadOperation::<_,semio_framework_value::DslValue>::new(source,JsonMemberPolicy::Reject,policy,&mut control).unwrap();let mut refusal=None;let mut turns=0;
        loop{
            let(result,born,released)=test_allocation::observe_backing(||operation.step(turn).map(|value|value.is_some()).map_err(JsonError::kind));let receipt=operation.normal_step_progress();assert_eq!((born,released),(receipt.retained_capacity_bytes,receipt.released_bytes));assert!(receipt.fits(turn));assert_eq!(operation.source().as_ptr(),source_pointer);
            match result{Ok(true)=>break,Ok(false)=>{},Err(kind)=>{refusal=Some(kind);break;}}
            turns+=1;assert!(turns<maximum_turns);
        }
        let position=operation.position();let normal_birth=operation.normal_receipt().retained_capacity_bytes;
        if let Some(value)=operation.value(){
            assert_eq!(serde_json::from_str::<serde_json::Value>(&to_json_string(value)).unwrap(),expected);
        }
        while !operation.terminal_is_empty(){let(result,born,released)=test_allocation::observe_backing(||operation.close_step(close_turn));let step=result.unwrap();assert_eq!((born,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));turns+=1;assert!(turns<maximum_turns);}
        assert_eq!(refusal.is_none(),row["expected"]=="admitted","source={source} maximumDepth={}",row["maximumDepth"]);
        if row["expected"]=="depthLimit"{assert_eq!(refusal,Some(ValueRefusalKind::DepthLimit));assert_eq!(position,row["refusedPosition"].as_u64().unwrap()as usize);if row["maximumDepth"]==0{assert_eq!(normal_birth,0);}}
        println!("[DEBUG] JSON source={source} original semantic maximumDepth={} position={position} normalBirth={normal_birth} paidClosureTurns={turns} independentSerde=true",row["maximumDepth"]);
    }
}
