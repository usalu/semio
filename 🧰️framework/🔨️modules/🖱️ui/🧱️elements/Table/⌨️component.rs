//! 🗂️ tui key-handling and paint functions for the Table element — extracted from `widget` mod's
//! inline body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a crate-root sibling
//! module of `crate::tui::widget` (see that mod's `use crate::tui::table::{table_on_key, paint_table,
//! paint_table_cell};`). `table_column_widths` is a Table-only layout helper (used solely by
//! `paint_table`) and moves along with it rather than staying behind in `widget`.

use crate::tui::cell::{attr, Cell, CellBuffer};
use crate::tui::event::{Key, KeyEvent};
use crate::tui::geometry::{Pos, Rect};
use crate::tui::text::truncate_to;
use crate::tui::theme::{Role, Surface, Theme};
use crate::tui::widget::{TableAlign, TableColumn, TableState, WidgetSignal};

pub(crate) fn table_on_key(t: &mut TableState, ev: &KeyEvent) -> Option<WidgetSignal> {
    let visible = t.visible_indices();
    if visible.is_empty() {
        return None;
    }
    let pos = visible.iter().position(|&i| i == t.selected).unwrap_or(0);
    match ev.key {
        Key::Up if pos > 0 => {
            t.selected = visible[pos - 1];
            Some(WidgetSignal::SelectionChanged(t.selected))
        }
        Key::Down if pos + 1 < visible.len() => {
            t.selected = visible[pos + 1];
            Some(WidgetSignal::SelectionChanged(t.selected))
        }
        Key::Right => {
            let row = &mut t.rows[t.selected];
            if row.has_children && !row.expanded {
                row.expanded = true;
                Some(WidgetSignal::SelectionChanged(t.selected))
            } else {
                None
            }
        }
        Key::Left => {
            let row = &mut t.rows[t.selected];
            if row.has_children && row.expanded {
                row.expanded = false;
                Some(WidgetSignal::SelectionChanged(t.selected))
            } else {
                None
            }
        }
        Key::Enter => {
            let row = &mut t.rows[t.selected];
            if row.has_children {
                row.expanded = !row.expanded;
                Some(WidgetSignal::SelectionChanged(t.selected))
            } else {
                Some(WidgetSignal::Activated(t.selected))
            }
        }
        _ => None,
    }
}

/// 📐️ Resolves each column's width: fixed columns keep their `width`, `width == 0` columns
/// split whatever space remains evenly.
fn table_column_widths(columns: &[TableColumn], total_width: u16) -> Vec<u16> {
    let fixed_total: u16 = columns.iter().filter(|c| c.width > 0).map(|c| c.width).sum();
    let gaps = columns.len().saturating_sub(1) as u16;
    let flex_count = columns.iter().filter(|c| c.width == 0).count() as u16;
    let remaining = total_width.saturating_sub(fixed_total + gaps);
    let flex_width = if flex_count > 0 { remaining / flex_count } else { 0 };
    columns.iter().map(|c| if c.width > 0 { c.width } else { flex_width }).collect()
}

pub(crate) fn paint_table_cell(buf: &mut CellBuffer, x: u16, y: u16, width: u16, text: &str, fg: [u8; 3], bg: [u8; 3], attrs: u8, align: TableAlign, clip: Rect) {
    let (t, tw) = truncate_to(text, width);
    let cell_x = match align {
        TableAlign::Left => x,
        TableAlign::Right => x + width.saturating_sub(tw),
    };
    buf.put_str(Pos { x: cell_x, y }, t, fg, bg, attrs, clip);
}

/// 🖌️ Header (muted, bold) + hairline underline, then hairline-separated body rows; tree rows
/// indent by level and carry a `▾️`/`▸️` expand marker — no vertical rules, no striping.
pub(crate) fn paint_table(t: &TableState, theme: &Theme, rect: Rect, buf: &mut CellBuffer, focused: bool) {
    if rect.width == 0 || rect.height == 0 || t.columns.is_empty() {
        return;
    }
    let bg = buf.get(rect.x, rect.y).map(|c| c.bg).unwrap_or(theme.surface(Surface::Window));
    buf.fill_rect(rect, Cell::blank(theme.role(Role::MutedForeground), bg));

    let widths = table_column_widths(&t.columns, rect.width);
    let mut xs = Vec::with_capacity(widths.len());
    let mut x = rect.x;
    for &w in &widths {
        xs.push(x);
        x += w + 1;
    }

    for ((col, &cx), &w) in t.columns.iter().zip(&xs).zip(&widths) {
        paint_table_cell(buf, cx, rect.y, w, &col.label, theme.role(Role::MutedForeground), bg, attr::BOLD, col.align, rect);
    }
    if rect.height == 1 {
        return;
    }
    buf.hline(Pos { x: rect.x, y: rect.y + 1 }, rect.width, '\u{2500}', theme.role(Role::BorderNormal), bg);
    if rect.height <= 2 {
        return;
    }

    let visible = t.visible_indices();
    if visible.is_empty() {
        let (text, w) = truncate_to("(empty)", rect.width);
        let cx = rect.x + rect.width.saturating_sub(w) / 2;
        buf.put_str(Pos { x: cx, y: rect.y + 2 }, text, theme.role(Role::MutedForeground), bg, 0, rect);
        return;
    }

    let body_height = rect.height - 2;
    let items_fit = usize::from((body_height / 2).max(1));
    let sel_pos = visible.iter().position(|&i| i == t.selected).unwrap_or(0);
    let first = if visible.len() <= items_fit { 0 } else { sel_pos.saturating_sub(items_fit / 2).min(visible.len() - items_fit) };

    let mut y = rect.y + 2;
    let bottom = rect.y + rect.height;
    for &row_idx in visible.iter().skip(first) {
        if y >= bottom {
            break;
        }
        let row = &t.rows[row_idx];
        let selected = row_idx == t.selected;
        let row_bg = if selected && focused { theme.role(Role::ActiveBase) } else { bg };
        let row_fg = if selected && focused { theme.role(Role::ActiveForeground) } else { theme.role(Role::MutedForeground) };
        buf.fill_rect(Rect::new(rect.x, y, rect.width, 1), Cell::blank(row_fg, row_bg));
        for (ci, ((col, &cx), &w)) in t.columns.iter().zip(&xs).zip(&widths).enumerate() {
            let text = if ci == 0 {
                let indent = "  ".repeat(usize::from(row.level));
                let marker = if !row.has_children {
                    "  "
                } else if row.expanded {
                    "\u{25be} "
                } else {
                    "\u{25b8} "
                };
                format!("{indent}{marker}{}", row.cells.first().map(String::as_str).unwrap_or(""))
            } else {
                row.cells.get(ci).cloned().unwrap_or_default()
            };
            paint_table_cell(buf, cx, y, w, &text, row_fg, row_bg, 0, col.align, rect);
        }
        y += 1;
        if y >= bottom {
            break;
        }
        buf.hline(Pos { x: rect.x, y }, rect.width, '\u{2500}', theme.role(Role::BorderNormal), bg);
        y += 1;
    }
}
