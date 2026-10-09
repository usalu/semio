//! ⌨️ How input becomes behavior: keys are resolved through the declarative keymap into action ids,
//! pointer and paste events are routed through the engine's signals, and every action is one
//! method. A terminal that owns the keyboard sees every key except the prefix key.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⚙️preferences/⌨️keymap/🔣️.json

use super::launcher::{Input, LauncherState, Outcome};
use super::windows::{Body, ContextMenu, Dashboard, Effect, Kind, LinkState, Place};
use crate::preferences::keymap::{KeyName, KeySpec, Scope};
use crate::preferences::Change;
use ui_tui::tui::engine::Tui;
use ui_tui::tui::event::{mods, Event, Key, KeyEvent, KeypadKey, MouseButton, MouseEvent, MouseKind};
use ui_tui::tui::layout::{reorder_stack_tab, resize_split, resize_window, zoom_window};
use ui_tui::tui::scene::NodeId;
use ui_tui::tui::widget::{WidgetSignal, WidgetState};

const GROW_STEP: f64 = 0.1;
const PREFIX_PATIENCE: std::time::Duration = std::time::Duration::from_secs(3);
const ARMED_FIRST: &[&str] = &["show-help", "new-task", "show-tasks", "split-right", "split-down", "next-window", "zoom", "close-window", "detach"];

/// 🎬️ Every action a keymap binding can name; the ids are data, this is the one place that knows what they do.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Action { NewTask, ShowTasks, ShowSettings, ShowHelp, ShowHiddenTasks, SearchOutput, RefreshCommands, CancelDiscovery, RestartTask, StopTask, KillTask, CopySelection, ToggleInput, ToggleAppearance, ToggleLanguage, NextWindow, PreviousWindow, SplitDown, SplitRight, GrowWindow, ShrinkWindow, Zoom, CloseWindow, Detach, Shutdown, CancelPrefix, SendPrefix }

impl Action {
    /// 🪪️ The id a keymap binding names this action by.
    pub fn id(self) -> &'static str {
        match self {
            Self::NewTask => "new-task", Self::ShowTasks => "show-tasks", Self::ShowSettings => "show-settings", Self::ShowHelp => "show-help", Self::ShowHiddenTasks => "show-hidden-tasks", Self::SearchOutput => "search-output",
            Self::RefreshCommands => "refresh-commands", Self::CancelDiscovery => "cancel-discovery", Self::RestartTask => "restart-task", Self::StopTask => "stop-task", Self::KillTask => "kill-task",
            Self::CopySelection => "copy-selection", Self::ToggleInput => "toggle-input", Self::ToggleAppearance => "toggle-appearance", Self::ToggleLanguage => "toggle-language",
            Self::NextWindow => "next-window", Self::PreviousWindow => "previous-window", Self::SplitDown => "split-down", Self::SplitRight => "split-right", Self::GrowWindow => "grow-window",
            Self::ShrinkWindow => "shrink-window", Self::Zoom => "zoom", Self::CloseWindow => "close-window", Self::Detach => "detach", Self::Shutdown => "shutdown", Self::CancelPrefix => "cancel-prefix",
            Self::SendPrefix => "send-prefix",
        }
    }

    pub fn parse(id: &str) -> Option<Self> {
        Some(match id {
            "new-task" => Self::NewTask, "show-tasks" => Self::ShowTasks, "show-settings" => Self::ShowSettings, "show-help" => Self::ShowHelp, "show-hidden-tasks" => Self::ShowHiddenTasks, "search-output" => Self::SearchOutput,
            "refresh-commands" => Self::RefreshCommands, "cancel-discovery" => Self::CancelDiscovery, "restart-task" => Self::RestartTask, "stop-task" => Self::StopTask, "kill-task" => Self::KillTask,
            "copy-selection" => Self::CopySelection, "toggle-input" => Self::ToggleInput, "toggle-appearance" => Self::ToggleAppearance, "toggle-language" => Self::ToggleLanguage,
            "next-window" => Self::NextWindow, "previous-window" => Self::PreviousWindow, "split-down" => Self::SplitDown, "split-right" => Self::SplitRight, "grow-window" => Self::GrowWindow,
            "shrink-window" => Self::ShrinkWindow, "zoom" => Self::Zoom, "close-window" => Self::CloseWindow, "detach" => Self::Detach, "shutdown" => Self::Shutdown, "cancel-prefix" => Self::CancelPrefix,
            "send-prefix" => Self::SendPrefix,
            _ => return None,
        })
    }
}

