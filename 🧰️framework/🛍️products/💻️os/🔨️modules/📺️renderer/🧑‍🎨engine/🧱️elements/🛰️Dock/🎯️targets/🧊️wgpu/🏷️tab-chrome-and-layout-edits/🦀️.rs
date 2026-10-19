//! 🏷️ Tab chrome and layout edits: the device widths, the tab chips and actions, the corner tab bars, the layout normalisation, the window removal and the dock node diff. See [`super`].

use super::*;

//#region DockFreeFunctions
/// 📑️ React caps a mode-dock tab at `max-w-[12rem]` and lets its label `truncate` inside that cap
/// (`🔨️modules/🎛️chrome-control-presentation/🟦️.ts:35`, `modeDockTabClassName`) — 12 × the 16 px root
/// font size. Without it a long window title widened the tab bar without limit.
pub const MODE_DOCK_TAB_MAX_WIDTH_PX: f32 = 192.0;

/// 📱️ The shell's `mobile` breakpoint — `UI_MOBILE_MAX_WIDTH_PX`
/// (`🧰️framework/🔨️modules/🖱️ui/📱️device/🟦️.ts`), read in LOGICAL (CSS) pixels. That module is the ONE
/// owner of the policy both renderers answer; the parity of these two literals with it is asserted by
/// `🖱️ui/📱️device/🧪️tests/🔬️unit/🟦️.ts`, which reads THIS file off disk.
pub const MODE_DOCK_MOBILE_MAX_WIDTH_PX: f32 = 767.0;

/// 📱️ The shell's `tablet` breakpoint — `UI_TABLET_MAX_WIDTH_PX`. Wide enough to carry the eight dock
/// anchors a phone collapses, too narrow for the desktop's simultaneous multi-panel layout.
pub const MODE_DOCK_TABLET_MAX_WIDTH_PX: f32 = 1023.0;

/// 📱️ The device a logical viewport width IS — the Rust twin of `elementsSurfaceDeviceForWidth`, with
/// the same inclusive-maximum comparisons, so the wgpu shell and the React shell can never disagree
/// about what a width means.
pub fn mode_dock_device_for_width(width_px: f32) -> &'static str {
    if !width_px.is_finite() {
        "desktop"
    } else if width_px <= MODE_DOCK_MOBILE_MAX_WIDTH_PX {
        "mobile"
    } else if width_px <= MODE_DOCK_TABLET_MAX_WIDTH_PX {
        "tablet"
    } else {
        "desktop"
    }
}

/// 📑️ Everything a tab spends beside its label text: icon slot, its gap, both side paddings and the
/// action chips.
pub(super) fn dock_tab_chrome_width(theme: &Theme, action_count: usize) -> f32 {
    14.0 + theme.gap_standard + theme.padding_standard * 2.0 + dock_tab_action_width(theme) * action_count as f32
}

/// 📑️ The label a tab actually paints and the chip width it occupies, capped at
/// [`MODE_DOCK_TAB_MAX_WIDTH_PX`] with a CSS-`truncate`-style ellipsis.
pub fn dock_tab_chip(atlas: &mut FontAtlas, theme: &Theme, label: &str, action_count: usize) -> (String, f32) {
    let chrome = dock_tab_chrome_width(theme, action_count);
    let full = dock_tab_content_width(atlas, theme, label) + dock_tab_action_width(theme) * action_count as f32;
    if full <= MODE_DOCK_TAB_MAX_WIDTH_PX {
        return (label.to_string(), full);
    }
    let display = truncate_label_to_width(atlas, label, theme.font_size_small, (MODE_DOCK_TAB_MAX_WIDTH_PX - chrome).max(0.0));
    (display, MODE_DOCK_TAB_MAX_WIDTH_PX)
}

/// ✂️ The longest leading run of `label` that still fits `max_w` once an ellipsis is appended — what
/// `text-overflow: ellipsis` does for React's truncated tab labels. Never splits a `char`.
pub fn truncate_label_to_width(atlas: &mut FontAtlas, label: &str, size: f32, max_w: f32) -> String {
    const ELLIPSIS: &str = "…";
    if atlas.measure_text(label, size).0 <= max_w {
        return label.to_string();
    }
    let ellipsis_w = atlas.measure_text(ELLIPSIS, size).0;
    let mut kept = 0usize;
    for boundary in label.char_indices().map(|(index, _)| index).skip(1).chain(std::iter::once(label.len())) {
        if atlas.measure_text(&label[..boundary], size).0 + ellipsis_w > max_w {
            break;
        }
        kept = boundary;
    }
    format!("{}{ELLIPSIS}", &label[..kept])
}

