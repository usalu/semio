use crate::tui::cell::{Cell, CellBuffer};
use crate::tui::footer::paint_footer;
use crate::tui::geometry::{Pos, Rect};
use crate::tui::layout::{solve_window_layout, WindowLayout};
use crate::tui::navbar::paint_navbar;
use crate::tui::scene::{Node, NodeContent, NodeId, Scene};
use crate::tui::text::{display_width, truncate_to};
use crate::tui::theme::{Role, Surface, Theme};
use crate::tui::window::paint_window;

#[derive(Clone)]
pub struct NavItem {
    pub id: String,
    pub label: String,
    pub active: bool,
}

pub struct NavbarState {
    pub left: Vec<NavItem>,
    pub center: Vec<NavItem>,
    pub right: Vec<NavItem>,
}

#[derive(Clone)]
pub struct KeyHint {
    pub key: String,
    pub label: String,
}

pub struct FooterState {
    pub hints: Vec<KeyHint>,
    pub status: String,
}

/// 🏷️ One stack tab: label plus the corner it docks into.
#[derive(Clone, Debug, PartialEq)]
pub struct WindowStackTabState {
    pub label: String,
    pub corner: crate::tui::layout::WindowStackCorner,
}

impl WindowStackTabState {
    pub fn new(label: impl Into<String>, corner: crate::tui::layout::WindowStackCorner) -> Self {
        Self { label: label.into(), corner }
    }

    pub fn top_left(label: impl Into<String>) -> Self {
        Self::new(label, crate::tui::layout::WindowStackCorner::TopLeft)
    }
}

pub struct WindowState {
    pub title: String,
    pub number: Option<String>,
    pub focused: bool,
    pub closable: bool,
    pub maximizable: bool,
    pub stack_tabs: Vec<WindowStackTabState>,
    pub active_stack_tab: usize,
}

impl WindowState {
    pub fn new(title: impl Into<String>) -> Self {
        Self { title: title.into(), number: None, focused: false, closable: true, maximizable: true, stack_tabs: Vec::new(), active_stack_tab: 0 }
    }

    /// 📑 Attaches per-stack tab labels (defaulting each to top-left); `active` is clamped into range.
    pub fn with_stack_tabs(mut self, tabs: Vec<String>, active: usize) -> Self {
        self.stack_tabs = tabs.into_iter().map(WindowStackTabState::top_left).collect();
        self.active_stack_tab = if self.stack_tabs.is_empty() { 0 } else { active.min(self.stack_tabs.len() - 1) };
        self
    }

    /// 🧭️ Attaches corner-aware stack tabs; `active` is clamped into range.
    pub fn with_stack_tab_states(mut self, tabs: Vec<WindowStackTabState>, active: usize) -> Self {
        self.active_stack_tab = if tabs.is_empty() { 0 } else { active.min(tabs.len() - 1) };
        self.stack_tabs = tabs;
        self
    }

    /// ↕️ Extra chrome rows consumed by raised top and/or bottom corner tab boxes.
    pub fn stack_tab_strip_height(&self) -> u16 {
        let tabs = effective_stack_tabs(self);
        let has_top = tabs.iter().any(|(_, _, c)| c.is_top());
        let has_bottom = tabs.iter().any(|(_, _, c)| !c.is_top());
        u16::from(has_top) + u16::from(has_bottom)
    }
}

/// 🪟 The concrete state of any semio chrome node.
pub enum ChromeState {
    Navbar(NavbarState),
    Footer(FooterState),
    Canvas,
    Window(WindowState),
}

impl ChromeState {
    /// 🖌️ Paints the chrome background/frame; window/content children paint over it.
    pub fn paint(&self, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
        match self {
            ChromeState::Navbar(n) => paint_navbar(n, theme, rect, buf),
            ChromeState::Footer(f) => paint_footer(f, theme, rect, buf),
            ChromeState::Canvas => {
                buf.fill_rect(rect, Cell::blank(theme.role(Role::Foreground), theme.surface(Surface::Base)));
            }
            ChromeState::Window(w) => paint_window(w, theme, rect, buf),
        }
    }

