//! 📝️ Bounded multiline UTF8 editing, grapheme navigation and terminal painting.

use semio_framework_ui_tui_core::cell::{Cell, CellBuffer};
use semio_framework_ui_tui_core::event::{mods, Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use semio_framework_ui_tui_core::geometry::{Pos, Rect};
use semio_framework_ui_tui_core::signal::{CursorShape, CursorSpec, WidgetSignal};
use semio_framework_ui_tui_core::text::{boundary_at_or_before, display_width, clusters as graphemes, next_boundary, previous_boundary};
use semio_framework_ui_tui_core::theme::{Role, Surface, Theme};

const MAXIMUM_BYTES: usize = 1_048_576;
const HISTORY_BYTES: usize = 16_777_216;

struct Edit { at: usize, removed: String, inserted: String, before: usize, after: usize }

/// 📝️ A local document projection with bounded reversible edit events and a byte-offset caret.
pub struct EditorState {
    pub value: String,
    pub cursor: usize,
    pub maximum_bytes: usize,
    pub rejected: bool,
    pub read_only: bool,
    pub anchor: Option<usize>,
    newline: &'static str,
    history: Vec<Edit>,
    future: Vec<Edit>,
    history_bytes: usize,
}

impl EditorState {
    pub fn new(value: impl Into<String>, maximum_bytes: usize) -> Self {
        let value = value.into();
        let newline = if value.contains("\r\n") { "\r\n" } else { "\n" };
        Self { value, newline, cursor: 0, maximum_bytes: maximum_bytes.min(MAXIMUM_BYTES), rejected: false, read_only: false, anchor: None, history: Vec::new(), future: Vec::new(), history_bytes: 0 }
    }

    fn caret(&self) -> usize { boundary_at_or_before(&self.value, self.cursor.min(self.value.len())) }

    fn row(&self) -> (usize, usize, usize) {
        let at = self.caret();
        let start = self.value[..at].rfind('\n').map_or(0, |index| index + 1);
        let end = self.value[at..].find('\n').map_or(self.value.len(), |index| at + index);
        (self.value[..start].bytes().filter(|byte| *byte == b'\n').count(), start, end)
    }

    pub fn position(&self) -> (usize, usize) {
        let (row, start, _) = self.row();
        (row + 1, column(&self.value[start..self.caret()]) + 1)
    }

    fn selection(&self) -> (usize, usize) { let caret = self.caret(); let anchor = self.anchor.map(|at| boundary_at_or_before(&self.value, at.min(self.value.len()))).unwrap_or(caret); (caret.min(anchor), caret.max(anchor)) }

    fn replace(&mut self, from: usize, to: usize, inserted: String) -> bool {
        if self.read_only { return false; }
        if self.value.len() - (to - from) + inserted.len() > self.maximum_bytes { self.rejected = true; return false; }
        self.rejected = false;
        let edit = Edit { at: from, removed: self.value[from..to].into(), after: from + inserted.len(), inserted, before: self.caret() };
        self.value.replace_range(from..to, &edit.inserted);
        self.cursor = edit.after;
        self.anchor = None;
        self.history_bytes += edit.removed.len() + edit.inserted.len();
        self.history.push(edit);
        self.future.clear();
        while self.history_bytes > HISTORY_BYTES || self.history.len() > 4096 {
            let old = self.history.remove(0); self.history_bytes -= old.removed.len() + old.inserted.len();
        }
        true
    }

    pub fn insert(&mut self, text: &str) -> bool {
        let newline = self.newline;
        let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
        let clean: String = normalized.chars().filter(|c| !c.is_control() || matches!(c, '\n' | '\t')).collect();
        let clean = clean.replace('\n', newline);
        if clean.is_empty() { return false; }
        let (from, to) = self.selection();
        self.replace(from, to, clean)
    }

    pub fn command(&mut self, command: &str) -> bool {
        if self.read_only && matches!(command, "backspace" | "delete" | "enter" | "undo" | "redo") { return false; }
        self.cursor = self.caret();
        let (_, start, end) = self.row();
        match command {
            "selectall" => { self.anchor = Some(0); self.cursor = self.value.len(); }
            "left" => self.cursor = previous_boundary(&self.value, self.cursor),
            "right" => self.cursor = next_boundary(&self.value, self.cursor),
            "home" => self.cursor = start,
            "end" => self.cursor = if end > start && self.value.as_bytes()[end - 1] == b'\r' { end - 1 } else { end },
            "up" | "down" => {
                let wanted = column(&self.value[start..self.cursor]);
                let target = if command == "up" { start.checked_sub(1).map(|end| { let start = self.value[..end].rfind('\n').map_or(0, |index| index + 1); (start, end) }) } else { (end < self.value.len()).then(|| { let start = end + 1; let end = self.value[start..].find('\n').map_or(self.value.len(), |index| start + index); (start, end) }) };
                if let Some((start, end)) = target { self.cursor = start + at_column(self.value[start..end].trim_end_matches('\r'), wanted); }
            }
            "backspace" if self.cursor > 0 => return self.replace(previous_boundary(&self.value, self.cursor), self.cursor, String::new()),
            "delete" if self.cursor < self.value.len() => return self.replace(self.cursor, next_boundary(&self.value, self.cursor), String::new()),
            "enter" => return self.insert("\n"),
            "undo" => if let Some(edit) = self.history.pop() { self.value.replace_range(edit.at..edit.at + edit.inserted.len(), &edit.removed); self.cursor = edit.before; self.anchor = None; self.history_bytes -= edit.removed.len() + edit.inserted.len(); self.future.push(edit); return true; },
            "redo" => if let Some(edit) = self.future.pop() { self.value.replace_range(edit.at..edit.at + edit.removed.len(), &edit.inserted); self.cursor = edit.after; self.anchor = None; self.history_bytes += edit.removed.len() + edit.inserted.len(); self.history.push(edit); return true; },
            _ => {}
        }
        false
    }
}

fn column(text: &str) -> usize { graphemes(text).fold(0, |cells, cluster| if cluster == "\t" { cells + 4 - cells % 4 } else { cells + usize::from(display_width(cluster)) }) }

fn at_column(text: &str, wanted: usize) -> usize {
    let mut cells = 0; let mut bytes = 0;
    for cluster in graphemes(text) { let next = if cluster == "\t" { cells + 4 - cells % 4 } else { cells + usize::from(display_width(cluster)) }; if next > wanted { break; } cells = next; bytes += cluster.len(); }
    bytes
}

fn viewport(editor: &EditorState, rect: Rect) -> (usize, usize) {
    let (row, start, _) = editor.row();
    (row.saturating_add(1).saturating_sub(usize::from(rect.height)), column(&editor.value[start..editor.caret()]).saturating_add(1).saturating_sub(usize::from(rect.width)))
}

pub fn editor_on_key(editor: &mut EditorState, event: &KeyEvent) -> Option<WidgetSignal> {
    let ctrl = event.mods & mods::CTRL != 0; let alt = event.mods & mods::ALT != 0;
    let (from, to) = editor.selection();
    if ctrl && event.key == Key::Char('a') { editor.command("selectall"); return None; }
    if ctrl && event.key == Key::Char('c') { return (from != to).then(|| WidgetSignal::Copy(editor.value[from..to].into())); }
    let moving = matches!(event.key, Key::Left | Key::Right | Key::Up | Key::Down | Key::Home | Key::End | Key::PageUp | Key::PageDown);
    if moving { if event.mods & mods::SHIFT != 0 { if editor.anchor.is_none() { editor.anchor = Some(editor.caret()); } } else { editor.anchor = None; } }
    let changed = match event.key {
        Key::Backspace | Key::Delete if from != to => editor.replace(from, to, String::new()),
        Key::Char('z') if ctrl => editor.command("undo"), Key::Char('y') if ctrl => editor.command("redo"),
        Key::Home if ctrl => { editor.cursor = 0; false }, Key::End if ctrl => { editor.cursor = editor.value.len(); false },
        Key::Char(c) if !ctrl && !alt && !c.is_control() => editor.insert(&c.to_string()),
        Key::Tab => editor.insert("\t"), Key::Enter => editor.command("enter"),
        Key::Left => editor.command("left"), Key::Right => editor.command("right"), Key::Up => editor.command("up"), Key::Down => editor.command("down"),
        Key::Home => editor.command("home"), Key::End => editor.command("end"), Key::Backspace => editor.command("backspace"), Key::Delete => editor.command("delete"),
        Key::PageUp | Key::PageDown => { for _ in 0..12 { editor.command(if event.key == Key::PageUp { "up" } else { "down" }); } false },
        _ => false,
    };
    changed.then_some(WidgetSignal::TextEdited)
}

pub fn editor_on_paste(editor: &mut EditorState, text: &str) -> Option<WidgetSignal> { editor.insert(text).then_some(WidgetSignal::TextEdited) }

pub fn editor_on_mouse(editor: &mut EditorState, rect: Rect, event: &MouseEvent) -> Option<WidgetSignal> {
    if !matches!(event.kind, MouseKind::Down(MouseButton::Left) | MouseKind::Drag(MouseButton::Left)) || !rect.contains(event.pos) { return None; }
    let (top, left) = viewport(editor, rect); let row = top + usize::from(event.pos.y - rect.y);
    let mut start = 0;
    for _ in 0..row { let Some(end) = editor.value[start..].find('\n') else { break }; start += end + 1; }
    let end = editor.value[start..].find('\n').map_or(editor.value.len(), |index| start + index);
    if matches!(event.kind, MouseKind::Down(_)) && event.mods & mods::SHIFT == 0 { editor.anchor = None; } else if editor.anchor.is_none() { editor.anchor = Some(editor.caret()); }
    editor.cursor = start + at_column(editor.value[start..end].trim_end_matches('\r'), left + usize::from(event.pos.x - rect.x));
    None
}

pub fn editor_cursor(editor: &EditorState, rect: Rect) -> Option<CursorSpec> {
    if rect.width == 0 || rect.height == 0 { return None; }
    let (row, start, _) = editor.row(); let (top, left) = viewport(editor, rect);
    Some(CursorSpec { pos: Pos { x: rect.x + (column(&editor.value[start..editor.caret()]) - left) as u16, y: rect.y + (row - top) as u16 }, shape: CursorShape::Bar, blink: true })
}

pub fn paint_editor(editor: &EditorState, theme: &Theme, rect: Rect, buffer: &mut CellBuffer, _focused: bool) {
    let bg = theme.surface(Surface::Panel); buffer.fill_rect(rect, Cell::blank(theme.role(Role::Foreground), bg));
    let (top, left) = viewport(editor, rect);
    let (selection_start, selection_end) = editor.selection();
    let mut byte = 0;
    for (row, line) in editor.value.split('\n').enumerate() {
        let line_start = byte; byte += line.len() + 1;
        if row < top { continue; } if row >= top + usize::from(rect.height) { break; }
        let mut cells = 0; let mut offset = line_start;
        for cluster in graphemes(line.trim_end_matches('\r')) {
            let width = if cluster == "\t" { 4 - cells % 4 } else { usize::from(display_width(cluster)) };
            if cells >= left + usize::from(rect.width) { break; }
            if cells >= left {
                let selected = offset < selection_end && offset + cluster.len() > selection_start;
                let fg = theme.role(if selected { Role::ActiveForeground } else { Role::Foreground });
                let background = if selected { theme.role(Role::ActiveBase) } else { bg };
                let text = if cluster == "\t" { " ".repeat(width) } else { cluster.to_string() };
                let clip = Rect::new(rect.x, rect.y + (row - top) as u16, rect.width, 1);
                buffer.put_str(Pos { x: rect.x + (cells - left) as u16, y: clip.y }, &text, fg, background, 0, clip);
            }
            cells += width; offset += cluster.len();
        }
    }
}

#[cfg(test)]
#[path = "../../🧪️tests/🦀️.rs"]
mod tests;
