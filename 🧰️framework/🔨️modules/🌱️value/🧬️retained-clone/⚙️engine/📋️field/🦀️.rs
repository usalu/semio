//! 📋️ Actual lazy field allocation, borrowed child forwarding and lifecycle engine.
#[macro_export]
macro_rules! field_cursor_engine{()=>{
use super::*;

pub struct RetainedFieldCursor<T: RetainedClone> {
    child: ManuallyDrop<Option<Box<T::Cursor>>>,
    native:ManuallyDrop<Option<NativeProjectionBacking<T::Cursor>>>,
    source: ManuallyDrop<RetainedOwnedProjectionAdmission<T>>,
    closing: bool,
    child_closing: bool,
    release_reply:ManuallyDrop<Option<Result<RetainedCloneStep,ValueError>>>,
}

impl<T:RetainedClone> RetainedFieldCursor<T>{

 fn native_frontier(&self,closing:bool)->Result<Option<RetainedCloneSelection>,ValueError>{
  let Some(native)=self.native.as_ref()else{return Ok(None)};
  if self.child.is_some(){return if native.terminal_is_empty(){Ok(None)}else{Ok(Some(RetainedCloneSelection::new(native.release_demand(None)?,native.release_work_demand())))}}
  if native.initialized(){return Ok(Some(RetainedCloneSelection::new(native.take_box_demand()?,native.take_box_work_demand())))}
  if closing{return if native.terminal_is_empty(){Ok(None)}else{Ok(Some(RetainedCloneSelection::new(native.release_demand(None)?,native.release_work_demand())))}}
  Ok(Some(RetainedCloneSelection::new(native.birth_demand()?,native.birth_work_demand())))
 }
 fn drive_native<'r>(&mut self,closing:bool,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
  let native=self.native.as_mut().ok_or_else(||ValueError::invariant("field native seed absent"))?;
  if self.child.is_some()||closing&&!native.initialized(){return native.release_retirement_into(None,effect,reply.progress())}
  if native.initialized(){return native.take_box_retirement_into(&mut self.child,effect,reply.progress())}
  native.birth_retirement_into(effect,reply.progress())
 }
 fn select_advance(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64)->Result<RetainedCloneSelection,ValueError>{
 #[cfg(test)] crate::retained_clone::selection_observation::record();

  if self.closing{return Err(ValueError::invariant("retained field cursor is closing"))}
  let capture=self.source.demand(source)?;if maximum==0{return Ok(RetainedCloneSelection::new(Default::default(),0))}
  if self.source.terminal_is_empty(){return Ok(RetainedCloneSelection::new(capture,self.source.work_demand(source)?))}
  if let Some(selected)=self.native_frontier(false)?{return Ok(selected)}
  if let Some(child)=self.child.as_ref(){return child.captured_advance_selection(self.source.borrow()?,body,maximum)}
  Ok(RetainedCloneSelection::new(RetirementDemand{copy_bytes:size_of::<T::Cursor>()+size_of::<Option<Box<T::Cursor>>>()+size_of::<Box<T::Cursor>>()+if size_of::<T::Cursor>()==0{0}else{2*size_of::<Option<usize>>()},capacity_bytes:size_of::<T::Cursor>(),depth:1,..Default::default()},1))
 }
 pub(crate) fn advance_selected_into<'r>(&mut self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64,selection:RetainedCloneSelection,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
  if self.closing{return Err(ValueError::invariant("retained field cursor is closing"))}let available=maximum.min(effect.remaining_work_units()?);let demand=selection.demand();if selection.work()==0||selection.work()>available||!retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if self.source.terminal_is_empty(){drop(self.source.admit(source,effect,OriginalEffectReply::borrow(&mut self.release_reply)?)?);return crate::forward_original_value_reply(&mut self.release_reply,reply)}
  if self.native_frontier(false)?.is_some(){return self.drive_native(false,effect,reply)}
  if let Some(child)=self.child.as_mut(){let captured=RetainedCapturedSource::new(&self.source,child.as_ref(),effect,selection)?;let grant=effect.grant();return child.advance_captured_into(captured,body,maximum,&mut effect.reborrow(grant,available)?,reply)}
  crate::record_retirement_work(effect,1).map_err(|(error,_)|error)?;let result=allocate_clone_storage::<T::Cursor>(effect).map(|mut storage|{storage.write(T::retained_clone_cursor());*self.child=Some(unsafe{storage.assume_init()});RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()})}).map_err(Into::into);Ok(reply.complete(result))
 }

    #[cfg(test)]
    pub(crate) fn test_captured_dispatch(&self,other:&Self,effect:&RetainedAllocationEffect<'_>)->Result<(bool,bool,bool),ValueError>{
        let child=self.child.as_ref().ok_or_else(||ValueError::invariant("captured test child absent"))?;
        let other=other.child.as_ref().ok_or_else(||ValueError::invariant("captured test foreign child absent"))?;
        let captured=RetainedCapturedSource::new(&self.source,child.as_ref(),effect,child.captured_advance_selection(self.source.borrow()?,4096,effect.remaining_work_units()?)?)?;
        let accepted=(captured.matches_cursor(child.as_ref(),effect,OriginalEffectCallbackMode::Receive),captured.matches_cursor(other.as_ref(),effect,OriginalEffectCallbackMode::Receive),captured.matches_cursor(child.as_ref(),effect,OriginalEffectCallbackMode::Recover));
        let original=captured.borrow_for(child.as_ref(),effect,OriginalEffectCallbackMode::Receive)?;
        if original.reference_identity()!=self.source.borrow()?.reference_identity(){return Err(ValueError::invariant("captured witness changed its typed source"))}
        Ok(accepted)
    }

    /// 🌱️ Counts actual Source capture, native seed and same-box handoff phases.
    pub fn native_birth_prefix_actions()->Option<usize>{T::NATIVE_CURSOR_SEED.and_then(|seed|seed.part_count().checked_add(5))}
    /// ⛽️ Quotes the six direct fields of an empty lazy field cursor.
    pub fn embedded_birth_work()->Result<u64,ValueError>{Ok(6)}
    /// 🪪️ Authenticates only the already retained immutable Source binding.
    pub fn authenticate_source(&self,source:RetainedCloneRef<'_,T>)->Result<(),ValueError>{self.source.borrow()?;self.source.demand(source)?;Ok(())}
}

