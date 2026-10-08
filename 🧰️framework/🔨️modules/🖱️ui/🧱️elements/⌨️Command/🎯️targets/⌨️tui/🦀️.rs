//! ⌨️ tui overlay Palette: a command palette with an incremental all-words filter over app-provided items.
//! Wired as a crate-root sibling module of `crate::tui::widget`; the engine owns placement, focus
//! trapping and dismissal (see `crate::tui::engine::Tui::open_palette`). Every string comes from the app.

use crate::tui::cell::{attr, Cell, CellBuffer};
use crate::tui::dialog::paint_frame;
use crate::tui::event::{mods, Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Rect, Size};
use crate::tui::text::{display_width, truncate_to};
use crate::tui::theme::{Role, Surface, Theme};
use crate::tui::widget::{CursorShape, CursorSpec, WidgetSignal};

const PALETTE_MAX_WIDTH: u16 = 72;
const PALETTE_MAX_ROWS: u16 = 10;
const FRAME_ROWS: u16 = 4;

/// 🧾 One palette entry; `hint` shows right-aligned (a shortcut or a category).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PaletteItem {
    pub label: String,
    pub hint: String,
}

impl PaletteItem {
    pub fn new(label: impl Into<String>, hint: impl Into<String>) -> Self {
        Self { label: label.into(), hint: hint.into() }
    }
}

/// 🪄 A palette: `visible` holds the item indices that match `query`, `selected` indexes `visible`.
pub struct PaletteState {
    pub title: String,
    pub placeholder: String,
    pub empty_text: String,
    pub query: String,
    pub items: Vec<PaletteItem>,
    pub selected: usize,
    lowered: Vec<String>,
    visible: Vec<usize>,
    offset: std::cell::Cell<usize>,
}

impl PaletteState {
    pub fn new(items: Vec<PaletteItem>) -> Self {
        let lowered = items.iter().map(|item| format!("{} {}", item.label, item.hint).to_lowercase()).collect();
        let visible = (0..items.len()).collect();
        Self { title: String::new(), placeholder: String::new(), empty_text: String::new(), query: String::new(), items, selected: 0, lowered, visible, offset: std::cell::Cell::new(0) }
    }

    /// 🔎️ Item indices matching every query word, in item order.
    pub fn visible(&self) -> &[usize] {
        &self.visible
    }

    fn refilter(&mut self) {
        let query = self.query.to_lowercase();
        let words: Vec<&str> = query.split_whitespace().collect();
        self.visible = self.lowered.iter().enumerate().filter(|(_, text)| words.iter().all(|word| text.contains(word))).map(|(index, _)| index).collect();
        self.selected = 0;
        self.offset.set(0);
    }

    fn move_by(&mut self, delta: i32) -> Option<WidgetSignal> {
        if self.visible.is_empty() {
            return None;
        }
        let next = (self.selected as i32 + delta).clamp(0, self.visible.len() as i32 - 1) as usize;
        if next == self.selected {
            return None;
        }
        self.selected = next;
        Some(WidgetSignal::SelectionChanged(next))
    }
}

/// 📐️ The overlay's size for `viewport`: capped width, up to ten result rows under the query row.
pub(crate) fn palette_size(p: &PaletteState, viewport: Size) -> Size {
    let width = PALETTE_MAX_WIDTH.min(viewport.width.saturating_sub(4)).max(20.min(viewport.width));
    let rows = (p.visible.len() as u16).clamp(1, PALETTE_MAX_ROWS);
    let height = (rows + FRAME_ROWS).min(viewport.height.saturating_sub(2).max(4));
    Size { width, height }
}

fn list_rect(rect: Rect) -> Rect {
    Rect::new(rect.x + 1, rect.y + 3, rect.width.saturating_sub(2), rect.height.saturating_sub(FRAME_ROWS))
}

fn view_offset(p: &PaletteState, rows: usize) -> usize {
    let rows = rows.max(1);
    let mut offset = p.offset.get().min(p.visible.len().saturating_sub(rows));
    if p.selected < offset {
        offset = p.selected;
    } else if p.selected >= offset + rows {
        offset = p.selected + 1 - rows;
    }
    p.offset.set(offset);
    offset
}

