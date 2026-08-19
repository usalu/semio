//! 🔎️ tui paint function for the Window element — extracted from `chrome` mod's inline body
//! (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a crate-root sibling module of
//! `crate::tui::chrome` (see that mod's `use crate::tui::window::paint_window;`). `paint_corner_tab` is a
//! private helper used only by `paint_window` and stays module-private here. `window_chip_layout`,
//! `WindowTab` and `WindowChipLayout`'s fields stay in `chrome` mod (shared with
//! `ChromeState::window_hit`) and were promoted to `pub(crate)` there so this sibling module
//! can reach them.

use crate::tui::cell::{Cell, CellBuffer};
use crate::tui::chrome::{window_chip_layout, WindowChipLayout, WindowCornerTab, WindowState, WindowTab};
use crate::tui::geometry::{Pos, Rect};
use crate::tui::layout::WindowStackCorner;
use crate::tui::theme::{Role, Surface, Theme};

/// 🪟 Paints one 2-row corner tab. Top tabs bend their short wall down into the body hairline;
/// bottom tabs bend upward. `bend` is false for non-innermost tabs in a multi-tab corner group.
async fn paint_corner_tab(
    buf: &mut CellBuffer,
    y: u16,
    tab: &WindowTab,
    short_wall_is_left: bool,
    is_bottom: bool,
    bend: bool,
    text_fg: [u8; 3],
    bg: [u8; 3],
    border: [u8; 3],
) {
    let width = tab.interior_width + 2;
    let top_y = y;
    let text_y = y + 1;
    let bend_y = y + 2;
    if is_bottom {
        // y is the body-hairline row; text at y+1; outer edge at y+2
        if bend {
            let ch = if short_wall_is_left { '\u{2510}' } else { '\u{250c}' };
            let bx = if short_wall_is_left { tab.x } else { tab.x + width - 1 };
            buf.put(bx, top_y, Cell { ch, fg: border, bg, attrs: 0, width: 1 });
        }
        buf.put(tab.x, text_y, Cell { ch: '\u{2502}', fg: border, bg, attrs: 0, width: 1 });
        buf.put_str(Pos { x: tab.x + 1, y: text_y }, &tab.interior, text_fg, bg, 0, Rect::new(tab.x + 1, text_y, tab.interior_width, 1));
        buf.put(tab.x + width - 1, text_y, Cell { ch: '\u{2502}', fg: border, bg, attrs: 0, width: 1 });
        buf.put(tab.x, bend_y, Cell { ch: '\u{2514}', fg: border, bg, attrs: 0, width: 1 });
        buf.hline(Pos { x: tab.x + 1, y: bend_y }, width.saturating_sub(2), '\u{2500}', border, bg);
        buf.put(tab.x + width - 1, bend_y, Cell { ch: '\u{2518}', fg: border, bg, attrs: 0, width: 1 });
    } else {
        buf.put(tab.x, top_y, Cell { ch: '\u{250c}', fg: border, bg, attrs: 0, width: 1 });
        buf.hline(Pos { x: tab.x + 1, y: top_y }, width.saturating_sub(2), '\u{2500}', border, bg);
        buf.put(tab.x + width - 1, top_y, Cell { ch: '\u{2510}', fg: border, bg, attrs: 0, width: 1 });
        buf.put(tab.x, text_y, Cell { ch: '\u{2502}', fg: border, bg, attrs: 0, width: 1 });
        buf.put_str(Pos { x: tab.x + 1, y: text_y }, &tab.interior, text_fg, bg, 0, Rect::new(tab.x + 1, text_y, tab.interior_width, 1));
        buf.put(tab.x + width - 1, text_y, Cell { ch: '\u{2502}', fg: border, bg, attrs: 0, width: 1 });
        if bend {
            if short_wall_is_left {
                buf.put(tab.x, bend_y, Cell { ch: '\u{2518}', fg: border, bg, attrs: 0, width: 1 });
            } else {
                buf.put(tab.x + width - 1, bend_y, Cell { ch: '\u{2514}', fg: border, bg, attrs: 0, width: 1 });
            }
        }
    }
}

async fn paint_group(
    buf: &mut CellBuffer,
    rect: Rect,
    layout: &WindowChipLayout,
    corner: WindowStackCorner,
    tabs: &[WindowCornerTab],
    w: &WindowState,
    theme: &Theme,
    bg: [u8; 3],
    border: [u8; 3],
) {
    if tabs.is_empty() {
        return;
    }
    let is_bottom = !corner.is_top();
    let short_wall_is_left = !corner.is_left();
    let y = if is_bottom {
        layout.bottom_body_y.unwrap_or(rect.y + rect.height.saturating_sub(3))
    } else {
        rect.y
    };
    for (i, tab) in tabs.iter().enumerate() {
        let bend = if short_wall_is_left { i == 0 } else { i + 1 == tabs.len() };
        let active = tab.index == w.active_stack_tab;
        let fg = if active { theme.role(Role::Accent) } else { theme.role(Role::MutedForeground) };
        paint_corner_tab(buf, y, &tab.as_window_tab(), short_wall_is_left, is_bottom, bend, fg, bg, border);
    }
}

