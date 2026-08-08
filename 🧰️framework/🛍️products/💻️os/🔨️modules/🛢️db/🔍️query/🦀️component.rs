//! 🗄️ `db_query` — the `db` family's query engine: consistency-mode resolution
//! (`Canonical`/`AtLeast`/`Exact`/`Historical`/`Speculative`/`PreviewAugmented`), a dynamic `Value`
//! tree with typed convenience conversions, a `Predicate`/`Select`/`Query` IR, a minimal
//! cost-free planner that recognizes full-text-pushdown opportunities, a streaming `QueryStream`,
//! and `LiveQuery` incremental diffing. Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`) and Part 2 of the approved plan.
//!
//! 🎯️ Design choice: `db_state`'s persistent structures (`PMap`/`PVec`/…) are `Rc`-based —
//! deliberately single-threaded, matching the one-actor-per-document mailbox model the family is
//! built on (see `db_state`'s own module doc). None of this crate's traits (`QuerySource`,
//! `FullTextLookup`, `ConsistencyResolver`) carry a `Send + Sync` supertrait bound for that reason:
//! adding one would make every `Rc`-based `db_state` collection unable to implement them.
//!
//! 🌉️ `db_projection` (this crate's fourth declared dependency, per the frozen `Cargo.toml`) has now
//! landed. `Consistency::Historical`/`PreviewAugmented` resolution stays expressed purely against
//! this crate's own `ConsistencyResolver` seam (`IndexConsistencyResolver` below, wired to the
//! sibling `db_index` crate) — a document's *frontier* never needed `db_projection` to resolve, only
//! `db_index`'s `CommitIndex`/`FrontierIndex`. What plugs in now, per the `🔖️ProjectionBridge`
//! region below: `Value` implements `db_projection::ProjectionState`, and `projection_query_source`
//! turns the plain state bytes a caller already retrieved from
//! `db_projection::ProjectionEngine::state_at` (for `Historical`/`Canonical`) or
//! `::preview_augmented` (for `Speculative`/`PreviewAugmented`) into a `ProjectionSource`
//! (`QuerySource`) this crate's `execute`/planner/`LiveQuery` machinery runs over unmodified. It
//! deliberately takes raw bytes rather than holding a `&ProjectionEngine` reference itself:
//! constructing/registering a real `db_projection::ErasedProjection` requires naming
//! `protocol::MutationEnvelope` in its `apply`/`apply_bytes` signature, and this crate's frozen
//! dependency list grants it `db_projection` but not `protocol` (only `db_document`, which already
//! owns envelope interpretation, has both) — so the layer that calls `state_at`/`preview_augmented`
//! hands this crate only the resulting bytes, never the engine or the envelope.

use {check_len, DbError, Frontier};
use db_index::{CommitIndex, FrontierIndex, FullTextIndex};
use db_projection::ProjectionState;
use db_state::PVec;
use std::cmp::Ordering;
use std::collections::BTreeMap;

//#region 🔖️Value
/// @emoji 🧬️ The dynamic value model every `Query` evaluates against. Deliberately this crate's
/// own type rather than `pack_value` (forbidden by the contract's hard dependency rules) or a
/// `db_state` structure directly (those are the *storage* representation; a document's queryable
/// shape is resolved into this tree by whichever layer above `db_query` owns the schema — typically
/// `db_document`).
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    Bytes(Vec<u8>),
    List(Vec<Value>),
    Map(BTreeMap<String, Value>),
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Int(v)
    }
}
impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Float(v)
    }
}
impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}
impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::Text(v)
    }
}
impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::Text(v.to_string())
    }
}
impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Value::Bytes(v)
    }
}

/// @emoji 🥇️ Cross-type ordering rank, used only as `compare_values`'s tie-breaker between values
/// of different variants — this crate's own choice of total order (the contract doesn't specify
/// one), documented once here rather than re-derived at every call site.
fn value_rank(value: &Value) -> u8 {
    match value {
        Value::Null => 0,
        Value::Bool(_) => 1,
        Value::Int(_) => 2,
        Value::Float(_) => 3,
        Value::Text(_) => 4,
        Value::Bytes(_) => 5,
        Value::List(_) => 6,
        Value::Map(_) => 7,
    }
}

/// @emoji ⚖️ A total order over `Value` (needed since `f64` alone isn't `Ord`): same-variant pairs
/// compare structurally (`Int`/`Float` cross-compare numerically), everything else falls back to
/// `value_rank`. `Ordering::Equal` on an unorderable float pair (`NaN`) rather than panicking.
pub fn compare_values(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Null, Value::Null) => Ordering::Equal,
        (Value::Bool(x), Value::Bool(y)) => x.cmp(y),
        (Value::Int(x), Value::Int(y)) => x.cmp(y),
        (Value::Float(x), Value::Float(y)) => x.partial_cmp(y).unwrap_or(Ordering::Equal),
        (Value::Int(x), Value::Float(y)) => (*x as f64).partial_cmp(y).unwrap_or(Ordering::Equal),
        (Value::Float(x), Value::Int(y)) => x.partial_cmp(&(*y as f64)).unwrap_or(Ordering::Equal),
        (Value::Text(x), Value::Text(y)) => x.cmp(y),
        (Value::Bytes(x), Value::Bytes(y)) => x.cmp(y),
        (Value::List(x), Value::List(y)) => {
            for (xi, yi) in x.iter().zip(y.iter()) {
                let ord = compare_values(xi, yi);
                if ord != Ordering::Equal {
                    return ord;
                }
            }
            x.len().cmp(&y.len())
        }
        (Value::Map(x), Value::Map(y)) => {
            for ((xk, xv), (yk, yv)) in x.iter().zip(y.iter()) {
                let key_ord = xk.cmp(yk);
                if key_ord != Ordering::Equal {
                    return key_ord;
                }
                let val_ord = compare_values(xv, yv);
                if val_ord != Ordering::Equal {
                    return val_ord;
                }
            }
            x.len().cmp(&y.len())
        }
        _ => value_rank(a).cmp(&value_rank(b)),
    }
}

/// @emoji 🧵️ A best-effort text rendering of any `Value`, used only by `Predicate::FullText`'s
/// index-free fallback match (see that variant's doc).
fn stringify_value(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Text(s) => s.clone(),
        Value::Bytes(b) => String::from_utf8_lossy(b).into_owned(),
        Value::List(items) => items.iter().map(stringify_value).collect::<Vec<_>>().join(" "),
        Value::Map(map) => map.values().map(stringify_value).collect::<Vec<_>>().join(" "),
    }
}

/// @emoji 🧩️ One step of a `Path`: a named field into a `Value::Map`, or a positional index into a
/// `Value::List`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PathSegment {
    Field(String),
    Index(usize),
}

/// @emoji 🛤️ A field/index path into a `Value` tree — the addressing scheme every `Predicate`/
/// `SortKey`/`Select::Paths` entry resolves through. Supports both the "typed" construction style
/// (`Path::field("a").push_field("b")`) and the "dynamic" style (`Path::parse("a.b.0")`) — the
/// contract's "typed+dynamic queries" duality lives here, one shared representation underneath.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Path(pub Vec<PathSegment>);

impl Path {
    pub fn empty() -> Path {
        Path(Vec::new())
    }

    pub fn field(name: impl Into<String>) -> Path {
        Path(vec![PathSegment::Field(name.into())])
    }

    pub fn push_field(mut self, name: impl Into<String>) -> Path {
        self.0.push(PathSegment::Field(name.into()));
        self
    }

    pub fn push_index(mut self, index: usize) -> Path {
        self.0.push(PathSegment::Index(index));
        self
    }

    /// @emoji 🔤️ Splits a dotted string into segments, treating any segment that parses as a plain
    /// `usize` as `PathSegment::Index` and everything else as `PathSegment::Field` — this crate's
    /// own convention for the "dynamic" half of typed+dynamic queries. `""` parses to `Path::empty()`
    /// (the whole document), matching `Predicate::FullText`'s use of an empty path for the same.
    pub fn parse(spec: &str) -> Path {
        if spec.is_empty() {
            return Path::empty();
        }
        Path(
            spec.split('.')
                .map(|segment| match segment.parse::<usize>() {
                    Ok(index) => PathSegment::Index(index),
                    Err(_) => PathSegment::Field(segment.to_string()),
                })
                .collect(),
        )
    }

