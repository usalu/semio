//! 🔎️ tui paint function for the Window element — extracted from `chrome` mod's inline body
//! (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a crate-root sibling module of
//! `crate::tui::chrome` (see that mod's `use crate::tui::window::paint_window;`). `paint_corner_tab` is a
//! private helper used only by `paint_window` and stays module-private here. `window_chip_layout`,
//! `WindowTab` and `WindowChipLayout`'s fields stay in `chrome` mod (shared with
//! `ChromeState::window_control_at`) and were promoted to `pub(crate)` there so this sibling module
//! can reach them.

use crate::tui::cell::{Cell, CellBuffer};
use crate::tui::chrome::{window_chip_layout, WindowState, WindowTab};
use crate::tui::geometry::{Pos, Rect};
use crate::tui::theme::{Role, Surface, Theme};

/// 🪟️ Paints one 2-row corner tab: a normal `┌️─️┐️ / │️text│️` box, then bends its *short* wall (the
/// one that is not also the window's own permanent side wall) down one row into the main body's
/// top hairline — `└️` when the short wall is on the right (title tab), `┘️` when on the left
/// (controls tab).
fn paint_corner_tab(buf: &mut CellBuffer, y: u16, tab: &WindowTab, short_wall_is_left: bool, text_fg: [u8; 3], bg: [u8; 3], border: [u8; 3]) {
    let width = tab.interior_width + 2;
    buf.put(tab.x, y, Cell { ch: '\u{250c}', fg: border, bg, attrs: 0, width: 1 });
    buf.hline(Pos { x: tab.x + 1, y }, width.saturating_sub(2), '\u{2500}', border, bg);
    buf.put(tab.x + width - 1, y, Cell { ch: '\u{2510}', fg: border, bg, attrs: 0, width: 1 });
    buf.put(tab.x, y + 1, Cell { ch: '\u{2502}', fg: border, bg, attrs: 0, width: 1 });
    buf.put_str(Pos { x: tab.x + 1, y: y + 1 }, &tab.interior, text_fg, bg, 0, Rect::new(tab.x + 1, y + 1, tab.interior_width, 1));
    buf.put(tab.x + width - 1, y + 1, Cell { ch: '\u{2502}', fg: border, bg, attrs: 0, width: 1 });
    if short_wall_is_left {
        buf.put(tab.x, y + 2, Cell { ch: '\u{2518}', fg: border, bg, attrs: 0, width: 1 });
    } else {
        buf.put(tab.x + width - 1, y + 2, Cell { ch: '\u{2514}', fg: border, bg, attrs: 0, width: 1 });
    }
}

/// 🖌️ Paints a window whose title/control tabs are recessed into its top corners: each is a real
/// 2-row box sharing the window's own left/right wall, with its short inner wall bending down
/// into the main body's top edge — the 🖋️semio-window.sty "flowing" tab look, not text cut into a
/// flat border line. A side with no tab (controls not wanted, or too narrow to fit) simply stays
/// flat, its corner sitting at the main body's top row instead of rising two rows like a tab.
pub(crate) fn paint_window(w: &WindowState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
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

    let top_y = rect.y + 2;

    // bottom edge: always flat, full width
    buf.hline(Pos { x: rect.x + 1, y: bottom_y }, rect.width.saturating_sub(2), '\u{2500}', border, bg);
    buf.put(rect.x, bottom_y, Cell { ch: '\u{2514}', fg: border, bg, attrs: 0, width: 1 });
    buf.put(right_x, bottom_y, Cell { ch: '\u{2518}', fg: border, bg, attrs: 0, width: 1 });

    // left side: always a raised title tab
    buf.vline(Pos { x: rect.x, y: rect.y + 1 }, bottom_y.saturating_sub(rect.y + 1), '\u{2502}', border, bg);
    paint_corner_tab(buf, rect.y, &layout.title, false, theme.role(Role::Accent), bg, border);

    // right side: a raised controls tab, or — when absent — a plain flat corner at the body's top
    let body_right_x = match &layout.controls {
        Some(controls) => {
            buf.vline(Pos { x: right_x, y: rect.y + 1 }, bottom_y.saturating_sub(rect.y + 1), '\u{2502}', border, bg);
            paint_corner_tab(buf, rect.y, controls, true, theme.role(Role::MutedForeground), bg, border);
            controls.x
        }
        None => {
            buf.vline(Pos { x: right_x, y: top_y + 1 }, bottom_y.saturating_sub(top_y + 1), '\u{2502}', border, bg);
            buf.put(right_x, top_y, Cell { ch: '\u{2510}', fg: border, bg, attrs: 0, width: 1 });
            right_x
        }
    };

    // main body's top edge: from the title tab's bend to the right side's bend/corner
    let body_left_x = layout.title.x + layout.title.interior_width + 2;
    if body_right_x > body_left_x {
        buf.hline(Pos { x: body_left_x, y: top_y }, body_right_x - body_left_x, '\u{2500}', border, bg);
    }

    // per-stack tab strip on the body's top hairline when the stack has multiple windows
    if w.stack_tabs.len() > 1 && body_right_x > body_left_x + 2 {
        let mut x = body_left_x + 1;
        for (i, tab) in w.stack_tabs.iter().enumerate() {
            if x >= body_right_x {
                break;
            }
            let active = i == w.active_stack_tab;
            let label = if active { format!("[{}]", tab) } else { format!(" {} ", tab) };
            let room = body_right_x.saturating_sub(x);
            let fg = if active { theme.role(Role::Accent) } else { theme.role(Role::MutedForeground) };
            let attrs = if active { 1 } else { 0 };
            let written = buf.put_str(Pos { x, y: top_y }, &label, fg, bg, attrs, Rect::new(x, top_y, room, 1));
            x = x.saturating_add(written.max(1));
        }
    }
}
