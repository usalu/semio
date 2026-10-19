//! 🔲️ Window silhouette and stack geometry: the window silhouette spans, edges and borders, the stack cap layout, the path parsing and the stack frame and body collection. See [`super`].

use super::*;

//#region DockFreeFunctions
//#region WindowSilhouetteGeometry
/// 🪟️ Normalized physical chip span in silhouette-local coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowSilhouetteSpan {
    pub left: f32,
    pub right: f32,
}

impl WindowSilhouetteSpan {
    pub fn new(left: f32, right: f32) -> Self {
        Self { left, right }
    }
}

/// 🪟️ Normalized top or bottom chrome band.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WindowSilhouetteEdge {
    pub depth: f32,
    pub spans: Vec<WindowSilhouetteSpan>,
}

impl WindowSilhouetteEdge {
    pub fn new(depth: f32, spans: Vec<WindowSilhouetteSpan>) -> Self {
        Self { depth, spans }
    }
}

/// 🪟️ Rust mirror of `window-silhouette-geometry/v1` using arbitrary merged top and bottom spans.
#[derive(Clone, Debug, PartialEq)]
pub struct WindowSilhouette {
    pub bounds: Rect,
    pub top: WindowSilhouetteEdge,
    pub bottom: WindowSilhouetteEdge,
}

impl WindowSilhouette {
    const CHIP_EPSILON: f32 = 0.5;

    /// 🪟️ Builds and normalizes a v1 silhouette from physical top and bottom chip spans.
    pub fn new(bounds: Rect, top: WindowSilhouetteEdge, bottom: WindowSilhouetteEdge) -> Self {
        let width = bounds.w.max(0.0);
        let height = bounds.h.max(0.0);
        let top_depth = top.depth.max(0.0).min(height);
        let bottom_depth = bottom.depth.max(0.0).min((height - top_depth).max(0.0));
        Self { bounds, top: WindowSilhouetteEdge::new(top_depth, Self::normalize_spans(top.spans, width)), bottom: WindowSilhouetteEdge::new(bottom_depth, Self::normalize_spans(bottom.spans, width)) }
    }

    /// 🪟️ Projects measured top/bottom chip spans into the normalized v1 model.
    pub fn from_measured_edges(bounds: Rect, top_spans: Vec<WindowSilhouetteSpan>, bottom_spans: Vec<WindowSilhouetteSpan>, top_depth: f32, bottom_depth: f32) -> Self {
        Self::new(bounds, WindowSilhouetteEdge::new(top_depth, top_spans), WindowSilhouetteEdge::new(bottom_depth, bottom_spans))
    }

    /// 🪟️ Projects the classic top-left tabs + top-right controls pair into the normalized v1 model.
    pub fn from_measured_top(bounds: Rect, tabs_w: f32, controls_w: f32, depth: f32) -> Self {
        let tabs_right = tabs_w.max(0.0).min(bounds.w);
        let controls_left = (bounds.w - controls_w.max(0.0)).max(tabs_right).min(bounds.w);
        Self::from_measured_edges(bounds, vec![WindowSilhouetteSpan::new(0.0, tabs_right), WindowSilhouetteSpan::new(controls_left, bounds.w)], Vec::new(), depth, 0.0)
    }

    fn normalize_spans(spans: Vec<WindowSilhouetteSpan>, width: f32) -> Vec<WindowSilhouetteSpan> {
        let mut spans: Vec<_> = spans
            .into_iter()
            .filter(|span| span.left.is_finite() && span.right.is_finite() && span.right > span.left)
            .map(|span| WindowSilhouetteSpan::new(span.left.clamp(0.0, width), span.right.clamp(0.0, width)))
            .filter(|span| span.right - span.left > Self::CHIP_EPSILON)
            .collect();
        spans.sort_by(|a, b| a.left.total_cmp(&b.left).then(a.right.total_cmp(&b.right)));
        let mut merged: Vec<WindowSilhouetteSpan> = Vec::new();
        for span in spans {
            if let Some(last) = merged.last_mut() {
                if span.left <= last.right + Self::CHIP_EPSILON {
                    last.right = last.right.max(span.right);
                    continue;
                }
            }
            merged.push(span);
        }
        merged
    }

    //#region ContentClip

