//! 🧪️ Neutral bitmap trace fixtures, bounded publication, topology and cancellation.
use super::*;
use std::collections::BTreeSet;
use serde_json::{Value,json};
fn rows()->Vec<Value> {serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()}
fn input<M>(row:&Value,mask:M)->BitmapTraceInput<M> {
 let v=&row["input"];BitmapTraceInput {width:v["width"].as_u64().unwrap() as u32,height:v["height"].as_u64().unwrap() as u32,mask,threshold:v["threshold"].as_f64().unwrap(),simplify_epsilon:v["simplifyEpsilon"].as_f64().unwrap(),max_pixels:v["maxPixels"].as_u64().unwrap() as usize,max_edges:v["maxEdges"].as_u64().unwrap() as usize,max_segments:v["maxSegments"].as_u64().unwrap() as usize,max_work:v["maxWork"].as_u64().unwrap()}
}
fn paths(segments:&[PathSegment])->Vec<Vec<Vec2>> {
 let mut result=Vec::<Vec<Vec2>>::new();for segment in segments {match segment {PathSegment::Move {to}=>result.push(vec![*to]),PathSegment::Line {to}=>result.last_mut().unwrap().push(*to),PathSegment::Close=>{},_=>panic!("Unexpected traced curve")}}result
}
pub(super) fn foreground<M>(segments:&[PathSegment],input:BitmapTraceInput<M>)->Vec<u8> {
 let rings=paths(segments);(0..input.width as usize*input.height as usize).map(|at|{
  let x=(at%input.width as usize) as f64+0.5;let y=(at/input.width as usize) as f64+0.5;let p=[x,y];let mut winding=0;
  for ring in &rings {for index in 0..ring.len() {let a=ring[index];let b=ring[(index+1)%ring.len()];let side=cross(a,b,p);if a[1]<=y&&b[1]>y&&side>0.0 {winding+=1;}else if a[1]>y&&b[1]<=y&&side<0.0 {winding-=1;}}}u8::from(winding!=0)
 }).collect()
}
#[path="../../🛤️path/🎟️work/🧪️tests/🦀️.rs"]mod funding;
fn wallet()->funding::Wallet{let data:Value=serde_json::from_str(include_str!("../🎟️work/📐️.json")).unwrap();let v=&data["stock"];funding::Wallet::from_grant(semio_framework_value::RetainedCloneGrant{maximum_items:v["items"].as_u64().unwrap()as usize,maximum_copy_bytes:v["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:v["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:v["release"].as_u64().unwrap()as usize,maximum_depth:v["depth"].as_u64().unwrap()as usize},data["maxTurns"].as_u64().unwrap()as usize)}
fn trace<M:AsRef<[u8]>+semio_framework_value::RetireOwned>(input:BitmapTraceInput<M>)->funding::Funded<BitmapTraceJob<M>>{funding::Funded::from_parts(BitmapTraceJob::new(input),wallet())}
fn through<M:AsRef<[u8]>+semio_framework_value::RetireOwned>(job:&mut funding::Funded<BitmapTraceJob<M>>,items:usize)->Result<BitmapTraceProgress,BitmapTraceError>{for _ in 0..262144{let p=job.turn(items,|owner,work|owner.advance(items,work))?;if p.done{return Ok(p);}assert!(job.last_turn_work()>0,"Trace finite stock blocked at {:?}",job.next_work_demand());}panic!("finite trace observation bound exhausted");}
fn refused<M:AsRef<[u8]>+semio_framework_value::RetireOwned>(input:BitmapTraceInput<M>){let mut job=trace(input);assert!(job.turn(1,|owner,work|owner.advance(1,work)).is_err());job.finish();}
fn complete<M:AsRef<[u8]>+semio_framework_value::RetireOwned>(input:BitmapTraceInput<M>,grant:usize)->(Vec<PathSegment>,Vec<BitmapTracePhase>){let max_work=input.max_work;let mut job=trace(input);assert_eq!(job.result().unwrap_err(),BitmapTraceError::Incomplete);let mut previous=0;let mut phases=Vec::new();loop{let p=job.turn(grant,|owner,work|owner.advance(grant,work)).unwrap();assert!(p.work-previous<=grant as u64);assert_eq!(p.work-previous,job.last_turn_semantic_work());previous=p.work;assert!(previous<=max_work);phases.push(p.phase);if p.done{let output=job.result().unwrap().to_vec();job.finish();return(output,phases);}assert_eq!(job.result().unwrap_err(),BitmapTraceError::Incomplete);}}
#[test]
fn trace_neutral_vectors_and_work_grants() {
 for row in rows() {let mask=row["input"]["mask"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect::<Vec<_>>();let input=input(&row,&mask);let expected=row["expected"]["foreground"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect::<Vec<_>>();let mut previous=None;
  for grant in [1,7,4096] {let (segments,_)=complete(input,grant);assert_eq!(foreground(&segments,input),expected,"{}",row["name"]);assert_eq!(paths(&segments).len(),row["expected"]["contours"].as_u64().unwrap() as usize);
   if let Some(expected)=row["expected"]["segments"].as_array() {let actual=segments.iter().map(|s|match s {PathSegment::Move{to}=>json!({"kind":"move","to":to}),PathSegment::Line{to}=>json!({"kind":"line","to":to}),PathSegment::Close=>json!({"kind":"close"}),_=>panic!("Unexpected trace segment")}).collect::<Vec<_>>();for (a,b) in actual.iter().zip(expected) {assert_eq!(a["kind"],b["kind"]);if a["kind"]!="close" {for axis in 0..2 {assert_eq!(a["to"][axis].as_f64(),b["to"][axis].as_f64());}}}assert_eq!(actual.len(),expected.len());}
   if let Some(previous)=&previous {assert_eq!(&segments,previous);}previous=Some(segments);
  }
 }
}
#[test]
fn trace_invalid_contracts_and_sticky_resource_refusals() {
 let mask=[255u8];let valid:BitmapTraceInput<&[u8]>=BitmapTraceInput {width:1,height:1,mask:&mask,threshold:0.5,simplify_epsilon:0.0,max_pixels:1,max_edges:4,max_segments:5,max_work:10000};
 for input in [BitmapTraceInput {width:0,..valid},BitmapTraceInput {height:8193,..valid},BitmapTraceInput {mask:&[],..valid},BitmapTraceInput {mask:&[255,255],..valid},BitmapTraceInput {threshold:-0.1,..valid},BitmapTraceInput {threshold:1.1,..valid},BitmapTraceInput {threshold:f64::NAN,..valid},BitmapTraceInput {simplify_epsilon:-1.0,..valid},BitmapTraceInput {simplify_epsilon:f64::INFINITY,..valid},BitmapTraceInput {max_pixels:0,..valid},BitmapTraceInput {max_edges:0,..valid},BitmapTraceInput {max_segments:0,..valid},BitmapTraceInput {max_work:0,..valid},BitmapTraceInput {max_pixels:16777217,..valid},BitmapTraceInput {max_edges:65537,..valid},BitmapTraceInput {max_segments:65537,..valid},BitmapTraceInput {max_work:1000000001,..valid}] {refused(input);}
 for input in [BitmapTraceInput {max_edges:3,..valid},BitmapTraceInput {max_segments:4,..valid},BitmapTraceInput {max_work:1,..valid}] {let mut job=trace(input);let error=through(&mut job,4096).unwrap_err();assert_eq!(job.result().unwrap_err(),error);assert_eq!(job.turn(1,|owner,work|owner.advance(1,work)).unwrap_err(),error);job.finish();}
 refused(BitmapTraceInput{width:2,mask:&[255,255][..],..valid});let mut job=trace(valid);assert!(job.turn(0,|owner,work|owner.advance(0,work)).is_err());job.finish();
}
#[test]
fn trace_cancels_every_phase_without_exposing_private_geometry() {
 let row=&rows()[13];let mask=row["input"]["mask"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect::<Vec<_>>();let input=input(row,&mask);let mut probe=trace(input);let mut seen=BTreeSet::new();let mut steps=0;
 loop {let phase=if steps==0 {BitmapTracePhase::Scan} else {probe.turn(1,|owner,work|owner.advance(1,work)).unwrap().phase};if seen.insert(phase.as_str()) {let mut job=trace(input);for _ in 0..steps {job.turn(1,|owner,work|owner.advance(1,work)).unwrap();}job.cancel();assert_eq!(job.result().unwrap_err(),BitmapTraceError::Cancelled);assert_eq!(job.turn(1,|owner,work|owner.advance(1,work)).unwrap_err(),BitmapTraceError::Cancelled);job.finish();}if phase==BitmapTracePhase::Complete {break;}steps+=1;}
 assert_eq!(seen,BTreeSet::from(["scan","contours","compact","simplify","topology","coverage","emit","complete"]));probe.finish();assert_eq!(mask,row["input"]["mask"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect::<Vec<_>>());
}
#[test]
fn trace_returns_empty_geometry_and_exact_resource_limits() {
 let valid:BitmapTraceInput<&[u8]>=BitmapTraceInput {width:1,height:1,mask:&[255],threshold:0.5,simplify_epsilon:0.0,max_pixels:1,max_edges:4,max_segments:5,max_work:10000};let (output,_)=complete(valid,1);assert_eq!(output.len(),5);let mut job=trace(valid);let p=through(&mut job,4096).unwrap();job.finish();let (exact,_)=complete(BitmapTraceInput {max_work:p.work,..valid},7);assert_eq!(output,exact);
 let mut cancelled=trace(valid);through(&mut cancelled,4096).unwrap();let published=cancelled.result().unwrap().to_vec();cancelled.cancel();assert_eq!(published,output);cancelled.finish();assert!(complete(BitmapTraceInput {mask:&[0][..],..valid},1).0.is_empty());
}
#[test]
fn trace_progress_matches_portable_field_taxonomy() {
 let row=&rows()[13];let mask=row["input"]["mask"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect::<Vec<_>>();let mut job=trace(input(row,&mask));
 loop {let p=job.turn(1,|owner,work|owner.advance(1,work)).unwrap();let value=json!({"phase":p.phase.as_str(),"scanned":p.scanned,"pixels":p.pixels,"edges":p.edges,"contours":p.contours,"segments":p.segments,"simplified":p.simplified,"work":p.work,"done":p.done});assert_eq!(value.as_object().unwrap().len(),9);assert!(p.edges<=65536);assert!(p.segments<=65536);if p.done {break;}}job.finish();
}
#[test]
fn trace_all_three_by_three_masks_and_blank_linear_work() {
 let mut simplified=0;for bits in 0..512u16 {let mask=(0..9).map(|at|if bits&(1<<at)==0 {0u8} else {255u8}).collect::<Vec<_>>();let expected=mask.iter().map(|value|u8::from(*value!=0)).collect::<Vec<_>>();
  for epsilon in [0.0,0.55,8192.0] {let input=BitmapTraceInput {width:3,height:3,mask:&mask,threshold:0.5,simplify_epsilon:epsilon,max_pixels:9,max_edges:36,max_segments:45,max_work:10000};let mut job=trace(input);let p=through(&mut job,10000).unwrap();assert!(p.done);simplified+=usize::from(p.simplified);assert_eq!(foreground(job.result().unwrap(),input),expected);job.finish();}
 }assert!(simplified>0);let mask=vec![0u8;8192];let mut job=trace(BitmapTraceInput {width:8192,height:1,mask:&mask,threshold:0.5,simplify_epsilon:0.0,max_pixels:8192,max_edges:1,max_segments:1,max_work:8196});let p=through(&mut job,8196).unwrap();assert!(p.done);let law:Value=serde_json::from_str(include_str!("../🎟️work/📐️.json")).unwrap();assert_eq!(p.work,law["blankSemanticWork"].as_u64().unwrap());assert_eq!(law["blankMaximumWork"],8196);assert!(job.result().unwrap().is_empty());job.finish();
}
#[test]
fn trace_owned_masks_match_neutral_vectors_and_release_at_every_cancel_phase(){
 for row in rows(){let mask=row["input"]["mask"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect::<Vec<_>>();let expected=complete(input(&row,&mask),4096).0;for grant in [1,7,4096]{let mut job=trace(input(&row,mask.clone()));assert_eq!(job.result().unwrap_err(),BitmapTraceError::Incomplete);while !job.turn(grant,|owner,work|owner.advance(grant,work)).unwrap().done{}assert_eq!(job.result().unwrap(),expected);job.finish();}}
 #[derive(semio_framework_value::RetireOwned)]struct Tracked{bytes:Vec<u8>,marker:MaskDrop}
 struct MaskDrop{drops:std::sync::Arc<std::sync::atomic::AtomicUsize>}
 impl Drop for MaskDrop{fn drop(&mut self){self.drops.fetch_add(1,std::sync::atomic::Ordering::SeqCst);}}
 semio_framework_value::artifact_retire_leaf!(MaskDrop);
 impl AsRef<[u8]> for Tracked{fn as_ref(&self)->&[u8]{&self.bytes}}
 let row=&rows()[13];let mask=row["input"]["mask"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect::<Vec<_>>();let mut probe=trace(input(row,&mask));let mut seen=BTreeSet::new();loop{let p=probe.turn(1,|job,work|job.advance(1,work)).unwrap();if seen.insert(p.phase.as_str()){let drops=std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));let mut job=trace(input(row,Tracked{bytes:mask.clone(),marker:MaskDrop{drops:drops.clone()}}));loop{if job.turn(1,|job,work|job.advance(1,work)).unwrap().phase==p.phase{break;}}assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst),0);job.cancel();assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst),0);assert_eq!(job.result().unwrap_err(),BitmapTraceError::Cancelled);assert_eq!(job.turn(1,|job,work|job.advance(1,work)).unwrap_err(),BitmapTraceError::Cancelled);let(job,mut bank)=job.into_parts();let(close,returned)=job.into_retirement();assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst),0);bank.close_owned(close);assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst),0);bank.close_owned(returned);assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst),1);}if p.done{break;}}probe.finish();
}