fn dock_tab_chip_width(atlas: &mut FontAtlas, theme: &Theme, label: &str, action_count: usize) -> f32 {
    dock_tab_chip(atlas, theme, label, action_count).1
}

pub(super) fn dock_tab_action_width(theme: &Theme) -> f32 {
    14.0 + theme.padding_standard
}

/// 📑️ The action chips one dock tab paints, in React's left-to-right order — Focus/Unfocus only when
/// `!mobile && canMaximize`, then Close, then the drag grip
/// (`🧱️elements/🎨️Canvas/🟦️.tsx:1072-1101`; the grip is `🧱️DragHandle/🟦️.tsx`'s `grip-vertical`).
///
/// 🩸️ This used to be a fixed `[focus, new, close]`. The `new` chip (icon `app-window`) had no React
/// counterpart and no dispatch arm anywhere in the shell — a dead button on every window tab that
/// also widened every tab by one chip against React's two.
///
/// 🩸️ The grip was missing entirely and the shell started a tab drag from ANYWHERE on the tab; React
/// starts one only from this chip (`onPointerDown` on the `DragHandle`) and keeps the label button a
/// pure select target.
pub fn dock_tab_actions(show_maximize: bool, maximized: bool) -> Vec<(&'static str, &'static str)> {
    let mut actions = Vec::with_capacity(3);
    if show_maximize {
        actions.push(("focus", if maximized { "minimize-2" } else { "maximize-2" }));
    }
    actions.push(("close", "x"));
    actions.push(("drag", "grip-vertical"));
    actions
}

/// 📑️ The label a tab shows for a window instance — its own instance label, else its kind's.
pub(super) fn dock_tab_label<'a>(tab: &'a DockStackTab, window_labels: &'a HashMap<String, String>) -> &'a str {
    window_labels.get(&tab.window_id).or_else(|| window_labels.get(&tab.window_kind_id)).map(String::as_str).unwrap_or(tab.window_id.as_str())
}

/// 📥️ An empty corner still offers a drop pad: React renders `mode-dock-corner-drop-pad` at
/// `min-h-medium min-w-medium` while a drag is live (`🧱️elements/🎨️Canvas/🟦️.tsx:1114`), and
/// `--size-medium` is `7 × --ui-spacing` — the same token `theme.control_height` carries.
pub(super) fn collect_corner_tab_bars_for_stack(
    path: &DockPath,
    windows: &[DockStackTab],
    bounds: Rect,
    theme: &Theme,
    atlas: &mut FontAtlas,
    window_labels: &HashMap<String, String>,
    action_count: usize,
    out: &mut Vec<(DockPath, WindowStackCorner, Rect, Vec<f32>)>,
) {
    let tab_h = dock_cap_depth(theme);
    let pad = theme.control_height;
    for (corner, tabs) in tabs_by_corner(windows) {
        let widths: Vec<f32> = tabs.iter().map(|tab| dock_tab_chip_width(atlas, theme, dock_tab_label(tab, window_labels), action_count)).collect();
        let width = widths.iter().sum::<f32>().max(pad);
        let rect = match corner {
            WindowStackCorner::TopLeft => Rect::new(bounds.x, bounds.y, width, tab_h),
            WindowStackCorner::TopRight => Rect::new(bounds.x + bounds.w - width, bounds.y, width, tab_h),
            WindowStackCorner::BottomLeft => Rect::new(bounds.x, bounds.y + bounds.h - tab_h, width, tab_h),
            WindowStackCorner::BottomRight => Rect::new(bounds.x + bounds.w - width, bounds.y + bounds.h - tab_h, width, tab_h),
        };
        out.push((path.to_vec(), corner, rect, widths));
    }
}

pub(super) fn collect_stack_corner_tab_bars(node: &DockNode, bounds: Rect, path: &[usize], theme: &Theme, atlas: &mut FontAtlas, window_labels: &HashMap<String, String>, action_count: usize, out: &mut Vec<(DockPath, WindowStackCorner, Rect, Vec<f32>)>) {
    match node {
        DockNode::Row(children) => {
            for slot in dock_axis_slots(children, bounds, true, theme.gap_standard) {
                let mut child_path = path.to_vec();
                child_path.push(slot.index);
                collect_stack_corner_tab_bars(slot.node, slot.rect, &child_path, theme, atlas, window_labels, action_count, out);
            }
        }
        DockNode::Column(children) => {
            for slot in dock_axis_slots(children, bounds, false, theme.gap_standard) {
                let mut child_path = path.to_vec();
                child_path.push(slot.index);
                collect_stack_corner_tab_bars(slot.node, slot.rect, &child_path, theme, atlas, window_labels, action_count, out);
            }
        }
        DockNode::Stack { windows, .. } => {
            collect_corner_tab_bars_for_stack(&path.to_vec(), windows, bounds, theme, atlas, window_labels, action_count, out);
        }
    }
}

