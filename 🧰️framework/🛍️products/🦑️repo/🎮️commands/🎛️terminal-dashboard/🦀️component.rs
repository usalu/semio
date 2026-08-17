use crate::command_tree_discovery::{CommandNode, CommandSpec};
use std::path::Path;
use ui_styling::appearance::AppearanceName;
use ui_tui::tui::backend::{NativeTerminal, TerminalBackend};
use ui_tui::tui::chrome::{shell, ChromeState, FooterState, KeyHint, NavItem, NavbarState, WindowState};
use ui_tui::tui::engine::Tui;
use ui_tui::tui::event::{mods, Event, Key, KeyEvent};
use ui_tui::tui::geometry::Size;
use ui_tui::tui::layout::{
    activate_stack_tab, create_default_layout, push_window_to_stack, remove_window, split_window, zoom_window, WindowLayout,
    WindowLayoutWindowNode,
};
use ui_tui::tui::pty::{Pty, PtySize};
use ui_tui::tui::scene::{Node, NodeContent, NodeId};
use ui_tui::tui::theme::Theme;
use ui_tui::tui::widget::{TerminalState, WidgetSignal, WidgetState, WizardState};
use ui_tui::tui::layout::{Constraint, Dimension, Direction};

// #region 🔖️Session
struct PtySession {
    pty: Pty,
}

impl PtySession {
    fn kill(&mut self) {
        let _ = self.pty.kill();
    }
}
// #endregion 🔖️Session

// #region 🔖️Window
enum WindowBody {
    Wizard { widget: NodeId, cursor: Vec<usize> },
    Output { terminal: NodeId, session: Option<PtySession> },
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
// #endregion 🔖️Window

// #region 🔖️TreeNav
fn current_node<'a>(tree: &'a CommandNode, cursor: &[usize]) -> &'a CommandNode {
    let mut node = tree;
    for &i in cursor {
        if let Some(c) = node.children.get(i) {
            node = c;
        }
    }
    node
}

fn wizard_steps(tree: &CommandNode, cursor: &[usize]) -> Vec<(String, String)> {
    let mut steps = Vec::new();
    let mut node = tree;
    for &i in cursor {
        if let Some(c) = node.children.get(i) {
            steps.push((c.key.clone(), c.label.clone()));
            node = c;
        }
    }
    steps
}
// #endregion 🔖️TreeNav

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
    tree: CommandNode,
    layout: WindowLayout,
    shell: ui_tui::tui::chrome::Shell,
    windows: Vec<DashboardWindow>,
    next_serial: u32,
    focused: String,
    leader: LeaderMode,
    terminal_input: bool,
}

impl Dashboard {
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

    fn refresh_wizard_for(&mut self, tui: &mut Tui, idx: usize) {
        let (widget, cursor) = match &self.windows[idx].body {
            WindowBody::Wizard { widget, cursor } => (*widget, cursor.clone()),
            _ => return,
        };
        let node = current_node(&self.tree, &cursor);
        let options: Vec<String> = node.children.iter().map(|c| c.label.clone()).collect();
        if let Some(WidgetState::Wizard(w)) = tui.scene.node_mut(widget).widget() {
            w.steps = wizard_steps(&self.tree, &cursor);
            w.options = options;
            w.selected = 0;
            w.offset = 0;
            w.filter.clear();
        }
    }

