//! 🧮️ Array cursor of the retained clone contract: the fixed array cursor with its default, cursor and retained clone implementations. See [`super`].

use super::*;

/// 🧮️ Clones a fixed array: scalar elements in one bitwise turn, every other element through its own bounded cursor.
pub struct ArrayCursor<T: RetainedClone, const N: usize> {
    slots: [Option<T>; N],
    pub(super) child: Option<T::Cursor>,
    pub(super) child_value: Option<T>,
    pub(super) index: usize,
    pub(super) source: Option<RetainedCloneBinding>,
    pub(super) output: Option<[T; N]>,
    pub(super) spent: bool,
    pub(super) closing: bool,
    pub(super) close: RetainedCloneClose,
}

impl<T: RetainedClone, const N: usize> Default for ArrayCursor<T, N> {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None), child: None, child_value: None, index: 0, source: None, output: None, spent: false, closing: false, close: RetainedCloneClose::default() }
    }
}

impl<T: RetainedClone, const N: usize> RetainedCloneCursor<[T; N]> for ArrayCursor<T, N> {
    fn output_ready(&self)->bool{self.output.is_some()}
    fn advance_demands(&self,source:RetainedCloneRef<'_,[T;N]>,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if let Some(demand)=source.binding_demand(&self.source)?{return Ok(demand)}if self.closing||self.spent{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"array original clone is closing or spent"))}if self.output.is_some(){return Ok(Default::default())}if T::BITWISE||self.index==N{return Ok(crate::RetirementDemand{copy_bytes:size_of::<[T;N]>(),depth:1,..Default::default()})}if self.child_value.is_some(){return clone_cursor_close_demand::<T>(self.child.as_ref().ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"array original child absent"))?,body)}let projected=source.project(self.index+1,|array|&array[self.index]);match self.child.as_ref(){Some(child)=>child.advance_demands(projected,body),None=>T::retained_clone_cursor().advance_demands(projected,body)}}

    fn advance(&mut self, source: RetainedCloneRef<'_, [T; N]>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained array clone cursor is closing")); }
        if self.spent { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained array clone cursor is spent")); }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.output.is_some() { return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())); }
        if let Some(progress) = source.bind(&mut self.source, grant)? { return Ok(RetainedCloneStep::Progress(progress)); }
        if T::BITWISE {
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<[T; N]>(), ..Default::default() };
            if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.output = Some(T::bitwise_array(source.get()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "bitwise array element lacks its bitwise copy"))?);
            return Ok(RetainedCloneStep::Complete(progress));
        }
        if self.child_value.is_some() {
            let child = self.child.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained array child cursor is missing"))?;
            if !child.terminal_is_empty() {
                if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let step = child.close_step(grant)?;
                return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained array child scaffold close")?.progress()));
            }
            if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.child = None;
            self.slots[self.index] = self.child_value.take();
            self.index += 1;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if self.index == N {
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<[T; N]>(), ..Default::default() };
            if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let mut taken = std::array::from_fn::<Option<T>, N, _>(|_| None);
            for (slot, target) in self.slots.iter_mut().zip(taken.iter_mut()) { *target = slot.take(); }
            if taken.iter().any(Option::is_none) {
                for (slot, target) in self.slots.iter_mut().zip(taken.iter_mut()) { if slot.is_none() { *slot = target.take(); } }
                return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained array lost an element owner"));
            }
            self.output = Some(taken.map(|slot| slot.expect("checked retained array element")));
            return Ok(RetainedCloneStep::Complete(progress));
        }
        let index = self.index;
        let child = self.child.get_or_insert_with(T::retained_clone_cursor);
        match child.advance(source.project(index + 1, |array| &array[index]), grant)? {
            RetainedCloneStep::Progress(progress) => Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant, progress, "retained array child")?)),
            RetainedCloneStep::Complete(progress) => {
                let progress = admit_retained_clone_progress(grant, progress, "retained array child")?;
                self.child_value = Some(child.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained array child completed without an owner"))?);
                let _ = child.begin_close();
                Ok(RetainedCloneStep::Progress(progress))
            }
        }
    }

    fn take(&mut self) -> Option<[T; N]> {
        let output = self.output.take();
        if output.is_some() { self.spent = true; }
        output
    }

    fn begin_close(&mut self) -> bool {
        if self.closing { return false; }
        self.closing = true;
        true
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "array clone cursor must begin close before granted retirement")); }
        if let Some(child) = self.child.as_mut() {
            if child.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            if !child.terminal_is_empty() { let step = child.close_step(grant)?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained array child close")?.progress())); }
            self.child = None;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.child_value, grant)? { return Ok(step); }
        for slot in self.slots.iter_mut() { if let Some(step) = self.close.begin_granted(slot, grant)? { return Ok(step); } }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child) = self.child.as_ref() { return child.next_close_depth_demand(); }
        self.close.next_owner_depth_with_binding(self.child_value.is_some() || self.output.is_some() || self.slots.iter().any(Option::is_some), &self.source)
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child) = self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_copy_byte_demand() }; }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child) = self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_capacity_byte_demand(maximum_release_bytes) }; }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if self.child_value.is_some() || self.slots.iter().any(Option::is_some) { return self.close.next_owner_capacity_with_binding::<T>(true, maximum_release_bytes, &self.source); }
        self.close.next_owner_capacity_with_binding::<[T; N]>(self.output.is_some(), maximum_release_bytes, &self.source)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child) = self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_release_byte_demand() }; }
        self.close.next_release_with_binding(&self.source)
    }
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.child.is_none() && self.child_value.is_none() && self.output.is_none() && self.slots.iter().all(Option::is_none) && self.close.is_empty() && self.source.is_none()
    }
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
