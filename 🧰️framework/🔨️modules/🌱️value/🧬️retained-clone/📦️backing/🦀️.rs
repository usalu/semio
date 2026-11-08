//! 🧳️ One native clone façade retains its original typed state at one funded address.
use crate::{NativeProjectionBacking,NativeSeedTemplate};
use semio_framework_value_control_contract::{ValueError,RetirementDemand};
use semio_framework_value_retirement::{OriginalRetirementFailure,OriginalRetirementPoll,RetireOwned};
use semio_framework_value_retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneRef,RetainedCloneStep,RetainedCloneGrant,RetainedAllocationEffect,OriginalEffectReply,OriginalEffectReplyComplete,OriginalRetirementCloseLoan,RetainedCursorRelease,RetainedCloneOwner,RetainedCloneSelection,retained_clone_grant_admits};
type Reply<'a>=OriginalEffectReply<'a,RetainedCloneStep,OriginalRetirementFailure>;
type Complete<'a>=OriginalEffectReplyComplete<'a,RetainedCloneStep,OriginalRetirementFailure>;
fn invalid()->ValueError{ValueError::invariant("native clone backing changed its original retained state")}
/// 🧩️ Supplies only a native vacant seed and an exact bounded pending-action observation.
/// # Safety
/// The constant seed must satisfy its native field contract. The pending query must be pure, callback-free, nonallocating and nonunwinding, and retain every unfinished native callback or borrowed reply.
pub unsafe trait NativeRetainedCloneState<T:RetainedClone>:RetainedCloneCursor<T>+Sized{const NATIVE_SEED:NativeSeedTemplate<Self>;fn has_pending_native_action(&self)->bool;}
/// 🪴️ Keeps the actual projection cursor out of whole-cursor retirement header moves.
pub struct NativeRetainedCloneBacking<T:RetainedClone,C:NativeRetainedCloneState<T>>{backing:NativeProjectionBacking<C>,closing:bool,marker:std::marker::PhantomData<fn()->T>}
impl<T:RetainedClone,C:NativeRetainedCloneState<T>> Default for NativeRetainedCloneBacking<T,C>{fn default()->Self{Self::new()}}
impl<T:RetainedClone,C:NativeRetainedCloneState<T>> NativeRetainedCloneBacking<T,C>{
 pub const fn new()->Self{Self{backing:NativeProjectionBacking::new(C::NATIVE_SEED),closing:false,marker:std::marker::PhantomData}}
 pub fn retained_cursor(&self)->Option<&C>{self.backing.get()}
 pub const fn birth_prefix_actions()->Option<usize>{match C::NATIVE_SEED.field_parts(){Some(parts)=>parts.checked_add(2),None=>None}}
 fn pending(&self)->bool{self.backing.get().is_some_and(|cursor|cursor.has_pending_native_action())}
 fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(invalid)?;Ok(demand)}
 fn child<'a>(effect:&'a mut RetainedAllocationEffect<'_>)->Result<RetainedAllocationEffect<'a>,ValueError>{let grant=effect.grant();let work=effect.remaining_work_units()?;effect.reborrow(RetainedCloneGrant{maximum_depth:grant.maximum_depth.checked_sub(1).ok_or_else(invalid)?,..grant},work)}
 fn native_proof(&self)->Option<RetainedCursorRelease<C>>{self.backing.retained().and_then(<C as RetainedCloneCursor<T>>::native_terminal_release)}
 fn release_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{let proof=self.native_proof();let mut result=None;self.backing.release_into(proof,effect,OriginalEffectReply::borrow(&mut result)?)?;Ok(reply.complete(result.ok_or_else(invalid)?.map_err(Into::into)))}
}
impl<T:RetainedClone,C:NativeRetainedCloneState<T>> RetainedCloneCursor<T> for NativeRetainedCloneBacking<T,C>{
 fn advance_selection(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64)->Result<RetainedCloneSelection,ValueError>{if self.closing{return Err(invalid())}if maximum==0{return Ok(RetainedCloneSelection::new(Default::default(),0))}match self.backing.get(){Some(cursor)=>{let selected=cursor.advance_selection(source,body,maximum)?;Ok(RetainedCloneSelection::new(Self::nested(selected.demand())?,if selected.work()<=maximum{selected.work()}else{0}))},None=>Ok(RetainedCloneSelection::new(self.backing.birth_demand()?,self.backing.birth_work_demand()))}}
 fn advance_demands(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{Ok(self.advance_selection(source,body,maximum)?.demand())}
 fn advance_work_demand(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64)->Result<u64,ValueError>{Ok(self.advance_selection(source,body,maximum)?.work())}
 fn advance_into<'r>(&mut self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{
  if self.closing{return Err(invalid())}self.backing.validate_effect(effect)?;let selected=self.advance_selection(source,body,maximum)?;if selected.work()==0||effect.remaining_work_units()?<selected.work()||!retained_clone_grant_admits(effect.grant(),selected.demand()){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if self.backing.initialized(){return self.backing.get_mut().ok_or_else(invalid)?.advance_into(source,body,maximum,&mut Self::child(effect)?,reply)}
  let mut result=None;self.backing.birth_into(effect,OriginalEffectReply::borrow(&mut result)?)?;Ok(reply.progress().complete(result.ok_or_else(invalid)?.map_err(Into::into)))
 }
 fn recover_advance_into<'r>(&mut self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.backing.validate_effect(effect)?;self.backing.get_mut().ok_or_else(invalid)?.recover_advance_into(source,body,maximum,&mut Self::child(effect)?,reply)}
 fn output_ready(&self)->bool{!self.closing&&self.backing.get().is_some_and(<C as RetainedCloneCursor<T>>::output_ready)}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if self.closing{return Err(invalid())}Self::nested(self.backing.get().ok_or_else(invalid)?.take_demand()?)}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand()?;self.backing.get().ok_or_else(invalid)?.take_work_demand()}
 fn take<'r>(&mut self,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{if self.closing{return Err(invalid())}self.backing.validate_effect(effect)?;let demand=self.take_demand()?;let work=self.take_work_demand()?;if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<work{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}self.backing.get_mut().ok_or_else(invalid)?.take(output,&mut Self::child(effect)?,reply)}
 fn recover_take<'r>(&mut self,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{self.backing.validate_effect(effect)?;self.backing.get_mut().ok_or_else(invalid)?.recover_take(output,&mut Self::child(effect)?,reply)}
 fn close_started(&self)->bool{self.closing}
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{if self.pending(){return Err(invalid())}Ok(crate::retained_clone::retained_close_start_demand(self.closing))}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{self.begin_close_demand()?;Ok(u64::from(!self.closing))}
 fn begin_close_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.begin_close_demand()?;self.backing.validate_effect(effect)?;let turn=crate::retained_clone::start_retained_close(&mut self.closing,effect);Ok(reply.complete(turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)))}
 fn recover_begin_close_into<'r>(&mut self,_:&mut RetainedAllocationEffect<'_>,_:Reply<'r>)->Result<Complete<'r>,ValueError>{Err(invalid())}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(invalid())}if let Some(cursor)=self.backing.get(){if !cursor.close_started(){return Self::nested(cursor.begin_close_demand()?)}if !cursor.terminal_is_empty(){return Self::nested(cursor.close_demands(body)?)}}
  let proof=self.native_proof();self.backing.release_demand(proof.as_ref())
 }
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.close_demands(body)?;if let Some(cursor)=self.backing.get(){if !cursor.close_started(){return cursor.begin_close_work_demand()}if !cursor.terminal_is_empty(){return cursor.close_work_demand(body)}}Ok(self.backing.release_work_demand())}
 fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{
  if !self.closing{return Err(invalid())}self.backing.validate_effect(effect)?;if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}let demand=self.close_demands(body)?;let work=self.close_work_demand(body)?;if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<work{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if let Some(cursor)=self.backing.get_mut(){if !cursor.close_started(){return cursor.begin_close_into(&mut Self::child(effect)?,reply.progress())}if !cursor.terminal_is_empty(){return cursor.close_into(body,&mut Self::child(effect)?,reply.progress())}}
  self.release_into(effect,reply)
 }
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.backing.validate_effect(effect)?;self.backing.get_mut().ok_or_else(invalid)?.recover_close_into(body,&mut Self::child(effect)?,reply.progress())}
 fn requires_original_birth(&self)->bool{self.backing.get().is_some_and(<C as RetainedCloneCursor<T>>::requires_original_birth)}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{let check=original.original_work().binding_identity().and_then(|work|self.backing.validate_work_binding(work));if let Err(error)=check{return semio_framework_value_control_contract::RetainedWorkPoll::Complete(semio_framework_value_control_contract::RetainedWorkOutcome{result:Err(error.into()),work:0})}if grant.maximum_depth==0{return semio_framework_value_control_contract::RetainedWorkPoll::Complete(semio_framework_value_control_contract::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}match self.backing.get_mut(){Some(cursor)=>cursor.close_original_birth_step(original,RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}),None=>semio_framework_value_control_contract::RetainedWorkPoll::Complete(semio_framework_value_control_contract::RetainedWorkOutcome{result:Err(invalid().into()),work:0})}}
 fn requires_original_control(&self)->bool{self.backing.get().is_some_and(<C as RetainedCloneCursor<T>>::requires_original_control)}
 fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.backing.validate_effect(effect)?;self.backing.validate_work_binding(original.original_work().binding_identity()?)?;if !self.closing{return Err(invalid())}if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}let demand=self.close_demands(body)?;let work=self.close_work_demand(body)?;if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<work{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if let Some(cursor)=self.backing.get_mut(){if !cursor.close_started(){return cursor.begin_close_into(&mut Self::child(effect)?,reply.progress())}if !cursor.terminal_is_empty(){return cursor.close_original_into(original,body,&mut Self::child(effect)?,reply.progress())}}self.release_into(effect,reply)}
 fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.backing.validate_effect(effect)?;self.backing.validate_work_binding(original.original_work().binding_identity()?)?;self.backing.get_mut().ok_or_else(invalid)?.recover_close_original_into(original,body,&mut Self::child(effect)?,reply.progress())}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
 fn terminal_is_empty(&self)->bool{self.closing&&self.backing.terminal_is_empty()}
 fn native_terminal_release(&self)->Option<RetainedCursorRelease<Self>>{self.terminal_is_empty().then(||unsafe{RetainedCursorRelease::native(|owner:&Self|owner.closing&&owner.backing.terminal_is_empty())})}
}

impl<T,C> RetireOwned for NativeRetainedCloneBacking<T,C>where T:RetainedClone<Cursor=NativeRetainedCloneBacking<T,C>>,C:NativeRetainedCloneState<T>+'static{
 type RetirementCursor=<RetainedCloneOwner<T> as RetireOwned>::RetirementCursor;
 fn retirement_seed(self)->Self::RetirementCursor{RetainedCloneOwner::from_cursor(self).retirement_seed()}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{T::controlled_retirement_supported()}
}
