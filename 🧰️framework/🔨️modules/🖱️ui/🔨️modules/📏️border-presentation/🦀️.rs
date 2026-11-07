//! 📏️ Exact shared-edge ownership matching the language-neutral border presentation vectors.

use std::cmp::Reverse;
#[path = "ordered.rs"]
mod ordered;
use ordered::Ordered;

/// 🎚️ Paint precedence shared by native and browser renderers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SharedBorderState { Normal, Focus, Active, Waiting, Loading, Introduced, Celebrated, Hover }

/// 📐 One identified axis-aligned edge in the caller's common paint space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SharedBorderSegment {
    pub id: usize,
    pub owner: usize,
    pub from: [f64; 2],
    pub to: [f64; 2],
    pub state: SharedBorderState,
}

/// 🧩 A disjoint edge retaining every incident contributor.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedSharedBorderSegment {
    pub segment: SharedBorderSegment,
    pub contributors: Vec<usize>,
}

/// 🪶 Winning paint and final incident source without allocating a contributor list.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResolvedSharedBorderOwnership {
    pub segment: SharedBorderSegment,
    pub last_contributor: usize,
}

/// 🛤️ One bounded ownership-only sweep result.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SharedBorderOwnershipStep { Pending, Segment(ResolvedSharedBorderOwnership), Complete }

/// 🛑 Invalid ownership or geometry cannot enter shared-border paint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SharedBorderError { DuplicateId, NonFinite, Diagonal, ResolutionStarted, CapacityOverflow }

impl std::fmt::Display for SharedBorderError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::DuplicateId => "Duplicate shared border identity",
            Self::NonFinite => "Shared border coordinates must be finite",
            Self::Diagonal => "Shared border must be axis aligned",
            Self::ResolutionStarted => "Shared border collection has ended",
            Self::CapacityOverflow => "Shared border storage extent overflow",
        })
    }
}

impl std::error::Error for SharedBorderError {}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Coordinate(f64);
impl Eq for Coordinate {}
impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.0.total_cmp(&other.0) }
}
impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Coordinate {
    fn new(value: f64) -> Self { Self(if value == 0.0 { 0.0 } else { value }) }
}

type Event = (bool, Coordinate, Coordinate, bool, usize);
type Ownership = (Reverse<SharedBorderState>, usize, usize);

#[derive(Clone, Debug)]
struct PendingBorder {
    segment: SharedBorderSegment,
    contributors: Box<[std::mem::MaybeUninit<usize>]>,
    filled: usize,
}

impl PendingBorder {
    /// 📦 Transfers a fully initialized exact-length backing allocation without resizing.
    fn finish(self) -> ResolvedSharedBorderSegment {
        assert_eq!(self.filled, self.contributors.len());
        ResolvedSharedBorderSegment { segment: self.segment, contributors: unsafe { self.contributors.assume_init() }.into_vec() }
    }
}

/// ⏩ One bounded resolution result; contributor accumulation also yields between items.
#[derive(Clone, Debug, PartialEq)]
pub enum SharedBorderResolutionStep { Pending, Segment(ResolvedSharedBorderSegment), Complete }

/// 🧭 Incremental exact-edge sweep with bounded ingestion, resolution and cancellation retirement.
#[derive(Clone, Debug)]
pub struct SharedBorderResolutionCursor {
    events: Ordered<Event, SharedBorderSegment>,
    ids: Ordered<usize, ()>,
    active: Ordered<Ownership, SharedBorderSegment>,
    contributors: Ordered<usize, ()>,
    position: Option<(bool, Coordinate, Coordinate)>,
    ready: bool,
    pending: Option<PendingBorder>,
    last_contributor: Option<usize>,
    started: bool,
    retiring: bool,
    finished: bool,
    summary: bool,
    summary_output: Option<ResolvedSharedBorderOwnership>,
}

impl Default for SharedBorderResolutionCursor {
    fn default() -> Self {
        Self { events: Ordered::new(), ids: Ordered::new(), active: Ordered::new(), contributors: Ordered::new(), position: None, ready: false, pending: None, last_contributor: None, started: false, retiring: false, finished: false, summary: false, summary_output: None }
    }
}

impl SharedBorderResolutionCursor {
    /// 🌱 Creates an empty sweep; callers assign stable numeric owner and source identities.
    pub fn new() -> Self { Self::default() }

    /// 📥 Adds one edge before the first resolution step without a bulk sorting phase.
    pub fn push(&mut self, segment: SharedBorderSegment) -> Result<(), SharedBorderError> {
        self.next_push_allocation_byte_demand(&segment)?;
        self.ids.insert(segment.id, ());
        if segment.from == segment.to { return Ok(()); }
        let vertical = segment.from[1] != segment.to[1];
        let axis = usize::from(vertical);
        let coordinate = Coordinate::new(segment.from[usize::from(!vertical)]);
        self.events.insert((vertical, coordinate, Coordinate::new(segment.from[axis].min(segment.to[axis])), true, segment.id), segment);
        self.events.insert((vertical, coordinate, Coordinate::new(segment.from[axis].max(segment.to[axis])), false, segment.id), segment);
        Ok(())
    }

