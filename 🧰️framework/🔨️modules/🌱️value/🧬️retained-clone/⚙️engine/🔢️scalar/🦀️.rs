//! 🔢️ Scalar native cursor and source custody engine.
#[macro_export]
macro_rules! scalar_cursor_engine{($d:tt)=>{

use super::*;

pub struct ScalarCursor<T> {
    pub(super) value: Option<T>,
    pub(super) source: Option<RetainedCloneBinding>,
    pub(super) spent: bool,
    pub(super) closing: bool,
    pub(super) close: RetainedCloneClose,
    close_route:Option<(crate::OriginalEffectInvocationIdentity,u8,usize,Option<(crate::NativeAllocationIdentity,Option<u64>,crate::OriginalWorkBindingIdentity)>)>,
}

impl<T> ScalarCursor<T>{
 const fn native_seed_template()->NativeSeedTemplate<Self>{unsafe{NativeSeedTemplate::fields(1,Self::native_seed_size,Self::native_seed_write)}}
 unsafe fn native_seed_size(_: *mut std::mem::MaybeUninit<Self>,index:usize)->Option<usize>{(index==0).then_some(size_of::<Self>())}
 unsafe fn native_seed_write(target:*mut std::mem::MaybeUninit<Self>,index:usize)->bool{if index!=0{return false}unsafe{target.write(std::mem::MaybeUninit::new(Self::default()))}true}
}

impl<T> Default for ScalarCursor<T> {
    fn default() -> Self {
        Self { value: None, source: None, spent: false, closing: false, close: RetainedCloneClose::default(), close_route:None }
    }
}

impl<T:RetainedClone+Copy> ScalarCursor<T>{fn advance_pure(&mut self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let before=effect.performed_work();let result=(||->Result<RetainedCloneStep,crate::OriginalRetirementFailure>{let maximum=maximum.min(effect.remaining_work_units()?);let demand=self.advance_demands(source,body,maximum)?;if maximum==0{return Ok(RetainedCloneStep::Progress(Default::default()))}if !retained_clone_grant_admits(effect.grant(),demand)||self.advance_work_demand(source,body,maximum)?>maximum{return Ok(RetainedCloneStep::Progress(Default::default()))}if self.output_ready(){return Ok(RetainedCloneStep::Complete(Default::default()))}if source.check_binding_paid(&self.source,effect).result?.is_none(){return Ok(RetainedCloneStep::Progress(Default::default()))}if self.source.is_none(){return source.bind(&mut self.source,effect).result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())).map_err(Into::into)}if self.value.is_some(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"scalar original work exceeded authority").into())}self.value=Some(*source.get());Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})();RetainedWorkOutcome{result,work:effect.performed_work()-before}}}
impl<T> RetainedCloneCursor<T> for ScalarCursor<T>
where
    T: RetainedClone + Copy,
{


 fn native_terminal_release(&self)->Option<RetainedCursorRelease<Self>>{Some(unsafe{RetainedCursorRelease::native(|cursor:&Self|cursor.terminal_is_empty())})}
 fn requires_original_birth(&self)->bool{if self.close_route.is_some(){return false}if !self.close.is_empty(){self.close.requires_original_birth()}else if self.value.is_some(){false}else{RetainedCloneBinding::requires_original_birth(&self.source)}}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if !self.close.is_empty(){self.close.step_original_birth(original,grant)}else if self.value.is_some(){crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("original Birth is not the selected cursor frontier").into()),work:0})}else{RetainedCloneBinding::close_original_birth_step(&mut self.source,original,grant)}}
 fn requires_original_control(&self)->bool{if let Some((_,route,_,_))=self.close_route{return route>=3}if !self.close.is_empty(){self.close.requires_original_control()}else if self.value.is_some(){false}else{RetainedCloneBinding::requires_original_control(&self.source)}}
 fn close_into<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if !self.closing{return Err(ValueError::invariant("cursor must begin close"))}if self.requires_original_birth()||self.requires_original_control(){return Err(ValueError::invariant("cursor requires selected original close"))}if !self.close.is_empty(){let invocation=effect.invocation().ok_or_else(||ValueError::invariant("cursor close lacks its original invocation"))?;if invocation.mode()!=crate::OriginalEffectCallbackMode::Receive{return Err(ValueError::invariant("cursor close is not its original receive invocation"))}if self.close_route.is_some(){return Err(ValueError::invariant("cursor has an unresolved original close invocation"))}let key=(invocation.identity(),1,body,None);let state=self;let mut scope=reply.progress().scope(&mut state.close_route,key)?;let done=state.close.step_into(body,effect,scope.borrow()?)?;drop(done);return Ok(scope.forwarded())}if self.value.is_some(){let state=self;let turn=state.close.begin_granted(&mut state.value,effect);return Ok(reply.complete(turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))} let invocation=effect.invocation().ok_or_else(||ValueError::invariant("cursor binding close lacks its original invocation"))?;if invocation.mode()!=crate::OriginalEffectCallbackMode::Receive{return Err(ValueError::invariant("cursor close is not its original receive invocation"))}if self.close_route.is_some(){return Err(ValueError::invariant("cursor has an unresolved original close invocation"))}let key=(invocation.identity(),2,body,None);let state=self;let mut scope=reply.progress().scope(&mut state.close_route,key)?;let done=RetainedCloneBinding::close_into(&mut state.source,body,effect,scope.borrow()?)?;drop(done);Ok(scope.forwarded())}
 fn recover_close_into<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{let (identity,route,selected_body,selected_original)=self.close_route.ok_or_else(||ValueError::invariant("cursor has no original close invocation"))?;let invocation=effect.invocation().ok_or_else(||ValueError::invariant("cursor close recovery lacks its original invocation"))?;if invocation.identity()!=identity||invocation.mode()!=crate::OriginalEffectCallbackMode::Recover||selected_body!=body||selected_original.is_some(){return Err(ValueError::invariant("cursor close recovery changed invocation"))}let state=self;let mut scope=reply.progress().scope(&mut state.close_route,(identity,route,selected_body,selected_original))?;let child=scope.borrow()?;let done=match route{1=>state.close.recover_step_into(body,effect,child),2=>RetainedCloneBinding::recover_close_into(&mut state.source,body,effect,child),_=>Err(ValueError::invariant("cursor close route changed"))}?;drop(done);Ok(scope.forwarded())}
 fn close_original_into<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if self.requires_original_birth(){return Err(ValueError::invariant("cursor Birth must settle before original close"))}if !self.requires_original_control(){return self.close_into(body,effect,reply)}let invocation=effect.invocation().ok_or_else(||ValueError::invariant("cursor original close lacks invocation"))?;if !self.close.is_empty(){if invocation.mode()!=crate::OriginalEffectCallbackMode::Receive{return Err(ValueError::invariant("cursor close is not its original receive invocation"))}if self.close_route.is_some(){return Err(ValueError::invariant("cursor has an unresolved original close invocation"))}let work=original.original_work().binding_identity()?;if !work.matches_effect(effect){return Err(ValueError::invariant("cursor original close changed Work"))}let key=(invocation.identity(),3,body,Some((original.ledger_identity(),original.retirement_recipient_identity(),work)));let state=self;let mut scope=reply.progress().scope(&mut state.close_route,key)?;let done=state.close.step_original_into(original,body,effect,scope.borrow()?)?;drop(done);Ok(scope.forwarded())}else{if invocation.mode()!=crate::OriginalEffectCallbackMode::Receive{return Err(ValueError::invariant("cursor close is not its original receive invocation"))}if self.close_route.is_some(){return Err(ValueError::invariant("cursor has an unresolved original close invocation"))}let work=original.original_work().binding_identity()?;if !work.matches_effect(effect){return Err(ValueError::invariant("cursor original close changed Work"))}let key=(invocation.identity(),4,body,Some((original.ledger_identity(),original.retirement_recipient_identity(),work)));let state=self;let mut scope=reply.progress().scope(&mut state.close_route,key)?;let done=RetainedCloneBinding::close_original_into(&mut state.source,original,body,effect,scope.borrow()?)?;drop(done);Ok(scope.forwarded())}}
 fn recover_close_original_into<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{let (identity,route,selected_body,selected_original)=self.close_route.ok_or_else(||ValueError::invariant("cursor has no original close invocation"))?;if !effect.invocation().is_some_and(|i|i.identity()==identity&&i.mode()==crate::OriginalEffectCallbackMode::Recover)||selected_body!=body{return Err(ValueError::invariant("cursor original close recovery changed invocation or body"))}if route==1||route==2{return self.recover_close_into(body,effect,reply)}let work=original.original_work().binding_identity()?;if !work.matches_effect(effect)||selected_original!=Some((original.ledger_identity(),original.retirement_recipient_identity(),work)){return Err(ValueError::invariant("cursor original close recovery changed loan"))}let state=self;let mut scope=reply.progress().scope(&mut state.close_route,(identity,route,selected_body,selected_original))?;let child=scope.borrow()?;let done=match route{3=>state.close.recover_step_original_into(original,body,effect,child),4=>RetainedCloneBinding::recover_close_original_into(&mut state.source,original,body,effect,child),_=>Err(ValueError::invariant("cursor close route changed"))}?;drop(done);Ok(scope.forwarded())}
 fn advance_into<'reply>(&mut self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{Ok(reply.complete(self.advance_pure(source,body,maximum,effect).result))}
 fn recover_advance_into<'reply>(&mut self,_source:RetainedCloneRef<'_,T>,_body:usize,_maximum:u64,_effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{Ok(reply.pending())}
 fn begin_close_into<'reply>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{let turn=self.begin_close_paid(effect);Ok(reply.complete(turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)))}
 fn recover_begin_close_into<'reply>(&mut self,_effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{Ok(reply.pending())}
    fn output_ready(&self)->bool{!self.closing&&!self.spent&&self.value.is_some()}
    fn advance_demands(&self,source:RetainedCloneRef<'_,T>,_body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.spent{return Err(ValueError::invariant("scalar clone is closing or spent"))}if maximum==0{return Ok(Default::default())}if let Some(d)=source.binding_demand(&self.source)?{return Ok(d)}Ok(if self.value.is_none(){RetirementDemand{copy_bytes:size_of::<Option<T>>(),depth:1,..Default::default()}}else{Default::default()})}
    fn advance_work_demand(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64)->Result<u64,ValueError>{self.advance_demands(source,body,maximum)?;if maximum==0||self.output_ready(){return Ok(0)}let check=RetainedCloneRef::<T>::check_binding_work(&self.source);let action=if self.source.is_none(){source.binding_work_demand(&self.source)?}else{u64::from(self.value.is_none())};check.checked_add(action).ok_or_else(||ValueError::invariant("scalar source work quote overflow"))}
    
    fn take_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(self.output_ready()))}
    fn close_bound_demands(&self,source:RetainedCloneRef<'_,T>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing&&self.close.is_empty()&&self.value.is_none()&&RetainedCloneBinding::bound_close_available(&self.source){RetainedCloneBinding::bound_close_demand(&self.source,source,body)}else{self.close_demands(body)}}
    fn close_bound_work_demand(&self,source:RetainedCloneRef<'_,T>,body:usize)->Result<u64,ValueError>{if self.closing&&self.close.is_empty()&&self.value.is_none()&&RetainedCloneBinding::bound_close_available(&self.source){RetainedCloneBinding::bound_close_work_demand(&self.source,source,body)}else{self.close_work_demand(body)}}
    fn close_bound_into<'r>(&mut self,source:RetainedCloneRef<'_,T>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if self.closing&&self.close.is_empty()&&self.value.is_none()&&self.close_route.is_none()&&RetainedCloneBinding::bound_close_available(&self.source){if let Err(error)=RetainedCloneBinding::bound_close_demand(&self.source,source,body){return Ok(reply.complete(Err(error.into())))}RetainedCloneBinding::close_bound_into(&mut self.source,source,body,effect,reply)}else{self.close_into(body,effect,reply)}}
    fn recover_close_bound_into<'r>(&mut self,_:RetainedCloneRef<'_,T>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{if self.close_route.is_some(){self.recover_close_into(body,effect,reply)}else{Err(ValueError::invariant("native bound close has no unrecorded action"))}}
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if self.close_route.is_some_and(|(_,_,selected,_)|selected!=body){return Err(ValueError::invariant("cursor close query changed original body"))}if !self.closing{return Ok(0)}self.close.next_owner_work::<T>(self.value.is_some(),&self.source,body)}
    fn take_demand(&self)->Result<RetirementDemand,ValueError>{crate::retained_clone::retained_clone_cursor_take_demand::<T,bool>()}
    fn take<'reply>(&mut self,output:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{crate::retained_clone::take_retained_clone_cursor_output!(self.output_ready(),self.value,output,self.spent,true,effect,reply,T,bool)}

    
    fn close_started(&self)->bool{self.closing}
    fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(!self.closing))}
    fn begin_close_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{let flags=usize::from(!self.closing);Ok(crate::RetirementDemand{copy_bytes:flags*std::mem::size_of::<bool>(),depth:usize::from(flags!=0),..Default::default()})}
fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<crate::RetainedCloneProgress,crate::ValueError>{crate::retained_clone::start_retained_close(&mut self.closing,effect)}

    

    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.close_route.is_some_and(|(_,_,selected,_)|selected!=body){return Err(ValueError::invariant("cursor close query changed original body"))}if !self.closing{return Ok(Default::default());}self.close.next_owner_demands::<T>(self.value.is_some(),&self.source,body)}

    fn next_close_depth_demand(&self)->Result<usize,ValueError>{if !self.closing {return Ok(0);} self.close.next_owner_depth_with_binding(self.value.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        Ok(self.close_demands(0)?.copy_bytes)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_owner_capacity_with_binding::<T>(self.value.is_some(), maximum_release_bytes,&self.source)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_release_with_binding(&self.source)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.value.is_none() && self.close.is_empty() && self.source.is_none() && self.close_route.is_none()
    }
}

macro_rules! retained_clone_scalar {
    ($d ($d type:ty),+ $d (,)?) => {$d  (
        impl RetainedClone for $d type {
            type Cursor = ScalarCursor<Self>;
            const NATIVE_CURSOR_SEED:Option<NativeSeedTemplate<Self::Cursor>>=Some(ScalarCursor::<Self>::native_seed_template());
            fn retained_clone_cursor() -> Self::Cursor { ScalarCursor::default() }
            fn native_copy()->Option<RetainedCopy<Self>>{Some(RetainedCopy::new())}
        }
    )+ };
}

retained_clone_scalar!((), bool, char, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

};}
