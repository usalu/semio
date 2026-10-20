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
    fn advance_demands(&self,source:RetainedCloneRef<'_,Option<T>>,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if let Some(demand)=source.binding_demand(&self.source)?{return Ok(demand)}if self.closing||self.spent{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"optional original clone is closing or spent"))}if self.output.is_some()||source.get().is_none(){return Ok(crate::RetirementDemand{depth:1,..Default::default()})}if self.child_value.is_some(){let mut demand=clone_cursor_close_demand::<T>(&self.child,body)?;demand.depth=demand.depth.max(1);return Ok(demand)}if self.child.output_ready(){return Ok(crate::RetirementDemand{depth:1,..Default::default()})}self.child.advance_demands(source.project(1,|value|value.as_ref().unwrap()),body)}

    fn advance(&mut self, source: RetainedCloneRef<'_, Option<T>>, body: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained optional clone cursor is closing"));
        }
        if self.spent {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained optional clone cursor is spent"));
        }
        if self.output.is_some() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)=source.bind(&mut self.source,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
        let source_value = source.get();
        match source_value {
            None => {
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                self.output = Some(None);
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
            Some(_) => {
                if self.child_value.is_some() {
                    if !self.child.terminal_is_empty() {
                        if grant.maximum_items == 0 {
                            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                        }
                        let step = self.child.close_step(grant)?;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.child.terminal_is_empty(), "retained optional child scaffold close")?.progress()));
                    }
                    if grant.maximum_items == 0 {
                        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                    }
                    self.output = Some(Some(self.child_value.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained optional child owner is missing"))?));
                    return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
                }
                match self.child.advance(source.project(1, |value| value.as_ref().expect("validated retained optional child")),body, grant)? {
                    RetainedCloneStep::Progress(progress) => Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant, progress, "retained optional child")?)),
                    RetainedCloneStep::Complete(progress) => {
                        let progress = admit_retained_clone_progress(grant, progress, "retained optional child")?;
                        if progress.copied_items >= grant.maximum_items {
                            return Ok(RetainedCloneStep::Progress(progress));
                        }
                        self.child_value = Some(self.child.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained optional child completed without an owner"))?);
                        let _ = self.child.begin_close();self.child_closing=true;
                        Ok(RetainedCloneStep::Progress(progress.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?))
                    }
                }
            }
        }
    }

    fn take(&mut self) -> Option<Option<T>> {
        if !self.output_ready(){return None;}
        let output = self.output.take();
        if output.is_some() {
            self.spent = true;
        }
        output
    }

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    

    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{
        if !self.closing{return Ok(Default::default());}
        if !self.child.terminal_is_empty(){let mut d=if self.child_closing{self.child.close_demands(body)?}else{Default::default()};d.depth=d.depth.max(1);return Ok(d);}
        if !self.close.is_empty(){return self.close.next_owner_demands::<T>(false,&None,body);}
        if self.child_value.is_some(){return self.close.next_owner_demands::<T>(true,&None,body);}
        self.close.next_owner_demands::<Option<T>>(self.output.is_some(),&self.source,body)
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "native owner must begin close before granted retirement")); }
        if !retained_clone_grant_admits(grant,self.close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if !self.child.terminal_is_empty() {
            if !self.child_closing {self.child.begin_close();self.child_closing=true; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            let step = self.child.close_step(grant)?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.child.terminal_is_empty(), "retained child close")?.progress()));
        }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.child_value, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if !self.closing {return Ok(0);} if !self.child.terminal_is_empty(){return self.child.next_close_depth_demand();} self.close.next_owner_depth_with_binding(self.child_value.is_some()||self.output.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if !self.child.terminal_is_empty() { return self.child.next_close_copy_byte_demand(); }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if !self.child.terminal_is_empty() { return self.child.next_close_capacity_byte_demand(maximum_release_bytes); }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if self.child_value.is_some() { return self.close.next_owner_capacity_with_binding::<T>(true,maximum_release_bytes,&self.source); }
        self.close.next_owner_capacity_with_binding::<Option<T>>(self.output.is_some(),maximum_release_bytes,&self.source)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
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
