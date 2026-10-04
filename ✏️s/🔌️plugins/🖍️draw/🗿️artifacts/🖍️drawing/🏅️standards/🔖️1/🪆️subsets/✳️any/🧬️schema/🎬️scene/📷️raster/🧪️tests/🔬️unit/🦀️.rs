use super::*;
#[test]
fn painted_scenes_filter_every_shared_image_under_bounded_grants(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🖼️images/🔣️.json")).unwrap();
 for row in cases.as_array().unwrap(){for grant in [1,7,4096]{let mut job=RasterSceneJob::new(input(&row["input"])).unwrap();let mut work=0;let mut done=false;for _ in 0..2000000{let p=job.advance(grant).unwrap();assert!(p.work-work<=grant as u64);work=p.work;if p.done{done=true;break;}}assert!(done);let expected:Vec<u8>=serde_json::from_value(row["expected"].clone()).unwrap();assert_eq!(job.result().unwrap().pixels,expected,"{} grant {grant}",row["name"]);}}
 eprintln!("[DEBUG] {} filtered scene images matched native neutral RGBA under grants 1, 7 and 4096",cases.as_array().unwrap().len());
}
#[test]
fn painted_scenes_filtering_admits_source_and_crop_and_cancels_reduction(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🖼️images/🔣️.json")).unwrap();
 let mut grid=input(&cases[0]["input"]);grid.max_pixels=1;let mut job=RasterSceneJob::new(grid.clone()).unwrap();assert_eq!(job.advance(100000).unwrap().pixels,1);assert!(job.result().is_ok());
 let mut shifted=grid.clone();shifted.width=2;shifted.nodes[0].transform[4]=0.5;shifted.max_pixels=2;let mut job=RasterSceneJob::new(shifted.clone()).unwrap();assert!(job.advance(100000).is_err());assert!(job.result().is_err());shifted.max_pixels=3;let mut job=RasterSceneJob::new(shifted).unwrap();assert_eq!(job.advance(100000).unwrap().pixels,3);assert!(job.result().is_ok());
 grid.nodes[0].transform[4]=1e8;let mut job=RasterSceneJob::new(grid.clone()).unwrap();assert_eq!(job.advance(100000).unwrap().pixels,0);
 grid.nodes[0].transform=[1.0/64.0,0.0,0.0,1.0/64.0,0.0,0.0];grid.nodes[0].content=RasterSceneContent::Pixels(Arc::new(RasterImage{width:64,height:64,pixels:vec![255;64*64*4]}));grid.max_pixels=4097;
 let mut job=RasterSceneJob::new(grid).unwrap();let mut seen=false;for _ in 0..100{let p=job.advance(1).unwrap();if p.phase=="image"{seen=true;break;}}assert!(seen);job.cancel();assert!(job.advance(1).is_err());assert!(job.result().is_err());
}
fn input(v:&serde_json::Value)->RasterSceneInput {
 RasterSceneInput {width:v["width"].as_u64().unwrap() as u32,height:v["height"].as_u64().unwrap() as u32,origin:serde_json::from_value(v["origin"].clone()).unwrap(),tolerance:v["tolerance"].as_f64().unwrap(),max_pixels:v["maxPixels"].as_u64().unwrap() as usize,max_source_bytes:v["maxSourceBytes"].as_u64().unwrap()as usize,max_bytes:v["maxBytes"].as_u64().unwrap()as usize,max_chunks:v["maxChunks"].as_u64().unwrap()as usize,assets:v["assets"].as_array().unwrap().iter().map(|a|RasterSceneAsset{id:a["id"].as_str().unwrap().into(),mime:a["mime"].as_str().unwrap().into(),data:Arc::new(a["data"].as_str().unwrap().into())}).collect(),nodes:v["nodes"].as_array().unwrap().iter().map(|n|{
 let c=&n["content"];let content=if c["kind"]=="path" {RasterSceneContent::Path {segments:serde_json::from_value(c["segments"].clone()).unwrap(),fill_rule:serde_json::from_value(c["fillRule"].clone()).unwrap(),fill:serde_json::from_value(c["fill"].clone()).unwrap(),stroke:serde_json::from_value(c["stroke"].clone()).unwrap()}}else if c["kind"]=="image"{RasterSceneContent::Image{asset:c["asset"].as_str().unwrap().into(),width:c["width"].as_f64().unwrap(),height:c["height"].as_f64().unwrap()}}else{let i=&c["image"];RasterSceneContent::Pixels(Arc::new(RasterImage {width:i["width"].as_u64().unwrap() as u32,height:i["height"].as_u64().unwrap() as u32,pixels:serde_json::from_value(i["pixels"].clone()).unwrap()}))};
 RasterSceneNode {id:n["id"].as_str().unwrap().into(),groups:n["groups"].as_array().unwrap().iter().map(|g|DrawingSceneGroup{id:g["id"].as_str().unwrap().into(),opacity:g["opacity"].as_f64().unwrap(),blend_mode:g["blendMode"].as_str().unwrap().into()}).collect(),transform:serde_json::from_value(n["transform"].clone()).unwrap(),opacity:n["opacity"].as_f64().unwrap(),blend_mode:n["blendMode"].as_str().unwrap().into(),visible:n["visible"].as_bool().unwrap(),content}
 }).collect()}
}
#[test]
fn painted_scenes_match_shared_rgba_under_bounded_grants() {
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
 for row in cases.as_array().unwrap(){for grant in [1,7,4096]{let mut job=RasterSceneJob::new(input(&row["input"])).unwrap();let mut work=0;let mut done=false;for _ in 0..2000000{let p=job.advance(grant).unwrap();assert!(p.work-work<=grant as u64);work=p.work;if p.done{done=true;break;}}assert!(done);let expected:Vec<u8>=serde_json::from_value(row["expected"].clone()).unwrap();assert_eq!(job.result().unwrap().pixels,expected,"{} grant {grant}",row["name"]);}}
 eprintln!("[DEBUG] Nineteen resolved scenes matched native RGBA under grants 1, 7 and 4096");
}
#[test]
fn painted_scenes_refuse_cancelled_partial_over_budget_and_reopened_scopes() {
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
 for steps in [0,1,3,10,30,100]{let mut job=RasterSceneJob::new(input(&cases[5]["input"])).unwrap();assert!(job.result().is_err());for _ in 0..steps{job.advance(1).unwrap();}job.cancel();assert!(job.advance(1).is_err());assert!(job.result().is_err());}
 let mut value=input(&cases[5]["input"]);value.max_pixels=1;let mut job=RasterSceneJob::new(value).unwrap();assert!(job.advance(100000).is_err());assert!(job.result().is_err());assert!(job.advance(1).is_err());
 let mut value=input(&cases[4]["input"]);let mut plain=value.nodes[0].clone();plain.groups.clear();value.nodes.insert(1,plain);let mut job=RasterSceneJob::new(value).unwrap();assert!(job.advance(100000).is_err());assert!(job.result().is_err());
}

