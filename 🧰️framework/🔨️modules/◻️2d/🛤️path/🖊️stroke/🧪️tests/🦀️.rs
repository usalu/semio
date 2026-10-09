//! 🧫️ Native stroke alpha vectors and preparation interruption.
use super::*;
use semio_framework_pixels::coverage::{CoverageInput,CoverageJob,CoverageRule};
fn fixtures()->serde_json::Value {serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
#[test]
fn stroke_retirement_transfers_complete_paint_and_drains_each_private_owner_under_grants(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();let rows=fixtures();
 for case in cases.as_array().unwrap(){let base=rows.as_array().unwrap().iter().find(|row|row["name"]==case["source"]).unwrap();for grant in [1,7,4096]{let mut source=input(base);if let Some(repeat)=case["repeat"].as_u64(){source.contours=vec![source.contours[0].clone();usize::try_from(repeat).unwrap()];}let stop=case["stop"].as_str().unwrap();if stop=="failure"{source.contours[0].points[0][0]=1000000001.0;}let mut job=StrokeOutlineJob::new(source.clone()).unwrap();
  if stop=="preparing"{job.advance(1).unwrap();}else if stop!="fresh"{let mut reached=false;for _ in 0..200000{match job.advance(1){Err(_)if stop=="failure"=>{reached=true;break;},Err(error)=>panic!("{error:?}"),Ok(_)=>{}}let at=match stop{"source"=>job.stage==Stage::Source,"offset"=>job.stage==Stage::Offset,"walk"=>job.stage==Stage::Walk,"finish"=>job.stage==Stage::Finish,"outline"=>job.stage==Stage::Outline,"queue"|"cancelled"=>!job.queue.is_empty(),"round"=>job.round.is_some(),"done"=>job.stage==Stage::Done,"failure"=>false,_=>panic!("unknown stroke checkpoint")};if at{reached=true;break;}}assert!(reached,"{}",case["name"]);}
  let external=case["publishedBeforeCancel"].as_bool().unwrap_or(false).then(||job.result().unwrap().to_vec());if stop=="cancelled"||external.is_some()||case["cancelBeforeTransfer"].as_bool().unwrap_or(false){job.cancel();}let published=case["output"].as_bool().unwrap();let expected_work=case["work"]["native"].as_u64().unwrap();assert_eq!((10+job.input.contours.len()+job.sources.len()+job.runs.len()+if published{0}else{job.polygons.len()}+job.queue.len())as u64,expected_work,"{}",case["name"]);
  let pointer=published.then(||job.result().unwrap().as_ptr());let(mut retirement,output)=job.into_retirement();assert_eq!(output.is_some(),published);if let Some(pointer)=pointer{assert_eq!(output.as_ref().unwrap().as_ptr(),pointer,"completed point buffers move without a geometry clone");}assert!(retirement.advance(0).is_err());if usize::BITS>53{assert!(retirement.advance(usize::MAX).is_err());}
  let mut work=0;for _ in 0..2_000_000{let progress=retirement.advance(grant).unwrap();assert!(progress.work>=work&&progress.work-work<=grant as u64);work=progress.work;if progress.done{break;}}
  assert!(work>0,"{}",case["name"]);assert!(retirement.terminal_is_empty());assert!(retirement.owner.original().is_none());let terminal=retirement.advance(1).unwrap();assert!(terminal.done);assert_eq!(terminal.work,work);
  if let Some(contours)=output.or(external){let expected:Vec<u8>=base["expected"].as_array().unwrap().iter().map(|value|u8::try_from(value.as_u64().unwrap()).unwrap()).collect();let mut coverage=CoverageJob::new(CoverageInput{width:u32::try_from(base["extent"][0].as_u64().unwrap()).unwrap(),height:u32::try_from(base["extent"][1].as_u64().unwrap()).unwrap(),transform:source.transform,rule:CoverageRule::NonZero,contours}).unwrap();while !coverage.advance(4096).unwrap().done{}assert_eq!(coverage.result().unwrap().coverage,expected);}
  println!("[DEBUG] Actual stroke retirement {} grant={grant} structural_work={work} complete_output={published} terminal_empty=true",case["name"]);
 }}
}
fn input(row:&serde_json::Value)->StrokeOutlineInput {
 StrokeOutlineInput {contours:vec![StrokeContour {points:row["points"].as_array().unwrap().iter().map(|p|[p[0].as_f64().unwrap(),p[1].as_f64().unwrap()]).collect(),closed:row["closed"].as_bool().unwrap()}],transform:[1.0,0.0,0.0,1.0,0.0,0.0],tolerance:0.0001,
 style:StrokeGeometryStyle {width:row["width"].as_f64().unwrap(),cap:match row["cap"].as_str().unwrap() {"butt"=>StrokeGeometryCap::Butt,"round"=>StrokeGeometryCap::Round,"square"=>StrokeGeometryCap::Square,_=>panic!("Invalid fixture")},join:match row["join"].as_str().unwrap() {"miter"=>StrokeGeometryJoin::Miter,"round"=>StrokeGeometryJoin::Round,"bevel"=>StrokeGeometryJoin::Bevel,_=>panic!("Invalid fixture")},miter_limit:row["miterLimit"].as_f64().unwrap(),dash:row["dash"].as_array().unwrap().iter().map(|v|v.as_f64().unwrap()).collect(),dash_offset:row["dashOffset"].as_f64().unwrap()}}
}
#[test]
fn shared_stroke_masks_are_budget_independent() {
 for row in fixtures().as_array().unwrap() {for grant in [1,7,4096] {
  let mut job=StrokeOutlineJob::new(input(row)).unwrap();let mut work=0;let mut done=false;
  for _ in 0..200000 {let p=job.advance(grant).unwrap();assert!(p.work-work<=grant as u64);work=p.work;if p.done {done=true;break;}}
  assert!(done,"{}",row["name"]);let contours=job.into_result().unwrap();
  let mut coverage=CoverageJob::new(CoverageInput {width:row["extent"][0].as_u64().unwrap() as u32,height:row["extent"][1].as_u64().unwrap() as u32,transform:[1.0,0.0,0.0,1.0,0.0,0.0],rule:CoverageRule::NonZero,contours}).unwrap();while !coverage.advance(4096).unwrap().done {}
  let expected:Vec<u8>=row["expected"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect();assert_eq!(coverage.result().unwrap().coverage,expected,"{}",row["name"]);
 }}
}
#[test]
fn cancellation_and_invalid_strokes_never_publish_candidates() {
 let rows=fixtures();let base=input(&rows[4]);
 for steps in [0,1,5,20] {let mut job=StrokeOutlineJob::new(base.clone()).unwrap();assert_eq!(job.result(),Err(StrokeOutlineError::Incomplete));for _ in 0..steps {job.advance(1).unwrap();}job.cancel();assert_eq!(job.advance(1),Err(StrokeOutlineError::Cancelled));assert_eq!(job.result(),Err(StrokeOutlineError::Cancelled));}
 for width in [-1.0,f64::NAN] {let mut bad=base.clone();bad.style.width=width;assert!(StrokeOutlineJob::new(bad).is_err());}
 let mut bad=base.clone();bad.style.dash=vec![-1.0,2.0];assert!(StrokeOutlineJob::new(bad).is_err());
 let mut bad=base.clone();bad.contours[0].points[0][0]=f64::NAN;let mut job=StrokeOutlineJob::new(bad).unwrap();assert!(job.advance(100).is_err());assert!(job.result().is_err());
 let mut job=StrokeOutlineJob::new(base).unwrap();assert!(job.advance(0).is_err());
}

#[test]
fn large_crossing_strokes_fit_coverage_and_dash_expansion_is_bounded() {
 let rows=fixtures();let mut source=input(&rows[2]);source.contours[0].points=(0..9001).map(|at|[(at%2) as f64,1.0]).collect();
 let mut job=StrokeOutlineJob::new(source).unwrap();while !job.advance(4096).unwrap().done {}let contours=job.into_result().unwrap();assert_eq!(contours.len(),9000);
 let mut coverage=CoverageJob::new(CoverageInput {width:1,height:2,transform:[1.0,0.0,0.0,1.0,0.0,0.0],rule:CoverageRule::NonZero,contours}).unwrap();while !coverage.advance(4096).unwrap().done {}assert_eq!(coverage.result().unwrap().coverage,vec![255,255]);
 let mut source=input(&rows[2]);source.style.dash=vec![0.000001,0.000001];let mut job=StrokeOutlineJob::new(source).unwrap();let mut error=None;
 for _ in 0..1000 {match job.advance(4096) {Ok(p)=>assert!(!p.done),Err(e)=>{error=Some(e);break;}}}
 assert!(matches!(error,Some(StrokeOutlineError::Invalid(message)) if message.contains("budget")));assert!(job.result().is_err());
}