    /// 🧾 Accepts one source and reports its exact backing growth and retained payload copies.
    pub fn push_with_progress(&mut self, segment: SharedBorderSegment) -> Result<semio_framework_job::RetainedCloneProgress, SharedBorderError> {
        let before = self.retained_capacity_bytes();
        self.push(segment)?;
        Ok(semio_framework_job::RetainedCloneProgress { copied_items: 1, copied_bytes: if segment.from == segment.to { 0 } else { 2 * std::mem::size_of::<SharedBorderSegment>() }, retained_capacity_bytes: self.retained_capacity_bytes() - before, released_bytes: 0 })
    }

    /// 🪙 Preflights exact additional backing capacity before accepting one source edge.
    pub fn next_push_allocation_byte_demand(&self, segment: &SharedBorderSegment) -> Result<usize, SharedBorderError> {
        if self.started { return Err(SharedBorderError::ResolutionStarted); }
        if self.ids.contains(segment.id) { return Err(SharedBorderError::DuplicateId); }
        if !segment.from.iter().chain(&segment.to).all(|value| value.is_finite()) { return Err(SharedBorderError::NonFinite); }
        if segment.from[0] != segment.to[0] && segment.from[1] != segment.to[1] { return Err(SharedBorderError::Diagonal); }
        let ids = self.ids.allocation_demand(1).ok_or(SharedBorderError::CapacityOverflow)?;
        let events = self.events.allocation_demand(if segment.from == segment.to { 0 } else { 2 }).ok_or(SharedBorderError::CapacityOverflow)?;
        ids.checked_add(events).ok_or(SharedBorderError::CapacityOverflow)
    }

    /// 🛡️ Bounds backing growth before a producer has emitted the next clipped source.
    pub fn maximum_push_allocation_byte_demand(&self) -> Result<usize, SharedBorderError> {
        if self.started { return Err(SharedBorderError::ResolutionStarted); }
        self.ids.allocation_demand(1).and_then(|ids| ids.checked_add(self.events.allocation_demand(2)?)).ok_or(SharedBorderError::CapacityOverflow)
    }

    /// 🪜 Resolves one event, one contributor, or one line transition per call.
    pub fn step(&mut self) -> SharedBorderResolutionStep {
        if self.retiring || self.finished { return SharedBorderResolutionStep::Complete; }
        self.started = true;
        if let Some(pending) = &mut self.pending {
            let next = match self.last_contributor {
                Some(last) => self.contributors.after(last),
                None => self.contributors.first(),
            };
            if let Some((id, ())) = next {
                pending.contributors[pending.filled].write(id);
                pending.filled += 1;
                self.last_contributor = Some(id);
                return SharedBorderResolutionStep::Pending;
            }
            self.last_contributor = None;
            return SharedBorderResolutionStep::Segment(self.pending.take().expect("Pending segment").finish());
        }
        if self.ready {
            let (vertical, coordinate, position) = self.position.expect("Sweep position");
            let next = self.events.first();
            if next.is_none_or(|(event, _)| (event.0, event.1, event.2) != (vertical, coordinate, position)) {
                self.ready = false;
                if let (Some((end, _)), Some((_, winner))) = (next, self.active.first()) {
                    if (end.0, end.1) == (vertical, coordinate) {
                        let from = if vertical { [coordinate.0, position.0] } else { [position.0, coordinate.0] };
                        let to = if vertical { [coordinate.0, end.2.0] } else { [end.2.0, coordinate.0] };
                        if self.summary {
                            self.summary_output = Some(ResolvedSharedBorderOwnership { segment: SharedBorderSegment { from, to, ..winner }, last_contributor: self.contributors.last().expect("Incident source").0 });
                            return SharedBorderResolutionStep::Pending;
                        }
                        self.pending = Some(PendingBorder { segment: SharedBorderSegment { from, to, ..winner }, contributors: Box::new_uninit_slice(self.contributors.len()), filled: 0 });
                    }
                }
                return SharedBorderResolutionStep::Pending;
            }
        }
        if let Some((event, segment)) = self.events.pop_first() {
            self.position = Some((event.0, event.1, event.2));
            self.ready = true;
            let key = (Reverse(segment.state), segment.owner, segment.id);
            if event.3 { self.active.insert(key, segment); self.contributors.insert(segment.id, ()); }
            else { self.active.remove(key); self.contributors.remove(segment.id); }
            return SharedBorderResolutionStep::Pending;
        }
        if self.ids.pop_first().is_some() { return SharedBorderResolutionStep::Pending; }
        self.finished = true;
        SharedBorderResolutionStep::Complete
    }

