use super::*;
fn input(value:&serde_json::Value)->PathRasterInput {
 PathRasterInput {width:value["width"].as_u64().unwrap() as u32,height:value["height"].as_u64().unwrap() as u32,origin:serde_json::from_value(value["origin"].clone()).unwrap(),segments:serde_json::from_value(value["segments"].clone()).unwrap(),transform:serde_json::from_value(value["transform"].clone()).unwrap(),tolerance:value["tolerance"].as_f64().unwrap(),fill_rule:serde_json::from_value(value["fillRule"].clone()).unwrap(),fill:serde_json::from_value(value["fill"].clone()).unwrap(),stroke:serde_json::from_value(value["stroke"].clone()).unwrap()}
}
#[test]
fn painted_paths_match_every_shared_rgba_fixture_under_bounded_grants() {
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
 for row in cases.as_array().unwrap() {for grant in [1,7,4096] {
  let mut job=PathRasterJob::new(input(&row["input"])).unwrap();let mut work=0;let mut done=false;
  for _ in 0..2000000 {let p=job.advance(grant).unwrap();assert!(p.work-work<=grant as u64);work=p.work;if p.done {done=true;break;}}
  assert!(done);let expected:Vec<u8>=serde_json::from_value(row["expected"].clone()).unwrap();assert_eq!(job.result().unwrap().pixels,expected,"{} grant {grant}",row["name"]);
 }}
 eprintln!("[DEBUG] All twenty painted path masks matched Rust output under grants 1, 7 and 4096");
}
#[test]
fn painted_paths_cancel_without_publication_and_keep_sticky_failure() {
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
 for steps in [0,1,10,30,80] {let mut job=PathRasterJob::new(input(&cases[15]["input"])).unwrap();assert!(job.result().is_err());for _ in 0..steps {job.advance(1).unwrap();}job.cancel();assert!(job.advance(1).is_err());assert!(job.result().is_err());}
 for grant in [0] {assert!(PathRasterJob::new(input(&cases[0]["input"])).unwrap().advance(grant).is_err());}
 let mut invalid=input(&cases[0]["input"]);invalid.segments=vec![PathSegment::Move {to:[f64::NAN,0.0]}];let mut job=PathRasterJob::new(invalid).unwrap();assert!(job.advance(4096).is_err());assert!(job.result().is_err());assert!(job.advance(1).is_err());
}

#[test]
fn painted_paths_retire_the_actual_stroke_before_downstream_coverage() {
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
 let rows:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧹️stroke/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap(){for grant in [1,7,4096]{
  let source=cases.as_array().unwrap().iter().find(|c|c["name"]==row["source"]).unwrap();let mut job=PathRasterJob::new(input(&source["input"])).unwrap();
  let expected=row["work"]["native"].as_u64().unwrap();let mut work=0;let mut cleanup_work=0;let mut observed=false;let mut done=false;let mut published=None;
  for _ in 0..2000000 {
   let closing=job.phase==PathRasterPhase::StrokeCleanup;
   if closing {
    observed=true;assert!(job.outline.is_none());assert!(job.coverage.is_none());assert!(job.fill_mask.is_none()&&job.stroke_mask.is_none());assert!(job.result().is_err());
    let pointer=job.stroke_polygons.as_ptr();if let Some(before)=published{assert_eq!(pointer,before);}else{published=Some(pointer);}
   }
   let budget=if !closing&&observed{grant}else{1};let p=job.advance(budget).unwrap();assert!(p.work-work<=budget as u64);work=p.work;
   if closing {
    if let Some(owner)=&mut job.outline_retirement {if owner.terminal_is_empty(){cleanup_work=owner.advance(1).unwrap().work;}}
    else {assert_eq!(cleanup_work,expected);}
   }
   if p.done{done=true;break;}
  }
  assert!(done);assert_eq!(observed,expected>0);assert_eq!(cleanup_work,expected);
  let expected_pixels:Vec<u8>=serde_json::from_value(source["expected"].clone()).unwrap();assert_eq!(job.result().unwrap().pixels,expected_pixels);
  eprintln!("[DEBUG] Actual painted stroke cleanup {:?}: grant={grant} child_work={cleanup_work} RGBA matched",row["source"]);
 }}
}

