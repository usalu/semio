//! 📏️ Original retained turns intersect every borrowed incoming native ceiling.
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueRefusalKind};
use std::panic::{catch_unwind,AssertUnwindSafe};
macro_rules! maximum_law{
 ($name:ident,$control:ident)=>{
 #[test]
 fn $name(){
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
  for row in fixture["cases"].as_array().unwrap(){
   let id=row["id"].as_str().unwrap();
   let mut events=Vec::new();
   let mut callback=|event|{events.push(event);true};
   let mut native=$control::new_retained(&mut callback);
   native.admit_turn_capacity(fixture["firstTurnCapacityBytes"].as_u64().unwrap()as usize).unwrap();
   native.charge(fixture["initialBytes"].as_u64().unwrap()as usize).unwrap();
   let mut observer=|_|id!="cancel";
   let result=catch_unwind(AssertUnwindSafe(||native.scoped_maximum(fixture["scopeMaximumBytes"].as_u64().unwrap()as usize,|native|native.scoped_observer(&mut observer,|native|{
    native.admit_turn_capacity(fixture["nextTurnCapacityBytes"].as_u64().unwrap()as usize)?;
    assert_eq!(native.maximum_bytes(),10,"original scoped ceiling survives independent turn admission");
    native.begin_stage(0)?;
    match id{
     "repeat"=>{native.charge(3)?;native.admit_turn_capacity(16)?;assert_eq!(native.maximum_bytes(),10);native.charge(1)},
     "nested"=>{native.scoped_maximum(9,|native|{native.admit_turn_capacity(16)?;assert_eq!(native.maximum_bytes(),9);native.charge(2)})?;assert_eq!(native.maximum_bytes(),10);native.charge(1)},
     "refuse"=>native.charge(4),
     "cancel"=>unreachable!("original observer must refuse before ownership changes"),
     "unwind"=>{native.charge(3)?;panic!("authored native scope unwind")},
     _=>unreachable!()
    }
   }))));
   let outcome=match result{Ok(Ok(()))=>"complete",Ok(Err(error))=>match error.kind{ValueRefusalKind::OwnershipLimit=>"limit",ValueRefusalKind::Canceled=>"canceled",_=>panic!("unexpected native scope refusal")},Err(_)=>"unwind"};
   assert_eq!(outcome,row["outcome"].as_str().unwrap(),"{id}");
   assert_eq!(native.owned_bytes(),row["ownedBytes"].as_u64().unwrap()as usize,"{id}");
   assert_eq!(native.maximum_bytes(),fixture["restoredMaximumBytes"].as_u64().unwrap()as usize,"{id}");
   native.admit_turn_capacity(16).unwrap();
   assert_eq!(native.maximum_bytes(),native.owned_bytes()+16,"{id} original retained identity survives the borrowed scope");
   drop(native);
   assert!(!events.is_empty());
   println!("[DEBUG] native scope {} outcome={} owned={}",id,outcome,row["ownedBytes"]);
  }
 }
 };
}
maximum_law!(native_decode_scoped_maximum_preserves_original_turn_authority,NativeDecodeControl);
maximum_law!(native_encode_scoped_maximum_preserves_original_turn_authority,NativeEncodeControl);


macro_rules! detached_receiving_law{
 ($name:ident,$control:ident,$forwarded:path,$recipient:path)=>{
 #[test]
 fn $name(){
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔌️detached/🔣️.json")).unwrap();
  let mut recipient=<$recipient>::new();let mut foreign=<$recipient>::new();let mut allocation_calls=Vec::new();let mut observe=|_|true;
  let mut allocate=|next|{allocation_calls.push(next);Ok(())};
  let mut native=$control::new_forwarded(fixture["maximumBytes"].as_u64().unwrap()as usize,&mut observe,&mut allocate);
  native.install_retirement_recipient(&mut recipient).unwrap();
  let first=native.copy_text(fixture["firstText"].as_str().unwrap()).unwrap();assert_eq!(serde_json::to_value(&first).unwrap(),fixture["firstText"]);assert_eq!(first.len(),fixture["firstOwnedBytes"].as_u64().unwrap()as usize);
  let receipt=match native.detach(){Ok(receipt)=>receipt,Err((error,_))=>panic!("original native detach: {error}")};
  let receipt=match <$forwarded>::rebind(receipt,&mut observe,&mut allocate,&mut foreign){Err((error,receipt))=>{assert_eq!(error.kind,ValueRefusalKind::InvariantViolated);receipt},Ok(_)=>panic!("foreign recipient gained original receipt")};
  assert_eq!(receipt.owned_bytes(),first.len());assert_eq!(receipt.maximum_bytes(),fixture["maximumBytes"].as_u64().unwrap()as usize);
  let mut native=match <$forwarded>::rebind(receipt,&mut observe,&mut allocate,&mut recipient){Ok(native)=>native,Err((error,_))=>panic!("same original recipient: {error}")};
  let second=native.copy_text(fixture["secondText"].as_str().unwrap()).unwrap();assert_eq!(serde_json::to_value(&second).unwrap(),fixture["secondText"]);assert_eq!(second.len(),fixture["secondOwnedBytes"].as_u64().unwrap()as usize);
  assert_eq!(native.owned_bytes(),fixture["finalOwnedBytes"].as_u64().unwrap()as usize);assert_eq!(native.maximum_bytes(),fixture["maximumBytes"].as_u64().unwrap()as usize);
  let receipt=match native.detach(){Ok(receipt)=>receipt,Err((error,_))=>panic!("resumed native detach: {error}")};
  assert_eq!(receipt.owned_bytes(),first.len()+second.len());assert_eq!(allocation_calls.len(),2);assert_eq!(allocation_calls[0].owned_bytes,0);assert_eq!(allocation_calls[1].owned_bytes,first.len());
  println!("[DEBUG] actual detached {} sameRecipient=true wrongRecipientRefused=true first={} cumulative={} originalCeiling={} independentSerde=true",stringify!($control),first.len(),receipt.owned_bytes(),receipt.maximum_bytes());
 }
 };
}
detached_receiving_law!(native_decode_scoped_maximum_detached_receiving_preserves_original_recipient,NativeDecodeControl,semio_framework_value::native_decoding::NativeForwardedDecodeControl,semio_framework_value::native_decoding::NativeDecodeRetirementRecipient);
detached_receiving_law!(native_encode_scoped_maximum_detached_receiving_preserves_original_recipient,NativeEncodeControl,semio_framework_value::native_encoding::NativeForwardedEncodeControl,semio_framework_value::native_encoding::NativeEncodeRetirementRecipient);