    /// 📋 Advances one scalar or payload step and reports its exact owned backing movement.
    pub fn step_with_progress(&mut self) -> (SharedBorderResolutionStep, semio_framework_job::RetainedCloneProgress) {
        let before = self.retained_capacity_bytes();
        let transferred = self.next_step_transferred_byte_demand();
        let copied_bytes = self.next_step_copy_byte_demand();
        let copied_items = usize::from(!self.finished && !self.retiring);
        let step = self.step();
        let capacity = self.retained_capacity_bytes().checked_add(transferred).and_then(|bytes| bytes.checked_sub(before)).expect("Owned border backing extent");
        (step, semio_framework_job::RetainedCloneProgress { copied_items, copied_bytes, retained_capacity_bytes: capacity, released_bytes: 0 })
    }

    /// 🗑️ Releases one retained item per call when resolution is cancelled.
    pub fn retire_step(&mut self) -> bool {
        self.started = true;
        self.retiring = true;
        self.summary_output = None;
        if let Some(pending) = &mut self.pending {
            if pending.filled > 0 { pending.filled -= 1; return false; }
            self.pending = None;
            self.last_contributor = None;
            return false;
        }
        self.events.retire_step() && self.active.retire_step() && self.contributors.retire_step() && self.ids.retire_step()
    }

    /// 💾 Exact backing capacity owned by the cursor, excluding the inline cursor value.
    pub fn retained_capacity_bytes(&self) -> usize {
        self.events.capacity_bytes() + self.ids.capacity_bytes() + self.active.capacity_bytes() + self.contributors.capacity_bytes() + self.pending.as_ref().map_or(0, |pending| pending.contributors.len() * std::mem::size_of::<usize>())
    }

    /// 🧮 Preflights additional backing bytes for the next bounded resolution step.
    pub fn next_step_allocation_byte_demand(&self) -> Result<usize, SharedBorderError> {
        if self.retiring { return Ok(0); }
        if self.pending.is_some() { return Ok(0); }
        if self.ready {
            let position = self.position.expect("Sweep position");
            if self.events.first().is_none_or(|(event, _)| (event.0, event.1, event.2) != position) {
                return if self.events.first().is_some_and(|(event, _)| (event.0, event.1) == (position.0, position.1)) && self.active.first().is_some() {
                    if self.summary { Ok(0) } else { self.contributors.len().checked_mul(std::mem::size_of::<usize>()).ok_or(SharedBorderError::CapacityOverflow) }
                } else { Ok(0) };
            }
        }
        if self.events.first().is_some_and(|(event, _)| event.3) {
            return self.active.allocation_demand(1).and_then(|active| active.checked_add(self.contributors.allocation_demand(1)?)).ok_or(SharedBorderError::CapacityOverflow);
        }
        Ok(0)
    }

    /// 📤 Exact backing transferred with the next emitted segment, rather than freed by this cursor.
    pub fn next_step_transferred_byte_demand(&self) -> usize {
        if self.finished || self.retiring { return 0; }
        self.pending.as_ref().filter(|pending| pending.filled == pending.contributors.len()).map_or(0, |pending| pending.contributors.len() * std::mem::size_of::<usize>())
    }

    /// 📄 Bounds payload bytes copied by one source ingestion step, independent of its geometry.
    pub fn maximum_push_copy_byte_demand(&self) -> usize { 2 * std::mem::size_of::<SharedBorderSegment>() }
    pub fn maximum_push_depth_demand(&self) -> usize { self.ids.depth().max(self.events.depth()) + 1 }
    pub fn next_step_depth_demand(&self) -> usize { self.events.depth().max(self.ids.depth()).max(self.active.depth() + 1).max(self.contributors.depth() + 1).max(1) }
    pub fn next_retirement_depth_demand(&self) -> usize { self.events.depth().max(self.ids.depth()).max(self.active.depth()).max(self.contributors.depth()).max(1) }

    /// 📑 Payload copying for the next event or contributor step; ordering metadata remains inline.
    pub fn next_step_copy_byte_demand(&self) -> usize {
        if self.finished || self.retiring { return 0; }
        if let Some(pending) = &self.pending { return if pending.filled < pending.contributors.len() { std::mem::size_of::<usize>() } else { 0 }; }
        if self.ready && self.events.first().is_none_or(|(event, _)| Some((event.0, event.1, event.2)) != self.position) {
            return if self.summary && self.active.first().is_some() && self.events.first().is_some_and(|(event, _)| self.position.is_some_and(|position| (event.0, event.1) == (position.0, position.1))) { std::mem::size_of::<ResolvedSharedBorderOwnership>() } else { 0 };
        }
        if self.events.first().is_some() { std::mem::size_of::<SharedBorderSegment>() } else { 0 }
    }

