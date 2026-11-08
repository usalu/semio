//! ♻️ A generated native record closes through its real finite Source and borrowed children.
use semio_framework_value_retirement as value;
use semio_framework_value_clone_source::RetainedCloneSource;
use semio_framework_value_clone_source_testing::FixtureSourceBirth;
#[derive(semio_framework_value_derive::RetireOwned)]
#[value(crate="value")]
struct OriginalRecord{first:u64,second:u64}
#[test]
fn actual_generated_record_keeps_the_original_effect_parameter(){
 assert!(<OriginalRecord as value::retirement::RetireOwned>::native_shared_original_admission().is_some());let law:serde_json::Value=serde_json::from_str(include_str!("effect.json")).unwrap();let mut original=RetainedCloneSource::from_owner(OriginalRecord{first:law["first"].as_u64().unwrap(),second:law["second"].as_u64().unwrap()});assert_eq!(original.borrow().get().first+original.borrow().get().second,law["sum"].as_u64().unwrap());let pointer=original.borrow().get()as *const OriginalRecord;let denied=value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64};assert!(matches!(original.close_with(4096,denied).unwrap(),semio_framework_value_clone_source_testing::FixtureCloseStep::Denied));assert_eq!(original.borrow().get()as *const OriginalRecord,pointer);assert_eq!(original.borrow().get().first+original.borrow().get().second,law["sum"].as_u64().unwrap());
 for _ in 0..law["maximumTurns"].as_u64().unwrap(){if original.terminal_is_empty(){break}let step=original.close_once().unwrap();assert!(!matches!(step,semio_framework_value_clone_source_testing::FixtureCloseStep::Denied|semio_framework_value_clone_source_testing::FixtureCloseStep::Blocked));}
 assert!(original.terminal_is_empty());
}

use semio_framework_value_control_contract::*;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use semio_framework_value_retirement::retirement::{RetireOwned,controlled::ControlledRetirement};
struct Close;
impl OriginalEffectProducer for Close{
 type Output=RetainedCloneStep;type Failure=value::OriginalRetirementFailure;type Context<'a,'b>=&'a mut ControlledRetirement<OriginalRecord> where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,_:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{None}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,Self::Failure>,ValueError>where 'b:'a{c.step_into(effect,reply)}
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,Self::Failure>,ValueError>where 'b:'a{c.recover_step_into(effect,reply)}
}
#[test]
fn actual_generated_controlled_record_uses_borrowed_effect_children(){
 let law:serde_json::Value=serde_json::from_str(include_str!("effect.json")).unwrap();let grant=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};let mut bank=EffectBank::new(Wallet::new(grant,10000));let mut cursor=ControlledRetirement::new(OriginalRecord{first:law["first"].as_u64().unwrap(),second:law["second"].as_u64().unwrap()}).map_err(|(error,_)|error).unwrap();let mut authority=Some(OriginalEffectAuthority::new(Close));
 for _ in 0..law["maximumTurns"].as_u64().unwrap(){if cursor.terminal_is_empty(){break}let d=cursor.next_demand(4096).unwrap();let g=bank.loan(1,d.copy_bytes);let units=cursor.next_computational_work(4096).unwrap();let mut action=None;let mut reply=None;let mut heap_sum=(0,0);
  for _ in 0..law["maximumTurns"].as_u64().unwrap(){let ((poll,actual),heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run(g,units,|effect|{let before=effect.performed_boundary();let ready=authority.as_mut().unwrap().drive(&mut &mut cursor,effect,&mut action).unwrap();let work=effect.performed_since(before).unwrap();if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})}else{RetainedWorkPoll::Pending{work}}}));heap_sum.0+=heap.0;heap_sum.1+=heap.1;if let Some(actual)=actual{assert_eq!(actual,heap_sum.0)}if matches!(poll,RetainedWorkPoll::Complete(_)){assert!(action.as_mut().unwrap().transfer_reply(&mut reply).unwrap());authority.as_mut().unwrap().finish(&mut action).unwrap();break}}
  let p=reply.expect("original generated record reply").unwrap().progress();assert_ne!(bank.take_admitted(),Some(false));bank.post(p,g,heap_sum);
 }
 assert!(cursor.terminal_is_empty());let d=OriginalEffectAuthority::<Close>::producer_transfer_demand().unwrap();let g=bank.loan(1,d.copy_bytes);let mut producer=None;let(poll,actual)=bank.run(g,1,|effect|RetainedWorkPoll::Complete(OriginalEffectAuthority::transfer_producer(&mut authority,&&mut cursor,&mut producer,effect)));assert_eq!(actual,None);let RetainedWorkPoll::Complete(turn)=poll else{panic!("native producer transfer pending")};assert_eq!(bank.take_admitted(),Some(true));bank.post(turn.result.unwrap().unwrap(),g,(0,0));
 let d=RetirementDemand{copy_bytes:std::mem::size_of_val(&producer),depth:1,..Default::default()};let g=bank.loan(1,d.copy_bytes);let(poll,actual)=bank.run(g,1,|effect|{assert!(effect.performed(1).unwrap());drop(producer.take());RetainedWorkPoll::Complete(RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}),work:1})});assert_eq!(actual,None);let RetainedWorkPoll::Complete(turn)=poll else{panic!("native producer clear pending")};assert_eq!(bank.take_admitted(),Some(true));bank.post(turn.result.unwrap(),g,(0,0));assert!(authority.is_none()&&producer.is_none());
}

struct Unknown(u64);
impl value::retirement::RetireOwned for Unknown{type RetirementCursor=<u64 as value::retirement::RetireOwned>::RetirementCursor;fn retirement_seed(self)->Self::RetirementCursor{self.0.retirement_seed()}fn controlled_retirement_supported()->bool{true}}
#[test]fn native_admission_is_absent_for_unknown_custom_owners(){assert!(Unknown::native_shared_original_admission().is_none())}
