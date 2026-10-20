//! 🗃️ Box cursor of the retained clone contract: the box cursor with its default, cursor and retained clone implementations. See [`super`].

use super::*;

pub struct BoxCursor<T: RetainedClone> {
    pub(super) child: Option<Box<T::Cursor>>,
    pub(super) value: Option<T>,
    pub(super) output: Option<Box<T>>,
    pub(super) source: Option<RetainedCloneBinding>,
    pub(super) spent: bool,
    pub(super) closing: bool,
    child_closing: bool,
    pub(super) close: RetainedCloneClose,
}

impl<T: RetainedClone> Default for BoxCursor<T> {
    fn default() -> Self {
        Self { child: None, value: None, output: None, source: None, spent: false, closing: false, child_closing:false, close: RetainedCloneClose::default() }
    }
}

impl<T: RetainedClone> RetainedCloneCursor<Box<T>> for BoxCursor<T> {
    fn output_ready(&self)->bool{!self.closing&&!self.spent&&self.output.is_some()}
    fn advance_demands(&self,source:RetainedCloneRef<'_,Box<T>>,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if let Some(demand)=source.binding_demand(&self.source)?{return Ok(demand)}if self.closing||self.spent{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"boxed original clone is closing or spent"))}if self.output.is_some(){return Ok(Default::default())}if self.value.is_some(){if let Some(child)=self.child.as_ref(){if !child.terminal_is_empty(){return clone_cursor_close_demand::<T>(&**child,body)}return Ok(crate::RetirementDemand{release_bytes:size_of::<T::Cursor>(),depth:1,..Default::default()})}return Ok(crate::RetirementDemand{capacity_bytes:size_of::<T>(),depth:1,..Default::default()})}if let Some(child)=self.child.as_ref(){let mut demand=child.advance_demands(source.project(1,|boxed|boxed.as_ref()),body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"boxed original clone quote depth overflow"))?;Ok(demand)}else{Ok(crate::RetirementDemand{capacity_bytes:size_of::<T::Cursor>(),depth:1,..Default::default()})}}

    fn advance(&mut self, source: RetainedCloneRef<'_, Box<T>>, body: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained box clone cursor is closing"));
        }
        if self.spent {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained box clone cursor is spent"));
        }
        if self.output.is_some() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)=source.bind(&mut self.source,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
        if self.value.is_some() {
            if let Some(child) = self.child.as_mut() {
                if !child.terminal_is_empty() {
                    if grant.maximum_items == 0 {
                        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                    }
                    let step = child.close_step(grant)?;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained box child scaffold close")?.progress()));
                }
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                let bytes = size_of::<T::Cursor>();
                if bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.child = None;self.child_closing=false;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: bytes }));
            }
            let capacity = size_of::<T>();
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity, released_bytes: 0 };
            if !progress.fits(grant) {
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
            }
            let value = self.value.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained box child owner is missing"))?;
            self.output = Some(Box::new(value));
            return Ok(RetainedCloneStep::Complete(progress));
        }
        if self.child.is_none() {
            if grant.maximum_depth == 0 {
                return Err(crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit, "retained box clone structural depth limit exceeded"));
            }
            let capacity = size_of::<T::Cursor>();
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity, released_bytes: 0 };
            if !progress.fits(grant) {
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
            }
            self.child = Some(Box::new(T::retained_clone_cursor()));
            return Ok(RetainedCloneStep::Progress(progress));
        }
        let child = self.child.as_mut().expect("initialized retained box child");
        if grant.maximum_depth==0{return Err(crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"retained box clone structural depth limit exceeded"));}
        let child_grant = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        match child.advance(source.project(1, |boxed| boxed.as_ref()),body, child_grant)? {
            RetainedCloneStep::Progress(progress) => Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(child_grant, progress, "retained box child")?)),
            RetainedCloneStep::Complete(progress) => {
                let progress = admit_retained_clone_progress(child_grant, progress, "retained box child")?;
                if progress.copied_items >= grant.maximum_items {
                    return Ok(RetainedCloneStep::Progress(progress));
                }
                self.value = Some(child.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained box child completed without an owner"))?);
                let _ = child.begin_close();self.child_closing=true;
                Ok(RetainedCloneStep::Progress(progress.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?))
            }
        }
    }

    fn take(&mut self) -> Option<Box<T>> {
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
        if let Some(child)=self.child.as_ref(){return if !self.child_closing{Ok(crate::RetirementDemand{depth:1,..Default::default()})}else if child.terminal_is_empty(){Ok(crate::RetirementDemand{release_bytes:size_of::<T::Cursor>(),depth:1,..Default::default()})}else{child.close_demands(body)};}
        if !self.close.is_empty(){return self.close.next_owner_demands::<T>(false,&None,body);}
        if self.value.is_some(){return self.close.next_owner_demands::<T>(true,&None,body);}
        self.close.next_owner_demands::<Box<T>>(self.output.is_some(),&self.source,body)
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "native owner must begin close before granted retirement")); }
        if !retained_clone_grant_admits(grant,self.close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(child) = self.child.as_mut() {
            if !self.child_closing {child.begin_close();self.child_closing=true; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            if !child.terminal_is_empty() { let step = child.close_step(grant)?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained child close")?.progress())); }
            if grant.maximum_depth==0 { return Err(crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"boxed clone scaffold release requires depth")); }
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: size_of::<T::Cursor>() };
            if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.child = None;self.child_closing=false;
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.value, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if !self.closing {return Ok(0);} if let Some(child)=self.child.as_ref(){return if child.terminal_is_empty(){Ok(1)}else{child.next_close_depth_demand()};} self.close.next_owner_depth_with_binding(self.value.is_some()||self.output.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child) = self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_copy_byte_demand() }; }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child)=self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_capacity_byte_demand(maximum_release_bytes) }; }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if self.value.is_some() { return self.close.next_owner_capacity_with_binding::<T>(true,maximum_release_bytes,&self.source); }
        self.close.next_owner_capacity_with_binding::<Box<T>>(self.output.is_some(),maximum_release_bytes,&self.source)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child)=self.child.as_ref() { return if child.terminal_is_empty() { Ok(size_of::<T::Cursor>()) } else { child.next_close_release_byte_demand() }; }
        self.close.next_release_with_binding(&self.source)
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.child.is_none() && self.value.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none() }
}

impl<T: RetainedClone> RetainedClone for Box<T> {
    type Cursor = BoxCursor<T>;
    fn retained_clone_cursor() -> Self::Cursor {
        BoxCursor::default()
    }
}