#[test]
fn painted_scenes_crops_retain_uncropped_curve_paint_and_stroke() {
 use crate::{StrokeCap,StrokeJoin,GradientStop};
 let mut paths=vec![
  vec![PathSegment::Move{to:[2.0,3.0]},PathSegment::Cubic{ctrl1:[13.0,-6.0],ctrl2:[-4.0,16.0],to:[10.0,9.0]},PathSegment::Close],
  vec![PathSegment::Move{to:[2.0,3.0]},PathSegment::Quad{ctrl:[9.0,15.0],to:[12.0,3.0]}],
 ];
 for large_arc in [false,true]{for sweep in [false,true]{paths.push(vec![PathSegment::Move{to:[2.0,3.0]},PathSegment::Arc{rx:7.0,ry:2.0,rotation:35.0,large_arc,sweep,to:[12.0,3.0]},PathSegment::Close]);}}
 let mut count=0;
 for segments in paths{for cap in [StrokeCap::Butt,StrokeCap::Round,StrokeCap::Square]{for join in [StrokeJoin::Miter,StrokeJoin::Round,StrokeJoin::Bevel]{
  let transform=if count%2==1{[1.0,0.2,0.4,0.8,5.0,4.0]}else{[-0.8,0.2,0.3,0.7,18.0,3.0]};
  let fill=Some(FillStyle::LinearGradient{x1:0.0,y1:0.0,x2:20.0,y2:12.0,stops:vec![GradientStop{offset:0.0,color:[1.0,0.0,0.0,1.0]},GradientStop{offset:1.0,color:[0.0,0.0,1.0,1.0]}]});
  let stroke=Some(StrokeStyle{color:[0.2,0.6,0.1,1.0],width:0.75,cap,join,dash:Some(vec![0.8,0.5])});
  let mut full=PathRasterJob::new(PathRasterInput{width:32,height:24,origin:[-0.25,0.5],segments:segments.clone(),transform,tolerance:0.001,fill_rule:FillRule::Evenodd,fill:fill.clone(),stroke:stroke.clone()}).unwrap();while !full.advance(4096).unwrap().done{}
  let node=RasterSceneNode{id:"path".into(),groups:Vec::new(),transform,opacity:1.0,blend_mode:"normal".into(),visible:true,content:RasterSceneContent::Path{segments:segments.clone(),fill_rule:FillRule::Evenodd,fill,stroke}};
  let mut scene=RasterSceneJob::new(RasterSceneInput{width:32,height:24,origin:[-0.25,0.5],tolerance:0.001,max_pixels:768,max_source_bytes:268439552,max_bytes:67108864,max_chunks:65536,assets:Vec::new(),nodes:vec![node]}).unwrap();while !scene.advance(4096).unwrap().done{}
  assert_eq!(scene.result().unwrap().pixels,full.result().unwrap().pixels,"crop {count}");count+=1;
 }}}
 eprintln!("[DEBUG] {count} native cropped curve/stroke scenes matched uncropped path output");
}

