use super::*;
use semio_framework_test_fixtures::fixture_text;
fn phase_name(phase:PathRasterPhase)->&'static str{match phase{PathRasterPhase::Preparing=>"preparing",PathRasterPhase::Flattening=>"flattening",PathRasterPhase::FlattenCleanup=>"flattenCleanup",PathRasterPhase::Contours=>"contours",PathRasterPhase::ContoursCleanup=>"contoursCleanup",PathRasterPhase::Stroke=>"stroke",PathRasterPhase::StrokeCleanup=>"strokeCleanup",PathRasterPhase::FillCoverage=>"fillCoverage",PathRasterPhase::FillCoverageCleanup=>"fillCoverageCleanup",PathRasterPhase::StrokeCoverage=>"strokeCoverage",PathRasterPhase::StrokeCoverageCleanup=>"strokeCoverageCleanup",PathRasterPhase::Painting=>"painting",PathRasterPhase::Complete=>"complete"}}
#[test]
fn whole_painted_path_retirement_retains_actual_children_private_candidates_and_complete_pixels(){
 let cases:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🧮️geometry/📷️raster/🔣️.json")).unwrap();let rows:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🧮️geometry/📷️raster/🧹️retirement/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap(){for grant in [1,7,4096]{let source=cases.as_array().unwrap().iter().find(|c|c["name"]==row["source"]).unwrap();let mut value=input(&source["input"]);let mode=row["mode"].as_str().unwrap();if mode=="failure"{value.segments=vec![PathSegment::Move{to:[0.0;2]},PathSegment::Line{to:[1e10,0.0]}];}let mut wallet=finite_raster_wallet();let mut job=PathRasterJob::new(value);
  if mode=="failure"{assert!(raster_turn(&mut job,&mut wallet,4096).is_err());assert!(job.failed.is_some());assert!(job.flatten.is_some());}else{let mut reached=false;for _ in 0..2000000{if phase_name(job.phase)==row["phase"].as_str().unwrap(){reached=true;break;}raster_turn(&mut job,&mut wallet,1).unwrap();}assert!(reached);let steps=row["steps"].as_u64().unwrap()as usize;if steps>0{raster_turn(&mut job,&mut wallet,steps).unwrap();}}
  let pointer=job.candidate.as_ref().unwrap().pixels.as_ptr();let sample=if mode=="published"{Some(job.result().unwrap().pixels.clone())}else{None};let flatten=job.flatten.is_some()||job.flatten_retirement.is_some();let outline=job.outline.is_some()||job.outline_retirement.is_some();let coverage=job.coverage.is_some()||job.coverage_retirement.is_some();let fill=job.fill.is_some();
  if mode=="cancelled"||mode=="published"{job.cancel();assert_eq!(job.candidate.as_ref().unwrap().pixels.as_ptr(),pointer);assert_eq!(job.flatten.is_some()||job.flatten_retirement.is_some(),flatten);assert_eq!(job.outline.is_some()||job.outline_retirement.is_some(),outline);assert_eq!(job.coverage.is_some()||job.coverage_retirement.is_some(),coverage);assert_eq!(job.fill.is_some(),fill);}
  let(mut retired,output)=job.into_retirement();let state=retired.owner.original().unwrap();assert!(state.flatten.is_none()&&state.outline.is_none()&&state.coverage.is_none()&&state.fill.is_none());assert_eq!(state.flatten_retirement.is_some(),flatten);assert_eq!(state.outline_retirement.is_some(),outline);assert_eq!(state.coverage_retirement.is_some(),coverage);assert_eq!(state.fill_retirement.is_some(),fill);assert_eq!(output.is_some(),row["phase"]=="complete"&&mode=="live");if let Some(image)=&output{assert_eq!(image.pixels.as_ptr(),pointer);assert_eq!(serde_json::to_value(&image.pixels).unwrap(),source["expected"]);}else{assert_eq!(state.candidate.as_ref().unwrap().pixels.as_ptr(),pointer);}let(zero,h)=semio_framework_trace::observe_heap_allocations_on_this_thread(||retired.close_step(Default::default()).unwrap());assert_eq!(zero.progress(),Default::default());assert_eq!((h.requested_bytes,h.released_bytes),(0,0));
  let mut work=0;let mut done=false;for _ in 0..2000000{let before=work;for _ in 0..grant{if retired.terminal_is_empty(){break}let d=path_raster_close_demand(&retired);let p=wallet.close_turn(d,|g|retired.close_step(g)).unwrap().progress();assert_ne!(p,Default::default());work+=p.copied_items as u64;}assert!(work>=before&&work-before<=grant as u64);if retired.terminal_is_empty(){done=true;break;}}
  assert!(done&&retired.terminal_is_empty());assert!(work>=16);assert!(retired.owner.original().is_none());let stable=retired.close_step(Default::default()).unwrap();assert!(matches!(stable,semio_framework_value::RetainedCloneStep::Complete(_)));assert_eq!(stable.progress(),Default::default());wallet.close_owned(retired);if let Some(image)=output{assert_eq!(serde_json::to_value(&image.pixels).unwrap(),source["expected"]);wallet.close_owned(image);}if let Some(sample)=sample{assert_eq!(serde_json::to_value(sample).unwrap(),source["expected"]);}
 }}
}
#[test]
fn painted_paths_consume_real_flattened_contours_before_conversion(){
 let cases:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🧮️geometry/📷️raster/🔣️.json")).unwrap();let rows:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🧮️geometry/📷️raster/🧹️flatten/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap(){for grant in [1,7,4096]{let source=cases.as_array().unwrap().iter().find(|c|c["name"]==row["source"]).unwrap();let mut wallet=finite_raster_wallet();let mut job=PathRasterJob::new(input(&source["input"]));let mut observed=false;let mut child_work=0;let mut pointer=None;let mut done=false;
  for _ in 0..2000000{let closing=job.phase==PathRasterPhase::FlattenCleanup;if closing{observed=true;assert!(job.flatten.is_none());assert!(job.fill_contours.is_empty()&&job.stroke_contours.is_empty());assert!(job.coverage.is_none());assert!(job.result().is_err());if let Some(before)=pointer{assert_eq!(job.flat.as_ptr(),before);}else{pointer=Some(job.flat.as_ptr());}}
   let budget=if observed&&!closing{grant}else{1};let before=job.work;let p=raster_turn(&mut job,&mut wallet,budget).unwrap();assert!(p.work-before<=budget as u64);if closing{if let Some(owner)=job.flatten_retirement.as_mut(){if owner.terminal_is_empty(){child_work=job.cleanup_work;}}else{assert!(child_work>0);}}if p.done{done=true;break;}}
  assert!(done&&observed);assert!(job.flatten_retirement.is_none());assert!(child_work>0);let expected:Vec<u8>=serde_json::from_value(source["expected"].clone()).unwrap();assert_eq!(job.result().unwrap().pixels,expected);finish_raster(job,&mut wallet);
 }}
}
fn input(value:&serde_json::Value)->PathRasterInput {
 PathRasterInput {width:value["width"].as_u64().unwrap() as u32,height:value["height"].as_u64().unwrap() as u32,origin:serde_json::from_value(value["origin"].clone()).unwrap(),segments:serde_json::from_value(value["segments"].clone()).unwrap(),transform:serde_json::from_value(value["transform"].clone()).unwrap(),tolerance:value["tolerance"].as_f64().unwrap(),fill_rule:serde_json::from_value(value["fillRule"].clone()).unwrap(),fill:serde_json::from_value(value["fill"].clone()).unwrap(),stroke:serde_json::from_value(value["stroke"].clone()).unwrap()}
}
#[test]
fn painted_paths_match_every_shared_rgba_fixture_under_bounded_grants() {
 let cases:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🧮️geometry/📷️raster/🔣️.json")).unwrap();
 for row in cases.as_array().unwrap() {for grant in [1,7,4096] {
  let mut wallet=finite_raster_wallet();let mut job=PathRasterJob::new(input(&row["input"]));let mut work=0;let mut done=false;
  for _ in 0..2000000 {let p=raster_turn(&mut job,&mut wallet,grant).unwrap();assert!(p.work-work<=grant as u64);work=p.work;if p.done {done=true;break;}}
  assert!(done);let expected:Vec<u8>=serde_json::from_value(row["expected"].clone()).unwrap();assert_eq!(job.result().unwrap().pixels,expected,"{} grant {grant}",row["name"]);finish_raster(job,&mut wallet);
 }}
}
#[test]
fn painted_paths_cancel_without_publication_and_keep_sticky_failure() {
 let cases:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🧮️geometry/📷️raster/🔣️.json")).unwrap();
 for steps in [0,1,10,30,80] {let mut wallet=finite_raster_wallet();let mut job=PathRasterJob::new(input(&cases[15]["input"]));assert!(job.result().is_err());for _ in 0..steps {raster_turn(&mut job,&mut wallet,1).unwrap();}job.cancel();assert!(raster_turn(&mut job,&mut wallet,1).is_err());assert!(job.result().is_err());finish_raster(job,&mut wallet);}
 for grant in [0] {let mut wallet=finite_raster_wallet();let mut job=PathRasterJob::new(input(&cases[0]["input"]));assert!(raster_turn(&mut job,&mut wallet,grant).is_err());finish_raster(job,&mut wallet);}
 let mut invalid=input(&cases[0]["input"]);invalid.segments=vec![PathSegment::Move {to:[f64::NAN,0.0]}];let mut wallet=finite_raster_wallet();let mut job=PathRasterJob::new(invalid);assert!(raster_turn(&mut job,&mut wallet,4096).is_err());assert!(job.result().is_err());assert!(raster_turn(&mut job,&mut wallet,1).is_err());finish_raster(job,&mut wallet);
}

