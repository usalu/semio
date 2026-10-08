//! 📎 The connection of a view to the workspace daemon. One thread owns the transport: it writes what the
//! view queued, reads what the daemon sent and hands it over in order, and it stops reading while the
//! view has not taken what it already received, so a slow view slows its own stream and nothing else.
//! Terminal input is queued without a count limit that could drop keys; the queue is bounded in bytes.
//!
//! A connection says hello as its first message: its protocol, its build and the environment of its
//! process, which is the environment the daemon starts the tasks it requests in.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🌀️daemon/🔣️.json

use super::ipc::{self, ClientMsg, ServerMsg};
use super::transport::{Poller, Stream, Token, Waker};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, SyncSender, TrySendError};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// 🧱 The most bytes a view queues for the daemon before its sends fail.
const QUEUED_BYTES: usize = 16 * 1024 * 1024;
const GREETING: Duration = Duration::from_secs(2);
const MAX_ENVIRONMENT: usize = 8192;

/// 📬 One thing the daemon told the view.
pub enum Message {
    Control(ServerMsg),
    Output { session_id: String, data: Vec<u8> },
    Disconnected(String),
}

/// ⚠️ How the daemon differs from this client: a different protocol cannot be served, a different build
/// is served but reported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skew {
    pub client_protocol: u32,
    pub daemon_protocol: u32,
    pub client_build: String,
    pub daemon_build: String,
}

impl Skew {
    /// 🚫 Whether the two ends cannot talk to each other at all.
    pub fn incompatible(&self) -> bool { self.client_protocol != self.daemon_protocol }

    /// 💬 The sentence that names the difference.
    pub fn describe(&self) -> String {
        if self.incompatible() {
            format!("daemon speaks protocol {} but this client speaks {}; stop the daemon once its tasks finish (`semio daemon stop`) and start it again", self.daemon_protocol, self.client_protocol)
        } else {
            format!("daemon build {} differs from this client's build {}; its tasks keep running, `semio daemon stop` and a new start bring the daemon up to date", self.daemon_build, self.client_build)
        }
    }
}

/// 📮 The frames a view queued for the daemon. The byte count changes in exactly two places, [`Outbound::push`] and
/// [`Outbound::pop`], each with the length of the frame it moves, so it is the sum of the queued frames at every moment.
#[derive(Default)]
pub struct Outbound {
    frames: VecDeque<Vec<u8>>,
    bytes: usize,
    closing: bool,
}

impl Outbound {
    pub fn push(&mut self, frame: Vec<u8>) {
        self.bytes += frame.len();
        self.frames.push_back(frame);
    }

    pub fn pop(&mut self) -> Option<Vec<u8>> {
        let frame = self.frames.pop_front()?;
        self.bytes -= frame.len();
        Some(frame)
    }

    /// 🧮 The bytes of the frames that wait to be written.
    pub fn queued_bytes(&self) -> usize { self.bytes }

    /// 🔒 Whether `frame` still fits under the limit of queued bytes.
    pub fn fits(&self, frame: &[u8]) -> bool { self.bytes + frame.len() <= QUEUED_BYTES }
}

/// 📣 What a view calls to wake its own wait: the connection calls it from its thread whenever a message arrived or the queue of sends drained.
pub type Notifier = Arc<dyn Fn() + Send + Sync>;

/// 🔌 A connection to the daemon of one workspace.
pub struct Connection {
    notifier: Arc<Mutex<Option<Notifier>>>,
    outbound: Arc<Mutex<Outbound>>,
    waker: Waker,
    receiver: Receiver<Message>,
    alive: Arc<AtomicBool>,
    daemon_pid: u32,
    daemon_protocol: u32,
    daemon_build: String,
}

/// 🌍 The environment of this process as a hello carries it; entries a task could not receive are left out.
pub fn process_environment() -> Vec<(String, String)> {
    std::env::vars_os().filter_map(|(name, value)| {
        let (name, value) = (name.into_string().ok()?, value.into_string().ok()?);
        (!name.is_empty() && !name.contains(['=', '\0']) && !value.contains('\0')).then_some((name, value))
    }).take(MAX_ENVIRONMENT).collect()
}

