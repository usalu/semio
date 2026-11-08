//! ✂️ Incremental projection of clipped inset border paint onto logical edges.

use super::{PagedListError, PagedVec, ScissorRect, SharedBorderSegment};
use semio_framework_job::RetainedCloneProgress;

/// 🧭 The side whose stroke is inset into its owning rectangle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SharedBorderSide { Top, Right, Bottom, Left }

/// ⏩ One bounded clipping operation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SharedBorderClipStep { Pending, Segment(SharedBorderSegment), Complete }

/// 🪟 Owns clipping pieces and incrementally merges their projected intervals.
#[derive(Clone)]
pub struct SharedBorderClipCursor {
    segment: SharedBorderSegment,
    stroke: f64,
    side: SharedBorderSide,
    scissor: Option<ScissorRect>,
    clipped: bool,
    pieces: PagedVec<ScissorRect>,
    scan: usize,
    phase: u8,
    cursor: f64,
    best: Option<(f64, f64)>,
    expanded: bool,
    progress: RetainedCloneProgress,
}

impl SharedBorderClipCursor {
    /// 🌱 Starts a clipped edge; seed each clip-union rectangle before stepping.
    pub fn new(segment: SharedBorderSegment, stroke: f32, side: SharedBorderSide, scissor: Option<ScissorRect>, clipped: bool) -> Self {
        let axis = usize::from(matches!(side, SharedBorderSide::Left | SharedBorderSide::Right));
        let cursor = segment.from[axis].min(segment.to[axis]);
        Self { segment, stroke: stroke as f64, side, scissor, clipped, pieces: PagedVec::new(), scan: 0, phase: 0, cursor, best: None, expanded: false, progress: RetainedCloneProgress::default() }
    }

    /// 🧾 Returns and clears the last operation’s exact retained-work receipt.
    pub fn take_progress(&mut self) -> RetainedCloneProgress { std::mem::take(&mut self.progress) }

    /// 🧮 Reports bytes retained by the clip-piece backing allocation.
    pub fn retained_capacity_bytes(&self) -> usize { self.pieces.allocated_bytes() }

    /// 📦 Reports bytes needed before appending one clip piece.
    pub fn next_push_allocation_byte_demand(&self) -> Result<usize, PagedListError> { self.pieces.next_allocation_bytes() }

    /// 📋 Reports payload bytes copied by the next reservation or append.
    pub fn next_push_copy_byte_demand(&self) -> Result<usize, PagedListError> {
        if self.pieces.has_reserved_slot() { Ok(std::mem::size_of::<ScissorRect>()) } else { self.pieces.next_reserve_copy_byte_demand() }
    }

    /// 🧭 Reports traversal depth needed for the next reservation or placement.
    pub fn next_push_depth_demand(&self) -> Result<usize, PagedListError> {
        if self.pieces.has_reserved_slot() { self.pieces.next_push_depth_demand() } else { self.pieces.next_reserve_depth_demand() }
    }

    /// 📉 Reservation never replaces an existing allocation.
    pub fn next_push_release_byte_demand(&self) -> usize { 0 }

    /// 📥 Admits one backing page step or places the clip when a slot is ready.
    pub fn push_clip(&mut self, clip: ScissorRect) -> Result<bool, ()> {
        assert_eq!(self.phase, 0);
        self.progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        if !self.pieces.has_reserved_slot() {
            let demand = self.pieces.next_allocation_bytes().map_err(|_| ())?;
            return match self.pieces.reserve_one_cold(demand) {
                Ok(progress) => { self.progress.retained_capacity_bytes = progress.allocated_bytes; self.progress.copied_bytes = progress.placed_bytes; Ok(false) },
                Err(error) => { self.progress.retained_capacity_bytes = error.allocated_bytes; Err(()) },
            };
        }
        self.pieces.push_reserved(clip).map_err(|_| ())?;
        self.progress.copied_bytes = std::mem::size_of::<ScissorRect>();
        Ok(true)
    }

    /// 📏 A resolution step itself allocates no storage.
    pub fn next_step_allocation_byte_demand(&self) -> usize { 0 }

    fn interval(&self, clip: Option<ScissorRect>) -> Option<(f64, f64)> {
        let vertical = matches!(self.side, SharedBorderSide::Left | SharedBorderSide::Right);
        let axis = usize::from(vertical);
        let normal = 1 - axis;
        let fixed = self.segment.from[normal];
        let inward_positive = matches!(self.side, SharedBorderSide::Top | SharedBorderSide::Left);
        let (normal_lo, normal_hi) = if inward_positive { (fixed, fixed + self.stroke) } else { (fixed - self.stroke, fixed) };
        let mut lo = self.segment.from[axis].min(self.segment.to[axis]);
        let mut hi = self.segment.from[axis].max(self.segment.to[axis]);
        for rect in [self.scissor, clip].into_iter().flatten() {
            let (left, top, right, bottom) = (rect.x as f64, rect.y as f64, rect.x as f64 + rect.w as f64, rect.y as f64 + rect.h as f64);
            let (a, b, c, d) = if vertical { (top, bottom, left, right) } else { (left, right, top, bottom) };
            if normal_hi <= c || normal_lo >= d { return None; }
            lo = lo.max(a);
            hi = hi.min(b);
        }
        (lo < hi).then_some((lo, hi))
    }

