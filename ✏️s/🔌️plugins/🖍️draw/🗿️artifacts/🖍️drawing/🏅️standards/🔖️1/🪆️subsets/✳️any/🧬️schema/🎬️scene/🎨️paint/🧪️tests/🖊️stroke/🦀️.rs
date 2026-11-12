//! 🖊️ Defining stroke-union phase custody remains private to the real scene owner.
use super::*;
use semio_framework_test_fixtures::fixture_text;
#[path="../🧫️fixtures/🦀️.rs"]
mod paint_input;
use paint_input::input;
#[path="../../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🛤️path/🎟️work/🧪️tests/🦀️.rs"]
mod funding;
funding::physical_close!(PaintedPreparationRetirement,funding::PhysicalClose);
use semio_framework_value_testing_allocator as _;
fn sources()->serde_json::Value{serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🧮️geometry/🎯️picking/🎨️paint/🔣️.json")).unwrap()}
fn test_work()->funding::Wallet{let value:serde_json::Value=serde_json::from_str(include_str!("../../../🔤️text/🧪️tests/🎟️original/🔣️.json")).unwrap();let v=&value["stock"];funding::Wallet::from_grant(semio_framework_value::RetainedCloneGrant{maximum_items:v["items"].as_u64().unwrap()as usize,maximum_copy_bytes:v["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:v["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:v["release"].as_u64().unwrap()as usize,maximum_depth:v["depth"].as_u64().unwrap()as usize},2_000_000)}
#[test]
fn prepared_regions_stroke_cancel_actual_union_sources_frontiers_and_partial_output_in_every_phase(){
 let source=&sources()[6];let segments:Vec<semio_s_artifact_draw_drawing_model::PathSegment>=serde_json::from_value(source["segments"].clone()).unwrap();let phases:Vec<String>=serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🎬️scene/🎨️paint/🧹️retirement/🖊️stroke/🔣️.json")).unwrap();
 for phase in phases{for grant in [1,7,4096]{let mut wallet=test_work();let mut job=PaintedPathPrepareJob::new(input(source));let mut reached=false;for _ in 0..2000000{if job.stroke_canonical.as_ref().is_some_and(|child|child.phase==phase){reached=true;break;}wallet.turn(1,|work|job.advance(1,|at|segments.get(at).cloned(),work)).unwrap();}assert!(reached,"{phase}");let child=job.stroke_canonical.as_ref().unwrap();let owners=(child.source.len(),child.current.len(),child.next.len(),child.active.len(),child.combined.len(),child.region.contours.len(),child.segments.len(),child.child.is_some(),child.child_retirement.is_some());job.cancel();let child=job.stroke_canonical.as_ref().unwrap();assert_eq!(owners,(child.source.len(),child.current.len(),child.next.len(),child.active.len(),child.combined.len(),child.region.contours.len(),child.segments.len(),child.child.is_some(),child.child_retirement.is_some()));let(close,output)=job.into_retirement();let mut close=funding::Funded::from_parts(close,wallet);assert!(output.is_none());let mut work=0;while !close.terminal_is_empty(){let p=close.close_turn(grant).unwrap();assert!(p.work>work&&p.work-work<=grant as u64);work=p.work;}assert!(close.owner.original().is_none());close.finish();}}
}
