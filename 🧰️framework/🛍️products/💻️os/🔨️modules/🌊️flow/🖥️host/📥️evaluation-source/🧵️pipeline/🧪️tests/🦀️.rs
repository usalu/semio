//! 🧪️ Actual pending payload headers cross the shared controller without copying or changing order.
use super::*;
#[test]
fn original_evaluation_prepared_preserves_the_neutral_wave_and_all_native_buffers(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){
  let pending:Vec<_>=row["pending"].as_array().unwrap().iter().map(|value|neural::PendingExtensionEval{neuron_id:value["neuronId"].as_str().unwrap().into(),extension_id:value["extensionId"].as_str().unwrap().into(),operator_id:value["operatorId"].as_str().unwrap().into(),node_hash:value["nodeHash"].as_u64().unwrap(),input_json:value["inputJson"].as_str().unwrap().into()}).collect();
  let pointer=pending.as_ptr();let capacity=pending.capacity();let buffers:Vec<_>=pending.iter().map(|v|[(v.neuron_id.as_ptr(),v.neuron_id.len(),v.neuron_id.capacity()),(v.extension_id.as_ptr(),v.extension_id.len(),v.extension_id.capacity()),(v.operator_id.as_ptr(),v.operator_id.len(),v.operator_id.capacity()),(v.input_json.as_ptr(),v.input_json.len(),v.input_json.capacity())]).collect();
  let output=FlowEvaluationPrepared::receive(pending,!row["remaining"].as_array().unwrap().is_empty());
  assert_eq!((output.pending.as_ptr(),output.pending.capacity()),(pointer,capacity));assert_eq!(output.unfinished,row["unfinished"].as_bool().unwrap());
  for(index,item)in output.pending.iter().enumerate(){let expected=&row["pending"][index];assert_eq!((item.neuron_id.as_str(),item.extension_id.as_str(),item.operator_id.as_str(),item.node_hash,item.input_json.as_str()),(expected["neuronId"].as_str().unwrap(),expected["extensionId"].as_str().unwrap(),expected["operatorId"].as_str().unwrap(),expected["nodeHash"].as_u64().unwrap(),expected["inputJson"].as_str().unwrap()));assert_eq!([(item.neuron_id.as_ptr(),item.neuron_id.len(),item.neuron_id.capacity()),(item.extension_id.as_ptr(),item.extension_id.len(),item.extension_id.capacity()),(item.operator_id.as_ptr(),item.operator_id.len(),item.operator_id.capacity()),(item.input_json.as_ptr(),item.input_json.len(),item.input_json.capacity())],buffers[index]);}
  let mut original=Some(output.pending);let mut close=RetainedCloneClose::default();
  for _ in 0..200{if original.is_none()&&close.is_empty(){break}let d=close.next_owner_demands::<Vec<neural::PendingExtensionEval>>(original.is_some(),&None,4096).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let turn=if close.is_empty(){let turn=close.begin_granted(&mut original,grant);RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}else{close.step_granted(grant)};assert!(turn.work<=1);assert!(turn.result.unwrap().progress().fits(grant));}
  assert!(original.is_none()&&close.is_empty());
 }
}
