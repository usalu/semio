//! ⚙️ Actual 🎁️option clone engine, instantiated in the trait owner.
#[macro_export]
macro_rules! option_cursor_engine{($d:tt)=>{

use super::*;

pub struct OptionCursor<T: RetainedClone> {
    pub(super) child: ManuallyDrop<T::Cursor>,
    pub(super) child_value: ManuallyDrop<Option<T>>,
    route:Option<(semio_framework_value_control_contract::OriginalEffectInvocationIdentity,u8)>,
    selected:Option<(RetirementDemand,u64)>,
    child_reply:ManuallyDrop<Option<Result<RetainedCloneStep,ValueError>>>,
    pub(super) output: ManuallyDrop<Option<Option<T>>>,
    pub(super) source: ManuallyDrop<Option<RetainedCloneBinding>>,
    pub(super) spent: bool,
    pub(super) closing: bool,
    pub(super) close: ManuallyDrop<RetainedCloneClose>,
}

impl<T: RetainedClone> Default for OptionCursor<T> {
    fn default() -> Self {
        Self { child: ManuallyDrop::new(T::retained_clone_cursor()), child_value: ManuallyDrop::new(None), route:None, selected:None, child_reply:ManuallyDrop::new(None), output: ManuallyDrop::new(None), source: ManuallyDrop::new(None), spent: false, closing: false, close: ManuallyDrop::new(RetainedCloneClose::default()) }
    }
}

impl<T: RetainedClone> RetainedCloneCursor<Option<T>> for OptionCursor<T> {
    fn native_terminal_release(&self)->Option<RetainedCursorRelease<Self>>{let terminal=|cursor:&Self|cursor.terminal_is_empty()&&cursor.child.native_terminal_release().is_some_and(|release|release.validates(&*cursor.child));terminal(self).then(||unsafe{RetainedCursorRelease::native(terminal)})}
    fn output_ready(&self)->bool{!self.closing&&!self.spent&&self.output.is_some()}
    fn advance_demands(&self,source:RetainedCloneRef<'_,Option<T>>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.route.is_some(){return self.selected.map(|s|s.0).ok_or_else(||ValueError::invariant("optional selected demand missing"))}let binding=source.binding_demand(&self.source)?;if self.closing||self.spent{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"optional original clone is closing or spent"))}if self.advance_work_demand(source,body,maximum_work_units)?==0{return Ok(Default::default())}if let Some(demand)=binding{return Ok(demand)}if source.get().is_none(){return Ok(Self::publication_demand())}if self.child_value.is_some(){if !self.child.close_started(){return child_close_start_demand(&*self.child)}if self.child.terminal_is_empty(){return Ok(Self::publication_demand())}let mut demand=clone_cursor_close_demand::<T>(&*self.child,body)?;demand.depth=demand.depth.max(1);return Ok(demand)}if self.child.output_ready(){return self.child.take_demand()}self.child.advance_demands(source.try_project(1,|value|value.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained optional child is absent")))?,body,maximum_work_units)}

    fn advance_work_demand(&self,source:RetainedCloneRef<'_,Option<T>>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{
     if self.route.is_some(){return self.selected.map(|s|s.1).ok_or_else(||ValueError::invariant("optional selected work missing"))}source.binding_demand(&self.source)?;if self.closing||self.spent{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"optional original clone is closing or spent"))}if self.output.is_some()||maximum_work_units==0{return Ok(0)}if self.source.is_none(){return source.binding_work_demand(&self.source)}
     let work=if source.get().is_none(){1}else if self.child_value.is_some(){if !self.child.close_started(){child_close_start_work(&*self.child)?}else if self.child.terminal_is_empty(){1}else{self.child.close_work_demand(body)?}}
     else if self.child.output_ready(){self.child.take_work_demand()?}else{return self.child.advance_work_demand(source.try_project(1,|value|value.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained optional child is absent")))?,body,maximum_work_units)};Ok(if work<=maximum_work_units{work}else{0})
    }



 fn advance_into<'r>(&mut self,source:RetainedCloneRef<'_,Option<T>>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.advance_reply(source,body,maximum,effect,reply,false)}
 fn recover_advance_into<'r>(&mut self,source:RetainedCloneRef<'_,Option<T>>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.advance_reply(source,body,maximum,effect,reply,true)}
 fn begin_close_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if self.route.is_some(){return Err(ValueError::invariant("optional original action remains pending"))}let turn=crate::retained_clone::start_retained_close(&mut self.closing,effect);Ok(reply.complete(turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)))}
 fn recover_begin_close_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.begin_close_into(effect,reply)}
 fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_reply(None,body,effect,reply,false)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_reply(None,body,effect,reply,true)}
 fn close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_reply(Some(original),body,effect,reply,false)}
 fn recover_close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_reply(Some(original),body,effect,reply,true)}


 fn requires_original_birth(&self)->bool{if self.route.is_some(){return false}if !self.child.terminal_is_empty(){return self.child.requires_original_birth()}if !self.close.is_empty(){return self.close.requires_original_birth()}self.child_value.is_none()&&self.output.is_none()&&RetainedCloneBinding::requires_original_birth(&self.source)}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if self.route.is_some(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("optional original action must settle before birth").into()),work:0})}if !self.child.terminal_is_empty(){return self.child.close_original_birth_step(original,grant)}if !self.close.is_empty(){return self.close.step_original_birth(original,grant)}RetainedCloneBinding::close_original_birth_step(&mut self.source,original,grant)}
 fn requires_original_control(&self)->bool{if let Some((_,code))=self.route{return match code{11=>self.child.requires_original_control(),12=>self.close.requires_original_control(),13=>RetainedCloneBinding::requires_original_control(&self.source),_=>false}}if !self.child.terminal_is_empty(){return self.child.requires_original_control()}if !self.close.is_empty(){return self.close.requires_original_control()}self.child_value.is_none()&&self.output.is_none()&&RetainedCloneBinding::requires_original_control(&self.source)}
    fn take_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(self.output_ready()))}
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if self.route.is_some(){return self.selected.map(|s|s.1).ok_or_else(||ValueError::invariant("optional selected close work missing"))}if !self.closing{return Ok(0)}if !self.child.terminal_is_empty(){return if self.child.close_started(){self.child.close_work_demand(body)}else{child_close_start_work(&*self.child)}}if !self.close.is_empty(){return self.close.next_owner_work::<T>(false,&None,body)}if self.child_value.is_some(){return self.close.next_owner_work::<T>(true,&None,body)}self.close.next_owner_work::<Option<T>>(self.output.is_some(),&self.source,body)}
    fn take_demand(&self)->Result<RetirementDemand,ValueError>{crate::retained_clone::retained_clone_cursor_take_demand::<Option<T>,bool>()}
    fn take<'reply>(&mut self,output:&mut Option<Option<T>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{crate::retained_clone::take_retained_clone_cursor_output!(self.output_ready(),self.output,output,self.spent,true,effect,reply,Option<T>,bool)}

    
    fn close_started(&self)->bool{self.closing}
    fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(!self.closing))}
    fn begin_close_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{let flags=usize::from(!self.closing);Ok(crate::RetirementDemand{copy_bytes:flags*std::mem::size_of::<bool>(),depth:usize::from(flags!=0),..Default::default()})}
fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<crate::RetainedCloneProgress,crate::ValueError>{crate::retained_clone::start_retained_close(&mut self.closing,effect)}

    

    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.route.is_some(){return self.selected.map(|s|s.0).ok_or_else(||ValueError::invariant("optional selected close demand missing"))}if !self.closing{return Ok(Default::default());}
        if !self.child.terminal_is_empty(){if !self.child.close_started(){return child_close_start_demand(&*self.child)}let mut d=self.child.close_demands(body)?;d.depth=d.depth.max(1);return Ok(d);}
        if !self.close.is_empty(){return self.close.next_owner_demands::<T>(false,&None,body);}
        if self.child_value.is_some(){return self.close.next_owner_demands::<T>(true,&None,body);}
        self.close.next_owner_demands::<Option<T>>(self.output.is_some(),&self.source,body)
    }



    fn next_close_depth_demand(&self)->Result<usize,ValueError>{if !self.closing {return Ok(0);} if !self.child.terminal_is_empty(){return self.child.next_close_depth_demand();} self.close.next_owner_depth_with_binding(self.child_value.is_some()||self.output.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        if !self.child.terminal_is_empty() { return self.child.next_close_copy_byte_demand(); }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        if !self.child.terminal_is_empty() { return self.child.next_close_capacity_byte_demand(maximum_release_bytes); }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if self.child_value.is_some() { return self.close.next_owner_capacity_with_binding::<T>(true,maximum_release_bytes,&self.source); }
        self.close.next_owner_capacity_with_binding::<Option<T>>(self.output.is_some(),maximum_release_bytes,&self.source)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        if !self.child.terminal_is_empty() { return self.child.next_close_release_byte_demand(); }
        self.close.next_release_with_binding(&self.source)
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.route.is_none() && self.child_reply.is_none() && self.child.terminal_is_empty() && self.child_value.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none() }
}

impl<T: RetainedClone> RetainedClone for Option<T> {
    type Cursor = OptionCursor<T>;
    fn retained_clone_cursor() -> Self::Cursor {
        OptionCursor::default()
    }
}

