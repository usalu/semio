//! 🎟️ The real original context carries one finite vector ledger and a separate semantic fuel receipt.
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OriginalTurnFailure,OperationId,Generation,close_original_retained_step};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_scene::preparation::{DocumentVectorJob,DocumentSceneLimits,DocumentAlgorithmLimits,DocumentSceneContent};
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use serde_json::Value;
fn now()->Option<u64>{Some(0)}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn stock(v:&Value)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:v["items"].as_u64().unwrap()as usize,maximum_copy_bytes:v["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:v["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:v["release"].as_u64().unwrap()as usize,maximum_depth:v["depth"].as_u64().unwrap()as usize}}
fn delta(a:RetainedCloneProgress,b:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:b.copied_items-a.copied_items,copied_bytes:b.copied_bytes-a.copied_bytes,retained_capacity_bytes:b.retained_capacity_bytes-a.retained_capacity_bytes,released_bytes:b.released_bytes-a.released_bytes}}
fn close_owned<T:RetireOwned>(value:T,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){
 let mut owner=ControlledRetirement::retain(value);
 for _ in 0..1_000_000{if owner.terminal_is_empty(){break}let d=owner.next_demand(caller.remaining_grant().maximum_copy_bytes).unwrap();let g=grant(d);let(result,heap)=observe(||caller.turn_with_decoder(g,decoder,|cx|cx.with_original_retirement(|cx|close_original_retained_step(cx,d,|g|owner.step(g)))));result.unwrap();let p=caller.receipt();assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));}
 assert!(owner.terminal_is_empty());
}
#[test]
fn original_vector_font_turn_posts_actual_receipt_and_cpu_once_on_success_error_and_block(){
 let law:Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 let rows:Value=serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🎬️scene/📋️prepare/🔣️.json")).unwrap();
 let source=rows.as_array().unwrap().iter().find(|row|row["name"]==law["source"]).unwrap();
 for case in law["cases"].as_array().unwrap(){
  let mut raw=source["document"].clone();if let Some(text)=case["content"].as_str(){raw["layers"][0]["content"]=text.into();}
  let document:DrawingSnapshot=serde_json::from_value(raw).unwrap();let v=&source["limits"];let limits=DocumentSceneLimits{max_nodes:v["maxNodes"].as_u64().unwrap()as usize,max_depth:v["maxDepth"].as_u64().unwrap()as usize,max_segments:v["maxSegments"].as_u64().unwrap()as usize,max_references:v["maxReferences"].as_u64().unwrap()as usize,max_source_bytes:v["maxSourceBytes"].as_u64().unwrap()as usize};
  let mut job=DocumentVectorJob::new(&document,limits,DocumentAlgorithmLimits::default()).unwrap();
  let issued=stock(&law["stock"]);let mut caller=OriginalJobFixtureCaller::admit(OperationId(law["operation"].as_u64().unwrap()),Generation(law["generation"].as_u64().unwrap()),issued,now,law["deadlineUs"].as_u64().unwrap(),law["fuel"].as_u64().unwrap());let mut decoder=OriginalJobDecodeOwner::empty();
  for denied in law["originalBlocked"].as_array().unwrap(){let fuel=caller.fuel_remaining();let demand=job.next_work_demand();let mut pending=None;let(result,heap)=observe(||caller.turn_with_decoder(issued,&mut decoder,|cx|cx.with_original_work_loan(denied["fuel"].as_u64().unwrap(),denied["deadlineUs"].as_u64().unwrap(),|same|{pending=Some(job.advance_original(same));Ok(())})));result.unwrap();let progress=pending.unwrap().unwrap();let zero=&denied["expectedReceipt"];assert_eq!([caller.receipt().copied_items,caller.receipt().copied_bytes,caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes],std::array::from_fn::<_,4,_>(|i|zero[i].as_u64().unwrap()as usize));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(fuel-caller.fuel_remaining(),denied["expectedFuel"].as_u64().unwrap());assert_eq!(progress.work,denied["expectedWork"].as_u64().unwrap());assert!(!progress.done);assert_eq!(job.last_turn_work(),0);assert_eq!(job.last_turn_semantic_work(),0);assert_eq!(job.next_work_demand(),demand);}
  let mut denied=[false;4];let mut previous_work=0;let mut completed=false;let mut fault=None;let mut capacity=0;let mut cpu=0;let mut paid_repeat=false;let mut final_error=None;
  for _ in 0..law["maxObservedTurns"].as_u64().unwrap(){
   let d=job.next_work_demand();let g=grant(d);
   for axis in 0..4{if denied[axis]||match axis{0=>false,1=>d.copy_bytes==0,2=>d.capacity_bytes==0,_=>d.depth==0}{continue}let mut no=g;match axis{0=>no.maximum_items=0,1=>no.maximum_copy_bytes-=1,2=>no.maximum_capacity_bytes-=1,_=>no.maximum_depth-=1}
    let before_fuel=caller.fuel_remaining();let(result,heap)=observe(||caller.turn_with_decoder(no,&mut decoder,|cx|Ok::<_,OriginalTurnFailure>(job.advance_original(cx))));let progress=result.unwrap().unwrap();assert!(!heap.overflowed);assert_eq!(caller.receipt(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(caller.fuel_remaining(),before_fuel);assert_eq!(job.last_turn_work(),0);assert_eq!(progress.work,previous_work);assert!(!progress.done);denied[axis]=true;
   }
   let before_fuel=caller.fuel_remaining();let(result,heap)=observe(||caller.turn_with_decoder(g,&mut decoder,|cx|{
    let before=cx.retained_progress();let fuel=cx.fuel_remaining();let result=job.advance_original(cx);let posted=delta(before,cx.retained_progress());assert!(posted.fits(g));assert_eq!(fuel-cx.fuel_remaining(),job.last_turn_work());
    if result.as_ref().is_ok_and(|p|!p.done)&&posted.copied_items==g.maximum_items&&!paid_repeat{let before=cx.retained_progress();let fuel=cx.fuel_remaining();let again=job.advance_original(cx).unwrap();assert!(!again.done);assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);assert_eq!(job.last_turn_work(),0);paid_repeat=true;}
    Ok::<_,OriginalTurnFailure>(result)
   }));let p=caller.receipt();assert!(!heap.overflowed);assert!(p.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));capacity+=p.retained_capacity_bytes;cpu+=before_fuel-caller.fuel_remaining();
   match result.unwrap(){Ok(progress)=>{assert!(progress.work>=previous_work);previous_work=progress.work;if progress.done{completed=true;break}},Err(error)=>{let expected=&case["finalError"];assert!(p.copied_items>=expected["minimumCopiedItems"].as_u64().unwrap()as usize);let turn_cpu=before_fuel-caller.fuel_remaining();assert!(turn_cpu>=expected["minimumCpu"].as_u64().unwrap());assert!(matches!(error.primary(),Some(semio_s_artifact_draw_drawing_scene::preparation::DocumentSceneError::Invalid(message))if message==expected["message"].as_str().unwrap()));assert!(error.receipt_refusal().is_none()&&error.context_refusal().is_none());assert_eq!(error.retained_progress(),p);final_error=Some((p.copied_items,p.copied_bytes,p.retained_capacity_bytes,p.released_bytes,turn_cpu,heap.requested_bytes,heap.released_bytes));fault=Some(error);break}}
  }
  assert!(denied.into_iter().all(|v|v));assert!(paid_repeat);assert!(capacity>0&&cpu>0);assert_eq!(completed,case["result"]=="complete");assert_eq!(fault.is_some(),case["result"]=="error");assert_eq!(final_error.is_some(),case["result"]=="error");
  let(mut closing,output)=job.into_retirement();assert_eq!(output.is_some(),completed);if let Some(output)=&output{assert!(output.nodes.iter().any(|n|matches!(&n.content,DocumentSceneContent::Glyphs{segments,..}if !segments.is_empty())));}
  for _ in 0..1_000_000{if closing.terminal_is_empty(){break}let copy=closing.next_copy_byte_demand().unwrap();let release=closing.next_release_byte_demand().unwrap();let d=RetirementDemand{copy_bytes:copy,capacity_bytes:closing.next_capacity_byte_demand(if copy>0{copy}else{release}).unwrap(),release_bytes:release,depth:closing.next_depth_demand().unwrap()};caller.turn_with_decoder(grant(d),&mut decoder,|cx|cx.with_original_retirement(|cx|close_original_retained_step(cx,d,|g|closing.close_step(g)))).unwrap();}
  assert!(closing.terminal_is_empty());let((),heap)=observe(||drop(closing));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));close_owned((output,fault,document),&mut caller,&mut decoder);while !decoder.terminal_is_empty(){caller.turn(issued,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(32768)?;close_original_retained_step(cx,d,|g|decoder.close_step(g))?;Ok(())})).unwrap();}
  caller.retire(issued);
 }
}