    /// 🪟️ Returns the disjoint body-and-chip union used by paint, content, and hit clipping.
    pub fn content_clip_rects(&self) -> Vec<Rect> {
        let mut regions = Vec::with_capacity(1 + self.top.spans.len() + self.bottom.spans.len());
        let body_h = (self.bounds.h - self.top.depth - self.bottom.depth).max(0.0);
        if self.bounds.w > Self::CHIP_EPSILON && body_h > Self::CHIP_EPSILON {
            regions.push(Rect::new(self.bounds.x, self.bounds.y + self.top.depth, self.bounds.w, body_h));
        }
        regions.extend(self.glass_regions());
        regions
    }

    /// 🪟️ Returns only chip regions, suitable for glass compositing.
    pub fn glass_regions(&self) -> Vec<Rect> {
        let mut regions = Vec::with_capacity(self.top.spans.len() + self.bottom.spans.len());
        if self.top.depth > Self::CHIP_EPSILON {
            regions.extend(self.top.spans.iter().map(|span| Rect::new(self.bounds.x + span.left, self.bounds.y, span.right - span.left, self.top.depth)));
        }
        if self.bottom.depth > Self::CHIP_EPSILON {
            let y = self.bounds.y + self.bounds.h - self.bottom.depth;
            regions.extend(self.bottom.spans.iter().map(|span| Rect::new(self.bounds.x + span.left, y, span.right - span.left, self.bottom.depth)));
        }
        regions
    }

    /// 🪟️ Returns the full content coordinate space before silhouette clipping.
    pub fn content_bounds(&self) -> Rect {
        self.bounds
    }

    /// 🪟️ Returns the chrome-safe center band for dock targets and auxiliary rails.
    pub fn safe_body_rect(&self) -> Rect {
        Rect::new(self.bounds.x, self.bounds.y + self.top.depth, self.bounds.w, (self.bounds.h - self.top.depth - self.bottom.depth).max(0.0))
    }

    /// 🪟️ Returns the normalized document-layout clearances `(top, bottom)`.
    pub fn safe_clearances(&self) -> (f32, f32) {
        (self.top.depth, self.bottom.depth)
    }

    /// 🪟️ Tests exact membership in the body-and-chip union, leaving all chrome gaps as cutouts.
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x.is_finite() && y.is_finite() && self.content_clip_rects().iter().any(|region| region.contains(x, y))
    }

    //#endregion ContentClip
}

/// 🪟️ Paints a hairline (or thicker) stroke along the dock-stack silhouette path, every segment
/// lying INSIDE the silhouette it outlines.
///
/// 🩸️ The horizontal segments were centred on their path (`edge - stroke * 0.5`) while both outer
/// verticals were already inset, so a window's cap border bled half a stroke ABOVE `bounds.y` and the
/// notch baseline half a stroke BELOW the cap — a border outside the window box, which React never
/// paints: its outline is inset by `WINDOW_SILHOUETTE_PATH_INSET` exactly so a non-scaling stroke
/// stays within the box (`🔲️WindowSilhouette/🟦️.tsx:85`, `:225`; the golden path `M1,1 H60 V25 H160
/// V1 H199 …` starts one inset in, not on the edge).
pub fn push_window_silhouette_border(draw: &mut DrawList, silhouette: &WindowSilhouette, stroke: f32, color: Rgba) {
    let b = silhouette.bounds;
    let mut paint_edge = |edge: &WindowSilhouetteEdge, outer: f32, inner: f32| {
        let downward = outer < inner;
        let outer_y = if downward { outer } else { outer - stroke };
        let inner_y = if downward { inner - stroke } else { inner };
        let wall_y = outer.min(inner);
        let wall_h = (outer - inner).abs();
        let mut cursor = 0.0;
        for span in &edge.spans {
            if span.left > cursor + WindowSilhouette::CHIP_EPSILON {
                draw.push_solid([b.x + cursor, inner_y, span.left - cursor, stroke], color);
            }
            draw.push_solid([b.x + span.left, outer_y, span.right - span.left, stroke], color);
            if span.left > WindowSilhouette::CHIP_EPSILON {
                draw.push_solid([b.x + span.left, wall_y, stroke, wall_h], color);
            }
            if span.right < b.w - WindowSilhouette::CHIP_EPSILON {
                draw.push_solid([b.x + span.right - stroke, wall_y, stroke, wall_h], color);
            }
            cursor = span.right;
        }
        if cursor < b.w - WindowSilhouette::CHIP_EPSILON {
            draw.push_solid([b.x + cursor, inner_y, b.w - cursor, stroke], color);
        }
    };
    paint_edge(&silhouette.top, b.y, b.y + silhouette.top.depth);
    paint_edge(&silhouette.bottom, b.y + b.h, b.y + b.h - silhouette.bottom.depth);
    let left_top = if silhouette.top.spans.first().is_some_and(|span| span.left <= WindowSilhouette::CHIP_EPSILON) { b.y } else { b.y + silhouette.top.depth };
    let left_bottom = if silhouette.bottom.spans.first().is_some_and(|span| span.left <= WindowSilhouette::CHIP_EPSILON) { b.y + b.h } else { b.y + b.h - silhouette.bottom.depth };
    let right_top = if silhouette.top.spans.last().is_some_and(|span| span.right >= b.w - WindowSilhouette::CHIP_EPSILON) { b.y } else { b.y + silhouette.top.depth };
    let right_bottom = if silhouette.bottom.spans.last().is_some_and(|span| span.right >= b.w - WindowSilhouette::CHIP_EPSILON) { b.y + b.h } else { b.y + b.h - silhouette.bottom.depth };
    draw.push_solid([b.x, left_top, stroke, (left_bottom - left_top).max(0.0)], color);
    draw.push_solid([b.x + b.w - stroke, right_top, stroke, (right_bottom - right_top).max(0.0)], color);
}
//#endregion WindowSilhouetteGeometry