#[test]
fn painted_paths_retire_the_actual_stroke_before_downstream_coverage() {
 let cases:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🧮️geometry/📷️raster/🔣️.json")).unwrap();
 let rows:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🧮️geometry/📷️raster/🧹️stroke/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap(){for grant in [1,7,4096]{
  let source=cases.as_array().unwrap().iter().find(|c|c["name"]==row["source"]).unwrap();let mut wallet=finite_raster_wallet();let mut job=PathRasterJob::new(input(&source["input"]));
  let expected=row["work"]["native"].as_u64().unwrap();let mut work=0;let mut cleanup_work=0;let mut observed=false;let mut done=false;let mut published=None;
  for _ in 0..2000000 {
   let closing=job.phase==PathRasterPhase::StrokeCleanup;
   if closing {
    observed=true;assert!(job.outline.is_none());assert!(job.coverage.is_none());assert!(job.fill_mask.is_none()&&job.stroke_mask.is_none());assert!(job.result().is_err());
    let pointer=job.stroke_polygons.as_ptr();if let Some(before)=published{assert_eq!(pointer,before);}else{published=Some(pointer);}
   }
   let budget=if !closing&&observed{grant}else{1};let p=raster_turn(&mut job,&mut wallet,budget).unwrap();assert!(p.work-work<=budget as u64);work=p.work;
   if closing {
    if let Some(owner)=&mut job.outline_retirement {if owner.terminal_is_empty(){cleanup_work=job.cleanup_work;}}
    else {assert_eq!(cleanup_work,expected);}
   }
   if p.done{done=true;break;}
  }
  assert!(done);assert_eq!(observed,expected>0);assert_eq!(cleanup_work,expected);
  let expected_pixels:Vec<u8>=serde_json::from_value(source["expected"].clone()).unwrap();assert_eq!(job.result().unwrap().pixels,expected_pixels);finish_raster(job,&mut wallet);
 }}
}

