//! 🔎️ tui paint function for the Divider element — extracted from `widget` mod's inline body
//! (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a crate-root sibling module of
//! `crate::tui::widget` (see that mod's `use crate::tui::divider::paint_divider;`).

use semio_ui_tui_core::cell::CellBuffer;
use semio_ui_tui_core::geometry::{Pos, Rect};
use semio_ui_tui_core::text::display_width;
use semio_ui_tui_core::theme::{Role, Surface, Theme};
use crate::widget::DividerState;

/// ➖️ Turns the vertical wall cell on either side of the rule into `├` / `┤` (heavy `┣` / `┫`), so a divider meeting a window's outline joins it instead of stopping against a `│`.
/// @see `⌨️tui/⚙️engine/🖌️paint-and-gutters/🦀️.rs`
fn join_walls(buf: &mut CellBuffer, rect: Rect) {
    let sides = [(rect.x.checked_sub(1), '\u{251c}', '\u{2523}'), (rect.x.checked_add(rect.width), '\u{2524}', '\u{252b}')];
    for (x, light, heavy) in sides {
        let Some(x) = x else { continue };
        let Some(cell) = buf.get(x, rect.y).copied() else { continue };
        let ch = match cell.ch {
            '\u{2502}' => light,
            '\u{2503}' => heavy,
            _ => continue,
        };
        buf.put_border(x, rect.y, semio_ui_tui_core::cell::Cell { ch, ..cell });
    }
}

pub fn paint_divider(d: &DividerState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    let bg = buf.get(rect.x, rect.y).map_or(theme.surface(Surface::Base), |c| c.bg);
    let fg = theme.role(Role::BorderNormal);
    buf.hline(Pos { x: rect.x, y: rect.y }, rect.width, '\u{2500}', fg, bg);
    join_walls(buf, rect);
    if let Some(label) = &d.label {
        let text = format!(" {label} ");
        let x = rect.x + rect.width.saturating_sub(display_width(&text)) / 2;
        buf.put_str(Pos { x, y: rect.y }, &text, theme.role(Role::Foreground), bg, 0, rect);
    }
}
