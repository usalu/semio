//! 🗄️ `db_observe` — the `db` family's observability seam: JSON-lines structured/audit event
//! sinks implementing `Emit`, cardinality-controlled metric registries (counter/gauge/
//! histogram), a bounded span registry, a component health registry, and a runtime determinism
//! verifier (cross-checks independently-produced state-hash streams for the same document, e.g.
//! a live execution against a replay). Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`).
//!
//! 🎯️ Design choice: `db_*` crates below `db_artifact` stay decoupled from this crate's concrete
//! sinks via `db_core`'s `Emit` trait (see its own doc) rather than depending on this crate
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

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use crate::*;

//#region 🔖️Util
/// @emoji 🔓️ Locks `mutex`, recovering the inner value even if a prior holder panicked while
/// holding it — an observability sink must never itself become a source of panics-under-panic
/// for the mailbox/actor code that's often mid-crash-handling when it calls into `Emit::emit`.
// 🚫️async: E1 pure accessor (no suspension: `Mutex::lock` on a never-genuinely-contended
// in-process lock) — see R9
fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}
//#endregion 🔖️Util

//#region 🔖️Json
/// @emoji 📝️ Escapes `raw` per RFC 8259 into `out` (no surrounding quotes).
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

/// @emoji 🧾️ Encodes `event` as one JSON object line (no trailing newline) — the wire shape both
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
/// @emoji 🚰️ Where a sink's JSON-lines actually land — implementable over memory (`MemorySink`,
/// tests/introspection), a file/pipe/`Vec<u8>` (`WriterSink`), or anything else ordered and
/// append-only. Mirrors `pack::PackSink`'s spirit but returns `DbError` (the
/// family's error type) instead of `PackError`, and writes pre-delimited lines rather than raw
/// byte ranges.
pub trait EventSink: Send + Sync {
    async fn write_line(&self, line: &str) -> Result<(), DbError>;
}

/// @emoji 🧠️ An in-memory `EventSink` — the default for tests and for introspecting what a sink
/// would have written without touching the filesystem.
#[derive(Default)]
pub struct MemorySink {
    lines: Mutex<Vec<String>>,
}

impl MemorySink {
    pub fn new() -> Self {
        Self::default()
    }

    /// @emoji 📜️ A snapshot of every line written so far, oldest first.
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

/// @emoji 📄️ An `EventSink` over any `std::io::Write` (a file, a pipe, `Vec<u8>`, …) — one JSON
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
/// @emoji 📡️ A `Emit` implementation that JSON-lines-encodes every event into an
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

    /// @emoji 🚨️ How many `emit` calls lost their event to a sink write failure. `Emit::emit`
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
/// @emoji 🔗️ One link in `AuditSink`'s tamper-evident chain: the sequence number, and the
/// checksum folding this record's line into every checksum before it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AuditLink {
    pub seq: u64,
    pub checksum: u32,
}

async fn fold_checksum(prev_checksum: u32, line: &str) -> u32 {
    let mut buf = Vec::with_capacity(4 + line.len());
    buf.extend_from_slice(&prev_checksum.to_le_bytes());
    buf.extend_from_slice(line.as_bytes());
    pack::crc32c(&buf).await
}

struct AuditChainState {
    base_checksum: u32,
    links: VecDeque<AuditLink>,
}

/// @emoji 🕵️ A `Emit` implementation for the audit trail: same JSON-lines wire shape as
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
    /// @emoji 🆕️ `max_retained` bounds the in-memory chain window `verify_chain` can check
    /// against (oldest links are dropped once exceeded, folded into a running `base_checksum` so
    /// the chain math for the surviving window stays exact) — the durable JSON-lines themselves
    /// are unbounded (owned by `S`), only the tamper-evidence window is capped.
    pub async fn new(sink: S, max_retained: usize) -> Self {
        Self { sink, next_seq: AtomicU64::new(0), state: Mutex::new(AuditChainState { base_checksum: 0, links: VecDeque::new() }), max_retained: max_retained.max(1), failed_writes: AtomicU64::new(0) }
    }

    /// @emoji 🚨️ See `StructuredSink::failed_writes` — same rationale (`Emit::emit` has no
    /// `Result`); a failed durable write is never folded into the chain (the chain only ever
    /// covers records that actually made it to `S`).
    pub async fn failed_writes(&self) -> u64 {
        self.failed_writes.load(Ordering::Relaxed)
    }

    /// @emoji 📜️ A snapshot of the retained chain window, oldest first.
    pub async fn chain(&self) -> Vec<AuditLink> {
        lock(&self.state).links.iter().copied().collect()
    }

    /// @emoji ✅️ Recomputes the checksum chain over `lines` (as read back from durable storage,
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
            let checksum = fold_checksum(prev_checksum, line).await;
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
        let checksum = fold_checksum(prev_checksum, &line).await;
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
/// @emoji 🏷️ A canonicalized (key-sorted) label set — the identity `MetricRegistry` and
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