    /// 🧭 Reports traversal depth needed for the next clipped-piece scan.
    pub fn next_step_depth_demand(&self) -> Result<usize, PagedListError> {
        if self.clipped && self.phase != 3 && self.scan < self.pieces.len() { self.pieces.next_get_depth_demand(self.scan) } else { Ok(0) }
    }

    /// 📋 Reports payload bytes emitted by the next bounded scan.
    pub fn next_step_copy_byte_demand(&self) -> usize {
        let count = if self.clipped { self.pieces.len() } else { 1 };
        if self.phase == 2 && self.scan >= count && !self.expanded { std::mem::size_of::<SharedBorderSegment>() } else { 0 }
    }

    /// 🧩 Scans one clip piece per step and returns disjoint spans with the original identity.
    pub fn step(&mut self) -> SharedBorderClipStep {
        self.progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        if self.phase == 3 { return SharedBorderClipStep::Complete; }
        if self.phase == 0 { self.phase = 1; }
        let count = if self.clipped { self.pieces.len() } else { 1 };
        if self.scan < count {
            let interval = self.interval(if self.clipped { self.pieces.get(self.scan).copied() } else { None });
            self.scan += 1;
            if let Some((lo, hi)) = interval {
                if hi > self.cursor {
                    let lo = lo.max(self.cursor);
                    if self.phase == 1 {
                        if self.best.is_none_or(|(best, _)| lo < best) { self.best = Some((lo, hi)); }
                    } else if let Some((start, end)) = self.best {
                        if lo <= end && hi > end { self.best = Some((start, hi)); self.expanded = true; }
                    }
                }
            }
            return SharedBorderClipStep::Pending;
        }
        if self.phase == 1 {
            if self.best.is_none() { self.phase = 3; return SharedBorderClipStep::Complete; }
            self.phase = 2; self.scan = 0;
            return SharedBorderClipStep::Pending;
        }
        if self.expanded { self.expanded = false; self.scan = 0; return SharedBorderClipStep::Pending; }
        let (lo, hi) = self.best.take().unwrap();
        let axis = usize::from(matches!(self.side, SharedBorderSide::Left | SharedBorderSide::Right));
        let mut segment = self.segment;
        segment.from[axis] = lo; segment.to[axis] = hi;
        self.cursor = hi; self.phase = 1; self.scan = 0;
        self.progress.copied_bytes = std::mem::size_of::<SharedBorderSegment>();
        SharedBorderClipStep::Segment(segment)
    }

    /// 🗑️ Releases one owned clip piece per call.
    pub fn retire_step(&mut self) -> Result<bool, PagedListError> {
        self.progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        if self.pieces.pop().is_some() { return Ok(false); }
        if self.pieces.terminal_is_empty() { return Ok(true); }
        let demand = self.pieces.next_release_allocation_bytes()?;
        let progress = self.pieces.release_empty_page_leased(demand)?;
        self.progress.released_bytes = progress.released_allocation_bytes;
        Ok(self.pieces.terminal_is_empty())
    }

    /// 🧭 Reports traversal depth needed for the next retirement step.
    pub fn next_retirement_depth_demand(&self) -> Result<usize, PagedListError> {
        if !self.pieces.is_empty() { self.pieces.next_pop_depth_demand() } else { self.pieces.next_release_depth_demand() }
    }

