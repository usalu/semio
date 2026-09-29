//! 🗄️ `db_observe` — the `db` family's observability seam: JSON-lines structured/audit event
//! sinks implementing `Emit`, cardinality-controlled metric registries (counter/gauge/
//! histogram), a bounded span registry, a component health registry, and a runtime determinism
//! verifier (cross-checks independently-produced state-hash streams for the same document, e.g.
//! a live execution against a replay). Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`).
//!
//! 🎯️ Design choice: `db_*` modules below `db_artifact` stay decoupled from this module's concrete
//! sinks via the `Emit` trait (`🔖️Emit` below, see its own doc) rather than naming the sinks
//! directly — `db_security` stays generic over `E: Emit`, `db_engine::Database` is generic over
//! its own `E: Emit` default-`StructuredSink<MemorySink>` (dedyn-emit-runtime, O1/R11(a): replaces
//! the former `Arc<dyn Emit>` erasure; `db_artifact`/`db_actor` use the concrete `NullEmit` instead,
//! per R11(c), since no real call site anywhere in this repo ever passes them anything else). This
//! crate supplies the real sinks (`StructuredSink`/`AuditSink`) that implement that trait, plus the standalone
//! registries (`MetricRegistry`, `SpanRegistry`, `HealthRegistry`, `DeterminismVerifier`) a
//! deployment wires up independently of the `Emit` event stream. Depends on `pack_core` (in
//! addition to `db_core`, which was already a transitive dependency) for two things only:
//! `crc32c` (the `AuditSink` tamper-evidence chain, see its doc for why CRC-32C rather than a
//! blake3 `ContentHash`) and `ContentHash` itself as `DeterminismVerifier`'s digest type, since
//! that's exactly the type `CommandReceipt.state_hash` carries per the frozen `db` facade API.

use crate::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::future::Future;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

//#region 🔖️Util
/// 🔓️ Locks `mutex`, recovering the inner value even if a prior holder panicked while
/// holding it — an observability sink must never itself become a source of panics-under-panic
/// for the mailbox/actor code that's often mid-crash-handling when it calls into `Emit::emit`.
// 🚫️async: E1 pure accessor (no suspension: `Mutex::lock` on a never-genuinely-contended
// in-process lock) — see R9
fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}
//#endregion 🔖️Util

//#region 🔖️Emit
/// 🏷️ One field attached to an `EmitEvent`, kept as a small closed set of primitive
/// shapes (no dynamic `Any`) so a sink can serialize/aggregate without reflection.
#[derive(Clone, Debug)]
pub enum EmitField {
    U64(u64),
    I64(i64),
    F64(f64),
    Bool(bool),
    Text(String),
}

/// 📣️ One observability event: a stable name plus an optional document scope and a small
/// bag of typed fields. `Emit::emit` takes this by value (not by reference) since a mailbox-
/// adjacent hot path may hand it across a thread boundary to a sink.
#[derive(Clone, Debug)]
pub struct EmitEvent {
    pub name: &'static str,
    pub document: Option<ArtifactId>,
    pub fields: Vec<(&'static str, EmitField)>,
}

impl EmitEvent {
    /// 🆕️ A bare event with `name` and no document/fields yet.
    // 🚫️async: E1 pure builder — `Emit::emit`'s real sinks may genuinely await (I/O), but building
    // the event value itself has no suspension point, and several call sites build one from a sync
    // context (e.g. `db_actor::run_actor_loop`, a raw OS thread with no executor) — see R9
    pub fn new(name: &'static str) -> Self {
        Self { name, document: None, fields: Vec::new() }
    }

    /// 🪪️ Scopes the event to `document` (builder-style).
    // 🚫️async: E1 pure builder — see `EmitEvent::new`
    pub fn with_document(mut self, document: ArtifactId) -> Self {
        self.document = Some(document);
        self
    }

