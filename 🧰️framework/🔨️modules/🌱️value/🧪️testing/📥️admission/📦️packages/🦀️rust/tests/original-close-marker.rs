//! 🧷️ Actual retirement allocations settle their marker inside the one funded action that made them.
use semio_framework_value_testing_admission::{BankWallet,EffectBank,FixtureRetirementProducer,Wallet};
use semio_framework_value_control_contract::{OriginalEffectAuthority,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,ValueError};
use semio_framework_value_retirement::{OriginalRetirementFailure,retirement::controlled::ControlledRetirement};
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
const BODY:usize=4096;
type Closer<'c>=OriginalEffectAuthority<FixtureRetirementProducer<'c,Vec<String>>>;
/// 🪙️ Drives one close action to its recorded reply, returning its step, work, actual allocation marker and heap.
fn turn(authority:&mut Closer<'_>,bank:&mut EffectBank,grant:RetainedCloneGrant,work:u64)->(Result<RetainedCloneStep,OriginalRetirementFailure>,u128,Option<usize>,(usize,usize)){
 let mut action=None;let(mut spent,mut actual_sum,mut heap_sum)=(0u128,None,(0usize,0usize));
 for _ in 0..10000{
  let((poll,actual),heap)=observe(||bank.run(grant,work,|effect|{let before=effect.performed_work();let ready=authority.drive(&mut (BODY,None),effect,&mut action).unwrap();let used=effect.performed_work()-before;if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work:used})}else{RetainedWorkPoll::Pending{work:used}}}));
  heap_sum.0+=heap.0;heap_sum.1+=heap.1;if let Some(actual)=actual{actual_sum=Some(actual_sum.unwrap_or(0)+actual)}assert_ne!(bank.take_admitted(),Some(false));
  match poll{RetainedWorkPoll::Pending{work}=>spent+=work,RetainedWorkPoll::Complete(done)=>{spent+=done.work;done.result.unwrap();break}}
 }
 let demand=Closer::transfer_reply_demand().unwrap();let reply_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let mut output=None;
 let((poll,_),heap)=observe(||bank.run(reply_grant,1,|effect|RetainedWorkPoll::Complete(authority.transfer_reply_into(&(BODY,None),&mut action,&mut output,effect))));let RetainedWorkPoll::Complete(moved)=poll else{panic!("reply transfer pending")};assert_ne!(bank.take_admitted(),Some(false));bank.post(moved.result.unwrap().unwrap(),reply_grant,heap);
 (output.take().expect("transferred close reply"),spent,actual_sum,heap_sum)
}
#[test]
fn close_marker_retains_real_birth_denies_zero_work_and_releases_every_backing(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../marker.json")).unwrap();let mut wallet=Wallet::from_profile(&law["stock"]);let mut bank=EffectBank::new(BankWallet::from_profile(&law["stock"]));
 let input=vec![law["value"].to_string(),"α🙂".to_owned(),"actual backing".to_owned()];let initial=input.capacity()*std::mem::size_of::<String>()+input.iter().map(String::capacity).sum::<usize>();
 let(mut owner,heap)=observe(||ControlledRetirement::new(input).unwrap_or_else(|_|panic!("fixture supports controlled retirement")));assert_eq!(heap,(0,0));
 let(mut allocated,mut released,mut births)=(0usize,0usize,0usize);let mut authority=Some(OriginalEffectAuthority::new(FixtureRetirementProducer::new(&mut owner,None)));
 for _ in 0..10000{
  let cursor=authority.as_ref().unwrap().producer().owner();if cursor.terminal_is_empty(){break}
  let grant=wallet.loan(1,BODY);let quoted=cursor.next_computational_work(BODY).unwrap();
  let(denied,spent,actual,heap)=turn(authority.as_mut().unwrap(),&mut bank,grant,0);assert_eq!(spent,0);assert_eq!(denied.unwrap().progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(actual,None);
  let(step,spent,actual,heap)=turn(authority.as_mut().unwrap(),&mut bank,grant,quoted);let p=step.as_ref().map(|s|s.progress()).unwrap_or_else(|e|e.retained_progress());assert_eq!(actual.unwrap_or(0),heap.0);assert!(spent<=u128::from(quoted));births+=usize::from(heap.0>0);allocated+=heap.0;released+=heap.1;wallet.post(p,spent,grant,heap);step.unwrap();
 }
 let demand=Closer::producer_transfer_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let mut producer=None;
 let((poll,_),heap)=observe(||bank.run(grant,1,|effect|RetainedWorkPoll::Complete(Closer::transfer_producer(&mut authority,&(BODY,None),&mut producer,effect))));let RetainedWorkPoll::Complete(moved)=poll else{panic!("producer transfer pending")};assert_ne!(bank.take_admitted(),Some(false));bank.post(moved.result.unwrap().unwrap_or_default(),grant,heap);producer.take();drop(authority);
 assert!(owner.terminal_is_empty());assert!(births>0);assert_eq!(released,initial+allocated);assert_eq!(observe(||drop(owner)).1,(0,0));
}
