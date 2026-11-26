//! 🫳️ Bounded identifier reads authenticate a fresh original loan at every turn.

use super::{pay, Puzzle2dLookupLocation, Puzzle2dLookupStep, COMPARE_BYTES};
use crate::Puzzle2dSnapshot;
use semio_framework_value::paged::PagedUtf8;
use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep};
use semio_framework_value::{OriginalRetirementPoll, RetainedAllocationEffect, RetirementDemand, RetirementPollExit, ValueError};
use std::{cmp::Ordering, num::NonZeroU64};

#[derive(Clone, Copy, Eq, PartialEq)]
struct Identity {
    source: NonZeroU64,
    projection: [usize; 3],
}

/// 🪪️ Fences a borrower to the same original source and projection without retaining its payload.
#[derive(Default)]
pub struct BorrowedIdentity(Option<Identity>);

impl BorrowedIdentity {
    pub fn bind<T: ?Sized>(&mut self, source: RetainedCloneRef<'_, T>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Option<RetainedCloneProgress>, ValueError> {
        let (id, parent, address, discriminator) = source.reference_identity();
        let source = NonZeroU64::new(id).ok_or_else(|| ValueError::invariant("borrowed preparation has no original source"))?;
        let expected = Identity { source, projection: [parent, address, discriminator] };
        if let Some(actual) = self.0 {
            return if actual == expected { Ok(None) } else { Err(ValueError::invariant("borrowed preparation changed its original projection")) };
        }
        let bytes = size_of::<Option<Identity>>();
        if !funded(effect, bytes)? { return Ok(Some(Default::default())); }
        self.0 = Some(expected);
        Ok(Some(progress(bytes)))
    }

    pub fn is_empty(&self) -> bool { self.0.is_none() }

    pub fn close_demand(&self) -> RetirementDemand {
        demand(if self.is_empty() { 0 } else { size_of::<Option<Identity>>() })
    }

    pub fn close(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        let bytes = self.close_demand().copy_bytes;
        if bytes == 0 || !funded(effect, bytes)? { return Ok(Default::default()); }
        self.0 = None;
        Ok(progress(bytes))
    }
}

#[derive(Default, PartialEq)]
struct ChunkPosition { chunk: usize, offset: usize }

/// 🔡️ Compares immutable identifier chunks in bounded windows, without an owned text copy.
#[derive(Default)]
pub struct BorrowedComparison {
    left: BorrowedIdentity,
    right: BorrowedIdentity,
    left_position: ChunkPosition,
    right_position: ChunkPosition,
    ordering: Option<Ordering>,
}

impl BorrowedComparison {
    pub fn compare(&mut self, left: RetainedCloneRef<'_, PagedUtf8<{ usize::MAX }>>, right: RetainedCloneRef<'_, PagedUtf8<{ usize::MAX }>>, effect: &mut RetainedAllocationEffect<'_>) -> Result<(Option<Ordering>, RetainedCloneProgress), ValueError> {
        if let Some(progress) = self.left.bind(left, effect)? { return Ok((None, progress)); }
        if let Some(progress) = self.right.bind(right, effect)? { return Ok((None, progress)); }
        if self.ordering.is_some() { return Ok((self.ordering, Default::default())); }
        let left = left.get().retained_chunks().get(self.left_position.chunk);
        let right = right.get().retained_chunks().get(self.right_position.chunk);
        for (chunk, position) in [(left, &mut self.left_position), (right, &mut self.right_position)] {
            if chunk.is_some_and(|chunk| position.offset == chunk.len()) {
                let bytes = size_of::<ChunkPosition>();
                if !funded(effect, bytes)? { return Ok((None, Default::default())); }
                *position = ChunkPosition { chunk: position.chunk + 1, offset: 0 };
                return Ok((None, progress(bytes)));
            }
        }
        let bytes = size_of::<Option<Ordering>>();
        let (Some(left), Some(right)) = (left, right) else {
            if !funded(effect, bytes)? { return Ok((None, Default::default())); }
            self.ordering = Some(left.is_some().cmp(&right.is_some()));
            return Ok((self.ordering, progress(bytes)));
        };
        let grant = effect.grant();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < 2 * size_of::<usize>() + bytes {
            return Ok((None, Default::default()));
        }
        let left = &left.as_bytes()[self.left_position.offset..];
        let right = &right.as_bytes()[self.right_position.offset..];
        let count = usize::try_from(effect.remaining_work_units()?).unwrap_or(usize::MAX).min(COMPARE_BYTES);
        let mut scanned = 0;
        for (&left, &right) in left.iter().zip(right).take(count) {
            if !pay(effect)? { break; }
            scanned += 1;
            if left != right {
                self.ordering = Some(left.cmp(&right));
                break;
            }
        }
        if scanned == 0 { return Ok((None, Default::default())); }
        self.left_position.offset += scanned;
        self.right_position.offset += scanned;
        Ok((self.ordering, progress(2 * size_of::<usize>() + if self.ordering.is_some() { bytes } else { 0 })))
    }

