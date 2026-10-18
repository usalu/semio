//! 📚️ Vec cursor of the retained clone contract: the vector cursor with its default, constructor, cursor and retained clone implementations. See [`super`].

use super::*;

pub struct VecCursor<T: RetainedClone> {
    pub(super) values: Vec<T>,
    pub(super) child: Option<T::Cursor>,
    pub(super) child_value: Option<T>,
    pub(super) index: usize,
    pub(super) source: Option<RetainedCloneBinding>,
    pub(super) phase: u8,
    pub(super) output: Option<Vec<T>>,
    pub(super) closing: bool,
    pub(super) close: RetainedCloneClose,
}

impl<T: RetainedClone> Default for VecCursor<T> {
    fn default() -> Self {
        Self { values: Vec::new(), child: None, child_value: None, index: 0, source: None, phase: 0, output: None, closing: false, close: RetainedCloneClose::default() }
    }
}

impl<T: RetainedClone> VecCursor<T> {
    /// 🚚️ Copies one bounded run of a bitwise vector per turn into the capacity reserved up front, so the clone costs one item and at most one page of payload per turn instead of a cursor lifecycle per element.
    fn advance_bitwise(&mut self, source: &[T], grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.index != self.values.len() || self.index > source.len() {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained bitwise vector clone progress disagrees with its source"));
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let width = size_of::<T>();
        let remaining = source.len() - self.index;
        let count = remaining.min(bulk_run_elements(grant, width));
        if remaining > 0 && count == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let end = self.index + count;
        if !T::bitwise_extend(&mut self.values, &source[self.index..end]) {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "bitwise vector element lacks its bitwise copy"));
        }
        self.index = end;
        let progress = bulk_run_progress(count, width);
        if end < source.len() { return Ok(RetainedCloneStep::Progress(progress)); }
        self.output = Some(std::mem::take(&mut self.values));
        self.phase = 2;
        Ok(RetainedCloneStep::Complete(progress))
    }
}

