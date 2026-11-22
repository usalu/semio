//! 🧫️ Native scene fixtures, foreign PNG decoding and exact physical writer custody.
use super::*;
#[path="../../../../../../🧪️tests/🎟️vector/🦀️.rs"]
mod original_vector;
use semio_framework_test_fixtures::fixture_text;
use semio_s_artifact_draw_drawing_scene::preparation::{DocumentSceneLimits, DocumentAlgorithmLimits, DocumentSceneNode};
use semio_s_artifact_draw_drawing_scene::{booleans::DocumentBooleanLimits, raster::RasterSceneAsset, trace::DocumentTraceLimits};
use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::RetainedCloneGrant};
use serde_json::Value;
use semio_s_artifact_draw_drawing_io::byte_writer::physical_close as close;
fn limits()->SvgWriteLimits{SvgWriteLimits{view_box:[0.0,0.0,2.0,1.0],max_output_bytes:65536,max_work:1000000000}}
fn algorithms()->DocumentAlgorithmLimits{DocumentAlgorithmLimits{max_work:10000000,trace:DocumentTraceLimits{max_pixels:100000,max_admitted_pixels:100000,max_source_bytes:100000,max_edges:65536,max_segments:65536,max_retained_segments:65536,max_work:10000000},booleans:DocumentBooleanLimits{tolerance:0.005,epsilon:1e-8,max_depth:32,max_references:256,max_edges:65536,max_parameters:262144,max_atomic_edges:65536,max_segments:65536,max_retained_segments:262144,max_work:10000000}}}
fn finish(job:&mut SvgWriteJob,grant:usize)->String{let mut work=0;for _ in 0..1000000{let p=job.advance(grant).unwrap();assert!(p.work-work<=grant as u64);work=p.work;if p.done{let mut bytes=Vec::new();for page in job.result().unwrap().iter(){assert!(page.len()<=PAGE);bytes.extend_from_slice(page);}return String::from_utf8(bytes).unwrap();}assert!(job.result().is_err());}panic!("SVG writer failed to terminate")}
#[test]
fn svg_write_neutral_styles_preserve_authoritative_scene_and_physical_close(){
 let rows:Vec<Value>=serde_json::from_str(fixture_text!("../../🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🧵️write/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let docs:Vec<Value>=serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🎬️scene/📋️prepare/🔣️.json")).unwrap();let styles:Vec<Value>=serde_json::from_str(fixture_text!("../../../../✏️editor/🎮️commands/📤️export-document/🧪️tests/🧫️fixtures/🎨️styles/🔣️.json")).unwrap();
 for row in rows{let source=docs.iter().find(|d|d["name"]==row["source"]).unwrap();let mut raw=source["document"].clone();if row["styles"]!=""{raw["layers"]=styles.iter().find(|s|s["name"]==row["styles"]).unwrap()["layers"].clone();}let document:crate::DrawingSnapshot=serde_json::from_value(raw).unwrap();let before=document.clone();for grant in [1,7,4096]{let l=&source["limits"];let(vector_close,plan)=original_vector::prepare(&document,DocumentSceneLimits{max_nodes:16,max_depth:l["maxDepth"].as_u64().unwrap()as usize,max_segments:l["maxSegments"].as_u64().unwrap()as usize,max_references:l["maxReferences"].as_u64().unwrap()as usize,max_source_bytes:l["maxSourceBytes"].as_u64().unwrap()as usize},algorithms());let mut job=SvgWriteJob::new(plan,limits());let svg=finish(&mut job,grant);assert!(semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::io::text::snapshot::parse_svg_xml(&svg).is_ok());match row["styles"].as_str().unwrap(){"linear gradient"=>assert!(svg.contains("<linearGradient")),"radial gradient"=>assert!(svg.contains("<radialGradient")),"multiply blend"=>assert!(svg.contains("mix-blend-mode:multiply")),"isolated group opacity"=>assert!(svg.contains("opacity=\"0.5\" style=\"isolation:isolate")),"evenodd hole"=>assert!(svg.contains("fill-rule=\"evenodd\"")),_=>assert!(svg.contains("rgb(255,0,0)"))};let(retired,pages)=job.into_retirement();close(retired);close(pages.unwrap());vector_close.close();assert_eq!(document,before);}}
}
#[test]
fn svg_write_shared_image_matches_foreign_png_and_original_lease() {
 let law: Value = serde_json::from_str(fixture_text!("../../🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🧵️write/🧪️tests/🧫️fixtures/📷️image/🔣️.json")).unwrap();
 let width = law["width"].as_u64().unwrap() as u32;
 let height = law["height"].as_u64().unwrap() as u32;
 let samples = law["rgba"].as_array().unwrap().iter().map(|n| n.as_u64().unwrap() as u8).collect();
 let image = std::sync::Arc::new(semio_framework_pixels::RasterImage { width, height, pixels: samples });
 let pointer = image.pixels.as_ptr();
 let plan = || DocumentScenePlan {
  assets: vec![RasterSceneAsset { id: "image".into(), image: image.clone() }],
  nodes: vec![DocumentSceneNode { source_path: vec![0], locked_ancestors: 0, id: law["id"].as_str().unwrap().into(), groups: Vec::new(), transform: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], opacity: 1.0, blend_mode: "normal".into(), visible: true, content: DocumentSceneContent::Image { asset: "image".into(), width: f64::from(width), height: f64::from(height) } }],
 };
 let mut standalone = SvgWriteJob::new(plan(), limits());
 let expected = finish(&mut standalone, 1);
 let (retired, pages) = standalone.into_retirement();
 close(retired);
 close(pages.unwrap());

 let mut job = SvgWriteJob::new(plan(), limits());
 let stock = RetainedCloneGrant { maximum_items: 1 << 16, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 256 };
 let mut caller = semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(41), semio_framework_job::Generation(1), stock, || Some(0), u64::MAX, 1 << 20);
 let png_demand = Encoder::birth_demand(&image, limits().max_output_bytes).unwrap();
 let mut tested_png_denial = false;
 let fuel = law["fuel"].as_u64().unwrap();
 use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
 for _ in 0..10000 {
  if !tested_png_denial && matches!(job.phase, 11 | 20) && job.next_demand().unwrap().capacity_bytes == png_demand.capacity_bytes {
   tested_png_denial = true;
   let before = job.progress();
   caller.bare_turn(RetainedCloneGrant { maximum_capacity_bytes: png_demand.capacity_bytes - 1, ..stock }, |cx| {
    let posted = cx.retained_progress();
    let (result, heap) = observe(|| job.advance_original(cx));
    let after = result.unwrap();
    assert_eq!((after.work, after.bytes), (before.work, before.bytes));
    assert_eq!(cx.retained_progress(), posted);
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    Ok(())
   }).unwrap();
  }
  let done = caller.bare_turn(stock, |cx| cx.with_original_work_loan(fuel, u64::MAX, |cx| {
   let before = cx.retained_progress();
   let (result, heap) = observe(|| job.advance_original(cx));
   let after = cx.retained_progress();
   assert!(!heap.overflowed);
   assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
   Ok(result.unwrap().done)
  })).unwrap();
  if done { break; }
 }
 assert!(tested_png_denial);
 caller.retire_exact();
 let bytes: Vec<_> = job.result().unwrap().iter().flatten().copied().collect();
 let svg = String::from_utf8(bytes).unwrap();
 assert_eq!(svg, expected);
 assert!(svg.contains(law["escapedId"].as_str().unwrap()));
 let body = svg.split("data:image/png;base64,").nth(1).unwrap().split('"').next().unwrap();
 let data = base64_codec::base64_standard_decode(body).unwrap();
 let mut reader = png::Decoder::new(std::io::Cursor::new(data)).read_info().unwrap();
 let mut pixels = vec![0; reader.output_buffer_size()];
 let frame = reader.next_frame(&mut pixels).unwrap();
 assert_eq!(&pixels[..frame.buffer_size()], &image.pixels);
 let (retired, pages) = job.into_retirement();
 close(retired);
 close(pages.unwrap());
 assert_eq!(std::sync::Arc::strong_count(&image), 1);
 assert_eq!(image.pixels.as_ptr(), pointer);
 close(RasterLease(image));
}
#[test]
fn svg_write_interruptions_and_refusals_keep_original_candidate_until_funded_close(){for stop in [0,1,7,40]{let mut job=SvgWriteJob::new(DocumentScenePlan::default(),limits());job.advance(stop.max(1)).unwrap();job.cancel();assert!(job.result().is_err());let(retired,pages)=job.into_retirement();assert!(pages.is_none());close(retired);}for maximum in [1,64]{let mut l=limits();l.max_output_bytes=maximum;let mut job=SvgWriteJob::new(DocumentScenePlan::default(),l);while job.advance(1).is_ok(){}assert!(job.result().is_err());let(retired,pages)=job.into_retirement();assert!(pages.is_none());close(retired);}}