    /// ➕️ Appends one field (builder-style).
    // 🚫️async: E1 pure builder — see `EmitEvent::new`
    pub fn field(mut self, key: &'static str, value: EmitField) -> Self {
        self.fields.push((key, value));
        self
    }
}

/// 📡️ The observability seam: every `db_*` module that wants to emit a metric/span/log event stays
/// generic over `E: Emit` (`db_security`, `db_engine::Database`) or uses the concrete `NullEmit`
/// (`db_artifact`, `db_actor` — R11(c): no real call site anywhere threads anything else through
/// them), so those modules name only this trait and never the concrete sinks below
/// (structured/audit JSON-lines, metric registries) that implement it — dedyn-emit-runtime,
/// O1/R11(a): replaces the former `&dyn Emit`/`Arc<dyn Emit>` erasure with trivial generics.
pub trait Emit: Send + Sync {
    fn emit(&self, event: EmitEvent) -> impl Future<Output = ()> + Send;
}

/// 🔇️ An `Emit` that discards every event — the default when no observability sink is
/// configured, and a convenient no-op for tests that don't care about emitted events.
#[derive(Clone, Copy, Default, Debug)]
pub struct NullEmit;

impl Emit for NullEmit {
    async fn emit(&self, _event: EmitEvent) {}
}
//#endregion 🔖️Emit

//#region 🔖️Json
/// 📝️ Escapes `raw` per RFC 8259 into `out` (no surrounding quotes).
fn escape_json_str(raw: &str, out: &mut String) {
    for ch in raw.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                use std::fmt::Write;
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
}

fn write_json_field_value(field: &EmitField, out: &mut String) {
    use std::fmt::Write;
    match field {
        EmitField::U64(v) => {
            let _ = write!(out, "{v}");
        }
        EmitField::I64(v) => {
            let _ = write!(out, "{v}");
        }
        EmitField::F64(v) => {
            if v.is_finite() {
                let _ = write!(out, "{v}");
            } else {
                // 🔒️ JSON has no NaN/Infinity literal — never emit a syntactically invalid line.
                out.push_str("null");
            }
        }
        EmitField::Bool(v) => out.push_str(if *v { "true" } else { "false" }),
        EmitField::Text(v) => {
            out.push('"');
            escape_json_str(v, out);
            out.push('"');
        }
    }
}

/// 🧾️ Encodes `event` as one JSON object line (no trailing newline) — the wire shape both
/// `StructuredSink` and `AuditSink` write, and what `db_cli`'s log tooling can `jq` over without
/// a schema. Hand-rolled rather than pulling `serde_json`: keeps this crate's dependency surface
/// at `db_core` + `pack_core`, matching the family's dependency-light convention.
pub fn encode_emit_event_json(event: &EmitEvent) -> String {
    let mut out = String::with_capacity(64 + event.fields.len() * 16);
    out.push('{');
    out.push_str("\"name\":\"");
    escape_json_str(event.name, &mut out);
    out.push('"');
    if let Some(document) = &event.document {
        out.push_str(",\"document\":\"");
        escape_json_str(&document.0, &mut out);
        out.push('"');
    }
    out.push_str(",\"fields\":{");
    for (i, (key, value)) in event.fields.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push('"');
        escape_json_str(key, &mut out);
        out.push_str("\":");
        write_json_field_value(value, &mut out);
    }
    out.push_str("}}");
    out
}
//#endregion 🔖️Json

//#region 🔖️Sink
/// 🚰️ Where a sink's JSON-lines actually land — implementable over memory (`MemorySink`,
/// tests/introspection), a file/pipe/`Vec<u8>` (`WriterSink`), or anything else ordered and
/// append-only. Mirrors `pack::PackSink`'s spirit but returns `DbError` (the
/// family's error type) instead of `PackError`, and writes pre-delimited lines rather than raw
/// byte ranges.
pub trait EventSink: Send + Sync {
    fn write_line(&self, line: &str) -> impl Future<Output = Result<(), DbError>> + Send;
}

/// 🧠️ An in-memory `EventSink` — the default for tests and for introspecting what a sink
/// would have written without touching the filesystem.
#[derive(Default)]
pub struct MemorySink {
    lines: Mutex<Vec<String>>,
}

impl MemorySink {
    pub fn new() -> Self {
        Self::default()
    }

    /// 📜️ A snapshot of every line written so far, oldest first.
    pub async fn lines(&self) -> Vec<String> {
        lock(&self.lines).clone()
    }
}

