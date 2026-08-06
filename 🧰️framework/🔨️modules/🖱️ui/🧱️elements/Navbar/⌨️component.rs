//! 🔎️ tui paint function for the Navbar element — extracted from `chrome` mod's inline body
//! (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a crate-root sibling module of
//! `crate::chrome` (see that mod's `use crate::navbar::paint_navbar;`). `paint_items` is a private
//! per-side helper used only by `paint_navbar` and stays module-private here.

use crate::cell::{Cell, CellBuffer};
use crate::chrome::{NavItem, NavbarState};
use crate::geometry::{Pos, Rect};
use crate::text::display_width;
use crate::theme::{Role, Surface, Theme};

fn paint_items(items: &[NavItem], theme: &Theme, mut x: u16, y: u16, bg: [u8; 3], rect: Rect, buf: &mut CellBuffer) -> u16 {
    for item in items {
        let fg = if item.active { theme.role(Role::Accent) } else { theme.role(Role::Foreground) };
        let label = format!(" {} ", item.label);
        let w = buf.put_str(Pos { x, y }, &label, fg, bg, 0, rect);
        x += w;
    }
    x
}

pub(crate) fn paint_navbar(n: &NavbarState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    let bg = theme.surface(Surface::Base);
    let (content, hairline) = rect.split_bottom(1);
    buf.fill_rect(content, Cell::blank(theme.role(Role::Foreground), bg));
    paint_items(&n.left, theme, content.x, content.y, bg, content, buf);
    let center_text: String = n.center.iter().map(|i| i.label.clone()).collect::<Vec<_>>().join(" ");
    let center_x = content.x + content.width.saturating_sub(display_width(&center_text)) / 2;
    buf.put_str(Pos { x: center_x, y: content.y }, &center_text, theme.role(Role::MutedForeground), bg, 0, content);
    let right_width: u16 = n.right.iter().map(|i| display_width(&i.label) + 2).sum();
    let right_x = content.x + content.width.saturating_sub(right_width);
    paint_items(&n.right, theme, right_x, content.y, bg, content, buf);
    buf.hline(Pos { x: hairline.x, y: hairline.y }, hairline.width, '\u{2500}', theme.role(Role::BorderNormal), bg);
}