struct StackCapTabLayout {
    pub window_id: String,
    pub label: String,
    pub(super) icon_id: String,
    pub rect: Rect,
}

struct StackCapGroupLayout {
    pub tabs: Vec<StackCapTabLayout>,
}

struct StackCapLayout {
    pub groups: Vec<StackCapGroupLayout>,
    top_spans: Vec<WindowSilhouetteSpan>,
    bottom_spans: Vec<WindowSilhouetteSpan>,
}

pub fn layout_stack_cap(windows: &[DockStackTab], labels: &HashMap<String, String>, icon_ids: &HashMap<String, String>, atlas: &mut FontAtlas, theme: &Theme, bounds: Rect, action_count: usize) -> StackCapLayout {
    let tab_h = dock_cap_depth(theme);
    let mut groups = Vec::new();
    let mut top_spans = Vec::new();
    let mut bottom_spans = Vec::new();
    for (corner, tabs) in tabs_by_corner(windows) {
        if tabs.is_empty() {
            continue;
        }
        let mut widths = Vec::with_capacity(tabs.len());
        let mut meta = Vec::with_capacity(tabs.len());
        for tab in &tabs {
            let icon_id = icon_ids.get(&tab.window_id).or_else(|| icon_ids.get(&tab.window_kind_id)).cloned().unwrap_or_else(|| "app-window".into());
            let (label, tw) = dock_tab_chip(atlas, theme, dock_tab_label(tab, labels), action_count);
            widths.push(tw);
            meta.push((tab.window_id.clone(), label, icon_id));
        }
        let total_w = widths.iter().sum::<f32>();
        let (origin_x, origin_y) = match corner {
            WindowStackCorner::TopLeft => (bounds.x, bounds.y),
            WindowStackCorner::TopRight => (bounds.x + bounds.w - total_w, bounds.y),
            WindowStackCorner::BottomLeft => (bounds.x, bounds.y + bounds.h - tab_h),
            WindowStackCorner::BottomRight => (bounds.x + bounds.w - total_w, bounds.y + bounds.h - tab_h),
        };
        let mut x = origin_x;
        let mut painted = Vec::new();
        for ((window_id, label, icon_id), tw) in meta.into_iter().zip(widths.into_iter()) {
            painted.push(StackCapTabLayout { window_id, label, icon_id, rect: Rect::new(x, origin_y, tw, tab_h) });
            x += tw;
        }
        let span = WindowSilhouetteSpan::new((origin_x - bounds.x).max(0.0), (origin_x - bounds.x + total_w).min(bounds.w));
        match corner {
            WindowStackCorner::TopLeft | WindowStackCorner::TopRight => top_spans.push(span),
            WindowStackCorner::BottomLeft | WindowStackCorner::BottomRight => bottom_spans.push(span),
        }
        groups.push(StackCapGroupLayout { tabs: painted });
    }
    StackCapLayout { groups, top_spans, bottom_spans }
}