impl EventSink for MemorySink {
    async fn write_line(&self, line: &str) -> Result<(), DbError> {
        lock(&self.lines).push(line.to_string());
        Ok(())
    }
}

/// 📄️ An `EventSink` over any `std::io::Write` (a file, a pipe, `Vec<u8>`, …) — one JSON
/// object per line, newline-terminated, flushed on every write (observability sinks favor
/// visibility over batching throughput; a deployment that wants batched flushing can wrap its own
/// `std::io::BufWriter` and flush on a timer around this). Wraps every `std::io::Error` into
/// `DbError::Io` at the boundary (repo rule: no `std::io::Error` in a public signature).
pub struct WriterSink<W: std::io::Write + Send> {
    writer: Mutex<W>,
}

impl<W: std::io::Write + Send> WriterSink<W> {
    pub fn new(writer: W) -> Self {
        Self { writer: Mutex::new(writer) }
    }
}

impl<W: std::io::Write + Send> EventSink for WriterSink<W> {
    async fn write_line(&self, line: &str) -> Result<(), DbError> {
        let mut writer = lock(&self.writer);
        writer.write_all(line.as_bytes()).map_err(|e| DbError::Io(e.to_string()))?;
        writer.write_all(b"\n").map_err(|e| DbError::Io(e.to_string()))?;
        writer.flush().map_err(|e| DbError::Io(e.to_string()))
    }
}
//#endregion 🔖️Sink

//#region 🔖️Structured
/// 📡️ A `Emit` implementation that JSON-lines-encodes every event into an
/// `EventSink`. The family's default observability sink: wiring one of these into a `Database`
/// deployment is the only thing needed to get structured logs — no `db_observe` dependency leaks
/// into `db_core..db_cluster`.
pub struct StructuredSink<S: EventSink> {
    sink: S,
    failed_writes: AtomicU64,
}

impl<S: EventSink> StructuredSink<S> {
    pub fn new(sink: S) -> Self {
        Self { sink, failed_writes: AtomicU64::new(0) }
    }

    /// 🚨️ How many `emit` calls lost their event to a sink write failure. `Emit::emit`
    /// cannot return `Result` (it's invoked from hot mailbox paths), so a failed write is counted
    /// here rather than silently dropped-and-forgotten or panicking.
    pub async fn failed_writes(&self) -> u64 {
        self.failed_writes.load(Ordering::Relaxed)
    }
}

impl<S: EventSink> Emit for StructuredSink<S> {
    async fn emit(&self, event: EmitEvent) {
        let line = encode_emit_event_json(&event);
        if self.sink.write_line(&line).await.is_err() {
            self.failed_writes.fetch_add(1, Ordering::Relaxed);
        }
    }
}
//#endregion 🔖️Structured

//#region 🔖️Audit
/// 🔗️ One link in `AuditSink`'s tamper-evident chain: the sequence number, and the
/// checksum folding this record's line into every checksum before it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AuditLink {
    pub seq: u64,
    pub checksum: u32,
}

fn fold_checksum(prev_checksum: u32, line: &str) -> u32 {
    let mut buf = Vec::with_capacity(4 + line.len());
    buf.extend_from_slice(&prev_checksum.to_le_bytes());
    buf.extend_from_slice(line.as_bytes());
    pack::crc32c(&buf)
}

struct AuditChainState {
    base_checksum: u32,
    links: VecDeque<AuditLink>,
}

/// 🕵️ A `Emit` implementation for the audit trail: same JSON-lines wire shape as
/// `StructuredSink`, but every record is folded into a CRC-32C hash chain
/// (`pack::crc32c(prev_checksum || line)`) so `verify_chain` can detect a single tampered,
/// reordered, or dropped line anywhere in the retained window. Deliberately CRC-32C, not a
/// `pack::ContentHash` (blake3): that type is for content-addressing across the family (WAL
/// payloads, snapshot pages, `DeterminismVerifier`'s digests below); this chain is a local
/// integrity check over an append-only log this sink owns end to end, and CRC-32C is what the
/// family already uses for that flavor of check (SPR frame checksums) — blake3 here would buy no
/// additional guarantee this sink needs.
pub struct AuditSink<S: EventSink> {
    sink: S,
    next_seq: AtomicU64,
    state: Mutex<AuditChainState>,
    max_retained: usize,
    failed_writes: AtomicU64,
}

