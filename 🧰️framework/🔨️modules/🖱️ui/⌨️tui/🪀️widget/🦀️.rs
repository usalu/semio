use crate::tui::cell::CellBuffer;
use crate::tui::chip::paint_chip;
use crate::tui::divider::paint_divider;
use crate::tui::event::{KeyEvent, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Rect, Size};
use crate::tui::input::{input_on_key, paint_input};
use crate::tui::label::paint_label;
use crate::tui::list::{list_on_key, paint_list};
use crate::tui::log::{log_on_key, paint_log};
use crate::tui::select::{paint_select, select_on_key};
use crate::tui::table::{paint_table, table_on_key};
use crate::tui::tabs::{paint_tabs, tabs_on_key};
use crate::tui::text::display_width;
use crate::tui::theme::{Role, Theme};
use crate::tui::wizard::{paint_wizard, wizard_hit, wizard_on_key};
use std::collections::VecDeque;

/// ??? A widget- or window-chrome-level result of handling input, surfaced to the app.
#[derive(Clone, Debug, PartialEq)]
pub enum WidgetSignal {
    Activated(usize),
    SelectionChanged(usize),
    ValueChanged(String),
    Toggled(bool),
    TabChanged(usize),
    NavigateBack,
    Hovered(Option<usize>),
    ContextMenu { pos: crate::tui::geometry::Pos, item: Option<usize> },
    Copy(String),
    OpenUrl(String),
    /// Bytes the embedded terminal encoded for its child; the host forwards them to the PTY.
    TerminalInput(Vec<u8>),
    WindowClose(usize),
    WindowMaximize,
    WindowNewTab,
    WindowTabActivated(usize),
    WindowFocus,
    TabMoved { from: usize, to: usize },
    SplitterDragged { path: Vec<usize>, delta: i16 },
}

/// 🖋️ How the terminal draws its text cursor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CursorShape {
    Block,
    Underline,
    Bar,
}

/// 📍️ Where and how the terminal cursor shows while a widget holds focus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CursorSpec {
    pub pos: crate::tui::geometry::Pos,
    pub shape: CursorShape,
    pub blink: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Align {
    Left,
    Center,
    Right,
}

/// ??? Static or dynamic single-line text.
pub struct LabelState {
    pub text: String,
    pub align: Align,
    pub role: Role,
}

/// ??? A scrollable, selectable, optionally multi-marked list.
pub struct ListState {
    pub items: Vec<String>,
    pub selected: usize,
    pub offset: usize,
    pub marks: Vec<bool>,
}

impl ListState {
    pub fn new(items: Vec<String>) -> Self {
        let marks = vec![false; items.len()];
        Self { items, selected: 0, offset: 0, marks }
    }
}

/// ??? A cycler for an `All | Individual(value)` style option pick.
pub struct SelectState {
    pub label: String,
    pub options: Vec<String>,
    pub index: usize,
}

pub struct TabsState {
    pub tabs: Vec<String>,
    pub active: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LogScroll {
    Follow,
    At(usize),
}

/// ??? A bounded scrollback log view.
pub struct LogState {
    lines: VecDeque<String>,
    pub capacity: usize,
    pub scroll: LogScroll,
}

impl LogState {
    pub fn new(capacity: usize) -> Self {
        Self { lines: VecDeque::with_capacity(capacity), capacity, scroll: LogScroll::Follow }
    }

    pub fn push(&mut self, line: &str) {
        if self.lines.len() >= self.capacity {
            self.lines.pop_front();
        }
        self.lines.push_back(line.to_string());
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.scroll = LogScroll::Follow;
    }

    pub fn lines(&self) -> &VecDeque<String> {
        &self.lines
    }
}

pub struct InputState {
    pub value: String,
    pub cursor: usize,
    pub placeholder: String,
}

#[derive(Default)]
pub struct DividerState {
    pub label: Option<String>,
}

pub struct ChipState {
    pub label: String,
    pub on: bool,
}

/// ??? Filterable option list for stepped command building.
pub struct WizardState {
    pub steps: Vec<(String, String)>,
    pub options: Vec<String>,
    pub selected: usize,
    pub offset: usize,
    pub filter: String,
}

impl WizardState {
    pub fn new(options: Vec<String>) -> Self {
        Self { steps: Vec::new(), options, selected: 0, offset: 0, filter: String::new() }
    }