pub fn stack_window_silhouette(bounds: Rect, theme: &Theme, layout: &StackCapLayout) -> WindowSilhouette {
    let top_depth = if layout.top_spans.is_empty() { 0.0 } else { dock_cap_depth(theme) };
    let bottom_depth = if layout.bottom_spans.is_empty() { 0.0 } else { dock_cap_depth(theme) };
    WindowSilhouette::from_measured_edges(bounds, layout.top_spans.clone(), layout.bottom_spans.clone(), top_depth, bottom_depth)
}

pub(super) fn dock_stack_body_rect(body: Rect, theme: &Theme) -> Rect {
    let inset = theme.padding_standard.min(body.w * 0.5);
    Rect::new(body.x + inset, body.y, (body.w - inset * 2.0).max(0.0), body.h)
}

pub fn path_str(path: &[usize]) -> String {
    path.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",")
}

pub fn parse_path(value: &str) -> DockPath {
    if value.is_empty() {
        return vec![];
    }
    value.split(',').filter_map(|part| part.parse().ok()).collect()
}

pub(super) fn collect_stack_frames(node: &DockNode, bounds: Rect, path: &[usize], separator: f32, out: &mut Vec<(DockPath, Rect, String)>) {
    match node {
        DockNode::Row(children) => {
            for slot in dock_axis_slots(children, bounds, true, separator) {
                let mut child_path = path.to_vec();
                child_path.push(slot.index);
                collect_stack_frames(slot.node, slot.rect, &child_path, separator, out);
            }
        }
        DockNode::Column(children) => {
            for slot in dock_axis_slots(children, bounds, false, separator) {
                let mut child_path = path.to_vec();
                child_path.push(slot.index);
                collect_stack_frames(slot.node, slot.rect, &child_path, separator, out);
            }
        }
        DockNode::Stack { active, .. } => out.push((path.to_vec(), bounds, active.clone())),
    }
}

/// 🪟️ Every stack's body rect and the window it shows, plus that window's silhouette. A stack holding
/// NO tabs answers its rect with an EMPTY window id — it is still a drop target, exactly as React's
/// `WindowChrome` keeps rendering for an empty stack — but it owns no silhouette and no window, because
/// React's `activeDescriptor` is `undefined` there and its `stackBody` renders nothing at all
/// (`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1176`/`:1212`). `ShellState::plan_dock_windows` is where that
/// empty id is dropped from the window-keyed registries.
pub(super) fn collect_stack_bodies(
    node: &DockNode,
    bounds: Rect,
    path: &[usize],
    theme: &Theme,
    window_labels: &HashMap<String, String>,
    atlas: &mut FontAtlas,
    state: &DockState,
    action_count: usize,
    out: &mut Vec<(DockPath, Rect, String)>,
    silhouettes: &mut HashMap<String, WindowSilhouette>,
) {
    match node {
        DockNode::Row(children) => {
            for slot in dock_axis_slots(children, bounds, true, theme.gap_standard) {
                let mut child_path = path.to_vec();
                child_path.push(slot.index);
                collect_stack_bodies(slot.node, slot.rect, &child_path, theme, window_labels, atlas, state, action_count, out, silhouettes);
            }
        }
        DockNode::Column(children) => {
            for slot in dock_axis_slots(children, bounds, false, theme.gap_standard) {
                let mut child_path = path.to_vec();
                child_path.push(slot.index);
                collect_stack_bodies(slot.node, slot.rect, &child_path, theme, window_labels, atlas, state, action_count, out, silhouettes);
            }
        }
        DockNode::Stack { windows, active } => {
            let _maximized = state.maximized_stack.as_ref().map(|p| p.as_slice()) == Some(path);
            let layout = layout_stack_cap(windows, window_labels, &HashMap::new(), atlas, theme, bounds, action_count);
            let silhouette = stack_window_silhouette(bounds, theme, &layout);
            out.push((path.to_vec(), dock_stack_body_rect(silhouette.safe_body_rect(), theme), active.clone()));
            if !active.is_empty() {
                silhouettes.insert(active.clone(), silhouette);
            }
        }
    }
}

pub(super) fn dock_text(ctx: &mut DockRenderContext<'_>, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let viewport_height = ctx.draw.screen_height();
    let mut widget_ctx = semio_framework_os_renderer_wgpu_contracts::framework_widget_context(ctx.draw, None, ctx.atlas, Some(ctx.icons), ctx.input, ctx.theme, &mut scroll, &mut collapsed, &mut selects, None, viewport_height);
    draw_text(&mut widget_ctx, text, x, y, size, color);
}
//#endregion DockFreeFunctions