    /// @emoji 🔎️ Walks `value` through this path's segments. `None` on any type mismatch (e.g. an
    /// `Index` segment against a `Map`) or out-of-range index/absent field along the way.
    pub fn get<'a>(&self, value: &'a Value) -> Option<&'a Value> {
        let mut current = value;
        for segment in &self.0 {
            current = match (segment, current) {
                (PathSegment::Field(name), Value::Map(map)) => map.get(name)?,
                (PathSegment::Index(index), Value::List(list)) => list.get(*index)?,
                _ => return None,
            };
        }
        Some(current)
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<String> = self
            .0
            .iter()
            .map(|segment| match segment {
                PathSegment::Field(name) => name.clone(),
                PathSegment::Index(index) => index.to_string(),
            })
            .collect();
        write!(f, "{}", parts.join("."))
    }
}
//#endregion 🔖️Value

//#region 🔖️Consistency
/// @emoji 🧭️ The six read-consistency modes `DocumentHandle::query` accepts (frozen in the db
/// facade's stable API): read the live head, wait for at least a given frontier, pin to an exact
/// one, replay as of a named commit, read through a named preview overlay only, or read canonical
/// state augmented by a named preview.
#[derive(Clone, Debug, PartialEq)]
pub enum Consistency {
    Canonical,
    AtLeast(Frontier),
    Exact(Frontier),
    Historical(String),
    Speculative(String),
    PreviewAugmented(String),
}

/// @emoji 🗺️ What a `Consistency` resolves to: the concrete `Frontier` a `QuerySource` should be
/// materialized at, which named preview (if any) participates, and whether this is a replay read
/// (informational — callers use it to decide whether write-side effects like live-query
/// registration are even meaningful for this read).
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedConsistency {
    pub frontier: Frontier,
    pub preview_id: Option<String>,
    pub historical: bool,
}

/// @emoji 🔌️ What `resolve_consistency` needs from its caller: the document's current frontier, and
/// a commit-id → frontier lookup for `Historical`. Kept minimal and storage-agnostic so this crate
/// never needs a concrete storage dependency of its own — `IndexConsistencyResolver` below is the
/// ready-made adapter over the sibling `db_index` crate.
pub trait ConsistencyResolver {
    fn current_frontier(&self) -> Result<Frontier, DbError>;
    fn frontier_for_commit(&self, commit_id: &str) -> Result<Frontier, DbError>;
}

/// @emoji 🧮️ Resolves `consistency` against `resolver` into a concrete `ResolvedConsistency`.
/// `AtLeast`/`Exact` are checked against `Frontier::dominates`/equality respectively — both are the
/// contract's own definition of those two modes, not this crate's invention.
pub fn resolve_consistency(consistency: &Consistency, resolver: &dyn ConsistencyResolver) -> Result<ResolvedConsistency, DbError> {
    match consistency {
        Consistency::Canonical => Ok(ResolvedConsistency { frontier: resolver.current_frontier()?, preview_id: None, historical: false }),
        Consistency::AtLeast(target) => {
            let current = resolver.current_frontier()?;
            if !current.dominates(target)? {
                return Err(DbError::Unavailable(format!("current frontier (head_seq {}) has not yet reached requested AtLeast frontier (head_seq {})", current.head_seq, target.head_seq)));
            }
            Ok(ResolvedConsistency { frontier: current, preview_id: None, historical: false })
        }
        Consistency::Exact(target) => {
            let current = resolver.current_frontier()?;
            if current != *target {
                return Err(DbError::NotFound(format!("no frontier exactly matching requested Exact frontier (head_seq {}, commit_seq {})", target.head_seq, target.commit_seq)));
            }
            Ok(ResolvedConsistency { frontier: current, preview_id: None, historical: false })
        }
        Consistency::Historical(commit_id) => Ok(ResolvedConsistency { frontier: resolver.frontier_for_commit(commit_id)?, preview_id: None, historical: true }),
        Consistency::Speculative(preview_id) => Ok(ResolvedConsistency { frontier: resolver.current_frontier()?, preview_id: Some(preview_id.clone()), historical: false }),
        Consistency::PreviewAugmented(preview_id) => Ok(ResolvedConsistency { frontier: resolver.current_frontier()?, preview_id: Some(preview_id.clone()), historical: false }),
    }
}

/// @emoji 🔗️ A `ConsistencyResolver` backed by real `db_index` typed indexes: `Historical`
/// resolution is exactly `CommitIndex::lookup` (commit id → command seq) followed by
/// `FrontierIndex::lookup` (command seq → frontier), matching `CommitIndex`'s own doc comment
/// ("for `Consistency::Historical(commit_id)` query resolution"). Construction needs a live
/// `db_storage::IndexStorage` on the caller's side — this crate stays storage-agnostic by only ever
/// holding the already-constructed typed index handles, never constructing them itself.
pub struct IndexConsistencyResolver<'a> {
    pub commits: CommitIndex<'a>,
    pub frontiers: FrontierIndex<'a>,
}

impl<'a> ConsistencyResolver for IndexConsistencyResolver<'a> {
    fn current_frontier(&self) -> Result<Frontier, DbError> {
        self.frontiers.latest()?.ok_or_else(|| DbError::NotFound("no frontier has been recorded for this document yet".to_string()))
    }

    fn frontier_for_commit(&self, commit_id: &str) -> Result<Frontier, DbError> {
        let command_seq = self.commits.lookup(commit_id)?.ok_or_else(|| DbError::NotFound(format!("unknown commit id {commit_id:?}")))?;
        self.frontiers.lookup(command_seq)?.ok_or_else(|| DbError::NotFound(format!("no frontier recorded at command_seq {command_seq}")))
    }
}
//#endregion 🔖️Consistency

//#region 🔖️Query
/// @emoji 🎯️ A single filter condition. `And`/`Or`/`Not` semio_compose_rs the rest into an arbitrary boolean
/// tree; `And([])` is vacuously true and `Or([])` is vacuously false, matching standard boolean
/// algebra rather than being treated as errors.
#[derive(Clone, Debug, PartialEq)]
pub enum Predicate {
    Eq(Path, Value),
    Ne(Path, Value),
    Lt(Path, Value),
    Lte(Path, Value),
    Gt(Path, Value),
    Gte(Path, Value),
    Exists(Path),
    Contains(Path, Value),
    /// @emoji 🔤️ Case-insensitive substring match. `Path::empty()` matches against the whole
    /// document's `stringify_value` rendering. Doubles as both the index-free evaluation rule (used
    /// on every row, including full-text-pushdown candidates, as the authoritative re-check — see
    /// `QueryPlan::FullTextPushdown`'s doc) and the predicate the planner recognizes for pushdown.
    FullText(Path, String),
    And(Vec<Predicate>),
    Or(Vec<Predicate>),
    Not(Box<Predicate>),
}

fn compare_op(path: &Path, expected: &Value, value: &Value, accept: fn(Ordering) -> bool) -> bool {
    path.get(value).is_some_and(|found| accept(compare_values(found, expected)))
}

