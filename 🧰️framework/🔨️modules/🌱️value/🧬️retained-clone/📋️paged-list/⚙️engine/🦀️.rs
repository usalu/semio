//! ⚙️ Original fixed-page clone phases depend on a first-party element protocol.
use super::*;
use std::{
    mem::{ManuallyDrop, size_of},
    ops::{Deref, DerefMut},
};

#[doc(hidden)]
pub struct PagedListCursorState<T: RetireOwned+Send+Sync+'static,C:PagedCloneElement<T>, const N: usize> {
    values: PagedList<T, N>,
    child: Option<C>,
    child_closing: bool,
    child_value: Option<T>,
    index: usize,
    source: Option<RetainedCloneBinding>,
    phase: u8,
    output: Option<PagedList<T, N>>,
    closing: bool,
    close: RetainedCloneClose,
}

pub struct PagedListCloneEngine<T: RetireOwned+Send+Sync+'static,C:PagedCloneElement<T>, const N: usize> {
    state: ManuallyDrop<PagedListCursorState<T,C,N>>,
}

impl<T: RetireOwned+Send+Sync+'static,C:PagedCloneElement<T>, const N: usize> Deref for PagedListCloneEngine<T,C,N> {
    type Target = PagedListCursorState<T,C,N>;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<T: RetireOwned+Send+Sync+'static,C:PagedCloneElement<T>, const N: usize> DerefMut for PagedListCloneEngine<T,C,N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl<T: RetireOwned+Send+Sync+'static,C:PagedCloneElement<T>, const N: usize> Default for PagedListCloneEngine<T,C,N> {
    fn default() -> Self {
        Self { state: ManuallyDrop::new(PagedListCursorState { values: PagedList::default(), child: None, child_closing:false, child_value: None, index: 0, source: None, phase: 0, output: None, closing: false, close: RetainedCloneClose::default() }) }
    }
}

impl<T: RetireOwned+Send+Sync+'static,C:PagedCloneElement<T>, const N: usize> PagedListCloneEngine<T,C,N> {
    fn selected_close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{
        if !self.closing{return Ok(Default::default());}
        if let Some(child)=self.child.as_ref(){return if !self.child_closing{child_start_demand(child)}else if child.terminal_is_empty(){Ok(crate::RetirementDemand {copy_bytes:size_of::<C>(),depth:1,..Default::default()})}else{nested_retained_clone_close_demands::<T,C>(child,body)};}
        if self.child_value.is_some(){return self.close.next_owner_demands::<T>(true,&self.source,body);}
        self.close.next_owner_demands::<PagedList<T,N>>(self.output.is_some()||!self.values.terminal_is_empty(),&self.source,body)
    }
    fn owner_state_is_empty(&self) -> bool {
        self.values.terminal_is_empty() && self.child.is_none() && self.child_value.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none()
    }
}

impl<T: RetireOwned+Send+Sync+'static,C:PagedCloneElement<T>, const N: usize> Drop for PagedListCloneEngine<T,C,N> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.owner_state_is_empty(), "retained paged list cursor abandoned before bounded terminal close");
        if self.owner_state_is_empty() {
            unsafe { ManuallyDrop::drop(&mut self.state) };
        }
    }
}

