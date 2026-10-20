//! 📋️ Retained cloning and retirement for the first-party fixed-page value list.

use super::{close_retained_binding, RetainedClone, RetainedCloneBinding, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, admit_retained_clone_progress};
use semio_framework_value_paged_list::PagedList;
use crate::{
    retirement::{RetireOwned, RetirementCursor, RetirementStep},
};
use std::{
    mem::{ManuallyDrop, size_of},
    ops::{Deref, DerefMut},
};

#[doc(hidden)]
pub struct PagedListCursorState<T: RetainedClone, const N: usize> {
    values: PagedList<T, N>,
    child: Option<T::Cursor>,
    child_closing: bool,
    child_value: Option<T>,
    index: usize,
    source: Option<RetainedCloneBinding>,
    phase: u8,
    output: Option<PagedList<T, N>>,
    closing: bool,
    close: RetainedCloneClose,
}

pub struct PagedListCursor<T: RetainedClone, const N: usize> {
    state: ManuallyDrop<PagedListCursorState<T, N>>,
}

impl<T: RetainedClone, const N: usize> Deref for PagedListCursor<T, N> {
    type Target = PagedListCursorState<T, N>;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<T: RetainedClone, const N: usize> DerefMut for PagedListCursor<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl<T: RetainedClone, const N: usize> Default for PagedListCursor<T, N> {
    fn default() -> Self {
        Self { state: ManuallyDrop::new(PagedListCursorState { values: PagedList::default(), child: None, child_closing:false, child_value: None, index: 0, source: None, phase: 0, output: None, closing: false, close: RetainedCloneClose::default() }) }
    }
}

impl<T: RetainedClone, const N: usize> PagedListCursor<T, N> {
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{
        if !self.closing{return Ok(Default::default());}
        if let Some(child)=self.child.as_ref(){return if !self.child_closing{Ok(crate::RetirementDemand{depth:1,..Default::default()})}else if child.terminal_is_empty(){Ok(crate::RetirementDemand {copy_bytes:size_of::<T::Cursor>(),depth:1,..Default::default()})}else{super::nested_retained_clone_close_demands::<T>(child,body)};}
        if self.child_value.is_some(){return self.close.next_owner_demands::<T>(true,&self.source,body);}
        self.close.next_owner_demands::<PagedList<T,N>>(self.output.is_some()||!self.values.terminal_is_empty(),&self.source,body)
    }
    fn owner_state_is_empty(&self) -> bool {
        self.values.terminal_is_empty() && self.child.is_none() && self.child_value.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none()
    }
}

impl<T: RetainedClone, const N: usize> Drop for PagedListCursor<T, N> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.owner_state_is_empty(), "retained paged list cursor abandoned before bounded terminal close");
        if self.owner_state_is_empty() {
            unsafe { ManuallyDrop::drop(&mut self.state) };
        }
    }
}

