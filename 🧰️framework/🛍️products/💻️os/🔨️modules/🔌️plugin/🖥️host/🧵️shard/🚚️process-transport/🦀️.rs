//! 🚚️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (P1-process-shards): `ProcessTransport` — the
//! `ShardTransport` impl `🎭️actor/🦀️.rs`'s own doc comment names as host-supplied
//! ("`WorkerTransport` (postMessage) and `ProcessTransport` (stdio) are host-supplied"). Lives here,
//! NOT in `🎭️actor`, because that crate must stay pure (no `std::process`/`std::thread`/
//! `SystemTime`) — see this ticket's `## purity grep` in the P1 report. `StdioTransport` (below,
//! `//#region 👶️StdioTransport`) is the mirror-image impl the `semio-shard` `[[bin]]`
//! (`../👶️child/🦀️.rs`) hosts its own `ShardLoop` over — same [`framing`] module, same
//! `Envelope`/`ShardOutcome` bytes, opposite ends of the same pipe.
//!
//! **Framing**: length-prefixed (`[tag:u8][len:u32 LE][payload]`), never newline/text-delimited —
//! `Envelope`/`ShardOutcome` bytes are arbitrary owned pack-encoded binary containing arbitrary
//! bytes in string fields) and could contain any byte value including `\n`, so a delimiter would
//! need escaping the design doc's own "stdio, length-prefixed" note (`📓️design-runtime.md` §
//! "ShardTransport") already rules out. `tag` distinguishes a real `Data` frame (envelope bytes one
//! way, `ShardOutcome` pack bytes the other) from a `Heartbeat` frame (empty payload) so a periodic
//! liveness signal can interleave on the SAME pipe without the reader ever mis-decoding a heartbeat
//! as `Envelope::pack_decode` input or vice versa.

use semio_framework_actor::ShardTransport;
use std::collections::VecDeque;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
#[cfg(test)]
use std::thread;
#[cfg(test)]
use std::time::Duration;

const PIPE_READ_BYTES_PER_POLL: usize = 64 * 1024;
const PIPE_FRAMES_PER_POLL: usize = 32;
const MAX_FRAME_BYTES: usize = 16 * 1024 * 1024;

fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_millis() as u64)
}

//#region 🚪️NonblockingPipe
#[cfg(unix)]
fn prepare_nonblocking<R: std::os::fd::AsRawFd>(reader: &R) -> io::Result<()> {
    use std::os::raw::c_int;
    unsafe extern "C" {
        fn fcntl(fd: c_int, command: c_int, ...) -> c_int;
    }
    const F_GETFL: c_int = 3;
    const F_SETFL: c_int = 4;
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "openbsd", target_os = "netbsd", target_os = "dragonfly"))]
    const O_NONBLOCK: c_int = 0x0004;
    #[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "freebsd", target_os = "openbsd", target_os = "netbsd", target_os = "dragonfly")))]
    const O_NONBLOCK: c_int = 0x0800;
    let descriptor = reader.as_raw_fd();
    let flags = unsafe { fcntl(descriptor, F_GETFL) };
    if flags < 0 || unsafe { fcntl(descriptor, F_SETFL, flags | O_NONBLOCK) } < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(unix)]
fn readable_bytes<R: std::os::fd::AsRawFd>(_reader: &R) -> io::Result<usize> {
    Ok(PIPE_READ_BYTES_PER_POLL)
}

#[cfg(windows)]
fn prepare_nonblocking<R: std::os::windows::io::AsRawHandle>(_reader: &R) -> io::Result<()> {
    Ok(())
}

#[cfg(windows)]
fn readable_bytes<R: std::os::windows::io::AsRawHandle>(reader: &R) -> io::Result<usize> {
    use std::ffi::c_void;
    unsafe extern "system" {
        fn PeekNamedPipe(handle: *mut c_void, buffer: *mut c_void, buffer_size: u32, bytes_read: *mut u32, total_available: *mut u32, bytes_left: *mut u32) -> i32;
    }
    let mut available = 0u32;
    let ok = unsafe { PeekNamedPipe(reader.as_raw_handle().cast(), std::ptr::null_mut(), 0, std::ptr::null_mut(), &mut available, std::ptr::null_mut()) };
    if ok == 0 {
        return Err(io::Error::last_os_error());
    }
    Ok((available as usize).min(PIPE_READ_BYTES_PER_POLL))
}
//#endregion 🚪️NonblockingPipe

//#region 📦️Framing
mod framing {
    use super::{io, Read, VecDeque, Write, MAX_FRAME_BYTES, PIPE_FRAMES_PER_POLL};

    pub const TAG_DATA: u8 = 0;
    pub const TAG_HEARTBEAT: u8 = 1;

    #[derive(Debug, PartialEq, Eq)]
    pub enum Frame {
        Data(Vec<u8>),
        Heartbeat,
    }

    #[derive(Default)]
    pub struct Decoder {
        bytes: Vec<u8>,
        frames: VecDeque<Frame>,
    }

    #[derive(Debug)]
    pub enum PipeState {
        Open,
        Eof,
    }