    /// 🔎️ Visible option identities under a Unicode-aware all-words filter.
    pub fn visible_indices(&self) -> Vec<usize> {
        let filter = self.filter.to_lowercase();
        let tokens: Vec<_> = filter.split_whitespace().collect();
        self.options.iter().enumerate().filter(|(_, option)| { let text = option.to_lowercase(); tokens.iter().all(|token| text.contains(token)) }).map(|(index, _)| index).collect()
    }
}

//#region ???Table
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableAlign {
    Left,
    Right,
}

/// ??? One table column; `width == 0` means "flex" ? split the remaining space evenly.
pub struct TableColumn {
    pub label: String,
    pub width: u16,
    pub align: TableAlign,
}

impl TableColumn {
    pub fn new(label: impl Into<String>, width: u16, align: TableAlign) -> Self {
        Self { label: label.into(), width, align }
    }
}

/// ??? One row, flat in display order; `level` and `has_children` express the tree ? a row is
/// hidden whenever a preceding, still-nesting ancestor has `expanded == false`.
pub struct TableRow {
    pub id: String,
    pub cells: Vec<String>,
    pub level: u16,
    pub has_children: bool,
    pub expanded: bool,
}

impl TableRow {
    pub fn parent(id: impl Into<String>, cells: Vec<String>) -> Self {
        Self { id: id.into(), cells, level: 0, has_children: true, expanded: true }
    }

    pub fn child(id: impl Into<String>, cells: Vec<String>, level: u16) -> Self {
        Self { id: id.into(), cells, level, has_children: false, expanded: true }
    }
}

/// ??? A semio-styled table: bold muted header with a hairline underline, hairline row
/// separators, no vertical rules, no striping ? mirrors `ui/js/react`'s `Table` and
/// `print/tex/???semio-table.sty`. Tree rows are plain indented rows in the same table.
pub struct TableState {
    pub columns: Vec<TableColumn>,
    pub rows: Vec<TableRow>,
    pub selected: usize,
}

impl TableState {
    pub fn new(columns: Vec<TableColumn>, rows: Vec<TableRow>) -> Self {
        Self { columns, rows, selected: 0 }
    }

    /// ??? Row indices in display order, skipping any row nested under a collapsed ancestor.
    pub fn visible_indices(&self) -> Vec<usize> {
        let mut out = Vec::new();
        let mut collapsed_from: Option<u16> = None;
        for (i, row) in self.rows.iter().enumerate() {
            if let Some(level) = collapsed_from {
                if row.level > level {
                    continue;
                }
                collapsed_from = None;
            }
            out.push(i);
            if row.has_children && !row.expanded {
                collapsed_from = Some(row.level);
            }
        }
        out
    }
}
//#endregion ???Table

//#region ???Terminal
/// ??? Inclusive cell-range selection inside a terminal pane (viewport coordinates).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TerminalSelection {
    pub start: crate::tui::geometry::Pos,
    pub end: crate::tui::geometry::Pos,
}

/// ??? VT pane state: scrollback viewport, search, selection, follow/pin, key passthrough.
pub struct TerminalState {
    pub screen: crate::tui::vt::VtScreen,
    pub scrollback_offset: usize,
    pub follow: bool,
    pub pinned: bool,
    pub search: String,
    pub search_active: bool,
    pub selection: Option<TerminalSelection>,
}

impl TerminalState {
    /// ?? Blank terminal pane of `size` with `scrollback_cap` (0 ? VT default).
    pub fn new(size: Size, scrollback_cap: usize) -> Self {
        Self { screen: crate::tui::vt::VtScreen::new(size, scrollback_cap), scrollback_offset: 0, follow: true, pinned: false, search: String::new(), search_active: false, selection: None }
    }

    /// ??? Feeds PTY bytes; keeps the viewport glued when following and not pinned.
    pub fn feed(&mut self, bytes: &[u8]) {
        self.screen.feed(bytes);
        if self.follow && !self.pinned {
            self.scrollback_offset = 0;
        }
    }

    /// ?? Resizes the underlying VT screen.
    pub fn resize(&mut self, size: Size) {
        self.screen.resize(size);
    }

    fn max_offset(&self) -> usize {
        self.screen.scrollback_len()
    }

    fn scroll_by(&mut self, delta: i32, page: u16) {
        let step = if delta == 0 { 0 } else { i32::from(page.max(1)) * delta.signum() };
        let next = (self.scrollback_offset as i32 + step).clamp(0, self.max_offset() as i32) as usize;
        self.scrollback_offset = next;
        self.follow = next == 0 && !self.pinned;
    }

