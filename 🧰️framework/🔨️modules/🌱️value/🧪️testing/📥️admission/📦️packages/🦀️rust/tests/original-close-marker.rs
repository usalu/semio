//! 🧷️ Actual retirement allocations settle their marker inside the one funded action that made them.
use semio_framework_value_testing_admission::{self as fixture,BankWallet,EffectBank,RetainedCloneProgress,RetainedWorkOutcome,RetainedWorkPoll,Wallet};
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
fn complete<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("synchronous closure stayed pending")}}
#[test]
fn close_marker_retains_real_birth_denies_zero_work_and_releases_every_backing(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../marker.json")).unwrap();let mut wallet=Wallet::from_profile(&law["stock"]);let mut bank=EffectBank::new(BankWallet::from_profile(&law["stock"]));let input=vec![law["value"].to_string(),"α🙂".to_owned(),"actual backing".to_owned()];let initial=input.capacity()*std::mem::size_of::<String>()+input.iter().map(String::capacity).sum::<usize>();let(mut owner,heap)=observe(||fixture::retirement::controlled::ControlledRetirement::new(input).unwrap_or_else(|_|panic!("fixture supports controlled retirement")));assert_eq!(heap,(0,0));let mut allocated=0usize;let mut released=0usize;let mut births=0;
 for _ in 0..10000{
  if owner.terminal_is_empty(){break}
  let grant=wallet.loan(1,4096);let quoted=owner.next_computational_work(4096).unwrap();
  let((denied,actual),heap)=observe(||{let(poll,actual)=bank.run(grant,0,|effect|owner.step(effect));(complete(poll),actual)});assert_eq!(denied.work,0);assert_eq!(denied.result.unwrap().progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(actual,None);assert_ne!(bank.take_admitted(),Some(false));
  let((turn,actual),heap)=observe(||{let(poll,actual)=bank.run(grant,quoted,|effect|owner.step(effect));(complete(poll),actual)});assert_ne!(bank.take_admitted(),Some(false));let p=turn.result.as_ref().map(|s|s.progress()).unwrap_or_else(|e|e.retained_progress());assert_eq!(actual.unwrap_or(0),heap.0);assert!(turn.work<=u128::from(quoted));births+=usize::from(heap.0>0);allocated+=heap.0;released+=heap.1;wallet.post(p,turn.work,grant,heap);turn.result.unwrap();
 }
 assert!(owner.terminal_is_empty());assert!(births>0);assert_eq!(released,initial+allocated);assert_eq!(observe(||drop(owner)).1,(0,0));
}
