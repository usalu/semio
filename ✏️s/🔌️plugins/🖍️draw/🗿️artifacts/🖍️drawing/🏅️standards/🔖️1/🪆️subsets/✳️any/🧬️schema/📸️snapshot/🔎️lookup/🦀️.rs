//! 🔎️ Borrowed native tree lookup admits each traversal page and UTF8 comparison separately.

#[path = "🔗️owned/🦀️.rs"]
pub mod owned;

use crate::{DrawingLayerNode, DrawingSnapshot};
use semio_framework_value::{list::PagedList, paged::PagedUtf8, SnapshotRetirementStep, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, paged::PagedUtf8BoundedOrdCursor, ordered_map::{BoundedOrdCursor, BoundedOrdGrant, BoundedOrdStep}};
use std::mem::size_of;

type Layers = PagedList<DrawingLayerNode, {usize::MAX}>;

struct Frame<'a> { layers: RetainedCloneRef<'a, Layers>, index: usize }

pub enum DrawingLayerLookupStep<'a> {
    Pending(RetainedCloneProgress),
    Complete { node: Option<RetainedCloneRef<'a, DrawingLayerNode>>, progress: RetainedCloneProgress },
}

pub struct DrawingLayerLookupCursor<'a> {
    source: Option<RetainedCloneRef<'a, DrawingSnapshot>>,
    target: Option<RetainedCloneRef<'a, PagedUtf8<{usize::MAX}>>>,
    frames: PagedList<Frame<'a>, {usize::MAX}>,
    pending: Option<Frame<'a>>,
    current: Option<RetainedCloneRef<'a, DrawingLayerNode>>,
    comparison: PagedUtf8BoundedOrdCursor<{usize::MAX}>,
    output: Option<Option<RetainedCloneRef<'a, DrawingLayerNode>>>,
    phase: u8,
    spent: bool,
    closing: bool,
}

impl<'a> DrawingLayerLookupCursor<'a> {
    pub fn new(source: RetainedCloneRef<'a, DrawingSnapshot>, target: RetainedCloneRef<'a, PagedUtf8<{usize::MAX}>>) -> Self {
        Self { source: Some(source), target: Some(target), frames: Default::default(), pending: Some(Frame { layers: source.project(1, |snapshot| &snapshot.layers), index: 0 }), current: None, comparison: Default::default(), output: None, phase: 0, spent: false, closing: false }
    }