    /// 🧾 Exact allocation released by the next incremental cancellation step.
    pub fn next_retirement_release_byte_demand(&self) -> usize {
        if let Some(pending) = &self.pending {
            return if pending.filled == 0 { pending.contributors.len() * std::mem::size_of::<usize>() } else { 0 };
        }
        if !self.events.retirement_is_empty() { return self.events.next_release_bytes(); }
        if !self.active.retirement_is_empty() { return self.active.next_release_bytes(); }
        if !self.contributors.retirement_is_empty() { return self.contributors.next_release_bytes(); }
        if !self.ids.retirement_is_empty() { return self.ids.next_release_bytes(); }
        0
    }
}

/// 🧷 Renderer sweep whose output remains bounded even when many sources share one edge.
#[derive(Clone, Debug)]
pub struct SharedBorderOwnershipCursor { inner: SharedBorderResolutionCursor }

impl Default for SharedBorderOwnershipCursor {
    fn default() -> Self { Self { inner: SharedBorderResolutionCursor { summary: true, ..SharedBorderResolutionCursor::new() } } }
}

impl SharedBorderOwnershipCursor {
    pub fn new() -> Self { Self::default() }
    pub fn push(&mut self, segment: SharedBorderSegment) -> Result<(), SharedBorderError> { self.inner.push(segment) }
    pub fn push_with_progress(&mut self, segment: SharedBorderSegment) -> Result<semio_framework_job::RetainedCloneProgress, SharedBorderError> { self.inner.push_with_progress(segment) }
    pub fn next_push_allocation_byte_demand(&self, segment: &SharedBorderSegment) -> Result<usize, SharedBorderError> { self.inner.next_push_allocation_byte_demand(segment) }
    pub fn maximum_push_allocation_byte_demand(&self) -> Result<usize, SharedBorderError> { self.inner.maximum_push_allocation_byte_demand() }
    pub fn maximum_push_copy_byte_demand(&self) -> usize { self.inner.maximum_push_copy_byte_demand() }
    pub fn maximum_push_depth_demand(&self) -> usize { self.inner.maximum_push_depth_demand() }
    pub fn next_step_depth_demand(&self) -> usize { self.inner.next_step_depth_demand() }
    pub fn next_retirement_depth_demand(&self) -> usize { self.inner.next_retirement_depth_demand() }
    pub fn next_step_allocation_byte_demand(&self) -> Result<usize, SharedBorderError> { self.inner.next_step_allocation_byte_demand() }
    pub fn next_step_copy_byte_demand(&self) -> usize { self.inner.next_step_copy_byte_demand() }
    pub fn next_step_transferred_byte_demand(&self) -> usize { 0 }
    pub fn next_step_emits_segment(&self) -> bool {
        !self.inner.finished && !self.inner.retiring && self.inner.ready && self.inner.active.first().is_some() && self.inner.events.first().is_some_and(|(event, _)| self.inner.position.is_some_and(|position| (event.0, event.1) == (position.0, position.1) && event.2 != position.2))
    }
    pub fn retained_capacity_bytes(&self) -> usize { self.inner.retained_capacity_bytes() }
    pub fn next_retirement_release_byte_demand(&self) -> usize { self.inner.next_retirement_release_byte_demand() }
    pub fn retire_step(&mut self) -> bool { self.inner.retire_step() }
    pub fn step(&mut self) -> SharedBorderOwnershipStep { self.step_with_progress().0 }
    pub fn step_with_progress(&mut self) -> (SharedBorderOwnershipStep, semio_framework_job::RetainedCloneProgress) {
        let (step, progress) = self.inner.step_with_progress();
        let step = if let Some(segment) = self.inner.summary_output.take() { SharedBorderOwnershipStep::Segment(segment) }
        else { match step { SharedBorderResolutionStep::Pending => SharedBorderOwnershipStep::Pending, SharedBorderResolutionStep::Complete => SharedBorderOwnershipStep::Complete, SharedBorderResolutionStep::Segment(_) => unreachable!("Ownership-only cursor") } };
        (step, progress)
    }
}

/// 🧮 Resolves a bounded caller-owned batch using the same incremental sweep as retained paint.
pub fn resolve_shared_border_segments(segments: &[SharedBorderSegment]) -> Result<Vec<ResolvedSharedBorderSegment>, SharedBorderError> {
    let mut cursor = SharedBorderResolutionCursor::new();
    for &segment in segments { cursor.push(segment)?; }
    let mut output = Vec::new();
    loop {
        match cursor.step() {
            SharedBorderResolutionStep::Pending => {},
            SharedBorderResolutionStep::Segment(segment) => output.push(segment),
            SharedBorderResolutionStep::Complete => return Ok(output),
        }
    }
}

#[cfg(test)]
#[path = "segments.test.rs"]
mod tests;