impl<T: RetainedClone, const N: usize> RetainedCloneCursor<PagedList<T, N>> for PagedListCursor<T, N> {
    fn output_ready(&self)->bool{!self.closing&&self.phase==1&&self.output.is_some()}
    fn close_started(&self)->bool{self.closing}
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{PagedListCursor::close_demands(self,body)}
    fn advance_demands(&self, source: RetainedCloneRef<'_, PagedList<T, N>>, body: usize) -> Result<crate::RetirementDemand, crate::ValueError> {
        if self.closing || self.phase == 2 { return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "paged list clone is closing or spent")); }
        if let Some(demand)=source.binding_demand(&self.source)?{return Ok(demand);}
        if self.output_ready() { return Ok(Default::default()); }
        let length = source.get().len();
        if length > N { return Err(crate::ValueError::literal(crate::ValueRefusalKind::OwnershipLimit, "paged list source exceeds its logical capacity")); }
        if self.values.capacity() < length { return Ok(crate::RetirementDemand { capacity_bytes: self.values.next_capacity_allocation_bytes(length)?.ok_or_else(|| crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "paged list capacity action is absent"))?, depth: self.values.next_reserve_depth_demand().map_err(crate::ValueError::from)?, ..Default::default() }); }
        if let Some(child) = self.child.as_ref() {
            if self.child_value.is_some() {
                return if child.terminal_is_empty() { Ok(crate::RetirementDemand { copy_bytes: size_of::<T>().checked_add(size_of::<T::Cursor>()).ok_or_else(|| crate::ValueError::literal(crate::ValueRefusalKind::WorkLimit, "paged list adoption extent overflow"))?, depth: 1, ..Default::default() }) } else { super::nested_retained_clone_close_demands::<T>(child, body) };
            }
            if child.output_ready() { return Ok(crate::RetirementDemand { copy_bytes: size_of::<T>(), depth: 1, ..Default::default() }); }
            let mut demand=super::nested_retained_clone_demands::<T>(child, source.project(self.index + 1, |values| values.get(self.index).expect("validated paged list source ordinal")), body)?;demand.depth=demand.depth.max(source.get().next_get_depth_demand(self.index).map_err(crate::ValueError::from)?);return Ok(demand);
        }
        Ok(crate::RetirementDemand { copy_bytes: if self.index == length { size_of::<PagedList<T, N>>() } else { size_of::<T::Cursor>() }, depth: 1, ..Default::default() })
    }



    fn advance(&mut self, source: RetainedCloneRef<'_, PagedList<T, N>>, body: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        let demand = self.advance_demands(source, body)?;
        if self.output_ready() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if !super::retained_clone_grant_admits(grant, demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress) = source.bind(&mut self.source, grant)? { return Ok(RetainedCloneStep::Progress(progress)); }
        let length = source.get().len();
        if self.values.capacity() < length {
            let progress = self.values.reserve_capacity_one(length, grant.maximum_capacity_bytes).map_err(|error| crate::ValueError::from(error.refusal()))?;
            return Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant, RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: progress.allocated_bytes, ..Default::default() }, "paged list capacity")?));
        }
        let index = self.index;
        let state = &mut *self.state;
        if let Some(child) = state.child.as_mut() {
            if state.child_value.is_some() {
                if !child.terminal_is_empty() {
                    let nested = super::nested_retained_clone_grant(grant)?;
                    let step = child.close_step(nested)?;
                    return Ok(RetainedCloneStep::Progress(super::admit_retained_clone_close(nested, step, child.terminal_is_empty(), "paged list child scaffold")?.progress()));
                }
                let progress = state.values.adopt_reserved(&mut state.child_value)?;
                if !progress.progressed { return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "paged list lost its reserved adoption slot")); }
                state.child = None;state.child_closing=false;
                state.index += 1;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
            }
            if child.output_ready() {
                state.child_value = Some(child.take().ok_or_else(|| crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "paged list ready child omitted owner"))?);
                child.begin_close();state.child_closing=true;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
            }
            let nested = super::nested_retained_clone_grant(grant)?;
            let step = child.advance(source.project(index + 1, |values| values.get(index).expect("validated paged list source ordinal")),body, nested)?;
            return Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(nested, step.progress(), "paged list child")?));
        }
        if index == length {
            state.output = Some(std::mem::take(&mut state.values));
            state.phase = 1;
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        state.child = Some(T::retained_clone_cursor());
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }
    fn take(&mut self) -> Option<PagedList<T, N>> {
        if self.phase != 1 {
            return None;
        }
        self.phase = 2;
        self.output.take()
    }

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "paged list must begin close before granted retirement")); }
        if !super::retained_clone_grant_admits(grant,super::retained_clone_close_demands::<PagedList<T,N>>(self,grant.maximum_copy_bytes)?) {return Ok(RetainedCloneStep::Progress(Default::default()));}
        let state = &mut *self.state;
        if let Some(child) = state.child.as_mut() {
            if !state.child_closing {child.begin_close();state.child_closing=true;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));}
            if !child.terminal_is_empty() { let step = child.close_step(super::nested_retained_clone_grant(grant)?)?; return Ok(RetainedCloneStep::Progress(super::admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained child close")?.progress())); }
            state.child = None;state.child_closing=false;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes:size_of::<T::Cursor>(), ..Default::default() }));
        }
        if !state.close.is_empty() { return state.close.step_granted(grant); }
        if let Some(step) = state.close.begin_granted(&mut state.child_value, grant)? { return Ok(step); }
        if let Some(step) = state.close.begin_granted(&mut state.output, grant)? { return Ok(step); }
        if !state.values.terminal_is_empty() {
            if let Some(step) = state.close.begin_default_granted(&mut state.values, grant)? { return Ok(step); }
        }
        close_retained_binding(&mut state.source, grant)
    }

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{Ok(self.close_demands(0)?.depth)}
    fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    fn terminal_is_empty(&self) -> bool { self.closing && self.owner_state_is_empty() }
}

impl<T: RetainedClone, const N: usize> RetainedClone for PagedList<T, N> {
    type Cursor = PagedListCursor<T, N>;

    fn retained_clone_cursor() -> Self::Cursor {
        PagedListCursor::default()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
