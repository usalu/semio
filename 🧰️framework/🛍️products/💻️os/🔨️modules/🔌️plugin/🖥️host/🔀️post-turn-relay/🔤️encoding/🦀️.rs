//! 🎟️ Actor budget JSON and its neutral funded Source encoder.
pub use semio_framework_pack_json_source::SourceJsonEncoder;
use semio_framework_value::{ValueError,ValueRefusalKind};
fn refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}

/// 🎟️ Actor admission and closure encode the exact total header plus the narrower purpose scope without projection allocations.
pub struct OriginalJobBudgetJson(pub semio_framework_actor::OriginalJobBudget);
impl semio_framework_pack_json::JsonWriteSource for OriginalJobBudgetJson{
 fn node_at_path(&self,path:&[usize])->Result<semio_framework_pack_json::JsonWriteNode<'_>,ValueError>{use semio_framework_pack_json::JsonWriteNode as N;use semio_framework_value::Number;let b=self.0;let number=|value|Ok(N::Number(Number::UInt(value)));match path{[]|[0]=>Ok(N::Object(5)),[0,0]=>number(b.input.operation),[0,1]=>number(b.input.generation),[0,2]=>number(b.input.epoch),[0,3]=>Ok(N::Object(3)),[0,3,0]=>number(b.input.work.clock_binding),[0,3,1]=>number(b.input.work.absolute_deadline_us),[0,3,2]=>number(b.input.work.fuel),[0,4,tail@..]=>grant_node(b.input.grant,tail),[1,tail@..]=>grant_node(b.retained_policy,tail),[2]=>number(b.fuel),[3]=>number(b.deadline_us),[4]=>Ok(N::Bool(b.cancelled)),_=>Err(refused("budget JSON path is outside the declared original schema"))}}
 fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{let keys:&[&str]=match path{[]=>&["input","retainedPolicy","fuel","deadlineUs","cancelled"],[0]=>&["operation","generation","epoch","work","grant"],[0,3]=>&["clockBinding","absoluteDeadlineUs","fuel"],[0,4]|[1]=>&["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumReleaseBytes","maximumDepth"],_=>return Err(refused("budget JSON object path is absent"))};keys.get(index).copied().ok_or_else(||refused("budget JSON member is absent"))}
}
fn grant_node(grant:semio_framework_value::RetainedCloneGrant,path:&[usize])->Result<semio_framework_pack_json::JsonWriteNode<'static>,ValueError>{use semio_framework_pack_json::JsonWriteNode as N;match path{[]=>Ok(N::Object(5)),[index]=>{let values=[grant.maximum_items,grant.maximum_copy_bytes,grant.maximum_capacity_bytes,grant.maximum_release_bytes,grant.maximum_depth];let value=values.get(*index).ok_or_else(||refused("budget JSON currency member is absent"))?;Ok(N::Number(semio_framework_value::Number::UInt(u64::try_from(*value).map_err(|_|refused("budget JSON currency exceeds its wire extent"))?)))},_=>Err(refused("budget JSON currency path is absent"))}}


#[cfg(test)]
mod tests{
 use super::*;
 use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
 use semio_framework_value::RetainedCloneGrant;
 use semio_framework_value::retirement::controlled::ControlledRetirement;
 fn clock()->Option<u64>{Some(1000)}
 #[test]
 fn exact_scoped_budget_view_matches_serde_without_projected_heap(){
  let law:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let budget:semio_framework_actor::OriginalJobBudget=serde_json::from_value(law["budget"].clone()).unwrap();assert_eq!(serde_json::to_string(&budget).unwrap(),law["expectedBudget"].as_str().unwrap());let grant=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:1048576,maximum_capacity_bytes:4194304,maximum_release_bytes:4194304,maximum_depth:64};let mut caller=OriginalJobFixtureCaller::admit(OperationId(71),Generation(3),grant,clock,1000000,1000000);let mut encoder=SourceJsonEncoder::new();let mut output=None;for epoch in 1..law["maximumSlices"].as_u64().unwrap(){if caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;encoder.advance(&OriginalJobBudgetJson(budget),&mut output,cx)}).unwrap(){break}}assert_eq!(std::str::from_utf8(output.as_ref().unwrap()).unwrap(),law["expectedBudget"].as_str().unwrap());
  let mut output=ControlledRetirement::retain(output.take().unwrap());for _ in 0..4096{if caller.turn(grant,|cx|{let grant=cx.retained_grant();let Some((poll,_))=cx.with_original_effect(grant,|_,effect|output.step(effect))? else{return Ok(false)};let turn=semio_framework_value::RetirementPollExit::settled(poll).ok().unwrap();semio_framework_job::receive_original_retained_work(cx,turn).map(|step|matches!(step,semio_framework_value::RetainedCloneStep::Complete(_)))}).unwrap(){break}}assert!(output.terminal_is_empty());for _ in 0..4096{if caller.turn(grant,|cx|encoder.close_step(cx)).unwrap(){break}}assert!(encoder.terminal_is_empty());caller.retire(grant);
 }

}