fn greeting(stream: &mut Stream, poller: &mut Poller, buffer: &mut ipc::FrameBuffer) -> std::io::Result<ServerMsg> {
    let deadline = Instant::now() + GREETING;
    let mut page = [0u8; 4096];
    loop {
        if let Some((kind, payload)) = buffer.next_frame()? {
            if kind != ipc::KIND_CONTROL { return Err(std::io::Error::other("invalid daemon greeting")); }
            return ipc::decode_control(payload);
        }
        match stream.try_read(&mut page)? {
            Some(0) => return Err(std::io::Error::other("daemon disconnected")),
            Some(count) => buffer.extend(&page[..count]),
            None => {
                if Instant::now() >= deadline { return Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "daemon greeting timed out")); }
                poller.begin();
                poller.stream(Token::Client(0), stream, true, false);
                poller.wait(Duration::from_millis(20))?;
            }
        }
    }
}

impl Connection {
    /// 🤝 Connects to the daemon of the workspace and says hello with this process's environment.
    pub fn connect(root: &std::path::Path) -> std::io::Result<Self> {
        Self::connect_with(root, "client", process_environment())
    }

    /// 🪝️ Connects as the client named `name`, whose tasks start in `environment`.
    pub fn connect_with(root: &std::path::Path, name: &str, environment: Vec<(String, String)>) -> std::io::Result<Self> {
        let mut stream = Stream::connect(root)?;
        let mut poller = Poller::new()?;
        let waker = poller.waker();
        waker.watch_stream(&stream);
        let mut buffer = ipc::FrameBuffer::default();
        let ServerMsg::Attached { daemon_pid, protocol, build_id } = greeting(&mut stream, &mut poller, &mut buffer)? else { return Err(std::io::Error::other("invalid daemon greeting")) };
        let hello = ClientMsg::Hello { client_id: format!("{name}-{}", std::process::id()), protocol: ipc::PROTOCOL, build_id: ipc::build_id().to_string(), env: environment };
        hello.validate()?;
        let first = ipc::control_frame(&hello)?;
        let outbound = Arc::new(Mutex::new(Outbound::default()));
        if let Ok(mut queue) = outbound.lock() { queue.push(first); }
        let (messages, receiver) = mpsc::sync_channel::<Message>(512);
        let alive = Arc::new(AtomicBool::new(true));
        let notifier: Arc<Mutex<Option<Notifier>>> = Arc::default();
        let (queued, running, notify) = (outbound.clone(), alive.clone(), notifier.clone());
        std::thread::Builder::new().name("dashboard connection".into()).spawn(move || {
            let reason = Self::run(stream, poller, buffer, &queued, &messages, &notify).err().map_or_else(|| "daemon disconnected".to_string(), |error| error.to_string());
            running.store(false, Ordering::SeqCst);
            Self::wake(&notify);
            let mut farewell = Message::Disconnected(reason);
            while let Err(TrySendError::Full(held)) = messages.try_send(farewell) {
                farewell = held;
                std::thread::sleep(Duration::from_millis(10));
            }
        })?;
        Ok(Self { notifier, outbound, waker, receiver, alive, daemon_pid, daemon_protocol: protocol, daemon_build: build_id })
    }

    fn wake(notifier: &Mutex<Option<Notifier>>) {
        let call = notifier.lock().ok().and_then(|slot| slot.clone());
        if let Some(call) = call { call(); }
    }

    fn run(mut stream: Stream, mut poller: Poller, mut buffer: ipc::FrameBuffer, queued: &Mutex<Outbound>, messages: &SyncSender<Message>, notifier: &Mutex<Option<Notifier>>) -> std::io::Result<()> {
        let mut held: VecDeque<Message> = VecDeque::new();
        let (mut front, mut sent): (Vec<u8>, usize) = (Vec::new(), 0);
        let mut page = vec![0u8; 64 * 1024];
        loop {
            let mut delivered = false;
            while let Some(message) = held.pop_front() {
                match messages.try_send(message) {
                    Ok(()) => delivered = true,
                    Err(TrySendError::Full(message)) => { held.push_front(message); break; }
                    Err(TrySendError::Disconnected(_)) => return Ok(()),
                }
            }
            if delivered { Self::wake(notifier); }
            let (mut drained, mut wrote) = (false, false);
            loop {
                if sent == front.len() {
                    front.clear();
                    sent = 0;
                    let Ok(mut outbound) = queued.lock() else { return Err(std::io::Error::other("connection queue poisoned")) };
                    match outbound.pop() {
                        Some(frame) => front = frame,
                        None => if outbound.closing { return Ok(()) } else { drained = wrote; break },
                    }
                }
                let written = stream.try_write(&front[sent..])?;
                if written == 0 { break; }
                sent += written;
                wrote = true;
            }
            if drained { Self::wake(notifier); }
            let backpressure = !held.is_empty();
            if !backpressure {
                while let Some(count) = stream.try_read(&mut page)? {
                    if count == 0 { return Err(std::io::Error::other("daemon disconnected")); }
                    buffer.extend(&page[..count]);
                    while let Some((kind, payload)) = buffer.next_frame()? {
                        held.push_back(match kind {
                            ipc::KIND_CONTROL => Message::Control(ipc::decode_control(payload)?),
                            ipc::KIND_OUTPUT => { let (session_id, data) = ipc::decode_output(payload)?; Message::Output { session_id, data } }
                            _ => return Err(std::io::Error::other("unknown daemon frame")),
                        });
                    }
                    if held.len() >= 64 { break; }
                }
            }
            poller.begin();
            poller.stream(Token::Client(0), &stream, !backpressure, sent < front.len());
            poller.wait(Duration::from_millis(if backpressure { 10 } else { 50 }))?;
        }
    }

