//! 🛡️ Every native owner closes its heterogeneous children through one ordered custody chain over the reply-shaped close entries.
use crate::scalar::{invalid,receipt,Quote};
use crate::snapshot::state::fields::{EntityRow,RetainedEntityLookup,RetainedBranchProjection,BranchProjectionOutput,RetainedSealedField};
use protocol::{retained_mutation_demand_fits};
use crate::retained::{Done,Flight,Reply,conclude};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedOwnedProjection,RetainedWorkOutcome,RetirementDemand,RetirementPollExit,ValueError,retirement::RetireOwned,retained_clone::{admit_retained_clone_progress,RetainedClone,RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource,RetainedCloneStep}};
use semio_framework_value_persistent::list::persistent::{PersistentSet,PersistentSplice};
use semio_framework_value_snapshot_projection::snapshot_projection::SnapshotProjection;
use std::mem::{size_of,ManuallyDrop};

/// 🧩️ One closeable child states its next close action, its custody needs and performs that action through the caller's reply.
pub trait Cursor{
 fn mark(&mut self){}
 fn finished(&self)->bool;
 fn native(&mut self,_:&mut RetainedAllocationEffect<'_>,_:&mut u128)->Result<Option<RetainedCloneProgress>,semio_framework_value::RetirementPollExit>{Ok(None)}
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn work(&self,body:usize)->Result<u64,ValueError>;
 fn birth(&self)->bool;
 fn control(&self)->bool;
 fn birth_step(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->OriginalRetirementPoll;
 fn close<'r>(&mut self,o:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>;
}
fn tagged(error:OriginalRetirementFailure,paid:bool)->OriginalRetirementFailure{if !paid{return error}let p=error.retained_progress();error.with_retained_progress(crate::scalar::add(p,1).unwrap_or(p))}
fn release<C>()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Option<Box<C>>>(),release_bytes:size_of::<C>(),depth:1,..Default::default()}}

macro_rules! entries{($s:ident,$o:ident,$body:ident,$e:ident,$r:ident,$recover:ident)=>{match($o,$recover){(None,false)=>$s.close_into($body,$e,$r),(None,true)=>$s.recover_close_into($body,$e,$r),(Some(o),false)=>$s.close_original_into(o,$body,$e,$r),(Some(o),true)=>$s.recover_close_original_into(o,$body,$e,$r)}}}
#[macro_export]
macro_rules! std_cursor{([$($g:tt)*] $t:ty,[$($w:tt)*],$mark:expr)=>{
 impl<$($g)*> $crate::custody::Cursor for $t where $($w)*{
  fn mark(&mut self){let mark:fn(&mut Self)=$mark;mark(self)}
  fn finished(&self)->bool{self.terminal_is_empty()}
  fn demand(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.close_demand(body)}
  fn work(&self,body:usize)->Result<u64,semio_framework_value::ValueError>{self.close_work_demand(body)}
  fn birth(&self)->bool{self.requires_original_birth()}
  fn control(&self)->bool{self.requires_original_control()}
  fn birth_step(&mut self,o:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,g:semio_framework_value::retained_clone::RetainedCloneGrant)->semio_framework_value::OriginalRetirementPoll{self.close_original_birth_step(o,g)}
  fn close<'r>(&mut self,o:Option<&mut semio_framework_value::OriginalRetirementCloseLoan<'_>>,body:usize,e:&mut semio_framework_value::RetainedAllocationEffect<'_>,r:$crate::retained::Reply<'r>,recover:bool)->Result<$crate::retained::Done<'r>,semio_framework_value::ValueError>{match(o,recover){(None,false)=>self.close_into(body,e,r),(None,true)=>self.recover_close_into(body,e,r),(Some(o),false)=>self.close_original_into(o,body,e,r),(Some(o),true)=>self.recover_close_original_into(o,body,e,r)}}
 }
}}
pub(crate) use std_cursor;
std_cursor!([T:EntityRow] RetainedEntityLookup<T>,[],|s|s.begin_close());
std_cursor!([B:SnapshotProjection<Output:BranchProjectionOutput<B>>+Send] RetainedBranchProjection<B>,[],|s|s.begin_close());
std_cursor!([T:RetainedClone+Send+Sync] RetainedSealedField<T>,[],|s|s.begin_close());
std_cursor!([R:Sync] RetainedOwnedProjection<R>,[],|_|{});
std_cursor!([const N:usize] protocol::RetainedDiagnosticPreparation<N>,[],|s|s.begin_close());