/// @emoji ✅️ Evaluates `predicate` against one materialized row `value`. A missing path is treated
/// as failing every comparison predicate (including `Ne`, deliberately: "the field isn't even
/// present" is not the same claim as "the field is present and differs").
fn eval_predicate(predicate: &Predicate, value: &Value) -> bool {
    match predicate {
        Predicate::Eq(path, expected) => compare_op(path, expected, value, |ord| ord == Ordering::Equal),
        Predicate::Ne(path, expected) => compare_op(path, expected, value, |ord| ord != Ordering::Equal),
        Predicate::Lt(path, expected) => compare_op(path, expected, value, |ord| ord == Ordering::Less),
        Predicate::Lte(path, expected) => compare_op(path, expected, value, |ord| ord != Ordering::Greater),
        Predicate::Gt(path, expected) => compare_op(path, expected, value, |ord| ord == Ordering::Greater),
        Predicate::Gte(path, expected) => compare_op(path, expected, value, |ord| ord != Ordering::Less),
        Predicate::Exists(path) => path.get(value).is_some(),
        Predicate::Contains(path, needle) => match path.get(value) {
            Some(Value::List(items)) => items.iter().any(|item| compare_values(item, needle) == Ordering::Equal),
            Some(Value::Text(text)) => match needle {
                Value::Text(needle_text) => text.contains(needle_text.as_str()),
                _ => false,
            },
            _ => false,
        },
        Predicate::FullText(path, term) => {
            let target = if path.0.is_empty() { Some(value.clone()) } else { path.get(value).cloned() };
            target.is_some_and(|found| stringify_value(&found).to_lowercase().contains(&term.to_lowercase()))
        }
        Predicate::And(predicates) => predicates.iter().all(|inner| eval_predicate(inner, value)),
        Predicate::Or(predicates) => predicates.iter().any(|inner| eval_predicate(inner, value)),
        Predicate::Not(inner) => !eval_predicate(inner, value),
    }
}

/// @emoji 🗂️ Which fields a query returns: the whole materialized `Value`, or a projected `Map`
/// keyed by each requested `Path`'s dotted `Display` string.
#[derive(Clone, Debug, PartialEq, Default)]
pub enum Select {
    #[default]
    All,
    Paths(Vec<Path>),
}

impl Select {
    fn project(&self, value: &Value) -> Value {
        match self {
            Select::All => value.clone(),
            Select::Paths(paths) => {
                let mut map = BTreeMap::new();
                for path in paths {
                    if let Some(found) = path.get(value) {
                        map.insert(path.to_string(), found.clone());
                    }
                }
                Value::Map(map)
            }
        }
    }
}

/// @emoji 🔀️ One sort key: a `Path` to compare by, and its direction. A missing path sorts as
/// `Value::Null` (via `value_rank`, the lowest rank), so rows lacking the sort field sort first
/// ascending / last descending, rather than being excluded or causing an error.
#[derive(Clone, Debug, PartialEq)]
pub struct SortKey {
    pub path: Path,
    pub descending: bool,
}

impl SortKey {
    pub fn ascending(path: Path) -> SortKey {
        SortKey { path, descending: false }
    }

    pub fn descending(path: Path) -> SortKey {
        SortKey { path, descending: true }
    }
}

fn compare_rows(a: &Value, b: &Value, sort: &[SortKey]) -> Ordering {
    for key in sort {
        let left = key.path.get(a).cloned().unwrap_or(Value::Null);
        let right = key.path.get(b).cloned().unwrap_or(Value::Null);
        let mut ordering = compare_values(&left, &right);
        if key.descending {
            ordering = ordering.reverse();
        }
        if ordering != Ordering::Equal {
            return ordering;
        }
    }
    Ordering::Equal
}

/// @emoji 📜️ A complete query: what to return (`select`), which rows qualify (`filter`), in what
/// order (`sort`), and how many (`limit`/`offset`). Builder-style construction (`Query::new()
/// .filter(...).sort(...).limit(...)`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Query {
    pub select: Select,
    pub filter: Option<Predicate>,
    pub sort: Vec<SortKey>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

impl Query {
    pub fn new() -> Query {
        Query::default()
    }

    pub fn select(mut self, select: Select) -> Query {
        self.select = select;
        self
    }

    pub fn filter(mut self, predicate: Predicate) -> Query {
        self.filter = Some(predicate);
        self
    }

    pub fn sort(mut self, sort: Vec<SortKey>) -> Query {
        self.sort = sort;
        self
    }

    pub fn limit(mut self, limit: u64) -> Query {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u64) -> Query {
        self.offset = Some(offset);
        self
    }
}
//#endregion 🔖️Query

//#region 🔖️Limits
/// @emoji 🛡️ Query-side ceilings, checked via `check_len` before allocating the next row
/// or byte, mirroring the family-wide "validate before allocating" invariant. `max_result_bytes`
/// defaults to `DbLimits::default().max_query_bytes` — the same budget the mailbox layer
/// already reserves for one query's wire payload, kept as a single source of truth.
#[derive(Clone, Debug)]
pub struct QueryLimits {
    pub max_scan_rows: u64,
    pub max_result_rows: u64,
    pub max_result_bytes: u64,
}

impl Default for QueryLimits {
    fn default() -> Self {
        QueryLimits { max_scan_rows: 1_000_000, max_result_rows: 10_000, max_result_bytes: DbLimits::default().max_query_bytes }
    }
}

/// @emoji 📐️ A conservative byte-size estimate of `value` — sums scalar widths and container
/// element/key bytes. Only used to enforce `QueryLimits::max_result_bytes`, so slight
/// under/over-estimation (e.g. `Value` enum tag overhead is ignored) is acceptable; being cheap and
/// allocation-free is what matters.
fn value_byte_estimate(value: &Value) -> u64 {
    match value {
        Value::Null | Value::Bool(_) => 1,
        Value::Int(_) | Value::Float(_) => 8,
        Value::Text(text) => text.len() as u64,
        Value::Bytes(bytes) => bytes.len() as u64,
        Value::List(items) => items.iter().map(value_byte_estimate).sum(),
        Value::Map(map) => map.iter().map(|(key, val)| key.len() as u64 + value_byte_estimate(val)).sum(),
    }
}

fn estimate_result_bytes(rows: &[(RowId, Value)]) -> u64 {
    rows.iter().map(|(_, value)| 8 + value_byte_estimate(value)).sum()
}
//#endregion 🔖️Limits

//#region 🔖️QuerySource
/// @emoji 🆔️ An opaque per-document row identifier. `u64` (not a `String`) to match `db_index`'s
/// own `doc_ref` convention for full-text/touched-region postings (see `db_index::FullTextIndex`'s
/// doc) — a `FullTextLookup`'s postings and a `QuerySource`'s row ids are meant to be the same
/// space, so pushdown candidates resolve back into `QuerySource::get` directly.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct RowId(pub u64);

/// @emoji 🚰️ What the planner/evaluator need from a materialized document: every row (for a full
/// scan), or one row by id (for a pushdown candidate list). No `Send + Sync` bound — see the module
/// doc's note on `db_state`'s `Rc`-based structures.
pub trait QuerySource {
    fn scan(&self) -> Box<dyn Iterator<Item = (RowId, Value)> + '_>;

    /// @emoji 🎯️ Default: linear `scan` + find. Override when a cheaper direct lookup exists (e.g.
    /// `PVec`'s below, which is index-addressed).
    fn get(&self, id: RowId) -> Option<Value> {
        self.scan().find(|(row_id, _)| *row_id == id).map(|(_, value)| value)
    }
}

/// @emoji 🧵️ The natural `QuerySource` over a `db_state::PVec`: row id = element index. This is the
/// crate's one built-in `QuerySource`, demonstrating the intended wiring to `db_state`'s persistent
/// structures — a caller with a richer per-document schema (`db_document`) supplies its own
/// `QuerySource` over whatever `PMap`/`PTree`/overlay shape it actually stores.
impl QuerySource for PVec<Value> {
    fn scan(&self) -> Box<dyn Iterator<Item = (RowId, Value)> + '_> {
        Box::new(self.iter().enumerate().map(|(index, value)| (RowId(index as u64), value.clone())))
    }

    fn get(&self, id: RowId) -> Option<Value> {
        PVec::get(self, id.0 as usize).cloned()
    }
}

/// @emoji 🔌️ What a full-text pushdown needs: term → candidate row ids. `db_index::FullTextIndex`
/// implements this directly below (its `doc_ref` postings are exactly this trait's `RowId`s).
pub trait FullTextLookup {
    fn search(&self, term: &str) -> Result<Vec<RowId>, DbError>;
}

impl<'a> FullTextLookup for FullTextIndex<'a> {
    fn search(&self, term: &str) -> Result<Vec<RowId>, DbError> {
        Ok(FullTextIndex::search(self, term)?.into_iter().map(RowId).collect())
    }
}
//#endregion 🔖️QuerySource

