//! 🧰️ Physical-close macros shared with the drawing writer laws (behind the `testkit` feature).

#[macro_export]
macro_rules! draw_scene_physical_turn{($owner:expr,$items:expr)=>{{let owner=&mut $owner;let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_release_byte_demand().unwrap();let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:$items,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(if copy>0{copy}else{release}).unwrap(),maximum_release_bytes:release,maximum_depth:owner.next_depth_demand().unwrap()};assert_eq!(owner.close_step(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:0,..grant}).unwrap().progress(),Default::default());let step=owner.close_step(grant).unwrap();assert!(step.progress().fits(grant));step}};}
#[macro_export]
macro_rules! draw_scene_physical_close{($owner:expr,$items:expr)=>{{let mut turns=0;while !$owner.terminal_is_empty(){crate::draw_scene_physical_turn!($owner,$items);turns+=1;assert!(turns<2_000_000);}turns}};}

/// 🚦️ Finite fixture authority is never renewed when a real producer cannot perform another turn.
#[macro_export]
macro_rules! draw_scene_funded_turn{($job:expr,$items:expr,$wallet:expr)=>{{let job=&mut $job;$wallet.turn($items,|work|{let before=work.receipt();let p=job.advance($items,work).unwrap();assert!(p.done||work.receipt()!=before||job.last_turn_work()>0,"finite scene fixture exhausted at {:?}",job.next_work_demand());p})}};}

/// 🧾️ The exact same finite fixture stock pays actual close progress without deriving new authority from demand.
#[macro_export]
macro_rules! draw_scene_funded_close{($owner:expr,$wallet:expr,$items:expr)=>{{let owner=&mut $owner;let wallet=&mut $wallet;let mut turns=0;let mut work=0;while !owner.terminal_is_empty(){for _ in 0..$items{if owner.terminal_is_empty(){break}let copy_bytes=owner.next_copy_byte_demand().unwrap();let release_bytes=owner.next_release_byte_demand().unwrap();let d=semio_framework_value::RetirementDemand{copy_bytes,capacity_bytes:owner.next_capacity_byte_demand(if copy_bytes>0{copy_bytes}else{release_bytes}).unwrap(),release_bytes,depth:owner.next_depth_demand().unwrap()};let p=wallet.close_turn(d,|g|owner.close_step(g)).unwrap().progress();assert_ne!(p,Default::default());work+=p.copied_items as u64;turns+=1;assert!(turns<2_000_000);}}work}};}

/// 🧪️ Drives the real document vector job to its completed plan under a finite stock and drains its physical owners.
pub fn scene_plan(document:&semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot)->crate::preparation::DocumentScenePlan{
 use semio_framework_value::{work::RetainedWorkLedger,retained_clone::RetainedCloneGrant};
 const STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1<<22,maximum_copy_bytes:1<<28,maximum_capacity_bytes:1<<25,maximum_release_bytes:1<<26,maximum_depth:256};
 let mut job=crate::preparation::DocumentVectorJob::new(document,Default::default(),Default::default()).unwrap();
 loop{let progress=job.advance(4096,&mut RetainedWorkLedger::new(STOCK)).unwrap();if progress.done{break}assert!(job.last_turn_work()>0,"scene fixture made no progress under its stock");}
 let(mut closing,plan)=job.into_retirement();
 draw_scene_physical_close!(closing,1);
 plan.unwrap()
}

/// 🧪️ Projects a completed plan onto the flat node records tests assert on: visible paths, glyph text and images.
pub fn scene_nodes_of_plan(plan:&crate::preparation::DocumentScenePlan)->Vec<semio_s_artifact_draw_drawing_model::DrawingSceneNode>{
 use crate::preparation::DocumentSceneContent as C;
 use semio_s_artifact_draw_drawing_model::{DrawingSceneNode,DrawingSceneText,DrawingSceneImage};
 let rule=|rule:&semio_s_artifact_draw_drawing_model::fill_rule::FillRule|match rule{semio_s_artifact_draw_drawing_model::fill_rule::FillRule::Evenodd=>"evenodd",semio_s_artifact_draw_drawing_model::fill_rule::FillRule::Nonzero=>"nonzero"};
 plan.nodes.iter().filter(|node|node.visible).filter_map(|node|{
  let mut out=DrawingSceneNode{id:node.id.clone(),groups:node.groups.clone(),transform:node.transform,segments:Default::default(),fill:None,stroke:None,opacity:node.opacity,blend_mode:node.blend_mode.clone().into(),visible:node.visible,fill_rule:None,text:None,image:None};
  match &node.content{
   C::Path{segments,fill_rule,fill,stroke}=>{out.segments=segments.clone().into();out.fill=fill.clone();out.stroke=stroke.clone();out.fill_rule=Some(rule(fill_rule).into());},
   C::Glyphs{content,x,y,size,font_family,fill,stroke,..}=>{out.transform=semio_s_artifact_draw_drawing_geometry::multiply(node.transform,[1.0,0.0,0.0,1.0,*x,*y]);out.fill=fill.clone();out.stroke=stroke.clone();out.text=Some(DrawingSceneText{content:content.clone().into(),size:*size,font_family:*font_family});},
   C::Image{asset,width,height}=>{out.image=Some(DrawingSceneImage{asset_id:asset.clone().into(),width:*width,height:*height});},
   _=>return None,
  }
  Some(out)
 }).collect()
}

/// 🧪️ Completed visible scene nodes of one document, produced by the real vector job.
pub fn scene_nodes(document:&semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot)->Vec<semio_s_artifact_draw_drawing_model::DrawingSceneNode>{scene_nodes_of_plan(&scene_plan(document))}
