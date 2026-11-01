//! 🧪️ Defining scene owners, original finite stock and exact child publication identity.
use super::*;
#[path="../../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🛤️path/🎟️work/🧪️tests/🦀️.rs"]mod funding;
fn raster_wallet()->funding::Wallet{let data:serde_json::Value=serde_json::from_str(include_str!("../../🎟️work/📐️.json")).unwrap();let s=&data["stock"];funding::Wallet::from_grant(semio_framework_value::RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:s["release"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize},data["maxTurns"].as_u64().unwrap()as usize)}
fn scene(input:RasterSceneInput)->funding::Funded<RasterSceneJob>{funding::Funded::from_parts(RasterSceneJob::new(input),raster_wallet())}
fn painted(input:PathRasterInput)->funding::Funded<PathRasterJob>{funding::Funded::from_parts(PathRasterJob::new(input),raster_wallet())}
fn through(job:&mut funding::Funded<RasterSceneJob>,quantum:usize)->Result<RasterSceneProgress,RasterSceneError>{for _ in 0..2000000{let p=job.turn(quantum,|owner,work|owner.advance(quantum,work))?;if p.done{return Ok(p);}assert!(job.last_turn_work()>0,"finite stock blocked at {:?}",job.next_work_demand());}panic!("finite scene observation bound exhausted");}
use crate::{booleans::*,identity::*,preparation::*,raster::*,retirement::*,text::*,trace::*,view::*};
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_model::fill_rule::FillRule;
use semio_framework_test_fixtures::fixture_text;

#[allow(unused_imports)] use semio_s_artifact_draw_drawing_model::{PathSegment, FillStyle, StrokeStyle};
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_model::DrawingSceneGroup;
#[allow(unused_imports)] use semio_framework_pixels::RasterImage;
#[allow(unused_imports)] use std::sync::Arc;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_geometry::raster::{PathRasterJob, PathRasterInput, PathRasterRetirement};
#[allow(unused_imports)] use std::collections::VecDeque;
#[allow(unused_imports)] use semio_framework_value::retirement::{RetireOwned,RetirementCursor};
use std::collections::BTreeSet;