//#region 🔖️ProjectionBridge
/// @emoji 🛡️ Ceiling on a decoded `Value::List`/`Value::Map`'s declared element count, checked via
/// `check_len` BEFORE `decode_value` allocates its `Vec`/`BTreeMap` — the same
/// "validate before allocating" invariant `QueryLimits` and every decoder across the family holds to.
const MAX_PROJECTION_VALUE_ELEMENTS: u64 = 1_000_000;

/// @emoji 👉️ A read-only cursor over `bytes`, used only by `decode_value` — every `take*` call
/// returns `DbError::Corrupt` (never panics/indexes out of bounds) on a truncated buffer.
struct ValueCursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> ValueCursor<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], DbError> {
        if self.pos + n > self.bytes.len() {
            return Err(DbError::Corrupt("projection value bytes truncated".to_string()));
        }
        let slice = &self.bytes[self.pos..self.pos + n];
        self.pos += n;
        Ok(slice)
    }

    fn take_u8(&mut self) -> Result<u8, DbError> {
        Ok(self.take(1)?[0])
    }

    fn take_u32(&mut self) -> Result<u32, DbError> {
        let bytes = self.take(4)?;
        Ok(u32::from_le_bytes(bytes.try_into().expect("take(4) returns exactly 4 bytes")))
    }
}

/// @emoji ✍️ `Value`'s own canonical binary encoding — this crate's own choice (the
/// `db_projection::ProjectionState` trait leaves the exact byte shape unspecified): a tag byte per
/// variant followed by the variant's payload, `List`/`Map` recursing depth-first. `Map` is
/// `BTreeMap`-backed, so its entries are already emitted in ascending key order.
fn encode_value(value: &Value, out: &mut Vec<u8>) {
    match value {
        Value::Null => out.push(0),
        Value::Bool(b) => {
            out.push(1);
            out.push(*b as u8);
        }
        Value::Int(i) => {
            out.push(2);
            out.extend_from_slice(&i.to_le_bytes());
        }
        Value::Float(f) => {
            out.push(3);
            out.extend_from_slice(&f.to_le_bytes());
        }
        Value::Text(s) => {
            out.push(4);
            out.extend_from_slice(&(s.len() as u32).to_le_bytes());
            out.extend_from_slice(s.as_bytes());
        }
        Value::Bytes(b) => {
            out.push(5);
            out.extend_from_slice(&(b.len() as u32).to_le_bytes());
            out.extend_from_slice(b);
        }
        Value::List(items) => {
            out.push(6);
            out.extend_from_slice(&(items.len() as u32).to_le_bytes());
            for item in items {
                encode_value(item, out);
            }
        }
        Value::Map(map) => {
            out.push(7);
            out.extend_from_slice(&(map.len() as u32).to_le_bytes());
            for (key, val) in map {
                out.extend_from_slice(&(key.len() as u32).to_le_bytes());
                out.extend_from_slice(key.as_bytes());
                encode_value(val, out);
            }
        }
    }
}

/// @emoji 📖️ Inverse of `encode_value`. Rejects an unknown tag, a truncated buffer, or an
/// over-large declared element count with `DbError::Corrupt`/`DbError::LimitExceeded` rather than
/// panicking or over-allocating.
fn decode_value(cursor: &mut ValueCursor<'_>) -> Result<Value, DbError> {
    match cursor.take_u8()? {
        0 => Ok(Value::Null),
        1 => Ok(Value::Bool(cursor.take_u8()? != 0)),
        2 => Ok(Value::Int(i64::from_le_bytes(cursor.take(8)?.try_into().expect("take(8) returns exactly 8 bytes")))),
        3 => Ok(Value::Float(f64::from_le_bytes(cursor.take(8)?.try_into().expect("take(8) returns exactly 8 bytes")))),
        4 => {
            let len = cursor.take_u32()? as usize;
            let bytes = cursor.take(len)?;
            String::from_utf8(bytes.to_vec()).map(Value::Text).map_err(|_| DbError::Corrupt("projection value text is not valid utf-8".to_string()))
        }
        5 => {
            let len = cursor.take_u32()? as usize;
            Ok(Value::Bytes(cursor.take(len)?.to_vec()))
        }
        6 => {
            let count = cursor.take_u32()? as u64;
            check_len(count, MAX_PROJECTION_VALUE_ELEMENTS, "db_query::projection_value_list_len")?;
            let mut items = Vec::with_capacity(count.min(1024) as usize);
            for _ in 0..count {
                items.push(decode_value(cursor)?);
            }
            Ok(Value::List(items))
        }
        7 => {
            let count = cursor.take_u32()? as u64;
            check_len(count, MAX_PROJECTION_VALUE_ELEMENTS, "db_query::projection_value_map_len")?;
            let mut map = BTreeMap::new();
            for _ in 0..count {
                let key_len = cursor.take_u32()? as usize;
                let key_bytes = cursor.take(key_len)?;
                let key = String::from_utf8(key_bytes.to_vec()).map_err(|_| DbError::Corrupt("projection value map key is not valid utf-8".to_string()))?;
                map.insert(key, decode_value(cursor)?);
            }
            Ok(Value::Map(map))
        }
        other => Err(DbError::Corrupt(format!("unknown projection value tag {other}"))),
    }
}

/// @emoji 🔌️ `Value`'s `db_projection::ProjectionState` impl — lets any `db_projection::ProjectionClass`
/// (registered by a higher layer, e.g. `db_document`, which owns the `protocol::MutationEnvelope`
/// interpretation this crate deliberately never touches — see the module doc) declare `State = Value`
/// and get this crate's query/planner/live-diff machinery for free over its checkpointed state, via
/// `projection_query_source` below.
impl ProjectionState for Value {
    fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        encode_value(self, &mut out);
        out
    }

    fn decode(bytes: &[u8]) -> Result<Value, DbError> {
        let mut cursor = ValueCursor { bytes, pos: 0 };
        let value = decode_value(&mut cursor)?;
        if cursor.pos != bytes.len() {
            return Err(DbError::Corrupt("trailing bytes after a projection value".to_string()));
        }
        Ok(value)
    }
}

/// @emoji 📽️ A `QuerySource` over one decoded projection state `Value`, row-shaped so `execute`/
/// `LiveQuery` can run over it exactly like any other source: a `List` becomes one row per element
/// (positional `RowId`, matching `PVec<Value>`'s convention above), a `Map` becomes one row per
/// entry (`RowId` assigned by ascending key order — `BTreeMap`'s natural iteration, so it's stable
/// across calls for the same map shape), and any other `Value` becomes a single `RowId(0)` row.
pub struct ProjectionSource(Vec<(RowId, Value)>);