#[test]
fn painted_stroke_cleanup_keeps_invalid_grants_and_interrupted_pixels_private() {
 let cases:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🧮️geometry/📷️raster/🔣️.json")).unwrap();
 let rows:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🧮️geometry/📷️raster/🧹️stroke/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap().iter().filter(|r|r["work"]["native"].as_u64().unwrap()>0){
  let source=cases.as_array().unwrap().iter().find(|c|c["name"]==row["source"]).unwrap();let mut wallet=finite_raster_wallet();let mut job=PathRasterJob::new(input(&source["input"]));
  for _ in 0..2000000{if job.phase==PathRasterPhase::StrokeCleanup{break;}raster_turn(&mut job,&mut wallet,1).unwrap();}
  assert_eq!(job.phase,PathRasterPhase::StrokeCleanup);let work=job.work;let pointer=job.stroke_polygons.as_ptr();let owner=job.outline_retirement.as_ref().unwrap() as *const _;
  assert!(raster_turn(&mut job,&mut wallet,0).is_err());if usize::BITS>53{assert!(raster_turn(&mut job,&mut wallet,usize::MAX).is_err());}
  assert_eq!(job.work,work);assert_eq!(job.stroke_polygons.as_ptr(),pointer);assert_eq!(job.outline_retirement.as_ref().unwrap() as *const _,owner);
  raster_turn(&mut job,&mut wallet,1).unwrap();job.cancel();assert_eq!(raster_turn(&mut job,&mut wallet,1),Err(PathRasterError::Cancelled));assert_eq!(job.result(),Err(PathRasterError::Cancelled));finish_raster(job,&mut wallet);
 }
}

