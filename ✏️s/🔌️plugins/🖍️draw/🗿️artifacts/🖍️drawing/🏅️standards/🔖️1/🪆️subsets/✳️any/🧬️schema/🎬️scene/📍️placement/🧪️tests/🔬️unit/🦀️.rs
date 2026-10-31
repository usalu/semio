#[allow(unused_imports)] use semio_s_artifact_draw_drawing_scene::{booleans::*,identity::*,paint::*,picking::*,placement::*,preparation::*,raster::*,retirement::*,text::*,trace::*,view::*};
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_model::fill_rule::FillRule;
use semio_framework_test_fixtures::fixture_text;

#[allow(unused_imports)] use semio_s_artifact_draw_drawing_scene::preparation::{DrawingSceneSource, DocumentVectorPreparationJob, DocumentSceneLimits, DocumentAlgorithmLimits, DocumentSceneContent};
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_scene::placement::*;
use semio_s_artifact_draw_drawing_model::{DrawingLayerNode, DrawingImageAsset};
use semio_s_artifact_draw_drawing_model::layer_base;


use semio_framework_value::paged::Utf8Text;
use serde_json::Value;
struct Source{roots:Vec<DrawingLayerNode>,assets:std::collections::BTreeMap<String,DrawingImageAsset>,selected:Vec<String>}
impl DrawingSceneSource for Source{
 fn root_count(&self)->usize{self.roots.len()}
 fn root(&self,index:usize)->Option<&DrawingLayerNode>{self.roots.get(index)}
 fn asset_count(&self)->usize{self.assets.len()}
 fn asset_at(&self,index:usize)->Option<(&dyn Utf8Text,&DrawingImageAsset)>{self.assets.iter().nth(index).map(|(key,asset)|(key as &dyn Utf8Text,asset))}
 fn asset(&self,key:&str)->Option<&DrawingImageAsset>{self.assets.get(key)}
 fn root_visible(&self,index:usize)->bool{self.roots.get(index).is_some_and(|root|self.selected.iter().any(|id|layer_base(root).id.eq_str(id)))}
}
fn rows()->Vec<Value>{serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎬️scene/📍️placement/🔣️.json")).unwrap()}
fn source(row:&Value)->Source{let sources:Vec<Value>=serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎬️scene/📋️prepare/🎬️vector/🔣️.json")).unwrap();let document=row["source"].as_str().map(|name|sources.iter().find(|source|source["name"]==name).unwrap()["document"].clone());Source{roots:serde_json::from_value(document.as_ref().map_or_else(||row["roots"].clone(),|document|document["layers"].clone())).unwrap(),assets:serde_json::from_value(document.as_ref().map_or_else(||row["assets"].clone(),|document|document["assets"].clone())).unwrap(),selected:serde_json::from_value(row["selected"].clone()).unwrap()}}
fn limits()->DocumentSceneLimits{DocumentSceneLimits{max_nodes:1024,max_depth:32,max_segments:65536,max_references:32768,max_source_bytes:268439552}}
fn algorithms()->DocumentAlgorithmLimits{DocumentAlgorithmLimits{max_work:1_000_000_000,booleans:DocumentBooleanLimits{tolerance:0.001,epsilon:1e-8,max_depth:32,max_references:32768,max_edges:65536,max_parameters:262144,max_atomic_edges:65536,max_segments:65536,max_retained_segments:262144,max_work:1_000_000_000},trace:DocumentTraceLimits{max_pixels:16777216,max_admitted_pixels:67108864,max_source_bytes:268439552,max_edges:65536,max_segments:65536,max_retained_segments:65536,max_work:1_000_000_000}}}
#[path="../../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🛤️path/🎟️work/🧪️tests/🦀️.rs"]mod funding;
fn original_fixture()->funding::Wallet{let law:Value=serde_json::from_str(include_str!("../../🎟️work/📐️.json")).unwrap();let v=&law["stock"];funding::Wallet::from_grant(semio_framework_value::RetainedCloneGrant{maximum_items:v["items"].as_u64().unwrap()as usize,maximum_copy_bytes:v["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:v["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:v["release"].as_u64().unwrap()as usize,maximum_depth:v["depth"].as_u64().unwrap()as usize},law["maxTurns"].as_u64().unwrap()as usize)}
#[test]
fn shared_placement_computes_real_area_length_bounds_and_empty_centroids(){for row in rows(){let mut previous=None;for grant in [1,7,4096]{let source=source(&row);let before=serde_json::to_value(&source.roots).unwrap();let mut job=DrawingScenePlacementJob::new(&source,limits(),algorithms(),0.001).unwrap();let mut funding=original_fixture();assert!(job.result().is_err());let(mut work,mut done)=(0,false);for _ in 0..2_000_000{let progress=funding.turn(grant,|work|job.advance(&source,grant,work)).unwrap();assert!(progress.work>=work&&progress.work-work<=grant as u64);assert_eq!(progress.work-work,job.last_turn_semantic_work());assert!(job.last_turn_work()>0||progress.done);work=progress.work;if progress.done{done=true;break;}}assert!(done,"{}",row["name"]);let output=job.result().unwrap();let expected:DrawingScenePlacement=serde_json::from_value(row["expected"].clone()).unwrap();assert_eq!(output.basis,expected.basis);assert_eq!(output.bounds,expected.bounds);assert!((output.area-expected.area).abs()<1e-8);for axis in 0..2{assert!((output.centroid[axis]-expected.centroid[axis]).abs()<1e-8);}if let Some(previous)=previous{assert_eq!(output,previous);}previous=Some(output);assert_eq!(serde_json::to_value(&source.roots).unwrap(),before);let(close,moved)=job.into_retirement();assert_eq!(moved,Some(output));funding.close_owned((close,moved));}}}
#[test]
fn placement_cancel_preserves_actual_packet_and_closes_private_children(){for row in rows(){for turns in [0,1,10,100]{let source=source(&row);let before=serde_json::to_value(&source.roots).unwrap();let mut job=DrawingScenePlacementJob::new(&source,limits(),algorithms(),0.001).unwrap();let mut funding=original_fixture();for _ in 0..turns{if funding.turn(1,|work|job.advance(&source,1,work)).unwrap().done{break;}}job.cancel();assert!(job.result().is_err());assert_eq!(serde_json::to_value(&source.roots).unwrap(),before);let(close,output)=job.into_retirement();assert!(output.is_none());funding.close_owned((close,output));}}}

#[test]
fn placement_real_frontier_and_heap_follow_one_finite_original_ledger(){
 use semio_framework_value::{RetainedCloneGrant,work::RetainedWorkLedger};
 let row=&rows()[0];let source=source(row);let before=serde_json::to_value(&source.roots).unwrap();let data:Value=serde_json::from_str(include_str!("../../🎟️work/📐️.json")).unwrap();let v=&data["stock"];
 let original=RetainedCloneGrant{maximum_items:v["items"].as_u64().unwrap()as usize,maximum_copy_bytes:v["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:v["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:v["release"].as_u64().unwrap()as usize,maximum_depth:v["depth"].as_u64().unwrap()as usize};
 let mut bank=funding::Wallet::from_grant(original,data["maxTurns"].as_u64().unwrap()as usize);let mut job=DrawingScenePlacementJob::new(&source,limits(),algorithms(),0.001).unwrap();let mut denied=[false;4];let mut complete=false;let mut saw_control=false;let mut semantic=0;
 for turn in 0..data["maxTurns"].as_u64().unwrap(){let d=job.next_work_demand();for axis in 0..4{let mut under=bank.remaining();let field=match axis{0=>&mut under.maximum_items,1=>&mut under.maximum_copy_bytes,2=>&mut under.maximum_capacity_bytes,_=>&mut under.maximum_depth};let requested=match axis{0=>1,1=>d.copy_bytes,2=>d.capacity_bytes,_=>d.depth};if denied[axis]||requested==0{continue;}*field=requested-1;let((result,receipt),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let mut no=RetainedWorkLedger::new(under);let result=job.advance(&source,1,&mut no);(result,no.receipt())});assert!(!heap.overflowed);assert!(!result.unwrap().done);assert_eq!(receipt,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(job.last_turn_work(),0);denied[axis]=true;}
  let before=bank.remaining();let progress=bank.turn(1,|work|job.advance(&source,1,work)).unwrap();assert_eq!(progress.work-semantic,job.last_turn_semantic_work());semantic=progress.work;if job.last_turn_work()>0&&job.last_turn_semantic_work()==0{saw_control=true;}if progress.done{complete=true;break;}assert!(bank.remaining()!=before||job.last_turn_work()>0,"actual finite stock exhausted at turn{turn}");assert!(job.last_turn_work()<=1);
 }assert!(complete);assert!(saw_control);assert!(denied.into_iter().all(|axis|axis));let actual=job.result().unwrap();let expected:DrawingScenePlacement=serde_json::from_value(row["expected"].clone()).unwrap();assert_eq!(actual.basis,expected.basis);assert!((actual.area-expected.area).abs()<1e-8);for axis in 0..2{assert!((actual.centroid[axis]-expected.centroid[axis]).abs()<1e-8);}let(close,output)=job.into_retirement();assert_eq!(output,Some(actual));bank.close_owned((close,output));assert_eq!(serde_json::to_value(&source.roots).unwrap(),before);
}
