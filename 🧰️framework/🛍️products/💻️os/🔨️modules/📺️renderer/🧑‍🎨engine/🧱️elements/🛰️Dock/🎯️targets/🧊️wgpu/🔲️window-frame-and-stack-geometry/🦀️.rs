//! 🔲️ Window frame and stack geometry: the frame border, the stack cap layout, the path parsing and the stack frame and body collection over [`semio_ui_window_frame::WindowFrame`]. See [`super`].

use super::*;

//#region DockFreeFunctions
//#region WindowFrameGeometry
/// 🖌️ Paints the frame outline and the chips' inner edges as hairline (or thicker) strokes, every segment lying INSIDE the frame.
///
/// 🩸️ The outline is inset exactly like React's `WINDOW_FRAME_PATH_INSET` path, so a stroke never bleeds past the frame box (`🔲️WindowFrame/🟦️.tsx`).
pub fn push_window_frame_border(draw: &mut DrawList, frame: &WindowFrame, stroke: f32, color: Rgba) {
    for edge in window_frame_border_edges(frame) {
        draw.push_solid(edge.paint_rect(stroke), color);
    }
}
/// 🧻️ Where a pane's content starts: document-like panes (text, forms) begin below the frame's top chip row and end above its
/// bottom chip row (`WindowFrame::clearance_rect`), while the pick clip and the scissor stay the whole frame so scrolled content
/// slides beneath the glass chips; edgeless panes (scene, canvas, map, world) are full-bleed from the first frame.
pub fn pane_document_rect(window_rect: Rect, frame: Option<&WindowFrame>, edgeless: bool) -> Rect {
    match frame {
        Some(frame) if !edgeless && frame.bounds == window_rect => frame.clearance_rect(),
        _ => window_rect,
    }
}
/// 🧱️ Whether a window of this declared kind mounts an engine host (world3d, node graph, tiled map, board2d), known from the manifest at creation and so right on the first laid-out frame, unlike the host registration that only lands once the document has painted.
pub fn surface_kind_is_engine_host(kind: Option<&semio_ui_declaration::ui::SurfaceKind>) -> bool {
    use semio_ui_declaration::ui::SurfaceKind;
    matches!(kind, Some(SurfaceKind::World3d | SurfaceKind::NodeGraph | SurfaceKind::TiledMap | SurfaceKind::Board2d))
}

/// 🧻️ [`pane_document_rect`] decided from the window's declared kind, so the first frame of a new window is already laid out final.
pub fn kind_pane_document_rect(window_rect: Rect, frame: Option<&WindowFrame>, kind: Option<&semio_ui_declaration::ui::SurfaceKind>) -> Rect {
    pane_document_rect(window_rect, frame, surface_kind_is_engine_host(kind))
}
//#endregion WindowFrameGeometry

/// 🏷️ One painted tab of a stack cap group, as returned inside [`StackCapGroupLayout`] by [`layout_stack_cap`].
pub struct StackCapTabLayout {
    pub window_id: String,
    pub label: String,
    pub(super) icon_id: String,
    pub rect: Rect,
}

/// 🧷️ The tabs of one corner group of a stack cap, as returned inside [`StackCapLayout`] by [`layout_stack_cap`].
pub struct StackCapGroupLayout {
    pub tabs: Vec<StackCapTabLayout>,
}

/// 🧢️ The solved cap of a dock stack: [`layout_stack_cap`] produces it and [`stack_window_frame`] consumes it, so it is public.
pub struct StackCapLayout {
    pub groups: Vec<StackCapGroupLayout>,
    top_chips: Vec<WindowFrameChip>,
    bottom_chips: Vec<WindowFrameChip>,
}

pub fn layout_stack_cap(windows: &[DockStackTab], labels: &HashMap<String, String>, icon_ids: &HashMap<String, String>, atlas: &mut FontAtlas, theme: &Theme, bounds: Rect, action_count: usize) -> StackCapLayout {
    let tab_h = dock_cap_depth(theme);
    let mut groups = Vec::new();
    let mut top_chips = Vec::new();
    let mut bottom_chips = Vec::new();
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
        let chip = WindowFrameChip::new((origin_x - bounds.x).max(0.0), (origin_x - bounds.x + total_w).min(bounds.w));
        match corner {
            WindowStackCorner::TopLeft | WindowStackCorner::TopRight => top_chips.push(chip),
            WindowStackCorner::BottomLeft | WindowStackCorner::BottomRight => bottom_chips.push(chip),
        }
        groups.push(StackCapGroupLayout { tabs: painted });
    }
    StackCapLayout { groups, top_chips, bottom_chips }
}

pub fn stack_window_frame(bounds: Rect, theme: &Theme, layout: &StackCapLayout) -> WindowFrame {
    let top_depth = if layout.top_chips.is_empty() { 0.0 } else { dock_cap_depth(theme) };
    let bottom_depth = if layout.bottom_chips.is_empty() { 0.0 } else { dock_cap_depth(theme) };
    WindowFrame::from_measured_edges(bounds, layout.top_chips.clone(), layout.bottom_chips.clone(), top_depth, bottom_depth)
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

/// 🪟️ Every stack's content rect (the whole frame) and the window it shows, plus that window's frame. A stack holding
/// NO tabs answers its rect with an EMPTY window id — it is still a drop target, exactly as React's
/// `WindowChrome` keeps rendering for an empty stack — but it owns no frame and no window, because
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
    frames: &mut HashMap<String, WindowFrame>,
) {
    match node {
        DockNode::Row(children) => {
            for slot in dock_axis_slots(children, bounds, true, 0.0) {
                let mut child_path = path.to_vec();
                child_path.push(slot.index);
                collect_stack_bodies(slot.node, slot.rect, &child_path, theme, window_labels, atlas, state, action_count, out, frames);
            }
        }
        DockNode::Column(children) => {
            for slot in dock_axis_slots(children, bounds, false, 0.0) {
                let mut child_path = path.to_vec();
                child_path.push(slot.index);
                collect_stack_bodies(slot.node, slot.rect, &child_path, theme, window_labels, atlas, state, action_count, out, frames);
            }
        }
        DockNode::Stack { windows, active } => {
            let _maximized = state.maximized_stack.as_ref().map(|p| p.as_slice()) == Some(path);
            let layout = layout_stack_cap(windows, window_labels, &HashMap::new(), atlas, theme, bounds, action_count);
            let frame = stack_window_frame(bounds, theme, &layout);
            out.push((path.to_vec(), frame.content_clip_rect(), active.clone()));
            if !active.is_empty() {
                frames.insert(active.clone(), frame);
            }
        }
    }
}

pub(super) fn dock_text(ctx: &mut DockRenderContext<'_>, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let viewport_height = ctx.draw.screen_height();
    let mut widget_ctx = semio_renderer_wgpu_contracts::framework_widget_context(ctx.draw, None, ctx.atlas, Some(ctx.icons), ctx.input, ctx.theme, &mut scroll, &mut collapsed, &mut selects, None, viewport_height);
    draw_text(&mut widget_ctx, text, x, y, size, color);
}
//#endregion DockFreeFunctions
