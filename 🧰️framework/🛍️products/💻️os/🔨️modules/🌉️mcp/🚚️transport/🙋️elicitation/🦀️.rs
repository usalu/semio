//! 🙋️ Elicitation of the consent crate: Stdio lines, elicitation and the approval coordinator.

#[cfg(test)]
use axum::response::IntoResponse;
use std::collections::VecDeque;
use std::io::BufRead;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_framework_os_mcp_wire::protocol::{ClientFeatures, intercept_cancellation};

//#region 🔖️StdioTransport

fn io_error(error: std::io::Error) -> GatewayError {
    GatewayError::new(GatewayErrorCode::Internal, format!("stdio transport io error: {error}"))
}

/// 📻️ The one duplex line channel a stdio connection owns. Both the serve loop AND a server-initiated
/// request made from INSIDE a tool call (`elicitation/create`) read and write through this single
/// owner, so exactly one component ever touches the real descriptors. A line the elicitation wait
/// reads that is not its own response is pushed onto `deferred`, and the serve loop drains that queue
/// before touching the stream again — the client's own ordering is preserved and nothing is dropped.
///
/// ⏱️ The client stream is drained by ONE owned reader thread into a channel rather than read
/// in place. That is what makes a wait on it deadline-bounded: `BufRead::read_line` blocks with no
/// timeout of any kind, so an elicitation waiting directly on the descriptor could never give up on
/// a client that goes silent (`📓️m4-mcp-bridge-approval-binding.md` §5.3). Every read still yields
/// lines in exact arrival order, and the thread ends on EOF, on an io error, or when the last
/// `StdioLines` is dropped. The one exception is `notifications/cancelled`, which this thread applies
/// immediately ([`crate::protocol::intercept_cancellation`]) so it interrupts the call still running.
pub struct StdioLines {
    pub output: Mutex<Box<dyn Write + Send>>,
    pub inbound: Mutex<std::sync::mpsc::Receiver<Result<String, String>>>,
    deferred: Mutex<VecDeque<String>>,
}

/// 📥️ What one bounded read of the client stream produced.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum StdioRead {
    Line(String),
    Eof,
    TimedOut,
}

impl StdioLines {
    pub fn new(input: Box<dyn BufRead + Send>, output: Box<dyn Write + Send>) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        let _ = std::thread::Builder::new().name("semio-mcp-stdio-reader".to_string()).spawn(move || {
            let mut input = input;
            loop {
                let mut line = String::new();
                match input.read_line(&mut line) {
                    Ok(0) => return,
                    Ok(_) => {
                        if intercept_cancellation(&line) {
                            continue;
                        }
                        if sender.send(Ok(line)).is_err() {
                            return;
                        }
                    }
                    Err(error) => {
                        let _ = sender.send(Err(error.to_string()));
                        return;
                    }
                }
            }
        });
        Self { output: Mutex::new(output), inbound: Mutex::new(receiver), deferred: Mutex::new(VecDeque::new()) }
    }

    pub fn write_line(&self, line: &str) -> Result<(), GatewayError> {
        let mut output = self.output.lock().expect("stdio line channel poisoned");
        writeln!(output, "{line}").map_err(io_error)?;
        output.flush().map_err(io_error)
    }

    /// 📥️ The next client line — a deferred one first, then a fresh one off the reader. `None` is EOF.
    pub fn read_line(&self) -> Result<Option<String>, GatewayError> {
        if let Some(line) = self.deferred.lock().expect("stdio deferred queue poisoned").pop_front() {
            return Ok(Some(line));
        }
        match self.inbound.lock().expect("stdio inbound channel poisoned").recv() {
            Ok(Ok(line)) => Ok(Some(line)),
            Ok(Err(error)) => Err(GatewayError::new(GatewayErrorCode::Internal, format!("stdio transport io error: {error}"))),
            Err(_) => Ok(None),
        }
    }

    /// 📥️ The next line straight off the client stream within `budget`, IGNORING the deferred queue —
    /// what an in-flight `elicitation/create` waits on. It must never re-read a line it just deferred
    /// (the answer it is waiting for has not been written by the client yet, so anything already
    /// queued is by construction not it), and the serve loop drains that queue afterwards in arrival
    /// order. A `budget` that elapses answers [`StdioRead::TimedOut`] and consumes nothing.
    pub(crate) fn read_line_direct_within(&self, budget: std::time::Duration) -> Result<StdioRead, GatewayError> {
        match self.inbound.lock().expect("stdio inbound channel poisoned").recv_timeout(budget) {
            Ok(Ok(line)) => Ok(StdioRead::Line(line)),
            Ok(Err(error)) => Err(GatewayError::new(GatewayErrorCode::Internal, format!("stdio transport io error: {error}"))),
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => Ok(StdioRead::TimedOut),
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => Ok(StdioRead::Eof),
        }
    }

    pub(crate) fn defer(&self, line: String) {
        self.deferred.lock().expect("stdio deferred queue poisoned").push_back(line);
    }
}

