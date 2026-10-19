//! 📐️ Layout conversion and drop zones: the conversion between window layouts and dock nodes, the layout edits, the split and drop zone geometry and the stack tab bars. See [`super`].

use super::*;

//#region DockFreeFunctions
pub fn dock_from_window_layout(root: &WindowLayoutRoot) -> DockNode {
    match root {
        WindowLayoutRoot::Axis(axis) => axis_from_children(&axis.kind, &axis.children, axis.size),
        WindowLayoutRoot::Stack(stack) => stack_from_node(stack),
    }
}

/// 🏷️ Every EXTRA window instance's authored title, keyed by instance id — the Rust twin of React's
/// `resolveFrameworkLayoutSeed`'s `extraInstances` (`🛠️ShellHelpers/🟦️.tsx:1555-1566`), which is where
/// `windowTitlesById[instance.id] ?? instance.title` (`🏛️ShellHost/🟦️.tsx:10296`) reads a pane's cap
/// label from. Only a node whose `instanceId` differs from its `windowKindId` is an extra instance;
/// a bare kind keeps the kind's own manifest label, exactly as React's `baseWindows` do.
///
/// 🩸️ Without it every pane of a split kind read the KIND label — puzzle3d's two panes both said
/// `Puzzle 3D` where React reads `Top` and `Perspective`, because `stack_from_node` dropped
/// `WindowLayoutWindowNode::title` on the floor.
pub fn window_layout_instance_titles(layout: &WindowLayout) -> HashMap<String, String> {
    fn stack(node: &WindowLayoutStackNode, out: &mut HashMap<String, String>) {
        for window in &node.children {
            let (Some(instance_id), Some(title)) = (window.instance_id.as_ref(), window.title.as_ref()) else { continue };
            if instance_id != &window.window_kind_id && !title.is_empty() {
                out.insert(instance_id.clone(), title.clone());
            }
        }
    }
    fn child(node: &WindowLayoutChild, out: &mut HashMap<String, String>) {
        match node {
            WindowLayoutChild::Axis(axis) => axis.children.iter().for_each(|child_node| child(child_node, out)),
            WindowLayoutChild::Stack(node) => stack(node, out),
        }
    }
    let mut out = HashMap::new();
    match &layout.root {
        WindowLayoutRoot::Axis(axis) => axis.children.iter().for_each(|child_node| child(child_node, &mut out)),
        WindowLayoutRoot::Stack(node) => stack(node, &mut out),
    }
    out
}

fn axis_from_children(kind: &str, children: &[WindowLayoutChild], size: Option<f64>) -> DockNode {
    let parsed: Vec<(DockNode, f32)> = children
        .iter()
        .map(|child| match child {
            WindowLayoutChild::Axis(axis) => (axis_from_children(&axis.kind, &axis.children, axis.size), axis.size.map(|v| v as f32).unwrap_or(1.0)),
            WindowLayoutChild::Stack(stack) => (stack_from_node(stack), stack.size.map(|v| v as f32).unwrap_or(1.0)),
        })
        .collect();
    let normalized = normalize_sizes(parsed, size.map(|v| v as f32));
    if kind == "column" {
        DockNode::Column(normalized)
    } else {
        DockNode::Row(normalized)
    }
}

fn stack_from_node(stack: &WindowLayoutStackNode) -> DockNode {
    let windows: Vec<DockStackTab> = stack
        .children
        .iter()
        .map(|window| DockStackTab::instance_template(window.instance_id.clone().unwrap_or_else(|| window.window_kind_id.clone()), window.window_kind_id.clone(), window.template_id.clone(), window.corner.unwrap_or(WindowStackCorner::TopLeft)))
        .collect();
    let active = stack
        .active_window_kind_id
        .as_ref()
        .and_then(|id| windows.iter().find(|window| window.window_id == *id).or_else(|| windows.iter().find(|window| window.window_kind_id == *id)))
        .map(|window| window.window_id.clone())
        .or_else(|| windows.first().map(|window| window.window_id.clone()))
        .unwrap_or_default();
    DockNode::Stack { windows, active }
}

