//! 📋️ Lazily admits one typed child cursor without eager recursive owner construction.

use super::*;

pub struct RetainedFieldCursor<T: RetainedClone> {
    child: Option<Box<T::Cursor>>,
    source: Option<RetainedCloneBinding>,
    closing: bool,
}

impl<T: RetainedClone> Default for RetainedFieldCursor<T> {
    fn default() -> Self { Self { child: None, source: None, closing: false } }
}

impl<T: RetainedClone> RetainedCloneCursor<T> for RetainedFieldCursor<T> {
    fn advance(&mut self, source: RetainedCloneRef<'_, T>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained field cursor is closing")); }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_depth == 0 { return Err(crate::ValueError::new(crate::ValueRefusalKind::DepthLimit, "retained typed field structural depth limit exceeded")); }
        source.bind(&mut self.source)?;
        if let Some(child) = self.child.as_mut() { return child.advance(source, RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant }); }
        let bytes = size_of::<T::Cursor>();
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: bytes, released_bytes: 0 };
        if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
        self.child = Some(Box::new(T::retained_clone_cursor()));
        Ok(RetainedCloneStep::Progress(progress))
    }
    fn take(&mut self) -> Option<T> { self.child.as_mut().and_then(|child| child.take()) }
    fn begin_close(&mut self) -> bool { if self.closing { return false; } self.closing = true; true }
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, crate::ValueError> {
        if !self.closing || maximum_items == 0 { return Ok(SnapshotRetirementStep::Blocked); }
        if let Some(child) = self.child.as_mut() {
            if child.begin_close() { return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }); }
            if !child.terminal_is_empty() {
                let step = admit_retained_clone_retirement(child.close_step(1, maximum_bytes)?, 1, maximum_bytes, "retained typed field close")?;
                if step != SnapshotRetirementStep::Complete { return Ok(step); }
                if !child.terminal_is_empty() { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained typed field reported terminal with owners")); }
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            let bytes = size_of::<T::Cursor>();
            if bytes > maximum_bytes { return Ok(SnapshotRetirementStep::Blocked); }
            self.child = None;
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes });
        }
        self.source = None;
        Ok(SnapshotRetirementStep::Complete)
    }
    fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "typed field must begin close before granted retirement")); }
        if let Some(child) = self.child.as_mut() {
            if child.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            if !child.terminal_is_empty() { let step = child.close_granted(grant)?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained child close")?.progress())); }
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: size_of::<T::Cursor>() };
            if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.child = None;
            return Ok(RetainedCloneStep::Progress(progress));
        }
        close_retained_binding(&mut self.source, grant)
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        match self.child.as_ref() {
            Some(child) if !child.terminal_is_empty() => child.next_close_copy_byte_demand(),
            _ => Ok(0),
        }
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        match self.child.as_ref() {
            Some(child) if !child.terminal_is_empty() => child.next_close_capacity_byte_demand(maximum_release_bytes),
            _ => Ok(0),
        }
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        match self.child.as_ref() {
            Some(child) if child.terminal_is_empty() => Ok(size_of::<T::Cursor>()),
            Some(child) => child.next_close_release_byte_demand(),
            None => Ok(0),
        }
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.child.is_none() && self.source.is_none() }
}

impl<T: RetainedClone> Drop for RetainedFieldCursor<T> {
    fn drop(&mut self) { assert!((self.child.is_none() && self.source.is_none()) || std::thread::panicking(), "retained typed field dropped before exact child closure"); }
}
