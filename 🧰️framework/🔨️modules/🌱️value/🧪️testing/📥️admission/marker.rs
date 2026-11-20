//! 🧷️ Staged canonical Source admission debits exactly its actual work once, publishes the value, and closes the Source to terminal-empty.
use semio_framework_value_testing_admission::{self as fixture,EffectBank,Wallet};
use semio_framework_value_control_contract::{RetainedCloneGrant,RetainedWorkPoll};
use semio_framework_value_retirement::OriginalRetirementFailure;
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
const BODY:usize=4096;
#[test]
fn canonical_staged_capture_posts_actual_work_markers_and_physical_births(){
 let law:serde_json::Value=serde_json::from_str(include_str!("marker.json")).unwrap();let mut bank=EffectBank::new(Wallet::from_profile(&law["stock"]));let fuel=bank.wallet().remaining_work();
 let grant=bank.loan(1,BODY);let(mut source,work)={let turn=fixture::admit_owned(&mut bank,law["value"].as_u64().unwrap(),(),grant);let work=turn.work;match turn.result{Ok((source,_))=>(source,work),Err(_)=>panic!("canonical staged admission refused")}};
 assert_eq!(serde_json::json!(*source.borrow().get()),law["value"]);assert_eq!(u128::from(fuel-bank.wallet().remaining_work()),work);
 for _ in 0..10000{
  if source.terminal_is_empty(){break}
  let d=source.next_close_demand(BODY).unwrap();let units=source.next_close_work_demand(BODY).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:BODY.max(d.copy_bytes),maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
  let((poll,_),heap)=observe(||fixture::driven::<OriginalRetirementFailure>(&mut bank,g,units.max(1),|effect,reply,recover|if recover{source.recover_close_into(BODY,effect,reply)}else{source.close_into(BODY,effect,reply)}));
  let RetainedWorkPoll::Complete(turn)=poll else{panic!("synchronous Source close stayed pending")};let p=turn.result.unwrap().progress();bank.post(p,g,heap);
 }
 assert!(source.terminal_is_empty());
}
