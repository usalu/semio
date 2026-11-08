use semio_framework_test_fixtures::fixture_text;
use semio_framework_value_clone_source_testing::{FixtureCloseStep,FixtureSource,FixtureSourceBirth};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use super::*;
use crate::observe_retirement_allocations as observe;

fn leaf<E:std::fmt::Debug>(bank:&mut EffectBank,grant:RetainedCloneGrant,units:u64,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,E>)->(RetainedCloneStep,(usize,usize),u128){
 let ((poll,actual),heap)=observe(||bank.run(grant,units,|effect|{let before=effect.performed_work();let result=operation(effect);RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:effect.performed_work()-before})}));
 let RetainedWorkPoll::Complete(turn)=poll else{panic!("native scalar fixture leaf retained an action")};let step=turn.result.unwrap();if let Some(actual)=actual{assert_eq!(actual,heap.0)}assert_ne!(bank.take_admitted(),Some(false));bank.post(step.progress(),grant,heap);(step,heap,turn.work)
}

#[test]
fn scalar_denied_payload_keeps_private_binding_and_value_empty(){
 let mut source=RetainedCloneSource::<u64>::from_owner(42);let mut cursor=u64::retained_clone_cursor();
 for grant in [RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:8,maximum_depth:1,..Default::default()},RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_depth:1,..Default::default()},RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:7,maximum_depth:1,..Default::default()},RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:8,maximum_depth:0,..Default::default()}]{
  let before=cursor.source.is_some();let(step,heap,work)=source.with_bank(|original,bank|leaf(bank,grant,1,|effect|{let mut answer=None;let done=cursor.advance_into(original,8,1,effect,OriginalEffectReply::borrow(&mut answer)?)?;drop(done);Ok::<_,ValueError>(answer.expect("scalar reply").expect("scalar denial preserves its original owner"))})).unwrap();
  assert_eq!(heap,(0,0));assert_eq!(work,0);assert_eq!(step.progress(),Default::default());assert_eq!(cursor.source.is_some(),before);assert!(cursor.value.is_none());
 }
 source.with_bank(|_,bank|{let demand=cursor.begin_close_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let units=cursor.begin_close_work_demand().unwrap();let(step,heap,_)=leaf(bank,grant,units,|effect|{let turn=cursor.begin_close_paid(effect);turn.result.map(RetainedCloneStep::Progress)});assert!(step.progress().fits(grant));assert_eq!(heap,(0,0));}).unwrap();
 assert!(cursor.terminal_is_empty());for n in 0..10000{if source.terminal_is_empty(){break}match source.close_once().unwrap(){FixtureCloseStep::Progress(_)|FixtureCloseStep::Complete=>{},other=>panic!("unexpected scalar source close: {other:?}")}assert!(n<9999)}assert!(source.terminal_is_empty());assert_eq!(observe(||drop((cursor,source))).1,(0,0));
}

#[test]
fn scalar_lifecycle_state_matrix_preserves_private_cursor_phase(){
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧮️scalar/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for row in law["lifecycle"].as_array().unwrap(){let mut cursor=ScalarCursor::<u64>::default();cursor.value=row["output"].as_bool().unwrap().then_some(42);cursor.closing=row["closing"].as_bool().unwrap();cursor.spent=row["spent"].as_bool().unwrap();assert_eq!(cursor.output_ready(),row["ready"].as_bool().unwrap());if !cursor.output_ready(){let mut bank=EffectBank::new(Wallet::new(RetainedCloneGrant::default(),0));let mut output=None;let(step,heap,work)=leaf(&mut bank,RetainedCloneGrant::default(),0,|effect|{let mut answer=None;let done=cursor.take(&mut output,effect,OriginalEffectReply::borrow(&mut answer)?)?;drop(done);answer.expect("scalar take reply")});assert_eq!(work,0);assert_eq!(heap,(0,0));assert_eq!(step.progress(),Default::default());assert!(output.is_none());}cursor.value=None;}
}
