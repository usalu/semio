use super::*;
#[test]
fn native_decoder_recipient_records_only_actual_close_work(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let grant:RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();
 let mut recipient=NativeDecodeRetirementRecipient::new();
 recipient.reserve().unwrap();
 let mut observations=Vec::new();
 for (label,g) in [
  ("deniedItems",RetainedCloneGrant{maximum_items:0,..grant}),
  ("deniedDepth",RetainedCloneGrant{maximum_depth:0,..grant}),
  ("paidClear",grant),("cachedComplete",grant)
 ]{
  let turn=ErasedSnapshotRetirement::close_step(&mut recipient,g);
  let p=match turn.result{
   Ok(step)=>step.progress(),
   Err(error)=>{assert_eq!(error.kind,crate::ValueRefusalKind::DepthLimit);error.retained_progress()}
  };
  assert_eq!((p.copied_bytes,p.retained_capacity_bytes,p.released_bytes),(0,0,0));
  observations.push(serde_json::json!({"label":label,"work":turn.work,"copiedItems":p.copied_items,"reserved":recipient.is_reserved()}));
 }
 assert_eq!(serde_json::Value::Array(observations),law["expected"]);
 assert!(ErasedSnapshotRetirement::terminal_is_empty(&recipient));
}