/// @emoji 🚧️ Bounds how many distinct label sets ("series") a metric name may accumulate — an
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

    /// @emoji 🚪️ Admits `labels` for `metric`: returns `labels` unchanged if it's already a known
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

/// @emoji 📊️ A point-in-time read of one histogram series: `bucket_counts[i]` counts
/// observations `<= bounds[i]`, and `bucket_counts[bounds.len()]` is the `+Inf` overflow bucket.
#[derive(Clone, Debug)]
pub struct HistogramSnapshot {
    pub bounds: Vec<f64>,
    pub bucket_counts: Vec<u64>,
    pub sum: f64,
    pub count: u64,
}

/// @emoji 📈️ Counter/gauge/histogram series, each keyed by `(name, Labels)` and admitted through
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

    /// @emoji ➕️ Monotonically increments the counter series `(name, labels)` by `delta`.
    pub async fn incr_counter(&self, name: &'static str, labels: Labels, delta: u64) {
        let labels = self.cardinality.admit(name, labels);
        let mut counters = lock(&self.counters);
        *counters.entry((name, labels.await)).or_insert(0) += delta;
    }

    pub async fn counter_value(&self, name: &'static str, labels: &Labels) -> u64 {
        lock(&self.counters).get(&(name, labels.clone())).copied().unwrap_or(0)
    }

    /// @emoji 🎚️ Sets the gauge series `(name, labels)` to `value`, overwriting any prior value.
    pub async fn set_gauge(&self, name: &'static str, labels: Labels, value: f64) {
        let labels = self.cardinality.admit(name, labels);
        lock(&self.gauges).insert((name, labels.await), value);
    }

    pub async fn gauge_value(&self, name: &'static str, labels: &Labels) -> Option<f64> {
        lock(&self.gauges).get(&(name, labels.clone())).copied()
    }

    /// @emoji 📊️ Records `value` into the histogram series `(name, labels)`, creating it with
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
/// @emoji ⏱️ Wall-clock seam so `SpanRegistry` durations are testable without real sleeps — the
/// family's `db_testkit::SimClock` (not a dependency of this crate) is the deterministic-
/// simulation analog; this crate only needs the read side.
pub trait Clock: Send + Sync {
    async fn now_ms(&self) -> u64;
}

/// @emoji 🕰️ The real wall clock — `Database::open_at`'s default `SpanRegistry` clock.
#[derive(Clone, Copy, Default, Debug)]
pub struct SystemClock;

impl Clock for SystemClock {
    async fn now_ms(&self) -> u64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as u64)
    }
}

/// @emoji 🔖️ A span's identity within its owning `SpanRegistry` (not globally unique — scope it
/// with the registry that issued it).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SpanId(u64);

/// @emoji 🌳️ A finished span: name, optional parent (for nesting) and document scope, and timing.
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

/// @emoji 🌲️ Tracks in-flight spans and retains the most recently completed ones in a bounded
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

    /// @emoji ▶️ Starts a new span, returning its id (pass to `end`).
    pub async fn start(&self, name: &'static str, parent: Option<SpanId>, document: Option<ArtifactId>) -> SpanId {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let start_ms = self.clock.now_ms().await;
        lock(&self.active).insert(id, ActiveSpan { name, parent, document, start_ms });
        SpanId(id)
    }

    /// @emoji 🏁️ Ends `id`, moving it from active into the completed ring buffer and returning
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

    /// @emoji 📜️ A snapshot of the retained completed spans, oldest first.
    pub async fn completed(&self) -> Vec<CompletedSpan> {
        lock(&self.completed).iter().cloned().collect()
    }
}
//#endregion 🔖️Span