fn layout_window_node(tab: &DockStackTab) -> WindowLayoutWindowNode {
    WindowLayoutWindowNode {
        kind: "window".into(),
        window_kind_id: tab.window_kind_id.clone(),
        title: None,
        instance_id: (tab.window_id != tab.window_kind_id).then(|| tab.window_id.clone()),
        template_id: tab.template_id.clone(),
        corner: Some(tab.corner),
    }
}

pub fn dock_node_to_layout_root(node: &DockNode) -> WindowLayoutRoot {
    match node {
        DockNode::Stack { windows, active } => WindowLayoutRoot::Stack(WindowLayoutStackNode { kind: "stack".into(), size: None, active_window_kind_id: Some(active.clone()), children: windows.iter().map(layout_window_node).collect() }),
        DockNode::Row(children) => WindowLayoutRoot::Axis(semio_framework_ui_declaration::layout::WindowLayoutAxisNode { kind: "row".into(), size: None, children: children.iter().map(|(child, size)| dock_child_from_node(child, *size)).collect() }),
        DockNode::Column(children) => WindowLayoutRoot::Axis(semio_framework_ui_declaration::layout::WindowLayoutAxisNode { kind: "column".into(), size: None, children: children.iter().map(|(child, size)| dock_child_from_node(child, *size)).collect() }),
    }
}

fn dock_child_from_node(node: &DockNode, size: f32) -> WindowLayoutChild {
    match node {
        DockNode::Stack { windows, active } => WindowLayoutChild::Stack(WindowLayoutStackNode { kind: "stack".into(), size: Some(size as f64), active_window_kind_id: Some(active.clone()), children: windows.iter().map(layout_window_node).collect() }),
        DockNode::Row(children) => WindowLayoutChild::Axis(semio_framework_ui_declaration::layout::WindowLayoutAxisNode { kind: "row".into(), size: Some(size as f64), children: children.iter().map(|(child, child_size)| dock_child_from_node(child, *child_size)).collect() }),
        DockNode::Column(children) => {
            WindowLayoutChild::Axis(semio_framework_ui_declaration::layout::WindowLayoutAxisNode { kind: "column".into(), size: Some(size as f64), children: children.iter().map(|(child, child_size)| dock_child_from_node(child, *child_size)).collect() })
        }
    }
}

fn axis_pair_from_stacks(first: &DockNode, second: &DockNode, side: DockSide) -> DockNode {
    let pair = vec![(first.clone(), 0.5), (second.clone(), 0.5)];
    match side {
        DockSide::Left | DockSide::Right => DockNode::Row(pair),
        DockSide::Top | DockSide::Bottom => DockNode::Column(pair),
    }
}

fn replace_node_at(root: &mut DockNode, path: &[usize], replacement: DockNode) {
    if path.is_empty() {
        *root = replacement;
        return;
    }
    if let Some((head, tail)) = path.split_first() {
        let children = match root {
            DockNode::Row(children) | DockNode::Column(children) => children,
            DockNode::Stack { .. } => return,
        };
        if tail.is_empty() {
            if let Some((slot, _)) = children.get_mut(*head) {
                *slot = replacement;
            }
        } else if let Some((slot, _)) = children.get_mut(*head) {
            replace_node_at(slot, tail, replacement);
        }
    }
}

/// 🪟️ React's `{ kind: "stack", children: [] }` empty root (`🧱️elements/🎨️Canvas/🟦️.tsx:1478`).
pub(super) fn empty_stack() -> DockNode {
    DockNode::Stack { windows: vec![], active: String::new() }
}

/// 🪟️ One stack node from window ids plus the kind each id renders — the shape every split and
/// tab-join builds around.
pub(super) fn stack_of(windows: &[String], active_id: &str, window_kinds: &HashMap<String, String>) -> DockNode {
    DockNode::Stack { windows: windows.iter().map(|id| DockStackTab::instance(id.clone(), window_kinds.get(id).cloned().unwrap_or_else(|| id.clone()), WindowStackCorner::TopLeft)).collect(), active: active_id.to_string() }
}

