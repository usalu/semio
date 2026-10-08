//! 🌳️ tui state, key, pointer, cursor and paint functions for the Tree element: a flat pre-order forest with
//! expand and collapse, a type-to-filter row and a filter that keeps the ancestors of every match. The filter
//! matches the whole path of a node, so a leaf is found by the names of its folders. Signals carry item indices.
//! Wired as a crate-root sibling module of `crate::tui::widget` (see that mod's `pub use crate::tui::tree::{TreeItem, TreeState};`).

use crate::tui::cell::{Cell, CellBuffer};
use crate::tui::event::{mods, Key, KeyEvent, MouseEvent};
use crate::tui::geometry::{Pos, Rect};
use crate::tui::rows::{navigate_to, paint_scroll_bar, row_style, FilterIndex, Pointer, Rows};
use crate::tui::text::{display_width, elide, Elision};
use crate::tui::theme::{Glyph, Role, Surface, Theme};
use crate::tui::widget::{CursorShape, CursorSpec, WidgetSignal};

const NONE: u32 = u32::MAX;

/// 🌿 One node of a tree given in pre-order: `depth` 0 is a root, each child is one deeper than its parent.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeItem {
    pub id: String,
    pub label: String,
    pub depth: u16,
}

impl TreeItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>, depth: u16) -> Self {
        Self { id: id.into(), label: label.into(), depth }
    }
}

/// 🌲 A navigable, filterable tree whose rows stay valid while folders open, close and the filter narrows.
pub struct TreeState {
    items: Vec<TreeItem>,
    parent: Vec<u32>,
    end: Vec<u32>,
    expanded: Vec<bool>,
    index: Box<FilterIndex>,
    query: String,
    matched: Vec<bool>,
    rows: Vec<u32>,
    pub view: Rows,
    /// 🈳️ The text shown when no node is visible; the application supplies it in the user's language.
    pub empty: String,
    /// ⌨️ Whether printable keys edit the filter.
    pub filterable: bool,
}