/// 🧩️ How a window takes input.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Class { Launcher, Pane, Terminal }

fn class_of(body: &Body) -> Class {
    match body { Body::Launcher(_) => Class::Launcher, Body::Overview | Body::Settings | Body::Help => Class::Pane, Body::Output { .. } => Class::Terminal }
}

/// 🚦️ Whether the dashboard goes on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Flow { Continue, Quit }

/// 📏️ The leading hints that fit whole beside a status that keeps up to a third of the row (the footer draws ` key ` and `label ` per hint).
pub(super) fn fit_hints(hints: Vec<ui_tui::tui::chrome::KeyHint>, width: u16) -> Vec<ui_tui::tui::chrome::KeyHint> {
    let mut room = i32::from(width) - i32::from(width / 3) - 1;
    hints.into_iter().take_while(|hint| {
        room -= i32::from(ui_tui::tui::text::display_width(&hint.key)) + i32::from(ui_tui::tui::text::display_width(&hint.label)) + 3;
        room >= 0
    }).collect()
}

// #region 🔖️Keys
/// 🔑️ The keymap's spelling of a key event (a keypad key is spelled like the main block key it types); `None` for a combination no binding can name.
pub(super) fn spec_of(event: &KeyEvent) -> Option<KeySpec> {
    let name = match event.key {
        Key::Char(c) => KeyName::Char(c),
        Key::Enter => KeyName::Named("enter".into()),
        Key::Esc => KeyName::Named("esc".into()),
        Key::Tab => KeyName::Named("tab".into()),
        Key::BackTab => KeyName::Named("backtab".into()),
        Key::Backspace => KeyName::Named("backspace".into()),
        Key::Delete => KeyName::Named("delete".into()),
        Key::Insert => KeyName::Named("insert".into()),
        Key::Up => KeyName::Named("up".into()),
        Key::Down => KeyName::Named("down".into()),
        Key::Left => KeyName::Named("left".into()),
        Key::Right => KeyName::Named("right".into()),
        Key::Home => KeyName::Named("home".into()),
        Key::End => KeyName::Named("end".into()),
        Key::PageUp => KeyName::Named("pageup".into()),
        Key::PageDown => KeyName::Named("pagedown".into()),
        Key::F(number) => KeyName::Named(format!("f{number}")),
        Key::Keypad(KeypadKey::Enter) => KeyName::Named("enter".into()),
        Key::Keypad(key) => KeyName::Char(key.character()),
    };
    let held = |flag: u8| event.mods & flag != 0;
    let spec = KeySpec { name, ctrl: held(mods::CTRL), alt: held(mods::ALT), shift: held(mods::SHIFT) };
    spec.clone().canonical().or_else(|_| KeySpec { shift: false, ..spec }.canonical()).ok()
}

/// 🎹️ The key event a keymap key stands for, used to send the prefix key itself to a terminal.
pub(super) fn event_of(spec: &KeySpec) -> KeyEvent {
    let key = match &spec.name {
        KeyName::Char(c) => Key::Char(*c),
        KeyName::Named(name) => match name.as_str() {
            "enter" => Key::Enter, "esc" => Key::Esc, "tab" => Key::Tab, "backtab" => Key::BackTab, "backspace" => Key::Backspace, "delete" => Key::Delete, "insert" => Key::Insert,
            "up" => Key::Up, "down" => Key::Down, "left" => Key::Left, "right" => Key::Right, "home" => Key::Home, "end" => Key::End, "pageup" => Key::PageUp, "pagedown" => Key::PageDown,
            other => Key::F(other.trim_start_matches('f').parse().unwrap_or(1)),
        },
    };
    KeyEvent { key, mods: if spec.ctrl { mods::CTRL } else { 0 } | if spec.alt { mods::ALT } else { 0 } | if spec.shift { mods::SHIFT } else { 0 } }
}

fn input_of(action: &str) -> Option<Input> {
    Some(match action {
        "up" => Input::Up, "down" => Input::Down, "left" => Input::Left, "right" => Input::Right, "page-up" => Input::PageUp, "page-down" => Input::PageDown, "first" => Input::First, "last" => Input::Last, "delete-word" => Input::DeleteWord, "activate" => Input::Activate, "clear" => Input::Clear, "back" => Input::Back,
        _ => return None,
    })
}
// #endregion 🔖️Keys

