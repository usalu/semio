use crate::tui::cell::{Cell, CellBuffer};
use crate::tui::footer::paint_footer;
use crate::tui::geometry::{Pos, Rect};
use crate::tui::layout::{solve_window_layout, weight_of, WindowLayout, WindowStackCorner, WINDOW_GAP};
use crate::tui::navbar::paint_navbar;
use crate::tui::scene::{AxisState, Node, NodeContent, NodeId, Scene};
use crate::tui::text::{display_width, elide_end};
use crate::tui::theme::{GlyphSet, Role, Status, Surface, Theme};
use crate::tui::widget::WidgetSignal;
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
    pub corner: WindowStackCorner,
    /// 🚦️ The task status drawn as a glyph in the tab's own colour role; `None` draws no glyph.
    pub status: Option<Status>,
}

impl WindowStackTabState {
    pub fn new(label: impl Into<String>, corner: WindowStackCorner) -> Self {
        Self { label: label.into(), corner, status: None }
    }

    pub fn with_status(mut self, status: Option<Status>) -> Self {
        self.status = status;
        self
    }

    pub fn top_left(label: impl Into<String>) -> Self {
        Self::new(label, WindowStackCorner::TopLeft)
    }
}

/// 🗣️ The accessible names of the window controls; the app supplies them in the user's language, empty hides the tooltip.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChromeLabels {
    pub close: String,
    pub maximize: String,
    pub restore: String,
    pub new_tab: String,
    pub previous_tabs: String,
    pub next_tabs: String,
}

/// 🎯 What a window chrome cell stands for; every control carries the stack tab index it acts on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowHit {
    Tab(usize),
    Close(usize),
    Maximize(usize),
    NewTab(usize),
    OverflowPrev(usize),
    OverflowNext(usize),
}

pub struct WindowState {
    pub title: String,
    pub number: Option<String>,
    pub focused: bool,
    pub closable: bool,
    pub maximizable: bool,
    pub stack_tabs: Vec<WindowStackTabState>,
    pub active_stack_tab: usize,
    pub compact: bool,
    pub zoomed: bool,
    pub peers: usize,
    pub new_tab: bool,
    pub labels: ChromeLabels,
    pub hover: Option<WindowHit>,
    pub drop_target: Option<usize>,
    /// 📌 This window's own task status; `Shell::set_status` mirrors it into every sibling's tab strip.
    pub status: Option<Status>,
    /// 🆔️ The window id behind each stack tab, parallel to `stack_tabs`, set when the layout is mounted.
    pub tab_ids: Vec<String>,
    /// 🌀️ The spinner frame of running tabs; the engine advances it on `tick`.
    pub spin: u64,
}

