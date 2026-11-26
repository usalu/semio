use super::*;
use semio_framework_job::{Generation,OperationId};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use semio_framework_value::RetainedCloneGrant;
use semio_plugin_host_guest_runtime::OwnedSemioExport;

fn clock()->Option<u64>{Some(1)}

#[test]
fn actor_failure_selector_qualifies_frozen_export_producer_and_extent(){
 let stock=RetainedCloneGrant{maximum_items:1_000_000,maximum_copy_bytes:32_000_000,maximum_capacity_bytes:32_000_000,maximum_release_bytes:32_000_000,maximum_depth:64};
 let law:serde_json::Value=serde_json::from_str(include_str!("frozen-frontier.json")).unwrap();
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(889),Generation(1),stock,clock,100_000_000,10_000_000);
 caller.admit_return_table();
 let mut leases=[None,None];
 for lease in &mut leases{
  let mut seed=Some(ActorFailureHome::empty());
  assert!((0..128).any(|_|caller.turn(stock,|cx|cx.admit_original_native(&mut seed,lease)).unwrap()));
 }
 let producer=leases[0].as_ref().unwrap().recipient_identity();
 let other=leases[1].as_ref().unwrap().recipient_identity();
 assert_ne!(producer,other);
 for row in law["cases"].as_array().unwrap(){
  let call=ActorFailureCallKind::from_tag(row["kind"].as_u64().unwrap()).unwrap();
  let export_kind=ActorFailureCallKind::from_tag(row["exportTag"].as_u64().unwrap()).unwrap();
  let export=OwnedSemioExport::from_actor_failure_call_kind(export_kind).unwrap();
  let actual=if row["sameProducer"].as_bool().unwrap(){producer}else{other};
  let length=row["length"].as_u64().unwrap()as usize;
  assert_eq!(verify_frozen_failure(call,producer,export,actual,length).is_ok(),row["accepted"].as_bool().unwrap());
 }
 for lease in &mut leases{
  assert!((0..128).any(|_|caller.retirement_turn(stock,|cx|OriginalNativeLease::return_slot_step(lease,cx)).unwrap()));
 }
 assert!((0..128).any(|_|caller.retirement_turn(stock,|cx|cx.close_original_native_step()).unwrap()));
 caller.retire(stock);
}
