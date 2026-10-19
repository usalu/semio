//! 🔎️ tui paint function for the Divider element — extracted from `widget` mod's inline body
//! (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a crate-root sibling module of
//! `crate::tui::widget` (see that mod's `use crate::tui::divider::paint_divider;`).

use semio_framework_ui_tui_core::cell::CellBuffer;
use semio_framework_ui_tui_core::geometry::{Pos, Rect};
use semio_framework_ui_tui_core::text::display_width;
use semio_framework_ui_tui_core::theme::{Role, Surface, Theme};
use crate::widget::DividerState;

pub fn paint_divider(d: &DividerState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    let bg = buf.get(rect.x, rect.y).map_or(theme.surface(Surface::Base), |c| c.bg);
    let fg = theme.role(Role::BorderNormal);
    buf.hline(Pos { x: rect.x, y: rect.y }, rect.width, '\u{2500}', fg, bg);
    if let Some(label) = &d.label {
        let text = format!(" {label} ");
        let x = rect.x + rect.width.saturating_sub(display_width(&text)) / 2;
        buf.put_str(Pos { x, y: rect.y }, &text, theme.role(Role::Foreground), bg, 0, rect);
    }
}
