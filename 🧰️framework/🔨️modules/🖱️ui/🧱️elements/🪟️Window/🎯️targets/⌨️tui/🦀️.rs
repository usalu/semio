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

/// 🪟 Paints one 2-row corner tab. The body hairline is drawn afterwards, so a tab stays closed until the active one is opened.
fn paint_corner_tab(buf: &mut CellBuffer, y: u16, tab: &WindowTab, is_bottom: bool, text_fg: [u8; 3], bg: [u8; 3], border: [u8; 3]) {
    let width = tab.interior_width + 2;
    let text_y = y + 1;
    let outer_y = y + 2;
    if is_bottom {
        buf.put(tab.x, text_y, Cell { ch: '\u{2502}', fg: border, bg, attrs: 0, width: 1 });
        buf.put_str(Pos { x: tab.x + 1, y: text_y }, &tab.interior, text_fg, bg, 0, Rect::new(tab.x + 1, text_y, tab.interior_width, 1));
        buf.put(tab.x + width - 1, text_y, Cell { ch: '\u{2502}', fg: border, bg, attrs: 0, width: 1 });
        buf.put(tab.x, outer_y, Cell { ch: '\u{2514}', fg: border, bg, attrs: 0, width: 1 });
        buf.hline(Pos { x: tab.x + 1, y: outer_y }, width.saturating_sub(2), '\u{2500}', border, bg);
        buf.put(tab.x + width - 1, outer_y, Cell { ch: '\u{2518}', fg: border, bg, attrs: 0, width: 1 });
    } else {
        buf.put(tab.x, y, Cell { ch: '\u{250c}', fg: border, bg, attrs: 0, width: 1 });
        buf.hline(Pos { x: tab.x + 1, y }, width.saturating_sub(2), '\u{2500}', border, bg);
        buf.put(tab.x + width - 1, y, Cell { ch: '\u{2510}', fg: border, bg, attrs: 0, width: 1 });
        buf.put(tab.x, text_y, Cell { ch: '\u{2502}', fg: border, bg, attrs: 0, width: 1 });
        buf.put_str(Pos { x: tab.x + 1, y: text_y }, &tab.interior, text_fg, bg, 0, Rect::new(tab.x + 1, text_y, tab.interior_width, 1));
        buf.put(tab.x + width - 1, text_y, Cell { ch: '\u{2502}', fg: border, bg, attrs: 0, width: 1 });
    }
}

fn paint_group(buf: &mut CellBuffer, rect: Rect, layout: &WindowChipLayout, corner: WindowStackCorner, tabs: &[WindowCornerTab], w: &WindowState, theme: &Theme, bg: [u8; 3], border: [u8; 3]) {
    if tabs.is_empty() {
        return;
    }
    let is_bottom = !corner.is_top();
    let y = if is_bottom { layout.bottom_body_y.unwrap_or(rect.y + rect.height.saturating_sub(3)) } else { rect.y };
    for tab in tabs {
        let active = tab.index == w.active_stack_tab;
        let fg = if active { theme.role(Role::Accent) } else { theme.role(Role::MutedForeground) };
        paint_corner_tab(buf, y, &tab.as_window_tab(), is_bottom, fg, bg, border);
    }
}

/// 🪟 Joins one tab to the body hairline. Inactive tabs keep that edge; only the active tab leaves it open.
fn paint_tab_seam(buf: &mut CellBuffer, tab: &WindowTab, y: u16, window_left: u16, window_right: u16, active: bool, is_bottom: bool, border: [u8; 3], bg: [u8; 3]) {
    let left = tab.x;
    let right = tab.x + tab.interior_width + 1;
    let at_left = left == window_left;
    let at_right = right == window_right;
    let (left_ch, right_ch) = if is_bottom {
        if active {
            (if at_left { '\u{2502}' } else { '\u{2510}' }, if at_right { '\u{2502}' } else { '\u{250c}' })
        } else {
            (if at_left { '\u{251c}' } else { '\u{252c}' }, if at_right { '\u{2524}' } else { '\u{252c}' })
        }
    } else if active {
        (if at_left { '\u{2502}' } else { '\u{2518}' }, if at_right { '\u{2502}' } else { '\u{2514}' })
    } else {
        (if at_left { '\u{251c}' } else { '\u{2534}' }, if at_right { '\u{2524}' } else { '\u{2534}' })
    };
    buf.put(left, y, Cell { ch: left_ch, fg: border, bg, attrs: 0, width: 1 });
    if tab.interior_width > 0 {
        let ch = if active { ' ' } else { '\u{2500}' };
        buf.hline(Pos { x: left + 1, y }, tab.interior_width, ch, if active { bg } else { border }, bg);
    }
    buf.put(right, y, Cell { ch: right_ch, fg: border, bg, attrs: 0, width: 1 });
}

