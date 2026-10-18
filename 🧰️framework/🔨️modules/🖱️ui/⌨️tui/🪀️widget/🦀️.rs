use semio_framework_ui_tui_core::cell::CellBuffer;
use crate::chip::paint_chip;
use crate::dialog::{dialog_on_key, dialog_on_mouse, dialog_set_hover, paint_dialog, DialogState};
use crate::divider::paint_divider;
use semio_framework_ui_tui_core::event::{KeyEvent, MouseEvent};
use semio_framework_ui_tui_core::geometry::{Rect, Size};
use crate::input::{input_cursor, input_on_key, input_on_mouse, input_on_paste, paint_input};
use crate::label::paint_label;
use crate::list::{list_on_key, list_on_mouse, list_set_hover, list_tick, paint_list};
use crate::log::{log_on_key, log_on_mouse, paint_log};
use crate::menu::{menu_on_key, menu_on_mouse, menu_set_hover, paint_menu, MenuState};
use crate::palette::{palette_cursor, palette_on_key, palette_on_mouse, palette_set_hover, paint_palette, PaletteState};
use crate::progress::{paint_progress, progress_tick};
use crate::scrollable::{paint_scrollable, scrollable_on_key, scrollable_on_mouse};
use crate::select::{paint_select, select_on_key, select_on_mouse};
use semio_framework_ui_tui_core::signal::{CursorSpec, WidgetSignal};
use crate::table::{paint_table, table_on_key, table_on_mouse, table_set_hover};
use crate::tabs::{paint_tabs, tabs_on_key, tabs_on_mouse, tabs_set_hover};
use semio_framework_ui_tui_core::text::display_width;
use semio_framework_ui_tui_core::theme::{Role, Theme};
use crate::toggle::{paint_toggle, toggle_on_key, toggle_on_mouse, toggle_set_hover};
use crate::tooltip::{paint_tooltip, TooltipState};
use crate::tree::{paint_tree, tree_cursor, tree_on_key, tree_on_mouse, tree_on_paste, tree_set_hover};
use semio_framework_ui_tui_vt::pane::TerminalState;
use crate::wizard::{paint_wizard, wizard_cursor, wizard_on_key, wizard_on_mouse, wizard_on_paste, wizard_set_hover};

pub use crate::input::InputState;
pub use crate::list::ListState;
pub use crate::log::{LogScroll, LogState};
pub use crate::progress::ProgressState;
pub use crate::scrollable::ScrollableState;
pub use crate::table::{TableAlign, TableColumn, TableRow, TableState};
pub use crate::toggle::ToggleState;
pub use crate::tree::{TreeItem, TreeState};
pub use crate::wizard::WizardState;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Align {
    Left,
    Center,
    Right,
}

/// 🏷️ Static or dynamic single-line text.
pub struct LabelState {
    pub text: String,
    pub align: Align,
    pub role: Role,
}

/// 🔁️ A cycler for an `All | Individual(value)` style option pick.
pub struct SelectState {
    pub label: String,
    pub options: Vec<String>,
    pub index: usize,
}

pub struct TabsState {
    pub tabs: Vec<String>,
    pub active: usize,
    pub hover: Option<usize>,
}

impl TabsState {
    pub fn new(tabs: Vec<String>, active: usize) -> Self {
        Self { tabs, active, hover: None }
    }
}

#[derive(Default)]
pub struct DividerState {
    pub label: Option<String>,
}

pub struct ChipState {
    pub label: String,
    pub on: bool,
}



/// 🧩️ The concrete state of any core widget.
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
    Tree(TreeState),
    Scrollable(ScrollableState),
    Progress(ProgressState),
    Toggle(ToggleState),
    Menu(MenuState),
    Dialog(DialogState),
    Palette(PaletteState),
    Tooltip(TooltipState),
}

impl WidgetState {
    pub fn preferred_size(&self) -> Size {
        match self {
            WidgetState::Label(l) => Size { width: display_width(&l.text), height: 1 },
            WidgetState::Select(s) => {
                let text = format!("{} \u{2039} {} \u{203a}", s.label, s.options.get(s.index).map_or("", String::as_str));
                Size { width: display_width(&text), height: 1 }
            }
            WidgetState::Chip(c) => Size { width: display_width(&c.label) + 2, height: 1 },
            WidgetState::Divider(_) => Size { width: 1, height: 1 },
            WidgetState::Terminal(t) => t.screen.size,
            WidgetState::Wizard(_) => Size { width: 1, height: 1 },
            WidgetState::Progress(p) => Size { width: p.preferred_width(), height: 1 },
            WidgetState::Toggle(t) => Size { width: t.preferred_width(), height: 1 },
            _ => Size { width: 0, height: 0 },
        }
    }

