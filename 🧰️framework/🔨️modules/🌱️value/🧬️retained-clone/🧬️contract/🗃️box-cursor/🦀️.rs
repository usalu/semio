//! 🗃️ Box cursor of the retained clone contract: the box cursor with its default, cursor and retained clone implementations. See [`super`].

use super::*;

pub struct BoxCursor<T: RetainedClone> {
    pub(super) child: Option<Box<T::Cursor>>,
    pub(super) value: Option<T>,
    pub(super) output: Option<Box<T>>,
    pub(super) source: Option<RetainedCloneBinding>,
    pub(super) spent: bool,
    pub(super) closing: bool,
    child_closing: bool,
    pub(super) close: RetainedCloneClose,
}

impl<T: RetainedClone> Default for BoxCursor<T> {
    fn default() -> Self {
        Self { child: None, value: None, output: None, source: None, spent: false, closing: false, child_closing:false, close: RetainedCloneClose::default() }
    }
}

impl<T: RetainedClone> RetainedCloneCursor<Box<T>> for BoxCursor<T> {
    fn output_ready(&self)->bool{!self.closing&&!self.spent&&self.output.is_some()}
    fn advance_demands(&self,source:RetainedCloneRef<'_,Box<T>>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{let binding=source.binding_demand(&self.source)?;if self.closing||self.spent{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"boxed original clone is closing or spent"))}if maximum_work_units==0{return Ok(Default::default())}if let Some(demand)=binding{return Ok(demand)}if self.output.is_some(){return Ok(Default::default())}if self.value.is_some(){if let Some(child)=self.child.as_ref(){if !child.terminal_is_empty(){return clone_cursor_close_demand::<T>(&**child,body)}return Ok(RetirementDemand{release_bytes:size_of::<T::Cursor>(),depth:1,..Default::default()})}return Ok(RetirementDemand{capacity_bytes:size_of::<T>(),depth:1,..Default::default()})}if let Some(child)=self.child.as_ref(){let mut demand=if child.output_ready(){child.take_demand()?}else{child.advance_demands(source.project(1,|boxed|boxed.as_ref()),body,maximum_work_units)?};demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"boxed original clone quote depth overflow"))?;Ok(demand)}else{Ok(RetirementDemand{capacity_bytes:size_of::<T::Cursor>(),depth:1,..Default::default()})}}

    fn advance_work_demand(&self,source:RetainedCloneRef<'_,Box<T>>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.advance_demands(source,body,maximum_work_units)?;if maximum_work_units==0{return Ok(0)}if self.source.is_none(){return Ok(1)}if self.output.is_some(){return Ok(0)}if self.value.is_some(){if let Some(child)=self.child.as_ref(){if !self.child_closing{return child_close_start_work(child.as_ref())}if !child.terminal_is_empty(){return child.close_work_demand(body)}}return Ok(1)}if let Some(child)=self.child.as_ref(){if child.output_ready(){child.take_work_demand()?.checked_add(1).ok_or_else(||ValueError::invariant("boxed child transfer work overflow"))}else{child.advance_work_demand(source.project(1,|boxed|boxed.as_ref()),body,maximum_work_units)}}else{Ok(1)}}

    fn advance(&mut self, source: RetainedCloneRef<'_, Box<T>>, body: usize, maximum_work_units:u64, effect:&mut crate::RetainedAllocationEffect<'_>) ->crate::OriginalRetirementOutcome{let grant=effect.grant();let maximum_work_units=maximum_work_units.min(effect.maximum_work_units());let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
        if self.closing {
            return Err((ValueError::literal(ValueRefusalKind::InvariantViolated, "retained box clone cursor is closing")).into());
        }
        if self.spent {
            return Err((ValueError::literal(ValueRefusalKind::InvariantViolated, "retained box clone cursor is spent")).into());
        }
        let selected=self.advance_work_demand(source,body,maximum_work_units)?;if selected==0||selected>maximum_work_units{return Ok(RetainedCloneStep::Progress(Default::default()))}let demand=self.advance_demands(source,body,maximum_work_units)?;
        if grant.maximum_items!=0&&grant.maximum_depth<demand.depth{return Err((ValueError::literal(ValueRefusalKind::DepthLimit,"retained box clone structural depth limit exceeded")).into());}
        if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.output.is_some() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)={let turn=source.bind(&mut self.source,grant);work+=turn.work;turn.result}?{return Ok(RetainedCloneStep::Progress(progress));}
        if self.value.is_some() {
            if let Some(child) = self.child.as_mut() {
                if !self.child_closing{let turn=start_child_close(child.as_mut(),&mut self.child_closing,grant);work+=turn.work;let progress=turn.result?;return Ok(RetainedCloneStep::Progress(progress))}
                if !child.terminal_is_empty() {
                    if grant.maximum_items == 0 {
                        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                    }
                    let step = {let turn=child.close_step(&mut effect.reborrow(grant,effect.maximum_work_units().saturating_sub(work))?);work+=turn.work;turn.result}?;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained box child scaffold close")?.progress()));
                }
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                let bytes = size_of::<T::Cursor>();
                if bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.child = None;self.child_closing=false;work+=1;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: bytes }));
            }
            let capacity = size_of::<T>();
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity, released_bytes: 0 };
            if !progress.fits(grant) {
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
            }
            work+=1;let mut storage=allocate_clone_storage::<T>(effect)?;let value=self.value.take().ok_or_else(||ValueError::invariant("retained box child owner is missing"))?;storage.write(value);self.output=Some(unsafe{storage.assume_init()});
            return Ok(RetainedCloneStep::Complete(progress));
        }
        if self.child.is_none() {
            if grant.maximum_depth == 0 {
                return Err((ValueError::literal(ValueRefusalKind::DepthLimit, "retained box clone structural depth limit exceeded")).into());
            }
            let capacity = size_of::<T::Cursor>();
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity, released_bytes: 0 };
            if !progress.fits(grant) {
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
            }
            work+=1;let mut storage=allocate_clone_storage::<T::Cursor>(effect)?;storage.write(T::retained_clone_cursor());self.child=Some(unsafe{storage.assume_init()});
            return Ok(RetainedCloneStep::Progress(progress));
        }
        let child = self.child.as_mut().expect("initialized retained box child");
        if grant.maximum_depth==0{return Err((ValueError::literal(ValueRefusalKind::DepthLimit,"retained box clone structural depth limit exceeded")).into());}
        let child_grant = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        if child.output_ready(){let turn=child.take(child_grant);work+=turn.work;let Some((value,p))=turn.result?else{return Ok(RetainedCloneStep::Progress(Default::default()))};self.value=Some(value);work+=1;return Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(child_grant,p,"boxed child output handoff")?))}
        let turn=child.advance(source.project(1,|boxed|boxed.as_ref()),body,maximum_work_units,&mut effect.reborrow(child_grant,maximum_work_units)?);work+=turn.work;Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(child_grant,turn.result?.progress(),"retained box child")?))
    })();RetainedWorkOutcome{result,work}}

    fn take_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(self.output_ready()))}
    fn close_started(&self)->bool{self.closing}
    fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(!self.closing))}
    fn begin_close_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{let flags=usize::from(!self.closing);Ok(crate::RetirementDemand{copy_bytes:flags*std::mem::size_of::<bool>(),depth:usize::from(flags!=0),..Default::default()})}
fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<crate::RetainedCloneProgress,crate::ValueError>{crate::retained_clone::start_retained_close(&mut self.closing,effect)}
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Ok(0)}if let Some(child)=self.child.as_ref(){if !self.child_closing{return child_close_start_work(child.as_ref())}if !child.terminal_is_empty(){return child.close_work_demand(body)}return Ok(1)}if !self.close.is_empty(){return self.close.next_owner_work::<T>(false,&None,body)}if self.value.is_some(){return self.close.next_owner_work::<T>(true,&None,body)}self.close.next_owner_work::<Box<T>>(self.output.is_some(),&self.source,body)}
    fn take_demand(&self)->Result<RetirementDemand,ValueError>{crate::retained_clone::retained_clone_cursor_take_demand::<Box<T>,bool>()}
    fn take<'reply>(&mut self,output:&mut Option<Box<T>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{crate::retained_clone::take_retained_clone_cursor_output!(self.output_ready(),self.output,output,self.spent,true,effect,reply,Box<T>,bool)}

    

    

    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.closing{return Ok(Default::default());}
        if let Some(child)=self.child.as_ref(){return if !self.child_closing{child_close_start_demand(child.as_ref())}else if child.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<T::Cursor>(),depth:1,..Default::default()})}else{child.close_demands(body)};}
        if !self.close.is_empty(){return self.close.next_owner_demands::<T>(false,&None,body);}
        if self.value.is_some(){return self.close.next_owner_demands::<T>(true,&None,body);}
        self.close.next_owner_demands::<Box<T>>(self.output.is_some(),&self.source,body)
    }

    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let grant=effect.grant();let mut work=0;let result=(||{
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "native owner must begin close before granted retirement").into()); }
        if !retained_clone_grant_admits(grant,self.close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(child) = self.child.as_mut() {
            if !self.child_closing {let turn=start_child_close(child.as_mut(),&mut self.child_closing,grant);work+=turn.work;let progress=turn.result?; return Ok(RetainedCloneStep::Progress(progress)); }
            if !child.terminal_is_empty() { let turn=child.close_step(&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect"));work+=turn.work;let step=turn.result?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained child close")?.progress())); }
            if grant.maximum_depth==0 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"boxed clone scaffold release requires depth").into()); }
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: size_of::<T::Cursor>() };
            if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.child = None;self.child_closing=false;work+=1;
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if !self.close.is_empty() { let turn=self.close.step_granted(effect);work+=turn.work;return turn.result; }
        {let turn=self.close.begin_granted(&mut self.value,effect);work+=turn.work;if let Some(step)=turn.result?{return Ok(step)}}
        {let turn=self.close.begin_granted(&mut self.output,effect);work+=turn.work;if let Some(step)=turn.result?{return Ok(step)}}
        {let turn=close_retained_binding(&mut self.source,effect);work+=turn.work;turn.result}
    })();RetainedWorkOutcome{result,work}}

    fn next_close_depth_demand(&self)->Result<usize,ValueError>{if !self.closing {return Ok(0);} if let Some(child)=self.child.as_ref(){return if child.terminal_is_empty(){Ok(1)}else{child.next_close_depth_demand()};} self.close.next_owner_depth_with_binding(self.value.is_some()||self.output.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child) = self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_copy_byte_demand() }; }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child)=self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_capacity_byte_demand(maximum_release_bytes) }; }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if self.value.is_some() { return self.close.next_owner_capacity_with_binding::<T>(true,maximum_release_bytes,&self.source); }
        self.close.next_owner_capacity_with_binding::<Box<T>>(self.output.is_some(),maximum_release_bytes,&self.source)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child)=self.child.as_ref() { return if child.terminal_is_empty() { Ok(size_of::<T::Cursor>()) } else { child.next_close_release_byte_demand() }; }
        self.close.next_release_with_binding(&self.source)
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.child.is_none() && self.value.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none() }
}

impl<T: RetainedClone> RetainedClone for Box<T> {
    type Cursor = BoxCursor<T>;
    fn retained_clone_cursor() -> Self::Cursor {
        BoxCursor::default()
    }
}
