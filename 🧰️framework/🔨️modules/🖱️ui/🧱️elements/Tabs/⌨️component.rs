//! 🗂️ tui key-handling and paint functions for the Tabs element — extracted from `widget` mod's
//! inline body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a crate-root sibling
//! module of `crate::widget` (see that mod's `use crate::tabs::{tabs_on_key, paint_tabs};`).

use crate::cell::{attr, Cell, CellBuffer};
use crate::event::{Key, KeyEvent};
use crate::geometry::{Pos, Rect};
use crate::theme::{Role, Surface, Theme};
use crate::widget::{TabsState, WidgetSignal};

pub(crate) fn tabs_on_key(t: &mut TabsState, ev: &KeyEvent) -> Option<WidgetSignal> {
    if t.tabs.is_empty() {
        return None;
    }
    match ev.key {
        Key::Left => {
            t.active = (t.active + t.tabs.len() - 1) % t.tabs.len();
            Some(WidgetSignal::TabChanged(t.active))
        }
        Key::Right => {
            t.active = (t.active + 1) % t.tabs.len();
            Some(WidgetSignal::TabChanged(t.active))
        }
        _ => None,
    }
}

pub(crate) fn paint_tabs(t: &TabsState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    let bg = theme.surface(Surface::Panel);
    buf.fill_rect(rect, Cell::blank(theme.role(Role::Foreground), bg));
    let mut x = rect.x;
    for (i, tab) in t.tabs.iter().enumerate() {
        let active = i == t.active;
        let fg = if active { theme.role(Role::Accent) } else { theme.role(Role::MutedForeground) };
        let label = format!(" {tab} ");
        let w = buf.put_str(Pos { x, y: rect.y }, &label, fg, bg, if active { attr::BOLD } else { 0 }, rect);
        x += w;
    }
}
