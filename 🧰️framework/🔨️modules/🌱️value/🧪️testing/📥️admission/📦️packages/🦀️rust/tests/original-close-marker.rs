//! 🧷️ Actual retirement allocations keep their marker until an independently paid clear.
use semio_framework_value_testing_admission::{self as fixture,Wallet,RetainedCloneProgress};
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
#[test]
fn close_marker_retains_real_birth_denies_zero_work_and_releases_every_backing(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../marker.json")).unwrap();let mut bank=Wallet::from_profile(&law["stock"]);let input=vec![law["value"].to_string(),"α🙂".to_owned(),"actual backing".to_owned()];let initial=input.capacity()*std::mem::size_of::<String>()+input.iter().map(String::capacity).sum::<usize>();let(mut owner,heap)=observe(||fixture::retirement::controlled::ControlledRetirement::new(input).unwrap_or_else(|_|panic!("fixture supports controlled retirement")));assert_eq!(heap,(0,0));let mut actual=None;let mut allocated=0usize;let mut released=0usize;let mut clears=0;
 for _ in 0..10000{
  if actual.is_none()&&owner.terminal_is_empty(){break}
  let grant=bank.loan(1,4096);let before=actual;let(denied,heap)=observe(||fixture::close_step(&mut actual,grant,0,owner.next_demand(4096),owner.next_computational_work(4096),|g,a|{let result=owner.step(g,a);fixture::RetainedWorkOutcome{result,work:owner.last_turn_work()}}));assert_eq!(denied.work,0);assert_eq!(denied.result.unwrap().progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(actual,before);
  let(turn,heap)=observe(||fixture::close_step(&mut actual,grant,bank.work_remaining(),owner.next_demand(4096),owner.next_computational_work(4096),|g,a|{let result=owner.step(g,a);fixture::RetainedWorkOutcome{result,work:owner.last_turn_work()}}));let p=turn.result.as_ref().map(|s|s.progress()).unwrap_or_else(|e|e.retained_progress());if before.is_some(){assert_eq!(heap,(0,0));assert_eq!(actual,None);assert_eq!(turn.work,1);assert_eq!(p.copied_bytes,std::mem::size_of::<Option<usize>>());clears+=1}allocated+=heap.0;released+=heap.1;bank.post(p,turn.work,grant,heap);turn.result.unwrap();
 }
 assert!(owner.terminal_is_empty());assert!(actual.is_none());assert!(clears>0);assert_eq!(released,initial+allocated);assert_eq!(observe(||drop(owner)).1,(0,0));
}
