//! 🖥️ Native developer task controls, a searchable launcher and managed terminal views.

use crate::command_tree::{CommandLeaf, CommandSpec, RepoAction};
use crate::daemon::client::{Connection, Message};
use crate::ipc::{ClientMsg, ServerMsg, SessionCommand, SessionInfo, SessionStatus};
use std::path::{Path, PathBuf};
use ui_styling::appearance::AppearanceName;
use ui_tui::tui::backend::{NativeTerminal, TerminalBackend};
use ui_tui::tui::chrome::{shell, ChromeState, FooterState, KeyHint, NavItem, NavbarState, WindowState};
use ui_tui::tui::engine::Tui;
use ui_tui::tui::event::{mods, Event, Key, KeyEvent};
use ui_tui::tui::geometry::Size;
use ui_tui::tui::layout::{activate_stack_tab, create_default_layout, push_window_to_stack, remove_window, split_window, zoom_window, WindowLayout, WindowLayoutWindowNode};
use ui_tui::tui::layout::{Constraint, Dimension, Direction};
use ui_tui::tui::scene::{Node, NodeContent, NodeId};
use ui_tui::tui::theme::Theme;
use ui_tui::tui::widget::{TerminalState, WidgetSignal, WidgetState, WizardState};

// #region 🔖️Window
enum WindowBody {
    Overview { widget: NodeId },
    Launcher { widget: NodeId },
    Settings { widget: NodeId },
    Output { terminal: NodeId, session: Option<SessionInfo> },
}

struct DashboardWindow {
    id: String,
    chrome: NodeId,
    body: WindowBody,
    focus: NodeId,
}

enum LeaderMode {
    Idle,
    Armed,
}

#[derive(Clone, Copy)]
enum Locale { English, German }
// #endregion 🔖️Window

