//! 🧪️ The original invocation encoder agrees with cold Host serialization and the neutral wire oracle.
use super::*;
fn text(row:&semio_framework_pack_json::Value,key:&str)->String{row.get(key).unwrap().as_str().unwrap().to_string()}
fn grant(demand:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}
fn close(work:&mut FlowInvocationRequestWork){work.begin_close();for _ in 0..100000{if work.terminal_is_empty(){return}let demand=work.next_close_demands(4096).unwrap();work.close_step(grant(demand)).unwrap();}panic!("original invocation request did not close")}
#[test]
fn original_invocation_request_matches_cold_and_neutral_json(){
 let cases=semio_framework_pack_json::parse(include_str!("📁️cases.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 for row in cases.as_array().unwrap(){
  let hash=row.get("nodeHash").unwrap();let node_hash=hash.as_str().map(|value|value.parse::<u64>().unwrap()).or_else(||hash.as_u64()).unwrap();
  let origin=FlowInvocationOrigin{window_id:text(row,"windowId"),window_kind_id:text(row,"windowKindId"),neuron_id:text(row,"neuronId"),extension_id:"extension".into(),operator_id:text(row,"operatorId"),node_hash,input_json:text(row,"inputJson"),dependency_json:text(row,"dependencyJson"),operator_version:text(row,"operatorVersion"),outer_node_hash:None,preview_source:None};
  let resume=row.get("resume").unwrap().as_bool().unwrap();let mut work=FlowInvocationRequestWork::new(resume);let mut output=None;
  for _ in 0..100000{let demand=work.next_demands(&origin,4096).unwrap();let before=work.test_diagnostic();let denied=RetainedCloneGrant{maximum_items:0,..grant(demand)};assert!(work.advance(&origin,denied).unwrap().is_none());assert_eq!(work.test_diagnostic(),before);if let Some(text)=work.advance(&origin,grant(demand)).unwrap(){output=Some(text);break}}
  let output=output.expect("original invocation request transfers once");assert_eq!(output,origin.request_json_cold(resume));assert_eq!(output,text(row,"expected"));
  work.restore_output(output);close(&mut work);
 }
}
#[test]
fn original_invocation_request_refuses_changed_source_and_retains_its_output(){
 let origin=FlowInvocationOrigin{window_id:"a".into(),window_kind_id:"preview".into(),neuron_id:"n".into(),extension_id:"e".into(),operator_id:"o".into(),node_hash:7,input_json:"{}".into(),dependency_json:"{}".into(),operator_version:"v".into(),outer_node_hash:None,preview_source:None};
 let other=FlowInvocationOrigin{window_id:"b".into(),window_kind_id:"preview".into(),neuron_id:"n".into(),extension_id:"e".into(),operator_id:"o".into(),node_hash:7,input_json:"{}".into(),dependency_json:"{}".into(),operator_version:"v".into(),outer_node_hash:None,preview_source:None};
 let mut work=FlowInvocationRequestWork::new(false);let demand=work.next_demands(&origin,4096).unwrap();work.advance(&origin,grant(demand)).unwrap();assert!(work.next_demands(&other,4096).is_err());close(&mut work);
}
