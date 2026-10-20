//! 🔤️ String cursor of the retained clone contract: the string cursor with its cursor implementation and the string retained clone implementation. See [`super`].

use super::*;

#[derive(Default)]
pub struct StringCursor {
    pub(super) output: Option<String>,
    pub(super) source: Option<RetainedCloneBinding>,
    pub(super) phase: u8,
    pub(super) closing: bool,
    pub(super) close: RetainedCloneClose,
}

impl RetainedCloneCursor<String> for StringCursor {
    fn output_ready(&self)->bool{!self.closing&&self.phase==2&&self.output.is_some()}
    fn advance_demands(&self,source:RetainedCloneRef<'_,String>,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if let Some(demand)=source.binding_demand(&self.source)?{return Ok(demand)}if self.closing||self.phase==3{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"string original clone is closing or spent"))}let original=source.get();let(copy,capacity)=match self.phase{0=>(0,original.len()),1=>{let start=self.output.as_ref().ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"string original output absent"))?.len();if start>original.len()||!original.is_char_boundary(start){return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"string original offset changed"))}let mut end=start+(original.len()-start).min(body.min(RETAINED_CLONE_BULK_PAGE_BYTES));while end>start&&!original.is_char_boundary(end){end-=1;}if end==start&&start<original.len(){end=start+original[start..].chars().next().unwrap().len_utf8();}(end-start,0)},_=>(0,0)};Ok(crate::RetirementDemand{copy_bytes:copy,capacity_bytes:capacity,depth:1,..Default::default()})}

    fn advance(&mut self, source: RetainedCloneRef<'_, String>, body: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        let demand = self.advance_demands(source,body)?;
        if !retained_clone_grant_admits(grant,demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone cursor is closing"));
        }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)=source.bind(&mut self.source,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
        let source = source.get();
        match self.phase {
            0 => {
                let planned = source.len();
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned, released_bytes: 0 };
                if !progress.fits(grant) {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                let mut output = String::new();
                output.try_reserve_exact(planned).map_err(|_| crate::ValueError::new(crate::ValueRefusalKind::AllocationFailed, "retained string clone capacity allocation failed"))?;
                let actual = output.capacity();
                if actual > grant.maximum_capacity_bytes {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone allocator exceeded its admitted capacity"));
                }
                self.output = Some(output);
                self.phase = 1;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { retained_capacity_bytes: actual, ..progress }))
            }
            1 => {
                let output = self.output.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone output owner is missing"))?;
                let start = output.len();
                if !source.is_char_boundary(start) {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone source content changed across a UTF-8 boundary"));
                }
                let remaining = source.len().checked_sub(start).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone source changed"))?;
                if remaining == 0 {
                    if grant.maximum_items == 0 {
                        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                    }
                    self.phase = 2;
                    return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
                }
                let maximum_end = start + remaining.min(demand.copy_bytes);
                let mut end = maximum_end;
                while end > start && !source.is_char_boundary(end) {
                    end -= 1;
                }
                if end == start {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                output.push_str(&source[start..end]);
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: end - start, retained_capacity_bytes: 0, released_bytes: 0 };
                Ok(RetainedCloneStep::Progress(progress))
            }
            2 => Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
            3 => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone cursor is spent")),
            _ => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone state is invalid")),
        }
    }

    fn take(&mut self) -> Option<String> {
        if !self.output_ready() {
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

    

    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if !self.closing{return Ok(Default::default());}self.close.next_owner_demands::<String>(self.output.is_some(),&self.source,body)}

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "clone cursor must begin close before granted retirement")); }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if !self.closing {return Ok(0);} self.close.next_owner_depth_with_binding(self.output.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_owner_capacity_with_binding::<String>(self.output.is_some(), maximum_release_bytes,&self.source)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_release_with_binding(&self.source)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.output.is_none() && self.close.is_empty() && self.source.is_none()
    }
}

impl RetainedClone for String {
    type Cursor = StringCursor;
    fn retained_clone_cursor() -> Self::Cursor {
        StringCursor::default()
    }
}
