//! 📽️ Required immutable snapshot projections retain original roots and separately fund every owner transfer.
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retirement::RetireOwned,shared_issuance::SealedShared,retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits,admit_retained_clone_progress,admit_retained_clone_close}};
use std::mem::{ManuallyDrop,size_of};
pub use std::mem::MaybeUninit;
#[path="📋️field/🦀️.rs"]
mod field;
pub use field::ProjectionField;
#[path="🪡️captured/🦀️.rs"] mod captured;
pub use captured::CapturedProjectionField;
#[path="📦️owned/🦀️.rs"]
mod owned;
pub use owned::OwnedFieldProjection;
mod output_placement{pub trait Sealed<T>{}impl<T> Sealed<T> for T{}impl<T> Sealed<T> for Box<T>{}}
/// 🪑️ Only the firstparty value and boxed-value output forms can initialize an admitted field.
/// The private seal fixes placement behavior; refusals return the same owner and leave the destination unchanged.
pub trait ProjectionOutput<T>:output_placement::Sealed<T>+RetireOwned+Sync+Sized{
 fn projected_value(&self)->&T;
 fn projected_value_mut(&mut self)->&mut T;
 fn placement_demand()->RetirementDemand;
 fn place_into<'r>(input:&mut Option<Self>,target:&mut MaybeUninit<T>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>;
}
impl<T:RetireOwned+Sync> ProjectionOutput<T> for T{
 fn projected_value(&self)->&T{self}
 fn projected_value_mut(&mut self)->&mut T{self}
 fn placement_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Self>>()+size_of::<T>(),depth:1,..Default::default()}}
 fn place_into<'r>(input:&mut Option<Self>,target:&mut MaybeUninit<T>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{if input.is_none(){return Err(invalid("projection placement input absent"))}let d=<Self as ProjectionOutput<T>>::placement_demand();if effect.remaining_work_units()?<1||!retained_clone_grant_admits(effect.grant(),d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"projection placement exceeded original Work"))))}let value=input.take().ok_or_else(||invalid("projection placement input absent"))?;target.write(value);Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))))}
}
impl<T:RetireOwned+Sync> ProjectionOutput<T> for Box<T>{
 fn projected_value(&self)->&T{self.as_ref()}
 fn projected_value_mut(&mut self)->&mut T{self.as_mut()}
 fn placement_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Self>>()+size_of::<T>(),release_bytes:size_of::<T>(),depth:1,..Default::default()}}
 fn place_into<'r>(input:&mut Option<Self>,target:&mut MaybeUninit<T>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{if input.is_none(){return Err(invalid("projection placement input absent"))}let d=<Self as ProjectionOutput<T>>::placement_demand();if effect.remaining_work_units()?<1||!retained_clone_grant_admits(effect.grant(),d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"projection placement exceeded original Work"))))}let value=input.take().ok_or_else(||invalid("projection placement input absent"))?;target.write(*value);Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))))}
}
/// 🧭️ Snapshot preparation declares its original immutable projection issuer instead of inheriting a whole-owner clone.
///
/// # Safety
/// The cursor seed must be bounded, nonallocating, callback-free and nonunwinding.
pub unsafe trait SnapshotProjection:RetireOwned+Sync+Sized{
 type Output:ProjectionOutput<Self>;
 type Projection:SnapshotProjectionCursor<Self>;
 fn projection_birth_demand()->RetirementDemand;
 fn projection_birth_work_demand()->Result<u64,ValueError>;
 fn projection_seed()->Self::Projection;
 /// 📋️ Supplies only genuine native Copy evidence for this original value.
 fn native_copy()->Option<crate::retained_clone::RetainedCopy<Self>>{None}
}
/// 🧳️ Original-bound cursors own every staged candidate through refusal, transfer and cancellation.
pub trait SnapshotProjectionCursor<S:SnapshotProjection>:Send+'static{
 fn demands(&self,original:RetainedCloneRef<'_,S>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>;
 fn advance_work_demand(&self,original:RetainedCloneRef<'_,S>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>;
 fn advance_into<'r>(&mut self,original:RetainedCloneRef<'_,S>,body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;
 fn recover_advance_into<'r>(&mut self,original:RetainedCloneRef<'_,S>,body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;
 fn ready(&self)->bool;
 fn take_demand(&self)->Result<RetirementDemand,ValueError>;
 fn take_work_demand(&self)->Result<u64,ValueError>;
 fn take_into<'r>(&mut self,output:&mut Option<S::Output>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>;
 fn recover_take_into<'r>(&mut self,output:&mut Option<S::Output>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>;
 fn close_started(&self)->bool;
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>;
 fn begin_close_work_demand(&self)->Result<u64,ValueError>;
 fn begin_close_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;
 fn recover_begin_close_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>;
 fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;
 fn requires_original_birth(&self)->bool;
 fn close_original_birth_step(&mut self,original:&mut crate::retained_clone::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll;
 fn requires_original_control(&self)->bool;
 fn close_original_into<'r>(&mut self,original:&mut crate::retained_clone::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;
 fn recover_close_original_into<'r>(&mut self,original:&mut crate::retained_clone::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;
 fn terminal_is_empty(&self)->bool;
 fn native_terminal_release(&self)->Option<crate::retained_clone::RetainedCursorRelease<Self>> where Self:Sized{None}
}
/// 🌳️ Fixed immutable roots project original authorities without traversing or cloning variable payloads.
///
/// # Safety
/// Implementations must publish through the supplied slot and reply using bounded, callback-free, nonallocating and nonunwinding native operations.
pub unsafe trait ImmutableRootProjection:RetireOwned+Sync+Sized{
 /// 📋️ Supplies native Copy evidence only for an actual Copy root.
 fn root_native_copy()->Option<crate::retained_clone::RetainedCopy<Self>>{None}
 fn root_projection_demand(original:RetainedCloneRef<'_,Self>)->Result<RetirementDemand,ValueError>;
 fn root_projection_work_demand(original:RetainedCloneRef<'_,Self>)->Result<u64,ValueError>;
 fn project_root_into<'r>(original:RetainedCloneRef<'_,Self>,output:&mut Option<Self>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;
 fn recover_project_root_into<'r>(original:RetainedCloneRef<'_,Self>,output:&mut Option<Self>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;
}
/// 🎟️ One original root binding and one retained candidate have independently paid output and closure frontiers.
pub struct ImmutableRootCursor<S:ImmutableRootProjection>{binding:ManuallyDrop<Option<RetainedCloneBinding>>,output:ManuallyDrop<Option<S>>,retirement:ManuallyDrop<RetainedCloneClose>,projected:bool,admitted:bool,closing:bool,project_route:Option<(crate::retained_clone::OriginalEffectInvocationIdentity,(u64,usize,usize,usize),usize,u64,RetainedCloneGrant)>,close_route:Option<(crate::retained_clone::OriginalEffectInvocationIdentity,u8,Option<(semio_framework_value_control_contract::NativeAllocationIdentity,Option<u64>,crate::retained_clone::OriginalWorkBindingIdentity)>,RetainedCloneGrant,usize)>}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
unsafe impl<S:ImmutableRootProjection> SnapshotProjection for S{
 type Output=Self;
 type Projection=ImmutableRootCursor<S>;
 fn projection_birth_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Self::Projection>>(),depth:1,..Default::default()}}
 fn projection_birth_work_demand()->Result<u64,ValueError>{Ok(1)}
 fn projection_seed()->Self::Projection{ImmutableRootCursor{binding:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:ManuallyDrop::new(Default::default()),projected:false,admitted:false,closing:false,project_route:None,close_route:None}}
 fn native_copy()->Option<crate::retained_clone::RetainedCopy<Self>>{S::root_native_copy()}
}
impl<S:ImmutableRootProjection> SnapshotProjectionCursor<S> for ImmutableRootCursor<S>{
 fn demands(&self,original:RetainedCloneRef<'_,S>,_body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||self.projected||self.project_route.is_some(){return Err(invalid("immutable projection is closing or already prepared"));}if maximum_work_units==0{return Ok(Default::default())}if self.output.is_some(){return Ok(RetirementDemand{copy_bytes:2*size_of::<bool>(),depth:1,..Default::default()})}
  if let Some(demand)=original.binding_demand(&self.binding)?{return Ok(demand)}
  S::root_projection_demand(original)
 }
 fn advance_work_demand(&self,original:RetainedCloneRef<'_,S>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.demands(original,body,maximum_work_units)?;if maximum_work_units==0{return Ok(0)}if self.output.is_some(){return Ok(1)}if !original.check_binding(&self.binding)?{return original.binding_work_demand(&self.binding)}S::root_projection_work_demand(original)}
 fn advance_into<'r>(&mut self,original:RetainedCloneRef<'_,S>,body:usize,maximum:u64,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{let d=self.demands(original,body,maximum)?;let work=self.advance_work_demand(original,body,maximum)?;if work==0||effect.remaining_work_units()?<work||!retained_clone_grant_admits(effect.grant(),d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if self.output.is_some(){if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"projection publication exceeded original Work").into())))}self.projected=true;self.admitted=true;return Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))}if !original.check_binding(&self.binding)?{let turn=original.bind(&mut self.binding,effect);return Ok(reply.complete(turn.result.map(|step|RetainedCloneStep::Progress(step.unwrap_or_default())).map_err(Into::into)))}let invocation=effect.invocation().ok_or_else(||invalid("projection lacks original invocation"))?;if invocation.mode()!=crate::retained_clone::OriginalEffectCallbackMode::Receive||invocation.source_turn()!=effect.work_source_turn(){return Err(invalid("projection changed original action"))}let key=(invocation.identity(),original.reference_identity(),body,maximum,effect.grant());let state=self;let mut scope=reply.progress().scope(&mut state.project_route,key)?;let done=S::project_root_into(original,&mut state.output,effect,scope.borrow()?)?;drop(done);Ok(scope.forwarded())}
 fn recover_advance_into<'r>(&mut self,original:RetainedCloneRef<'_,S>,body:usize,maximum:u64,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{let key=self.project_route.ok_or_else(||invalid("projection recovery absent"))?;let invocation=effect.invocation().ok_or_else(||invalid("projection recovery lacks invocation"))?;if invocation.mode()!=crate::retained_clone::OriginalEffectCallbackMode::Recover||invocation.identity()!=key.0||original.reference_identity()!=key.1||body!=key.2||maximum!=key.3||effect.grant()!=key.4||invocation.source_turn()!=effect.work_source_turn(){return Err(invalid("projection recovery changed original action"))}let state=self;let mut scope=reply.progress().scope(&mut state.project_route,key)?;let done=S::recover_project_root_into(original,&mut state.output,effect,scope.borrow()?)?;drop(done);Ok(scope.forwarded())}
 fn ready(&self)->bool{!self.closing&&self.admitted&&self.output.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid("immutable projection has no admitted output"));}Ok(RetirementDemand{copy_bytes:2*size_of::<Option<S>>(),depth:1,..Default::default()})}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand()?;Ok(1)}
 fn take_into<'r>(&mut self,output:&mut Option<S>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{if output.is_some(){return Err(invalid("immutable projection output occupied"))}let d=self.take_demand()?;if effect.remaining_work_units()?<1||!retained_clone_grant_admits(effect.grant(),d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"projection output exceeded original Work"))))}*output=self.output.take();Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))}
 fn recover_take_into<'r>(&mut self,output:&mut Option<S>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{let _=(output,effect,reply);Err(invalid("native projection output has no unrecorded callback"))}
 
 fn close_started(&self)->bool{self.closing}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(!self.closing))}
 fn begin_close_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{let flags=usize::from(!self.closing);Ok(crate::RetirementDemand{copy_bytes:flags*std::mem::size_of::<bool>(),depth:usize::from(flags!=0),..Default::default()})}