//#endregion 🔖️StdioTransport

//#region 🔖️Elicitation

/// 🙋 The MCP `elicitation/create` server→client request channel — the spec's own human-in-the-loop
/// primitive, and the FIRST place a destructive capability's approval is offered when the connected
/// client advertises `capabilities.elicitation` (`🛡️policy`'s `ApprovalCoordinator` owns the chain).
/// Late-bound through [`ElicitationSlot`] for the same reason `🖥️ui::BridgeSlot` is: the tool
/// registry is built before any transport exists.
pub type ElicitationSlot = Arc<OnceLock<Arc<ElicitationChannel>>>;

/// 🙅 Why an elicitation could not produce a decision — never confused with an explicit `Decline`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElicitationUnavailable {
    NoChannel,
    ClientDoesNotSupportIt,
    ClientClosed,
    TimedOut,
    Malformed(String),
}

/// ⏱️ How long an `elicitation/create` waits for the connected client's human before the lane is
/// declared closed. Deliberately the same budget as `🛡️policy`'s `SHELL_APPROVAL_TIMEOUT_MS`: a
/// human deciding in an MCP client and a human deciding in the OS shell get the same amount of time,
/// and a silent one closes its lane the same way rather than wedging the agent forever.
pub const ELICITATION_TIMEOUT_MS: u64 = 120_000;

/// 🕰️ The wall clock an elicitation deadline is measured against, injected so a test can drive the
/// deadline arithmetic without spending the deadline. `now_ms` is monotonic, in milliseconds, from
/// an arbitrary origin — only differences are ever read.
pub trait ElicitationClock: Send + Sync {
    fn now_ms(&self) -> u64;
}

/// 🕰️ The real clock: `Instant` elapsed since the channel was built.
pub struct SystemElicitationClock {
    pub origin: std::time::Instant,
}

impl Default for SystemElicitationClock {
    fn default() -> Self {
        Self { origin: std::time::Instant::now() }
    }
}

impl ElicitationClock for SystemElicitationClock {
    fn now_ms(&self) -> u64 {
        u64::try_from(self.origin.elapsed().as_millis()).unwrap_or(u64::MAX)
    }
}

/// ⏲️ One tick of an [`ElicitationClock`]. A floored millisecond reading can sit up to one tick
/// behind the instant it was taken, so the difference of two readings can overstate the real wait by
/// up to one tick. The deadline is set one tick past the budget so the wait never ends before it.
const ELICITATION_CLOCK_TICK_MS: u64 = 1;

/// ⏳️ The longest a single bounded read blocks before the deadline is re-checked, so a fake clock
/// that jumps forward is noticed promptly instead of at the end of one enormous `recv_timeout`.
const ELICITATION_READ_SLICE_MS: u64 = 25;

/// 🗳️ The three `elicitation/create` result actions, verbatim from the MCP schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElicitationAction {
    Accept,
    Decline,
    Cancel,
}

pub struct ElicitationChannel {
    pub lines: Arc<StdioLines>,
    pub features: Arc<ClientFeatures>,
    pub next_id: AtomicU64,
    pub timeout_ms: u64,
    pub clock: Box<dyn ElicitationClock>,
}

