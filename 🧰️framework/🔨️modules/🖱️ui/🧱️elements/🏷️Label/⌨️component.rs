//! 🏷️ tui paint function for the Label element (foundational static/dynamic single-line text
//! primitive, mirroring the other `🫀️core/` concepts already extracted in ui-react) — extracted
//! from `widget` mod's inline body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as
//! a crate-root sibling module of `crate::tui::widget` (see that mod's `use crate::tui::label::paint_label;`).

use crate::tui::cell::CellBuffer;
use crate::tui::geometry::{Pos, Rect};
use crate::tui::text::truncate_to;
use crate::tui::theme::{Surface, Theme};
use crate::tui::widget::{Align, LabelState};

pub(crate) async fn paint_label(l: &LabelState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    let bg = buf.get(rect.x, rect.y).map(|c| c.bg).unwrap_or(theme.surface(Surface::Base));
    let fg = theme.role(l.role);
    let (text, width) = truncate_to(&l.text, rect.width);
    let x = match l.align {
        Align::Left => rect.x,
        Align::Center => rect.x + rect.width.saturating_sub(width) / 2,
        Align::Right => rect.x + rect.width.saturating_sub(width),
    };
    buf.put_str(Pos { x, y: rect.y }, text, fg, bg, 0, rect);
}
