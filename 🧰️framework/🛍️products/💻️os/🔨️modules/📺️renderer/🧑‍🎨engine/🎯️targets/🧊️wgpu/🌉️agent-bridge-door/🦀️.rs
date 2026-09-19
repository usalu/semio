//! 🌉️ The MCP agent bridge's TRANSPORT — the socket React's `useAgentBridge` dials with
//! `new WebSocket(admittedConfig.url, [...bridgeProtocols(admittedConfig)])` and the wgpu shell had
//! none of, on EITHER target (`📓️w1j-shell-overlays.md` gap 2, `📓️audit-w14-transport-residual.md`
//! row 3).
//!
//! ⚖️ What was already there and what is added: `🧱️elements/🔗️AgentBridge`'s `AgentBridgeState` owns
//! the frame codec and the whole consumer (presence, approvals, conversation) and is transport-free
//! by design; `AgentBridgeDialer` owns WHEN to dial/announce/ping/back off, and is equally
//! transport-free. This file owns the remaining third: the bytes. Both halves below drive the SAME
//! `SocketLane` (`🔌️socket-door/🦀️.rs`), so the bounded-queue and loss-reporting laws hold
//! identically on the browser and native builds.
//!
//! 🌐️ Browser: the page owns the socket (`🚪️host-io/🟦️.ts`'s duplex door). The frame Worker has no
//! `window` — `WebSocket` itself would construct there, but the shell's whole browser I/O surface is
//! ONE page-owned door, which is what keeps a single place to audit every origin this shell talks to
//! and what lets the page-mounted variant of the renderer answer identically.
//!
//! 🖥️ Native: one `tokio-tungstenite` connection on the shared pool's `Lane::Io`, dialled through
//! `os_directory::client::native::open_client_ws` — the SAME dial the directory stream uses, with
//! the directory's own grant policy lifted out of it rather than copied. The socket never touches
//! the render thread: the I/O task owns it and the shell only ever reads the bounded lane.

use crate::agent_bridge::{AgentBridgeConfig, AgentBridgeDialTurn, AgentBridgeDialer, AgentBridgeSocketState, AgentBridgeState};
use crate::socket_door::SocketMessage;

//#region 🔖️Bounds
/// ⏱️ How long a native dial may take before it is abandoned and the ladder backs off. The browser
/// half has no twin: the page's own socket carries the browser's connect timeout.
pub const AGENT_BRIDGE_DIAL_TIMEOUT_MS: u64 = 5_000;

/// 📥️ How many inbound frames ONE pump turn applies to the consumer. A bounded drain, for the same
/// reason every other shell lane is bounded: one busy agent must not spend a whole frame budget.
pub const AGENT_BRIDGE_PUMP_MAX_FRAMES: usize = 16;
//#endregion 🔖️Bounds

//#region 🔖️Transport
/// 🌉️ The one socket this shell holds for the gateway, on whichever target it was built for.
#[derive(Default)]
pub struct AgentBridgeTransport {
    pub dialer: AgentBridgeDialer,
    socket: Option<AgentBridgeSocket>,
    /// 📉️ Frames lost to the bounded lanes since the last report — surfaced into the consumer's
    /// `last_error` so a dropped turn is visible instead of silently missing from the transcript.
    reported_drops: u32,
}

impl AgentBridgeTransport {
    pub fn new() -> Self {
        Self::default()
    }

    /// 🔗️ Installs the supervisor's config; `None` retires the socket and parks the bridge back in
    /// `Disabled`, React's own behaviour for a config that goes away.
    pub fn set_config(&mut self, config: Option<AgentBridgeConfig>, state: &mut AgentBridgeState) -> bool {
        if !self.dialer.set_config(config, state) {
            return false;
        }
        self.retire();
        true
    }

    pub fn is_armed(&self) -> bool {
        self.dialer.is_armed()
    }

    /// 🔌️ What the transport holds, as the dialer needs to see it.
    fn socket_state(&self) -> AgentBridgeSocketState {
        match &self.socket {
            None => AgentBridgeSocketState::Absent,
            Some(socket) => socket.state(),
        }
    }

