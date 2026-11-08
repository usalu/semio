//! 🧷️ Native typed projection admission expands beside its original Source binding owner.
#[macro_export]
macro_rules! retained_projection_admission_engine {()=>{
/// 🧷️ Retains the actual issued child alias before caller-slot projection publication.
pub struct RetainedOwnedProjectionAdmission<T:?Sized+Sync>{binding:Option<RetainedCloneBinding>,value:Option<NonNull<T>>}
/// 🚚️ The retained immutable root owns every referenced byte.
unsafe impl<T:?Sized+Sync> Send for RetainedOwnedProjectionAdmission<T>{}
/// 🪢️ Only shared immutable references escape this alias owner.
unsafe impl<T:?Sized+Sync> Sync for RetainedOwnedProjectionAdmission<T>{}
impl<T:?Sized+Sync> Default for RetainedOwnedProjectionAdmission<T>{fn default()->Self{Self{binding:None,value:None}}}
impl<T:?Sized+Sync> RetainedOwnedProjectionAdmission<T>{
 pub fn bound_close_available(&self)->bool{RetainedCloneBinding::bound_close_available(&self.binding)}
 fn header_demand(&self)->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:usize::from(self.value.is_some())*size_of::<Option<NonNull<T>>>(),depth:1,..Default::default()}}
 fn clear_value(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedCloneStep,crate::OriginalRetirementFailure>{
  let before=effect.performed_work();let result=(||{if self.value.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()))}let demand=self.header_demand();if !crate::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<1{return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"projection header clear exceeded original work").into())}self.value=None;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})();crate::RetainedWorkOutcome{result,work:effect.performed_work()-before}
 }
 /// 🪢️ Quotes the genuine borrowed alias before its separately funded typed-pointer clear.
 pub fn bound_close_demand(&self,source:RetainedCloneRef<'_,T>,body:usize)->Result<crate::RetirementDemand,ValueError>{if self.binding.is_none(){Ok(self.header_demand())}else{RetainedCloneBinding::bound_close_demand(&self.binding,source,body)}}
 pub fn bound_close_work_demand(&self,source:RetainedCloneRef<'_,T>,body:usize)->Result<u64,ValueError>{if self.binding.is_none(){Ok(u64::from(self.value.is_some()))}else{RetainedCloneBinding::bound_close_work_demand(&self.binding,source,body)}}
 /// 📥️ Keeps the pointer metadata resident until its own funded native clear.
 pub fn close_bound_into<'r>(&mut self,source:RetainedCloneRef<'_,T>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if self.binding.is_none(){return Ok(reply.complete(self.clear_value(effect).result))}RetainedCloneBinding::close_bound_into(&mut self.binding,source,body,effect,reply.progress())}
 pub fn recover_close_bound_into<'r>(&mut self,_source:RetainedCloneRef<'_,T>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.recover_close_into(body,effect,reply)}

 /// 📏️ Quotes the defining alias placement and retained projected reference header.
 pub fn demand(&self,source:RetainedCloneRef<'_,T>)->Result<crate::RetirementDemand,ValueError>{
  match source.binding_demand(&self.binding)?{Some(mut demand)=>{demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<Option<NonNull<T>>>()).ok_or_else(||ValueError::invariant("projection admission copy extent overflow"))?;Ok(demand)},None=>{let retained=self.borrow()?;if !std::ptr::eq(retained.get(),source.get()){return Err(ValueError::invariant("projection reference metadata changed"))}Ok(Default::default())}}
 }
 pub fn work_demand(&self,source:RetainedCloneRef<'_,T>)->Result<u64,ValueError>{self.demand(source)?;source.binding_work_demand(&self.binding)}
 /// 📥️ Captures the actual immutable child alias before recording its synchronous native reply.
 pub fn admit<'reply>(&mut self,source:RetainedCloneRef<'_,T>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{
  let result=(||{let demand=self.demand(source)?;if self.binding.is_some(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !crate::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<self.work_demand(source)?{return Ok(RetainedCloneStep::Progress(Default::default()))}
   let turn=source.bind(&mut self.binding,effect);let mut progress=turn.result?.unwrap_or_default();if self.binding.is_none(){return Ok(RetainedCloneStep::Progress(progress))}self.value=Some(NonNull::from(source.get()));progress.copied_bytes=progress.copied_bytes.checked_add(size_of::<Option<NonNull<T>>>()).expect("preflight projection header extent");Ok(RetainedCloneStep::Complete(progress))
  })();Ok(reply.complete(result))
 }
 pub fn recover_admit<'reply>(&mut self,reply:crate::OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{Ok(reply.pending())}
 /// 📖️ Reborrows the exact retained child without repeating its original getter.
 pub fn borrow(&self)->Result<RetainedCloneRef<'_,T>,ValueError>{let binding=self.binding.as_ref().ok_or_else(||ValueError::invariant("projection admission has no original binding"))?;let lease=binding.lease.as_ref().ok_or_else(||ValueError::invariant("projection admission binding is closing"))?;let value=self.value.ok_or_else(||ValueError::invariant("projection admission has no retained reference"))?;Ok(RetainedCloneRef{value:unsafe{value.as_ref()},lease,projection:binding.projection})}
 /// 📏️ Quotes only actual resident header movement into the caller output.
 pub fn take_demand(&self)->Result<crate::RetirementDemand,ValueError>{self.borrow()?;Ok(crate::RetirementDemand{copy_bytes:size_of::<Option<RetainedCloneBinding>>()+size_of::<Option<NonNull<T>>>()+size_of::<RetainedOwnedProjection<T>>()+size_of::<Option<RetainedOwnedProjection<T>>>(),depth:1,..Default::default()})}
 pub fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand()?;Ok(1)}
 /// 📤️ Publishes the retained child before recording its reply, without a Source callback.
 pub fn take<'reply>(&mut self,output:&mut Option<RetainedOwnedProjection<T>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{
  let result=(||{if output.is_some(){return Err(ValueError::invariant("projection output slot is occupied"))}let demand=self.take_demand()?;if !crate::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<1{return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"projection publication exceeded original work"))}let projection=self.binding.as_ref().expect("validated original binding").projection;let value=self.value.take().expect("validated retained reference");let binding=self.binding.take().expect("validated original binding");*output=Some(RetainedOwnedProjection{value,binding:ManuallyDrop::new(Some(binding)),projection});Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})();Ok(reply.complete(result))
 }
 pub fn recover_take<'reply>(&mut self,reply:crate::OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{Ok(reply.pending())}
 pub fn close_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{if self.binding.is_none(){Ok(self.header_demand())}else{RetainedCloneBinding::close_demand(&self.binding,body)}}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if self.binding.is_none(){Ok(u64::from(self.value.is_some()))}else{RetainedCloneBinding::close_work_demand(&self.binding,body)}}
 pub fn close_step(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{if self.binding.is_none(){crate::RetainedWorkPoll::Complete(self.clear_value(effect))}else{RetainedCloneBinding::close_one(&mut self.binding,body,effect).map(|step|RetainedCloneStep::Progress(step.progress()))}}
 pub fn requires_original_birth(&self)->bool{RetainedCloneBinding::requires_original_birth(&self.binding)}
 pub fn requires_original_control(&self)->bool{RetainedCloneBinding::requires_original_control(&self.binding)}
    /// 📥️ Lends the actual original alias close reply without moving its owner.
    pub fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if self.binding.is_none(){return Ok(reply.complete(self.clear_value(effect).result))}RetainedCloneBinding::close_into(&mut self.binding,body,effect,reply.progress())}
    /// 🔁️ Recovers the same recorded alias close without selecting another child.
    pub fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if self.binding.is_none(){return Err(ValueError::invariant("native projection header clear has no unrecorded action"))}RetainedCloneBinding::recover_close_into(&mut self.binding,body,effect,reply.progress())}
    /// 🧵️ Lends the same original close control to the retained alias producer.
    pub fn close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if self.binding.is_none(){return Ok(reply.complete(self.clear_value(effect).result))}RetainedCloneBinding::close_original_into(&mut self.binding,original,body,effect,reply.progress())}
    /// 🪡️ Recovers the same original-control alias close and reply.
    pub fn recover_close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if self.binding.is_none(){return Err(ValueError::invariant("native projection header clear has no unrecorded action"))}RetainedCloneBinding::recover_close_original_into(&mut self.binding,original,body,effect,reply.progress())}

 pub fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{RetainedCloneBinding::close_original_birth_step(&mut self.binding,original,grant)}
 pub fn close_original_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{if self.binding.is_none(){crate::RetainedWorkPoll::Complete(self.clear_value(effect))}else{RetainedCloneBinding::close_original_step(&mut self.binding,original,body,effect).map(|step|RetainedCloneStep::Progress(step.progress()))}}
 pub fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.value.is_none()}
}
impl<T:?Sized+Sync> Drop for RetainedOwnedProjectionAdmission<T>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original projection admission requires funded alias and metadata closure")}}

};}

