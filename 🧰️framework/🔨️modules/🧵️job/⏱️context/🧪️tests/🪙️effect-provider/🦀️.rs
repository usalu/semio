//! 🪙️ The context's effect provider settles one performed-work receipt per effect and debits nothing itself: the single fuel debit stays with the converted work outcome.
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation,OriginalTurnFailure};
static NOW:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(0);
fn now()->Option<u64>{Some(NOW.load(std::sync::atomic::Ordering::SeqCst))}
use semio_framework_value::RetainedCloneGrant;
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:128}}
fn quantum()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:128}}
fn caller(operation:u64,deadline:u64)->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OperationId(operation),Generation(3),stock(),now,deadline,1000000)}
#[test]
fn an_effect_without_performed_work_hands_over_its_grant_and_charges_nothing(){
 let mut caller=caller(7601,u64::MAX);
 caller.turn(quantum(),|cx|{
  let before=cx.fuel_remaining();
  let (grant,performed)=cx.with_original_effect(quantum(),|_,effect|effect.grant())?.expect("an admitted quote runs");
  assert_eq!((grant,performed,cx.fuel_remaining()),(quantum(),0,before));
  Ok::<_,OriginalTurnFailure>(())
 }).unwrap();
 caller.retire(quantum());
}
#[test]
fn performed_work_is_reported_once_and_left_for_the_single_outcome_debit(){
 let mut caller=caller(7602,u64::MAX);
 caller.turn(quantum(),|cx|{
  let before=cx.fuel_remaining();
  let (admitted,performed)=cx.with_original_effect(quantum(),|_,effect|effect.performed(7))?.expect("an admitted quote runs");
  assert_eq!((admitted?,performed,cx.fuel_remaining()),(true,7,before));
  cx.consume_fuel(u64::try_from(performed).unwrap());
  assert_eq!(cx.fuel_remaining(),before-7);
  Ok::<_,OriginalTurnFailure>(())
 }).unwrap();
 caller.retire(quantum());
}
#[test]
fn an_unadmitted_quote_runs_nothing_and_charges_nothing(){
 let mut caller=caller(7603,5);
 caller.turn(quantum(),|cx|{
  NOW.store(10,std::sync::atomic::Ordering::SeqCst);let before=cx.fuel_remaining();let mut ran=false;
  let outcome=cx.with_original_effect(quantum(),|_,_|{ran=true})?;
  assert!(outcome.is_none()&&!ran&&cx.fuel_remaining()==before);
  Ok::<_,OriginalTurnFailure>(())
 }).unwrap();
 NOW.store(0,std::sync::atomic::Ordering::SeqCst);
 caller.retire(quantum());
}

#[test]
fn the_shared_corpus_settles_identically_through_the_job_provider(){
 let corpus:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::repo_text!("🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️tests/🪙️effect-provider/🧫️fixtures/🔣️.json")).unwrap();
 for (index,case) in corpus["cases"].as_array().unwrap().iter().enumerate(){
  let mut caller=caller(7700+index as u64,u64::MAX);
  caller.turn(quantum(),|cx|{
   let before=cx.fuel_remaining();let steps:Vec<u64>=case["performs"].as_array().unwrap().iter().map(|step|step.as_u64().unwrap()).collect();
   let (admitted,performed)=cx.with_original_effect(quantum(),|_,effect|steps.iter().all(|step|effect.performed(*step).unwrap()))?.expect("an admitted quote runs");
   assert!(admitted,"{}",case["name"]);assert_eq!(performed,u128::from(case["performed"].as_u64().unwrap()),"{}",case["name"]);assert!(performed<=u128::from(case["maximumWork"].as_u64().unwrap()));assert_eq!(cx.fuel_remaining(),before,"the provider debits nothing itself");
   Ok::<_,OriginalTurnFailure>(())
  }).unwrap();
  caller.retire(quantum());
 }
}
#[test]
fn every_context_and_pump_effect_carries_its_original_producer_invocation(){
 let mut caller=caller(7609,u64::MAX);
 caller.turn(quantum(),|cx|{
  let (plain,_)=cx.with_original_effect(quantum(),|_,effect|effect.invocation().is_some())?.expect("an admitted quote runs");
  let (sourced,_)=cx.with_original_source_effect(quantum(),|_,effect|effect.invocation().map(|invocation|invocation.source_turn()))?.expect("an admitted quote runs");
  assert!(plain);assert!(sourced.is_some());
  Ok::<_,OriginalTurnFailure>(())
 }).unwrap();
 caller.retire(quantum());
 let mut turn=semio_framework_job::OriginalRetirementPump::new().turn(8,u64::MAX,now,quantum()).unwrap();
 let (carried,_)=turn.with_effect(|effect|effect.invocation().map(|invocation|invocation.source_turn())).unwrap().expect("an admitted quote runs");
 assert_eq!(carried,Some(Some(turn.source_turn())));
}
#[test]
fn a_one_fuel_turn_closes_through_its_effect_charging_each_unit_once(){
 use semio_framework_value::retirement::controlled::ControlledRetirement;
 let mut caller=caller(7610,u64::MAX);let mut owner=ControlledRetirement::retain(String::from("π"));
 for _ in 0..64{
  if owner.terminal_is_empty(){break}
  caller.turn(quantum(),|cx|{
   let body=cx.retained_grant().maximum_copy_bytes;let demand=owner.selected_demand(body)?;let spent=cx.original_work_units_spent();
   cx.with_original_work_loan(1,cx.deadline_us(),|cx|semio_framework_job::close_original_effect_step(cx,demand,|effect|owner.step(body,effect)))?;
   assert!(cx.original_work_units_spent()-spent<=1);
   Ok::<_,OriginalTurnFailure>(())
  }).unwrap();
 }
 assert!(owner.terminal_is_empty());caller.retire(quantum());
}
