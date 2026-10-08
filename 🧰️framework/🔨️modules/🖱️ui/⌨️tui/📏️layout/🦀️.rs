use crate::tui::geometry::Rect;
use crate::tui::scene::{NodeContent, NodeId, Scene};

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
    pub padding: [u16; 4],
}

fn measure(scene: &Scene, id: NodeId) -> (u16, u16) {
    match &scene.node(id).content {
        NodeContent::Text(s) => (crate::tui::text::display_width(s), 1),
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

/// 🧮️ Recomputes rects for the whole tree from `viewport` down (no-operation-safe to call every frame).
pub fn solve(scene: &mut Scene, viewport: Rect) {
    layout_node(scene, scene.root(), viewport);
}

fn layout_node(scene: &mut Scene, id: NodeId, rect: Rect) {
    scene.node_raw_mut(id).rect = rect;
    let constraint = scene.node(id).constraint;
    let [top, right, bottom, left] = match &scene.node(id).content {
        NodeContent::Chrome(crate::tui::chrome::ChromeState::Window(window)) => crate::tui::chrome::window_content_padding(window, rect),
        _ => constraint.padding,
    };
    let inner = rect.inset_sides(top, right, bottom, left);
    let children: Vec<NodeId> = scene.node(id).children().to_vec();
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
            let sizes = distribute(&dims, &measured, inner.width, constraint.gap);
            let mut x = inner.x;
            for (i, &child) in children.iter().enumerate() {
                let w = sizes[i];
                layout_node(scene, child, Rect::new(x, inner.y, w, inner.height));
                x += w + constraint.gap;
            }
        }
        Direction::Column => {
            let dims: Vec<Dimension> = children.iter().map(|c| scene.node(*c).constraint.height).collect();
            let measured: Vec<u16> = children.iter().map(|c| measure(scene, *c).1).collect();
            let sizes = distribute(&dims, &measured, inner.height, constraint.gap);
            let mut y = inner.y;
            for (i, &child) in children.iter().enumerate() {
                let h = sizes[i];
                layout_node(scene, child, Rect::new(inner.x, y, inner.width, h));
                y += h + constraint.gap;
            }
        }
    }
}

//#region 🔖️WindowLayout
/// 📏️ The gutter between tiled stacks: the draggable splitter.
pub const WINDOW_GAP: u16 = 1;

/// 🤏 The smallest share a tiled stack keeps when a splitter is dragged.
pub const SPLIT_MIN_FRACTION: f64 = 0.08;

/// 🧭️ Corner of a window stack where a tab chip docks.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WindowStackCorner {
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl WindowStackCorner {
    pub fn is_top(self) -> bool {
        matches!(self, Self::TopLeft | Self::TopRight)
    }

    pub fn is_left(self) -> bool {
        matches!(self, Self::TopLeft | Self::BottomLeft)
    }
}

/// 🪟 One tiled window leaf: which content it hosts and its display title.
#[derive(Clone, Debug, PartialEq)]
pub struct WindowLayoutWindowNode {
    pub window_kind_id: String,
    pub title: Option<String>,
    pub corner: Option<WindowStackCorner>,
}

/// 🗂️ A stack of windows sharing one area; only `active_window_kind_id` is visible.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct WindowLayoutStackNode {
    pub size: Option<f64>,
    pub active_window_kind_id: Option<String>,
    pub children: Vec<WindowLayoutWindowNode>,
}

