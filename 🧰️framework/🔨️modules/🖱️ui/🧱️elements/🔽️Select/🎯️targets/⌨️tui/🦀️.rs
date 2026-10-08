//! 🔎️ tui key-handling, pointer and paint functions for the Select element — a cycler over its options, wired as a
//! crate-root sibling module of `crate::tui::widget` (see that mod's `use crate::tui::select::{select_on_key, paint_select};`).

use crate::tui::cell::CellBuffer;
use crate::tui::event::{Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Rect};
use crate::tui::text::{elide, Elision};
use crate::tui::theme::{Glyph, Role, Surface, Theme};
use crate::tui::widget::{SelectState, WidgetSignal};

fn move_to(s: &mut SelectState, index: usize) -> Option<WidgetSignal> {
    let index = index.min(s.options.len().saturating_sub(1));
    (std::mem::replace(&mut s.index, index) != index).then_some(WidgetSignal::SelectionChanged(index))
}

fn cycle(s: &mut SelectState, forward: bool) -> WidgetSignal {
    let count = s.options.len();
    s.index = if forward { (s.index + 1) % count } else { (s.index + count - 1) % count };
    WidgetSignal::SelectionChanged(s.index)
}

pub(crate) fn select_on_key(s: &mut SelectState, ev: &KeyEvent) -> Option<WidgetSignal> {
    if s.options.is_empty() {
        return None;
    }
    match ev.key {
        Key::Left | Key::Up | Key::PageUp => Some(cycle(s, false)),
        Key::Right | Key::Down | Key::PageDown | Key::Enter => Some(cycle(s, true)),
        Key::Home => move_to(s, 0),
        Key::End => move_to(s, s.options.len() - 1),
        _ => None,
    }
}

pub(crate) fn select_on_mouse(s: &mut SelectState, rect: Rect, event: &MouseEvent) -> Option<WidgetSignal> {
    if s.options.is_empty() || !rect.contains(event.pos) {
        return None;
    }
    match event.kind {
        MouseKind::Down(MouseButton::Left) => Some(cycle(s, true)),
        MouseKind::Down(MouseButton::Right) => Some(cycle(s, false)),
        MouseKind::Scroll { dy, .. } if dy != 0 => Some(cycle(s, dy > 0)),
        _ => None,
    }
}

pub(crate) fn paint_select(s: &SelectState, theme: &Theme, rect: Rect, buf: &mut CellBuffer, focused: bool) {
    let fg = if focused { theme.role(Role::Accent) } else { theme.role(Role::Foreground) };
    let bg = buf.get(rect.x, rect.y).map_or(theme.surface(Surface::Panel), |c| c.bg);
    let value = s.options.get(s.index).map_or("", String::as_str);
    let text = format!("{}: {} {} {}", s.label, theme.glyphs.glyph(Glyph::PreviousOption), value, theme.glyphs.glyph(Glyph::NextOption));
    let text = elide(&text, rect.width, Elision::End, theme.glyphs.ellipsis());
    buf.put_str(Pos { x: rect.x, y: rect.y }, &text, fg, bg, 0, rect);
}
