//! 📡️ The dashboard's conversation with the workspace daemon and with command discovery: starts go
//! out as resolved launches only, session changes come in and become windows, tabs and titles, and
//! the registry arrives in the background.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌀️daemon/✉️ipc/🦀️.rs
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/🦀️.rs

use super::launcher::Start;
use super::windows::{Body, Dashboard, Kind, LinkState, Place};
use crate::daemon::client::{Connection, Message};
use crate::inventory::Update;
use crate::ipc::{ClientMsg, ServerMsg, SessionCommand, SessionInfo, SessionStatus, SpawnGroup};
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use ui_tui::tui::engine::Tui;
use ui_tui::tui::geometry::Size;
use ui_tui::tui::layout::activate_stack_tab;
use ui_tui::tui::widget::WidgetState;

const PENDING_LIMIT: usize = 128;
const SHUTDOWN_PATIENCE: Duration = Duration::from_secs(5);
/// 🧹 Full reset plus cleared history: what a terminal needs before the daemon replays its screen.
const RESET: &[u8] = &[0x1b, b'c', 0x1b, b'[', b'3', b'J'];
const RECONNECT_FIRST: Duration = Duration::from_millis(500);
const RECONNECT_LAST: Duration = Duration::from_secs(8);

/// 🔗️ The connection to the daemon and the work waiting for it.
pub(super) struct Link {
    pub connection: Option<Connection>,
    connecting: Option<std::sync::mpsc::Receiver<std::io::Result<Connection>>>,
    next_reconnect: Instant,
    pub pending: VecDeque<ClientMsg>,
    pub restoring: bool,
    pub enabled: bool,
    pub outbox: Vec<ClientMsg>,
    backoff: Duration,
    #[cfg(test)]
    pub online_for_test: bool,
    #[cfg(test)]
    pub fail_sends: bool,
}

