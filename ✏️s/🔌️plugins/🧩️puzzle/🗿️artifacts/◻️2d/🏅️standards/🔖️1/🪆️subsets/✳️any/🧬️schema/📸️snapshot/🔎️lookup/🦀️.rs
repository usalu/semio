//! 🔎️ Borrowed native identifier lookup retains topology and exact comparison grants.

use crate::Puzzle2dSnapshot;
use semio_framework_value::{paged::PagedUtf8, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneBinding, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, paged::PagedUtf8BoundedOrdCursor, ordered_map::{BoundedOrdCursor, BoundedOrdGrant, BoundedOrdStep}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle2dLookupScope { Node, Edge, Region, Handle }

impl Puzzle2dLookupScope {
    fn identifier(self, snapshot: &Puzzle2dSnapshot, outer: usize, inner: usize) -> Option<&PagedUtf8<{usize::MAX}>> {
        match self {
            Self::Node => snapshot.nodes.get(outer).map(|row| &row.id),
            Self::Edge => snapshot.edges.get(outer).map(|row| &row.id),
            Self::Region => snapshot.target_regions.get(outer).map(|row| &row.id),
            Self::Handle => snapshot.nodes.get(outer)?.handles.get(inner).map(|row| &row.id),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Puzzle2dLookupLocation { pub outer: usize, pub inner: Option<usize> }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle2dLookupStep { Pending(RetainedCloneProgress), Complete { location: Option<Puzzle2dLookupLocation>, progress: RetainedCloneProgress } }

pub struct Puzzle2dLookupCursor {
    scope: Puzzle2dLookupScope,
    outer: usize,
    inner: usize,
    phase: u8,
    source: Option<RetainedCloneBinding>,
    target: Option<RetainedCloneBinding>,
    comparison: PagedUtf8BoundedOrdCursor<{usize::MAX}>,
    output: Option<Option<Puzzle2dLookupLocation>>,
    spent: bool,
    closing: bool,
}

impl Puzzle2dLookupCursor {
    pub fn new(scope: Puzzle2dLookupScope) -> Self {
        Self { scope, outer: 0, inner: 0, phase: 0, source: None, target: None, comparison: Default::default(), output: None, spent: false, closing: false }
    }

    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, target: RetainedCloneRef<'_, PagedUtf8<{usize::MAX}>>, grant: RetainedCloneGrant) -> Result<Puzzle2dLookupStep, ValueError> {
        if self.closing || self.spent { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "native lookup cursor is closing or spent")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(Puzzle2dLookupStep::Pending(Default::default())); }
        source.bind(&mut self.source)?;
        target.bind(&mut self.target)?;
        if let Some(location) = self.output { return Ok(Puzzle2dLookupStep::Complete { location, progress: Default::default() }); }
        if self.phase == 1 {
            let step = self.comparison.close_step(grant)?;
            if self.comparison.terminal_is_empty() { self.comparison = Default::default(); self.phase = 2; }
            return Ok(Puzzle2dLookupStep::Pending(step.progress()));
        }
        if self.phase == 2 {
            if self.scope == Puzzle2dLookupScope::Handle && source.get().nodes.get(self.outer).is_some_and(|node| self.inner + 1 < node.handles.len()) { self.inner += 1; } else { self.outer += 1; self.inner = 0; }
            self.phase = 0;
            return Ok(Puzzle2dLookupStep::Pending(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if self.scope.identifier(source.get(), self.outer, self.inner).is_none() {
            if self.scope == Puzzle2dLookupScope::Handle && source.get().nodes.get(self.outer).is_some() { self.outer += 1; self.inner = 0; return Ok(Puzzle2dLookupStep::Pending(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            self.output = Some(None);
            return Ok(Puzzle2dLookupStep::Complete { location: None, progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } });
        }
        let scope = self.scope;
        let (outer, inner) = (self.outer, self.inner);
        match self.comparison.compare(source.project(1, |snapshot| scope.identifier(snapshot, outer, inner).expect("immutable native lookup candidate")), target, BoundedOrdGrant { maximum_items: grant.maximum_items, maximum_bytes: grant.maximum_copy_bytes })? {
            BoundedOrdStep::Progress(progress) => Ok(Puzzle2dLookupStep::Pending(RetainedCloneProgress { copied_items: progress.compared_items, copied_bytes: progress.compared_bytes, ..Default::default() })),
            BoundedOrdStep::Complete { ordering, progress } => {
                let progress = RetainedCloneProgress { copied_items: progress.compared_items, copied_bytes: progress.compared_bytes, ..Default::default() };
                if ordering == std::cmp::Ordering::Equal {
                    let location = Some(Puzzle2dLookupLocation { outer, inner: (scope == Puzzle2dLookupScope::Handle).then_some(inner) });
                    self.output = Some(location);
                    Ok(Puzzle2dLookupStep::Complete { location, progress })
                } else { self.comparison.begin_close(); self.phase = 1; Ok(Puzzle2dLookupStep::Pending(progress)) }
            }
        }
    }

    pub fn take(&mut self) -> Option<Option<Puzzle2dLookupLocation>> {
        if self.closing { return None; }
        let output = self.output.take();
        if output.is_some() { self.spent = true; }
        output
    }

    pub fn begin_close(&mut self) { self.closing = true; self.comparison.begin_close(); }

    pub fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        if self.output.is_some() { return Ok(std::mem::size_of_val(&self.output)); }
        if !self.comparison.terminal_is_empty() { return self.comparison.next_close_copy_byte_demand(); }
        RetainedCloneBinding::copy_demand(if self.source.is_some() { &self.source } else { &self.target })
    }

    pub fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> {
        if self.output.is_some() { return Ok(0); }
        if !self.comparison.terminal_is_empty() { return self.comparison.next_close_capacity_byte_demand(body); }
        RetainedCloneBinding::capacity_demand(if self.source.is_some() { &self.source } else { &self.target }, body)
    }

    pub fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if self.output.is_some() { return Ok(0); }
        if !self.comparison.terminal_is_empty() { return self.comparison.next_close_release_byte_demand(); }
        RetainedCloneBinding::release_demand(if self.source.is_some() { &self.source } else { &self.target })
    }

    pub fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        if self.output.is_some() { return Ok(1); }
        if !self.comparison.terminal_is_empty() { return BoundedOrdCursor::next_close_depth_demand(&self.comparison); }
        RetainedCloneBinding::depth_demand(if self.source.is_some() { &self.source } else { &self.target })
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "native lookup closure was not started")); }
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.output.is_some() {
            let bytes = std::mem::size_of_val(&self.output);
            if grant.maximum_copy_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.output = None;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }));
        }
        if !self.comparison.terminal_is_empty() { return self.comparison.close_step(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        let step = RetainedCloneBinding::close_one(if self.source.is_some() { &mut self.source } else { &mut self.target }, grant)?;
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.output.is_none() && self.comparison.terminal_is_empty() && self.source.is_none() && self.target.is_none() }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