impl ProjectionSource {
    pub fn from_value(value: Value) -> ProjectionSource {
        match value {
            Value::List(items) => ProjectionSource(items.into_iter().enumerate().map(|(index, item)| (RowId(index as u64), item)).collect()),
            Value::Map(map) => ProjectionSource(map.into_values().enumerate().map(|(index, item)| (RowId(index as u64), item)).collect()),
            other => ProjectionSource(vec![(RowId(0), other)]),
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl QuerySource for ProjectionSource {
    fn scan(&self) -> Box<dyn Iterator<Item = (RowId, Value)> + '_> {
        Box::new(self.0.iter().cloned())
    }
}

/// @emoji 🌉️ The bridge the module doc's `db_projection`-integration note describes: decodes
/// `state_bytes` (whatever a caller already retrieved from `db_projection::ProjectionEngine::state_at`
/// for `Consistency::Historical`/`Canonical`, or `::preview_augmented` for
/// `Consistency::Speculative`/`PreviewAugmented` — both return exactly this shape, plain
/// `ProjectionState`-encoded bytes with any version-prefix framing already stripped by
/// `ProjectionEngine` itself) into a `ProjectionSource` this crate's `execute`/`LiveQuery` can run
/// over. See the module doc for why this takes raw bytes rather than a `&ProjectionEngine` reference.
pub fn projection_query_source(state_bytes: &[u8]) -> Result<ProjectionSource, DbError> {
    Ok(ProjectionSource::from_value(Value::decode(state_bytes)?))
}
//#endregion 🔖️ProjectionBridge

//#region 🔖️Planner
/// @emoji 🗺️ The chosen execution strategy for a `Query`. `FullTextPushdown`'s `term` is always
/// re-verified against the full `Predicate` tree during evaluation (see `execute`) — pushdown is
/// purely a candidate-narrowing optimization, never a correctness shortcut, so its result set is
/// guaranteed identical to `FullScan`'s for the same query (exercised by
/// `tests::planner::pushdown_matches_full_scan_when_the_index_is_exhaustive`).
#[derive(Clone, Debug, PartialEq)]
pub enum QueryPlan {
    FullScan,
    FullTextPushdown { term: String },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QueryPlanKind {
    FullScan,
    FullTextPushdown,
}

impl QueryPlan {
    pub fn kind(&self) -> QueryPlanKind {
        match self {
            QueryPlan::FullScan => QueryPlanKind::FullScan,
            QueryPlan::FullTextPushdown { .. } => QueryPlanKind::FullTextPushdown,
        }
    }
}

/// @emoji 🔍️ Finds the first `Predicate::FullText` term reachable through a top-level conjunction
/// (a bare `FullText` predicate, or one `And` branch) — the only shape this planner currently
/// recognizes as pushdown-eligible. `Or`/`Not` wrapping a `FullText` predicate is deliberately left
/// as `FullScan` (pushdown under `Or`/`Not` would need to reason about set complement/union, which
/// this planner does not attempt) — a real limitation, not a bug: `FullScan` is always correct, only
/// potentially slower.
fn extract_full_text_term(predicate: &Predicate) -> Option<String> {
    match predicate {
        Predicate::FullText(_, term) => Some(term.clone()),
        Predicate::And(predicates) => predicates.iter().find_map(extract_full_text_term),
        _ => None,
    }
}

/// @emoji 🧠️ Chooses a `QueryPlan` for `query`. Currently a single heuristic (full-text pushdown);
/// the extension point for future pushdown kinds (indexed equality, etc.) is this one function.
pub fn plan(query: &Query) -> QueryPlan {
    match query.filter.as_ref().and_then(extract_full_text_term) {
        Some(term) => QueryPlan::FullTextPushdown { term },
        None => QueryPlan::FullScan,
    }
}
//#endregion 🔖️Planner

//#region 🔖️Execute
/// @emoji 📊️ What `execute` observed while producing a `QueryResult` — the contract's "diagnostics"
/// bullet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QueryDiagnostics {
    pub plan: QueryPlanKind,
    pub rows_scanned: u64,
    pub rows_matched: u64,
    pub rows_returned: u64,
}

/// @emoji 📦️ A fully materialized query result: the projected, sorted, paginated rows, plus
/// `QueryDiagnostics`. Convert to a `QueryStream` via `into_stream` for incremental consumption.
#[derive(Clone, Debug, PartialEq)]
pub struct QueryResult {
    pub rows: Vec<(RowId, Value)>,
    pub diagnostics: QueryDiagnostics,
}

/// @emoji ▶️ Plans and evaluates `query` against `source`. `fulltext` is only consulted if the
/// planner chose `QueryPlan::FullTextPushdown`; passing `None` for a query the planner would push
/// down surfaces `DbError::InvalidArgument` rather than silently falling back to a full scan (a
/// caller that owns a `FullTextLookup` should always pass it — silent fallback would hide a
/// wiring bug as a performance regression instead of a compile/runtime-visible one).
pub fn execute(query: &Query, source: &dyn QuerySource, fulltext: Option<&dyn FullTextLookup>, limits: &QueryLimits) -> Result<QueryResult, DbError> {
    let chosen_plan = plan(query);
    let mut scanned: u64 = 0;
    let mut matched: Vec<(RowId, Value)> = Vec::new();

    match &chosen_plan {
        QueryPlan::FullScan => {
            for (id, value) in source.scan() {
                scanned += 1;
                check_len(scanned, limits.max_scan_rows, "db_query::rows_scanned")?;
                if query.filter.as_ref().is_none_or(|predicate| eval_predicate(predicate, &value)) {
                    matched.push((id, value));
                }
            }
        }
        QueryPlan::FullTextPushdown { term } => {
            let lookup = fulltext.ok_or_else(|| DbError::InvalidArgument("query planned a full-text pushdown but no FullTextLookup was supplied".to_string()))?;
            for id in lookup.search(term)? {
                let Some(value) = source.get(id) else { continue };
                scanned += 1;
                check_len(scanned, limits.max_scan_rows, "db_query::rows_scanned")?;
                if query.filter.as_ref().is_none_or(|predicate| eval_predicate(predicate, &value)) {
                    matched.push((id, value));
                }
            }
        }
    }

    if !query.sort.is_empty() {
        matched.sort_by(|(_, a), (_, b)| compare_rows(a, b, &query.sort));
    }
    let rows_matched = matched.len() as u64;

    let offset = query.offset.unwrap_or(0) as usize;
    let mut rows: Vec<(RowId, Value)> = matched.into_iter().skip(offset).collect();
    if let Some(limit) = query.limit {
        rows.truncate(usize::try_from(limit).unwrap_or(usize::MAX));
    }
    check_len(rows.len() as u64, limits.max_result_rows, "db_query::result_rows")?;

    let projected: Vec<(RowId, Value)> = rows.into_iter().map(|(id, value)| (id, query.select.project(&value))).collect();
    check_len(estimate_result_bytes(&projected), limits.max_result_bytes, "db_query::result_bytes")?;

    let rows_returned = projected.len() as u64;
    Ok(QueryResult { rows: projected, diagnostics: QueryDiagnostics { plan: chosen_plan.kind(), rows_scanned: scanned, rows_matched, rows_returned } })
}
//#endregion 🔖️Execute

//#region 🔖️Stream
/// @emoji 🌊️ A `QueryResult`'s rows as an `Iterator`, for callers (`DocumentHandle::query`'s
/// contract-frozen return type) that want to consume incrementally rather than hold the whole
/// `Vec`. Backed by an already-materialized `Vec::IntoIter` — see `db_state::PMap::iter`'s doc for
/// this crate family's established "eagerly materialize, simple to reason about" precedent; a
/// truly lazy pull-based evaluator is a straightforward future optimization.
pub struct QueryStream {
    rows: std::vec::IntoIter<(RowId, Value)>,
    pub diagnostics: QueryDiagnostics,
}

impl QueryResult {
    pub fn into_stream(self) -> QueryStream {
        QueryStream { rows: self.rows.into_iter(), diagnostics: self.diagnostics }
    }
}

impl Iterator for QueryStream {
    type Item = (RowId, Value);

    fn next(&mut self) -> Option<Self::Item> {
        self.rows.next()
    }
}
//#endregion 🔖️Stream

//#region 🔖️LiveQuery
/// @emoji 📡️ A `Query` plus the `Consistency` it should be (re-)evaluated under — what a caller
/// hands to `DocumentHandle::subscribe`. This crate only owns the diffing law (below); actor-level
/// registration/notification wiring belongs to `db_document`.
#[derive(Clone, Debug, PartialEq)]
pub struct LiveQuerySpec {
    pub query: Query,
    pub consistency: Consistency,
}

/// @emoji 🔀️ The change between two successive evaluations of a `LiveQuery`'s `Query`: rows newly
/// present, rows no longer present, and rows present in both but with a changed `Value`.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct QueryDiff {
    pub added: Vec<(RowId, Value)>,
    pub removed: Vec<RowId>,
    pub updated: Vec<(RowId, Value)>,
}

/// @emoji 📺️ Tracks one live query's last-seen result set so `refresh` can emit a `QueryDiff`
/// instead of the caller having to re-diff two full `QueryResult`s itself. The law this crate's
/// tests hold it to: applying a `QueryDiff` to the pre-refresh snapshot (add `added`, drop
/// `removed`, overwrite `updated`) always reconstructs exactly the post-refresh snapshot.
pub struct LiveQuery {
    spec: LiveQuerySpec,
    snapshot: BTreeMap<RowId, Value>,
}

impl LiveQuery {
    pub fn new(spec: LiveQuerySpec) -> LiveQuery {
        LiveQuery { spec, snapshot: BTreeMap::new() }
    }

