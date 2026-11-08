//! 🧪️ Shared subdivision vectors and native preparation limits.
use super::*;
#[path="../../🎟️work/🧪️tests/🦀️.rs"]
mod funding;
funding::physical_close!(PathFlattenRetirement,funding::PhysicalClose);
fn funded(input:PathFlattenInput)->funding::Funded<PathFlattenJob>{funding::Funded::new(PathFlattenJob::new(input),include_str!("🎟️work/🔣️.json"))}
fn assert_refused(input:PathFlattenInput){let mut job=funded(input);let mut refused=false;for _ in 0..job.turn_bound(){match job.turn(1,|job,work,alloc|job.advance(1,work,alloc)){Err(PathFlattenError::Invalid(_))=>{refused=true;break;},Err(error)=>panic!("unexpected invalid-source refusal: {error:?}"),Ok(progress)=>assert!(!progress.done,"invalid source published completed geometry")}}assert!(refused,"declared invalid source never reached real validation refusal");assert!(matches!(job.result(),Err(PathFlattenError::Invalid(_))));job.finish();}
fn at_semantic_work(job:&mut funding::Funded<PathFlattenJob>,target:u64)->Result<PathFlattenProgress,PathFlattenError>{if target==0{return Ok(job.progress());}for _ in 0..job.turn_bound(){let progress=job.turn(1,|job,work,alloc|job.advance(1,work,alloc))?;if progress.work>=target||progress.done{return Ok(progress);}}panic!("Semantic checkpoint did not settle within its declared turn bound")}

