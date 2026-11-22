//! 🧫️ PDFium renders native paged bytes from shared authored styles under exact allocator custody.
use super::*;
#[path="../../../../../../🧪️tests/🎟️vector/🦀️.rs"]
mod original_vector;
use semio_framework_test_fixtures::fixture_text;
use semio_s_artifact_draw_drawing_scene::preparation::{DocumentSceneLimits, DocumentAlgorithmLimits};
use semio_s_artifact_draw_drawing_scene::{booleans::DocumentBooleanLimits, raster::RasterSceneAsset, trace::DocumentTraceLimits};
use semio_s_artifact_draw_drawing_io::byte_writer::physical_close as close;
use serde_json::Value;
use std::io::Write as _;
use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller};
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use semio_framework_value::RetainedCloneGrant;

fn finish_original(job: &mut PdfWriteJob, law: &Value) -> Vec<u8> {
 let fuel = law["fuel"].as_u64().unwrap();
 let page_capacity = law["pageCapacity"].as_u64().unwrap() as usize;
 let stock = RetainedCloneGrant { maximum_items: 1 << 17, maximum_copy_bytes: 1 << 24, maximum_capacity_bytes: 1 << 24, maximum_release_bytes: 1 << 24, maximum_depth: 256 };
 let mut caller = OriginalJobFixtureCaller::admit(OperationId(31), Generation(1), stock, || Some(0), u64::MAX, 1 << 24);
 let mut tested_page_denial = false;
 for _ in 0..100000 {
  let check_page_birth = job.next_demand().unwrap().capacity_bytes == page_capacity && !tested_page_denial;
  let grant = if check_page_birth {
   tested_page_denial = true;
   let previous = job.progress();
   caller.bare_turn(RetainedCloneGrant { maximum_capacity_bytes: page_capacity - 1, ..stock }, |cx| {
    let before = cx.retained_progress();
    let (result, heap) = observe(|| job.advance_original(cx));
    let after = result.unwrap();
    assert_eq!((after.phase, after.work, after.bytes), (previous.phase, previous.work, previous.bytes));
    assert_eq!(cx.retained_progress(), before);
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    Ok(())
   }).unwrap();
   RetainedCloneGrant { maximum_capacity_bytes: page_capacity, ..stock }
  } else { stock };
  let done = caller.bare_turn(grant, |cx| cx.with_original_work_loan(fuel, u64::MAX, |cx| {
   let before = cx.retained_progress();
   let (result, heap) = observe(|| job.advance_original(cx));
   let after = cx.retained_progress();
   assert!(!heap.overflowed);
   assert_eq!(heap.requested_bytes, after.retained_capacity_bytes - before.retained_capacity_bytes);
   assert_eq!(heap.released_bytes, after.released_bytes - before.released_bytes);
   if check_page_birth { assert_eq!(heap.requested_bytes, page_capacity); }
   Ok(result.unwrap().done)
  })).unwrap();
  if done {
   assert!(tested_page_denial);
   caller.retire_exact();
   return job.result().unwrap().iter().flatten().copied().collect();
  }
 }
 panic!("Original PDF writer failed to terminate")
}
fn limits()->PdfWriteLimits{PdfWriteLimits{view_box:[0.0,0.0,2.0,1.0],max_output_bytes:65536,max_work:1000000000}}
fn algorithms()->DocumentAlgorithmLimits{DocumentAlgorithmLimits{max_work:10000000,trace:DocumentTraceLimits{max_pixels:100000,max_admitted_pixels:100000,max_source_bytes:100000,max_edges:65536,max_segments:65536,max_retained_segments:65536,max_work:10000000},booleans:DocumentBooleanLimits{tolerance:0.005,epsilon:1e-8,max_depth:32,max_references:256,max_edges:65536,max_parameters:262144,max_atomic_edges:65536,max_segments:65536,max_retained_segments:262144,max_work:10000000}}}
fn finish(job:&mut PdfWriteJob,grant:usize)->Vec<u8>{let mut work=0;for _ in 0..1000000{let p=job.advance(grant).unwrap();assert!(p.work-work<=grant as u64);work=p.work;if p.done{let mut bytes=Vec::new();for page in job.result().unwrap().iter(){assert!(page.len()<=PAGE);bytes.extend_from_slice(page);}return bytes;}assert!(job.result().is_err());}panic!("PDF writer failed to terminate")}
fn render(documents: &[Vec<u8>]) -> Vec<[u8; 8]> {
 let mut process = std::process::Command::new("repo")
  .args(["tools", "exec", "--light", "uv", "--", "run", "--quiet", "--isolated", "--no-project", "--no-build", "--with", "pypdfium2==5.15.2", "python", "-I", "-c", include_str!("🔮️oracle/🐍️.py")])
  .stdin(std::process::Stdio::piped())
  .stdout(std::process::Stdio::piped())
  .stderr(std::process::Stdio::piped())
  .spawn()
  .expect("repo-provisioned PDFium oracle");
 let mut input = process.stdin.take().unwrap();
 for bytes in documents {
  input.write_all(&u32::try_from(bytes.len()).unwrap().to_be_bytes()).unwrap();
  input.write_all(bytes).unwrap();
 }
 drop(input);
 let output = process.wait_with_output().unwrap();
 assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
 assert_eq!(output.stdout.len(), documents.len() * 8);
 output.stdout.chunks_exact(8).map(|pixel| pixel.try_into().unwrap()).collect()
}
#[test]
fn pdf_write_neutral_styles_match_pdfium_and_exact_physical_close(){
 let rows:Vec<Value>=serde_json::from_str(fixture_text!("../../🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🧵️write/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let docs:Vec<Value>=serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🎬️scene/📋️prepare/🔣️.json")).unwrap();let mut styles:Vec<Value>=serde_json::from_str(fixture_text!("../../../../✏️editor/🎮️commands/📤️export-document/🧪️tests/🧫️fixtures/🎨️styles/🔣️.json")).unwrap();styles.extend(serde_json::from_str::<Vec<Value>>(fixture_text!("../../🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🧵️write/🧪️tests/🧫️fixtures/🎨️paint/🔣️.json")).unwrap());
 let mut documents = Vec::new();
 for row in &rows {
  let source = docs.iter().find(|document| document["name"] == row["source"]).unwrap();
  let mut raw = source["document"].clone();
  if row["styles"] != "" {
   raw["layers"] = styles.iter().find(|style| style["name"] == row["styles"]).unwrap()["layers"].clone();
  }
  let document: crate::DrawingSnapshot = serde_json::from_value(raw).unwrap();
  let before = document.clone();
  let row_start = documents.len();
  for grant in [1, 7, 4096] {
   let limits = &source["limits"];
   let (vector_close, plan) = original_vector::prepare(&document, DocumentSceneLimits {
    max_nodes: 16,
    max_depth: limits["maxDepth"].as_u64().unwrap() as usize,
    max_segments: limits["maxSegments"].as_u64().unwrap() as usize,
    max_references: limits["maxReferences"].as_u64().unwrap() as usize,
    max_source_bytes: limits["maxSourceBytes"].as_u64().unwrap() as usize,
   }, algorithms());
   let mut job = PdfWriteJob::new(plan, self::limits());
   let bytes = if grant == 1 && row.get("originalFunding").is_some() {
    finish_original(&mut job, &row["originalFunding"])
   } else { finish(&mut job, grant) };
   if grant != 1 { assert_eq!(&bytes, &documents[row_start]); }
   documents.push(bytes);
   let (retired, pages) = job.into_retirement();
   close(retired);
   close(pages.unwrap());
   vector_close.close();
   assert_eq!(document, before);
  }
 }
 let samples = render(&documents);
 for (row, sample) in rows.iter().zip(samples.chunks_exact(3)) {
  let tolerance = row["channelTolerance"].as_u64().unwrap() as u8;
  for pixels in sample {
   for (at, expected) in row["expected"].as_array().unwrap().iter().enumerate() {
    assert!(pixels[at].abs_diff(expected.as_u64().unwrap() as u8) <= tolerance, "{} {at}: {} != {expected}", row["styles"], pixels[at]);
   }
  }
 }
}
#[test]
fn pdf_write_shared_image_preserves_channels_alpha_and_original_lease(){let image=std::sync::Arc::new(semio_framework_pixels::RasterImage{width:2,height:1,pixels:vec![255,0,0,255,0,0,255,128]});let pointer=image.pixels.as_ptr();let plan=DocumentScenePlan{assets:vec![RasterSceneAsset{id:"image".into(),image:image.clone()}],nodes:vec![DocumentSceneNode{source_path:vec![0],locked_ancestors:0,id:"image-layer".into(),groups:Vec::new(),transform:[1.0,0.0,0.0,1.0,0.0,0.0],opacity:1.0,blend_mode:"normal".into(),visible:true,content:DocumentSceneContent::Image{asset:"image".into(),width:2.0,height:1.0}}]};let mut job=PdfWriteJob::new(plan,limits());assert_eq!(render(&[finish(&mut job,1)])[0],[255,0,0,255,127,127,255,255]);assert_eq!(image.pixels.as_ptr(),pointer);let(retired,pages)=job.into_retirement();close(retired);close(pages.unwrap());assert_eq!(std::sync::Arc::strong_count(&image),1);close(semio_framework_pixels::retirement::RasterLease(image));}
#[test]
fn pdf_write_interruptions_and_refusals_keep_original_candidate_until_funded_close(){for stop in [0,1,7,40,300]{let mut job=PdfWriteJob::new(DocumentScenePlan::default(),limits());if stop>0{job.advance(stop).unwrap();}job.cancel();assert!(job.result().is_err());let(retired,pages)=job.into_retirement();assert!(pages.is_none());close(retired);}for patch in [0,1,2]{let mut l=limits();if patch==0{l.max_output_bytes=64;}else if patch==1{l.max_work=1;}else{l.view_box[2]=0.0;}let mut job=PdfWriteJob::new(DocumentScenePlan::default(),l);while job.advance(1).is_ok(){}assert!(job.result().is_err());let(retired,pages)=job.into_retirement();assert!(pages.is_none());close(retired);}}

