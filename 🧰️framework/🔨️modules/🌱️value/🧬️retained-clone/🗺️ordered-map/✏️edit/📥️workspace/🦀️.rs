//! 📥️ Native edit inputs share one original lookup and workspace handback lifecycle.
use super::*;
use super::staged_lease::StagedLease;
use crate::retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,OriginalEffectCallbackMode,OriginalEffectInvocationIdentity,OriginalWorkBindingIdentity,OriginalRetirementCloseLoan};
use std::mem::ManuallyDrop;
type Reply<'r>=OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>;
type Complete<'r>=OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>;
type Effect<'a>=crate::RetainedAllocationEffect<'a>;
#[derive(crate::RetireOwned)]
#[value(crate="crate")]
pub(super) struct EditWorkspace<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync>{pub map:RetainedOrderedMap<K,V>,pub key:K,pub value:Option<V>}
#[derive(Clone,Copy,PartialEq,Eq)]
struct Route{invocation:OriginalEffectInvocationIdentity,operation:u8,body:usize,comparison:BoundedOrdGrant,grant:RetainedCloneGrant,work:OriginalWorkBindingIdentity,original:Option<(crate::NativeAllocationIdentity,Option<u64>,OriginalWorkBindingIdentity)>}
/// 🧳️ Keeps every unpublished Source, lookup result and returned workspace in its actual resident slot.
pub(super) struct EditInput<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync>{
 lease:StagedLease<EditWorkspace<K,V>>,
 lookup:RetainedOrderedMapLookupCursor<K>,
 workspace:ManuallyDrop<Option<EditWorkspace<K,V>>>,
 result:Option<RetainedOrderedMapLookup>,
 close:ManuallyDrop<RetainedCloneClose>,
 phase:u8,
 closing:bool,
 binding:Option<OriginalWorkBindingIdentity>,
 route:Option<Route>,
}
impl<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync> EditInput<K,V>{
 pub fn new(map:RetainedOrderedMap<K,V>,key:K,value:Option<V>)->Self{Self{lease:StagedLease::new(EditWorkspace{map,key,value}),lookup:Default::default(),workspace:ManuallyDrop::new(None),result:None,close:ManuallyDrop::new(RetainedCloneClose::vacant()),phase:0,closing:false,binding:None,route:None}}
 fn invalid()->ValueError{ValueError::invariant("native edit input changed its original selected action")}
 fn fixed(bytes:usize)->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
 fn binding_demand()->crate::RetirementDemand{Self::fixed(size_of::<Option<OriginalWorkBindingIdentity>>())}
 fn nested(mut d:crate::RetirementDemand)->Result<crate::RetirementDemand,ValueError>{d.depth=d.depth.checked_add(1).ok_or_else(Self::invalid)?;Ok(d)}
 fn funded(effect:&Effect<'_>,d:crate::RetirementDemand,units:u64)->Result<bool,ValueError>{Ok(retained_clone_grant_admits(effect.grant(),d)&&effect.remaining_work_units()?>=units)}
 pub fn validate(&self,effect:&Effect<'_>)->Result<(),ValueError>{if self.binding.is_some_and(|binding|!binding.matches_effect(effect)){Err(Self::invalid())}else{Ok(())}}
 #[cfg(test)]
 pub fn original_key(&self)->Option<&K>{self.lease.original_authority().map(|w|&w.key)}
 pub fn comparison_progress(&self)->BoundedOrdProgress{self.lookup.comparison_totals()}
 pub fn birth_complete(&self)->bool{self.phase!=0||self.lease.ready()}
 pub fn ready(&self)->bool{self.phase==7&&self.workspace.is_some()&&self.route.is_none()&&!self.closing}
 pub fn has_pending(&self)->bool{self.route.is_some()}
 pub fn consumed(&self)->bool{self.phase==8&&self.workspace.is_none()}
 fn transition_demand()->crate::RetirementDemand{Self::fixed(size_of::<u8>())}
 fn native<'r>(&mut self,d:crate::RetirementDemand,effect:&mut Effect<'_>,reply:Reply<'r>,operation:u8)->Result<Complete<'r>,ValueError>{if !Self::funded(effect,d,1)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if operation==0&&effect.work_source_turn().is_none(){return Err(Self::invalid())}if !effect.performed(1)?{return Err(Self::invalid())}match operation{0=>self.binding=Some(OriginalWorkBindingIdentity::from_effect(effect)),1=>self.phase=self.phase.checked_add(1).ok_or_else(Self::invalid)?,2=>self.closing=true,3=>{self.result=None;self.binding=None;},_=>return Err(Self::invalid())}Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))}
 pub fn next_demand(&self,comparison:BoundedOrdGrant,body:usize)->Result<crate::RetirementDemand,ValueError>{
  if self.closing||self.route.is_some(){return Err(Self::invalid())}if self.binding.is_none(){return Ok(Self::binding_demand())}
  match self.phase{0 if !self.lease.ready()=>Self::nested(self.lease.birth_demand()?),0=>Ok(Self::transition_demand()),1 if self.lookup.outcome().is_none()=>Self::nested(self.lookup.next_demand(self.lease.borrow(1,|w|&w.map),self.lease.borrow(2,|w|&w.key),comparison,body)?),1=>Ok(Self::transition_demand()),2 if self.result.is_none()=>Self::nested(self.lookup.take_output_demand()),2=>Ok(Self::transition_demand()),3 if !self.lookup.close_started()=>Self::nested(self.lookup.begin_close_demand()),3=>Ok(Self::transition_demand()),4 if !self.lookup.terminal_is_empty()=>Self::nested(self.lookup.close_demands(body)?),4=>Ok(Self::transition_demand()),5 if !self.lease.terminal_is_empty()=>Self::nested(self.lease.next_take_demands(body)?),5=>Ok(Self::transition_demand()),6=>Ok(Self::transition_demand()),7|8=>Ok(Default::default()),_=>Err(Self::invalid())}
 }
 pub fn next_work_demand(&self,comparison:BoundedOrdGrant,body:usize)->Result<u64,ValueError>{self.next_demand(comparison,body)?;if self.binding.is_none(){return Ok(1)}match self.phase{0 if !self.lease.ready()=>self.lease.birth_work_demand(),1 if self.lookup.outcome().is_none()=>self.lookup.next_work_demand(self.lease.borrow(1,|w|&w.map),self.lease.borrow(2,|w|&w.key),comparison,body),2 if self.result.is_none()=>Ok(self.lookup.take_output_work_demand()),3 if !self.lookup.close_started()=>Ok(self.lookup.begin_close_work_demand()),4 if !self.lookup.terminal_is_empty()=>self.lookup.close_work_demand(body),5 if !self.lease.terminal_is_empty()=>self.lease.take_work_demand(body),7|8=>Ok(0),_=>Ok(1)}}
 fn key(effect:&Effect<'_>,operation:u8,comparison:BoundedOrdGrant,body:usize,original:Option<&mut OriginalRetirementCloseLoan<'_>>,recover:bool)->Result<Route,ValueError>{let invocation=effect.invocation().ok_or_else(Self::invalid)?;if invocation.mode()!=if recover{OriginalEffectCallbackMode::Recover}else{OriginalEffectCallbackMode::Receive}{return Err(Self::invalid())}let original=match original{Some(original)=>{let work=original.original_work().binding_identity()?;if !work.matches_effect(effect){return Err(Self::invalid())}Some((original.ledger_identity(),original.retirement_recipient_identity(),work))},None=>None};Ok(Route{invocation:invocation.identity(),operation,body,comparison,grant:effect.grant(),work:OriginalWorkBindingIdentity::from_effect(effect),original})}
 fn child<'r>(&mut self,operation:u8,comparison:BoundedOrdGrant,body:usize,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut Effect<'_>,reply:Reply<'r>,recover:bool)->Result<Complete<'r>,ValueError>{
  let key=Self::key(effect,operation,comparison,body,original.as_deref_mut(),recover)?;if recover&&self.route!=Some(key)||!recover&&self.route.is_some(){return Err(Self::invalid())}let mut scope=reply.progress().scope(&mut self.route,key)?;let grant=nested_retained_clone_grant(effect.grant())?;let units=effect.remaining_work_units()?;let mut nested=effect.reborrow(grant,units)?;
  let done=match operation{
   1=>{let map=self.lease.borrow(1,|w|&w.map);let target=self.lease.borrow(2,|w|&w.key);if recover{self.lookup.recover_advance_into(map,target,comparison,body,&mut nested,scope.borrow()?)}else{self.lookup.advance_into(map,target,comparison,body,&mut nested,scope.borrow()?)}},
   2=>if recover{self.lookup.recover_take_output_into(&mut self.result,&mut nested,scope.borrow()?)}else{self.lookup.take_output_into(&mut self.result,&mut nested,scope.borrow()?)},
   3=>if recover{self.lookup.recover_begin_close_into(&mut nested,scope.borrow()?)}else{self.lookup.begin_close_into(&mut nested,scope.borrow()?)},
   4=>match(original,recover){(None,false)=>self.lookup.close_into(body,&mut nested,scope.borrow()?),(None,true)=>self.lookup.recover_close_into(body,&mut nested,scope.borrow()?),(Some(original),false)=>self.lookup.close_original_into(original,body,&mut nested,scope.borrow()?),(Some(original),true)=>self.lookup.recover_close_original_into(original,body,&mut nested,scope.borrow()?)},
   5=>self.lease.take_authority_into(&mut self.workspace,body,original,&mut nested,scope.borrow()?,recover),
   6=>self.lease.close_into(body,original,&mut nested,scope.borrow()?,recover),
   7=>match(original,recover){(None,false)=>self.close.step_into(body,&mut nested,scope.borrow()?),(None,true)=>self.close.recover_step_into(body,&mut nested,scope.borrow()?),(Some(original),false)=>self.close.step_original_into(original,body,&mut nested,scope.borrow()?),(Some(original),true)=>self.close.recover_step_original_into(original,body,&mut nested,scope.borrow()?)},
   _=>Err(Self::invalid())
  }?;drop(done);Ok(scope.forwarded())
 }
 pub fn advance_into<'r>(&mut self,comparison:BoundedOrdGrant,body:usize,original:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut Effect<'_>,reply:Reply<'r>,recover:bool)->Result<Complete<'r>,ValueError>{
  self.validate(effect)?;if self.closing{return Err(Self::invalid())}if recover{let operation=self.route.ok_or_else(Self::invalid)?.operation;if operation>5{return Err(Self::invalid())}return self.child(operation,comparison,body,original,effect,reply,true)}let d=self.next_demand(comparison,body)?;let units=self.next_work_demand(comparison,body)?;if self.ready()||self.consumed(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}if !Self::funded(effect,d,units)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if self.binding.is_none(){return self.native(d,effect,reply,0)}
  let operation=match self.phase{0 if !self.lease.ready()=>{let grant=nested_retained_clone_grant(effect.grant())?;let units=effect.remaining_work_units()?;return self.lease.advance_birth_into(&mut effect.reborrow(grant,units)?,reply.progress())},1 if self.lookup.outcome().is_none()=>1,2 if self.result.is_none()=>2,3 if !self.lookup.close_started()=>3,4 if !self.lookup.terminal_is_empty()=>4,5 if !self.lease.terminal_is_empty()=>5,_=>0};if operation!=0{return self.child(operation,comparison,body,original,effect,reply,false)}self.native(d,effect,reply,1)
 }
 pub fn take_demand(&self)->Result<crate::RetirementDemand,ValueError>{if !self.ready(){return Err(Self::invalid())}Ok(Self::fixed(2*size_of::<Option<EditWorkspace<K,V>>>()+2*size_of::<Option<RetainedOrderedMapLookup>>()+size_of::<u8>()))}
 pub fn take_into<'r>(&mut self,workspace:&mut Option<EditWorkspace<K,V>>,result:&mut Option<RetainedOrderedMapLookup>,effect:&mut Effect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.validate(effect)?;let d=self.take_demand()?;if workspace.is_some()||result.is_some(){return Err(Self::invalid())}if !Self::funded(effect,d,1)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Err(Self::invalid())}*workspace=self.workspace.take();*result=self.result.take();self.phase=8;Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))}
 pub fn close_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{
  if self.route.is_some_and(|route|route.body!=body){return Err(Self::invalid())}if self.terminal_is_empty(){return Ok(Default::default())}if self.binding.is_none(){return Ok(Self::binding_demand())}if !self.closing{return Ok(Self::fixed(size_of::<bool>()))}if !self.lookup.terminal_is_empty(){return Self::nested(if !self.lookup.close_started(){self.lookup.begin_close_demand()}else{self.lookup.close_demands(body)?})}if !self.lease.terminal_is_empty(){return Self::nested(self.lease.next_close_demand(body)?)}if self.workspace.is_some()||!self.close.is_empty(){return Self::nested(self.close.next_owner_demands::<EditWorkspace<K,V>>(self.workspace.is_some(),&None,body)?)}Ok(Self::fixed(size_of::<Option<RetainedOrderedMapLookup>>()+size_of::<Option<OriginalWorkBindingIdentity>>()))
 }
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.close_demand(body)?;if self.terminal_is_empty(){return Ok(0)}if self.binding.is_none()||!self.closing{return Ok(1)}if !self.lookup.terminal_is_empty(){return if self.lookup.close_started(){self.lookup.close_work_demand(body)}else{Ok(self.lookup.begin_close_work_demand())}}if !self.lease.terminal_is_empty(){return self.lease.close_work_demand(body)}if self.workspace.is_some()||!self.close.is_empty(){return self.close.next_owner_work::<EditWorkspace<K,V>>(self.workspace.is_some(),&None,body)}Ok(1)}
 pub fn close_into<'r>(&mut self,body:usize,original:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut Effect<'_>,reply:Reply<'r>,recover:bool)->Result<Complete<'r>,ValueError>{
  self.validate(effect)?;if recover{let route=self.route.ok_or_else(Self::invalid)?;if route.operation<3||route.operation==5{return Err(Self::invalid())}return self.child(route.operation,Default::default(),body,original,effect,reply,true)}if self.route.is_some(){return Err(Self::invalid())}if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}let d=self.close_demand(body)?;if !Self::funded(effect,d,self.close_work_demand(body)?)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if self.binding.is_none(){return self.native(d,effect,reply,0)}if !self.closing{return self.native(d,effect,reply,2)}
  let operation=if !self.lookup.terminal_is_empty(){if self.lookup.close_started(){4}else{3}}else if !self.lease.terminal_is_empty(){6}else if !self.close.is_empty(){7}else{0};if operation!=0{return self.child(operation,Default::default(),body,original,effect,reply,false)}if self.workspace.is_some(){let grant=nested_retained_clone_grant(effect.grant())?;let units=effect.remaining_work_units()?;let turn=self.close.begin_granted(&mut self.workspace,&mut effect.reborrow(grant,units)?);return Ok(reply.progress().complete(turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}self.native(d,effect,reply,3)
 }
 pub fn requires_original_birth(&self)->bool{if !self.lookup.terminal_is_empty(){self.lookup.requires_original_birth()}else if !self.lease.terminal_is_empty(){self.lease.requires_original_birth()}else{self.close.requires_original_birth()}}
 pub fn requires_original_control(&self)->bool{if !self.lookup.terminal_is_empty(){self.lookup.requires_original_control()}else if !self.lease.terminal_is_empty(){self.lease.requires_original_control()}else{self.close.requires_original_control()}}
 pub fn original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if !self.lookup.terminal_is_empty(){self.lookup.close_original_birth_step(original,grant)}else if !self.lease.terminal_is_empty(){self.lease.original_birth_step(original,grant)}else{self.close.step_original_birth(original,grant)}}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.lookup.terminal_is_empty()&&self.lease.terminal_is_empty()&&self.workspace.is_none()&&self.result.is_none()&&self.close.is_empty()&&self.binding.is_none()&&self.route.is_none()}
}
impl<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync> Drop for EditInput<K,V>{fn drop(&mut self){if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.workspace);ManuallyDrop::drop(&mut self.close)}}else if !std::thread::panicking(){panic!("native edit input requires original paid retirement")}}}