/// 🚪 Fills the union of the body and raised chips. Notches stay on the parent surface.
fn fill_silhouette(buf: &mut CellBuffer, rect: Rect, layout: &WindowChipLayout, cell: Cell) {
    if !layout.has_tabs {
        buf.fill_rect(rect, cell);
        return;
    }
    let has_top = layout.groups.iter().any(|group| group.corner.is_top());
    let has_bottom = layout.groups.iter().any(|group| !group.corner.is_top());
    let bottom_y = rect.y + rect.height - 1;
    let top_edge = if has_top { layout.top_body_y } else { rect.y };
    let bottom_edge = if has_bottom { layout.bottom_body_y.unwrap_or(bottom_y) } else { bottom_y };
    if bottom_edge >= top_edge {
        buf.fill_rect(Rect::new(rect.x, top_edge, rect.width, bottom_edge - top_edge + 1), cell);
    }
    for group in &layout.groups {
        for tab in &group.tabs {
            let width = tab.interior_width.saturating_add(2);
            if group.corner.is_top() {
                let height = layout.top_body_y.saturating_sub(rect.y);
                if height > 0 {
                    buf.fill_rect(Rect::new(tab.x, rect.y, width, height), cell);
                }
            } else if let Some(hairline) = layout.bottom_body_y {
                let height = bottom_y.saturating_sub(hairline);
                if height > 0 {
                    buf.fill_rect(Rect::new(tab.x, hairline.saturating_add(1), width, height), cell);
                }
            }
        }
    }
}

/// 🖌️ Paints one closed window outline. Raised chips step out of the body and the notches beside them stay outside it.
pub(crate) fn paint_window(w: &WindowState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    if rect.width < 2 || rect.height < 2 {
        return;
    }
    let bg = theme.surface(Surface::Window);
    let border = if w.focused { theme.role(Role::ActiveBase) } else { theme.role(Role::BorderNormal) };
    let fg = theme.role(Role::Foreground);
    let bottom_y = rect.y + rect.height - 1;
    let right_x = rect.x + rect.width - 1;
    let layout = window_chip_layout(w, rect);
    fill_silhouette(buf, rect, &layout, Cell::blank(fg, bg));

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

    let wall_top = rect.y + 1;
    let wall_last = if has_bottom { bottom_body_y.saturating_sub(1) } else { bottom_y.saturating_sub(1) };
    if wall_last >= wall_top {
        let len = wall_last - wall_top + 1;
        buf.vline(Pos { x: rect.x, y: wall_top }, len, '\u{2502}', border, bg);
        buf.vline(Pos { x: right_x, y: wall_top }, len, '\u{2502}', border, bg);
    }

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

    let span = rect.width.saturating_sub(2);
    if has_top {
        if span > 0 {
            buf.hline(Pos { x: rect.x + 1, y: top_body_y }, span, '\u{2500}', border, bg);
        }
        let has_tr = layout.groups.iter().any(|g| g.corner == WindowStackCorner::TopRight);
        let has_tl = layout.groups.iter().any(|g| g.corner == WindowStackCorner::TopLeft);
        if !has_tl {
            buf.put(rect.x, top_body_y, Cell { ch: '\u{250c}', fg: border, bg, attrs: 0, width: 1 });
        }
        if !has_tr {
            buf.put(right_x, top_body_y, Cell { ch: '\u{2510}', fg: border, bg, attrs: 0, width: 1 });
        }
        for group in layout.groups.iter().filter(|group| group.corner.is_top()) {
            for tab in &group.tabs {
                paint_tab_seam(buf, &tab.as_window_tab(), top_body_y, rect.x, right_x, tab.index == w.active_stack_tab, false, border, bg);
            }
        }
    }

    if has_bottom {
        if span > 0 {
            buf.hline(Pos { x: rect.x + 1, y: bottom_body_y }, span, '\u{2500}', border, bg);
        }
        let has_br = layout.groups.iter().any(|g| g.corner == WindowStackCorner::BottomRight);
        let has_bl = layout.groups.iter().any(|g| g.corner == WindowStackCorner::BottomLeft);
        if !has_bl {
            buf.put(rect.x, bottom_body_y, Cell { ch: '\u{2514}', fg: border, bg, attrs: 0, width: 1 });
        }
        if !has_br {
            buf.put(right_x, bottom_body_y, Cell { ch: '\u{2518}', fg: border, bg, attrs: 0, width: 1 });
        }
        for group in layout.groups.iter().filter(|group| !group.corner.is_top()) {
            for tab in &group.tabs {
                paint_tab_seam(buf, &tab.as_window_tab(), bottom_body_y, rect.x, right_x, tab.index == w.active_stack_tab, true, border, bg);
            }
        }
    }
}
