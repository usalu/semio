//! 🧵️ Existing retirement laws share one genuine finite Source, Work receipt and resident producer.
use crate::{RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,OriginalRetirementFailure};
use crate::retirement::{RetireOwned,controlled::ControlledRetirement};
use semio_framework_value_control_contract::{OriginalEffectProducer,OriginalEffectAuthority,OriginalEffectAction,OriginalEffectReply,OriginalEffectReplyComplete,OriginalSourceTurnIdentity,RetainedWorkPoll,RetainedWorkOutcome};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
use std::mem::{ManuallyDrop,size_of};
enum Bank<'a>{Owned(EffectBank),Borrowed(&'a mut EffectBank)}
impl std::ops::Deref for Bank<'_>{type Target=EffectBank;fn deref(&self)->&EffectBank{match self{Self::Owned(value)=>value,Self::Borrowed(value)=>value}}}
impl std::ops::DerefMut for Bank<'_>{fn deref_mut(&mut self)->&mut EffectBank{match self{Self::Owned(value)=>value,Self::Borrowed(value)=>value}}}
struct Context{source:OriginalSourceTurnIdentity,body:usize}
struct Producer<'a,T:RetireOwned>{owner:&'a mut ControlledRetirement<T>,source:OriginalSourceTurnIdentity}
impl<T:RetireOwned> OriginalEffectProducer for Producer<'_,T>{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=Context where 'b:'a;
 fn validate_context<'a,'b>(&self,c:&Context)->Result<(),ValueError>where 'b:'a{if c.source!=self.source{return Err(ValueError::invariant("retirement fixture changed its genuine Source"))}Ok(())}
 fn source_turn<'a,'b>(&self,_:&Context)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(self.source)}
 fn receive<'r,'a,'b>(&mut self,c:&mut Context,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{self.owner.step_into(c.body,e,r)}
 fn recover<'r,'a,'b>(&mut self,c:&mut Context,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{self.owner.recover_step_into(c.body,e,r)}
}
/// 🧪️ Retains the real controlled owner through all selected actions and funded final producer release.
pub(crate) struct RetirementFixture<'b,T:RetireOwned>{authority:ManuallyDrop<Option<OriginalEffectAuthority<Producer<'b,T>>>>,action:ManuallyDrop<Option<OriginalEffectAction<RetainedCloneStep,OriginalRetirementFailure>>>,reply:ManuallyDrop<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>>,returned:ManuallyDrop<Option<Producer<'b,T>>>,bank:&'b mut EffectBank,last_work:u128,last_heap:(usize,usize)}
pub(crate) struct RetirementSeed<'a,T:RetireOwned>{owner:ControlledRetirement<T>,bank:Bank<'a>}
impl<T:RetireOwned> RetirementSeed<'_,T>{pub(crate) fn lend(&mut self)->RetirementFixture<'_,T>{let source=self.bank.source_witness().identity();RetirementFixture{authority:ManuallyDrop::new(Some(OriginalEffectAuthority::new(Producer{owner:&mut self.owner,source}))),action:ManuallyDrop::new(None),reply:ManuallyDrop::new(None),returned:ManuallyDrop::new(None),bank:&mut self.bank,last_work:0,last_heap:(0,0)}}}
impl<'b,T:RetireOwned> RetirementFixture<'b,T>{
 pub(crate) fn new(value:T)->Result<RetirementSeed<'b,T>,(ValueError,T)>{let stock=RetainedCloneGrant{maximum_items:65536,maximum_copy_bytes:16777216,maximum_capacity_bytes:16777216,maximum_release_bytes:33554432,maximum_depth:4096};Ok(RetirementSeed{owner:ControlledRetirement::new(value)?,bank:Bank::Owned(EffectBank::new(Wallet::new(stock,32768)))})}
 pub(crate) fn with_bank(value:T,bank:&'b mut EffectBank)->Result<RetirementSeed<'b,T>,(ValueError,T)>{Ok(RetirementSeed{owner:ControlledRetirement::new(value)?,bank:Bank::Borrowed(bank)})}
 fn owner(&self)->&ControlledRetirement<T>{&self.authority.as_ref().expect("live original producer").producer().owner}
 pub(crate) fn original(&self)->Option<&T>{self.owner().original()}
 pub(crate) fn terminal_is_empty(&self)->bool{self.authority.as_ref().is_none_or(|a|a.producer().owner.terminal_is_empty())&&self.action.is_none()&&self.reply.is_none()}
 pub(crate) fn selected_demand(&self,b:usize)->Result<RetirementDemand,ValueError>{self.owner().selected_demand(b)}
 pub(crate) fn next_demand(&self,b:usize)->Result<RetirementDemand,ValueError>{self.owner().next_demand(b)}
 pub(crate) fn next_computational_work(&self,b:usize)->Result<u64,ValueError>{self.owner().next_computational_work(b)}
 pub(crate) fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.owner().next_copy_byte_demand()}
 pub(crate) fn next_capacity_byte_demand(&self,b:usize)->Result<usize,ValueError>{self.owner().next_capacity_byte_demand(b)}
 pub(crate) fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.owner().next_release_byte_demand()}
 pub(crate) fn next_depth_demand(&self)->Result<usize,ValueError>{self.owner().next_depth_demand()}
 pub(crate) fn last_turn_work(&self)->u64{self.last_work.try_into().expect("selected original Work remains within the u64 quote")}
 pub(crate) fn advance_original(&mut self,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,std::convert::Infallible>{
  let maximum=self.owner().next_computational_work(body).unwrap();let mut heap=(0usize,0usize);let before=*self.bank.wallet().original_work();
  for _ in 0..10000{
   let((poll,marker),observed)=observe(||self.bank.run_with_source(grant,maximum,|effect,turn|{let before=effect.performed_boundary();let mut c=Context{source:turn.identity(),body};let done=self.authority.as_mut().unwrap().drive(&mut c,effect,&mut self.action).unwrap();let work=effect.performed_since(before).unwrap();if done{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<_,ValueError>(()),work})}else{RetainedWorkPoll::Pending{work}}}));
   heap.0+=observed.0;heap.1+=observed.1;if let Some(marker)=marker{assert_eq!(marker,heap.0)}
   let RetainedWorkPoll::Complete(turn)=poll else{continue};turn.result.unwrap();assert!(self.action.as_mut().unwrap().transfer_reply(&mut self.reply).unwrap());self.authority.as_mut().unwrap().finish(&mut self.action).unwrap();self.last_work=before-*self.bank.wallet().original_work();let p=self.reply.as_ref().unwrap().as_ref().map_or_else(|e|e.retained_progress(),|s|s.progress());assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert!(p.fits(grant));assert_ne!(self.bank.take_admitted(),Some(false));self.bank.post(p,grant,heap);self.last_heap=heap;if let Err(error)=self.reply.as_ref().unwrap(){panic!("original retirement failure remains resident: {error:?}")}let result=self.reply.take().unwrap();return Ok(result.ok().expect("native fixture accepted only success"));
  }
  panic!("original retirement fixture action remained pending")
 }
 pub(crate) fn advance_observed(&mut self,body:usize,grant:RetainedCloneGrant)->Result<(RetainedCloneStep,(usize,usize)),std::convert::Infallible>{let step=self.advance_original(body,grant)?;Ok((step,self.last_heap))}
 pub(crate) fn finish(&mut self){
  if self.authority.is_none(){return}assert!(self.terminal_is_empty());let demand=OriginalEffectAuthority::<Producer<'b,T>>::producer_transfer_demand().unwrap();let grant=self.bank.loan(1,demand.copy_bytes);
  let(poll,marker)=self.bank.run_with_source(grant,1,|effect,turn|{let c=Context{source:turn.identity(),body:0};RetainedWorkPoll::Complete(OriginalEffectAuthority::transfer_producer(&mut self.authority,&c,&mut self.returned,effect))});let RetainedWorkPoll::Complete(turn)=poll else{panic!("original retirement producer return pending")};let progress=turn.result.unwrap().unwrap();assert_eq!(marker,None);assert_eq!(self.bank.take_admitted(),Some(true));self.bank.post(progress,grant,(0,0));
  let bytes=size_of::<Option<Producer<'b,T>>>();let grant=self.bank.loan(1,bytes);let(poll,marker)=self.bank.run(grant,1,|effect|{assert!(self.returned.as_ref().unwrap().owner.terminal_is_empty());assert!(effect.performed(1).unwrap());drop(self.returned.take());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<_,ValueError>(()),work:1})});let RetainedWorkPoll::Complete(turn)=poll else{panic!("native terminal retirement producer release pending")};turn.result.unwrap();assert_eq!(marker,None);assert_eq!(self.bank.take_admitted(),Some(true));self.bank.post(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()},grant,(0,0))
 }
}
impl<T:RetireOwned> Drop for RetirementFixture<'_,T>{fn drop(&mut self){let empty=self.authority.is_none()&&self.action.is_none()&&self.reply.is_none()&&self.returned.is_none();assert!(std::thread::panicking()||empty,"retirement fixture retains its actual original producer");if empty{unsafe{ManuallyDrop::drop(&mut self.authority);ManuallyDrop::drop(&mut self.action);ManuallyDrop::drop(&mut self.reply);ManuallyDrop::drop(&mut self.returned)}}}}