    pub fn close_demand(&self) -> RetirementDemand {
        if !self.left.is_empty() { return self.left.close_demand(); }
        if !self.right.is_empty() { return self.right.close_demand(); }
        if self.left_position != ChunkPosition::default() || self.right_position != ChunkPosition::default() {
            return demand(size_of::<ChunkPosition>());
        }
        demand(if self.ordering.is_some() { size_of::<Option<Ordering>>() } else { 0 })
    }

    pub fn close(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        if !self.left.is_empty() { return self.left.close(effect); }
        if !self.right.is_empty() { return self.right.close(effect); }
        let bytes = self.close_demand().copy_bytes;
        if bytes == 0 || !funded(effect, bytes)? { return Ok(Default::default()); }
        if self.left_position != ChunkPosition::default() { self.left_position = Default::default(); }
        else if self.right_position != ChunkPosition::default() { self.right_position = Default::default(); }
        else { self.ordering = None; }
        Ok(progress(bytes))
    }

    pub fn is_empty(&self) -> bool { self.close_demand().copy_bytes == 0 }
}

/// 🔎️ Scans node identifiers while the caller keeps the original immutable loans.
#[derive(Default)]
pub struct BorrowedNodeLookup {
    source: BorrowedIdentity,
    target: BorrowedIdentity,
    comparison: BorrowedComparison,
    index: usize,
    output: Option<Option<Puzzle2dLookupLocation>>,
    phase: u8,
    closing: bool,
}

impl BorrowedNodeLookup {
    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, target: RetainedCloneRef<'_, PagedUtf8<{ usize::MAX }>>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Puzzle2dLookupStep, ValueError> {
        use Puzzle2dLookupStep::{Complete, Pending};
        if self.closing || self.phase == 3 { return Err(ValueError::invariant("borrowed lookup is closing or spent")); }
        if let Some(progress) = self.source.bind(source, effect)? { return Ok(Pending(progress)); }
        if let Some(progress) = self.target.bind(target, effect)? { return Ok(Pending(progress)); }
        if let Some(location) = self.output { return Ok(Complete { location, progress: Default::default() }); }
        if self.phase == 1 {
            if !self.comparison.is_empty() { return Ok(Pending(self.comparison.close(effect)?)); }
            let bytes = size_of::<usize>() + size_of::<u8>();
            if !funded(effect, bytes)? { return Ok(Pending(Default::default())); }
            self.index += 1;
            self.phase = 0;
            return Ok(Pending(progress(bytes)));
        }
        let index = self.index;
        if self.phase == 2 || source.get().nodes.get(index).is_none() {
            let bytes = size_of_val(&self.output);
            if !funded(effect, bytes)? { return Ok(Pending(Default::default())); }
            let location = (self.phase == 2).then_some(Puzzle2dLookupLocation { outer: index, inner: None });
            self.output = Some(location);
            return Ok(Complete { location, progress: progress(bytes) });
        }
        let candidate = source.try_project(1, |snapshot| snapshot.nodes.get(index).map(|node| &node.id).ok_or_else(|| ValueError::invariant("borrowed lookup candidate is absent")))?;
        if let Some(ordering) = self.comparison.ordering {
            let bytes = size_of::<u8>();
            if !funded(effect, bytes)? { return Ok(Pending(Default::default())); }
            self.phase = if ordering == Ordering::Equal { 2 } else { 1 };
            return Ok(Pending(progress(bytes)));
        }
        let (_, progress) = self.comparison.compare(candidate, target, effect)?;
        Ok(Pending(progress))
    }

    pub fn take(&mut self) -> Option<Option<Puzzle2dLookupLocation>> {
        if self.closing { return None; }
        let output = self.output.take();
        if output.is_some() { self.phase = 3; }
        output
    }

    pub fn begin_close(&mut self) { self.closing = true; }

    pub fn close_demands(&self, _: usize) -> Result<RetirementDemand, ValueError> {
        Ok(if self.output.is_some() { demand(size_of_val(&self.output)) }
            else if !self.comparison.is_empty() { self.comparison.close_demand() }
            else if !self.source.is_empty() { self.source.close_demand() }
            else { self.target.close_demand() })
    }

    pub fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
    pub fn next_close_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }
    pub fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    pub fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.depth) }

    pub fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        let result = (|| {
            if !self.closing { return Err(ValueError::invariant("borrowed lookup closure was not started").into()); }
            let progress = if self.output.is_some() {
                let bytes = size_of_val(&self.output);
                if !funded(effect, bytes)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.output = None;
                progress(bytes)
            } else if !self.comparison.is_empty() { self.comparison.close(effect)? }
            else if !self.source.is_empty() { self.source.close(effect)? }
            else { self.target.close(effect)? };
            Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
        })();
        RetirementPollExit::settle(effect, result)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.output.is_none() && self.comparison.is_empty() && self.source.is_empty() && self.target.is_empty()
    }
}

fn funded(effect: &mut RetainedAllocationEffect<'_>, bytes: usize) -> Result<bool, ValueError> {
    let grant = effect.grant();
    if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < bytes { return Ok(false); }
    pay(effect)
}

fn progress(bytes: usize) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }
}

fn demand(bytes: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes: bytes, depth: usize::from(bytes != 0), ..Default::default() }
}