fn close_scene(job:funding::Funded<RasterSceneJob>){job.finish();}
fn finish_scene(job:funding::Funded<RasterSceneJob>)->RasterImage{let image=job.result().unwrap().clone();job.finish();image}
fn child_handoff(kind:&str,sources:serde_json::Value,cases:serde_json::Value){for row in cases.as_array().unwrap(){for grant in [1,7,4096]{let source=sources.as_array().unwrap().iter().find(|v|v["name"]==row["source"]).unwrap();let mut job=scene(input(&source["input"]));let mut parents=0;let mut moved=None;let mut image_count=0;let mut entered=false;let mut done=false;for _ in 0..2000000{let active=job.phase==kind;if active&&!entered{entered=true;parents+=1;let image=if kind=="imageCleanup"{assert!(job.sampler.is_none());assert!(!job.sampler_retirement.as_ref().unwrap().terminal_is_empty());job.sampled.as_ref().unwrap()}else{assert!(job.painter.is_none());assert!(!job.painter_retirement.as_ref().unwrap().terminal_is_empty());job.painted.as_ref().unwrap()};moved=Some(image.pixels.as_ptr());image_count=job.images.0.len();}if !active{entered=false;}let before=job.work;let budget=if matches!(job.phase,"path"|"image"|"pathCleanup"|"imageCleanup"|"publishing"){1}else{grant};let p=job.turn(budget,|owner,work|owner.advance(budget,work)).unwrap();assert!(p.work-before<=budget as u64);if job.images.0.len()>image_count{if let Some(pointer)=moved.take(){assert_eq!(job.images.0.last().unwrap().1.0.pixels.as_ptr(),pointer);if kind=="imageCleanup"{assert!(!job.retired_samplers.last().unwrap().terminal_is_empty());}else{assert!(!job.retired_painters.last().unwrap().terminal_is_empty());}}}if p.done{done=true;break;}}assert!(done);assert_eq!(parents,row["parents"].as_u64().unwrap());assert!(job.retired_painters.iter().all(|child|!child.terminal_is_empty()));assert!(job.retired_samplers.iter().all(|child|!child.terminal_is_empty()));assert_eq!(serde_json::to_value(&finish_scene(job).pixels).unwrap(),source["expected"]);}}}
#[test]
fn scene_sampled_pixels_retain_actual_affine_owner_until_caller_close(){child_handoff("imageCleanup",serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎬️scene/📷️raster/🖼️images/🔣️.json")).unwrap(),serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎬️scene/📷️raster/🧹️image/🔣️.json")).unwrap());}
#[test]
fn scene_path_pixels_retain_actual_whole_parent_until_caller_close(){child_handoff("pathCleanup",serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎬️scene/📷️raster/🔣️.json")).unwrap(),serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎬️scene/📷️raster/🧹️path/🔣️.json")).unwrap());}
#[test]
fn painted_scenes_filter_every_shared_image_under_bounded_grants(){
 let cases:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎬️scene/📷️raster/🖼️images/🔣️.json")).unwrap();
 for row in cases.as_array().unwrap(){for grant in [1,7,4096]{let mut job=scene(input(&row["input"]));let mut work=0;let mut done=false;for _ in 0..2000000{let p=job.turn(grant,|owner,work|owner.advance(grant,work)).unwrap();assert!(p.work-work<=grant as u64);work=p.work;if p.done{done=true;break;}}assert!(done);let expected:Vec<u8>=serde_json::from_value(row["expected"].clone()).unwrap();assert_eq!(job.result().unwrap().pixels,expected,"{} grant {grant}",row["name"]);close_scene(job);}}
}
#[test]
fn painted_scenes_filtering_admits_source_and_crop_and_cancels_reduction(){
 let cases:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎬️scene/📷️raster/🖼️images/🔣️.json")).unwrap();
 let mut grid=input(&cases[0]["input"]);grid.max_pixels=1;let mut job=scene(grid.clone());assert_eq!(through(&mut job,100000).unwrap().pixels,1);assert!(job.result().is_ok());close_scene(job);
 let mut shifted=grid.clone();shifted.width=2;shifted.nodes[0].transform[4]=0.5;shifted.max_pixels=2;let mut job=scene(shifted.clone());assert!(through(&mut job,100000).is_err());assert!(job.result().is_err());close_scene(job);shifted.max_pixels=3;let mut job=scene(shifted);assert_eq!(through(&mut job,100000).unwrap().pixels,3);assert!(job.result().is_ok());close_scene(job);
 grid.nodes[0].transform[4]=1e8;let mut job=scene(grid.clone());assert_eq!(through(&mut job,100000).unwrap().pixels,0);close_scene(job);
 grid.nodes[0].transform=[1.0/64.0,0.0,0.0,1.0/64.0,0.0,0.0];grid.nodes[0].content=RasterSceneContent::Pixels(Arc::new(RasterImage{width:64,height:64,pixels:vec![255;64*64*4]}));grid.max_pixels=4097;
 let mut job=scene(grid);let mut seen=false;for _ in 0..100{let p=job.turn(1,|owner,work|owner.advance(1,work)).unwrap();if p.phase=="image"{seen=true;break;}}assert!(seen);job.cancel();assert!(job.turn(1,|owner,work|owner.advance(1,work)).is_err());assert!(job.result().is_err());close_scene(job);
}
fn input(v:&serde_json::Value)->RasterSceneInput {
 RasterSceneInput {width:v["width"].as_u64().unwrap() as u32,height:v["height"].as_u64().unwrap() as u32,origin:serde_json::from_value(v["origin"].clone()).unwrap(),tolerance:v["tolerance"].as_f64().unwrap(),max_pixels:v["maxPixels"].as_u64().unwrap() as usize,max_source_bytes:v["maxSourceBytes"].as_u64().unwrap()as usize,assets:v["assets"].as_array().unwrap().iter().map(|a|RasterSceneAsset{id:a["id"].as_str().unwrap().into(),image:Arc::new(RasterImage{width:a["image"]["width"].as_u64().unwrap()as u32,height:a["image"]["height"].as_u64().unwrap()as u32,pixels:serde_json::from_value(a["image"]["pixels"].clone()).unwrap()})}).collect(),nodes:v["nodes"].as_array().unwrap().iter().map(|n|{
 let c=&n["content"];let content=if c["kind"]=="path" {RasterSceneContent::Path {segments:serde_json::from_value(c["segments"].clone()).unwrap(),fill_rule:serde_json::from_value(c["fillRule"].clone()).unwrap(),fill:serde_json::from_value(c["fill"].clone()).unwrap(),stroke:serde_json::from_value(c["stroke"].clone()).unwrap()}}else if c["kind"]=="image"{RasterSceneContent::Image{asset:c["asset"].as_str().unwrap().into(),width:c["width"].as_f64().unwrap(),height:c["height"].as_f64().unwrap()}}else{let i=&c["image"];RasterSceneContent::Pixels(Arc::new(RasterImage {width:i["width"].as_u64().unwrap() as u32,height:i["height"].as_u64().unwrap() as u32,pixels:serde_json::from_value(i["pixels"].clone()).unwrap()}))};
 RasterSceneNode {id:n["id"].as_str().unwrap().into(),groups:n["groups"].as_array().unwrap().iter().map(|g|DrawingSceneGroup{id:g["id"].as_str().unwrap().into(),opacity:g["opacity"].as_f64().unwrap(),blend_mode:g["blendMode"].as_str().unwrap().into()}).collect(),transform:serde_json::from_value(n["transform"].clone()).unwrap(),opacity:n["opacity"].as_f64().unwrap(),blend_mode:n["blendMode"].as_str().unwrap().into(),visible:n["visible"].as_bool().unwrap(),content}
 }).collect()}
}
#[test]
fn painted_scenes_match_shared_rgba_under_bounded_grants() {
 let cases:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎬️scene/📷️raster/🔣️.json")).unwrap();
 for row in cases.as_array().unwrap(){for grant in [1,7,4096]{let mut job=scene(input(&row["input"]));let mut work=0;let mut done=false;for _ in 0..2000000{let p=job.turn(grant,|owner,work|owner.advance(grant,work)).unwrap();assert!(p.work-work<=grant as u64);work=p.work;if p.done{done=true;break;}}assert!(done);let expected:Vec<u8>=serde_json::from_value(row["expected"].clone()).unwrap();assert_eq!(job.result().unwrap().pixels,expected,"{} grant {grant}",row["name"]);close_scene(job);}}
}
#[test]
fn painted_scenes_refuse_cancelled_partial_over_budget_and_reopened_scopes() {
 let cases:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎬️scene/📷️raster/🔣️.json")).unwrap();
 for steps in [0,1,3,10,30,100]{let mut job=scene(input(&cases[5]["input"]));assert!(job.result().is_err());for _ in 0..steps{job.turn(1,|owner,work|owner.advance(1,work)).unwrap();}job.cancel();assert!(job.turn(1,|owner,work|owner.advance(1,work)).is_err());assert!(job.result().is_err());close_scene(job);}
 let mut value=input(&cases[5]["input"]);value.max_pixels=1;let mut job=scene(value);assert!(through(&mut job,100000).is_err());assert!(job.result().is_err());assert!(job.turn(1,|owner,work|owner.advance(1,work)).is_err());close_scene(job);
 let mut value=input(&cases[4]["input"]);let mut plain=value.nodes[0].clone();plain.groups.clear();value.nodes.insert(1,plain);let mut job=scene(value);assert!(through(&mut job,100000).is_err());assert!(job.result().is_err());close_scene(job);
}