#[test]
fn painted_scenes_large_ellipse_bounds_retain_visible_short_arc() {
 use crate::{StrokeCap,StrokeJoin};
 let segments=vec![PathSegment::Move{to:[2.0,3.0]},PathSegment::Arc{rx:1e9,ry:1e9,rotation:0.0,large_arc:false,sweep:true,to:[12.0,3.0]}];let transform=IDENTITY;
 let stroke=Some(StrokeStyle{color:[1.0,0.0,0.0,1.0],width:0.75,cap:StrokeCap::Round,join:StrokeJoin::Miter,dash:None});
 let mut full=PathRasterJob::new(PathRasterInput{width:16,height:8,origin:[0.0;2],segments:segments.clone(),transform,tolerance:0.001,fill_rule:FillRule::Nonzero,fill:None,stroke:stroke.clone()}).unwrap();while !full.advance(4096).unwrap().done{}
 let node=RasterSceneNode{id:"short-arc".into(),groups:Vec::new(),transform,opacity:1.0,blend_mode:"normal".into(),visible:true,content:RasterSceneContent::Path{segments,fill_rule:FillRule::Nonzero,fill:None,stroke}};
 let mut scene=RasterSceneJob::new(RasterSceneInput{width:16,height:8,origin:[0.0;2],tolerance:0.001,max_pixels:128,max_source_bytes:268439552,max_bytes:67108864,max_chunks:65536,assets:Vec::new(),nodes:vec![node]}).unwrap();while !scene.advance(4096).unwrap().done{}
 assert_eq!(scene.result().unwrap().pixels,full.result().unwrap().pixels);eprintln!("[DEBUG] Native large ellipse bounds preserved the visible short arc");
}

