//! 🔎️ tui paint function for the Chip element — extracted from `widget` mod's inline body
//! (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a crate-root sibling module of
//! `crate::tui::widget` (see that mod's `use crate::tui::chip::paint_chip;`).

use crate::tui::cell::{Cell, CellBuffer};
use crate::tui::geometry::{Pos, Rect};
use crate::tui::theme::{Role, Surface, Theme};
use crate::tui::widget::ChipState;

pub(crate) fn paint_chip(c: &ChipState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    let bg = if c.on { theme.role(Role::Accent) } else { theme.surface(Surface::Panel) };
    let fg = if c.on { theme.role(Role::AccentForeground) } else { theme.role(Role::MutedForeground) };
    buf.fill_rect(rect, Cell::blank(fg, bg));
    let text = format!(" {} ", c.label);
    buf.put_str(Pos { x: rect.x, y: rect.y }, &text, fg, bg, 0, rect);
}
