//! 🧪️ Initial origin publication preserves genuine pending field allocations and paid custody.
use super::*;
use semio_framework_value::FromValue;
fn grant(demand:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}
fn close(work:&mut FlowInvocationOriginWork){work.begin_close();for _ in 0..100000{if work.terminal_is_empty(){return}let demand=work.next_close_demands(4096).unwrap();work.close_step(grant(demand)).unwrap();}panic!("original origin did not close")}
fn text(row:&semio_framework_pack_json::Value,key:&str)->String{row.get(key).unwrap().as_str().unwrap().to_string()}
#[test]
fn initial_origin_work_preserves_original_pending_fields_and_dependency(){
 let cases=semio_framework_pack_json::parse(include_str!("📁️cases.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let sources=semio_framework_pack_json::parse(include_str!("../../🌳️dependency-work/🧪️laws/📁️cases.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 for row in cases.as_array().unwrap(){
  let source_row=sources.as_array().unwrap().iter().find(|source|source.get("name").unwrap().as_str()==row.get("source").unwrap().as_str()).unwrap();
  let mut source=FlowHostSnapshot::default();source.widgets=Vec::<Widget>::from_value(semio_framework_pack_json::to_dsl_value(source_row.get("widgets").unwrap())).unwrap();source.synapses=Vec::<SynapseSpec>::from_value(semio_framework_pack_json::to_dsl_value(source_row.get("synapses").unwrap())).unwrap();
  let pending=row.get("pending").unwrap();let pending=neural::PendingExtensionEval{neuron_id:text(pending,"neuronId"),extension_id:text(pending,"extensionId"),operator_id:text(pending,"operatorId"),node_hash:pending.get("nodeHash").unwrap().as_u64().unwrap(),input_json:text(pending,"inputJson")};
  let identity=[pending.neuron_id.as_ptr()as usize,pending.extension_id.as_ptr()as usize,pending.operator_id.as_ptr()as usize,pending.input_json.as_ptr()as usize];let hash=pending.node_hash;
  let window=row.get("windowId").unwrap().as_str().unwrap();let kind=row.get("windowKindId").unwrap().as_str().unwrap();let generation=row.get("registryGeneration").unwrap().as_u64().unwrap();
  let mut work=FlowInvocationOriginWork::new(pending,generation);let mut output=None;
  for _ in 0..200000{let demand=work.next_demands(&source,window,kind,4096).unwrap();let original_grant=grant(demand);let phase=work.test_diagnostic();assert!(work.advance(&source,window,kind,RetainedCloneGrant{maximum_items:0,..original_grant}).unwrap().is_none());assert_eq!(work.test_diagnostic(),phase);
   let result=work.advance(&source,window,kind,original_grant).unwrap();let progress=work.step_progress();assert!(progress.copied_items<=1&&progress.copied_bytes<=original_grant.maximum_copy_bytes&&progress.retained_capacity_bytes<=original_grant.maximum_capacity_bytes&&progress.released_bytes<=original_grant.maximum_release_bytes);if let Some(origin)=result{output=Some(origin);break}
  }
  let origin=output.expect("original origin is transferred once");assert_eq!(origin.window_id,window);assert_eq!(origin.window_kind_id,kind);assert_eq!(origin.node_hash,hash);assert_eq!(origin.operator_version,format!("registry:{generation};geometry:1;policy:1"));assert_eq!(origin.dependency_json,source_row.get("expected").unwrap().as_str().unwrap());assert_eq!([origin.neuron_id.as_ptr()as usize,origin.extension_id.as_ptr()as usize,origin.operator_id.as_ptr()as usize,origin.input_json.as_ptr()as usize],identity);
  work.restore_output(origin);close(&mut work);source.retire_cold();
 }
}
#[test]
fn initial_origin_cancellation_preserves_and_closes_its_pending_source(){
 let mut source=FlowHostSnapshot::default();source.widgets.push(Widget::InputNote{id:"node".into(),text:"source".into()});
 for stop in 0..50{let pending=neural::PendingExtensionEval{neuron_id:"node".into(),extension_id:"extension".into(),operator_id:"operator".into(),node_hash:7,input_json:"{}".into()};let mut work=FlowInvocationOriginWork::new(pending,3);for _ in 0..stop{let demand=work.next_demands(&source,"window","preview",4096).unwrap();assert!(work.advance(&source,"window","preview",grant(demand)).unwrap().is_none())}close(&mut work)}
 source.retire_cold();
}
