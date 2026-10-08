//! 📜️ Row-list model shared by List, Wizard, Tree, Table, Log and Scrollable: an incremental filter index that stays
//! fast for tens of thousands of rows, a viewport that follows the selection, hover, select-then-activate pointer
//! handling and a scroll bar painter. It knows rows by position and options by identity (their index in the
//! unfiltered list), so signals stay stable while the filter narrows and widens.

use crate::tui::cell::{Cell, CellBuffer};
use crate::tui::event::{Key, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Rect};
use crate::tui::theme::{Glyph, Rgb, Role, Theme};
use std::cell::Cell as Shared;
use std::ops::Range;

//#region 🔎️Filter Index
const HISTORY: usize = 32;

struct Query {
    tokens: Vec<Box<str>>,
    rows: Vec<u32>,
}

/// 🔎️ Lowercase search keys with an all-tokens-match filter that narrows the previous answer when the query extends it.
pub struct FilterIndex {
    keys: Vec<Box<str>>,
    all: Vec<u32>,
    history: Vec<Query>,
    scanned: u64,
}

fn tokens_of(query: &str) -> Vec<Box<str>> {
    let lowered = query.to_lowercase();
    let mut tokens: Vec<&str> = lowered.split_whitespace().collect();
    tokens.sort_by(|a, b| b.len().cmp(&a.len()).then_with(|| a.cmp(b)));
    let mut kept: Vec<&str> = Vec::new();
    for token in tokens {
        if !kept.iter().any(|longer| longer.contains(token)) {
            kept.push(token);
        }
    }
    kept.into_iter().map(Box::from).collect()
}