fn fixtures()->serde_json::Value {serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()}
fn point(v:&serde_json::Value)->Vec2 {[v[0].as_f64().unwrap(),v[1].as_f64().unwrap()]}
fn input(v:&serde_json::Value)->PathFlattenInput {
 let v=&v["input"];
 let segments=v["segments"].as_array().unwrap().iter().map(|s|match s["kind"].as_str().unwrap() {
  "move"=>PathSegment::Move{to:point(&s["to"])},"line"=>PathSegment::Line{to:point(&s["to"])},
  "quad"=>PathSegment::Quad{ctrl:point(&s["ctrl"]),to:point(&s["to"])},"cubic"=>PathSegment::Cubic{ctrl1:point(&s["ctrl1"]),ctrl2:point(&s["ctrl2"]),to:point(&s["to"])},
  "arc"=>PathSegment::Arc{rx:s["rx"].as_f64().unwrap(),ry:s["ry"].as_f64().unwrap(),rotation:s["rotation"].as_f64().unwrap(),large_arc:s["largeArc"].as_bool().unwrap(),sweep:s["sweep"].as_bool().unwrap(),to:point(&s["to"])},"close"=>PathSegment::Close,_=>panic!("Invalid fixture")
 }).collect();
 PathFlattenInput {segments,transform:std::array::from_fn(|at|v["transform"][at].as_f64().unwrap()),tolerance:v["tolerance"].as_f64().unwrap()}
}
#[test]
fn shared_contours_are_budget_independent() {
 for row in fixtures().as_array().unwrap() {for grant in [1,7,4096] {
  let mut job=funded(input(row));let mut work=0;let mut done=false;
  for _ in 0..200000 {let p=job.turn(grant,|job,work,alloc|job.advance(grant,work,alloc)).unwrap();assert!(p.work-work<=grant as u64);work=p.work;if p.done {done=true;break;}}
  assert!(done,"{}",row["name"]);let actual=job.result().unwrap();let expected=row["expected"].as_array().unwrap();assert_eq!(actual.len(),expected.len());
  for (contour,expected) in actual.iter().zip(expected) {
   assert_eq!(contour.closed,expected["closed"].as_bool().unwrap());let points=expected["points"].as_array().unwrap();assert_eq!(contour.points.len(),points.len(),"{}",row["name"]);
   for (p,e) in contour.points.iter().zip(points) {for axis in 0..2 {assert!((p[axis]-e[axis].as_f64().unwrap()).abs()<1e-10,"{}",row["name"]);}}
  }
  assert_eq!(job.result().unwrap().len(),expected.len());job.finish();
 }}
}
#[test]
fn partial_and_cancelled_contours_are_unavailable() {
 let rows=fixtures();
 for steps in [0,1,2,3] {
  let mut job=funded(input(&rows[7]));assert_eq!(job.result(),Err(PathFlattenError::Incomplete));
  at_semantic_work(&mut job,steps as u64).unwrap();job.cancel();assert_eq!(job.turn(1,|job,work,alloc|job.advance(1,work,alloc)),Err(PathFlattenError::Cancelled));assert_eq!(job.result(),Err(PathFlattenError::Cancelled));job.finish();
 }
}
#[test]
fn invalid_contracts_points_and_output_budgets_are_rejected() {
 let rows=fixtures();let base=input(&rows[7]);
 for tolerance in [0.0,f64::INFINITY] {let mut bad=base.clone();bad.tolerance=tolerance;assert_refused(bad);}
 let mut bad=base.clone();bad.transform[4]=f64::NAN;assert_refused(bad);
 let mut bad=base.clone();bad.segments=vec![PathSegment::Close;65537];assert_refused(bad);
 let mut bad=base.clone();bad.segments=vec![PathSegment::Move{to:[f64::NAN,0.0]}];let mut job=funded(bad);assert!(job.turn(4096,|job,work,alloc|job.advance(4096,work,alloc)).is_err());assert!(job.result().is_err());job.finish();
 let mut job=funded(base.clone());assert!(job.turn(0,|job,work,alloc|job.advance(0,work,alloc)).is_err());job.finish();
 for(name,segments,message,completed,points,contours)in [
  ("contour budget",vec![PathSegment::Move{to:[0.0,0.0]};4097],"Path preparation exceeds output budget",4096,4096,4096),
  ("point budget",vec![PathSegment::Line{to:[1.0,0.0]};65536],"Path preparation exceeds point budget",65535,65536,1),
 ]{
  let mut job=funded(PathFlattenInput{segments,..base.clone()});let mut error=None;
  for _ in 0..job.turn_bound(){match job.turn(4096,|job,work,alloc|job.advance(4096,work,alloc)){Ok(progress)=>{assert!(!progress.done,"{name} published invalid geometry");assert!(job.last_turn_work()>0,"{name} exhausted its declared physical stock");},Err(refusal)=>{error=Some(refusal);break;}}}
  assert_eq!(error,Some(PathFlattenError::Invalid(message)),"{name} did not reach its exact guard");assert_eq!((job.progress().completed,job.progress().points,job.contours.len()),(completed,points,contours),"{name} changed its semantic frontier");assert_eq!(job.result(),Err(PathFlattenError::Invalid(message)));job.finish();
 }
}