#[test]
fn pdf_write_shared_public_phase_cancellation_retains_original_candidate(){let law:Value=serde_json::from_str(fixture_text!("../../🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🧵️write/🧪️tests/🧫️fixtures/⏹️cancellation/🔣️.json")).unwrap();let docs:Vec<Value>=serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🎬️scene/📋️prepare/🔣️.json")).unwrap();let styles:Vec<Value>=serde_json::from_str(fixture_text!("../../🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🧵️write/🧪️tests/🧫️fixtures/🎨️paint/🔣️.json")).unwrap();let source=docs.iter().find(|row|row["name"]=="rectangle").unwrap();let mut raw=source["document"].clone();raw["layers"]=styles.iter().find(|row|row["name"]==law["styles"]).unwrap()["layers"].clone();let document:crate::DrawingSnapshot=serde_json::from_value(raw).unwrap();for phase in law["phases"].as_array().unwrap(){let l=&source["limits"];let(vector_close,plan)=original_vector::prepare(&document,DocumentSceneLimits{max_nodes:16,max_depth:l["maxDepth"].as_u64().unwrap()as usize,max_segments:l["maxSegments"].as_u64().unwrap()as usize,max_references:l["maxReferences"].as_u64().unwrap()as usize,max_source_bytes:l["maxSourceBytes"].as_u64().unwrap()as usize},algorithms());let mut job=PdfWriteJob::new(plan,limits());let mut reached=false;for _ in 0..100000{let p=job.advance(1).unwrap();if p.phase==phase.as_str().unwrap(){reached=true;break;}if p.done{break;}}assert!(reached,"{phase}");job.cancel();assert!(job.result().is_err());let(retired,pages)=job.into_retirement();assert!(pages.is_none());close(retired);vector_close.close();}}