impl FilterIndex {
    pub fn new<I, S>(labels: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let keys: Vec<Box<str>> = labels.into_iter().map(|label| label.as_ref().to_lowercase().into_boxed_str()).collect();
        let all = (0..keys.len() as u32).collect();
        Self { keys, all, history: Vec::new(), scanned: 0 }
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// 🧮️ Rows examined by all filter passes so far: the counted-operations proxy for filter cost.
    pub fn scanned(&self) -> u64 {
        self.scanned
    }

    /// 🔬 Indices of the rows whose key contains every whitespace separated token of `query`, in row order.
    pub fn matches(&mut self, query: &str) -> &[u32] {
        let tokens = tokens_of(query);
        if tokens.is_empty() {
            return &self.all;
        }
        if let Some(at) = self.history.iter().position(|entry| entry.tokens == tokens) {
            let entry = self.history.remove(at);
            self.history.push(entry);
            return self.history.last().map_or(&self.all[..0], |entry| &entry.rows[..]);
        }
        let narrower = self.history.iter().filter(|entry| entry.tokens.iter().all(|old| tokens.iter().any(|new| new.contains(&**old)))).min_by_key(|entry| entry.rows.len());
        let rows: Vec<u32> = {
            let keys = &self.keys;
            let accepts = |row: u32| tokens.iter().all(|token| keys[row as usize].contains(&**token));
            match narrower {
                Some(entry) => {
                    self.scanned += entry.rows.len() as u64;
                    entry.rows.iter().copied().filter(|&row| accepts(row)).collect()
                }
                None => {
                    self.scanned += self.all.len() as u64;
                    self.all.iter().copied().filter(|&row| accepts(row)).collect()
                }
            }
        };
        if self.history.len() == HISTORY {
            self.history.remove(0);
        }
        self.history.push(Query { tokens, rows });
        self.history.last().map_or(&self.all[..0], |entry| &entry.rows[..])
    }
}
//#endregion 🔎️Filter Index

//#region 🪟️Viewport
const WHEEL_ROWS: isize = 3;

/// 🪟️ The first row of a view `rows` tall that keeps `selected` visible with the least movement from `top`.
pub fn follow_top(selected: usize, top: usize, rows: usize, count: usize) -> usize {
    let rows = rows.max(1);
    let top = top.min(count.saturating_sub(rows));
    if count == 0 {
        0
    } else if selected < top {
        selected
    } else if selected >= top + rows {
        selected + 1 - rows
    } else {
        top
    }
}

/// ⌨️ Where Up, Down, PageUp, PageDown, Home and End lead from `selected` in `count` rows of `page`; `None` for other keys.
pub fn navigate_to(key: Key, selected: usize, count: usize, page: usize) -> Option<usize> {
    let last = count.saturating_sub(1);
    let page = page.max(1);
    Some(match key {
        Key::Up => selected.saturating_sub(1),
        Key::Down => selected.saturating_add(1),
        Key::PageUp => selected.saturating_sub(page),
        Key::PageDown => selected.saturating_add(page),
        Key::Home => 0,
        Key::End => last,
        _ => return None,
    }
    .min(last))
}

/// 🖱️ The row position under `pos` for rows drawn in `area` starting at row `top`.
pub fn row_under(area: Rect, pos: Pos, top: usize, count: usize) -> Option<usize> {
    row_under_with(area, pos, top, count, 1)
}

/// 🧱 Like `row_under` for rows that take `stride` terminal lines each (a table row and its hairline); the extra lines belong to no row.
pub fn row_under_with(area: Rect, pos: Pos, top: usize, count: usize, stride: u16) -> Option<usize> {
    let stride = stride.max(1);
    if !area.contains(pos) || !(pos.y - area.y).is_multiple_of(stride) {
        return None;
    }
    let position = top + usize::from((pos.y - area.y) / stride);
    (position < count).then_some(position)
}

/// 🎨️ Foreground, background and attributes of a row: active fill when selected and focused, bold when selected
/// without focus, hover fill otherwise.
pub fn row_style(theme: &Theme, base: Rgb, selected: bool, focused: bool, hovered: bool) -> (Rgb, Rgb, u8) {
    match (selected, focused, hovered) {
        (true, true, _) => (theme.role(Role::ActiveForeground), theme.role(Role::ActiveBase), 0),
        (true, false, _) => (theme.role(Role::Foreground), base, crate::tui::cell::attr::BOLD),
        (false, _, true) => (theme.role(Role::Foreground), theme.role(Role::HoverInteractive), 0),
        _ => (theme.role(Role::Foreground), base, 0),
    }
}

/// 🗃️ Selection, scroll offset and hover of a row list, all by row position. The offset persists between
/// paints and follows the selection unless the wheel moved it away.
#[derive(Clone, Debug, Default)]
pub struct Rows {
    selected: usize,
    top: Shared<usize>,
    page: Shared<usize>,
    anchored: Shared<bool>,
    hover: Option<usize>,
}

impl Rows {
    pub fn new() -> Self {
        Self { selected: 0, top: Shared::new(0), page: Shared::new(0), anchored: Shared::new(true), hover: None }
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn top(&self) -> usize {
        self.top.get()
    }

    /// 📏️ How many rows fit at the last paint; 1 before the first paint.
    pub fn page(&self) -> usize {
        self.page.get().max(1)
    }

    pub fn hover(&self) -> Option<usize> {
        self.hover
    }

    /// 👆️ Moves the hover row; true when it changed.
    pub fn set_hover(&mut self, position: Option<usize>) -> bool {
        std::mem::replace(&mut self.hover, position) != position
    }

    /// 🎯️ Selects `position` clamped into `count` rows and follows it; true when the selection moved.
    pub fn select(&mut self, position: usize, count: usize) -> bool {
        let next = position.min(count.saturating_sub(1));
        self.anchored.set(true);
        std::mem::replace(&mut self.selected, next) != next
    }

    pub fn step(&mut self, delta: isize, count: usize) -> bool {
        self.select(self.selected.saturating_add_signed(delta), count)
    }

    pub fn page_step(&mut self, direction: isize, count: usize) -> bool {
        self.step(direction.signum() * self.page() as isize, count)
    }

    pub fn home(&mut self, count: usize) -> bool {
        self.select(0, count)
    }

    pub fn end(&mut self, count: usize) -> bool {
        self.select(count.saturating_sub(1), count)
    }

    /// 🛞 Scrolls the view by `wheel` steps (`dy > 0` down) without moving the selection.
    pub fn scroll(&mut self, wheel: isize, count: usize) {
        let max_top = count.saturating_sub(self.page());
        self.top.set(self.top.get().saturating_add_signed(wheel * WHEEL_ROWS).min(max_top));
        self.anchored.set(false);
    }

    /// 🕹️ Applies Up, Down, PageUp, PageDown, Home and End; `None` when `key` is not a navigation key.
    pub fn navigate(&mut self, key: Key, count: usize) -> Option<bool> {
        Some(match key {
            Key::Up => self.step(-1, count),
            Key::Down => self.step(1, count),
            Key::PageUp => self.page_step(-1, count),
            Key::PageDown => self.page_step(1, count),
            Key::Home => self.home(count),
            Key::End => self.end(count),
            _ => return None,
        })
    }

    /// 🔭 The row positions to draw in a view `rows` tall; fixes the offset so the selection stays visible unless the wheel parked it.
    pub fn window(&self, rows: usize, count: usize) -> Range<usize> {
        self.window_for(self.selected, rows, count)
    }

    /// 🪢 Like `window` for an owner that keeps the selected row somewhere else and passes it in.
    pub fn window_for(&self, selected: usize, rows: usize, count: usize) -> Range<usize> {
        self.page.set(rows);
        let rows = rows.max(1);
        let top = if self.anchored.get() { follow_top(selected, self.top.get(), rows, count) } else { self.top.get().min(count.saturating_sub(rows)) };
        self.top.set(top);
        top..(top + rows).min(count)
    }

    /// 🪞 The row position drawn `view_row` rows below the top of the view.
    pub fn row_at(&self, view_row: usize, count: usize) -> Option<usize> {
        let position = self.top.get() + view_row;
        (position < count).then_some(position)
    }

    /// 🔁️ Keeps selection and offset inside `count` rows after the list changed.
    pub fn clamp(&mut self, count: usize) {
        self.selected = self.selected.min(count.saturating_sub(1));
        self.top.set(self.top.get().min(count.saturating_sub(self.page())));
        self.hover = self.hover.filter(|&position| position < count);
    }

    pub fn reset(&mut self) {
        self.selected = 0;
        self.top.set(0);
        self.anchored.set(true);
        self.hover = None;
    }
}
//#endregion 🪟️Viewport

//#region 🖱️Pointer
/// 🖲️ What a pointer event did to a row list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pointer {
    Selected(usize),
    Activated(usize),
    Context { position: Pos, row: Option<usize> },
    Scrolled,
    Ignored,
}

impl Rows {
    /// ☝️ Select-then-activate: a press selects the row under the pointer, a double press activates it, a right
    /// press asks for its context menu and the wheel scrolls. `area` is where the rows are drawn, `count` how many exist.
    pub fn pointer(&mut self, area: Rect, event: &MouseEvent, count: usize) -> Pointer {
        self.pointer_with(area, event, count, 1)
    }

