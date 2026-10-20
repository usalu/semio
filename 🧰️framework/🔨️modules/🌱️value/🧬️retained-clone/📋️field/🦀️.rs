//! 📋️ Lazily admits one typed child cursor without eager recursive owner construction.

use super::*;

pub struct RetainedFieldCursor<T: RetainedClone> {
    child: Option<Box<T::Cursor>>,
    source: Option<RetainedCloneBinding>,
    closing: bool,
    child_closing: bool,
}

impl<T: RetainedClone> Default for RetainedFieldCursor<T> {
    fn default() -> Self { Self { child: None, source: None, closing: false, child_closing:false } }
}

impl<T: RetainedClone> RetainedCloneCursor<T> for RetainedFieldCursor<T> {
    fn output_ready(&self)->bool{!self.closing&&self.child.as_ref().is_some_and(|child|child.output_ready())}
    fn advance_demands(&self,source:RetainedCloneRef<'_,T>,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"typed field original clone is closing"))}if let Some(demand)=source.binding_demand(&self.source)?{return Ok(demand)}if let Some(child)=self.child.as_ref(){let mut demand=child.advance_demands(source,body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"typed field original quote depth overflow"))?;Ok(demand)}else{Ok(crate::RetirementDemand{capacity_bytes:size_of::<T::Cursor>(),depth:1,..Default::default()})}}

    fn advance(&mut self, source: RetainedCloneRef<'_, T>, body: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained field cursor is closing")); }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_depth == 0 { return Err(crate::ValueError::new(crate::ValueRefusalKind::DepthLimit, "retained typed field structural depth limit exceeded")); }
        if let Some(progress)=source.bind(&mut self.source,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
        if let Some(child) = self.child.as_mut() { return child.advance(source,body, RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant }); }
        let bytes = size_of::<T::Cursor>();
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: bytes, released_bytes: 0 };
        if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
        self.child = Some(Box::new(T::retained_clone_cursor()));
        Ok(RetainedCloneStep::Progress(progress))
    }
    fn take(&mut self) -> Option<T> { if !self.output_ready(){return None;}self.child.as_mut().and_then(|child| child.take()) }
    fn begin_close(&mut self) -> bool { if self.closing { return false; } self.closing = true; true }
    
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError> {
        if !self.closing { return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"typed field must begin close before granted retirement")); }
        if !retained_clone_grant_admits(grant,retained_clone_close_demands::<T>(self,grant.maximum_copy_bytes)?) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(child)=self.child.as_mut() {
            if !self.child_closing {child.begin_close();self.child_closing=true; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()})); }
            if !child.terminal_is_empty() { let nested=nested_retained_clone_grant(grant)?;let step=child.close_step(nested)?;return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(nested,step,child.terminal_is_empty(),"field original child close")?.progress())); }
            self.child=None;self.child_closing=false;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:size_of::<T::Cursor>(),..Default::default()}));
        }
        close_retained_binding(&mut self.source,grant)
    }
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError> {
        if !self.closing{return Ok(Default::default());}
        if let Some(child)=self.child.as_ref(){if !self.child_closing{return Ok(crate::RetirementDemand{depth:1,..Default::default()});}return if child.terminal_is_empty(){Ok(crate::RetirementDemand{release_bytes:size_of::<T::Cursor>(),depth:1,..Default::default()})}else{nested_retained_clone_close_demands::<T>(child.as_ref(),body)};}
        RetainedCloneBinding::close_demand(&self.source,body)
    }
    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{Ok(self.close_demands(0)?.depth)}
    fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    fn terminal_is_empty(&self) -> bool { self.closing && self.child.is_none() && self.source.is_none() }
}

impl<T: RetainedClone> Drop for RetainedFieldCursor<T> {
    fn drop(&mut self) { assert!((self.child.is_none() && self.source.is_none()) || std::thread::panicking(), "retained typed field dropped before exact child closure"); }
}