/// 🗄️ Inserts every window of `windows` into the stack at `path`, at `corner`'s local `index`,
/// skipping any already present — React's `insertWindowAsTabAtCorner`/`mergeStackTabsIntoStack`
/// (`🧱️elements/🎨️Canvas/🟦️.tsx:355-363`, `:380-389`). Tree-only: focus bookkeeping belongs to the
/// caller, because the same insert serves both the committed tree and a derived one.
pub(super) fn insert_tabs_at_corner_in_node(root: &mut DockNode, path: &DockPath, windows: &[String], corner: WindowStackCorner, index: Option<usize>, active_id: &str, window_kinds: &HashMap<String, String>) -> bool {
    if windows.is_empty() {
        return false;
    }
    let Some(DockNode::Stack { windows: target, active }) = node_at_mut(root, path) else {
        return false;
    };
    let mut insert_at = flat_index_for_corner_insert(target, corner, index);
    let mut inserted_any = false;
    for window_id in windows {
        if target.iter().any(|tab| tab.window_id == *window_id) {
            continue;
        }
        target.insert(insert_at, DockStackTab::instance(window_id.clone(), window_kinds.get(window_id).cloned().unwrap_or_else(|| window_id.clone()), corner));
        insert_at += 1;
        inserted_any = true;
    }
    if !inserted_any {
        return false;
    }
    *active = active_id.to_string();
    true
}

/// 🪟️ Replaces the stack at `path` with an even two-child axis holding it and `incoming` — React's
/// `splitWithWindow`/`splitWithStack` (`🧱️elements/🎨️Canvas/🟦️.tsx:403-421`, `:437-445`).
pub(super) fn split_node_at_path(root: &mut DockNode, path: &DockPath, incoming: DockNode, side: DockSide) -> bool {
    let Some(target) = node_at(root, path).cloned() else {
        return false;
    };
    if !matches!(target, DockNode::Stack { .. }) {
        return false;
    }
    if matches!(&target, DockNode::Stack { windows, .. } if windows.is_empty()) {
        replace_node_at(root, path, incoming);
        return true;
    }
    let replacement = match side {
        DockSide::Left | DockSide::Top => axis_pair_from_stacks(&incoming, &target, side),
        DockSide::Right | DockSide::Bottom => axis_pair_from_stacks(&target, &incoming, side),
    };
    replace_node_at(root, path, replacement);
    true
}

pub(super) fn set_window_template(node: &mut DockNode, window_id: &str, template_id: Option<String>) -> bool {
    match node {
        DockNode::Stack { windows, .. } => {
            let Some(tab) = windows.iter_mut().find(|tab| tab.window_id == window_id) else { return false };
            tab.template_id = template_id;
            true
        }
        DockNode::Row(children) | DockNode::Column(children) => children.iter_mut().any(|(child, _)| set_window_template(child, window_id, template_id.clone())),
    }
}

/// 🪟️ Splits the mode root with `incoming` — React's `splitRootWithWindow`/`splitRootWithStack`
/// (`🧱️elements/🎨️Canvas/🟦️.tsx:414-421`). An empty root simply BECOMES `incoming`: there is nothing
/// to share the axis with.
pub(super) fn split_root_node(root: DockNode, incoming: DockNode, side: DockSide) -> DockNode {
    if matches!(&root, DockNode::Stack { windows, .. } if windows.is_empty()) {
        return incoming;
    }
    match side {
        DockSide::Left | DockSide::Top => axis_pair_from_stacks(&incoming, &root, side),
        DockSide::Right | DockSide::Bottom => axis_pair_from_stacks(&root, &incoming, side),
    }
}

