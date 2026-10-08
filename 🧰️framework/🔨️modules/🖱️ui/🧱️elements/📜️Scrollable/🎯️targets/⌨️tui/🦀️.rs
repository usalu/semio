//! 📜️ tui state, key, pointer and paint functions for the Scrollable element: a read-only text viewport that
//! scrolls vertically and horizontally by key, wheel and scroll bar. Wired as a crate-root sibling module of
//! `crate::tui::widget` (see that mod's `pub use crate::tui::scrollable::ScrollableState;`).

use crate::tui::cell::{Cell, CellBuffer};
use crate::tui::event::{Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Rect};
use crate::tui::rows::paint_scroll_bar;
use crate::tui::text::{display_width, window_cells};
use crate::tui::theme::{Role, Surface, Theme};
use crate::tui::widget::WidgetSignal;
use std::cell::Cell as Shared;

const WHEEL_LINES: isize = 3;
const COLUMN_STEP: isize = 4;

/// 📄 Lines of text with a vertical and a horizontal scroll offset.
pub struct ScrollableState {
    lines: Vec<String>,
    widest: u16,
    pub top: usize,
    pub left: u16,
    rows: Shared<usize>,
    columns: Shared<u16>,
}

impl ScrollableState {
    pub fn new(lines: Vec<String>) -> Self {
        let widest = lines.iter().map(|line| display_width(line)).max().unwrap_or(0);
        Self { lines, widest, top: 0, left: 0, rows: Shared::new(0), columns: Shared::new(0) }
    }

    /// 📝️ Replaces the text, keeping the scroll position inside it.
    pub fn set_lines(&mut self, lines: Vec<String>) {
        *self = Self { top: self.top, left: self.left, rows: self.rows.clone(), columns: self.columns.clone(), ..Self::new(lines) };
        self.clamp();
    }

    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    fn page(&self) -> usize {
        self.rows.get().max(1)
    }

    fn max_top(&self) -> usize {
        self.lines.len().saturating_sub(self.page())
    }

    fn max_left(&self) -> u16 {
        self.widest.saturating_sub(self.columns.get().max(1))
    }

    fn clamp(&mut self) {
        self.top = self.top.min(self.max_top());
        self.left = self.left.min(self.max_left());
    }

    fn scroll_vertical(&mut self, delta: isize) {
        self.top = self.top.saturating_add_signed(delta).min(self.max_top());
    }

    fn scroll_horizontal(&mut self, delta: isize) {
        self.left = self.left.saturating_add_signed(delta.clamp(-i16::MAX as isize, i16::MAX as isize) as i16).min(self.max_left());
    }
}

pub(crate) fn scrollable_on_key(s: &mut ScrollableState, ev: &KeyEvent) -> Option<WidgetSignal> {
    let page = s.page() as isize;
    match ev.key {
        Key::Up => s.scroll_vertical(-1),
        Key::Down => s.scroll_vertical(1),
        Key::PageUp => s.scroll_vertical(-page),
        Key::PageDown => s.scroll_vertical(page),
        Key::Home => {
            s.top = 0;
            s.left = 0;
        }
        Key::End => s.top = s.max_top(),
        Key::Left => s.scroll_horizontal(-COLUMN_STEP),
        Key::Right => s.scroll_horizontal(COLUMN_STEP),
        _ => {}
    }
    None
}

pub(crate) fn scrollable_on_mouse(s: &mut ScrollableState, rect: Rect, event: &MouseEvent) -> Option<WidgetSignal> {
    if !rect.contains(event.pos) {
        return None;
    }
    match event.kind {
        MouseKind::Scroll { dx, dy } => {
            s.scroll_vertical(isize::from(dy) * WHEEL_LINES);
            s.scroll_horizontal(isize::from(dx) * COLUMN_STEP);
        }
        MouseKind::Down(MouseButton::Left) | MouseKind::Drag(MouseButton::Left) if event.pos.x == rect.x + rect.width - 1 && s.lines.len() > usize::from(rect.height) => {
            let span = usize::from(rect.height.saturating_sub(1)).max(1);
            let at = usize::from(event.pos.y - rect.y);
            s.top = (at * s.max_top() + span / 2) / span;
            s.top = s.top.min(s.max_top());
        }
        _ => {}
    }
    None
}

pub(crate) fn paint_scrollable(s: &ScrollableState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    if rect.width == 0 || rect.height == 0 {
        return;
    }
    let bg = theme.surface(Surface::Window);
    let fg = theme.role(Role::Foreground);
    buf.fill_rect(rect, Cell::blank(fg, bg));
    let rows = usize::from(rect.height);
    let overflow = s.lines.len() > rows && rect.width > 1;
    let body = Rect::new(rect.x, rect.y, rect.width - u16::from(overflow), rect.height);
    s.rows.set(rows);
    s.columns.set(body.width);
    let top = s.top.min(s.lines.len().saturating_sub(rows));
    let left = s.left.min(s.widest.saturating_sub(body.width.max(1)));
    for (index, line) in s.lines.iter().skip(top).take(rows).enumerate() {
        let y = body.y + index as u16;
        let (visible, lead) = window_cells(line, left, body.width);
        buf.put_str(Pos { x: body.x + lead, y }, visible, fg, bg, 0, Rect::new(body.x, y, body.width, 1));
    }
    paint_scroll_bar(buf, theme, Rect::new(rect.x + rect.width - 1, rect.y, 1, rect.height), top, rows, s.lines.len());
}
