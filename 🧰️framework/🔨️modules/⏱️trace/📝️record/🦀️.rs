//! 📝️ Structured request-scoped observability: one [`TraceRecord`] per server span or event,
//! carrying the five fields an operator needs to answer "who did what, to which artifact, with what
//! outcome, and how long did it take" — request id, principal, space/artifact id, outcome, duration
//! — plus a bounded per-event counter table a live introspection route reads.
//!
//! 🧭 **Why this and not `tracing`.** `AGENTS.md`: no runtime dependencies on external libraries.
//! The sibling latency primitives in `⏱️trace/🦀️.rs` already answer "did this stay interactive";
//! what a *server* additionally needs is a record with identity on it and a sink an operator can
//! point somewhere. That is this file, and it stays inside the same zero-dependency budget: pure
//! `std`, hand-written JSON, no allocation outside the record itself.
//!
//! 🎛️ **No global mutable tracer.** A [`Tracer`] is a value the embedder owns and clones into its
//! state, not a process-wide singleton. Two consequences that both matter: a test injects its own
//! [`CapturingSink`] and asserts on exactly the records its own request produced, with no
//! cross-test interference and no serialisation mutex; and a process may run more than one tracer
//! (a hub and an embedded relay) without one silently stealing the other's records.
//!
//! 🤫 **Silent by default.** [`Tracer::disabled`] holds a [`NullSink`], and that is what a build
//! that never configures one gets. The only I/O in this module is [`StreamSink`] and [`FileSink`],
//! which the embedder constructs explicitly — so the crate keeps the "no I/O" character its module
//! doc claims for every consumer that does not ask for a writing sink.
//!
//! 🌍️ The event and outcome vocabularies are language-agnostic data
//! (`🧫️fixtures/🛰️span-vocabulary/🔣️.json`); the Rust constants below are held against that file
//! by a law rather than being a second hand-kept copy.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError};

use crate::{PercentileRing, try_now_us};

//#region 🔖️Vocabulary
/// 🌐️ Environment variable naming the lowest level a [`Tracer::from_environment`] emits.
pub const TRACE_LEVEL_ENV: &str = "SEMIO_TRACE_LEVEL";

/// 🌐️ Environment variable naming where [`Tracer::from_environment`] writes: `stderr`, `stdout`,
/// `none` or `file:<path>` (JSON lines appended to that file).
pub const TRACE_SINK_ENV: &str = "SEMIO_TRACE_SINK";

/// 🚰️ The event a tracer reports its own sink configuration under.
pub const TRACE_SINK_EVENT: &str = "trace.sink";

/// 🪜️ How much a tracer says. Ordered, so `record.level <= tracer.level` is the whole admission
/// test and [`TraceLevel::Off`] admits nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TraceLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
}

impl TraceLevel {
    /// 🏷️ The wire spelling, which is also what [`TRACE_LEVEL_ENV`] accepts.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Error => "error",
            Self::Warn => "warn",
            Self::Info => "info",
            Self::Debug => "debug",
        }
    }

    /// 🔤️ Parses one spelling, case- and space-insensitively. An unrecognised value is `None` so
    /// the caller decides between a default and a refusal instead of being handed a silent one.
    pub fn parse(text: &str) -> Option<TraceLevel> {
        match text.trim().to_ascii_lowercase().as_str() {
            "off" | "none" | "silent" => Some(Self::Off),
            "error" => Some(Self::Error),
            "warn" | "warning" => Some(Self::Warn),
            "info" => Some(Self::Info),
            "debug" | "trace" => Some(Self::Debug),
            _ => None,
        }
    }
}

/// 🎯️ How one span ended. `Started` is the only one a span emits before it knows, and it is
/// emitted at [`TraceLevel::Debug`] so an operator running at `info` sees one record per request
/// rather than two.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TraceOutcome {
    Started,
    Ok,
    Refused,
    Failed,
    Cancelled,
}

impl TraceOutcome {
    /// 🏷️ The wire spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Started => "started",
            Self::Ok => "ok",
            Self::Refused => "refused",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    /// 🪜️ The level a record with this outcome is emitted at, unless the caller overrides it.
    pub fn level(self) -> TraceLevel {
        match self {
            Self::Started => TraceLevel::Debug,
            Self::Ok | Self::Cancelled => TraceLevel::Info,
            Self::Refused => TraceLevel::Warn,
            Self::Failed => TraceLevel::Error,
        }
    }
}