fn begin_close_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if self.project_route.is_some(){return Err(invalid("projection retains original action"))}if self.closing{return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}let d=self.begin_close_demand()?;if effect.remaining_work_units()?<1||!retained_clone_grant_admits(effect.grant(),d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"projection close exceeded original Work").into())))}self.closing=true;Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))}
 fn recover_begin_close_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{let _=(effect,reply);Err(invalid("native projection close start has no unrecorded callback"))}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid("immutable projection closure has not begun"));}self.retirement.next_owner_demands::<S>(self.output.is_some(),&self.binding,body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid("immutable projection closure has not begun"))}self.retirement.next_owner_work::<S>(self.output.is_some(),&self.binding,body)}
 fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_dispatch(None,body,effect,reply,false)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_dispatch(None,body,effect,reply,true)}
 fn requires_original_birth(&self)->bool{if self.close_route.is_some(){return false}if !self.retirement.is_empty(){self.retirement.requires_original_birth()}else if self.output.is_none(){RetainedCloneBinding::requires_original_birth(&self.binding)}else{false}}
 fn close_original_birth_step(&mut self,original:&mut crate::retained_clone::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if !self.retirement.is_empty(){self.retirement.step_original_birth(original,grant)}else{RetainedCloneBinding::close_original_birth_step(&mut self.binding,original,grant)}}
 fn requires_original_control(&self)->bool{if self.close_route.is_some_and(|(_,r,_,_,_)|r>=4){return true}if !self.retirement.is_empty(){self.retirement.requires_original_control()}else if self.output.is_none(){RetainedCloneBinding::requires_original_control(&self.binding)}else{false}}
 fn close_original_into<'r>(&mut self,original:&mut crate::retained_clone::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_dispatch(Some(original),body,effect,reply,false)}
 fn recover_close_original_into<'r>(&mut self,original:&mut crate::retained_clone::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_dispatch(Some(original),body,effect,reply,true)}
 fn terminal_is_empty(&self)->bool{self.closing&&self.project_route.is_none()&&self.close_route.is_none()&&self.output.is_none()&&self.binding.is_none()&&self.retirement.is_empty()}
 fn native_terminal_release(&self)->Option<crate::retained_clone::RetainedCursorRelease<Self>>{if !self.terminal_is_empty(){return None}Some(unsafe{crate::retained_clone::RetainedCursorRelease::native(|owner:&Self|owner.terminal_is_empty())})}
}
impl<S:ImmutableRootProjection> ImmutableRootCursor<S>{
 fn close_dispatch<'r>(&mut self,mut original:Option<&mut crate::retained_clone::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{use crate::retained_clone::OriginalEffectCallbackMode;if !self.closing||self.project_route.is_some(){return Err(invalid("immutable projection closure is not ready"))}let invocation=effect.invocation().ok_or_else(||invalid("projection close lacks original invocation"))?;let mode=if recover{OriginalEffectCallbackMode::Recover}else{OriginalEffectCallbackMode::Receive};if invocation.mode()!=mode||invocation.source_turn()!=effect.work_source_turn(){return Err(invalid("projection close changed original action"))}let binding=match original.as_mut(){Some(loan)=>{let binding=loan.original_work().binding_identity()?;if !binding.matches_effect(effect){return Err(ValueError::invariant("projection original loan changed Work"))}Some((loan.ledger_identity(),loan.retirement_recipient_identity(),binding))},None=>None};let route=if recover{let key=self.close_route.ok_or_else(||invalid("projection close recovery absent"))?;if key.0!=invocation.identity()||key.2!=binding||key.3!=effect.grant()||key.4!=body{return Err(invalid("projection close recovery changed owner"))}key.1}else{if self.close_route.is_some(){return Err(invalid("projection close action occupied"))}if !self.retirement.is_empty(){if original.is_some(){4}else{1}}else if self.output.is_some(){if original.is_some(){return Err(invalid("projection output birth has no original route"))}2}else if self.binding.is_some(){if original.is_some(){5}else{3}}else{return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}};if route==2{let turn=self.retirement.begin_granted(&mut self.output,effect);return Ok(reply.complete(turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}let key=(invocation.identity(),route,binding,effect.grant(),body);let state=self;let mut scope=reply.progress().scope(&mut state.close_route,key)?;let done=match(route,recover,original){(1,false,None)=>state.retirement.step_into(body,effect,scope.borrow()?)?,(1,true,None)=>state.retirement.recover_step_into(body,effect,scope.borrow()?)?,(3,false,None)=>RetainedCloneBinding::close_into(&mut state.binding,body,effect,scope.borrow()?)?,(3,true,None)=>RetainedCloneBinding::recover_close_into(&mut state.binding,body,effect,scope.borrow()?)?,(4,false,Some(loan))=>state.retirement.step_original_into(loan,body,effect,scope.borrow()?)?,(4,true,Some(loan))=>state.retirement.recover_step_original_into(loan,body,effect,scope.borrow()?)?,(5,false,Some(loan))=>RetainedCloneBinding::close_original_into(&mut state.binding,loan,body,effect,scope.borrow()?)?,(5,true,Some(loan))=>RetainedCloneBinding::recover_close_original_into(&mut state.binding,loan,body,effect,scope.borrow()?)?,_=>return Err(invalid("projection close route changed"))};drop(done);Ok(scope.forwarded())}
}
impl<S:ImmutableRootProjection> Drop for ImmutableRootCursor<S>{fn drop(&mut self){if std::thread::panicking(){return}assert!(self.project_route.is_none()&&self.close_route.is_none()&&self.output.is_none()&&self.binding.is_none()&&self.retirement.is_empty(),"immutable projection retains original candidate, binding or action");unsafe{ManuallyDrop::drop(&mut self.binding);ManuallyDrop::drop(&mut self.output);ManuallyDrop::drop(&mut self.retirement)}}}
unsafe impl<T:Send+Sync+'static> ImmutableRootProjection for SealedShared<T>{
 fn root_projection_demand(_:RetainedCloneRef<'_,Self>)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:Self::duplicate_copy_bytes(),depth:1,..Default::default()})}
 fn root_projection_work_demand(original:RetainedCloneRef<'_,Self>)->Result<u64,ValueError>{original.get().duplicate_work_demand()}
 fn project_root_into<'r>(original:RetainedCloneRef<'_,Self>,output:&mut Option<Self>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{let turn=original.get().duplicate_into(output,effect);Ok(reply.complete(turn.result.map_err(Into::into)))}
 fn recover_project_root_into<'r>(_:RetainedCloneRef<'_,Self>,_:&mut Option<Self>,_:&mut crate::RetainedAllocationEffect<'_>,_:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{Err(invalid("native sealed projection has no unrecorded callback"))}
}
macro_rules! scalar_roots{($($kind:ty),*$(,)?)=>{$(unsafe impl ImmutableRootProjection for $kind{
 fn root_native_copy()->Option<crate::retained_clone::RetainedCopy<Self>>{Some(crate::retained_clone::RetainedCopy::new())}
 fn root_projection_demand(_:RetainedCloneRef<'_,Self>)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:size_of::<Self>()+size_of::<Option<Self>>(),depth:1,..Default::default()})}
 fn root_projection_work_demand(_:RetainedCloneRef<'_,Self>)->Result<u64,ValueError>{Ok(1)}
 fn project_root_into<'r>(original:RetainedCloneRef<'_,Self>,output:&mut Option<Self>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if output.is_some(){return Err(invalid("scalar projection output occupied"))}let d=Self::root_projection_demand(original)?;if effect.remaining_work_units()?<1||!retained_clone_grant_admits(effect.grant(),d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"scalar projection exceeded original Work").into())))}*output=Some(*original.get());Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))}
 fn recover_project_root_into<'r>(_:RetainedCloneRef<'_,Self>,_:&mut Option<Self>,_:&mut crate::RetainedAllocationEffect<'_>,_:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{Err(invalid("native scalar projection has no unrecorded callback"))}
 })*};}
scalar_roots!((),bool,char,u8,u16,u32,u64,u128,usize,i8,i16,i32,i64,i128,isize,f32,f64);
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;


#[path="👁️post/🦀️.rs"]
pub mod post;

#[cfg(test)]
#[path="🎟️work/🧪️tests/🦀️.rs"]
mod actual_work_tests;

#[path="🧮️array/🦀️.rs"]mod array_projection;
pub use array_projection::{ArrayProjection,BoundedArrayProjection};
#[path="🧮️array/📍️placement.rs"]mod array_placement;
pub use array_placement::NativeArrayFieldPlacement;

#[path="🧬️clone/🦀️.rs"]pub mod immutable_clone;

pub use semio_framework_value_native_clone_backing::{NativeProjectionBacking,NativeSeedTemplate};
