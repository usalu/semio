//! 👥️ The views attached to the daemon. A view reads what it subscribed to at its own pace: for every
//! session it follows, the daemon keeps a position in the session's output and sends the next piece
//! whenever the connection accepts bytes. A view that stalls only falls behind; the daemon never waits
//! for it, never buffers its backlog and never disconnects it. A view that falls so far behind that the
//! output it still needs is gone is sent a fresh replay.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🌀️daemon/🔣️.json

use super::ipc::{self, code, ClientMsg, ErrorCode, ServerMsg, SessionInfo};
use super::replay::{Fetch, SessionLog};
use super::transport::Stream;
use std::collections::{BTreeMap, VecDeque};

/// 📦 The most terminal bytes the daemon puts into one output frame.
pub const CHUNK_BYTES: usize = 32 * 1024;
/// 🚰 The most bytes the daemon hands to one view in one turn, so no view starves the others.
const PUMP_BYTES: usize = 4 * 1024 * 1024;
/// 📨 The most control frames queued for one view before it is told the full state instead.
const CONTROL_FRAMES: usize = 4096;
const READ_BYTES: usize = 256 * 1024;
const FRAMES_PER_TURN: usize = 256;
const UNREAD_BYTES: usize = 8 * 1024 * 1024;
/// 📄 How many sessions one page of the listing carries.
pub const PAGE_SESSIONS: usize = 16;

/// 🗃️ What a view reads from: the output logs and the session projections of the daemon.
pub trait Outputs {
    fn log(&mut self, session_id: &str) -> Option<&mut SessionLog>;
    fn infos(&self) -> Vec<SessionInfo>;
}

