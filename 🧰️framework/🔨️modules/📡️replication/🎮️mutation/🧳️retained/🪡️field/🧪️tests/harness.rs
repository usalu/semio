//! 🧪️ One reply-turn harness drives a retained field call through the real original Work authority until its reply is recorded.
use super::{Done,Reply};
use semio_framework_value::{OriginalRetirementFailure,RetainedAllocationEffect,RetirementDemand,ValueError};
use semio_framework_value::retained_clone::{OriginalEffectAuthority,OriginalEffectProducer,OriginalSourceTurnIdentity,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use semio_framework_value::retained_clone::RetainedCloneSource;
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};
use semio_framework_value_clone_source_testing::FixtureSource;
use std::mem::size_of;

pub(crate) type Call<'a>=dyn for<'r,'e,'x> FnMut(&'e mut RetainedAllocationEffect<'x>,Reply<'r>,bool)->Result<Done<'r>,ValueError>+'a;
struct Turn(OriginalSourceTurnIdentity);
struct Frame<'a,'b>{call:&'a mut Call<'b>}
impl OriginalEffectProducer for Turn{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=Frame<'a,'b> where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,_:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(self.0)}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>where 'b:'a{(c.call)(effect,reply,false)}
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>where 'b:'a{(c.call)(effect,reply,true)}
}
/// 🧾️ One recorded reply with the work, physical receipt and marker its original action performed.
pub(crate) struct Recorded{pub result:Result<RetainedCloneStep,OriginalRetirementFailure>,pub work:u128,pub heap:(usize,usize),pub marker:Option<usize>}
pub(crate) use semio_framework_value_testing_allocator::{observe_retirement_allocations as observe,refuse_one_allocation_on_this_thread as refuse_one};
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
const STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:4194304,maximum_copy_bytes:16777216,maximum_capacity_bytes:16777216,maximum_release_bytes:33554432,maximum_depth:256};
fn fresh()->EffectBank{EffectBank::new(Wallet::new(RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:1<<40,maximum_capacity_bytes:1<<40,maximum_release_bytes:1<<40,maximum_depth:1<<10},1<<50))}
/// 🏦️ Owns one finite Source stock across every turn of a test.
pub(crate) struct Harness{pub bank:EffectBank}
impl Harness{
 pub fn new()->Self{Self{bank:fresh()}}
 /// 🧵️ Runs one reply-shaped call to its recorded reply through any number of Pending turns.
 pub fn turn(&mut self,g:RetainedCloneGrant,units:u64,call:&mut Call<'_>)->Recorded{
  let source=self.bank.source_witness().identity();let mut authority=OriginalEffectAuthority::new(Turn(source));let mut context=Frame{call};
  let(mut action,mut output)=(None,None);let mut heap=(0usize,0usize);let mut marker=None;let before=*self.bank.wallet().original_work();let mut complete=false;
  for _ in 0..4096{
   let((poll,actual),observed)=observe(||self.bank.run(g,units,|effect|{let start=effect.performed_boundary();let ready=authority.drive(&mut context,effect,&mut action).unwrap();let work=effect.performed_since(start).unwrap();if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})}else{RetainedWorkPoll::Pending{work}}}));
   heap.0+=observed.0;heap.1+=observed.1;if actual.is_some(){marker=actual}
   if matches!(poll,RetainedWorkPoll::Pending{..}){continue}
   let progress=match action.as_ref().unwrap().recorded_reply().unwrap(){Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
   assert!(progress.fits(g));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));assert_ne!(self.bank.take_admitted(),Some(false));self.bank.post(progress,g,heap);complete=true;break
  }
  assert!(complete,"retained field action exceeded its original bound");
  let work=before-*self.bank.wallet().original_work();
  let g=grant(OriginalEffectAuthority::<Turn>::transfer_reply_demand().unwrap());
  let((poll,_),observed)=observe(||self.bank.run(g,1,|effect|RetainedWorkPoll::Complete(authority.transfer_reply_into(&mut context,&mut action,&mut output,effect))));let RetainedWorkPoll::Complete(turn)=poll else{panic!("reply transfer pending")};assert_ne!(self.bank.take_admitted(),Some(false));self.bank.post(turn.result.unwrap().unwrap(),g,observed);
  let mut result=None;let d=RetirementDemand{copy_bytes:2*size_of::<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>>(),depth:1,..Default::default()};let g=grant(d);
  let((poll,_),observed)=observe(||self.bank.run(g,1,|effect|{assert!(effect.performed(1).unwrap());result=output.take();RetainedWorkPoll::Complete(RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}),work:1})}));let RetainedWorkPoll::Complete(turn)=poll else{panic!("reply retirement pending")};assert_ne!(self.bank.take_admitted(),Some(false));self.bank.post(turn.result.unwrap(),g,observed);
  assert!(authority.into_producer().is_ok());
  Recorded{result:result.unwrap(),work,heap,marker}
 }
 /// 🧵️ Runs one native reply-shaped call that never goes Pending.
 pub fn native<T>(&mut self,g:RetainedCloneGrant,units:u64,call:impl FnOnce(&mut RetainedAllocationEffect<'_>)->T)->(T,u128,(usize,usize))where T:Sized{
  let before=*self.bank.wallet().original_work();let mut value=None;
  let((poll,_),heap)=observe(||self.bank.run(g,units,|effect|{value=Some(call(&mut *effect));RetainedWorkPoll::Complete(RetainedWorkOutcome::<_,ValueError>{result:Ok(()),work:effect.performed_work()})}));
  let RetainedWorkPoll::Complete(turn)=poll else{panic!("native call pending")};turn.result.unwrap();assert_ne!(self.bank.take_admitted(),Some(false));let work=before-*self.bank.wallet().original_work();(value.unwrap(),work,heap)
 }
 /// 🪪️ Admits one actual original Source on this harness's finite stock.
 pub fn source<T:RetireOwned+Sync+'static>(&mut self,value:T)->RetainedCloneSource<T>{match semio_framework_value_clone_source_testing::admit_owned(&mut self.bank,value,(),STOCK).result{Ok((source,_))=>source,Err(_)=>panic!("original Source admission refused")}}
 /// 🧹️ Closes one original Source through its own fixture close authority.
 pub fn close_source<T:RetireOwned+Sync+'static>(&mut self,source:RetainedCloneSource<T>){let bank=std::mem::replace(&mut self.bank,fresh());let mut fixture=match FixtureSource::from_admitted(source,bank){Ok(fixture)=>fixture,Err(_)=>panic!("original bank still active")};for _ in 0..100000{if fixture.terminal_is_empty(){break}match fixture.close_once(){Ok(_)=>{},Err(_)=>panic!("original Source close refused")}}assert!(fixture.terminal_is_empty());self.bank=fixture.into_bank().ok().expect("terminal fixture bank");}
 /// 🗑️ Drains one owned value through its controlled retirement.
 pub fn retire<T:RetireOwned>(&mut self,value:T){let mut owner=ControlledRetirement::retain(value);for _ in 0..100000{if owner.terminal_is_empty(){break}let d=owner.selected_demand(4096).unwrap();let work=owner.next_computational_work(4096).unwrap();let recorded=self.turn(grant(d),work,&mut |e,r,rec|if rec{owner.recover_step_into(4096,e,r)}else{owner.step_into(4096,e,r)});recorded.result.unwrap();}assert!(owner.terminal_is_empty());}
}