impl WindowState {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            number: None,
            focused: false,
            closable: true,
            maximizable: true,
            stack_tabs: Vec::new(),
            active_stack_tab: 0,
            compact: false,
            zoomed: false,
            peers: 1,
            new_tab: false,
            labels: ChromeLabels::default(),
            hover: None,
            drop_target: None,
            status: None,
            tab_ids: Vec::new(),
            spin: 0,
        }
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

    /// 🔎️ Whether the maximize control shows: only when there is something to maximize against, or something to restore.
    pub fn show_maximize(&self) -> bool {
        self.maximizable && (self.peers > 1 || self.zoomed)
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
    Window(Box<WindowState>),
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

    /// 🕵️ The window control or tab under `pos`, resolved from the same geometry the painter uses.
    pub fn window_target(&self, rect: Rect, pos: Pos) -> Option<WindowHit> {
        let ChromeState::Window(w) = self else { return None };
        let layout = window_chip_layout(w, rect);
        if layout.mode == ChromeMode::Flat {
            return None;
        }
        for group in &layout.groups {
            if pos.y != layout.text_y(group.corner, rect) {
                continue;
            }
            for tab in &group.tabs {
                let right = tab.x.saturating_add(tab.interior_width.saturating_add(1));
                if pos.x <= tab.x || pos.x >= right {
                    continue;
                }
                return match tab.kind {
                    TabKind::Tab if tab.close_x == Some(pos.x) && w.closable => Some(WindowHit::Close(tab.index)),
                    TabKind::Tab => Some(WindowHit::Tab(tab.index)),
                    TabKind::Controls if tab.maximize_x == Some(pos.x) && w.show_maximize() => Some(WindowHit::Maximize(w.active_stack_tab)),
                    TabKind::Controls if tab.new_x == Some(pos.x) && w.new_tab => Some(WindowHit::NewTab(w.active_stack_tab)),
                    TabKind::Controls => None,
                    TabKind::OverflowPrev(target) => Some(WindowHit::OverflowPrev(target)),
                    TabKind::OverflowNext(target) => Some(WindowHit::OverflowNext(target)),
                };
            }
        }
        None
    }

    /// 👇 Resolves a left-press on window chrome into the signal the app acts on.
    pub fn window_hit(&self, rect: Rect, pos: Pos) -> Option<WidgetSignal> {
        Some(match self.window_target(rect, pos)? {
            WindowHit::Tab(i) | WindowHit::OverflowPrev(i) | WindowHit::OverflowNext(i) => WidgetSignal::WindowTabActivated(i),
            WindowHit::Close(i) => WidgetSignal::WindowClose(i),
            WindowHit::Maximize(i) => WidgetSignal::WindowMaximize(i),
            WindowHit::NewTab(i) => WidgetSignal::WindowNewTab(i),
        })
    }

    /// 🎚️ Control-only hit testing; delegates to `window_hit`.
    pub fn window_control_at(&self, rect: Rect, pos: Pos) -> Option<WidgetSignal> {
        match self.window_hit(rect, pos)? {
            s @ (WidgetSignal::WindowClose(_) | WidgetSignal::WindowMaximize(_) | WidgetSignal::WindowNewTab(_)) => Some(s),
            _ => None,
        }
    }

    /// 👆️ Tracks the pointer over the window controls; true when the highlighted control changed.
    pub fn set_hover(&mut self, rect: Rect, pos: Option<Pos>) -> bool {
        let hover = pos.and_then(|pos| self.window_target(rect, pos));
        let ChromeState::Window(w) = self else { return false };
        let changed = w.hover != hover;
        w.hover = hover;
        changed
    }

    /// 💡️ The tooltip for the control or elided tab under `pos`.
    pub fn tooltip(&self, rect: Rect, pos: Pos) -> Option<String> {
        let ChromeState::Window(w) = self else { return None };
        let text = match self.window_target(rect, pos)? {
            WindowHit::Close(_) => w.labels.close.clone(),
            WindowHit::Maximize(_) if w.zoomed => w.labels.restore.clone(),
            WindowHit::Maximize(_) => w.labels.maximize.clone(),
            WindowHit::NewTab(_) => w.labels.new_tab.clone(),
            WindowHit::OverflowPrev(_) => w.labels.previous_tabs.clone(),
            WindowHit::OverflowNext(_) => w.labels.next_tabs.clone(),
            WindowHit::Tab(i) => {
                let label = effective_stack_tabs(w).into_iter().find(|(index, _, _)| *index == i).map(|(_, label, _)| label)?;
                if display_width(&label) <= TAB_LABEL_CELLS {
                    return None;
                }
                label
            }
        };
        (!text.is_empty()).then_some(text)
    }

    /// 🧲 The stack tab a drag at `pos` would drop onto: the tab under it, else the nearest end.
    pub fn window_drop_index(&self, rect: Rect, pos: Pos) -> Option<usize> {
        let ChromeState::Window(w) = self else { return None };
        let layout = window_chip_layout(w, rect);
        let tabs: Vec<&WindowCornerTab> = layout.groups.iter().filter(|group| group.corner.is_top()).flat_map(|group| group.tabs.iter()).filter(|tab| tab.kind == TabKind::Tab).collect();
        let first = tabs.first()?;
        let last = tabs.last()?;
        if pos.x < first.x {
            return Some(first.index);
        }
        tabs.iter().find(|tab| pos.x >= tab.x && pos.x <= tab.x + tab.interior_width + 1).map(|tab| tab.index).or(Some(last.index))
    }

    /// 🚩 Marks the tab a running drag would drop onto.
    pub fn set_drop_target(&mut self, target: Option<usize>) {
        if let ChromeState::Window(w) = self {
            w.drop_target = target;
        }
    }
}

const WINDOW_TAB_MAXIMIZE_GLYPH: char = '\u{2922}';
const WINDOW_TAB_RESTORE_GLYPH: char = '\u{2921}';
const WINDOW_TAB_NEW_GLYPH: char = '+';
const WINDOW_TAB_CLOSE_GLYPH: char = '\u{2715}';
const OVERFLOW_PREV_GLYPH: char = '\u{2039}';
const OVERFLOW_NEXT_GLYPH: char = '\u{203a}';

