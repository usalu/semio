//! 🧪️ Neutral curves, world transforms, work grants and publication boundaries.
use super::*;
use serde_json::{Value,json};
use std::collections::BTreeSet;
fn rows()->Vec<Value>{serde_json::from_str::<Value>(include_str!("../🧫️fixtures/🔣️.json")).unwrap()["cases"].as_array().unwrap().clone()}
fn point(v:&Value)->Vec2{[v[0].as_f64().unwrap(),v[1].as_f64().unwrap()]}
fn segment(v:&Value)->PathSegment{match v["kind"].as_str().unwrap(){
 "move"=>PathSegment::Move{to:point(&v["to"])},"line"=>PathSegment::Line{to:point(&v["to"])},"quad"=>PathSegment::Quad{ctrl:point(&v["ctrl"]),to:point(&v["to"])},
 "cubic"=>PathSegment::Cubic{ctrl1:point(&v["ctrl1"]),ctrl2:point(&v["ctrl2"]),to:point(&v["to"])},
 "arc"=>PathSegment::Arc{rx:v["rx"].as_f64().unwrap(),ry:v["ry"].as_f64().unwrap(),rotation:v["rotation"].as_f64().unwrap(),large_arc:v["largeArc"].as_bool().unwrap(),sweep:v["sweep"].as_bool().unwrap(),to:point(&v["to"])},
 "close"=>PathSegment::Close,_=>panic!("Unknown fixture segment")}}
fn input(row:&Value)->PathBooleanInput{PathBooleanInput{operation:BooleanOperation::parse(row["operation"].as_str().unwrap()).unwrap(),operands:row["operands"].as_array().unwrap().iter().map(|v|PathBooleanOperand{
 segments:v["segments"].as_array().unwrap().iter().map(segment).collect(),transform:std::array::from_fn(|at|v["transform"][at].as_f64().unwrap()),tolerance:v["tolerance"].as_f64().unwrap(),fill_rule:if v["fillRule"]=="evenodd"{BooleanFillRule::Evenodd}else{BooleanFillRule::Nonzero}}).collect(),epsilon:1e-8,max_edges:65536,max_parameters:262144,max_atomic_edges:65536,max_segments:65536,max_work:10000000}}
fn finish(value:PathBooleanInput,grant:usize)->(Vec<PathSegment>,PathBooleanProgress){
 let mut job=PathBooleanJob::new(value).unwrap();assert_eq!(job.result().unwrap_err(),BooleanError::Incomplete);let mut work=0;
 loop{let p=job.advance(grant).unwrap();assert!(p.work-work<=grant as u64);work=p.work;if p.done{return(job.into_result().unwrap(),p);}assert_eq!(job.result().unwrap_err(),BooleanError::Incomplete);}
}
fn close(job:PathBooleanJob){let(mut retired,_)=job.into_retirement();while !retired.advance(4096).unwrap().done{}}
fn area(segments:&[PathSegment])->f64{let(mut origin,mut previous,mut sum)=([0.0;2],[0.0;2],0.0);for s in segments{match s{PathSegment::Move{to}=>{origin=*to;previous=*to;},PathSegment::Line{to}=>{sum+=((previous[0]-origin[0])*(to[1]-origin[1])-(previous[1]-origin[1])*(to[0]-origin[0]))/2.0;previous=*to;},PathSegment::Close=>{},_=>panic!("Unexpected unresolved curve")}}sum}
#[test]
fn neutral_curves_and_world_transforms_have_stable_grant_outputs(){
 for row in rows(){let source=input(&row);let mut previous=None;for grant in [1,7,4096]{let(result,p)=finish(source.clone(),grant);assert_eq!(result.iter().filter(|s|matches!(s,PathSegment::Move{..})).count(),row["contours"].as_u64().unwrap() as usize,"{}",row["id"]);if let Some(expected)=row["area"].as_f64(){assert!((area(&result)-expected).abs()<1e-8);}if let Some(before)=previous{assert_eq!(before,result);}previous=Some(result);assert_eq!(p.operands,source.operands.len());assert!(p.points<=source.max_edges);println!("[DEBUG] curved boolean {} grant={} work={} points={}",row["id"],grant,p.work,p.points);}}
}
#[test]
fn source_point_output_and_work_caps_fail_privately_and_stay_failed(){
 let source=input(&rows()[0]);let mut empty=source.clone();empty.operands.clear();assert!(PathBooleanJob::new(empty).is_err());let mut zero=source.clone();zero.max_work=0;assert!(PathBooleanJob::new(zero).is_err());
 for kind in 0..7{let mut value=source.clone();match kind{0=>value.max_edges=4,1=>value.max_work=2,2=>value.max_segments=2,3=>value.operands[0].transform[0]=f64::NAN,4=>value.operands[0].segments[0]=PathSegment::Move{to:[f64::NAN,0.0]},5=>value.operands[0].tolerance=0.0,_=>{value=input(&rows()[1]);value.max_edges=16;}}
  let mut job=PathBooleanJob::new(value).unwrap();let error=job.advance(10000000).unwrap_err();assert_eq!(job.advance(1).unwrap_err(),error);assert_eq!(job.result().unwrap_err(),error);close(job);}
 assert!(PathBooleanJob::new(source).unwrap().advance(0).is_err());
}
#[test]
fn cancellation_covers_every_path_stage_and_owned_results_survive(){
 let source=input(&rows()[1]);let mut probe=PathBooleanJob::new(source.clone()).unwrap();let mut seen=BTreeSet::new();loop{let p=probe.advance(1).unwrap();if seen.insert(p.phase.as_str()){let mut job=PathBooleanJob::new(source.clone()).unwrap();job.advance(p.work as usize).unwrap();job.cancel();assert_eq!(job.advance(1).unwrap_err(),BooleanError::Cancelled);assert_eq!(job.result().unwrap_err(),BooleanError::Cancelled);close(job);}if p.done{break;}}
 close(probe);assert_eq!(seen,BTreeSet::from(["admitting","flattening","transforming","boolean","complete"]));let mut job=PathBooleanJob::new(source).unwrap();job.advance(10000000).unwrap();let published=job.result().unwrap().to_vec();job.cancel();assert!(!published.is_empty());close(job);
}
#[test]
fn exact_caps_and_progress_fields_match_the_neutral_contract(){
 let source=input(&rows()[0]);let(expected,p)=finish(source.clone(),1);let b=p.boolean.unwrap();let mut exact=source;exact.max_edges=p.source_segments.max(p.points);exact.max_parameters=b.parameters;exact.max_atomic_edges=b.atomic_edges;exact.max_segments=b.segments;exact.max_work=p.work;assert_eq!(finish(exact,7).0,expected);
 let value=json!({"phase":p.phase.as_str(),"operands":p.operands,"sourceSegments":p.source_segments,"points":p.points,"work":p.work,"flatten":p.flatten.map(|p|json!({"phase":match p.phase{crate::flatten::PathFlattenPhase::Preparing=>"preparing",crate::flatten::PathFlattenPhase::Subdividing=>"subdividing",crate::flatten::PathFlattenPhase::Complete=>"complete"},"completed":p.completed,"total":p.total,"points":p.points,"work":p.work,"done":p.done})),"boolean":b.work,"done":p.done});assert_eq!(value.as_object().unwrap().len(),8);
}