impl<C:Cursor> Cursor for Option<Box<C>>{
 fn mark(&mut self){if let Some(c)=self{c.mark();}}
 fn finished(&self)->bool{self.is_none()}
 fn native(&mut self,e:&mut RetainedAllocationEffect<'_>,work:&mut u128)->Result<Option<RetainedCloneProgress>,semio_framework_value::RetirementPollExit>{match self{Some(c) if c.finished()=>{*work+=1;*self=None;Ok(Some(receipt(release::<C>())))},Some(c)=>c.native(e,work),None=>Err(invalid().into())}}
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{let c=self.as_deref().ok_or_else(invalid)?;if c.finished(){Ok(release::<C>())}else{c.demand(body)}}
 fn work(&self,body:usize)->Result<u64,ValueError>{let c=self.as_deref().ok_or_else(invalid)?;if c.finished(){Ok(1)}else{c.work(body)}}
 fn birth(&self)->bool{self.as_deref().is_some_and(|c|!c.finished()&&c.birth())}
 fn control(&self)->bool{self.as_deref().is_some_and(|c|!c.finished()&&c.control())}
 fn birth_step(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->OriginalRetirementPoll{match self{Some(c)=>c.birth_step(o,g),None=>semio_framework_value::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}}
 fn close<'r>(&mut self,o:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{self.as_deref_mut().ok_or_else(invalid)?.close(o,body,e,r,recover)}
}

/// 🧳️ An owned value and its own retirement cursor leave the owner together or not at all.
pub struct Held<T:RetireOwned>{value:ManuallyDrop<Option<T>>,close:RetainedCloneClose}
impl<T:RetireOwned> Default for Held<T>{fn default()->Self{Self{value:ManuallyDrop::new(None),close:Default::default()}}}
impl<T:RetireOwned> Held<T>{
 pub fn set(&mut self,value:T){*self.value=Some(value);}
 pub fn take(&mut self)->Option<T>{self.value.take()}
 pub fn is_some(&self)->bool{self.value.is_some()}
 pub fn is_none(&self)->bool{self.value.is_none()}
 pub fn get(&self)->Option<&T>{self.value.as_ref()}
 pub fn get_opt(&self)->&Option<T>{&self.value}
 pub fn get_mut(&mut self)->Option<&mut T>{self.value.as_mut()}
 pub fn slot(&mut self)->&mut Option<T>{&mut self.value}
}
impl<T:RetireOwned+Send> Cursor for Held<T>{
 fn finished(&self)->bool{self.value.is_none()&&self.close.is_empty()}
 fn native(&mut self,e:&mut RetainedAllocationEffect<'_>,work:&mut u128)->Result<Option<RetainedCloneProgress>,semio_framework_value::RetirementPollExit>{if !self.close.is_empty()||self.value.is_none(){return Ok(None)}let turn=self.close.begin_granted(&mut self.value,e);*work+=turn.work;match turn.result?{Some(step)=>Ok(Some(step.progress())),None=>Err(invalid().into())}}
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close.next_owner_demands::<T>(self.value.is_some(),&None,body)}
 fn work(&self,body:usize)->Result<u64,ValueError>{self.close.next_owner_work::<T>(self.value.is_some(),&None,body)}
 fn birth(&self)->bool{!self.close.is_empty()&&self.close.requires_original_birth()}
 fn control(&self)->bool{!self.close.is_empty()&&self.close.requires_original_control()}
 fn birth_step(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->OriginalRetirementPoll{self.close.step_original_birth(o,g)}
 fn close<'r>(&mut self,o:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{let s=&mut self.close;match(o,recover){(None,false)=>s.step_into(body,e,r),(None,true)=>s.recover_step_into(body,e,r),(Some(o),false)=>s.step_original_into(o,body,e,r),(Some(o),true)=>s.recover_step_original_into(o,body,e,r)}}
}
impl<T:RetireOwned> Drop for Held<T>{fn drop(&mut self){let empty=self.value.is_none()&&self.close.is_empty();assert!(std::thread::panicking()||empty,"native Energy owned value retains custody");if empty{unsafe{ManuallyDrop::drop(&mut self.value);}}}}

/// 🔗️ A bound original alias closes through the shared binding entries.
#[derive(Default)]pub struct Bound(pub Option<RetainedCloneBinding>);
impl Cursor for Bound{
 fn finished(&self)->bool{self.0.is_none()}
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{RetainedCloneBinding::close_demand(&self.0,body)}
 fn work(&self,body:usize)->Result<u64,ValueError>{RetainedCloneBinding::close_work_demand(&self.0,body)}
 fn birth(&self)->bool{RetainedCloneBinding::requires_original_birth(&self.0)}
 fn control(&self)->bool{RetainedCloneBinding::requires_original_control(&self.0)}
 fn birth_step(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->OriginalRetirementPoll{RetainedCloneBinding::close_original_birth_step(&mut self.0,o,g)}
 fn close<'r>(&mut self,o:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{let b=&mut self.0;match(o,recover){(None,false)=>RetainedCloneBinding::close_into(b,body,e,r),(None,true)=>RetainedCloneBinding::recover_close_into(b,body,e,r),(Some(o),false)=>RetainedCloneBinding::close_original_into(b,o,body,e,r),(Some(o),true)=>RetainedCloneBinding::recover_close_original_into(b,o,body,e,r)}}
}

/// 🪟️ An admitted row source closes through its own source entries and leaves an empty slot behind.
pub struct Sourced<T:Sync+'static,A:RetireOwned+Sync=T>(pub ManuallyDrop<Option<RetainedCloneSource<T,A>>>);
impl<T:Sync+'static,A:RetireOwned+Sync> Default for Sourced<T,A>{fn default()->Self{Self(ManuallyDrop::new(None))}}
impl<T:Sync+'static,A:RetireOwned+Sync> Sourced<T,A>{
 pub fn slot(&mut self)->&mut Option<RetainedCloneSource<T,A>>{&mut self.0}
 pub fn get(&self)->Option<&RetainedCloneSource<T,A>>{self.0.as_ref()}
}
impl<T:Sync+'static,A:RetireOwned+Sync> Cursor for Sourced<T,A>{
 fn finished(&self)->bool{self.0.is_none()}
 fn native(&mut self,_:&mut RetainedAllocationEffect<'_>,work:&mut u128)->Result<Option<RetainedCloneProgress>,semio_framework_value::RetirementPollExit>{if self.0.as_ref().is_some_and(|s|s.terminal_is_empty()){*work+=1;drop(self.0.take());return Ok(Some(receipt(RetirementDemand{copy_bytes:2*size_of::<Option<RetainedCloneSource<T,A>>>(),depth:1,..Default::default()})))}Ok(None)}
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{let s=self.0.as_ref().ok_or_else(invalid)?;if s.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:2*size_of::<Option<RetainedCloneSource<T,A>>>(),depth:1,..Default::default()})}else{s.next_close_demand(body)}}
 fn work(&self,body:usize)->Result<u64,ValueError>{let s=self.0.as_ref().ok_or_else(invalid)?;if s.terminal_is_empty(){Ok(1)}else{s.next_close_work_demand(body)}}
 fn birth(&self)->bool{self.0.as_ref().is_some_and(|s|!s.terminal_is_empty()&&s.requires_original_birth())}
 fn control(&self)->bool{self.0.as_ref().is_some_and(|s|!s.terminal_is_empty()&&s.requires_original_control())}
 fn birth_step(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->OriginalRetirementPoll{match self.0.as_mut(){Some(s)=>s.close_original_birth_step(o,g),None=>semio_framework_value::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}}
 fn close<'r>(&mut self,o:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{let s=self.0.as_mut().ok_or_else(invalid)?;entries!(s,o,body,e,r,recover)}
}
impl<T:Sync+'static,A:RetireOwned+Sync> Drop for Sourced<T,A>{fn drop(&mut self){let empty=self.0.is_none();assert!(std::thread::panicking()||empty,"native Energy row source retains custody");if empty{unsafe{ManuallyDrop::drop(&mut self.0);}}}}

macro_rules! paid_cursor{([$($g:tt)*] $t:ty)=>{
 impl<$($g)*> Cursor for $t{
  fn mark(&mut self){}
  fn finished(&self)->bool{self.terminal_is_empty()}
  fn native(&mut self,e:&mut RetainedAllocationEffect<'_>,work:&mut u128)->Result<Option<RetainedCloneProgress>,semio_framework_value::RetirementPollExit>{
   if self.terminal_is_empty(){return Err(invalid().into())}
   if !self.is_closing(){let turn=self.begin_close_paid(e);*work+=turn.work;return Ok(Some(turn.result?.progress()))}
   Ok(None)
  }
  fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.is_closing(){Ok(self.begin_close_demand())}else{self.close_demand(body)}}
  fn work(&self,body:usize)->Result<u64,ValueError>{if !self.is_closing(){Ok(self.begin_close_work_demand())}else{self.close_work_demand(body)}}
  fn birth(&self)->bool{self.is_closing()&&self.requires_original_birth()}
  fn control(&self)->bool{self.is_closing()&&self.requires_original_control()}
  fn birth_step(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->OriginalRetirementPoll{self.close_original_birth_step(o,g)}
  fn close<'r>(&mut self,o:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{entries!(self,o,body,e,r,recover)}
 }
}}
paid_cursor!([T:RetireOwned+Send+Sync+'static] PersistentSet<T>);
paid_cursor!([T:RetireOwned+Send+Sync+'static] PersistentSplice<T>);
paid_cursor!([T:EntityRow] semio_framework_value_persistent::list::persistent::PersistentListRead<T>);

/// 📏️ The chain's next close action is that of its first child that still holds custody.
pub fn chain_demand(items:&[&dyn Cursor],body:usize)->Result<RetirementDemand,ValueError>{items.iter().find(|c|!c.finished()).ok_or_else(invalid)?.demand(body)}
pub fn chain_work(items:&[&dyn Cursor],body:usize)->Result<u64,ValueError>{items.iter().find(|c|!c.finished()).ok_or_else(invalid)?.work(body)}
pub fn chain_birth(flight:&Flight,items:&[&dyn Cursor])->bool{flight.idle()&&items.iter().find(|c|!c.finished()).is_some_and(|c|c.birth())}
pub fn chain_control(flight:&Flight,items:&[&dyn Cursor])->bool{if let Some(loaned)=flight.loaned(){return loaned}items.iter().find(|c|!c.finished()).is_some_and(|c|c.control())}
pub fn chain_birth_step(items:&mut [&mut dyn Cursor],o:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->OriginalRetirementPoll{match items.iter_mut().find(|c|!c.finished()){Some(c)=>c.birth_step(o,g),None=>semio_framework_value::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}}
pub fn chain_empty(flight:&Flight,items:&[&dyn Cursor])->bool{flight.idle()&&items.iter().all(|c|c.finished())}
pub fn chain_mark(items:&mut [&mut dyn Cursor]){for c in items{c.mark();}}

/// ♻️ One close step runs the first unfinished child's next action under the owner's retained route and charges its work to `work`.
pub fn chain_step(flight:&mut Flight,items:&mut [&mut dyn Cursor],mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,work:&mut u128,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
 let g=effect.grant();let route:u8;
 if recover{route=flight.route().ok_or_else(invalid)?;}else{
  if !flight.idle(){return Err(invalid().into())}
  let Some(at)=items.iter().position(|c|!c.finished())else{return Ok(RetainedCloneStep::Complete(Default::default()))};
  let c=&mut *items[at];let demand=c.demand(body)?;
  if !retained_mutation_demand_fits(demand,g){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if c.birth(){return Err(invalid().into())}
  if c.control()&&original.is_none(){return Err(invalid().into())}
  if let Some(p)=c.native(effect,work)?{return Ok(RetainedCloneStep::Progress(p))}
  route=u8::try_from(at+1).map_err(|_|invalid())?;
 }
 let c=&mut *items[usize::from(route)-1];
 let loan=if c.control(){original.as_deref_mut()}else{None};let before=*work;
 let step=protocol::retained_routed!(flight,effect,recover,route,body,0,loan,*work,|r| c.close(original.as_deref_mut(),body,effect,r,false),c.close(original.as_deref_mut(),body,effect,r,true));
 let step=step.map_err(|error|tagged(error,*work!=before))?;
 Ok(RetainedCloneStep::Progress(step.progress()))
}
/// ♻️ One reply-shaped close turn of an owner settles a [`chain_step`] against its effect.
pub fn close_chain<'r>(flight:&mut Flight,items:&mut [&mut dyn Cursor],original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
 let base=effect.performed_work();let mut work:u128=0;
 let result=chain_step(flight,items,original,body,effect,&mut work,recover);
 conclude(effect,base,result,work,|step|step.progress(),reply)
}
/// 🏁️ A forward phase closes one boxed child to its release through the same retained route.
pub fn finish<C:Cursor>(flight:&mut Flight,child:&mut Option<Box<C>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,work:&mut u128,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{chain_step(flight,&mut [child as &mut dyn Cursor],None,body,effect,work,recover)}

/// 🧰️ Generates an owner's complete close surface over its ordered children and the owner's `flight` field.
#[macro_export]
macro_rules! custody_impl{($($f:ident),+ $(,)?)=>{
 pub fn mark_close(&mut self){$crate::custody::chain_mark(&mut [$(&mut self.$f as &mut dyn $crate::custody::Cursor),+]);}
 pub fn close_demand(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{$crate::custody::chain_demand(&[$(&self.$f as &dyn $crate::custody::Cursor),+],body)}
 pub fn close_work_demand(&self,body:usize)->Result<u64,semio_framework_value::ValueError>{$crate::custody::chain_work(&[$(&self.$f as &dyn $crate::custody::Cursor),+],body)}
 pub fn requires_original_birth(&self)->bool{$crate::custody::chain_birth(&self.flight,&[$(&self.$f as &dyn $crate::custody::Cursor),+])}
 pub fn requires_original_control(&self)->bool{$crate::custody::chain_control(&self.flight,&[$(&self.$f as &dyn $crate::custody::Cursor),+])}
 pub fn close_original_birth_step(&mut self,o:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,g:semio_framework_value::retained_clone::RetainedCloneGrant)->semio_framework_value::OriginalRetirementPoll{$crate::custody::chain_birth_step(&mut [$(&mut self.$f as &mut dyn $crate::custody::Cursor),+],o,g)}
 pub fn custody_empty(&self)->bool{$crate::custody::chain_empty(&self.flight,&[$(&self.$f as &dyn $crate::custody::Cursor),+])}
 pub fn close_reply<'r>(&mut self,original:Option<&mut semio_framework_value::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:crate::retained::Reply<'r>,recover:bool)->Result<crate::retained::Done<'r>,semio_framework_value::ValueError>{$crate::custody::close_chain(&mut self.flight,&mut [$(&mut self.$f as &mut dyn $crate::custody::Cursor),+],original,body,effect,reply,recover)}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.custody_empty()}
 pub fn close_into<'r>(&mut self,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:crate::retained::Reply<'r>)->Result<crate::retained::Done<'r>,semio_framework_value::ValueError>{self.close_reply(None,body,effect,reply,false)}
 pub fn recover_close_into<'r>(&mut self,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:crate::retained::Reply<'r>)->Result<crate::retained::Done<'r>,semio_framework_value::ValueError>{self.close_reply(None,body,effect,reply,true)}
 pub fn close_original_into<'r>(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:crate::retained::Reply<'r>)->Result<crate::retained::Done<'r>,semio_framework_value::ValueError>{self.close_reply(Some(original),body,effect,reply,false)}
 pub fn recover_close_original_into<'r>(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:crate::retained::Reply<'r>)->Result<crate::retained::Done<'r>,semio_framework_value::ValueError>{self.close_reply(Some(original),body,effect,reply,true)}
}}
pub(crate) use custody_impl;

/// ⏱️ Starting a close is one paid unit that either happens whole or not at all.
pub fn begin_paid(closing:&mut bool,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,ValueError>{
 let d=begin_demand(*closing);
 if *closing||effect.maximum_work_units()==0||!retained_mutation_demand_fits(d,effect.grant()){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
 match effect.performed(1){Ok(true)=>{*closing=true;RetainedWorkOutcome{result:Ok(receipt(d)),work:1}},Ok(false)=>RetainedWorkOutcome{result:Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"native Energy close start exceeded its original action")),work:1},Err(error)=>RetainedWorkOutcome{result:Err(error),work:1}}
}
pub fn begin_demand(closing:bool)->RetirementDemand{if closing{Default::default()}else{RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()}}}

/// 📨️ Diagnostic messages leave through the shared ledger retirement in bounded sync steps.
#[derive(Default)]pub struct Ledger{messages:ManuallyDrop<Option<Vec<protocol::MutationMessage>>>,close:Option<protocol::MutationMessageLedgerRetirement>}
impl Ledger{
 pub fn set(&mut self,messages:Vec<protocol::MutationMessage>){*self.messages=Some(messages);}
 pub fn take(&mut self)->Option<Vec<protocol::MutationMessage>>{self.messages.take()}
 pub fn is_some(&self)->bool{self.messages.is_some()}
}
impl Cursor for Ledger{
 fn finished(&self)->bool{self.messages.is_none()&&self.close.is_none()}
 fn native(&mut self,e:&mut RetainedAllocationEffect<'_>,work:&mut u128)->Result<Option<RetainedCloneProgress>,semio_framework_value::RetirementPollExit>{
  if let Some(close)=self.close.as_mut(){let step=close.close_step(e.grant())?;*work+=u128::from(close.last_turn_work()).max(1);if close.terminal_is_empty(){self.close=None;}return Ok(Some(step.progress()))}
  if let Some(messages)=self.messages.take(){self.close=Some(protocol::MutationMessageLedgerRetirement::new(String::new(),messages));*work+=1;return Ok(Some(receipt(RetirementDemand{copy_bytes:size_of::<protocol::MutationMessageLedgerRetirement>()+2*size_of::<Option<Vec<protocol::MutationMessage>>>(),depth:1,..Default::default()})))}
  Err(invalid().into())
 }
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(close)=self.close.as_ref(){return Ok(RetirementDemand{copy_bytes:close.next_copy_byte_demand(),capacity_bytes:close.next_capacity_byte_demand(body)?,release_bytes:close.next_release_byte_demand()?,depth:close.next_depth_demand()?})}Ok(RetirementDemand{copy_bytes:size_of::<protocol::MutationMessageLedgerRetirement>()+2*size_of::<Option<Vec<protocol::MutationMessage>>>(),depth:1,..Default::default()})}
 fn work(&self,_:usize)->Result<u64,ValueError>{Ok(1)}
 fn birth(&self)->bool{false}
 fn control(&self)->bool{false}
 fn birth_step(&mut self,_:&mut OriginalRetirementCloseLoan<'_>,_:RetainedCloneGrant)->OriginalRetirementPoll{semio_framework_value::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}
 fn close<'r>(&mut self,_:Option<&mut OriginalRetirementCloseLoan<'_>>,_:usize,_:&mut RetainedAllocationEffect<'_>,_:Reply<'r>,_:bool)->Result<Done<'r>,ValueError>{Err(invalid())}
}
impl Drop for Ledger{fn drop(&mut self){let empty=self.messages.is_none()&&self.close.is_none();assert!(std::thread::panicking()||empty,"native Energy diagnostics retain custody");if empty{unsafe{ManuallyDrop::drop(&mut self.messages);}}}}

/// 📋️ A persistent list change delegates every close action to its set or splice cursor.
macro_rules! paid_list_cursor{($t:ident)=>{
 impl<T:RetireOwned+Send+Sync+'static> $crate::custody::Cursor for $t<T>{
  fn finished(&self)->bool{match self{Self::Set(c)=>$crate::custody::Cursor::finished(c),Self::Splice{cursor,..}=>$crate::custody::Cursor::finished(cursor)}}
  fn native(&mut self,e:&mut semio_framework_value::RetainedAllocationEffect<'_>,w:&mut u128)->Result<Option<semio_framework_value::retained_clone::RetainedCloneProgress>,semio_framework_value::RetirementPollExit>{match self{Self::Set(c)=>$crate::custody::Cursor::native(c,e,w),Self::Splice{cursor,..}=>$crate::custody::Cursor::native(cursor,e,w)}}
  fn demand(&self,b:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{match self{Self::Set(c)=>$crate::custody::Cursor::demand(c,b),Self::Splice{cursor,..}=>$crate::custody::Cursor::demand(cursor,b)}}
  fn work(&self,b:usize)->Result<u64,semio_framework_value::ValueError>{match self{Self::Set(c)=>$crate::custody::Cursor::work(c,b),Self::Splice{cursor,..}=>$crate::custody::Cursor::work(cursor,b)}}
  fn birth(&self)->bool{match self{Self::Set(c)=>$crate::custody::Cursor::birth(c),Self::Splice{cursor,..}=>$crate::custody::Cursor::birth(cursor)}}
  fn control(&self)->bool{match self{Self::Set(c)=>$crate::custody::Cursor::control(c),Self::Splice{cursor,..}=>$crate::custody::Cursor::control(cursor)}}
  fn birth_step(&mut self,o:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,g:semio_framework_value::retained_clone::RetainedCloneGrant)->semio_framework_value::OriginalRetirementPoll{match self{Self::Set(c)=>$crate::custody::Cursor::birth_step(c,o,g),Self::Splice{cursor,..}=>$crate::custody::Cursor::birth_step(cursor,o,g)}}
  fn close<'r>(&mut self,o:Option<&mut semio_framework_value::OriginalRetirementCloseLoan<'_>>,b:usize,e:&mut semio_framework_value::RetainedAllocationEffect<'_>,r:crate::retained::Reply<'r>,recover:bool)->Result<crate::retained::Done<'r>,semio_framework_value::ValueError>{match self{Self::Set(c)=>$crate::custody::Cursor::close(c,o,b,e,r,recover),Self::Splice{cursor,..}=>$crate::custody::Cursor::close(cursor,o,b,e,r,recover)}}
 }
}}
pub(crate) use paid_list_cursor;

/// 🚪️ Runs one gated step of a synchronous or routed owner against its quote and settles the owed work on the effect.
pub fn gated<'r>(effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,quote:Option<Result<Quote,ValueError>>,max:u64,body:impl FnOnce(&mut RetainedAllocationEffect<'_>,&mut u128)->Result<RetainedCloneStep,RetirementPollExit>)->Result<Done<'r>,ValueError>{
 let g=effect.grant();let base=effect.performed_work();let mut work:u128=0;
 let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
  let mut units=0u64;
  if let Some(q)=quote{let(d,u)=q?;if u>max||effect.remaining_work_units()?<u||!retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()))}units=u;}
  let step=body(effect,&mut work)?;
  admit_retained_clone_progress(g,step.progress(),"native Energy owner step")?;
  work=work.max(u128::from(units)).max(u128::from(effect.performed_work().saturating_sub(base)));
  Ok(step)
 })();conclude(effect,base,result,work,|step|step.progress(),reply)
}