// #region 🔖️Events
impl Dashboard {
    /// 🎮️ Routes one event; the only entry point of input, with or without a real terminal.
    pub fn handle(&mut self, tui: &mut Tui, event: &Event) -> Flow {
        match event {
            Event::Key(key) => self.on_key(tui, key),
            Event::Mouse(mouse) => { self.on_mouse(tui, event, mouse); Flow::Continue }
            Event::Paste(text) => { self.on_paste(tui, event, text); Flow::Continue }
            Event::Resize(_) | Event::FocusGained | Event::FocusLost => { let signals = tui.dispatch(event); self.on_signals(tui, signals); Flow::Continue }
            Event::Wake => Flow::Continue,
        }
    }

    fn on_key(&mut self, tui: &mut Tui, key: &KeyEvent) -> Flow {
        if !tui.overlays().is_empty() {
            let signals = tui.dispatch(&Event::Key(*key));
            self.on_signals(tui, signals);
            return if self.quit_requested { Flow::Quit } else { Flow::Continue };
        }
        let Some(spec) = spec_of(key) else { return self.pass_key(tui, key) };
        if let Some(chord) = self.escape_then_prefix(&spec) {
            if self.terminal_has_keyboard(tui) { let _ = self.pass_key(tui, &KeyEvent { key: Key::Esc, mods: 0 }); } else { self.view_key(tui, &KeyEvent { key: Key::Esc, mods: 0 }, &KeySpec { name: KeyName::Named("esc".into()), ctrl: false, alt: false, shift: false }); }
            return self.arm(chord);
        }
        if self.armed {
            self.armed = false;
            self.armed_at = None;
            self.notice = None;
            return match self.keymap.resolve(Scope::Prefix, &spec).map(str::to_string) {
                Some(id) => self.run_action(tui, &id).unwrap_or(Flow::Continue),
                None => { self.notice = Some(self.text().key_unbound.fill(&[("keys", &spec.label())]).into_string()); Flow::Continue }
            };
        }
        if self.keymap.is_prefix(&spec) { return self.arm(spec); }
        if self.terminal_has_keyboard(tui) { return self.pass_key(tui, key); }
        if let Some(flow) = self.keymap.resolve(Scope::Window, &spec).map(str::to_string).and_then(|id| self.run_action(tui, &id)) { return flow; }
        self.view_key(tui, key, &spec);
        Flow::Continue
    }

    fn arm(&mut self, _prefix: KeySpec) -> Flow {
        self.armed = true;
        self.armed_at = Some(std::time::Instant::now());
        self.notice = Some(self.text().key_armed.as_str().into());
        Flow::Continue
    }

    /// 🧲️ A terminal reports Esc followed at once by the prefix key as one Alt chord; when no binding owns that chord it was two key presses.
    fn escape_then_prefix(&self, spec: &KeySpec) -> Option<KeySpec> {
        if !spec.alt || self.keymap.prefix().alt { return None; }
        let plain = KeySpec { alt: false, ..spec.clone() };
        let owned = Scope::ALL.into_iter().any(|scope| self.keymap.resolve(scope, spec).is_some());
        (self.keymap.is_prefix(&plain) && !owned).then_some(plain)
    }

    /// ⏱️ Disarms the prefix when the next key did not come in time; true when the screen must change.
    pub fn expire_prefix(&mut self, now: std::time::Instant) -> bool {
        if self.armed && self.armed_at.is_some_and(|at| now.duration_since(at) >= PREFIX_PATIENCE) {
            self.armed = false;
            self.armed_at = None;
            self.notice = None;
            return true;
        }
        false
    }

    /// ⏳️ How long until the armed prefix gives up, if it is armed.
    pub fn prefix_wait(&self, now: std::time::Instant) -> Option<std::time::Duration> {
        self.armed_at.filter(|_| self.armed).map(|at| PREFIX_PATIENCE.saturating_sub(now.duration_since(at)))
    }

    fn pass_key(&mut self, tui: &mut Tui, key: &KeyEvent) -> Flow {
        let index = self.focused_index(tui);
        if let Some(node) = self.terminal_node(index) {
            let signal = tui.scene.node_mut(node).widget().and_then(|widget| widget.on_key(key));
            if let Some(signal) = signal { self.on_signals(tui, vec![(node, signal)]); }
        }
        Flow::Continue
    }

