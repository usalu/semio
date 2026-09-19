//! 🔌️ The wgpu renderer's DUPLEX socket door — the second door of the browser build, beside the
//! request/response mailbox `🚪️host-io/🟦️.ts` already services for files, directory HTTP and
//! preferences.
//!
//! ⚖️ Why a second shape and not another `op` on the first: every existing door hop is one request
//! answered once. A WebSocket is a long-lived duplex channel whose inbound side is driven by the
//! PEER, not by the shell, so a mailbox cannot carry it. What CAN carry it is the pairing this
//! module owns — a page-owned socket plus a shell-owned [`SocketLane`], joined by four bounded
//! mailbox hops (`open`/`send`/`poll`/`close`). The shell never blocks: `poll` returns at most one
//! PAGE of whatever the peer sent, exactly like the world-asset lanes return one page of a mesh, and
//! says whether more is waiting.
//!
//! 🌊️ Back-pressure, stated as a law rather than a hope: the page queue is bounded on BOTH counts
//! ([`SOCKET_DOOR_QUEUE_MAX_MESSAGES`], [`SOCKET_DOOR_QUEUE_MAX_BYTES`]) and a peer that outruns the
//! shell's poll cadence loses the OLDEST frames, counted in `dropped` and reported on the very next
//! answer. A socket transport cannot apply real TCP back-pressure from a Worker (the browser owns
//! the receive side), so the honest choice is a bounded queue that SAYS what it dropped rather than
//! an unbounded one that eventually takes the tab down. `dropped > 0` is a fault the consumer
//! reports, never a silence.
//!
//! 🚪️ Why the page and not `web_sys::WebSocket` directly in this isolate: `WebSocket` does exist in
//! a dedicated Worker, so the constructor would work — but the shell's whole browser I/O surface is
//! ONE door the page owns (`📓️w1e-space-administration-wasm.md`, `🚪️host-io/🟦️.ts`'s own header),
//! and that is what makes the same-site session cookie travel on the handshake, what keeps a single
//! place to audit every origin the shell talks to, and what lets the page-mounted variant of this
//! renderer (`🎬️renderer-boot/🟦️.ts`, no Worker at all) answer identically. A socket opened inside
//! the Worker would be the one I/O path with two implementations and one of them untested.
//!
//! 🔌️ Two consumers, one lane: the hub directory stream (`📇️directory-door/🦀️.rs`'s
//! `BrowserDoorWsConnection`, `/directory/socket/v1`) and the MCP agent bridge
//! (`🧱️elements/🔗️AgentBridge`, `semio.mcp.bridge.v1`). Both are ordinary [`SocketLane`] owners;
//! the lane itself is target-neutral, so the native agent-bridge transport drives the identical
//! object from a `tokio-tungstenite` connection on an I/O worker.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

//#region 🔖️Bounds
/// 🚪️ The door op every socket hop carries. One literal, declared beside both halves, so the Rust
/// encoder and `🚪️host-io/🟦️.ts`'s decoder cannot drift into two spellings.
pub const SOCKET_DOOR_OP: &str = "socket";

/// 📃️ How many messages ONE `poll` hop may carry back. A page, not the queue: a shell that has been
/// away for a second must not receive a second's worth of traffic inside one frame opportunity.
pub const SOCKET_DOOR_POLL_MAX_MESSAGES: usize = 32;

/// 📃️ How many payload bytes ONE `poll` hop may carry back. Below the 64 KiB contiguous-request
/// ceiling the guest heap enforces (`project-guest-contiguous-request-ceiling`), so a page can
/// always be materialised.
pub const SOCKET_DOOR_POLL_MAX_BYTES: usize = 48 * 1024;

/// 🌊️ How many messages the PAGE may hold for one socket before the oldest are dropped.
pub const SOCKET_DOOR_QUEUE_MAX_MESSAGES: usize = 256;

/// 🌊️ How many payload bytes the PAGE may hold for one socket before the oldest are dropped.
pub const SOCKET_DOOR_QUEUE_MAX_BYTES: usize = 1024 * 1024;

/// 📤️ The largest single frame this door will SEND. The directory stream's own frames and every
/// bridge frame are far below it; a larger one is a caller defect, refused at the encoder rather
/// than handed to a socket that would close on it.
pub const SOCKET_DOOR_SEND_MAX_BYTES: usize = 48 * 1024;