#[test]
fn flatten_retirement_preserves_owned_contours_and_drains_private_curves_under_grants(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap(){for grant in [1,7,4096]{let mut job=funded(input(row));let phase=row["phase"].as_str().unwrap();let steps=row["steps"].as_u64().unwrap()as usize;if phase=="failure"{assert!(at_semantic_work(&mut job,steps as u64).is_err());}else if steps>0{let p=at_semantic_work(&mut job,steps as u64).unwrap();if phase!="cancelled"{let actual=match p.phase{PathFlattenPhase::Preparing=>"preparing",PathFlattenPhase::Subdividing=>"subdividing",PathFlattenPhase::Complete=>"complete"};assert_eq!(actual,phase);}}
  if phase=="cancelled"{job.cancel();}let published=if phase=="complete"{Some(job.result().unwrap().as_ptr())}else{None};let inventory=serde_json::json!(job.contours.iter().map(|c|c.points.len()).collect::<Vec<_>>());let expected=3+if published.is_some(){0}else{inventory.as_array().unwrap().len()as u64};assert_eq!(expected,row["work"].as_u64().unwrap());
  let(mut close,output)=job.retire_with(|job|job.into_retirement());if let Some(ptr)=published{assert_eq!(output.as_ref().unwrap().as_ptr(),ptr);}if let Some(contours)=&output{let expected=row["output"].as_array().unwrap();assert_eq!(contours.len(),expected.len());for(c,e)in contours.iter().zip(expected){assert_eq!(c.closed,e["closed"].as_bool().unwrap());let points=e["points"].as_array().unwrap();assert_eq!(c.points.len(),points.len());for(a,b)in c.points.iter().zip(points){for axis in 0..2{assert_eq!(a[axis],b[axis].as_f64().unwrap());}}}}else{assert!(row["output"].is_null());}assert!(close.close_turn(0).is_err());let mut work=0;loop{let before_count=close.owner.original().map_or(0,|j|j.contours.len());let p=close.close_turn(grant).unwrap();let after_count=close.owner.original().map_or(0,|j|j.contours.len());assert!(after_count<=before_count);assert!(p.work>=work&&p.work-work<=grant as u64);work=p.work;if p.done{break;}}assert!(work>0);assert!(close.terminal_is_empty());assert!(close.owner.original().is_none());let p=close.close_turn(1).unwrap();assert_eq!((p.phase,p.work,p.done),("complete",work,true));let mut wallet=close.finish();wallet.close_owned(output);
 }}
}

#[test]
fn cancellation_retains_actual_private_buffers_until_consuming_retirement(){
 let rows=fixtures();for completed in [false,true]{let mut job=funded(input(&rows[7]));if completed{while !job.turn(4096,|job,work,alloc|job.advance(4096,work,alloc)).unwrap().done{}}else{at_semantic_work(&mut job,2).unwrap();}let segments=job.input.segments.as_ptr();let contours=job.contours.as_ptr();let points=job.contours[0].points.as_ptr();let stack=job.stack.as_ptr();let count=job.contours.len();job.cancel();assert_eq!(job.input.segments.as_ptr(),segments);assert_eq!(job.contours.as_ptr(),contours);assert_eq!(job.contours[0].points.as_ptr(),points);assert_eq!(job.stack.as_ptr(),stack);assert_eq!(job.result(),Err(PathFlattenError::Cancelled));let(mut retired,output)=job.retire_with(|job|job.into_retirement());assert!(output.is_none());assert_eq!(retired.owner.original().unwrap().contours.as_ptr(),contours);let mut progress=retired.close_turn(1).unwrap();while !progress.done{progress=retired.close_turn(1).unwrap();}assert!(progress.work>0);assert!(retired.terminal_is_empty());let mut wallet=retired.finish();wallet.close_owned(output);}
}

#[test]
fn retained_admission_keeps_refused_flatten_source(){let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧹️retire/🧪️tests/🧫️fixtures/📥️admission/🔣️.json")).unwrap();for row in fixture["cases"].as_array().unwrap(){if row["kind"]!="flatten"{continue;}let source=PathFlattenInput{segments:vec![PathSegment::Move{to:[3.0,7.0]}],transform:[1.0,0.0,0.0,1.0,0.0,0.0],tolerance:row["tolerance"].as_f64().unwrap()};let segments=source.segments.as_ptr();let mut job=funded(source);assert_eq!(job.input.segments.as_ptr(),segments);assert!(job.turn(1,|job,work,alloc|job.advance(1,work,alloc)).is_err());assert!(job.result().is_err());let(mut close,output)=job.retire_with(|job|job.into_retirement());assert!(output.is_none());assert_eq!(close.owner.original().unwrap().input.segments.as_ptr(),segments);while !close.close_turn(1).unwrap().done{}assert!(close.terminal_is_empty());let mut wallet=close.finish();wallet.close_owned(output);}}

#[path="🎟️work/🦀️.rs"]
mod work;