    /// ✍️ One frame: `[tag][len:u32 LE][payload]`. A single `lock()`-guarded writer (both
    /// [`super::ProcessTransport::send`]/`kill` on the parent and [`super::StdioTransport::send`]/
    /// its heartbeat thread on the child) must own the destination for the whole call — three
    /// separate `write_all`s interleaved with another thread's frame would corrupt the boundary,
    /// which is why both sides share one `Mutex`-guarded handle rather than re-acquiring
    /// `io::stdout()` per call.
    pub fn write_frame<W: Write>(writer: &mut W, tag: u8, payload: &[u8]) -> io::Result<()> {
        if payload.len() > MAX_FRAME_BYTES {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "process transport frame exceeds byte ceiling"));
        }
        writer.write_all(&[tag])?;
        writer.write_all(&(payload.len() as u32).to_le_bytes())?;
        writer.write_all(payload)?;
        writer.flush()
    }

    impl Decoder {
        pub fn poll<R: Read>(&mut self, reader: &mut R, readable: usize) -> io::Result<PipeState> {
            if readable > 0 {
                let mut chunk = vec![0u8; readable.min(super::PIPE_READ_BYTES_PER_POLL)];
                match reader.read(&mut chunk) {
                    Ok(0) => return Ok(PipeState::Eof),
                    Ok(read) => self.bytes.extend_from_slice(&chunk[..read]),
                    Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
                    Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
                    Err(error) => return Err(error),
                }
            }
            self.decode()?;
            Ok(PipeState::Open)
        }

        pub fn pop(&mut self) -> Option<Frame> {
            self.frames.pop_front()
        }

        fn decode(&mut self) -> io::Result<()> {
            for _ in 0..PIPE_FRAMES_PER_POLL {
                if self.bytes.len() < 5 {
                    break;
                }
                let len = u32::from_le_bytes(self.bytes[1..5].try_into().expect("five-byte frame prefix")) as usize;
                if len > MAX_FRAME_BYTES {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "process transport frame exceeds byte ceiling"));
                }
                let frame_len = 5usize.checked_add(len).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "process transport frame length overflow"))?;
                if self.bytes.len() < frame_len {
                    break;
                }
                let tag = self.bytes[0];
                let payload = self.bytes[5..frame_len].to_vec();
                self.bytes.drain(..frame_len);
                self.frames.push_back(if tag == TAG_HEARTBEAT { Frame::Heartbeat } else { Frame::Data(payload) });
            }
            Ok(())
        }
    }

    #[cfg(test)]
    pub fn read_frame<R: Read>(reader: &mut R) -> io::Result<Option<Frame>> {
        let mut tag = [0u8; 1];
        match reader.read_exact(&mut tag) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(error) => return Err(error),
        }
        let mut len_bytes = [0u8; 4];
        reader.read_exact(&mut len_bytes)?;
        let len = u32::from_le_bytes(len_bytes) as usize;
        if len > MAX_FRAME_BYTES {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "process transport frame exceeds byte ceiling"));
        }
        let mut payload = vec![0u8; len];
        reader.read_exact(&mut payload)?;
        Ok(Some(if tag[0] == TAG_HEARTBEAT { Frame::Heartbeat } else { Frame::Data(payload) }))
    }
}
//#endregion 📦️Framing

//#region 🖥️ProcessTransport
/// 🖥️ Parent-side `ShardTransport`: spawns a `semio-shard` child, writes `Envelope` bytes to its
/// stdin, and incrementally reads `ShardOutcome` bytes off nonblocking stdout during bounded
/// transport turns (mirrors
/// `ThreadTransport`'s `Mutex<Receiver<Vec<u8>>>` shape — `recv()` must never block past whatever is
/// ALREADY buffered, `ShardLoop::pump`'s own drain-loop contract). `heartbeat()` is wall-clock ms of
/// the last frame (data OR heartbeat) seen from the child; [`Self::is_child_alive`] is the EOF
/// signal beyond the trait's own surface (a concrete-type-only method, same reasoning `ThreadTransport
/// ::beat` uses for its own extra, non-trait method) — `ProcessShardWatchdog` (below) reads both.
pub struct ProcessTransport {
    child: Mutex<Child>,
    stdin: Mutex<ChildStdin>,
    stdout: Mutex<ChildStdout>,
    decoder: Mutex<framing::Decoder>,
    inbound: Mutex<VecDeque<Vec<u8>>>,
    heartbeat_ms: Arc<AtomicU64>,
    alive: Arc<AtomicBool>,
    killed: Arc<AtomicBool>,
}

impl ProcessTransport {
    /// 🚀️ Spawns `program args…` with piped stdin/stdout (stderr inherited — the child's own
    /// `[DEBUG]`/panic output belongs in the PARENT's own log, not silently swallowed) and starts the
    /// stdout in nonblocking mode. [`ShardTransport::recv`] performs at most one 64 KiB read and 32
    /// frame decodes per turn, so pipe I/O consumes finite shared-pool work and owns no thread.
    pub async fn spawn(program: &Path, args: &[String]) -> io::Result<Self> {
        let mut child = Command::new(program).args(args).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::inherit()).spawn()?;
        let stdin = child.stdin.take().expect("Command::stdin(Stdio::piped()) guarantees Some");
        let stdout = child.stdout.take().expect("Command::stdout(Stdio::piped()) guarantees Some");
        prepare_nonblocking(&stdout)?;
        let heartbeat_ms = Arc::new(AtomicU64::new(now_ms()));
        let alive = Arc::new(AtomicBool::new(true));

