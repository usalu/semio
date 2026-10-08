//! ✏️ Borrowed rename preparation preserves the original inverse before its semantic disposition.

#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
#[path = "🔗️owned/🦀️.rs"]
pub mod owned;

use super::mutation::RenameLayer;
use crate::{DrawingLayerNode, DrawingSnapshot};
use crate::standards::v1::subsets::any::schema::snapshot::lookup::{DrawingLayerLookupCursor, DrawingLayerLookupStep};
use semio_framework_value::{paged::PagedUtf8, SnapshotRetirementStep, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, ordered_map::{BoundedOrdCursor, BoundedOrdGrant, BoundedOrdStep}, paged::PagedUtf8BoundedOrdCursor};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawingRenameDisposition { Missing, NoOp, Changed }

impl DrawingRenameDisposition {
    pub fn as_str(self) -> &'static str { match self { Self::Missing => "missing", Self::NoOp => "no-op", Self::Changed => "changed" } }
}

#[derive(Clone, Copy)]
pub struct DrawingRenamePlan<'a> {
    pub disposition: DrawingRenameDisposition,
    pub payload: RetainedCloneRef<'a, RenameLayer>,
    node: Option<RetainedCloneRef<'a, DrawingLayerNode>>,
}

impl<'a> DrawingRenamePlan<'a> {
    pub fn inverse_name(self) -> Option<RetainedCloneRef<'a, PagedUtf8<{usize::MAX}>>> {
        self.node.map(|node| node.project(2, |node| &crate::schema::layer_base(node).name))
    }
}

pub enum DrawingRenamePreparationStep<'a> {
    Pending(RetainedCloneProgress),
    Complete { plan: DrawingRenamePlan<'a>, progress: RetainedCloneProgress },
}

pub struct DrawingRenamePreparationCursor<'a> {
    source: Option<RetainedCloneRef<'a, DrawingSnapshot>>,
    payload: Option<RetainedCloneRef<'a, RenameLayer>>,
    lookup: Option<DrawingLayerLookupCursor<'a>>,
    node: Option<RetainedCloneRef<'a, DrawingLayerNode>>,
    comparison: PagedUtf8BoundedOrdCursor<{usize::MAX}>,
    disposition: Option<DrawingRenameDisposition>,
    output: Option<DrawingRenamePlan<'a>>,
    phase: u8,
    spent: bool,
    closing: bool,
}

impl<'a> DrawingRenamePreparationCursor<'a> {
    pub fn new(source: RetainedCloneRef<'a, DrawingSnapshot>, payload: RetainedCloneRef<'a, RenameLayer>) -> Self {
        Self { source: Some(source), payload: Some(payload), lookup: Some(DrawingLayerLookupCursor::new(source, payload.project(1, |payload| &payload.layer_id))), node: None, comparison: Default::default(), disposition: None, output: None, phase: 0, spent: false, closing: false }
    }

