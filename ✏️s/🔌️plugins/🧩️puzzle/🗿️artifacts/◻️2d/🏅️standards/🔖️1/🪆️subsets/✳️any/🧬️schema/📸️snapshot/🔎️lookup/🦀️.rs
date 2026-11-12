//! 🔎️ Borrowed native identifier lookup retains topology and exact comparison under the original effect.

use crate::Puzzle2dSnapshot;
use semio_framework_value::paged::PagedUtf8;
use semio_framework_value::paged::retained_clone::cursors::PagedUtf8BoundedOrdCursor;
use semio_framework_value::{OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkPoll, RetirementPollExit, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{OriginalEffectReply, OriginalEffectReplyComplete, RetainedCloneBinding, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, ordered_map::{BoundedOrdCursor, BoundedOrdGrant}};

const COMPARE_BYTES: usize = 64;

pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::preparation::native_preparation_child::{pay, replied, settled_step};

/// 🔎️ One owned UTF-8 identifier comparison whose bindings close under the caller's original effect before the next comparison.
#[derive(Default)]
pub struct Puzzle2dIdentifierComparison { cursor: PagedUtf8BoundedOrdCursor<{usize::MAX}> }

impl Puzzle2dIdentifierComparison {
    pub fn compare(&mut self, left: RetainedCloneRef<'_, PagedUtf8<{usize::MAX}>>, right: RetainedCloneRef<'_, PagedUtf8<{usize::MAX}>>, effect: &mut RetainedAllocationEffect<'_>) -> Result<(Option<std::cmp::Ordering>, RetainedCloneProgress), ValueError> {
        let grant = effect.grant();
        let window = BoundedOrdGrant { maximum_items: grant.maximum_items, maximum_bytes: grant.maximum_copy_bytes.min(COMPARE_BYTES) };
        let step = replied(effect, |effect, reply| self.cursor.compare_into(left, right, window, COMPARE_BYTES, effect, reply))?;
        Ok((self.cursor.ordering(), step.progress()))
    }

    pub fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        let result = (|| -> Result<RetainedCloneStep, RetirementPollExit> {
            if self.cursor.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
            if !self.cursor.close_started() { return Ok(replied(effect, |effect, reply| self.cursor.begin_close_into(effect, reply))?); }
            let body = effect.grant().maximum_copy_bytes;
            let step = RetirementPollExit::resume(RetainedCloneBinding::close_one(if self.cursor.left.is_some() { &mut self.cursor.left } else { &mut self.cursor.right }, body, effect))?;
            Ok(if self.cursor.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
        })();
        RetirementPollExit::settle(effect, result)
    }

    pub fn reset(&mut self) { self.cursor = Default::default(); }

    pub fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.cursor.close_started() { self.cursor.close_demands(body) } else if self.cursor.terminal_is_empty() { Ok(Default::default()) } else { Ok(self.cursor.begin_close_demand()) }
    }

    pub fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.copy_bytes) }

    pub fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.close_demands(body)?.capacity_bytes) }

    pub fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.release_bytes) }

    pub fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.depth) }

    pub fn terminal_is_empty(&self) -> bool { self.cursor.terminal_is_empty() }
}

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
    comparison: Puzzle2dIdentifierComparison,
    output: Option<Option<Puzzle2dLookupLocation>>,
    spent: bool,
    closing: bool,
}

