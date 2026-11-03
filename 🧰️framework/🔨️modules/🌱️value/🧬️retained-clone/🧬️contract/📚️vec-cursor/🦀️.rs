//! 📚️ Vec cursor of the retained clone contract: the vector cursor with its default, constructor, cursor and retained clone implementations. See [`super`].

use super::*;

pub struct VecCursor<T: RetainedClone> {
    pub(super) values: Vec<T>,
    pub(super) child: Option<T::Cursor>,
    pub(super) child_value: Option<T>,
    child_closing: bool,
    pub(super) index: usize,
    pub(super) source: Option<RetainedCloneBinding>,
    pub(super) phase: u8,
    pub(super) output: Option<Vec<T>>,
    pub(super) closing: bool,
    pub(super) close: RetainedCloneClose,
}

impl<T: RetainedClone> Default for VecCursor<T> {
    fn default() -> Self {
        Self { values: Vec::new(), child: None, child_value: None, child_closing: false, index: 0, source: None, phase: 0, output: None, closing: false, close: RetainedCloneClose::default() }
    }
}

impl<T: RetainedClone> VecCursor<T> {
    /// 🚚️ Copies one bounded run of a bitwise vector per turn into the capacity reserved up front, so the clone costs one item and at most one page of payload per turn instead of a cursor lifecycle per element.
    fn advance_bitwise(&mut self, source: &[T], body: usize, grant: RetainedCloneGrant) ->RetainedWorkOutcome{let mut work=0;let result=(||{
        if self.index != self.values.len() || self.index > source.len() {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "retained bitwise vector clone progress disagrees with its source"));
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let width = size_of::<T>();
        let remaining = source.len() - self.index;
        let count = remaining.min(bulk_run_elements(RetainedCloneGrant{maximum_copy_bytes:body,..grant}, width).max(1));
        if remaining > 0 && count == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let end = self.index + count;
        work+=1;if !T::bitwise_extend(&mut self.values, &source[self.index..end]) {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "bitwise vector element lacks its bitwise copy"));
        }
        self.index = end;
        let progress = bulk_run_progress(count, width);
        if end < source.len() { return Ok(RetainedCloneStep::Progress(progress)); }
        
        self.phase = 2;
        Ok(RetainedCloneStep::Progress(progress))
    })();RetainedWorkOutcome{result,work}}
}

