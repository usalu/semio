//! 🧪️ The borrowed dependency continuation agrees with the independent cold ancestry oracle.
use super::*;
use semio_framework_value::FromValue;
fn grant(demand:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}
fn close(work:&mut FlowInferenceDependencyWork){work.begin_close();for _ in 0..100000{if work.terminal_is_empty(){return}let demand=work.next_close_demands(4096).unwrap();work.close_step(grant(demand)).unwrap();}panic!("original dependency did not close")}
#[test]
fn original_dependency_work_matches_genuine_cold_and_neutral_ancestry(){
 let cases=semio_framework_pack_json::parse(include_str!("📁️cases.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 for row in cases.as_array().unwrap(){
  let target=row.get("target").unwrap().as_str().unwrap();
  let mut source=FlowHostSnapshot::default();source.widgets=Vec::<Widget>::from_value(semio_framework_pack_json::to_dsl_value(row.get("widgets").unwrap())).unwrap();source.synapses=Vec::<SynapseSpec>::from_value(semio_framework_pack_json::to_dsl_value(row.get("synapses").unwrap())).unwrap();
  let expected=semio_os_flow_host_core::testkit::flow_inference_dependency_json(&source,target);
  let mut work=FlowInferenceDependencyWork::new();let mut output=None;
  for _ in 0..100000{
   let demand=work.next_demands(&source,target,4096).unwrap();let before=work.test_diagnostic();let original_grant=grant(demand);
   assert!(work.advance(&source,target,RetainedCloneGrant{maximum_items:0,..original_grant}).unwrap().is_none());assert_eq!(work.test_diagnostic(),before);
   let result=work.advance(&source,target,original_grant).unwrap();let progress=work.step_progress();assert!(progress.copied_items<=1&&progress.copied_bytes<=original_grant.maximum_copy_bytes&&progress.retained_capacity_bytes<=original_grant.maximum_capacity_bytes&&progress.released_bytes<=original_grant.maximum_release_bytes);
   if let Some(text)=result{output=Some(text);break}
  }
  let output=output.expect("original dependency transfers once");assert_eq!(output,expected,"{}",row.get("name").unwrap().as_str().unwrap());assert_eq!(output,row.get("expected").unwrap().as_str().unwrap());
  work.restore_output(output);close(&mut work);source.retire_cold();
 }
}
#[test]
fn original_dependency_cancellation_closes_each_live_phase(){
 let mut source=FlowHostSnapshot::default();source.widgets.push(Widget::InputNote{id:"node".into(),text:"source".into()});let target="node";
 for stop in 0..25{
  let mut work=FlowInferenceDependencyWork::new();for _ in 0..stop{let demand=work.next_demands(&source,target,4096).unwrap();assert!(work.advance(&source,target,grant(demand)).unwrap().is_none())}close(&mut work);
 }
 source.retire_cold();
}
