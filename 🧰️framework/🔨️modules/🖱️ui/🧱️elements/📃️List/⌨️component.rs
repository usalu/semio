//! 📃️ tui key-handling and paint functions for the List element — extracted from `widget` mod's
//! inline body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a crate-root sibling
//! module of `crate::tui::widget` (see that mod's `use crate::tui::list::{list_on_key, paint_list};`).

use crate::tui::cell::{Cell, CellBuffer};
use crate::tui::event::{Key, KeyEvent};
use crate::tui::geometry::{Pos, Rect};
use crate::tui::text::truncate_to;
use crate::tui::theme::{Role, Surface, Theme};
use crate::tui::widget::{ListState, WidgetSignal};

pub(crate) async fn list_on_key(l: &mut ListState, ev: &KeyEvent) -> Option<WidgetSignal> {
    match ev.key {
        Key::Up if l.selected > 0 => {
            l.selected -= 1;
            Some(WidgetSignal::SelectionChanged(l.selected))
        }
        Key::Down if l.selected + 1 < l.items.len() => {
            l.selected += 1;
            Some(WidgetSignal::SelectionChanged(l.selected))
        }
        Key::Char(' ') => {
            if let Some(m) = l.marks.get_mut(l.selected) {
                *m = !*m;
                return Some(WidgetSignal::Toggled(*m));
            }
            None
        }
        Key::Enter => Some(WidgetSignal::Activated(l.selected)),
        _ => None,
    }
}

pub(crate) async fn paint_list(l: &ListState, theme: &Theme, rect: Rect, buf: &mut CellBuffer, focused: bool) {
    let bg = theme.surface(Surface::Window);
    for row in 0..rect.height {
        let idx = l.offset + usize::from(row);
        let Some(item) = l.items.get(idx) else { break };
        let selected = idx == l.selected;
        let row_bg = if selected && focused { theme.role(Role::ActiveBase) } else { bg };
        let row_fg = if selected && focused { theme.role(Role::ActiveForeground) } else { theme.role(Role::Foreground) };
        buf.fill_rect(Rect::new(rect.x, rect.y + row, rect.width, 1), Cell::blank(row_fg, row_bg));
        let mark = l.marks.get(idx).copied().unwrap_or(false);
        let prefix = if mark { "\u{2713} " } else { "  " };
        let text = format!("{prefix}{item}");
        let (text, _) = truncate_to(&text, rect.width);
        buf.put_str(Pos { x: rect.x, y: rect.y + row }, text, row_fg, row_bg, 0, rect);
    }
}