#[test]
fn painted_paths_consume_actual_coverage_masks_before_downstream_work() {
 let cases:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🧮️geometry/📷️raster/🔣️.json")).unwrap();let rows:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🧮️geometry/📷️raster/🧹️coverage/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap(){for grant in [1,7,4096]{
  let source=cases.as_array().unwrap().iter().find(|c|c["name"]==row["source"]).unwrap();let mut wallet=finite_raster_wallet();let mut job=PathRasterJob::new(input(&source["input"]));let mut observed=Vec::<&str>::new();let mut active=None;let mut pointer=None;let mut cleanup_work=0;let mut work=0;let mut done=false;
  for _ in 0..2000000 {
   let closing=match job.phase {PathRasterPhase::FillCoverageCleanup=>Some("fill"),PathRasterPhase::StrokeCoverageCleanup=>Some("stroke"),_=>None};
   if let Some(kind)=closing {if active.is_none(){observed.push(kind);active=Some(kind);cleanup_work=0;}assert_eq!(active,Some(kind));assert!(job.coverage.is_none());let mask=if kind=="fill"{job.fill_mask.as_ref().unwrap()}else{job.stroke_mask.as_ref().unwrap()};if let Some(before)=pointer{assert_eq!(mask.coverage.as_ptr(),before);}else{pointer=Some(mask.coverage.as_ptr());}assert!(job.candidate.as_ref().unwrap().pixels.iter().all(|v|*v==0));assert!(job.result().is_err());}
   let budget=if job.phase==PathRasterPhase::Painting{grant}else{1};let p=raster_turn(&mut job,&mut wallet,budget).unwrap();assert!(p.work-work<=budget as u64);work=p.work;
   if closing.is_some(){if let Some(owner)=job.coverage_retirement.as_mut(){if owner.terminal_is_empty(){cleanup_work=job.cleanup_work;}}else{assert!(cleanup_work>=14);active=None;pointer=None;}}
   if p.done{done=true;break;}
  }
  assert!(done);assert!(job.coverage_retirement.is_none());let expected:Vec<String>=serde_json::from_value(row["coverage"].clone()).unwrap();assert_eq!(observed,expected);let expected_pixels:Vec<u8>=serde_json::from_value(source["expected"].clone()).unwrap();assert_eq!(job.result().unwrap().pixels,expected_pixels);finish_raster(job,&mut wallet);
 }}
}