    /// 🪜 Like `pointer` for rows that take `stride` terminal lines each.
    pub fn pointer_with(&mut self, area: Rect, event: &MouseEvent, count: usize, stride: u16) -> Pointer {
        let row = row_under_with(area, event.pos, self.top.get(), count, stride);
        match event.kind {
            MouseKind::Down(MouseButton::Left) => match row {
                Some(position) if event.clicks >= 2 => {
                    self.select(position, count);
                    Pointer::Activated(position)
                }
                Some(position) => {
                    self.select(position, count);
                    Pointer::Selected(position)
                }
                None => Pointer::Ignored,
            },
            MouseKind::Down(MouseButton::Right) => {
                if let Some(position) = row {
                    self.select(position, count);
                }
                Pointer::Context { position: event.pos, row }
            }
            MouseKind::Scroll { dy, .. } if dy != 0 && area.contains(event.pos) => {
                self.scroll(isize::from(dy), count);
                Pointer::Scrolled
            }
            _ => Pointer::Ignored,
        }
    }

    /// 🧲 Moves the hover to the row under `pos` (or none); true when the hover changed.
    pub fn hover_at(&mut self, area: Rect, pos: Option<Pos>, count: usize) -> bool {
        self.hover_with(area, pos, count, 1)
    }

    /// 🧭 Like `hover_at` for rows that take `stride` terminal lines each.
    pub fn hover_with(&mut self, area: Rect, pos: Option<Pos>, count: usize, stride: u16) -> bool {
        let row = pos.and_then(|pos| row_under_with(area, pos, self.top.get(), count, stride));
        self.set_hover(row)
    }
}
//#endregion 🖱️Pointer

//#region 📃️Listing
/// 📃️ A list of labelled options with a filter query, a key-stable selection and a persistent viewport.
pub struct ListModel {
    labels: Vec<String>,
    index: FilterIndex,
    query: String,
    visible: Option<Vec<u32>>,
    pub rows: Rows,
}

impl ListModel {
    pub fn new(labels: Vec<String>) -> Self {
        let index = FilterIndex::new(&labels);
        Self { labels, index, query: String::new(), visible: None, rows: Rows::new() }
    }

    /// 📋 All options, filtered or not.
    pub fn len(&self) -> usize {
        self.labels.len()
    }

    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }

    pub fn labels(&self) -> &[String] {
        &self.labels
    }