    pub fn spec(&self) -> &LiveQuerySpec {
        &self.spec
    }

    pub fn snapshot(&self) -> &BTreeMap<RowId, Value> {
        &self.snapshot
    }

    /// @emoji 🔁️ Re-executes `self.spec.query` against `source` and diffs the result against the
    /// previous snapshot, updating the snapshot in place. `source`/`fulltext` are expected to
    /// already be materialized at whatever frontier `resolve_consistency(&self.spec.consistency,
    /// ..)` resolved to — resolving that frontier and building the matching `QuerySource` is the
    /// caller's job (it owns the actual document state), not this crate's.
    pub fn refresh(&mut self, source: &dyn QuerySource, fulltext: Option<&dyn FullTextLookup>, limits: &QueryLimits) -> Result<QueryDiff, DbError> {
        let result = execute(&self.spec.query, source, fulltext, limits)?;
        let new_snapshot: BTreeMap<RowId, Value> = result.rows.into_iter().collect();

        let mut diff = QueryDiff::default();
        for (id, value) in &new_snapshot {
            match self.snapshot.get(id) {
                None => diff.added.push((*id, value.clone())),
                Some(old_value) if old_value != value => diff.updated.push((*id, value.clone())),
                _ => {}
            }
        }
        for id in self.snapshot.keys() {
            if !new_snapshot.contains_key(id) {
                diff.removed.push(*id);
            }
        }

        self.snapshot = new_snapshot;
        Ok(diff)
    }
}
//#endregion 🔖️LiveQuery

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_row(name: &str, age: i64, tags: Vec<&str>) -> Value {
        let mut map = BTreeMap::new();
        map.insert("name".to_string(), Value::Text(name.to_string()));
        map.insert("age".to_string(), Value::Int(age));
        map.insert("tags".to_string(), Value::List(tags.into_iter().map(Value::from).collect()));
        Value::Map(map)
    }

    fn sample_source() -> PVec<Value> {
        let mut vec = PVec::new();
        vec = vec.push_back(sample_row("alice", 30, vec!["admin", "eng"]));
        vec = vec.push_back(sample_row("bob", 25, vec!["eng"]));
        vec = vec.push_back(sample_row("cara", 40, vec!["admin"]));
        vec
    }

    //#region 🔖️Value
    mod value {
        use super::*;

        #[test]
        fn path_get_walks_nested_map_and_list() {
            let row = sample_row("alice", 30, vec!["admin", "eng"]);
            assert_eq!(Path::field("name").get(&row), Some(&Value::Text("alice".to_string())));
            assert_eq!(Path::parse("tags.0").get(&row), Some(&Value::Text("admin".to_string())));
            assert_eq!(Path::parse("missing").get(&row), None);
            assert_eq!(Path::empty().get(&row), Some(&row));
        }

        #[test]
        fn path_get_rejects_type_mismatch() {
            let row = sample_row("alice", 30, vec!["admin"]);
            assert_eq!(Path::parse("name.0").get(&row), None);
            assert_eq!(Path::parse("age.field").get(&row), None);
        }

        #[test]
        fn compare_values_orders_numerics_across_int_and_float() {
            assert_eq!(compare_values(&Value::Int(1), &Value::Float(1.5)), Ordering::Less);
            assert_eq!(compare_values(&Value::Float(2.0), &Value::Int(2)), Ordering::Equal);
        }

        #[test]
        fn compare_values_falls_back_to_rank_across_variants() {
            assert_eq!(compare_values(&Value::Null, &Value::Bool(false)), Ordering::Less);
            assert_eq!(compare_values(&Value::Text("z".to_string()), &Value::Int(0)), Ordering::Greater);
        }

        #[test]
        fn path_display_round_trips_through_parse() {
            let path = Path::parse("a.b.3");
            assert_eq!(path.to_string(), "a.b.3");
        }
    }
    //#endregion 🔖️Value

    //#region 🔖️Query
    mod query {
        use super::*;

        #[test]
        fn eq_predicate_matches_and_rejects() {
            let row = sample_row("alice", 30, vec!["admin"]);
            assert!(eval_predicate(&Predicate::Eq(Path::field("age"), Value::Int(30)), &row));
            assert!(!eval_predicate(&Predicate::Eq(Path::field("age"), Value::Int(31)), &row));
        }

        #[test]
        fn ne_treats_missing_path_as_failing() {
            let row = sample_row("alice", 30, vec!["admin"]);
            assert!(!eval_predicate(&Predicate::Ne(Path::field("missing"), Value::Int(1)), &row));
        }

        #[test]
        fn and_or_not_compose() {
            let row = sample_row("alice", 30, vec!["admin"]);
            let is_admin = Predicate::Contains(Path::field("tags"), Value::from("admin"));
            let is_old = Predicate::Gte(Path::field("age"), Value::Int(40));
            assert!(eval_predicate(&Predicate::And(vec![is_admin.clone()]), &row));
            assert!(eval_predicate(&Predicate::Or(vec![is_admin.clone(), is_old.clone()]), &row));
            assert!(!eval_predicate(&Predicate::And(vec![is_admin, is_old.clone()]), &row));
            assert!(eval_predicate(&Predicate::Not(Box::new(is_old)), &row));
        }

        #[test]
        fn full_text_matches_case_insensitively_over_whole_document() {
            let row = sample_row("Alice", 30, vec!["admin"]);
            assert!(eval_predicate(&Predicate::FullText(Path::empty(), "ALICE".to_string()), &row));
            assert!(!eval_predicate(&Predicate::FullText(Path::empty(), "dave".to_string()), &row));
        }

        #[test]
        fn select_paths_projects_a_map_keyed_by_dotted_path() {
            let row = sample_row("alice", 30, vec!["admin"]);
            let projected = Select::Paths(vec![Path::field("name")]).project(&row);
            match projected {
                Value::Map(map) => assert_eq!(map.get("name"), Some(&Value::Text("alice".to_string()))),
                other => panic!("expected a map, got {other:?}"),
            }
        }
    }
    //#endregion 🔖️Query

    //#region 🔖️ProjectionBridge
    mod projection_bridge {
        use super::*;

        fn nested_sample() -> Value {
            let mut inner = BTreeMap::new();
            inner.insert("nickname".to_string(), Value::Text("ally".to_string()));
            inner.insert("verified".to_string(), Value::Bool(true));
            let mut row = BTreeMap::new();
            row.insert("name".to_string(), Value::Text("alice".to_string()));
            row.insert("age".to_string(), Value::Int(30));
            row.insert("score".to_string(), Value::Float(2.5));
            row.insert("blob".to_string(), Value::Bytes(vec![9, 8, 7]));
            row.insert("tags".to_string(), Value::List(vec![Value::from("admin"), Value::Null]));
            row.insert("profile".to_string(), Value::Map(inner));
            Value::Map(row)
        }

        #[test]
        fn value_projection_state_round_trips_every_variant_including_nesting() {
            let value = nested_sample();
            let decoded = Value::decode(&ProjectionState::encode(&value)).expect("round trip decodes");
            assert_eq!(decoded, value);
        }

        #[test]
        fn value_projection_state_round_trips_null_and_empty_containers() {
            for value in [Value::Null, Value::List(Vec::new()), Value::Map(BTreeMap::new())] {
                assert_eq!(Value::decode(&ProjectionState::encode(&value)).unwrap(), value);
            }
        }

        #[test]
        fn decode_rejects_truncated_bytes_and_unknown_tag_without_panicking() {
            assert!(matches!(Value::decode(&[4u8, 5, 0, 0, 0]), Err(DbError::Corrupt(_))), "declared text len 5 but no bytes follow");
            assert!(matches!(Value::decode(&[200u8]), Err(DbError::Corrupt(_))), "tag 200 is not a valid Value variant");
            assert!(matches!(Value::decode(&[]), Err(DbError::Corrupt(_))));
        }