#[path="../../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🛤️path/🎟️work/🧪️tests/🦀️.rs"] mod raster_wallet;
fn raster_funding_fixture()->serde_json::Value{serde_json::from_str(include_str!("../../🎟️work/🧫️fixtures/🔣️.json")).unwrap()}
fn finite_raster_wallet()->raster_wallet::Wallet{let v=raster_funding_fixture();let s=&v["stock"];let n=|key|s[key].as_u64().unwrap()as usize;raster_wallet::Wallet::from_grant(semio_framework_value::RetainedCloneGrant{maximum_items:n("items"),maximum_copy_bytes:n("copy"),maximum_capacity_bytes:n("capacity"),maximum_release_bytes:n("release"),maximum_depth:n("depth")},v["maxTurns"].as_u64().unwrap()as usize)}
fn funded_rectangle()->PathRasterInput{let v=raster_funding_fixture();let w=v["width"].as_u64().unwrap()as u32;let h=v["height"].as_u64().unwrap()as u32;PathRasterInput{width:w,height:h,origin:[0.;2],segments:vec![PathSegment::Move{to:[0.;2]},PathSegment::Line{to:[w as f64,0.]},PathSegment::Line{to:[w as f64,h as f64]},PathSegment::Line{to:[0.,h as f64]},PathSegment::Close],transform:[1.,0.,0.,1.,0.,0.],tolerance:0.1,fill_rule:FillRule::Nonzero,fill:Some(FillStyle::Solid{color:serde_json::from_value(v["color"].clone()).unwrap()}),stroke:None}}
#[test]
fn path_raster_original_storage_and_lower_jobs_share_one_finite_stock(){
 use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;use semio_framework_value::work::RetainedWorkLedger;
 let v=raster_funding_fixture();let original=funded_rectangle();let pointer=original.segments.as_ptr();let(mut job,h)=observe(||PathRasterJob::new(original));assert!(!h.overflowed);assert_eq!((h.requested_bytes,h.released_bytes),(0,0));assert_eq!(job.source.as_ptr(),pointer);let mut wallet=finite_raster_wallet();let mut done=false;
 for _ in 0..v["maxTurns"].as_u64().unwrap(){let d=job.next_work_demand();for axis in v["axes"].as_array().unwrap(){let axis=axis.as_str().unwrap();if match axis{"items"=>false,"copy"=>d.copy_bytes==0,"capacity"=>d.capacity_bytes==0,"release"=>d.release_bytes==0,"depth"=>d.depth==0,_=>unreachable!()}{continue}let mut g=wallet.remaining();g.maximum_items=1;match axis{"items"=>g.maximum_items=0,"copy"=>g.maximum_copy_bytes=d.copy_bytes-1,"capacity"=>g.maximum_capacity_bytes=d.capacity_bytes-1,"release"=>g.maximum_release_bytes=d.release_bytes-1,"depth"=>g.maximum_depth=d.depth-1,_=>unreachable!()}let mut ledger=RetainedWorkLedger::new(g);let before=job.progress();let(result,h)=observe(||job.advance(1,&mut ledger));assert_eq!(result.unwrap(),before);assert_eq!(job.last_turn_work(),0);assert_eq!(job.last_turn_semantic_work(),0);assert_eq!(ledger.receipt(),Default::default());assert_eq!((h.requested_bytes,h.released_bytes),(0,0));}
 let p=wallet.turn(1,|work|job.advance(1,work)).unwrap();if p.done{done=true;break}assert!(job.last_turn_work()>0,"finite raster stock exhausted");}assert!(done);let expected:Vec<u8>=serde_json::from_value(v["expected"].clone()).unwrap();assert_eq!(job.result().unwrap().pixels,expected);let mut reference=tiny_skia::Pixmap::new(8,8).unwrap();let mut path=tiny_skia::PathBuilder::new();path.push_rect(tiny_skia::Rect::from_xywh(0.,0.,8.,8.).unwrap());let mut paint=tiny_skia::Paint::default();paint.set_color_rgba8(255,0,0,255);reference.fill_path(&path.finish().unwrap(),&paint,tiny_skia::FillRule::Winding,tiny_skia::Transform::identity(),None);assert_eq!(job.result().unwrap().pixels,reference.data());let pointer=job.result().unwrap().pixels.as_ptr();let((owner,output),h)=observe(||job.into_retirement());assert_eq!((h.requested_bytes,h.released_bytes),(0,0));let output=output.unwrap();assert_eq!(output.pixels.as_ptr(),pointer);wallet.close_owned(owner);wallet.close_owned(output);
}
#[test]
fn invalid_owned_raster_and_cached_errors_keep_actual_custody(){
 use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
 let mut input=funded_rectangle();input.width=0;let pointer=input.segments.as_ptr();let(mut job,h)=observe(||PathRasterJob::new(input));assert_eq!((h.requested_bytes,h.released_bytes),(0,0));assert_eq!(job.source.as_ptr(),pointer);let mut wallet=finite_raster_wallet();assert!(wallet.turn(1,|work|job.advance(1,work)).is_err());assert_eq!(job.last_turn_work(),1);assert_eq!(job.last_turn_semantic_work(),1);let mut zero=semio_framework_value::work::RetainedWorkLedger::new(Default::default());let(result,h)=observe(||job.advance(1,&mut zero));assert!(result.is_err());assert_eq!(job.last_turn_work(),0);assert_eq!(job.last_turn_semantic_work(),0);assert_eq!((h.requested_bytes,h.released_bytes),(0,0));let((owner,output),h)=observe(||job.into_retirement());assert!(output.is_none());assert_eq!((h.requested_bytes,h.released_bytes),(0,0));wallet.close_owned(owner);
}

