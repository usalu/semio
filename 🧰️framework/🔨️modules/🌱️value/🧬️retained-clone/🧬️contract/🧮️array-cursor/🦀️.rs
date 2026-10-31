//! 🧮️ Array cursor of the retained clone contract: the fixed array cursor with its default, cursor and retained clone implementations. See [`super`].

use super::*;

/// 📃️ Clones bounded bitwise pages or typed rows; only slots before `index` own initialized values until the final paid array move resets that prefix.
pub struct ArrayCursor<T: RetainedClone, const N: usize> {
    slots: [std::mem::MaybeUninit<T>; N],
    pub(super) child: ManuallyDrop<Option<T::Cursor>>,
    pub(super) child_value: ManuallyDrop<Option<T>>,
    child_closing: bool,
    pub(super) index: usize,
    pub(super) source: ManuallyDrop<Option<RetainedCloneBinding>>,
    pub(super) output: ManuallyDrop<Option<[T; N]>>,
    pub(super) spent: bool,
    pub(super) closing: bool,
    pub(super) close: RetainedCloneClose,
}

impl<T: RetainedClone, const N: usize> Default for ArrayCursor<T, N> {
    fn default() -> Self {
        Self { slots: unsafe { std::mem::MaybeUninit::uninit().assume_init() }, child: ManuallyDrop::new(None), child_value: ManuallyDrop::new(None), child_closing: false, index: 0, source: ManuallyDrop::new(None), output: ManuallyDrop::new(None), spent: false, closing: false, close: RetainedCloneClose::default() }
    }
}

impl<T: RetainedClone, const N: usize> RetainedCloneCursor<[T; N]> for ArrayCursor<T, N> {
    fn output_ready(&self) -> bool { !self.closing && !self.spent && self.output.is_some() }
    fn close_started(&self) -> bool { self.closing }

