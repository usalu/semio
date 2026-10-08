//! 🧪️ Retained receiving input grants preserve the original source across finite upload, decode and close.
use super::{SceneInputCursor,SceneInputGrant};
use semio_framework_pack::intrinsic::IntrinsicFormat;
use semio_framework_surface::node_graph::SceneDecodeLimits;
use semio_framework_value::{ValueRefusalKind,retained_clone::RetainedCloneGrant};
use serde_json::Value;

fn corpus()->Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn wire()->Vec<u8>{let value:Value=serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🗺️surface/🕸️node-graph/📡️scene/🧫️fixtures/📦️body/🔣️.json")).unwrap();value["bodyHex"].as_str().unwrap().as_bytes().chunks_exact(2).map(|pair|u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()).collect()}
fn limits()->SceneDecodeLimits{let value=corpus();SceneDecodeLimits{maximum_owned_bytes:value["maximumOwnedBytes"].as_u64().unwrap()as usize,maximum_depth:value["maximumDepth"].as_u64().unwrap()as usize,maximum_items:value["maximumItems"].as_u64().unwrap()as usize}}
fn input(format:IntrinsicFormat)->Vec<u8>{if format==IntrinsicFormat::Body{return wire()}let value=semio_framework_value::DslValue::from(&serde_json::json!({"nodes":[{"id":corpus()["expectedNode"]}],"edges":[]}));let mut allow=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(1<<22,&mut allow);semio_framework_pack::record::intrinsic::encode_document(&value,&Default::default(),&mut control).unwrap()}
fn close(cursor:&mut SceneInputCursor){for _ in 0..1000000{if cursor.terminal_is_empty(){return}let copy=cursor.next_close_copy_byte_demand().max(3);let release=cursor.next_close_release_byte_demand().unwrap();let capacity=cursor.next_close_capacity_byte_demand(copy.max(release)).unwrap();let depth=cursor.next_close_depth_demand().unwrap();cursor.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth}).unwrap();}panic!("receiving input owner did not close")}