/// 📦️ Allocates one exact vector under the effect's allocation marker and reports its backing bytes.
pub fn exact_vec<T>(count:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<(Vec<T>,usize),ValueError>{
 if effect.actual().is_some(){return Err(ValueError::invariant("original vector allocation marker is occupied"))}
 let bytes=semio_framework_value::retained_clone::allocation::backing::exact_vector_bytes::<T>(count)?;
 if bytes!=0{*effect.actual()=Some(0);}
 let rows=semio_framework_value::retained_clone::allocation::backing::allocate_exact_vector::<T>(count)?;
 if bytes!=0{*effect.actual()=Some(bytes);}
 Ok((rows,bytes))
}

/// 🧱️ A prepared field value closes through its paid start and its reply entries.
impl<T:RetainedClone> Cursor for protocol::RetainedFieldPreparation<T>{
 fn finished(&self)->bool{self.terminal_is_empty()}
 fn native(&mut self,e:&mut RetainedAllocationEffect<'_>,work:&mut u128)->Result<Option<RetainedCloneProgress>,semio_framework_value::RetirementPollExit>{if !self.close_started(){let turn=self.begin_close_paid(e);*work+=turn.work;return Ok(Some(turn.result?))}Ok(None)}
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.close_started(){Ok(self.begin_close_demand())}else{self.close_demand(body)}}
 fn work(&self,body:usize)->Result<u64,ValueError>{if !self.close_started(){Ok(self.begin_close_work_demand())}else{self.close_work_demand(body)}}
 fn birth(&self)->bool{self.close_started()&&self.requires_original_birth()}
 fn control(&self)->bool{self.close_started()&&self.requires_original_control()}
 fn birth_step(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->OriginalRetirementPoll{self.close_original_birth_step(o,g)}
 fn close<'r>(&mut self,o:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{entries!(self,o,body,e,r,recover)}
}
/// ⚖️ An equality owner closes through its reply-shaped start and entries.
impl<T:semio_framework_value_persistent::RetainedEquality> Cursor for semio_framework_value_persistent::RetainedEqualityOwner<T>{
 fn finished(&self)->bool{self.terminal_is_empty()}
 fn native(&mut self,e:&mut RetainedAllocationEffect<'_>,work:&mut u128)->Result<Option<RetainedCloneProgress>,semio_framework_value::RetirementPollExit>{if !self.close_started(){let step=crate::retained::native_child(e,work,|e,r|self.begin_close_into(e,r))??;return Ok(Some(step.progress()))}Ok(None)}
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.close_started(){Ok(self.begin_close_demand())}else{self.close_demand(body)}}
 fn work(&self,body:usize)->Result<u64,ValueError>{if !self.close_started(){Ok(1)}else{self.close_work_demand(body)}}
 fn birth(&self)->bool{self.close_started()&&self.requires_original_birth()}
 fn control(&self)->bool{self.close_started()&&self.requires_original_control()}
 fn birth_step(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->OriginalRetirementPoll{self.close_original_birth_step(o,g)}
 fn close<'r>(&mut self,o:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{entries!(self,o,body,e,r,recover)}
}
