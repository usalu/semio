//! 📦️ Contiguous owned fields project through genuine fragment-copy and retirement cursors.
use super::{SnapshotProjection,SnapshotProjectionCursor};
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedClone,RetainedCloneCursor,RetainedFieldCursor,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits,retained_clone_close_demands,admit_retained_clone_progress,admit_retained_clone_close}};
use std::mem::size_of;

/// 🧺️ One explicitly owned field retains its original issuer and funds every copied fragment.
pub struct OwnedFieldProjection<T:RetainedClone>{cursor:RetainedFieldCursor<T>,closing:bool,close_route:Option<(crate::retained_clone::OriginalEffectInvocationIdentity,u8,Option<(semio_framework_value_control_contract::NativeAllocationIdentity,Option<u64>,crate::retained_clone::OriginalWorkBindingIdentity)>,(crate::RetirementDemand,u64,crate::retained_clone::OriginalWorkActionBoundary),usize)>}
impl<T:RetainedClone> OwnedFieldProjection<T>{
 /// 📏️ Quotes the fixed original-field projection body before any source is inspected.
 pub fn birth_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Self>>(),depth:1,..Default::default()}}

}
impl<T:RetainedClone+SnapshotProjection<Output=T>> OwnedFieldProjection<T>{
 fn close_dispatch<'r>(&mut self,mut original:Option<&mut crate::retained_clone::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{use crate::retained_clone::OriginalEffectCallbackMode;if !self.closing{return Err(invalid())}let invocation=effect.invocation().ok_or_else(||ValueError::invariant("projection close lacks original invocation"))?;let mode=if recover{OriginalEffectCallbackMode::Recover}else{OriginalEffectCallbackMode::Receive};if invocation.mode()!=mode||invocation.source_turn()!=effect.work_source_turn(){return Err(ValueError::invariant("projection close changed original action"))}let binding=match original.as_mut(){Some(loan)=>{let binding=loan.original_work().binding_identity()?;if !binding.matches_effect(effect){return Err(ValueError::invariant("projection original loan changed Work"))}Some((loan.ledger_identity(),loan.retirement_recipient_identity(),binding))},None=>None};let body=if recover{self.close_route.map_or(body,|key|key.4)}else{body};let route=if recover{let key=self.close_route.ok_or_else(||ValueError::invariant("projection close recovery absent"))?;if key.0!=invocation.identity()||key.2!=binding{return Err(ValueError::invariant("projection close recovery changed owner"))}if !effect.admits_recovery(key.3.0,key.3.1,key.3.2)?{return Ok(reply.pending())}key.1}else{if self.close_route.is_some(){return Err(ValueError::invariant("projection close action occupied"))}if !self.cursor.close_started(){if original.is_some(){return Err(invalid())}1}else if original.is_some(){3}else{2}};let key=(invocation.identity(),route,binding,match self.close_route{Some(key)=>key.3,None=>(self.close_demands(body)?,self.close_work_demand(body)?,effect.action_boundary())},body);let state=self;let mut scope=reply.progress().scope(&mut state.close_route,key)?;let done=match(route,recover,original){(1,false,None)=>state.cursor.begin_close_into(effect,scope.borrow()?)?,(1,true,None)=>state.cursor.recover_begin_close_into(effect,scope.borrow()?)?,(2,false,None)=>state.cursor.close_into(body,effect,scope.borrow()?)?,(2,true,None)=>state.cursor.recover_close_into(body,effect,scope.borrow()?)?,(3,false,Some(loan))=>state.cursor.close_original_into(loan,body,effect,scope.borrow()?)?,(3,true,Some(loan))=>state.cursor.recover_close_original_into(loan,body,effect,scope.borrow()?)?,_=>return Err(invalid())};drop(done);Ok(scope.forwarded())}
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"owned field projection is closing, transferred or incomplete")}
impl<T:RetainedClone+SnapshotProjection<Output=T>> SnapshotProjectionCursor<T> for OwnedFieldProjection<T>{
 fn demands(&self,original:RetainedCloneRef<'_,T>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{if self.closing{return Err(invalid())}self.cursor.advance_demands(original,body,maximum)}
 fn advance_work_demand(&self,original:RetainedCloneRef<'_,T>,body:usize,maximum:u64)->Result<u64,ValueError>{self.demands(original,body,maximum)?;self.cursor.advance_work_demand(original,body,maximum)}
 fn advance_into<'r>(&mut self,original:RetainedCloneRef<'_,T>,body:usize,maximum:u64,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if self.closing{return Err(invalid())}self.cursor.advance_into(original,body,maximum,effect,reply)}
 fn recover_advance_into<'r>(&mut self,original:RetainedCloneRef<'_,T>,body:usize,maximum:u64,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.cursor.recover_advance_into(original,body,maximum,effect,reply)}
 fn ready(&self)->bool{!self.closing&&self.cursor.output_ready()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid())}self.cursor.take_demand()}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand()?;self.cursor.take_work_demand()}
 fn take_into<'r>(&mut self,output:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{if !self.ready(){return Err(invalid())}self.cursor.take(output,effect,reply)}
 fn recover_take_into<'r>(&mut self,output:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{self.cursor.recover_take(output,effect,reply)}
 fn close_started(&self)->bool{self.closing}
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{Ok(if self.closing{Default::default()}else{RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()}})}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(!self.closing))}
 fn begin_close_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{let d=self.begin_close_demand()?;if self.closing{return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}if effect.remaining_work_units()?<1||!retained_clone_grant_admits(effect.grant(),d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"projection close start exceeded original Work").into())))}self.closing=true;Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))}
 fn recover_begin_close_into<'r>(&mut self,_:&mut crate::RetainedAllocationEffect<'_>,_:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{Err(ValueError::invariant("native projection close start has no unrecorded callback"))}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing||self.close_route.is_some(){return Err(invalid())}if !self.cursor.close_started(){self.cursor.begin_close_demand()}else{self.cursor.close_demands(body)}}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.close_demands(body)?;if !self.cursor.close_started(){self.cursor.begin_close_work_demand()}else{self.cursor.close_work_demand(body)}}
 fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_dispatch(None,body,effect,reply,false)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_dispatch(None,body,effect,reply,true)}
 fn requires_original_birth(&self)->bool{self.closing&&self.close_route.is_none()&&self.cursor.close_started()&&self.cursor.requires_original_birth()}
 fn close_original_birth_step(&mut self,original:&mut crate::retained_clone::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{self.cursor.close_original_birth_step(original,grant)}
 fn requires_original_control(&self)->bool{self.closing&&(self.close_route.is_some_and(|(_,route,_,_,_)|route==3)||self.cursor.close_started()&&self.cursor.requires_original_control())}
 fn close_original_into<'r>(&mut self,original:&mut crate::retained_clone::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_dispatch(Some(original),body,effect,reply,false)}
 fn recover_close_original_into<'r>(&mut self,original:&mut crate::retained_clone::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_dispatch(Some(original),body,effect,reply,true)}
 fn terminal_is_empty(&self)->bool{self.closing&&self.close_route.is_none()&&self.cursor.terminal_is_empty()}
 fn native_terminal_release(&self)->Option<crate::retained_clone::RetainedCursorRelease<Self>>{let witness=self.cursor.native_terminal_release()?;if !self.closing||self.close_route.is_some()||!witness.validates(&self.cursor){return None}Some(unsafe{crate::retained_clone::RetainedCursorRelease::native(|owner:&Self|owner.closing&&owner.close_route.is_none()&&owner.cursor.native_terminal_release().is_some_and(|witness|witness.validates(&owner.cursor)))})}
}
unsafe impl SnapshotProjection for String{
 type Output=Self;
 type Projection=OwnedFieldProjection<Self>;
 fn projection_birth_demand()->RetirementDemand{Self::Projection::birth_demand()}
 fn projection_birth_work_demand()->Result<u64,ValueError>{Ok(1)}
 fn projection_seed()->Self::Projection{Self::Projection{cursor:Default::default(),closing:false,close_route:None}}
}
unsafe impl<T:RetainedClone> SnapshotProjection for Vec<T>{
 type Output=Self;
 type Projection=OwnedFieldProjection<Self>;
 fn projection_birth_demand()->RetirementDemand{Self::Projection::birth_demand()}
 fn projection_birth_work_demand()->Result<u64,ValueError>{Ok(1)}
 fn projection_seed()->Self::Projection{Self::Projection{cursor:Default::default(),closing:false,close_route:None}}
}

unsafe impl<T:RetainedClone> SnapshotProjection for Option<T>{
 type Output=Self;
 type Projection=OwnedFieldProjection<Self>;
 fn projection_birth_demand()->RetirementDemand{Self::Projection::birth_demand()}
 fn projection_birth_work_demand()->Result<u64,ValueError>{Ok(1)}
 fn projection_seed()->Self::Projection{Self::Projection{cursor:Default::default(),closing:false,close_route:None}}
}

unsafe impl<T:RetainedClone,const N:usize> SnapshotProjection for [T;N]{
 type Output=Box<Self>;
 type Projection=super::BoundedArrayProjection<T,N>;
 fn projection_birth_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Self::Projection>>(),depth:1,..Default::default()}}
 fn projection_birth_work_demand()->Result<u64,ValueError>{Ok(1)}
 fn projection_seed()->Self::Projection{super::BoundedArrayProjection::new()}
 fn native_copy()->Option<crate::retained_clone::RetainedCopy<Self>>{T::native_copy().map(|copy|copy.array_witness::<N>())}
}