/// 🧱️ A row or column of tiled children.
#[derive(Clone, Debug, PartialEq)]
pub struct WindowLayoutAxisNode {
    pub kind: String,
    pub size: Option<f64>,
    pub children: Vec<WindowLayoutChild>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WindowLayoutChild {
    Axis(WindowLayoutAxisNode),
    Stack(WindowLayoutStackNode),
}

#[derive(Clone, Debug, PartialEq)]
pub enum WindowLayoutRoot {
    Axis(WindowLayoutAxisNode),
    Stack(WindowLayoutStackNode),
}

/// 🌳️ A full tiling window arrangement (rows/columns/stacks with weights).
#[derive(Clone, Debug, PartialEq)]
pub struct WindowLayout {
    pub root: WindowLayoutRoot,
    /// 🖼️ When set, only this window fills the canvas (zoom / maximize).
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

fn axis_child_size(child: &WindowLayoutChild) -> f64 {
    match child {
        WindowLayoutChild::Axis(a) => a.size.unwrap_or(1.0),
        WindowLayoutChild::Stack(s) => s.size.unwrap_or(1.0),
    }
}

/// ⚖️ The integer weight a layout size maps to, shared by the pure solver and the mounted scene tree.
pub(crate) fn weight_of(size: Option<f64>) -> u16 {
    ((size.unwrap_or(1.0) * 100.0).round() as u16).max(1)
}

fn axis_cells(node: &WindowLayoutAxisNode, extent: u16) -> Vec<u16> {
    let dims: Vec<Dimension> = node.children.iter().map(|child| Dimension::Weight(weight_of(Some(axis_child_size(child))))).collect();
    distribute(&dims, &vec![0; dims.len()], extent, WINDOW_GAP)
}

fn solve_axis(node: &WindowLayoutAxisNode, area: Rect, out: &mut Vec<WindowMeasure>) {
    let is_row = node.kind == "row";
    let sizes = axis_cells(node, if is_row { area.width } else { area.height });
    let mut offset = 0u16;
    for (child, size) in node.children.iter().zip(sizes) {
        let child_rect = if is_row { Rect::new(area.x + offset, area.y, size, area.height) } else { Rect::new(area.x, area.y + offset, area.width, size) };
        match child {
            WindowLayoutChild::Axis(a) => solve_axis(a, child_rect, out),
            WindowLayoutChild::Stack(s) => solve_stack(s, child_rect, out),
        }
        offset += size + WINDOW_GAP;
    }
}

fn solve_stack(node: &WindowLayoutStackNode, area: Rect, out: &mut Vec<WindowMeasure>) {
    if node.children.is_empty() {
        return;
    }
    let tabs: Vec<String> = node.children.iter().map(|c| c.window_kind_id.clone()).collect();
    let active = node.active_window_kind_id.clone().unwrap_or_else(|| node.children[0].window_kind_id.clone());
    let title = node.children.iter().find(|c| c.window_kind_id == active).and_then(|c| c.title.clone());
    out.push(WindowMeasure { window_kind_id: active, title, rect: area, active: true, stack_tabs: tabs });
}

fn find_stack_measure(node: &WindowLayoutChild, id: &str, area: Rect) -> Option<WindowMeasure> {
    match node {
        WindowLayoutChild::Stack(s) => {
            if s.children.iter().any(|c| c.window_kind_id == id) {
                let mut out = Vec::new();
                let mut forced = s.clone();
                forced.active_window_kind_id = Some(id.to_string());
                solve_stack(&forced, area, &mut out);
                out.into_iter().next()
            } else {
                None
            }
        }
        WindowLayoutChild::Axis(a) => a.children.iter().find_map(|c| find_stack_measure(c, id, area)),
    }
}

fn find_stack_measure_root(root: &WindowLayoutRoot, id: &str, area: Rect) -> Option<WindowMeasure> {
    match root {
        WindowLayoutRoot::Stack(s) => find_stack_measure(&WindowLayoutChild::Stack(s.clone()), id, area),
        WindowLayoutRoot::Axis(a) => a.children.iter().find_map(|c| find_stack_measure(c, id, area)),
    }
}

/// 🗺️ Resolves a `WindowLayout` into concrete on-screen `WindowMeasure`s.
pub fn solve_window_layout(layout: &WindowLayout, area: Rect) -> Vec<WindowMeasure> {
    if let Some(zid) = layout.zoomed.as_deref() {
        if let Some(m) = find_stack_measure_root(&layout.root, zid, area) {
            return vec![m];
        }
    }
    let mut out = Vec::new();
    match &layout.root {
        WindowLayoutRoot::Axis(a) => solve_axis(a, area, &mut out),
        WindowLayoutRoot::Stack(s) => solve_stack(s, area, &mut out),
    }
    out
}

/// 🏗️ Builds a row/column layout of individually-sized windows.
pub fn create_default_layout(window_ids: &[String], direction: &str, sizes: Option<&[f64]>, titles: Option<&[String]>) -> WindowLayout {
    let children = window_ids
        .iter()
        .enumerate()
        .map(|(i, id)| {
            WindowLayoutChild::Stack(WindowLayoutStackNode {
                size: sizes.and_then(|s| s.get(i)).copied(),
                active_window_kind_id: Some(id.clone()),
                children: vec![WindowLayoutWindowNode { window_kind_id: id.clone(), title: titles.and_then(|t| t.get(i)).cloned(), corner: None }],
            })
        })
        .collect();
    WindowLayout { root: WindowLayoutRoot::Axis(WindowLayoutAxisNode { kind: direction.to_string(), size: None, children }), zoomed: None }
}

/// 🟰 Builds an evenly-weighted row layout.
pub fn even_window_layout(window_ids: &[String]) -> WindowLayout {
    create_default_layout(window_ids, "row", None, None)
}

//#region 🔖️WindowLayoutMutations
fn stack_contains(stack: &WindowLayoutStackNode, id: &str) -> bool {
    stack.children.iter().any(|c| c.window_kind_id == id)
}

fn take_window_node(root: &mut WindowLayoutRoot, id: &str) -> Option<WindowLayoutWindowNode> {
    fn from_stack(stack: &mut WindowLayoutStackNode, id: &str) -> Option<WindowLayoutWindowNode> {
        if let Some(i) = stack.children.iter().position(|c| c.window_kind_id == id) {
            let node = stack.children.remove(i);
            if stack.active_window_kind_id.as_deref() == Some(id) {
                stack.active_window_kind_id = stack.children.first().map(|c| c.window_kind_id.clone());
            }
            return Some(node);
        }
        None
    }
    fn walk(child: &mut WindowLayoutChild, id: &str) -> Option<WindowLayoutWindowNode> {
        match child {
            WindowLayoutChild::Stack(s) => from_stack(s, id),
            WindowLayoutChild::Axis(a) => {
                for c in &mut a.children {
                    if let Some(n) = walk(c, id) {
                        return Some(n);
                    }
                }
                None
            }
        }
    }
    match root {
        WindowLayoutRoot::Stack(s) => from_stack(s, id),
        WindowLayoutRoot::Axis(a) => {
            for c in &mut a.children {
                if let Some(n) = walk(c, id) {
                    return Some(n);
                }
            }
            None
        }
    }
}

fn with_stack_mut(root: &mut WindowLayoutRoot, id: &str, f: &mut dyn FnMut(&mut WindowLayoutStackNode) -> bool) -> bool {
    fn walk(child: &mut WindowLayoutChild, id: &str, f: &mut dyn FnMut(&mut WindowLayoutStackNode) -> bool) -> bool {
        match child {
            WindowLayoutChild::Stack(s) if stack_contains(s, id) => f(s),
            WindowLayoutChild::Axis(a) => a.children.iter_mut().any(|c| walk(c, id, f)),
            _ => false,
        }
    }
    match root {
        WindowLayoutRoot::Stack(s) if stack_contains(s, id) => f(s),
        WindowLayoutRoot::Axis(a) => a.children.iter_mut().any(|c| walk(c, id, f)),
        _ => false,
    }
}

/// 🔍 Zooms `window_kind_id` to fill the canvas; `None` restores the tiling.
pub fn zoom_window(layout: &mut WindowLayout, window_kind_id: Option<&str>) {
    layout.zoomed = window_kind_id.map(str::to_string);
}

/// 🏷️ Activates `tab_id` inside its stack.
pub fn activate_stack_tab(layout: &mut WindowLayout, tab_id: &str) -> bool {
    with_stack_mut(&mut layout.root, tab_id, &mut |s| {
        if stack_contains(s, tab_id) {
            s.active_window_kind_id = Some(tab_id.to_string());
            true
        } else {
            false
        }
    })
}

/// 🔁 Cycles the active tab in the stack containing `window_kind_id` by `delta`.
pub fn cycle_stack_tab(layout: &mut WindowLayout, window_kind_id: &str, delta: i32) -> bool {
    with_stack_mut(&mut layout.root, window_kind_id, &mut |s| {
        if s.children.is_empty() {
            return false;
        }
        let cur = s.active_window_kind_id.as_ref().and_then(|id| s.children.iter().position(|c| &c.window_kind_id == id)).unwrap_or(0);
        let len = s.children.len() as i32;
        let next = ((cur as i32 + delta).rem_euclid(len)) as usize;
        s.active_window_kind_id = Some(s.children[next].window_kind_id.clone());
        true
    })
}

/// ✂️ Splits the stack containing `window_kind_id` into an axis of two equal stacks.
pub fn split_window(layout: &mut WindowLayout, window_kind_id: &str, direction: &str, new_id: &str, new_title: Option<String>) -> bool {
    let direction = if direction == "column" { "column" } else { "row" };
    let new_stack =
        WindowLayoutChild::Stack(WindowLayoutStackNode { size: Some(1.0), active_window_kind_id: Some(new_id.to_string()), children: vec![WindowLayoutWindowNode { window_kind_id: new_id.to_string(), title: new_title, corner: None }] });
    fn split_in_axis(axis: &mut WindowLayoutAxisNode, window_kind_id: &str, direction: &str, new_stack: &WindowLayoutChild) -> bool {
        for i in 0..axis.children.len() {
            match &axis.children[i] {
                WindowLayoutChild::Stack(s) if stack_contains(s, window_kind_id) => {
                    let old = axis.children[i].clone();
                    let old = match old {
                        WindowLayoutChild::Stack(mut s) => {
                            s.size = Some(1.0);
                            WindowLayoutChild::Stack(s)
                        }
                        other => other,
                    };
                    let parent_size = axis_child_size(&old);
                    axis.children[i] = WindowLayoutChild::Axis(WindowLayoutAxisNode { kind: direction.to_string(), size: Some(parent_size), children: vec![old, new_stack.clone()] });
                    return true;
                }
                WindowLayoutChild::Axis(_) => {
                    if let WindowLayoutChild::Axis(a) = &mut axis.children[i] {
                        if split_in_axis(a, window_kind_id, direction, new_stack) {
                            return true;
                        }
                    }
                }
                _ => {}
            }
        }
        false
    }
    match &mut layout.root {
        WindowLayoutRoot::Stack(s) if stack_contains(s, window_kind_id) => {
            let mut old = s.clone();
            old.size = Some(1.0);
            layout.root = WindowLayoutRoot::Axis(WindowLayoutAxisNode { kind: direction.to_string(), size: None, children: vec![WindowLayoutChild::Stack(old), new_stack] });
            true
        }
        WindowLayoutRoot::Axis(a) => split_in_axis(a, window_kind_id, direction, &new_stack),
        _ => false,
    }
}

/// 🛠️ Nudges the weight of the stack containing `window_kind_id` by `delta`; a direct sibling slot absorbs `-delta`, and a deeper nesting nudges the level of its immediate match stack.
pub fn resize_window(layout: &mut WindowLayout, window_kind_id: &str, delta: f64) -> bool {
    fn child_has(id: &str, child: &WindowLayoutChild) -> bool {
        match child {
            WindowLayoutChild::Stack(s) => stack_contains(s, id),
            WindowLayoutChild::Axis(a) => a.children.iter().any(|c| child_has(id, c)),
        }
    }
    fn nudge(axis: &mut WindowLayoutAxisNode, window_kind_id: &str, delta: f64) -> bool {
        if let Some(i) = axis.children.iter().position(|c| child_has(window_kind_id, c)) {
            let direct = matches!(&axis.children[i], WindowLayoutChild::Stack(s) if stack_contains(s, window_kind_id));
            if direct {
                let j = if i + 1 < axis.children.len() {
                    i + 1
                } else if i > 0 {
                    i - 1
                } else {
                    return false;
                };
                let set_size = |child: &mut WindowLayoutChild, size: f64| match child {
                    WindowLayoutChild::Stack(s) => s.size = Some(size.max(0.05)),
                    WindowLayoutChild::Axis(a) => a.size = Some(size.max(0.05)),
                };
                let a = axis_child_size(&axis.children[i]);
                let b = axis_child_size(&axis.children[j]);
                set_size(&mut axis.children[i], a + delta);
                set_size(&mut axis.children[j], (b - delta).max(0.05));
                return true;
            }
        }
        for c in &mut axis.children {
            if let WindowLayoutChild::Axis(a) = c {
                if nudge(a, window_kind_id, delta) {
                    return true;
                }
            }
        }
        false
    }
    match &mut layout.root {
        WindowLayoutRoot::Axis(a) => nudge(a, window_kind_id, delta),
        _ => false,
    }
}

/// 🚚 Moves `window_kind_id` into the stack that hosts `target_window_kind_id` as a new tab.
pub fn move_window_to_stack(layout: &mut WindowLayout, window_kind_id: &str, target_window_kind_id: &str) -> bool {
    if window_kind_id == target_window_kind_id {
        return false;
    }
    let Some(node) = take_window_node(&mut layout.root, window_kind_id) else {
        return false;
    };
    let node_id = node.window_kind_id.clone();
    let placed = with_stack_mut(&mut layout.root, target_window_kind_id, &mut |s| {
        s.children.push(node.clone());
        s.active_window_kind_id = Some(node_id.clone());
        true
    });
    if placed {
        return true;
    }
    match &mut layout.root {
        WindowLayoutRoot::Axis(a) => {
            a.children.push(WindowLayoutChild::Stack(WindowLayoutStackNode { size: Some(1.0), active_window_kind_id: Some(node.window_kind_id.clone()), children: vec![node] }));
            true
        }
        WindowLayoutRoot::Stack(s) => {
            s.children.push(node);
            true
        }
    }
}

/// 🗑️ Removes `window_kind_id` from the layout tree.
pub fn remove_window(layout: &mut WindowLayout, window_kind_id: &str) -> bool {
    take_window_node(&mut layout.root, window_kind_id).is_some()
}

/// 📎️ Appends a window tab to the stack containing `host_window_kind_id`.
pub fn push_window_to_stack(layout: &mut WindowLayout, host_window_kind_id: &str, node: WindowLayoutWindowNode) -> bool {
    let id = node.window_kind_id.clone();
    let mut node = Some(node);
    with_stack_mut(&mut layout.root, host_window_kind_id, &mut |s| match node.take() {
        Some(node) => {
            s.children.push(node);
            s.active_window_kind_id = Some(id.clone());
            true
        }
        None => false,
    })
}

/// 🔭 The stack that hosts `window_kind_id`, cloned.
pub fn stack_hosting(layout: &WindowLayout, window_kind_id: &str) -> Option<WindowLayoutStackNode> {
    fn walk(child: &WindowLayoutChild, id: &str) -> Option<WindowLayoutStackNode> {
        match child {
            WindowLayoutChild::Stack(s) => stack_contains(s, id).then(|| s.clone()),
            WindowLayoutChild::Axis(a) => a.children.iter().find_map(|c| walk(c, id)),
        }
    }
    match &layout.root {
        WindowLayoutRoot::Stack(s) => stack_contains(s, window_kind_id).then(|| s.clone()),
        WindowLayoutRoot::Axis(a) => a.children.iter().find_map(|c| walk(c, window_kind_id)),
    }
}

/// 🪪 Sets the display title of `window_kind_id` on its layout node.
pub fn set_window_title(layout: &mut WindowLayout, window_kind_id: &str, title: Option<String>) -> bool {
    let mut done = false;
    let mut title = Some(title);
    with_stack_mut(&mut layout.root, window_kind_id, &mut |s| {
        if let Some(node) = s.children.iter_mut().find(|c| c.window_kind_id == window_kind_id) {
            node.title = title.take().flatten();
            done = true;
        }
        done
    });
    done
}

/// 🔀 Moves the stack tab at `from` to `to` inside the stack hosting `window_kind_id` (a tab drag).
pub fn reorder_stack_tab(layout: &mut WindowLayout, window_kind_id: &str, from: usize, to: usize) -> bool {
    with_stack_mut(&mut layout.root, window_kind_id, &mut |s| {
        if from == to || from >= s.children.len() || to >= s.children.len() {
            return false;
        }
        let node = s.children.remove(from);
        s.children.insert(to, node);
        true
    })
}

/// 🪝 Applies a `SplitterDragged { path, delta }` to `layout` laid out over `area`: the stack before the splitter grows by
/// `delta` cells, the one after shrinks by as much; neither goes under `SPLIT_MIN_FRACTION` of the axis.
pub fn resize_split(layout: &mut WindowLayout, area: Rect, path: &[usize], delta: i16) -> bool {
    let Some((&index, axis_path)) = path.split_last() else { return false };
    let WindowLayoutRoot::Axis(root) = &mut layout.root else { return false };
    let mut axis = root;
    let mut rect = area;
    for &step in axis_path {
        let is_row = axis.kind == "row";
        let sizes = axis_cells(axis, if is_row { rect.width } else { rect.height });
        let before: u16 = sizes.iter().take(step).map(|s| s + WINDOW_GAP).sum();
        let Some(size) = sizes.get(step).copied() else { return false };
        rect = if is_row { Rect::new(rect.x + before, rect.y, size, rect.height) } else { Rect::new(rect.x, rect.y + before, rect.width, size) };
        let Some(WindowLayoutChild::Axis(next)) = axis.children.get_mut(step) else { return false };
        axis = next;
    }
    let is_row = axis.kind == "row";
    let extent = if is_row { rect.width } else { rect.height };
    let sizes = axis_cells(axis, extent);
    if index + 1 >= sizes.len() {
        return false;
    }
    let usable = f64::from(extent.saturating_sub(WINDOW_GAP * (sizes.len() as u16 - 1))).max(1.0);
    let floor = ((usable * SPLIT_MIN_FRACTION).round() as i32).max(1);
    let (a, b) = (i32::from(sizes[index]), i32::from(sizes[index + 1]));
    if a + b < 2 * floor {
        return false;
    }
    let moved = i32::from(delta).clamp(floor - a, b - floor);
    if moved == 0 {
        return false;
    }
    let total_weight: f64 = axis.children.iter().map(axis_child_size).sum();
    let per_cell = total_weight / usable;
    let set = |child: &mut WindowLayoutChild, cells: i32| {
        let size = Some(f64::from(cells) * per_cell);
        match child {
            WindowLayoutChild::Axis(node) => node.size = size,
            WindowLayoutChild::Stack(node) => node.size = size,
        }
    };
    set(&mut axis.children[index], a + moved);
    set(&mut axis.children[index + 1], b - moved);
    true
}
//#endregion 🔖️WindowLayoutMutations

//#endregion 🔖️WindowLayout
