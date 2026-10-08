//! 🔎️ Borrowed native identifier lookup retains topology and exact comparison grants.

use crate::Puzzle2dSnapshot;
use semio_framework_value::{paged::PagedUtf8, SnapshotRetirementStep, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneBinding, RetainedCloneRef, paged::PagedUtf8BoundedOrdCursor, ordered_map::{BoundedOrdCursor, BoundedOrdGrant, BoundedOrdProgress, BoundedOrdStep}};

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
pub enum Puzzle2dLookupStep { Pending(BoundedOrdProgress), Complete { location: Option<Puzzle2dLookupLocation>, progress: BoundedOrdProgress } }

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

    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, target: RetainedCloneRef<'_, PagedUtf8<{usize::MAX}>>, grant: BoundedOrdGrant) -> Result<Puzzle2dLookupStep, ValueError> {
        if self.closing || self.spent { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "native lookup cursor is closing or spent")); }
        if grant.maximum_items == 0 { return Ok(Puzzle2dLookupStep::Pending(BoundedOrdProgress::default())); }
        source.bind(&mut self.source)?;
        target.bind(&mut self.target)?;
        if let Some(location) = self.output { return Ok(Puzzle2dLookupStep::Complete { location, progress: BoundedOrdProgress::default() }); }
        if self.phase == 1 {
            return match self.comparison.close_step(grant.maximum_items, grant.maximum_bytes)? {
                SnapshotRetirementStep::Complete => { self.comparison = Default::default(); self.phase = 2; Ok(Puzzle2dLookupStep::Pending(BoundedOrdProgress::default())) },
                SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(Puzzle2dLookupStep::Pending(BoundedOrdProgress { compared_items: released_items, compared_bytes: released_bytes })),
                SnapshotRetirementStep::Blocked => Ok(Puzzle2dLookupStep::Pending(BoundedOrdProgress::default())),
            };
        }
        if self.phase == 2 {
            if self.scope == Puzzle2dLookupScope::Handle && source.get().nodes.get(self.outer).is_some_and(|node| self.inner + 1 < node.handles.len()) { self.inner += 1; } else { self.outer += 1; self.inner = 0; }
            self.phase = 0;
            return Ok(Puzzle2dLookupStep::Pending(BoundedOrdProgress { compared_items: 1, compared_bytes: 0 }));
        }
        if self.scope.identifier(source.get(), self.outer, self.inner).is_none() {
            if self.scope == Puzzle2dLookupScope::Handle && source.get().nodes.get(self.outer).is_some() { self.outer += 1; self.inner = 0; return Ok(Puzzle2dLookupStep::Pending(BoundedOrdProgress { compared_items: 1, compared_bytes: 0 })); }
            self.output = Some(None);
            return Ok(Puzzle2dLookupStep::Complete { location: None, progress: BoundedOrdProgress { compared_items: 1, compared_bytes: 0 } });
        }
        let scope = self.scope;
        let (outer, inner) = (self.outer, self.inner);
        match self.comparison.compare(source.project(1, |snapshot| scope.identifier(snapshot, outer, inner).expect("immutable native lookup candidate")), target, grant)? {
            BoundedOrdStep::Progress(progress) => Ok(Puzzle2dLookupStep::Pending(progress)),
            BoundedOrdStep::Complete { ordering, progress } => {
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

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
        if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "native lookup closure was not started")); }
        if self.output.is_some() {
            if maximum_items == 0 { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
            self.output = None;
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        let step = self.comparison.close_step(maximum_items, maximum_bytes)?;
        if step != SnapshotRetirementStep::Complete { return Ok(step); }
        let step = RetainedCloneBinding::close_one(&mut self.source, maximum_items)?;
        if step != SnapshotRetirementStep::Complete { return Ok(step); }
        RetainedCloneBinding::close_one(&mut self.target, maximum_items)
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.output.is_none() && self.comparison.terminal_is_empty() && self.source.is_none() && self.target.is_none() }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
