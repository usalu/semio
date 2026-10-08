//! 💡️ tui overlay Tooltip: the hint a chrome control shows after the pointer rests on it for
//! `TOOLTIP_DELAY_MS` (the React `ChromeControlHint` delay). It never takes the pointer or the focus.
//! Wired as a crate-root sibling module of `crate::tui::widget`; the engine owns the dwell timer and
//! placement (see `crate::tui::engine::Tui::tick`).

use crate::tui::cell::{Cell, CellBuffer};
use crate::tui::geometry::{Pos, Rect, Size};
use crate::tui::text::{display_width, truncate_to};
use crate::tui::theme::{Role, Surface, Theme};

/// ⏱️ How long the pointer must rest before a tooltip opens.
pub const TOOLTIP_DELAY_MS: u64 = 400;

const TOOLTIP_MAX_WIDTH: u16 = 48;

/// 💭 Tooltip text, one line per `\n`.
pub struct TooltipState {
    pub text: String,
}

impl TooltipState {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

/// 📐️ The overlay's size for `viewport`: padded by one cell on each side.
pub(crate) fn tooltip_size(t: &TooltipState, viewport: Size) -> Size {
    let widest = t.text.split('\n').map(display_width).max().unwrap_or(0);
    let width = (widest + 2).min(TOOLTIP_MAX_WIDTH).min(viewport.width.max(1));
    let height = (t.text.split('\n').count() as u16).min(viewport.height.max(1));
    Size { width, height }
}

pub(crate) fn paint_tooltip(t: &TooltipState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    let bg = theme.surface(Surface::Menu);
    let fg = theme.role(Role::Foreground);
    buf.fill_rect(rect, Cell::blank(fg, bg));
    for (row, line) in t.text.split('\n').take(usize::from(rect.height)).enumerate() {
        let (head, _) = truncate_to(line, rect.width.saturating_sub(2));
        let clip = Rect::new(rect.x, rect.y + row as u16, rect.width, 1);
        buf.put_str(Pos { x: rect.x + 1, y: clip.y }, head, fg, bg, 0, clip);
    }
}
