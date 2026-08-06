//! ⌨️ tui key-handling and paint functions for the Input element — extracted from `widget` mod's
//! inline body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a crate-root sibling
//! module of `crate::widget` (see that mod's `use crate::input::{input_on_key, paint_input};`).

use crate::cell::{Cell, CellBuffer};
use crate::event::{Key, KeyEvent};
use crate::geometry::{Pos, Rect};
use crate::text::{display_width, truncate_to};
use crate::theme::{Role, Surface, Theme};
use crate::widget::{InputState, WidgetSignal};

pub(crate) fn input_on_key(i: &mut InputState, ev: &KeyEvent) -> Option<WidgetSignal> {
    match ev.key {
        Key::Char(c) => {
            i.value.insert(i.cursor, c);
            i.cursor += c.len_utf8();
            Some(WidgetSignal::ValueChanged(i.value.clone()))
        }
        Key::Backspace if i.cursor > 0 => {
            let prev = i.value[..i.cursor].chars().next_back().map(char::len_utf8).unwrap_or(1);
            i.cursor -= prev;
            i.value.remove(i.cursor);
            Some(WidgetSignal::ValueChanged(i.value.clone()))
        }
        Key::Left if i.cursor > 0 => {
            i.cursor -= 1;
            None
        }
        Key::Right if i.cursor < i.value.len() => {
            i.cursor += 1;
            None
        }
        _ => None,
    }
}

pub(crate) fn paint_input(i: &InputState, theme: &Theme, rect: Rect, buf: &mut CellBuffer, focused: bool) {
    let bg = theme.surface(Surface::Panel);
    buf.fill_rect(rect, Cell::blank(theme.role(Role::Foreground), bg));
    let (text, fg) = if i.value.is_empty() { (i.placeholder.as_str(), theme.role(Role::MutedForeground)) } else { (i.value.as_str(), theme.role(Role::Foreground)) };
    let (text, _) = truncate_to(text, rect.width);
    buf.put_str(Pos { x: rect.x, y: rect.y }, text, fg, bg, 0, rect);
    if focused && rect.width > 0 {
        let cx = (rect.x + display_width(&i.value[..i.cursor.min(i.value.len())])).min(rect.x + rect.width - 1);
        buf.put(cx, rect.y, Cell { ch: '\u{2588}', fg: theme.role(Role::Accent), bg, attrs: 0, width: 1 });
    }
}