/// 📏️ The widest a tab chip grows, walls included (the 12 rem cap of the dock).
pub const TAB_CHIP_CELLS: u16 = 24;
/// 🔤 The widest a tab label shows before it is elided (chip minus walls, padding and the close glyph).
pub const TAB_LABEL_CELLS: u16 = TAB_CHIP_CELLS - 6;
const MIN_FULL_ROWS_ONE_ROW: u16 = 4;
const MIN_FULL_ROWS_TWO_ROWS: u16 = 6;

/// 🔩 One chip's seam geometry: `x` is its left-wall column, the interior of `interior_width` cells sits between walls.
/// `pub(crate)`: shared with `crate::tui::window`'s `paint_window`/`paint_tab_seam`.
pub(crate) struct WindowTab {
    pub(crate) x: u16,
    pub(crate) interior_width: u16,
}

/// 🪪 What a chip stands for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TabKind {
    Tab,
    OverflowPrev(usize),
    OverflowNext(usize),
    Controls,
}

/// 🔖 One corner chip with absolute glyph columns for hit-testing.
pub(crate) struct WindowCornerTab {
    pub(crate) x: u16,
    pub(crate) interior: String,
    pub(crate) interior_width: u16,
    pub(crate) index: usize,
    pub(crate) kind: TabKind,
    pub(crate) status: Option<Status>,
    pub(crate) maximize_x: Option<u16>,
    pub(crate) new_x: Option<u16>,
    pub(crate) close_x: Option<u16>,
}

impl WindowCornerTab {
    pub(crate) fn as_window_tab(&self) -> WindowTab {
        WindowTab { x: self.x, interior_width: self.interior_width }
    }
}

/// 🗂️ Tabs docked into one stack corner.
pub(crate) struct WindowCornerChipGroup {
    pub(crate) corner: WindowStackCorner,
    pub(crate) tabs: Vec<WindowCornerTab>,
}

/// 🧱️ How much chrome a window can afford.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ChromeMode {
    Flat,
    Full,
    Compact,
}

/// 🔗 `pub(crate)`: shared with `crate::tui::window`'s `paint_window`.
pub(crate) struct WindowChipLayout {
    pub(crate) mode: ChromeMode,
    pub(crate) groups: Vec<WindowCornerChipGroup>,
    pub(crate) top_body_y: u16,
    pub(crate) bottom_body_y: Option<u16>,
}

impl WindowChipLayout {
    fn flat(rect: Rect) -> Self {
        Self { mode: ChromeMode::Flat, groups: Vec::new(), top_body_y: rect.y, bottom_body_y: None }
    }

    /// 🪧 The row that carries a corner group's text.
    pub(crate) fn text_y(&self, corner: WindowStackCorner, rect: Rect) -> u16 {
        match (self.mode, corner.is_top()) {
            (ChromeMode::Compact, _) => rect.y,
            (_, true) => rect.y + 1,
            (_, false) => rect.y + rect.height.saturating_sub(2),
        }
    }
}

fn effective_stack_tabs(w: &WindowState) -> Vec<(usize, String, WindowStackCorner)> {
    if w.stack_tabs.is_empty() {
        let number_prefix = w.number.as_ref().map(|n| format!("{n} ")).unwrap_or_default();
        return vec![(0, format!("{number_prefix}{}", w.title), WindowStackCorner::TopLeft)];
    }
    w.stack_tabs.iter().enumerate().map(|(i, t)| (i, t.label.clone(), t.corner)).collect()
}

/// 🪡 A tab's interior ` label ✕ ` within `max_interior` cells; the label is elided, the close glyph never is.
fn tab_interior(label: &str, closable: bool, status: bool, max_interior: u16) -> Option<(String, u16, Option<u16>)> {
    let overhead = 2 + if closable { 2 } else { 0 } + if status { 2 } else { 0 };
    if max_interior < overhead + 1 {
        return None;
    }
    let shown = elide_end(label, (max_interior - overhead).min(TAB_LABEL_CELLS));
    let mut interior = if status { format!(" {} {shown} ", Status::Waiting.glyph(GlyphSet::Unicode, 0)) } else { format!(" {shown} ") };
    let mut close_off = None;
    if closable {
        close_off = Some(display_width(&interior));
        interior.push(WINDOW_TAB_CLOSE_GLYPH);
        interior.push(' ');
    }
    let width = display_width(&interior);
    Some((interior, width, close_off))
}