// #region 🔖️Keys
fn key_to_pty_bytes(ev: &KeyEvent) -> Option<Vec<u8>> {
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
// #endregion 🔖️Keys

// #region 🔖️Dashboard
struct Dashboard {
    root: PathBuf,
    commands: Vec<(String, CommandLeaf)>,
    inventory: Option<crate::inventory::Job>,
    inventory_status: String,
    last_progress: u64,
    preferences: crate::preferences::Preferences,
    preference_path: PathBuf,
    shared_preferences: bool,
    saving_preferences: Option<std::sync::mpsc::Receiver<(crate::preferences::Change, std::io::Result<u64>)>>,
    layout: WindowLayout,
    shell: ui_tui::tui::chrome::Shell,
    windows: Vec<DashboardWindow>,
    next_serial: u32,
    focused: String,
    leader: LeaderMode,
    terminal_input: bool,
    connection: Option<Connection>,
    pending_starts: std::collections::VecDeque<ClientMsg>,
    connecting: Option<std::sync::mpsc::Receiver<std::io::Result<Connection>>>,
    connection_status: String,
    next_reconnect: std::time::Instant,
    restoring: bool,
    sessions: std::collections::BTreeMap<String, SessionInfo>,
    hidden: std::collections::HashSet<String>,
    locale: Locale,
    light: bool,
}

impl Dashboard {
    fn text(&self, en: &str, de: &str) -> String {
        match self.locale { Locale::English => en.into(), Locale::German => de.into() }
    }

    fn window_order(&self) -> Vec<String> {
        self.windows.iter().map(|w| w.id.clone()).collect()
    }

    fn sync_chrome_focus(&mut self, tui: &mut Tui) {
        for w in &self.windows {
            if let Some(chrome) = tui.scene.node_mut(w.chrome).chrome() {
                if let ChromeState::Window(ws) = chrome {
                    ws.focused = w.id == self.focused;
                }
            }
        }
    }

    fn focused_window(&self) -> Option<&DashboardWindow> {
        self.windows.iter().find(|w| w.id == self.focused)
    }

    fn refresh_view_for(&mut self, tui: &mut Tui, idx: usize) {
        let (widget, options) = match &self.windows[idx].body {
            WindowBody::Launcher { widget } => (*widget, self.commands.iter().map(|(label, _)| label.clone()).collect()),
            WindowBody::Overview { widget } => {
                let mut options = vec![self.text("New task — type to find a command", "Neue Aufgabe — Befehl suchen"), self.text("Settings", "Einstellungen"), self.text("Refresh commands", "Befehle aktualisieren"), self.text("Cancel discovery", "Suche abbrechen")];
                options.extend(self.sessions.values().map(|session| format!("[{}{}] {} {}", match session.status { SessionStatus::Running => self.text("running", "läuft"), SessionStatus::Stopping => self.text("stopping", "stoppt"), SessionStatus::Exited => self.text("exited", "beendet"), SessionStatus::Failed => self.text("failed", "fehlgeschlagen") }, session.code.map(|code| format!(" · exit {code}")).unwrap_or_default(), session.command.cmd, session.command.args.join(" "))));
                (*widget, options)
            }
            WindowBody::Settings { widget } => (*widget, vec![format!("{}: {}", self.text("Language", "Sprache"), self.preferences.language), format!("{}: {}", self.text("Appearance", "Darstellung"), self.preferences.appearance), format!("{}: {}", self.text("Terminology", "Begriffe"), self.preferences.terminology), format!("{}: {}", self.text("Preferred renderer", "Bevorzugter Renderer"), self.preferences.renderer), format!("{}: {}", self.text("Output layout", "Ausgabelayout"), self.preferences.layout), format!("{}: {}", self.text("Save scope", "Speicherbereich"), if self.shared_preferences { self.text("workspace shared", "Arbeitsbereich gemeinsam") } else { self.text("local only", "nur lokal") }), self.text("Back to tasks", "Zurück zu Aufgaben")]),
            _ => return,
        };
        let title = match self.windows[idx].body { WindowBody::Overview { .. } => self.text("Tasks", "Aufgaben"), WindowBody::Settings { .. } => self.text("Settings", "Einstellungen"), _ => self.text("Commands", "Befehle") };
        if let Some(ChromeState::Window(chrome)) = tui.scene.node_mut(self.windows[idx].chrome).chrome() { chrome.title = title; }
        if let Some(WidgetState::Wizard(state)) = tui.scene.node_mut(widget).widget() {
            let selected = state.visible_indices().get(state.selected).and_then(|index| state.options.get(*index)).cloned();
            state.options = options;
            state.selected = selected.and_then(|label| state.visible_indices().iter().position(|index| state.options[*index] == label)).unwrap_or(0);
            state.offset = 0;
        }
    }

    fn refresh_views(&mut self, tui: &mut Tui) { for index in 0..self.windows.len() { self.refresh_view_for(tui, index); } }

    fn sort_commands(&mut self) {
        let renderer = format!(" / {} / ", self.preferences.renderer);
        self.commands.sort_by_key(|(label, _)| !label.contains(&renderer));
    }

    fn place_window(&mut self, id: &str) {
        match self.preferences.layout.as_str() {
            "columns" | "rows" => { let direction = if self.preferences.layout == "columns" { "row" } else { "column" }; let _ = split_window(&mut self.layout, &self.focused, direction, id, None); }
            _ => { push_window_to_stack(&mut self.layout, &self.focused, WindowLayoutWindowNode { window_kind_id: id.into(), title: None, corner: None }); }
        }
    }

    fn queue_preference(&mut self, change: crate::preferences::Change) {
        if self.saving_preferences.is_some() { self.connection_status = self.text("Saving preferences; try again shortly", "Einstellungen werden gespeichert; gleich erneut versuchen"); return; }
        let path = if self.shared_preferences { crate::preferences::shared_path(&self.root) } else { self.preference_path.clone() };
        let (sender, receiver) = std::sync::mpsc::channel();
        std::thread::spawn(move || { let result = crate::preferences::append(&path, &change); let _ = sender.send((change, result)); });
        self.saving_preferences = Some(receiver);
        self.connection_status = self.text("Saving preferences", "Einstellungen speichern");
    }

    fn poll_inventory(&mut self, tui: &mut Tui) -> bool {
        let mut changed = false;
        if let Some((change, result)) = self.saving_preferences.as_ref().and_then(|receiver| receiver.try_recv().ok()) {
            self.saving_preferences = None;
            match result {
                Ok(_) => {
                    self.preferences.apply(&change);
                    self.locale = if self.preferences.language == "de" { Locale::German } else { Locale::English };
                    self.light = self.preferences.appearance == "light";
                    tui.set_appearance(if self.light { AppearanceName::Light } else { AppearanceName::Dark });
                    self.connection_status = self.text("Preferences saved", "Einstellungen gespeichert");
                    self.sort_commands();
                    self.refresh_views(tui);
                }
                Err(error) => self.connection_status = error.to_string(),
            }
            changed = true;
        }
        let updates: Vec<_> = self.inventory.as_ref().map(|job| job.receiver.try_iter().collect()).unwrap_or_default();
        for update in updates {
            match update {
                crate::inventory::Update::Ready(commands, status) => {
                    self.commands = commands;
                    self.sort_commands();
                    self.inventory_status = status; self.refresh_views(tui);
                }
                crate::inventory::Update::Finished => { self.inventory = None; }
                crate::inventory::Update::Failed(error) => self.inventory_status = error,
            }
            changed = true;
        }
        if let Some(job) = &self.inventory {
            let elapsed = job.started.elapsed().as_secs();
            if elapsed != self.last_progress { self.last_progress = elapsed; changed = true; }
        }
        changed
    }

    fn refresh_inventory(&mut self) {
        self.inventory = Some(crate::inventory::start(self.root.clone(), true));
        self.inventory_status = self.text("Discovering commands", "Befehle suchen");
    }

    fn cancel_inventory(&mut self) {
        self.inventory = None;
        self.inventory_status = self.text("Discovery cancelled; available commands retained", "Suche abgebrochen; verfügbare Befehle bleiben");
    }

    fn replace_view(&mut self, tui: &mut Tui, index: usize, mode: &str) {
        let id = self.windows[index].id.clone(); let chrome = self.windows[index].chrome;
        match self.windows[index].body { WindowBody::Overview { widget } | WindowBody::Launcher { widget } | WindowBody::Settings { widget } => tui.scene.remove(widget), WindowBody::Output { .. } => return }
        self.windows[index] = self.attach_view(tui, chrome, &id, mode);
        self.focused = id; self.terminal_input = false;
        tui.set_focus(Some(self.windows[index].focus)); self.remount(tui);
    }

    fn show_home(&mut self, tui: &mut Tui) {
        if let Some(window) = self.windows.iter().find(|window| matches!(window.body, WindowBody::Overview { .. })) {
            self.focused = window.id.clone(); self.terminal_input = false; tui.set_focus(Some(window.focus));
            activate_stack_tab(&mut self.layout, &self.focused); self.remount(tui);
            return;
        }
        let id = format!("w{}", self.next_serial); self.next_serial += 1;
        push_window_to_stack(&mut self.layout, &self.focused, WindowLayoutWindowNode { window_kind_id: id.clone(), title: None, corner: None });
        let window = self.add_launcher_window(tui, id.clone(), "Tasks"); self.windows.push(window);
        self.replace_view(tui, self.windows.len() - 1, "overview");
    }

    fn open_output(&mut self, tui: &mut Tui, win_id: &str, title: String) -> (NodeId, Size) {
        let win = self.windows.iter_mut().find(|w| w.id == win_id).expect("window");
        if let WindowBody::Launcher { widget } | WindowBody::Overview { widget } | WindowBody::Settings { widget } = &win.body {
            tui.scene.remove(*widget);
        }
        let chrome = win.chrome;
        let _ = tui.render_full();
        let inner = tui.scene.rect(chrome);
        let term_size = Size { width: inner.width.saturating_sub(4).max(1), height: inner.height.saturating_sub(4).max(1) };
        let term_id = tui.scene.add(win.chrome, Node::new(NodeContent::Widget(WidgetState::Terminal(TerminalState::new(term_size, 8000)))));
        tui.scene.node_mut(term_id).set_constraint(Constraint { width: Dimension::Weight(1), height: Dimension::Weight(1), ..Default::default() });
        win.body = WindowBody::Output { terminal: term_id, session: None };
        win.focus = term_id;
        tui.set_focus(Some(term_id));
        self.terminal_input = true;
        if let Some(ChromeState::Window(ws)) = tui.scene.node_mut(win.chrome).chrome() {
            ws.title = title;
        }
        (term_id, term_size)
    }

    fn feed(tui: &mut Tui, terminal: NodeId, text: &str) {
        if let Some(WidgetState::Terminal(t)) = tui.scene.node_mut(terminal).widget() {
            t.feed(text.replace("\r\n", "\n").replace('\n', "\r\n").as_bytes());
        }
    }

    fn spawn_output(&mut self, tui: &mut Tui, win_id: &str, spec: CommandSpec) {
        let command = SessionCommand { cmd: spec.cmd, args: spec.args, cwd: spec.cwd.display().to_string(), env: spec.env, cols: 80, rows: 24 };
        if let Some(session) = self.sessions.values().find(|session| matches!(session.status, SessionStatus::Running | SessionStatus::Stopping) && session.command.same_task(&command)).cloned() {
            self.hidden.remove(&session.session_id);
            self.update_session(tui, session.clone());
            if let Some(window) = self.windows.iter().find(|window| matches!(&window.body, WindowBody::Output { session: Some(info), .. } if info.session_id == session.session_id)) {
                self.focused = window.id.clone();
                tui.set_focus(Some(window.focus));
                self.terminal_input = true;
                activate_stack_tab(&mut self.layout, &self.focused);
                self.remount(tui);
                self.restore_sessions();
            }
            return;
        }
        let (term_id, term_size) = self.open_output(tui, win_id, format!("{} {}", command.cmd, command.args.join(" ")));
        let mut command = command;
        command.cols = term_size.width.max(1);
        command.rows = term_size.height.max(1);
        let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos();
        let session_id = format!("task-{}-{nonce}", std::process::id());
        let session = SessionInfo { session_id: session_id.clone(), command: command.clone(), status: SessionStatus::Running, pid: None, code: None };
        let request = ClientMsg::Spawn { session_id, command };
        if self.pending_starts.len() >= 128 { Dashboard::feed(tui, term_id, "[semio] pending task limit reached\n"); return; }
        if self.connection.is_some() {
            if let Err(error) = self.send(&request) { Dashboard::feed(tui, term_id, &format!("[semio] {error}\n")); return; }
        } else {
            self.pending_starts.push_back(request);
            self.connection_status = self.text("Waiting for workspace daemon; Ctrl+B c cancels", "Warten auf Arbeitsbereich; Ctrl+B c bricht ab");
            Dashboard::feed(tui, term_id, &self.text("[semio] waiting for workspace daemon\n", "[semio] warten auf Arbeitsbereich\n"));
        }
        self.sessions.insert(session.session_id.clone(), session.clone());
        if let Some(win) = self.windows.iter_mut().find(|w| w.id == win_id) {
            win.body = WindowBody::Output { terminal: term_id, session: Some(session) };
        }
    }

    fn cancel_pending_start(&mut self, tui: &mut Tui, session_id: &str) -> bool {
        let Some(index) = self.pending_starts.iter().position(|request| matches!(request, ClientMsg::Spawn { session_id: id, .. } if id == session_id)) else { return false; };
        self.pending_starts.remove(index);
        if let Some(mut session) = self.sessions.get(session_id).cloned() { session.status = SessionStatus::Exited; session.code = Some(130); self.update_session(tui, session); }
        true
    }

    fn show_repo_output(&mut self, tui: &mut Tui, win_id: &str, action: &RepoAction) {
        let Ok(executable) = std::env::current_exe() else { return; };
        let Ok(encoded) = serde_json::to_string(action) else { return; };
        self.spawn_output(tui, win_id, CommandSpec { cmd: executable.display().to_string(), args: vec!["repo-view".into(), "--action".into(), encoded], cwd: self.root.clone(), env: Vec::new() });
    }

    fn send(&mut self, message: &ClientMsg) -> std::io::Result<()> {
        self.connection.as_mut().ok_or_else(|| std::io::Error::other("dashboard daemon disconnected"))?.send(message)
    }

    fn focused_session(&self) -> Option<String> {
        self.focused_window().and_then(|window| match &window.body { WindowBody::Output { session: Some(session), .. } => Some(session.session_id.clone()), _ => None })
    }

    fn update_session(&mut self, tui: &mut Tui, info: SessionInfo) {
        self.sessions.insert(info.session_id.clone(), info.clone());
        if self.hidden.contains(&info.session_id) { return; }
        let existing = self.windows.iter().position(|window| matches!(&window.body, WindowBody::Output { session: Some(session), .. } if session.session_id == info.session_id));
        let index = if let Some(index) = existing { index } else {
            let id = format!("w{}", self.next_serial);
            self.next_serial += 1;
            self.place_window(&id);
            let window = self.add_launcher_window(tui, id.clone(), "task");
            self.windows.push(window);
            self.remount(tui);
            let _ = tui.render_full();
            let focused = self.focused.clone();
            let terminal_input = self.terminal_input;
            self.open_output(tui, &id, "task".into());
            self.focused = focused;
            self.terminal_input = terminal_input;
            activate_stack_tab(&mut self.layout, &self.focused);
            if let Some(window) = self.focused_window() { tui.set_focus(Some(window.focus)); }
            self.remount(tui);
            self.windows.len() - 1
        };
        let status = match info.status { SessionStatus::Running => self.text("running", "läuft"), SessionStatus::Stopping => self.text("stopping", "stoppt"), SessionStatus::Exited => self.text("exited", "beendet"), SessionStatus::Failed => self.text("failed", "fehlgeschlagen") };
        let window = &mut self.windows[index];
        let detail = info.code.map(|code| format!("exit {code}")).or_else(|| info.pid.map(|pid| format!("pid {pid}"))).unwrap_or_default();
        if let Some(ChromeState::Window(chrome)) = tui.scene.node_mut(window.chrome).chrome() {
            chrome.title = format!("[{status} {detail}] {} {}", info.command.cmd, info.command.args.join(" "));
        }
        if let WindowBody::Output { session, .. } = &mut window.body { *session = Some(info); }
        self.remount(tui);
        self.refresh_views(tui);
    }

    fn remount(&mut self, tui: &mut Tui) {
        let titles: Vec<_> = self.windows.iter().filter_map(|window| match &tui.scene.node(window.chrome).content { NodeContent::Chrome(ChromeState::Window(chrome)) => Some((window.chrome, chrome.title.clone())), _ => None }).collect();
        self.shell.windows = self.windows.iter().map(|w| (w.id.clone(), w.chrome)).collect();
        self.shell.remount(&mut tui.scene, &self.layout);
        let labels: std::collections::BTreeMap<_, _> = self.windows.iter().filter_map(|window| titles.iter().find(|(id, _)| *id == window.chrome).map(|(_, title)| (window.id.clone(), title.clone()))).collect();
        for (window, title) in titles {
            if let Some(ChromeState::Window(chrome)) = tui.scene.node_mut(window).chrome() {
                chrome.title = title;
                for tab in &mut chrome.stack_tabs {
                    if let Some(label) = labels.get(&tab.label) { tab.label = label.clone(); }
                }
            }
        }
    }

    fn restore_sessions(&mut self) {
        self.restoring = true;
        if let Err(error) = self.send(&ClientMsg::Attach { client_id: format!("dashboard-{}", std::process::id()) }) { self.connection_status = error.to_string(); }
    }

    fn attach_view(&self, tui: &mut Tui, chrome: NodeId, id: &str, mode: &str) -> DashboardWindow {
        let options = if mode == "launcher" { self.commands.iter().map(|(label, _)| label.clone()).collect() } else { vec![self.text("New task — type to find a command", "Neue Aufgabe — Befehl suchen"), self.text("Settings", "Einstellungen"), self.text("Refresh commands", "Befehle aktualisieren"), self.text("Cancel discovery", "Suche abbrechen")] };
        let mut state = WizardState::new(options);
        if mode == "launcher" { state.steps = vec![(self.text("Type to search · Enter runs · Backspace returns", "Tippen zum Suchen · Enter startet · Rücktaste zurück"), String::new())]; }
        let widget = tui.scene.add(chrome, Node::new(NodeContent::Widget(WidgetState::Wizard(state))));
        tui.scene.node_mut(widget).set_constraint(Constraint { width: Dimension::Weight(1), height: Dimension::Weight(1), ..Default::default() });
        if let Some(ChromeState::Window(window)) = tui.scene.node_mut(chrome).chrome() { window.title = match mode { "launcher" => self.text("Commands", "Befehle"), "settings" => self.text("Settings", "Einstellungen"), _ => self.text("Tasks", "Aufgaben") }; }
        DashboardWindow { id: id.into(), chrome, body: match mode { "launcher" => WindowBody::Launcher { widget }, "settings" => WindowBody::Settings { widget }, _ => WindowBody::Overview { widget } }, focus: widget }
    }

    fn attach_launcher(&self, tui: &mut Tui, chrome: NodeId, id: &str) -> DashboardWindow { self.attach_view(tui, chrome, id, "launcher") }

    fn add_launcher_window(&mut self, tui: &mut Tui, id: String, title: &str) -> DashboardWindow {
        let chrome = tui.scene.add(self.shell.canvas, Node::new(NodeContent::Chrome(ChromeState::Window(WindowState::new(title).with_stack_tabs(vec![id.clone()], 0)))));
        tui.scene.node_mut(chrome).set_constraint(Constraint { width: Dimension::Weight(1), direction: Direction::Column, padding: [2, 1, 1, 1], gap: 1, ..Default::default() });
        self.attach_launcher(tui, chrome, &id)
    }

    fn close_window(&mut self, tui: &mut Tui, id: &str) -> bool {
        let idx = self.windows.iter().position(|w| w.id == id);
        if idx.is_none() {
            return false;
        }
        let idx = idx.unwrap();
        let win = self.windows.remove(idx);
        if let WindowBody::Output { session: Some(session), .. } = &win.body { self.hidden.insert(session.session_id.clone()); }
        match &win.body {
            WindowBody::Launcher { widget } | WindowBody::Overview { widget } | WindowBody::Settings { widget } => tui.scene.remove(*widget),
            WindowBody::Output { terminal, .. } => tui.scene.remove(*terminal),
        }
        tui.scene.remove(win.chrome);
        remove_window(&mut self.layout, id);
        if self.windows.is_empty() {
            return true;
        }
        let order = self.window_order();
        self.focused = order[0].clone();
        self.terminal_input = matches!(self.windows[0].body, WindowBody::Output { .. });
        tui.set_focus(Some(self.windows.iter().find(|w| w.id == self.focused).map(|w| w.focus).unwrap_or(self.windows[0].focus)));
        self.remount(tui);
        false
    }

    fn handle_view_signal(&mut self, tui: &mut Tui, win_id: &str, signal: WidgetSignal) {
        let Some(index) = self.windows.iter().position(|window| window.id == win_id) else { return; };
        match signal {
            WidgetSignal::Activated(selected) => match self.windows[index].body {
                WindowBody::Overview { .. } => match selected {
                    0 => self.replace_view(tui, index, "launcher"),
                    1 => { self.replace_view(tui, index, "settings"); self.refresh_view_for(tui, index); }
                    2 => self.refresh_inventory(),
                    3 => self.cancel_inventory(),
                    _ => {
                        if let Some(session) = self.sessions.values().nth(selected - 4).cloned() {
                            self.hidden.remove(&session.session_id); self.update_session(tui, session.clone());
                            if let Some(window) = self.windows.iter().find(|window| matches!(&window.body, WindowBody::Output { session: Some(info), .. } if info.session_id == session.session_id)) {
                                self.focused = window.id.clone(); self.terminal_input = false;
                                activate_stack_tab(&mut self.layout, &self.focused); tui.set_focus(Some(window.focus)); self.remount(tui);
                            }
                            self.restore_sessions();
                        }
                    }
                },
                WindowBody::Launcher { .. } => {
                    if let Some((_, leaf)) = self.commands.get(selected).cloned() {
                        match leaf { CommandLeaf::Process(mut spec) => { self.preferences.bind(&mut spec); self.spawn_output(tui, win_id, spec); }, CommandLeaf::Repo(action) => self.show_repo_output(tui, win_id, &action) }
                    }
                }
                WindowBody::Settings { .. } => {
                    let mut change = crate::preferences::Change::default();
                    match selected {
                        0 => change.language = Some(if self.preferences.language == "en" { "de" } else { "en" }.into()),
                        1 => change.appearance = Some(if self.light { "dark" } else { "light" }.into()),
                        2 => change.terminology = Some(if self.preferences.terminology == "native" { "reuse" } else { "native" }.into()),
                        3 => change.renderer = Some(match self.preferences.renderer.as_str() { "react" => "wgpu-wasm", "wgpu-wasm" => "wgpu-native", _ => "react" }.into()),
                        4 => change.layout = Some(match self.preferences.layout.as_str() { "tabs" => "columns", "columns" => "rows", _ => "tabs" }.into()),
                        5 => { self.shared_preferences = !self.shared_preferences; self.refresh_view_for(tui, index); return; }
                        _ => { self.replace_view(tui, index, "overview"); self.refresh_view_for(tui, index); return; }
                    }
                    self.queue_preference(change);
                }
                _ => {}
            },
            WidgetSignal::NavigateBack => { self.replace_view(tui, index, "overview"); self.refresh_view_for(tui, index); }
            _ => {}
        }
    }

    fn poll_sessions(&mut self, tui: &mut Tui) -> bool {
        if self.connection.is_none() && self.connecting.is_none() && std::time::Instant::now() >= self.next_reconnect {
            self.next_reconnect = std::time::Instant::now() + std::time::Duration::from_millis(500);
            let (sender, receiver) = std::sync::mpsc::sync_channel(1);
            let root = self.root.clone();
            std::thread::spawn(move || {
                let result = Connection::connect(&root).or_else(|_| {
                    let executable = std::env::current_exe()?;
                    if crate::daemon::supervisor::start_detached(&root, &executable) != 0 { return Err(std::io::Error::other("daemon startup failed")); }
                    Connection::connect(&root)
                });
                let _ = sender.send(result);
            });
            self.connecting = Some(receiver);
        }
        let mut connected = false;
        if let Some(result) = self.connecting.as_ref().and_then(|receiver| receiver.try_recv().ok()) {
            self.connecting = None;
            if let Err(error) = &result { self.connection_status = error.to_string(); }
            if let Ok(mut connection) = result {
                connected = true;
                let _ = connection.send(&ClientMsg::Attach { client_id: format!("dashboard-{}", std::process::id()) });
                self.connection = Some(connection);
                self.restoring = true;
                self.connection_status = "connected".into();
            }
        }
        let Some(connection) = self.connection.as_mut() else { return connected };
        while let Some(request) = self.pending_starts.front() {
            if let Err(error) = connection.send(request) { self.connection_status = error.to_string(); break; }
            self.pending_starts.pop_front(); connected = true;
        }
        let messages = match connection.receive(std::time::Duration::ZERO) {
            Ok(messages) => messages,
            Err(_) => { self.connection = None; self.connection_status = "reconnecting".into(); return true; }
        };
        let changed = connected || !messages.is_empty();
        for message in messages {
            match message {
                Message::Control(ServerMsg::Sessions { sessions }) => {
                    if self.restoring {
                        for window in &self.windows {
                            if let WindowBody::Output { terminal, .. } = window.body {
                                let size = tui.scene.rect(terminal);
                                if let Some(WidgetState::Terminal(state)) = tui.scene.node_mut(terminal).widget() { *state = TerminalState::new(Size { width: size.width.max(1), height: size.height.max(1) }, 8000); }
                            }
                        }
                    }
                    for session in sessions { self.update_session(tui, session); }
                    self.refresh_views(tui);
                }
                Message::Control(ServerMsg::SessionChanged { session }) => self.update_session(tui, session),
                Message::Output { session_id, data } => {
                    for window in &self.windows {
                        if let WindowBody::Output { terminal, session: Some(session) } = &window.body {
                            if session.session_id == session_id {
                                if let Some(WidgetState::Terminal(state)) = tui.scene.node_mut(*terminal).widget() { state.feed(&data); }
                            }
                        }
                    }
                }
                Message::Control(ServerMsg::ReplayComplete {}) => { self.restoring = false; self.resize_terminals(tui); }
                Message::Control(ServerMsg::Error { message }) => {
                    self.connection_status = message.clone();
                    if let Some(window) = self.focused_window() {
                        if let WindowBody::Output { terminal, .. } = window.body { Dashboard::feed(tui, terminal, &format!("[semio] {message}\n")); }
                    }
                }
                Message::Disconnected(message) => { self.connection = None; self.connection_status = message; }
                Message::Control(ServerMsg::Shutdown {}) => { self.connection = None; self.connection_status = "daemon stopped".into(); }
                _ => {}
            }
        }
        changed
    }

    fn resize_terminals(&mut self, tui: &mut Tui) {
        let mut commands = Vec::new();
        for win in &mut self.windows {
            if let WindowBody::Output { terminal, session } = &mut win.body {
                let rect = tui.scene.rect(*terminal);
                if rect.width == 0 || rect.height == 0 { continue; }
                let size = Size { width: rect.width.max(1), height: rect.height.max(1) };
                if let Some(WidgetState::Terminal(t)) = tui.scene.node_mut(*terminal).widget() {
                    t.resize(size);
                }
                if let Some(s) = session.as_mut() {
                    commands.push(ClientMsg::Resize { session_id: s.session_id.clone(), cols: size.width, rows: size.height });
                }
            }
        }
        for command in commands { let _ = self.send(&command); }
    }

    fn footer_hints(&self) -> Vec<KeyHint> {
        if matches!(self.leader, LeaderMode::Armed) {
            return [("n", "new task", "neue Aufgabe"), ("s", "show tasks", "Aufgaben zeigen"), ("r", "restart", "neu starten"), ("c", "cancel", "abbrechen"), ("k", "kill", "beenden"), ("d", "detach", "trennen"), ("Q", "shutdown all", "alles stoppen"), ("a", "appearance", "Darstellung"), ("l", "language", "Sprache"), ("h", "tasks", "Aufgaben"), ("p", "settings", "Einstellungen"), ("f", "refresh", "aktualisieren"), ("e", "cancel discovery", "Suche abbrechen")]
                .into_iter().map(|(key, en, de)| KeyHint { key: key.into(), label: self.text(en, de) }).collect();
        }
        let output = self.focused_window().map(|w| matches!(w.body, WindowBody::Output { .. })).unwrap_or(false);
        let hints = if output && self.terminal_input {
            vec![("Esc", "pane", "Fenster"), ("C-B", "controls", "Steuerung")]
        } else if output {
            vec![("C-B", "controls", "Steuerung"), ("C-B n", "new task", "neue Aufgabe"), ("C-w", "close view", "Ansicht schließen"), ("q", "detach", "trennen")]
        } else {
            vec![("arrows", "move", "bewegen"), ("Enter", "select", "auswählen"), ("Tab", "window", "Fenster"), ("C-B", "controls", "Steuerung"), ("q", "detach", "trennen")]
        };
        hints.into_iter().map(|(key, en, de)| KeyHint { key: key.into(), label: self.text(en, de) }).collect()
    }

    fn window_id_for_chrome(&self, chrome: NodeId) -> Option<String> {
        self.windows.iter().find(|w| w.chrome == chrome || w.focus == chrome).map(|w| w.id.clone())
    }

    fn stack_tab_window_id(&self, host_id: &str, tab_index: usize) -> Option<String> {
        fn collect_stack(layout: &WindowLayout, host: &str) -> Option<Vec<String>> {
            fn walk_child(child: &ui_tui::tui::layout::WindowLayoutChild, host: &str) -> Option<Vec<String>> {
                match child {
                    ui_tui::tui::layout::WindowLayoutChild::Stack(s) => {
                        if s.children.iter().any(|c| c.window_kind_id == host) {
                            Some(s.children.iter().map(|c| c.window_kind_id.clone()).collect())
                        } else {
                            None
                        }
                    }
                    ui_tui::tui::layout::WindowLayoutChild::Axis(a) => a.children.iter().find_map(|c| walk_child(c, host)),
                }
            }
            match &layout.root {
                ui_tui::tui::layout::WindowLayoutRoot::Stack(s) => {
                    if s.children.iter().any(|c| c.window_kind_id == host) {
                        Some(s.children.iter().map(|c| c.window_kind_id.clone()).collect())
                    } else {
                        None
                    }
                }
                ui_tui::tui::layout::WindowLayoutRoot::Axis(a) => a.children.iter().find_map(|c| walk_child(c, host)),
            }
        }
        collect_stack(&self.layout, host_id).and_then(|tabs| tabs.get(tab_index).cloned())
    }
}
// #endregion 🔖️Dashboard

// #region 🔖️Run
/// 🎛️ Starts native task controls immediately, then connects and discovers asynchronously.
pub fn run(root: &Path) -> i32 { run_with(root, &crate::args::ParsedArgs::default()) }

/// ⚙️ Starts a native dashboard with strict optional configuration overrides.
pub fn run_with(root: &Path, parsed: &crate::args::ParsedArgs) -> i32 {
    let started = std::time::Instant::now();
    if !parsed.segments.is_empty() { eprintln!("[dashboard] unexpected arguments: {}", parsed.segments.join(" ")); return 2; }
    if parsed.has_flag("help") { println!("semio [dashboard] [--root PATH] [--config JOURNAL] [--language en|de] [--appearance dark|light] [--terminology native|reuse] [--renderer react|wgpu-wasm|wgpu-native] [--layout tabs|columns|rows]\nCtrl+B: n commands, h tasks, p settings, f refresh, e cancel discovery, r restart, c cancel, k kill, d detach, Q shutdown"); return 0; }
    let root = crate::ipc::canonical_path(&parsed.flag("root").map_or_else(|| root.to_path_buf(), PathBuf::from));
    let preferences = match crate::preferences::load(&root, parsed) { Ok(preferences) => preferences, Err(error) => { eprintln!("[dashboard] {error}"); return 2; } };
    let Ok(mut term) = NativeTerminal::new() else { eprintln!("[dashboard] failed to attach to the terminal"); return 1; };
    if term.enter().is_err() { return 1; }
    let size = term.size().unwrap_or(Size { width: 100, height: 32 });
    let light = preferences.appearance == "light";
    let mut tui = Tui::new(size, Theme::new(if light { AppearanceName::Light } else { AppearanceName::Dark }));
    let navbar = NavbarState { left: vec![NavItem { id: "logo".into(), label: "semio".into(), active: true }], center: vec![NavItem { id: "mode".into(), label: "dashboard".into(), active: false }], right: vec![] };
    let footer = FooterState { hints: vec![], status: "connecting · discovering commands".into() };
    let layout = create_default_layout(&["w1".into()], "row", None, Some(&["Tasks".into()]));
    let shell = shell(&mut tui.scene, navbar, footer, &layout);
    let locale = if preferences.language == "de" { Locale::German } else { Locale::English };
    let mut dash = Dashboard { root: root.clone(), commands: Vec::new(), inventory: None, inventory_status: "discovering commands".into(), last_progress: 0, preferences, preference_path: crate::preferences::local_path(&root, parsed), saving_preferences: None, shared_preferences: parsed.has_flag("workspace"), layout, shell, windows: Vec::new(), next_serial: 2, focused: "w1".into(), leader: LeaderMode::Idle, terminal_input: false, connection: None, pending_starts: Default::default(), connecting: None, connection_status: "connecting".into(), next_reconnect: std::time::Instant::now(), restoring: true, sessions: Default::default(), hidden: Default::default(), locale, light };
    let (w1_id, w1_chrome) = dash.shell.windows[0].clone();
    let w1 = dash.attach_view(&mut tui, w1_chrome, &w1_id, "overview");
    dash.windows.push(w1); dash.sort_commands(); dash.remount(&mut tui);
    tui.set_focus(Some(dash.windows[0].focus)); dash.sync_chrome_focus(&mut tui);
    if let Some(ChromeState::Footer(footer)) = tui.scene.node_mut(dash.shell.footer).chrome() { footer.hints = dash.footer_hints(); }
    term.present(&tui.render_full()).ok();
    if std::env::var_os("SEMIO_DASHBOARD_TRACE").is_some() { eprintln!("[DEBUG] dashboard usable first frame elapsed_us={}", started.elapsed().as_micros()); }
    dash.inventory = Some(crate::inventory::start(root, false));

    loop {
        let output_changed = dash.poll_sessions(&mut tui) | dash.poll_inventory(&mut tui);
        let events = term.poll(std::time::Duration::from_millis(80)).unwrap_or_default();
        let mut quit = false;
        let mut need_paint = output_changed || !events.is_empty();

        for event in &events {
            match event {
                Event::Resize(_) => {
                    tui.dispatch(event);
                    dash.resize_terminals(&mut tui);
                    need_paint = true;
                }
                Event::Mouse(_) => {
                    for (chrome_id, signal) in tui.dispatch(event) {
                        if let Some(win_id) = dash.window_id_for_chrome(chrome_id) {
                            match signal {
                                WidgetSignal::WindowClose => {
                                    if dash.close_window(&mut tui, &win_id) {
                                        quit = true;
                                    }
                                    need_paint = true;
                                }
                                WidgetSignal::WindowMaximize => {
                                    if dash.layout.zoomed.is_some() {
                                        zoom_window(&mut dash.layout, None);
                                    } else {
                                        zoom_window(&mut dash.layout, Some(&win_id));
                                    }
                                    dash.remount(&mut tui);
                                    need_paint = true;
                                }
                                WidgetSignal::WindowNewTab => {
                                    let new_id = format!("w{}", dash.next_serial);
                                    dash.next_serial += 1;
                                    push_window_to_stack(&mut dash.layout, &win_id, WindowLayoutWindowNode { window_kind_id: new_id.clone(), title: Some("Commands".into()), corner: None });
                                    let w = dash.add_launcher_window(&mut tui, new_id.clone(), "Commands");
                                    dash.windows.push(w);
                                    dash.focused = new_id;
                                    dash.terminal_input = false;
                                    tui.set_focus(Some(dash.windows.last().unwrap().focus));
                                    dash.remount(&mut tui);
                                    need_paint = true;
                                }
                                WidgetSignal::WindowTabActivated(tab_i) => {
                                    if let Some(tab_id) = dash.stack_tab_window_id(&win_id, tab_i) {
                                        activate_stack_tab(&mut dash.layout, &tab_id);
                                        dash.focused = tab_id.clone();
                                        if let Some(w) = dash.windows.iter().find(|w| w.id == tab_id) {
                                            dash.terminal_input = matches!(w.body, WindowBody::Output { .. });
                                            tui.set_focus(Some(w.focus));
                                        }
                                        dash.remount(&mut tui);
                                    }
                                    need_paint = true;
                                }
                                WidgetSignal::Activated(_) | WidgetSignal::NavigateBack => {
                                    dash.focused = win_id.clone();
                                    dash.handle_view_signal(&mut tui, &win_id, signal);
                                    need_paint = true;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Event::Key(k) => {
                    if k.key == Key::Char('b') && k.mods & mods::CTRL != 0 {
                        dash.leader = LeaderMode::Armed;
                        need_paint = true;
                        continue;
                    }
                    if matches!(dash.leader, LeaderMode::Armed) {
                        let consumed = match k.key {
                            Key::Char('a') => {
                                dash.queue_preference(crate::preferences::Change { appearance: Some(if dash.light { "dark" } else { "light" }.into()), ..Default::default() });
                                dash.leader = LeaderMode::Idle;
                                true
                            }
                            Key::Char('l') => {
                                dash.queue_preference(crate::preferences::Change { language: Some(if matches!(dash.locale, Locale::English) { "de" } else { "en" }.into()), ..Default::default() });
                                dash.leader = LeaderMode::Idle;
                                true
                            }
                            Key::Char('h') => { dash.show_home(&mut tui); dash.leader = LeaderMode::Idle; true }
                            Key::Char('p') => {
                                dash.show_home(&mut tui);
                                if let Some(index) = dash.windows.iter().position(|window| window.id == dash.focused) { dash.replace_view(&mut tui, index, "settings"); dash.refresh_view_for(&mut tui, index); }
                                dash.leader = LeaderMode::Idle; true
                            }
                            Key::Char('f') => { dash.refresh_inventory(); dash.leader = LeaderMode::Idle; true }
                            Key::Char('e') => { dash.cancel_inventory(); dash.leader = LeaderMode::Idle; true }
                            Key::Char('r') | Key::Char('c') | Key::Char('k') => {
                                if let Some(session_id) = dash.focused_session() {
                                    if matches!(k.key, Key::Char('c') | Key::Char('k')) && dash.cancel_pending_start(&mut tui, &session_id) { dash.leader = LeaderMode::Idle; need_paint = true; continue; }
                                    let command = match k.key { Key::Char('r') => ClientMsg::Restart { session_id }, Key::Char('c') => ClientMsg::Stop { session_id }, _ => ClientMsg::Kill { session_id } };
                                    if let Err(error) = dash.send(&command) { dash.connection_status = error.to_string(); }
                                }
                                dash.leader = LeaderMode::Idle;
                                true
                            }
                            Key::Char('s') => {
                                dash.hidden.clear();
                                dash.restore_sessions();
                                dash.leader = LeaderMode::Idle;
                                true
                            }
                            Key::Char('d') => { quit = true; dash.leader = LeaderMode::Idle; true }
                            Key::Char('Q') => {
                                match dash.send(&ClientMsg::Shutdown {}) { Ok(()) => quit = true, Err(error) => dash.connection_status = error.to_string() }
                                dash.leader = LeaderMode::Idle;
                                true
                            }
                            Key::Char('z') => {
                                if dash.layout.zoomed.is_some() {
                                    zoom_window(&mut dash.layout, None);
                                } else {
                                    zoom_window(&mut dash.layout, Some(&dash.focused));
                                }
                                dash.remount(&mut tui);
                                dash.leader = LeaderMode::Idle;
                                true
                            }
                            Key::Char('-') => {
                                let new_id = format!("w{}", dash.next_serial);
                                dash.next_serial += 1;
                                let _ = split_window(&mut dash.layout, &dash.focused, "column", &new_id, Some("shell".into()));
                                let w = dash.add_launcher_window(&mut tui, new_id.clone(), "Commands");
                                dash.windows.push(w);
                                dash.focused = new_id;
                                dash.terminal_input = false;
                                tui.set_focus(Some(dash.windows.last().unwrap().focus));
                                dash.remount(&mut tui);
                                dash.leader = LeaderMode::Idle;
                                true
                            }
                            Key::Char('|') => {
                                let new_id = format!("w{}", dash.next_serial);
                                dash.next_serial += 1;
                                let _ = split_window(&mut dash.layout, &dash.focused, "row", &new_id, Some("shell".into()));
                                let w = dash.add_launcher_window(&mut tui, new_id.clone(), "Commands");
                                dash.windows.push(w);
                                dash.focused = new_id;
                                dash.terminal_input = false;
                                tui.set_focus(Some(dash.windows.last().unwrap().focus));
                                dash.remount(&mut tui);
                                dash.leader = LeaderMode::Idle;
                                true
                            }
                            Key::Char('x') => {
                                let focused = dash.focused.clone();
                                if dash.close_window(&mut tui, &focused) {
                                    quit = true;
                                }
                                dash.leader = LeaderMode::Idle;
                                true
                            }
                            Key::Char('t') => {
                                let toggle = dash.focused_window().and_then(|w| if matches!(w.body, WindowBody::Output { .. }) { Some(w.focus) } else { None });
                                if let Some(focus) = toggle {
                                    dash.terminal_input = !dash.terminal_input;
                                    if dash.terminal_input {
                                        tui.set_focus(Some(focus));
                                    }
                                }
                                dash.leader = LeaderMode::Idle;
                                true
                            }
                            Key::Char('n') => {
                                let new_id = format!("w{}", dash.next_serial);
                                dash.next_serial += 1;
                                dash.place_window(&new_id);
                                let w = dash.add_launcher_window(&mut tui, new_id.clone(), "Commands");
                                dash.windows.push(w);
                                dash.focused = new_id;
                                dash.terminal_input = false;
                                tui.set_focus(Some(dash.windows.last().unwrap().focus));
                                dash.remount(&mut tui);
                                dash.leader = LeaderMode::Idle;
                                true
                            }
                            Key::Esc => {
                                dash.leader = LeaderMode::Idle;
                                true
                            }
                            _ => {
                                dash.leader = LeaderMode::Idle;
                                false
                            }
                        };
                        if consumed {
                            need_paint = true;
                            continue;
                        }
                    }

                    if k.key == Key::Char('q') && k.mods == 0 && !dash.terminal_input && !dash.focused_window().is_some_and(|window| matches!(window.body, WindowBody::Launcher { .. })) {
                        quit = true;
                        break;
                    }
                    if k.key == Key::Char('w') && k.mods & mods::CTRL != 0 {
                        let focused = dash.focused.clone();
                        if dash.close_window(&mut tui, &focused) {
                            quit = true;
                        }
                        need_paint = true;
                        continue;
                    }

                    if k.key == Key::Tab || k.key == Key::BackTab {
                        let order = dash.window_order();
                        let idx = order.iter().position(|w| *w == dash.focused).unwrap_or(0);
                        let next = if k.key == Key::Tab { (idx + 1) % order.len() } else { (idx + order.len() - 1) % order.len() };
                        dash.focused = order[next].clone();
                        activate_stack_tab(&mut dash.layout, &dash.focused);
                        dash.remount(&mut tui);
                        if let Some(w) = dash.windows.iter().find(|w| w.id == dash.focused) {
                            dash.terminal_input = matches!(w.body, WindowBody::Output { .. });
                            tui.set_focus(Some(w.focus));
                        }
                        need_paint = true;
                        continue;
                    }

                    let fid = dash.focused.clone();
                    if let Some(w) = dash.windows.iter().find(|w| w.id == fid) {
                        if dash.terminal_input && matches!(w.body, WindowBody::Output { .. }) {
                            if k.key == Key::Esc {
                                dash.terminal_input = false;
                                need_paint = true;
                                continue;
                            }
                            let term_id = match &w.body {
                                WindowBody::Output { terminal, .. } => *terminal,
                                _ => continue,
                            };
                            tui.set_focus(Some(term_id));
                            for (_, signal) in tui.dispatch(event) {
                                if signal == WidgetSignal::TerminalPassthrough {
                                    if let (Some(data), Some(session_id)) = (key_to_pty_bytes(k), dash.focused_session()) {
                                        if let Err(error) = dash.send(&ClientMsg::Input { session_id, data }) { dash.connection_status = error.to_string(); }
                                    }
                                }
                            }
                            need_paint = true;
                            continue;
                        }
                        if let WindowBody::Launcher { widget } | WindowBody::Overview { widget } | WindowBody::Settings { widget } = &w.body {
                            tui.set_focus(Some(*widget));
                            let key_ev = *k;
                            if k.key == Key::Esc && matches!(&tui.scene.node(*widget).content, NodeContent::Widget(WidgetState::Wizard(state)) if state.filter.is_empty()) { dash.handle_view_signal(&mut tui, &fid, WidgetSignal::NavigateBack); need_paint = true; continue; }
                            if let Some(widget_state) = tui.scene.node_mut(*widget).widget() {
                                if let Some(signal) = widget_state.on_key(&key_ev) {
                                    dash.handle_view_signal(&mut tui, &fid, signal);
                                }
                            }
                            need_paint = true;
                        }
                    }
                }
                _ => {
                    tui.dispatch(event);
                    need_paint = true;
                }
            }
        }

        if quit {
            break;
        }
        dash.sync_chrome_focus(&mut tui);
        if let Some(chrome) = tui.scene.node_mut(dash.shell.footer).chrome() {
            if let ChromeState::Footer(f) = chrome {
                f.hints = dash.footer_hints();
                let running = dash.sessions.values().filter(|session| matches!(session.status, SessionStatus::Running | SessionStatus::Stopping)).count();
                let progress = dash.inventory.as_ref().map(|job| format!(" · discovering {}s · C-B e cancels", job.started.elapsed().as_secs())).unwrap_or_else(|| format!(" · {} commands · {}", dash.commands.len(), dash.inventory_status));
                f.status = format!("{running} · {}{}", dash.connection_status, progress);
            }
        }
        if need_paint {
            term.present(&tui.render_full()).ok();
        }
    }

    let _ = dash.send(&ClientMsg::Detach {});
    let _ = term.leave();
    0
}
// #endregion 🔖️Run

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