/// 🪓️ Detaches the stack at `path`, answering `(remainder, detached)` — React's
/// `extractStackFromLayout` (`🧱️elements/🎨️Canvas/🟦️.tsx:421-434`). The remainder collapses by the
/// same rule every other removal uses; a `None` remainder is React's `null`, i.e. the mode is empty.
pub(super) fn extract_stack_from_layout(root: &DockNode, path: &DockPath) -> (Option<DockNode>, Option<DockNode>) {
    let Some(node) = node_at(root, path) else {
        return (Some(root.clone()), None);
    };
    if !matches!(node, DockNode::Stack { .. }) {
        return (Some(root.clone()), None);
    }
    let stack = node.clone();
    let Some((index, parent_path)) = path.split_last() else {
        return (None, Some(stack));
    };
    let Some(parent) = node_at(root, parent_path) else {
        return (Some(root.clone()), None);
    };
    let (DockNode::Row(children) | DockNode::Column(children)) = parent else {
        return (Some(root.clone()), None);
    };
    let kept: Vec<(DockNode, f32)> = children.iter().enumerate().filter(|(slot, _)| slot != index).map(|(_, child)| child.clone()).collect();
    let next_parent = collapse_layout_node(&rebuild_axis(parent, kept), 1.0).map(|(node, _)| node);
    if parent_path.is_empty() {
        return (next_parent, Some(stack));
    }
    let mut without = root.clone();
    replace_node_at(&mut without, parent_path, next_parent.unwrap_or_else(empty_stack));
    (collapse_layout_node(&without, 1.0).map(|(node, _)| node), Some(stack))
}

/// 🪓️ The tree a live drag renders: the dragged tab or stack lifted out — React's `modeDockOutLayout`
/// (`🧱️elements/🎨️Canvas/🟦️.tsx:841-847`). A tab drag whose window is the mode's last one keeps the
/// committed tree, exactly as React's `?? committed` does.
pub(super) fn dock_out_layout(root: &DockNode, drag: &DockDragPayload) -> DockNode {
    match drag.kind {
        DockDragKind::Stack => extract_stack_from_layout(root, &drag.source_path).0.unwrap_or_else(empty_stack),
        DockDragKind::Tab => remove_window_from_layout(root, &drag.window_id).unwrap_or_else(|| root.clone()),
        DockDragKind::NewWindow => root.clone(),
    }
}

/// 📱️ React's `mobileFlatStack` (`🧱️elements/🎨️Canvas/🟦️.tsx:1884-1891`): below the breakpoint there
/// is no split-pane window manager at all — every window of the tree becomes one tab of ONE stack,
/// in layout order, at the default corner, with the active window (or the first) focused.
pub(super) fn mobile_flat_stack(root: &DockNode, active_window_id: Option<&str>) -> DockNode {
    let mut windows = Vec::new();
    collect_flat_tabs(root, &mut windows);
    let active = active_window_id.filter(|id| windows.iter().any(|tab| tab.window_id == *id)).map(str::to_string).or_else(|| windows.first().map(|tab| tab.window_id.clone())).unwrap_or_default();
    DockNode::Stack { windows, active }
}

fn collect_flat_tabs(node: &DockNode, out: &mut Vec<DockStackTab>) {
    match node {
        DockNode::Stack { windows, .. } => out.extend(windows.iter().map(|tab| DockStackTab::instance_template(tab.window_id.clone(), tab.window_kind_id.clone(), tab.template_id.clone(), WindowStackCorner::TopLeft))),
        DockNode::Row(children) | DockNode::Column(children) => children.iter().for_each(|(child, _)| collect_flat_tabs(child, out)),
    }
}

pub fn resolve_split_side(local_x: f32, local_y: f32, width: f32, height: f32) -> DockSide {
    let mid_x = width * 0.5;
    let mid_y = height * 0.5;
    let dx = (local_x - mid_x).abs();
    let dy = (local_y - mid_y).abs();
    if dx >= dy {
        if local_x < mid_x {
            DockSide::Left
        } else {
            DockSide::Right
        }
    } else if local_y < mid_y {
        DockSide::Top
    } else {
        DockSide::Bottom
    }
}

pub fn compute_tab_insert_index(pointer_x: f32, tab_bar: Rect, tab_widths: &[f32], gap: f32) -> usize {
    let mut x = tab_bar.x + gap;
    for (index, width) in tab_widths.iter().enumerate() {
        if pointer_x < x + width * 0.5 {
            return index;
        }
        x += width + gap;
    }
    tab_widths.len()
}