/// 📥️ How many messages the SHELL-side lane retains between polls. The same ceiling as the page's
/// so neither half can be the silent bottleneck.
pub const SOCKET_LANE_MAX_MESSAGES: usize = SOCKET_DOOR_QUEUE_MAX_MESSAGES;
//#endregion 🔖️Bounds

//#region 🔖️Wire
/// 🔤️ The four verbs of the duplex door, in lifecycle order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SocketDoorVerb {
    Open,
    Send,
    Poll,
    Close,
}

impl SocketDoorVerb {
    pub fn wire(self) -> &'static str {
        match self {
            SocketDoorVerb::Open => "open",
            SocketDoorVerb::Send => "send",
            SocketDoorVerb::Poll => "poll",
            SocketDoorVerb::Close => "close",
        }
    }
}

/// 📨️ One hop handed to the page. Every field past `verb` is optional because the four verbs carry
/// disjoint payloads, and a JSON shape with absent keys is what the TypeScript half's own
/// discriminated union reads — the same convention `📇️directory-door` already uses.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocketDoorRequestV1 {
    pub op: String,
    pub verb: String,
    pub socket_id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocols: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_messages: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bytes: Option<usize>,
}

/// 📬️ One message the peer sent, as the page reports it. Exactly one of `text`/`binary` is present;
/// `binary` is standard base64 because the door is a JSON mailbox.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocketDoorFrameV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary: Option<String>,
}

/// 📬️ One answer the page hands back. `state` is the socket's own readiness, `messages` is at most
/// one page of what the peer sent, `more` says a further `poll` would return without waiting, and
/// `dropped` is how many frames the bounded page queue discarded since the previous hop — never
/// silently zero when frames were lost.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocketDoorAnswerV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<SocketDoorFrameV1>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub more: bool,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub dropped: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_code: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

fn is_false(value: &bool) -> bool {
    !*value
}

fn is_zero(value: &u32) -> bool {
    *value == 0
}

/// 🔌️ The socket's own readiness, as both halves spell it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SocketDoorState {
    #[default]
    Connecting,
    Open,
    Closed,
}

impl SocketDoorState {
    pub fn wire(self) -> &'static str {
        match self {
            SocketDoorState::Connecting => "connecting",
            SocketDoorState::Open => "open",
            SocketDoorState::Closed => "closed",
        }
    }

    pub fn from_wire(value: &str) -> Option<Self> {
        match value {
            "connecting" => Some(SocketDoorState::Connecting),
            "open" => Some(SocketDoorState::Open),
            "closed" => Some(SocketDoorState::Closed),
            _ => None,
        }
    }
}

/// 🔌️ Seals the `open` hop — `url` plus the EXACT ordered subprotocol list, which is how both
/// consumers keep their admission proof out of urls, logs and referrers (React's `bridgeProtocols`
/// and `socketGrantProtocolsV1` are the two producers of that list).
pub fn encode_socket_open(socket_id: u32, url: &str, protocols: &[String]) -> Result<String, String> {
    if url.is_empty() {
        return Err("socket door open carries no url".to_string());
    }
    let request = SocketDoorRequestV1 { op: SOCKET_DOOR_OP.to_string(), verb: SocketDoorVerb::Open.wire().to_string(), socket_id, url: Some(url.to_string()), protocols: protocols.to_vec(), ..SocketDoorRequestV1::default() };
    serde_json::to_string(&request).map_err(|error| error.to_string())
}

/// 📤️ Seals one TEXT frame for the peer.
pub fn encode_socket_send_text(socket_id: u32, text: &str) -> Result<String, String> {
    if text.len() > SOCKET_DOOR_SEND_MAX_BYTES {
        return Err(format!("socket door send exceeds {SOCKET_DOOR_SEND_MAX_BYTES} bytes"));
    }
    let request = SocketDoorRequestV1 { op: SOCKET_DOOR_OP.to_string(), verb: SocketDoorVerb::Send.wire().to_string(), socket_id, text: Some(text.to_string()), ..SocketDoorRequestV1::default() };
    serde_json::to_string(&request).map_err(|error| error.to_string())
}

