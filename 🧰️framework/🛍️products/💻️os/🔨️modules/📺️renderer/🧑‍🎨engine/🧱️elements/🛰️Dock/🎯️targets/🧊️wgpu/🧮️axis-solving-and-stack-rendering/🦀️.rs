//! 🧮️ Axis solving and stack rendering: the axis slots, the node bounds solver, the node and axis rendering, the resize hits and the stack rendering. See [`super`].

use super::*;

//#region DockFreeFunctions
struct DockAxisSlots<'a> {
    pub(super) children: &'a [(DockNode, f32)],
    pub(super) bounds: Rect,
    horizontal: bool,
    pub(super) separator: f32,
    distributable: f32,
    total_weight: f32,
    pub(super) cursor: f32,
    pub(super) index: usize,
}

impl<'a> Iterator for DockAxisSlots<'a> {
    type Item = DockAxisSlot<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        let (node, weight) = self.children.get(index)?;
        let positive_weight = if weight.is_finite() { weight.max(0.0) } else { 0.0 };
        let normalized_weight = if self.total_weight > f32::EPSILON { positive_weight / self.total_weight } else { 1.0 / self.children.len() as f32 };
        let extent = if index + 1 == self.children.len() {
            let far = if self.horizontal { self.bounds.x + self.bounds.w } else { self.bounds.y + self.bounds.h };
            (far - self.cursor).max(0.0)
        } else {
            self.distributable * normalized_weight
        };
        let rect = if self.horizontal { Rect::new(self.cursor, self.bounds.y, extent, self.bounds.h) } else { Rect::new(self.bounds.x, self.cursor, self.bounds.w, extent) };
        let separator_after =
            (index + 1 < self.children.len()).then(|| if self.horizontal { Rect::new(self.cursor + extent, self.bounds.y, self.separator, self.bounds.h) } else { Rect::new(self.bounds.x, self.cursor + extent, self.bounds.w, self.separator) });
        self.cursor += extent + separator_after.map_or(0.0, |_| self.separator);
        self.index += 1;
        Some(DockAxisSlot { index, node, rect, separator_after })
    }
}

fn axis_separator_extent(extent: f32, child_count: usize, separator: f32) -> f32 {
    let boundaries = child_count.saturating_sub(1);
    if boundaries == 0 {
        return 0.0;
    }
    separator.max(0.0).min(extent.max(0.0) / boundaries as f32)
}

pub(super) fn axis_distributable_extent(extent: f32, child_count: usize, separator: f32) -> f32 {
    let separator = axis_separator_extent(extent, child_count, separator);
    (extent.max(0.0) - separator * child_count.saturating_sub(1) as f32).max(0.0)
}

pub(super) fn dock_axis_slots(children: &[(DockNode, f32)], bounds: Rect, horizontal: bool, separator: f32) -> DockAxisSlots<'_> {
    let extent = if horizontal { bounds.w } else { bounds.h };
    let separator = axis_separator_extent(extent, children.len(), separator);
    let total_weight = children.iter().map(|(_, weight)| if weight.is_finite() { weight.max(0.0) } else { 0.0 }).sum();
    DockAxisSlots { children, bounds, horizontal, separator, distributable: axis_distributable_extent(extent, children.len(), separator), total_weight, cursor: if horizontal { bounds.x } else { bounds.y }, index: 0 }
}

pub(super) fn solve_node_bounds(node: &DockNode, bounds: Rect, target_path: &[usize], current_path: &[usize], separator: f32) -> Option<Rect> {
    if current_path == target_path {
        return Some(bounds);
    }
    match node {
        DockNode::Row(children) => {
            for slot in dock_axis_slots(children, bounds, true, separator) {
                let mut path = current_path.to_vec();
                path.push(slot.index);
                if let Some(found) = solve_node_bounds(slot.node, slot.rect, target_path, &path, separator) {
                    return Some(found);
                }
            }
            None
        }
        DockNode::Column(children) => {
            for slot in dock_axis_slots(children, bounds, false, separator) {
                let mut path = current_path.to_vec();
                path.push(slot.index);
                if let Some(found) = solve_node_bounds(slot.node, slot.rect, target_path, &path, separator) {
                    return Some(found);
                }
            }
            None
        }
        DockNode::Stack { .. } => None,
    }
}

