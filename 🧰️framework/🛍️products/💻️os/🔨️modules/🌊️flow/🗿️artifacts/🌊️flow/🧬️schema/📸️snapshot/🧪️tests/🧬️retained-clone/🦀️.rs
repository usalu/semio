//! 🧪️ Native Flow cloning equals the original model and funds every cursor allocation and release.
use crate::{FlowHostSnapshot,widget_id_for};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::retained_clone::{RetainedCloneSource,RetainedCloneOwner};
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn observed<T>(f:impl FnOnce()->T)->(T,usize,usize){let(v,h)=semio_framework_trace::observe_heap_allocations_on_this_thread(f);assert!(!h.overflowed);(v,h.requested_bytes,h.released_bytes)}
fn receipt(p:RetainedCloneProgress,g:RetainedCloneGrant,born:usize,released:usize){assert!(p.fits(g));assert_eq!((born,released),(p.retained_capacity_bytes,p.released_bytes))}
#[test]
fn native_flow_snapshot_clone_equals_original_model_with_exact_receipts(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("📁️cases.json")).unwrap();
 for row in cases.as_array().unwrap(){
  let model:FlowHostSnapshot=semio_framework_pack_json::from_json_str(&row["snapshot"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let expected=model.clone();
  let mut admission=RetainedCloneSource::<FlowHostSnapshot>::admission_owned(model,());let mut source=None;
  for _ in 0..100{if admission.terminal_is_empty(){break}let g=grant(if admission.phase()=="ready"{RetirementDemand{depth:1,..Default::default()}}else{admission.next_demand().unwrap()});let(turn,born,released)=observed(||if admission.phase()=="ready"{admission.take_output(&mut source,g)}else{admission.advance(g)});receipt(turn.result.unwrap().progress(),g,born,released)}
  assert!(admission.terminal_is_empty());let mut source=source.unwrap();let mut cursor=RetainedCloneOwner::<FlowHostSnapshot>::default();let mut done=false;
  for _ in 0..100000{let d=cursor.advance_demands(source.borrow(),4).unwrap();let g=grant(d);let mut denied=g;denied.maximum_items=0;let(turn,born,released)=observed(||cursor.advance(source.borrow(),4,denied));assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!((born,released),(0,0));let(turn,born,released)=observed(||cursor.advance(source.borrow(),4,g));let step=turn.result.unwrap();receipt(step.progress(),g,born,released);if matches!(step,RetainedCloneStep::Complete(_)){done=true;break}}
  assert!(done);let g=grant(cursor.take_demand().unwrap());let(turn,born,released)=observed(||cursor.take(g));let(output,p)=turn.result.unwrap().unwrap();receipt(p,g,born,released);
  assert_eq!(output,expected);let ids:Vec<_>=output.widgets.iter().map(widget_id_for).collect();let expected_ids:Vec<_>=row["expectedIds"].as_array().unwrap().iter().map(|id|id.as_str().unwrap()).collect();assert_eq!(ids,expected_ids);
  let ids:Vec<_>=output.synapses.iter().map(|edge|edge.id.as_str()).collect();let expected_ids:Vec<_>=row["expectedSynapses"].as_array().unwrap().iter().map(|id|id.as_str().unwrap()).collect();assert_eq!(ids,expected_ids);
  for _ in 0..100000{if cursor.terminal_is_empty(){break}let g=grant(cursor.selected_close_demand(4).unwrap());let(turn,born,released)=observed(||cursor.close_step(g));receipt(turn.result.unwrap().progress(),g,born,released)}assert!(cursor.terminal_is_empty());
  for _ in 0..100000{if source.terminal_is_empty(){break}let g=grant(source.next_close_demand(4).unwrap());let(turn,born,released)=observed(||source.close_step(g));receipt(turn.result.unwrap().progress(),g,born,released)}assert!(source.terminal_is_empty());
  output.retire_cold();expected.retire_cold();
 }
}

#[test]
fn native_flow_snapshot_clone_cancels_nested_fields_with_exact_receipts(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("📁️cases.json")).unwrap();
 for row in cases.as_array().unwrap(){for stop in [1,40,140,360]{
  let model:FlowHostSnapshot=semio_framework_pack_json::from_json_str(&row["snapshot"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let mut admission=RetainedCloneSource::<FlowHostSnapshot>::admission_owned(model,());let mut source=None;
  for _ in 0..100{if admission.terminal_is_empty(){break}let g=grant(if admission.phase()=="ready"{RetirementDemand{depth:1,..Default::default()}}else{admission.next_demand().unwrap()});let(turn,born,released)=observed(||if admission.phase()=="ready"{admission.take_output(&mut source,g)}else{admission.advance(g)});receipt(turn.result.unwrap().progress(),g,born,released)}
  assert!(admission.terminal_is_empty());let mut source=source.unwrap();let mut cursor=RetainedCloneOwner::<FlowHostSnapshot>::default();
  for _ in 0..stop{let g=grant(cursor.advance_demands(source.borrow(),4).unwrap());let(turn,born,released)=observed(||cursor.advance(source.borrow(),4,g));let step=turn.result.unwrap();receipt(step.progress(),g,born,released);if matches!(step,RetainedCloneStep::Complete(_)){break}}
  for _ in 0..100000{if cursor.terminal_is_empty(){break}let d=cursor.selected_close_demand(4).unwrap();let g=grant(d);let mut denied=g;denied.maximum_items=0;let(turn,born,released)=observed(||cursor.close_step(denied));assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!((born,released),(0,0));let(turn,born,released)=observed(||cursor.close_step(g));receipt(turn.result.unwrap().progress(),g,born,released)}assert!(cursor.terminal_is_empty());
  for _ in 0..100000{if source.terminal_is_empty(){break}let g=grant(source.next_close_demand(4).unwrap());let(turn,born,released)=observed(||source.close_step(g));receipt(turn.result.unwrap().progress(),g,born,released)}assert!(source.terminal_is_empty());
 }}
}

fn canonical_tree(value:&FlowHostSnapshot)->&dyn semio_framework_pack_json::ArtifactCanonicalJsonTree{value}
#[test]
fn native_flow_snapshot_canonical_bytes_equal_original_pack_wire(){
 use semio_framework_pack_json::{ArtifactCanonicalJsonTree,ArtifactCanonicalJsonTreeCursor};
 use semio_framework_value::retained_clone::RetainedOwnedProjection;
 let cases:serde_json::Value=serde_json::from_str(include_str!("📁️cases.json")).unwrap();
 for row in cases.as_array().unwrap(){for scalar in row["floatWords"].as_array().unwrap(){
  let value=f64::from_bits(u64::from_str_radix(scalar["bits"].as_str().unwrap(),16).unwrap());let expected_scalar=scalar["json"].as_str().unwrap();assert_eq!(semio_framework_pack_json::to_json_string(&value),expected_scalar);assert_eq!(serde_json::to_string(&value).unwrap(),expected_scalar);
  let mut model:FlowHostSnapshot=semio_framework_pack_json::from_json_str(&row["snapshot"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();if let crate::Widget::InputSlider{value:slot,..}=&mut model.widgets[0]{*slot=value}else{panic!("fixture starts with original native slider")}
  let expected=semio_framework_pack_json::to_json_string(&model);
  let mut admission=RetainedCloneSource::<FlowHostSnapshot>::admission_owned(model,());let mut source=None;
  for _ in 0..100{if admission.terminal_is_empty(){break}let g=grant(if admission.phase()=="ready"{RetirementDemand{depth:1,..Default::default()}}else{admission.next_demand().unwrap()});let(turn,born,released)=observed(||if admission.phase()=="ready"{admission.take_output(&mut source,g)}else{admission.advance(g)});receipt(turn.result.unwrap().progress(),g,born,released)}
  assert!(admission.terminal_is_empty());let mut source=source.unwrap();
  let g=grant(RetainedOwnedProjection::<dyn ArtifactCanonicalJsonTree>::constructor_demand());let(turn,born,released)=observed(||source.project_owned(3,canonical_tree,g));let(projection,p)=turn.result.unwrap();receipt(p,g,born,released);
  let g=grant(ArtifactCanonicalJsonTreeCursor::constructor_demand());let(result,born,released)=observed(||ArtifactCanonicalJsonTreeCursor::admit(projection,g));let(mut cursor,p)=result.unwrap_or_else(|_|panic!("original native canonical source admission"));receipt(p,g,born,released);
  let mut actual=Vec::new();
  for _ in 0..100000{if cursor.is_complete(){break}let d=cursor.next_demand().unwrap();let g=grant(d);let mut output=[0u8;64];let(result,born,released)=observed(||cursor.advance(&mut output,g));let step=result.unwrap();receipt(step.ownership.progress(),g,born,released);actual.extend_from_slice(&output[..step.written_bytes])}
  assert!(cursor.is_complete());assert_eq!(actual,expected.as_bytes(),"whole native wire scalar {}",scalar["bits"]);
  for _ in 0..100000{if source.terminal_is_empty(){break}let g=grant(source.next_close_demand(4).unwrap());let(turn,born,released)=observed(||source.close_step(g));receipt(turn.result.unwrap().progress(),g,born,released)}assert!(source.terminal_is_empty());
 }}
}
