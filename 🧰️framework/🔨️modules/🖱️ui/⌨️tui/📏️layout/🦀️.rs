pub use semio_ui_contract_presentation::window_layout_ops::window_id;
pub use semio_ui_contract_presentation::{Axis, WindowLayoutNode};
use semio_ui_tui_core::docking::DockPlacement;
pub use semio_ui_tui_core::docking::WindowStackCorner;
use semio_ui_tui_core::geometry::Rect;
use crate::scene::{NodeContent, NodeId, Scene};

/// 📐️ How one axis of a node's size is determined.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Dimension {
    #[default]
    Auto,
    Cells(u16),
    Weight(u16),
}

/// ↔ How a node arranges its children.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Direction {
    #[default]
    Row,
    Column,
    Stack,
}

/// 📦️ A node's layout intent.
#[derive(Clone, Copy, Debug, Default)]
pub struct Constraint {
    pub direction: Direction,
    pub width: Dimension,
    pub height: Dimension,
    pub gap: u16,
    /// 🧲️ Share one boundary cell between neighboring children; dimensions describe their exclusive spans.
    pub shared: bool,
    pub padding: [u16; 4],
}

fn measure(scene: &Scene, id: NodeId) -> (u16, u16) {
    match &scene.node(id).content {
        NodeContent::Text(s) => (semio_ui_tui_core::text::display_width(s), 1),
        NodeContent::Widget(w) => {
            let size = w.preferred_size();
            (size.width, size.height)
        }
        _ => (0, 0),
    }
}

fn distribute(dims: &[Dimension], measured: &[u16], total: u16, gap: u16) -> Vec<u16> {
    let n = dims.len();
    if n == 0 {
        return Vec::new();
    }
    let gaps = gap.saturating_mul(n.saturating_sub(1) as u16);
    let mut sizes = vec![0u16; n];
    let mut weight_total = 0u32;
    let mut fixed_total = gaps;
    for (i, d) in dims.iter().enumerate() {
        match d {
            Dimension::Cells(c) => {
                sizes[i] = *c;
                fixed_total += c;
            }
            Dimension::Auto => {
                sizes[i] = measured[i];
                fixed_total += measured[i];
            }
            Dimension::Weight(w) => weight_total += u32::from(*w),
        }
    }
    let remaining = u32::from(total).saturating_sub(u32::from(fixed_total));
    if let Some(weight_total) = std::num::NonZeroU32::new(weight_total) {
        let mut remainders: Vec<(usize, u32)> = Vec::new();
        let mut used = 0u32;
        for (i, d) in dims.iter().enumerate() {
            if let Dimension::Weight(w) = d {
                let share = remaining * u32::from(*w) / weight_total;
                sizes[i] = share as u16;
                used += share;
                remainders.push((i, remaining * u32::from(*w) % weight_total));
            }
        }
        let mut leftover = remaining.saturating_sub(used);
        remainders.sort_by_key(|remainder| std::cmp::Reverse(remainder.1));
        for (i, _) in remainders {
            if leftover == 0 {
                break;
            }
            sizes[i] += 1;
            leftover -= 1;
        }
    }
    sizes
}

fn distribute_shared(dims: &[Dimension], measured: &[u16], extent: u16) -> Vec<u16> {
    if extent == 0 { return vec![0; dims.len()]; }
    distribute(dims, measured, extent - 1, 0).into_iter().map(|size| size + 1).collect()
}

/// 🧮️ Recomputes rects for the whole tree from `viewport` down (no-operation-safe to call every frame).
pub fn solve(scene: &mut Scene, viewport: Rect) {
    layout_node(scene, scene.root(), viewport);
}

fn clear_rects(scene: &mut Scene, id: NodeId) {
    scene.node_raw_mut(id).rect = Rect::default();
    for child in scene.node(id).children().to_vec() { clear_rects(scene, child); }
}