#[test]
fn painted_scenes_crops_retain_uncropped_curve_paint_and_stroke() {
 use semio_s_artifact_draw_drawing_model::stroke::{StrokeCap, StrokeJoin};
 use semio_s_artifact_draw_drawing_model::GradientStop;
 let mut paths=vec![
  vec![PathSegment::Move{to:[2.0,3.0]},PathSegment::Cubic{ctrl1:[13.0,-6.0],ctrl2:[-4.0,16.0],to:[10.0,9.0]},PathSegment::Close],
  vec![PathSegment::Move{to:[2.0,3.0]},PathSegment::Quad{ctrl:[9.0,15.0],to:[12.0,3.0]}],
 ];
 for large_arc in [false,true]{for sweep in [false,true]{paths.push(vec![PathSegment::Move{to:[2.0,3.0]},PathSegment::Arc{rx:7.0,ry:2.0,rotation:35.0,large_arc,sweep,to:[12.0,3.0]},PathSegment::Close]);}}
 let mut count=0;
 for segments in paths{for cap in [StrokeCap::Butt,StrokeCap::Round,StrokeCap::Square]{for join in [StrokeJoin::Miter,StrokeJoin::Round,StrokeJoin::Bevel]{
  let transform=if count%2==1{[1.0,0.2,0.4,0.8,5.0,4.0]}else{[-0.8,0.2,0.3,0.7,18.0,3.0]};
  let fill=Some(FillStyle::LinearGradient{x1:0.0,y1:0.0,x2:20.0,y2:12.0,stops:vec![GradientStop{offset:0.0,color:[1.0,0.0,0.0,1.0]},GradientStop{offset:1.0,color:[0.0,0.0,1.0,1.0]}].into()});
  let stroke=Some(StrokeStyle{color:[0.2,0.6,0.1,1.0],width:0.75,cap,join,dash:Some(vec![0.8,0.5].into())});
  let mut full=painted(PathRasterInput{width:32,height:24,origin:[-0.25,0.5],segments:segments.clone(),transform,tolerance:0.001,fill_rule:FillRule::Evenodd,fill:fill.clone(),stroke:stroke.clone()});while !full.turn(4096,|owner,work|owner.advance(4096,work)).unwrap().done{}
  let node=RasterSceneNode{id:"path".into(),groups:Vec::new(),transform,opacity:1.0,blend_mode:"normal".into(),visible:true,content:RasterSceneContent::Path{segments:segments.clone(),fill_rule:FillRule::Evenodd,fill,stroke}};
  let mut scene=scene(RasterSceneInput{width:32,height:24,origin:[-0.25,0.5],tolerance:0.001,max_pixels:768,max_source_bytes:268439552,assets:Vec::new(),nodes:vec![node]});while !scene.turn(4096,|owner,work|owner.advance(4096,work)).unwrap().done{}
  assert_eq!(scene.result().unwrap().pixels,full.result().unwrap().pixels,"crop {count}");full.finish();close_scene(scene);count+=1;
 }}}
}