#[test]
fn painted_scenes_decode_shared_encoded_assets_under_every_grant(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🖼️assets/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap(){for grant in [1,7,4096]{let mut job=RasterSceneJob::new(input(&row["input"])).unwrap();let mut work=0;let mut done=false;
  for _ in 0..2000000{let p=job.advance(grant).unwrap();assert!(p.work-work<=grant as u64);work=p.work;if p.done{assert_eq!(p.decodes,row["expectedDecodes"].as_u64().unwrap()as usize,"{}",row["name"]);assert_eq!(p.source_bytes,row["expectedSourceBytes"].as_u64().unwrap()as usize);assert_eq!(p.assets,row["input"]["assets"].as_array().unwrap().len());if let Some(pixels)=row["expectedAllocatedPixels"].as_u64(){assert_eq!(p.pixels,pixels as usize);}done=true;break;}}
  assert!(done);let expected:Vec<u8>=serde_json::from_value(row["expected"].clone()).unwrap();assert_eq!(job.into_result().unwrap().pixels,expected,"{} grant {grant}",row["name"]);
 }}
 eprintln!("[DEBUG] All {} encoded image scenes preserved native neutral RGBA and shared asset budgets",rows.as_array().unwrap().len());
}
#[test]
fn painted_scenes_refuse_malformed_encoded_assets_and_catalogs(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🖼️assets/⚠️invalid/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap(){if let Ok(mut job)=RasterSceneJob::new(input(&row["input"])){let mut failed=false;for _ in 0..2000000{match job.advance(7){Err(_)=>{failed=true;break;},Ok(p)=>if p.done{break;}}}assert!(failed,"{}",row["name"]);assert!(job.result().is_err());assert!(job.advance(1).is_err());}}
 eprintln!("[DEBUG] All {} invalid encoded scene inputs refused native publication",rows.as_array().unwrap().len());
}
#[test]
fn painted_scenes_cancel_live_source_asset_phases_and_count_utf8_bytes(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🖼️assets/🔣️.json")).unwrap();let value=input(&rows[5]["input"]);let mut probe=RasterSceneJob::new(value.clone()).unwrap();let mut seen=BTreeSet::new();let mut steps=0;
 loop{let p=probe.advance(1).unwrap();steps+=1;if !p.done&&seen.insert(p.phase){let mut job=RasterSceneJob::new(value.clone()).unwrap();job.advance(steps).unwrap();job.cancel();assert!(job.advance(1).is_err());assert!(job.result().is_err());}if p.done{break;}}
 for phase in ["assets","source","image","compositing"]{assert!(seen.contains(phase));}
 let mut value=input(&rows[0]["input"]);value.nodes.clear();value.assets=vec![RasterSceneAsset{id:"unicode".into(),mime:"image/png".into(),data:Arc::new("é😀".into())}];value.max_source_bytes=6;
 let mut job=RasterSceneJob::new(value.clone()).unwrap();assert_eq!(job.advance(100000).unwrap().source_bytes,6);assert!(job.result().is_ok());value.max_source_bytes=5;let mut job=RasterSceneJob::new(value).unwrap();assert!(job.advance(100000).is_err());
 eprintln!("[DEBUG] Native encoded scene admission/source/filter/composition phases cancelled without publication; UTF-8 admission verified");
}
#[test]
fn painted_scenes_forward_partial_source_decode_progress(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🖼️assets/🔣️.json")).unwrap();let row=&rows[31];let mut job=RasterSceneJob::new(input(&row["input"])).unwrap();let mut phases=Vec::new();let mut partial=false;
 loop{let p=job.advance(1).unwrap();if let Some(image)=p.decoding{if phases.last()!=Some(&image.phase){phases.push(image.phase);}if image.phase=="png"&&image.pixels>0&&image.pixels<image.total_pixels{partial=true;assert!(job.result().is_err());}}if p.done{break;}}
 let expected:Vec<String>=serde_json::from_value(row["expectedDecodePhases"].clone()).unwrap();assert_eq!(phases,expected);assert!(partial);
 eprintln!("[DEBUG] Native scene observers received all source and partial PNG phases without partial image publication");
}
