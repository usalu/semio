//! 🎟️ Original whole-owner ceilings bound retained turns before address-space arithmetic.
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueRefusalKind};
use std::cell::Cell;
fn amount(value:&serde_json::Value)->usize{match value.as_str(){Some("addressSpace")=>isize::MAX as usize,Some("unsignedMaximum")=>usize::MAX,None=>usize::try_from(value.as_u64().unwrap()).unwrap(),_=>unreachable!()}}
macro_rules! turn_authority_law{
 ($name:ident,$control:ident)=>{
 #[test]
 fn $name(){
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
  for row in fixture["cases"].as_array().unwrap(){
   let id=row["id"].as_str().unwrap();let callbacks=Cell::new(0);let mut observer=|_|{callbacks.set(callbacks.get()+1);true};
   let initial=amount(&row["initialMaximumBytes"]);let owned=amount(&row["ownedBytes"]);
   let mut native=if row["receiving"].as_bool().unwrap(){let mut native=$control::new_retained(&mut observer);native.admit_turn_capacity(initial).unwrap();native}else{$control::new(initial,&mut observer)};
   native.charge(owned).unwrap();let before_callbacks=callbacks.get();
   let mut preflight=|native:&mut $control<'_>|{
    let(result,born,released)=crate::test_allocation::observe_backing(||native.admit_turn_capacity(amount(&row["turnCapacityBytes"])));
    let outcome=match result{Ok(())=>"complete",Err(error)=>{assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit,"{id}");"limit"}};
    assert_eq!(native.owned_bytes(),owned,"{id} preserves cumulative ownership");
    Ok::<_,semio_framework_value::ValueError>((outcome,native.maximum_bytes(),born,released,callbacks.get()-before_callbacks))
   };
   let(outcome,during,born,released,calls)=if row["scopeMaximumBytes"].is_null(){preflight(&mut native).unwrap()}else{native.scoped_maximum(amount(&row["scopeMaximumBytes"]),preflight).unwrap()};
   let actual=serde_json::json!({"outcome":outcome,"maximumDuringBytes":during,"maximumAfterBytes":native.maximum_bytes(),"heapBornBytes":born,"heapReleasedBytes":released,"observerCalls":calls});
   let mut expected=row["expected"].clone();for field in ["maximumDuringBytes","maximumAfterBytes"]{expected[field]=serde_json::json!(amount(&expected[field]));}
   assert_eq!(actual,expected,"{id}: original native authority and physical preflight receipt");
   println!("[DEBUG] native turn {} control={} outcome={} owned={} during={} restored={} heapBorn={} heapReleased={} observerCalls={} independentSerde=true",id,stringify!($control),outcome,owned,during,native.maximum_bytes(),born,released,calls);
  }
 }
 };
}
turn_authority_law!(native_decode_turn_intersects_original_whole_owner_before_address_space,NativeDecodeControl);
turn_authority_law!(native_encode_turn_intersects_original_whole_owner_before_address_space,NativeEncodeControl);