impl<T: RetireOwned+Send+Sync+'static,C:PagedCloneElement<T>, const N: usize> PagedListCloneEngine<T,C,N> {
    pub fn output_ready(&self)->bool{!self.closing&&self.phase==1&&self.output.is_some()}
    pub fn close_started(&self)->bool{self.closing}
    pub fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{self.selected_close_demands(body)}
    pub fn advance_demands(&self, source: RetainedCloneRef<'_, PagedList<T, N>>, body: usize,maximum_work_units:u64) -> Result<crate::RetirementDemand, crate::ValueError> {
        if self.closing || self.phase == 2 { return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "paged list clone is closing or spent")); }
        if let Some(demand)=source.binding_demand(&self.source)?{return Ok(demand);}
        if self.output_ready() { return Ok(Default::default()); }
        let length = source.get().len();
        if length > N { return Err(crate::ValueError::literal(crate::ValueRefusalKind::OwnershipLimit, "paged list source exceeds its logical capacity")); }
        if self.values.capacity() < length { return Ok(crate::RetirementDemand { copy_bytes:self.values.next_capacity_copy_byte_demand(length).map_err(crate::ValueError::from)?, capacity_bytes: self.values.next_capacity_allocation_bytes(length)?.ok_or_else(|| crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "paged list capacity action is absent"))?, depth: self.values.next_capacity_depth_demand(length).map_err(crate::ValueError::from)?, ..Default::default() }); }
        if let Some(child) = self.child.as_ref() {
            if self.child_value.is_some() {
                if !self.child_closing{return child_start_demand(child)}return if child.terminal_is_empty() { self.values.adoption_demand(&self.child_value) } else { nested_retained_clone_close_demands::<T,C>(child, body) };
            }
            if child.output_ready() { return nested_retained_clone_take_demands(child); }
            let mut demand=nested_retained_clone_demands::<T,C>(child, source.project(self.index + 1, |values| values.get(self.index).expect("validated paged list source ordinal")), body,maximum_work_units)?;demand.depth=demand.depth.max(source.get().next_get_depth_demand(self.index).map_err(crate::ValueError::from)?);return Ok(demand);
        }
        Ok(crate::RetirementDemand { copy_bytes: if self.index == length { size_of::<PagedList<T, N>>() } else { size_of::<C>() }, depth: 1, ..Default::default() })
    }



    pub fn advance(&mut self, source: RetainedCloneRef<'_, PagedList<T, N>>, body: usize,maximum_work_units:u64, effect:&mut crate::RetainedAllocationEffect<'_>) ->crate::OriginalRetirementOutcome{let grant=effect.grant();let maximum_work_units=maximum_work_units.min(effect.maximum_work_units());let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
        let selected=self.advance_work_demand(source,body,maximum_work_units)?;if selected==0||selected>maximum_work_units{return Ok(RetainedCloneStep::Progress(Default::default()))}let demand = self.advance_demands(source, body,maximum_work_units)?;
        if self.output_ready() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if !retained_clone_grant_admits(grant, demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress) = {let turn=source.bind(&mut self.source,effect);work+=turn.work;turn.result}? { return Ok(RetainedCloneStep::Progress(progress)); }
        let length = source.get().len();
        if self.values.capacity() < length {
            let turn=self.values.reserve_capacity_one_funded(length,effect);work+=turn.work;return match turn.result{Ok(crate::PagedListReservationOutcome::Denied)=>Ok(RetainedCloneStep::Progress(Default::default())),Ok(crate::PagedListReservationOutcome::Ready(progress))=>Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant,progress,"paged list capacity")?)),Err(crate::PagedListReservationFailure::Work(error))=>Err(error.into()),Err(crate::PagedListReservationFailure::Allocation{error,copied_bytes})=>Err(crate::ValueError::from(error.refusal()).with_retained_progress(RetainedCloneProgress{copied_items:usize::from(error.allocated_bytes!=0||copied_bytes!=0),copied_bytes,retained_capacity_bytes:error.allocated_bytes,..Default::default()}).into())};
        }
        let index = self.index;
        let state = &mut *self.state;
        if let Some(child) = state.child.as_mut() {
            if state.child_value.is_some() {
                if !state.child_closing{let turn=start_child_close(child,&mut state.child_closing,effect);work+=turn.work;return turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)}
                if !child.terminal_is_empty() {
                    let nested = nested_retained_clone_grant(grant)?;
                    let step = {let turn=settle_child_close(child.close_step(&mut effect.nested(nested)));work+=turn.work;turn.result}?;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(nested, step, child.terminal_is_empty(), "paged list child scaffold")?.progress()));
                }
                let turn=state.values.adopt_reserved(&mut state.child_value,effect);work+=turn.work;let progress=match turn.result{Ok(p)=>p,Err(error)=>return Err(error.into())};
                if !progress.progressed { return Err((crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "paged list lost its reserved adoption slot")).into()); }
                state.child = None;state.child_closing=false;
                state.index += 1;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
            }
            if child.output_ready() {
                let nested=nested_retained_clone_grant(grant)?;let mut slot=None;let boundary=effect.performed_boundary();{let reply=crate::OriginalEffectReply::borrow(&mut slot)?;child.take(&mut state.child_value,&mut effect.nested(nested),reply)?;}work+=effect.performed_since(boundary)?;let progress=slot.take().ok_or_else(||crate::ValueError::invariant("paged list child take left no reply"))??.progress();
                
                return Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(nested,progress,"paged list child take")?));
            }
            let nested = nested_retained_clone_grant(grant)?;
            let step = {let turn=child.advance(source.try_project(index + 1, |values| values.get(index).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"paged list source ordinal is absent")))?,body,maximum_work_units,&mut effect.nested_within(nested,maximum_work_units));work+=turn.work;turn.result}?;
            return Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(nested, step.progress(), "paged list child")?));
        }
        if index == length {
            work+=1;state.output = Some(std::mem::take(&mut state.values));
            state.phase = 1;
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        work+=1;state.child = Some(C::new());
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    })();crate::RetainedWorkOutcome{result,work}}
    pub fn take_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{semio_framework_value_control_contract::retained_clone_cursor_take_demand::<PagedList<T,N>,u8>()}
    pub fn take<'reply>(&mut self,output:&mut Option<PagedList<T, N>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<crate::OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{semio_framework_value_control_contract::take_retained_clone_cursor_output!(self.output_ready(),self.state.output,output,self.state.phase,2,effect,reply,PagedList<T,N>,u8)}

    pub fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:usize::from(!self.closing),depth:usize::from(!self.closing),..Default::default()})}
 pub fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(!self.closing))}
 pub fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress>{crate::start_retained_close(&mut self.closing,effect)}

    

    pub fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let grant=effect.grant();let mut work=0;let result=(||->Result<RetainedCloneStep,semio_framework_value_retirement::retirement_contract::RetirementPollExit>{
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "paged list must begin close before granted retirement").into()); }
        if !retained_clone_grant_admits(grant,self.selected_close_demands(grant.maximum_copy_bytes)?) {return Ok(RetainedCloneStep::Progress(Default::default()));}
        let state = &mut *self.state;
        if let Some(child) = state.child.as_mut() {
            if !state.child_closing {let turn=start_child_close(child,&mut state.child_closing,effect);work+=turn.work;let progress=turn.result?;return Ok(RetainedCloneStep::Progress(progress))}
            if !child.terminal_is_empty() { let step = {let turn=semio_framework_value_retirement::retirement_contract::RetirementPollExit::settled(child.close_step(&mut effect.nested(nested_retained_clone_grant(grant)?)))?;work+=turn.work;turn.result}?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained child close")?.progress())); }
            state.child = None;state.child_closing=false;work+=1;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes:size_of::<C>(), ..Default::default() }));
        }
        if !state.close.is_empty() { return {let turn=semio_framework_value_retirement::retirement_contract::RetirementPollExit::settled(state.close.step_granted(effect))?;work+=turn.work;turn.result.map_err(Into::into)}; }
        if let Some(step) = {let turn=state.close.begin_granted(&mut state.child_value,effect);work+=turn.work;turn.result}? { return Ok(step); }
        if let Some(step) = {let turn=state.close.begin_granted(&mut state.output,effect);work+=turn.work;turn.result}? { return Ok(step); }
        if !state.values.terminal_is_empty() {
            if let Some(step) = {let turn=state.close.begin_empty_granted(&mut state.values,effect);work+=turn.work;turn.result}? { return Ok(step); }
        }
        {let turn=semio_framework_value_retirement::retirement_contract::RetirementPollExit::settled(close_retained_binding(&mut state.source,effect))?;work+=turn.work;turn.result.map_err(Into::into)}
    })();semio_framework_value_retirement::retirement_contract::RetirementPollExit::finish(result,work)}

    pub fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{Ok(self.close_demands(0)?.depth)}
    pub fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    pub fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    pub fn terminal_is_empty(&self) -> bool { self.closing && self.owner_state_is_empty() }
}



