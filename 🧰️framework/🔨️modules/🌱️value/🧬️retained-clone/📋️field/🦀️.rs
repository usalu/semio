//! 📋️ Lazily admits one typed child cursor without eager recursive owner construction.

use super::*;

pub struct RetainedFieldCursor<T: RetainedClone> {
    child: Option<Box<T::Cursor>>,
    source: Option<RetainedCloneBinding>,
    closing: bool,
    child_closing: bool,
}

impl<T:RetainedClone> RetainedFieldCursor<T>{
    /// ⛽️ Quotes the four direct fields of an empty lazy field cursor.
    pub fn embedded_birth_work()->Result<u64,ValueError>{Ok(4)}
}

impl<T: RetainedClone> Default for RetainedFieldCursor<T> {
    fn default() -> Self { Self { child: None, source: None, closing: false, child_closing:false } }
}

impl<T: RetainedClone> RetainedCloneCursor<T> for RetainedFieldCursor<T> {
    fn output_ready(&self)->bool{!self.closing&&self.child.as_ref().is_some_and(|child|child.output_ready())}
    fn advance_demands(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"typed field original clone is closing"))}let binding=source.binding_demand(&self.source)?;if maximum_work_units==0{return Ok(Default::default())}if let Some(demand)=binding{return Ok(demand)}if let Some(child)=self.child.as_ref(){let mut demand=child.advance_demands(source,body,maximum_work_units)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"typed field original quote depth overflow"))?;Ok(demand)}else{Ok(RetirementDemand{capacity_bytes:size_of::<T::Cursor>(),depth:1,..Default::default()})}}

    fn advance_work_demand(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.advance_demands(source,body,maximum_work_units)?;if maximum_work_units==0{return Ok(0)}if self.source.is_none(){let w=source.binding_work_demand(&self.source)?;return Ok(if w<=maximum_work_units{w}else{0})}match self.child.as_ref(){Some(child)=>child.advance_work_demand(source,body,maximum_work_units),None=>Ok(1)}}
    fn advance(&mut self, source: RetainedCloneRef<'_, T>, body: usize, maximum_work_units:u64, effect:&mut crate::RetainedAllocationEffect<'_>) ->crate::OriginalRetirementOutcome{let grant=effect.grant();let maximum_work_units=maximum_work_units.min(effect.maximum_work_units());let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
        if self.closing { return Err((ValueError::literal(ValueRefusalKind::InvariantViolated, "retained field cursor is closing")).into()); }
        let selected=self.advance_work_demand(source,body,maximum_work_units)?;if selected==0||selected>maximum_work_units{return Ok(RetainedCloneStep::Progress(Default::default()))}
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_depth == 0 { return Err((ValueError::literal(ValueRefusalKind::DepthLimit, "retained typed field structural depth limit exceeded")).into()); }
        if let Some(progress)={let turn=source.bind(&mut self.source,effect);work+=turn.work;turn.result}?{return Ok(RetainedCloneStep::Progress(progress));}
        if let Some(child) = self.child.as_mut() { return {let turn=child.advance(source,body,maximum_work_units,&mut effect.nested_within(RetainedCloneGrant { maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant },maximum_work_units));work+=turn.work;turn.result}; }
        let bytes = size_of::<T::Cursor>();
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: bytes, released_bytes: 0 };
        if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
        work+=1;let mut storage=allocate_clone_storage::<T::Cursor>(effect)?;storage.write(T::retained_clone_cursor());self.child=Some(unsafe{storage.assume_init()});
        Ok(RetainedCloneStep::Progress(progress))
    })();RetainedWorkOutcome{result,work}}
    fn take_work_demand(&self)->Result<u64,ValueError>{if !self.output_ready(){return Ok(0)}self.child.as_ref().ok_or_else(||ValueError::invariant("field child absent"))?.take_work_demand()}
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Ok(0)}if let Some(child)=self.child.as_ref(){return if !self.child_closing{child_close_start_work(child.as_ref())}else if child.terminal_is_empty(){Ok(1)}else{child.close_work_demand(body)}}RetainedCloneBinding::close_work_demand(&self.source,body)}
    fn take_demand(&self)->Result<RetirementDemand,ValueError>{let Some(child)=self.child.as_ref()else{return Ok(Default::default())};let mut d=child.take_demand()?;d.depth=d.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"field output handoff depth overflow"))?;Ok(d)}
    fn take<'reply>(&mut self,output:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{
     let grant=effect.grant();let idle=||Ok::<_,ValueError>(RetainedCloneStep::Progress(Default::default()));
     if !self.output_ready(){return Ok(reply.complete(idle()))}
     let d=self.take_demand()?;
     if !retained_clone_grant_admits(grant,d){return Ok(reply.complete(idle()))}
     let child=self.child.as_mut().ok_or_else(||ValueError::invariant("child absent"))?;
     let mut nested=effect.nested(RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant});
     child.take(output,&mut nested,reply)
    }
    
    fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(!self.closing))}
    fn begin_close_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{let flags=usize::from(!self.closing);Ok(crate::RetirementDemand{copy_bytes:flags*std::mem::size_of::<bool>(),depth:usize::from(flags!=0),..Default::default()})}
fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<crate::RetainedCloneProgress,crate::ValueError>{crate::retained_clone::start_retained_close(&mut self.closing,effect)}
    fn close_started(&self)->bool{self.closing}
    
    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let grant=effect.grant();let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::RetirementPollExit>{
        if !self.closing { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"typed field must begin close before granted retirement").into()); }
        if !retained_clone_grant_admits(grant,retained_clone_close_demands::<T>(self,grant.maximum_copy_bytes)?) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(child)=self.child.as_mut() {
            if !self.child_closing {let turn=start_child_close(child.as_mut(),&mut self.child_closing,effect);work+=turn.work;let progress=turn.result?;return Ok(RetainedCloneStep::Progress(progress)); }
            if !child.terminal_is_empty() { let nested=nested_retained_clone_grant(grant)?;let step={let turn=crate::RetirementPollExit::settled(child.close_step(&mut effect.nested(nested)))?;work+=turn.work;turn.result}?;return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(nested,step,child.terminal_is_empty(),"field original child close")?.progress())); }
            self.child=None;self.child_closing=false;work+=1;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:size_of::<T::Cursor>(),..Default::default()}));
        }
        {let turn=crate::RetirementPollExit::settled(close_retained_binding(&mut self.source,effect))?;work+=turn.work;turn.result.map_err(Into::into)}
    })();crate::RetirementPollExit::finish(result,work)}
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError> {
        if !self.closing{return Ok(Default::default());}
        if let Some(child)=self.child.as_ref(){if !self.child_closing{return child_close_start_demand(child.as_ref());}return if child.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<T::Cursor>(),depth:1,..Default::default()})}else{nested_retained_clone_close_demands::<T>(child.as_ref(),body)};}
        RetainedCloneBinding::close_demand(&self.source,body)
    }
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    fn terminal_is_empty(&self) -> bool { self.closing && self.child.is_none() && self.source.is_none() }
}

impl<T: RetainedClone> Drop for RetainedFieldCursor<T> {
    fn drop(&mut self) { assert!((self.child.is_none() && self.source.is_none()) || std::thread::panicking(), "retained typed field dropped before exact child closure"); }
}