//#region 🔖️Health
/// @emoji 🩺️ One component's health, worst-of-aggregated by `HealthRegistry::report` into the
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

/// @emoji 📋️ A point-in-time health read: every component's individual state plus the worst-of
/// overall.
#[derive(Clone, Debug)]
pub struct HealthReport {
    pub overall: HealthState,
    pub components: Vec<(&'static str, HealthState)>,
}

/// @emoji 🩺️ Aggregates named component health into one worst-of-the-set overall status —
/// `Database::health()`'s data source.
#[derive(Default)]
pub struct HealthRegistry {
    components: Mutex<HashMap<&'static str, HealthState>>,
}

impl HealthRegistry {
    pub fn new() -> HealthRegistry {
        HealthRegistry::default()
    }

    /// @emoji ✏️ Sets (or overwrites) `component`'s current state.
    pub fn set(&self, component: &'static str, state: HealthState) {
        lock(&self.components).insert(component, state);
    }

    /// @emoji 📊️ Snapshots every component (sorted by name for determinism) plus the worst-of
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
/// @emoji ⚠️ What `DeterminismVerifier::record` returns when the labeled digest streams for one
/// `seq` disagree — every expected label's digest, sorted by label for a stable diff.
#[derive(Clone, Debug)]
pub struct DivergenceReport {
    pub seq: u64,
    pub digests: Vec<(String, ContentHash)>,
}

/// @emoji 🧬️ Runtime cross-check that two (or more) independently-produced state-hash streams
/// for the same document agree at every sequence number — e.g. a live execution's per-command
/// `state_hash` (see the frozen `CommandReceipt`) against a replay's recomputation. Complements
/// (does not replace) `db_testkit::assert_replay_deterministic`, a test-only harness in a crate
/// this one may not depend on; this is the always-on runtime version.
pub struct DeterminismVerifier {
    expected_labels: Vec<String>,
    pending: Mutex<HashMap<u64, HashMap<String, ContentHash>>>,
    max_pending: usize,
}

impl DeterminismVerifier {
    /// @emoji 🆕️ `expected_labels` names every stream that must agree (e.g. `["primary",
    /// "replay"]`); `max_pending` bounds how many not-yet-fully-reported sequence numbers this
    /// verifier holds onto at once — a stream that stalls forever can't grow this unboundedly.
    pub async fn new(expected_labels: impl IntoIterator<Item = impl Into<String>>, max_pending: usize) -> DeterminismVerifier {
        DeterminismVerifier { expected_labels: expected_labels.into_iter().map(Into::into).collect(), pending: Mutex::new(HashMap::new()), max_pending: max_pending.max(1) }
    }

    /// @emoji 📮️ Records `label`'s digest for `seq`. Once every expected label has reported for
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
/// @emoji 🛰️ Where a completed span goes for OpenTelemetry export — gated behind the `otel`
/// Cargo feature per the contract. Extension seam: no OTLP/otel crate is a workspace dependency
/// today, so this defines the trait shape a real exporter would implement without committing to
/// one yet (repo rule: don't add a dependency that isn't genuinely needed). See
/// `UnwiredOtelExporter` for the honest not-yet-implemented default.
#[cfg(feature = "otel")]
pub trait OtelSpanExporter: Send + Sync {
    async fn export(&self, span: &CompletedSpan) -> Result<(), DbError>;
}

/// @emoji 🚫️ An `OtelSpanExporter` that reports `DbError::Unimplemented` rather than silently
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
mod tests {
    use super::*;

    async fn hash(byte: u8) -> pack::ContentHash {
        pack::ContentHash([byte; 32])
    }

