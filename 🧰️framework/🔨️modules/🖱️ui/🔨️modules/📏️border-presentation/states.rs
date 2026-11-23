//! 🫧 State ownership of controls and enclosing outlines.

use super::SharedBorderState;

/// 🏷️ Own state markers of one box as a bit set over [`SharedBorderState`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BoxStateMarks(u16);

impl BoxStateMarks {
    /// 🌱 No markers.
    pub const NONE: Self = Self(0);

    /// ➕ The set extended with one marker; `Normal` is not a marker.
    pub const fn with(self, state: SharedBorderState) -> Self { Self(self.0 | (1 << state as u16)) }

    /// ❓ Whether the marker is set.
    pub const fn contains(self, state: SharedBorderState) -> bool { self.0 & (1 << state as u16) != 0 }
}

/// 📦 One box of the state tree; `id` is the caller's stable identity and `parent` names another box's `id`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoxStateInput<Id> {
    pub id: Id,
    pub parent: Option<Id>,
    pub interactive: bool,
    pub hover_within: bool,
    pub disabled: bool,
    pub marks: BoxStateMarks,
}

/// 🛑 The box tree cannot be resolved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoxStateError { DuplicateId, UnknownBox, Cycle, CapacityOverflow }

impl std::fmt::Display for BoxStateError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::DuplicateId => "Duplicate box identity",
            Self::UnknownBox => "Box names an unknown parent or target",
            Self::Cycle => "Box parents form a cycle",
            Self::CapacityOverflow => "Box state storage extent overflow",
        })
    }
}

impl std::error::Error for BoxStateError {}

/// 🥇 Final state of one box from its own markers and the pointer and keyboard ownership it won.
pub fn own_box_state(marks: BoxStateMarks, hovered: bool, focused: bool) -> SharedBorderState {
    let marks = if hovered { marks.with(SharedBorderState::Hover) } else { marks };
    let marks = if focused { marks.with(SharedBorderState::Focus) } else { marks };
    [SharedBorderState::Hover, SharedBorderState::Focus, SharedBorderState::Celebrated, SharedBorderState::Introduced, SharedBorderState::Loading, SharedBorderState::Waiting, SharedBorderState::Invalid, SharedBorderState::Active]
        .into_iter()
        .find(|&state| marks.contains(state))
        .unwrap_or(SharedBorderState::Normal)
}

/// 🧮 Bytes [`resolve_box_states`] allocates for `count` boxes: the result and the id index.
pub fn box_state_allocation_byte_demand<Id>(count: usize) -> Result<usize, BoxStateError> {
    count.checked_mul(std::mem::size_of::<SharedBorderState>() + std::mem::size_of::<(Id, usize)>()).ok_or(BoxStateError::CapacityOverflow)
}

fn index_of<Id: Copy + Ord>(index: &[(Id, usize)], id: Id) -> Option<usize> { index.binary_search_by_key(&id, |entry| entry.0).ok().map(|slot| index[slot].1) }

fn innermost<Id: Copy + Ord>(boxes: &[BoxStateInput<Id>], index: &[(Id, usize)], start: Option<Id>, accepts: impl Fn(&BoxStateInput<Id>) -> bool) -> Result<Option<usize>, BoxStateError> {
    let mut cursor = match start { Some(id) => Some(index_of(index, id).ok_or(BoxStateError::UnknownBox)?), None => None };
    for _ in 0..=boxes.len() {
        let Some(at) = cursor else { return Ok(None) };
        if accepts(&boxes[at]) { return Ok(Some(at)); }
        cursor = match boxes[at].parent { Some(parent) => Some(index_of(index, parent).ok_or(BoxStateError::UnknownBox)?), None => None };
    }
    Err(BoxStateError::Cycle)
}

/// 🏆 Resolves local keyboard and marked states, plus hover on the innermost enabled control and opted-in outlines in its pointer path.
pub fn resolve_box_states<Id: Copy + Ord>(boxes: &[BoxStateInput<Id>], pointer: Option<Id>, focus_visible: Option<Id>) -> Result<Vec<SharedBorderState>, BoxStateError> {
    box_state_allocation_byte_demand::<Id>(boxes.len())?;
    let mut index: Vec<(Id, usize)> = boxes.iter().enumerate().map(|(slot, entry)| (entry.id, slot)).collect();
    index.sort_unstable();
    if index.windows(2).any(|pair| pair[0].0 == pair[1].0) { return Err(BoxStateError::DuplicateId); }
    let focused = innermost(boxes, &index, focus_visible, |entry| entry.interactive)?;
    let mut states: Vec<_> = boxes.iter().enumerate().map(|(slot, entry)| own_box_state(entry.marks, false, focused == Some(slot))).collect();
    let mut cursor = pointer;
    let mut has_control = false;
    for _ in 0..=boxes.len() {
        let Some(id) = cursor else { return Ok(states) };
        let slot = index_of(&index, id).ok_or(BoxStateError::UnknownBox)?;
        let entry = &boxes[slot];
        if !entry.disabled {
            if entry.hover_within || (!has_control && entry.interactive) { states[slot] = SharedBorderState::Hover; }
            has_control |= entry.interactive;
        }
        cursor = entry.parent;
    }
    Err(BoxStateError::Cycle)
}