impl Puzzle2dLookupCursor {
    pub fn new(scope: Puzzle2dLookupScope) -> Self {
        Self { scope, outer: 0, inner: 0, phase: 0, source: None, target: None, comparison: Default::default(), output: None, spent: false, closing: false }
    }

    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, target: RetainedCloneRef<'_, PagedUtf8<{usize::MAX}>>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Puzzle2dLookupStep, ValueError> {
        if self.closing || self.spent { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "native lookup cursor is closing or spent")); }
        let grant = effect.grant();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(Puzzle2dLookupStep::Pending(Default::default())); }
        if let Some(progress) = source.bind(&mut self.source, effect).result? { return Ok(Puzzle2dLookupStep::Pending(progress)); }
        if let Some(progress) = target.bind(&mut self.target, effect).result? { return Ok(Puzzle2dLookupStep::Pending(progress)); }
        if let Some(location) = self.output { return Ok(Puzzle2dLookupStep::Complete { location, progress: Default::default() }); }
        let event = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        if self.phase == 1 {
            let step = settled_step(self.comparison.close_step(effect))?;
            if self.comparison.terminal_is_empty() { self.comparison.reset(); self.phase = 2; }
            return Ok(Puzzle2dLookupStep::Pending(step.progress()));
        }
        if self.phase == 2 {
            if !pay(effect)? { return Ok(Puzzle2dLookupStep::Pending(Default::default())); }
            if self.scope == Puzzle2dLookupScope::Handle && source.get().nodes.get(self.outer).is_some_and(|node| self.inner + 1 < node.handles.len()) { self.inner += 1; } else { self.outer += 1; self.inner = 0; }
            self.phase = 0;
            return Ok(Puzzle2dLookupStep::Pending(event));
        }
        if self.scope.identifier(source.get(), self.outer, self.inner).is_none() {
            if !pay(effect)? { return Ok(Puzzle2dLookupStep::Pending(Default::default())); }
            if self.scope == Puzzle2dLookupScope::Handle && source.get().nodes.get(self.outer).is_some() { self.outer += 1; self.inner = 0; return Ok(Puzzle2dLookupStep::Pending(event)); }
            self.output = Some(None);
            return Ok(Puzzle2dLookupStep::Complete { location: None, progress: event });
        }
        let scope = self.scope;
        let (outer, inner) = (self.outer, self.inner);
        let candidate = source.try_project(1, |snapshot| scope.identifier(snapshot, outer, inner).ok_or_else(|| ValueError::invariant("immutable native lookup candidate is absent")))?;
        let (ordering, progress) = self.comparison.compare(candidate, target, effect)?;
        let Some(ordering) = ordering else { return Ok(Puzzle2dLookupStep::Pending(progress)) };
        if ordering == std::cmp::Ordering::Equal {
            let location = Some(Puzzle2dLookupLocation { outer, inner: (scope == Puzzle2dLookupScope::Handle).then_some(inner) });
            self.output = Some(location);
            Ok(Puzzle2dLookupStep::Complete { location, progress })
        } else { self.phase = 1; Ok(Puzzle2dLookupStep::Pending(progress)) }
    }

    pub fn take(&mut self) -> Option<Option<Puzzle2dLookupLocation>> {
        if self.closing { return None; }
        let output = self.output.take();
        if output.is_some() { self.spent = true; }
        output
    }

    pub fn begin_close(&mut self) { self.closing = true; }

    pub fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.output.is_some() { return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of_val(&self.output), depth: 1, ..Default::default() }); }
        if !self.comparison.terminal_is_empty() { return self.comparison.close_demands(body); }
        RetainedCloneBinding::close_demand(if self.source.is_some() { &self.source } else { &self.target }, body)
    }

    pub fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.copy_bytes) }

    pub fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.close_demands(body)?.capacity_bytes) }

    pub fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.release_bytes) }

    pub fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.depth) }

    pub fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        let result = (|| -> Result<RetainedCloneStep, RetirementPollExit> {
            if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "native lookup closure was not started").into()); }
            if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
            let grant = effect.grant();
            if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
            if self.output.is_some() {
                let bytes = size_of_val(&self.output);
                if grant.maximum_copy_bytes < bytes || !pay(effect)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.output = None;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }));
            }
            if !self.comparison.terminal_is_empty() { return Ok(RetainedCloneStep::Progress(RetirementPollExit::resume(self.comparison.close_step(effect))?.progress())); }
            let step = RetirementPollExit::resume(RetainedCloneBinding::close_one(if self.source.is_some() { &mut self.source } else { &mut self.target }, grant.maximum_copy_bytes, effect))?;
            Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
        })();
        RetirementPollExit::settle(effect, result)
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.output.is_none() && self.comparison.terminal_is_empty() && self.source.is_none() && self.target.is_none() }
}