    /// 🎯 Resolves window chrome hits: per-tab glyphs and label activation across corner groups.
    pub fn window_hit(&self, rect: Rect, pos: Pos) -> Option<crate::tui::widget::WidgetSignal> {
        let ChromeState::Window(w) = self else { return None };
        let layout = window_chip_layout(w, rect);
        if !layout.has_tabs {
            return None;
        }
        for group in &layout.groups {
            let text_y = match group.corner {
                crate::tui::layout::WindowStackCorner::TopLeft | crate::tui::layout::WindowStackCorner::TopRight => rect.y + 1,
                crate::tui::layout::WindowStackCorner::BottomLeft | crate::tui::layout::WindowStackCorner::BottomRight => rect.y + rect.height.saturating_sub(2),
            };
            if pos.y != text_y {
                continue;
            }
            for tab in &group.tabs {
                if tab.close_x == Some(pos.x) && w.closable {
                    return Some(crate::tui::widget::WidgetSignal::WindowClose(w.active_stack_tab));
                }
                if tab.maximize_x == Some(pos.x) && w.maximizable {
                    return Some(crate::tui::widget::WidgetSignal::WindowMaximize);
                }
                if tab.new_x == Some(pos.x) {
                    return Some(crate::tui::widget::WidgetSignal::WindowNewTab);
                }
                let tab_right = tab.x.saturating_add(tab.interior_width.saturating_add(1));
                if pos.x > tab.x && pos.x < tab_right {
                    return Some(crate::tui::widget::WidgetSignal::WindowTabActivated(tab.index));
                }
            }
        }
        None
    }

    /// 🎯 Control-only hit testing; delegates to `window_hit`.
    pub fn window_control_at(&self, rect: Rect, pos: Pos) -> Option<crate::tui::widget::WidgetSignal> {
        match self.window_hit(rect, pos)? {
            s @ (crate::tui::widget::WidgetSignal::WindowClose(_) | crate::tui::widget::WidgetSignal::WindowMaximize) => Some(s),
            _ => None,
        }
    }
}

const WINDOW_TAB_MAXIMIZE_GLYPH: char = '\u{2922}';
const WINDOW_TAB_NEW_GLYPH: char = '\u{29C9}';
const WINDOW_TAB_CLOSE_GLYPH: char = '\u{2715}';

/// 🪟 One 2-row tab recessed into a corner: `x` is its left-wall column, `interior` sits between walls.
/// `pub(crate)`: shared with `crate::tui::window`'s `paint_window`/`paint_corner_tab`.
pub(crate) struct WindowTab {
    pub(crate) x: u16,
    pub(crate) interior: String,
    pub(crate) interior_width: u16,
}

/// 🏷️ One corner tab chip with absolute glyph columns for hit-testing.
pub(crate) struct WindowCornerTab {
    pub(crate) x: u16,
    pub(crate) interior: String,
    pub(crate) interior_width: u16,
    pub(crate) index: usize,
    pub(crate) maximize_x: Option<u16>,
    pub(crate) new_x: Option<u16>,
    pub(crate) close_x: Option<u16>,
}

impl WindowCornerTab {
    pub(crate) fn as_window_tab(&self) -> WindowTab {
        WindowTab { x: self.x, interior: self.interior.clone(), interior_width: self.interior_width }
    }
}

/// 🧭️ Tabs docked into one stack corner.
pub(crate) struct WindowCornerChipGroup {
    pub(crate) corner: crate::tui::layout::WindowStackCorner,
    pub(crate) tabs: Vec<WindowCornerTab>,
}

/// 🪟 `pub(crate)`: shared with `crate::tui::window`'s `paint_window`.
pub(crate) struct WindowChipLayout {
    pub(crate) has_tabs: bool,
    pub(crate) groups: Vec<WindowCornerChipGroup>,
    pub(crate) top_body_y: u16,
    pub(crate) bottom_body_y: Option<u16>,
    #[allow(dead_code)]
    pub(crate) top_left_end_x: u16,
    #[allow(dead_code)]
    pub(crate) top_right_start_x: u16,
    #[allow(dead_code)]
    pub(crate) bottom_left_end_x: u16,
    #[allow(dead_code)]
    pub(crate) bottom_right_start_x: u16,
}

fn effective_stack_tabs(w: &WindowState) -> Vec<(usize, String, crate::tui::layout::WindowStackCorner)> {
    if w.stack_tabs.is_empty() {
        let number_prefix = w.number.as_ref().map(|n| format!("{n} ")).unwrap_or_default();
        return vec![(0, format!("{number_prefix}{}", w.title), crate::tui::layout::WindowStackCorner::TopLeft)];
    }
    w.stack_tabs.iter().enumerate().map(|(i, t)| (i, t.label.clone(), t.corner)).collect()
}

