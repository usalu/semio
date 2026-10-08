//! 🧪️ Explicit scene framing and neutral wire owners exercise finite first-party ingress.
use super::*;
use serde_json::Value;
use semio_framework_value::ValueRefusalKind;
use pack::intrinsic::{IntrinsicFormat,RetainedIntrinsicInput};
use semio_framework_value::{DslValue,retained_clone::RetainedCloneGrant};

fn corpus()->Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn wire()->Value{serde_json::from_str(include_str!("../🧫️fixtures/📦️body/🔣️.json")).unwrap()}
fn bytes(hex:&str)->Vec<u8>{hex.as_bytes().chunks_exact(2).map(|pair|u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()).collect()}
fn close(cursor:&mut SceneDecodeCursor<'_>){for _ in 0..1000000{if cursor.terminal_is_empty(){return}let copy=cursor.next_close_copy_byte_demand().unwrap().max(3);let release=cursor.next_close_release_byte_demand().unwrap();let capacity=cursor.next_close_capacity_byte_demand(copy.max(release)).unwrap();let depth=cursor.next_close_depth_demand().unwrap();cursor.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth}).unwrap();}panic!("scene ingress did not close")}
fn limits(maximum_owned_bytes:usize)->SceneDecodeLimits{SceneDecodeLimits{maximum_owned_bytes,maximum_depth:128,maximum_items:100000}}
fn embedded<'a>(payload:&'a NodeGraphScenePayload,field:&str)->Option<&'a str>{match field{"previewOffJson"=>payload.preview_off_json.as_deref(),"lodJson"=>payload.lod_json.as_deref(),"controlsJson"=>payload.controls_json.as_deref(),"clustersJson"=>payload.clusters_json.as_deref(),"computingJson"=>payload.computing_json.as_deref(),"statusJson"=>payload.status_json.as_deref(),"capabilitiesJson"=>payload.capabilities_json.as_deref(),"hostSnapshotJson"=>payload.host_snapshot_json.as_deref(),_=>panic!("unauthored embedded field")}}
fn encode(value:&Value,format:IntrinsicFormat)->Vec<u8>{let value=DslValue::from(value);let mut allow=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(1<<24,&mut allow);match format{IntrinsicFormat::Body=>pack::record::intrinsic::encode_body(&value,&Default::default(),&mut control),IntrinsicFormat::Document=>pack::record::intrinsic::encode_document(&value,&Default::default(),&mut control)}.unwrap()}

#[test]
fn retained_scene_original_wire_all_embedded_fields_and_explicit_document(){
 let plan=corpus();let fixed=wire();let mut scene=plan["scene"].clone();for(field,encoded)in fixed["embedded"].as_object().unwrap(){scene[field]=encoded.clone();}
 for format in [IntrinsicFormat::Body,IntrinsicFormat::Document]{let input=if format==IntrinsicFormat::Body{bytes(fixed["combinedHex"].as_str().unwrap())}else{encode(&scene,format)};for units in [1,3,256]{
  let mut cursor=SceneDecodeCursor::new(RetainedIntrinsicInput::Borrowed(&input),format,limits(1<<24)).unwrap();assert_eq!(cursor.advance(0,false).unwrap().units,0);for _ in 0..1000000{let demand=cursor.next_work_demand();if demand>units{let inert=cursor.advance(units,false).unwrap();assert_eq!(inert.units,0);assert_eq!(cursor.next_work_demand(),demand);}let grant=units.max(demand);let step=cursor.advance(grant,false).unwrap();assert!(step.units<=grant);if step.complete{break}}
  let payload=cursor.take_output();close(&mut cursor);let payload=payload.expect("terminal typed scene");assert_eq!(payload.nodes.len(),1);assert_eq!(payload.nodes[0].id,"雪😀");assert_eq!(payload.nodes[0].label.as_deref(),Some("Mesh 雪 😀"));assert_eq!(payload.nodes[0].outputs.as_ref().unwrap()[0].id,"out");assert_eq!(payload.viewport.unwrap(),Viewport2d{x:8.5,y:-0.25,zoom:2.0});
  for row in plan["embedded"].as_array().unwrap(){let json=embedded(&payload,row["field"].as_str().unwrap()).unwrap();let expected=row["nativeJson"].as_str().unwrap();assert_eq!(serde_json::from_str::<Value>(json).unwrap(),serde_json::from_str::<Value>(expected).unwrap());assert_eq!(json,expected);}
 }}
}

