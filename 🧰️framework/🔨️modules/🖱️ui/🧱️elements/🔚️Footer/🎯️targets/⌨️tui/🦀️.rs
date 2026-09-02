//! 🔎️ tui paint function for the Footer element — extracted from `chrome` mod's inline body
//! (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a crate-root sibling module of
//! `crate::tui::chrome` (see that mod's `use crate::tui::footer::paint_footer;`).

use crate::tui::cell::{Cell, CellBuffer};
use crate::tui::chrome::FooterState;
use crate::tui::geometry::{Pos, Rect};
use crate::tui::text::truncate_to;
use crate::tui::theme::{Role, Surface, Theme};

pub(crate) fn paint_footer(f: &FooterState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    let bg = theme.surface(Surface::Base);
    let (hairline, content) = rect.split_top(1);
    buf.hline(Pos { x: hairline.x, y: hairline.y }, hairline.width, '\u{2500}', theme.role(Role::BorderNormal), bg);
    buf.fill_rect(content, Cell::blank(theme.role(Role::Foreground), bg));
    let mut x = content.x;
    for hint in &f.hints {
        let key = format!(" {} ", hint.key);
        x += buf.put_str(Pos { x, y: content.y }, &key, theme.role(Role::Accent), bg, 0, content);
        let label = format!("{} ", hint.label);
        x += buf.put_str(Pos { x, y: content.y }, &label, theme.role(Role::MutedForeground), bg, 0, content);
    }
    let (status, status_w) = truncate_to(&f.status, content.width.saturating_sub(x - content.x));
    let status_x = content.x + content.width.saturating_sub(status_w);
    buf.put_str(Pos { x: status_x, y: content.y }, status, theme.role(Role::MutedForeground), bg, 0, content);
}