impl<S: EventSink> AuditSink<S> {
    /// 🆕️ `max_retained` bounds the in-memory chain window `verify_chain` can check
    /// against (oldest links are dropped once exceeded, folded into a running `base_checksum` so
    /// the chain math for the surviving window stays exact) — the durable JSON-lines themselves
    /// are unbounded (owned by `S`), only the tamper-evidence window is capped.
    pub async fn new(sink: S, max_retained: usize) -> Self {
        Self { sink, next_seq: AtomicU64::new(0), state: Mutex::new(AuditChainState { base_checksum: 0, links: VecDeque::new() }), max_retained: max_retained.max(1), failed_writes: AtomicU64::new(0) }
    }

    /// 🚨️ See `StructuredSink::failed_writes` — same rationale (`Emit::emit` has no
    /// `Result`); a failed durable write is never folded into the chain (the chain only ever
    /// covers records that actually made it to `S`).
    pub async fn failed_writes(&self) -> u64 {
        self.failed_writes.load(Ordering::Relaxed)
    }

    /// 📜️ A snapshot of the retained chain window, oldest first.
    pub async fn chain(&self) -> Vec<AuditLink> {
        lock(&self.state).links.iter().copied().collect()
    }

    /// ✅️ Recomputes the checksum chain over `lines` (as read back from durable storage,
    /// oldest-first, aligned to exactly the current retained window — the caller is responsible
    /// for skipping any lines older than the window, e.g. via a companion compaction checkpoint;
    /// out of scope for this sink) and compares it link by link. `Ok(())` iff every retained link
    /// still matches; otherwise `DbError::Corrupt` naming the first divergent seq.
    pub async fn verify_chain(&self, lines: &[String]) -> Result<(), DbError> {
        let state = lock(&self.state);
        if lines.len() != state.links.len() {
            return Err(DbError::Corrupt(format!("audit chain length mismatch: expected {} retained lines, got {}", state.links.len(), lines.len())));
        }
        let mut prev_checksum = state.base_checksum;
        for (line, expected) in lines.iter().zip(state.links.iter()) {
            let checksum = fold_checksum(prev_checksum, line);
            if checksum != expected.checksum {
                return Err(DbError::Corrupt(format!("audit chain diverges at seq {}", expected.seq)));
            }
            prev_checksum = checksum;
        }
        Ok(())
    }
}

impl<S: EventSink> Emit for AuditSink<S> {
    async fn emit(&self, event: EmitEvent) {
        let line = encode_emit_event_json(&event);
        if self.sink.write_line(&line).await.is_err() {
            self.failed_writes.fetch_add(1, Ordering::Relaxed);
            return;
        }
        let seq = self.next_seq.fetch_add(1, Ordering::Relaxed);
        let mut state = lock(&self.state);
        let prev_checksum = state.links.back().map_or(state.base_checksum, |link| link.checksum);
        let checksum = fold_checksum(prev_checksum, &line);
        state.links.push_back(AuditLink { seq, checksum });
        while state.links.len() > self.max_retained {
            if let Some(evicted) = state.links.pop_front() {
                state.base_checksum = evicted.checksum;
            }
        }
    }
}
//#endregion 🔖️Audit

//#region 🔖️Cardinality
/// 🏷️ A canonicalized (key-sorted) label set — the identity `MetricRegistry` and
/// `CardinalityLimiter` key series by. Sorting on construction means two callers who build the
/// same labels in different insertion order collide into the same series instead of silently
/// doubling it.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Labels(Vec<(&'static str, String)>);