    pub fn advance(&mut self, grant: RetainedCloneGrant) -> Result<DrawingLayerLookupStep<'a>, ValueError> {
        if self.closing || self.spent { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "Drawing lookup is closing or spent")); }
        if grant.maximum_items == 0 { return Ok(DrawingLayerLookupStep::Pending(Default::default())); }
        if let Some(node) = self.output { return Ok(DrawingLayerLookupStep::Complete { node, progress: Default::default() }); }
        if self.pending.is_some() {
            if self.frames.len() == self.frames.capacity() {
                let progress = self.frames.reserve_one(grant.maximum_capacity_bytes).map_err(|error| ValueError::from(error.refusal()))?;
                return Ok(DrawingLayerLookupStep::Pending(RetainedCloneProgress { copied_items: usize::from(progress.progressed), retained_capacity_bytes: progress.allocated_bytes, ..Default::default() }));
            }
            if grant.maximum_copy_bytes < size_of::<Frame<'a>>() { return Ok(DrawingLayerLookupStep::Pending(Default::default())); }
            self.frames.push_reserved(self.pending.take().expect("checked pending frame")).map_err(|_| ValueError::new(ValueRefusalKind::InvariantViolated, "Drawing lookup frame placement refused"))?;
            return Ok(DrawingLayerLookupStep::Pending(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Frame<'a>>(), ..Default::default() }));
        }
        if self.phase == 1 {
            let step = self.comparison.close_step(1, grant.maximum_release_bytes)?;
            if step == SnapshotRetirementStep::Complete { self.comparison = Default::default(); self.phase = 2; }
            return Ok(DrawingLayerLookupStep::Pending(close_progress(step)));
        }
        if self.phase == 2 {
            if grant.maximum_copy_bytes < size_of::<Frame<'a>>() { return Ok(DrawingLayerLookupStep::Pending(Default::default())); }
            let current = self.current.take().expect("compared Drawing lookup candidate");
            let last = self.frames.len() - 1;
            self.frames[last].index += 1;
            if matches!(current.get(), DrawingLayerNode::Group(_)) {
                self.pending = Some(Frame { layers: current.project(1, |layer| match layer { DrawingLayerNode::Group(group) => &group.children, _ => unreachable!() }), index: 0 });
            }
            self.phase = 0;
            return Ok(DrawingLayerLookupStep::Pending(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Frame<'a>>(), ..Default::default() }));
        }
        if self.current.is_none() {
            let Some(last) = self.frames.len().checked_sub(1) else { self.output = Some(None); return Ok(DrawingLayerLookupStep::Complete { node: None, progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } }); };
            let frame = &self.frames[last];
            if frame.index == frame.layers.get().len() {
                if grant.maximum_copy_bytes < size_of::<Frame<'a>>() { return Ok(DrawingLayerLookupStep::Pending(Default::default())); }
                self.frames.pop();
                return Ok(DrawingLayerLookupStep::Pending(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Frame<'a>>(), ..Default::default() }));
            }
            if grant.maximum_copy_bytes < size_of::<RetainedCloneRef<'a, DrawingLayerNode>>() { return Ok(DrawingLayerLookupStep::Pending(Default::default())); }
            let index = frame.index;
            self.current = Some(frame.layers.project(index, |layers| &layers[index]));
            return Ok(DrawingLayerLookupStep::Pending(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<RetainedCloneRef<'a, DrawingLayerNode>>(), ..Default::default() }));
        }
        let current = self.current.expect("selected Drawing lookup candidate");
        let step = self.comparison.compare(current.project(1, crate::schema::layer_id), self.target.expect("retained Drawing lookup target"), BoundedOrdGrant { maximum_items: 1, maximum_bytes: grant.maximum_copy_bytes })?;
        let (ordering, progress) = match step { BoundedOrdStep::Progress(progress) => (None, progress), BoundedOrdStep::Complete { ordering, progress } => (Some(ordering), progress) };
        let progress = RetainedCloneProgress { copied_items: progress.compared_items, copied_bytes: progress.compared_bytes, ..Default::default() };
        if ordering == Some(std::cmp::Ordering::Equal) {
            self.output = Some(Some(current));
            return Ok(DrawingLayerLookupStep::Complete { node: Some(current), progress });
        }
        if ordering.is_some() { self.comparison.begin_close(); self.phase = 1; }
        Ok(DrawingLayerLookupStep::Pending(progress))
    }

    /// 🧭️ Reads the retained first owner's numeric path before result handoff or closure.
    pub fn path_len(&self) -> Option<usize> {
        if self.closing || !matches!(self.output, Some(Some(_))) { return None; }
        Some(self.frames.len())
    }

    /// 📍️ Copies one ordinal from the retained native traversal frames without path reconstruction.
    pub fn path_index(&self, index: usize) -> Option<usize> {
        let length = self.path_len()?;
        let frame = self.frames.get(index)?;
        if index + 1 == length { Some(frame.index) } else { frame.index.checked_sub(1) }
    }

    pub fn take(&mut self) -> Option<Option<RetainedCloneRef<'a, DrawingLayerNode>>> {
        if self.closing { return None; }
        let output = self.output.take();
        if output.is_some() { self.spent = true; }
        output
    }

    pub fn begin_close(&mut self) { self.closing = true; self.comparison.begin_close(); }

    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "Drawing lookup closure was not started")); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let step = self.comparison.close_step(1, grant.maximum_release_bytes)?;
        if step != SnapshotRetirementStep::Complete { return Ok(RetainedCloneStep::Progress(close_progress(step))); }
        if self.output.take().is_some() || self.current.take().is_some() || self.pending.take().is_some() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
        if !self.frames.is_empty() {
            if grant.maximum_copy_bytes < size_of::<Frame<'a>>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.frames.pop();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Frame<'a>>(), ..Default::default() }));
        }
        if !self.frames.terminal_is_empty() {
            let progress = self.frames.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(progress.progressed), copied_bytes: 0, released_bytes: progress.released_allocation_bytes, ..Default::default() }));
        }
        if self.source.take().is_some() || self.target.take().is_some() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.source.is_none() && self.target.is_none() && self.frames.terminal_is_empty() && self.pending.is_none() && self.current.is_none() && self.output.is_none() && self.comparison.terminal_is_empty() }
}

fn close_progress(step: SnapshotRetirementStep) -> RetainedCloneProgress {
    match step { SnapshotRetirementStep::Pending { released_items, released_bytes } => RetainedCloneProgress { copied_items: released_items, released_bytes, ..Default::default() }, SnapshotRetirementStep::Complete => RetainedCloneProgress { copied_items: 1, ..Default::default() }, SnapshotRetirementStep::Blocked => Default::default() }
}

impl Drop for DrawingLayerLookupCursor<'_> {
    fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "Drawing lookup reached Drop before granted closure"); }
}