/// 📤️ Seals one BINARY frame for the peer — every agent-bridge frame takes this path.
pub fn encode_socket_send_binary(socket_id: u32, bytes: &[u8]) -> Result<String, String> {
    if bytes.len() > SOCKET_DOOR_SEND_MAX_BYTES {
        return Err(format!("socket door send exceeds {SOCKET_DOOR_SEND_MAX_BYTES} bytes"));
    }
    let request = SocketDoorRequestV1 { op: SOCKET_DOOR_OP.to_string(), verb: SocketDoorVerb::Send.wire().to_string(), socket_id, binary: Some(semio_framework_io_base64::base64_standard_encode(bytes)), ..SocketDoorRequestV1::default() };
    serde_json::to_string(&request).map_err(|error| error.to_string())
}

/// 📥️ Seals one bounded `poll` hop. The caps are clamped HERE rather than trusted from the caller,
/// so no consumer can widen the page by asking for more.
pub fn encode_socket_poll(socket_id: u32) -> Result<String, String> {
    let request = SocketDoorRequestV1 {
        op: SOCKET_DOOR_OP.to_string(),
        verb: SocketDoorVerb::Poll.wire().to_string(),
        socket_id,
        max_messages: Some(SOCKET_DOOR_POLL_MAX_MESSAGES),
        max_bytes: Some(SOCKET_DOOR_POLL_MAX_BYTES),
        ..SocketDoorRequestV1::default()
    };
    serde_json::to_string(&request).map_err(|error| error.to_string())
}

/// 🧯️ Seals the `close` hop. A socket the page has already forgotten answers `{}` rather than an
/// error: closing twice is not a defect.
pub fn encode_socket_close(socket_id: u32) -> Result<String, String> {
    let request = SocketDoorRequestV1 { op: SOCKET_DOOR_OP.to_string(), verb: SocketDoorVerb::Close.wire().to_string(), socket_id, ..SocketDoorRequestV1::default() };
    serde_json::to_string(&request).map_err(|error| error.to_string())
}

/// 📬️ One decoded answer, with its base64 already resolved.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SocketDoorPage {
    pub state: SocketDoorState,
    pub messages: Vec<SocketMessage>,
    pub more: bool,
    pub dropped: u32,
    pub close_code: Option<u16>,
}

/// 💬️ One frame in either direction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SocketMessage {
    Text(String),
    Binary(Vec<u8>),
}

impl SocketMessage {
    pub fn byte_len(&self) -> usize {
        match self {
            SocketMessage::Text(text) => text.len(),
            SocketMessage::Binary(bytes) => bytes.len(),
        }
    }
}

/// 📬️ Reads one door answer. A refusal is an `Err`, never a fabricated empty page: "the peer sent
/// nothing" and "this hop was refused" are different answers and every consumer branches on the
/// difference. An unreadable frame inside an otherwise good page is DROPPED and counted, because one
/// malformed base64 blob must not discard the frames beside it.
pub fn decode_socket_answer(answer: &str) -> Result<SocketDoorPage, String> {
    let parsed: SocketDoorAnswerV1 = serde_json::from_str(answer).map_err(|error| format!("socket door answer unreadable: {error}"))?;
    if let Some(error) = parsed.error {
        return Err(error);
    }
    let state = match parsed.state.as_deref() {
        Some(value) => SocketDoorState::from_wire(value).ok_or_else(|| format!("socket door answer carries an unknown state {value:?}"))?,
        None => return Err("socket door answer carries no state".to_string()),
    };
    let mut dropped = parsed.dropped;
    let mut messages = Vec::with_capacity(parsed.messages.len());
    for frame in parsed.messages {
        match (frame.text, frame.binary) {
            (Some(text), None) => messages.push(SocketMessage::Text(text)),
            (None, Some(binary)) => match semio_framework_io_base64::base64_standard_decode(&binary) {
                Ok(bytes) => messages.push(SocketMessage::Binary(bytes)),
                Err(_) => dropped = dropped.saturating_add(1),
            },
            _ => dropped = dropped.saturating_add(1),
        }
    }
    Ok(SocketDoorPage { state, messages, more: parsed.more, dropped, close_code: parsed.close_code })
}
//#endregion 🔖️Wire

//#region 🔖️Lane
/// 🌊️ The SHELL-side half of one duplex socket: bounded inbound queue, bounded outbound queue, the
/// lifecycle the consumer reads, and an honest loss counter. Target-neutral and transport-free by
/// construction — the browser half drives it from the page door and the native half drives it from a
/// `tokio-tungstenite` connection on an I/O worker, so every law below holds on both.
#[derive(Clone, Debug, Default)]
pub struct SocketLane {
    state: SocketDoorState,
    inbound: VecDeque<SocketMessage>,
    outbound: VecDeque<SocketMessage>,
    dropped_inbound: u32,
    dropped_outbound: u32,
    last_error: Option<String>,
    close_code: Option<u16>,
}

