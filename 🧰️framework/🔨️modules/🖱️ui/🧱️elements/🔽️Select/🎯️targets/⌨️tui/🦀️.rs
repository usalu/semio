//! 🔎️ tui key-handling and paint functions for the Select element — extracted from `widget` mod's
//! inline body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a crate-root sibling
//! module of `crate::tui::widget` (see that mod's `use crate::tui::select::{select_on_key, paint_select};`).

use crate::tui::cell::CellBuffer;
use crate::tui::event::{Key, KeyEvent};
use crate::tui::geometry::{Pos, Rect};
use crate::tui::text::truncate_to;
use crate::tui::theme::{Role, Surface, Theme};
use crate::tui::widget::{SelectState, WidgetSignal};

pub(crate) fn select_on_key(s: &mut SelectState, ev: &KeyEvent) -> Option<WidgetSignal> {
    if s.options.is_empty() {
        return None;
    }
    match ev.key {
        Key::Left => {
            s.index = (s.index + s.options.len() - 1) % s.options.len();
            Some(WidgetSignal::SelectionChanged(s.index))
        }
        Key::Right | Key::Enter => {
            s.index = (s.index + 1) % s.options.len();
            Some(WidgetSignal::SelectionChanged(s.index))
        }
        _ => None,
    }
}

pub(crate) fn paint_select(s: &SelectState, theme: &Theme, rect: Rect, buf: &mut CellBuffer, focused: bool) {
    let fg = if focused { theme.role(Role::Accent) } else { theme.role(Role::Foreground) };
    let bg = buf.get(rect.x, rect.y).map(|c| c.bg).unwrap_or(theme.surface(Surface::Panel));
    let value = s.options.get(s.index).map(String::as_str).unwrap_or("");
    let text = format!("{}: \u{2039} {} \u{203a}", s.label, value);
    let (text, _) = truncate_to(&text, rect.width);
    buf.put_str(Pos { x: rect.x, y: rect.y }, text, fg, bg, 0, rect);
}