#[test]
fn trace_retirement_transfers_masks_and_drains_every_actual_phase_under_exact_grants(){let close_rows:Vec<Value>=serde_json::from_str(include_str!("🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();for row in close_rows{for quantum in [1,7,4096]{let source=rows().into_iter().find(|v|v["name"]==row["source"]).unwrap();let mask=source["input"]["mask"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect::<Vec<_>>();let ptr=mask.as_ptr();let mut contract=input(&source,mask);if let Some(cap)=row["maxEdges"].as_u64(){contract.max_edges=cap as usize;}let mut job=trace(contract);let phase=row["phase"].as_str().unwrap();if phase=="cancelled"{job.cancel();}else if phase=="failure"{assert!(through(&mut job,quantum).is_err());assert!(job.result().is_err());}else if phase!="fresh"{let mut found=false;for _ in 0..262144{let p=job.turn(1,|owner,work|owner.advance(1,work)).unwrap();if p.phase.as_str()==phase{found=true;break;}if p.done{break;}}assert!(found,"{}",row);}let(job,mut bank)=job.into_parts();let(close,returned)=job.into_retirement();assert_eq!(returned.as_ref().unwrap().as_ptr(),ptr);assert_eq!(returned.as_ref().unwrap(),&source["input"]["mask"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect::<Vec<_>>());bank.close_owned((close,returned));}}}
