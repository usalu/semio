//! 🫥 Closed state outlines: every stated box paints its whole perimeter.

use super::{resolve_shared_border_segments, ResolvedSharedBorderSegment, SharedBorderError, SharedBorderSegment, SharedBorderState};

/// 🧭 Sides in segment-identity order: top, right, bottom, left.
pub const BOX_SIDE_COUNT: usize = 4;

/// 📦 One box of the outline tree in a common paint space; `rect` is `[left, top, right, bottom]` and `edges` the sides it declares in top, right, bottom, left order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoxOutlineInput {
    pub id: usize,
    pub parent: Option<usize>,
    pub owner: usize,
    pub rect: [f64; 4],
    pub edges: [bool; BOX_SIDE_COUNT],
    pub state: SharedBorderState,
}

/// 🛑 The box tree cannot project to segments.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoxOutlineError { UnknownParent, Border(SharedBorderError) }

impl std::fmt::Display for BoxOutlineError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self { Self::UnknownParent => formatter.write_str("Box names an unknown parent"), Self::Border(error) => error.fmt(formatter) }
    }
}

impl std::error::Error for BoxOutlineError {}

impl From<SharedBorderError> for BoxOutlineError { fn from(error: SharedBorderError) -> Self { Self::Border(error) } }

/// 🔑 Segment identity of one side of a box.
pub const fn box_segment_id(box_id: usize, side: usize) -> usize { box_id * BOX_SIDE_COUNT + side }

/// 🧮 Bytes the source segments of `count` boxes occupy.
pub fn box_outline_allocation_byte_demand(count: usize) -> Option<usize> { count.checked_mul(BOX_SIDE_COUNT)?.checked_mul(std::mem::size_of::<SharedBorderSegment>()) }

fn drawn(entry: &BoxOutlineInput) -> [bool; BOX_SIDE_COUNT] { if entry.state == SharedBorderState::Normal { entry.edges } else { [true; BOX_SIDE_COUNT] } }

fn sides(rect: [f64; 4]) -> [([f64; 2], [f64; 2]); BOX_SIDE_COUNT] {
    let [left, top, right, bottom] = rect;
    [([left, top], [right, top]), ([right, top], [right, bottom]), ([left, bottom], [right, bottom]), ([left, top], [left, bottom])]
}

fn flush(rect: [f64; 4], parent: [f64; 4], side: usize) -> bool { rect[[1, 2, 3, 0][side]] == parent[[1, 2, 3, 0][side]] }

/// 🧱 Source segments of every drawn side; a normal box drops sides flush on an edge its parent does not draw, a stated box draws all four.
pub fn box_outline_source_segments(boxes: &[BoxOutlineInput]) -> Result<Vec<SharedBorderSegment>, BoxOutlineError> {
    let mut output = Vec::with_capacity(box_outline_allocation_byte_demand(boxes.len()).ok_or(SharedBorderError::CapacityOverflow)? / std::mem::size_of::<SharedBorderSegment>());
    for entry in boxes {
        let parent = match entry.parent { Some(id) => Some(boxes.iter().find(|candidate| candidate.id == id).ok_or(BoxOutlineError::UnknownParent)?), None => None };
        let parent_drawn = parent.map(drawn);
        for (side, (from, to)) in sides(entry.rect).into_iter().enumerate() {
            let suppressed = entry.state == SharedBorderState::Normal && parent.zip(parent_drawn).is_some_and(|(parent, parent_drawn)| !parent_drawn[side] && flush(entry.rect, parent.rect, side));
            if drawn(entry)[side] && !suppressed { output.push(SharedBorderSegment { id: box_segment_id(entry.id, side), owner: entry.owner, from, to, state: entry.state }); }
        }
    }
    Ok(output)
}

/// 🖼️ Resolved outline: on every collinear overlap the highest-priority state wins, so a stated box's perimeter is always covered.
pub fn box_outline_segments(boxes: &[BoxOutlineInput]) -> Result<Vec<ResolvedSharedBorderSegment>, BoxOutlineError> {
    Ok(resolve_shared_border_segments(&box_outline_source_segments(boxes)?)?)
}