fn build_corner_tab_interior(label: &str, w: &WindowState, room: u16) -> (String, u16, Option<u16>, Option<u16>, Option<u16>) {
    if room < 3 {
        return (String::new(), 0, None, None, None);
    }
    let mut show_max = w.maximizable;
    let mut show_close = w.closable;
    let mut show_new_glyph = true;
    loop {
        let actions = u16::from(show_max) * 2 + u16::from(show_new_glyph) * 2 + u16::from(show_close) * 2;
        let label_room = room.saturating_sub(2 + actions).max(1);
        let (label_trunc, _) = truncate_to(label, label_room);
        let mut interior = format!(" {label_trunc} ");
        let mut maximize_off = None;
        let mut new_off = None;
        let mut close_off = None;
        if show_max {
            maximize_off = Some(display_width(&interior));
            interior.push(WINDOW_TAB_MAXIMIZE_GLYPH);
            interior.push(' ');
        }
        if show_new_glyph {
            new_off = Some(display_width(&interior));
            interior.push(WINDOW_TAB_NEW_GLYPH);
            interior.push(' ');
        }
        if show_close {
            close_off = Some(display_width(&interior));
            interior.push(WINDOW_TAB_CLOSE_GLYPH);
            interior.push(' ');
        }
        let width = display_width(&interior);
        if width <= room {
            return (interior, width, maximize_off, new_off, close_off);
        }
        if show_new_glyph {
            show_new_glyph = false;
            continue;
        }
        if show_max {
            show_max = false;
            continue;
        }
        if show_close {
            show_close = false;
            continue;
        }
        let (label_trunc, _) = truncate_to(label, room.saturating_sub(2).max(1));
        let interior = format!(" {label_trunc} ");
        return (interior.clone(), display_width(&interior).min(room), None, None, None);
    }
}

fn layout_corner_tabs(entries: &[(usize, String)], w: &WindowState, start_x: u16, end_x: u16, from_left: bool) -> Vec<WindowCornerTab> {
    if entries.is_empty() || end_x <= start_x + 2 {
        return Vec::new();
    }
    let span = end_x.saturating_sub(start_x);
    let mut tabs = Vec::new();
    if from_left {
        let mut x = start_x;
        for (index, label) in entries {
            if x + 3 >= end_x {
                break;
            }
            let room = end_x.saturating_sub(x + 2);
            let (interior, interior_width, max_off, new_off, close_off) = build_corner_tab_interior(label, w, room);
            if interior_width < 3 {
                break;
            }
            let width = interior_width + 2;
            if x + width > end_x {
                break;
            }
            tabs.push(WindowCornerTab { x, interior, interior_width, index: *index, maximize_x: max_off.map(|o| x + 1 + o), new_x: new_off.map(|o| x + 1 + o), close_x: close_off.map(|o| x + 1 + o) });
            x = x.saturating_add(width);
        }
    } else {
        let mut right = end_x;
        let mut rev = Vec::new();
        for (index, label) in entries.iter().rev() {
            if right <= start_x + 3 {
                break;
            }
            let room = right.saturating_sub(start_x + 2).min(span);
            let (interior, interior_width, max_off, new_off, close_off) = build_corner_tab_interior(label, w, room);
            if interior_width < 3 {
                break;
            }
            let width = interior_width + 2;
            if right < start_x + width {
                break;
            }
            let x = right - width;
            if x < start_x {
                break;
            }
            rev.push(WindowCornerTab { x, interior, interior_width, index: *index, maximize_x: max_off.map(|o| x + 1 + o), new_x: new_off.map(|o| x + 1 + o), close_x: close_off.map(|o| x + 1 + o) });
            right = x;
        }
        rev.reverse();
        tabs = rev;
    }
    tabs
}

