//! 🧙️ tui key-handling and paint functions for the Wizard element.

use crate::tui::cell::{Cell, CellBuffer};
use crate::tui::event::KeyEvent;
use crate::tui::geometry::{Pos, Rect};
use crate::tui::text::truncate_to;
use crate::tui::theme::{Role, Surface, Theme};
use crate::tui::widget::{WidgetSignal, WizardState};

fn visible_indices(w: &WizardState) -> Vec<usize> {
    if w.filter.is_empty() {
        return (0..w.options.len()).collect();
    }
    let f = w.filter.to_ascii_lowercase();
    w.options.iter().enumerate().filter(|(_, o)| o.to_ascii_lowercase().contains(&f)).map(|(i, _)| i).collect()
}

pub(crate) fn wizard_on_key(w: &mut WizardState, ev: &KeyEvent) -> Option<WidgetSignal> {
    let vis = visible_indices(w);
    if vis.is_empty() && !matches!(ev.key, crate::tui::event::Key::Backspace | crate::tui::event::Key::Esc) {
        return None;
    }
    match ev.key {
        crate::tui::event::Key::Up => {
            if w.selected > 0 {
                w.selected -= 1;
                Some(WidgetSignal::SelectionChanged(w.selected))
            } else {
                None
            }
        }
        crate::tui::event::Key::Down => {
            if w.selected + 1 < vis.len() {
                w.selected += 1;
                Some(WidgetSignal::SelectionChanged(w.selected))
            } else {
                None
            }
        }
        crate::tui::event::Key::Enter => vis.get(w.selected).map(|&i| WidgetSignal::Activated(i)),
        crate::tui::event::Key::Esc => {
            w.filter.clear();
            w.selected = 0;
            None
        }
        crate::tui::event::Key::Backspace => {
            if !w.filter.is_empty() {
                w.filter.pop();
                w.selected = 0;
                None
            } else {
                Some(WidgetSignal::NavigateBack)
            }
        }
        crate::tui::event::Key::Char(c) if ev.mods == 0 => {
            w.filter.push(c);
            w.selected = 0;
            None
        }
        _ => None,
    }
}

pub(crate) fn paint_wizard(w: &WizardState, theme: &Theme, rect: Rect, buf: &mut CellBuffer, focused: bool) {
    let bg = theme.surface(Surface::Window);
    let vis = visible_indices(w);
    let breadcrumb = if w.steps.is_empty() { String::new() } else { w.steps.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>().join(" › ") };
    let mut y = rect.y;
    if !breadcrumb.is_empty() && rect.height > 0 {
        let fg = theme.role(Role::MutedForeground);
        buf.fill_rect(Rect::new(rect.x, y, rect.width, 1), Cell::blank(fg, bg));
        let (text, _) = truncate_to(&breadcrumb, rect.width);
        buf.put_str(Pos { x: rect.x, y }, text, fg, bg, 0, Rect::new(rect.x, y, rect.width, 1));
        y += 1;
    }
    let list_top = y;
    let list_height = rect.height.saturating_sub(y.saturating_sub(rect.y));
    for row in 0..list_height {
        let idx = w.offset + usize::from(row);
        let Some(&opt_i) = vis.get(idx) else { break };
        let item = &w.options[opt_i];
        let selected = idx == w.selected;
        let row_bg = if selected && focused { theme.role(Role::ActiveBase) } else { bg };
        let row_fg = if selected && focused { theme.role(Role::ActiveForeground) } else { theme.role(Role::Foreground) };
        let row_y = list_top + row;
        buf.fill_rect(Rect::new(rect.x, row_y, rect.width, 1), Cell::blank(row_fg, row_bg));
        let prefix = if selected && focused { "› " } else { "  " };
        let text = format!("{prefix}{item}");
        let (text, _) = truncate_to(&text, rect.width);
        buf.put_str(Pos { x: rect.x, y: row_y }, text, row_fg, row_bg, 0, Rect::new(rect.x, row_y, rect.width, 1));
    }
    if vis.is_empty() && list_height > 0 {
        let fg = theme.role(Role::MutedForeground);
        buf.put_str(Pos { x: rect.x, y: list_top }, "no matches", fg, bg, 0, Rect::new(rect.x, list_top, rect.width, 1));
    }
}