pub(super) fn render_node(state: &DockState, ctx: &mut DockRenderContext<'_>, node: &DockNode, bounds: Rect, path: &[usize], body_fill: bool, render_body: &mut dyn FnMut(Rect, &str), outer_split: Option<(DockPath, usize, bool)>) {
    match node {
        DockNode::Row(children) => render_axis(state, ctx, children, bounds, path, true, body_fill, render_body, outer_split),
        DockNode::Column(children) => render_axis(state, ctx, children, bounds, path, false, body_fill, render_body, outer_split),
        DockNode::Stack { .. } => {
            let maximized = state.maximized_stack.as_ref().map(|p| p.as_slice()) == Some(path);
            render_stack(state, ctx, path, node, bounds, maximized, body_fill, render_body)
        }
    }
}

const SPLIT_VIS_PX: f32 = 6.0;

const SPLIT_HIT_MIN_PX: f32 = 20.0;

fn render_axis(state: &DockState, ctx: &mut DockRenderContext<'_>, children: &[(DockNode, f32)], bounds: Rect, path: &[usize], horizontal: bool, body_fill: bool, render_body: &mut dyn FnMut(Rect, &str), outer_split: Option<(DockPath, usize, bool)>) {
    for slot in dock_axis_slots(children, bounds, horizontal, ctx.theme.gap_standard) {
        let mut child_path = path.to_vec();
        child_path.push(slot.index);
        render_node(state, ctx, slot.node, slot.rect, &child_path, body_fill, render_body, Some((path.to_vec(), slot.index, horizontal)));
    }
    let _ = outer_split;
}

pub(super) fn walk_resize_hits(state: &DockState, ctx: &mut DockRenderContext<'_>, node: &DockNode, bounds: Rect, path: &[usize], outer_split: Option<(DockPath, usize, bool)>) {
    match node {
        DockNode::Row(children) => walk_resize_axis(state, ctx, children, bounds, path, true, outer_split),
        DockNode::Column(children) => walk_resize_axis(state, ctx, children, bounds, path, false, outer_split),
        DockNode::Stack { .. } => {}
    }
}

fn walk_resize_axis(state: &DockState, ctx: &mut DockRenderContext<'_>, children: &[(DockNode, f32)], bounds: Rect, path: &[usize], horizontal: bool, outer_split: Option<(DockPath, usize, bool)>) {
    for slot in dock_axis_slots(children, bounds, horizontal, ctx.theme.gap_standard) {
        let mut child_path = path.to_vec();
        child_path.push(slot.index);
        walk_resize_hits(state, ctx, slot.node, slot.rect, &child_path, Some((path.to_vec(), slot.index, horizontal)));
        if let Some(separator) = slot.separator_after {
            let handle = centered_axis_hit(bounds, separator, horizontal, SPLIT_HIT_MIN_PX.max(SPLIT_VIS_PX));
            register_split_hit(ctx, path, slot.index, handle, if horizontal { DragAxis::Horizontal } else { DragAxis::Vertical });
            if let Some((parent_path, parent_index, parent_horizontal)) = &outer_split {
                if *parent_horizontal != horizontal {
                    register_join_corner_hits(ctx, path, slot.index, parent_path, *parent_index, handle, horizontal);
                }
            }
        }
    }
}

fn centered_axis_hit(bounds: Rect, separator: Rect, horizontal: bool, hit_extent: f32) -> Rect {
    if horizontal {
        let center = separator.x + separator.w * 0.5;
        let left = (center - hit_extent * 0.5).max(bounds.x);
        let right = (center + hit_extent * 0.5).min(bounds.x + bounds.w);
        Rect::new(left, bounds.y, (right - left).max(0.0), bounds.h)
    } else {
        let center = separator.y + separator.h * 0.5;
        let top = (center - hit_extent * 0.5).max(bounds.y);
        let bottom = (center + hit_extent * 0.5).min(bounds.y + bounds.h);
        Rect::new(bounds.x, top, bounds.w, (bottom - top).max(0.0))
    }
}