/// 📥 One thing a view said.
#[derive(Debug)]
pub enum Inbound {
    Control(ClientMsg),
    Input(String, Vec<u8>),
    Rejected(ErrorCode, String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Phase {
    Start,
    Data(u64),
    Live,
}

struct Sub {
    cursor: u64,
    phase: Phase,
    truncated: bool,
    preamble: Vec<u8>,
}

/// 🖥️ One attached view.
pub struct Client {
    pub id: u64,
    pub stream: Stream,
    pub environment: Option<Vec<(String, String)>>,
    pub watching: bool,
    pub attached: bool,
    pub closing: bool,
    inbound: ipc::FrameBuffer,
    subs: BTreeMap<String, Sub>,
    control: VecDeque<Vec<u8>>,
    front: Vec<u8>,
    sent: usize,
    rotation: usize,
    resync: bool,
    replaying_all: bool,
}

impl Client {
    pub fn new(id: u64, stream: Stream) -> Self {
        Self { id, stream, environment: None, watching: false, attached: false, closing: false, inbound: ipc::FrameBuffer::default(), subs: BTreeMap::new(), control: VecDeque::new(), front: Vec::new(), sent: 0, rotation: 0, resync: false, replaying_all: false }
    }

    /// ✉️ Queues one control message behind everything queued before it.
    pub fn tell(&mut self, message: &ServerMsg) {
        if let Ok(frame) = ipc::control_frame(message) { self.push(frame); }
    }

    /// 📮️ Queues one encoded control frame.
    pub fn push(&mut self, frame: Vec<u8>) {
        if self.control.len() >= CONTROL_FRAMES {
            self.control.clear();
            self.resync = true;
        }
        self.control.push_back(frame);
    }

    /// 📃️ Queues the listing of sessions in pages.
    pub fn list(&mut self, sessions: Vec<SessionInfo>) {
        if sessions.is_empty() { return self.tell(&ServerMsg::Sessions { sessions, more: false }); }
        let pages: Vec<&[SessionInfo]> = sessions.chunks(PAGE_SESSIONS).collect();
        for (index, page) in pages.iter().enumerate() { self.tell(&ServerMsg::Sessions { sessions: page.to_vec(), more: index + 1 < pages.len() }); }
    }

    /// 📡 Starts following a session: its output is replayed from the place its log offers, then continues live.
    pub fn follow(&mut self, session_id: &str, log: &mut SessionLog) {
        let (cursor, preamble, truncated) = log.replay();
        self.subs.insert(session_id.to_string(), Sub { cursor, phase: Phase::Start, truncated, preamble });
    }

    /// ⏹️ Stops following a session.
    pub fn unfollow(&mut self, session_id: &str) {
        self.subs.remove(session_id);
    }

    pub fn follows(&self, session_id: &str) -> bool { self.subs.contains_key(session_id) }

    /// 🔁 The replay of every followed session ends with one message that says all of them are live.
    pub fn replay_all(&mut self) { self.replaying_all = true; }

    /// 🔌 Whether the view has bytes the connection did not take yet.
    pub fn wants_write(&self) -> bool { self.sent < self.front.len() || !self.control.is_empty() }

    /// 🏁 Whether everything told to the view has been written.
    pub fn flushed(&self) -> bool { !self.wants_write() }

    /// 👂️ Reads what the view sent; the second value tells whether the connection is still open.
    pub fn receive(&mut self) -> (Vec<Inbound>, bool) {
        let mut open = true;
        let mut page = [0u8; 32 * 1024];
        let mut total = 0;
        while total < READ_BYTES && self.inbound.pending() < UNREAD_BYTES {
            match self.stream.try_read(&mut page) {
                Ok(None) => break,
                Ok(Some(0)) | Err(_) => { open = false; break; }
                Ok(Some(count)) => { self.inbound.extend(&page[..count]); total += count; }
            }
        }
        let mut messages = Vec::new();
        for _ in 0..FRAMES_PER_TURN {
            match self.inbound.next_frame() {
                Ok(None) => break,
                Ok(Some((kind, payload))) => messages.push(Self::decode(kind, payload)),
                Err(error) => { messages.push(Inbound::Rejected(code::PROTOCOL, error.to_string())); open = false; break; }
            }
        }
        (messages, open)
    }

    fn decode(kind: u8, payload: &[u8]) -> Inbound {
        match kind {
            ipc::KIND_CONTROL => match ipc::decode_control::<ClientMsg>(payload) {
                Ok(message) => Inbound::Control(message),
                Err(error) => Inbound::Rejected(code::DECODE, error.to_string()),
            },
            ipc::KIND_INPUT => match ipc::decode_chunk(payload) {
                Ok((session_id, data)) => Inbound::Input(session_id.to_string(), data.to_vec()),
                Err(error) => Inbound::Rejected(code::DECODE, error.to_string()),
            },
            other => Inbound::Rejected(code::PROTOCOL, format!("unknown frame kind {other}")),
        }
    }

    /// 📤 Writes as much as the connection takes: queued control messages first, then the next piece of
    /// every followed session in turn. The answer tells whether more is ready to be written.
    pub fn pump(&mut self, outputs: &mut dyn Outputs) -> std::io::Result<bool> {
        let mut budget = PUMP_BYTES;
        loop {
            if self.sent < self.front.len() {
                let written = self.stream.try_write(&self.front[self.sent..])?;
                self.sent += written;
                budget = budget.saturating_sub(written);
                if self.sent < self.front.len() { return Ok(false); }
                self.front.clear();
                self.sent = 0;
            }
            if std::mem::take(&mut self.resync) {
                let infos = outputs.infos();
                self.list(infos);
            }
            if let Some(frame) = self.control.pop_front() {
                self.front = frame;
                continue;
            }
            if budget == 0 { return Ok(true); }
            match self.next_frame(outputs)? {
                Some(frame) => self.front = frame,
                None => return Ok(false),
            }
        }
    }

    fn next_frame(&mut self, outputs: &mut dyn Outputs) -> std::io::Result<Option<Vec<u8>>> {
        let ids: Vec<String> = self.subs.keys().cloned().collect();
        for step in 0..ids.len() {
            let id = &ids[(self.rotation + step) % ids.len()];
            let Some(log) = outputs.log(id) else { self.subs.remove(id); continue };
            let Some(sub) = self.subs.get_mut(id) else { continue };
            let limit = match sub.phase {
                Phase::Start => {
                    let frame = ipc::control_frame(&ServerMsg::ReplayStart { session_id: id.clone(), truncated: sub.truncated })?;
                    if !sub.preamble.is_empty() { self.control.push_front(ipc::chunk_frame(ipc::KIND_OUTPUT, id, &std::mem::take(&mut sub.preamble))?); }
                    sub.phase = Phase::Data(log.end());
                    self.rotation = self.rotation.wrapping_add(step + 1);
                    return Ok(Some(frame));
                }
                Phase::Data(until) if sub.cursor >= until => {
                    sub.phase = Phase::Live;
                    self.rotation = self.rotation.wrapping_add(step + 1);
                    return Ok(Some(ipc::control_frame(&ServerMsg::ReplayComplete { session_id: Some(id.clone()) })?));
                }
                Phase::Data(until) => CHUNK_BYTES.min((until - sub.cursor) as usize),
                Phase::Live => CHUNK_BYTES,
            };
            let mut data = Vec::with_capacity(limit);
            match log.read(sub.cursor, limit, &mut data) {
                Fetch::Data if !data.is_empty() => {
                    sub.cursor += data.len() as u64;
                    self.rotation = self.rotation.wrapping_add(step + 1);
                    return Ok(Some(ipc::chunk_frame(ipc::KIND_OUTPUT, id, &data)?));
                }
                Fetch::Lost => {
                    let (cursor, preamble, _) = log.replay();
                    if cursor <= sub.cursor {
                        sub.cursor = log.end();
                        sub.phase = Phase::Live;
                        continue;
                    }
                    *sub = Sub { cursor, phase: Phase::Start, truncated: true, preamble };
                    return self.next_frame(outputs);
                }
                Fetch::Data | Fetch::Current => {}
            }
        }
        if self.replaying_all && self.subs.values().all(|sub| sub.phase == Phase::Live) {
            self.replaying_all = false;
            return Ok(Some(ipc::control_frame(&ServerMsg::ReplayComplete { session_id: None })?));
        }
        Ok(None)
    }
}