#[test]
fn retained_scene_structural_fallback_and_typed_capacity_cancellation_refusals(){
 for row in corpus()["structural"].as_array().unwrap(){let input=encode(&row["scene"],IntrinsicFormat::Body);let mut cursor=SceneDecodeCursor::new(RetainedIntrinsicInput::Borrowed(&input),IntrinsicFormat::Body,limits(1<<24)).unwrap();let mut refusal=None;for _ in 0..1000000{match cursor.advance(1,false){Ok(step)=>if step.complete{break},Err(error)=>{refusal=Some(error.kind);break}}}let payload=cursor.take_output();close(&mut cursor);
  if row["refusal"]=="InvalidValue"{assert_eq!(refusal,Some(ValueRefusalKind::InvalidValue));assert!(payload.is_none());}else{assert!(refusal.is_none());let payload=payload.unwrap();assert_eq!(payload.nodes.len(),row["nodes"].as_u64().unwrap()as usize);assert_eq!(payload.edges.len(),row["edges"].as_u64().unwrap()as usize);assert!(payload.viewport.is_none());}
 }
 let input=bytes(wire()["combinedHex"].as_str().unwrap());let mut cursor=SceneDecodeCursor::new(RetainedIntrinsicInput::Borrowed(&input),IntrinsicFormat::Body,limits(1)).unwrap();let refusal=cursor.advance(1,false).unwrap_err().kind;close(&mut cursor);assert_eq!(refusal,ValueRefusalKind::OwnershipLimit);
 for cutoff in [0,1,3,input.len(),input.len()*3]{let mut cursor=SceneDecodeCursor::new(RetainedIntrinsicInput::Borrowed(&input),IntrinsicFormat::Body,limits(1<<24)).unwrap();cursor.advance(cutoff,false).unwrap();assert_eq!(cursor.advance(1,true).unwrap_err().kind,ValueRefusalKind::Canceled);assert!(cursor.take_output().is_none());close(&mut cursor);}
}

#[test]
fn retained_scene_original_source_and_all_physical_custody_receipts_are_exact(){
 let input=bytes(wire()["combinedHex"].as_str().unwrap());let pointer=input.as_ptr();let backing=input.capacity();
 let(mut cursor,requests,releases)=crate::test_allocation::observe_backing(||SceneDecodeCursor::new(RetainedIntrinsicInput::OwnedBytes(input),IntrinsicFormat::Body,limits(1<<24)).unwrap());assert_eq!((requests,releases),(0,0));assert_eq!(cursor.admitted_bytes(),backing);
 for _ in 0..1000000{let before=cursor.admitted_bytes();let demand=cursor.next_work_demand();let(result,requests,releases)=crate::test_allocation::observe_backing(||cursor.advance(demand,false));let step=match result{Ok(step)=>step,Err(error)=>{close(&mut cursor);panic!("typed scene refused: {error:?}")}};assert_eq!(requests,step.admitted_bytes-before);assert_eq!(releases,0);assert_eq!(cursor.input().unwrap().as_ptr(),pointer);if step.complete{break}}
 assert!(cursor.advance(1,true).is_err());assert!(cursor.take_output().is_none());
 let(_,requests,releases)=crate::test_allocation::observe_backing(||cursor.close_step(RetainedCloneGrant::default()).unwrap());assert_eq!((requests,releases),(0,0));let mut released=0;
 for _ in 0..1000000{if cursor.terminal_is_empty(){break}let(demand,requests,releases)=crate::test_allocation::observe_backing(||cursor.next_close_copy_byte_demand());assert_eq!((requests,releases),(0,0));let copy=demand.unwrap().max(3);let release=cursor.next_close_release_byte_demand().unwrap();let capacity=cursor.next_close_capacity_byte_demand(copy.max(release)).unwrap();let depth=cursor.next_close_depth_demand().unwrap();let(step,requests,releases)=crate::test_allocation::observe_backing(||cursor.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth}).unwrap());let receipt=step.progress();assert_eq!(requests,receipt.retained_capacity_bytes);assert_eq!(releases,receipt.released_bytes);released+=releases;}
 assert!(cursor.terminal_is_empty());assert!(released>=backing);println!("[DEBUG] Surface retained original wire, typed records, eight embedded codec outputs and writer scaffolds close under exact physical receipts");
}

#[test]
fn retained_scene_neutral_phase_cancellation_preserves_each_actual_partial_owner(){
 let input=bytes(wire()["combinedHex"].as_str().unwrap());
 for phase in corpus()["cancelStages"].as_array().unwrap(){let phase=phase.as_str().unwrap();let mut cursor=SceneDecodeCursor::new(RetainedIntrinsicInput::Borrowed(&input),IntrinsicFormat::Body,limits(1<<24)).unwrap();if phase=="retirement"{cursor.advance(1,false).unwrap();cursor.close_step(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()}).unwrap();}else{for _ in 0..1000000{if cursor.phase().as_str()==phase{break}assert!(!cursor.advance(cursor.next_work_demand(),false).unwrap().complete,"phase {phase} was never reached");}}
  assert_eq!(cursor.phase().as_str(),phase);let(result,requests,releases)=crate::test_allocation::observe_backing(||cursor.advance(1,true));assert_eq!(result.unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!((requests,releases),(0,0));assert!(cursor.take_output().is_none());close(&mut cursor);
 }
 println!("[DEBUG] Surface cancellation retains actual input, projection, Base64, Pack, JSON measure/write and retirement partial owners");
}