    fn view_key(&mut self, tui: &mut Tui, key: &KeyEvent, spec: &KeySpec) {
        let index = self.focused_index(tui);
        if matches!(&self.windows[index].body, Body::Launcher(launcher) if launcher.editing()) && matches!(key.key, Key::Char(_) | Key::Backspace | Key::Delete | Key::Left | Key::Right | Key::Home | Key::End | Key::Keypad(_)) {
            let event = Event::Key(event_of(spec));
            let signals = tui.dispatch(&event);
            self.on_signals(tui, signals);
            return;
        }
        let action = self.keymap.resolve(Scope::View, spec).map(str::to_string);
        let typed = match key.key { Key::Char(c) if key.mods & (mods::CTRL | mods::ALT) == 0 => Some(Input::Char(c)), Key::Backspace => Some(Input::Backspace), _ => None };
        let input = action.as_deref().and_then(input_of).or(typed);
        let Some(input) = input else { return };
        match class_of(&self.windows[index].body) {
            Class::Launcher => self.launcher_input(tui, index, input),
            Class::Pane => self.pane_input(tui, index, input),
            Class::Terminal => { if input == Input::Activate { self.input_mode = true; } else { let _ = self.pass_key(tui, key); } }
        }
    }

    /// 🚀️ Applies an input to the launcher of a window and carries out what it asks.
    pub fn launcher_input(&mut self, tui: &mut Tui, index: usize, input: Input) {
        let outcome = self.with_launcher(tui, index, |launcher, widget| launcher.apply(widget, input));
        self.carry_out(tui, index, outcome);
    }

    /// 🌳️ Runs something on the launcher model of a window together with its tree widget.
    fn with_launcher(&mut self, tui: &mut Tui, index: usize, act: impl FnOnce(&mut LauncherState, &mut WidgetState) -> Outcome) -> Outcome {
        let Some(tree) = self.windows[index].tree else { return Outcome::Idle };
        let mut node = tui.scene.node_mut(tree);
        match (node.widget(), &mut self.windows[index].body) {
            (Some(widget), Body::Launcher(launcher)) => act(launcher, widget),
            _ => Outcome::Idle,
        }
    }

    fn pane_input(&mut self, tui: &mut Tui, index: usize, input: Input) {
        let page = self.windows[index].pane.page.max(1) as isize;
        let overview = matches!(self.windows[index].body, Body::Overview);
        match input {
            Input::Up => self.windows[index].pane.select(-1),
            Input::Down => self.windows[index].pane.select(1),
            Input::PageUp => self.windows[index].pane.select(-page),
            Input::PageDown => self.windows[index].pane.select(page),
            Input::First => self.windows[index].pane.select(isize::MIN / 2),
            Input::Last => self.windows[index].pane.select(isize::MAX / 2),
            Input::Activate => self.activate_row(tui, index),
            Input::Back => { if !overview { self.set_body(tui, index, Kind::Overview, None); } }
            Input::Char(c) if overview => { self.set_body(tui, index, Kind::Launcher, None); self.launcher_input(tui, index, Input::Char(c)); }
            _ => {}
        }
        if self.windows.get(index).is_some_and(|window| class_of(&window.body) == Class::Pane) { self.refresh_pane(tui, index); }
    }

    fn on_paste(&mut self, tui: &mut Tui, event: &Event, text: &str) {
        let index = self.focused_index(tui);
        if self.terminal_has_keyboard(tui) || matches!(&self.windows[index].body, Body::Launcher(launcher) if launcher.editing()) { let signals = tui.dispatch(event); self.on_signals(tui, signals); return; }
        if matches!(self.windows[index].body, Body::Launcher(_)) { for c in text.chars().filter(|c| !c.is_control()) { self.launcher_input(tui, index, Input::Char(c)); } }
    }

    fn on_mouse(&mut self, tui: &mut Tui, event: &Event, mouse: &MouseEvent) {
        let before = self.focused_index(tui);
        let signals = tui.dispatch(event);
        let target = tui.scene.hit(mouse.pos).and_then(|node| self.window_of(tui, node));
        if let (MouseKind::Down(_), Some(index)) = (mouse.kind, target) {
            if index != before || tui.focus().is_none() { tui.set_focus(Some(self.windows[index].focus())); self.input_mode = self.windows[index].is_output(); }
        }
        if let Some(index) = target {
            let rect = tui.scene.rect(self.windows[index].list);
            let row = usize::from(mouse.pos.y.saturating_sub(rect.y));
            let inside = rect.contains(mouse.pos);
            match (mouse.kind, class_of(&self.windows[index].body)) {
                (MouseKind::Down(MouseButton::Left), Class::Launcher) if inside => {
                    let outcome = self.with_launcher(tui, index, |launcher, widget| launcher.click(widget, row));
                    self.carry_out(tui, index, outcome);
                }
                (MouseKind::Down(MouseButton::Left), Class::Pane) if inside => {
                    let line = self.windows[index].pane.offset + row;
                    if line < self.windows[index].pane.rows.len() {
                        let again = self.windows[index].pane.selected == line;
                        self.windows[index].pane.selected = line;
                        if again { self.activate_row(tui, index); }
                        if self.windows.get(index).is_some_and(|window| class_of(&window.body) == Class::Pane) { self.refresh_pane(tui, index); }
                    }
                }
                (MouseKind::Scroll { dy, .. }, Class::Pane) if inside => { self.windows[index].pane.select(isize::from(dy.signum()) * 3); self.refresh_pane(tui, index); }
                _ => {}
            }
        }
        self.on_signals(tui, signals);
    }

