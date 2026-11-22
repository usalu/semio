//! 🫥️ Group alpha: CSS `opacity` as a real group. Every layer from the begin layer up to the next layer outside the group carries the innermost group (groups nest to [`GROUP_ALPHA_DEPTH`] levels; a layer back in the parent after an inner group carries the parent); the prepared encoder paints those layers (and their glass and shared borders) into one pooled offscreen target and composites it once with the group alpha, so overlapping children are not individually faded. See [`super`].

use super::*;

/// 🪜️ How deep group-alpha groups nest; a deeper group is refused like a full-alpha one.
pub const GROUP_ALPHA_DEPTH: usize = 4;

/// 🫥️ The group a layer belongs to: its alpha, the logical clip its composite covers, whether this layer opens it and its nesting level (one for an outermost group).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GroupAlpha {
    pub alpha: f32,
    pub bounds: ScissorRect,
    pub begin: bool,
    pub depth: u8,
}

impl GroupAlpha {
    pub fn continued(self) -> Self {
        Self { begin: false, ..self }
    }
}

/// 🔖️ Proof that [`DrawList::begin_group_alpha`] opened a group; closing a refused one does nothing.
#[derive(Clone, Copy, Debug)]
pub struct GroupAlphaMark(bool);

impl DrawList {
    /// 🫥️ The innermost open group, the one every new layer joins.
    pub(super) fn active_group(&self) -> Option<GroupAlpha> {
        self.group_stack.iter().rev().flatten().next().copied()
    }

    /// 🫥️ Opens a group over `bounds` (clipped to the active scissor and the enclosing group) painted at `alpha`. Groups nest up to [`GROUP_ALPHA_DEPTH`] levels, and an alpha of one or more is no group.
    pub fn begin_group_alpha(&mut self, bounds: semio_ui_theme::geometry::Rect, alpha: f32) -> GroupAlphaMark {
        let parent = self.active_group();
        let level = self.group_stack.iter().flatten().count();
        if level >= GROUP_ALPHA_DEPTH || !alpha.is_finite() || alpha >= 1.0 || !self.claim_retained_output(1, size_of::<DrawLayer>()) {
            return GroupAlphaMark(false);
        }
        let mut clip = ScissorRect::from_rect(bounds, self.screen_h);
        if let Some(scissor) = self.scissor_stack.last() {
            clip = scissor.intersect(&clip);
        }
        if let Some(parent) = parent {
            clip = parent.bounds.intersect(&clip);
        }
        let group = GroupAlpha { alpha: alpha.max(0.0), bounds: clip, begin: false, depth: level as u8 + 1 };
        self.group_stack[level] = Some(group);
        self.layers.push(DrawLayer { scissor: self.scissor_stack.last().copied(), clip: self.clip_stack.last().cloned(), foreground_of: self.active_foreground_of(), group: Some(GroupAlpha { begin: true, ..group }), ..DrawLayer::default() });
        GroupAlphaMark(true)
    }

    /// 🫥️ Closes the innermost group `mark` opened: later paint lands in a layer outside it, back in the enclosing group when there is one.
    pub fn end_group_alpha(&mut self, mark: GroupAlphaMark) {
        let Some(level) = self.group_stack.iter().rposition(Option::is_some).filter(|_| mark.0) else { return };
        self.group_stack[level] = None;
        if !self.claim_retained_output(1, size_of::<DrawLayer>()) {
            return;
        }
        self.layers.push(DrawLayer { scissor: self.scissor_stack.last().copied(), clip: self.clip_stack.last().cloned(), foreground_of: self.active_foreground_of(), group: self.active_group().map(GroupAlpha::continued), ..DrawLayer::default() });
    }
}