fn layout_node(scene: &mut Scene, id: NodeId, rect: Rect) {
    if !scene.node(id).visible { clear_rects(scene, id); return; }
    scene.node_raw_mut(id).rect = rect;
    let constraint = scene.node(id).constraint;
    let [top, right, bottom, left] = match &scene.node(id).content {
        NodeContent::Chrome(crate::chrome::ChromeState::Window(window)) => crate::chrome::window_content_padding(window, rect),
        _ => constraint.padding,
    };
    let inner = rect.inset_sides(top, right, bottom, left);
    let siblings = scene.node(id).children().to_vec();
    let children: Vec<NodeId> = siblings.iter().copied().filter(|child| scene.node(*child).visible).collect();
    for child in siblings { if !scene.node(child).visible { clear_rects(scene, child); } }
    if children.is_empty() {
        return;
    }
    match constraint.direction {
        Direction::Stack => {
            for &child in &children {
                layout_node(scene, child, inner);
            }
        }
        Direction::Row => {
            let dims: Vec<Dimension> = children.iter().map(|c| scene.node(*c).constraint.width).collect();
            let measured: Vec<u16> = children.iter().map(|c| measure(scene, *c).0).collect();
            let shared = constraint.shared || matches!(scene.node(id).content, NodeContent::Axis(_));
            let sizes = if shared { distribute_shared(&dims, &measured, inner.width) } else { distribute(&dims, &measured, inner.width, constraint.gap) };
            let mut x = inner.x;
            for (i, &child) in children.iter().enumerate() {
                let w = sizes[i];
                layout_node(scene, child, Rect::new(x, inner.y, w, inner.height));
                x += if shared { w.saturating_sub(1) } else { w + constraint.gap };
            }
        }
        Direction::Column => {
            let dims: Vec<Dimension> = children.iter().map(|c| scene.node(*c).constraint.height).collect();
            let measured: Vec<u16> = children.iter().map(|c| measure(scene, *c).1).collect();
            let shared = constraint.shared || matches!(scene.node(id).content, NodeContent::Axis(_));
            let sizes = if shared { distribute_shared(&dims, &measured, inner.height) } else { distribute(&dims, &measured, inner.height, constraint.gap) };
            let mut y = inner.y;
            for (i, &child) in children.iter().enumerate() {
                let h = sizes[i];
                layout_node(scene, child, Rect::new(inner.x, y, inner.width, h));
                y += if shared { h.saturating_sub(1) } else { h + constraint.gap };
            }
        }
    }
}

//#region 🔖️WindowLayout
/// 🤏 The smallest share a tiled stack keeps when a splitter is dragged.
pub const SPLIT_MIN_FRACTION: f64 = 0.08;

type N = WindowLayoutNode;

/// 🌳️ A full tiling window arrangement: the schema-owned contract tree plus the TUI's own view state.
#[derive(Clone, Debug, PartialEq)]
pub struct WindowLayout {
    pub root: WindowLayoutNode,
    /// 🖼️ When set, only this window fills the canvas (zoom / maximize); view state, never part of the contract tree.
    pub zoomed: Option<String>,
}

/// 🧩 The resolved on-screen placement of one visible window.
#[derive(Clone, Debug, PartialEq)]
pub struct WindowMeasure {
    pub window_kind_id: String,
    pub title: Option<String>,
    pub rect: Rect,
    pub active: bool,
    pub stack_tabs: Vec<String>,
}

/// 🪟 One tiled window leaf: which content it hosts, its display title and its tab corner.
pub fn window_node(window_kind_id: String, title: Option<String>, corner: Option<WindowStackCorner>) -> WindowLayoutNode {
    N::Window { window_kind_id, title, instance_id: None, template_id: None, corner }
}

/// 🗂️ A stack of windows sharing one area; only `active` is visible.
pub fn stack_node(size: Option<f64>, active: Option<String>, children: Vec<WindowLayoutNode>) -> WindowLayoutNode {
    N::Stack { size, active_window_id: active, children }
}

/// 🧱️ A row (`Axis::Horizontal`) or column (`Axis::Vertical`) of tiled children.
pub fn axis_node(axis: Axis, size: Option<f64>, children: Vec<WindowLayoutNode>) -> WindowLayoutNode {
    N::Split { axis, size, children }
}

/// 🏷️ The display label of a window leaf: its title, else its id.
pub fn window_label(node: &WindowLayoutNode) -> String {
    match node {
        N::Window { title: Some(title), .. } => title.clone(),
        _ => window_id(node).to_string(),
    }
}