/// 🎯️ Every outcome, in declaration order — the Rust half of the language-agnostic vocabulary.
pub const TRACE_OUTCOMES: [TraceOutcome; 5] = [TraceOutcome::Started, TraceOutcome::Ok, TraceOutcome::Refused, TraceOutcome::Failed, TraceOutcome::Cancelled];

/// 🛰️ The declared server span/event names, sorted. A server may emit an event outside this list —
/// the tracer never refuses one — but everything an operator is expected to be able to alert on is
/// here, and the fixture beside it is what a non-Rust consumer reads.
pub const SERVER_SPAN_EVENTS: [&str; 25] = [
    "server.artifact.creation",
    "server.artifact.maintenance",
    "server.auth.agent.delegate",
    "server.auth.agent.list",
    "server.auth.agent.revoke",
    "server.auth.agent.session",
    "server.auth.credential.change",
    "server.auth.session.mint",
    "server.auth.session.read",
    "server.auth.session.revoke",
    "server.boot",
    "server.catalog.publication",
    "server.directory.backend",
    "server.directory.command",
    "server.directory.socket",
    "server.document.check-in",
    "server.document.socket",
    "server.presence.expiry",
    "server.presence.join",
    "server.presence.leave",
    "server.rate-limit",
    "server.readiness",
    "server.request",
    "server.saga.drain",
    "server.shutdown",
];
//#endregion 🔖️Vocabulary

//#region 🔖️Record
/// 📝️ One structured observation. Every identity field is optional because not every event has
/// one — boot has no principal, a refused sign-in has no artifact — and an absent field is omitted
/// from the rendered line rather than written as `null`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraceRecord {
    pub level: TraceLevel,
    pub event: String,
    pub outcome: TraceOutcome,
    pub request_id: Option<String>,
    pub principal: Option<String>,
    pub space: Option<String>,
    pub artifact: Option<String>,
    pub duration_us: Option<u64>,
    pub detail: Option<String>,
}

impl TraceRecord {
    /// 🌱️ A record with nothing but its event and outcome, at the outcome's own level.
    pub fn new(event: impl Into<String>, outcome: TraceOutcome) -> TraceRecord {
        TraceRecord { level: outcome.level(), event: event.into(), outcome, request_id: None, principal: None, space: None, artifact: None, duration_us: None, detail: None }
    }

    /// 🧾️ One JSON object on one line, field order fixed so two runs of the same request produce
    /// diffable output. Hand-written because this crate carries no serialiser and must not grow one.
    pub fn to_json_line(&self) -> String {
        let mut line = String::with_capacity(160);
        line.push('{');
        write_string_field(&mut line, "level", self.level.as_str(), true);
        write_string_field(&mut line, "event", &self.event, false);
        write_string_field(&mut line, "outcome", self.outcome.as_str(), false);
        write_optional_string_field(&mut line, "requestId", self.request_id.as_deref());
        write_optional_string_field(&mut line, "principal", self.principal.as_deref());
        write_optional_string_field(&mut line, "space", self.space.as_deref());
        write_optional_string_field(&mut line, "artifact", self.artifact.as_deref());
        if let Some(duration_us) = self.duration_us {
            let _ = write!(line, ",\"durationUs\":{duration_us}");
        }
        write_optional_string_field(&mut line, "detail", self.detail.as_deref());
        line.push('}');
        line
    }
}

fn write_string_field(line: &mut String, key: &str, value: &str, first: bool) {
    if !first {
        line.push(',');
    }
    line.push('"');
    line.push_str(key);
    line.push_str("\":");
    write_json_string(line, value);
}

fn write_optional_string_field(line: &mut String, key: &str, value: Option<&str>) {
    if let Some(value) = value {
        write_string_field(line, key, value, false);
    }
}