    fn carry_out(&mut self, tui: &mut Tui, index: usize, outcome: Outcome) {
        match outcome {
            Outcome::Idle => {}
            Outcome::Leave => self.set_body(tui, index, Kind::Overview, None),
            Outcome::Start(request) => { if let Err(error) = self.start(tui, index, &request) { self.notice = Some(self.text().form_problem.fill(&[("problem", &error)]).into_string()); } }
        }
    }

    /// 📣️ Acts on what widgets and window chrome reported.
    pub fn on_signals(&mut self, tui: &mut Tui, signals: Vec<(NodeId, WidgetSignal)>) {
        for (node, signal) in signals {
            let window = self.window_of(tui, node);
            match (signal, window) {
                (WidgetSignal::ValueChanged(value), Some(index)) if self.windows[index].input == Some(node) => {
                    if let Body::Launcher(launcher) = &mut self.windows[index].body { launcher.edited(value); }
                }
                (WidgetSignal::TerminalInput(data), Some(index)) => {
                    if let Some(session_id) = self.windows[index].session().map(|session| session.session_id.clone()) { self.send_input(&session_id, data); }
                }
                (WidgetSignal::Copy(text), _) => self.effects.push(Effect::Copy(text)),
                (WidgetSignal::ContextMenu { pos, item }, window) => self.open_context_menu(tui, pos, item, window),
                (WidgetSignal::Activated(row), _) if self.menu.as_ref().is_some_and(|menu| menu.node == node) => {
                    let Some(menu) = self.menu.take() else { continue };
                    if let Some(Some(action)) = menu.actions.get(row).copied() {
                        if menu.window < self.windows.len() { self.focus_window(tui, menu.window); }
                        if self.perform(tui, action) == Flow::Quit { self.quit_requested = true; }
                    }
                }
                (WidgetSignal::Dismissed, _) if self.menu.as_ref().is_some_and(|menu| menu.node == node) => self.menu = None,
                (WidgetSignal::WindowClose(tab), Some(index)) => { if let Some(target) = self.tab_window(index, tab) { if self.close_window(tui, target) { self.quit_requested = true; } } }
                (WidgetSignal::WindowMaximize(tab), Some(index)) => { if let Some(target) = self.tab_window(index, tab) { self.toggle_zoom(tui, target); } }
                (WidgetSignal::WindowNewTab(tab), Some(index)) => {
                    let host = self.tab_window(index, tab).map_or_else(|| self.windows[index].id.clone(), |target| self.windows[target].id.clone());
                    let created = self.create_window(tui, Kind::Launcher, Place::Stack(&host));
                    self.focus_window(tui, created);
                }
                (WidgetSignal::WindowTabActivated(tab), Some(index)) => { if let Some(target) = self.tab_window(index, tab) { self.focus_window(tui, target); } }
                (WidgetSignal::Activated(item), Some(index)) if self.windows[index].tree == Some(node) => {
                    if let Body::Launcher(launcher) = &mut self.windows[index].body { launcher.activated(item); }
                    tui.set_focus(Some(self.windows[index].focus()));
                }
                (WidgetSignal::WindowFocus, Some(index)) => self.focus_window(tui, index),
                (WidgetSignal::TabMoved { from, to }, Some(index)) => { let id = self.windows[index].id.clone(); if reorder_stack_tab(&mut self.layout, &id, from, to) { self.remount(tui); } }
                (WidgetSignal::SplitterDragged { path, delta }, _) => { let area = tui.scene.rect(self.shell.canvas); if resize_split(&mut self.layout, area, &path, delta) { self.remount(tui); } }
                _ => {}
            }
        }
    }