        Ok(Self { child: Mutex::new(child), stdin: Mutex::new(stdin), stdout: Mutex::new(stdout), decoder: Mutex::new(framing::Decoder::default()), inbound: Mutex::new(VecDeque::new()), heartbeat_ms, alive, killed: Arc::new(AtomicBool::new(false)) })
    }

    fn poll_pipe(&self) {
        if !self.alive.load(Ordering::SeqCst) {
            return;
        }
        let Ok(mut stdout) = self.stdout.lock() else {
            self.alive.store(false, Ordering::SeqCst);
            return;
        };
        let Ok(mut decoder) = self.decoder.lock() else {
            self.alive.store(false, Ordering::SeqCst);
            return;
        };
        let state = readable_bytes(&*stdout).and_then(|readable| decoder.poll(&mut *stdout, readable));
        if matches!(state, Ok(framing::PipeState::Eof) | Err(_)) {
            self.alive.store(false, Ordering::SeqCst);
        }
        while let Some(frame) = decoder.pop() {
            self.heartbeat_ms.store(now_ms(), Ordering::SeqCst);
            if let framing::Frame::Data(bytes) = frame {
                if let Ok(mut inbound) = self.inbound.lock() {
                    inbound.push_back(bytes);
                }
            }
        }
    }

    /// 💀️ `false` once a bounded pipe poll has observed EOF/an error on the child's stdout — the
    /// fast, definitive half of "the parent detects it (heartbeat/EOF)"; [`ShardTransport::
    /// heartbeat`] going stale is the slower half (a hung-but-not-dead child, e.g. `SIGSTOP`).
    pub async fn is_child_alive(&self) -> bool {
        self.poll_pipe();
        self.alive.load(Ordering::SeqCst)
    }

    /// 🪪️ The child's OS pid — for logging, and for a test/demo driver that wants to simulate an
    /// INVOLUNTARY death (`kill -9 <pid>` from outside this type, i.e. NOT via [`Self::kill`]) to
    /// prove detection rather than merely proving this type's own `kill()` works.
    pub async fn child_id(&self) -> Option<u32> {
        self.child.lock().ok().map(|child| child.id())
    }
}

impl ShardTransport for ProcessTransport {
    async fn send(&self, bytes: &[u8]) {
        if self.killed.load(Ordering::SeqCst) {
            return;
        }
        if let Ok(mut stdin) = self.stdin.lock() {
            let _ = framing::write_frame(&mut *stdin, framing::TAG_DATA, bytes);
        }
    }

    async fn recv(&self) -> Option<Vec<u8>> {
        self.poll_pipe();
        self.inbound.lock().ok().and_then(|mut inbound| inbound.pop_front())
    }

    async fn heartbeat(&self) -> u64 {
        self.poll_pipe();
        self.heartbeat_ms.load(Ordering::SeqCst)
    }

    /// 🔪️ Deliberate, parent-initiated termination — `design`'s "kill() that actually terminates the
    /// child": sends the child a hard kill (`Child::kill`, `SIGKILL` on unix) and reaps it so it
    /// never lingers as a zombie. Distinct from the involuntary-death path [`Self::is_child_alive`]/
    /// `heartbeat()` detect — this is the CAUSE, that is the SYMPTOM a supervisor observes when the
    /// cause was something else (an external `kill -9`, a real crash).
    async fn kill(&self) {
        self.killed.store(true, Ordering::SeqCst);
        if let Ok(mut child) = self.child.lock() {
            let _ = child.kill();
            let _ = child.wait();
        }
        self.alive.store(false, Ordering::SeqCst);
    }
}

impl Drop for ProcessTransport {
    fn drop(&mut self) {
        if !self.killed.load(Ordering::SeqCst) {
            self.killed.store(true, Ordering::SeqCst);
            if let Ok(mut child) = self.child.lock() {
                let _ = child.kill();
                let _ = child.wait();
            }
            self.alive.store(false, Ordering::SeqCst);
        }
    }
}
//#endregion 🖥️ProcessTransport

//#region 👶️StdioTransport
/// 👶️ Child-side mirror of [`ProcessTransport`] — hosted by `semio-shard` (`../👶️child/🦀️.rs`)
/// so its own `ShardLoop` can be driven exactly like a `ThreadTransport`-backed one, per
/// `🧵️shard/🦀️.rs`'s own module doc ("this file never branches on which [`ShardTransport`]
/// impl it got"). Sends `ShardOutcome` bytes on stdout, receives `Envelope` bytes on stdin, and runs
/// a background thread that emits a `Heartbeat` frame every `heartbeat_interval_ms` — the "real
/// heartbeat the parent can observe" the P1 packet brief asks for, independent of whether this shard
/// currently has any outcome to send (an idle shard must still prove it is alive).
pub struct StdioTransport {
    stdout: Arc<Mutex<io::Stdout>>,
    stdin: Mutex<io::Stdin>,
    decoder: Mutex<framing::Decoder>,
    inbound: Mutex<VecDeque<Vec<u8>>>,
    alive: Arc<AtomicBool>,
    _heartbeat: super::PeriodicPoolTimer,
}