#[test]
fn painted_scenes_large_ellipse_bounds_retain_visible_short_arc() {
 use semio_s_artifact_draw_drawing_model::stroke::{StrokeCap, StrokeJoin};
 let segments=vec![PathSegment::Move{to:[2.0,3.0]},PathSegment::Arc{rx:1e9,ry:1e9,rotation:0.0,large_arc:false,sweep:true,to:[12.0,3.0]}];let transform=IDENTITY;
 let stroke=Some(StrokeStyle{color:[1.0,0.0,0.0,1.0],width:0.75,cap:StrokeCap::Round,join:StrokeJoin::Miter,dash:None});
 let mut full=painted(PathRasterInput{width:16,height:8,origin:[0.0;2],segments:segments.clone(),transform,tolerance:0.001,fill_rule:FillRule::Nonzero,fill:None,stroke:stroke.clone()});while !full.turn(4096,|owner,work|owner.advance(4096,work)).unwrap().done{}
 let node=RasterSceneNode{id:"short-arc".into(),groups:Vec::new(),transform,opacity:1.0,blend_mode:"normal".into(),visible:true,content:RasterSceneContent::Path{segments,fill_rule:FillRule::Nonzero,fill:None,stroke}};
 let mut scene=scene(RasterSceneInput{width:16,height:8,origin:[0.0;2],tolerance:0.001,max_pixels:128,max_source_bytes:268439552,assets:Vec::new(),nodes:vec![node]});while !scene.turn(4096,|owner,work|owner.advance(4096,work)).unwrap().done{}
 assert_eq!(scene.result().unwrap().pixels,full.result().unwrap().pixels);full.finish();close_scene(scene);
}

