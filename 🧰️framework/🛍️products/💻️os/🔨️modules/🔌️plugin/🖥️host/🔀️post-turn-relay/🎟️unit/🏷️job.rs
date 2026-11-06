//! 🏷️ Borrowed tagged Job input with independent JSON conformance.
use semio_framework_value::{ValueError,ValueRefusalKind};
/// 🏷️ Borrowed Job discriminator.
pub struct GuestRelayJobInput<'a,S:semio_framework_pack_json::JsonWriteSource+?Sized>(pub &'a S);
impl<S:semio_framework_pack_json::JsonWriteSource+?Sized> semio_framework_pack_json::JsonWriteSource for GuestRelayJobInput<'_,S>{
 fn node_at_path(&self,path:&[usize])->Result<semio_framework_pack_json::JsonWriteNode<'_>,ValueError>{
  match path.split_first(){None=>Ok(semio_framework_pack_json::JsonWriteNode::Object(1)),Some((0,tail))=>self.0.node_at_path(tail),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"bad path"))}
 }
 fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{
  match path.split_first(){None if index==0=>Ok("Job"),Some((0,tail))=>self.0.object_key_at_path(tail,index),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"bad key"))}
 }
}

#[cfg(test)]
mod tests{
 use super::*;use super::super::GuestRelayUnitFlow;use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};use semio_framework_value::{DslValue,RetainedCloneGrant};fn clock()->Option<u64>{Some(1000)}
 #[test]
 fn tagged_job_matches_serde(){
  let law:serde_json::Value=serde_json::from_str(include_str!("law.json")).unwrap();let raw=serde_json::to_string(&law["jobSource"]).unwrap();let view:DslValue=semio_framework_pack_json::from_json_str(&raw,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let grant=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:4096,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};let mut caller=OriginalJobFixtureCaller::admit(OperationId(793),Generation(1),grant,clock,1000000,1000000);let mut flow=GuestRelayUnitFlow::empty();let mut epoch=0;
  for _ in 0..4096{epoch+=1;if caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;flow.encode_step(&GuestRelayJobInput(&view),cx)}).unwrap(){break}}assert!(flow.encoded().is_some());assert_eq!(serde_json::from_slice::<serde_json::Value>(flow.encoded().unwrap()).unwrap(),serde_json::json!({"Job":law["jobSource"]}));
  caller.cancel().cancel_now();for _ in 0..4096{epoch+=1;if caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;cx.with_original_retirement(|cx|flow.close_unopened(cx))}).unwrap(){break}}assert!(flow.terminal_is_empty());caller.retire(grant);
 }
}