    fn spawn_output(&mut self, tui: &mut Tui, win_id: &str, spec: CommandSpec) {
        let win = self.windows.iter_mut().find(|w| w.id == win_id).expect("window");
        if let WindowBody::Wizard { widget, .. } = &win.body {
            tui.scene.remove(*widget);
        }
        let inner = tui.scene.rect(win.chrome);
        let term_size = Size {
            width: inner.width.saturating_sub(4),
            height: inner.height.saturating_sub(4),
        };
        let term_id = tui.scene.add(
            win.chrome,
            Node::new(NodeContent::Widget(WidgetState::Terminal(TerminalState::new(term_size, 8000)))),
        );
        tui.scene.node_mut(term_id).set_constraint(Constraint { height: Dimension::Weight(1), ..Default::default() });
        let args: Vec<&str> = spec.args.iter().map(String::as_str).collect();
        let env_refs: Vec<(&str, &str)> = spec.env.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
        let pty_size = PtySize { cols: term_size.width.max(1), rows: term_size.height.max(1) };
        let session = match Pty::spawn(&spec.cmd, &args, &env_refs, Some(spec.cwd.as_path()), pty_size) {
            Ok(pty) => {
                eprintln!("[DEBUG] spawned {} pid={}", win_id, pty.pid());
                Some(PtySession { pty })
            }
            Err(e) => {
                if let Some(WidgetState::Terminal(t)) = tui.scene.node_mut(term_id).widget() {
                    t.feed(format!("[semio] spawn failed: {e}\r\n").as_bytes());
                }
                None
            }
        };
        win.body = WindowBody::Output { terminal: term_id, session };
        win.focus = term_id;
        tui.set_focus(Some(term_id));
        self.terminal_input = true;
        if let Some(chrome) = tui.scene.node_mut(win.chrome).chrome() {
            if let ChromeState::Window(ws) = chrome {
                ws.title = format!("{} {}", spec.cmd, spec.args.join(" "));
            }
        }
    }

    fn kill_session(win: &mut DashboardWindow) {
        if let WindowBody::Output { session, .. } = &mut win.body {
            if let Some(mut s) = session.take() {
                eprintln!("[DEBUG] killing session for {}", win.id);
                s.kill();
            }
        }
    }

    fn remount(&mut self, tui: &mut Tui) {
        self.shell.windows = self.windows.iter().map(|w| (w.id.clone(), w.chrome)).collect();
        self.shell.remount(&mut tui.scene, &self.layout);
    }

    fn attach_wizard(&mut self, tui: &mut Tui, chrome: NodeId, id: &str) -> DashboardWindow {
        let node = current_node(&self.tree, &[]);
        let options: Vec<String> = node.children.iter().map(|c| c.label.clone()).collect();
        let widget = tui.scene.add(chrome, Node::new(NodeContent::Widget(WidgetState::Wizard(WizardState::new(options)))));
        tui.scene.node_mut(widget).set_constraint(Constraint { height: Dimension::Weight(1), ..Default::default() });
        DashboardWindow {
            id: id.to_string(),
            chrome,
            body: WindowBody::Wizard { widget, cursor: Vec::new() },
            focus: widget,
        }
    }

    fn add_wizard_window(&mut self, tui: &mut Tui, id: String, title: &str) -> DashboardWindow {
        let chrome = tui.scene.add(
            self.shell.canvas,
            Node::new(NodeContent::Chrome(ChromeState::Window(WindowState::new(title).with_stack_tabs(vec![id.clone()], 0)))),
        );
        tui.scene.node_mut(chrome).set_constraint(Constraint {
            width: Dimension::Weight(1),
            direction: Direction::Column,
            padding: [2, 1, 1, 1],
            gap: 1,
            ..Default::default()
        });
        self.attach_wizard(tui, chrome, &id)
    }

