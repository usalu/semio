//! ♻️ Actual clone close-frame private custody preserves original terminal release laws.
use semio_framework_test_fixtures::fixture_text;
use super::*;
use crate::observe_retirement_allocations;
#[test]
fn retained_clone_close_depth_observes_pending_birth_and_preserves_owner_on_refusal() {
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/🪜️close-demands/🔣️.json")).unwrap();
    let mut cursor=crate::retained_clone::ScalarCursor::<u64>::default();
    cursor.value=Some(law["owner"].as_u64().unwrap());
    assert_eq!(cursor.next_close_depth_demand().unwrap(),law["idleDepth"].as_u64().unwrap()as usize);
    let started=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:cursor.begin_close_demand().unwrap().copy_bytes,maximum_depth:1,..Default::default()};effect::begin_close_paid(&mut cursor,started).result.unwrap();
    let ((depth,capacity),heap)=observe_retirement_allocations(||(cursor.next_close_depth_demand().unwrap(),cursor.next_close_capacity_byte_demand(0).unwrap()));
    assert_eq!(heap,(0,0));assert_eq!(depth,law["pendingBirthDepth"].as_u64().unwrap()as usize);
    let denied=RetainedCloneGrant{maximum_items:law["refusedBirth"]["maximumItems"].as_u64().unwrap()as usize,maximum_capacity_bytes:capacity,maximum_depth:law["refusedBirth"]["maximumDepth"].as_u64().unwrap()as usize,..Default::default()};
    let(result,heap)=observe_retirement_allocations(||effect::close_step(&mut cursor,denied));
    assert_eq!(heap,(0,0));assert_eq!(result.result.unwrap().progress(),Default::default());assert_eq!(cursor.value,Some(law["owner"].as_u64().unwrap()));
    let birth=RetainedCloneGrant{maximum_items:law["fundedBirth"]["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:cursor.next_close_copy_byte_demand().unwrap(),maximum_capacity_bytes:capacity,maximum_depth:depth,..Default::default()};
    let(step,heap)=observe_retirement_allocations(||effect::close_step(&mut cursor,birth).result.unwrap());
    assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(step.progress().fits(birth));assert!(cursor.value.is_none());
    for _ in 0..law["maximumTurns"].as_u64().unwrap(){
        if cursor.terminal_is_empty(){break;}
        let d=cursor.close_demands(4096).unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(cursor.close_demands(grant.maximum_copy_bytes).unwrap(),d);
        let(step,heap)=observe_retirement_allocations(||effect::close_step(&mut cursor,grant).result.unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
    }
    assert!(cursor.terminal_is_empty());assert_eq!(cursor.next_close_depth_demand().unwrap(),law["terminalDepth"].as_u64().unwrap()as usize);
}