fn controls_interior(w: &WindowState) -> Option<(String, u16, Option<u16>, Option<u16>)> {
    let show_max = w.show_maximize();
    if !show_max && !w.new_tab {
        return None;
    }
    let mut interior = String::from(" ");
    let mut max_off = None;
    let mut new_off = None;
    if show_max {
        max_off = Some(display_width(&interior));
        interior.push(if w.zoomed { WINDOW_TAB_RESTORE_GLYPH } else { WINDOW_TAB_MAXIMIZE_GLYPH });
        interior.push(' ');
    }
    if w.new_tab {
        new_off = Some(display_width(&interior));
        interior.push(WINDOW_TAB_NEW_GLYPH);
        interior.push(' ');
    }
    let width = display_width(&interior);
    Some((interior, width, max_off, new_off))
}

fn overflow_chip(glyph: char, count: usize, leading: bool) -> (String, u16) {
    let interior = if leading { format!(" {glyph}{count} ") } else { format!(" {count}{glyph} ") };
    let width = display_width(&interior);
    (interior, width)
}

/// 🎞️ One tab strip: chips in order, the chip index of the first and last shown tab, hidden counts on each side.
struct Strip {
    chips: Vec<WindowCornerTab>,
}

/// 🧮️ The `[start, end)` slice of `widths` that fits `budget` and keeps `active` in view, preferring to extend to the right.
fn fit_window(widths: &[u16], active: usize, budget: u16, chip_overhead: u16, count_chip: impl Fn(usize) -> u16) -> (usize, usize) {
    let n = widths.len();
    if n == 0 {
        return (0, 0);
    }
    let total: u16 = widths.iter().map(|w| w + chip_overhead).sum();
    if total <= budget {
        return (0, n);
    }
    let active = active.min(n - 1);
    let (mut start, mut end) = (active, active + 1);
    let used = |start: usize, end: usize| -> u16 {
        let tabs: u16 = widths[start..end].iter().map(|w| w + chip_overhead).sum();
        tabs + if start > 0 { count_chip(start) + chip_overhead } else { 0 } + if end < n { count_chip(n - end) + chip_overhead } else { 0 }
    };
    loop {
        let mut grew = false;
        if end < n && used(start, end + 1) <= budget {
            end += 1;
            grew = true;
        }
        if start > 0 && used(start - 1, end) <= budget {
            start -= 1;
            grew = true;
        }
        if !grew {
            break;
        }
    }
    (start, end)
}

/// 🛤️ Builds the chips of one strip inside `[x0, x1)`; `from_left` anchors it to the left edge, else to the right.
fn build_strip(entries: &[(usize, String)], w: &WindowState, x0: u16, x1: u16, from_left: bool, chip_overhead: u16) -> Strip {
    let budget = x1.saturating_sub(x0);
    if entries.is_empty() || budget < chip_overhead + 3 {
        return Strip { chips: Vec::new() };
    }
    let max_interior = TAB_CHIP_CELLS - chip_overhead;
    let has_status = |index: usize| w.stack_tabs.get(index).is_some_and(|tab| tab.status.is_some());
    let natural: Vec<(String, u16, Option<u16>)> = entries.iter().filter_map(|(index, label)| tab_interior(label, w.closable, has_status(*index), max_interior)).collect();
    if natural.len() != entries.len() {
        return Strip { chips: Vec::new() };
    }
    let widths: Vec<u16> = natural.iter().map(|(_, width, _)| *width).collect();
    let active_pos = entries.iter().position(|(index, _)| *index == w.active_stack_tab).unwrap_or(0);
    let count_chip = |count: usize| overflow_chip(OVERFLOW_PREV_GLYPH, count, true).1;
    let (start, end) = fit_window(&widths, active_pos, budget, chip_overhead, count_chip);
    let left_chip = if start > 0 { overflow_chip(OVERFLOW_PREV_GLYPH, start, true).1 + chip_overhead } else { 0 };
    let right_chip = if end < entries.len() { overflow_chip(OVERFLOW_NEXT_GLYPH, entries.len() - end, false).1 + chip_overhead } else { 0 };
    let tab_budget = budget.saturating_sub(left_chip + right_chip);
    let (show_left, show_right, tab_budget) = if tab_budget < chip_overhead + 3 { (false, false, budget) } else { (start > 0, end < entries.len(), tab_budget) };
    let mut chips: Vec<WindowCornerTab> = Vec::new();
    let mut push = |interior: String, interior_width: u16, index: usize, kind: TabKind, close_off: Option<u16>| {
        chips.push(WindowCornerTab { x: 0, interior, interior_width, index, kind, status: None, maximize_x: None, new_x: None, close_x: close_off });
    };
    if show_left {
        let (interior, width) = overflow_chip(OVERFLOW_PREV_GLYPH, start, true);
        push(interior, width, usize::MAX, TabKind::OverflowPrev(entries[start - 1].0), None);
    }
    for k in start..end {
        let (interior, width, close_off) = natural[k].clone();
        let mut room_width = width;
        let mut interior = interior;
        let mut close_off = close_off;
        let used: u16 = widths[start..end].iter().map(|w| w + chip_overhead).sum();
        if end - start == 1 && used > tab_budget {
            if let Some((fitted, fitted_width, fitted_close)) = tab_interior(&entries[k].1, w.closable, has_status(entries[k].0), tab_budget.saturating_sub(chip_overhead)) {
                interior = fitted;
                room_width = fitted_width;
                close_off = fitted_close;
            }
        }
        push(interior, room_width, entries[k].0, TabKind::Tab, close_off);
    }
    if show_right {
        let (interior, width) = overflow_chip(OVERFLOW_NEXT_GLYPH, entries.len() - end, false);
        push(interior, width, usize::MAX, TabKind::OverflowNext(entries[end].0), None);
    }
    let total: u16 = chips.iter().map(|chip| chip.interior_width + chip_overhead).sum();
    let mut x = if from_left { x0 } else { x1.saturating_sub(total).max(x0) };
    for chip in &mut chips {
        chip.x = x;
        chip.close_x = chip.close_x.map(|offset| x + 1 + offset);
        if chip.kind == TabKind::Tab {
            chip.status = w.stack_tabs.get(chip.index).and_then(|tab| tab.status);
        }
        x += chip.interior_width + chip_overhead;
    }
    Strip { chips }
}

