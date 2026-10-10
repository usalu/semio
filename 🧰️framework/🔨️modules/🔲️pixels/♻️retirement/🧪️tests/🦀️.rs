//! 🧪️ Neutral backing witnesses exercise refused, borrowed and final mask releases.
use super::*;
use std::{alloc::{GlobalAlloc,Layout,System},cell::Cell};
use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::{RetainedCloneStep,RetainedCloneGrant}};
thread_local!{static TRACK:Cell<Option<(usize,usize)>>=const{Cell::new(None)};}
struct PhysicalAllocator;
#[global_allocator]
static ALLOCATOR:PhysicalAllocator=PhysicalAllocator;
unsafe impl GlobalAlloc for PhysicalAllocator {
 unsafe fn alloc(&self,layout:Layout)->*mut u8 {let pointer=unsafe{System.alloc(layout)};if !pointer.is_null(){let _=TRACK.try_with(|track|if let Some((allocated,released))=track.get(){track.set(Some((allocated+layout.size(),released)));});}pointer}
 unsafe fn dealloc(&self,pointer:*mut u8,layout:Layout){let _=TRACK.try_with(|track|if let Some((allocated,released))=track.get(){track.set(Some((allocated,released+layout.size())));});unsafe{System.dealloc(pointer,layout);}}
}
pub(crate) fn observed<T>(step:impl FnOnce()->T)->(T,(usize,usize)){TRACK.with(|track|{assert!(track.get().is_none());track.set(Some((0,0)));});let output=step();let bytes=TRACK.with(|track|track.replace(None).unwrap());(output,bytes)}
pub(crate) fn drain<T:RetireOwned>(value:T)->(usize,usize){
 let mut owner=ControlledRetirement::new(value).unwrap_or_else(|(error,_)|panic!("unsupported test owner: {error}"));let mut allocated=0;let mut released=0;
 for _ in 0..2_000_000 {if owner.terminal_is_empty(){break;}let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(if copy>0{copy}else{owner.next_release_byte_demand().unwrap()}).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
  let (zero,physical)=observed(||owner.step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(physical,(0,0));assert!(matches!(zero,RetainedCloneStep::Progress(p)if p==Default::default()));
  let (step,physical)=observed(||owner.step(grant).unwrap());let (RetainedCloneStep::Progress(receipt)|RetainedCloneStep::Complete(receipt))=step;assert!(receipt.fits(grant));assert!(physical.0<=grant.maximum_capacity_bytes,"physical allocation {} exceeds capacity {}",physical.0,grant.maximum_capacity_bytes);assert!(physical.1<=grant.maximum_release_bytes,"physical release {} exceeds funded release {}",physical.1,grant.maximum_release_bytes);assert_eq!(physical.0,receipt.retained_capacity_bytes);assert_eq!(physical.1,receipt.released_bytes);allocated+=physical.0;released+=physical.1;
 }
 assert!(owner.terminal_is_empty());let (_,physical)=observed(||drop(owner));assert_eq!(physical,(0,0));(allocated,released)
}
#[test]
fn retained_physical_raster_children_obey_allocator_grants(){
 use crate::{coverage::{CoverageInput,CoverageJob,CoverageRule},affine_sampling::{AffineImageInput,AffineImageJob,AffineSampling},compositing::{CompositeJob,CompositeLayer,CompositeContent,CompositeBlend}};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for stop in fixture["interruptions"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){
  let mut coverage=CoverageJob::new(CoverageInput{width:8,height:8,transform:[1.,0.,0.,1.,0.,0.],rule:CoverageRule::NonZero,contours:vec![vec![[0.,0.],[8.,0.],[8.,8.],[0.,8.]]]}).unwrap();if stop>0{coverage.advance(stop).unwrap();}coverage.cancel();let retired=coverage.into_retirement().0;let(a,r)=drain(retired);assert!(r>a);
  let source=Arc::new(RasterImage{width:2,height:2,pixels:vec![255;16]});let mut sample=AffineImageJob::new(AffineImageInput{source:source.clone(),width:8,height:8,origin:[0.,0.],transform:[4.,0.,0.,4.,0.,0.],sampling:AffineSampling::Area}).unwrap();if stop>0{sample.advance(stop).unwrap();}sample.cancel();let(a,r)=drain(sample.into_retirement().0);assert!(r>a);assert_eq!(source.pixels,vec![255;16]);assert_eq!(Arc::strong_count(&source),1);
  let mut compositor=CompositeJob::new_owned(8,8,[0.,0.],vec![("image".into(),RasterLease(source.clone()))],vec![CompositeLayer{opacity:1.,blend:CompositeBlend::Normal,visible:true,transform:[4.,0.,0.,4.,0.,0.],mask:None,content:CompositeContent::Pixels("image".into())}]).unwrap();if stop>0{compositor.advance(stop).unwrap();}compositor.cancel();let(a,r)=drain(compositor.into_retirement().0);assert!(r>a);assert_eq!(Arc::strong_count(&source),1);
 }
 eprintln!("[DEBUG] Physical raster leaves: 15 interrupted owners; every allocator allocation/release matched its exact receipt; terminal destruction released zero backing");
}
#[test]
fn retained_physical_geometry_children_obey_allocator_grants(){
 use semio_framework_2d::{PathSegment,flatten::{PathFlattenInput,PathFlattenJob},stroke::{StrokeOutlineInput,StrokeOutlineJob,StrokeContour,StrokeGeometryStyle,StrokeGeometryCap,StrokeGeometryJoin}};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for stop in fixture["interruptions"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){
  let mut flatten=PathFlattenJob::new(PathFlattenInput{segments:vec![PathSegment::Move{to:[0.,0.]},PathSegment::Cubic{ctrl1:[0.,8.],ctrl2:[8.,8.],to:[8.,0.]},PathSegment::Close],transform:[1.,0.,0.,1.,0.,0.],tolerance:0.1}).unwrap();if stop>0{flatten.advance(stop).unwrap();}flatten.cancel();let(a,r)=drain(flatten.into_retirement().0);assert!(r>a);
  let mut stroke=StrokeOutlineJob::new(StrokeOutlineInput{contours:vec![StrokeContour{points:vec![[0.,0.],[8.,0.],[8.,8.]],closed:false}],transform:[1.,0.,0.,1.,0.,0.],tolerance:0.1,style:StrokeGeometryStyle{width:2.,cap:StrokeGeometryCap::Round,join:StrokeGeometryJoin::Round,miter_limit:4.,dash:vec![1.,1.],dash_offset:0.}}).unwrap();if stop>0{stroke.advance(stop).unwrap();}stroke.cancel();let(a,r)=drain(stroke.into_retirement().0);assert!(r>a);
 }
 eprintln!("[DEBUG] Physical geometry leaves: 10 interrupted owners; exact allocator receipts and empty terminal destructors");
}
#[test]
fn retained_physical_numeric_scratch_matches_reference_and_allocator(){
 use semio_framework_value::numeric_scratch::NumericIndex;use std::collections::BTreeMap;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🌱️value/🗂️ordered/🔢️numeric/🧮️scratch/🧪️tests/📜️fixtures/🔣️.json")).unwrap();let mut index=NumericIndex::<i64,i64>::new();let mut oracle=BTreeMap::new();let mut output=Vec::new();
 for row in fixture["operations"].as_array().unwrap(){let key=row["key"].as_i64().unwrap_or(0);let result=match row["kind"].as_str().unwrap(){"set"=>{let value=row["value"].as_i64().unwrap();let actual=index.insert(key,value);assert_eq!(actual,oracle.insert(key,value));serde_json::json!(actual)},"remove"=>{let actual=index.remove(&key);assert_eq!(actual,oracle.remove(&key));serde_json::json!(actual)},"get"=>{let actual=index.get(&key).copied();assert_eq!(actual,oracle.get(&key).copied());serde_json::json!(actual)},"first"=>{let actual=index.pop_first();assert_eq!(actual,oracle.pop_first());serde_json::json!(actual.map(|(key,value)|[key,value]))},"reset"=>{index.reset();oracle.clear();serde_json::Value::Null},_=>unreachable!()};assert_eq!(index.len(),oracle.len());output.push(result);}
 assert_eq!(serde_json::json!(output),fixture["expected"]);for key in 0..4096{index.insert(key,key);}let stored=index.stored_keys();index.reset();for key in (0..4096).rev(){index.insert(key,key);}assert_eq!(index.stored_keys(),stored);for key in 0..4096{assert_eq!(index.pop_first(),Some((key,key)));}assert!(index.is_empty());let(a,r)=drain(index);assert!(r>a);
 eprintln!("[DEBUG] Numeric scratch AVL shared fixture/BTreeMap oracle and4096 reused slots; exact paged allocator receipts");
}
#[test]
fn retained_physical_boolean_and_trace_children_obey_allocator_grants(){
 use semio_framework_2d::{PathSegment,booleans::{BooleanJob,BooleanInput,BooleanOperand,BooleanOperation,BooleanFillRule},trace::{BitmapTraceJob,BitmapTraceInput}};
 let boolean_rows:serde_json::Value=serde_json::from_str(include_str!("../../../◻️2d/🔀️booleans/🧫️fixtures/🔣️.json")).unwrap();let trace_rows:serde_json::Value=serde_json::from_str(include_str!("../../../◻️2d/🔍️trace/🧫️fixtures/🔣️.json")).unwrap();
 for row in boolean_rows.as_array().unwrap(){let input=&row["input"];let source=BooleanInput{operation:BooleanOperation::parse(input["operation"].as_str().unwrap()).unwrap(),operands:input["operands"].as_array().unwrap().iter().map(|operand|BooleanOperand{fill_rule:if operand["fillRule"]=="evenodd"{BooleanFillRule::Evenodd}else{BooleanFillRule::Nonzero},contours:operand["contours"].as_array().unwrap().iter().map(|ring|ring.as_array().unwrap().iter().map(|p|[p[0].as_f64().unwrap(),p[1].as_f64().unwrap()]).collect()).collect()}).collect(),epsilon:input["epsilon"].as_f64().unwrap(),max_edges:input["maxEdges"].as_u64().unwrap()as usize,max_parameters:input["maxParameters"].as_u64().unwrap()as usize,max_atomic_edges:input["maxAtomicEdges"].as_u64().unwrap()as usize,max_segments:input["maxSegments"].as_u64().unwrap()as usize,max_work:input["maxWork"].as_u64().unwrap()};
  for stop in [0,1,10,100,4096]{let mut job=BooleanJob::new(source.clone()).unwrap();if stop>0{job.advance(stop).unwrap();}job.cancel();let(retired,operands,output)=job.into_retirement();assert!(output.is_none());drain(retired);drain(operands);}
  let mut job=BooleanJob::new(source).unwrap();while !job.advance(4096).unwrap().done{}let(retired,operands,output)=job.into_retirement();let output=output.unwrap();let mut first=[0.;2];let mut previous=[0.;2];let mut area=0.;let mut contours=0;for segment in &output{match segment{PathSegment::Move{to}=>{first=*to;previous=*to;contours+=1;},PathSegment::Line{to}=>{area+=previous[0]*to[1]-to[0]*previous[1];previous=*to;},PathSegment::Close=>area+=previous[0]*first[1]-first[0]*previous[1],_=>unreachable!()}}assert_eq!(contours,row["expected"]["contours"].as_u64().unwrap());assert!((area/2.-row["expected"]["area"].as_f64().unwrap()).abs()<1e-5);drain(retired);drain(operands);drain(output);
 }
 for row in trace_rows.as_array().unwrap(){let input=&row["input"];let source=BitmapTraceInput{width:input["width"].as_u64().unwrap()as u32,height:input["height"].as_u64().unwrap()as u32,mask:input["mask"].as_array().unwrap().iter().map(|p|p.as_u64().unwrap()as u8).collect::<Vec<_>>(),threshold:input["threshold"].as_f64().unwrap(),simplify_epsilon:input["simplifyEpsilon"].as_f64().unwrap(),max_pixels:input["maxPixels"].as_u64().unwrap()as usize,max_edges:input["maxEdges"].as_u64().unwrap()as usize,max_segments:input["maxSegments"].as_u64().unwrap()as usize,max_work:input["maxWork"].as_u64().unwrap()};for stop in [0,1,10,100,4096]{let mut job=BitmapTraceJob::new(source.clone()).unwrap();if stop>0{job.advance(stop).unwrap();}job.cancel();let(retired,mask)=job.into_retirement();assert_eq!(mask.as_ref(),Some(&source.mask));drain(retired);drain(mask);}let mut job=BitmapTraceJob::new(source).unwrap();while !job.advance(4096).unwrap().done{}assert_eq!(job.result().unwrap().iter().filter(|s|matches!(s,PathSegment::Move{..})).count(),row["expected"]["contours"].as_u64().unwrap()as usize);let(retired,mask)=job.into_retirement();drain(retired);drain(mask);}
 eprintln!("[DEBUG] Physical Boolean/Trace: shared neutral outputs plus five interrupted native owners per fixture; allocator receipts and terminal destruction matched");
}
#[test]
fn retained_physical_png_encoder_obeys_allocator_grants(){
 use crate::png_encoding::PngEncodeJob;
 for stop in [0,1,2]{let mut job=PngEncodeJob::new(RasterImage{width:8,height:8,pixels:vec![127;256]}).unwrap();for _ in 0..stop{job.advance().unwrap();}job.cancel();assert!(job.result().is_err());let(a,r)=drain(job);assert!(r>a);}
 let expected=vec![127;256];let mut job=PngEncodeJob::new(RasterImage{width:8,height:8,pixels:expected.clone()}).unwrap();while !job.advance().unwrap().done{}let output=job.take_result().unwrap();let(a,r)=drain(job);assert!(r>a);
 let mut decoder=png::Decoder::new(std::io::Cursor::new(&output.data)).read_info().unwrap();let mut pixels=vec![0;decoder.output_buffer_size()];let info=decoder.next_frame(&mut pixels).unwrap();assert_eq!((info.width,info.height),(8,8));assert_eq!(pixels[..info.buffer_size()],expected);
 drain(output);
 eprintln!("[DEBUG] Physical PNG encoder: three cancellations and completed publication; exact allocator receipts; independent PNG decoder preserved pixels");
}
#[test]
fn retained_physical_mask_leases_require_actual_backing_grants(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap().iter().filter(|row|row["input"]["kind"]=="mask"){
  let bytes:Vec<u8>=row["input"]["bytes"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect();let lease=MaskLease::from(bytes.clone());let source=lease.0;
  let alias=(row["input"]["aliases"].as_u64().unwrap()>0).then(||source.clone());let weak=(row["input"]["weakLeases"].as_u64().unwrap()>0).then(||Arc::downgrade(&source));
  let demand=semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<MaskBytes>();let mut cursor=SharedControlledRetirement::lease(source);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_release_bytes:demand,maximum_capacity_bytes:0,maximum_depth:1};
  let(zero,physical)=observed(||cursor.step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert!(matches!(zero,RetainedCloneStep::Progress(p)if p==Default::default()));assert_eq!(physical,(0,0));
  if let Some(weak)=&weak{let(step,physical)=observed(||cursor.step(grant).unwrap());assert!(matches!(step,RetainedCloneStep::Progress(p)if p==Default::default()));assert_eq!(physical,(0,0));assert_eq!(weak.upgrade().unwrap().as_ref().0,bytes);}drop(weak);
  let(step,physical)=observed(||cursor.step(RetainedCloneGrant{maximum_release_bytes:demand-1,..grant}).unwrap());assert!(matches!(step,RetainedCloneStep::Progress(p)if p==Default::default()));assert_eq!(physical,(0,0));
  let(step,physical)=observed(||cursor.step(grant).unwrap());let RetainedCloneStep::Progress(receipt)=step else{panic!("funded lease did not close")};assert_eq!(physical,(0,receipt.released_bytes));assert_eq!(receipt.released_bytes,if alias.is_some(){0}else{demand});drain(cursor);if let Some(alias)=alias{assert_eq!(alias.0,bytes);}
 }
}
#[test]
fn retained_physical_curved_boolean_children_obey_allocator_grants(){
 use semio_framework_2d::{PathSegment,booleans::{BooleanOperation,BooleanFillRule,paths::{PathBooleanJob,PathBooleanInput,PathBooleanOperand}}};
 fn point(value:&serde_json::Value)->[f64;2]{[value[0].as_f64().unwrap(),value[1].as_f64().unwrap()]}
 fn segment(value:&serde_json::Value)->PathSegment{match value["kind"].as_str().unwrap(){"move"=>PathSegment::Move{to:point(&value["to"])},"line"=>PathSegment::Line{to:point(&value["to"])},"quad"=>PathSegment::Quad{ctrl:point(&value["ctrl"]),to:point(&value["to"])},"cubic"=>PathSegment::Cubic{ctrl1:point(&value["ctrl1"]),ctrl2:point(&value["ctrl2"]),to:point(&value["to"])},"arc"=>PathSegment::Arc{rx:value["rx"].as_f64().unwrap(),ry:value["ry"].as_f64().unwrap(),rotation:value["rotation"].as_f64().unwrap(),large_arc:value["largeArc"].as_bool().unwrap(),sweep:value["sweep"].as_bool().unwrap(),to:point(&value["to"])},"close"=>PathSegment::Close,_=>unreachable!()}}
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../◻️2d/🔀️booleans/🛤️paths/🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){let source=PathBooleanInput{operation:BooleanOperation::parse(row["operation"].as_str().unwrap()).unwrap(),operands:row["operands"].as_array().unwrap().iter().map(|v|PathBooleanOperand{segments:v["segments"].as_array().unwrap().iter().map(segment).collect(),transform:std::array::from_fn(|i|v["transform"][i].as_f64().unwrap()),tolerance:v["tolerance"].as_f64().unwrap(),fill_rule:if v["fillRule"]=="evenodd"{BooleanFillRule::Evenodd}else{BooleanFillRule::Nonzero}}).collect(),epsilon:1e-8,max_edges:65536,max_parameters:262144,max_atomic_edges:65536,max_segments:65536,max_work:10000000};
  for stop in [0,1,10,100,4096]{let mut job=PathBooleanJob::new(source.clone()).unwrap();if stop>0{job.advance(stop).unwrap();}job.cancel();let(retired,output)=job.into_retirement();assert!(output.is_none());drain(retired);}
  let mut job=PathBooleanJob::new(source).unwrap();while !job.advance(4096).unwrap().done{}let(retired,output)=job.into_retirement();let output=output.unwrap();assert_eq!(output.iter().filter(|s|matches!(s,PathSegment::Move{..})).count(),row["contours"].as_u64().unwrap()as usize);drain(retired);drain(output);
 }
 eprintln!("[DEBUG] Physical curved Boolean: shared transforms/curves, five interruption stops and sealed output; real queue and child backing receipts matched");
}