#[test]
fn retained_scene_input_neutral_zero_work_and_one_short_birth_refuse_without_allocation(){
 let input=wire();let configured=limits();let(mut cursor,requests,releases)=crate::test_allocation::observe_backing(||SceneInputCursor::new(input.len(),IntrinsicFormat::Body,configured).unwrap());assert_eq!((requests,releases),(0,0));
 let(step,requests,releases)=crate::test_allocation::observe_backing(||cursor.append(&input,SceneInputGrant{maximum_units:0,maximum_capacity_bytes:input.len(),cancelled:false}).unwrap());assert_eq!((step.units,step.copied_bytes,step.admitted_bytes,requests,releases),(0,0,0,0,0));assert!(cursor.input().is_empty());
 close(&mut cursor);for offset in corpus()["capacityOffsets"].as_array().unwrap(){let offset=offset.as_i64().unwrap()as isize;let mut cursor=SceneInputCursor::new(input.len(),IntrinsicFormat::Body,configured).unwrap();let capacity=input.len().checked_add_signed(offset).unwrap();let(result,requests,releases)=crate::test_allocation::observe_backing(||cursor.append(&input,SceneInputGrant{maximum_units:1,maximum_capacity_bytes:capacity,cancelled:false}));let admitted=cursor.admitted_bytes();let initialized=cursor.input().len();close(&mut cursor);assert_eq!(initialized,0);assert_eq!(releases,0);if offset<0{assert_eq!(result.unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!((admitted,requests),(0,0))}else{let step=result.unwrap();assert_eq!((step.units,step.copied_bytes),(1,0));assert_eq!((step.admitted_bytes,admitted,requests),(input.len(),input.len(),input.len()))}}
 let mut small=limits();small.maximum_owned_bytes=input.len()-1;let(result,requests,releases)=crate::test_allocation::observe_backing(||SceneInputCursor::new(input.len(),IntrinsicFormat::Body,small));assert_eq!(result.err().unwrap().kind,ValueRefusalKind::OwnershipLimit);assert_eq!((requests,releases),(0,0));
 println!("[DEBUG] OS scene receiving input constructor and denied capacity grants perform no heap work");
}

#[test]
fn retained_scene_input_neutral_partial_cancellation_preserves_exact_source_custody(){
 let input=wire();for cutoff in corpus()["cancelledOffsets"].as_array().unwrap(){let cutoff=(cutoff.as_u64().unwrap()as usize).min(input.len());let mut cursor=SceneInputCursor::new(input.len(),IntrinsicFormat::Body,limits()).unwrap();
  let grant=if cutoff==0{0}else{cutoff+1};let(step,requests,releases)=crate::test_allocation::observe_backing(||cursor.append(&input,SceneInputGrant{maximum_units:grant,maximum_capacity_bytes:input.len(),cancelled:false}).unwrap());assert_eq!(step.units,grant);assert_eq!(step.copied_bytes,cutoff);assert_eq!(cursor.input(),&input[..cutoff]);assert_eq!(requests,step.admitted_bytes);assert_eq!(releases,0);let pointer=cursor.input().as_ptr();let before=cursor.admitted_bytes();
  let(result,requests,releases)=crate::test_allocation::observe_backing(||cursor.append(&input[cutoff..],SceneInputGrant{maximum_units:256,maximum_capacity_bytes:0,cancelled:true}));assert_eq!(result.unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!((requests,releases),(0,0));assert_eq!(cursor.input().as_ptr(),pointer);assert_eq!(cursor.input(),&input[..cutoff]);assert_eq!(cursor.admitted_bytes(),before);assert!(cursor.take_output().is_none());
  let(_,requests,releases)=crate::test_allocation::observe_backing(||cursor.close_step(Default::default()).unwrap());assert_eq!((requests,releases),(0,0));close(&mut cursor);
 }
 println!("[DEBUG] OS cancelled partial uploads retain initialized bytes and exact original backing until explicit close");
}

#[test]
fn retained_scene_input_neutral_finite_upload_moves_original_backing_into_general_decoder(){
 for framing in corpus()["formats"].as_array().unwrap(){let format=match framing.as_str().unwrap(){"Body"=>IntrinsicFormat::Body,"Document"=>IntrinsicFormat::Document,_=>panic!("unauthored framing")};let input=input(format);for units in corpus()["units"].as_array().unwrap().iter().filter_map(Value::as_u64).filter(|value|*value>0){let units=units as usize;let mut cursor=SceneInputCursor::new(input.len(),format,limits()).unwrap();let mut offset=0;let mut pointer=None;
  while offset<input.len(){let before=cursor.admitted_bytes();let(step,requests,releases)=crate::test_allocation::observe_backing(||cursor.append(&input[offset..],SceneInputGrant{maximum_units:units,maximum_capacity_bytes:input.len(),cancelled:false}).unwrap());assert!(step.units<=units);assert!(step.units>0);assert!(step.copied_bytes<=step.units);offset+=step.copied_bytes;assert_eq!(cursor.input(),&input[..offset]);assert_eq!(requests,step.admitted_bytes-before);assert_eq!(releases,0);if let Some(pointer)=pointer{assert_eq!(cursor.input().as_ptr(),pointer)}else{pointer=Some(cursor.input().as_ptr())}}
  for _ in 0..1000000{let before=cursor.admitted_bytes();let demand=cursor.next_work_demand();let(step,requests,releases)=crate::test_allocation::observe_backing(||cursor.advance(demand,false).unwrap());assert!(step.units<=demand);assert_eq!(requests,step.admitted_bytes-before);assert_eq!(releases,0);assert_eq!(Some(cursor.input().as_ptr()),pointer);if step.complete{break}}
  assert_eq!(cursor.next_work_demand(),0);let(step,requests,releases)=crate::test_allocation::observe_backing(||cursor.advance(0,false).unwrap());assert_eq!((step.units,step.complete,requests,releases),(0,true,0,0));
  let output=cursor.take_output().unwrap();close(&mut cursor);assert_eq!(output.nodes.len(),1);assert_eq!(output.nodes[0].id,corpus()["expectedNode"].as_str().unwrap());let expected:Value=serde_json::from_str("{\"nodes\":[{\"id\":\"雪😀\"}]}").unwrap();assert_eq!(output.nodes[0].id,expected["nodes"][0]["id"].as_str().unwrap());
 }}
 println!("[DEBUG] OS finite uploads preserve the actual backing pointer through General retained scene materialization");
}

#[test]
fn retained_scene_input_neutral_terminal_close_keeps_copy_capacity_and_physical_release_independent(){
 let input=wire();for decode in [false,true]{let mut cursor=SceneInputCursor::new(input.len(),IntrinsicFormat::Body,limits()).unwrap();let units=if decode{input.len()+1}else{4};cursor.append(&input,SceneInputGrant{maximum_units:units,maximum_capacity_bytes:input.len(),cancelled:false}).unwrap();if decode{for _ in 0..1000000{if cursor.advance(cursor.next_work_demand(),false).unwrap().complete{break}}}assert_eq!(cursor.advance(1,true).unwrap_err().kind,ValueRefusalKind::Canceled);let mut born=cursor.admitted_bytes();let mut released=0;
  for _ in 0..1000000{if cursor.terminal_is_empty(){break}let copy=cursor.next_close_copy_byte_demand().max(3);let release=cursor.next_close_release_byte_demand().unwrap();let capacity=cursor.next_close_capacity_byte_demand(copy.max(release)).unwrap();let depth=cursor.next_close_depth_demand().unwrap();let(step,requests,releases)=crate::test_allocation::observe_backing(||cursor.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth}).unwrap());let receipt=step.progress();assert_eq!(requests,receipt.retained_capacity_bytes);assert_eq!(releases,receipt.released_bytes);assert!(receipt.copied_bytes<=copy);assert!(requests<=capacity);assert!(releases<=release);born+=requests;released+=releases;}
  assert!(cursor.terminal_is_empty());assert_eq!(released,born);
 }
 println!("[DEBUG] OS partial source and completed retained decoder release every admitted backing under independent exact receipts");
}