#[test]
fn raster_owned_retirement_failure_is_staged_before_shared_publication(){
 use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;use semio_framework_value::{ValueError,ValueRefusalKind,work::RetainedWorkLedger};
 let mut job=PathRasterJob::new(funded_rectangle());let original=ValueError::new(ValueRefusalKind::AllocationFailed,"retained cleanup cause 🖍️");let pointer=original.message.as_ptr();let((),h)=observe(||job.stage_close_error(original));assert_eq!((h.requested_bytes,h.released_bytes),(0,0));assert_eq!(job.pending_close_error.as_ref().unwrap().message.as_ptr(),pointer);let mut zero=RetainedWorkLedger::new(Default::default());let(result,h)=observe(||job.advance(1,&mut zero));assert!(result.is_ok());assert_eq!(job.last_turn_work(),0);assert_eq!((h.requested_bytes,h.released_bytes),(0,0));let mut wallet=finite_raster_wallet();let mut failed=false;for _ in 0..100{if let Err(error)=wallet.turn(1,|work|job.advance(1,work)){assert_eq!(error.to_string(),"retained cleanup cause 🖍️");let(message,h)=observe(||error.message());assert_eq!(message,"retained cleanup cause 🖍️");assert_eq!((h.requested_bytes,h.released_bytes),(0,0));failed=true;break}}assert!(failed);let(result,h)=observe(||job.advance(1,&mut zero));assert!(result.is_err());assert_eq!(job.last_turn_work(),0);assert_eq!((h.requested_bytes,h.released_bytes),(0,0));wallet.close_owned(job.into_retirement().0);
}

fn raster_turn(job:&mut PathRasterJob,wallet:&mut raster_wallet::Wallet,items:usize)->Result<PathRasterProgress,PathRasterError>{wallet.turn(items.min(4096),|ledger|job.advance(items,ledger))}
fn finish_raster(job:PathRasterJob,wallet:&mut raster_wallet::Wallet){let(owner,output)=job.into_retirement();wallet.close_owned(owner);if let Some(output)=output{wallet.close_owned(output)}}
fn path_raster_close_demand(owner:&PathRasterRetirement)->semio_framework_value::RetirementDemand{let copy_bytes=owner.next_copy_byte_demand().unwrap();let release_bytes=owner.next_release_byte_demand().unwrap();semio_framework_value::RetirementDemand{copy_bytes,capacity_bytes:owner.next_capacity_byte_demand(if copy_bytes>0{copy_bytes}else{release_bytes}).unwrap(),release_bytes,depth:owner.next_depth_demand().unwrap()}}

#[test]
fn uniquely_owned_flattened_points_move_directly_into_the_first_paint_consumer(){
 let law=raster_funding_fixture();for mode in law["ownershipModes"].as_array().unwrap(){let mode=mode.as_str().unwrap();let mut input=funded_rectangle();if mode!="fill"{input.stroke=Some(serde_json::from_value(law["stroke"].clone()).unwrap())}if mode=="stroke"{input.fill=None}let mut job=PathRasterJob::new(input);let mut wallet=finite_raster_wallet();let mut reached=false;for _ in 0..law["maxTurns"].as_u64().unwrap(){if job.phase==PathRasterPhase::Contours&&!job.flat.is_empty(){reached=true;break}raster_turn(&mut job,&mut wallet,1).unwrap();}assert!(reached);let pointer=job.flat[0].points.as_ptr();let capacity=job.flat[0].points.capacity();let mut moved=false;
 for _ in 0..100{let(_,receipt)=wallet.turn(1,|work|{let p=job.advance(1,work).unwrap();(p,work.receipt())});let points=if mode=="stroke"{job.stroke_contours.first().map(|contour|&contour.points)}else{job.fill_contours.first()};if let Some(points)=points{assert_eq!(points.as_ptr(),pointer);assert_eq!(points.capacity(),capacity);assert_eq!(job.flat[0].points.capacity(),0);assert_eq!(receipt.copied_bytes,if mode=="stroke"{std::mem::size_of::<StrokeContour>()}else{std::mem::size_of::<Vec<[f64;2]>>()});assert_eq!(receipt.retained_capacity_bytes,0);assert_eq!(receipt.released_bytes,0);moved=true;break;}}assert!(moved);let mut done=false;for _ in 0..law["maxTurns"].as_u64().unwrap(){if raster_turn(&mut job,&mut wallet,1).unwrap().done{done=true;break}}assert!(done);finish_raster(job,&mut wallet);
 }
}
