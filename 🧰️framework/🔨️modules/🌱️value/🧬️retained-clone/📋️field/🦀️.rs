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
        if grant.maximum_depth == 0 { return Err(crate::ValueError::new(crate::ValueRefusalKind::DepthLimit, "retained typed field structural depth limit exceeded")); }
        source.bind(&mut self.source)?;
        if let Some(child) = self.child.as_mut() { return child.advance(source, RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant }); }
        let bytes = size_of::<T::Cursor>();
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: bytes };
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
    fn terminal_is_empty(&self) -> bool { self.closing && self.child.is_none() && self.source.is_none() }
}

impl<T: RetainedClone> Drop for RetainedFieldCursor<T> {
    fn drop(&mut self) { assert!((self.child.is_none() && self.source.is_none()) || std::thread::panicking(), "retained typed field dropped before exact child closure"); }
}