    fn retire(&mut self) {
        if let Some(mut socket) = self.socket.take() {
            socket.close();
        }
    }

    /// 🎬️ One bounded turn: advance the ladder, then move whatever bytes are ready in both
    /// directions. Answers whether anything the chrome renders changed.
    pub fn pump(&mut self, state: &mut AgentBridgeState, shell_session_id: &str, principal_actor: &str, now_ms: f64) -> bool {
        let mut changed = false;
        match self.dialer.turn(state, self.socket_state(), now_ms) {
            AgentBridgeDialTurn::Idle | AgentBridgeDialTurn::Wait { .. } => {}
            AgentBridgeDialTurn::Retire => {
                self.drain_inbound(state, now_ms);
                self.retire();
                return true;
            }
            AgentBridgeDialTurn::Dial { url, protocols } => {
                match AgentBridgeSocket::open(&url, &protocols) {
                    Ok(socket) => self.socket = Some(socket),
                    Err(error) => {
                        state.last_error = Some(error);
                        // 🧯️ A refused dial is a closed socket as far as the ladder is concerned, so
                        // the next turn arms the backoff rather than hammering the gateway.
                        state.note_socket_closed();
                    }
                }
                return true;
            }
            AgentBridgeDialTurn::Announce => {
                state.note_socket_opened(shell_session_id, principal_actor, Default::default());
                changed = true;
            }
            AgentBridgeDialTurn::Ping => state.queue_ping(),
        }
        changed |= self.flush_outbox(state);
        changed |= self.drain_inbound(state, now_ms);
        changed
    }

    /// 📤️ Writes everything the consumer queued. A frame that cannot be written is recorded rather
    /// than dropped silently — the gateway never learns of it either way, but the human does.
    fn flush_outbox(&mut self, state: &mut AgentBridgeState) -> bool {
        let frames = state.take_outbox_encoded();
        if frames.is_empty() {
            return false;
        }
        let Some(socket) = self.socket.as_mut() else {
            state.last_error = Some("agent bridge has no socket to write to".to_string());
            return true;
        };
        for frame in frames {
            if let Err(error) = socket.send(SocketMessage::Binary(frame)) {
                state.last_error = Some(error);
            }
        }
        true
    }

    /// 📥️ Applies at most [`AGENT_BRIDGE_PUMP_MAX_FRAMES`] inbound frames to the consumer and
    /// surfaces any lane loss. A TEXT frame on this endpoint is off-contract (the bridge wire is
    /// binary) and is counted, never guessed at.
    fn drain_inbound(&mut self, state: &mut AgentBridgeState, now_ms: f64) -> bool {
        let Some(socket) = self.socket.as_mut() else { return false };
        let mut changed = false;
        for message in socket.drain(AGENT_BRIDGE_PUMP_MAX_FRAMES) {
            match message {
                SocketMessage::Binary(bytes) => {
                    let _ = state.apply_encoded_frame(&bytes, now_ms);
                    changed = true;
                }
                SocketMessage::Text(_) => {
                    state.last_error = Some("agent bridge received a text frame on a binary wire".to_string());
                    changed = true;
                }
            }
        }
        let dropped = socket.dropped();
        if dropped > self.reported_drops {
            state.last_error = Some(format!("agent bridge dropped {} frame(s) to its bounded lane", dropped - self.reported_drops));
            self.reported_drops = dropped;
            changed = true;
        }
        changed
    }
}
//#endregion 🔖️Transport

//#region 🌐️BrowserSocket
/// 🌐️ The browser socket: the page owns it, the lane pages it.
#[cfg(target_arch = "wasm32")]
struct AgentBridgeSocket {
    socket: crate::socket_door::browser::BrowserDoorSocket,
}

#[cfg(target_arch = "wasm32")]
impl AgentBridgeSocket {
    fn open(url: &str, protocols: &[String; 2]) -> Result<Self, String> {
        crate::socket_door::browser::BrowserDoorSocket::open(url, protocols).map(|socket| Self { socket })
    }

