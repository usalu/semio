//! 🚦️ A local control keeps its caller ceiling across turns that grant no capacity.
use super::*;

#[test]
fn local_control_keeps_the_caller_ceiling_across_zero_capacity_turns(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let ceiling=fixture["ceilingBytes"].as_u64().unwrap()as usize;let funded=fixture["fundedTurnBytes"].as_u64().unwrap()as usize;let refused=fixture["refusedBytes"].as_u64().unwrap()as usize;
    let mut admitted=|_:NativeEncodeProgress|true;let mut control=NativeEncodeControl::new(ceiling,&mut admitted);
    for _ in 0..fixture["zeroCapacityTurns"].as_u64().unwrap(){control.admit_turn_capacity(0).unwrap();assert_eq!(control.maximum_bytes(),0);}
    control.admit_turn_capacity(funded).unwrap();control.charge(funded).unwrap();assert_eq!(control.owned_bytes(),funded);
    control.admit_turn_capacity(refused).unwrap();assert_eq!(control.charge(refused).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);
}