/// 🪟️ Wgpu-local adapter: builds the balanced fallback layout via
/// `ui_wgpu::wgpu::even_window_layout` and converts it to a runtime `DockNode`.
pub fn even_layout(window_ids: &[String]) -> DockNode {
    dock_from_window_layout(&even_window_layout(window_ids).root)
}

pub(super) fn normalize_sizes(children: Vec<(DockNode, f32)>, axis_size: Option<f32>) -> Vec<(DockNode, f32)> {
    if children.is_empty() {
        return children;
    }
    let total: f32 = children.iter().map(|(_, s)| *s).sum();
    let scale = axis_size.unwrap_or(total).max(0.001);
    let sum = total.max(0.001);
    children.into_iter().map(|(node, size)| (node, size / sum * scale)).collect()
}

pub(super) fn first_window_id(node: &DockNode) -> Option<String> {
    match node {
        DockNode::Stack { active, .. } if !active.is_empty() => Some(active.clone()),
        DockNode::Row(children) | DockNode::Column(children) => children.iter().find_map(|(child, _)| first_window_id(child)),
        DockNode::Stack { .. } => None,
    }
}

pub fn find_stack_path(node: &DockNode, window_id: &str, path: &mut DockPath) -> Option<DockPath> {
    match node {
        DockNode::Stack { windows, .. } if windows.iter().any(|tab| tab.window_id == window_id) => Some(path.clone()),
        DockNode::Row(children) | DockNode::Column(children) => {
            for (index, (child, _)) in children.iter().enumerate() {
                path.push(index);
                if let Some(found) = find_stack_path(child, window_id, path) {
                    return Some(found);
                }
                path.pop();
            }
            None
        }
        DockNode::Stack { .. } => None,
    }
}

pub fn node_at<'a>(node: &'a DockNode, path: &[usize]) -> Option<&'a DockNode> {
    let mut current = node;
    for index in path {
        current = match current {
            DockNode::Row(children) | DockNode::Column(children) => children.get(*index).map(|(n, _)| n)?,
            DockNode::Stack { .. } => return None,
        };
    }
    Some(current)
}

pub(super) fn node_at_mut<'a>(node: &'a mut DockNode, path: &[usize]) -> Option<&'a mut DockNode> {
    if path.is_empty() {
        return Some(node);
    }
    let (head, tail) = path.split_first()?;
    let child = match node {
        DockNode::Row(children) | DockNode::Column(children) => children.get_mut(*head).map(|(n, _)| n)?,
        DockNode::Stack { .. } => return None,
    };
    node_at_mut(child, &tail.to_vec())
}

/// 🪟️ Drops empty stacks and HOISTS a single-child axis into its own slot — the twin of React's
/// `collapseLayout` (`🧱️elements/🎨️Canvas/🟦️.tsx:225-236`), including its `only.size ?? node.size`
/// rule: the hoisted child keeps the ratio it held among its (now gone) siblings, not the ratio its
/// parent held. `None` is React's `null` — nothing is left to render.
///
/// 🩸️ The drag lane used to collapse on a weaker sibling rule (`collapse_empty`: prune empty stacks,
/// never hoist) because it removed the dragged window from the COMMITTED tree at drag promotion. The
/// removal is now a render-only derivation, so this is the one collapse rule in the element.
pub(super) fn collapse_layout_node(node: &DockNode, size: f32) -> Option<(DockNode, f32)> {
    match node {
        DockNode::Stack { windows, .. } => (!windows.is_empty()).then(|| (node.clone(), size)),
        DockNode::Row(children) | DockNode::Column(children) => {
            let kept: Vec<(DockNode, f32)> = children.iter().filter_map(|(child, child_size)| collapse_layout_node(child, *child_size)).collect();
            match kept.len() {
                0 => None,
                1 => kept.into_iter().next(),
                _ => Some((rebuild_axis(node, kept), size)),
            }
        }
    }
}