fn controls_chip(w: &WindowState, x: u16) -> Option<WindowCornerTab> {
    let (interior, interior_width, max_off, new_off) = controls_interior(w)?;
    Some(WindowCornerTab { x, interior, interior_width, index: usize::MAX, kind: TabKind::Controls, status: None, maximize_x: max_off.map(|o| x + 1 + o), new_x: new_off.map(|o| x + 1 + o), close_x: None })
}

/// 🧷 Shared by paint and click hit-testing so the two can never drift apart.
/// Returns up to four corner chip groups; each tab carries its close glyph column, the controls chip the others.
/// `pub(crate)`: called from `crate::tui::window`'s `paint_window`.
pub(crate) fn window_chip_layout(w: &WindowState, rect: Rect) -> WindowChipLayout {
    if rect.width < 4 || rect.height < 3 {
        return WindowChipLayout::flat(rect);
    }
    let effective = effective_stack_tabs(w);
    let mut by_corner: [Vec<(usize, String)>; 4] = Default::default();
    for (index, label, corner) in &effective {
        let slot = match corner {
            WindowStackCorner::TopLeft => 0,
            WindowStackCorner::TopRight => 1,
            WindowStackCorner::BottomLeft => 2,
            WindowStackCorner::BottomRight => 3,
        };
        by_corner[slot].push((*index, label.clone()));
    }
    let has_top = !by_corner[0].is_empty() || !by_corner[1].is_empty();
    let has_bottom = !by_corner[2].is_empty() || !by_corner[3].is_empty();
    let min_rows = match (has_top, has_bottom) {
        (true, true) => MIN_FULL_ROWS_TWO_ROWS,
        (true, false) | (false, true) => MIN_FULL_ROWS_ONE_ROW,
        (false, false) => return WindowChipLayout::flat(rect),
    };
    if w.compact || rect.height < min_rows {
        return compact_layout(w, rect, &effective);
    }
    let controls_on_top = has_top;
    let controls = controls_chip(w, 0);
    let controls_width = controls.as_ref().map_or(0, |chip| chip.interior_width + 2);
    let mut groups = Vec::new();
    for (row_is_top, left, right) in [(true, 0usize, 1usize), (false, 2, 3)] {
        let reserve = if row_is_top == controls_on_top { controls_width } else { 0 };
        let avail = rect.width.saturating_sub(reserve);
        let (left_entries, right_entries) = (&by_corner[left], &by_corner[right]);
        let mid = rect.x + if left_entries.is_empty() || right_entries.is_empty() { avail } else { avail.div_ceil(2) };
        let corners = [(left, true), (right, false)];
        for (slot, from_left) in corners {
            let entries = &by_corner[slot];
            let (x0, x1) = match (from_left, right_entries.is_empty(), left_entries.is_empty()) {
                (true, true, _) => (rect.x, rect.x + avail),
                (true, false, _) => (rect.x, mid),
                (false, _, true) => (rect.x, rect.x + avail),
                (false, _, false) => (mid, rect.x + avail),
            };
            let strip = build_strip(entries, w, x0, x1, from_left, 2);
            if !strip.chips.is_empty() {
                let corner = match (row_is_top, from_left) {
                    (true, true) => WindowStackCorner::TopLeft,
                    (true, false) => WindowStackCorner::TopRight,
                    (false, true) => WindowStackCorner::BottomLeft,
                    (false, false) => WindowStackCorner::BottomRight,
                };
                groups.push(WindowCornerChipGroup { corner, tabs: strip.chips });
            }
        }
    }
    if let Some(mut chip) = controls {
        let x = rect.x + rect.width - controls_width;
        chip.x = x;
        chip.maximize_x = chip.maximize_x.map(|o| x + o);
        chip.new_x = chip.new_x.map(|o| x + o);
        let corner = if controls_on_top { WindowStackCorner::TopRight } else { WindowStackCorner::BottomRight };
        match groups.iter_mut().find(|group| group.corner == corner) {
            Some(group) => group.tabs.push(chip),
            None => groups.push(WindowCornerChipGroup { corner, tabs: vec![chip] }),
        }
    }
    if groups.is_empty() {
        return WindowChipLayout::flat(rect);
    }
    groups.sort_by_key(|group| match group.corner {
        WindowStackCorner::TopLeft => 0,
        WindowStackCorner::TopRight => 1,
        WindowStackCorner::BottomLeft => 2,
        WindowStackCorner::BottomRight => 3,
    });
    let has_top = groups.iter().any(|group| group.corner.is_top());
    let has_bottom = groups.iter().any(|group| !group.corner.is_top());
    WindowChipLayout {
        mode: ChromeMode::Full,
        groups,
        top_body_y: if has_top { rect.y + 2 } else { rect.y },
        bottom_body_y: if has_bottom { Some(rect.y + rect.height.saturating_sub(3)) } else { None },
    }
}