    /// 📉 Reports the next bounded page or branch release.
    pub fn next_retirement_release_byte_demand(&self) -> Result<usize, PagedListError> {
        if !self.pieces.is_empty() || self.pieces.terminal_is_empty() { Ok(0) } else { self.pieces.next_release_allocation_bytes() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SharedBorderState;

    fn collect(mut cursor: SharedBorderClipCursor) -> Vec<SharedBorderSegment> {
        let mut result = Vec::new();
        loop {
            let demand = cursor.next_step_copy_byte_demand();
            let step = cursor.step();
            assert_eq!(cursor.take_progress().copied_bytes, demand);
            match step { SharedBorderClipStep::Pending => {}, SharedBorderClipStep::Segment(segment) => result.push(segment), SharedBorderClipStep::Complete => break }
        }
        result
    }

    #[test]
    fn clips_and_merges_overlapping_projections() {
        let segment = SharedBorderSegment { id: 3, owner: 2, from: [0.0, 10.0], to: [20.0, 10.0], state: SharedBorderState::Hover };
        let fixture: serde_json::Value = serde_json::from_str(include_str!("cases.json")).unwrap();
        let rect = |value: &serde_json::Value| ScissorRect { x: value[0].as_u64().unwrap() as u32, y: value[1].as_u64().unwrap() as u32, w: value[2].as_u64().unwrap() as u32, h: value[3].as_u64().unwrap() as u32 };
        let mut cursor = SharedBorderClipCursor::new(segment, 4.0, SharedBorderSide::Top, Some(rect(&fixture["scissor"])), true);
        for piece in fixture["pieces"].as_array().unwrap() { while !cursor.push_clip(rect(piece)).unwrap() {} }
        let result = collect(cursor);
        let expected: Vec<_> = fixture["expected"].as_array().unwrap().iter().map(|pair| (pair[0].as_f64().unwrap(), pair[1].as_f64().unwrap())).collect();
        assert_eq!(result.iter().map(|s| (s.from[0], s.to[0])).collect::<Vec<_>>(), expected);
        assert!(result.iter().all(|s| s.id == 3 && s.owner == 2 && s.state == SharedBorderState::Hover));
    }

    #[test]
    fn empty_union_and_inset_normal_are_respected() {
        let segment = SharedBorderSegment { id: 1, owner: 1, from: [5.0, 0.0], to: [5.0, 10.0], state: SharedBorderState::Normal };
        assert!(collect(SharedBorderClipCursor::new(segment, 1.0, SharedBorderSide::Right, None, true)).is_empty());
        let mut cursor = SharedBorderClipCursor::new(segment, 1.0, SharedBorderSide::Right, None, true);
        while !cursor.push_clip(ScissorRect { x: 5, y: 0, w: 3, h: 10 }).unwrap() {}
        assert!(collect(cursor).is_empty());
        assert_eq!(collect(SharedBorderClipCursor::new(segment, 1.0, SharedBorderSide::Right, None, false)), vec![segment]);
    }
    #[test]
    fn clip_allocation_preflight_tracks_growth_and_retirement() {
        let segment = SharedBorderSegment { id: 1, owner: 1, from: [0.0, 0.0], to: [10.0, 0.0], state: SharedBorderState::Normal };
        let mut cursor = SharedBorderClipCursor::new(segment, 1.0, SharedBorderSide::Top, None, true);
        let rect = ScissorRect { x: 0, y: 0, w: 1, h: 1 };
        for _ in 0..100 {
            let capacity = cursor.next_push_allocation_byte_demand().unwrap();
            let copy = cursor.next_push_copy_byte_demand().unwrap();
            cursor.push_clip(rect).unwrap();
            let progress = cursor.take_progress();
            assert_eq!(progress.copied_items, 1);
            assert!(progress.copied_bytes <= copy);
            assert!(progress.retained_capacity_bytes <= capacity);
            assert_eq!(progress.released_bytes, 0);
        }
        let mut released = 0;
        loop {
            let demand = cursor.next_retirement_release_byte_demand().unwrap();
            let complete = cursor.retire_step().unwrap();
            let progress = cursor.take_progress();
            assert_eq!(progress.released_bytes, demand);
            released += progress.released_bytes;
            if complete { break; }
        }
        assert!(released > 0);
        assert_eq!(cursor.retained_capacity_bytes(), 0);
    }

    #[test]
    fn large_clip_union_stays_within_fixed_worker_grants() {
        let segment = SharedBorderSegment { id: 1, owner: 1, from: [0.0, 0.0], to: [100.0, 0.0], state: SharedBorderState::Normal };
        let mut cursor = SharedBorderClipCursor::new(segment, 1.0, SharedBorderSide::Top, None, true);
        let rect = ScissorRect { x: 0, y: 0, w: 1, h: 1 };
        for _ in 0..10_000 {
            loop {
                let capacity = cursor.next_push_allocation_byte_demand().unwrap();
                let copy = cursor.next_push_copy_byte_demand().unwrap();
                assert!(capacity <= 64 * 1024 && copy <= 64 * 1024);
                let done = cursor.push_clip(rect).unwrap();
                let progress = cursor.take_progress();
                assert!(progress.retained_capacity_bytes <= capacity && progress.copied_bytes <= copy);
                if done { break; }
            }
        }
        assert!(cursor.retained_capacity_bytes() > 64 * 1024);
        loop {
            let release = cursor.next_retirement_release_byte_demand().unwrap();
            assert!(release <= 262 * 1024);
            let done = cursor.retire_step().unwrap();
            assert!(cursor.take_progress().released_bytes <= release);
            if done { break; }
        }
    }

}