impl StdioTransport {
    pub async fn new(heartbeat_interval_ms: u64) -> Self {
        let stdout = Arc::new(Mutex::new(io::stdout()));
        let stdin = io::stdin();
        prepare_nonblocking(&stdin).expect("configure shard stdin as nonblocking");
        let alive = Arc::new(AtomicBool::new(true));

        // 🧵️ P1f: a periodic sleep+write, not a blocking pipe read — driven off the shared
        // `WorkerPool`'s `Lane::Timer` ([`super::PeriodicPoolTimer`], the same mechanism
        // `EpochTicker` uses) instead of a dedicated `"semio-shard-heartbeat"` OS thread.
        // `super::plugin_host_worker_pool()` is already constructed in this process by the time
        // `StdioTransport::new` runs — `👶️child/🦀️.rs`'s `main` builds a `WasmtimeRuntime` (which
        // starts its own `EpochTicker` on this same singleton) before it ever opens the transport.
        let heartbeat = {
            let alive = alive.clone();
            let stdout = stdout.clone();
            super::PeriodicPoolTimer::start(&super::plugin_host_worker_pool(), super::Lane::Timer, heartbeat_interval_ms, move || {
                if !alive.load(Ordering::SeqCst) {
                    return false;
                }
                let Ok(mut guard) = stdout.lock() else { return false };
                framing::write_frame(&mut *guard, framing::TAG_HEARTBEAT, &[]).is_ok()
            })
        };

        Self { stdout, stdin: Mutex::new(stdin), decoder: Mutex::new(framing::Decoder::default()), inbound: Mutex::new(VecDeque::new()), alive, _heartbeat: heartbeat }
    }

    fn poll_pipe(&self) {
        if !self.alive.load(Ordering::SeqCst) {
            return;
        }
        let Ok(mut stdin) = self.stdin.lock() else {
            self.alive.store(false, Ordering::SeqCst);
            return;
        };
        let Ok(mut decoder) = self.decoder.lock() else {
            self.alive.store(false, Ordering::SeqCst);
            return;
        };
        let state = readable_bytes(&*stdin).and_then(|readable| decoder.poll(&mut *stdin, readable));
        if matches!(state, Ok(framing::PipeState::Eof) | Err(_)) {
            self.alive.store(false, Ordering::SeqCst);
        }
        while let Some(frame) = decoder.pop() {
            if let framing::Frame::Data(bytes) = frame {
                if let Ok(mut inbound) = self.inbound.lock() {
                    inbound.push_back(bytes);
                }
            }
        }
    }
}

impl ShardTransport for StdioTransport {
    async fn send(&self, bytes: &[u8]) {
        if let Ok(mut guard) = self.stdout.lock() {
            let _ = framing::write_frame(&mut *guard, framing::TAG_DATA, bytes);
        }
    }

    async fn recv(&self) -> Option<Vec<u8>> {
        self.poll_pipe();
        self.inbound.lock().ok().and_then(|mut queue| queue.pop_front())
    }

    /// 🚧️ No caller in this crate reads a `StdioTransport`'s own `heartbeat()` (the CHILD never
    /// needs to watch itself for liveness) — `0` is an honest "not tracked here", matching
    /// `LoopbackTransport::heartbeat`'s own precedent in `🧵️shard/🦀️.rs`'s test module.
    async fn heartbeat(&self) -> u64 {
        0
    }

    async fn kill(&self) {
        self.alive.store(false, Ordering::SeqCst);
    }
}
//#endregion 👶️StdioTransport

//#region 🧭️Selection
/// 🧭️ P1 packet brief: "wire it into the native host behind a flag/config, so process shards are
/// selectable instead of thread shards" — never the default this packet (`📌️important.md`'s P1
/// scope). Repo-wide grep before writing this (`grep -rn "ThreadTransport::new_pair"`) found ZERO
/// non-test callers: no live `Kernel`/`ShardTable` constructs a shard of EITHER kind outside
/// `🧵️shard/🦀️.rs`'s own `#[cfg(test)]` module and `🦀️.rs`'s wasmtime tests. This
/// enum is therefore the selection SEAM a future scheduler wires through — landed ahead of its
/// caller, the same "land the seam, not a synthesized caller" shape `design-runtime.md`'s own
/// `GuestRuntime` trait + `MockGuestRuntime` landed in before `WasmtimeRuntime` existed — not a
/// switch with an existing thread-shard call site on the other end of it. Documented gap, not a
/// faked wiring; see the P1 report's `## gaps`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShardRuntimeKind {
    Thread,
    Process,
}

impl ShardRuntimeKind {
    /// 🚩️ `SEMIO_SHARD_KIND=process` opts in; anything else (including unset) is `Thread`.
    pub async fn from_env() -> Self {
        match std::env::var("SEMIO_SHARD_KIND").ok().as_deref() {
            Some("process") => ShardRuntimeKind::Process,
            _ => ShardRuntimeKind::Thread,
        }
    }
}
//#endregion 🧭️Selection