/// 📱️ One-row chrome: every tab sits on the top border, `┌ ◐ dev ✕ │ ✓ Tasks ✕ ──── ⤢ ┐`.
fn compact_layout(w: &WindowState, rect: Rect, effective: &[(usize, String, WindowStackCorner)]) -> WindowChipLayout {
    let entries: Vec<(usize, String)> = effective.iter().map(|(index, label, _)| (*index, label.clone())).collect();
    let controls = controls_chip(w, 0);
    let controls_width = controls.as_ref().map_or(0, |chip| chip.interior_width + 1);
    let x0 = rect.x;
    let x1 = (rect.x + rect.width).saturating_sub(1 + controls_width);
    let strip = build_strip(&entries, w, x0, x1.saturating_sub(1), true, 1);
    let mut tabs = strip.chips;
    if tabs.is_empty() && controls.is_none() {
        return WindowChipLayout::flat(rect);
    }
    if let Some(mut chip) = controls {
        let x = rect.x + rect.width - 1 - controls_width;
        chip.x = x;
        chip.maximize_x = chip.maximize_x.map(|o| x + o);
        chip.new_x = chip.new_x.map(|o| x + o);
        tabs.push(chip);
    }
    WindowChipLayout { mode: ChromeMode::Compact, groups: vec![WindowCornerChipGroup { corner: WindowStackCorner::TopLeft, tabs }], top_body_y: rect.y, bottom_body_y: None }
}