/// 🪟 The window leaves of a stack node (empty for any other node).
pub fn stack_windows(stack: &WindowLayoutNode) -> &[WindowLayoutNode] {
    match stack {
        N::Stack { children, .. } => children,
        _ => &[],
    }
}

/// 🪟 Every window leaf of the layout tree, in tree order.
pub fn layout_windows_mut(layout: &mut WindowLayout) -> Vec<&mut WindowLayoutNode> {
    fn walk<'a>(node: &'a mut N, out: &mut Vec<&'a mut N>) {
        match node {
            N::Window { .. } => out.push(node),
            N::Stack { children, .. } | N::Split { children, .. } => children.iter_mut().for_each(|child| walk(child, out)),
        }
    }
    let mut out = Vec::new();
    walk(&mut layout.root, &mut out);
    out
}

/// 📐️ The corner a window leaf docks its tab at.
pub fn window_corner(node: &WindowLayoutNode) -> WindowStackCorner {
    match node {
        N::Window { corner, .. } => corner.unwrap_or_default(),
        _ => WindowStackCorner::TopLeft,
    }
}

/// 📐️ The corner a window leaf explicitly docks its tab at, if one was set.
pub fn window_corner_set(node: &WindowLayoutNode) -> Option<WindowStackCorner> {
    match node {
        N::Window { corner, .. } => *corner,
        _ => None,
    }
}

fn set_window_corner(node: &mut N, next: Option<WindowStackCorner>) {
    if let N::Window { corner, .. } = node {
        *corner = next;
    }
}

fn node_size(node: &N) -> Option<f64> {
    match node {
        N::Window { .. } => None,
        N::Stack { size, .. } | N::Split { size, .. } => *size,
    }
}

fn set_node_size(node: &mut N, next: Option<f64>) {
    if let N::Stack { size, .. } | N::Split { size, .. } = node {
        *size = next;
    }
}

fn axis_child_size(child: &N) -> f64 {
    node_size(child).unwrap_or(1.0)
}

/// ⚖️ The integer weight a layout size maps to, shared by the pure solver and the mounted scene tree.
pub(crate) fn weight_of(size: Option<f64>) -> u16 {
    ((size.unwrap_or(1.0) * 1000.0).round() as u16).max(1)
}

fn axis_cells(children: &[N], extent: u16) -> Vec<u16> {
    let dims: Vec<Dimension> = children.iter().map(|child| Dimension::Weight(weight_of(Some(axis_child_size(child))))).collect();
    distribute_shared(&dims, &vec![0; dims.len()], extent)
}

fn solve_axis(axis: Axis, children: &[N], area: Rect, out: &mut Vec<WindowMeasure>) {
    let is_row = axis == Axis::Horizontal;
    let sizes = axis_cells(children, if is_row { area.width } else { area.height });
    let mut offset = 0u16;
    for (child, size) in children.iter().zip(sizes) {
        let child_rect = if is_row { Rect::new(area.x + offset, area.y, size, area.height) } else { Rect::new(area.x, area.y + offset, area.width, size) };
        solve_node(child, child_rect, out);
        offset += size.saturating_sub(1);
    }
}

fn solve_stack(node: &N, area: Rect, out: &mut Vec<WindowMeasure>) {
    let N::Stack { active_window_id, children, .. } = node else { return };
    if children.is_empty() {
        return;
    }
    let tabs: Vec<String> = children.iter().map(|c| window_id(c).to_string()).collect();
    let active = active_window_id.clone().unwrap_or_else(|| window_id(&children[0]).to_string());
    let title = children.iter().find(|c| window_id(c) == active).and_then(|c| match c {
        N::Window { title, .. } => title.clone(),
        _ => None,
    });
    out.push(WindowMeasure { window_kind_id: active, title, rect: area, active: true, stack_tabs: tabs });
}

fn solve_node(node: &N, area: Rect, out: &mut Vec<WindowMeasure>) {
    match node {
        N::Split { axis, children, .. } => solve_axis(*axis, children, area, out),
        N::Stack { .. } => solve_stack(node, area, out),
        N::Window { .. } => {}
    }
}