/// 📥️ Tabs are painted edge to edge (`layout_stack_cap` advances by the chip width alone),
/// so the insert midpoints must be measured with no gap — a phantom gap drifted the split
/// point by `gap × index` and let a drop land one tab off at the end of a long bar.
pub fn compute_dock_drop_zone(pointer_x: f32, pointer_y: f32, tab_bars: &[(DockPath, WindowStackCorner, Rect, Vec<f32>)], bodies: &[(DockPath, Rect, String)], canvas: Rect) -> Option<DockDropZone> {
    for (path, corner, rect, widths) in tab_bars {
        if rect.contains(pointer_x, pointer_y) {
            let index = compute_tab_insert_index(pointer_x, *rect, widths, 0.0);
            return Some(DockDropZone::Tab { stack_path: path.clone(), corner: *corner, index });
        }
    }
    for (path, rect, _) in bodies {
        if rect.contains(pointer_x, pointer_y) {
            return Some(DockDropZone::Split { stack_path: path.clone(), side: resolve_split_side(pointer_x - rect.x, pointer_y - rect.y, rect.w, rect.h) });
        }
    }
    if canvas.contains(pointer_x, pointer_y) {
        return Some(DockDropZone::RootSplit { side: resolve_split_side(pointer_x - canvas.x, pointer_y - canvas.y, canvas.w, canvas.h) });
    }
    None
}

/// 📐️ Half-panel rectangle for split drop preview inside a stack body.
pub fn split_drop_preview_in_body(body: Rect, side: DockSide) -> Rect {
    let half_w = body.w * 0.5;
    let half_h = body.h * 0.5;
    match side {
        DockSide::Left => Rect::new(body.x, body.y, half_w, body.h),
        DockSide::Right => Rect::new(body.x + body.w - half_w, body.y, half_w, body.h),
        DockSide::Top => Rect::new(body.x, body.y, body.w, half_h),
        DockSide::Bottom => Rect::new(body.x, body.y + body.h - half_h, body.w, half_h),
    }
}

/// 🎯️ Resolves the on-canvas indicator rect for an active dock drop zone.
pub fn drop_zone_indicator_rect(zone: &DockDropZone, tab_bars: &[(DockPath, WindowStackCorner, Rect, Vec<f32>)], bodies: &[(DockPath, Rect, String)], canvas: Rect, gap: f32) -> Option<Rect> {
    match zone {
        DockDropZone::Tab { stack_path, corner, index } => {
            let (_, _, tab_bar, widths) = tab_bars.iter().find(|(path, bar_corner, _, _)| path == stack_path && bar_corner == corner)?;
            let mut x = tab_bar.x + gap;
            for width in widths.iter().take(*index) {
                x += width + gap;
            }
            let preview_w = widths.get(*index).copied().unwrap_or(88.0).clamp(48.0, 120.0);
            Some(Rect::new(x, tab_bar.y + gap * 0.5, preview_w, tab_bar.h - gap))
        }
        DockDropZone::Split { stack_path, side } => {
            let (_, body, _) = bodies.iter().find(|(path, _, _)| path == stack_path)?;
            Some(split_drop_preview_in_body(*body, *side))
        }
        DockDropZone::RootSplit { side } => Some(split_drop_preview_in_body(canvas, *side)),
    }
}

pub fn dock_cap_depth(theme: &Theme) -> f32 {
    theme.control_height + theme.padding_standard * 2.0
}

pub fn stack_tab_bar_rect(bounds: Rect, theme: &Theme) -> Rect {
    Rect::new(bounds.x, bounds.y, bounds.w, dock_cap_depth(theme))
}

pub(super) fn collect_stack_tab_bars(node: &DockNode, bounds: Rect, path: &[usize], theme: &Theme, out: &mut Vec<(DockPath, Rect)>) {
    match node {
        DockNode::Row(children) => {
            for slot in dock_axis_slots(children, bounds, true, theme.gap_standard) {
                let mut child_path = path.to_vec();
                child_path.push(slot.index);
                collect_stack_tab_bars(slot.node, slot.rect, &child_path, theme, out);
            }
        }
        DockNode::Column(children) => {
            for slot in dock_axis_slots(children, bounds, false, theme.gap_standard) {
                let mut child_path = path.to_vec();
                child_path.push(slot.index);
                collect_stack_tab_bars(slot.node, slot.rect, &child_path, theme, out);
            }
        }
        DockNode::Stack { .. } => {
            out.push((path.to_vec(), stack_tab_bar_rect(bounds, theme)));
        }
    }
}
//#endregion DockFreeFunctions
