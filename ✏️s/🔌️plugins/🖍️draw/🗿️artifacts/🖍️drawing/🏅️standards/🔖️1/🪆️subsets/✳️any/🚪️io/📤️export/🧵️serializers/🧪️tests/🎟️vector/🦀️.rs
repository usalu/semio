//! 🎟️ Serializer fixtures retain one finite original Vector authority through paid controlled close.
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OriginalTurnFailure,OperationId,Generation,close_original_retained_step};
use semio_framework_value::{RetainedCloneGrant,RetirementDemand};
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_scene::preparation::{DocumentVectorJob,DocumentVectorRetirement,DocumentSceneLimits,DocumentAlgorithmLimits,DocumentScenePlan};
use serde_json::Value;
fn now()->Option<u64>{Some(0)}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn stock(v:&Value)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:v["items"].as_u64().unwrap()as usize,maximum_copy_bytes:v["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:v["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:v["release"].as_u64().unwrap()as usize,maximum_depth:v["depth"].as_u64().unwrap()as usize}}
pub struct VectorFixture<'a>{closing:DocumentVectorRetirement<'a>,caller:OriginalJobFixtureCaller,decoder:OriginalJobDecodeOwner,issued:RetainedCloneGrant,max_turns:u64}
impl VectorFixture<'_>{
 pub fn close(self){let Self{mut closing,mut caller,mut decoder,issued,max_turns}=self;
  for _ in 0..max_turns{if closing.terminal_is_empty(){break}let copy=closing.next_copy_byte_demand().unwrap();let release=closing.next_release_byte_demand().unwrap();let d=RetirementDemand{copy_bytes:copy,capacity_bytes:closing.next_capacity_byte_demand(if copy>0{copy}else{release}).unwrap(),release_bytes:release,depth:closing.next_depth_demand().unwrap()};
   let(result,heap)=observe(||caller.turn_with_decoder(grant(d),&mut decoder,|cx|cx.with_original_retirement(|cx|close_original_retained_step(cx,d,|g|closing.close_step(g)))));result.unwrap();assert!(!heap.overflowed);let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));
  }assert!(closing.terminal_is_empty());let((),heap)=observe(||drop(closing));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  for _ in 0..max_turns{if decoder.terminal_is_empty(){break}caller.turn(issued,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(32768)?;close_original_retained_step(cx,d,|g|decoder.close_step(g))?;Ok(())})).unwrap();}assert!(decoder.terminal_is_empty());caller.retire(issued);
 }
}
pub fn prepare<'a>(document:&'a DrawingSnapshot,limits:DocumentSceneLimits,algorithms:DocumentAlgorithmLimits)->(VectorFixture<'a>,DocumentScenePlan){
 let law:Value=serde_json::from_str(include_str!("../../../../../🧬️schema/🎬️scene/🔤️text/🧪️tests/🎟️original/🔣️.json")).unwrap();let issued=stock(&law["stock"]);let max_turns=law["maxObservedTurns"].as_u64().unwrap();let(mut job,heap)=observe(||DocumentVectorJob::new(document,limits,algorithms).unwrap());assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(law["operation"].as_u64().unwrap()),Generation(law["generation"].as_u64().unwrap()),issued,now,law["deadlineUs"].as_u64().unwrap(),law["fuel"].as_u64().unwrap());let mut decoder=OriginalJobDecodeOwner::empty();let mut done=false;let mut previous=0;
 for _ in 0..max_turns{let g=grant(job.next_work_demand());let before=caller.fuel_remaining();let(result,heap)=observe(||caller.turn_with_decoder(g,&mut decoder,|cx|Ok::<_,OriginalTurnFailure>(job.advance_original(cx))));let p=caller.receipt();assert!(!heap.overflowed);assert!(p.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert_eq!(before-caller.fuel_remaining(),job.last_turn_work());let progress=result.unwrap().unwrap();assert!(progress.work>=previous);previous=progress.work;if progress.done{done=true;break}}
 assert!(done,"funded Vector fixture failed to terminate");let(closing,plan)=job.into_retirement();(VectorFixture{closing,caller,decoder,issued,max_turns},plan.unwrap())
}