fn find_stack_measure(node: &N, id: &str, area: Rect) -> Option<WindowMeasure> {
    match node {
        N::Stack { active_window_id: _, size, children } if children.iter().any(|c| window_id(c) == id) => {
            let mut out = Vec::new();
            solve_stack(&N::Stack { size: *size, active_window_id: Some(id.to_string()), children: children.clone() }, area, &mut out);
            out.into_iter().next()
        }
        N::Split { children, .. } => children.iter().find_map(|c| find_stack_measure(c, id, area)),
        _ => None,
    }
}

/// 🗺️ Resolves a `WindowLayout` into concrete on-screen `WindowMeasure`s.
pub fn solve_window_layout(layout: &WindowLayout, area: Rect) -> Vec<WindowMeasure> {
    if let Some(zid) = layout.zoomed.as_deref() {
        if let Some(m) = find_stack_measure(&layout.root, zid, area) {
            return vec![m];
        }
    }
    let mut out = Vec::new();
    solve_node(&layout.root, area, &mut out);
    out
}

/// 🏗️ Builds a row/column layout of individually-sized windows (one single-window stack each, active on its window). The TUI keeps a one-window axis so the mounted splitter chrome is identical for every window count.
pub fn create_default_layout(window_ids: &[String], axis: Axis, sizes: Option<&[f64]>, titles: Option<&[String]>) -> WindowLayout {
    let children = window_ids
        .iter()
        .enumerate()
        .map(|(i, id)| stack_node(sizes.and_then(|s| s.get(i)).copied(), Some(id.clone()), vec![window_node(id.clone(), titles.and_then(|t| t.get(i)).cloned(), None)]))
        .collect();
    WindowLayout { root: axis_node(axis, None, children), zoomed: None }
}

/// 🟰 Builds an evenly-weighted row layout.
pub fn even_window_layout(window_ids: &[String]) -> WindowLayout {
    create_default_layout(window_ids, Axis::Horizontal, None, None)
}

//#region 🔖️WindowLayoutMutations
fn stack_contains(node: &N, id: &str) -> bool {
    matches!(node, N::Stack { children, .. } if children.iter().any(|c| window_id(c) == id))
}

fn stack_parts(node: &mut N) -> Option<(&mut Option<f64>, &mut Option<String>, &mut Vec<N>)> {
    match node {
        N::Stack { size, active_window_id, children } => Some((size, active_window_id, children)),
        _ => None,
    }
}

fn corner_slots(children: &[N], corner: WindowStackCorner) -> Vec<usize> {
    children.iter().enumerate().filter_map(|(i, node)| (window_corner(node) == corner).then_some(i)).collect()
}

fn take_window_node(node: &mut N, id: &str) -> Option<N> {
    match node {
        N::Stack { active_window_id, children, .. } => {
            let i = children.iter().position(|c| window_id(c) == id)?;
            let taken = children.remove(i);
            if active_window_id.as_deref() == Some(id) {
                *active_window_id = children.first().map(|c| window_id(c).to_string());
            }
            Some(taken)
        }
        N::Split { children, .. } => children.iter_mut().find_map(|c| take_window_node(c, id)),
        N::Window { .. } => None,
    }
}

fn with_stack_mut(node: &mut N, id: &str, f: &mut dyn FnMut(&mut N) -> bool) -> bool {
    if stack_contains(node, id) {
        return f(node);
    }
    match node {
        N::Split { children, .. } => children.iter_mut().any(|c| with_stack_mut(c, id, f)),
        _ => false,
    }
}

/// 🔍 Zooms `window_kind_id` to fill the canvas; `None` restores the tiling.
pub fn zoom_window(layout: &mut WindowLayout, window_kind_id: Option<&str>) {
    layout.zoomed = window_kind_id.map(str::to_string);
}

/// 🏷️ Activates `tab_id` inside its stack.
pub fn activate_stack_tab(layout: &mut WindowLayout, tab_id: &str) -> bool {
    let zoomed = layout.zoomed.as_deref();
    let mut keep_zoom = false;
    let activated = with_stack_mut(&mut layout.root, tab_id, &mut |stack| {
        keep_zoom = zoomed.is_some_and(|id| stack_contains(stack, id));
        if let Some((_, active, _)) = stack_parts(stack) {
            *active = Some(tab_id.into());
        }
        true
    });
    if keep_zoom { layout.zoomed = Some(tab_id.into()); }
    activated
}