impl<T: RetainedClone> Default for RetainedFieldCursor<T> {
    fn default() -> Self { Self { child: ManuallyDrop::new(None),native:ManuallyDrop::new(T::NATIVE_CURSOR_SEED.map(NativeProjectionBacking::new)), source: ManuallyDrop::new(Default::default()), closing: false, child_closing:false,release_reply:ManuallyDrop::new(None) } }
}

impl<T: RetainedClone> RetainedCloneCursor<T> for RetainedFieldCursor<T> {
 fn advance_selection(&self,s:RetainedCloneRef<'_,T>,b:usize,m:u64)->Result<RetainedCloneSelection,ValueError>{self.select_advance(s,b,m)}
 fn captured_advance_selection(&self,s:RetainedCloneRef<'_,T>,b:usize,m:u64)->Result<RetainedCloneSelection,ValueError>{self.select_advance(s,b,m)}
 fn advance_captured_into<'r>(&mut self,s:RetainedCapturedSource<'_,T,Self>,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{let(source,selected)=s.into_selected(self,e,OriginalEffectCallbackMode::Receive)?;self.advance_selected_into(source,b,m,selected,e,r)}
 fn recover_advance_captured_into<'r>(&mut self,s:RetainedCapturedSource<'_,T,Self>,b:usize,m:u64,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{let(source,_)=s.into_selected(self,e,OriginalEffectCallbackMode::Recover)?;self.recover_advance_into(source,b,m,e,r)}

    fn native_terminal_release(&self)->Option<RetainedCursorRelease<Self>>{self.terminal_is_empty().then(||unsafe{RetainedCursorRelease::native(|cursor:&Self|cursor.terminal_is_empty())})}
    fn advance_into<'r>(&mut self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{let selected=self.select_advance(source,body,maximum.min(effect.remaining_work_units()?))?;self.advance_selected_into(source,body,maximum,selected,effect,reply)}
    fn recover_advance_into<'r>(&mut self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.authenticate_source(source)?;let child=self.child.as_mut().ok_or_else(||ValueError::invariant("field recovery lost its original child"))?;let available=maximum.min(effect.remaining_work_units()?);let selected=RetainedCloneSelection::new(Default::default(),0);let captured=RetainedCapturedSource::new(&self.source,child.as_ref(),effect,selected)?;let grant=effect.grant();child.recover_advance_captured_into(captured,body,maximum,&mut effect.reborrow(grant,available)?,reply)}
    fn begin_close_into<'reply>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{let turn=crate::start_retained_close(&mut self.closing,effect);Ok(reply.complete(turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)))}

    fn close_bound_demands(&self,source:RetainedCloneRef<'_,T>,body:usize)->Result<RetirementDemand,ValueError>{if self.native_frontier(true)?.is_some(){return self.close_demands(body)}if !self.closing||!self.source.bound_close_available(){return self.close_demands(body)}self.authenticate_source(source)?;if let Some(child)=self.child.as_ref(){if self.child_closing&&!child.terminal_is_empty(){let mut demand=child.close_bound_demands(source,body)?;return Ok(demand)}}else if !self.child_closing{return self.source.bound_close_demand(source,body)}self.close_demands(body)}
    fn close_bound_work_demand(&self,source:RetainedCloneRef<'_,T>,body:usize)->Result<u64,ValueError>{self.close_bound_demands(source,body)?;if self.native_frontier(true)?.is_some(){return self.close_work_demand(body)}if self.closing&&self.source.bound_close_available(){if let Some(child)=self.child.as_ref(){if self.child_closing&&!child.terminal_is_empty(){return child.close_bound_work_demand(source,body)}}else if !self.child_closing{return self.source.bound_close_work_demand(source,body)}}self.close_work_demand(body)}
    fn close_bound_into<'r>(&mut self,source:RetainedCloneRef<'_,T>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.bound_close_reply(source,body,effect,reply,false)}
    fn recover_close_bound_into<'r>(&mut self,source:RetainedCloneRef<'_,T>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.bound_close_reply(source,body,effect,reply,true)}
    fn close_into<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_reply(None,body,effect,reply,false)}
    fn recover_close_into<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_reply(None,body,effect,reply,true)}
    fn close_original_into<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_reply(Some(original),body,effect,reply,false)}
    fn recover_close_original_into<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_reply(Some(original),body,effect,reply,true)}

    fn output_ready(&self)->bool{!self.closing&&self.child.as_ref().is_some_and(|child|child.output_ready())}
    fn advance_demands(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{Ok(self.select_advance(source,body,maximum)?.demand())}

    fn advance_work_demand(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64)->Result<u64,ValueError>{Ok(self.select_advance(source,body,maximum)?.work())}

    fn take_work_demand(&self)->Result<u64,ValueError>{if !self.output_ready(){return Ok(0)}self.child.as_ref().ok_or_else(||ValueError::invariant("field child absent"))?.take_work_demand()}
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Ok(0)}if let Some(selected)=self.native_frontier(true)?{return Ok(selected.work())}if let Some(child)=self.child.as_ref(){return if !self.child_closing{child_close_start_work(child.as_ref())}else if child.terminal_is_empty(){child.native_terminal_release().map(|release|release.inline_work()).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"field child has no native terminal lifecycle"))}else{child.close_work_demand(body)}}if self.child_closing{return Ok(1)}self.source.close_work_demand(body)}
    fn take_demand(&self)->Result<RetirementDemand,ValueError>{let Some(child)=self.child.as_ref()else{return Ok(Default::default())};let mut d=child.take_demand()?;Ok(d)}
    fn take<'reply>(&mut self,output:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{
     if !self.output_ready(){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
     let demand=self.take_demand()?;if !retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
     let grant=effect.grant();let maximum=effect.remaining_work_units()?;let mut child_effect=effect.reborrow(grant,maximum)?;
     self.child.as_mut().ok_or_else(||ValueError::invariant("field child output absent"))?.take(output,&mut child_effect,reply)
    }
    fn recover_take<'reply>(&mut self,output:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{
     let grant=effect.grant();let maximum=effect.remaining_work_units()?;let mut child_effect=effect.reborrow(grant,maximum)?;
     self.child.as_mut().ok_or_else(||ValueError::invariant("field recovery child absent"))?.recover_take(output,&mut child_effect,reply)
    }

    fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(!self.closing))}
    fn begin_close_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{let flags=usize::from(!self.closing);Ok(crate::RetirementDemand{copy_bytes:flags*std::mem::size_of::<bool>(),depth:usize::from(flags!=0),..Default::default()})}
fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<crate::RetainedCloneProgress,crate::ValueError>{crate::retained_clone::start_retained_close(&mut self.closing,effect)}
    fn close_started(&self)->bool{self.closing}
    

    fn requires_original_birth(&self)->bool{if self.native.as_ref().is_some_and(|native|!native.terminal_is_empty()){return false}if let Some(child)=self.child.as_ref(){return self.closing&&self.child_closing&&!child.terminal_is_empty()&&child.requires_original_birth()}if self.child_closing{return false}self.source.requires_original_birth()}
    fn requires_original_control(&self)->bool{if self.native.as_ref().is_some_and(|native|!native.terminal_is_empty()){return false}if let Some(child)=self.child.as_ref(){return self.closing&&self.child_closing&&!child.terminal_is_empty()&&child.requires_original_control()}if self.child_closing{return false}self.source.requires_original_control()}
    fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if let Some(child)=self.child.as_mut(){return child.close_original_birth_step(original,grant)}self.source.close_original_birth_step(original,grant)}
   
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError> {
        if !self.closing{return Ok(Default::default());}if let Some(selected)=self.native_frontier(true)?{return Ok(selected.demand())}
        if let Some(child)=self.child.as_ref(){if !self.child_closing{return child_close_start_demand(child.as_ref());}return if child.terminal_is_empty(){child.native_terminal_release().map(|release|release.boxed_demand()).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"field child has no native terminal lifecycle"))}else{nested_retained_clone_close_demands::<T>(child.as_ref(),body)};}
        if self.child_closing{return Ok(RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()})}
        self.source.close_demand(body)
    }
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    fn terminal_is_empty(&self) -> bool { self.closing && self.native.as_ref().is_none_or(|native|native.terminal_is_empty()) && self.child.is_none() && !self.child_closing && self.release_reply.is_none() && self.source.terminal_is_empty() }
}