/// 🔐️ Minimal RFC 8259 string escaping: the two mandatory escapes, the five short forms, and
/// `\u00XX` for everything else below space. Anything above is passed through as UTF-8.
fn write_json_string(line: &mut String, value: &str) {
    line.push('"');
    for character in value.chars() {
        match character {
            '"' => line.push_str("\\\""),
            '\\' => line.push_str("\\\\"),
            '\n' => line.push_str("\\n"),
            '\r' => line.push_str("\\r"),
            '\t' => line.push_str("\\t"),
            '\u{08}' => line.push_str("\\b"),
            '\u{0c}' => line.push_str("\\f"),
            control if control < ' ' => {
                let _ = write!(line, "\\u{:04x}", control as u32);
            }
            other => line.push(other),
        }
    }
    line.push('"');
}
//#endregion 🔖️Record

//#region 🔖️Sink
/// 🚰️ Where records go. One method, taking a shared reference, because a sink is shared by every
/// request thread and must never need the caller to hold a write lock to say something happened.
pub trait TraceSink: Send + Sync {
    fn emit(&self, record: &TraceRecord);
}

/// 🕳️ Accepts and discards. The default, so a process that configures nothing pays one virtual
/// call and no formatting.
pub struct NullSink;

impl TraceSink for NullSink {
    fn emit(&self, _record: &TraceRecord) {}
}

/// 🖨️ Writes one JSON line per record to a standard stream. The only I/O in this module, and the
/// embedder constructs it explicitly.
pub struct StreamSink {
    stderr: bool,
}

impl StreamSink {
    /// 🖨️ To standard error — the default for a server, so records never contaminate a stdout the
    /// operator is piping somewhere.
    pub fn stderr() -> StreamSink {
        StreamSink { stderr: true }
    }

    /// 🖨️ To standard output, for a collector that reads it.
    pub fn stdout() -> StreamSink {
        StreamSink { stderr: false }
    }
}

impl TraceSink for StreamSink {
    fn emit(&self, record: &TraceRecord) {
        use std::io::Write as _;
        let line = record.to_json_line();
        if self.stderr {
            let mut stream = std::io::stderr().lock();
            let _ = writeln!(stream, "{line}");
        } else {
            let mut stream = std::io::stdout().lock();
            let _ = writeln!(stream, "{line}");
        }
    }
}

/// 🗃️ Appends one JSON line per record to a file the embedder named, so an operator (or a probe)
/// reads the structured log without it interleaving with the process's own stderr text.
pub struct FileSink {
    file: Mutex<std::fs::File>,
}

impl FileSink {
    /// 🗃️ Opens `path` for appending, creating it when absent, so a restarted process continues
    /// the same log.
    pub fn append(path: &std::path::Path) -> std::io::Result<FileSink> {
        Ok(FileSink { file: Mutex::new(std::fs::OpenOptions::new().create(true).append(true).open(path)?) })
    }
}

impl TraceSink for FileSink {
    fn emit(&self, record: &TraceRecord) {
        use std::io::Write as _;
        let mut line = record.to_json_line();
        line.push('\n');
        let _ = self.file.lock().unwrap_or_else(PoisonError::into_inner).write_all(line.as_bytes());
    }
}

/// 🧪️ Keeps every record it is handed, in order. The sink a test injects; never used in a
/// production configuration, because it grows without bound on purpose — a test that captures
/// records wants all of them.
#[derive(Default)]
pub struct CapturingSink {
    records: Mutex<Vec<TraceRecord>>,
}

impl CapturingSink {
    /// 🧪️ An empty capture.
    pub fn new() -> CapturingSink {
        CapturingSink { records: Mutex::new(Vec::new()) }
    }

    /// 📜️ Everything captured so far, oldest first.
    pub fn records(&self) -> Vec<TraceRecord> {
        self.records.lock().unwrap_or_else(PoisonError::into_inner).clone()
    }

    /// 🔍️ Every captured record whose event is exactly `event`.
    pub fn records_for(&self, event: &str) -> Vec<TraceRecord> {
        self.records().into_iter().filter(|record| record.event == event).collect()
    }

    /// 🧹️ Forgets everything, for a test that reuses one tracer across phases.
    pub fn clear(&self) {
        self.records.lock().unwrap_or_else(PoisonError::into_inner).clear();
    }
}

impl TraceSink for CapturingSink {
    fn emit(&self, record: &TraceRecord) {
        self.records.lock().unwrap_or_else(PoisonError::into_inner).push(record.clone());
    }
}
//#endregion 🔖️Sink