/// 🎯 Shared by paint and click hit-testing so the two can never drift apart.
/// Returns up to four corner chip groups; each tab carries inline action glyph columns.
/// `pub(crate)`: called from `crate::tui::window`'s `paint_window`.
pub(crate) fn window_chip_layout(w: &WindowState, rect: Rect) -> WindowChipLayout {
    use crate::tui::layout::WindowStackCorner;

    let flat = WindowChipLayout {
        has_tabs: false,
        groups: Vec::new(),
        top_body_y: rect.y,
        bottom_body_y: None,
        top_left_end_x: rect.x,
        top_right_start_x: rect.x + rect.width.saturating_sub(1),
        bottom_left_end_x: rect.x,
        bottom_right_start_x: rect.x + rect.width.saturating_sub(1),
    };
    if rect.width < 4 || rect.height < 4 {
        return flat;
    }

    let effective = effective_stack_tabs(w);
    let mut tl = Vec::new();
    let mut tr = Vec::new();
    let mut bl = Vec::new();
    let mut br = Vec::new();
    for (index, label, corner) in &effective {
        match corner {
            WindowStackCorner::TopLeft => tl.push((*index, label.clone())),
            WindowStackCorner::TopRight => tr.push((*index, label.clone())),
            WindowStackCorner::BottomLeft => bl.push((*index, label.clone())),
            WindowStackCorner::BottomRight => br.push((*index, label.clone())),
        }
    }

    let has_top = !tl.is_empty() || !tr.is_empty();
    let has_bottom = !bl.is_empty() || !br.is_empty();
    let min_h = match (has_top, has_bottom) {
        (true, true) => 6,
        (true, false) | (false, true) => 4,
        (false, false) => 2,
    };
    if rect.height < min_h || (!has_top && !has_bottom) {
        return flat;
    }

    let mid = rect.x + rect.width / 2;
    let right = rect.x + rect.width;

    let tl_tabs = layout_corner_tabs(&tl, w, rect.x, mid.saturating_add(1).max(rect.x + 3), true);
    let tr_tabs = layout_corner_tabs(&tr, w, mid.saturating_sub(1).min(right.saturating_sub(3)), right, false);
    let bl_tabs = layout_corner_tabs(&bl, w, rect.x, mid.saturating_add(1).max(rect.x + 3), true);
    let br_tabs = layout_corner_tabs(&br, w, mid.saturating_sub(1).min(right.saturating_sub(3)), right, false);

    // Resolve collisions on an edge: prefer left group, shrink right start.
    let top_left_end_x = tl_tabs.last().map(|t| t.x + t.interior_width + 2).unwrap_or(rect.x);
    let mut top_right_start_x = tr_tabs.first().map(|t| t.x).unwrap_or(right.saturating_sub(1));
    if !tr_tabs.is_empty() && top_right_start_x < top_left_end_x.saturating_add(1) {
        top_right_start_x = top_left_end_x.saturating_add(1).min(right.saturating_sub(1));
    }
    let bottom_left_end_x = bl_tabs.last().map(|t| t.x + t.interior_width + 2).unwrap_or(rect.x);
    let mut bottom_right_start_x = br_tabs.first().map(|t| t.x).unwrap_or(right.saturating_sub(1));
    if !br_tabs.is_empty() && bottom_right_start_x < bottom_left_end_x.saturating_add(1) {
        bottom_right_start_x = bottom_left_end_x.saturating_add(1).min(right.saturating_sub(1));
    }

    let mut groups = Vec::new();
    if !tl_tabs.is_empty() {
        groups.push(WindowCornerChipGroup { corner: WindowStackCorner::TopLeft, tabs: tl_tabs });
    }
    if !tr_tabs.is_empty() {
        // Drop colliding right tabs that start before left end.
        let tabs: Vec<_> = tr_tabs.into_iter().filter(|t| t.x >= top_left_end_x.saturating_add(1)).collect();
        if !tabs.is_empty() {
            top_right_start_x = tabs.first().map(|t| t.x).unwrap_or(top_right_start_x);
            groups.push(WindowCornerChipGroup { corner: WindowStackCorner::TopRight, tabs });
        } else {
            top_right_start_x = right.saturating_sub(1);
        }
    }
    if !bl_tabs.is_empty() {
        groups.push(WindowCornerChipGroup { corner: WindowStackCorner::BottomLeft, tabs: bl_tabs });
    }
    if !br_tabs.is_empty() {
        let tabs: Vec<_> = br_tabs.into_iter().filter(|t| t.x >= bottom_left_end_x.saturating_add(1)).collect();
        if !tabs.is_empty() {
            bottom_right_start_x = tabs.first().map(|t| t.x).unwrap_or(bottom_right_start_x);
            groups.push(WindowCornerChipGroup { corner: WindowStackCorner::BottomRight, tabs });
        } else {
            bottom_right_start_x = right.saturating_sub(1);
        }
    }

    if groups.is_empty() {
        return flat;
    }

    let top_body_y = if has_top { rect.y + 2 } else { rect.y };
    let bottom_body_y = if has_bottom { Some(rect.y + rect.height.saturating_sub(3)) } else { None };

    WindowChipLayout {
        has_tabs: true,
        groups,
        top_body_y,
        bottom_body_y,
        top_left_end_x: if has_top { top_left_end_x } else { rect.x },
        top_right_start_x: if has_top { top_right_start_x } else { right.saturating_sub(1) },
        bottom_left_end_x: if has_bottom { bottom_left_end_x } else { rect.x },
        bottom_right_start_x: if has_bottom { bottom_right_start_x } else { right.saturating_sub(1) },
    }
}