impl<T:RetainedClone> OptionCursor<T>{
 fn publication_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<T>>()+size_of::<Option<Option<T>>>(),depth:1,..Default::default()}}
 fn key(&self,effect:&RetainedAllocationEffect<'_>,code:u8)->Result<(semio_framework_value_control_contract::OriginalEffectInvocationIdentity,u8),ValueError>{let key=(effect.invocation().ok_or_else(||ValueError::invariant("optional child requires its original invocation"))?.identity(),code);if self.route.is_some_and(|original|original!=key){return Err(ValueError::invariant("optional child invocation changed"))}Ok(key)}
 fn advance_reply<'r>(&mut self,source:RetainedCloneRef<'_,Option<T>>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
  let code=if let Some((_,code))=self.route{code}else{
   if recover{return Err(ValueError::invariant("optional advance recovery has no original action"))}
   let demand=self.advance_demands(source,body,maximum)?;let work=self.advance_work_demand(source,body,maximum)?;if work==0&&!self.output_ready()||effect.remaining_work_units()?<work||!retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   if self.output_ready(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
   if self.source.is_none(){let turn=source.bind(&mut self.source,effect);return Ok(reply.complete(turn.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())).map_err(Into::into)))}
   if source.get().is_none()||self.child_value.is_some()&&self.child.terminal_is_empty(){crate::record_retirement_work(effect,1).map_err(|(e,_)|e)?;*self.output=Some(self.child_value.take());return Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:Self::publication_demand().copy_bytes,..Default::default()}))))}
   self.selected=Some((demand,work));if self.child_value.is_some(){if self.child.close_started(){3}else{2}}else if self.child.output_ready(){1}else{0}
  };let key=self.key(effect,code)?;
  if code==1{
   self.route=Some(key);
   if self.child_reply.is_none(){let child=OriginalEffectReply::borrow(&mut self.child_reply)?;if recover{self.child.recover_take(&mut self.child_value,effect,child)?;}else{self.child.take(&mut self.child_value,effect,child)?;}}
   let Some(result)=self.child_reply.take()else{return Ok(reply.pending())};let mut scope=reply.progress().scope(&mut self.route,key)?;scope.borrow()?.complete(result.map_err(Into::into));return Ok(scope.forwarded())
  }
  let mut scope=reply.progress().scope(&mut self.route,key)?;let child=scope.borrow()?;
  match code{0=>{let projected=source.try_project(1,|value|value.as_ref().ok_or_else(||ValueError::invariant("optional original child absent")))?;if recover{self.child.recover_advance_into(projected,body,maximum,effect,child)?;}else{self.child.advance_into(projected,body,maximum,effect,child)?;}},2=>{if recover{self.child.recover_begin_close_into(effect,child)?;}else{self.child.begin_close_into(effect,child)?;}},3=>{if recover{self.child.recover_close_into(body,effect,child)?;}else{self.child.close_into(body,effect,child)?;}},_=>return Err(ValueError::invariant("optional advance route invalid"))}Ok(scope.forwarded())
 }
 fn close_reply<'r>(&mut self,original:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
  if !self.closing{return Err(ValueError::invariant("optional close has not started"))}
  let code=if let Some((_,code))=self.route{code}else{
   if recover{return Err(ValueError::invariant("optional close recovery has no original action"))}let demand=self.close_demands(body)?;let work=self.close_work_demand(body)?;if effect.remaining_work_units()?<work||!retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   if !self.child.terminal_is_empty(){self.selected=Some((demand,work));if self.child.close_started(){11}else{10}}
   else if !self.close.is_empty(){self.selected=Some((demand,work));12}
   else if self.child_value.is_some(){let turn=self.close.begin_granted(&mut self.child_value,effect);return Ok(reply.complete(turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}
   else if self.output.is_some(){let turn=self.close.begin_granted(&mut self.output,effect);return Ok(reply.complete(turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}
   else{self.selected=Some((demand,work));13}
  };let key=self.key(effect,code)?;let reply=if code==13{reply}else{reply.progress()};let mut scope=reply.scope(&mut self.route,key)?;let child=scope.borrow()?;
  match(code,original){(10,_)=>{if recover{self.child.recover_begin_close_into(effect,child)?;}else{self.child.begin_close_into(effect,child)?;}},(11,Some(original))=>{if recover{self.child.recover_close_original_into(original,body,effect,child)?;}else{self.child.close_original_into(original,body,effect,child)?;}},(11,None)=>{if recover{self.child.recover_close_into(body,effect,child)?;}else{self.child.close_into(body,effect,child)?;}},(12,Some(original))=>{if recover{self.close.recover_step_original_into(original,body,effect,child)?;}else{self.close.step_original_into(original,body,effect,child)?;}},(12,None)=>{if recover{self.close.recover_step_into(body,effect,child)?;}else{self.close.step_into(body,effect,child)?;}},(13,Some(original))=>{if recover{RetainedCloneBinding::recover_close_original_into(&mut self.source,original,body,effect,child)?;}else{RetainedCloneBinding::close_original_into(&mut self.source,original,body,effect,child)?;}},(13,None)=>{if recover{RetainedCloneBinding::recover_close_into(&mut self.source,body,effect,child)?;}else{RetainedCloneBinding::close_into(&mut self.source,body,effect,child)?;}},_=>return Err(ValueError::invariant("optional close route invalid"))}Ok(scope.forwarded())
 }
}

impl<T:RetainedClone> Drop for OptionCursor<T>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(empty||std::thread::panicking(),"original OptionCursor requires paid closure");if empty{unsafe{ManuallyDrop::drop(&mut self.child);ManuallyDrop::drop(&mut self.child_value);ManuallyDrop::drop(&mut self.output);ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.close);ManuallyDrop::drop(&mut self.child_reply);}}}}

};}