/// 🔁 Cycles the active tab in the stack containing `window_kind_id` by `delta`.
pub fn cycle_stack_tab(layout: &mut WindowLayout, window_kind_id: &str, delta: i32) -> bool {
    with_stack_mut(&mut layout.root, window_kind_id, &mut |s| {
        let Some((_, active, children)) = stack_parts(s) else { return false };
        if children.is_empty() {
            return false;
        }
        let cur = active.as_deref().and_then(|id| children.iter().position(|c| window_id(c) == id)).unwrap_or(0);
        let len = children.len() as i32;
        let next = ((cur as i32 + delta).rem_euclid(len)) as usize;
        *active = Some(window_id(&children[next]).to_string());
        true
    })
}

/// ✂️ Splits the stack containing `window_kind_id` into an axis of two equal stacks.
pub fn split_window(layout: &mut WindowLayout, window_kind_id: &str, axis: Axis, new_id: &str, new_title: Option<String>) -> bool {
    let new_stack = stack_node(Some(1.0), Some(new_id.to_string()), vec![window_node(new_id.to_string(), new_title, None)]);
    fn split_in_axis(children: &mut [N], window_kind_id: &str, axis: Axis, new_stack: &N) -> bool {
        for i in 0..children.len() {
            if stack_contains(&children[i], window_kind_id) {
                let parent_size = axis_child_size(&children[i]);
                let mut old = children[i].clone();
                set_node_size(&mut old, Some(1.0));
                children[i] = axis_node(axis, Some(parent_size), vec![old, new_stack.clone()]);
                return true;
            }
            if let N::Split { children: inner, .. } = &mut children[i] {
                if split_in_axis(inner, window_kind_id, axis, new_stack) {
                    return true;
                }
            }
        }
        false
    }
    if stack_contains(&layout.root, window_kind_id) {
        let mut old = layout.root.clone();
        set_node_size(&mut old, Some(1.0));
        layout.root = axis_node(axis, None, vec![old, new_stack]);
        return true;
    }
    match &mut layout.root {
        N::Split { children, .. } => split_in_axis(children, window_kind_id, axis, &new_stack),
        _ => false,
    }
}

/// 🛠️ Nudges the weight of the stack containing `window_kind_id` by `delta`; a direct sibling slot absorbs `-delta`, and a deeper nesting nudges the level of its immediate match stack.
pub fn resize_window(layout: &mut WindowLayout, window_kind_id: &str, delta: f64) -> bool {
    fn child_has(id: &str, child: &N) -> bool {
        match child {
            N::Stack { .. } => stack_contains(child, id),
            N::Split { children, .. } => children.iter().any(|c| child_has(id, c)),
            N::Window { .. } => false,
        }
    }
    fn nudge(children: &mut [N], window_kind_id: &str, delta: f64) -> bool {
        if let Some(i) = children.iter().position(|c| child_has(window_kind_id, c)) {
            if stack_contains(&children[i], window_kind_id) {
                let j = if i + 1 < children.len() {
                    i + 1
                } else if i > 0 {
                    i - 1
                } else {
                    return false;
                };
                let a = axis_child_size(&children[i]);
                let b = axis_child_size(&children[j]);
                set_node_size(&mut children[i], Some((a + delta).max(0.05)));
                set_node_size(&mut children[j], Some((b - delta).max(0.05)));
                return true;
            }
        }
        for c in children.iter_mut() {
            if let N::Split { children: inner, .. } = c {
                if nudge(inner, window_kind_id, delta) {
                    return true;
                }
            }
        }
        false
    }
    match &mut layout.root {
        N::Split { children, .. } => nudge(children, window_kind_id, delta),
        _ => false,
    }
}

/// 🚚 Moves `window_kind_id` into the stack that hosts `target_window_kind_id` as a new tab.
pub fn move_window_to_stack(layout: &mut WindowLayout, window_kind_id: &str, target_window_kind_id: &str) -> bool {
    dock_window(layout, window_kind_id, target_window_kind_id, DockPlacement::Tabs, None)
}