    /// ?? Extracts selected text from the active buffer (simple row-major slice).
    pub fn selected_text(&self) -> Option<String> {
        let sel = self.selection?;
        let (a, b) = if (sel.start.y, sel.start.x) <= (sel.end.y, sel.end.x) { (sel.start, sel.end) } else { (sel.end, sel.start) };
        let mut out = String::new();
        for y in a.y..=b.y {
            let x0 = if y == a.y { a.x } else { 0 };
            let x1 = if y == b.y { b.x } else { self.screen.size.width.saturating_sub(1) };
            if y > a.y {
                out.push('\n');
            }
            for x in x0..=x1 {
                if let Some(cell) = self.screen.cell_at(x, y) {
                    if cell.ch != '\0' {
                        out.push(cell.ch);
                    }
                }
            }
        }
        Some(out)
    }
}

/// ⌨️ Interim key encoder for the terminal's child: the fixed table the dashboard used to own.
fn key_to_pty_bytes(ev: &KeyEvent) -> Option<Vec<u8>> {
    use crate::tui::event::{mods, Key};
    match ev.key {
        Key::Char(c) if ev.mods == 0 => Some(c.to_string().into_bytes()),
        Key::Char(c) if ev.mods & mods::CTRL != 0 && c.is_ascii_lowercase() => Some(vec![(c as u8) & 0x1f]),
        Key::Enter => Some(vec![b'\r']),
        Key::Tab => Some(vec![b'\t']),
        Key::Backspace => Some(vec![0x7f]),
        Key::Esc => Some(vec![0x1b]),
        Key::Up => Some(b"\x1b[A".to_vec()),
        Key::Down => Some(b"\x1b[B".to_vec()),
        Key::Right => Some(b"\x1b[C".to_vec()),
        Key::Left => Some(b"\x1b[D".to_vec()),
        Key::Home => Some(b"\x1b[H".to_vec()),
        Key::End => Some(b"\x1b[F".to_vec()),
        Key::PageUp => Some(b"\x1b[5~".to_vec()),
        Key::PageDown => Some(b"\x1b[6~".to_vec()),
        _ => None,
    }
}

fn terminal_on_key(term: &mut TerminalState, ev: &KeyEvent) -> Option<WidgetSignal> {
    use crate::tui::event::{mods, Key};
    if term.search_active {
        match ev.key {
            Key::Esc => {
                term.search_active = false;
                term.search.clear();
                None
            }
            Key::Backspace => {
                term.search.pop();
                Some(WidgetSignal::ValueChanged(term.search.clone()))
            }
            Key::Enter => Some(WidgetSignal::ValueChanged(term.search.clone())),
            Key::Char(c) if ev.mods == 0 || ev.mods == mods::SHIFT => {
                term.search.push(c);
                Some(WidgetSignal::ValueChanged(term.search.clone()))
            }
            _ => None,
        }
    } else {
        match (ev.key, ev.mods) {
            (Key::PageUp, _) => {
                term.pinned = false;
                term.scroll_by(1, term.screen.size.height);
                None
            }
            (Key::PageDown, _) => {
                term.scroll_by(-1, term.screen.size.height);
                None
            }
            (Key::Home, m) if m & mods::CTRL != 0 => {
                term.scrollback_offset = term.max_offset();
                term.follow = false;
                None
            }
            (Key::End, _) => {
                term.scrollback_offset = 0;
                term.follow = true;
                None
            }
            (Key::Char('/'), 0) => {
                term.search_active = true;
                term.search.clear();
                None
            }
            (Key::Char('p'), m) if m == mods::CTRL => {
                term.pinned = !term.pinned;
                if !term.pinned && term.follow {
                    term.scrollback_offset = 0;
                }
                Some(WidgetSignal::Toggled(term.pinned))
            }
            _ => key_to_pty_bytes(ev).map(WidgetSignal::TerminalInput),
        }
    }
}

fn paint_terminal(term: &TerminalState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    let bg = theme.surface(crate::tui::theme::Surface::Window);
    let fg = theme.role(Role::Foreground);
    buf.fill_rect(rect, crate::tui::cell::Cell::blank(fg, bg));
    if rect.width == 0 || rect.height == 0 {
        return;
    }
    let (body, search_row) = if term.search_active && rect.height > 1 { (Rect::new(rect.x, rect.y, rect.width, rect.height - 1), Some(rect.y + rect.height - 1)) } else { (rect, None) };
    term.screen.blit_to(buf, body, term.scrollback_offset);
    if let Some(y) = search_row {
        let label = format!("/{}", term.search);
        let muted = theme.role(Role::MutedForeground);
        buf.fill_rect(Rect::new(rect.x, y, rect.width, 1), crate::tui::cell::Cell::blank(muted, bg));
        buf.put_str(crate::tui::geometry::Pos { x: rect.x, y }, &label, muted, bg, 0, Rect::new(rect.x, y, rect.width, 1));
    }
}
//#endregion ???Terminal