    pub fn label(&self, option: usize) -> Option<&str> {
        self.labels.get(option).map(String::as_str)
    }

    /// 📊 Rows examined by filtering so far.
    pub fn scanned(&self) -> u64 {
        self.index.scanned()
    }

    /// 🔢️ Rows that pass the filter.
    pub fn count(&self) -> usize {
        self.visible.as_ref().map_or(self.labels.len(), Vec::len)
    }

    /// 🆔️ The option shown at row `position`.
    pub fn option_at(&self, position: usize) -> Option<usize> {
        match &self.visible {
            Some(rows) => rows.get(position).map(|&row| row as usize),
            None => (position < self.labels.len()).then_some(position),
        }
    }

    /// 📍️ The row where `option` is shown, if the filter lets it through.
    pub fn position_of(&self, option: usize) -> Option<usize> {
        match &self.visible {
            Some(rows) => rows.binary_search(&(option as u32)).ok(),
            None => (option < self.labels.len()).then_some(option),
        }
    }

    pub fn selected_option(&self) -> Option<usize> {
        self.option_at(self.rows.selected())
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    /// 🔍 Replaces the filter; the selection returns to the first match. True when the query changed.
    pub fn set_query(&mut self, query: &str) -> bool {
        if query == self.query {
            return false;
        }
        self.query = query.to_string();
        self.visible = if tokens_of(query).is_empty() { None } else { Some(self.index.matches(query).to_vec()) };
        self.rows.reset();
        true
    }

    pub fn push_query(&mut self, c: char) {
        let mut query = self.query.clone();
        query.push(c);
        self.set_query(&query);
    }

    /// ⌫️ Drops the last character of the query; false when it was already empty.
    pub fn pop_query(&mut self) -> bool {
        let Some(last) = self.query.chars().next_back() else { return false };
        let shorter = self.query[..self.query.len() - last.len_utf8()].to_string();
        self.set_query(&shorter);
        true
    }

    /// 🔄 Replaces the options, keeping the selected one when its label survives and the query active.
    pub fn set_labels(&mut self, labels: Vec<String>) {
        let selected = self.selected_option().and_then(|option| self.labels.get(option).cloned());
        self.index = FilterIndex::new(&labels);
        self.labels = labels;
        self.visible = if tokens_of(&self.query).is_empty() { None } else { Some(self.index.matches(&self.query).to_vec()) };
        let kept = selected.and_then(|label| self.labels.iter().position(|candidate| *candidate == label)).and_then(|option| self.position_of(option));
        let count = self.count();
        match kept {
            Some(position) => {
                self.rows.select(position, count);
            }
            None => self.rows.reset(),
        }
        self.rows.clamp(count);
    }
}
//#endregion 📃️Listing

//#region ↕️Scroll Bar
/// ↕️ Paints a one-column scroll bar in `track` when more than `rows` of `count` rows exist; true when it drew one.
pub fn paint_scroll_bar(buf: &mut CellBuffer, theme: &Theme, track: Rect, top: usize, rows: usize, count: usize) -> bool {
    if track.width == 0 || track.height == 0 || count <= rows || rows == 0 {
        return false;
    }
    let height = usize::from(track.height);
    let thumb = (rows * height / count).clamp(1, height);
    let travel = height - thumb;
    let max_top = count - rows;
    let offset = (top.min(max_top) * travel + max_top / 2).checked_div(max_top).unwrap_or(0);
    let bg = buf.get(track.x, track.y).map_or(theme.surface(crate::tui::theme::Surface::Window), |cell| cell.bg);
    let track_glyph = theme.glyphs.glyph(Glyph::ScrollTrack);
    let thumb_glyph = theme.glyphs.glyph(Glyph::ScrollThumb);
    for row in 0..height {
        let inside = (offset..offset + thumb).contains(&row);
        let (glyph, role) = if inside { (thumb_glyph, Role::Accent) } else { (track_glyph, Role::BorderNormal) };
        buf.fill_rect(Rect::new(track.x, track.y + row as u16, 1, 1), Cell::blank(theme.role(role), bg));
        buf.put_str(Pos { x: track.x, y: track.y + row as u16 }, glyph, theme.role(role), bg, 0, Rect::new(track.x, track.y + row as u16, 1, 1));
    }
    true
}
//#endregion ↕️Scroll Bar

#[cfg(test)]
#[path = "../🧪️tests/📜️rows/🦀️.rs"]
mod tests;
