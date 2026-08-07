//! 📜️ tui key-handling and paint functions for the Log element (scrollback/follow-mode text
//! pane) — extracted from `widget` mod's inline body (ticket
//! 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a crate-root sibling module of
//! `crate::tui::widget` (see that mod's `use crate::tui::log::{log_on_key, paint_log};`).

use crate::tui::cell::{Cell, CellBuffer};
use crate::tui::event::{Key, KeyEvent};
use crate::tui::geometry::Pos;
use crate::tui::geometry::Rect;
use crate::tui::text::truncate_to;
use crate::tui::theme::{Role, Surface, Theme};
use crate::tui::widget::{LogScroll, LogState};

pub(crate) fn log_on_key(log: &mut LogState, ev: &KeyEvent) {
    let len = log.lines().len();
    match ev.key {
        Key::PageUp => {
            log.scroll = LogScroll::At(match log.scroll {
                LogScroll::Follow => len.saturating_sub(1),
                LogScroll::At(n) => n.saturating_sub(10),
            })
        }
        Key::PageDown => {
            let next = match log.scroll {
                LogScroll::Follow => return,
                LogScroll::At(n) => n + 10,
            };
            log.scroll = if next + 1 >= len { LogScroll::Follow } else { LogScroll::At(next) };
        }
        Key::Home => log.scroll = LogScroll::At(0),
        Key::End => log.scroll = LogScroll::Follow,
        _ => {}
    }
}

pub(crate) fn paint_log(log: &LogState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    let bg = theme.surface(Surface::Window);
    let fg = theme.role(Role::Foreground);
    buf.fill_rect(rect, Cell::blank(fg, bg));
    let len = log.lines().len();
    let last = match log.scroll {
        LogScroll::Follow => len,
        LogScroll::At(n) => (n + 1).min(len),
    };
    let first = last.saturating_sub(usize::from(rect.height));
    for (row, line) in log.lines().iter().skip(first).take(last - first).enumerate() {
        let (text, _) = truncate_to(line, rect.width);
        buf.put_str(Pos { x: rect.x, y: rect.y + row as u16 }, text, fg, bg, 0, rect);
    }
}