//#region 🚑️Watchdog
/// 🚑️ Native counterpart of `🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`'s
/// `ShardClient.checkHeartbeats` — SAME semantics, deliberately: three CONSECUTIVE stale windows
/// (never three raw `poll` calls) before a shard counts as lost, not one. The web class watches
/// SAB/`postMessage` heartbeats against in-flight-request age; this watches
/// `ProcessTransport::heartbeat()` (wall-clock ms of the last frame seen) against wall-clock `now`,
/// since a process shard's "in flight" concept (this packet does not build a request-tracking layer
/// — that is scheduler territory, still unbuilt per `//#region 🧭️Selection`'s own finding) doesn't
/// exist yet; a shard that has gone silent for `timeout_ms` counts the same as one with a stuck
/// request. [`Self::poll_with_liveness`] adds the EOF fast path `ProcessTransport::is_child_alive`
/// gives that the web transport has no equivalent of (a `Worker` never reports "my process is gone"
/// separately from "no heartbeat yet").
pub struct ProcessShardWatchdog {
    timeout_ms: u64,
    last_seen_ms: u64,
    last_miss_counted_ms: u64,
    missed: u32,
}

impl ProcessShardWatchdog {
    pub async fn new(timeout_ms: u64, now_ms: u64) -> Self {
        Self { timeout_ms, last_seen_ms: now_ms, last_miss_counted_ms: now_ms, missed: 0 }
    }

    /// ▶️ Call periodically with the transport's own `heartbeat()` reading. `true` once three
    /// consecutive `timeout_ms` windows have elapsed with no fresher heartbeat — mirrors
    /// `ShardClient.checkHeartbeats`'s `HEARTBEAT_MISSED_LIMIT = 3`.
    pub async fn poll(&mut self, heartbeat_ms: u64, now_ms: u64) -> bool {
        if heartbeat_ms > self.last_seen_ms {
            self.last_seen_ms = heartbeat_ms;
            self.last_miss_counted_ms = now_ms;
            self.missed = 0;
            return false;
        }
        let silent_for = now_ms.saturating_sub(self.last_seen_ms);
        if silent_for <= self.timeout_ms {
            return false;
        }
        if now_ms.saturating_sub(self.last_miss_counted_ms) < self.timeout_ms {
            return false;
        }
        self.missed += 1;
        self.last_miss_counted_ms = now_ms;
        self.missed >= 3
    }

    /// ▶️ [`Self::poll`] plus the EOF fast path: an observed-dead child is lost IMMEDIATELY, never
    /// waiting out three heartbeat windows for a signal that has already definitively arrived.
    pub async fn poll_with_liveness(&mut self, heartbeat_ms: u64, child_alive: bool, now_ms: u64) -> bool {
        if !child_alive {
            return true;
        }
        self.poll(heartbeat_ms, now_ms).await
    }
}
//#endregion 🚑️Watchdog

