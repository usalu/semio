//! 🖌️ Semantic silhouette edges resolved before emission, preserving inward paint and chip cutouts.

use super::*;
use super::shared_borders::{resolve_border_segments, BorderPoint, BorderSegment, BorderState};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SilhouetteBorderEdge {
    pub from: BorderPoint,
    pub to: BorderPoint,
    pub inward: f32,
}

impl SilhouetteBorderEdge {
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

/// 🔲 Projects every silhouette path edge without changing the body, cap or chip geometry.
pub fn window_silhouette_border_edges(silhouette: &WindowSilhouette) -> Vec<SilhouetteBorderEdge> {
    let b = silhouette.bounds;
    let mut edges = Vec::new();
    let mut emit = |x1: f32, y1: f32, x2: f32, y2: f32, inward: f32| {
        if x2 > x1 || y2 > y1 {
            edges.push(SilhouetteBorderEdge { from: BorderPoint { x: x1 as f64, y: y1 as f64 }, to: BorderPoint { x: x2 as f64, y: y2 as f64 }, inward });
        }
    };
    for (edge, outer, inner, inward) in [(&silhouette.top, b.y, b.y + silhouette.top.depth, 1.0), (&silhouette.bottom, b.y + b.h, b.y + b.h - silhouette.bottom.depth, -1.0)] {
        let baseline_inward = inward;
        let mut cursor = 0.0;
        for span in &edge.spans {
            if span.left > cursor + 0.5 {
                emit(b.x + cursor, inner, b.x + span.left, inner, baseline_inward);
            }
            emit(b.x + span.left, outer, b.x + span.right, outer, inward);
            if span.left > 0.5 {
                emit(b.x + span.left, outer.min(inner), b.x + span.left, outer.max(inner), 1.0);
            }
            if span.right < b.w - 0.5 {
                emit(b.x + span.right, outer.min(inner), b.x + span.right, outer.max(inner), -1.0);
            }
            cursor = span.right;
        }
        if cursor < b.w - 0.5 {
            emit(b.x + cursor, inner, b.x + b.w, inner, baseline_inward);
        }
    }
    let left_top = if silhouette.top.spans.first().is_some_and(|span| span.left <= 0.5) { b.y } else { b.y + silhouette.top.depth };
    let left_bottom = if silhouette.bottom.spans.first().is_some_and(|span| span.left <= 0.5) { b.y + b.h } else { b.y + b.h - silhouette.bottom.depth };
    let right_top = if silhouette.top.spans.last().is_some_and(|span| span.right >= b.w - 0.5) { b.y } else { b.y + silhouette.top.depth };
    let right_bottom = if silhouette.bottom.spans.last().is_some_and(|span| span.right >= b.w - 0.5) { b.y + b.h } else { b.y + b.h - silhouette.bottom.depth };
    emit(b.x, left_top, b.x, left_bottom, 1.0);
    emit(b.x + b.w, right_top, b.x + b.w, right_bottom, -1.0);
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
        let silhouette = stack_window_silhouette(rect, ctx.theme, &layout);
        let hovered = silhouette.contains(ctx.input.pointer_x, ctx.input.pointer_y);
        let active = state.active_stack.as_deref() == Some(path.as_slice());
        let border_state = if hovered { BorderState::Hover } else if active { BorderState::Active } else { BorderState::Normal };
        let color = if hovered { ctx.theme.border_emphasized } else if active { ctx.theme.selected } else { ctx.theme.border_normal };
        let owner = path_str(&path);
        for (index, edge) in window_silhouette_border_edges(&silhouette).into_iter().enumerate() {
            let id = format!("{owner}:{index}");
            segments.push(BorderSegment { id: id.clone(), owner: owner.clone(), from: edge.from, to: edge.to, state: border_state });
            paints.insert(id, (edge.inward, color));
        }
    }
    if let Ok(resolved) = resolve_border_segments(&segments) {
        for border in resolved {
            let (inward, color) = paints[&border.segment.id];
            let edge = SilhouetteBorderEdge { from: border.segment.from, to: border.segment.to, inward };
            ctx.draw.push_solid(edge.paint_rect(ctx.theme.stroke_hairline), color);
        }
    }
}