impl Default for Link {
    fn default() -> Self { Self { connection: None, connecting: None, next_reconnect: Instant::now(), pending: VecDeque::new(), restoring: true, enabled: true, outbox: Vec::new(), backoff: RECONNECT_FIRST, #[cfg(test)] online_for_test: false, #[cfg(test)] fail_sends: false } }
}

impl Link {
    fn pretend_online(&self) -> bool { #[cfg(test)] { self.online_for_test } #[cfg(not(test))] { false } }

    fn fails(&self) -> bool { #[cfg(test)] { self.fail_sends } #[cfg(not(test))] { false } }

    /// ⏳️ Schedules the next connection attempt: the wait doubles after every failure up to a ceiling and starts over after a success.
    pub fn schedule_reconnect(&mut self, failed: bool, now: Instant) {
        self.backoff = if failed { (self.backoff * 2).min(RECONNECT_LAST) } else { RECONNECT_FIRST };
        self.next_reconnect = now + if failed { self.backoff } else { RECONNECT_FIRST };
    }

    #[cfg(test)]
    pub fn next_attempt(&self) -> Instant { self.next_reconnect }

    #[cfg(test)]
    pub fn backoff(&self) -> Duration { self.backoff }

    /// 🧪️ A link that never reaches for a daemon: starts queue and every other message lands in the outbox.
    #[cfg(test)]
    pub fn offline() -> Self { Self { enabled: false, restoring: false, ..Self::default() } }
}

/// 📣️ What polling found.
#[derive(Default)]
pub(super) struct Polled { pub changed: bool, pub quit: bool }



fn without_graph_reuse(command: &SessionCommand) -> SessionCommand {
    let mut command = command.clone();
    command.env.retain(|(key, _)| key != crate::registry::GRAPH_REUSE.0);
    command
}


// #region 🔖️Starts
impl Dashboard {
    /// 🔌️ Whether a message can be handed to a connection right now.
    pub fn online(&self) -> bool { self.link.connection.is_some() || self.link.pretend_online() }

    pub fn send(&mut self, message: &ClientMsg) -> std::io::Result<()> {
        if !self.link.enabled {
            if self.link.fails() { return Err(std::io::Error::other("dashboard daemon connection unavailable")); }
            self.link.outbox.push(message.clone());
            return Ok(());
        }
        self.link.connection.as_mut().ok_or_else(|| std::io::Error::other("dashboard daemon disconnected"))?.send(message)
    }

    /// 🚀️ Resolves a launcher request through the registry and starts it: the launcher window becomes the
    /// terminal of the primary process; services it requires and the other members of a compound follow
    /// as windows of their own once the daemon announces them.
    pub fn start(&mut self, tui: &mut Tui, index: usize, request: &Start) -> Result<(), String> {
        let registry = self.registry.clone().ok_or_else(|| self.text().start_unavailable.as_str().to_string())?;
        let launch = registry.resolve(&request.id, &request.request())?;
        let mut group = SpawnGroup::from_launch(&launch, &[]);
        let Some(primary) = group.members.first() else { return Err(format!("{} starts nothing", request.id)) };
        if let Some(live) = self.sessions.values().find(|session| matches!(session.status, SessionStatus::Running | SessionStatus::Stopping) && without_graph_reuse(&session.command).same_task(&without_graph_reuse(&primary.command))).cloned() {
            self.reveal_session(tui, &live.session_id);
            return Ok(());
        }
        tui.layout();
        let rect = tui.scene.rect(self.windows[index].root);
        let size = if rect.width > 1 && rect.height > 1 { Size { width: rect.width, height: rect.height } } else { Size { width: 80, height: 24 } };
        for member in group.requires.iter_mut().chain(group.members.iter_mut()) { (member.command.cols, member.command.rows) = (size.width, size.height); }
        group.requires.retain(|member| !self.sessions.values().any(|session| session.status.live() && session.command.command_id == member.command.command_id));
        if self.link.pending.len() >= PENDING_LIMIT { return Err(self.text().start_pending_full.as_str().to_string()); }
        let primary_id = group.members[0].session_id.clone();
        let info = SessionInfo { session_id: primary_id.clone(), command: group.members[0].command.clone(), status: SessionStatus::Pending, pid: None, code: None, ..Default::default() };
        self.set_body(tui, index, Kind::Output, Some(size));
        self.sessions.insert(primary_id, info.clone());
        if let Body::Output { session, sent } = &mut self.windows[index].body { *session = Some(Box::new(info)); *sent = size; }
        let message = group.message();
        if self.online() {
            if self.send(&message).is_err() {
                self.link.pending.push_back(message);
                self.notice = Some(self.text().start_requeued.as_str().to_string());
            }
        } else {
            self.link.pending.push_back(message);
            self.notice = Some(self.text().start_waiting.fill(&[("cancel", &self.keymap.keys_label(crate::preferences::keymap::Scope::Prefix, "stop-task").unwrap_or_default())]).into_string());
            let line = self.text().pane_waiting.as_str().to_string();
            self.feed(tui, index, &line);
        }
        self.remount(tui);
        Ok(())
    }

    /// 🖨️ Writes a line into the terminal of a window.
    pub fn feed(&mut self, tui: &mut Tui, index: usize, line: &str) { self.feed_output(tui, index, format!("{line}\r\n").as_bytes()); }

    /// 📥️ Feeds child output into the terminal of a window; replayed history is not answered, live output is.
    pub fn feed_output(&mut self, tui: &mut Tui, index: usize, data: &[u8]) {
        let Some(node) = self.terminal_node(index) else { return };
        let replay = self.link.restoring;
        let signals = match tui.scene.node_mut(node).widget() {
            Some(WidgetState::Terminal(terminal)) if replay => { terminal.feed_replay(data); Vec::new() }
            Some(WidgetState::Terminal(terminal)) => terminal.feed(data),
            _ => Vec::new(),
        };
        if !signals.is_empty() { self.on_signals(tui, signals.into_iter().map(|signal| (node, signal)).collect()); }
    }

    /// ✋️ Withdraws a start that never reached the daemon.
    pub fn cancel_pending_start(&mut self, tui: &mut Tui, session_id: &str) -> bool {
        let Some(position) = self.link.pending.iter().position(|message| matches!(message, ClientMsg::SpawnGroup { members, requires, .. } if members.iter().chain(requires).any(|member| member.session_id == session_id))) else { return false };
        self.link.pending.remove(position);
        if let Some(mut session) = self.sessions.get(session_id).cloned() { session.status = SessionStatus::Exited; session.code = Some(130); self.update_session(tui, session); }
        true
    }

    /// 🎛️ Restarts, stops or kills the task of a window.
    pub fn control_session(&mut self, tui: &mut Tui, index: usize, action: &str) {
        let Some(session_id) = self.windows.get(index).and_then(|window| window.session()).map(|session| session.session_id.clone()) else { return };
        if action != "restart-task" && self.cancel_pending_start(tui, &session_id) { return; }
        let message = match action { "restart-task" => ClientMsg::Restart { session_id }, "stop-task" => ClientMsg::Stop { session_id }, _ => ClientMsg::Kill { session_id } };
        if let Err(error) = self.send(&message) { self.notice = Some(error.to_string()); }
    }

    /// ⌨️ Sends terminal input of a window to its session.
    pub fn send_input(&mut self, session_id: &str, data: Vec<u8>) {
        if !self.link.enabled { self.link.outbox.push(ClientMsg::Input { session_id: session_id.to_string(), data }); return; }
        let result = match self.link.connection.as_mut() { Some(connection) => connection.input(session_id, &data), None => Err(std::io::Error::other(self.text().state_reconnecting.as_str())) };
        if let Err(error) = result { self.notice = Some(error.to_string()); }
    }

    /// 🛑️ Asks the daemon to stop; the view leaves when it confirms, never earlier.
    pub fn begin_shutdown(&mut self) {
        match self.send(&ClientMsg::Shutdown {}) {
            Ok(()) => self.shutdown_since = Some(Instant::now()),
            Err(error) => self.notice = Some(error.to_string()),
        }
    }
}
// #endregion 🔖️Starts

// #region 🔖️Sessions
impl Dashboard {
    /// 🔁️ Re-attaches so the daemon replays every session's screen.
    pub fn restore_sessions(&mut self) {
        self.link.restoring = true;
        if let Err(error) = self.send(&ClientMsg::Attach { client_id: format!("dashboard-{}", std::process::id()) }) { self.notice = Some(error.to_string()); }
    }

    /// 🔝️ Focuses the most recently started visible task after the requested snapshot completes.
    pub fn focus_newest_session(&mut self, tui: &mut Tui) {
        let newest = self.sessions.values().max_by_key(|session| (session.started_ms, &session.session_id)).map(|session| session.session_id.clone());
        if let Some(target) = newest.and_then(|id| self.windows.iter().position(|window| window.session().is_some_and(|session| session.session_id == id))) { self.focus_window(tui, target); }
    }

    /// 👁️ Shows a task's window again and brings it forward.
    pub fn reveal_session(&mut self, tui: &mut Tui, session_id: &str) {
        self.hidden.remove(session_id);
        if let Some(info) = self.sessions.get(session_id).cloned() { self.update_session(tui, info); }
        if let Some(index) = self.windows.iter().position(|window| window.session().is_some_and(|session| session.session_id == session_id)) {
            self.focus_window(tui, index);
            self.restore_sessions();
        }
    }

    /// 🪟️ Projects one session change onto its window, creating the window for a task seen first.
    pub fn update_session(&mut self, tui: &mut Tui, info: SessionInfo) {
        self.sessions.insert(info.session_id.clone(), info.clone());
        if self.hidden.contains(&info.session_id) { return; }
        let existing = self.windows.iter().position(|window| window.session().is_some_and(|session| session.session_id == info.session_id));
        let index = match existing {
            Some(index) => index,
            None => {
                let focused = self.windows.get(self.focused_index(tui)).map(|window| window.id.clone());
                let index = self.create_window(tui, Kind::Output, Place::Preference);
                if let Some(focused) = focused { activate_stack_tab(&mut self.layout, &focused); }
                self.remount(tui);
                index
            }
        };
        if let Body::Output { session, .. } = &mut self.windows[index].body { *session = Some(Box::new(info)); }
        self.remount(tui);
        self.refresh_panes(tui);
    }

    pub fn apply(&mut self, tui: &mut Tui, message: Message) -> bool {
        match message {
            Message::Control(ServerMsg::Sessions { sessions, more }) => {
                for session in sessions { self.update_session(tui, session); }
                if !more && std::mem::take(&mut self.restore_focus) {
                    self.focus_newest_session(tui);
                }
            }
            Message::Control(ServerMsg::ReplayStart { session_id, truncated }) => {
                self.link.restoring = true;
                if let Some(index) = self.windows.iter().position(|window| window.session().is_some_and(|session| session.session_id == session_id)) { self.reset_terminal(tui, index); }
                if truncated { self.notice = Some(self.text().replay_truncated.as_str().into()); }
            }
            Message::Control(ServerMsg::SessionChanged { session }) => self.update_session(tui, *session),
            Message::Control(ServerMsg::SessionRemoved { session_id }) => { self.sessions.remove(&session_id); self.refresh_panes(tui); }
            Message::Output { session_id, data } => {
                let found = self.windows.iter().position(|window| window.session().is_some_and(|session| session.session_id == session_id));
                if let Some(index) = found { self.feed_output(tui, index, &data); }
            }
            Message::Control(ServerMsg::ReplayComplete { session_id: None }) => {
                self.link.restoring = false;
                for window in &mut self.windows { if let Body::Output { sent, .. } = &mut window.body { *sent = Size::default(); } }
            }
            Message::Control(ServerMsg::ReplayComplete { session_id: Some(_) }) => {}
            Message::Control(ServerMsg::Error { message, code: Some(crate::ipc::ErrorCode::ViewLimit), .. }) => {
                let text = self.text().error(Some(crate::ipc::ErrorCode::ViewLimit), &message);
                self.link.connection = None;
                self.link.schedule_reconnect(false, Instant::now());
                self.state = LinkState::Failed(text.clone());
                self.notice = Some(text);
            }
            Message::Control(ServerMsg::Error { message, code, .. }) => self.notice = Some(self.text().error(code, &message)),
            Message::Disconnected(message) => { self.link.connection = None; self.link.schedule_reconnect(false, Instant::now()); self.state = if self.shutdown_since.is_some() { LinkState::Stopped } else { LinkState::Failed(message) }; return self.shutdown_since.is_some(); }
            Message::Control(ServerMsg::Shutdown {}) => { self.link.connection = None; self.state = LinkState::Stopped; return self.shutdown_since.is_some(); }
            Message::Control(_) => {}
        }
        false
    }

    fn reset_terminal(&mut self, tui: &mut Tui, index: usize) {
        if let Some(WidgetState::Terminal(terminal)) = tui.scene.node_mut(self.windows[index].list).widget() { terminal.feed_replay(RESET); }
    }

    /// 🔄️ Connects in the background, flushes waiting starts and applies what the daemon sent.
    pub fn poll_daemon(&mut self, tui: &mut Tui) -> Polled {
        let mut polled = Polled::default();
        if self.shutdown_since.is_some_and(|since| since.elapsed() > SHUTDOWN_PATIENCE) {
            self.shutdown_since = None;
            self.notice = Some(self.text().state_shutdown_timeout.as_str().into());
            polled.changed = true;
        }
        if !self.link.enabled { return polled; }
        if self.link.connection.is_none() && self.link.connecting.is_none() && Instant::now() >= self.link.next_reconnect {
            let (sender, receiver) = std::sync::mpsc::sync_channel(1);
            let root = self.root.clone();
            let text = self.text();
            std::thread::spawn(move || {
                let result = Connection::connect(&root).or_else(|_| {
                    let executable = std::env::current_exe()?;
                    crate::daemon::supervisor::spawn_daemon(&root, &executable).map_err(|error| std::io::Error::other(format!("{}: {error}", text.cli_daemon_startup_failed.as_str())))?;
                    Connection::connect(&root)
                });
                let _ = sender.send(result);
            });
            self.link.connecting = Some(receiver);
        }
        if let Some(result) = self.link.connecting.as_ref().and_then(|receiver| receiver.try_recv().ok()) {
            self.link.connecting = None;
            match result {
                Ok(mut connection) => {
                    let skew = connection.skew();
                    if skew.as_ref().is_some_and(crate::daemon::client::Skew::incompatible) {
                        self.state = LinkState::Failed(skew.map(|skew| self.text().skew(&skew)).unwrap_or_default());
                        self.link.schedule_reconnect(true, Instant::now());
                        polled.changed = true;
                        return polled;
                    }
                    if let Some(skew) = skew { self.notice = Some(self.text().skew(&skew)); }
                    if let Some(waker) = self.waker.clone() { connection.set_notifier(std::sync::Arc::new(move || waker.wake())); }
                    let _ = connection.send(&ClientMsg::Attach { client_id: format!("dashboard-{}", std::process::id()) });
                    self.link.connection = Some(connection);
                    self.link.schedule_reconnect(false, Instant::now());
                    self.link.restoring = true;
                    self.state = LinkState::Connected;
                }
                Err(error) => { self.state = LinkState::Failed(error.to_string()); self.link.schedule_reconnect(true, Instant::now()); }
            }
            polled.changed = true;
        }
        let Some(connection) = self.link.connection.as_mut() else { return polled };
        while let Some(request) = self.link.pending.front() {
            if let Err(error) = connection.send(request) { self.notice = Some(error.to_string()); break; }
            self.link.pending.pop_front();
            polled.changed = true;
        }
        let messages = match connection.receive(Duration::ZERO) {
            Ok(messages) => messages,
            Err(_) => { self.link.connection = None; self.link.schedule_reconnect(false, Instant::now()); self.state = LinkState::Reconnecting; polled.changed = true; return polled; }
        };
        polled.changed |= !messages.is_empty();
        for message in messages { polled.quit |= self.apply(tui, message); }
        polled
    }
}
// #endregion 🔖️Sessions

// #region 🔖️Discovery
impl Dashboard {
    pub fn refresh_inventory(&mut self) {
        self.inventory = Some(crate::inventory::start(&self.root, true));
        self.notice = Some(self.text().state_discovering.as_str().into());
    }

    pub fn cancel_inventory(&mut self) {
        self.inventory = None;
        self.notice = Some(self.text().state_discovery_cancelled.as_str().into());
    }

    /// 📚️ Adopts a newer registry in the dashboard and in every launcher, whose trees keep their open folders, filter and selection.
    pub fn set_registry(&mut self, tui: &mut Tui, registry: std::sync::Arc<crate::registry::Registry>) {
        for window in &mut self.windows {
            let (Body::Launcher(launcher), Some(node)) = (&mut window.body, window.tree) else { continue };
            launcher.set_registry(registry.clone());
            let mut found = tui.scene.node_mut(node);
            if let Some(WidgetState::Tree(tree)) = found.widget() { *tree = launcher.rebuild_tree(Some(tree)); }
        }
        self.registry = Some(registry);
    }

    /// 🔃️ Takes what discovery and the preference writer have finished since the last poll.
    pub fn poll_background(&mut self, tui: &mut Tui) -> bool {
        let mut changed = false;
        if let Some((change, result)) = self.saving_preferences.as_ref().and_then(|receiver| receiver.try_recv().ok()) {
            self.saving_preferences = None;
            match result { Ok(_) => self.adopt_preference(tui, &change), Err(error) => self.notice = Some(error.to_string()) }
            changed = true;
        }
        let updates: Vec<Update> = self.inventory.as_ref().map(|job| job.receiver.try_iter().collect()).unwrap_or_default();
        for update in updates {
            match update {
                Update::Ready(..) => { if let Some(registry) = self.inventory.as_ref().and_then(crate::inventory::Job::registry) { self.set_registry(tui, registry); } }
                Update::Finished => self.inventory = None,
                Update::Failed(error) => self.notice = Some(error),
            }
            changed = true;
        }
        if let Some(job) = &self.inventory {
            self.phase = Some(job.progress.phase());
            let elapsed = job.started.elapsed().as_secs();
            if elapsed != self.last_progress { self.last_progress = elapsed; changed = true; }
        }
        changed
    }
}
// #endregion 🔖️Discovery
