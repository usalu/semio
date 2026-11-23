//! 🖌️ Semantic frame edges resolved before emission: the outline plus the chips' inner edges, painted inward.

use super::*;
use super::shared_borders::{resolve_border_segments, BorderPoint, BorderSegment, BorderState};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrameBorderEdge {
    pub from: BorderPoint,
    pub to: BorderPoint,
    pub inward: f32,
}

impl FrameBorderEdge {
    pub fn paint_rect(&self, stroke: f32) -> [f32; 4] {
        let horizontal = self.from.y == self.to.y;
        let inset = if self.inward < 0.0 { -stroke } else { 0.0 };
        if horizontal {
            [self.from.x as f32, self.from.y as f32 + inset, (self.to.x - self.from.x) as f32, stroke]
        } else {
            [self.from.x as f32 + inset, self.from.y as f32, stroke, (self.to.y - self.from.y) as f32]
        }
    }
}

/// 🔲 Projects the frame outline plus the inner edges of every chip, the only edges that do not lie on the outline.
pub fn window_frame_border_edges(frame: &WindowFrame) -> Vec<FrameBorderEdge> {
    let b = frame.bounds;
    let mut edges = Vec::new();
    let mut emit = |x1: f32, y1: f32, x2: f32, y2: f32, inward: f32| {
        if x2 > x1 || y2 > y1 {
            edges.push(FrameBorderEdge { from: BorderPoint { x: x1 as f64, y: y1 as f64 }, to: BorderPoint { x: x2 as f64, y: y2 as f64 }, inward });
        }
    };
    emit(b.x, b.y, b.x + b.w, b.y, 1.0);
    emit(b.x, b.y + b.h, b.x + b.w, b.y + b.h, -1.0);
    emit(b.x, b.y, b.x, b.y + b.h, 1.0);
    emit(b.x + b.w, b.y, b.x + b.w, b.y + b.h, -1.0);
    for region in frame.chip_regions() {
        let r = region.rect;
        let (baseline, baseline_inward) = match region.dock {
            WindowFrameDock::Top => (r.y + r.h, 1.0),
            WindowFrameDock::Bottom => (r.y, -1.0),
        };
        if (baseline - b.y).abs() > 0.5 && (baseline - (b.y + b.h)).abs() > 0.5 {
            emit(r.x, baseline, r.x + r.w, baseline, baseline_inward);
        }
        if r.x - b.x > 0.5 {
            emit(r.x, r.y, r.x, r.y + r.h, 1.0);
        }
        if b.x + b.w - (r.x + r.w) > 0.5 {
            emit(r.x + r.w, r.y, r.x + r.w, r.y + r.h, -1.0);
        }
    }
    edges
}

pub(super) fn paint_dock_borders(state: &DockState, ctx: &mut DockRenderContext<'_>, bounds: Rect) {
    let mut segments = Vec::new();
    let mut paints = HashMap::new();
    for (path, rect, _) in state.stack_frame_rects(bounds) {
        let Some(DockNode::Stack { windows, .. }) = node_at(&state.root, &path) else { continue };
        if windows.is_empty() {
            continue;
        }
        let layout = layout_stack_cap(windows, ctx.window_labels, ctx.window_icon_ids, ctx.atlas, ctx.theme, rect, state.tab_action_count());
        let frame = stack_window_frame(rect, ctx.theme, &layout);
        let hovered = frame.contains(ctx.input.pointer_x, ctx.input.pointer_y);
        let active = state.active_stack.as_deref() == Some(path.as_slice());
        let border_state = if hovered { BorderState::Hover } else if active { BorderState::Active } else { BorderState::Normal };
        let color = if hovered { ctx.theme.border_emphasized } else if active { ctx.theme.selected } else { ctx.theme.border_normal };
        let owner = path_str(&path);
        for (index, edge) in window_frame_border_edges(&frame).into_iter().enumerate() {
            let id = format!("{owner}:{index}");
            segments.push(BorderSegment { id: id.clone(), owner: owner.clone(), from: edge.from, to: edge.to, state: border_state });
            paints.insert(id, (edge.inward, color));
        }
    }
    if let Ok(resolved) = resolve_border_segments(&segments) {
        for border in resolved {
            let (inward, color) = paints[&border.segment.id];
            let edge = FrameBorderEdge { from: border.segment.from, to: border.segment.to, inward };
            ctx.draw.push_solid(edge.paint_rect(ctx.theme.stroke_hairline), color);
        }
    }
}