//#region 🔖️Counters
/// 📊️ What one event has done since boot: how many times it ended each way, and the latency
/// distribution of the ones that reported a duration.
#[derive(Clone, Debug, Default)]
pub struct EventCounters {
    pub started: u64,
    pub ok: u64,
    pub refused: u64,
    pub failed: u64,
    pub cancelled: u64,
    durations: PercentileRing,
}

impl EventCounters {
    /// 🔢️ Every terminal observation of this event — `started` is excluded because it is the same
    /// request counted a second time.
    pub fn total(&self) -> u64 {
        self.ok.saturating_add(self.refused).saturating_add(self.failed).saturating_add(self.cancelled)
    }

    /// 📈️ Median, p95 and p99 microseconds over the last samples this event reported.
    pub fn percentiles_us(&self) -> (u32, u32, u32) {
        (self.durations.p50(), self.durations.p95(), self.durations.p99())
    }

    /// 🔢️ How many latency samples back the percentiles.
    pub fn samples(&self) -> usize {
        self.durations.len()
    }

    fn admit(&mut self, record: &TraceRecord) {
        match record.outcome {
            TraceOutcome::Started => self.started = self.started.saturating_add(1),
            TraceOutcome::Ok => self.ok = self.ok.saturating_add(1),
            TraceOutcome::Refused => self.refused = self.refused.saturating_add(1),
            TraceOutcome::Failed => self.failed = self.failed.saturating_add(1),
            TraceOutcome::Cancelled => self.cancelled = self.cancelled.saturating_add(1),
        }
        if let Some(duration_us) = record.duration_us {
            self.durations.record(duration_us);
        }
    }
}

/// 📊️ One row of [`Tracer::counters`].
#[derive(Clone, Debug)]
pub struct EventCount {
    pub event: String,
    pub counters: EventCounters,
}

/// 🧯️ Distinct events a tracer keeps counters for. A server emits a fixed vocabulary; the bound
/// exists so an event name accidentally built from a path parameter cannot grow the table forever.
pub const COUNTER_EVENT_CAPACITY: usize = 64;
//#endregion 🔖️Counters

//#region 🔖️Tracer
struct TracerInner {
    level: TraceLevel,
    sink: Box<dyn TraceSink>,
    counters: Mutex<BTreeMap<String, EventCounters>>,
    dropped_events: AtomicU64,
    requests: AtomicU64,
}

/// 🔭️ One configured observer: a level, a sink and the counter table. Cloning is an `Arc` bump, so
/// a server clones it into every handler's state.
#[derive(Clone)]
pub struct Tracer {
    inner: Arc<TracerInner>,
}

impl Tracer {
    /// 🔭️ A tracer at `level` writing to `sink`.
    pub fn new(level: TraceLevel, sink: Box<dyn TraceSink>) -> Tracer {
        Tracer { inner: Arc::new(TracerInner { level, sink, counters: Mutex::new(BTreeMap::new()), dropped_events: AtomicU64::new(0), requests: AtomicU64::new(0) }) }
    }

    /// 🤫️ Says nothing and counts nothing — what a build that configured no observability gets.
    pub fn disabled() -> Tracer {
        Tracer::new(TraceLevel::Off, Box::new(NullSink))
    }

    /// 🧪️ A tracer at `level` over a fresh [`CapturingSink`], handed back so the test can read it.
    pub fn capturing(level: TraceLevel) -> (Tracer, Arc<CapturingSink>) {
        let sink = Arc::new(CapturingSink::new());
        (Tracer::new(level, Box::new(SharedSink(Arc::clone(&sink)))), sink)
    }