fn register_join_corner_hits(ctx: &mut DockRenderContext<'_>, path: &[usize], split_index: usize, parent_path: &DockPath, parent_index: usize, handle: Rect, horizontal: bool) {
    let corner = 10.0;
    let corners = if horizontal {
        [Rect::new(handle.x - corner * 0.5, handle.y, corner, corner), Rect::new(handle.x - corner * 0.5, handle.y + handle.h - corner, corner, corner)]
    } else {
        [Rect::new(handle.x, handle.y - corner * 0.5, corner, corner), Rect::new(handle.x + handle.w - corner, handle.y - corner * 0.5, corner, corner)]
    };
    for (corner_slot, rect) in corners.iter().enumerate() {
        let _ = corner_slot;
        ctx.input.register_hit(HitTarget {
            rect: *rect,
            event: None,
            control_id: Some(format!("dock.corner.r/{}/{}/c/{}/{}", path_str(path), split_index, path_str(parent_path), parent_index)),
            kind: HitKind::DockJoinCorner,
            drag_axis: Some(DragAxis::Both),
            drag_data: None,
        });
    }
}

/// 📑️ One tab's width with no action chips — its icon slot, gap, label and both side paddings.
pub fn dock_tab_content_width(atlas: &mut FontAtlas, theme: &Theme, label: &str) -> f32 {
    dock_tab_chrome_width(theme, 0) + atlas.measure_text(label, theme.font_size_small).0
}

fn paint_dock_tab_icon(ctx: &mut DockRenderContext<'_>, icon_id: &str, x: f32, tab_rect: Rect, color: Rgba) -> f32 {
    const ICON_TINY: f32 = 14.0;
    if let Some(uv) = ctx.icons.icon_uv(icon_id) {
        ctx.draw.push_textured([x, tab_rect.y + (tab_rect.h - ICON_TINY) * 0.5, ICON_TINY, ICON_TINY], uv, color);
        ICON_TINY + ctx.theme.gap_standard
    } else {
        0.0
    }
}

fn register_split_hit(ctx: &mut DockRenderContext<'_>, path: &[usize], index: usize, rect: Rect, axis: DragAxis) {
    ctx.input.register_hit(HitTarget { rect, event: None, control_id: Some(format!("dock.split.{}.{index}", path_str(path))), kind: HitKind::DockSplit, drag_axis: Some(axis), drag_data: None });
}