/// 🔬️ Defines the native field witness beside the retained projection admission owner.
#[macro_export]
macro_rules! retained_native_field_projection_engine {()=>{
/// 🧭️ A fixed native field accessor is admitted only through its original retained Source.
pub struct RetainedNativeFieldProjection<S:?Sized,T:?Sized>{accessor:for<'a> fn(&'a S)->&'a T,discriminator:usize}
impl<S:?Sized+Sync,T:?Sized+Sync> RetainedNativeFieldProjection<S,T>{
 /// 🪪️ Creates a witness for one actual initialized native field.
 ///
 /// # Safety
 /// The accessor must return only that field of its input and be bounded, callback-free, nonallocating and nonunwinding.
 pub const unsafe fn new(accessor:for<'a> fn(&'a S)->&'a T,discriminator:usize)->Self{Self{accessor,discriminator}}
 /// 📏️ Quotes the actual original alias and resident field-pointer headers.
 pub fn demand(&self,source:RetainedCloneRef<'_,S>,output:&RetainedOwnedProjectionAdmission<T>)->Result<crate::RetirementDemand,ValueError>{if !output.terminal_is_empty(){return Err(ValueError::invariant("native field projection output occupied"))}let mut demand=source.binding_demand(&output.binding)?.ok_or_else(||ValueError::invariant("native field projection binding occupied"))?;demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<Option<NonNull<T>>>()).ok_or_else(||ValueError::invariant("native field projection extent overflow"))?;Ok(demand)}
 /// ⛽️ Quotes the fixed native getter and its genuine original alias work.
 pub fn work_demand(&self,source:RetainedCloneRef<'_,S>,output:&RetainedOwnedProjectionAdmission<T>)->Result<u64,ValueError>{self.demand(source,output)?;source.binding_work_demand(&output.binding)?.checked_add(1).ok_or_else(||ValueError::invariant("native field projection work overflow"))}
 /// 📥️ Publishes the actual projected alias before recording its borrowed reply.
 pub fn admit_into<'r>(&self,source:RetainedCloneRef<'_,S>,output:&mut RetainedOwnedProjectionAdmission<T>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<crate::OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{let demand=self.demand(source,output)?;let work=self.work_demand(source,output)?;if effect.work_source_turn().is_none(){return Err(ValueError::invariant("native field projection lacks original Source"))}if effect.remaining_work_units()?<work||!crate::retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native field projection exceeded original Work"))))}let field=source.project(self.discriminator,self.accessor);output.admit(field,effect,reply)}
}
};}
