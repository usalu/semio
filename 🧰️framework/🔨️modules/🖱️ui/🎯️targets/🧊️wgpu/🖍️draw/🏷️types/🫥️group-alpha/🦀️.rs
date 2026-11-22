//! 🫥️ Group alpha: CSS `opacity` as a real group. Every layer from the begin layer up to the next layer outside the group carries the group; the prepared encoder paints those layers (and their glass and shared borders) into one pooled offscreen target and composites it once with the group alpha, so overlapping children are not individually faded. See [`super`].

use super::*;

/// 🫥️ The group a layer belongs to: its alpha, the logical clip its composite covers, and whether this layer opens it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GroupAlpha {
    pub alpha: f32,
    pub bounds: ScissorRect,
    pub begin: bool,
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
    /// 🫥️ Opens a group over `bounds` (clipped to the active scissor) painted at `alpha`. Groups do not nest, and an alpha of one or more is no group.
    pub fn begin_group_alpha(&mut self, bounds: semio_ui_theme::geometry::Rect, alpha: f32) -> GroupAlphaMark {
        if self.active_group.is_some() || !alpha.is_finite() || alpha >= 1.0 || !self.claim_retained_output(1, size_of::<DrawLayer>()) {
            return GroupAlphaMark(false);
        }
        let mut clip = ScissorRect::from_rect(bounds, self.screen_h);
        if let Some(parent) = self.scissor_stack.last() {
            clip = parent.intersect(&clip);
        }
        let group = GroupAlpha { alpha: alpha.max(0.0), bounds: clip, begin: false };
        self.active_group = Some(group);
        self.layers.push(DrawLayer { scissor: self.scissor_stack.last().copied(), clip: self.clip_stack.last().cloned(), foreground_of: self.active_foreground_of(), group: Some(GroupAlpha { begin: true, ..group }), ..DrawLayer::default() });
        GroupAlphaMark(true)
    }

    /// 🫥️ Closes the group `mark` opened: later paint lands in a layer outside it.
    pub fn end_group_alpha(&mut self, mark: GroupAlphaMark) {
        if !mark.0 || self.active_group.take().is_none() || !self.claim_retained_output(1, size_of::<DrawLayer>()) {
            return;
        }
        self.layers.push(DrawLayer { scissor: self.scissor_stack.last().copied(), clip: self.clip_stack.last().cloned(), foreground_of: self.active_foreground_of(), ..DrawLayer::default() });
    }
}
