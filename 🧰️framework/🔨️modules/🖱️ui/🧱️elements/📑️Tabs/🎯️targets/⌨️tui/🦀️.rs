//! 🗂️ tui key, pointer and paint functions for the Tabs element — extracted from `widget` mod's
//! inline body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a crate-root sibling
//! module of `crate::tui::widget` (see that mod's `use crate::tui::tabs::{...};`).

use crate::tui::cell::{attr, Cell, CellBuffer};
use crate::tui::event::{Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Rect};
use crate::tui::text::display_width;
use crate::tui::theme::{Role, Surface, Theme};
use crate::tui::widget::{TabsState, WidgetSignal};

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

/// 📏️ Each tab's cell span inside `rect`: one padding cell on both sides of its label, clipped at the right edge.
fn tab_spans(t: &TabsState, rect: Rect) -> Vec<Rect> {
    let mut x = rect.x;
    let end = rect.x + rect.width;
    let mut spans = Vec::new();
    for tab in &t.tabs {
        let width = (display_width(tab) + 2).min(end.saturating_sub(x));
        spans.push(Rect::new(x, rect.y, width, 1));
        x += width;
    }
    spans
}

fn tab_at(t: &TabsState, rect: Rect, pos: Pos) -> Option<usize> {
    tab_spans(t, rect).iter().position(|span| span.contains(pos))
}

/// 🖱️ A press selects the tab under it, a double press activates it, the wheel steps through the tabs without wrapping.
pub(crate) fn tabs_on_mouse(t: &mut TabsState, rect: Rect, event: &MouseEvent) -> Option<WidgetSignal> {
    match event.kind {
        MouseKind::Down(MouseButton::Left) => {
            let index = tab_at(t, rect, event.pos)?;
            if index == t.active {
                return (event.clicks == 2).then_some(WidgetSignal::Activated(index));
            }
            t.active = index;
            Some(WidgetSignal::TabChanged(index))
        }
        MouseKind::Scroll { dy, .. } if dy != 0 && !t.tabs.is_empty() => {
            let next = (t.active as i32 + i32::from(dy).signum()).clamp(0, t.tabs.len() as i32 - 1) as usize;
            if next == t.active {
                return None;
            }
            t.active = next;
            Some(WidgetSignal::TabChanged(next))
        }
        _ => None,
    }
}

pub(crate) fn tabs_set_hover(t: &mut TabsState, rect: Rect, pos: Option<Pos>) -> bool {
    let hover = pos.and_then(|pos| tab_at(t, rect, pos));
    let changed = t.hover != hover;
    t.hover = hover;
    changed
}

pub(crate) fn paint_tabs(t: &TabsState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    let bg = theme.surface(Surface::Panel);
    buf.fill_rect(rect, Cell::blank(theme.role(Role::Foreground), bg));
    for (i, (tab, span)) in t.tabs.iter().zip(tab_spans(t, rect)).enumerate() {
        let (fg, tab_bg, attrs) = if i == t.active {
            (theme.role(Role::ActiveForeground), theme.role(Role::ActiveBase), attr::BOLD)
        } else if t.hover == Some(i) {
            (theme.role(Role::Foreground), theme.role(Role::HoverInteractive), 0)
        } else {
            (theme.role(Role::MutedForeground), bg, 0)
        };
        buf.fill_rect(span, Cell::blank(fg, tab_bg));
        buf.put_str(Pos { x: span.x + 1, y: span.y }, tab, fg, tab_bg, attrs, span);
    }
}