    /// 🥞️ Whether this widget is an overlay surface the engine places above the scene.
    pub fn is_overlay(&self) -> bool {
        matches!(self, WidgetState::Menu(_) | WidgetState::Dialog(_) | WidgetState::Palette(_) | WidgetState::Tooltip(_))
    }

    /// 📐️ The size an overlay wants inside `viewport` (zero for ordinary widgets).
    pub fn overlay_size(&self, viewport: Size) -> Size {
        match self {
            WidgetState::Menu(m) => crate::menu::menu_size(m, viewport),
            WidgetState::Dialog(d) => crate::dialog::dialog_size(d, viewport),
            WidgetState::Palette(p) => crate::palette::palette_size(p, viewport),
            WidgetState::Tooltip(t) => crate::tooltip::tooltip_size(t, viewport),
            _ => Size { width: 0, height: 0 },
        }
    }

    /// ⇥ Whether Tab belongs to the widget (shell completion, a dialog button cycle) instead of moving focus.
    pub fn claims_tab(&self) -> bool {
        matches!(self, WidgetState::Terminal(_) | WidgetState::Menu(_) | WidgetState::Dialog(_) | WidgetState::Palette(_))
    }

    /// ⌨️ Handles one key press, returning a signal when it changes visible state.
    pub fn on_key(&mut self, ev: &KeyEvent) -> Option<WidgetSignal> {
        if let WidgetState::Terminal(t) = self {
            return t.on_key(ev);
        }
        let text = ev.text_equivalent();
        let ev = &text;
        match self {
            WidgetState::List(l) => list_on_key(l, ev),
            WidgetState::Select(s) => select_on_key(s, ev),
            WidgetState::Tabs(t) => tabs_on_key(t, ev),
            WidgetState::Input(i) => input_on_key(i, ev),
            WidgetState::Table(t) => table_on_key(t, ev),
            WidgetState::Wizard(w) => wizard_on_key(w, ev),
            WidgetState::Tree(t) => tree_on_key(t, ev),
            WidgetState::Scrollable(s) => scrollable_on_key(s, ev),
            WidgetState::Toggle(t) => toggle_on_key(t, ev),
            WidgetState::Log(log) => {
                log_on_key(log, ev);
                None
            }
            WidgetState::Menu(m) => menu_on_key(m, ev),
            WidgetState::Dialog(d) => dialog_on_key(d, ev),
            WidgetState::Palette(p) => palette_on_key(p, ev),
            _ => None,
        }
    }

    /// 🖱️ Handles one pointer event over `rect`, returning a signal when it changes visible state.
    pub fn on_mouse(&mut self, rect: Rect, event: &MouseEvent) -> Option<WidgetSignal> {
        match (self, event.kind) {
            (WidgetState::List(l), _) => list_on_mouse(l, rect, event),
            (WidgetState::Wizard(w), _) => wizard_on_mouse(w, rect, event),
            (WidgetState::Table(t), _) => table_on_mouse(t, rect, event),
            (WidgetState::Input(i), _) => input_on_mouse(i, rect, event),
            (WidgetState::Select(s), _) => select_on_mouse(s, rect, event),
            (WidgetState::Tree(t), _) => tree_on_mouse(t, rect, event),
            (WidgetState::Scrollable(s), _) => scrollable_on_mouse(s, rect, event),
            (WidgetState::Toggle(t), _) => toggle_on_mouse(t, rect, event),
            (WidgetState::Log(log), _) => {
                log_on_mouse(log, rect, event);
                None
            }
            (WidgetState::Terminal(t), _) => t.on_mouse(rect, event),
            (WidgetState::Tabs(t), _) => tabs_on_mouse(t, rect, event),
            (WidgetState::Menu(m), _) => menu_on_mouse(m, rect, event),
            (WidgetState::Dialog(d), _) => dialog_on_mouse(d, rect, event),
            (WidgetState::Palette(p), _) => palette_on_mouse(p, rect, event),
            _ => None,
        }
    }