        #[test]
        fn decode_rejects_trailing_bytes_after_a_complete_value() {
            let mut bytes = ProjectionState::encode(&Value::Bool(true));
            bytes.push(0xFF);
            assert!(matches!(Value::decode(&bytes), Err(DbError::Corrupt(_))));
        }

        #[test]
        fn decode_value_rejects_an_over_large_declared_element_count_before_allocating() {
            let mut list_bytes = vec![6u8];
            list_bytes.extend_from_slice(&((MAX_PROJECTION_VALUE_ELEMENTS + 1) as u32).to_le_bytes());
            assert!(matches!(Value::decode(&list_bytes), Err(DbError::LimitExceeded(_))));

            let mut map_bytes = vec![7u8];
            map_bytes.extend_from_slice(&((MAX_PROJECTION_VALUE_ELEMENTS + 1) as u32).to_le_bytes());
            assert!(matches!(Value::decode(&map_bytes), Err(DbError::LimitExceeded(_))));
        }

        #[test]
        fn projection_source_shapes_list_map_and_scalar_values_into_rows() {
            let list_source = ProjectionSource::from_value(Value::List(vec![Value::from(1i64), Value::from(2i64)]));
            assert_eq!(list_source.len(), 2);
            assert_eq!(list_source.scan().collect::<Vec<_>>(), vec![(RowId(0), Value::from(1i64)), (RowId(1), Value::from(2i64))]);

            let mut map = BTreeMap::new();
            map.insert("a".to_string(), Value::from("first"));
            map.insert("b".to_string(), Value::from("second"));
            let map_source = ProjectionSource::from_value(Value::Map(map));
            assert_eq!(map_source.scan().collect::<Vec<_>>(), vec![(RowId(0), Value::from("first")), (RowId(1), Value::from("second"))]);

            let scalar_source = ProjectionSource::from_value(Value::Int(42));
            assert!(!scalar_source.is_empty());
            assert_eq!(scalar_source.scan().collect::<Vec<_>>(), vec![(RowId(0), Value::Int(42))]);
        }

        /// @emoji ⚖️ The end-to-end law this bridge exists for: bytes a caller retrieved from
        /// `db_projection::ProjectionEngine::state_at`/`preview_augmented` (simulated here by
        /// `ProjectionState::encode` on a hand-built row set, since this crate cannot construct a
        /// real `ProjectionEngine` without a `protocol::MutationEnvelope` — see the module doc)
        /// decode through `projection_query_source` into a `QuerySource` this crate's ordinary
        /// `execute` runs over identically to any other source.
        #[test]
        fn projection_query_source_decodes_bytes_into_a_queryable_source() {
            let rows = Value::List(vec![sample_row("alice", 30, vec!["admin", "eng"]), sample_row("bob", 25, vec!["eng"])]);
            let state_bytes = ProjectionState::encode(&rows);

            let source = projection_query_source(&state_bytes).expect("decodes");
            let query = Query::new().filter(Predicate::Gte(Path::field("age"), Value::Int(30)));
            let result = execute(&query, &source, None, &QueryLimits::default()).expect("query succeeds");
            assert_eq!(result.rows.len(), 1);
            assert_eq!(Path::field("name").get(&result.rows[0].1), Some(&Value::Text("alice".to_string())));
        }

        #[test]
        fn projection_query_source_surfaces_corrupt_bytes_as_an_error_not_a_panic() {
            assert!(matches!(projection_query_source(&[200u8]), Err(DbError::Corrupt(_))));
        }
    }
    //#endregion 🔖️ProjectionBridge

    //#region 🔖️Execute
    mod execute_tests {
        use super::*;

        #[test]
        fn full_scan_filters_sorts_and_paginates() {
            let source = sample_source();
            let query = Query::new().filter(Predicate::Gte(Path::field("age"), Value::Int(25))).sort(vec![SortKey::descending(Path::field("age"))]).limit(2);
            let result = execute(&query, &source, None, &QueryLimits::default()).expect("query succeeds");
            assert_eq!(result.diagnostics.plan, QueryPlanKind::FullScan);
            assert_eq!(result.diagnostics.rows_matched, 3);
            assert_eq!(result.diagnostics.rows_returned, 2);
            let names: Vec<String> = result
                .rows
                .iter()
                .map(|(_, value)| match Path::field("name").get(value) {
                    Some(Value::Text(name)) => name.clone(),
                    _ => panic!("expected a name"),
                })
                .collect();
            assert_eq!(names, vec!["cara".to_string(), "alice".to_string()]);
        }

        #[test]
        fn offset_skips_matched_rows_before_limit_applies() {
            let source = sample_source();
            let query = Query::new().sort(vec![SortKey::ascending(Path::field("age"))]).offset(1).limit(1);
            let result = execute(&query, &source, None, &QueryLimits::default()).expect("query succeeds");
            assert_eq!(result.rows.len(), 1);
            assert_eq!(Path::field("name").get(&result.rows[0].1), Some(&Value::Text("alice".to_string())));
        }

        #[test]
        fn max_result_rows_limit_is_enforced() {
            let source = sample_source();
            let limits = QueryLimits { max_result_rows: 1, ..QueryLimits::default() };
            let error = execute(&Query::new(), &source, None, &limits).unwrap_err();
            assert!(matches!(error, DbError::LimitExceeded(_)));
        }

        #[test]
        fn into_stream_yields_the_same_rows_as_the_result() {
            let source = sample_source();
            let result = execute(&Query::new(), &source, None, &QueryLimits::default()).expect("query succeeds");
            let expected_len = result.rows.len();
            let stream = result.into_stream();
            assert_eq!(stream.count(), expected_len);
        }

        /// @emoji 🧪️ A hand-rolled `FullTextLookup` double — exercises pushdown without needing a
        /// real `db_storage::IndexStorage` (not a dependency of this crate; see module doc).
        pub(super) struct FakeFullText(pub std::collections::HashMap<String, Vec<RowId>>);
        impl FullTextLookup for FakeFullText {
            fn search(&self, term: &str) -> Result<Vec<RowId>, DbError> {
                Ok(self.0.get(term).cloned().unwrap_or_default())
            }
        }

        #[test]
        fn full_text_pushdown_without_a_lookup_is_an_error() {
            let source = sample_source();
            let query = Query::new().filter(Predicate::FullText(Path::empty(), "alice".to_string()));
            let error = execute(&query, &source, None, &QueryLimits::default()).unwrap_err();
            assert!(matches!(error, DbError::InvalidArgument(_)));
        }
    }
    //#endregion 🔖️Execute

    //#region 🔖️Planner
    mod planner {
        use super::execute_tests::FakeFullText;
        use super::*;

        #[test]
        fn plan_recognizes_bare_and_conjoined_full_text_predicates() {
            let bare = Query::new().filter(Predicate::FullText(Path::empty(), "x".to_string()));
            assert_eq!(plan(&bare), QueryPlan::FullTextPushdown { term: "x".to_string() });

            let conjoined = Query::new().filter(Predicate::And(vec![Predicate::Eq(Path::field("age"), Value::Int(1)), Predicate::FullText(Path::empty(), "y".to_string())]));
            assert_eq!(plan(&conjoined), QueryPlan::FullTextPushdown { term: "y".to_string() });

            let disjoined = Query::new().filter(Predicate::Or(vec![Predicate::FullText(Path::empty(), "z".to_string())]));
            assert_eq!(plan(&disjoined), QueryPlan::FullScan);
        }