    //#region 🔖️Json
    #[semio_framework_async_macros::async_test]
    async fn encode_emit_event_json_escapes_and_shapes_fields() {
        let event = EmitEvent::new("wal.append")
            .with_document(ArtifactId::from("doc\"1"))
            .field("bytes", EmitField::U64(42))
            .field("ok", EmitField::Bool(true))
            .field("note", EmitField::Text("line\nbreak".to_string()));

        let json = encode_emit_event_json(&event);
        assert!(json.starts_with("{\"name\":\"wal.append\""));
        assert!(json.contains("\"document\":\"doc\\\"1\""));
        assert!(json.contains("\"bytes\":42"));
        assert!(json.contains("\"ok\":true"));
        assert!(json.contains("\"note\":\"line\\nbreak\""));
        assert!(json.ends_with("}}"));
    }

    #[semio_framework_async_macros::async_test]
    async fn encode_emit_event_json_never_emits_nan_or_infinity_literals() {
        let event = EmitEvent::new("x").field("v", EmitField::F64(f64::NAN));
        let json = encode_emit_event_json(&event);
        assert!(json.contains("\"v\":null"));
        assert!(!json.contains("NaN"));
    }
    //#endregion 🔖️Json

    //#region 🔖️Sink
    #[semio_framework_async_macros::async_test]
    async fn writer_sink_appends_newline_terminated_lines() {
        let sink = WriterSink::new(Vec::<u8>::new());
        sink.write_line("a").await.unwrap();
        sink.write_line("b").await.unwrap();
        let bytes = lock(&sink.writer).clone();
        assert_eq!(String::from_utf8(bytes).unwrap(), "a\nb\n");
    }

    struct FailingSink;
    impl EventSink for FailingSink {
        async fn write_line(&self, _line: &str) -> Result<(), DbError> {
            Err(DbError::Io("disk full".to_string()))
        }
    }
    //#endregion 🔖️Sink

