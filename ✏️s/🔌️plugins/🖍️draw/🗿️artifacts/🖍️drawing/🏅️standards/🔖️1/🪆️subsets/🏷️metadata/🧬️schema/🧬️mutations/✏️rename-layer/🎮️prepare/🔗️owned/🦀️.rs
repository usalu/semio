//! 🔗️ Static rename preparation retains the native root and reborrows original payloads each turn.

use super::DrawingRenameDisposition;
use super::super::mutation::RenameLayer;
use crate::{DrawingLayerNode, DrawingSnapshot};
use crate::standards::v1::subsets::any::schema::snapshot::lookup::owned::{DrawingOwnedLayerLookupCursor, DrawingOwnedLayerLookupStep};
use semio_framework_value::{list::PagedList, paged::PagedUtf8, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneBinding, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, RetainedOwnedProjection, paged::PagedUtf8BoundedOrdCursor, ordered_map::{BoundedOrdCursor, BoundedOrdGrant, BoundedOrdStep}};

/// 🧾️ Keeps the actual original old name through the native immutable layer projection.
pub struct DrawingOwnedRenamePlan {
    pub disposition: DrawingRenameDisposition,
    node: Option<RetainedOwnedProjection<DrawingLayerNode>>,
    payload: Option<RetainedCloneBinding>,
    path: PagedList<usize, {usize::MAX}>,
    closing: bool,
}

impl DrawingOwnedRenamePlan {
    /// 🧷️ Checks that later inverse or candidate assembly reborrows the exact original payload.
    pub fn check_original(&mut self, payload: RetainedCloneRef<'_, RenameLayer>) -> Result<(), ValueError> {
        if self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "owned Drawing rename plan is closing")); }
        if payload.bind(&mut self.payload, RetainedCloneGrant::default())?.is_some() { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "owned Drawing rename plan lost its original payload binding")); }
        Ok(())
    }

    /// ↩️ Borrows the original old name without copying native text or retaining an original payload.
    pub fn inverse_name(&self) -> Result<Option<RetainedCloneRef<'_, PagedUtf8<{usize::MAX}>>>, ValueError> {
        if self.closing { return Ok(None); }
        self.node.as_ref().map(|node| node.borrow().map(|node| node.project(2, |node| &crate::schema::layer_base(node).name))).transpose()
    }

    /// 🧭️ Reads one actual saved ordinal without reconstructing the root path.
    pub fn path_index(&self, index: usize) -> Option<usize> { self.path_len()?; self.path.get(index).copied() }

    /// 📏️ Reads the native saved depth while the semantic plan remains available.
    pub fn path_len(&self) -> Option<usize> { if self.closing || self.disposition == DrawingRenameDisposition::Missing { None } else { Some(self.path.len()) } }

    /// 🛑️ Begins explicit retirement of the original-name alias and saved native ordinal pages.
    pub fn begin_close(&mut self) { self.closing = true; }

    /// ♻️ Releases one actual alias, saved ordinal or native empty page per admitted turn.
    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "owned Drawing rename plan closure was not started")); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(node) = &mut self.node {
            let step = node.close_step(grant)?;
            if node.terminal_is_empty() { self.node = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if !self.path.terminal_is_empty() { return close_path(&mut self.path, grant); }
        if self.payload.is_some() {
            let step = RetainedCloneBinding::close_one(&mut self.payload, grant)?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }

    /// ✅️ Requires the native old-name projection to be explicitly closed before Drop.
    pub fn terminal_is_empty(&self) -> bool { self.closing && self.node.is_none() && self.payload.is_none() && self.path.terminal_is_empty() }
}

impl Drop for DrawingOwnedRenamePlan {
    fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "owned Drawing rename plan reached Drop before exact alias and ordinal-page closure"); }
}

pub enum DrawingOwnedRenamePreparationStep {
    Pending(RetainedCloneProgress),
    Complete { disposition: DrawingRenameDisposition, progress: RetainedCloneProgress },
}

pub struct DrawingOwnedRenamePreparationCursor {
    payload: Option<RetainedCloneBinding>,
    lookup: Option<DrawingOwnedLayerLookupCursor>,
    node: Option<RetainedOwnedProjection<DrawingLayerNode>>,
    path: PagedList<usize, {usize::MAX}>,
    comparison: PagedUtf8BoundedOrdCursor<{usize::MAX}>,
    disposition: Option<DrawingRenameDisposition>,
    output: Option<DrawingOwnedRenamePlan>,
    phase: u8,
    spent: bool,
    closing: bool,
}

