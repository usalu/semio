//! 🔤️ The private byte owner copies only the bound immutable String and becomes a String after the whole original byte sequence is retained.

use super::*;

#[derive(Default)]
pub struct StringCursor {
    output: Option<Vec<u8>>,
    source: Option<RetainedCloneBinding>,
    phase: u8,
    closing: bool,
    close: RetainedCloneClose,
}

impl RetainedCloneCursor<String> for StringCursor {
    fn output_ready(&self)->bool{!self.closing&&self.phase==2&&self.output.is_some()}
    fn advance_demands(&self,source:RetainedCloneRef<'_,String>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{let binding=source.binding_demand(&self.source)?;if self.closing||self.phase==3{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"string original clone is closing or spent"))}if maximum_work_units==0{return Ok(Default::default())}if let Some(demand)=binding{return Ok(demand)}let original=source.get();let(copy,capacity)=match self.phase{0=>(0,original.len()),1=>{let start=self.output.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"string original output absent"))?.len();if start>original.len(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"string original offset changed"))}((original.len()-start).min(body).min(RETAINED_CLONE_BULK_PAGE_BYTES).min(usize::try_from(maximum_work_units).unwrap_or(usize::MAX)),0)},_=>(0,0)};Ok(RetirementDemand{copy_bytes:copy,capacity_bytes:capacity,depth:1,..Default::default()})}

    fn advance_work_demand(&self,source:RetainedCloneRef<'_,String>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{let demand=self.advance_demands(source,body,maximum_work_units)?;if maximum_work_units==0{return Ok(0)}if self.source.is_none()||self.phase==0{return Ok(1)}if self.phase==1{if demand.copy_bytes>0{return Ok(demand.copy_bytes as u64)}let done=self.output.as_ref().ok_or_else(||ValueError::invariant("string original output absent"))?.len()==source.get().len();return Ok(u64::from(done))}Ok(0)}
    fn advance(&mut self, source: RetainedCloneRef<'_, String>, body: usize, maximum_work_units:u64, grant: RetainedCloneGrant) ->crate::OriginalRetirementOutcome{let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
        let demand = self.advance_demands(source,body,maximum_work_units)?;
        if maximum_work_units==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        if !retained_clone_grant_admits(grant,demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.closing {
            return Err((ValueError::literal(ValueRefusalKind::InvariantViolated, "retained string clone cursor is closing")).into());
        }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)={let turn=source.bind(&mut self.source,grant);work+=turn.work;turn.result}?{return Ok(RetainedCloneStep::Progress(progress));}
        let source = source.get();
        match self.phase {
            0 => {
                let planned = source.len();
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned, released_bytes: 0 };
                if !progress.fits(grant) {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                let mut output = Vec::new();
                work+=1;output.try_reserve_exact(planned).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "retained string clone capacity allocation failed"))?;
                let actual = output.capacity();
                if actual > grant.maximum_capacity_bytes {
                    self.output=Some(output);return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"retained string clone allocator exceeded its admitted capacity").with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:actual,..progress})).into());
                }
                self.output = Some(output);
                self.phase = 1;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { retained_capacity_bytes: actual, ..progress }))
            }
            1 => {
                let output = self.output.as_mut().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "retained string clone output owner is missing"))?;
                let start = output.len();
                let remaining = source.len().checked_sub(start).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "retained string clone source changed"))?;
                if remaining == 0 {
                    if grant.maximum_items == 0 {
                        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                    }
                    self.phase = 2;work+=1;
                    return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
                }
                let end = start + remaining.min(demand.copy_bytes);
                if end == start {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                output.extend_from_slice(&source.as_bytes()[start..end]);work+=(end-start) as u64;
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: end - start, retained_capacity_bytes: 0, released_bytes: 0 };
                Ok(RetainedCloneStep::Progress(progress))
            }
            2 => Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
            3 => Err((ValueError::literal(ValueRefusalKind::InvariantViolated, "retained string clone cursor is spent")).into()),
            _ => Err((ValueError::literal(ValueRefusalKind::InvariantViolated, "retained string clone state is invalid")).into()),
        }
    })();RetainedWorkOutcome{result,work}}

    fn take_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(self.output_ready()))}
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Ok(0)}self.close.next_owner_work::<Vec<u8>>(self.output.is_some(),&self.source,body)}
    fn take_demand(&self)->Result<RetirementDemand,ValueError>{Ok(retained_clone_take_demand())}
    fn take(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(String,RetainedCloneProgress)>>{if !self.output_ready(){return RetainedWorkOutcome{result:Ok(None),work:0}}let turn=take_retained_clone_output(&mut self.output,grant);let result=turn.result.map(|value|value.map(|(bytes,progress)|{self.phase=3;(unsafe{String::from_utf8_unchecked(bytes)},progress)}));RetainedWorkOutcome{result,work:turn.work}}

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }
    fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(!self.closing))}
    fn begin_close_paid(&mut self)->RetainedWorkOutcome<(),ValueError>{if self.closing{return RetainedWorkOutcome{result:Ok(()),work:0}}self.closing=true;RetainedWorkOutcome{result:Ok(()),work:1}}

    

    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Ok(Default::default());}self.close.next_owner_demands::<Vec<u8>>(self.output.is_some(),&self.source,body)}

    fn close_step(&mut self, grant: RetainedCloneGrant) ->crate::OriginalRetirementOutcome{let mut work=0;let result=(||{
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "clone cursor must begin close before granted retirement").into()); }
        if !self.close.is_empty() { let turn=self.close.step_granted(grant);work+=turn.work;return turn.result; }
        {let turn=self.close.begin_granted(&mut self.output, grant);work+=turn.work;if let Some(step)=turn.result?{return Ok(step)}}
        {let turn=close_retained_binding(&mut self.source, grant);work+=turn.work;turn.result}
    })();RetainedWorkOutcome{result,work}}

    fn next_close_depth_demand(&self)->Result<usize,ValueError>{if !self.closing {return Ok(0);} self.close.next_owner_depth_with_binding(self.output.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_owner_capacity_with_binding::<Vec<u8>>(self.output.is_some(), maximum_release_bytes,&self.source)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
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