    /// 🔔️ Names what to call, from the connection's thread, when a message arrived or the queue of sends drained, so a view wakes its own wait at once.
    pub fn set_notifier(&self, notifier: Notifier) {
        if let Ok(mut slot) = self.notifier.lock() { *slot = Some(notifier); }
    }

    pub fn daemon_pid(&self) -> u32 { self.daemon_pid }

    pub fn daemon_protocol(&self) -> u32 { self.daemon_protocol }

    pub fn daemon_build(&self) -> &str { &self.daemon_build }

    /// 🩺️ How the daemon differs from this client, when it does.
    pub fn skew(&self) -> Option<Skew> {
        (self.daemon_protocol != ipc::PROTOCOL || self.daemon_build != ipc::build_id()).then(|| Skew { client_protocol: ipc::PROTOCOL, daemon_protocol: self.daemon_protocol, client_build: ipc::build_id().to_string(), daemon_build: self.daemon_build.clone() })
    }

    /// 🧵 Whether the connection's thread still serves it.
    pub fn alive(&self) -> bool { self.alive.load(Ordering::SeqCst) }

    fn queue(&self, frame: Vec<u8>) -> std::io::Result<()> {
        let mut outbound = self.outbound.lock().map_err(|_| std::io::Error::other("connection queue poisoned"))?;
        if outbound.closing || !self.alive() { return Err(std::io::Error::other("dashboard daemon connection unavailable")); }
        if !outbound.fits(&frame) { return Err(std::io::Error::other("dashboard daemon connection backlog exceeded")); }
        outbound.push(frame);
        drop(outbound);
        self.waker.wake();
        Ok(())
    }

    /// ✉️ Queues one control message.
    pub fn send(&mut self, command: &ClientMsg) -> std::io::Result<()> {
        command.validate()?;
        self.queue(ipc::control_frame(command)?)
    }

    /// ⌨️ Queues terminal input for a session as input frames, which carry bytes instead of a number per byte.
    pub fn input(&mut self, session_id: &str, data: &[u8]) -> std::io::Result<()> {
        ClientMsg::Input { session_id: session_id.to_string(), data: Vec::new() }.validate()?;
        for chunk in data.chunks(ipc::MAX_CHUNK_BYTES) { self.queue(ipc::chunk_frame(ipc::KIND_INPUT, session_id, chunk)?)?; }
        Ok(())
    }

    /// 📪️ Waits up to `timeout` for the daemon's next message and answers everything that arrived.
    pub fn receive(&mut self, timeout: Duration) -> std::io::Result<Vec<Message>> {
        let mut messages = Vec::new();
        match self.receiver.recv_timeout(timeout) {
            Ok(message) => messages.push(message),
            Err(mpsc::RecvTimeoutError::Timeout) => return Ok(messages),
            Err(mpsc::RecvTimeoutError::Disconnected) => return Err(std::io::Error::other("dashboard daemon disconnected")),
        }
        messages.extend(self.receiver.try_iter().take(255));
        Ok(messages)
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        if let (Ok(mut outbound), Ok(frame)) = (self.outbound.lock(), ipc::control_frame(&ClientMsg::Detach {})) {
            outbound.push(frame);
            outbound.closing = true;
        }
        self.waker.wake();
    }
}