impl DrawingOwnedRenamePreparationCursor {
    /// 🌳️ Takes only an actual native root projection and allocates no cursor scaffold.
    pub fn new(source: RetainedOwnedProjection<DrawingSnapshot>) -> Self {
        Self { payload: None, lookup: Some(DrawingOwnedLayerLookupCursor::new(source)), node: None, path: Default::default(), comparison: Default::default(), disposition: None, output: None, phase: 0, spent: false, closing: false }
    }

    /// 🔁️ Checks the original payload lease and advances one bounded semantic or ownership unit.
    pub fn advance(&mut self, payload: RetainedCloneRef<'_, RenameLayer>, grant: RetainedCloneGrant) -> Result<DrawingOwnedRenamePreparationStep, ValueError> {
        if self.closing || self.spent { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "owned Drawing rename preparation is closing or spent")); }
        if grant.maximum_items == 0 { return Ok(pending(Default::default())); }
        if let Some(plan) = &mut self.output {
            plan.check_original(payload)?;
            return Ok(DrawingOwnedRenamePreparationStep::Complete { disposition: plan.disposition, progress: Default::default() });
        }
        if let Some(progress) = payload.bind(&mut self.payload, grant)? { return Ok(pending(progress)); }
        if self.phase == 0 {
            let lookup = self.lookup.as_mut().expect("owned native Drawing rename lookup");
            return match lookup.advance(payload.project(1, |payload| &payload.layer_id), grant)? {
                DrawingOwnedLayerLookupStep::Pending(progress) => Ok(pending(progress)),
                DrawingOwnedLayerLookupStep::Complete { progress, .. } => {
                    self.phase = 5;
                    Ok(pending(progress))
                }
            };
        }
        if self.phase == 5 {
            let lookup = self.lookup.as_mut().expect("retained original Drawing ordinal path");
            if lookup.path_len().is_some_and(|length| self.path.len() < length) {
                if !self.path.has_reserved_slot() {
                    let progress = self.path.reserve_one(grant.maximum_capacity_bytes).map_err(|error| ValueError::from(error.refusal()))?;
                    return Ok(pending(RetainedCloneProgress { copied_items: usize::from(progress.progressed), retained_capacity_bytes: progress.allocated_bytes, ..Default::default() }));
                }
                if grant.maximum_copy_bytes < std::mem::size_of::<usize>() { return Ok(pending(Default::default())); }
                let index = lookup.path_index(self.path.len()).expect("actual retained Drawing ordinal");
                self.path.push_reserved(index).map_err(|_| ValueError::new(ValueRefusalKind::InvariantViolated, "Drawing ordinal placement refused"))?;
                return Ok(pending(RetainedCloneProgress { copied_items: 1, copied_bytes: std::mem::size_of::<usize>(), ..Default::default() }));
            }
            let bytes = std::mem::size_of::<Option<RetainedOwnedProjection<DrawingLayerNode>>>();
            if grant.maximum_copy_bytes < bytes { return Ok(pending(Default::default())); }
            self.node = lookup.take().expect("completed owned Drawing lookup");
            lookup.begin_close();
            self.phase = 1;
            return Ok(pending(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }));
        }
        if self.phase == 1 {
            let lookup = self.lookup.as_mut().expect("closing owned Drawing rename lookup");
            let step = lookup.close_granted(grant)?;
            if matches!(step, RetainedCloneStep::Complete(_)) {
                self.lookup = None;
                if self.node.is_none() { self.disposition = Some(DrawingRenameDisposition::Missing); self.comparison.begin_close(); self.phase = 3; }
                else { self.phase = 2; }
            }
            return Ok(pending(step.progress()));
        }
        if self.phase == 2 {
            let old_name = self.node.as_ref().expect("owned found Drawing rename layer").borrow()?.project(2, |node| &crate::schema::layer_base(node).name);
            let new_name = payload.project(2, |payload| &payload.new_name);
            let step = self.comparison.compare(old_name, new_name, BoundedOrdGrant { maximum_items: 1, maximum_bytes: grant.maximum_copy_bytes }, grant)?;
            let (ordering, progress) = match step {
                BoundedOrdStep::Authority(progress) => return Ok(pending(progress)),
                BoundedOrdStep::Progress(progress) => (None, progress),
                BoundedOrdStep::Complete { ordering, progress } => (Some(ordering), progress),
            };
            if let Some(ordering) = ordering {
                self.disposition = Some(if ordering == std::cmp::Ordering::Equal { DrawingRenameDisposition::NoOp } else { DrawingRenameDisposition::Changed });
                self.comparison.begin_close();
                self.phase = 3;
            }
            return Ok(pending(RetainedCloneProgress { copied_items: progress.compared_items, copied_bytes: progress.compared_bytes, ..Default::default() }));
        }
        if self.phase == 3 {
            let step = self.comparison.close_step(grant)?;
            if self.comparison.terminal_is_empty() { self.phase = 4; }
            return Ok(pending(step.progress()));
        }
        if grant.maximum_copy_bytes < std::mem::size_of::<DrawingOwnedRenamePlan>() { return Ok(pending(Default::default())); }
        let disposition = self.disposition.expect("settled owned Drawing rename disposition");
        self.output = Some(DrawingOwnedRenamePlan { disposition, node: self.node.take(), payload: self.payload.take(), path: std::mem::take(&mut self.path), closing: false });
        Ok(DrawingOwnedRenamePreparationStep::Complete { disposition, progress: RetainedCloneProgress { copied_items: 1, copied_bytes: std::mem::size_of::<DrawingOwnedRenamePlan>(), ..Default::default() } })
    }

    /// 🫴️ Transfers the actual old-name projection exactly once, retaining no original payload reference.
    pub fn take(&mut self) -> Option<DrawingOwnedRenamePlan> {
        if self.closing { return None; }
        let output = self.output.take();
        if output.is_some() { self.spent = true; }
        output
    }

    /// 🛑️ Begins cancellation or normal closure of actual children and aliases.
    pub fn begin_close(&mut self) { self.closing = true; if let Some(lookup) = &mut self.lookup { lookup.begin_close(); } if let Some(plan) = &mut self.output { plan.begin_close(); } self.comparison.begin_close(); }

    /// ♻️ Drains one admitted child, native page or actual alias per turn.
    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "owned Drawing rename closure was not started")); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(lookup) = &mut self.lookup {
            let step = lookup.close_granted(grant)?;
            if matches!(step, RetainedCloneStep::Complete(_)) { self.lookup = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        let step = self.comparison.close_step(grant)?;
        if !self.comparison.terminal_is_empty() { return Ok(RetainedCloneStep::Progress(step.progress())); }
        if let Some(plan) = &mut self.output {
            let step = plan.close_granted(grant)?;
            if matches!(step, RetainedCloneStep::Complete(_)) { self.output = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if !self.path.terminal_is_empty() { return close_path(&mut self.path, grant); }
        if let Some(node) = &mut self.node {
            let step = node.close_step(grant)?;
            if node.terminal_is_empty() { self.node = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.payload.is_some() {
            let step = RetainedCloneBinding::close_one(&mut self.payload, grant)?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }

    /// ✅️ Requires every actual child, old-name alias and original binding to be empty.
    pub fn terminal_is_empty(&self) -> bool { self.closing && self.lookup.is_none() && self.comparison.terminal_is_empty() && self.payload.is_none() && self.node.is_none() && self.output.is_none() && self.path.terminal_is_empty() }
}

fn close_path(path: &mut PagedList<usize, {usize::MAX}>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    if !path.is_empty() {
        if grant.maximum_copy_bytes < std::mem::size_of::<usize>() { return Ok(RetainedCloneStep::Progress(Default::default())); }
        path.pop();
        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: std::mem::size_of::<usize>(), ..Default::default() }));
    }
    if !path.terminal_is_empty() {
        let progress = path.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;
        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(progress.progressed), released_bytes: progress.released_allocation_bytes, ..Default::default() }));
    }
    Ok(RetainedCloneStep::Complete(Default::default()))
}

fn pending(progress: RetainedCloneProgress) -> DrawingOwnedRenamePreparationStep { DrawingOwnedRenamePreparationStep::Pending(progress) }
impl Drop for DrawingOwnedRenamePreparationCursor {
    fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "owned Drawing rename preparation reached Drop before exact closure"); }
}
