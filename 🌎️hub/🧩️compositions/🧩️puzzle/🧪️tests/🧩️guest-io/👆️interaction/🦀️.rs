//! 👆️ Real Puzzle lifecycle receipts and paged history commands prove interaction after cancellation.
use super::driver::Actor;
use semio_framework::kernel::*;
use semio_framework_actor::{OriginalJobTurnLoan,RetainedTurnInput};
use semio_framework_job::{StepContext};
use semio_framework_value::{DslValue,RetainedCloneProgress};
use semio_plugin_host::GuestRuntime;
use semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;

fn ready<T>(future:impl std::future::Future<Output=T>)->T{semio_framework_async::poll::resolve_ready(future)}
fn turns(law:&serde_json::Value)->u64{law["execution"]["maximumTurns"].as_u64().unwrap()}
fn budget(cx:&StepContext<'_>,input:RetainedTurnInput)->Budget{
 let lane=semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::UserVisible);
 Budget{retained:input,retained_policy:cx.retained_grant(),work_fuel:cx.fuel_remaining(),fuel:cx.fuel_remaining(),deadline_us:cx.deadline_us(),max_effects:lane.max_effects,max_patch_bytes:lane.max_patch_bytes,max_frames:lane.max_frames}
}
fn retire_command(actor:&mut Actor,law:&serde_json::Value,command:protocol::AppCommand){
 let mut owner=protocol::DecodedAppCommandOwner::new(command);
 for _ in 0..turns(law){if owner.terminal_is_empty(){return}actor.turn(|_,_,cx|cx.with_original_retirement(|cx|owner.close_original_step(cx)));}
 panic!("actual command source did not reach paid terminal retirement")
}
fn retire_turn(actor:&mut Actor,law:&serde_json::Value,mut result:TurnResult){
 assert!(result.ui_patch_receipt.is_none()&&result.ui_patches.is_empty(),"headless law has no visible surface publication to acknowledge");
 let mut patches_closed=false;
 for _ in 0..turns(law){
  let complete=actor.turn(|_,_,cx|{
   let grant=cx.retained_grant();let step=result.ui_patches.close_step_with_grant(grant.maximum_items,grant.maximum_release_bytes).expect("actual turn patch retirement");
   cx.consume_retained(RetainedCloneProgress{copied_items:step.released_items,released_bytes:step.released_bytes,..Default::default()})?;
   cx.consume_fuel(u64::from(step.progressed));Ok(step.complete)
  });if complete{patches_closed=true;break}
 }
 assert!(patches_closed,"actual patch page must reach terminal retirement");
 assert!(result.presence.is_empty()&&result.presence.capacity()==0,"headless history law must not silently discard presence");
 actor.retire(law,result.effects);
 match result.status{TurnStatus::Idle|TurnStatus::MoreWork=>{},other=>panic!("actual Puzzle actor turn failed: {other:?}")}
}
fn poll(actor:&mut Actor,events:&[Event],identity:&mut EntityIdentityAuthority<'_>)->TurnResult{
 actor.turn(|runtime,instance,cx|{
  let input=RetainedTurnInput::borrow_original(cx)?;let mut loan=OriginalJobTurnLoan::new();
  let held=loan.checkout(input,cx)?.expect("funded original actor command crossing");
  let result=ready(runtime.execute_turn(instance,events,budget(cx,input),identity)).expect("actual Puzzle actor executes within its original turn");
  result.retained_receipt.validate_for(input)?;
  assert!(result.fuel_used<=held.fuel,"actual interpreter execution must fit its held turn");
  loan.receive(result.retained_receipt.input,result.retained_receipt.spent,result.retained_receipt.work_units_spent,cx)?;
  assert!(loan.terminal_is_empty());Ok(result)
 })
}
fn context(actor:&mut Actor,law:&serde_json::Value,identity:&mut EntityIdentityAuthority<'_>,closing:bool){
 for _ in 0..turns(law){
  let complete=actor.turn(|runtime,instance,cx|{
   let input=RetainedTurnInput::borrow_original(cx)?;let mut loan=OriginalJobTurnLoan::new();loan.checkout(input,cx)?.expect("funded original actor context crossing");
   let complete=if closing{
    let reply=ready(runtime.close_actor_context(instance,input,budget(cx,input),identity)).expect("actual actor context closure");
    loan.receive(reply.input,reply.spent,reply.work_units_spent,cx)?;assert!(reply.refusal.is_none());assert!(!reply.blocked,"all actual child loans must return before actor context closes");reply.complete
   }else{
    let reply=ready(runtime.admit_actor_context(instance,input,budget(cx,input),identity)).expect("actual actor context admission");
    loan.receive(reply.input,reply.spent,reply.work_units_spent,cx)?;assert!(reply.refusal.is_none());reply.ready
   };assert!(loan.terminal_is_empty());Ok(complete)
  });if complete{return}
 }
 panic!("actual actor context did not reach its paid terminal state")
}
pub struct Session{lifetime:ActorInstanceLifetime,initial_alternative:Option<String>,instance:u32}
fn retire_driver(actor:&mut Actor,driver:CommandBatchDriver,release_ceiling:usize){
 assert!(driver.terminal_is_empty());let mut owner=Some(driver);
 actor.turn(|_,_,cx|{
  let grant=cx.retained_grant();assert!(grant.maximum_items>0&&grant.maximum_depth>0&&grant.maximum_release_bytes>=release_ceiling&&!cx.retirement_should_yield(),"original turn must preadmit the measured command spine ceiling");
  let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner.take()));
  assert!(!heap.overflowed);assert_eq!(heap.requested_bytes,0);assert!(heap.released_bytes<=release_ceiling);
  cx.consume_retained(RetainedCloneProgress{copied_items:1,released_bytes:heap.released_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(())
 });
}
fn exchange(actor:&mut Actor,law:&serde_json::Value,identity:&mut EntityIdentityAuthority<'_>,instance:u32,seq:u64,command:protocol::AppCommand,mut observe:impl FnMut(&protocol::AppFrame)->bool){
 let encoded=ready(protocol::encode_app_command(&command)).expect("real command encoder");
 let(driver,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
  let mut owners=CommandEnvelopeSet::try_new().expect("actual fixed command authority");
  owners.try_push(CommandEnvelope{instance,seq,command:encoded}).unwrap_or_else(|_|panic!("actual command envelope admission"));
  let batch=CommandBatch::try_new(seq,owners).unwrap_or_else(|_|panic!("actual command batch admission"));
  CommandBatchDriver::new(seq,batch)
 });
 assert!(!birth.overflowed);let release_ceiling=birth.requested_bytes;let mut driver=driver;let mut answered=false;let mut completed=false;
 for _ in 0..turns(law){
  let event=driver.next_page().expect("actual command page").map(|(cursor,bytes)|Event::CommandIngressPage{cursor,bytes}).unwrap_or(Event::Wake);
  let result=poll(actor,std::slice::from_ref(&event),identity);
  actor.turn(|_,_,cx|{
   let grant=cx.retained_grant();assert!(grant.maximum_items>0&&grant.maximum_depth>0&&grant.maximum_release_bytes>=COMMAND_PAGE_MAXIMUM_BYTES&&!cx.retirement_should_yield(),"original turn must preadmit a physical command page release");
   let(progress,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||driver.observe(&result.command_ingress,grant.maximum_release_bytes.min(COMMAND_PAGE_MAXIMUM_BYTES)));
   assert!(!heap.overflowed);assert_eq!(heap.requested_bytes,0,"acknowledgement must not create an unquoted owner");
   cx.consume_retained(RetainedCloneProgress{copied_items:1,released_bytes:heap.released_bytes,..Default::default()})?;cx.consume_fuel(1);
   match progress.expect("actual command acknowledgement"){CommandBatchProgress::Complete=>completed=true,CommandBatchProgress::Faulted=>panic!("actual Puzzle command ingress faulted: {:?}",result.command_ingress),_=>{}}Ok(())
  });
  for effect in &result.effects{
   let Effect::SendMessage{target:MessageEndpoint::Shell{instance:addressed},payload}=effect else{continue};
   if addressed.0!=instance.to_string(){continue}
   let Ok(frame)=ready(protocol::decode_app_frame(payload))else{continue};
   if let protocol::AppFrame::Error{in_reply_to,fault,..}=&frame{assert_ne!(*in_reply_to,Some(seq),"actual app rejected command: {fault:?}");}
   answered|=observe(&frame);
  }
  actor.retire(law,event);retire_turn(actor,law,result);
  if completed&&answered{retire_driver(actor,driver,release_ceiling);retire_command(actor,law,command);return}
 }
 panic!("actual command must both acknowledge ingress and publish its matching app response")
}
fn history(actor:&mut Actor,law:&serde_json::Value,identity:&mut EntityIdentityAuthority<'_>,instance:u32,seq:u64)->HistoryPatch{
 let mut found=None;
 exchange(actor,law,identity,instance,seq,protocol::AppCommand::ReadHistory{seq},|frame|{
  if let protocol::AppFrame::HistorySnapshot{in_reply_to,history_patch}=frame{if *in_reply_to==seq{found=Some(ready(semio_plugin_plugin_program::decode_wire_serialized::<HistoryPatch>(history_patch)).expect("actual typed history patch"));return true}}false
 });found.expect("actual history reply")
}
pub fn setup(actor:&mut Actor,law:&serde_json::Value,identity:&mut EntityIdentityAuthority<'_>)->Session{
 context(actor,law,identity,false);
 let instance=73;let request=ActorInstanceOpenRequest{activation_generation:1,instance_id:instance,request_sequence:1};
 let event=Event::InstanceOpen{request,app_id:AppInstanceId(law["component"]["appId"].as_str().unwrap().into()),actor:"puzzle-export-cancel".into(),config:Vec::new(),assets:Vec::new(),capabilities:Vec::new(),quotas:QuotaSchema::default()};
 let mut event=Some(event);let mut captured=None;
 for _ in 0..turns(law){
  let result=poll(actor,event.as_ref().map(std::slice::from_ref).unwrap_or(&[Event::Wake]),identity);
  if let Some(receipt)=result.lifecycle_receipt{assert!(matches!(receipt,ActorInstanceLifecycleReceipt::Captured{..}));assert_eq!(receipt.request_sequence(),request.request_sequence);captured=Some(receipt);}
  if let Some(event)=event.take(){actor.retire(law,event)}retire_turn(actor,law,result);if captured.is_some(){break}
 }
 let receipt=captured.expect("real Puzzle instance must publish Captured");
 let result=poll(actor,&[Event::InstanceLifecycleAck(ActorInstanceLifecycleAck{receipt})],identity);retire_turn(actor,law,result);
 let before=history(actor,law,identity,instance,10);let initial_alternative=before.active_alternative_id.clone();actor.retire(law,before);
 Session{lifetime:receipt.lifetime(),initial_alternative,instance}
}
pub fn assert_after_cancel(actor:&mut Actor,law:&serde_json::Value,identity:&mut EntityIdentityAuthority<'_>,session:&Session){
 let action=law["afterCancel"]["command"].as_str().unwrap();assert_eq!(action,"createAlternative");
 let source=DslValue::object(vec![("action".into(),DslValue::String(action.into())),("args".into(),DslValue::object(vec![("name".into(),DslValue::String(law["afterCancel"]["args"]["name"].as_str().unwrap().into()))]))]);
 let command=semio_plugin_plugin_program::encode_wire_serialized(&source);let seq=11;
 exchange(actor,law,identity,session.instance,seq,protocol::AppCommand::ArtifactCommand{seq,command},|frame|matches!(frame,protocol::AppFrame::Invocation{in_reply_to,..}|protocol::AppFrame::Done{in_reply_to}if *in_reply_to==seq));
 actor.retire(law,source);
 let after=history(actor,law,identity,session.instance,12);
 assert!(after.active_alternative_id.is_some(),"actual app creates an alternative");assert_ne!(after.active_alternative_id,session.initial_alternative,"same real Puzzle instance must mutate history after export cancellation");
 assert!(after.upserts.iter().any(|entry|entry.action_id==action),"actual app history must retain the dispatched action");actor.retire(law,after);
}
pub fn close(actor:&mut Actor,law:&serde_json::Value,identity:&mut EntityIdentityAuthority<'_>,session:Session){
 let request=ActorInstanceCloseRequest{lifetime:session.lifetime,request_sequence:20};let mut event=Some(Event::InstanceClose(request));let mut retired=false;
 for _ in 0..turns(law){
  let result=poll(actor,event.as_ref().map(std::slice::from_ref).unwrap_or(&[Event::Wake]),identity);let receipt=result.lifecycle_receipt;
  if let Some(event)=event.take(){actor.retire(law,event)}retire_turn(actor,law,result);
  if let Some(receipt)=receipt{
   assert_eq!(receipt.lifetime(),session.lifetime);assert_eq!(receipt.request_sequence(),request.request_sequence);retired|=matches!(receipt,ActorInstanceLifecycleReceipt::Retired{..});
   let result=poll(actor,&[Event::InstanceLifecycleAck(ActorInstanceLifecycleAck{receipt})],identity);retire_turn(actor,law,result);
  }if retired{break}
 }
 assert!(retired,"real Puzzle instance must retire before its actor context");actor.retire(law,session.initial_alternative);context(actor,law,identity,true);
}