#[test]
fn retained_scene_neutral_exact_and_one_short_cumulative_materialization_bounds(){
 let input=bytes(wire()["combinedHex"].as_str().unwrap());let mut baseline=SceneDecodeCursor::new(RetainedIntrinsicInput::Borrowed(&input),IntrinsicFormat::Body,limits(1<<24)).unwrap();for _ in 0..1000000{if baseline.advance(baseline.next_work_demand(),false).unwrap().complete{break}}let required=baseline.admitted_bytes();close(&mut baseline);
 for offset in corpus()["ownershipOffsets"].as_array().unwrap(){let offset=offset.as_i64().unwrap();let maximum=required.checked_add_signed(offset as isize).unwrap();let mut cursor=SceneDecodeCursor::new(RetainedIntrinsicInput::Borrowed(&input),IntrinsicFormat::Body,limits(maximum)).unwrap();let mut refusal=None;let mut complete=false;let mut mismatch=None;for _ in 0..1000000{let before=cursor.admitted_bytes();let phase=cursor.phase();let(result,requests,releases)=crate::test_allocation::observe_backing(||cursor.advance(cursor.next_work_demand(),false));let admitted=cursor.admitted_bytes()-before;if requests!=admitted||releases!=0{mismatch=Some((phase,requests,admitted,releases));break}match result{Ok(step)=>if step.complete{complete=true;break},Err(error)=>{refusal=Some(error.kind);break}}}let admitted=cursor.admitted_bytes();let output_absent=cursor.take_output().is_none();close(&mut cursor);assert_eq!(mismatch,None,"ownership offset {offset}");assert_eq!(complete,offset>=0);if offset<0{assert_eq!(refusal,Some(ValueRefusalKind::OwnershipLimit));assert!(output_absent)}else{assert!(refusal.is_none());assert_eq!(admitted,required)}
 }
 println!("[DEBUG] Surface original eight-field corpus respects exact and one-short cumulative materialization admission");
}

#[test]
fn retained_scene_neutral_malformed_embedded_text_and_explicit_framing_never_default(){
 let plan=corpus();for source in plan["malformedBase64"].as_array().unwrap(){let input=encode(&serde_json::json!({"nodes":[],"controlsJson":source}),IntrinsicFormat::Body);let mut cursor=SceneDecodeCursor::new(RetainedIntrinsicInput::Borrowed(&input),IntrinsicFormat::Body,limits(1<<24)).unwrap();let mut refusal=None;for _ in 0..1000000{match cursor.advance(cursor.next_work_demand(),false){Ok(step)=>assert!(!step.complete),Err(error)=>{refusal=Some(error.kind);break}}}assert_eq!(refusal,Some(ValueRefusalKind::InvalidValue));assert!(cursor.take_output().is_none());close(&mut cursor);}
 for(written,selected)in[(IntrinsicFormat::Body,IntrinsicFormat::Document),(IntrinsicFormat::Document,IntrinsicFormat::Body)]{let input=encode(&plan["scene"],written);let mut cursor=SceneDecodeCursor::new(RetainedIntrinsicInput::Borrowed(&input),selected,limits(1<<24)).unwrap();let mut refused=false;for _ in 0..1000000{match cursor.advance(cursor.next_work_demand(),false){Ok(step)=>assert!(!step.complete),Err(_)=>{refused=true;break}}}assert!(refused);assert!(cursor.take_output().is_none());close(&mut cursor);}
}

#[test]
fn retained_scene_neutral_semantic_viewport_refusal_has_no_unadmitted_heap_work(){
 for row in corpus()["structural"].as_array().unwrap().iter().filter(|row|row["refusalMessage"].is_string()){let input=encode(&row["scene"],IntrinsicFormat::Body);let mut cursor=SceneDecodeCursor::new(RetainedIntrinsicInput::Borrowed(&input),IntrinsicFormat::Body,limits(1<<22)).unwrap();let mut refused=None;let mut mismatch=None;
  for _ in 0..1000000{let before=cursor.admitted_bytes();let phase=cursor.phase();let(result,requests,releases)=crate::test_allocation::observe_backing(||cursor.advance(cursor.next_work_demand(),false));let born=cursor.admitted_bytes()-before;if requests!=born||releases!=0{mismatch=Some((phase,requests,born,releases));}match result{Ok(step)=>assert!(!step.complete),Err(error)=>{refused=Some(error);break}}}
  close(&mut cursor);assert_eq!(mismatch,None,"semantic refusal {}",row["id"]);let error=refused.expect("semantic viewport refused");assert_eq!(error.kind,ValueRefusalKind::InvalidValue);assert_eq!(error.message.as_ref(),row["refusalMessage"].as_str().unwrap());
 }
 println!("[DEBUG] Surface semantic zoom refusals retain exact classification/prose without unadmitted error allocation or release");
}