impl SocketLane {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn state(&self) -> SocketDoorState {
        self.state
    }

    pub fn is_open(&self) -> bool {
        matches!(self.state, SocketDoorState::Open)
    }

    pub fn is_closed(&self) -> bool {
        matches!(self.state, SocketDoorState::Closed)
    }

    /// 📉️ Frames this lane lost to its own ceilings, inbound plus outbound. A consumer that reports
    /// zero here really did see everything.
    pub fn dropped(&self) -> u32 {
        self.dropped_inbound.saturating_add(self.dropped_outbound)
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn close_code(&self) -> Option<u16> {
        self.close_code
    }

    pub fn inbound_len(&self) -> usize {
        self.inbound.len()
    }

    pub fn outbound_len(&self) -> usize {
        self.outbound.len()
    }

    /// 🔌️ The socket reached `open`. Re-announcing an already-open lane is a no-op, so a duplicated
    /// `poll` answer cannot reset anything.
    pub fn note_open(&mut self) {
        if matches!(self.state, SocketDoorState::Closed) {
            return;
        }
        self.state = SocketDoorState::Open;
    }

    /// 🧯️ The socket closed, for whatever reason. Terminal: everything still queued outbound is
    /// discarded (it can never reach this peer) while the inbound queue is KEPT, because those
    /// frames really did arrive and the consumer has not read them yet.
    pub fn note_closed(&mut self, code: Option<u16>) {
        self.state = SocketDoorState::Closed;
        self.close_code = code;
        self.outbound.clear();
    }

    /// ⚠️ Records one transport refusal without closing the lane — a failed `send` is not a closed
    /// socket, and the consumer decides whether to redial.
    pub fn note_error(&mut self, error: impl Into<String>) {
        self.last_error = Some(error.into());
    }

    /// 📥️ Admits one frame the peer sent. Over the ceiling the OLDEST is dropped and counted: a live
    /// view must show the newest traffic, and an unbounded queue in a fixed-credit renderer is not an
    /// option.
    pub fn push_inbound(&mut self, message: SocketMessage) {
        if self.inbound.len() >= SOCKET_LANE_MAX_MESSAGES {
            self.inbound.pop_front();
            self.dropped_inbound = self.dropped_inbound.saturating_add(1);
        }
        self.inbound.push_back(message);
    }

    /// 📥️ Applies one whole decoded door page: state, every frame, and the page's own loss count.
    pub fn apply_page(&mut self, page: SocketDoorPage) {
        match page.state {
            SocketDoorState::Open => self.note_open(),
            SocketDoorState::Closed => self.note_closed(page.close_code),
            SocketDoorState::Connecting => {}
        }
        self.dropped_inbound = self.dropped_inbound.saturating_add(page.dropped);
        for message in page.messages {
            self.push_inbound(message);
        }
    }

    /// 📤️ Queues one frame for the peer. A closed lane refuses rather than parking a frame nothing
    /// will ever write, and an over-ceiling queue drops the OLDEST — the newest turn is the one the
    /// human is waiting on.
    pub fn push_outbound(&mut self, message: SocketMessage) -> bool {
        if matches!(self.state, SocketDoorState::Closed) {
            return false;
        }
        if message.byte_len() > SOCKET_DOOR_SEND_MAX_BYTES {
            self.dropped_outbound = self.dropped_outbound.saturating_add(1);
            self.note_error(format!("socket lane refused a {} byte frame", message.byte_len()));
            return false;
        }
        if self.outbound.len() >= SOCKET_LANE_MAX_MESSAGES {
            self.outbound.pop_front();
            self.dropped_outbound = self.dropped_outbound.saturating_add(1);
        }
        self.outbound.push_back(message);
        true
    }

    /// 📥️ Takes exactly one inbound frame, for a consumer that steps rather than drains (the
    /// `DirectoryWsConnection::try_recv_text` seam is one-frame-per-call by contract).
    pub fn pop_inbound(&mut self) -> Option<SocketMessage> {
        self.inbound.pop_front()
    }

    /// 📥️ Takes at most one PAGE of inbound frames — the same ceilings the door itself applies, so a
    /// consumer draining the lane can never spend more than one page's work per opportunity.
    pub fn take_inbound_page(&mut self) -> Vec<SocketMessage> {
        let mut page = Vec::new();
        let mut bytes = 0usize;
        while page.len() < SOCKET_DOOR_POLL_MAX_MESSAGES {
            let Some(next) = self.inbound.front() else { break };
            let next_bytes = bytes.saturating_add(next.byte_len());
            if !page.is_empty() && next_bytes > SOCKET_DOOR_POLL_MAX_BYTES {
                break;
            }
            bytes = next_bytes;
            page.push(self.inbound.pop_front().expect("front was just observed"));
        }
        page
    }

    /// 📤️ Takes at most one PAGE of outbound frames for the transport to write.
    pub fn take_outbound_page(&mut self) -> Vec<SocketMessage> {
        let mut page = Vec::new();
        let mut bytes = 0usize;
        while page.len() < SOCKET_DOOR_POLL_MAX_MESSAGES {
            let Some(next) = self.outbound.front() else { break };
            let next_bytes = bytes.saturating_add(next.byte_len());
            if !page.is_empty() && next_bytes > SOCKET_DOOR_POLL_MAX_BYTES {
                break;
            }
            bytes = next_bytes;
            page.push(self.outbound.pop_front().expect("front was just observed"));
        }
        page
    }

    /// 🔁️ Resets the lane for a redial, keeping nothing from the dead socket — a reconnect resumes
    /// the CONSUMER's protocol (the bridge re-sends `Hello`, the directory stream resubscribes from
    /// its own frontier), never a half-written frame from the previous connection.
    pub fn reset_for_redial(&mut self) {
        self.state = SocketDoorState::Connecting;
        self.inbound.clear();
        self.outbound.clear();
        self.close_code = None;
    }
}
//#endregion 🔖️Lane

//#region 🔖️BrowserDoor
/// 🌐️ The browser half: one page-owned socket, addressed by id, driven entirely through
/// `semioWgpuHostIo`.
///
/// ⏳️ Why every hop is detached rather than awaited in place: the two trait seams this door has to
/// satisfy — `DirectoryTransport::open_ws` and `DirectoryWsConnection::{send_text, try_recv_text}` —
/// are SYNCHRONOUS, and a wasm isolate may not block. So a hop is posted and its answer lands in the
/// lane on a later turn; `open` reports `Connecting` until the page says otherwise, and
/// `try_recv_text` answers `Pending` in the meantime. That is the real shape of the thing, not a
/// workaround: a socket IS asynchronous, and the trait's `Pending` arm exists for exactly this.
#[cfg(target_arch = "wasm32")]
pub mod browser {
    use super::{decode_socket_answer, encode_socket_close, encode_socket_open, encode_socket_poll, encode_socket_send_binary, encode_socket_send_text, SocketLane, SocketMessage};
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::rc::Rc;