    /// 📋️ Handles one bracketed paste.
    pub fn on_paste(&mut self, text: &str) -> Option<WidgetSignal> {
        if let WidgetState::Terminal(t) = self {
            return t.on_paste(text);
        }
        let text = text.replace(['\r', '\n'], " ");
        match self {
            WidgetState::Input(i) => input_on_paste(i, &text),
            WidgetState::Wizard(w) => wizard_on_paste(w, &text),
            WidgetState::Tree(t) => tree_on_paste(t, &text),
            WidgetState::Palette(p) => {
                for c in text.chars() {
                    palette_on_key(p, &KeyEvent { key: semio_framework_ui_tui_core::event::Key::Char(c), mods: 0 });
                }
                Some(WidgetSignal::ValueChanged(p.query.clone()))
            }
            _ => None,
        }
    }

    /// 🎯️ Reports that the widget gained or lost focus; the terminal tells its child when the child asked for it.
    pub fn on_focus(&mut self, gained: bool) -> Option<WidgetSignal> {
        match self {
            WidgetState::Terminal(t) => t.on_focus(gained),
            _ => None,
        }
    }

    /// 👆️ Moves the hover to `pos` inside `rect` or clears it; true when the widget must repaint.
    pub fn set_hover(&mut self, rect: Rect, pos: Option<semio_framework_ui_tui_core::geometry::Pos>) -> bool {
        match self {
            WidgetState::Tabs(t) => tabs_set_hover(t, rect, pos),
            WidgetState::List(l) => list_set_hover(l, rect, pos),
            WidgetState::Wizard(w) => wizard_set_hover(w, rect, pos),
            WidgetState::Table(t) => table_set_hover(t, rect, pos),
            WidgetState::Tree(t) => tree_set_hover(t, rect, pos),
            WidgetState::Toggle(t) => toggle_set_hover(t, rect, pos),
            WidgetState::Menu(m) => menu_set_hover(m, rect, pos),
            WidgetState::Dialog(d) => dialog_set_hover(d, rect, pos),
            WidgetState::Palette(p) => palette_set_hover(p, rect, pos),
            _ => false,
        }
    }

    /// ✏️ Where the terminal cursor belongs while this widget, laid out at `rect`, holds focus.
    pub fn cursor(&self, rect: Rect) -> Option<CursorSpec> {
        match self {
            WidgetState::Input(i) => input_cursor(i, rect),
            WidgetState::Wizard(w) => wizard_cursor(w, rect),
            WidgetState::Tree(t) => tree_cursor(t, rect),
            WidgetState::Terminal(t) => t.cursor(rect),
            WidgetState::Palette(p) => palette_cursor(p, rect),
            _ => None,
        }
    }

    /// 🔘 Whether the widget takes focus and input.
    pub fn interactive(&self) -> bool {
        !matches!(self, WidgetState::Label(_) | WidgetState::Divider(_) | WidgetState::Chip(_) | WidgetState::Tooltip(_) | WidgetState::Progress(_))
    }

    /// ⏱️ Advances time-driven state to `now_ms`; true when the widget must repaint.
    pub fn tick(&mut self, now_ms: u64) -> bool {
        match self {
            WidgetState::Progress(p) => progress_tick(p, now_ms),
            WidgetState::List(l) => list_tick(l, now_ms),
            _ => false,
        }
    }

    /// 🖌️ Paints this widget's content into `rect` of `buf`.
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
            WidgetState::Terminal(t) => t.paint(theme, rect, buf),
            WidgetState::Wizard(w) => paint_wizard(w, theme, rect, buf, focused),
            WidgetState::Tree(t) => paint_tree(t, theme, rect, buf, focused),
            WidgetState::Scrollable(s) => paint_scrollable(s, theme, rect, buf),
            WidgetState::Progress(p) => paint_progress(p, theme, rect, buf),
            WidgetState::Toggle(t) => paint_toggle(t, theme, rect, buf, focused),
            WidgetState::Menu(m) => paint_menu(m, theme, rect, buf),
            WidgetState::Dialog(d) => paint_dialog(d, theme, rect, buf),
            WidgetState::Palette(p) => paint_palette(p, theme, rect, buf),
            WidgetState::Tooltip(t) => paint_tooltip(t, theme, rect, buf),
        }
    }
}
