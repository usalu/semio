//! 🔗️ Native recursive lookup owns root-backed projection frames and reborrows targets per turn.

use crate::{DrawingLayerNode, DrawingSnapshot};
use semio_framework_value::{list::PagedList, paged::PagedUtf8, SnapshotRetirementStep, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneBinding, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, RetainedOwnedProjection, paged::PagedUtf8BoundedOrdCursor, ordered_map::{BoundedOrdCursor, BoundedOrdGrant, BoundedOrdStep}};
use std::mem::size_of;

type Layers = PagedList<DrawingLayerNode, {usize::MAX}>;
type Node = RetainedOwnedProjection<DrawingLayerNode>;
struct Frame { layers: RetainedOwnedProjection<Layers>, index: usize }

pub enum DrawingOwnedLayerLookupStep {
    Pending(RetainedCloneProgress),
    Complete { found: bool, progress: RetainedCloneProgress },
}

pub struct DrawingOwnedLayerLookupCursor {
    source: Option<RetainedOwnedProjection<DrawingSnapshot>>,
    target: Option<RetainedCloneBinding>,
    frames: PagedList<Frame, {usize::MAX}>,
    pending: Option<Frame>,
    current: Option<Node>,
    comparison: PagedUtf8BoundedOrdCursor<{usize::MAX}>,
    output: Option<Option<Node>>,
    phase: u8,
    spent: bool,
    closing: bool,
}

impl DrawingOwnedLayerLookupCursor {
    /// 🌳️ Retains native root ownership without a copied snapshot or heap allocation.
    pub fn new(source: RetainedOwnedProjection<DrawingSnapshot>) -> Result<Self, ValueError> {
        let pending = Some(Frame { layers: source.project(1, |snapshot| &snapshot.layers)?, index: 0 });
        Ok(Self { source: Some(source), target: None, frames: Default::default(), pending, current: None, comparison: Default::default(), output: None, phase: 0, spent: false, closing: false })
    }