/// 🚪 Content inset that keeps children inside the closed outline, under the hairline a chip bends into.
pub(crate) fn window_content_padding(w: &WindowState, rect: Rect) -> [u16; 4] {
    let layout = window_chip_layout(w, rect);
    if !layout.has_tabs {
        return [1, 1, 1, 1];
    }
    let has_top = layout.groups.iter().any(|group| group.corner.is_top());
    let has_bottom = layout.groups.iter().any(|group| !group.corner.is_top());
    let top = if has_top { layout.top_body_y.saturating_sub(rect.y).saturating_add(1) } else { 1 };
    let bottom = if has_bottom {
        let bottom_y = rect.y + rect.height - 1;
        bottom_y.saturating_sub(layout.bottom_body_y.unwrap_or(bottom_y)).saturating_add(1)
    } else {
        1
    };
    [top, 1, bottom, 1]
}

/// ??? The three fixed shell regions plus one Window node per resolved `WindowMeasure`.
pub struct Shell {
    pub navbar: NodeId,
    pub canvas: NodeId,
    pub footer: NodeId,
    pub windows: Vec<(String, NodeId)>,
    pub mount_root: Option<NodeId>,
}

/// ??? Builds navbar(top) + canvas(fill) + footer(bottom), then one Window per tiled slot.
pub fn shell(scene: &mut Scene, navbar: NavbarState, footer: FooterState, layout: &WindowLayout) -> Shell {
    let root = scene.root();
    let navbar_id = scene.add(root, Node::new(NodeContent::Chrome(ChromeState::Navbar(navbar))));
    let canvas_id = scene.add(root, Node::new(NodeContent::Chrome(ChromeState::Canvas)));
    let footer_id = scene.add(root, Node::new(NodeContent::Chrome(ChromeState::Footer(footer))));
    {
        let mut root_mut = scene.node_mut(root);
        root_mut.set_constraint(crate::tui::layout::Constraint { direction: crate::tui::layout::Direction::Column, ..Default::default() });
    }
    scene.node_mut(navbar_id).set_constraint(crate::tui::layout::Constraint { height: crate::tui::layout::Dimension::Cells(2), ..Default::default() });
    scene.node_mut(canvas_id).set_constraint(crate::tui::layout::Constraint { height: crate::tui::layout::Dimension::Weight(1), direction: crate::tui::layout::Direction::Stack, ..Default::default() });
    scene.node_mut(footer_id).set_constraint(crate::tui::layout::Constraint { height: crate::tui::layout::Dimension::Cells(2), ..Default::default() });
    let mut windows = Vec::new();
    for measure in solve_window_layout(layout, Rect::default()) {
        let id = scene.add(canvas_id, Node::new(NodeContent::Chrome(ChromeState::Window(WindowState::new(measure.window_kind_id.clone())))));
        scene.node_mut(id).set_constraint(crate::tui::layout::Constraint {
            width: crate::tui::layout::Dimension::Weight(1),
            height: crate::tui::layout::Dimension::Weight(1),
            direction: crate::tui::layout::Direction::Column,
            padding: [2, 1, 1, 1],
            gap: 1,
            ..Default::default()
        });
        windows.push((measure.window_kind_id, id));
    }
    Shell { navbar: navbar_id, canvas: canvas_id, footer: footer_id, windows, mount_root: None }
}