/// 🪟️ Rebuilds an axis node of the same orientation with new children.
pub(super) fn rebuild_axis(node: &DockNode, children: Vec<(DockNode, f32)>) -> DockNode {
    match node {
        DockNode::Column(_) => DockNode::Column(children),
        _ => DockNode::Row(children),
    }
}

/// 🪟️ Removes one window from a layout tree and collapses what it empties — twin of React's
/// `removeWindowFromLayout` (`🧱️elements/🎨️Canvas/🟦️.tsx:307-318`). A stack whose `active` tab is the
/// one being removed re-focuses `children[0]`, exactly like React.
fn remove_window_from_layout_node(node: &DockNode, window_id: &str, size: f32) -> Option<(DockNode, f32)> {
    match node {
        DockNode::Stack { windows, active } => {
            let kept: Vec<DockStackTab> = windows.iter().filter(|tab| tab.window_id != window_id).cloned().collect();
            let first = kept.first()?.window_id.clone();
            let active = if active == window_id { first } else { active.clone() };
            Some((DockNode::Stack { windows: kept, active }, size))
        }
        DockNode::Row(children) | DockNode::Column(children) => {
            let kept: Vec<(DockNode, f32)> = children.iter().filter_map(|(child, child_size)| remove_window_from_layout_node(child, window_id, *child_size)).collect();
            if kept.is_empty() {
                return None;
            }
            collapse_layout_node(&rebuild_axis(node, kept), size)
        }
    }
}

/// 🪟️ Root-level [`remove_window_from_layout_node`] — `None` when the last window is gone, which
/// React answers with its `{ kind: "stack", children: [] }` empty root.
pub fn remove_window_from_layout(root: &DockNode, window_id: &str) -> Option<DockNode> {
    remove_window_from_layout_node(root, window_id, 1.0).and_then(|(node, size)| collapse_layout_node(&node, size)).map(|(node, _)| node)
}

/// 🔑️ Diffs an incoming `next` node (freshly parsed from a `WindowLayout`) against the corresponding
/// `old` node by stable window-id key: identical stacks/axes are reused wholesale, a stack that's
/// merely reordered keeps its old `active` tab focused (rather than reverting to whatever `next` says),
/// and a structural-kind change (e.g. `Stack` -> `Row`) just adopts `next` outright since there's no
/// shared identity to preserve. Mirrors the intent of `ui/js/react/index.tsx`'s keyed reconciliation
/// helpers (`mapLayoutStacks` family) for this crate's `DockNode` shape. See `DockState::apply_layout_diff`.
pub fn diff_dock_node(old: &DockNode, next: DockNode) -> DockNode {
    match (old, &next) {
        (DockNode::Stack { windows: old_windows, active: old_active }, DockNode::Stack { windows: next_windows, active: next_active }) => {
            if old_windows == next_windows {
                return old.clone();
            }
            let same_membership = old_windows.len() == next_windows.len() && old_windows.iter().all(|tab| next_windows.iter().any(|next| next.window_id == tab.window_id));
            let active = if same_membership && old_windows.iter().any(|tab| tab.window_id == *old_active) { old_active.clone() } else { next_active.clone() };
            DockNode::Stack { windows: next_windows.clone(), active }
        }
        (DockNode::Row(old_children), DockNode::Row(next_children)) => DockNode::Row(diff_axis_children(old_children, next_children)),
        (DockNode::Column(old_children), DockNode::Column(next_children)) => DockNode::Column(diff_axis_children(old_children, next_children)),
        _ => next,
    }
}

/// 🔑️ Pairwise-diffs axis children by index (the only stable positional key an `Axis` node offers) —
/// children beyond `old`'s length are new inserts adopted as-is from `next`.
fn diff_axis_children(old_children: &[(DockNode, f32)], next_children: &[(DockNode, f32)]) -> Vec<(DockNode, f32)> {
    next_children
        .iter()
        .enumerate()
        .map(|(index, (next_child, next_size))| {
            let merged = match old_children.get(index) {
                Some((old_child, _)) => diff_dock_node(old_child, next_child.clone()),
                None => next_child.clone(),
            };
            (merged, *next_size)
        })
        .collect()
}

#[derive(Clone, Copy, Debug)]
pub(super) struct DockAxisSlot<'a> {
    pub(super) index: usize,
    pub(super) node: &'a DockNode,
    pub(super) rect: Rect,
    pub(super) separator_after: Option<Rect>,
}
//#endregion DockFreeFunctions