    /// 📋️ Opens the context menu of a window, a tab chip or a terminal; every row names its keys from the keymap.
    fn open_context_menu(&mut self, tui: &mut Tui, pos: ui_tui::tui::geometry::Pos, item: Option<usize>, window: Option<usize>) {
        use ui_tui::tui::menu::MenuItem;
        let Some(window) = window else { return };
        let target = item.and_then(|tab| self.tab_window(window, tab)).unwrap_or(window);
        let output = self.windows[target].is_output();
        let rows: Vec<Option<Action>> = if output {
            vec![Some(Action::CopySelection), Some(Action::ToggleInput), None, Some(Action::RestartTask), Some(Action::StopTask), Some(Action::KillTask), None, Some(Action::SplitRight), Some(Action::SplitDown), Some(Action::Zoom), Some(Action::CloseWindow)]
        } else {
            vec![Some(Action::NewTask), None, Some(Action::SplitRight), Some(Action::SplitDown), Some(Action::Zoom), Some(Action::CloseWindow)]
        };
        let text = self.text();
        let items: Vec<MenuItem> = rows.iter().map(|row| match row {
            None => MenuItem::separator(),
            Some(action) => {
                let scope = if self.keymap.keys_label(Scope::Prefix, action.id()).is_some() { Scope::Prefix } else { Scope::Window };
                MenuItem::new(text.action(action.id())).with_shortcut(self.keymap.keys_label(scope, action.id()).unwrap_or_default())
            }
        }).collect();
        let node = tui.open_menu(pos, items);
        self.menu = Some(ContextMenu { node, window: target, actions: rows });
    }

    fn tab_window(&self, index: usize, tab: usize) -> Option<usize> {
        let id = super::windows::stack_of(&self.layout, &self.windows[index].id).get(tab)?.clone();
        self.window_index(&id)
    }

    fn toggle_zoom(&mut self, tui: &mut Tui, index: usize) {
        if self.layout.zoomed.is_some() { zoom_window(&mut self.layout, None); } else { zoom_window(&mut self.layout, Some(&self.windows[index].id.clone())); }
        self.remount(tui);
    }
}
// #endregion 🔖️Events

// #region 🔖️Actions
impl Dashboard {
    fn show(&mut self, tui: &mut Tui, kind: Kind) {
        let existing = self.windows.iter().position(|window| matches!((&window.body, kind), (Body::Overview, Kind::Overview) | (Body::Settings, Kind::Settings) | (Body::Help, Kind::Help)));
        let index = existing.unwrap_or_else(|| self.create_window(tui, kind, Place::Preference));
        self.focus_window(tui, index);
    }

    /// ⚙️ Runs the action a keymap binding names; `None` when this build knows no such action.
    pub fn run_action(&mut self, tui: &mut Tui, id: &str) -> Option<Flow> { Action::parse(id).map(|action| self.perform(tui, action)) }

    /// 🕹️ Performs one action on the focused window.
    pub fn perform(&mut self, tui: &mut Tui, action: Action) -> Flow {
        let index = self.focused_index(tui);
        match action {
            Action::NewTask => { let created = self.create_window(tui, Kind::Launcher, Place::Preference); self.focus_window(tui, created); }
            Action::ShowTasks => self.show(tui, Kind::Overview),
            Action::ShowSettings => self.show(tui, Kind::Settings),
            Action::ShowHelp => self.show(tui, Kind::Help),
            Action::ShowHiddenTasks => {
                self.hidden.clear();
                let mut sessions: Vec<_> = self.sessions.values().cloned().collect();
                sessions.sort_by_key(|session| (session.started_ms, session.session_id.clone()));
                let newest = sessions.last().map(|session| session.session_id.clone());
                for session in sessions { self.update_session(tui, session); }
                if let Some(target) = newest.and_then(|id| self.windows.iter().position(|window| window.session().is_some_and(|session| session.session_id == id))) { self.focus_window(tui, target); }
                self.restore_focus = true;
                self.restore_sessions();
            }
            Action::SearchOutput => {
                if let Some(node) = self.terminal_node(index) {
                    let mut found = tui.scene.node_mut(node);
                    if let Some(WidgetState::Terminal(terminal)) = found.widget() { terminal.begin_search(); }
                }
            }
            Action::RefreshCommands => self.refresh_inventory(),
            Action::CancelDiscovery => self.cancel_inventory(),
            Action::RestartTask => self.control_session(tui, index, "restart-task"),
            Action::StopTask => self.control_session(tui, index, "stop-task"),
            Action::KillTask => self.control_session(tui, index, "kill-task"),
            Action::CopySelection => {
                let signal = self.terminal_node(index).and_then(|node| match tui.scene.node_mut(node).widget() { Some(WidgetState::Terminal(terminal)) => terminal.copy_selection(), _ => None });
                if let Some(WidgetSignal::Copy(text)) = signal { self.effects.push(Effect::Copy(text)); }
            }
            Action::ToggleInput => { if self.windows[index].is_output() { self.input_mode = !self.input_mode; tui.set_focus(Some(self.windows[index].focus())); } }
            Action::ToggleAppearance => self.queue_preference(Change { appearance: Some(if self.light { "dark" } else { "light" }.into()), ..Default::default() }),
            Action::ToggleLanguage => self.queue_preference(Change { language: Some(if self.preferences.language == "en" { "de" } else { "en" }.into()), ..Default::default() }),
            Action::NextWindow => self.cycle_window(tui, 1),
            Action::PreviousWindow => self.cycle_window(tui, -1),
            Action::SplitDown | Action::SplitRight => { let created = self.create_window(tui, Kind::Launcher, Place::Split(if action == Action::SplitDown { "column" } else { "row" })); self.focus_window(tui, created); }
            Action::GrowWindow | Action::ShrinkWindow => { let window = self.windows[index].id.clone(); resize_window(&mut self.layout, &window, if action == Action::GrowWindow { GROW_STEP } else { -GROW_STEP }); self.remount(tui); }
            Action::Zoom => self.toggle_zoom(tui, index),
            Action::CloseWindow => { if self.close_window(tui, index) { return Flow::Quit; } }
            Action::Detach => return Flow::Quit,
            Action::Shutdown => self.begin_shutdown(),
            Action::CancelPrefix => {}
            Action::SendPrefix => { let key = event_of(self.keymap.prefix()); if self.terminal_has_keyboard(tui) { let _ = self.pass_key(tui, &key); } }
        }
        Flow::Continue
    }
}
// #endregion 🔖️Actions