/// 🚪 Content inset that keeps children inside the closed outline, under the hairline a chip bends into.
pub(crate) fn window_content_padding(w: &WindowState, rect: Rect) -> [u16; 4] {
    let layout = window_chip_layout(w, rect);
    if layout.mode != ChromeMode::Full {
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

/// 🏗️ The three fixed shell regions plus one Window node per resolved `WindowMeasure`.
pub struct Shell {
    pub navbar: NodeId,
    pub canvas: NodeId,
    pub footer: NodeId,
    pub windows: Vec<(String, NodeId)>,
    pub mount_root: Option<NodeId>,
}

/// 🏛️ Builds navbar(top) + canvas(fill) + footer(bottom), then one Window per tiled slot.
pub fn shell(scene: &mut Scene, navbar: NavbarState, footer: FooterState, layout: &WindowLayout) -> Shell {
    use crate::tui::layout::{Constraint, Dimension, Direction};
    let root = scene.root();
    let navbar_id = scene.add(root, Node::new(NodeContent::Chrome(ChromeState::Navbar(navbar))));
    let canvas_id = scene.add(root, Node::new(NodeContent::Chrome(ChromeState::Canvas)));
    let footer_id = scene.add(root, Node::new(NodeContent::Chrome(ChromeState::Footer(footer))));
    scene.node_mut(root).set_constraint(Constraint { direction: Direction::Column, ..Default::default() });
    scene.node_mut(navbar_id).set_constraint(Constraint { height: Dimension::Cells(2), ..Default::default() });
    scene.node_mut(canvas_id).set_constraint(Constraint { height: Dimension::Weight(1), direction: Direction::Stack, ..Default::default() });
    scene.node_mut(footer_id).set_constraint(Constraint { height: Dimension::Cells(2), ..Default::default() });
    let mut windows = Vec::new();
    for measure in solve_window_layout(layout, Rect::default()) {
        let title = measure.title.clone().unwrap_or_else(|| measure.window_kind_id.clone());
        let id = scene.add(canvas_id, Node::new(NodeContent::Chrome(ChromeState::Window(Box::new(WindowState::new(title))))));
        scene.node_mut(id).set_constraint(window_constraint());
        windows.push((measure.window_kind_id, id));
    }
    Shell { navbar: navbar_id, canvas: canvas_id, footer: footer_id, windows, mount_root: None }
}

fn window_constraint() -> crate::tui::layout::Constraint {
    use crate::tui::layout::{Constraint, Dimension, Direction};
    Constraint { width: Dimension::Weight(1), height: Dimension::Weight(1), direction: Direction::Column, ..Default::default() }
}

/// 🪆 Mirrors `layout` into nested row/column/stack boxes under `canvas`, reparenting existing window nodes.
/// Axes become splitter-bearing `Axis` nodes one gutter cell apart; a zoomed layout hides every other window.
pub fn mount_window_layout(scene: &mut Scene, canvas: NodeId, layout: &WindowLayout, windows: &[(String, NodeId)], mount_root: &mut Option<NodeId>) {
    use crate::tui::layout::{Constraint, Dimension, Direction, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode};

    if let Some(old) = *mount_root {
        for (_, window) in windows {
            scene.reparent(*window, canvas);
        }
        scene.remove(old);
        *mount_root = None;
    }
    let mount = scene.add(canvas, Node::new(NodeContent::Box));
    scene.node_mut(mount).set_constraint(Constraint { direction: Direction::Stack, width: Dimension::Weight(1), height: Dimension::Weight(1), ..Default::default() });
    *mount_root = Some(mount);

    fn find_window(windows: &[(String, NodeId)], id: &str) -> Option<NodeId> {
        windows.iter().find(|(k, _)| k == id).map(|(_, n)| *n)
    }

    fn count_stacks(child: &WindowLayoutChild) -> usize {
        match child {
            WindowLayoutChild::Stack(s) => usize::from(!s.children.is_empty()),
            WindowLayoutChild::Axis(a) => a.children.iter().map(count_stacks).sum(),
        }
    }

    fn tab_label(child: &crate::tui::layout::WindowLayoutWindowNode) -> String {
        child.title.clone().unwrap_or_else(|| child.window_kind_id.clone())
    }

    /// 🧿 Which tab of a stack a window node shows, how many stacks share the canvas and whether it is zoomed.
    struct StackSlot<'a> {
        own: &'a str,
        active: &'a str,
        peers: usize,
        zoomed: bool,
    }

    fn style_window(scene: &mut Scene, windows: &[(String, NodeId)], win_id: NodeId, stack: &WindowLayoutStackNode, slot: &StackSlot<'_>) {
        let StackSlot { own, active, peers, zoomed } = *slot;
        let status_of = |scene: &Scene, id: &str| windows.iter().find(|(known, _)| known == id).and_then(|(_, node)| match &scene.node(*node).content {
            NodeContent::Chrome(ChromeState::Window(state)) => state.status,
            _ => None,
        });
        let tabs: Vec<WindowStackTabState> = stack.children.iter().map(|c| WindowStackTabState { label: tab_label(c), corner: c.corner.unwrap_or_default(), status: status_of(scene, &c.window_kind_id) }).collect();
        let ids: Vec<String> = stack.children.iter().map(|c| c.window_kind_id.clone()).collect();
        let active_idx = stack.children.iter().position(|t| t.window_kind_id == active).unwrap_or(0);
        let title = stack.children.iter().find(|c| c.window_kind_id == own).map(tab_label);
        let visible = own == active;
        scene.node_mut(win_id).set_visible(visible);
        scene.node_mut(win_id).set_constraint(window_constraint());
        if let Some(ChromeState::Window(ws)) = scene.node_mut(win_id).chrome() {
            ws.stack_tabs = tabs;
            ws.tab_ids = ids;
            ws.active_stack_tab = active_idx;
            ws.peers = peers;
            ws.zoomed = zoomed;
            if let Some(title) = title {
                ws.title = title;
            }
        }
    }

    fn mount_stack(scene: &mut Scene, parent: NodeId, stack: &WindowLayoutStackNode, windows: &[(String, NodeId)], weight: u16, peers: usize) {
        let box_id = scene.add(parent, Node::new(NodeContent::Box));
        scene.node_mut(box_id).set_constraint(Constraint { direction: Direction::Stack, width: Dimension::Weight(weight), height: Dimension::Weight(weight), ..Default::default() });
        let active = stack.active_window_kind_id.as_deref().unwrap_or_else(|| stack.children.first().map_or("", |c| c.window_kind_id.as_str()));
        for child in &stack.children {
            if let Some(win_id) = find_window(windows, &child.window_kind_id) {
                scene.reparent(win_id, box_id);
                style_window(scene, windows, win_id, stack, &StackSlot { own: &child.window_kind_id, active, peers, zoomed: false });
            }
        }
    }

    fn mount_child(scene: &mut Scene, parent: NodeId, child: &WindowLayoutChild, windows: &[(String, NodeId)], path: &[usize], peers: usize) {
        match child {
            WindowLayoutChild::Axis(axis) => {
                let is_row = axis.kind == "row";
                let weight = weight_of(axis.size);
                let box_id = scene.add(parent, Node::new(NodeContent::Axis(AxisState::new(is_row, path.to_vec()))));
                scene.node_mut(box_id).set_constraint(Constraint {
                    direction: if is_row { Direction::Row } else { Direction::Column },
                    width: Dimension::Weight(weight),
                    height: Dimension::Weight(weight),
                    gap: WINDOW_GAP,
                    ..Default::default()
                });
                for (i, c) in axis.children.iter().enumerate() {
                    let mut child_path = path.to_vec();
                    child_path.push(i);
                    mount_child(scene, box_id, c, windows, &child_path, peers);
                }
            }
            WindowLayoutChild::Stack(stack) => mount_stack(scene, parent, stack, windows, weight_of(stack.size), peers),
        }
    }

    let peers = match &layout.root {
        WindowLayoutRoot::Axis(axis) => count_stacks(&WindowLayoutChild::Axis(axis.clone())),
        WindowLayoutRoot::Stack(stack) => usize::from(!stack.children.is_empty()),
    };

    if let Some(zid) = layout.zoomed.as_deref() {
        if let Some(zoomed) = find_window(windows, zid) {
            for (id, window) in windows {
                scene.reparent(*window, mount);
                scene.node_mut(*window).set_visible(id == zid);
            }
            let stack = crate::tui::layout::stack_hosting(layout, zid);
            match stack {
                Some(stack) => style_window(scene, windows, zoomed, &stack, &StackSlot { own: zid, active: zid, peers, zoomed: true }),
                None => {
                    scene.node_mut(zoomed).set_constraint(window_constraint());
                    if let Some(ChromeState::Window(ws)) = scene.node_mut(zoomed).chrome() {
                        ws.zoomed = true;
                        ws.peers = peers;
                    }
                }
            }
            return;
        }
    }

    match &layout.root {
        WindowLayoutRoot::Axis(axis) => mount_child(scene, mount, &WindowLayoutChild::Axis(axis.clone()), windows, &[], peers),
        WindowLayoutRoot::Stack(stack) => mount_stack(scene, mount, stack, windows, 1, peers),
    }
}

impl Shell {
    /// 📍 Sets the task status of window `id` and mirrors it into the tab strip of every window that shows it as a tab.
    pub fn set_status(&self, scene: &mut Scene, id: &str, status: Option<Status>) {
        for (_, node) in &self.windows {
            if let Some(ChromeState::Window(state)) = scene.node_mut(*node).chrome() {
                if self.windows.iter().any(|(known, own)| known == id && own == node) {
                    state.status = status;
                }
                let tabs = state.tab_ids.clone();
                for (index, tab_id) in tabs.iter().enumerate() {
                    if tab_id == id {
                        if let Some(tab) = state.stack_tabs.get_mut(index) {
                            tab.status = status;
                        }
                    }
                }
            }
        }
    }

    /// 🔨 Rebuilds the tiling mount tree and reparents window nodes.
    pub fn remount(&mut self, scene: &mut Scene, layout: &WindowLayout) {
        mount_window_layout(scene, self.canvas, layout, &self.windows, &mut self.mount_root);
    }
}