impl<T:RetireOwned+Send+Sync+'static,C:PagedCloneElement<T>,const N:usize>PagedListCloneEngine<T,C,N>{
 pub fn advance_work_demand(&self,source:RetainedCloneRef<'_,PagedList<T,N>>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{let _=self.advance_demands(source,body,maximum_work_units)?;if maximum_work_units==0||self.output_ready(){return Ok(0)}let selected=if !source.check_binding(&self.source)?{source.binding_work_demand(&self.source)?}else if self.values.capacity()<source.get().len(){1}else if let Some(child)=self.child.as_ref(){if self.child_value.is_some(){if !self.child_closing{if child.close_started(){1}else{child.begin_close_work_demand()?}}else if child.terminal_is_empty(){1}else{child.close_work_demand(body)?}}else if child.output_ready(){child.take_work_demand()?}else{child.advance_work_demand(source.project(self.index+1,|values|values.get(self.index).expect("validated paged list ordinal")),body,maximum_work_units)?}}else{1};Ok(if selected<=maximum_work_units{selected}else{0})}
 pub fn take_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(self.output_ready()))}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Ok(0)}if let Some(child)=self.child.as_ref(){return if !self.child_closing{if child.close_started(){Ok(1)}else{child.begin_close_work_demand()}}else if child.terminal_is_empty(){Ok(1)}else{child.close_work_demand(body)}}if self.child_value.is_some(){return self.close.next_owner_work::<T>(true,&self.source,body)}self.close.next_owner_work::<PagedList<T,N>>(self.output.is_some()||!self.values.terminal_is_empty(),&self.source,body)}
}
#[cfg(test)]
use semio_framework_value_retained_clone::retained_clone::paged_list::PagedListCursor;
#[cfg(test)]
#[path="../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
