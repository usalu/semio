//! 🧪️ Neutral bitmap trace fixtures, bounded publication, topology and cancellation.
use super::*;
use std::collections::BTreeSet;
use serde_json::{Value,json};
fn rows()->Vec<Value> {serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn input<M>(row:&Value,mask:M)->BitmapTraceInput<M> {
 let v=&row["input"];BitmapTraceInput {width:v["width"].as_u64().unwrap() as u32,height:v["height"].as_u64().unwrap() as u32,mask,threshold:v["threshold"].as_f64().unwrap(),simplify_epsilon:v["simplifyEpsilon"].as_f64().unwrap(),max_pixels:v["maxPixels"].as_u64().unwrap() as usize,max_edges:v["maxEdges"].as_u64().unwrap() as usize,max_segments:v["maxSegments"].as_u64().unwrap() as usize,max_work:v["maxWork"].as_u64().unwrap()}
}
fn paths(segments:&[PathSegment])->Vec<Vec<Vec2>> {
 let mut result=Vec::<Vec<Vec2>>::new();for segment in segments {match segment {PathSegment::Move {to}=>result.push(vec![*to]),PathSegment::Line {to}=>result.last_mut().unwrap().push(*to),PathSegment::Close=>{},_=>panic!("Unexpected traced curve")}}result
}
fn foreground<M>(segments:&[PathSegment],input:BitmapTraceInput<M>)->Vec<u8> {
 let rings=paths(segments);(0..input.width as usize*input.height as usize).map(|at|{
  let x=(at%input.width as usize) as f64+0.5;let y=(at/input.width as usize) as f64+0.5;let p=[x,y];let mut winding=0;
  for ring in &rings {for index in 0..ring.len() {let a=ring[index];let b=ring[(index+1)%ring.len()];let side=cross(a,b,p);if a[1]<=y&&b[1]>y&&side>0.0 {winding+=1;}else if a[1]>y&&b[1]<=y&&side<0.0 {winding-=1;}}}u8::from(winding!=0)
 }).collect()
}
fn complete<M:AsRef<[u8]>>(input:BitmapTraceInput<M>,grant:usize)->(Vec<PathSegment>,Vec<BitmapTracePhase>) {
 let max_work=input.max_work;let mut job=BitmapTraceJob::new(input).unwrap();assert_eq!(job.result().unwrap_err(),BitmapTraceError::Incomplete);let mut work=0;let mut phases=Vec::new();
 loop {let p=job.advance(grant).unwrap();assert!(p.work-work<=grant as u64);work=p.work;assert!(work<=max_work);phases.push(p.phase);if p.done {return (job.into_result().unwrap(),phases);}assert_eq!(job.result().unwrap_err(),BitmapTraceError::Incomplete);}
}
#[test]
fn trace_neutral_vectors_and_work_grants() {
 for row in rows() {let mask=row["input"]["mask"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect::<Vec<_>>();let input=input(&row,&mask);let expected=row["expected"]["foreground"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect::<Vec<_>>();let mut previous=None;
  for grant in [1,7,4096] {let (segments,_)=complete(input,grant);assert_eq!(foreground(&segments,input),expected,"{}",row["name"]);assert_eq!(paths(&segments).len(),row["expected"]["contours"].as_u64().unwrap() as usize);
   if let Some(expected)=row["expected"]["segments"].as_array() {let actual=segments.iter().map(|s|match s {PathSegment::Move{to}=>json!({"kind":"move","to":to}),PathSegment::Line{to}=>json!({"kind":"line","to":to}),PathSegment::Close=>json!({"kind":"close"}),_=>panic!("Unexpected trace segment")}).collect::<Vec<_>>();for (a,b) in actual.iter().zip(expected) {assert_eq!(a["kind"],b["kind"]);if a["kind"]!="close" {for axis in 0..2 {assert_eq!(a["to"][axis].as_f64(),b["to"][axis].as_f64());}}}assert_eq!(actual.len(),expected.len());}
   if let Some(previous)=&previous {assert_eq!(&segments,previous);}previous=Some(segments);
  }
  println!("[DEBUG] trace fixture {} completed with identical paths at grants 1, 7 and 4096",row["name"]);
 }
}
#[test]
fn trace_invalid_contracts_and_sticky_resource_refusals() {
 let mask=[255u8];let valid:BitmapTraceInput<&[u8]>=BitmapTraceInput {width:1,height:1,mask:&mask,threshold:0.5,simplify_epsilon:0.0,max_pixels:1,max_edges:4,max_segments:5,max_work:10000};
 for input in [BitmapTraceInput {width:0,..valid},BitmapTraceInput {height:8193,..valid},BitmapTraceInput {mask:&[],..valid},BitmapTraceInput {mask:&[255,255],..valid},BitmapTraceInput {threshold:-0.1,..valid},BitmapTraceInput {threshold:1.1,..valid},BitmapTraceInput {threshold:f64::NAN,..valid},BitmapTraceInput {simplify_epsilon:-1.0,..valid},BitmapTraceInput {simplify_epsilon:f64::INFINITY,..valid},BitmapTraceInput {max_pixels:0,..valid},BitmapTraceInput {max_edges:0,..valid},BitmapTraceInput {max_segments:0,..valid},BitmapTraceInput {max_work:0,..valid},BitmapTraceInput {max_pixels:16777217,..valid},BitmapTraceInput {max_edges:65537,..valid},BitmapTraceInput {max_segments:65537,..valid},BitmapTraceInput {max_work:1000000001,..valid}] {assert!(BitmapTraceJob::new(input).is_err());}
 for input in [BitmapTraceInput {max_edges:3,..valid},BitmapTraceInput {max_segments:4,..valid},BitmapTraceInput {max_work:1,..valid}] {let mut job=BitmapTraceJob::new(input).unwrap();let error=job.advance(4096).unwrap_err();assert_eq!(job.result().unwrap_err(),error);assert_eq!(job.advance(1).unwrap_err(),error);}
 assert!(BitmapTraceJob::new(BitmapTraceInput {width:2,mask:&[255,255][..],..valid}).is_err());assert!(BitmapTraceJob::new(valid).unwrap().advance(0).is_err());
}
#[test]
fn trace_cancels_every_phase_without_exposing_private_geometry() {
 let row=&rows()[13];let mask=row["input"]["mask"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect::<Vec<_>>();let input=input(row,&mask);let mut probe=BitmapTraceJob::new(input).unwrap();let mut seen=BTreeSet::new();let mut steps=0;
 loop {let phase=if steps==0 {BitmapTracePhase::Scan} else {probe.advance(1).unwrap().phase};if seen.insert(phase.as_str()) {let mut job=BitmapTraceJob::new(input).unwrap();for _ in 0..steps {job.advance(1).unwrap();}job.cancel();assert_eq!(job.result().unwrap_err(),BitmapTraceError::Cancelled);assert_eq!(job.advance(1).unwrap_err(),BitmapTraceError::Cancelled);}if phase==BitmapTracePhase::Complete {break;}steps+=1;}
 assert_eq!(seen,BTreeSet::from(["scan","contours","compact","simplify","topology","coverage","emit","complete"]));assert_eq!(mask,row["input"]["mask"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect::<Vec<_>>());
}
#[test]
fn trace_returns_empty_geometry_and_exact_resource_limits() {
 let valid:BitmapTraceInput<&[u8]>=BitmapTraceInput {width:1,height:1,mask:&[255],threshold:0.5,simplify_epsilon:0.0,max_pixels:1,max_edges:4,max_segments:5,max_work:10000};let (output,_)=complete(valid,1);assert_eq!(output.len(),5);let mut job=BitmapTraceJob::new(valid).unwrap();let p=job.advance(4096).unwrap();let (exact,_)=complete(BitmapTraceInput {max_work:p.work,..valid},7);assert_eq!(output,exact);
 let mut cancelled=BitmapTraceJob::new(valid).unwrap();cancelled.advance(4096).unwrap();let published=cancelled.result().unwrap().to_vec();cancelled.cancel();assert_eq!(published,output);assert!(complete(BitmapTraceInput {mask:&[0][..],..valid},1).0.is_empty());
 println!("[DEBUG] trace exact limits published {} segments in {} granted operations",output.len(),p.work);
}
#[test]
fn trace_progress_matches_portable_field_taxonomy() {
 let row=&rows()[13];let mask=row["input"]["mask"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect::<Vec<_>>();let mut job=BitmapTraceJob::new(input(row,&mask)).unwrap();
 loop {let p=job.advance(1).unwrap();let value=json!({"phase":p.phase.as_str(),"scanned":p.scanned,"pixels":p.pixels,"edges":p.edges,"contours":p.contours,"segments":p.segments,"simplified":p.simplified,"work":p.work,"done":p.done});assert_eq!(value.as_object().unwrap().len(),9);assert!(p.edges<=65536);assert!(p.segments<=65536);if p.done {break;}}
}
#[test]
fn trace_all_three_by_three_masks_and_blank_linear_work() {
 let mut simplified=0;for bits in 0..512u16 {let mask=(0..9).map(|at|if bits&(1<<at)==0 {0u8} else {255u8}).collect::<Vec<_>>();let expected=mask.iter().map(|value|u8::from(*value!=0)).collect::<Vec<_>>();
  for epsilon in [0.0,0.55,8192.0] {let input=BitmapTraceInput {width:3,height:3,mask:&mask,threshold:0.5,simplify_epsilon:epsilon,max_pixels:9,max_edges:36,max_segments:45,max_work:10000};let mut job=BitmapTraceJob::new(input).unwrap();let p=job.advance(10000).unwrap();assert!(p.done);simplified+=usize::from(p.simplified);assert_eq!(foreground(job.result().unwrap(),input),expected);}
 }assert!(simplified>0);let mask=vec![0u8;8192];let mut job=BitmapTraceJob::new(BitmapTraceInput {width:8192,height:1,mask:&mask,threshold:0.5,simplify_epsilon:0.0,max_pixels:8192,max_edges:1,max_segments:1,max_work:8196}).unwrap();let p=job.advance(8196).unwrap();assert!(p.done);assert_eq!(p.work,8196);assert!(job.result().unwrap().is_empty());println!("[DEBUG] trace retained 512 masks at three tolerances and completed a wide blank in linear work");
}
#[test]
fn trace_owned_masks_match_neutral_vectors_and_release_at_every_cancel_phase(){
 for row in rows(){let mask=row["input"]["mask"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect::<Vec<_>>();let expected=complete(input(&row,&mask),4096).0;for grant in [1,7,4096]{let mut job=BitmapTraceJob::new(input(&row,mask.clone())).unwrap();assert_eq!(job.result().unwrap_err(),BitmapTraceError::Incomplete);while !job.advance(grant).unwrap().done{}assert_eq!(job.into_result().unwrap(),expected);}}
 struct Tracked{bytes:Vec<u8>,drops:std::rc::Rc<std::cell::Cell<usize>>}
 impl AsRef<[u8]> for Tracked{fn as_ref(&self)->&[u8]{&self.bytes}}
 impl Drop for Tracked{fn drop(&mut self){self.drops.set(self.drops.get()+1);}}
 let row=&rows()[13];let mask=row["input"]["mask"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect::<Vec<_>>();let mut probe=BitmapTraceJob::new(input(row,&mask)).unwrap();let mut seen=BTreeSet::new();
 loop{let p=probe.advance(1).unwrap();if seen.insert(p.phase.as_str()){let drops=std::rc::Rc::new(std::cell::Cell::new(0));let mut job=BitmapTraceJob::new(input(row,Tracked{bytes:mask.clone(),drops:drops.clone()})).unwrap();job.advance(p.work as usize).unwrap();assert_eq!(drops.get(),0);job.cancel();assert_eq!(drops.get(),0);assert_eq!(job.result().unwrap_err(),BitmapTraceError::Cancelled);assert_eq!(job.advance(1).unwrap_err(),BitmapTraceError::Cancelled);drop(job);assert_eq!(drops.get(),1);}if p.done{break;}}
 println!("[DEBUG] Native moved trace masks match neutral fixtures and release storage at every cancellation phase");
}

#[test]
fn trace_retirement_transfers_masks_and_drains_every_actual_phase_under_exact_grants(){
 let close_rows:Vec<Value>=serde_json::from_str(include_str!("../🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();
 for row in close_rows{for grant in [1,7,4096]{let source=rows().into_iter().find(|v|v["name"]==row["source"]).unwrap();let mask=source["input"]["mask"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect::<Vec<_>>();let ptr=mask.as_ptr();let mut contract=input(&source,mask);if let Some(cap)=row["maxEdges"].as_u64(){contract.max_edges=cap as usize;}let mut job=BitmapTraceJob::new(contract).unwrap();
  let phase=row["phase"].as_str().unwrap();if phase=="cancelled"{job.cancel();}else if phase=="failure"{assert!(job.advance(4096).is_err());assert!(job.result().is_err());}else if phase!="fresh"{let mut found=false;for _ in 0..job.input.max_work{let p=job.advance(1).unwrap();if p.phase.as_str()==phase{found=true;break;}if p.done{break;}}assert!(found,"{}",row);}
  let(mut close,returned)=job.into_retirement();assert_eq!(returned.as_ref().unwrap().as_ptr(),ptr);assert!(close.advance(0).is_err());let mut work=0;loop{let p=close.advance(grant).unwrap();assert!(p.work>=work&&p.work-work<=grant as u64);work=p.work;if p.done{break;}}
  if grant==1{println!("[DEBUG] Native trace retirement {} work={work}",phase);}assert!(work>0);assert!(close.terminal_is_empty());let p=close.advance(1).unwrap();assert_eq!((p.work,p.done,p.phase),(work,true,"complete"));assert!(close.owner.original().is_none());if let Some(mask)=returned{assert_eq!(mask,source["input"]["mask"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect::<Vec<_>>());}
 }println!("[DEBUG] Native active trace ownership retired its exact JSON inventory: {}",row["phase"]);}
}