/// ??? The concrete state of any core widget.
pub enum WidgetState {
    Label(LabelState),
    List(ListState),
    Select(SelectState),
    Tabs(TabsState),
    Log(LogState),
    Input(InputState),
    Divider(DividerState),
    Chip(ChipState),
    Table(TableState),
    Terminal(TerminalState),
    Wizard(WizardState),
}

impl WidgetState {
    pub fn preferred_size(&self) -> Size {
        match self {
            WidgetState::Label(l) => Size { width: display_width(&l.text), height: 1 },
            WidgetState::Select(s) => {
                let text = format!("{} \u{2039} {} \u{203a}", s.label, s.options.get(s.index).map(String::as_str).unwrap_or(""));
                Size { width: display_width(&text), height: 1 }
            }
            WidgetState::Chip(c) => Size { width: display_width(&c.label) + 2, height: 1 },
            WidgetState::Divider(_) => Size { width: 1, height: 1 },
            WidgetState::Terminal(t) => t.screen.size,
            WidgetState::Wizard(_) => Size { width: 1, height: 1 },
            _ => Size { width: 0, height: 0 },
        }
    }

    /// ?? Handles one key press, returning a signal when it changes visible state.
    pub fn on_key(&mut self, ev: &KeyEvent) -> Option<WidgetSignal> {
        match self {
            WidgetState::List(l) => list_on_key(l, ev),
            WidgetState::Select(s) => select_on_key(s, ev),
            WidgetState::Tabs(t) => tabs_on_key(t, ev),
            WidgetState::Input(i) => input_on_key(i, ev),
            WidgetState::Table(t) => table_on_key(t, ev),
            WidgetState::Wizard(w) => wizard_on_key(w, ev),
            WidgetState::Log(log) => {
                log_on_key(log, ev);
                None
            }
            WidgetState::Terminal(t) => terminal_on_key(t, ev),
            _ => None,
        }
    }

    /// 🖱️ Handles one pointer event over `rect`, returning a signal when it changes visible state.
    pub fn on_mouse(&mut self, rect: Rect, event: &MouseEvent) -> Option<WidgetSignal> {
        match (self, event.kind) {
            (WidgetState::Wizard(w), MouseKind::Down(MouseButton::Left)) => wizard_hit(w, rect, event.pos),
            _ => None,
        }
    }

    /// 📋️ Handles one bracketed paste.
    pub fn on_paste(&mut self, text: &str) -> Option<WidgetSignal> {
        let _ = text;
        None
    }

    /// 👆️ Moves the hover to `pos` inside `rect` or clears it; true when the widget must repaint.
    pub fn set_hover(&mut self, rect: Rect, pos: Option<crate::tui::geometry::Pos>) -> bool {
        let _ = (rect, pos);
        false
    }

    /// ✏️ Where the terminal cursor belongs while this widget, laid out at `rect`, holds focus.
    pub fn cursor(&self, rect: Rect) -> Option<CursorSpec> {
        let _ = rect;
        None
    }

    /// 🎯 Whether the widget takes focus and input.
    pub fn interactive(&self) -> bool {
        true
    }

    /// ⏱️ Advances time-driven state to `now_ms`; true when the widget must repaint.
    pub fn tick(&mut self, now_ms: u64) -> bool {
        let _ = now_ms;
        false
    }

    /// ??? Paints this widget's content into `rect` of `buf`.
    pub fn paint(&self, theme: &Theme, rect: Rect, buf: &mut CellBuffer, focused: bool) {
        match self {
            WidgetState::Label(l) => paint_label(l, theme, rect, buf),
            WidgetState::List(l) => paint_list(l, theme, rect, buf, focused),
            WidgetState::Select(s) => paint_select(s, theme, rect, buf, focused),
            WidgetState::Tabs(t) => paint_tabs(t, theme, rect, buf),
            WidgetState::Log(log) => paint_log(log, theme, rect, buf),
            WidgetState::Input(i) => paint_input(i, theme, rect, buf, focused),
            WidgetState::Divider(d) => paint_divider(d, theme, rect, buf),
            WidgetState::Chip(c) => paint_chip(c, theme, rect, buf),
            WidgetState::Table(t) => paint_table(t, theme, rect, buf, focused),
            WidgetState::Terminal(t) => paint_terminal(t, theme, rect, buf),
            WidgetState::Wizard(w) => paint_wizard(w, theme, rect, buf, focused),
        }
    }
}