    /// 🔁️ Advances one native page, ordinal, alias, or bounded UTF8 comparison.
    pub fn advance(&mut self, target: RetainedCloneRef<'_, PagedUtf8<{usize::MAX}>>, grant: RetainedCloneGrant) -> Result<DrawingOwnedLayerLookupStep, ValueError> {
        if self.closing || self.spent { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "owned Drawing lookup is closing or spent")); }
        if grant.maximum_items == 0 { return Ok(DrawingOwnedLayerLookupStep::Pending(Default::default())); }
        let first_target = self.target.is_none();
        target.bind(&mut self.target)?;
        if first_target { return Ok(pending_unit(0)); }
        if let Some(node) = &self.output { return Ok(DrawingOwnedLayerLookupStep::Complete { found: node.is_some(), progress: Default::default() }); }
        if self.phase == 4 {
            if grant.maximum_copy_bytes < size_of::<Node>() { return Ok(pending_unit_inert()); }
            self.output = Some(self.current.take());
            return Ok(DrawingOwnedLayerLookupStep::Complete { found: true, progress: RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Node>(), ..Default::default() } });
        }
        if self.phase == 1 {
            let step = self.comparison.close_step(1, grant.maximum_release_bytes)?;
            if step == SnapshotRetirementStep::Complete { self.comparison = Default::default(); self.phase = 2; }
            return Ok(DrawingOwnedLayerLookupStep::Pending(close_progress(step)));
        }
        if self.phase == 2 {
            if grant.maximum_copy_bytes < size_of::<Frame>() { return Ok(pending_unit_inert()); }
            let current = self.current.as_ref().expect("compared native Drawing node");
            self.frames.last_mut().expect("retained current frame").index += 1;
            if matches!(current.borrow()?.get(), DrawingLayerNode::Group(_)) {
                self.pending = Some(Frame { layers: current.project(1, |layer| match layer { DrawingLayerNode::Group(group) => &group.children, _ => unreachable!() })?, index: 0 });
            }
            self.phase = 3;
            return Ok(pending_unit(size_of::<Frame>()));
        }
        if self.phase == 3 {
            let step = self.current.as_mut().expect("owned compared node").close_step(1)?;
            if step == SnapshotRetirementStep::Complete { self.current = None; self.phase = 0; }
            return Ok(DrawingOwnedLayerLookupStep::Pending(close_progress(step)));
        }
        if self.phase == 5 {
            if grant.maximum_copy_bytes < size_of::<Frame>() { return Ok(pending_unit_inert()); }
            self.frames.pop();
            self.phase = 0;
            return Ok(pending_unit(size_of::<Frame>()));
        }
        if let Some(frame) = &mut self.pending {
            if !self.frames.has_reserved_slot() {
                let progress = self.frames.reserve_one(grant.maximum_capacity_bytes).map_err(|error| ValueError::from(error.refusal()))?;
                return Ok(DrawingOwnedLayerLookupStep::Pending(RetainedCloneProgress { copied_items: usize::from(progress.progressed), retained_capacity_bytes: progress.allocated_bytes, ..Default::default() }));
            }
            if grant.maximum_copy_bytes < size_of::<Frame>() { return Ok(pending_unit_inert()); }
            assert!(!frame.layers.terminal_is_empty());
            self.frames.push_reserved(self.pending.take().expect("pending native frame")).map_err(|_| ValueError::new(ValueRefusalKind::InvariantViolated, "owned Drawing frame placement refused"))?;
            return Ok(pending_unit(size_of::<Frame>()));
        }
        if self.current.is_none() {
            let Some(frame) = self.frames.last_mut() else {
                self.output = Some(None);
                return Ok(DrawingOwnedLayerLookupStep::Complete { found: false, progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } });
            };
            if frame.index == frame.layers.borrow()?.get().len() {
                let step = frame.layers.close_step(1)?;
                if frame.layers.terminal_is_empty() { self.phase = 5; }
                return Ok(DrawingOwnedLayerLookupStep::Pending(close_progress(step)));
            }
            if grant.maximum_copy_bytes < size_of::<Node>() { return Ok(pending_unit_inert()); }
            let index = frame.index;
            self.current = Some(frame.layers.project(index, |layers| &layers[index])?);
            return Ok(pending_unit(size_of::<Node>()));
        }
        let current = self.current.as_ref().expect("selected native Drawing node");
        let step = self.comparison.compare(current.borrow()?.project(1, crate::schema::layer_id), target, BoundedOrdGrant { maximum_items: 1, maximum_bytes: grant.maximum_copy_bytes })?;
        let (ordering, progress) = match step { BoundedOrdStep::Progress(progress) => (None, progress), BoundedOrdStep::Complete { ordering, progress } => (Some(ordering), progress) };
        if ordering == Some(std::cmp::Ordering::Equal) { self.phase = 4; }
        else if ordering.is_some() { self.comparison.begin_close(); self.phase = 1; }
        Ok(DrawingOwnedLayerLookupStep::Pending(RetainedCloneProgress { copied_items: progress.compared_items, copied_bytes: progress.compared_bytes, ..Default::default() }))
    }

    /// 🧭️ Reads one retained first-owner ordinal before handoff or closure.
    pub fn path_index(&self, index: usize) -> Option<usize> {
        let length = self.path_len()?;
        let frame = self.frames.get(index)?;
        if index + 1 == length { Some(frame.index) } else { frame.index.checked_sub(1) }
    }

    /// 📏️ Reads actual traversal depth without rebuilding a path owner.
    pub fn path_len(&self) -> Option<usize> { if self.closing || !matches!(&self.output, Some(Some(_))) { None } else { Some(self.frames.len()) } }

    /// 🫴️ Transfers one actual native projection while the external root source remains retained.
    pub fn take(&mut self) -> Option<Option<Node>> {
        if self.closing { return None; }
        let output = self.output.take();
        if output.is_some() { self.spent = true; }
        output
    }

    /// 🛑️ Starts explicit child alias and traversal-page retirement.
    pub fn begin_close(&mut self) { self.closing = true; self.comparison.begin_close(); }

    /// ♻️ Closes one alias, frame placement or actual empty page per granted turn.
    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "owned Drawing lookup closure was not started")); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let step = self.comparison.close_step(1, grant.maximum_release_bytes)?;
        if step != SnapshotRetirementStep::Complete { return Ok(RetainedCloneStep::Progress(close_progress(step))); }
        if let Some(Some(node)) = &mut self.output {
            let step = node.close_step(1)?;
            if step == SnapshotRetirementStep::Complete { self.output = None; }
            return Ok(RetainedCloneStep::Progress(close_progress(step)));
        }
        if self.output.take().is_some() { return Ok(close_unit(0)); }
        if let Some(node) = &mut self.current {
            let step = node.close_step(1)?;
            if step == SnapshotRetirementStep::Complete { self.current = None; }
            return Ok(RetainedCloneStep::Progress(close_progress(step)));
        }
        if let Some(frame) = &mut self.pending {
            let step = frame.layers.close_step(1)?;
            if step == SnapshotRetirementStep::Complete { self.pending = None; }
            return Ok(RetainedCloneStep::Progress(close_progress(step)));
        }
        if let Some(frame) = self.frames.last_mut() {
            if !frame.layers.terminal_is_empty() { return Ok(RetainedCloneStep::Progress(close_progress(frame.layers.close_step(1)?))); }
            if grant.maximum_copy_bytes < size_of::<Frame>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.frames.pop();
            return Ok(close_unit(size_of::<Frame>()));
        }
        if !self.frames.terminal_is_empty() {
            let progress = self.frames.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(progress.progressed), copied_bytes: 0, released_bytes: progress.released_allocation_bytes, ..Default::default() }));
        }
        let step = RetainedCloneBinding::close_one(&mut self.target, 1)?;
        if step != SnapshotRetirementStep::Complete { return Ok(RetainedCloneStep::Progress(close_progress(step))); }
        if let Some(source) = &mut self.source {
            let step = source.close_step(1)?;
            if step == SnapshotRetirementStep::Complete { self.source = None; }
            return Ok(RetainedCloneStep::Progress(close_progress(step)));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }

    /// ✅️ Requires every actual alias and native page to have detached before Drop.
    pub fn terminal_is_empty(&self) -> bool { self.closing && self.source.is_none() && self.target.is_none() && self.frames.terminal_is_empty() && self.pending.is_none() && self.current.is_none() && self.output.is_none() && self.comparison.terminal_is_empty() }
}

fn pending_unit(bytes: usize) -> DrawingOwnedLayerLookupStep { DrawingOwnedLayerLookupStep::Pending(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }) }
fn pending_unit_inert() -> DrawingOwnedLayerLookupStep { DrawingOwnedLayerLookupStep::Pending(Default::default()) }
fn close_unit(bytes: usize) -> RetainedCloneStep { RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }) }
fn close_progress(step: SnapshotRetirementStep) -> RetainedCloneProgress { match step { SnapshotRetirementStep::Pending { released_items, released_bytes } => RetainedCloneProgress { copied_items: released_items, released_bytes, ..Default::default() }, SnapshotRetirementStep::Complete => RetainedCloneProgress { copied_items: 1, ..Default::default() }, SnapshotRetirementStep::Blocked => Default::default() } }

impl Drop for DrawingOwnedLayerLookupCursor {
    fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "owned Drawing lookup reached Drop before actual alias and page closure"); }
}