/// 🧹️ Removes empty groups and collapses single-child axes while retaining the parent's weight.
fn compact_layout(layout: &mut WindowLayout) {
    fn compact(node: N) -> Option<N> {
        match node {
            N::Stack { size, active_window_id, children } => (!children.is_empty()).then_some(N::Stack { size, active_window_id, children }),
            N::Split { axis, size, children } => {
                let mut kept: Vec<N> = children.into_iter().filter_map(compact).collect();
                match kept.len() {
                    0 => None,
                    1 => {
                        let mut only = kept.remove(0);
                        set_node_size(&mut only, size);
                        Some(only)
                    }
                    _ => Some(N::Split { axis, size, children: kept }),
                }
            }
            window => Some(window),
        }
    }
    let root = std::mem::replace(&mut layout.root, stack_node(None, None, vec![]));
    layout.root = compact(root).unwrap_or_else(|| stack_node(None, None, vec![]));
}

/// 🧲️ Commits one docking command atomically, preserving the held window and pruning its vacated group.
pub fn dock_window(layout: &mut WindowLayout, source: &str, target: &str, placement: DockPlacement, tab_index: Option<usize>) -> bool {
    if stack_hosting(layout, target).is_none() { return false; }
    if source == target {
        let DockPlacement::Corner(corner) = placement else { return false };
        return with_stack_mut(&mut layout.root, source, &mut |stack| {
            let Some((_, active, children)) = stack_parts(stack) else { return false };
            let Some(index) = children.iter().position(|node| window_id(node) == source) else { return false };
            let mut node = children.remove(index);
            set_window_corner(&mut node, Some(corner));
            let slots = corner_slots(children, corner);
            let insert = tab_index.and_then(|i| slots.get(i).copied()).unwrap_or_else(|| slots.last().map_or(children.len(), |i| i + 1));
            children.insert(insert, node);
            *active = Some(source.into());
            true
        });
    }
    let mut next = layout.clone();
    let Some(mut node) = take_window_node(&mut next.root, source) else { return false; };
    if let DockPlacement::Corner(corner) = placement { set_window_corner(&mut node, Some(corner)); }
    fn place(target_node: &mut N, target: &str, node: &N, placement: DockPlacement, index: Option<usize>) -> bool {
        if stack_contains(target_node, target) {
            if matches!(placement, DockPlacement::Tabs | DockPlacement::Corner(_)) {
                let Some((_, active, children)) = stack_parts(target_node) else { return false };
                let insert = if let DockPlacement::Corner(corner) = placement {
                    let slots = corner_slots(children, corner);
                    index.and_then(|i| slots.get(i).copied()).unwrap_or_else(|| slots.last().map_or(children.len(), |i| i + 1))
                } else { index.unwrap_or(children.len()).min(children.len()) };
                children.insert(insert, node.clone());
                *active = Some(window_id(node).to_string());
            } else {
                let parent_size = node_size(target_node);
                let mut old = target_node.clone();
                set_node_size(&mut old, Some(1.0));
                let fresh = stack_node(Some(1.0), Some(window_id(node).to_string()), vec![node.clone()]);
                let before = matches!(placement, DockPlacement::Left | DockPlacement::Top);
                *target_node = axis_node(
                    if matches!(placement, DockPlacement::Left | DockPlacement::Right) { Axis::Horizontal } else { Axis::Vertical },
                    parent_size,
                    if before { vec![fresh, old] } else { vec![old, fresh] },
                );
            }
            return true;
        }
        match target_node {
            N::Split { children, .. } => children.iter_mut().any(|child| place(child, target, node, placement, index)),
            _ => false,
        }
    }
    if !place(&mut next.root, target, &node, placement, tab_index) { return false; }
    next.zoomed = None;
    compact_layout(&mut next);
    *layout = next;
    true
}

/// 🗑️ Removes `window_kind_id` from the layout tree.
pub fn remove_window(layout: &mut WindowLayout, window_kind_id: &str) -> bool {
    let removed = take_window_node(&mut layout.root, window_kind_id).is_some();
    if removed { compact_layout(layout); }
    removed
}

/// 📎️ Appends a window tab to the stack containing `host_window_kind_id`.
pub fn push_window_to_stack(layout: &mut WindowLayout, host_window_kind_id: &str, node: WindowLayoutNode) -> bool {
    let id = window_id(&node).to_string();
    let mut node = Some(node);
    with_stack_mut(&mut layout.root, host_window_kind_id, &mut |s| {
        let Some((_, active, children)) = stack_parts(s) else { return false };
        match node.take() {
            Some(node) => {
                children.push(node);
                *active = Some(id.clone());
                true
            }
            None => false,
        }
    })
}

