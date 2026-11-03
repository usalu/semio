//! 🎁️ Option cursor of the retained clone contract: the option cursor with its default, cursor and retained clone implementations. See [`super`].

use super::*;

pub struct OptionCursor<T: RetainedClone> {
    pub(super) child: T::Cursor,
    pub(super) child_value: Option<T>,
    child_closing: bool,
    pub(super) output: Option<Option<T>>,
    pub(super) source: Option<RetainedCloneBinding>,
    pub(super) spent: bool,
    pub(super) closing: bool,
    pub(super) close: RetainedCloneClose,
}

impl<T: RetainedClone> Default for OptionCursor<T> {
    fn default() -> Self {
        Self { child: T::retained_clone_cursor(), child_value: None, child_closing: false, output: None, source: None, spent: false, closing: false, close: RetainedCloneClose::default() }
    }
}

impl<T: RetainedClone> RetainedCloneCursor<Option<T>> for OptionCursor<T> {
    fn output_ready(&self)->bool{!self.closing&&!self.spent&&self.output.is_some()}
    fn advance_demands(&self,source:RetainedCloneRef<'_,Option<T>>,body:usize)->Result<RetirementDemand,ValueError>{if let Some(demand)=source.binding_demand(&self.source)?{return Ok(demand)}if self.closing||self.spent{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"optional original clone is closing or spent"))}if self.output.is_some()||source.get().is_none(){return Ok(RetirementDemand{depth:1,..Default::default()})}if self.child_value.is_some(){let mut demand=clone_cursor_close_demand::<T>(&self.child,body)?;demand.depth=demand.depth.max(1);return Ok(demand)}if self.child.output_ready(){return self.child.take_demand()}self.child.advance_demands(source.try_project(1,|value|value.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained optional child is absent")))?,body)}

    fn advance(&mut self, source: RetainedCloneRef<'_, Option<T>>, body: usize, grant: RetainedCloneGrant) ->RetainedWorkOutcome{let mut work=0;let result=(||{
        if self.closing {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "retained optional clone cursor is closing"));
        }
        if self.spent {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "retained optional clone cursor is spent"));
        }
        if !retained_clone_grant_admits(grant,self.advance_demands(source,body)?){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.output.is_some() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)={let turn=source.bind(&mut self.source,grant);work+=turn.work;turn.result}?{return Ok(RetainedCloneStep::Progress(progress));}
        let source_value = source.get();
        match source_value {
            None => {
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                self.output = Some(None);work+=1;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
            Some(_) => {
                if self.child_value.is_some() {
                    if !self.child.terminal_is_empty() {
                        if grant.maximum_items == 0 {
                            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                        }
                        let step = {let turn=self.child.close_step(grant);work+=turn.work;turn.result}?;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.child.terminal_is_empty(), "retained optional child scaffold close")?.progress()));
                    }
                    if grant.maximum_items == 0 {
                        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                    }
                    work+=1;self.output = Some(Some(self.child_value.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "retained optional child owner is missing"))?));
                    return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
                }
                if self.child.output_ready(){
                 let turn=self.child.take(grant);work+=turn.work;let Some((value,p))=turn.result?else{return Ok(RetainedCloneStep::Progress(Default::default()))};self.child_value=Some(value);self.child.begin_close();self.child_closing=true;
                 return Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant,p,"optional child output handoff")?))
                }
                let turn=self.child.advance(source.try_project(1,|value|value.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained optional child is absent")))?,body,grant);work+=turn.work;
                Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant,turn.result?.progress(),"retained optional child")?))
            }
        }
    })();RetainedWorkOutcome{result,work}}

    fn take_demand(&self)->Result<RetirementDemand,ValueError>{Ok(retained_clone_take_demand())}
    fn take(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Option<T>,RetainedCloneProgress)>>{if !self.output_ready(){return RetainedWorkOutcome{result:Ok(None),work:0}}let turn=take_retained_clone_output(&mut self.output,grant);if matches!(&turn.result,Ok(Some(_))){self.spent = true;}turn}

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    

    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.closing{return Ok(Default::default());}
        if !self.child.terminal_is_empty(){let mut d=if self.child_closing{self.child.close_demands(body)?}else{Default::default()};d.depth=d.depth.max(1);return Ok(d);}
        if !self.close.is_empty(){return self.close.next_owner_demands::<T>(false,&None,body);}
        if self.child_value.is_some(){return self.close.next_owner_demands::<T>(true,&None,body);}
        self.close.next_owner_demands::<Option<T>>(self.output.is_some(),&self.source,body)
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) ->RetainedWorkOutcome{let mut work=0;let result=(||{
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "native owner must begin close before granted retirement")); }
        if !retained_clone_grant_admits(grant,self.close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if !self.child.terminal_is_empty() {
            if !self.child_closing {self.child.begin_close();self.child_closing=true;work+=1; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            let turn=self.child.close_step(grant);work+=turn.work;let step=turn.result?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.child.terminal_is_empty(), "retained child close")?.progress()));
        }
        if !self.close.is_empty() { let turn=self.close.step_granted(grant);work+=turn.work;return turn.result; }
        {let turn=self.close.begin_granted(&mut self.child_value, grant);work+=turn.work;if let Some(step)=turn.result?{return Ok(step)}}
        {let turn=self.close.begin_granted(&mut self.output, grant);work+=turn.work;if let Some(step)=turn.result?{return Ok(step)}}
        {let turn=close_retained_binding(&mut self.source, grant);work+=turn.work;turn.result}
    })();RetainedWorkOutcome{result,work}}

    fn next_close_depth_demand(&self)->Result<usize,ValueError>{if !self.closing {return Ok(0);} if !self.child.terminal_is_empty(){return self.child.next_close_depth_demand();} self.close.next_owner_depth_with_binding(self.child_value.is_some()||self.output.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        if !self.child.terminal_is_empty() { return self.child.next_close_copy_byte_demand(); }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        if !self.child.terminal_is_empty() { return self.child.next_close_capacity_byte_demand(maximum_release_bytes); }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if self.child_value.is_some() { return self.close.next_owner_capacity_with_binding::<T>(true,maximum_release_bytes,&self.source); }
        self.close.next_owner_capacity_with_binding::<Option<T>>(self.output.is_some(),maximum_release_bytes,&self.source)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.closing { return Ok(0); }
        if !self.child.terminal_is_empty() { return self.child.next_close_release_byte_demand(); }
        self.close.next_release_with_binding(&self.source)
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.child.terminal_is_empty() && self.child_value.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none() }
}

impl<T: RetainedClone> RetainedClone for Option<T> {
    type Cursor = OptionCursor<T>;
    fn retained_clone_cursor() -> Self::Cursor {
        OptionCursor::default()
    }
}
