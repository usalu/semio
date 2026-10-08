//! 📜️ tui state, key, pointer and paint functions for the Log element (scrollback/follow-mode text pane),
//! wired as a crate-root sibling module of `crate::tui::widget`
//! (see that mod's `pub use crate::tui::log::{LogScroll, LogState};`).

use crate::tui::cell::{Cell, CellBuffer};
use crate::tui::event::{Key, KeyEvent, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Rect};
use crate::tui::rows::paint_scroll_bar;
use crate::tui::text::{elide, Elision};
use crate::tui::theme::{Role, Surface, Theme};
use std::cell::Cell as Shared;
use std::collections::VecDeque;

const WHEEL_LINES: usize = 3;

/// 📍️ Where the view of a log sits: glued to the newest line, or at a fixed first visible line.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LogScroll {
    Follow,
    At(usize),
}

/// 🪵️ A bounded scrollback log view.
pub struct LogState {
    lines: VecDeque<String>,
    pub capacity: usize,
    pub scroll: LogScroll,
    page: Shared<usize>,
}

impl LogState {
    pub fn new(capacity: usize) -> Self {
        Self { lines: VecDeque::with_capacity(capacity), capacity, scroll: LogScroll::Follow, page: Shared::new(0) }
    }

    pub fn push(&mut self, line: &str) {
        if self.lines.len() >= self.capacity {
            self.lines.pop_front();
        }
        self.lines.push_back(line.to_string());
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.scroll = LogScroll::Follow;
    }

    pub fn lines(&self) -> &VecDeque<String> {
        &self.lines
    }

    fn page(&self) -> usize {
        self.page.get().max(1)
    }

    fn top(&self, rows: usize) -> usize {
        let last_top = self.lines.len().saturating_sub(rows);
        match self.scroll {
            LogScroll::Follow => last_top,
            LogScroll::At(top) => top.min(last_top),
        }
    }

    fn scroll_to(&mut self, top: usize) {
        let last_top = self.lines.len().saturating_sub(self.page());
        self.scroll = if top >= last_top { LogScroll::Follow } else { LogScroll::At(top) };
    }

    fn scroll_by(&mut self, delta: isize) {
        let current = self.top(self.page());
        self.scroll_to(current.saturating_add_signed(delta));
    }
}

pub(crate) fn log_on_key(log: &mut LogState, ev: &KeyEvent) {
    let page = log.page() as isize;
    match ev.key {
        Key::Up => log.scroll_by(-1),
        Key::Down => log.scroll_by(1),
        Key::PageUp => log.scroll_by(-page),
        Key::PageDown => log.scroll_by(page),
        Key::Home => log.scroll = LogScroll::At(0),
        Key::End => log.scroll = LogScroll::Follow,
        _ => {}
    }
}

pub(crate) fn log_on_mouse(log: &mut LogState, rect: Rect, event: &MouseEvent) {
    if let MouseKind::Scroll { dy, .. } = event.kind {
        if dy != 0 && rect.contains(event.pos) {
            log.scroll_by(isize::from(dy) * WHEEL_LINES as isize);
        }
    }
}

pub(crate) fn paint_log(log: &LogState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    if rect.width == 0 || rect.height == 0 {
        return;
    }
    let bg = theme.surface(Surface::Window);
    let fg = theme.role(Role::Foreground);
    buf.fill_rect(rect, Cell::blank(fg, bg));
    let rows = usize::from(rect.height);
    log.page.set(rows);
    let len = log.lines.len();
    let overflow = len > rows && rect.width > 1;
    let body = Rect::new(rect.x, rect.y, rect.width - u16::from(overflow), rect.height);
    let first = log.top(rows);
    for (row, line) in log.lines.iter().skip(first).take(rows).enumerate() {
        let line_rect = Rect::new(body.x, body.y + row as u16, body.width, 1);
        buf.put_str(Pos { x: body.x, y: line_rect.y }, &elide(line, body.width, Elision::End, theme.glyphs.ellipsis()), fg, bg, 0, line_rect);
    }
    paint_scroll_bar(buf, theme, Rect::new(rect.x + rect.width - 1, rect.y, 1, rect.height), first, rows, len);
}