impl<T: RetainedClone> RetainedCloneCursor<Vec<T>> for VecCursor<T> {
    fn output_ready(&self)->bool{!self.closing&&self.phase!=3&&self.output.is_some()}
    fn advance_demands(&self,source:RetainedCloneRef<'_,Vec<T>>,body:usize)->Result<RetirementDemand,ValueError>{
     if let Some(d)=source.binding_demand(&self.source)?{return Ok(d)}if self.closing||self.phase==3{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"vector original clone is closing or spent"))}let original=source.get();
     if self.phase==0{return Ok(RetirementDemand{capacity_bytes:original.len().checked_mul(size_of::<T>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"vector original reserve overflow"))?,depth:1,..Default::default()})}
     if self.phase==2||self.index==original.len(){return Ok(RetirementDemand{depth:1,..Default::default()})}
     if T::BITWISE{let width=size_of::<T>();let count=(original.len()-self.index).min(bulk_run_elements(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body,..Default::default()},width).max(1));return Ok(RetirementDemand{copy_bytes:count.saturating_mul(width),depth:1,..Default::default()})}
     if let Some(child)=self.child.as_ref(){if self.child_value.is_some(){return if child.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()})}else{child.close_demands(body)}}if child.output_ready(){return child.take_demand()}return child.advance_demands(source.project(self.index+1,|values|&values[self.index]),body)}
     Ok(RetirementDemand{depth:1,..Default::default()})
    }

    fn advance(&mut self, source: RetainedCloneRef<'_, Vec<T>>, body: usize, grant: RetainedCloneGrant) ->RetainedWorkOutcome{let mut work=0;let result=(||{
        let demand = self.advance_demands(source,body)?;
        if !retained_clone_grant_admits(grant,demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.closing {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "retained vector clone cursor is closing"));
        }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)={let turn=source.bind(&mut self.source,grant);work+=turn.work;turn.result}?{return Ok(RetainedCloneStep::Progress(progress));}
        let source_value = source.get();
        if self.phase == 0 {
            let planned = source_value.len().checked_mul(size_of::<T>()).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "retained vector clone capacity overflow"))?;
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned, released_bytes: 0 };
            if !progress.fits(grant) {
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
            }
            work+=1;self.values.try_reserve_exact(source_value.len()).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "retained vector clone capacity allocation failed"))?;
            let actual = self.values.capacity().checked_mul(size_of::<T>()).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "retained vector clone actual capacity overflow"))?;
            if actual > grant.maximum_capacity_bytes {
                return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "retained vector clone allocator exceeded its admitted capacity").with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:actual,..progress}));
            }
            self.phase = 1;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { retained_capacity_bytes: actual, ..progress }));
        }
        if self.phase == 2 || self.index == source_value.len() {
            if self.output.is_some() { return Ok(RetainedCloneStep::Complete(Default::default())); }
            let progress=RetainedCloneProgress{copied_items:1,copied_bytes:0,..Default::default()};
            if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
            work+=1;self.output=Some(std::mem::take(&mut self.values));self.phase=2;return Ok(RetainedCloneStep::Complete(progress));
        }
        if self.phase == 3 {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "retained vector clone cursor is spent"));
        }
        if T::BITWISE { return {let turn=self.advance_bitwise(source_value, body, grant);work+=turn.work;turn.result}; }
        if self.child_value.is_some(){
         let child=self.child.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"vector original child absent"))?;
         if !child.terminal_is_empty(){let turn=child.close_step(grant);work+=turn.work;let step=turn.result?;return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant,step,child.terminal_is_empty(),"retained vector child scaffold close")?.progress()))}
         self.child=None;self.child_closing=false;let value=self.child_value.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained vector child owner is missing"))?;self.values.push(value);self.index+=1;work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<T>(),..Default::default()}))
        }
        if self.child.is_none(){self.child=Some(T::retained_clone_cursor());work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
        let child=self.child.as_mut().ok_or_else(||ValueError::invariant("child absent"))?;
        if child.output_ready(){let turn=child.take(grant);work+=turn.work;let Some((value,p))=turn.result?else{return Ok(RetainedCloneStep::Progress(Default::default()))};self.child_value=Some(value);child.begin_close();self.child_closing=true;return Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant,p,"vector child output handoff")?))}
        let turn=child.advance(source.project(self.index+1,|values|&values[self.index]),body,grant);work+=turn.work;Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant,turn.result?.progress(),"retained vector child")?))
    })();RetainedWorkOutcome{result,work}}

    fn take_demand(&self)->Result<RetirementDemand,ValueError>{Ok(retained_clone_take_demand())}
    fn take(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Vec<T>,RetainedCloneProgress)>>{if !self.output_ready(){return RetainedWorkOutcome{result:Ok(None),work:0}}let turn=take_retained_clone_output(&mut self.output,grant);if matches!(&turn.result,Ok(Some(_))){self.phase = 3;}turn}

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    

    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.closing{return Ok(Default::default());}
        if let Some(child)=self.child.as_ref(){let mut d=if !self.child_closing||child.terminal_is_empty(){Default::default()}else{child.close_demands(body)?};d.depth=d.depth.max(1);return Ok(d);}
        if !self.close.is_empty(){return self.close.next_owner_demands::<T>(false,&None,body);}
        if self.child_value.is_some(){return self.close.next_owner_demands::<T>(true,&None,body);}
        self.close.next_owner_demands::<Vec<T>>(self.output.is_some()||!self.values.is_empty()||(size_of::<T>()!=0&&self.values.capacity()>0),&self.source,body)
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) ->RetainedWorkOutcome{let mut work=0;let result=(||{
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "native owner must begin close before granted retirement")); }
        if !retained_clone_grant_admits(grant,self.close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(child) = self.child.as_mut() {
            if !self.child_closing {child.begin_close();self.child_closing=true;work+=1; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            if !child.terminal_is_empty() { let turn=child.close_step(grant);work+=turn.work;let step=turn.result?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained child close")?.progress())); }
            let progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
            self.child = None;self.child_closing=false;work+=1;
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if !self.close.is_empty() { let turn=self.close.step_granted(grant);work+=turn.work;return turn.result; }
        {let turn=self.close.begin_granted(&mut self.child_value, grant);work+=turn.work;if let Some(step)=turn.result?{return Ok(step)}}
        {let turn=self.close.begin_granted(&mut self.output, grant);work+=turn.work;if let Some(step)=turn.result?{return Ok(step)}}
        if !self.values.is_empty() || (size_of::<T>() != 0 && self.values.capacity() > 0) { {let turn=self.close.begin_empty_granted(&mut self.values, grant);work+=turn.work;if let Some(step)=turn.result?{return Ok(step)}} }
        {let turn=close_retained_binding(&mut self.source, grant);work+=turn.work;turn.result}
    })();RetainedWorkOutcome{result,work}}

    fn next_close_depth_demand(&self)->Result<usize,ValueError>{if !self.closing {return Ok(0);} if let Some(child)=self.child.as_ref(){return child.next_close_depth_demand();} self.close.next_owner_depth_with_binding(self.child_value.is_some()||self.output.is_some()||!self.values.is_empty()||(size_of::<T>()!=0&&self.values.capacity()>0),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child) = self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_copy_byte_demand() }; }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child)=self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_capacity_byte_demand(maximum_release_bytes) }; }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if self.child_value.is_some() { return self.close.next_owner_capacity_with_binding::<T>(true,maximum_release_bytes,&self.source); }
        self.close.next_owner_capacity_with_binding::<Vec<T>>(self.output.is_some()||!self.values.is_empty()||(size_of::<T>()!=0&&self.values.capacity()!=0),maximum_release_bytes,&self.source)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
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