    //#region 🔖️Structured
    #[semio_framework_async_macros::async_test]
    async fn structured_sink_writes_one_json_line_per_event() {
        let memory = MemorySink::new();
        let structured = StructuredSink::new(memory);
        structured.emit(EmitEvent::new("doc.commit").field("seq", EmitField::U64(7))).await;
        let lines = structured.sink.lines().await;
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("\"name\":\"doc.commit\""));
        assert_eq!(structured.failed_writes().await, 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn structured_sink_counts_failed_writes_instead_of_dropping_silently_or_panicking() {
        let structured = StructuredSink::new(FailingSink);
        structured.emit(EmitEvent::new("x")).await;
        structured.emit(EmitEvent::new("y")).await;
        assert_eq!(structured.failed_writes().await, 2);
    }
    //#endregion 🔖️Structured

    //#region 🔖️Audit
    #[semio_framework_async_macros::async_test]
    async fn audit_sink_verify_chain_accepts_an_untampered_log() {
        let memory = MemorySink::new();
        let audit = AuditSink::new(memory, 100).await;
        for i in 0..5u64 {
            audit.emit(EmitEvent::new("audit.write").field("seq", EmitField::U64(i))).await;
        }
        let lines = audit.sink.lines().await;
        assert_eq!(lines.len(), 5);
        assert_eq!(audit.chain().await.len(), 5);
        assert!(audit.verify_chain(&lines).await.is_ok());
    }

    #[semio_framework_async_macros::async_test]
    async fn audit_sink_verify_chain_detects_a_single_tampered_line() {
        let memory = MemorySink::new();
        let audit = AuditSink::new(memory, 100).await;
        for i in 0..5u64 {
            audit.emit(EmitEvent::new("audit.write").field("seq", EmitField::U64(i))).await;
        }
        let mut lines = audit.sink.lines().await;
        lines[2] = lines[2].replace("\"seq\":2", "\"seq\":999");
        let err = audit.verify_chain(&lines).await.unwrap_err();
        match err {
            DbError::Corrupt(message) => assert!(message.contains("seq 2")),
            other => panic!("expected Corrupt, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn audit_sink_bounded_retention_still_verifies_the_surviving_window() {
        let memory = MemorySink::new();
        let audit = AuditSink::new(memory, 2).await;
        for i in 0..5u64 {
            audit.emit(EmitEvent::new("audit.write").field("seq", EmitField::U64(i))).await;
        }
        let all_lines = audit.sink.lines().await;
        assert_eq!(all_lines.len(), 5);
        let chain = audit.chain().await;
        assert_eq!(chain.len(), 2);
        assert_eq!(chain[0].seq, 3);
        assert_eq!(chain[1].seq, 4);
        let retained_lines = all_lines[3..].to_vec();
        assert!(audit.verify_chain(&retained_lines).await.is_ok());
    }

    #[semio_framework_async_macros::async_test]
    async fn audit_sink_does_not_chain_a_failed_write() {
        let audit = AuditSink::new(FailingSink, 10).await;
        audit.emit(EmitEvent::new("x")).await;
        assert_eq!(audit.failed_writes().await, 1);
        assert!(audit.chain().await.is_empty());
    }
    //#endregion 🔖️Audit

    //#region 🔖️Cardinality
    #[semio_framework_async_macros::async_test]
    async fn cardinality_limiter_admits_up_to_the_limit_then_collapses_to_overflow() {
        let limiter = CardinalityLimiter::new(2).await;
        let a = limiter.admit("m", Labels::new([("k", "a".to_string())])).await;
        let b = limiter.admit("m", Labels::new([("k", "b".to_string())])).await;
        let c = limiter.admit("m", Labels::new([("k", "c".to_string())])).await;
        assert_eq!(a, Labels::new([("k", "a".to_string())]));
        assert_eq!(b, Labels::new([("k", "b".to_string())]));
        assert_eq!(c, Labels::new([("cardinality", "overflow".to_string())]));
        assert_eq!(limiter.series_count("m").await, 2);

        // 🔒️ a previously-admitted series stays itself, never gets swept into overflow later.
        let a_again = limiter.admit("m", Labels::new([("k", "a".to_string())])).await;
        assert_eq!(a_again, Labels::new([("k", "a".to_string())]));
    }

    #[semio_framework_async_macros::async_test]
    async fn labels_canonicalize_insertion_order() {
        let l1 = Labels::new([("b", "2".to_string()), ("a", "1".to_string())]);
        let l2 = Labels::new([("a", "1".to_string()), ("b", "2".to_string())]);
        assert_eq!(l1, l2);
    }
    //#endregion 🔖️Cardinality

    //#region 🔖️Metrics
    #[semio_framework_async_macros::async_test]
    async fn metric_registry_counter_accumulates_per_series() {
        let metrics = MetricRegistry::new(16).await;
        metrics.incr_counter("cmd.count", Labels::none().await, 1).await;
        metrics.incr_counter("cmd.count", Labels::none().await, 4).await;
        assert_eq!(metrics.counter_value("cmd.count", &Labels::none().await).await, 5);
    }

    #[semio_framework_async_macros::async_test]
    async fn metric_registry_gauge_overwrites() {
        let metrics = MetricRegistry::new(16).await;
        metrics.set_gauge("mailbox.depth", Labels::none().await, 3.0).await;
        metrics.set_gauge("mailbox.depth", Labels::none().await, 7.0).await;
        assert_eq!(metrics.gauge_value("mailbox.depth", &Labels::none().await).await, Some(7.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn metric_registry_histogram_buckets_are_upper_inclusive_with_overflow_bucket() {
        let metrics = MetricRegistry::new(16).await;
        let bounds = [1.0, 5.0, 10.0];
        for v in [0.5, 1.0, 3.0, 5.0, 8.0, 20.0] {
            metrics.observe_histogram("latency_ms", Labels::none().await, &bounds, v).await.unwrap();
        }
        let snap = metrics.histogram_snapshot("latency_ms", &Labels::none().await).await.unwrap();
        assert_eq!(snap.bucket_counts, vec![2, 2, 1, 1]);
        assert_eq!(snap.count, 6);
    }

    #[semio_framework_async_macros::async_test]
    async fn metric_registry_histogram_rejects_a_bounds_length_change() {
        let metrics = MetricRegistry::new(16).await;
        metrics.observe_histogram("h", Labels::none().await, &[1.0, 2.0], 1.0).await.unwrap();
        let err = metrics.observe_histogram("h", Labels::none().await, &[1.0], 1.0).await.unwrap_err();
        assert!(matches!(err, DbError::InvalidArgument(_)));
    }
    //#endregion 🔖️Metrics

    //#region 🔖️Span
    struct FakeClock(AtomicU64);
    impl Clock for FakeClock {
        async fn now_ms(&self) -> u64 {
            self.0.fetch_add(10, Ordering::Relaxed)
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn span_registry_records_duration_and_moves_active_to_completed() {
        let registry = SpanRegistry::with_clock(FakeClock(AtomicU64::new(0)), 16).await;
        let id = registry.start("doc.commit", None, None).await;
        assert_eq!(registry.active_count().await, 1);
        let completed = registry.end(id).await.unwrap();
        assert_eq!(completed.duration_ms, 10);
        assert_eq!(registry.active_count().await, 0);
        assert_eq!(registry.completed().await.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn span_registry_double_end_returns_none_instead_of_panicking() {
        let registry = SpanRegistry::with_clock(FakeClock(AtomicU64::new(0)), 16).await;
        let id = registry.start("s", None, None).await;
        assert!(registry.end(id).await.is_some());
        assert!(registry.end(id).await.is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn span_registry_retention_is_bounded() {
        let registry = SpanRegistry::with_clock(FakeClock(AtomicU64::new(0)), 2).await;
        for _ in 0..5 {
            let id = registry.start("s", None, None).await;
            registry.end(id).await;
        }
        assert_eq!(registry.completed().await.len(), 2);
    }
    //#endregion 🔖️Span

    //#region 🔖️Health
    #[semio_framework_async_macros::async_test]
    async fn health_registry_reports_worst_of_all_components() {
        let health = HealthRegistry::new();
        health.set("wal", HealthState::Healthy);
        health.set("storage", HealthState::Degraded("slow fsync".to_string()));
        let report = health.report();
        assert_eq!(report.overall, HealthState::Degraded("slow fsync".to_string()));
        assert_eq!(report.components.len(), 2);

        health.set("storage", HealthState::Unhealthy("disk full".to_string()));
        assert_eq!(health.report().overall, HealthState::Unhealthy("disk full".to_string()));
    }

    #[semio_framework_async_macros::async_test]
    async fn health_registry_defaults_to_healthy_with_no_components() {
        let health = HealthRegistry::new();
        assert_eq!(health.report().overall, HealthState::Healthy);
    }
    //#endregion 🔖️Health

    //#region 🔖️Determinism
    #[semio_framework_async_macros::async_test]
    async fn determinism_verifier_reports_no_divergence_on_matching_digests() {
        let verifier = DeterminismVerifier::new(["primary", "replay"], 16).await;
        assert!(verifier.record(0, "primary", hash(1).await).await.unwrap().is_none());
        assert!(verifier.record(0, "replay", hash(1).await).await.unwrap().is_none());
        assert_eq!(verifier.pending_count().await, 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn determinism_verifier_reports_divergence_on_mismatched_digests() {
        let verifier = DeterminismVerifier::new(["primary", "replay"], 16).await;
        verifier.record(3, "primary", hash(1).await).await.unwrap();
        let report = verifier.record(3, "replay", hash(2).await).await.unwrap().unwrap();
        assert_eq!(report.seq, 3);
        assert_eq!(report.digests.len(), 2);
        assert_eq!(verifier.pending_count().await, 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn determinism_verifier_bounds_pending_window() {
        let verifier = DeterminismVerifier::new(["primary", "replay"], 2).await;
        verifier.record(0, "primary", hash(1).await).await.unwrap();
        verifier.record(1, "primary", hash(1).await).await.unwrap();
        let err = verifier.record(2, "primary", hash(1).await).await.unwrap_err();
        assert!(matches!(err, DbError::LimitExceeded(_)));
    }
    //#endregion 🔖️Determinism

    //#region 🔖️Otel
    #[cfg(feature = "otel")]
    #[semio_framework_async_macros::async_test]
    async fn unwired_otel_exporter_reports_unimplemented_rather_than_panicking() {
        let span = CompletedSpan { id: SpanId(0), name: "s", parent: None, document: None, start_ms: 0, end_ms: 1, duration_ms: 1 };
        let err = UnwiredOtelExporter.export(&span).await.unwrap_err();
        assert!(matches!(err, DbError::Unimplemented(_)));
    }
    //#endregion 🔖️Otel
}
//#endregion 🧪️Tests