#[test]
fn path_boolean_retirement_composes_child_owners_and_keeps_completed_output(){
 let fixtures:Value=serde_json::from_str(include_str!("../🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();let source_row=rows().into_iter().find(|r|r["id"]==fixtures["source"]).unwrap();
 for row in fixtures["cases"].as_array().unwrap(){for grant in [1,7,4096]{let mut source=input(&source_row);if let Some(n)=row["maxWork"].as_u64(){source.max_work=n;}if let Some(n)=row["maxSegments"].as_u64(){source.max_segments=n as usize;}let mut job=PathBooleanJob::new(source).unwrap();let phase=row["phase"].as_str().unwrap();
  if phase=="failure"{assert!(job.advance(100000).is_err());if row["maxWork"].is_number(){assert!(job.current.is_some());}}else if phase!="fresh"{if phase!="cancelled"{let mut reached=false;for _ in 0..100000{let p=job.advance(1).unwrap();if match phase{"flattenClosing"=>job.flat_retirement.is_some(),"booleanClosing"=>job.boolean_retirement.is_some(),_=>p.phase.as_str()==phase}{reached=true;break;}}assert!(reached);}for _ in 0..row["offset"].as_u64().unwrap(){job.advance(1).unwrap();}}
  if phase=="cancelled"{job.cancel();}let ptr=if job.phase==PathBooleanPhase::Complete&&!job.cancelled&&job.failure.is_none(){Some(job.result().unwrap().as_ptr())}else{None};let(mut close,output)=job.into_retirement();if let Some(ptr)=ptr{assert_eq!(output.as_ref().unwrap().as_ptr(),ptr);assert_eq!(output.unwrap(),finish(input(&source_row),4096).0);}else{assert!(output.is_none());}
  assert!(close.advance(0).is_err());let mut work=0;loop{let p=close.advance(grant).unwrap();assert!(p.work>=work&&p.work-work<=grant as u64);work=p.work;if p.done{break;}}
  assert!(work>0);assert!(close.terminal_is_empty());assert!(close.owner.original().is_none());let p=close.advance(1).unwrap();assert_eq!((p.phase,p.work,p.done),("complete",work,true));println!("[DEBUG] Native Path Boolean retirement {}: {} cleanup steps at grant {}",phase,work,grant);
 }}
}