    fn close_window(&mut self, tui: &mut Tui, id: &str) -> bool {
        let idx = self.windows.iter().position(|w| w.id == id);
        if idx.is_none() {
            return false;
        }
        let idx = idx.unwrap();
        let mut win = self.windows.remove(idx);
        Dashboard::kill_session(&mut win);
        match &win.body {
            WindowBody::Wizard { widget, .. } => tui.scene.remove(*widget),
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

    fn handle_wizard_signal(&mut self, tui: &mut Tui, win_id: &str, signal: WidgetSignal) {
        let idx = self.windows.iter().position(|w| w.id == win_id).unwrap_or(0);
        match signal {
            WidgetSignal::Activated(i) => {
                let cursor = match &self.windows[idx].body {
                    WindowBody::Wizard { cursor, .. } => cursor.clone(),
                    _ => return,
                };
                let parent = current_node(&self.tree, &cursor);
                if let Some(child) = parent.children.get(i) {
                    if child.children.is_empty() {
                        if let Some(spec) = child.spec.clone() {
                            self.spawn_output(tui, win_id, spec);
                        }
                    } else if let WindowBody::Wizard { cursor: c, .. } = &mut self.windows[idx].body {
                        c.push(i);
                        self.refresh_wizard_for(tui, idx);
                    }
                }
            }
            WidgetSignal::NavigateBack => {
                if let WindowBody::Wizard { cursor, .. } = &mut self.windows[idx].body {
                    if !cursor.is_empty() {
                        cursor.pop();
                        self.refresh_wizard_for(tui, idx);
                    }
                }
            }
            _ => {}
        }
    }

    fn poll_pty(&mut self, tui: &mut Tui) {
        for win in &mut self.windows {
            if let WindowBody::Output { terminal, session } = &mut win.body {
                if let Some(s) = session.as_mut() {
                    let mut buf = [0u8; 4096];
                    loop {
                        match s.pty.try_read(&mut buf) {
                            Ok(0) => break,
                            Ok(n) => {
                                if let Some(WidgetState::Terminal(t)) = tui.scene.node_mut(*terminal).widget() {
                                    t.feed(&buf[..n]);
                                }
                            }
                            Err(_) => break,
                        }
                    }
                    if let Ok(Some(_)) = s.pty.try_wait() {
                        session.take();
                    }
                }
            }
        }
    }

    fn resize_terminals(&mut self, tui: &mut Tui) {
        for win in &mut self.windows {
            if let WindowBody::Output { terminal, session } = &mut win.body {
                let rect = tui.scene.rect(*terminal);
                let size = Size { width: rect.width.max(1), height: rect.height.max(1) };
                if let Some(WidgetState::Terminal(t)) = tui.scene.node_mut(*terminal).widget() {
                    t.resize(size);
                }
                if let Some(s) = session.as_mut() {
                    let _ = s.pty.resize(PtySize { cols: size.width, rows: size.height });
                }
            }
        }
    }

    fn footer_hints(&self) -> Vec<KeyHint> {
        let output = self.focused_window().map(|w| matches!(w.body, WindowBody::Output { .. })).unwrap_or(false);
        if output && self.terminal_input {
            vec![
                KeyHint { key: "Esc".into(), label: "pane".into() },
                KeyHint { key: "C-Space".into(), label: "leader".into() },
                KeyHint { key: "q".into(), label: "quit".into() },
            ]
        } else if output {
            vec![
                KeyHint { key: "C-Space t".into(), label: "term".into() },
                KeyHint { key: "C-w".into(), label: "close".into() },
                KeyHint { key: "q".into(), label: "quit".into() },
            ]
        } else {
            vec![
                KeyHint { key: "jk/arrows".into(), label: "move".into() },
                KeyHint { key: "\u{21b5}".into(), label: "select".into() },
                KeyHint { key: "Tab".into(), label: "window".into() },
                KeyHint { key: "q".into(), label: "quit".into() },
            ]
        }
    }

    fn window_id_for_chrome(&self, chrome: NodeId) -> Option<String> {
        self.windows.iter().find(|w| w.chrome == chrome).map(|w| w.id.clone())
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
/// 🎛️ Interactive semio dashboard: wizard-built commands in tiled PTY windows.
pub fn run(root: &Path) -> i32 {
    let tree = crate::command_tree_discovery::discover(root);
    if tree.children.is_empty() {
        eprintln!("[semio] no runnable commands discovered in the repo.");
        return 1;
    }
    let Ok(mut term) = NativeTerminal::new() else {
        eprintln!("[semio] failed to attach to the terminal");
        return 1;
    };
    if term.enter().is_err() {
        return 1;
    }
    let size = term.size().unwrap_or(Size { width: 100, height: 32 });
    let mut tui = Tui::new(size, Theme::new(AppearanceName::Dark));

    let navbar = NavbarState {
        left: vec![NavItem { id: "logo".into(), label: "semio".into(), active: true }],
        center: vec![NavItem { id: "mode".into(), label: "dashboard".into(), active: false }],
        right: vec![],
    };
    let footer = FooterState { hints: vec![], status: "wizard".into() };
    let layout = create_default_layout(&["w1".into()], "row", None, Some(&["wizard".into()]));
    let shell = shell(&mut tui.scene, navbar, footer, &layout);

    let mut dash = Dashboard {
        tree,
        layout,
        shell,
        windows: Vec::new(),
        next_serial: 2,
        focused: "w1".into(),
        leader: LeaderMode::Idle,
        terminal_input: false,
    };
    let (w1_id, w1_chrome) = dash.shell.windows[0].clone();
    let w1 = dash.attach_wizard(&mut tui, w1_chrome, &w1_id);
    dash.windows.push(w1);
    dash.remount(&mut tui);
    tui.set_focus(Some(dash.windows[0].focus));
    dash.sync_chrome_focus(&mut tui);
    term.present(&tui.render_full()).ok();

    loop {
        dash.poll_pty(&mut tui);
        let events = term.poll(std::time::Duration::from_millis(80)).unwrap_or_default();
        let mut quit = false;
        let mut need_paint = !events.is_empty();

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
                                    push_window_to_stack(
                                        &mut dash.layout,
                                        &win_id,
                                        WindowLayoutWindowNode { window_kind_id: new_id.clone(), title: Some("wizard".into()), corner: None },
                                    );
                                    let w = dash.add_wizard_window(&mut tui, new_id.clone(), "wizard");
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
                                _ => {}
                            }
                        }
                    }
                }
                Event::Key(k) => {
                    if k.key == Key::Char(' ') && k.mods & mods::CTRL != 0 {
                        dash.leader = LeaderMode::Armed;
                        need_paint = true;
                        continue;
                    }
                    if matches!(dash.leader, LeaderMode::Armed) {
                        let consumed = match k.key {
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
                                let w = dash.add_wizard_window(&mut tui, new_id.clone(), "wizard");
                                dash.windows.push(w);
                                dash.focused = new_id;
                                dash.remount(&mut tui);
                                dash.leader = LeaderMode::Idle;
                                true
                            }
                            Key::Char('|') => {
                                let new_id = format!("w{}", dash.next_serial);
                                dash.next_serial += 1;
                                let _ = split_window(&mut dash.layout, &dash.focused, "row", &new_id, Some("shell".into()));
                                let w = dash.add_wizard_window(&mut tui, new_id.clone(), "wizard");
                                dash.windows.push(w);
                                dash.focused = new_id;
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
                                let toggle = dash.focused_window().and_then(|w| {
                                    if matches!(w.body, WindowBody::Output { .. }) {
                                        Some(w.focus)
                                    } else {
                                        None
                                    }
                                });
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
                                push_window_to_stack(
                                    &mut dash.layout,
                                    &dash.focused,
                                    WindowLayoutWindowNode { window_kind_id: new_id.clone(), title: Some("wizard".into()), corner: None },
                                );
                                let w = dash.add_wizard_window(&mut tui, new_id.clone(), "wizard");
                                dash.windows.push(w);
                                dash.focused = new_id;
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

                    if k.key == Key::Char('q') && k.mods == 0 {
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
                        let next = if k.key == Key::Tab {
                            (idx + 1) % order.len()
                        } else {
                            (idx + order.len() - 1) % order.len()
                        };
                        dash.focused = order[next].clone();
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
                                    if let Some(win) = dash.windows.iter_mut().find(|w| w.id == fid) {
                                        if let WindowBody::Output { session, .. } = &mut win.body {
                                            if let (Some(bytes), Some(s)) = (key_to_pty_bytes(k), session.as_mut()) {
                                                let _ = s.pty.write_all(&bytes);
                                            }
                                        }
                                    }
                                }
                            }
                            need_paint = true;
                            continue;
                        }
                        if let WindowBody::Wizard { widget, .. } = &w.body {
                            tui.set_focus(Some(*widget));
                            let key_ev = match (k.key, k.mods) {
                                (Key::Char('j'), 0) => KeyEvent { key: Key::Down, mods: 0 },
                                (Key::Char('k'), 0) => KeyEvent { key: Key::Up, mods: 0 },
                                (Key::Char('h'), 0) => KeyEvent { key: Key::Left, mods: 0 },
                                (Key::Char('l'), 0) => KeyEvent { key: Key::Right, mods: 0 },
                                _ => *k,
                            };
                            if let Some(widget_state) = tui.scene.node_mut(*widget).widget() {
                                if let Some(signal) = widget_state.on_key(&key_ev) {
                                    dash.handle_wizard_signal(&mut tui, &fid, signal);
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
                f.status = format!("{} windows", dash.windows.len());
            }
        }
        if need_paint {
            term.present(&tui.render_full()).ok();
        }
    }

    while let Some(mut win) = dash.windows.pop() {
        Dashboard::kill_session(&mut win);
    }
    let _ = term.leave();
    0
}
// #endregion 🔖️Run