impl ElicitationChannel {
    pub fn new(lines: Arc<StdioLines>, features: Arc<ClientFeatures>) -> Self {
        Self { lines, features, next_id: AtomicU64::new(1), timeout_ms: ELICITATION_TIMEOUT_MS, clock: Box::new(SystemElicitationClock::default()) }
    }

    /// ⏱️ Rebinds the wait's budget and the clock it is measured against — the seam the deadline tests
    /// drive, and the one a future launch flag would set.
    #[must_use]
    pub fn with_deadline(mut self, timeout_ms: u64, clock: Box<dyn ElicitationClock>) -> Self {
        self.timeout_ms = timeout_ms;
        self.clock = clock;
        self
    }

    pub fn client_supports_elicitation(&self) -> bool {
        self.features.elicitation.load(Ordering::SeqCst)
    }

    /// 🙋 Issues one `elicitation/create` and waits on THIS connection for the client's answer.
    /// Every other line that arrives meanwhile is deferred back onto the serve loop in arrival order,
    /// so a client that keeps sending requests while a human decides loses nothing. The wait ends on
    /// the matching response, on EOF, or on {@link ELICITATION_TIMEOUT_MS} of wall clock — a client
    /// whose human walks away closes this lane exactly as a silent OS shell closes the bridge lane,
    /// and the approval chain moves on instead of wedging the agent forever.
    pub fn request_boolean(&self, message: &str, title: &str) -> Result<ElicitationAction, ElicitationUnavailable> {
        if !self.client_supports_elicitation() {
            return Err(ElicitationUnavailable::ClientDoesNotSupportIt);
        }
        let id = format!("semio-elicit-{}", self.next_id.fetch_add(1, Ordering::Relaxed));
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "elicitation/create",
            "params": {
                "message": message,
                "requestedSchema": {
                    "type": "object",
                    "title": title,
                    "properties": { "approve": { "type": "boolean", "title": "Approve", "description": "Approve this destructive action once." } },
                    "required": ["approve"],
                },
            },
        });
        let line = serde_json::to_string(&request).map_err(|error| ElicitationUnavailable::Malformed(error.to_string()))?;
        self.lines.write_line(&line).map_err(|error| ElicitationUnavailable::Malformed(error.message))?;
        let deadline_ms = self.clock.now_ms().saturating_add(self.timeout_ms).saturating_add(ELICITATION_CLOCK_TICK_MS);
        loop {
            let now_ms = self.clock.now_ms();
            if now_ms >= deadline_ms {
                return Err(ElicitationUnavailable::TimedOut);
            }
            let budget = std::time::Duration::from_millis((deadline_ms - now_ms).min(ELICITATION_READ_SLICE_MS));
            let inbound = match self.lines.read_line_direct_within(budget).map_err(|error| ElicitationUnavailable::Malformed(error.message))? {
                StdioRead::Line(line) => line,
                StdioRead::Eof => return Err(ElicitationUnavailable::ClientClosed),
                StdioRead::TimedOut => continue,
            };
            let trimmed = inbound.trim();
            if trimmed.is_empty() {
                continue;
            }
            match serde_json::from_str::<serde_json::Value>(trimmed) {
                Ok(value) if value.get("id").and_then(serde_json::Value::as_str) == Some(id.as_str()) => return Ok(elicitation_action(&value)),
                _ => self.lines.defer(inbound),
            }
        }
    }
}

/// 🗳️ Reads the action out of one `elicitation/create` response: an `error` member (the client
/// cancelled or refused to ask) is `Cancel`, `action: "accept"` with `content.approve == true` is
/// `Accept`, and anything else is `Decline` — never an approval by default.
fn elicitation_action(response: &serde_json::Value) -> ElicitationAction {
    if response.get("error").is_some() {
        return ElicitationAction::Cancel;
    }
    let Some(result) = response.get("result") else { return ElicitationAction::Cancel };
    match result.get("action").and_then(serde_json::Value::as_str) {
        Some("accept") if result.pointer("/content/approve").and_then(serde_json::Value::as_bool) == Some(true) => ElicitationAction::Accept,
        Some("cancel") => ElicitationAction::Cancel,
        _ => ElicitationAction::Decline,
    }
}

//#endregion 🔖️Elicitation
