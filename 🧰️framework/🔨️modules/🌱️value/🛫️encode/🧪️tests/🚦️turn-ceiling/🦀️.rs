//! 🚦️ A local control keeps its caller ceiling across turns that grant no capacity.
use super::*;

#[test]
fn local_control_keeps_the_caller_ceiling_across_zero_capacity_turns(){
    let fixture:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🧪️tests/🚦️turn-ceiling/🧫️fixtures/🔣️.json")).unwrap();
    let ceiling=fixture["ceilingBytes"].as_u64().unwrap()as usize;let funded=fixture["fundedTurnBytes"].as_u64().unwrap()as usize;let refused=fixture["refusedBytes"].as_u64().unwrap()as usize;
    let mut admitted=|_:NativeEncodeProgress|true;let mut control=NativeEncodeControl::new(ceiling,&mut admitted);
    for _ in 0..fixture["zeroCapacityTurns"].as_u64().unwrap(){control.admit_turn_capacity(0).unwrap();assert_eq!(control.maximum_bytes(),0);}
    control.admit_turn_capacity(funded).unwrap();control.charge(funded).unwrap();assert_eq!(control.owned_bytes(),funded);
    control.admit_turn_capacity(refused).unwrap();assert_eq!(control.charge(refused).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);
}

#[test]
fn iterative_depth_observes_original_source_and_enclosing_scope(){
 crate::eff::with_recorded_source(0,2,|control|{
 assert!(control.check_additional_depth(2).is_ok());assert_eq!(control.check_additional_depth(3).unwrap_err().kind,ValueRefusalKind::DepthLimit);
 control.scoped_depth(2,|same|->Result<(),ValueError>{assert!(same.check_additional_depth(1).is_ok());assert_eq!(same.check_additional_depth(2).unwrap_err().kind,ValueRefusalKind::DepthLimit);Ok(())}).unwrap();assert!(control.check_additional_depth(2).is_ok());assert_eq!(control.check_additional_depth(usize::MAX).unwrap_err().kind,ValueRefusalKind::DepthLimit);
 });
}

#[test]
fn iterative_scope_checks_enclosing_depth_and_restores_after_refusal(){
 crate::eff::with_recorded_source(0,2,|control|{
 let result=control.scoped_additional_depth(2,|same|->Result<(),ValueError>{assert_eq!(same.check_additional_depth(1).unwrap_err().kind,ValueRefusalKind::DepthLimit);Err(ValueError::literal(ValueRefusalKind::Canceled,"fixture refusal"))});assert_eq!(result.unwrap_err().kind,ValueRefusalKind::Canceled);assert!(control.check_additional_depth(2).is_ok());let result=control.scoped_additional_depth(3,|_|->Result<(),ValueError>{panic!("denied depth executed effect")});assert_eq!(result.unwrap_err().kind,ValueRefusalKind::DepthLimit);assert!(control.check_additional_depth(2).is_ok());
 });
}
