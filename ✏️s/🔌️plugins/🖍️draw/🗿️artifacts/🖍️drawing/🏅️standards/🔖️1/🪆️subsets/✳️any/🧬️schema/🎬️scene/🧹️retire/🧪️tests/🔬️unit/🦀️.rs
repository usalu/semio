use super::*;
use crate::schema::scene_preparation::{DocumentSceneJob,DocumentSceneLimits};
use serde_json::Value;
use std::sync::Arc;
fn observed<T>(step:impl FnOnce()->T)->(T,(usize,usize)){let(result,allocation)=semio_framework_trace::observe_heap_allocations_on_this_thread(step);assert!(!allocation.overflowed);(result,(allocation.requested_bytes,allocation.released_bytes))}
fn drain(plan:DocumentScenePlan){
 let retained:Vec<_>=plan.assets.iter().map(|asset|asset.image.clone()).collect();
 let expected:Vec<_>=retained.iter().map(|image|image.pixels.clone()).collect();
 let mut owner=ScenePlanCloseJob::new(plan);
 for _ in 0..2_000_000{
  if owner.terminal_is_empty(){break;}
  let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_release_byte_demand().unwrap();
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(if copy==0{release}else{copy}).unwrap(),maximum_release_bytes:release,maximum_depth:owner.next_depth_demand().unwrap()};
  let(zero,physical)=observed(||owner.close_step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());
  assert_eq!(physical,(0,0));assert_eq!(zero.progress(),Default::default());
  let(step,physical)=observed(||owner.close_step(grant).unwrap());let receipt=step.progress();
  assert!(receipt.fits(grant));assert_eq!(physical.0,receipt.retained_capacity_bytes);assert_eq!(physical.1,receipt.released_bytes);
 }
 assert!(owner.terminal_is_empty());let(_,physical)=observed(||drop(owner));assert_eq!(physical,(0,0));
 for(image,expected)in retained.iter().zip(expected.iter()){assert_eq!(&image.pixels,expected);assert_eq!(Arc::strong_count(image),1);}
}
#[test]
fn painted_scenes_retirement_funds_original_allocations_and_releases(){
 let sources:Vec<Value>=serde_json::from_str(include_str!("../../../📋️prepare/🧫️fixtures/🔣️.json")).unwrap();
 let contract:Value=serde_json::from_str(include_str!("../../📏️physical/🧫️fixtures/🔣️.json")).unwrap();
 for row in contract["cases"].as_array().unwrap(){
  if row["source"].is_null(){drain(DocumentScenePlan::default());}
  else if row["source"]=="raster"{drain(DocumentScenePlan{assets:vec![crate::schema::scene_raster::RasterSceneAsset{id:"image".into(),image:Arc::new(semio_framework_pixels::RasterImage{width:2,height:2,pixels:vec![255;16]})}],nodes:Vec::new()});}
  else{for source in &sources{let document:crate::DrawingSnapshot=serde_json::from_value(source["document"].clone()).unwrap();let v=&source["limits"];let mut job=DocumentSceneJob::new(&document,DocumentSceneLimits{max_nodes:v["maxNodes"].as_u64().unwrap()as usize,max_depth:v["maxDepth"].as_u64().unwrap()as usize,max_segments:v["maxSegments"].as_u64().unwrap()as usize,max_references:v["maxReferences"].as_u64().unwrap()as usize,max_source_bytes:v["maxSourceBytes"].as_u64().unwrap()as usize}).unwrap();let mut complete=false;for _ in 0..100000{if job.advance(4096).unwrap().done{complete=true;break;}}assert!(complete,"authored fixture preparation terminates");drain(job.result().unwrap());let(mut closing,output)=job.into_retirement();if let Some(output)=output{drain(output);}crate::draw_scene_physical_close!(closing,1);}}
  eprintln!("[DEBUG] Native scene physical retirement fixture completed: {}",row["name"]);
 }
}
#[test]
fn painted_scenes_retirement_zero_grants_preserve_original_owner(){
 let mut close=ScenePlanCloseJob::new(DocumentScenePlan::default());
 assert_eq!(close.close_step(RetainedCloneGrant::default()).unwrap().progress(),Default::default());assert!(!close.terminal_is_empty());
 crate::draw_scene_physical_close!(close,1);
 assert_eq!(close.close_step(RetainedCloneGrant::default()).unwrap().progress(),Default::default());
}