    thread_local! {
        /// 🔢️ Monotonic socket ids for this isolate. Never reused: a late answer from a closed
        /// socket must be attributable to the socket it belonged to, not to its successor.
        static NEXT_SOCKET_ID: RefCell<u32> = const { RefCell::new(1) };
        /// 🗂️ Every live lane, so a detached hop can settle into the one it belongs to even after
        /// the consumer dropped its own handle.
        static SOCKETS: RefCell<BTreeMap<u32, Rc<RefCell<SocketLane>>>> = RefCell::new(BTreeMap::new());
    }

    /// 🔌️ One page-owned socket the shell holds. Dropping it closes the page's socket.
    pub struct BrowserDoorSocket {
        socket_id: u32,
        lane: Rc<RefCell<SocketLane>>,
        polling: Rc<RefCell<bool>>,
    }

    impl BrowserDoorSocket {
        /// 🔌️ Posts the dial and answers immediately in `Connecting`.
        pub fn open(url: &str, protocols: &[String]) -> Result<Self, String> {
            let socket_id = NEXT_SOCKET_ID.with(|next| {
                let mut next = next.borrow_mut();
                let id = *next;
                *next = next.saturating_add(1);
                id
            });
            let request = encode_socket_open(socket_id, url, protocols)?;
            let lane = Rc::new(RefCell::new(SocketLane::new()));
            SOCKETS.with(|sockets| sockets.borrow_mut().insert(socket_id, lane.clone()));
            let settle = lane.clone();
            crate::spawn_app_task(async move {
                match crate::shell::host_io_call(&request, None).await {
                    Ok(answer) => match decode_socket_answer(&answer) {
                        Ok(page) => settle.borrow_mut().apply_page(page),
                        Err(error) => {
                            let mut lane = settle.borrow_mut();
                            lane.note_error(error);
                            lane.note_closed(None);
                        }
                    },
                    Err(error) => {
                        let mut lane = settle.borrow_mut();
                        lane.note_error(error);
                        lane.note_closed(None);
                    }
                }
            });
            Ok(Self { socket_id, lane, polling: Rc::new(RefCell::new(false)) })
        }