impl TreeState {
    pub fn new(mut items: Vec<TreeItem>) -> Self {
        let mut previous: Option<u16> = None;
        for item in &mut items {
            item.depth = previous.map_or(0, |depth| item.depth.min(depth + 1));
            previous = Some(item.depth);
        }
        let count = items.len();
        let mut parent = vec![NONE; count];
        let mut end = vec![count as u32; count];
        let mut stack: Vec<u32> = Vec::new();
        let mut path: Vec<&str> = Vec::new();
        let mut keys = Vec::with_capacity(count);
        for (at, item) in items.iter().enumerate() {
            while stack.last().is_some_and(|&top| items[top as usize].depth >= item.depth) {
                if let Some(closed) = stack.pop() {
                    end[closed as usize] = at as u32;
                }
            }
            parent[at] = stack.last().copied().unwrap_or(NONE);
            path.truncate(usize::from(item.depth));
            path.push(&item.label);
            keys.push(path.join(" "));
            stack.push(at as u32);
        }
        let mut state = Self { index: Box::new(FilterIndex::new(&keys)), items, parent, end, expanded: vec![false; count], query: String::new(), matched: vec![false; count], rows: Vec::new(), view: Rows::new(), empty: String::new(), filterable: true };
        state.rebuild(None);
        state
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn item(&self, item: usize) -> Option<&TreeItem> {
        self.items.get(item)
    }

    pub fn is_folder(&self, item: usize) -> bool {
        self.end.get(item).is_some_and(|&end| end as usize > item + 1)
    }

    pub fn is_expanded(&self, item: usize) -> bool {
        self.expanded.get(item).copied().unwrap_or(false)
    }

    pub fn parent_of(&self, item: usize) -> Option<usize> {
        self.parent.get(item).copied().filter(|&parent| parent != NONE).map(|parent| parent as usize)
    }

    /// 🔢️ Rows currently shown.
    pub fn count(&self) -> usize {
        self.rows.len()
    }

    pub fn row_item(&self, position: usize) -> Option<usize> {
        self.rows.get(position).map(|&item| item as usize)
    }

    pub fn position_of(&self, item: usize) -> Option<usize> {
        self.rows.binary_search(&(item as u32)).ok()
    }

    pub fn selected_item(&self) -> Option<usize> {
        self.row_item(self.view.selected())
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    fn filtering(&self) -> bool {
        self.query.split_whitespace().next().is_some()
    }

    /// 📂️ Opens or closes a folder; true when its state changed.
    pub fn set_expanded(&mut self, item: usize, expanded: bool) -> bool {
        if !self.is_folder(item) || self.expanded[item] == expanded {
            return false;
        }
        self.expanded[item] = expanded;
        let keep = self.selected_item();
        self.rebuild(keep);
        true
    }

    /// 📖 Opens every folder shallower than `depth`.
    pub fn expand_to_depth(&mut self, depth: u16) {
        for at in 0..self.items.len() {
            if self.is_folder(at) && self.items[at].depth < depth {
                self.expanded[at] = true;
            }
        }
        let keep = self.selected_item();
        self.rebuild(keep);
    }

    /// 🎯️ Selects `item`, opening its ancestors so it is visible; false when it cannot be shown under the filter.
    pub fn select_item(&mut self, item: usize) -> bool {
        let mut ancestor = self.parent_of(item);
        while let Some(at) = ancestor {
            self.expanded[at] = true;
            ancestor = self.parent_of(at);
        }
        self.rebuild(Some(item));
        self.position_of(item).is_some()
    }

    /// 🔎️ Replaces the filter; ancestors of matches stay visible and the selection moves to the first match.
    pub fn set_query(&mut self, query: &str) -> bool {
        if query == self.query {
            return false;
        }
        let keep = if query.split_whitespace().next().is_none() { self.selected_item() } else { None };
        self.query = query.to_string();
        match keep {
            Some(item) => {
                self.select_item(item);
            }
            None => {
                self.rebuild(None);
                self.view.reset();
                if let Some(first) = self.rows.iter().position(|&item| self.matched[item as usize]) {
                    self.view.select(first, self.rows.len());
                }
            }
        }
        true
    }

    fn rebuild(&mut self, keep: Option<usize>) {
        self.rows.clear();
        if self.filtering() {
            let hits = self.index.matches(&self.query).to_vec();
            self.matched.iter_mut().for_each(|flag| *flag = false);
            let mut shown = vec![false; self.items.len()];
            for &hit in &hits {
                self.matched[hit as usize] = true;
                let mut at = hit;
                while at != NONE && !shown[at as usize] {
                    shown[at as usize] = true;
                    at = self.parent[at as usize];
                }
            }
            self.rows.extend((0..self.items.len() as u32).filter(|&at| shown[at as usize]));
        } else {
            self.matched.iter_mut().for_each(|flag| *flag = false);
            let mut at = 0usize;
            while at < self.items.len() {
                self.rows.push(at as u32);
                at = if self.is_folder(at) && !self.expanded[at] { self.end[at] as usize } else { at + 1 };
            }
        }
        let count = self.rows.len();
        match keep.and_then(|item| self.position_of(item)) {
            Some(position) => {
                self.view.select(position, count);
            }
            None => self.view.clamp(count),
        }
    }

    fn marker_cells(&self, item: usize) -> u16 {
        self.items[item].depth.saturating_mul(2)
    }
}

fn filter_rows(t: &TreeState, rect: Rect) -> u16 {
    u16::from(t.filterable && rect.height > 1)
}

fn list_area(t: &TreeState, rect: Rect) -> Rect {
    let (_, rest) = rect.split_top(filter_rows(t, rect));
    let overflow = t.count() > usize::from(rest.height) && rest.width > 1;
    Rect::new(rest.x, rest.y, rest.width - u16::from(overflow), rest.height)
}

fn plain(ev: &KeyEvent) -> bool {
    ev.mods & !mods::SHIFT == 0
}

fn selection(t: &TreeState) -> Option<WidgetSignal> {
    t.selected_item().map(WidgetSignal::SelectionChanged)
}

pub(crate) fn tree_on_key(t: &mut TreeState, ev: &KeyEvent) -> Option<WidgetSignal> {
    let count = t.count();
    if count > 0 {
        if let Some(next) = navigate_to(ev.key, t.view.selected(), count, t.view.page()) {
            return t.view.select(next, count).then(|| selection(t)).flatten();
        }
    }
    let current = t.selected_item();
    match ev.key {
        Key::Right => {
            let item = current.filter(|&item| t.is_folder(item))?;
            if !t.filtering() && !t.is_expanded(item) {
                t.set_expanded(item, true);
                Some(WidgetSignal::Toggled(true))
            } else {
                let child = t.view.selected() + 1;
                t.view.select(child, t.count());
                selection(t)
            }
        }
        Key::Left => {
            let item = current?;
            if !t.filtering() && t.is_folder(item) && t.is_expanded(item) {
                t.set_expanded(item, false);
                Some(WidgetSignal::Toggled(false))
            } else {
                let parent = t.parent_of(item)?;
                t.view.select(t.position_of(parent)?, t.count());
                selection(t)
            }
        }
        Key::Enter => {
            let item = current?;
            if t.is_folder(item) {
                let open = !t.is_expanded(item);
                t.set_expanded(item, open);
                Some(WidgetSignal::Toggled(open))
            } else {
                Some(WidgetSignal::Activated(item))
            }
        }
        Key::Esc if !t.query.is_empty() => {
            t.set_query("");
            Some(WidgetSignal::ValueChanged(String::new()))
        }
        Key::Backspace if t.filterable && !t.query.is_empty() => {
            let mut query = t.query.clone();
            query.pop();
            t.set_query(&query);
            Some(WidgetSignal::ValueChanged(query))
        }
        Key::Char('u') if t.filterable && ev.mods == mods::CTRL && !t.query.is_empty() => {
            t.set_query("");
            Some(WidgetSignal::ValueChanged(String::new()))
        }
        Key::Char(c) if t.filterable && plain(ev) && !c.is_control() => {
            let mut query = t.query.clone();
            query.push(c);
            t.set_query(&query);
            Some(WidgetSignal::ValueChanged(query))
        }
        _ => None,
    }
}

pub(crate) fn tree_on_paste(t: &mut TreeState, text: &str) -> Option<WidgetSignal> {
    if !t.filterable {
        return None;
    }
    let mut query = t.query.clone();
    query.extend(text.chars().map(|c| if c.is_whitespace() { ' ' } else { c }).filter(|c| !c.is_control()));
    t.set_query(&query).then_some(WidgetSignal::ValueChanged(query))
}

pub(crate) fn tree_on_mouse(t: &mut TreeState, rect: Rect, event: &MouseEvent) -> Option<WidgetSignal> {
    let area = list_area(t, rect);
    let count = t.count();
    let on_marker = |t: &TreeState, position: usize| -> bool {
        t.row_item(position).is_some_and(|item| {
            let marker = area.x + t.marker_cells(item);
            t.is_folder(item) && event.pos.x >= marker && event.pos.x < marker + 2
        })
    };
    match t.view.pointer(area, event, count) {
        Pointer::Selected(position) if on_marker(t, position) => {
            let item = t.row_item(position)?;
            let open = !t.is_expanded(item);
            t.set_expanded(item, open);
            Some(WidgetSignal::Toggled(open))
        }
        Pointer::Selected(_) => selection(t),
        Pointer::Activated(position) => {
            let item = t.row_item(position)?;
            if t.is_folder(item) {
                let open = !t.is_expanded(item);
                t.set_expanded(item, open);
                Some(WidgetSignal::Toggled(open))
            } else {
                Some(WidgetSignal::Activated(item))
            }
        }
        Pointer::Context { position, row } => Some(WidgetSignal::ContextMenu { pos: position, item: row.and_then(|row| t.row_item(row)) }),
        Pointer::Scrolled | Pointer::Ignored => None,
    }
}

pub(crate) fn tree_set_hover(t: &mut TreeState, rect: Rect, pos: Option<Pos>) -> bool {
    let area = list_area(t, rect);
    let count = t.count();
    t.view.hover_at(area, pos, count)
}

pub(crate) fn tree_cursor(t: &TreeState, rect: Rect) -> Option<CursorSpec> {
    if filter_rows(t, rect) == 0 || rect.width == 0 {
        return None;
    }
    let x = (rect.x + 2 + display_width(&t.query)).min(rect.x + rect.width - 1);
    Some(CursorSpec { pos: Pos { x, y: rect.y }, shape: CursorShape::Bar, blink: true })
}

pub(crate) fn paint_tree(t: &TreeState, theme: &Theme, rect: Rect, buf: &mut CellBuffer, focused: bool) {
    if rect.width == 0 || rect.height == 0 {
        return;
    }
    let bg = theme.surface(Surface::Window);
    let muted = theme.role(Role::MutedForeground);
    let ellipsis = theme.glyphs.ellipsis();
    buf.fill_rect(rect, Cell::blank(theme.role(Role::Foreground), bg));
    if filter_rows(t, rect) == 1 {
        let line = Rect::new(rect.x, rect.y, rect.width, 1);
        let fg = if t.query.is_empty() { muted } else { theme.role(Role::Foreground) };
        buf.put_str(Pos { x: rect.x, y: rect.y }, &elide(&format!("/ {}", t.query), rect.width, Elision::Start, ellipsis), fg, bg, 0, line);
    }
    let area = list_area(t, rect);
    let count = t.count();
    let rows = usize::from(area.height);
    let window = t.view.window(rows, count);
    for (index, position) in window.clone().enumerate() {
        let Some(item) = t.row_item(position) else { continue };
        let y = area.y + index as u16;
        let (fg, row_bg, attrs) = row_style(theme, bg, position == t.view.selected(), focused, t.view.hover() == Some(position));
        let line = Rect::new(area.x, y, area.width, 1);
        buf.fill_rect(line, Cell::blank(fg, row_bg));
        let indent = t.marker_cells(item);
        let marker = if !t.is_folder(item) {
            " "
        } else if t.is_expanded(item) || t.filtering() {
            theme.glyphs.glyph(Glyph::Expanded)
        } else {
            theme.glyphs.glyph(Glyph::Collapsed)
        };
        let label_fg = if t.filtering() && !t.matched[item] { muted } else { fg };
        buf.put_str(Pos { x: area.x + indent, y }, marker, fg, row_bg, attrs, line);
        let room = area.width.saturating_sub(indent + 2);
        buf.put_str(Pos { x: area.x + indent + 2, y }, &elide(&t.items[item].label, room, Elision::End, ellipsis), label_fg, row_bg, attrs, line);
    }
    if count == 0 && area.height > 0 && !t.empty.is_empty() {
        buf.put_str(Pos { x: area.x, y: area.y }, &elide(&t.empty, area.width, Elision::End, ellipsis), muted, bg, 0, Rect::new(area.x, area.y, area.width, 1));
    }
    paint_scroll_bar(buf, theme, Rect::new(rect.x + rect.width - 1, area.y, 1, area.height), window.start, rows, count);
}