// #region 🔖️Frame
impl Dashboard {
    /// 🖼️ Brings everything that depends on layout up to date before painting: sizes, list slices,
    /// launcher screens, focus marks, the footer.
    pub fn frame(&mut self, tui: &mut Tui) {
        let (focused, input) = (self.focused_index(tui), self.input_mode);
        for (index, window) in self.windows.iter().enumerate() {
            if !window.is_output() { continue; }
            if let Some(WidgetState::Terminal(terminal)) = tui.scene.node_mut(window.list).widget() { terminal.set_passthrough(input && index == focused); }
        }
        for message in self.sync_sizes(tui) { let _ = self.send(&message); }
        self.refresh_panes(tui);
        for index in 0..self.windows.len() { self.paint_launcher(tui, index); }
        self.follow_stage(tui, focused);
        self.sync_chrome(tui);
        self.sync_footer(tui);
    }

    fn paint_launcher(&mut self, tui: &mut Tui, index: usize) {
        let window = &mut self.windows[index];
        let (Body::Launcher(launcher), Some(tree)) = (&mut window.body, window.tree) else { return };
        let browse = launcher.stage() == super::launcher::Stage::Browse;
        let editor_slot = launcher.editor_slot();
        let (list, input, caption) = (window.list, window.input, window.caption);
        for (node, shown) in [(Some(tree), browse), (Some(list), !browse), (input, !browse)] {
            if let Some(node) = node { if tui.scene.node(node).visible != shown { tui.scene.node_mut(node).set_visible(shown); } }
        }
        let caption_text = if browse {
            match &tui.scene.node(tree).content { ui_tui::tui::scene::NodeContent::Widget(WidgetState::Tree(state)) => launcher.caption(state), _ => String::new() }
        } else {
            launcher.set_page(usize::from(tui.scene.rect(list).height).max(1));
            let screen = launcher.screen();
            if let Some(WidgetState::List(state)) = tui.scene.node_mut(list).widget() { state.marks = vec![false; screen.items.len()]; state.items = screen.items; state.selected = screen.selected; state.offset = 0; }
            if let Some(node) = input { if let Some(WidgetState::Input(state)) = tui.scene.node_mut(node).widget() { if window.editor_slot != editor_slot || state.value != screen.input { state.set_value(screen.input); } state.placeholder = screen.placeholder; } }
            screen.caption
        };
        window.editor_slot = editor_slot;
        if let Some(node) = caption {
            let changed = matches!(&tui.scene.node(node).content, ui_tui::tui::scene::NodeContent::Widget(WidgetState::Label(label)) if label.text != caption_text);
            if changed { if let Some(WidgetState::Label(label)) = tui.scene.node_mut(node).widget() { label.text = caption_text; } }
        }
    }