#[test]
fn painted_scenes_decode_shared_encoded_assets_under_every_grant(){
 let rows:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎬️scene/📷️raster/🖼️assets/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap(){for grant in [1,7,4096]{let mut job=scene(input(&row["input"]));let mut work=0;let mut done=false;
  for _ in 0..2000000{let p=job.turn(grant,|owner,work|owner.advance(grant,work)).unwrap();assert!(p.work-work<=grant as u64);work=p.work;if p.done{assert_eq!(p.admitted_images,row["expectedDecodes"].as_u64().unwrap()as usize,"{}",row["name"]);assert_eq!(p.source_bytes,row["expectedSourceBytes"].as_u64().unwrap()as usize);assert_eq!(p.assets,row["input"]["assets"].as_array().unwrap().len());if let Some(pixels)=row["expectedAllocatedPixels"].as_u64(){assert_eq!(p.pixels,pixels as usize);}done=true;break;}}
  assert!(done);let expected:Vec<u8>=serde_json::from_value(row["expected"].clone()).unwrap();assert_eq!(finish_scene(job).pixels,expected,"{} grant {grant}",row["name"]);
 }}
}
#[test]
fn painted_scenes_refuse_malformed_encoded_assets_and_catalogs(){let rows:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎬️scene/📷️raster/🖼️assets/⚠️invalid/🔣️.json")).unwrap();for row in rows.as_array().unwrap(){let mut job=scene(input(&row["input"]));let mut failed=false;for _ in 0..2000000{match job.turn(7,|owner,work|owner.advance(7,work)){Err(_)=>{failed=true;break;},Ok(p)=>if p.done{break;}}}assert!(failed,"{}",row["name"]);assert!(job.result().is_err());assert!(job.turn(1,|owner,work|owner.advance(1,work)).is_err());close_scene(job);}}
#[test]
fn painted_scenes_cancel_live_source_asset_phases_and_count_utf8_bytes(){
 let rows:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎬️scene/📷️raster/🖼️assets/🔣️.json")).unwrap();let value=input(&rows[5]["input"]);let mut probe=scene(value.clone());let mut seen=BTreeSet::new();let mut steps=0;
 loop{let p=probe.turn(1,|owner,work|owner.advance(1,work)).unwrap();steps+=1;if !p.done&&seen.insert(p.phase){let mut job=scene(value.clone());for _ in 0..steps{job.turn(1,|owner,work|owner.advance(1,work)).unwrap();}job.cancel();assert!(job.turn(1,|owner,work|owner.advance(1,work)).is_err());assert!(job.result().is_err());close_scene(job);}if p.done{break;}}close_scene(probe);
 for phase in ["assets","image","compositing"]{assert!(seen.contains(phase));}
 let mut value=input(&rows[0]["input"]);value.nodes.clear();value.assets=vec![RasterSceneAsset{id:"samples".into(),image:Arc::new(RasterImage{width:1,height:1,pixels:vec![1,2,3,4]})}];value.max_source_bytes=4;
 let mut job=scene(value.clone());assert_eq!(through(&mut job,100000).unwrap().source_bytes,4);assert!(job.result().is_ok());close_scene(job);value.max_source_bytes=3;let mut job=scene(value);assert!(through(&mut job,100000).is_err());close_scene(job);
}
#[test]
fn painted_scenes_receive_admitted_sample_owners_without_physical_decode(){
 let asset=RasterSceneAsset{id:"sample-owner".into(),image:Arc::new(RasterImage{width:1,height:1,pixels:vec![10,20,30,255]})};let original=Arc::clone(&asset.image);
 let mut input=input(&serde_json::from_str::<serde_json::Value>(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎬️scene/📷️raster/🔣️.json")).unwrap()[0]["input"]);input.nodes.clear();input.assets=vec![asset];let mut job=scene(input);assert!(job.result().is_err());while !job.turn(1,|owner,work|owner.advance(1,work)).unwrap().done{}assert_eq!(original.pixels,vec![10,20,30,255]);close_scene(job);assert_eq!(Arc::strong_count(&original),1);
}

#[test]
fn painted_scene_actual_compositor_retains_private_owner_until_caller_close(){
 let rows:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎬️scene/📷️raster/🧹️compositing/🔣️.json")).unwrap();let cases:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎬️scene/📷️raster/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap(){for grant in [1,7,4096]{let source=cases.as_array().unwrap().iter().find(|v|v["name"]==row["source"]).unwrap();let mut job=scene(input(&source["input"]));let mut moved=None;let mut done=false;
  for _ in 0..2_000_000{if job.phase=="compositingCleanup"{assert!(job.compositor.is_none());assert!(job.result().is_err());if let Some(owner)=&job.compositor_retirement{assert!(!owner.terminal_is_empty());}else{assert!(!job.retired_compositors.last().unwrap().terminal_is_empty());}moved=Some(job.output.as_ref().unwrap().pixels.as_ptr());}if job.turn(grant,|owner,work|owner.advance(grant,work)).unwrap().done{done=true;break;}}
  assert!(done);assert_eq!(job.retired_compositors.len(),1);assert!(!job.retired_compositors[0].terminal_is_empty());if let Some(pointer)=moved{assert_eq!(job.result().unwrap().pixels.as_ptr(),pointer);}let output=finish_scene(job);let expected:Vec<u8>=serde_json::from_value(source["expected"].clone()).unwrap();assert_eq!(output.pixels,expected);
 }}
}

#[test]fn actual_scene_raster_owns_funded_payloads_through_denial_and_close(){let rows:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎬️scene/📷️raster/🔣️.json")).unwrap();let row=&rows.as_array().unwrap()[0];let mut job=RasterSceneJob::new(input(&row["input"]));let mut bank=raster_wallet();let mut denied=[false;4];loop{let d=job.next_work_demand();for axis in 0..4{let n=match axis{0=>1,1=>d.copy_bytes,2=>d.capacity_bytes,_=>d.depth};if n==0||denied[axis]{continue;}let mut grant=bank.remaining();*match axis{0=>&mut grant.maximum_items,1=>&mut grant.maximum_copy_bytes,2=>&mut grant.maximum_capacity_bytes,_=>&mut grant.maximum_depth}=n-1;let((p,receipt),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let mut work=semio_framework_value::work::RetainedWorkLedger::new(grant);let p=job.advance(1,&mut work).unwrap();(p,work.receipt())});assert!(!p.done);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(receipt,Default::default());assert_eq!(job.last_turn_work(),0);denied[axis]=true;}if bank.turn(1,|work|job.advance(1,work)).unwrap().done{break;}}assert!(denied.into_iter().all(|v|v));let expected:Vec<u8>=serde_json::from_value(row["expected"].clone()).unwrap();assert_eq!(job.result().unwrap().pixels,expected);bank.close_owned(job.into_retirement());}