    /// 🎛️ The configuration [`TRACE_LEVEL_ENV`] and [`TRACE_SINK_ENV`] ask for, resolved through
    /// `lookup` so the caller owns where variables come from (and a test needs no process
    /// environment). An unset level means [`TraceLevel::Info`]; an unset sink means stderr; an
    /// unparseable value of either falls back to the same default rather than failing a boot over
    /// a diagnostic setting. A `file:<path>` that cannot be opened writes to stderr and says so in
    /// its first record ([`TRACE_SINK_EVENT`], refused).
    pub fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> Tracer {
        let level = lookup(TRACE_LEVEL_ENV).and_then(|value| TraceLevel::parse(&value)).unwrap_or(TraceLevel::Info);
        let requested = lookup(TRACE_SINK_ENV).map(|value| value.trim().to_string());
        let mut refusal = None;
        let sink: Box<dyn TraceSink> = match requested.as_deref() {
            Some(value) if value.eq_ignore_ascii_case("stdout") => Box::new(StreamSink::stdout()),
            Some(value) if ["none", "null", "off"].iter().any(|name| value.eq_ignore_ascii_case(name)) => Box::new(NullSink),
            Some(value) if value.starts_with("file:") => match FileSink::append(std::path::Path::new(&value["file:".len()..])) {
                Ok(sink) => Box::new(sink),
                Err(error) => {
                    refusal = Some(format!("file-sink-unavailable {:?}", error.kind()));
                    Box::new(StreamSink::stderr())
                }
            },
            _ => Box::new(StreamSink::stderr()),
        };
        let tracer = Tracer::new(level, sink);
        if let Some(detail) = refusal {
            let mut record = TraceRecord::new(TRACE_SINK_EVENT, TraceOutcome::Refused);
            record.detail = Some(detail);
            tracer.emit(record);
        }
        tracer
    }

    /// 🎛️ [`Tracer::from_lookup`] over this process's own environment.
    #[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
    pub fn from_environment() -> Tracer {
        Tracer::from_lookup(|key| std::env::var(key).ok())
    }

    /// 🪜️ The level this tracer admits up to.
    pub fn level(&self) -> TraceLevel {
        self.inner.level
    }

    /// 🪜️ Whether a record at `level` would be emitted — the cheap test a caller makes before
    /// building a detail string it would otherwise throw away.
    pub fn enabled(&self, level: TraceLevel) -> bool {
        level != TraceLevel::Off && level <= self.inner.level
    }

    /// 🆔️ A fresh request id for this tracer: a monotonic counter rendered in lower-case hex. Not
    /// globally unique and not meant to be — it identifies one request inside one process's log,
    /// which is the scope every field on a [`TraceRecord`] is scoped to.
    pub fn allocate_request_id(&self) -> String {
        format!("r{:012x}", self.inner.requests.fetch_add(1, Ordering::Relaxed))
    }

    /// 📝️ Counts `record` and hands it to the sink, if the level admits it. Counting happens
    /// first and unconditionally for an admitted record, so the introspection table is the truth
    /// about what happened even when the sink is [`NullSink`].
    pub fn emit(&self, record: TraceRecord) {
        if !self.enabled(record.level) {
            return;
        }
        {
            let mut counters = self.inner.counters.lock().unwrap_or_else(PoisonError::into_inner);
            let has_capacity = counters.len() < COUNTER_EVENT_CAPACITY;
            match counters.get_mut(&record.event) {
                Some(existing) => existing.admit(&record),
                None if has_capacity => counters.entry(record.event.clone()).or_default().admit(&record),
                None => {
                    self.inner.dropped_events.fetch_add(1, Ordering::Relaxed);
                }
            }
        }
        self.inner.sink.emit(&record);
    }

    /// ⏱️ Opens a span on `event`. The span carries the identity fields and measures its own
    /// duration; nothing is emitted until it finishes.
    pub fn span(&self, event: &str) -> Span {
        Span { tracer: self.clone(), event: event.to_string(), started_us: try_now_us(), request_id: None, principal: None, space: None, artifact: None }
    }

    /// 📊️ Every event this tracer has seen, sorted by event name.
    pub fn counters(&self) -> Vec<EventCount> {
        self.inner.counters.lock().unwrap_or_else(PoisonError::into_inner).iter().map(|(event, counters)| EventCount { event: event.clone(), counters: counters.clone() }).collect()
    }

    /// 🧯️ How many records were counted against no row because the table was already at
    /// [`COUNTER_EVENT_CAPACITY`]. A non-zero value means the event vocabulary is unbounded
    /// somewhere and is itself worth alerting on.
    pub fn dropped_event_count(&self) -> u64 {
        self.inner.dropped_events.load(Ordering::Relaxed)
    }
}

struct SharedSink(Arc<CapturingSink>);

impl TraceSink for SharedSink {
    fn emit(&self, record: &TraceRecord) {
        self.0.emit(record);
    }
}
//#endregion 🔖️Tracer