    pub fn advance(&mut self, grant: RetainedCloneGrant) -> Result<DrawingRenamePreparationStep<'a>, ValueError> {
        if self.closing || self.spent { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "Drawing rename preparation is closing or spent")); }
        if grant.maximum_items == 0 { return Ok(DrawingRenamePreparationStep::Pending(Default::default())); }
        if let Some(plan) = self.output { return Ok(DrawingRenamePreparationStep::Complete { plan, progress: Default::default() }); }
        if self.phase == 0 {
            let lookup = self.lookup.as_mut().expect("retained Drawing rename lookup");
            return match lookup.advance(grant)? {
                DrawingLayerLookupStep::Pending(progress) => Ok(DrawingRenamePreparationStep::Pending(progress)),
                DrawingLayerLookupStep::Complete { progress, .. } => {
                    self.node = lookup.take().expect("completed Drawing rename lookup");
                    lookup.begin_close();
                    self.phase = 1;
                    Ok(DrawingRenamePreparationStep::Pending(progress))
                }
            };
        }
        if self.phase == 1 {
            let lookup = self.lookup.as_mut().expect("closing Drawing rename lookup");
            let step = lookup.close_granted(grant)?;
            if matches!(step, RetainedCloneStep::Complete(_)) {
                self.lookup.take();
                if self.node.is_none() { self.disposition = Some(DrawingRenameDisposition::Missing); self.comparison.begin_close(); self.phase = 3; } else { self.phase = 2; }
            }
            return Ok(DrawingRenamePreparationStep::Pending(step.progress()));
        }
        if self.phase == 2 {
            let old_name = self.node.expect("found Drawing rename layer").project(2, |node| &crate::schema::layer_base(node).name);
            let new_name = self.payload.expect("retained Drawing rename payload").project(2, |payload| &payload.new_name);
            let step = self.comparison.compare(old_name, new_name, BoundedOrdGrant { maximum_items: 1, maximum_bytes: grant.maximum_copy_bytes })?;
            let (ordering, progress) = match step { BoundedOrdStep::Progress(progress) => (None, progress), BoundedOrdStep::Complete { ordering, progress } => (Some(ordering), progress) };
            if let Some(ordering) = ordering {
                self.disposition = Some(if ordering == std::cmp::Ordering::Equal { DrawingRenameDisposition::NoOp } else { DrawingRenameDisposition::Changed });
                self.comparison.begin_close();
                self.phase = 3;
            }
            return Ok(DrawingRenamePreparationStep::Pending(RetainedCloneProgress { copied_items: progress.compared_items, copied_bytes: progress.compared_bytes, ..Default::default() }));
        }
        if self.phase == 3 {
            let step = self.comparison.close_step(1, grant.maximum_release_bytes)?;
            if step == SnapshotRetirementStep::Complete { self.phase = 4; }
            return Ok(DrawingRenamePreparationStep::Pending(close_progress(step)));
        }
        if grant.maximum_copy_bytes < std::mem::size_of::<DrawingRenamePlan<'a>>() { return Ok(DrawingRenamePreparationStep::Pending(Default::default())); }
        let plan = DrawingRenamePlan { disposition: self.disposition.expect("settled Drawing rename disposition"), payload: self.payload.expect("retained Drawing rename payload"), node: self.node };
        self.output = Some(plan);
        Ok(DrawingRenamePreparationStep::Complete { plan, progress: RetainedCloneProgress { copied_items: 1, copied_bytes: std::mem::size_of::<DrawingRenamePlan<'a>>(), ..Default::default() } })
    }

    pub fn take(&mut self) -> Option<DrawingRenamePlan<'a>> {
        if self.closing { return None; }
        let output = self.output.take();
        if output.is_some() { self.spent = true; }
        output
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
        if let Some(lookup) = &mut self.lookup { lookup.begin_close(); }
        self.comparison.begin_close();
    }

    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "Drawing rename preparation closure was not begun")); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(lookup) = &mut self.lookup {
            let step = lookup.close_granted(grant)?;
            if matches!(step, RetainedCloneStep::Complete(_)) { self.lookup.take(); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        let step = self.comparison.close_step(1, grant.maximum_release_bytes)?;
        if step != SnapshotRetirementStep::Complete { return Ok(RetainedCloneStep::Progress(close_progress(step))); }
        if self.output.take().is_some() || self.node.take().is_some() || self.source.take().is_some() || self.payload.take().is_some() {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.lookup.is_none() && self.comparison.terminal_is_empty() && self.source.is_none() && self.payload.is_none() && self.node.is_none() && self.output.is_none() }
}

fn close_progress(step: SnapshotRetirementStep) -> RetainedCloneProgress {
    match step { SnapshotRetirementStep::Pending { released_items, released_bytes } => RetainedCloneProgress { copied_items: released_items, released_bytes, ..Default::default() }, SnapshotRetirementStep::Complete => RetainedCloneProgress { copied_items: 1, ..Default::default() }, SnapshotRetirementStep::Blocked => Default::default() }
}

impl Drop for DrawingRenamePreparationCursor<'_> {
    fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "Drawing rename preparation reached Drop before granted closure"); }
}