#[test]
fn painted_stroke_cleanup_keeps_invalid_grants_and_interrupted_pixels_private() {
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
 let rows:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧹️stroke/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap().iter().filter(|r|r["work"]["native"].as_u64().unwrap()>0){
  let source=cases.as_array().unwrap().iter().find(|c|c["name"]==row["source"]).unwrap();let mut job=PathRasterJob::new(input(&source["input"])).unwrap();
  for _ in 0..2000000{if job.phase==PathRasterPhase::StrokeCleanup{break;}job.advance(1).unwrap();}
  assert_eq!(job.phase,PathRasterPhase::StrokeCleanup);let work=job.work;let pointer=job.stroke_polygons.as_ptr();let owner=job.outline_retirement.as_ref().unwrap() as *const _;
  assert!(job.advance(0).is_err());if usize::BITS>53{assert!(job.advance(usize::MAX).is_err());}
  assert_eq!(job.work,work);assert_eq!(job.stroke_polygons.as_ptr(),pointer);assert_eq!(job.outline_retirement.as_ref().unwrap() as *const _,owner);
  job.advance(1).unwrap();job.cancel();assert_eq!(job.advance(1),Err(PathRasterError::Cancelled));assert_eq!(job.result(),Err(PathRasterError::Cancelled));
 }
}

#[test]
fn painted_paths_consume_actual_coverage_masks_before_downstream_work() {
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let rows:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧹️coverage/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap(){for grant in [1,7,4096]{
  let source=cases.as_array().unwrap().iter().find(|c|c["name"]==row["source"]).unwrap();let mut job=PathRasterJob::new(input(&source["input"])).unwrap();let mut observed=Vec::<&str>::new();let mut active=None;let mut pointer=None;let mut cleanup_work=0;let mut work=0;let mut done=false;
  for _ in 0..2000000 {
   let closing=match job.phase {PathRasterPhase::FillCoverageCleanup=>Some("fill"),PathRasterPhase::StrokeCoverageCleanup=>Some("stroke"),_=>None};
   if let Some(kind)=closing {if active.is_none(){observed.push(kind);active=Some(kind);cleanup_work=0;}assert_eq!(active,Some(kind));assert!(job.coverage.is_none());let mask=if kind=="fill"{job.fill_mask.as_ref().unwrap()}else{job.stroke_mask.as_ref().unwrap()};if let Some(before)=pointer{assert_eq!(mask.coverage.as_ptr(),before);}else{pointer=Some(mask.coverage.as_ptr());}assert!(job.candidate.as_ref().unwrap().pixels.iter().all(|v|*v==0));assert!(job.result().is_err());}
   let budget=if job.phase==PathRasterPhase::Painting{grant}else{1};let p=job.advance(budget).unwrap();assert!(p.work-work<=budget as u64);work=p.work;
   if closing.is_some(){if let Some(owner)=job.coverage_retirement.as_mut(){if owner.terminal_is_empty(){cleanup_work=owner.advance(1).unwrap().work;}}else{assert!(cleanup_work>=14);active=None;pointer=None;}}
   if p.done{done=true;break;}
  }
  assert!(done);assert!(job.coverage_retirement.is_none());let expected:Vec<String>=serde_json::from_value(row["coverage"].clone()).unwrap();assert_eq!(observed,expected);let expected_pixels:Vec<u8>=serde_json::from_value(source["expected"].clone()).unwrap();assert_eq!(job.result().unwrap().pixels,expected_pixels);
  eprintln!("[DEBUG] Actual painted coverage handoff {:?}: grant={grant}, stages={observed:?}, RGBA matched",row["source"]);
 }}
}