//#region 🔖️Span
/// ⏱️ One in-flight observation. Built by [`Tracer::span`], decorated with whatever identity the
/// handler has resolved so far, and closed exactly once with an outcome.
///
/// 🕰️ Duration is measured with [`try_now_us`] rather than [`now_us`](crate::now_us): a span must
/// never panic a request path because no clock was installed. A span opened without a clock simply
/// reports no duration.
pub struct Span {
    tracer: Tracer,
    event: String,
    started_us: Option<u64>,
    request_id: Option<String>,
    principal: Option<String>,
    space: Option<String>,
    artifact: Option<String>,
}

impl Span {
    /// 🆔️ Binds the request id this span belongs to.
    pub fn request(mut self, request_id: impl Into<String>) -> Span {
        self.request_id = Some(request_id.into());
        self
    }

    /// 👤️ Binds who is asking, as soon as the handler knows.
    pub fn principal(mut self, principal: impl Into<String>) -> Span {
        self.principal = Some(principal.into());
        self
    }

    /// 🪐️ Binds the space this span acts in.
    pub fn space(mut self, space: impl Into<String>) -> Span {
        self.space = Some(space.into());
        self
    }

    /// 🗿️ Binds the artifact/document this span acts on.
    pub fn artifact(mut self, artifact: impl Into<String>) -> Span {
        self.artifact = Some(artifact.into());
        self
    }

    /// 👤️ Binds the principal only when one was resolved, for a path where anonymity is normal.
    pub fn maybe_principal(mut self, principal: Option<String>) -> Span {
        self.principal = principal;
        self
    }

    /// 🧬️ A new span on the same event with the same request id and identity, timed from now — the
    /// lifetime span of something this span just admitted (a socket), so the admission record and
    /// the record that ends the session correlate by `requestId`.
    pub fn fork(&self) -> Span {
        Span { tracer: self.tracer.clone(), event: self.event.clone(), started_us: try_now_us(), request_id: self.request_id.clone(), principal: self.principal.clone(), space: self.space.clone(), artifact: self.artifact.clone() }
    }

    /// 🏷️ The event this span will report under.
    pub fn event(&self) -> &str {
        &self.event
    }

    /// 🚩️ Emits the opening record, at [`TraceLevel::Debug`]. Optional: a span that is only closed
    /// produces one line per request, which is what an operator at `info` wants.
    pub fn started(&self) {
        self.tracer.emit(self.record(TraceOutcome::Started, None, None));
    }

    /// ✅️ Closes the span as successful.
    pub fn ok(self) {
        self.finish(TraceOutcome::Ok, None);
    }

    /// 🚫️ Closes the span as refused — an authorization denial, a rate limit, a malformed body.
    /// `reason` is a stable short code, never a formatted user message.
    pub fn refused(self, reason: &str) {
        self.finish(TraceOutcome::Refused, Some(reason.to_string()));
    }

    /// 💥️ Closes the span as failed — the server could not carry the request out.
    pub fn failed(self, reason: &str) {
        self.finish(TraceOutcome::Failed, Some(reason.to_string()));
    }

    /// 🛑️ Closes the span as cancelled — the peer went away, or a socket closed normally.
    pub fn cancelled(self, reason: &str) {
        self.finish(TraceOutcome::Cancelled, Some(reason.to_string()));
    }

    /// 🎯️ Closes the span with an explicit outcome and optional detail.
    pub fn finish(self, outcome: TraceOutcome, detail: Option<String>) {
        let elapsed_us = match (self.started_us, try_now_us()) {
            (Some(started), Some(now)) => Some(now.saturating_sub(started)),
            _ => None,
        };
        self.tracer.emit(self.record(outcome, elapsed_us, detail));
    }

    fn record(&self, outcome: TraceOutcome, duration_us: Option<u64>, detail: Option<String>) -> TraceRecord {
        TraceRecord {
            level: outcome.level(),
            event: self.event.clone(),
            outcome,
            request_id: self.request_id.clone(),
            principal: self.principal.clone(),
            space: self.space.clone(),
            artifact: self.artifact.clone(),
            duration_us,
            detail,
        }
    }
}
//#endregion 🔖️Span

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