impl Labels {
    // 🚫️async: E1 pure constructor consumed synchronously by dozens of test/production call sites — see R9
    pub fn new(pairs: impl IntoIterator<Item = (&'static str, String)>) -> Labels {
        let mut pairs: Vec<_> = pairs.into_iter().collect();
        pairs.sort_by_key(|(k, _)| *k);
        Labels(pairs)
    }

    pub async fn none() -> Labels {
        Labels(Vec::new())
    }

    pub async fn as_slice(&self) -> &[(&'static str, String)] {
        &self.0
    }
}

/// 🚧️ Bounds how many distinct label sets ("series") a metric name may accumulate — an
/// unbounded label (a raw document id, a request path with path params) turned directly into a
/// label value is the classic metrics cardinality-explosion bug; this collapses everything past
/// the limit into one shared overflow series rather than growing forever.
pub struct CardinalityLimiter {
    max_series_per_metric: usize,
    seen: Mutex<HashMap<&'static str, HashSet<Labels>>>,
}

impl CardinalityLimiter {
    pub async fn new(max_series_per_metric: usize) -> CardinalityLimiter {
        CardinalityLimiter { max_series_per_metric: max_series_per_metric.max(1), seen: Mutex::new(HashMap::new()) }
    }

    /// 🚪️ Admits `labels` for `metric`: returns `labels` unchanged if it's already a known
    /// series or `metric` is still under its limit, otherwise returns the shared overflow series
    /// (`[("cardinality", "overflow")]`) instead — a metric's tracked series count never exceeds
    /// `max_series_per_metric` (the overflow series itself is exactly one of those tracked slots,
    /// reused by every caller that overflows).
    pub async fn admit(&self, metric: &'static str, labels: Labels) -> Labels {
        let mut seen = lock(&self.seen);
        let series = seen.entry(metric).or_default();
        if series.contains(&labels) || series.len() < self.max_series_per_metric {
            series.insert(labels.clone());
            labels
        } else {
            let overflow = Labels::new([("cardinality", "overflow".to_string())]);
            // 🔒️ Only claim a tracked slot for the overflow marker if one is actually free —
            // when real distinct series have already filled `max_series_per_metric`, inserting
            // here unconditionally would push `series_count` past the very limit this type
            // exists to enforce. Once it does occupy a slot, `contains` short-circuits above so
            // it is never re-inserted.
            if series.len() < self.max_series_per_metric {
                series.insert(overflow.clone());
            }
            overflow
        }
    }

    pub async fn series_count(&self, metric: &'static str) -> usize {
        lock(&self.seen).get(metric).map_or(0, HashSet::len)
    }
}
//#endregion 🔖️Cardinality

//#region 🔖️Metrics
struct HistogramState {
    bounds: Vec<f64>,
    bucket_counts: Vec<u64>,
    sum: f64,
    count: u64,
}

/// 📊️ A point-in-time read of one histogram series: `bucket_counts[i]` counts
/// observations `<= bounds[i]`, and `bucket_counts[bounds.len()]` is the `+Inf` overflow bucket.
#[derive(Clone, Debug)]
pub struct HistogramSnapshot {
    pub bounds: Vec<f64>,
    pub bucket_counts: Vec<u64>,
    pub sum: f64,
    pub count: u64,
}

/// 📈️ Counter/gauge/histogram series, each keyed by `(name, Labels)` and admitted through
/// a shared `CardinalityLimiter` so no single metric name can grow the registry unboundedly.
pub struct MetricRegistry {
    cardinality: CardinalityLimiter,
    counters: Mutex<HashMap<(&'static str, Labels), u64>>,
    gauges: Mutex<HashMap<(&'static str, Labels), f64>>,
    histograms: Mutex<HashMap<(&'static str, Labels), HistogramState>>,
}

impl MetricRegistry {
    pub async fn new(max_series_per_metric: usize) -> MetricRegistry {
        MetricRegistry { cardinality: CardinalityLimiter::new(max_series_per_metric).await, counters: Mutex::new(HashMap::new()), gauges: Mutex::new(HashMap::new()), histograms: Mutex::new(HashMap::new()) }
    }

    /// ➕️ Monotonically increments the counter series `(name, labels)` by `delta`.
    pub async fn incr_counter(&self, name: &'static str, labels: Labels, delta: u64) {
        let labels = self.cardinality.admit(name, labels);
        let mut counters = lock(&self.counters);
        *counters.entry((name, labels.await)).or_insert(0) += delta;
    }

    pub async fn counter_value(&self, name: &'static str, labels: &Labels) -> u64 {
        lock(&self.counters).get(&(name, labels.clone())).copied().unwrap_or(0)
    }

    /// 🎚️ Sets the gauge series `(name, labels)` to `value`, overwriting any prior value.
    pub async fn set_gauge(&self, name: &'static str, labels: Labels, value: f64) {
        let labels = self.cardinality.admit(name, labels);
        lock(&self.gauges).insert((name, labels.await), value);
    }

    pub async fn gauge_value(&self, name: &'static str, labels: &Labels) -> Option<f64> {
        lock(&self.gauges).get(&(name, labels.clone())).copied()
    }

    /// 📊️ Records `value` into the histogram series `(name, labels)`, creating it with
    /// `bounds` (ascending upper-inclusive bucket boundaries, must be non-empty) on first
    /// observation. Later calls for the same series reuse the bounds fixed at creation — a
    /// mismatched `bounds.len()` is a caller bug (`DbError::InvalidArgument`), not silently
    /// ignored.
    pub async fn observe_histogram(&self, name: &'static str, labels: Labels, bounds: &[f64], value: f64) -> Result<(), DbError> {
        if bounds.is_empty() {
            return Err(DbError::InvalidArgument("histogram bounds must not be empty".to_string()));
        }
        let labels = self.cardinality.admit(name, labels);
        let mut histograms = lock(&self.histograms);
        let state = histograms.entry((name, labels.await)).or_insert_with(|| HistogramState { bounds: bounds.to_vec(), bucket_counts: vec![0; bounds.len() + 1], sum: 0.0, count: 0 });
        if state.bounds.len() != bounds.len() {
            return Err(DbError::InvalidArgument(format!("histogram {name} bounds length changed: {} vs {}", state.bounds.len(), bounds.len())));
        }
        let bucket = state.bounds.iter().position(|&b| value <= b).unwrap_or(state.bounds.len());
        state.bucket_counts[bucket] += 1;
        state.sum += value;
        state.count += 1;
        Ok(())
    }

    pub async fn histogram_snapshot(&self, name: &'static str, labels: &Labels) -> Option<HistogramSnapshot> {
        lock(&self.histograms).get(&(name, labels.clone())).map(|s| HistogramSnapshot { bounds: s.bounds.clone(), bucket_counts: s.bucket_counts.clone(), sum: s.sum, count: s.count })
    }
}
//#endregion 🔖️Metrics

//#region 🔖️Span
/// ⏱️ Wall-clock seam so `SpanRegistry` durations are testable without real sleeps — the
/// family's `db_fault_testing::SimClock` (not a dependency of this crate) is the deterministic-
/// simulation analog; this crate only needs the read side.
pub trait Clock: Send + Sync {
    async fn now_ms(&self) -> u64;
}

/// 🕰️ The real wall clock — `Database::open_at`'s default `SpanRegistry` clock.
#[derive(Clone, Copy, Default, Debug)]
pub struct SystemClock;

impl Clock for SystemClock {
    async fn now_ms(&self) -> u64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as u64)
    }
}

/// 🔖️ A span's identity within its owning `SpanRegistry` (not globally unique — scope it
/// with the registry that issued it).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SpanId(u64);

/// 🌳️ A finished span: name, optional parent (for nesting) and document scope, and timing.
#[derive(Clone, Debug)]
pub struct CompletedSpan {
    pub id: SpanId,
    pub name: &'static str,
    pub parent: Option<SpanId>,
    pub document: Option<ArtifactId>,
    pub start_ms: u64,
    pub end_ms: u64,
    pub duration_ms: u64,
}

struct ActiveSpan {
    name: &'static str,
    parent: Option<SpanId>,
    document: Option<ArtifactId>,
    start_ms: u64,
}

/// 🌲️ Tracks in-flight spans and retains the most recently completed ones in a bounded
/// ring buffer — unbounded retention would make a long-lived process's span log an unbounded-
/// memory leak; bounding it is this crate's own choice (the contract doesn't specify a number).
pub struct SpanRegistry<C: Clock = SystemClock> {
    clock: C,
    next_id: AtomicU64,
    active: Mutex<HashMap<u64, ActiveSpan>>,
    completed: Mutex<VecDeque<CompletedSpan>>,
    max_retained: usize,
}

impl SpanRegistry<SystemClock> {
    pub async fn new(max_retained: usize) -> SpanRegistry<SystemClock> {
        SpanRegistry::with_clock(SystemClock, max_retained).await
    }
}

impl<C: Clock> SpanRegistry<C> {
    pub async fn with_clock(clock: C, max_retained: usize) -> SpanRegistry<C> {
        SpanRegistry { clock, next_id: AtomicU64::new(0), active: Mutex::new(HashMap::new()), completed: Mutex::new(VecDeque::new()), max_retained: max_retained.max(1) }
    }

    /// ▶️ Starts a new span, returning its id (pass to `end`).
    pub async fn start(&self, name: &'static str, parent: Option<SpanId>, document: Option<ArtifactId>) -> SpanId {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let start_ms = self.clock.now_ms().await;
        lock(&self.active).insert(id, ActiveSpan { name, parent, document, start_ms });
        SpanId(id)
    }

    /// 🏁️ Ends `id`, moving it from active into the completed ring buffer and returning
    /// it. `None` if `id` was never started or was already ended (a caller bug this reports
    /// rather than panics on).
    pub async fn end(&self, id: SpanId) -> Option<CompletedSpan> {
        let active = lock(&self.active).remove(&id.0)?;
        let end_ms = self.clock.now_ms().await;
        let span = CompletedSpan { id, name: active.name, parent: active.parent, document: active.document, start_ms: active.start_ms, end_ms, duration_ms: end_ms.saturating_sub(active.start_ms) };
        let mut completed = lock(&self.completed);
        completed.push_back(span.clone());
        while completed.len() > self.max_retained {
            completed.pop_front();
        }
        Some(span)
    }

    pub async fn active_count(&self) -> usize {
        lock(&self.active).len()
    }

    /// 📜️ A snapshot of the retained completed spans, oldest first.
    pub async fn completed(&self) -> Vec<CompletedSpan> {
        lock(&self.completed).iter().cloned().collect()
    }
}
//#endregion 🔖️Span

//#region 🔖️Health
/// 🩺️ One component's health, worst-of-aggregated by `HealthRegistry::report` into the
/// overall `Database::health()` status.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum HealthState {
    Healthy,
    Degraded(String),
    Unhealthy(String),
}

impl HealthState {
    // 🚫️async: E4 fn-pointer slot (used as `Iterator::max_by_key(HealthState::rank)`) — see R9
    fn rank(&self) -> u8 {
        match self {
            HealthState::Healthy => 0,
            HealthState::Degraded(_) => 1,
            HealthState::Unhealthy(_) => 2,
        }
    }
}

/// 📋️ A point-in-time health read: every component's individual state plus the worst-of
/// overall.
#[derive(Clone, Debug)]
pub struct HealthReport {
    pub overall: HealthState,
    pub components: Vec<(&'static str, HealthState)>,
}

/// 🩺️ Aggregates named component health into one worst-of-the-set overall status —
/// `Database::health()`'s data source.
#[derive(Default)]
pub struct HealthRegistry {
    components: Mutex<HashMap<&'static str, HealthState>>,
}

impl HealthRegistry {
    pub fn new() -> HealthRegistry {
        HealthRegistry::default()
    }

    /// ✏️ Sets (or overwrites) `component`'s current state.
    pub fn set(&self, component: &'static str, state: HealthState) {
        lock(&self.components).insert(component, state);
    }

    /// 📊️ Snapshots every component (sorted by name for determinism) plus the worst-of
    /// overall (`Healthy` if no component has ever reported).
    pub fn report(&self) -> HealthReport {
        let components = lock(&self.components);
        let mut entries: Vec<_> = components.iter().map(|(k, v)| (*k, v.clone())).collect();
        entries.sort_by_key(|(k, _)| *k);
        let overall = entries.iter().map(|(_, s)| s.clone()).max_by_key(HealthState::rank).unwrap_or(HealthState::Healthy);
        HealthReport { overall, components: entries }
    }
}
//#endregion 🔖️Health

//#region 🔖️Determinism
/// ⚠️ What `DeterminismVerifier::record` returns when the labeled digest streams for one
/// `seq` disagree — every expected label's digest, sorted by label for a stable diff.
#[derive(Clone, Debug)]
pub struct DivergenceReport {
    pub seq: u64,
    pub digests: Vec<(String, ContentHash)>,
}

/// 🧬️ Runtime cross-check that two (or more) independently-produced state-hash streams
/// for the same document agree at every sequence number — e.g. a live execution's per-command
/// `state_hash` (see the frozen `CommandReceipt`) against a replay's recomputation. Complements
/// (does not replace) `db_fault_testing::assert_replay_deterministic`, a test-only harness in a crate
/// this one may not depend on; this is the always-on runtime version.
pub struct DeterminismVerifier {
    expected_labels: Vec<String>,
    pending: Mutex<HashMap<u64, HashMap<String, ContentHash>>>,
    max_pending: usize,
}

impl DeterminismVerifier {
    /// 🆕️ `expected_labels` names every stream that must agree (e.g. `["primary",
    /// "replay"]`); `max_pending` bounds how many not-yet-fully-reported sequence numbers this
    /// verifier holds onto at once — a stream that stalls forever can't grow this unboundedly.
    pub async fn new(expected_labels: impl IntoIterator<Item = impl Into<String>>, max_pending: usize) -> DeterminismVerifier {
        DeterminismVerifier { expected_labels: expected_labels.into_iter().map(Into::into).collect(), pending: Mutex::new(HashMap::new()), max_pending: max_pending.max(1) }
    }

    /// 📮️ Records `label`'s digest for `seq`. Once every expected label has reported for
    /// `seq`: returns `Ok(Some(report))` if any two digests disagree, `Ok(None)` if they all
    /// agree — either way `seq` is pruned afterward, so a completed sequence never grows the
    /// pending set. Errs with `LimitExceeded` if `seq` is new and the pending window is already
    /// full (protects against an expected label that never reports).
    pub async fn record(&self, seq: u64, label: &str, digest: ContentHash) -> Result<Option<DivergenceReport>, DbError> {
        let mut pending = lock(&self.pending);
        if !pending.contains_key(&seq) && pending.len() >= self.max_pending {
            return Err(DbError::LimitExceeded("determinism verifier pending-sequence window exceeded"));
        }
        let entry = pending.entry(seq).or_default();
        entry.insert(label.to_string(), digest);

        if !self.expected_labels.iter().all(|l| entry.contains_key(l)) {
            return Ok(None);
        }

        let mut digests: Vec<(String, ContentHash)> = self.expected_labels.iter().map(|l| (l.clone(), entry[l])).collect();
        let all_match = digests.windows(2).all(|w| w[0].1 == w[1].1);
        pending.remove(&seq);

        if all_match {
            Ok(None)
        } else {
            digests.sort_by(|a, b| a.0.cmp(&b.0));
            Ok(Some(DivergenceReport { seq, digests }))
        }
    }

    pub async fn pending_count(&self) -> usize {
        lock(&self.pending).len()
    }
}
//#endregion 🔖️Determinism

//#region 🔖️Otel
/// 🛰️ Where a completed span goes for OpenTelemetry export — gated behind the `otel`
/// Cargo feature per the contract. Extension seam: no OTLP/otel crate is a workspace dependency
/// today, so this defines the trait shape a real exporter would implement without committing to
/// one yet (repo rule: don't add a dependency that isn't genuinely needed). See
/// `UnwiredOtelExporter` for the honest not-yet-implemented default.
#[cfg(feature = "otel")]
pub trait OtelSpanExporter: Send + Sync {
    async fn export(&self, span: &CompletedSpan) -> Result<(), DbError>;
}

/// 🚫️ An `OtelSpanExporter` that reports `DbError::Unimplemented` rather than silently
/// dropping spans or panicking — the honest placeholder until a real OTLP exporter crate is added
/// as a workspace dependency.
#[cfg(feature = "otel")]
#[derive(Clone, Copy, Default, Debug)]
pub struct UnwiredOtelExporter;

#[cfg(feature = "otel")]
impl OtelSpanExporter for UnwiredOtelExporter {
    async fn export(&self, _span: &CompletedSpan) -> Result<(), DbError> {
        Err(DbError::Unimplemented("otel export requires an OTLP exporter crate not yet a workspace dependency"))
    }
}
//#endregion 🔖️Otel

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