pub(crate) fn palette_on_key(p: &mut PaletteState, ev: &KeyEvent) -> Option<WidgetSignal> {
    match ev.key {
        Key::Up => p.move_by(-1),
        Key::Down | Key::Tab => p.move_by(1),
        Key::BackTab => p.move_by(-1),
        Key::PageUp => p.move_by(-i32::from(PALETTE_MAX_ROWS)),
        Key::PageDown => p.move_by(i32::from(PALETTE_MAX_ROWS)),
        Key::Home => p.move_by(i32::MIN / 2),
        Key::End => p.move_by(i32::MAX / 2),
        Key::Enter => p.visible.get(p.selected).map(|&index| WidgetSignal::Activated(index)),
        Key::Backspace if !p.query.is_empty() => {
            p.query.pop();
            p.refilter();
            Some(WidgetSignal::ValueChanged(p.query.clone()))
        }
        Key::Char('u') if ev.mods & mods::CTRL != 0 && !p.query.is_empty() => {
            p.query.clear();
            p.refilter();
            Some(WidgetSignal::ValueChanged(String::new()))
        }
        Key::Char(c) if ev.mods & (mods::CTRL | mods::ALT) == 0 => {
            p.query.push(c);
            p.refilter();
            Some(WidgetSignal::ValueChanged(p.query.clone()))
        }
        _ => None,
    }
}

fn row_at(p: &PaletteState, rect: Rect, pos: Pos) -> Option<usize> {
    let list = list_rect(rect);
    if !list.contains(pos) {
        return None;
    }
    let slot = view_offset(p, usize::from(list.height)) + usize::from(pos.y - list.y);
    (slot < p.visible.len()).then_some(slot)
}

pub(crate) fn palette_on_mouse(p: &mut PaletteState, rect: Rect, event: &MouseEvent) -> Option<WidgetSignal> {
    match event.kind {
        MouseKind::Down(MouseButton::Left) => {
            let slot = row_at(p, rect, event.pos)?;
            p.selected = slot;
            p.visible.get(slot).map(|&index| WidgetSignal::Activated(index))
        }
        MouseKind::Scroll { dy, .. } if dy != 0 => p.move_by(i32::from(dy).signum() * 3),
        _ => None,
    }
}

pub(crate) fn palette_set_hover(p: &mut PaletteState, rect: Rect, pos: Option<Pos>) -> bool {
    let Some(slot) = pos.and_then(|pos| row_at(p, rect, pos)) else { return false };
    if p.selected != slot {
        p.selected = slot;
        return true;
    }
    false
}

/// ✏️ The caret sits after the query on the first content row.
pub(crate) fn palette_cursor(p: &PaletteState, rect: Rect) -> Option<CursorSpec> {
    if rect.width < 6 || rect.height < 3 {
        return None;
    }
    let x = (rect.x + 3 + display_width(&p.query)).min(rect.x + rect.width - 2);
    Some(CursorSpec { pos: Pos { x, y: rect.y + 1 }, shape: CursorShape::Bar, blink: true })
}

pub(crate) fn paint_palette(p: &PaletteState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    paint_frame(buf, rect, theme, Surface::Dialog, &p.title);
    if rect.width < 6 || rect.height < FRAME_ROWS {
        return;
    }
    let bg = theme.surface(Surface::Dialog);
    let fg = theme.role(Role::Foreground);
    let muted = theme.role(Role::MutedForeground);
    let inner = Rect::new(rect.x + 1, rect.y + 1, rect.width - 2, 1);
    buf.put_str(Pos { x: inner.x, y: inner.y }, "\u{203a}", theme.role(Role::Accent), bg, attr::BOLD, inner);
    let (text, color) = if p.query.is_empty() { (p.placeholder.as_str(), muted) } else { (p.query.as_str(), fg) };
    buf.put_str(Pos { x: inner.x + 2, y: inner.y }, text, color, bg, 0, inner);
    buf.hline(Pos { x: inner.x, y: rect.y + 2 }, inner.width, '\u{2500}', theme.role(Role::BorderNormal), bg);
    let list = list_rect(rect);
    if p.visible.is_empty() {
        buf.put_str(Pos { x: list.x + 1, y: list.y }, &p.empty_text, muted, bg, 0, list);
        return;
    }
    let offset = view_offset(p, usize::from(list.height));
    for row in 0..list.height {
        let Some(&index) = p.visible.get(offset + usize::from(row)) else { break };
        let item = &p.items[index];
        let selected = offset + usize::from(row) == p.selected;
        let (row_fg, row_bg) = if selected { (theme.role(Role::ActiveForeground), theme.role(Role::ActiveBase)) } else { (fg, bg) };
        let line = Rect::new(list.x, list.y + row, list.width, 1);
        buf.fill_rect(line, Cell::blank(row_fg, row_bg));
        let hint_width = display_width(&item.hint);
        let label_room = if hint_width == 0 { line.width.saturating_sub(2) } else { line.width.saturating_sub(hint_width + 4) };
        let (label, _) = truncate_to(&item.label, label_room);
        buf.put_str(Pos { x: line.x + 1, y: line.y }, label, row_fg, row_bg, 0, line);
        if hint_width > 0 && hint_width + 2 < line.width {
            buf.put_str(Pos { x: line.x + line.width - 1 - hint_width, y: line.y }, &item.hint, row_fg, row_bg, attr::DIM, line);
        }
    }
}