    fn advance_demands(&self, source: RetainedCloneRef<'_, [T; N]>, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.closing || self.spent { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"array clone cannot quote a closing or spent owner")); }
        if !source.check_binding(&self.source)? { return Ok(RetirementDemand { copy_bytes: source.binding_copy_bytes(), depth: 1, ..Default::default() }); }
        if self.output_ready() { return Ok(Default::default()); }
        if T::BITWISE && self.index==0 && N<=RETAINED_CLONE_BULK_PAGE_ITEMS && size_of::<[T;N]>()<=RETAINED_CLONE_BULK_PAGE_BYTES && size_of::<[T;N]>()<=body { return Ok(RetirementDemand {copy_bytes:size_of::<[T;N]>(),depth:1,..Default::default()}); }
        if let Some(child) = self.child.as_ref() {
            if self.child_value.is_some() {
                return if child.terminal_is_empty() { Ok(RetirementDemand { copy_bytes: size_of::<T>(), depth: 1, ..Default::default() }) } else { nested_retained_clone_close_demands::<T>(child,body) };
            }
            if child.output_ready(){let mut d=child.take_demand()?;d.depth=d.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"array child output depth overflow"))?;return Ok(d)}
            return nested_retained_clone_demands::<T>(child,source.project(self.index + 1, |values| &values[self.index]),body);
        }
        if T::BITWISE && self.index<N && size_of::<T>()<=RETAINED_CLONE_BULK_PAGE_BYTES {
            let count=(N-self.index).min(bulk_run_elements(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body,..Default::default()},size_of::<T>()).max(1));
            return Ok(RetirementDemand {copy_bytes:count.saturating_mul(size_of::<T>()),depth:1,..Default::default()});
        }
        Ok(RetirementDemand { copy_bytes: 0, depth: 1, ..Default::default() })
    }

    fn advance(&mut self, source: RetainedCloneRef<'_, [T; N]>, body: usize, grant: RetainedCloneGrant) ->RetainedWorkOutcome{let mut work=0;let result=(||{
        let demand = self.advance_demands(source,body)?;
        if self.output_ready() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if !retained_clone_grant_admits(grant,demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress) = {let turn=source.bind(&mut self.source,grant);work+=turn.work;turn.result}? { return Ok(RetainedCloneStep::Progress(progress)); }
        if T::BITWISE && self.index==0 && N<=RETAINED_CLONE_BULK_PAGE_ITEMS && size_of::<[T;N]>()<=RETAINED_CLONE_BULK_PAGE_BYTES && size_of::<[T;N]>()<=body {
            work+=1;*self.output=Some(T::bitwise_array(source.get()).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"bitwise array refused bounded original copy"))?);
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));
        }
        if let Some(child) = self.child.as_mut() {
            if self.child_value.is_some() {
                if !child.terminal_is_empty() {
                    let nested = nested_retained_clone_grant(grant)?;
                    let step = {let turn=child.close_step(nested);work+=turn.work;turn.result}?;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(nested,step,child.terminal_is_empty(),"array original child scaffold")?.progress()));
                }
                *self.child = None;work+=1;
                self.child_closing = false;
                self.slots[self.index].write(self.child_value.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated,"array cloned row owner absent"))?);
                self.index += 1;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
            }
            if child.output_ready(){let nested=nested_retained_clone_grant(grant)?;let turn=child.take(nested);work+=turn.work;let Some((value,p))=turn.result?else{return Ok(RetainedCloneStep::Progress(Default::default()))};*self.child_value=Some(value);child.begin_close();self.child_closing=true;return Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(nested,p,"array child output handoff")?))}
            let nested = nested_retained_clone_grant(grant)?;
            let step = {let turn=child.advance(source.project(self.index + 1, |values| &values[self.index]),body,nested);work+=turn.work;turn.result}?;
            return Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(nested,step.progress(),"array original row clone")?));
        }
        if self.index == N {
            work+=1;*self.output = Some(unsafe { self.slots.as_ptr().cast::<[T; N]>().read() });
            self.index = 0;
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        if T::BITWISE && size_of::<T>()<=RETAINED_CLONE_BULK_PAGE_BYTES {
            let count=(N-self.index).min(bulk_run_elements(RetainedCloneGrant{maximum_copy_bytes:body,..grant},size_of::<T>()).max(1));
            let first=self.index;work+=1;
            for _ in 0..count {
                let row:&[T;1]=source.get()[self.index..self.index+1].try_into().expect("one original array element");
                let Some([value])=T::bitwise_array(row) else { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"bitwise array row refused original copy").with_retained_progress(bulk_run_progress(self.index-first,size_of::<T>()))); };
                self.slots[self.index].write(value);
                self.index+=1;
            }
            return Ok(RetainedCloneStep::Progress(bulk_run_progress(count,size_of::<T>())));
        }
        work+=1;*self.child = Some(T::retained_clone_cursor());
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    })();crate::RetainedWorkOutcome{result,work}}

    fn take_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::retained_clone::retained_clone_take_demand())}
    fn take(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<([T; N],RetainedCloneProgress)>>{if !self.output_ready(){return crate::RetainedWorkOutcome{result:Ok(None),work:0}}let turn=crate::retained_clone::take_retained_clone_output(&mut *self.output,grant);if matches!(&turn.result,Ok(Some(_))){self.spent = true;}turn}

    fn begin_close(&mut self) -> bool {
        if self.closing { return false; }
        self.closing = true;
        true
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) ->RetainedWorkOutcome{let mut work=0;let result=(||{
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"array clone must begin close before granted retirement")); }
        if !retained_clone_grant_admits(grant,retained_clone_close_demands::<[T; N]>(self,grant.maximum_copy_bytes)?) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(child) = self.child.as_mut() {
            if !self.child_closing { child.begin_close(); self.child_closing=true;work+=1; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            if !child.terminal_is_empty() { let nested = nested_retained_clone_grant(grant)?; let turn=child.close_step(nested);work+=turn.work;let step=turn.result?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(nested,step,child.terminal_is_empty(),"array child close")?.progress())); }
            *self.child = None;
            self.child_closing = false;work+=1;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<T::Cursor>(), ..Default::default() }));
        }
        if !self.close.is_empty() { let turn=self.close.step_granted(grant);work+=turn.work;return turn.result; }
        {let turn=self.close.begin_granted(&mut self.child_value,grant);work+=turn.work;if let Some(step)=turn.result?{return Ok(step)}}
        if self.index > 0 {
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<T>(), ..Default::default() };
            if grant.maximum_depth == 0 || !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.index -= 1;work+=1;
            *self.child_value = Some(unsafe { self.slots[self.index].assume_init_read() });
            return Ok(RetainedCloneStep::Progress(progress));
        }
        {let turn=self.close.begin_granted(&mut self.output,grant);work+=turn.work;if let Some(step)=turn.result?{return Ok(step)}}
        {let turn=close_retained_binding(&mut self.source,grant);work+=turn.work;turn.result}
    })();crate::RetainedWorkOutcome{result,work}}

    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Ok(Default::default());}
        if let Some(child)=self.child.as_ref(){if !self.child_closing{return Ok(RetirementDemand{depth:1,..Default::default()});}return if child.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<T::Cursor>(),depth:1,..Default::default()})}else{nested_retained_clone_close_demands::<T>(child,body)};}
        if !self.close.is_empty(){return self.close.next_owner_demands::<[T;N]>(false,&self.source,body);}
        if self.child_value.is_some(){return self.close.next_owner_demands::<T>(true,&self.source,body);}
        if self.index>0{return Ok(RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()});}
        self.close.next_owner_demands::<[T;N]>(self.output.is_some(),&self.source,body)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    fn terminal_is_empty(&self) -> bool { self.closing && self.child.is_none() && self.child_value.is_none() && self.output.is_none() && self.index == 0 && self.close.is_empty() && self.source.is_none() }
}

impl<T:RetainedClone,const N:usize> Drop for ArrayCursor<T,N> {
    fn drop(&mut self) { assert!((self.index==0 && self.child.is_none() && self.child_value.is_none() && self.output.is_none() && self.source.is_none() && self.close.is_empty()) || std::thread::panicking(),"array clone dropped before original owner closure"); }
}

impl<T: RetainedClone, const N: usize> RetainedClone for [T; N] {
    type Cursor = ArrayCursor<T, N>;
    fn retained_clone_cursor() -> Self::Cursor { ArrayCursor::default() }
    const BITWISE: bool = T::BITWISE;
    fn bitwise_array<const M: usize>(source: &[Self; M]) -> Option<[Self; M]> {
        if !T::BITWISE { return None; }
        Some(std::array::from_fn(|index| T::bitwise_array(&source[index]).expect("bitwise array rows copy bitwise")))
    }
    fn bitwise_extend(target: &mut Vec<Self>, source: &[Self]) -> bool {
        if !T::BITWISE { return false; }
        target.extend(source.iter().map(|row| T::bitwise_array(row).expect("bitwise array rows copy bitwise")));
        true
    }
}