#[test]
fn svg_write_page_bound_ignores_allocator_extra_capacity() {
 let law: Value = serde_json::from_str(fixture_text!("../../🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🧵️write/🧪️tests/🧫️fixtures/📷️image/🔣️.json")).unwrap();
 let bounds = &law["pageBounds"];
 let page_capacity = bounds["pageCapacity"].as_u64().unwrap() as usize;
 let allocator_capacity = bounds["allocatorCapacity"].as_u64().unwrap() as usize;
 let final_page = bounds["finalPageBytes"].as_u64().unwrap() as usize;
 let excess = bounds["excessBytes"].as_u64().unwrap() as usize;
 let limit = page_capacity + final_page;
 let mut job = SvgWriteJob::new(DocumentScenePlan::default(), SvgWriteLimits { max_output_bytes: limit, ..limits() });
 job.owners.pages.try_push(Vec::with_capacity(allocator_capacity)).unwrap();
 job.pending.write_bytes(&vec![b'x'; limit + excess]).unwrap();
 loop {
  if let Err(error) = job.advance(1) {
   assert_eq!(error, "SVG output byte limit exceeded");
   break;
  }
 }
 assert_eq!(job.owners.pages.iter().map(Vec::len).collect::<Vec<_>>(), [page_capacity, final_page]);
 assert_eq!(job.pending.len - job.pending.at, excess);
 let (retired, output) = job.into_retirement();
 assert!(output.is_none());
 close(retired);
}