/// 🔭 The stack node that hosts `window_kind_id`, cloned.
pub fn stack_hosting(layout: &WindowLayout, window_kind_id: &str) -> Option<WindowLayoutNode> {
    fn walk(node: &N, id: &str) -> Option<N> {
        match node {
            N::Stack { .. } => stack_contains(node, id).then(|| node.clone()),
            N::Split { children, .. } => children.iter().find_map(|c| walk(c, id)),
            N::Window { .. } => None,
        }
    }
    walk(&layout.root, window_kind_id)
}

/// 🪪 Sets the display title of `window_kind_id` on its layout node.
pub fn set_window_title(layout: &mut WindowLayout, window_kind_id: &str, title: Option<String>) -> bool {
    let mut done = false;
    let mut title = Some(title);
    with_stack_mut(&mut layout.root, window_kind_id, &mut |s| {
        let Some((_, _, children)) = stack_parts(s) else { return false };
        if let Some(N::Window { title: slot, .. }) = children.iter_mut().find(|c| window_id(c) == window_kind_id) {
            *slot = title.take().flatten();
            done = true;
        }
        done
    });
    done
}

/// 🔀 Moves the stack tab at `from` to `to` inside the stack hosting `window_kind_id` (a tab drag).
pub fn reorder_stack_tab(layout: &mut WindowLayout, window_kind_id: &str, from: usize, to: usize) -> bool {
    with_stack_mut(&mut layout.root, window_kind_id, &mut |s| {
        let Some((_, _, children)) = stack_parts(s) else { return false };
        if from == to || from >= children.len() || to >= children.len() {
            return false;
        }
        let node = children.remove(from);
        children.insert(to, node);
        true
    })
}

/// 🪝 Applies a `SplitterDragged { path, delta }` to `layout` laid out over `area`: the stack before the splitter grows by
/// `delta` cells, the one after shrinks by as much; neither goes under `SPLIT_MIN_FRACTION` of the axis.
pub fn resize_split(layout: &mut WindowLayout, area: Rect, path: &[usize], delta: i16) -> bool {
    let Some((&index, axis_path)) = path.split_last() else { return false };
    let N::Split { axis: root_axis, children: root_children, .. } = &mut layout.root else { return false };
    let mut axis = *root_axis;
    let mut children: &mut Vec<N> = root_children;
    let mut rect = area;
    for &step in axis_path {
        let is_row = axis == Axis::Horizontal;
        let sizes = axis_cells(children, if is_row { rect.width } else { rect.height });
        let before: u16 = sizes.iter().take(step).map(|s| s.saturating_sub(1)).sum();
        let Some(size) = sizes.get(step).copied() else { return false };
        rect = if is_row { Rect::new(rect.x + before, rect.y, size, rect.height) } else { Rect::new(rect.x, rect.y + before, rect.width, size) };
        let Some(N::Split { axis: next_axis, children: next, .. }) = children.get_mut(step) else { return false };
        axis = *next_axis;
        children = next;
    }
    let is_row = axis == Axis::Horizontal;
    let extent = if is_row { rect.width } else { rect.height };
    let sizes = axis_cells(children, extent);
    if index + 1 >= sizes.len() {
        return false;
    }
    let usable = f64::from(extent.saturating_sub(1)).max(1.0);
    let floor = ((usable * SPLIT_MIN_FRACTION).round() as i32).max(1);
    let (a, b) = (i32::from(sizes[index].saturating_sub(1)), i32::from(sizes[index + 1].saturating_sub(1)));
    if a + b < 2 * floor {
        return false;
    }
    let moved = i32::from(delta).clamp(floor - a, b - floor);
    if moved == 0 {
        return false;
    }
    let total_weight: f64 = children.iter().map(axis_child_size).sum();
    let per_cell = total_weight / usable;
    set_node_size(&mut children[index], Some(f64::from(a + moved) * per_cell));
    set_node_size(&mut children[index + 1], Some(f64::from(b - moved) * per_cell));
    true
}
//#endregion 🔖️WindowLayoutMutations

//#endregion 🔖️WindowLayout