/// 🖌️ Paints a window whose stack tabs are recessed into up to four corners, each with inline
/// action glyphs. Body hairlines connect between the chip groups on the top and bottom edges.
pub(crate) async fn paint_window(w: &WindowState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    if rect.width < 2 || rect.height < 2 {
        return;
    }
    let bg = theme.surface(Surface::Window);
    let border = if w.focused { theme.role(Role::BorderEmphasized) } else { theme.role(Role::BorderNormal) };
    let fg = theme.role(Role::Foreground);
    buf.fill_rect(rect, Cell::blank(fg, bg));

    let bottom_y = rect.y + rect.height - 1;
    let right_x = rect.x + rect.width - 1;
    let layout = window_chip_layout(w, rect);

    if !layout.has_tabs {
        buf.hline(Pos { x: rect.x + 1, y: rect.y }, rect.width.saturating_sub(2), '\u{2500}', border, bg);
        buf.hline(Pos { x: rect.x + 1, y: bottom_y }, rect.width.saturating_sub(2), '\u{2500}', border, bg);
        buf.vline(Pos { x: rect.x, y: rect.y + 1 }, bottom_y.saturating_sub(rect.y + 1), '\u{2502}', border, bg);
        buf.vline(Pos { x: right_x, y: rect.y + 1 }, bottom_y.saturating_sub(rect.y + 1), '\u{2502}', border, bg);
        buf.put(rect.x, rect.y, Cell { ch: '\u{250c}', fg: border, bg, attrs: 0, width: 1 });
        buf.put(right_x, rect.y, Cell { ch: '\u{2510}', fg: border, bg, attrs: 0, width: 1 });
        buf.put(rect.x, bottom_y, Cell { ch: '\u{2514}', fg: border, bg, attrs: 0, width: 1 });
        buf.put(right_x, bottom_y, Cell { ch: '\u{2518}', fg: border, bg, attrs: 0, width: 1 });
        return;
    }

    let has_top = layout.groups.iter().any(|g| g.corner.is_top());
    let has_bottom = layout.groups.iter().any(|g| !g.corner.is_top());
    let top_body_y = layout.top_body_y;
    let bottom_body_y = layout.bottom_body_y.unwrap_or(bottom_y);

    // Left / right walls.
    let left_top = if has_top { rect.y + 1 } else { rect.y + 1 };
    let left_bot = if has_bottom { bottom_y.saturating_sub(1) } else { bottom_y.saturating_sub(1) };
    buf.vline(Pos { x: rect.x, y: left_top }, left_bot.saturating_sub(left_top).saturating_add(1).min(bottom_y.saturating_sub(rect.y)), '\u{2502}', border, bg);
    buf.vline(Pos { x: right_x, y: left_top }, left_bot.saturating_sub(left_top).saturating_add(1).min(bottom_y.saturating_sub(rect.y)), '\u{2502}', border, bg);

    if !has_top {
        buf.hline(Pos { x: rect.x + 1, y: rect.y }, rect.width.saturating_sub(2), '\u{2500}', border, bg);
        buf.put(rect.x, rect.y, Cell { ch: '\u{250c}', fg: border, bg, attrs: 0, width: 1 });
        buf.put(right_x, rect.y, Cell { ch: '\u{2510}', fg: border, bg, attrs: 0, width: 1 });
    }
    if !has_bottom {
        buf.hline(Pos { x: rect.x + 1, y: bottom_y }, rect.width.saturating_sub(2), '\u{2500}', border, bg);
        buf.put(rect.x, bottom_y, Cell { ch: '\u{2514}', fg: border, bg, attrs: 0, width: 1 });
        buf.put(right_x, bottom_y, Cell { ch: '\u{2518}', fg: border, bg, attrs: 0, width: 1 });
    }

    for group in &layout.groups {
        paint_group(buf, rect, &layout, group.corner, &group.tabs, w, theme, bg, border);
    }

    // Top hairline between corner groups (or to a flat opposite corner).
    if has_top {
        let left = layout.top_left_end_x.max(rect.x);
        let right = layout.top_right_start_x.min(right_x);
        if right > left {
            buf.hline(Pos { x: left, y: top_body_y }, right - left, '\u{2500}', border, bg);
        }
        let has_tr = layout.groups.iter().any(|g| g.corner == WindowStackCorner::TopRight);
        let has_tl = layout.groups.iter().any(|g| g.corner == WindowStackCorner::TopLeft);
        if !has_tl {
            buf.put(rect.x, top_body_y, Cell { ch: '\u{250c}', fg: border, bg, attrs: 0, width: 1 });
        }
        if !has_tr {
            buf.put(right_x, top_body_y, Cell { ch: '\u{2510}', fg: border, bg, attrs: 0, width: 1 });
        }
    }

    // Bottom hairline between corner groups.
    if has_bottom {
        let left = layout.bottom_left_end_x.max(rect.x);
        let right = layout.bottom_right_start_x.min(right_x);
        if right > left {
            buf.hline(Pos { x: left, y: bottom_body_y }, right - left, '\u{2500}', border, bg);
        }
        let has_br = layout.groups.iter().any(|g| g.corner == WindowStackCorner::BottomRight);
        let has_bl = layout.groups.iter().any(|g| g.corner == WindowStackCorner::BottomLeft);
        if !has_bl {
            buf.put(rect.x, bottom_y, Cell { ch: '\u{2514}', fg: border, bg, attrs: 0, width: 1 });
            // flat bottom-left up to hairline
            buf.put(rect.x, bottom_body_y, Cell { ch: '\u{2514}', fg: border, bg, attrs: 0, width: 1 });
        }
        if !has_br {
            buf.put(right_x, bottom_y, Cell { ch: '\u{2518}', fg: border, bg, attrs: 0, width: 1 });
            buf.put(right_x, bottom_body_y, Cell { ch: '\u{2518}', fg: border, bg, attrs: 0, width: 1 });
        }
    }
}