impl<T: RetainedClone> RetainedCloneCursor<Vec<T>> for VecCursor<T> {
    fn output_ready(&self)->bool{self.output.is_some()}
    fn advance_demands(&self,source:RetainedCloneRef<'_,Vec<T>>,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if let Some(demand)=source.binding_demand(&self.source)?{return Ok(demand)}if self.closing||self.phase==3{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"vector original clone is closing or spent"))}let original=source.get();if self.phase==0{return Ok(crate::RetirementDemand{capacity_bytes:original.len().checked_mul(size_of::<T>()).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"vector original reserve overflow"))?,depth:1,..Default::default()})}if self.phase==2||self.index==original.len(){return Ok(crate::RetirementDemand{depth:1,..Default::default()})}if T::BITWISE{let width=size_of::<T>();let count=(original.len()-self.index).min(if width==0{usize::MAX}else{body.min(RETAINED_CLONE_BULK_PAGE_BYTES)/width}).max(1);return Ok(crate::RetirementDemand{copy_bytes:count.saturating_mul(width),depth:1,..Default::default()})}if self.child_value.is_some(){return clone_cursor_close_demand::<T>(self.child.as_ref().ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"vector original child absent"))?,body)}let projected=source.project(self.index+1,|values|&values[self.index]);match self.child.as_ref(){Some(child)=>child.advance_demands(projected,body),None=>T::retained_clone_cursor().advance_demands(projected,body)}}

    fn advance(&mut self, source: RetainedCloneRef<'_, Vec<T>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained vector clone cursor is closing"));
        }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)=source.bind(&mut self.source,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
        let source_value = source.get();
        if self.phase == 0 {
            let planned = source_value.len().checked_mul(size_of::<T>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained vector clone capacity overflow"))?;
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned, released_bytes: 0 };
            if !progress.fits(grant) {
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
            }
            self.values.try_reserve_exact(source_value.len()).map_err(|_| crate::ValueError::new(crate::ValueRefusalKind::AllocationFailed, "retained vector clone capacity allocation failed"))?;
            let actual = self.values.capacity().checked_mul(size_of::<T>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained vector clone actual capacity overflow"))?;
            if actual > grant.maximum_capacity_bytes {
                return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained vector clone allocator exceeded its admitted capacity"));
            }
            self.phase = 1;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { retained_capacity_bytes: actual, ..progress }));
        }
        if self.phase == 2 {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if self.phase == 3 {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained vector clone cursor is spent"));
        }
        if T::BITWISE { return self.advance_bitwise(source_value, grant); }
        let mut used = RetainedCloneProgress::default();
        loop {
            let remaining = RetainedCloneGrant {
                maximum_items: grant.maximum_items.saturating_sub(used.copied_items),
                maximum_copy_bytes: grant.maximum_copy_bytes.saturating_sub(used.copied_bytes),
                maximum_capacity_bytes: grant.maximum_capacity_bytes.saturating_sub(used.retained_capacity_bytes),
                maximum_depth: grant.maximum_depth, maximum_release_bytes: grant.maximum_release_bytes.saturating_sub(used.released_bytes) };
            if let Some(child) = self.child.as_mut() {
                if self.child_value.is_some() {
                    if !child.terminal_is_empty() {
                        if remaining.maximum_items == 0 {
                            return Ok(RetainedCloneStep::Progress(used));
                        }
                        let step = child.close_step(remaining)?;
                        let progress = admit_retained_clone_close(remaining, step, child.terminal_is_empty(), "retained vector child scaffold close")?.progress();
                        if progress == RetainedCloneProgress::default() { return Ok(RetainedCloneStep::Progress(used)); }
                        used = used.checked_add(progress)?;
                        continue;
                    }
                    if remaining.maximum_items == 0 {
                        return Ok(RetainedCloneStep::Progress(used));
                    }
                    self.child = None;
                    self.values.push(self.child_value.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained vector child owner is missing"))?);
                    self.index += 1;
                    used = used.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                    if used.copied_items == grant.maximum_items {
                        return Ok(RetainedCloneStep::Progress(used));
                    }
                    continue;
                }
            }
            if self.index == source_value.len() {
                if remaining.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(used));
                }
                self.output = Some(std::mem::take(&mut self.values));
                self.phase = 2;
                used = used.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                return Ok(RetainedCloneStep::Complete(used));
            }
            if remaining.maximum_items == 0 && remaining.maximum_copy_bytes == 0 && remaining.maximum_capacity_bytes == 0 && remaining.maximum_release_bytes == 0 {
                return Ok(RetainedCloneStep::Progress(used));
            }
            let child = self.child.get_or_insert_with(T::retained_clone_cursor);
            match child.advance(source.project(self.index + 1, |values| &values[self.index]), remaining)? {
                RetainedCloneStep::Progress(progress) => {
                    let progress = admit_retained_clone_progress(remaining, progress, "retained vector child")?;
                    if progress == RetainedCloneProgress::default() {
                        return Ok(RetainedCloneStep::Progress(used));
                    }
                    used = used.checked_add(progress)?;
                }
                RetainedCloneStep::Complete(progress) => {
                    let progress = admit_retained_clone_progress(remaining, progress, "retained vector child")?;
                    used = used.checked_add(progress)?;
                    if used.copied_items == grant.maximum_items {
                        return Ok(RetainedCloneStep::Progress(used));
                    }
                    self.child_value = Some(child.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained vector child completed without an owner"))?);
                    let _ = child.begin_close();
                    used = used.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                    continue;
                }
            }
        }
    }

    fn take(&mut self) -> Option<Vec<T>> {
        if self.phase != 2 {
            return None;
        }
        self.phase = 3;
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
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "native owner must begin close before granted retirement")); }
        if let Some(child) = self.child.as_mut() {
            if child.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            if !child.terminal_is_empty() { let step = child.close_step(grant)?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained child close")?.progress())); }
            let progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
            self.child = None;
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.child_value, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        if !self.values.is_empty() || (size_of::<T>() != 0 && self.values.capacity() > 0) { if let Some(step) = self.close.begin_default_granted(&mut self.values, grant)? { return Ok(step); } }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if !self.closing {return Ok(0);} if let Some(child)=self.child.as_ref(){return child.next_close_depth_demand();} self.close.next_owner_depth_with_binding(self.child_value.is_some()||self.output.is_some()||!self.values.is_empty()||(size_of::<T>()!=0&&self.values.capacity()>0),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child) = self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_copy_byte_demand() }; }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child)=self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_capacity_byte_demand(maximum_release_bytes) }; }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if self.child_value.is_some() { return self.close.next_owner_capacity_with_binding::<T>(true,maximum_release_bytes,&self.source); }
        self.close.next_owner_capacity_with_binding::<Vec<T>>(self.output.is_some()||!self.values.is_empty()||(size_of::<T>()!=0&&self.values.capacity()!=0),maximum_release_bytes,&self.source)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child)=self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_release_byte_demand() }; }
        self.close.next_release_with_binding(&self.source)
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.values.is_empty() && (size_of::<T>() == 0 || self.values.capacity() == 0) && self.child.is_none() && self.child_value.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none() }
}

impl<T: RetainedClone> RetainedClone for Vec<T> {
    type Cursor = VecCursor<T>;
    fn retained_clone_cursor() -> Self::Cursor {
        VecCursor::default()
    }
}
