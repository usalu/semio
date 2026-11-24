//! 🔁️ The real guest entry admits a kernel successor only when its actor is quiescent; driven by `🎟️retained-turn/🔁️succession/🧪️tests/🧫️fixtures/🔣️.json`.
use super::OriginalActorGuestEntry;
use semio_framework_actor::{RetainedTurnInput,retained_turn::OriginalActorTurnWork};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};
fn clock()->Option<u64>{Some(1)}
/// 🔁️ A successor enters only a quiescent actor: a held job source refuses it without spending, and the same successor is admitted once the job is gone.
#[test]
fn original_guest_succession_requires_a_quiescent_actor_and_keeps_its_entry(){
 use semio_framework_job::{StepContext,OriginalTurnFailure};
 struct Idle(u64,u64);
 impl semio_framework_actor::OriginalActorJobProvider for Idle{
  fn identity(&self)->(u64,u64){(self.0,self.1)}
  fn next_owned(&self)->Option<u64>{None}
  fn is_cancelled(&self,_:u64)->Option<bool>{None}
  fn cancel(&mut self,_:u64,_:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{Ok(true)}
  fn step(&mut self,_:u64,_:&mut StepContext<'_>)->Result<Option<semio_framework_actor::OriginalJobStep>,OriginalTurnFailure>{Ok(None)}
  fn close(&mut self,_:u64,_:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{Ok(true)}
 }
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/🔁️succession/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let stock:RetainedCloneGrant=serde_json::from_value(law["stock"]["grant"].clone()).unwrap();let fuel=law["stock"]["fuel"].as_u64().unwrap();let source=&law["source"];
 let mut input=RetainedTurnInput{operation:source["operation"].as_u64().unwrap(),generation:source["generation"].as_u64().unwrap(),epoch:1,work:OriginalActorTurnWork{clock_binding:source["clockBinding"].as_u64().unwrap(),absolute_deadline_us:source["deadlineUs"].as_u64().unwrap(),fuel},grant:stock};
 let next=|input:&mut RetainedTurnInput,spent,work|{let returned=input.return_original(spent,work).unwrap();input.epoch+=1;input.grant=returned.remaining;input.work.fuel=returned.remaining_fuel;};
 let budget=|input:RetainedTurnInput|semio_framework_actor::OriginalJobBudget{input,retained_policy:input.grant,fuel:input.work.fuel,deadline_us:input.work.absolute_deadline_us,cancelled:false};
 let mut entry=OriginalActorGuestEntry::new();
 let mut ready=false;for _ in 0..64{let call=entry.admit_step(input,budget(input).step_budget(),clock);ready=call.result.unwrap().ready;next(&mut input,call.receipt_spent,call.work_units_spent);if ready{break}}assert!(ready);
 let mut incoming=Some(semio_framework_actor::OriginalJobSource{job:5,kind:"kind".into(),input:Some(vec![1,2]),checkpoint:None});let mut spent=RetainedCloneProgress::default();
 let call=entry.dispatch_job_start_step(input,budget(input).step_budget(),&mut spent,clock,&mut incoming,|_,_|Ok(false));assert!(!call.result.unwrap());next(&mut input,call.receipt_spent,call.work_units_spent);assert!(entry.has_original_job_source());
 let offered=|input:RetainedTurnInput|semio_framework_actor::OriginalActorSuccession{operation:input.operation,generation:input.generation,epoch:input.epoch,successor:input.operation+1,grant:stock,fuel};
 let held=entry.succeed_step(budget(input),clock,offered(input));assert_eq!(held.result.unwrap_err().primary_value().unwrap().kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);assert_eq!((held.receipt_spent,held.work_units_spent),(Default::default(),0));assert_eq!(entry.tenure(),Some(input.operation));assert!(entry.has_original_job_source());
 let mut provider=Idle(input.operation,input.generation);
 let mut closed=false;for _ in 0..256{let mut spent=RetainedCloneProgress::default();let call=entry.close_jobs_step(input,budget(input).step_budget(),&mut spent,clock,&mut provider);closed=call.result.unwrap();next(&mut input,call.receipt_spent,call.work_units_spent);if closed{break}}assert!(closed&&!entry.has_original_job_source());
 let cancelled=entry.succeed_step(semio_framework_actor::OriginalJobBudget{cancelled:true,..budget(input)},clock,offered(input));assert_eq!(cancelled.result.unwrap_err().primary_value().unwrap().kind,semio_framework_value::ValueRefusalKind::Canceled);
 let admitted=entry.succeed_step(budget(input),clock,offered(input));assert!(admitted.result.unwrap().ready);assert_eq!(entry.tenure(),Some(input.operation+1));
 input=RetainedTurnInput{epoch:input.epoch+1,grant:stock,work:OriginalActorTurnWork{fuel,..input.work},..input};
 let mut spent=RetainedCloneProgress::default();let call=entry.dispatch_value_step(input,budget(input).step_budget(),&mut spent,clock,|cx|{cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok::<_,OriginalTurnFailure>(11)});assert_eq!(call.result.unwrap(),11);next(&mut input,call.receipt_spent,call.work_units_spent);
 for _ in 0..1024{if entry.terminal_is_empty(){break}let call=entry.close_plugin_step(semio_framework_actor::OriginalJobBudget{cancelled:true,..budget(input)});assert!(call.result.is_ok());next(&mut input,call.receipt_spent,call.work_units_spent)}
 assert!(entry.terminal_is_empty());
}