#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧪️FramingTests
    #[semio_framework_async_macros::async_test]
    async fn a_data_frame_round_trips_arbitrary_bytes_including_zero_and_newline() {
        let payload = vec![0u8, 1, 2, b'\n', 255, 0u8];
        let mut buffer = Vec::new();
        framing::write_frame(&mut buffer, framing::TAG_DATA, &payload).expect("write");
        let mut cursor = io::Cursor::new(buffer);
        match framing::read_frame(&mut cursor).expect("read").expect("some frame") {
            framing::Frame::Data(bytes) => assert_eq!(bytes, payload),
            framing::Frame::Heartbeat => panic!("expected Data"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn a_heartbeat_frame_carries_no_payload_and_does_not_desync_the_next_frame() {
        let mut buffer = Vec::new();
        framing::write_frame(&mut buffer, framing::TAG_HEARTBEAT, &[]).expect("write heartbeat");
        framing::write_frame(&mut buffer, framing::TAG_DATA, b"after").expect("write data");
        let mut cursor = io::Cursor::new(buffer);
        assert!(matches!(framing::read_frame(&mut cursor).expect("read").expect("some"), framing::Frame::Heartbeat));
        match framing::read_frame(&mut cursor).expect("read").expect("some") {
            framing::Frame::Data(bytes) => assert_eq!(bytes, b"after"),
            framing::Frame::Heartbeat => panic!("expected Data"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn read_frame_reports_clean_eof_as_none_not_an_error() {
        let mut cursor = io::Cursor::new(Vec::<u8>::new());
        assert!(framing::read_frame(&mut cursor).expect("EOF is Ok(None), not Err").is_none());
    }

    #[test]
    fn nonblocking_decoder_preserves_fragmented_and_concatenated_frames() {
        let mut encoded = Vec::new();
        framing::write_frame(&mut encoded, framing::TAG_DATA, b"first").expect("first");
        framing::write_frame(&mut encoded, framing::TAG_HEARTBEAT, &[]).expect("heartbeat");
        framing::write_frame(&mut encoded, framing::TAG_DATA, b"second").expect("second");
        let split_a = 3;
        let split_b = encoded.len() - 4;
        let mut decoder = framing::Decoder::default();
        for fragment in [&encoded[..split_a], &encoded[split_a..split_b], &encoded[split_b..]] {
            let mut cursor = io::Cursor::new(fragment);
            assert!(matches!(decoder.poll(&mut cursor, fragment.len()).expect("decode fragment"), framing::PipeState::Open));
        }
        assert_eq!(decoder.pop(), Some(framing::Frame::Data(b"first".to_vec())));
        assert_eq!(decoder.pop(), Some(framing::Frame::Heartbeat));
        assert_eq!(decoder.pop(), Some(framing::Frame::Data(b"second".to_vec())));
        assert_eq!(decoder.pop(), None);
    }

    #[test]
    fn nonblocking_decoder_rejects_oversized_prefix_before_payload_allocation() {
        let mut prefix = vec![framing::TAG_DATA];
        prefix.extend_from_slice(&((MAX_FRAME_BYTES as u32) + 1).to_le_bytes());
        let mut cursor = io::Cursor::new(prefix.clone());
        let mut decoder = framing::Decoder::default();
        let error = decoder.poll(&mut cursor, prefix.len()).expect_err("oversized prefix must fail");
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }
    //#endregion 🧪️FramingTests

    //#region 🧪️SelectionTests
    #[semio_framework_async_macros::async_test]
    async fn shard_runtime_kind_defaults_to_thread_and_opts_into_process_explicitly() {
        // 🧯️ Env-var mutation makes this test order-sensitive vs. any OTHER test reading the same
        // var in the same process; none exists in this crate today (grepped before writing), and
        // `cargo test`'s default multi-threaded runner still serializes same-process env access
        // adequately for a single var this test both sets and restores.
        let previous = std::env::var("SEMIO_SHARD_KIND").ok();
        std::env::remove_var("SEMIO_SHARD_KIND");
        assert_eq!(ShardRuntimeKind::from_env().await, ShardRuntimeKind::Thread, "unset must default to Thread — never Process by default in P1");
        std::env::set_var("SEMIO_SHARD_KIND", "process");
        assert_eq!(ShardRuntimeKind::from_env().await, ShardRuntimeKind::Process);
        match previous {
            Some(value) => std::env::set_var("SEMIO_SHARD_KIND", value),
            None => std::env::remove_var("SEMIO_SHARD_KIND"),
        }
    }
    //#endregion 🧪️SelectionTests

    //#region 🧪️WatchdogTests
    #[semio_framework_async_macros::async_test]
    async fn watchdog_does_not_fire_while_heartbeats_keep_advancing() {
        let mut watchdog = ProcessShardWatchdog::new(1000, 0).await;
        assert!(!watchdog.poll(100, 100).await);
        assert!(!watchdog.poll(1200, 1300).await);
        assert!(!watchdog.poll(2500, 2600).await);
    }

    #[semio_framework_async_macros::async_test]
    async fn watchdog_fires_after_three_consecutive_stale_windows() {
        let mut watchdog = ProcessShardWatchdog::new(1000, 0).await;
        // heartbeat frozen at 0 forever — three separate `timeout_ms`-apart polls must each count
        // exactly one miss, matching `ShardClient`'s `lastMissCountedAtMs` gate (a flurry of polls
        // inside the SAME window must not multi-count).
        assert!(!watchdog.poll(0, 1500).await);
        assert!(!watchdog.poll(0, 1600).await, "same window as the previous miss — must not double-count");
        assert!(!watchdog.poll(0, 2600).await);
        assert!(watchdog.poll(0, 3700).await, "third consecutive stale window — must fire");
    }

    #[semio_framework_async_macros::async_test]
    async fn watchdog_resets_the_miss_count_on_a_fresh_heartbeat() {
        let mut watchdog = ProcessShardWatchdog::new(1000, 0).await;
        assert!(!watchdog.poll(0, 1500).await);
        assert!(!watchdog.poll(0, 2600).await);
        assert!(!watchdog.poll(5000, 5000).await, "a fresh heartbeat must reset the miss streak");
        assert!(!watchdog.poll(5000, 6600).await);
        assert!(!watchdog.poll(5000, 7700).await);
        assert!(watchdog.poll(5000, 8800).await, "three fresh consecutive misses after the reset");
    }

    #[semio_framework_async_macros::async_test]
    async fn poll_with_liveness_fires_immediately_on_a_dead_child_without_waiting_out_the_timeout() {
        let mut watchdog = ProcessShardWatchdog::new(1000, 0).await;
        assert!(watchdog.poll_with_liveness(0, false, 5).await, "EOF is definitive — no need to wait for three stale windows");
    }
    //#endregion 🧪️WatchdogTests

    //#region 🧪️ProcessTransportTests
    /// 🎯️ The one test in this module that spawns a REAL child process (not the `semio-shard`
    /// binary — a plain `cat`, which echoes stdin to stdout byte-for-byte) — proves `ProcessTransport
    /// ::spawn`/`send`/`recv`/`kill` against an actual OS process without needing a wasmtime
    /// component built first. The `semio-shard`-hosted, real-wasmtime-actor version of this same
    /// proof (kill -9, detect, rebuild, sibling unaffected) is `👶️child/🦀️.rs`'s own
    /// `#[ignore]`d integration test — see the P1 report's `## kill-rebuild-evidence`.
    #[semio_framework_async_macros::async_test]
    async fn process_transport_round_trips_bytes_through_a_real_child_process() {
        // 👶️ host-dedyn: `#[test] fn` is a sanctioned `block_on` entry point (R4 clause 5) —
        // `ShardTransport`'s methods are `async fn` now (O1); every impl here resolves on its
        // first poll (pure `Mutex`/`AtomicBool`/pipe I/O, no real suspension), so `block_on` never
        // actually parks.
        let transport = ProcessTransport::spawn(Path::new("cat"), &[]).await.expect("spawn cat");
        semio_framework_async::block_on(transport.send(b"hello-process-shard"));
        let mut received = None;
        for _ in 0..200 {
            if let Some(bytes) = semio_framework_async::block_on(transport.recv()) {
                received = Some(bytes);
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
        assert_eq!(received, Some(b"hello-process-shard".to_vec()));
        semio_framework_async::block_on(transport.kill());
    }

    #[semio_framework_async_macros::async_test]
    async fn kill_terminates_the_child_and_is_observed_as_eof_on_recv_side() {
        let transport = ProcessTransport::spawn(Path::new("cat"), &[]).await.expect("spawn cat");
        assert!(transport.is_child_alive().await);
        semio_framework_async::block_on(transport.kill());
        let mut dead = false;
        for _ in 0..200 {
            if !transport.is_child_alive().await {
                dead = true;
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
        assert!(dead, "reader thread must observe EOF after kill()");
    }

    #[semio_framework_async_macros::async_test]
    async fn an_externally_killed_child_is_detected_as_dead_without_this_type_calling_kill() {
        let transport = ProcessTransport::spawn(Path::new("sleep"), &["30".to_string()]).await.expect("spawn sleep 30");
        let pid = transport.child_id().await.expect("pid");
        // 🔪️ An INVOLUNTARY death — `kill -9` from OUTSIDE this type, mirroring the packet's
        // required proof ("kill -9 a shard child -> the parent detects it") rather than merely
        // exercising this type's OWN `kill()` method (the test above already covers that).
        let status = Command::new("kill").args(["-9", &pid.to_string()]).status().expect("run kill -9");
        assert!(status.success());
        let mut dead = false;
        for _ in 0..300 {
            if !transport.is_child_alive().await {
                dead = true;
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
        assert!(dead, "an externally SIGKILLed child must be observed as dead via EOF");
    }
    //#endregion 🧪️ProcessTransportTests

    //#region 🧪️KillRebuildEvidence
    /// 🎯️ P1's headline acceptance proof, against a REAL `semio-shard` child hosting a REAL
    /// `wasm32-wasip2 world actor` component (the F1 scale fixture) through a REAL `WasmtimeRuntime`
    /// — not `MockGuestRuntime`, not an in-process loopback. `#[ignore]`d (not part of the default
    /// 74-test baseline this packet must not regress) because it needs a pre-built component:
    ///
    /// ```text
    /// CARGO_TARGET_DIR=<ticket>/🎯️target-p1 cargo component build -p semio-framework-os-scale-fixture \
    ///   --target wasm32-wasip2 --features component-guest
    /// SEMIO_SCALE_FIXTURE_WASM=<ticket>/🎯️target-p1/wasm32-wasip2/wasm-dev/semio_framework_os_scale_fixture.wasm \
    ///   cargo test -p semio-framework-plugin-host --lib -- --ignored process_shard_kill_is_detected
    /// ```
    ///
    /// Sequence: spawn TWO process shards (`a`, `b`), activate one actor on each (`InstanceOpen`
    /// with the fixture's `idle` profile), confirm both reply with a real `ShardOutcome::Turn`.
    /// `kill -9` shard `a`'s child from OUTSIDE this process's own `ProcessTransport::kill()` (the
    /// packet's exact requirement: "kill -9 a shard child -> the parent detects it"). Poll
    /// `ProcessShardWatchdog` until it reports the shard lost. Spawn a FRESH child at the same
    /// routing slot ("rebuild"), activate a fresh actor on it, confirm it replies — proving the
    /// shard is usable again. Throughout, shard `b` — untouched — must still answer a SECOND turn,
    /// proving the failure was isolated to `a`.
    #[semio_framework_async_macros::async_test]
    #[ignore = "needs a pre-built wasm32-wasip2 component at SEMIO_SCALE_FIXTURE_WASM; see this test's own doc comment"]
    async fn process_shard_kill_is_detected_and_the_shard_rebuilds_while_a_sibling_shard_stays_healthy() {
        let Ok(wasm_path) = std::env::var("SEMIO_SCALE_FIXTURE_WASM") else {
            eprintln!("[skip] SEMIO_SCALE_FIXTURE_WASM not set — see this test's doc comment for the build command");
            return;
        };
        let shard_bin = std::env::var("CARGO_BIN_EXE_semio-shard").expect("cargo test sets CARGO_BIN_EXE_semio-shard for this package's own [[bin]] target");

        let shard_a = ProcessTransport::spawn(Path::new(&shard_bin), &[wasm_path.clone(), "scale-fixture-a".to_string(), "1".to_string()]).await.expect("spawn shard a");
        let shard_b = ProcessTransport::spawn(Path::new(&shard_bin), &[wasm_path.clone(), "scale-fixture-b".to_string(), "2".to_string()]).await.expect("spawn shard b");

        semio_framework_async::block_on(shard_a.send(&instance_open_envelope(1, 1, "idle").await));
        semio_framework_async::block_on(shard_b.send(&instance_open_envelope(2, 1, "idle").await));

        let outcome_a = recv_outcome(&shard_a, 400).await.expect("shard a must reply to InstanceOpen with a real ShardOutcome");
        let outcome_b = recv_outcome(&shard_b, 400).await.expect("shard b must reply to InstanceOpen with a real ShardOutcome");
        assert!(matches!(outcome_a, crate::shard::ShardOutcome::Turn { actor: 1, .. }), "shard a: expected ShardOutcome::Turn, got {outcome_a:?}");
        assert!(matches!(outcome_b, crate::shard::ShardOutcome::Turn { actor: 2, .. }), "shard b: expected ShardOutcome::Turn, got {outcome_b:?}");

        let pid_a = shard_a.child_id().await.expect("shard a pid");
        // 🔪️ Involuntary death, exactly the packet's required proof — `kill -9` from OUTSIDE this
        // process's own `ProcessTransport::kill()`.
        let status = Command::new("kill").args(["-9", &pid_a.to_string()]).status().expect("run kill -9 on shard a");
        assert!(status.success(), "kill -9 shard a must succeed");

        let mut watchdog = ProcessShardWatchdog::new(500, now_ms()).await;
        let mut lost = false;
        for _ in 0..100 {
            thread::sleep(Duration::from_millis(50));
            if watchdog.poll_with_liveness(semio_framework_async::block_on(shard_a.heartbeat()), shard_a.is_child_alive().await, now_ms()).await {
                lost = true;
                break;
            }
        }
        assert!(lost, "the watchdog must detect shard a as lost after the external kill -9");

        // ▶️ Rebuild: a fresh child at a fresh actor id (a real restart would restore-from-checkpoint
        // here — out of this test's scope, `GuestRuntime::checkpoint`/`restore` are proven separately
        // by `🧵️shard/🦀️.rs`'s K1 tests; this proves the PROCESS half of rebuild).
        let shard_a2 = ProcessTransport::spawn(Path::new(&shard_bin), &[wasm_path, "scale-fixture-a".to_string(), "3".to_string()]).await.expect("rebuild shard a");
        semio_framework_async::block_on(shard_a2.send(&instance_open_envelope(3, 1, "idle").await));
        let outcome_a2 = recv_outcome(&shard_a2, 400).await.expect("rebuilt shard a must reply");
        assert!(matches!(outcome_a2, crate::shard::ShardOutcome::Turn { actor: 3, .. }), "rebuilt shard a: expected ShardOutcome::Turn, got {outcome_a2:?}");

        // 🎯️ Sibling isolation: shard b, never touched, is still alive and answers a SECOND turn.
        assert!(shard_b.is_child_alive().await, "shard b must be unaffected by shard a's death");
        semio_framework_async::block_on(shard_b.send(&wake_envelope(2, 2).await));
        let outcome_b2 = recv_outcome(&shard_b, 400).await.expect("shard b must still respond after shard a's kill+rebuild");
        assert!(matches!(outcome_b2, crate::shard::ShardOutcome::Turn { actor: 2, .. }), "shard b second turn: expected ShardOutcome::Turn, got {outcome_b2:?}");

        semio_framework_async::block_on(shard_a2.kill());
        semio_framework_async::block_on(shard_b.kill());
    }

    /// terra-shard-grants: wraps in `crate::shard::ShardFrame::Envelope` — the wire now carries
    /// `ShardFrame`, not raw `Envelope` bytes (`ShardLoop::pump`'s own change), so this fixture's
    /// hand-rolled encoder must wrap here too, even though its one caller is `#[ignore]`d.
    async fn encode_envelope(actor: u64, seq: u64, payload: semio_framework_actor::Payload) -> Vec<u8> {
        let envelope =
            semio_framework_actor::Envelope { to: semio_framework_actor::ActorId(actor), from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Interactive, seq, deadline_ms: None, coalesce: None, cancel_of: None, payload };
        let mut bytes = Vec::new();
        crate::shard::ShardFrame::Envelope(envelope).pack_encode(&mut bytes).await;
        bytes
    }

    async fn instance_open_envelope(actor: u64, seq: u64, profile: &str) -> Vec<u8> {
        let config = format!("{{\"profile\":\"{profile}\"}}").into_bytes();
        let event = semio_framework::kernel::Event::InstanceOpen {
            request: semio_framework::kernel::ActorInstanceOpenRequest { activation_generation: 1, instance_id: u32::try_from(actor).expect("fixture actor fits instance id"), request_sequence: seq },
            app_id: semio_framework::kernel::AppInstanceId("scale-fixture".to_string()),
            actor: actor.to_string(),
            config,
            assets: Vec::new(),
            capabilities: Vec::new(),
            quotas: semio_framework::kernel::QuotaSchema::default(),
        };
        let event_bytes = serde_json::to_vec(&event).expect("encode Event::InstanceOpen");
        encode_envelope(actor, seq, semio_framework_actor::Payload::Event { bytes: event_bytes }).await
    }

    async fn wake_envelope(actor: u64, seq: u64) -> Vec<u8> {
        let event_bytes = serde_json::to_vec(&semio_framework::kernel::Event::Wake).expect("encode Event::Wake");
        encode_envelope(actor, seq, semio_framework_actor::Payload::Event { bytes: event_bytes }).await
    }

    async fn recv_outcome(transport: &ProcessTransport, attempts: u32) -> Option<crate::shard::ShardOutcome> {
        for _ in 0..attempts {
            if let Some(bytes) = semio_framework_async::block_on(transport.recv()) {
                let mut pos = 0usize;
                return semio_framework_async::block_on(crate::shard::ShardOutcome::pack_decode(&bytes, &mut pos)).ok();
            }
            thread::sleep(Duration::from_millis(50));
        }
        None
    }
    //#endregion 🧪️KillRebuildEvidence
}
