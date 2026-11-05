//! 🧪️ The complete invocation dispatch owns its exact fields under the original per-turn grants.
use super::*;
fn grant(demand:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}
fn close(work:&mut FlowInvocationDispatchWork){work.begin_close();for _ in 0..100000{if work.terminal_is_empty(){return}let demand=work.next_close_demands(4096).unwrap();work.close_step(grant(demand)).unwrap();}panic!("original dispatch did not close")}
fn text(row:&semio_framework_pack_json::Value,key:&str)->String{row.get(key).unwrap().as_str().unwrap().to_string()}
#[test]
fn original_dispatch_wire_matches_all_genuine_invocation_fields(){
 let cases=semio_framework_pack_json::parse(include_str!("📁️cases.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let requests=semio_framework_pack_json::parse(include_str!("../../🧾️request-work/🧪️laws/📁️cases.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 for case in cases.as_array().unwrap(){
  let row=&requests.as_array().unwrap()[case.get("requestCase").unwrap().as_u64().unwrap()as usize];
  let hash=row.get("nodeHash").unwrap();let node_hash=hash.as_str().map(|value|value.parse::<u64>().unwrap()).or_else(||hash.as_u64()).unwrap();
  let origin=FlowInvocationOrigin{window_id:text(row,"windowId"),window_kind_id:text(row,"windowKindId"),neuron_id:text(row,"neuronId"),extension_id:"geometry-🧊️".into(),operator_id:text(row,"operatorId"),node_hash,input_json:text(row,"inputJson"),dependency_json:text(row,"dependencyJson"),operator_version:text(row,"operatorVersion"),outer_node_hash:None,preview_source:None};
  let resume=row.get("resume").unwrap().as_bool().unwrap();let mut work=FlowInvocationDispatchWork::new(resume);let mut output=None;
  for _ in 0..200000{let demand=work.next_demands(&origin,4096).unwrap();let original_grant=grant(demand);assert!(work.advance(&origin,RetainedCloneGrant{maximum_items:0,..original_grant}).unwrap().is_none());let result=work.advance(&origin,original_grant).unwrap();let progress=work.step_progress();assert!(progress.copied_items<=1&&progress.copied_bytes<=original_grant.maximum_copy_bytes&&progress.retained_capacity_bytes<=original_grant.maximum_capacity_bytes&&progress.released_bytes<=original_grant.maximum_release_bytes);if let Some(dispatch)=result{output=Some(dispatch);break}}
  let output=output.expect("original dispatch transfers once");assert_eq!(output.extension_id,origin.extension_id);assert_eq!(output.capability,case.get("capability").unwrap().as_str().unwrap());assert_eq!(output.response_action,case.get("responseAction").unwrap().as_str().unwrap());assert_eq!(output.request_json,origin.request_json_cold(resume));assert_eq!(output.request_json,row.get("expected").unwrap().as_str().unwrap());work.restore_output(output);close(&mut work);
 }
}
#[test]
fn original_dispatch_cancellation_closes_its_partial_wire(){
 let origin=FlowInvocationOrigin{window_id:"window".into(),window_kind_id:"preview".into(),neuron_id:"node".into(),extension_id:"extension".into(),operator_id:"operator".into(),node_hash:7,input_json:"{}".into(),dependency_json:"{}".into(),operator_version:"v".into(),outer_node_hash:None,preview_source:None};
 for stop in 0..50{let mut work=FlowInvocationDispatchWork::new(false);for _ in 0..stop{let demand=work.next_demands(&origin,4096).unwrap();assert!(work.advance(&origin,grant(demand)).unwrap().is_none())}close(&mut work)}
}
