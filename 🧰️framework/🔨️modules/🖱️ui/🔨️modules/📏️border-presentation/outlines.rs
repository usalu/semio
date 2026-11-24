//! 🫥 State outlines: a hovered box emphasises the lines that lie on its perimeter without the hover and adds none, a box in any other state paints its whole perimeter.

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

/// 🖍️ Whether `state` closes the whole perimeter of its box. Hover does not: it emphasises the lines that lie on the perimeter without the hover and never adds one.
pub const fn border_state_closes(state: SharedBorderState) -> bool { !matches!(state, SharedBorderState::Normal | SharedBorderState::Hover) }

fn drawn(entry: &BoxOutlineInput) -> [bool; BOX_SIDE_COUNT] { if border_state_closes(entry.state) { [true; BOX_SIDE_COUNT] } else { entry.edges } }

fn sides(rect: [f64; 4]) -> [([f64; 2], [f64; 2]); BOX_SIDE_COUNT] {
    let [left, top, right, bottom] = rect;
    [([left, top], [right, top]), ([right, top], [right, bottom]), ([left, bottom], [right, bottom]), ([left, top], [left, bottom])]
}

fn flush(rect: [f64; 4], parent: [f64; 4], side: usize) -> bool { rect[[1, 2, 3, 0][side]] == parent[[1, 2, 3, 0][side]] }

fn collect(boxes: &[BoxOutlineInput], emphasis: bool) -> Result<Vec<SharedBorderSegment>, BoxOutlineError> {
    let mut output = Vec::with_capacity(box_outline_allocation_byte_demand(boxes.len()).ok_or(SharedBorderError::CapacityOverflow)? / std::mem::size_of::<SharedBorderSegment>());
    for entry in boxes {
        let parent = match entry.parent { Some(id) => Some(boxes.iter().find(|candidate| candidate.id == id).ok_or(BoxOutlineError::UnknownParent)?), None => None };
        let parent_drawn = parent.map(drawn);
        for (side, (from, to)) in sides(entry.rect).into_iter().enumerate() {
            let suppressed = !border_state_closes(entry.state) && parent.zip(parent_drawn).is_some_and(|(parent, parent_drawn)| !parent_drawn[side] && flush(entry.rect, parent.rect, side));
            let painted = drawn(entry)[side] && !suppressed;
            if if emphasis { !painted && entry.state == SharedBorderState::Hover } else { painted } { output.push(SharedBorderSegment { id: box_segment_id(entry.id, side), owner: entry.owner, from, to, state: entry.state }); }
        }
    }
    Ok(output)
}

/// 🧱 Source segments of every drawn side; a normal or hovered box drops sides flush on an edge its parent does not draw, a box in any other state draws all four.
pub fn box_outline_source_segments(boxes: &[BoxOutlineInput]) -> Result<Vec<SharedBorderSegment>, BoxOutlineError> { collect(boxes, false) }

/// 🖍️ Emphasis segments: the sides a hovered box does not draw. They take the hover state onto the lines of other boxes they lie on and paint nothing where no other segment lies.
pub fn box_outline_emphasis_segments(boxes: &[BoxOutlineInput]) -> Result<Vec<SharedBorderSegment>, BoxOutlineError> { collect(boxes, true) }

/// 🖼️ Resolved outline: on every collinear overlap the highest-priority state wins, so the perimeter of a box whose state closes it is always covered and a hovered box recolours only lines that are drawn without the hover.
pub fn box_outline_segments(boxes: &[BoxOutlineInput]) -> Result<Vec<ResolvedSharedBorderSegment>, BoxOutlineError> {
    let (mut sources, emphasis) = (box_outline_source_segments(boxes)?, box_outline_emphasis_segments(boxes)?);
    sources.extend(emphasis.iter().copied());
    let mut resolved = resolve_shared_border_segments(&sources)?;
    resolved.retain(|span| !span.contributors.iter().all(|id| emphasis.iter().any(|segment| segment.id == *id)));
    Ok(resolved)
}