        pub fn lane(&self) -> std::cell::Ref<'_, SocketLane> {
            self.lane.borrow()
        }

        /// 📤️ Queues one frame and flushes whatever the lane holds, one detached hop per frame.
        pub fn send(&mut self, message: SocketMessage) -> Result<(), String> {
            if !self.lane.borrow_mut().push_outbound(message) {
                return Err(self.lane.borrow().last_error().unwrap_or("socket lane refused the frame").to_string());
            }
            self.flush();
            Ok(())
        }

        fn flush(&self) {
            let page = self.lane.borrow_mut().take_outbound_page();
            for message in page {
                let request = match &message {
                    SocketMessage::Text(text) => encode_socket_send_text(self.socket_id, text),
                    SocketMessage::Binary(bytes) => encode_socket_send_binary(self.socket_id, bytes),
                };
                let Ok(request) = request else {
                    self.lane.borrow_mut().note_error("socket door refused to seal a frame");
                    continue;
                };
                let settle = self.lane.clone();
                crate::spawn_app_task(async move {
                    match crate::shell::host_io_call(&request, None).await {
                        Ok(answer) => match decode_socket_answer(&answer) {
                            Ok(page) => settle.borrow_mut().apply_page(page),
                            Err(error) => settle.borrow_mut().note_error(error),
                        },
                        Err(error) => settle.borrow_mut().note_error(error),
                    }
                });
            }
        }

        /// 📥️ Arms ONE outstanding poll hop at a time — the back-pressure that keeps a slow page
        /// from accumulating a queue of identical requests behind it.
        pub fn arm_poll(&self) {
            if *self.polling.borrow() || self.lane.borrow().is_closed() {
                return;
            }
            let Ok(request) = encode_socket_poll(self.socket_id) else { return };
            *self.polling.borrow_mut() = true;
            let settle = self.lane.clone();
            let polling = self.polling.clone();
            crate::spawn_app_task(async move {
                let outcome = crate::shell::host_io_call(&request, None).await;
                *polling.borrow_mut() = false;
                match outcome {
                    Ok(answer) => match decode_socket_answer(&answer) {
                        Ok(page) => settle.borrow_mut().apply_page(page),
                        Err(error) => {
                            let mut lane = settle.borrow_mut();
                            lane.note_error(error);
                            lane.note_closed(None);
                        }
                    },
                    Err(error) => {
                        let mut lane = settle.borrow_mut();
                        lane.note_error(error);
                        lane.note_closed(None);
                    }
                }
            });
        }

        /// 📥️ One frame the peer sent, or `None` while nothing has landed. Takes exactly one, so a
        /// stepping consumer never loses the frames behind it.
        pub fn try_recv(&mut self) -> Option<SocketMessage> {
            self.arm_poll();
            self.lane.borrow_mut().pop_inbound()
        }

        /// 📥️ The whole page at once, for consumers that drain rather than step.
        pub fn drain(&mut self) -> Vec<SocketMessage> {
            self.arm_poll();
            self.lane.borrow_mut().take_inbound_page()
        }

        pub fn close(&mut self) {
            if self.lane.borrow().is_closed() {
                return;
            }
            self.lane.borrow_mut().note_closed(None);
            SOCKETS.with(|sockets| sockets.borrow_mut().remove(&self.socket_id));
            let Ok(request) = encode_socket_close(self.socket_id) else { return };
            crate::spawn_app_task(async move {
                let _ = crate::shell::host_io_call(&request, None).await;
            });
        }
    }

    impl Drop for BrowserDoorSocket {
        fn drop(&mut self) {
            self.close();
        }
    }
}
//#endregion 🔖️BrowserDoor

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