/// ?? Mirrors `layout` into nested row/column/stack boxes under `canvas`, reparenting existing window nodes.
pub fn mount_window_layout(scene: &mut Scene, canvas: NodeId, layout: &WindowLayout, windows: &[(String, NodeId)], mount_root: &mut Option<NodeId>) {
    use crate::tui::layout::{Dimension, Direction, WindowLayoutChild, WindowLayoutRoot};

    if let Some(old) = *mount_root {
        for (_, window) in windows {
            scene.reparent(*window, canvas);
        }
        scene.remove(old);
        *mount_root = None;
    }
    let mount = scene.add(canvas, Node::new(NodeContent::Box));
    scene.node_mut(mount).set_constraint(crate::tui::layout::Constraint { direction: Direction::Stack, width: Dimension::Weight(1), height: Dimension::Weight(1), ..Default::default() });
    *mount_root = Some(mount);

    fn weight(size: Option<f64>) -> u16 {
        ((size.unwrap_or(1.0) * 100.0).round() as u16).max(1)
    }

    fn find_window(windows: &[(String, NodeId)], id: &str) -> Option<NodeId> {
        windows.iter().find(|(k, _)| k == id).map(|(_, n)| *n)
    }

    fn mount_stack(scene: &mut Scene, parent: NodeId, stack: &crate::tui::layout::WindowLayoutStackNode, windows: &[(String, NodeId)], w: u16) {
        let box_id = scene.add(parent, Node::new(NodeContent::Box));
        scene.node_mut(box_id).set_constraint(crate::tui::layout::Constraint { direction: Direction::Stack, width: Dimension::Weight(w), height: Dimension::Weight(w), ..Default::default() });
        let active = stack.active_window_kind_id.as_deref().unwrap_or_else(|| stack.children.first().map(|c| c.window_kind_id.as_str()).unwrap_or(""));
        let tabs: Vec<WindowStackTabState> = stack.children.iter().map(|c| WindowStackTabState { label: c.window_kind_id.clone(), corner: c.corner.unwrap_or_default() }).collect();
        let active_idx = tabs.iter().position(|t| t.label == active).unwrap_or(0);
        for child in &stack.children {
            if let Some(win_id) = find_window(windows, &child.window_kind_id) {
                scene.reparent(win_id, box_id);
                let visible = child.window_kind_id == active;
                scene.node_mut(win_id).set_visible(visible);
                scene.node_mut(win_id).set_constraint(crate::tui::layout::Constraint { width: Dimension::Weight(1), height: Dimension::Weight(1), direction: Direction::Column, padding: [2, 1, 1, 1], gap: 1, ..Default::default() });
                if let Some(chrome) = scene.node_mut(win_id).chrome() {
                    if let ChromeState::Window(ref mut ws) = chrome {
                        ws.stack_tabs = tabs.clone();
                        ws.active_stack_tab = active_idx;
                        ws.title = child.title.clone().unwrap_or_else(|| child.window_kind_id.clone());
                    }
                }
            }
        }
    }

    fn mount_child(scene: &mut Scene, parent: NodeId, child: &WindowLayoutChild, windows: &[(String, NodeId)]) {
        match child {
            WindowLayoutChild::Axis(axis) => {
                let is_row = axis.kind == "row";
                let box_id = scene.add(parent, Node::new(NodeContent::Box));
                scene.node_mut(box_id).set_constraint(crate::tui::layout::Constraint {
                    direction: if is_row { Direction::Row } else { Direction::Column },
                    width: Dimension::Weight(weight(axis.size)),
                    height: Dimension::Weight(weight(axis.size)),
                    ..Default::default()
                });
                for c in &axis.children {
                    mount_child(scene, box_id, c, windows);
                }
            }
            WindowLayoutChild::Stack(stack) => mount_stack(scene, parent, stack, windows, weight(stack.size)),
        }
    }

    if let Some(zid) = layout.zoomed.as_deref() {
        if let Some(win_id) = find_window(windows, zid) {
            scene.reparent(win_id, mount);
            scene.node_mut(win_id).set_visible(true);
            scene.node_mut(win_id).set_constraint(crate::tui::layout::Constraint { width: Dimension::Weight(1), height: Dimension::Weight(1), direction: Direction::Column, padding: [2, 1, 1, 1], gap: 1, ..Default::default() });
        }
        return;
    }

    match &layout.root {
        WindowLayoutRoot::Axis(axis) => mount_child(scene, mount, &WindowLayoutChild::Axis(axis.clone()), windows),
        WindowLayoutRoot::Stack(stack) => mount_stack(scene, mount, stack, windows, 1),
    }
}

impl Shell {
    /// ?? Rebuilds the tiling mount tree and reparents window nodes.
    pub fn remount(&mut self, scene: &mut Scene, layout: &WindowLayout) {
        mount_window_layout(scene, self.canvas, layout, &self.windows, &mut self.mount_root);
    }
}
