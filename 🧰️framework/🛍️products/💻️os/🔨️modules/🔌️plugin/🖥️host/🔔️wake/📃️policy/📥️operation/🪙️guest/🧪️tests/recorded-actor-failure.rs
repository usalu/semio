use semio_framework_actor::ActorFailureCallKind;
use semio_plugin_guest_contract::original_actor_context::{
 ActorFailureFixedReceipt, ActorFailureStatus, OriginalActorFixedKind, OriginalActorFixedReceipt,
 validate_original_source_header_body,
};

#[test]
fn authentic_note_failure_receipts_preserve_cause_and_terminal_handback() {
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/recorded-actor-failure.json")).unwrap();
 let reference=law["failedEpoch"].as_u64().unwrap();
 let size=law["publicationLength"].as_u64().unwrap();
 let mut call_epoch=reference;
 let mut steps=Vec::new();
 for row in law["frames"].as_array().unwrap() {
  let source=row["rawHex"].as_str().or_else(||row["header"].as_str()).unwrap();
  let bytes:Vec<_>=(0..source.len()).step_by(2).map(|index|u8::from_str_radix(&source[index..index+2],16).unwrap()).collect();
  validate_original_source_header_body(&bytes).unwrap();
  if row["stage"]=="genuine-poll-failure" {
   let (call,_)=OriginalActorFixedReceipt::read(&bytes).unwrap();
   assert_eq!(call.input.epoch,reference);
   assert!(matches!(call.kind,OriginalActorFixedKind::Failure{authority:None}));
   continue;
  }
  assert!(OriginalActorFixedReceipt::read(&bytes).is_err());
  let (call,escrow)=ActorFailureFixedReceipt::read(&bytes).unwrap();
  assert_eq!(call.reference.failed_epoch,reference);
  assert_eq!(call.reference.call_kind,ActorFailureCallKind::Poll);
  assert!(call.input.epoch>call_epoch);
  assert!(call.refusal.is_none()&&escrow.pointer!=0);
  call_epoch=call.input.epoch;
  steps.push(match call.status {
   ActorFailureStatus::Publish{outcome:Some(outcome)}=>{assert_eq!(outcome.byte_length,size);1},
   ActorFailureStatus::Read{chunk:Some(chunk)} if !chunk.output_closed=>{assert_eq!((chunk.offset,chunk.byte_length),(0,Some(size)));2},
   ActorFailureStatus::AcknowledgeChunk{acknowledged:true}=>3,
   ActorFailureStatus::Read{chunk:Some(chunk)}=>{assert_eq!((chunk.offset,chunk.byte_length),(size,None));4},
   ActorFailureStatus::Acknowledge{acknowledged:true}=>5,
   status=>panic!("unexpected recovered failure status {status:?}"),
  });
 }
 assert_eq!(steps,[1,2,3,4,5]);
}