    /// 🎯️ Keeps the engine's focus on the node of the focused window that takes the keys now, as the launcher moves between its stages.
    fn follow_stage(&mut self, tui: &mut Tui, index: usize) {
        if !tui.overlays().is_empty() { return; }
        let Some(window) = self.windows.get(index) else { return };
        let want = window.focus();
        if tui.focus() != Some(want) && tui.focus().is_none_or(|node| self.window_of(tui, node) == Some(index)) { tui.set_focus(Some(want)); }
    }

    fn sync_footer(&mut self, tui: &mut Tui) {
        let hints = self.footer_hints(tui);
        let status = self.footer_status(tui);
        let node = self.shell.footer;
        let same = matches!(&tui.scene.node(node).content, ui_tui::tui::scene::NodeContent::Chrome(ui_tui::tui::chrome::ChromeState::Footer(footer)) if footer.status == status && footer.hints.len() == hints.len() && footer.hints.iter().zip(&hints).all(|(a, b)| a.key == b.key && a.label == b.label));
        if same { return; }
        if let Some(ui_tui::tui::chrome::ChromeState::Footer(footer)) = tui.scene.node_mut(node).chrome() { footer.hints = hints; footer.status = status; }
    }

    /// 💡️ The footer hints, generated from the keymap and the text catalogue; nothing is spelled twice. Only
    /// hints that fit whole next to the status are returned, most important first, so none is ever cut.
    pub fn footer_hints(&self, tui: &Tui) -> Vec<ui_tui::tui::chrome::KeyHint> {
        use ui_tui::tui::chrome::KeyHint;
        let text = self.text();
        let hint = |scope: Scope, action: &str| self.keymap.keys_label(scope, action).map(|keys| KeyHint { key: keys, label: text.action(action).to_string() });
        let wanted: Vec<KeyHint> = if self.armed {
            let strip = |hint: KeyHint| KeyHint { key: hint.key.strip_prefix(&format!("{} ", self.keymap.prefix().label())).unwrap_or(&hint.key).to_string(), label: hint.label };
            ARMED_FIRST.iter().filter_map(|action| hint(Scope::Prefix, action).map(strip)).chain(hint(Scope::Prefix, "cancel-prefix").map(strip)).collect()
        } else {
            let controls = KeyHint { key: self.keymap.prefix().label(), label: text.hint_controls.as_str().into() };
            if self.terminal_has_keyboard(tui) { vec![controls] } else {
                std::iter::once(controls).chain([(Scope::View, "activate"), (Scope::View, "back"), (Scope::View, "up"), (Scope::View, "down"), (Scope::Window, "next-window")].into_iter().filter_map(|(scope, action)| hint(scope, action).map(|found| if action == "activate" { KeyHint { label: text.hint_activate.as_str().to_string(), ..found } } else { found }))).collect()
            }
        };
        fit_hints(wanted, tui.size().width)
    }

    fn footer_status(&self, tui: &Tui) -> String {
        let text = self.text();
        let link = match &self.state {
            LinkState::Connecting => text.state_connecting.as_str().to_string(),
            LinkState::Connected => text.state_connected.as_str().to_string(),
            LinkState::Reconnecting => text.state_reconnecting.as_str().to_string(),
            LinkState::Stopped => text.state_daemon_stopped.as_str().to_string(),
            LinkState::Failed(message) => message.clone(),
        };
        let discovery = match (&self.inventory, &self.registry) {
            (Some(job), _) => text.state_discovering_progress.fill(&[("seconds", &job.started.elapsed().as_secs().to_string()), ("cancel", &self.keymap.keys_label(Scope::Prefix, "cancel-discovery").unwrap_or_default())]).into_string(),
            (None, Some(registry)) => text.state_commands.fill(&[("count", &registry.entries().len().to_string())]).into_string(),
            (None, None) => String::new(),
        };
        let phase = self.phase.map(|phase| match phase { crate::inventory::Phase::Known => text.phase_known, crate::inventory::Phase::Sources => text.phase_sources, crate::inventory::Phase::Walk => text.phase_walk, crate::inventory::Phase::Complete => text.phase_complete }.as_str().to_string()).filter(|_| self.inventory.is_some());
        [self.notice.clone(), self.focus_title(tui), Some(text.state_running.fill(&[("count", &self.running().to_string())]).into_string()), Some(link), Some(discovery).filter(|value| !value.is_empty()), phase].into_iter().flatten().collect::<Vec<_>>().join(" · ")
    }
}
// #endregion 🔖️Frame