    fn state(&self) -> AgentBridgeSocketState {
        let lane = self.socket.lane();
        if lane.is_closed() {
            AgentBridgeSocketState::Closed
        } else if lane.is_open() {
            AgentBridgeSocketState::Open
        } else {
            AgentBridgeSocketState::Connecting
        }
    }

    fn send(&mut self, message: SocketMessage) -> Result<(), String> {
        self.socket.send(message)
    }

    fn drain(&mut self, limit: usize) -> Vec<SocketMessage> {
        let mut page = self.socket.drain();
        page.truncate(limit);
        page
    }

    fn dropped(&self) -> u32 {
        self.socket.lane().dropped()
    }

    fn close(&mut self) {
        self.socket.close();
    }
}
//#endregion 🌐️BrowserSocket

//#region 🖥️NativeSocket
/// 🖥️ The native socket: ONE `Lane::Io` task owns the `tokio-tungstenite` connection and the shell
/// only ever touches the shared bounded lane, so no socket read ever happens on the render thread.
#[cfg(not(target_arch = "wasm32"))]
struct AgentBridgeSocket {
    lane: std::sync::Arc<std::sync::Mutex<crate::socket_door::SocketLane>>,
    cancel: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

/// ⏱️ How long an idle native socket task waits before its next bounded read turn. A re-armed timer
/// rather than a spin: an agent that says nothing for a minute must cost nothing.
#[cfg(not(target_arch = "wasm32"))]
const AGENT_BRIDGE_IDLE_POLL_MS: u64 = 25;

#[cfg(not(target_arch = "wasm32"))]
type AgentBridgeLaneHandle = std::sync::Arc<std::sync::Mutex<crate::socket_door::SocketLane>>;

#[cfg(not(target_arch = "wasm32"))]
type AgentBridgeCancel = std::sync::Arc<std::sync::atomic::AtomicBool>;

/// 🔁️ ONE bounded turn of the native socket: write whatever the shell queued, read at most a page,
/// then re-arm itself — immediately when it did work, on the idle timer when it did not. Never a
/// loop inside one job, so an `Io` worker is never held by a quiet socket.
#[cfg(not(target_arch = "wasm32"))]
fn drive_native_agent_socket(pool: semio_framework_async::WorkerPool, lane: AgentBridgeLaneHandle, cancel: AgentBridgeCancel, mut connection: semio_framework_os_kernel::os_directory::client::native::TungsteniteConnection) {
    use semio_framework_os_kernel::os_directory::client::native::NativeWsPoll;
    use semio_framework_os_kernel::os_directory::client::DirectoryWsConnection;
    if cancel.load(std::sync::atomic::Ordering::Relaxed) {
        connection.close();
        lane.lock().expect("agent bridge lane poisoned").note_closed(None);
        return;
    }
    let outbound = lane.lock().expect("agent bridge lane poisoned").take_outbound_page();
    let mut worked = !outbound.is_empty();
    for message in outbound {
        let written = match message {
            SocketMessage::Binary(bytes) => connection.send_binary(bytes),
            SocketMessage::Text(text) => connection.send_text(text),
        };
        if let Err(error) = written {
            let mut lane = lane.lock().expect("agent bridge lane poisoned");
            lane.note_error(error.to_string());
            lane.note_closed(None);
            return;
        }
    }
    for _ in 0..AGENT_BRIDGE_PUMP_MAX_FRAMES {
        match connection.try_recv_any() {
            Ok(NativeWsPoll::Binary(bytes)) => {
                worked = true;
                lane.lock().expect("agent bridge lane poisoned").push_inbound(SocketMessage::Binary(bytes));
            }
            Ok(NativeWsPoll::Text(text)) => {
                worked = true;
                lane.lock().expect("agent bridge lane poisoned").push_inbound(SocketMessage::Text(text));
            }
            Ok(NativeWsPoll::Pending) => break,
            Ok(NativeWsPoll::Closed(code)) => {
                lane.lock().expect("agent bridge lane poisoned").note_closed(code);
                return;
            }
            Err(error) => {
                let mut guard = lane.lock().expect("agent bridge lane poisoned");
                guard.note_error(error.to_string());
                guard.note_closed(None);
                return;
            }
        }
    }
    let next = pool.clone();
    let job = Box::new(move || drive_native_agent_socket(next, lane, cancel, connection));
    if worked {
        pool.submit(semio_framework_async::Lane::Io, job);
    } else {
        pool.submit_at(pool.now_ms().saturating_add(AGENT_BRIDGE_IDLE_POLL_MS), semio_framework_async::Lane::Io, job);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl AgentBridgeSocket {
    fn open(url: &str, protocols: &[String; 2]) -> Result<Self, String> {
        use semio_framework_async::{CancelToken, Lane, OperationContext, TraceId};
        let lane: AgentBridgeLaneHandle = std::sync::Arc::new(std::sync::Mutex::new(crate::socket_door::SocketLane::new()));
        let cancel: AgentBridgeCancel = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let (task_lane, task_cancel) = (lane.clone(), cancel.clone());
        let (url, protocols) = (url.to_string(), protocols.to_vec());
        let pool = crate::renderer_worker_pool();
        let dial_pool = pool.clone();
        // 🔌️ The dial itself BLOCKS (one bounded `connect_timeout` per resolved address), so it runs
        // on `Lane::Io` and nowhere near the render thread; the driver re-arms from inside it.
        pool.submit(
            Lane::Io,
            Box::new(move || {
                let ctx = OperationContext { actor: 0, capability: None, cancel: CancelToken::root_now(), deadline_ms: None, generation: 0, lane: 0, trace: TraceId(0) };
                match semio_framework_os_kernel::os_directory::client::native::open_client_ws(&ctx, &url, &protocols, AGENT_BRIDGE_DIAL_TIMEOUT_MS, Some(crate::agent_bridge::BRIDGE_SUBPROTOCOL)) {
                    Ok(connection) => {
                        task_lane.lock().expect("agent bridge lane poisoned").note_open();
                        drive_native_agent_socket(dial_pool, task_lane, task_cancel, connection);
                    }
                    Err(error) => {
                        let mut guard = task_lane.lock().expect("agent bridge lane poisoned");
                        guard.note_error(error.to_string());
                        guard.note_closed(None);
                    }
                }
            }),
        );
        Ok(Self { cancel, lane })
    }

    fn with_lane<T>(&self, read: impl FnOnce(&mut crate::socket_door::SocketLane) -> T) -> T {
        let mut lane = self.lane.lock().expect("agent bridge lane poisoned");
        read(&mut lane)
    }

    fn state(&self) -> AgentBridgeSocketState {
        self.with_lane(|lane| {
            if lane.is_closed() {
                AgentBridgeSocketState::Closed
            } else if lane.is_open() {
                AgentBridgeSocketState::Open
            } else {
                AgentBridgeSocketState::Connecting
            }
        })
    }

    fn send(&mut self, message: SocketMessage) -> Result<(), String> {
        let accepted = self.with_lane(|lane| lane.push_outbound(message));
        if accepted {
            Ok(())
        } else {
            Err(self.with_lane(|lane| lane.last_error().unwrap_or("agent bridge lane refused the frame").to_string()))
        }
    }

    fn drain(&mut self, limit: usize) -> Vec<SocketMessage> {
        self.with_lane(|lane| {
            let mut page = lane.take_inbound_page();
            page.truncate(limit);
            page
        })
    }

    fn dropped(&self) -> u32 {
        self.with_lane(|lane| lane.dropped())
    }

    fn close(&mut self) {
        self.cancel.store(true, std::sync::atomic::Ordering::Relaxed);
        self.with_lane(|lane| lane.note_closed(None));
    }
}
//#endregion 🖥️NativeSocket

// 🧪️ This module's own laws live with the policy they enforce — `AgentBridgeDialer`'s whole
// dial/announce/ping/back-off ladder is transport-free by construction and is proven in
// `🧱️elements/🔗️AgentBridge/🧪️tests/🔬️wgpu-unit/🦀️.rs` beside the codec it drives. What is left here
// is the byte plumbing, which needs a real page or a real socket and belongs to W14d's live loop.