impl<T:RetainedClone> RetainedFieldCursor<T>{

 fn bound_close_reply<'r>(&mut self,source:RetainedCloneRef<'_,T>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
  if self.native_frontier(true)?.is_some(){return self.close_reply(None,body,effect,reply,recover)}if !self.closing||!self.source.bound_close_available(){return self.close_reply(None,body,effect,reply,recover)}self.authenticate_source(source)?;if !recover{let demand=self.close_bound_demands(source,body)?;let work=self.close_bound_work_demand(source,body)?;if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<work{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}}
  if let Some(child)=self.child.as_mut(){if self.child_closing&&(recover||!child.terminal_is_empty()){let grant=effect.grant();let remaining=effect.remaining_work_units()?;let mut child_effect=effect.reborrow(grant,remaining)?;return if recover{child.recover_close_bound_into(source,body,&mut child_effect,reply.progress())}else{child.close_bound_into(source,body,&mut child_effect,reply.progress())}}}else if !self.child_closing{return if recover{self.source.recover_close_into(body,effect,reply)}else{self.source.close_bound_into(source,body,effect,reply)}}self.close_reply(None,body,effect,reply,recover)
 }
 fn close_reply<'reply>(&mut self,original:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
  if effect.invocation().is_none(){return Err(ValueError::invariant("field close requires its original producer invocation"))}
  if !recover{
   if !self.closing{return Ok(reply.complete(Err(ValueError::invariant("typed field must begin close").into())))}
   let demand=self.close_demands(body)?;let work=self.close_work_demand(body)?;if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<work{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  }
  if self.native_frontier(true)?.is_some(){return self.drive_native(true,effect,reply)}
  if let Some(child)=self.child.as_mut(){
   if !self.child_closing{
    if recover{return child.recover_begin_close_into(effect,reply.progress())}
    if !child.close_started(){return child.begin_close_into(effect,reply.progress())}
    let turn=crate::start_retained_close(&mut self.child_closing,effect);return Ok(reply.complete(turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)))
   }
   if recover||!child.terminal_is_empty(){let grant=effect.grant();let maximum=effect.remaining_work_units()?;let mut effect=effect.reborrow(grant,maximum)?;return match original{Some(original)=>if recover{child.recover_close_original_into(original,body,&mut effect,reply.progress())}else{child.close_original_into(original,body,&mut effect,reply.progress())},None=>if recover{child.recover_close_into(body,&mut effect,reply.progress())}else{child.close_into(body,&mut effect,reply.progress())}}}
   let release=child.native_terminal_release().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"field child has no native terminal lifecycle"))?;drop(release.boxed_into(&mut self.child,effect,OriginalEffectReply::borrow(&mut self.release_reply)?)?);return crate::forward_original_value_reply(&mut self.release_reply,reply.progress())
  }
  if self.child_closing{crate::record_retirement_work(effect,1).map_err(|(error,_)|error)?;self.child_closing=false;return Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()}))))}
  match original{Some(original)=>if recover{self.source.recover_close_original_into(original,body,effect,reply)}else{self.source.close_original_into(original,body,effect,reply)},None=>if recover{self.source.recover_close_into(body,effect,reply)}else{self.source.close_into(body,effect,reply)}}
 }


}

impl<T:RetainedClone> Drop for RetainedFieldCursor<T>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(empty||std::thread::panicking(),"typed field cursor requires paid original closure");if empty{unsafe{ManuallyDrop::drop(&mut self.child);ManuallyDrop::drop(&mut self.native);ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.release_reply)}}}}

};}
