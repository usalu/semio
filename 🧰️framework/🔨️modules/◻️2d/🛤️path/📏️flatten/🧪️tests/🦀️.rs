//! 🧪️ Shared subdivision vectors and native preparation limits.
use super::*;
fn fixtures()->serde_json::Value {serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
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
  let mut job=PathFlattenJob::new(input(row)).unwrap();let mut work=0;let mut done=false;
  for _ in 0..200000 {let p=job.advance(grant).unwrap();assert!(p.work-work<=grant as u64);work=p.work;if p.done {done=true;break;}}
  assert!(done,"{}",row["name"]);let actual=job.result().unwrap();let expected=row["expected"].as_array().unwrap();assert_eq!(actual.len(),expected.len());
  for (contour,expected) in actual.iter().zip(expected) {
   assert_eq!(contour.closed,expected["closed"].as_bool().unwrap());let points=expected["points"].as_array().unwrap();assert_eq!(contour.points.len(),points.len(),"{}",row["name"]);
   for (p,e) in contour.points.iter().zip(points) {for axis in 0..2 {assert!((p[axis]-e[axis].as_f64().unwrap()).abs()<1e-10,"{}",row["name"]);}}
  }
  assert_eq!(job.into_result().unwrap().len(),expected.len());
 }}
}
#[test]
fn partial_and_cancelled_contours_are_unavailable() {
 let rows=fixtures();
 for steps in [0,1,2,3] {
  let mut job=PathFlattenJob::new(input(&rows[7])).unwrap();assert_eq!(job.result(),Err(PathFlattenError::Incomplete));
  for _ in 0..steps {job.advance(1).unwrap();}job.cancel();assert_eq!(job.advance(1),Err(PathFlattenError::Cancelled));assert_eq!(job.result(),Err(PathFlattenError::Cancelled));
 }
}
#[test]
fn invalid_contracts_points_and_output_budgets_are_rejected() {
 let rows=fixtures();let base=input(&rows[7]);
 for tolerance in [0.0,f64::INFINITY] {let mut bad=base.clone();bad.tolerance=tolerance;assert!(PathFlattenJob::new(bad).is_err());}
 let mut bad=base.clone();bad.transform[4]=f64::NAN;assert!(PathFlattenJob::new(bad).is_err());
 let mut bad=base.clone();bad.segments=vec![PathSegment::Close;65537];assert!(PathFlattenJob::new(bad).is_err());
 let mut bad=base.clone();bad.segments=vec![PathSegment::Move{to:[f64::NAN,0.0]}];let mut job=PathFlattenJob::new(bad).unwrap();assert!(job.advance(4096).is_err());assert!(job.result().is_err());
 let mut job=PathFlattenJob::new(base.clone()).unwrap();assert!(job.advance(0).is_err());
 for segments in [vec![PathSegment::Move{to:[0.0,0.0]};4097],vec![PathSegment::Line{to:[1.0,0.0]};65536]] {
  let mut job=PathFlattenJob::new(PathFlattenInput{segments,..base.clone()}).unwrap();let mut error=None;
  for _ in 0..100 {match job.advance(4096) {Ok(p)=>assert!(!p.done),Err(e)=>{error=Some(e);break;}}}
  assert!(matches!(error,Some(PathFlattenError::Invalid(message)) if message.contains("budget")));assert!(job.result().is_err());
 }
}

#[test]
fn flatten_retirement_preserves_owned_contours_and_drains_private_curves_under_grants(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap(){for grant in [1,7,4096]{let mut job=PathFlattenJob::new(input(row)).unwrap();let phase=row["phase"].as_str().unwrap();let steps=row["steps"].as_u64().unwrap()as usize;if phase=="failure"{assert!(job.advance(steps).is_err());}else if steps>0{let p=job.advance(steps).unwrap();if phase!="cancelled"{let actual=match p.phase{PathFlattenPhase::Preparing=>"preparing",PathFlattenPhase::Subdividing=>"subdividing",PathFlattenPhase::Complete=>"complete"};assert_eq!(actual,phase);}}
  if phase=="cancelled"{job.cancel();}let published=if phase=="complete"{Some(job.result().unwrap().as_ptr())}else{None};let inventory=serde_json::json!(job.contours.iter().map(|c|c.points.len()).collect::<Vec<_>>());let expected=3+if published.is_some(){0}else{inventory.as_array().unwrap().len()as u64};assert_eq!(expected,row["work"].as_u64().unwrap());
  let(mut close,output)=job.into_retirement();if let Some(ptr)=published{assert_eq!(output.as_ref().unwrap().as_ptr(),ptr);}if let Some(contours)=&output{let expected=row["output"].as_array().unwrap();assert_eq!(contours.len(),expected.len());for(c,e)in contours.iter().zip(expected){assert_eq!(c.closed,e["closed"].as_bool().unwrap());let points=e["points"].as_array().unwrap();assert_eq!(c.points.len(),points.len());for(a,b)in c.points.iter().zip(points){for axis in 0..2{assert_eq!(a[axis],b[axis].as_f64().unwrap());}}}}else{assert!(row["output"].is_null());}assert!(close.advance(0).is_err());let mut work=0;loop{let before_count=close.job.as_ref().map_or(0,|j|j.contours.len());let p=close.advance(grant).unwrap();let after_count=close.job.as_ref().map_or(0,|j|j.contours.len());assert!(before_count-after_count<=grant);assert!(p.work>work&&p.work-work<=grant as u64);work=p.work;if p.done{break;}}assert_eq!(work,expected);assert!(close.terminal_is_empty());assert!(close.job.is_none());let p=close.advance(1).unwrap();assert_eq!((p.phase,p.work,p.done),("complete",work,true));
 }println!("[DEBUG] Native flatten retirement kept its output and drained private owners: {}",row["phase"]);}
}