        /// @emoji ⚖️ The correctness law `QueryPlan::FullTextPushdown`'s doc promises: a pushdown
        /// plan and a full scan must agree exactly, for the same query, modulo which rows the
        /// (possibly stale/approximate) full-text index happens to surface as candidates.
        #[test]
        fn pushdown_matches_full_scan_when_the_index_is_exhaustive() {
            let source = sample_source();
            let query = Query::new().filter(Predicate::FullText(Path::empty(), "admin".to_string()));

            let full_scan_result = execute(&query, &source, None, &QueryLimits::default());
            assert!(matches!(full_scan_result, Err(DbError::InvalidArgument(_))));

            let mut postings = std::collections::HashMap::new();
            postings.insert("admin".to_string(), vec![RowId(0), RowId(1), RowId(2)]);
            let lookup = FakeFullText(postings);

            let pushdown = execute(&query, &source, Some(&lookup), &QueryLimits::default()).expect("pushdown succeeds");
            assert_eq!(pushdown.diagnostics.plan, QueryPlanKind::FullTextPushdown);
            assert_eq!(pushdown.rows.len(), 2);
            let names: std::collections::HashSet<String> = pushdown
                .rows
                .iter()
                .map(|(_, value)| match Path::field("name").get(value) {
                    Some(Value::Text(name)) => name.clone(),
                    _ => panic!("expected a name"),
                })
                .collect();
            assert_eq!(names, std::collections::HashSet::from(["alice".to_string(), "cara".to_string()]));
        }
    }
    //#endregion 🔖️Planner

    //#region 🔖️Consistency
    mod consistency {
        use super::*;

        struct FakeResolver {
            current: Frontier,
            commits: BTreeMap<String, Frontier>,
        }

        impl ConsistencyResolver for FakeResolver {
            fn current_frontier(&self) -> Result<Frontier, DbError> {
                Ok(self.current.clone())
            }
            fn frontier_for_commit(&self, commit_id: &str) -> Result<Frontier, DbError> {
                self.commits.get(commit_id).cloned().ok_or_else(|| DbError::NotFound(commit_id.to_string()))
            }
        }

        fn frontier_at(seq: u64) -> Frontier {
            Frontier { document: DocumentId::from("doc-1"), head_seq: seq, commit_seq: seq, chain_hash: [0u8; 32], epoch: 0 }
        }

        #[test]
        fn canonical_resolves_to_current_frontier() {
            let resolver = FakeResolver { current: frontier_at(5), commits: BTreeMap::new() };
            let resolved = resolve_consistency(&Consistency::Canonical, &resolver).expect("resolves");
            assert_eq!(resolved.frontier, frontier_at(5));
            assert!(!resolved.historical);
            assert_eq!(resolved.preview_id, None);
        }

        #[test]
        fn at_least_succeeds_when_dominated_and_fails_otherwise() {
            let resolver = FakeResolver { current: frontier_at(5), commits: BTreeMap::new() };
            assert!(resolve_consistency(&Consistency::AtLeast(frontier_at(3)), &resolver).is_ok());
            let error = resolve_consistency(&Consistency::AtLeast(frontier_at(10)), &resolver).unwrap_err();
            assert!(matches!(error, DbError::Unavailable(_)));
        }

        #[test]
        fn exact_requires_a_bytewise_match() {
            let resolver = FakeResolver { current: frontier_at(5), commits: BTreeMap::new() };
            assert!(resolve_consistency(&Consistency::Exact(frontier_at(5)), &resolver).is_ok());
            assert!(resolve_consistency(&Consistency::Exact(frontier_at(6)), &resolver).is_err());
        }

        #[test]
        fn historical_resolves_via_commit_lookup() {
            let mut commits = BTreeMap::new();
            commits.insert("ck-abc".to_string(), frontier_at(2));
            let resolver = FakeResolver { current: frontier_at(5), commits };
            let resolved = resolve_consistency(&Consistency::Historical("ck-abc".to_string()), &resolver).expect("resolves");
            assert_eq!(resolved.frontier, frontier_at(2));
            assert!(resolved.historical);

            let error = resolve_consistency(&Consistency::Historical("ck-missing".to_string()), &resolver).unwrap_err();
            assert!(matches!(error, DbError::NotFound(_)));
        }

        #[test]
        fn speculative_and_preview_augmented_carry_the_preview_id() {
            let resolver = FakeResolver { current: frontier_at(5), commits: BTreeMap::new() };
            let speculative = resolve_consistency(&Consistency::Speculative("pv-1".to_string()), &resolver).expect("resolves");
            assert_eq!(speculative.preview_id, Some("pv-1".to_string()));
            let augmented = resolve_consistency(&Consistency::PreviewAugmented("pv-2".to_string()), &resolver).expect("resolves");
            assert_eq!(augmented.preview_id, Some("pv-2".to_string()));
        }
    }
    //#endregion 🔖️Consistency

    //#region 🔖️LiveQuery
    mod live_query {
        use super::*;

        fn source_with(rows: Vec<Value>) -> PVec<Value> {
            let mut vec = PVec::new();
            for row in rows {
                vec = vec.push_back(row);
            }
            vec
        }

        /// @emoji 🆔️ `PVec`'s `RowId` is positional (index-based — see its `QuerySource` impl's
        /// doc), so a diff's `added`/`removed`/`updated` classification is keyed by position, not
        /// by any notion of row identity: replacing `bob` with `cara` at the same index is an
        /// `updated` row, not a `removed` + `added` pair. This test exercises all three by keeping
        /// the vector's length changes and value changes at distinct positions.
        #[test]
        fn refresh_reports_added_removed_and_updated_rows() {
            let spec = LiveQuerySpec { query: Query::new(), consistency: Consistency::Canonical };
            let mut live = LiveQuery::new(spec);

            let first = source_with(vec![sample_row("alice", 30, vec!["admin"]), sample_row("bob", 25, vec!["eng"])]);
            let diff = live.refresh(&first, None, &QueryLimits::default()).expect("refresh succeeds");
            assert_eq!(diff.added.len(), 2);
            assert!(diff.removed.is_empty());
            assert!(diff.updated.is_empty());

            let second = source_with(vec![sample_row("alice", 31, vec!["admin"]), sample_row("bob", 25, vec!["eng"]), sample_row("cara", 40, vec!["admin"])]);
            let diff = live.refresh(&second, None, &QueryLimits::default()).expect("refresh succeeds");
            assert_eq!(diff.added.len(), 1);
            assert!(diff.removed.is_empty());
            assert_eq!(diff.updated.len(), 1);

            let third = source_with(vec![sample_row("alice", 31, vec!["admin"])]);
            let diff = live.refresh(&third, None, &QueryLimits::default()).expect("refresh succeeds");
            assert!(diff.added.is_empty());
            assert_eq!(diff.removed.len(), 2);
            assert!(diff.updated.is_empty());
        }

        /// @emoji ⚖️ The round-trip law `LiveQuery`'s doc promises: old snapshot ⊕ diff == new
        /// snapshot, exactly.
        #[test]
        fn diff_applied_to_old_snapshot_reconstructs_new_snapshot() {
            let spec = LiveQuerySpec { query: Query::new(), consistency: Consistency::Canonical };
            let mut live = LiveQuery::new(spec);

            let first = source_with(vec![sample_row("alice", 30, vec!["admin"]), sample_row("bob", 25, vec!["eng"])]);
            live.refresh(&first, None, &QueryLimits::default()).expect("refresh succeeds");
            let mut reconstructed = live.snapshot().clone();

            let second = source_with(vec![sample_row("alice", 31, vec!["admin"]), sample_row("cara", 40, vec!["admin"])]);
            let diff = live.refresh(&second, None, &QueryLimits::default()).expect("refresh succeeds");

            for id in &diff.removed {
                reconstructed.remove(id);
            }
            for (id, value) in diff.added.iter().chain(diff.updated.iter()) {
                reconstructed.insert(*id, value.clone());
            }

            assert_eq!(&reconstructed, live.snapshot());
        }
    }
    //#endregion 🔖️LiveQuery

    //#region 🔖️Limits
    mod limits {
        use super::*;

        #[test]
        fn default_result_bytes_matches_db_core_query_budget() {
            assert_eq!(QueryLimits::default().max_result_bytes, DbLimits::default().max_query_bytes);
        }

        #[test]
        fn max_scan_rows_is_enforced_even_when_nothing_matches() {
            let source = sample_source();
            let limits = QueryLimits { max_scan_rows: 1, ..QueryLimits::default() };
            let query = Query::new().filter(Predicate::Eq(Path::field("age"), Value::Int(999)));
            let error = execute(&query, &source, None, &limits).unwrap_err();
            assert!(matches!(error, DbError::LimitExceeded(_)));
        }
    }
    //#endregion 🔖️Limits
}
//#endregion 🧪️Tests