/// 🪟️ The focus visual: a hairline stroke along the WHOLE stack silhouette (tabs + cutouts +
/// body), `--active-base` when the stack holds the active window and `--border-normal-color`
/// otherwise — React's `windowSilhouetteBorderPaint`'s `active`/`normal` kinds
/// (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:7494-7509`) over the same v1 geometry.
///
/// 🩸️ The call sat COMMENTED OUT here, so no window ever had an outline on wgpu and the only
/// focus cue was the active tab's text tint.
/// 📑️ Paints one tab stack and registers its pointer targets in REACT'S OWN ORDER: a tab's SELECT
/// target first, then its action chips left to right (`focus`, `close`, `drag`) — the DOM order of
/// `🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1034-1100`, where the `<button role="tab">` label precedes the
/// `mode-dock-tab-focus`/`-close` chips and the `DragHandle`.
///
/// 🩸️ The chips used to be registered BEFORE the select target, so the first `dock.tab.…` row this
/// renderer published for a stack was its DESTRUCTIVE `close` chip. Anything that resolves "the dock
/// tab" by scanning the published rows — the parity probe's `reopenWindow` does exactly that — landed
/// on `close` here while React's same scan landed on the `mode-dock-tabbar` container that owns the
/// tabs, so the journey's `window-reopen` step CLOSED the last remaining world pane and every later
/// window-owned chord (`escape` → `engagementAbort`, `mod+z` → `undo`) became a hinted no-op against
/// an empty dock (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w12c-chords-and-camera-live.md` §4.2,
/// `🗑️generated/w12c-parity-run-19/steps.json` step 18: `dock.tab..puzzle3d-main-perspective.close`).
/// The rects do not overlap, so this is publication order only — never which row a press resolves.
pub(super) fn render_stack(state: &DockState, ctx: &mut DockRenderContext<'_>, path: &[usize], node: &DockNode, bounds: Rect, maximized: bool, body_fill: bool, render_body: &mut dyn FnMut(Rect, &str)) {
    let DockNode::Stack { windows, active } = node else {
        return;
    };
    if windows.is_empty() {
        return;
    }
    let theme = ctx.theme;
    let tab_h = dock_cap_depth(theme);
    let globally_active = state.active_stack.as_ref().map(|p| p.as_slice()) == Some(path);

    let actions = dock_tab_actions(state.show_maximize(), maximized);
    let layout = layout_stack_cap(windows, ctx.window_labels, ctx.window_icon_ids, ctx.atlas, theme, bounds, actions.len());
    let silhouette = stack_window_silhouette(bounds, theme, &layout);

    if body_fill {
        let content_bounds = silhouette.content_bounds();
        let body_bounds = dock_stack_body_rect(silhouette.safe_body_rect(), theme);
        ctx.draw.begin_silhouette_clip(&silhouette.content_clip_rects());
        ctx.draw.push_solid([content_bounds.x, content_bounds.y, content_bounds.w, content_bounds.h], theme.canvas_clear);
        render_body(body_bounds, active);
        ctx.draw.end_silhouette_clip();
    }

    for group in &layout.groups {
        if group.tabs.is_empty() {
            continue;
        }
        let group_w = group.tabs.iter().map(|tab| tab.rect.w).sum::<f32>();
        let group_rect = Rect::new(group.tabs[0].rect.x, group.tabs[0].rect.y, group_w, tab_h);
        let group_glass = ctx.draw.push_glass([group_rect.x, group_rect.y, group_rect.w, group_rect.h], 0.0, theme.glass(Level::Window));
        ctx.draw.begin_glass_content(group_glass);
        for tab in &group.tabs {
            let is_active = tab.window_id == *active;
            let stack_active_tab = is_active && globally_active;
            let hovered = tab.rect.contains(ctx.input.pointer_x, ctx.input.pointer_y);
            if stack_active_tab {
                ctx.draw.push_solid([tab.rect.x, tab.rect.y, tab.rect.w, tab.rect.h], theme.selected);
            }
            let tint = if stack_active_tab {
                theme.active_foreground
            } else if hovered {
                theme.border_emphasized
            } else {
                theme.text_element
            };
            let control_rect = Rect::new(tab.rect.x, tab.rect.y + theme.padding_standard, tab.rect.w, theme.control_height);
            let mut content_x = control_rect.x + theme.padding_standard;
            let icon_w = paint_dock_tab_icon(ctx, &tab.icon_id, content_x, control_rect, tint);
            content_x += icon_w;
            dock_text(ctx, &tab.label, content_x, control_rect.y + (control_rect.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, tint);
            let action_w = dock_tab_action_width(theme);
            let select_w = (tab.rect.w - action_w * actions.len() as f32).max(theme.padding_standard * 2.0);
            let select_rect = Rect::new(tab.rect.x, tab.rect.y, select_w, tab.rect.h);
            let select_id = format!("dock.tab.{}.{}", path_str(path), tab.window_id);
            if let Some(names) = ctx.control_names.as_mut() {
                names(&select_id, DockControlName::Tab { title: &tab.label, selected: is_active, panel: active });
            }
            ctx.input.register_hit(HitTarget { rect: select_rect, event: None, control_id: Some(select_id), kind: HitKind::Window, drag_axis: None, drag_data: None });
            content_x = tab.rect.x + tab.rect.w - action_w * actions.len() as f32;
            for (action, icon_id) in actions.iter() {
                let action_rect = Rect::new(content_x, control_rect.y, action_w, control_rect.h);
                let action_hovered = action_rect.contains(ctx.input.pointer_x, ctx.input.pointer_y);
                let action_tint = chrome_item_text(theme, false, action_hovered);
                let _ = paint_dock_tab_icon(ctx, icon_id, action_rect.x + theme.padding_standard * 0.5, action_rect, action_tint);
                let action_id = format!("dock.tab.{}.{}.{}", path_str(path), tab.window_id, action);
                if let Some(names) = ctx.control_names.as_mut() {
                    let name = match *action {
                        "focus" if maximized => Some(DockControlName::Unfocus),
                        "focus" => Some(DockControlName::Focus),
                        "close" => Some(DockControlName::Close),
                        "drag" => Some(DockControlName::Drag(&tab.label)),
                        _ => None,
                    };
                    if let Some(name) = name {
                        names(&action_id, name);
                    }
                }
                ctx.input.register_hit(HitTarget { rect: action_rect, event: None, control_id: Some(action_id), kind: HitKind::Button, drag_axis: None, drag_data: None });
                content_x += action_w;
            }
        }
        ctx.draw.end_glass_content();
    }

    push_window_silhouette_border(&mut *ctx.draw, &silhouette, theme.stroke_hairline, if globally_active { theme.selected } else { theme.border_normal });
}
//#endregion DockFreeFunctions
