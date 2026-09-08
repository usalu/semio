//! 🗄️ `db_query` — the `db` family's query engine: consistency-mode resolution
//! (`Canonical`/`AtLeast`/`Exact`/`Historical`/`Speculative`/`PreviewAugmented`), a dynamic `Value`
//! tree with typed convenience conversions, a `Predicate`/`Select`/`Query` IR, a minimal
//! cost-free planner that recognizes full-text-pushdown opportunities, a streaming `QueryStream`,
//! and `LiveQuery` incremental diffing. Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
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
//! dependency list grants it `db_projection` but not `protocol` (only `db_artifact`, which already
//! owns envelope interpretation, has both) — so the layer that calls `state_at`/`preview_augmented`
//! hands this crate only the resulting bytes, never the engine or the envelope.

use crate::db_durability::Frontier;
use crate::db_ids::{check_len, DbError};
use crate::*;
use db_index::{CommitIndex, FrontierIndex, FullTextIndex};
use std::cmp::Ordering;
use std::collections::BTreeMap;

//#region 🔖️Value
pub struct QueryCursorControl {
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    deadline: std::time::Instant,
    fuel: usize,
}

impl QueryCursorControl {
    pub fn new(cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>, deadline: std::time::Instant, fuel: usize) -> Result<Self, DbError> {
        if fuel == 0 {
            return Err(DbError::LimitExceeded("query cursor fuel"));
        }
        Ok(Self { cancelled, deadline, fuel })
    }

    pub fn replenish(&mut self, deadline: std::time::Instant, fuel: usize) -> Result<(), DbError> {
        if fuel == 0 {
            return Err(DbError::LimitExceeded("query cursor fuel"));
        }
        self.deadline = deadline;
        self.fuel = fuel;
        Ok(())
    }

    pub fn grant(&mut self) -> Result<(), DbError> {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            return Err(DbError::Unavailable("query cursor cancelled".to_string()));
        }
        if std::time::Instant::now() >= self.deadline {
            return Err(DbError::Unavailable("query cursor deadline reached".to_string()));
        }
        self.fuel = self.fuel.checked_sub(1).ok_or(DbError::LimitExceeded("query cursor fuel"))?;
        Ok(())
    }
}

pub struct QueryBytes {
    pages: db_storage::DbIoPages,
}

impl QueryBytes {
    pub fn from_pages(pages: db_storage::DbIoPages) -> Result<Self, (DbError, db_storage::DbIoPages)> {
        if pages.operation() == 0 {
            return Err((DbError::InvalidArgument("query bytes require a nonzero operation".to_string()), pages));
        }
        Ok(Self { pages })
    }

    pub async fn copy_from_pages(source: &db_storage::DbIoPages, control: &mut QueryCursorControl) -> Result<Self, DbError> {
        let mut writer = db_storage::DbIoPageWriter::try_reserve(source.len().div_ceil(db_storage::DB_IO_PAGE_BYTES)).map_err(db_storage::DbIoPageWriterRejected::into_error)?;
        for fragment in source.fragments() {
            control.grant()?;
            if writer.write_fragment(fragment)? != fragment.len() {
                return Err(DbError::Internal("query byte copy made partial progress".to_string()));
            }
            semio_framework_async::yield_once().await;
        }
        Ok(Self { pages: writer.seal_retained().await.map_err(db_storage::DbIoPageWriterRejected::into_error)? })
    }

    pub fn len(&self) -> usize {
        self.pages.len()
    }

    pub fn fragments(&self) -> db_storage::DbIoPageReader<'_> {
        self.pages.fragments()
    }

    pub fn close_step(&mut self) -> Result<Option<usize>, DbError> {
        self.pages.close_step()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.pages.terminal_is_empty()
    }
}

impl std::fmt::Debug for QueryBytes {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("QueryBytes").field("operation", &self.pages.operation()).field("len", &self.len()).finish()
    }
}

impl PartialEq for QueryBytes {
    fn eq(&self, other: &Self) -> bool {
        self.pages == other.pages
    }
}

fn compare_query_bytes(left: &QueryBytes, right: &QueryBytes) -> Ordering {
    let mut left_fragments = left.fragments();
    let mut right_fragments = right.fragments();
    let (mut left_fragment, mut right_fragment) = (left_fragments.next().unwrap_or_default(), right_fragments.next().unwrap_or_default());
    let (mut left_offset, mut right_offset) = (0, 0);
    loop {
        let compared = (left_fragment.len() - left_offset).min(right_fragment.len() - right_offset);
        let order = left_fragment[left_offset..left_offset + compared].cmp(&right_fragment[right_offset..right_offset + compared]);
        if order != Ordering::Equal {
            return order;
        }
        left_offset += compared;
        right_offset += compared;
        if left_offset == left_fragment.len() {
            match left_fragments.next() {
                Some(next) => {
                    left_fragment = next;
                    left_offset = 0;
                }
                None => return if right_offset == right_fragment.len() && right_fragments.next().is_none() { Ordering::Equal } else { Ordering::Less },
            }
        }
        if right_offset == right_fragment.len() {
            match right_fragments.next() {
                Some(next) => {
                    right_fragment = next;
                    right_offset = 0;
                }
                None => return Ordering::Greater,
            }
        }
    }
}

/// @emoji 🧬️ The dynamic value model every `Query` evaluates against. Deliberately this crate's
/// own type rather than `pack_value` (forbidden by the contract's hard dependency rules) or a
/// `db_state` structure directly (those are the *storage* representation; a document's queryable
/// shape is resolved into this tree by whichever layer above `db_query` owns the schema — typically
/// `db_artifact`).
/// 🧪️ `Serialize`/`Deserialize` (added for `LiveQuery`'s `QueryResultField: InferredField<..>`
/// routing — see `🔖️LiveQuery` below): `infer_field`'s cache stores `F::Value` bytes via
/// `serde_json`, so any `InferredField::Value` must round-trip through serde regardless of whether
/// caching is actually enabled at runtime (a static bound on the trait, not a runtime condition).
#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    Bytes(QueryBytes),
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
        (Value::Bytes(x), Value::Bytes(y)) => compare_query_bytes(x, y),
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
// 🚫️async: E1 pure, self-recursive accessor consumed by a sync Iterator::map — see R9
fn stringify_value(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Text(s) => s.clone(),
        Value::Bytes(bytes) => {
            let mut text = String::new();
            for fragment in bytes.fragments() {
                text.push_str(&String::from_utf8_lossy(fragment));
            }
            text
        }
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
/// @emoji 🧭️ The six read-consistency modes `ArtifactHandle::query` accepts (frozen in the db
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
    async fn current_frontier(&self) -> Result<Frontier, DbError>;
    async fn frontier_for_commit(&self, commit_id: &str) -> Result<Frontier, DbError>;
}

/// @emoji 🧮️ Resolves `consistency` against `resolver` into a concrete `ResolvedConsistency`.
/// `AtLeast`/`Exact` are checked against `Frontier::dominates`/equality respectively — both are the
/// contract's own definition of those two modes, not this crate's invention.
pub async fn resolve_consistency(consistency: &Consistency, resolver: &impl ConsistencyResolver) -> Result<ResolvedConsistency, DbError> {
    match consistency {
        Consistency::Canonical => Ok(ResolvedConsistency { frontier: resolver.current_frontier().await?, preview_id: None, historical: false }),
        Consistency::AtLeast(target) => {
            let current = resolver.current_frontier().await?;
            if !current.dominates(target)? {
                return Err(DbError::Unavailable(format!("current frontier (head_seq {}) has not yet reached requested AtLeast frontier (head_seq {})", current.head_seq, target.head_seq)));
            }
            Ok(ResolvedConsistency { frontier: current, preview_id: None, historical: false })
        }
        Consistency::Exact(target) => {
            let current = resolver.current_frontier().await?;
            if current != *target {
                return Err(DbError::NotFound(format!("no frontier exactly matching requested Exact frontier (head_seq {}, commit_seq {})", target.head_seq, target.commit_seq)));
            }
            Ok(ResolvedConsistency { frontier: current, preview_id: None, historical: false })
        }
        Consistency::Historical(commit_id) => Ok(ResolvedConsistency { frontier: resolver.frontier_for_commit(commit_id).await?, preview_id: None, historical: true }),
        Consistency::Speculative(preview_id) => Ok(ResolvedConsistency { frontier: resolver.current_frontier().await?, preview_id: Some(preview_id.clone()), historical: false }),
        Consistency::PreviewAugmented(preview_id) => Ok(ResolvedConsistency { frontier: resolver.current_frontier().await?, preview_id: Some(preview_id.clone()), historical: false }),
    }
}

/// @emoji 🔗️ A `ConsistencyResolver` backed by real `db_index` typed indexes: `Historical`
/// resolution is exactly `CommitIndex::lookup` (commit id → command seq) followed by
/// `FrontierIndex::lookup` (command seq → frontier), matching `CommitIndex`'s own doc comment
/// ("for `Consistency::Historical(commit_id)` query resolution"). Construction needs a live
/// `db_storage::IndexStorage` on the caller's side — this crate stays storage-agnostic by only ever
/// holding the already-constructed typed index handles, never constructing them itself.
pub struct IndexConsistencyResolver<'a, S: db_storage::IndexStorage> {
    pub commits: CommitIndex<'a, S>,
    pub frontiers: FrontierIndex<'a, S>,
}

impl<'resolver, S: db_storage::IndexStorage> ConsistencyResolver for IndexConsistencyResolver<'resolver, S> {
    async fn current_frontier(&self) -> Result<Frontier, DbError> {
        self.frontiers.latest().await?.ok_or_else(|| DbError::NotFound("no frontier has been recorded for this document yet".to_string()))
    }

    async fn frontier_for_commit(&self, commit_id: &str) -> Result<Frontier, DbError> {
        let command_seq = self.commits.lookup(commit_id).await?.ok_or_else(|| DbError::NotFound(format!("unknown commit id {commit_id:?}")))?;
        self.frontiers.lookup(command_seq).await?.ok_or_else(|| DbError::NotFound(format!("no frontier recorded at command_seq {command_seq}")))
    }
}
//#endregion 🔖️Consistency

//#region 🔖️Query
/// @emoji 🎯️ A single filter condition. `And`/`Or`/`Not` semio_compose_rs the rest into an arbitrary boolean
/// tree; `And([])` is vacuously true and `Or([])` is vacuously false, matching standard boolean
/// algebra rather than being treated as errors.
#[derive(Debug, PartialEq)]
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
            let target = if path.0.is_empty() { Some(value) } else { path.get(value) };
            target.is_some_and(|found| stringify_value(found).to_lowercase().contains(&term.to_lowercase()))
        }
        Predicate::And(predicates) => predicates.iter().all(|inner| eval_predicate(inner, value)),
        Predicate::Or(predicates) => predicates.iter().any(|inner| eval_predicate(inner, value)),
        Predicate::Not(inner) => !eval_predicate(inner, value),
    }
}

/// @emoji 🗂️ Which fields a query returns: the whole materialized `Value`, or a projected `Map`
/// keyed by each requested `Path`'s dotted `Display` string.
#[derive(Debug, PartialEq, Default)]
pub enum Select {
    #[default]
    All,
    Paths(Vec<Path>),
}

impl Select {
    fn project(&self, mut value: Value) -> Value {
        match self {
            Select::All => value,
            Select::Paths(paths) => {
                let mut map = BTreeMap::new();
                for path in paths {
                    if let Some(found) = take_path(&mut value, &path.0) {
                        map.insert(path.to_string(), found);
                    }
                }
                Value::Map(map)
            }
        }
    }
}

fn take_path(value: &mut Value, path: &[PathSegment]) -> Option<Value> {
    let Some((head, tail)) = path.split_first() else {
        return Some(std::mem::replace(value, Value::Null));
    };
    match (head, value) {
        (PathSegment::Field(name), Value::Map(map)) => {
            if tail.is_empty() {
                map.remove(name)
            } else {
                map.get_mut(name).and_then(|value| take_path(value, tail))
            }
        }
        (PathSegment::Index(index), Value::List(list)) => list.get_mut(*index).and_then(|value| take_path(value, tail)),
        _ => None,
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
        let left = key.path.get(a);
        let right = key.path.get(b);
        let mut ordering = match (left, right) {
            (Some(left), Some(right)) => compare_values(left, right),
            (None, None) => Ordering::Equal,
            (None, Some(right)) => compare_values(&Value::Null, right),
            (Some(left), None) => compare_values(left, &Value::Null),
        };
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
#[derive(Debug, Default, PartialEq)]
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
// 🚫️async: E1 pure, self-recursive accessor consumed by a sync Iterator::map — see R9
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

// 🚫️async: E1 pure accessor consumed by a sync Iterator::map — see R9
//#endregion 🔖️Limits

//#region 🔖️QuerySource
/// @emoji 🆔️ An opaque per-document row identifier. `u64` (not a `String`) to match `db_index`'s
/// own `doc_ref` convention for full-text/touched-region postings (see `db_index::FullTextIndex`'s
/// doc) — a `FullTextLookup`'s postings and a `QuerySource`'s row ids are meant to be the same
/// space, so pushdown candidates resolve back into `QuerySource::get` directly.
/// 🧪️ `Serialize`/`Deserialize` — same reason as `Value`'s: `RowId` is `QueryResultField::Key`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, serde::Serialize, serde::Deserialize)]
pub struct RowId(pub u64);

#[derive(Debug)]
pub struct QueryRow {
    id: RowId,
    value: Value,
}

impl QueryRow {
    pub fn new(id: RowId, value: Value) -> Self {
        Self { id, value }
    }

    pub fn id(&self) -> RowId {
        self.id
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn into_parts(self) -> (RowId, Value) {
        (self.id, self.value)
    }
}

const QUERY_ROW_SLOTS: usize = 64;
const QUERY_RETIRED_ROW_SETS: usize = 64;

#[derive(Debug)]
pub struct QueryRows {
    slots: [Option<QueryRow>; QUERY_ROW_SLOTS],
    len: u8,
    retirement: Option<QueryRowsRetirementReservation>,
}

impl QueryRows {
    pub fn new() -> Self {
        Self { slots: std::array::from_fn(|_| None), len: 0, retirement: None }
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// @emoji 🛂️ Reserves the exact retirement authority before a caller acquires its next row.
    pub fn preflight_push(&mut self) -> Result<(), DbError> {
        if self.len() == QUERY_ROW_SLOTS {
            return Err(DbError::LimitExceeded("query row slots"));
        }
        if self.len == 0 && self.retirement.is_none() {
            self.retirement = Some(reserve_query_rows_retirement().ok_or_else(|| DbError::Unavailable("query row retirement pressure refused admission".to_string()))?);
        }
        Ok(())
    }

    /// @emoji 📥️ Installs one row after `preflight_push` preserved its exact refusal boundary.
    pub fn push_preflighted(&mut self, row: QueryRow) {
        let index = self.len();
        self.slots[index] = Some(row);
        self.len += 1;
    }

    pub fn push(&mut self, row: QueryRow) -> Result<(), QueryRow> {
        if self.preflight_push().is_err() {
            return Err(row);
        }
        self.push_preflighted(row);
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &QueryRow> {
        self.slots[..self.len()].iter().flatten()
    }

    pub fn get(&self, index: usize) -> Option<&QueryRow> {
        self.slots.get(index).and_then(Option::as_ref)
    }

    fn sort_by(&mut self, mut compare: impl FnMut(&QueryRow, &QueryRow) -> Ordering) {
        for index in 1..self.len() {
            let mut cursor = index;
            while cursor > 0 {
                let order = compare(self.slots[cursor - 1].as_ref().expect("live query row slot"), self.slots[cursor].as_ref().expect("live query row slot"));
                if order != Ordering::Greater {
                    break;
                }
                self.slots.swap(cursor - 1, cursor);
                cursor -= 1;
            }
        }
    }

    fn take(&mut self, index: usize) -> Option<QueryRow> {
        if index >= self.len() {
            return None;
        }
        let row = self.slots[index].take();
        for cursor in index + 1..self.len() {
            self.slots[cursor - 1] = self.slots[cursor].take();
        }
        self.len -= 1;
        if self.len == 0 {
            release_query_rows_retirement(&mut self.retirement);
        }
        row
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        if self.len == 0 {
            return Ok(false);
        }
        let index = self.len() - 1;
        let row = self.slots[index].as_mut().ok_or_else(|| DbError::Internal("query close lost row".to_string()))?;
        if value_close_step(&mut row.value)? {
            return Ok(true);
        }
        self.slots[index] = None;
        self.len -= 1;
        if self.len == 0 {
            release_query_rows_retirement(&mut self.retirement);
        }
        Ok(true)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.slots.iter().all(Option::is_none)
    }
}

impl Default for QueryRows {
    fn default() -> Self {
        Self::new()
    }
}

static QUERY_RETIRED_ROWS: std::sync::Mutex<[Option<QueryRows>; QUERY_RETIRED_ROW_SETS]> = std::sync::Mutex::new([const { None }; QUERY_RETIRED_ROW_SETS]);
static QUERY_RETIRED_ROWS_OVERFLOW: std::sync::Mutex<[Option<QueryRows>; QUERY_RETIRED_ROW_SETS]> = std::sync::Mutex::new([const { None }; QUERY_RETIRED_ROW_SETS]);
static QUERY_RETIRED_ROWS_QUARANTINE: std::sync::Mutex<[Option<QueryRows>; QUERY_RETIRED_ROW_SETS]> = std::sync::Mutex::new([const { None }; QUERY_RETIRED_ROW_SETS]);
static QUERY_RETIRED_ROWS_RESERVATIONS: [std::sync::atomic::AtomicU64; 3] = [const { std::sync::atomic::AtomicU64::new(0) }; 3];
static QUERY_RETIREMENT_PRESSURE_FAULT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[derive(Clone, Copy, Debug)]
struct QueryRowsRetirementReservation {
    tier: u8,
    index: u8,
}

fn reserve_query_rows_retirement() -> Option<QueryRowsRetirementReservation> {
    for tier in 0..3u8 {
        for index in 0..QUERY_RETIRED_ROW_SETS as u8 {
            let bit = 1u64 << index;
            if QUERY_RETIRED_ROWS_RESERVATIONS[tier as usize].fetch_or(bit, std::sync::atomic::Ordering::AcqRel) & bit != 0 {
                continue;
            }
            let vacant = match tier {
                0 => QUERY_RETIRED_ROWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index as usize].is_none(),
                1 => QUERY_RETIRED_ROWS_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index as usize].is_none(),
                _ => QUERY_RETIRED_ROWS_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index as usize].is_none(),
            };
            if vacant {
                if tier != 0 {
                    QUERY_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
                }
                return Some(QueryRowsRetirementReservation { tier, index });
            }
            QUERY_RETIRED_ROWS_RESERVATIONS[tier as usize].fetch_and(!bit, std::sync::atomic::Ordering::AcqRel);
        }
    }
    QUERY_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
    None
}

fn release_query_rows_retirement(reservation: &mut Option<QueryRowsRetirementReservation>) {
    if let Some(reservation) = reservation.take() {
        QUERY_RETIRED_ROWS_RESERVATIONS[reservation.tier as usize].fetch_and(!(1u64 << reservation.index), std::sync::atomic::Ordering::AcqRel);
    }
}

fn query_rows_vacant_retirement_slot(tier: usize, slots: &[Option<QueryRows>]) -> Option<usize> {
    let reserved = QUERY_RETIRED_ROWS_RESERVATIONS[tier].load(std::sync::atomic::Ordering::Acquire);
    slots.iter().enumerate().position(|(index, slot)| slot.is_none() && reserved & (1u64 << index) == 0)
}

fn install_reserved_query_rows(owner: QueryRows) {
    let reservation = owner.retirement.unwrap_or(QueryRowsRetirementReservation { tier: 0, index: 0 });
    match reservation.tier {
        0 => QUERY_RETIRED_ROWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[reservation.index as usize] = Some(owner),
        1 => QUERY_RETIRED_ROWS_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[reservation.index as usize] = Some(owner),
        _ => QUERY_RETIRED_ROWS_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[reservation.index as usize] = Some(owner),
    }
}

#[cfg(test)]
fn retire_query_rows(owner: QueryRows) -> Result<(), QueryRows> {
    if owner.retirement.is_some() {
        install_reserved_query_rows(owner);
        return Ok(());
    }
    let mut retired = QUERY_RETIRED_ROWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(index) = query_rows_vacant_retirement_slot(0, &retired[..]) {
        retired[index] = Some(owner);
        Ok(())
    } else {
        drop(retired);
        QUERY_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
        let mut overflow = QUERY_RETIRED_ROWS_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(index) = query_rows_vacant_retirement_slot(1, &overflow[..]) {
            overflow[index] = Some(owner);
            return Ok(());
        }
        drop(overflow);
        let mut quarantine = QUERY_RETIRED_ROWS_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(index) = query_rows_vacant_retirement_slot(2, &quarantine[..]) else { return Err(owner) };
        quarantine[index] = Some(owner);
        Ok(())
    }
}

fn retire_query_row_at(rows: &mut QueryRows, index: usize) -> Result<(), DbError> {
    let retirement = reserve_query_rows_retirement().ok_or_else(|| DbError::Unavailable("query row retirement pressure refused removal".to_string()))?;
    let row = rows.take(index).ok_or_else(|| DbError::Internal("query row retirement lost preflighted row".to_string()))?;
    let mut retired = QueryRows::new();
    retired.retirement = Some(retirement);
    retired.slots[0] = Some(row);
    retired.len = 1;
    install_reserved_query_rows(retired);
    Ok(())
}

fn retire_query_row_with_reservation(row: QueryRow, retirement: QueryRowsRetirementReservation) {
    let mut rows = QueryRows::new();
    rows.retirement = Some(retirement);
    rows.slots[0] = Some(row);
    rows.len = 1;
    install_reserved_query_rows(rows);
}

pub fn query_rows_maintenance_step() -> Result<bool, DbError> {
    let mut retired = QUERY_RETIRED_ROWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(slot) = retired.iter_mut().find(|slot| slot.is_some()) else {
        drop(retired);
        let mut overflow = QUERY_RETIRED_ROWS_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(index) = overflow.iter().position(Option::is_some) else {
            drop(overflow);
            let mut quarantine = QUERY_RETIRED_ROWS_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(index) = quarantine.iter().position(Option::is_some) else { return Ok(false) };
            let mut retired = QUERY_RETIRED_ROWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(target) = query_rows_vacant_retirement_slot(0, &retired[..]) else {
                drop(retired);
                let owner = quarantine[index].as_mut().ok_or_else(|| DbError::Internal("query quarantine retirement changed row owner".to_string()))?;
                if !owner.close_step()? {
                    quarantine[index] = None;
                }
                return Ok(true);
            };
            retired[target] = quarantine[index].take();
            return Ok(true);
        };
        let mut retired = QUERY_RETIRED_ROWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(target) = query_rows_vacant_retirement_slot(0, &retired[..]) else {
            drop(retired);
            let owner = overflow[index].as_mut().ok_or_else(|| DbError::Internal("query overflow retirement changed row owner".to_string()))?;
            if !owner.close_step()? {
                overflow[index] = None;
            }
            return Ok(true);
        };
        retired[target] = overflow[index].take();
        return Ok(true);
    };
    let owner = slot.as_mut().ok_or_else(|| DbError::Internal("query retired row set changed owner".to_string()))?;
    if owner.close_step()? {
        return Ok(true);
    }
    *slot = None;
    Ok(true)
}

impl Drop for QueryRows {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            release_query_rows_retirement(&mut self.retirement);
            return;
        }
        install_reserved_query_rows(std::mem::replace(self, Self::new()));
    }
}

fn value_close_step(value: &mut Value) -> Result<bool, DbError> {
    match value {
        Value::Bytes(bytes) => Ok(bytes.close_step()?.is_some()),
        Value::List(items) => {
            let Some(item) = items.last_mut() else { return Ok(false) };
            if value_close_step(item)? {
                return Ok(true);
            }
            items.pop();
            Ok(true)
        }
        Value::Map(map) => {
            let Some((key, mut item)) = map.pop_last() else { return Ok(false) };
            if value_close_step(&mut item)? {
                map.insert(key, item);
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}

/// @emoji 🚰️ What the planner/evaluator need from a materialized document: every row (for a full
/// scan), or one row by id (for a pushdown candidate list). No `Send + Sync` bound — see the module
/// doc's note on `db_state`'s `Rc`-based structures.
pub trait QuerySource {
    async fn scan(&self, control: &mut QueryCursorControl) -> Result<QueryRows, DbError>;

    /// @emoji 🎯️ Default: linear `scan` + find. Override when a cheaper direct lookup exists (e.g.
    /// `PVec`'s below, which is index-addressed).
    async fn get(&self, id: RowId, control: &mut QueryCursorControl) -> Result<Option<QueryRow>, DbError> {
        let mut rows = self.scan(control).await?;
        let found_index = rows.iter().position(|row| row.id() == id);
        let found = found_index.and_then(|index| rows.take(index));
        control.grant()?;
        let _ = rows.close_step()?;
        drop(rows);
        Ok(found)
    }
}

/// @emoji 🧵️ The natural `QuerySource` over a `db_state::PVec`: row id = element index. This is the
/// crate's one built-in `QuerySource`, demonstrating the intended wiring to `db_state`'s persistent
/// structures — a caller with a richer per-document schema (`db_artifact`) supplies its own
/// `QuerySource` over whatever `PMap`/`PTree`/overlay shape it actually stores.
/// @emoji 🔌️ What a full-text pushdown needs: term → candidate row ids. `db_index::FullTextIndex`
/// implements this directly below (its `doc_ref` postings are exactly this trait's `RowId`s).
pub trait FullTextLookup {
    async fn search(&self, term: &str) -> Result<db_storage::DbIoU64List, DbError>;
}

/// @emoji 🚫️ The phantom `FullTextLookup` type for `execute`/`refresh`'s `fulltext: None` call
/// sites — replaces the old `Option<&dyn FullTextLookup>` (per ruling **O1**, `dyn FullTextLookup`
/// stopped being object-safe the moment `search` became a real `async fn`), so the generic
/// `fulltext: Option<&impl FullTextLookup>` parameter needs a concrete type on every `None` call
/// site for inference to succeed. Never actually constructed or called.
pub enum NoFullTextLookup {}

impl FullTextLookup for NoFullTextLookup {
    async fn search(&self, _term: &str) -> Result<db_storage::DbIoU64List, DbError> {
        match *self {}
    }
}

impl<'index, S: db_storage::IndexStorage> FullTextLookup for FullTextIndex<'index, S> {
    async fn search(&self, term: &str) -> Result<db_storage::DbIoU64List, DbError> {
        FullTextIndex::search(self, term).await
    }
}
//#endregion 🔖️QuerySource

//#region 🔖️ProjectionBridge
/// @emoji 🛡️ Ceiling on a decoded `Value::List`/`Value::Map`'s declared element count, checked via
/// `check_len` BEFORE `decode_value` allocates its `Vec`/`BTreeMap` — the same
/// "validate before allocating" invariant `QueryLimits` and every decoder across the family holds to.
/// @emoji ✍️ `Value`'s own canonical binary encoding — this crate's own choice (the
/// `db_projection::ProjectionState` trait leaves the exact byte shape unspecified): a tag byte per
/// variant followed by the variant's payload, `List`/`Map` recursing depth-first. `Map` is
/// `BTreeMap`-backed, so its entries are already emitted in ascending key order.
/// @emoji 🔌️ `Value`'s `db_projection::ProjectionState` impl — lets any `db_projection::ProjectionClass`
/// (registered by a higher layer, e.g. `db_artifact`, which owns the `protocol::MutationEnvelope`
/// interpretation this crate deliberately never touches — see the module doc) declare `State = Value`
/// and get this crate's query/planner/live-diff machinery for free over its checkpointed state, via
/// `projection_query_source` below.
/// @emoji 📽️ A `QuerySource` over one decoded projection state `Value`, row-shaped so `execute`/
/// `LiveQuery` can run over it exactly like any other source: a `List` becomes one row per element
/// (positional `RowId`, matching `PVec<Value>`'s convention above), a `Map` becomes one row per
/// entry (`RowId` assigned by ascending key order — `BTreeMap`'s natural iteration, so it's stable
/// across calls for the same map shape), and any other `Value` becomes a single `RowId(0)` row.
pub struct ProjectionSource(std::cell::RefCell<QueryRows>);

impl ProjectionSource {
    pub async fn from_value(value: Value) -> Result<ProjectionSource, DbError> {
        let mut rows = QueryRows::new();
        let count = match &value {
            Value::List(items) => items.len(),
            Value::Map(map) => map.len(),
            _ => 1,
        };
        check_len(count as u64, QUERY_ROW_SLOTS as u64, "projection query row slots")?;
        if count != 0 {
            rows.preflight_push()?;
        }
        match value {
            Value::List(items) => {
                for (index, item) in items.into_iter().enumerate() {
                    rows.push_preflighted(QueryRow::new(RowId(index as u64), item));
                }
            }
            Value::Map(map) => {
                for (index, item) in map.into_values().enumerate() {
                    rows.push_preflighted(QueryRow::new(RowId(index as u64), item));
                }
            }
            other => rows.push_preflighted(QueryRow::new(RowId(0), other)),
        }
        Ok(ProjectionSource(std::cell::RefCell::new(rows)))
    }

    pub async fn len(&self) -> usize {
        self.0.borrow().len()
    }

    pub async fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }
}

impl QuerySource for ProjectionSource {
    async fn scan(&self, control: &mut QueryCursorControl) -> Result<QueryRows, DbError> {
        control.grant()?;
        Ok(std::mem::take(&mut *self.0.borrow_mut()))
    }
}

/// @emoji 🌉️ The bridge the module doc's `db_projection`-integration note describes: decodes
/// `state_bytes` (whatever a caller already retrieved from `db_projection::ProjectionEngine::state_at`
/// for `Consistency::Historical`/`Canonical`, or `::preview_augmented` for
/// `Consistency::Speculative`/`PreviewAugmented` — both return exactly this shape, plain
/// `ProjectionState`-encoded bytes with any version-prefix framing already stripped by
/// `ProjectionEngine` itself) into a `ProjectionSource` this crate's `execute`/`LiveQuery` can run
/// over. See the module doc for why this takes raw bytes rather than a `&ProjectionEngine` reference.
pub async fn projection_query_source(value: Value) -> Result<ProjectionSource, DbError> {
    ProjectionSource::from_value(value).await
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
    pub async fn kind(&self) -> QueryPlanKind {
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
// 🚫️async: E1 pure, self-recursive accessor consumed by a sync Iterator::find_map — see R9
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
#[derive(Debug)]
pub struct QueryResult {
    pub rows: QueryRows,
    pub diagnostics: QueryDiagnostics,
}

/// @emoji ▶️ Plans and evaluates `query` against `source`. `fulltext` is only consulted if the
/// planner chose `QueryPlan::FullTextPushdown`; passing `None` for a query the planner would push
/// down surfaces `DbError::InvalidArgument` rather than silently falling back to a full scan (a
/// caller that owns a `FullTextLookup` should always pass it — silent fallback would hide a
/// wiring bug as a performance regression instead of a compile/runtime-visible one).
// 🔀️ `source: &impl QuerySource` (was `&dyn`) — same R11(a) treatment `FullTextLookup` already got
// on this trait's neighbor above: `QuerySource::scan`'s `async fn` broke `dyn` object-safety, and
// every call site here already passes exactly one concrete type (`PVec<Value>`, `ProjectionSource`,
// or `db_artifact`'s own `StateQuerySource`), never a runtime-chosen mix — parameter-position
// generic, no design question.
pub async fn execute(query: &Query, source: &impl QuerySource, fulltext: Option<&impl FullTextLookup>, limits: &QueryLimits, control: &mut QueryCursorControl) -> Result<QueryResult, DbError> {
    query_rows_maintenance_step()?;
    let chosen_plan = plan(query);
    let mut scanned: u64 = 0;
    let mut matched = source.scan(control).await?;

    match &chosen_plan {
        QueryPlan::FullScan => {
            let mut index = 0;
            while index < matched.len() {
                control.grant()?;
                scanned += 1;
                check_len(scanned, limits.max_scan_rows, "db_query::rows_scanned")?;
                let accepted = query.filter.as_ref().is_none_or(|predicate| eval_predicate(predicate, matched.slots[index].as_ref().expect("live query row slot").value()));
                if accepted {
                    index += 1;
                } else {
                    retire_query_row_at(&mut matched, index)?;
                }
            }
        }
        QueryPlan::FullTextPushdown { term } => {
            let lookup = fulltext.ok_or_else(|| DbError::InvalidArgument("query planned a full-text pushdown but no FullTextLookup was supplied".to_string()))?;
            let mut ids = lookup.search(term).await?;
            let mut index = 0;
            while index < matched.len() {
                control.grant()?;
                scanned += 1;
                check_len(scanned, limits.max_scan_rows, "db_query::rows_scanned")?;
                let row = matched.slots[index].as_ref().expect("live query row slot");
                let accepted = ids.as_slice().contains(&row.id().0) && query.filter.as_ref().is_none_or(|predicate| eval_predicate(predicate, row.value()));
                if accepted {
                    index += 1;
                } else {
                    retire_query_row_at(&mut matched, index)?;
                }
            }
            control.grant()?;
            let _ = ids.close_step();
            drop(ids);
        }
    }

    if !query.sort.is_empty() {
        matched.sort_by(|left, right| compare_rows(left.value(), right.value(), &query.sort));
    }
    let rows_matched = matched.len() as u64;

    let offset = query.offset.unwrap_or(0) as usize;
    for _ in 0..offset.min(matched.len()) {
        retire_query_row_at(&mut matched, 0)?;
    }
    if let Some(limit) = query.limit.and_then(|limit| usize::try_from(limit).ok()) {
        while matched.len() > limit {
            let index = matched.len() - 1;
            retire_query_row_at(&mut matched, index)?;
        }
    }
    check_len(matched.len() as u64, limits.max_result_rows, "db_query::result_rows")?;

    let mut projected = QueryRows::new();
    while !matched.is_empty() {
        control.grant()?;
        projected.preflight_push()?;
        let row = matched.take(0).ok_or_else(|| DbError::Internal("query projection lost its preflighted row".to_string()))?;
        let (id, value) = row.into_parts();
        projected.push_preflighted(QueryRow::new(id, query.select.project(value)));
    }
    let result_bytes = projected.iter().map(|row| 8 + value_byte_estimate(row.value())).sum();
    check_len(result_bytes, limits.max_result_bytes, "db_query::result_bytes")?;

    let rows_returned = projected.len() as u64;
    Ok(QueryResult { rows: projected, diagnostics: QueryDiagnostics { plan: chosen_plan.kind().await, rows_scanned: scanned, rows_matched, rows_returned } })
}
//#endregion 🔖️Execute

//#region 🔖️Stream
/// @emoji 🌊️ A `QueryResult`'s rows as an `Iterator`, for callers (`ArtifactHandle::query`'s
/// contract-frozen return type) that want to consume incrementally rather than hold the whole
/// `Vec`. Backed by an already-materialized `Vec::IntoIter` — see `db_state::PMap::iter`'s doc for
/// this crate family's established "eagerly materialize, simple to reason about" precedent; a
/// truly lazy pull-based evaluator is a straightforward future optimization.
pub struct QueryStream {
    rows: QueryRows,
    pub diagnostics: QueryDiagnostics,
}

impl QueryResult {
    pub async fn into_stream(self) -> QueryStream {
        QueryStream { rows: self.rows, diagnostics: self.diagnostics }
    }
}

impl Iterator for QueryStream {
    type Item = (RowId, Value);

    fn next(&mut self) -> Option<Self::Item> {
        self.rows.take(0).map(QueryRow::into_parts)
    }
}
//#endregion 🔖️Stream

//#region 🔖️LiveQuery
/// @emoji 📡️ A `Query` plus the `Consistency` it should be (re-)evaluated under — what a caller
/// hands to `ArtifactHandle::subscribe`. This crate only owns the diffing law (below); actor-level
/// registration/notification wiring belongs to `db_artifact`.
pub struct LiveQuerySpec {
    pub query: Query,
    pub consistency: Consistency,
}

/// @emoji 🔀️ The change between two successive evaluations of a `LiveQuery`'s `Query`: rows newly
/// present, rows no longer present, and rows present in both but with a changed `Value`.
pub struct QueryDiff {
    pub added: QueryRows,
    pub removed: db_storage::DbIoU64List,
    pub updated: QueryRows,
}

impl Default for QueryDiff {
    fn default() -> Self {
        Self { added: QueryRows::new(), removed: db_storage::DbIoU64List::new(), updated: QueryRows::new() }
    }
}

fn hash_query_value(value: &Value, hash: &mut semio_framework_hash::Hasher) {
    match value {
        Value::Null => {
            hash.update(&[0]);
        }
        Value::Bool(value) => {
            hash.update(&[1, u8::from(*value)]);
        }
        Value::Int(value) => {
            hash.update(&[2]);
            hash.update(&value.to_le_bytes());
        }
        Value::Float(value) => {
            hash.update(&[3]);
            hash.update(&value.to_le_bytes());
        }
        Value::Text(value) => {
            hash.update(&[4]);
            hash.update(&(value.len() as u64).to_le_bytes());
            hash.update(value.as_bytes());
        }
        Value::Bytes(value) => {
            hash.update(&[5]);
            hash.update(&(value.len() as u64).to_le_bytes());
            for fragment in value.fragments() {
                hash.update(fragment);
            }
        }
        Value::List(values) => {
            hash.update(&[6]);
            hash.update(&(values.len() as u64).to_le_bytes());
            for value in values {
                hash_query_value(value, hash);
            }
        }
        Value::Map(values) => {
            hash.update(&[7]);
            hash.update(&(values.len() as u64).to_le_bytes());
            for (key, value) in values {
                hash.update(&(key.len() as u64).to_le_bytes());
                hash.update(key.as_bytes());
                hash_query_value(value, hash);
            }
        }
    }
}

fn query_value_hash(value: &Value) -> [u8; 32] {
    let mut hash = semio_framework_hash::Hasher::new();
    hash_query_value(value, &mut hash);
    *hash.finalize().as_bytes()
}

/// @emoji 📺️ Tracks one live query's last-seen result set so `refresh` can emit a `QueryDiff`
/// instead of the caller having to re-diff two full `QueryResult`s itself. The law this crate's
/// tests hold it to: applying a `QueryDiff` to the pre-refresh snapshot (add `added`, drop
/// `removed`, overwrite `updated`) always reconstructs exactly the post-refresh snapshot. Per-row
/// content is now sourced through `QueryResultField`/`InferenceCache` (`🔖️QueryResultField` above)
/// rather than held as this struct's own ad hoc cache — `LiveQuery` itself is left owning only the
/// diff-adapter comparison (this refresh's spine-derived rows vs. the previous refresh's), which is
/// NOT redundant with the spine's own hit/miss bookkeeping: a `DepHash` cache hit means "this exact
/// row content has been seen before, at any point", not "unchanged since the immediately preceding
/// refresh" (a row that oscillates between two values would warm-hit the cache every other refresh
/// while still being a real `updated` event each time) — so the two mechanisms are complementary,
/// not duplicate.
pub struct LiveQuery {
    spec: LiveQuerySpec,
    snapshot: [Option<(RowId, [u8; 32])>; QUERY_ROW_SLOTS],
    snapshot_len: u8,
}

impl LiveQuery {
    pub async fn new(spec: LiveQuerySpec) -> LiveQuery {
        LiveQuery { spec, snapshot: [None; QUERY_ROW_SLOTS], snapshot_len: 0 }
    }

    pub async fn spec(&self) -> &LiveQuerySpec {
        &self.spec
    }

    pub fn snapshot(&self) -> impl Iterator<Item = (RowId, [u8; 32])> + '_ {
        self.snapshot[..self.snapshot_len as usize].iter().flatten().copied()
    }

    /// @emoji 🔁️ Re-executes `self.spec.query` against `source` and diffs the result against the
    /// previous snapshot, updating the snapshot in place. `source`/`fulltext` are expected to
    /// already be materialized at whatever frontier `resolve_consistency(&self.spec.consistency,
    /// ..)` resolved to — resolving that frontier and building the matching `QuerySource` is the
    /// caller's job (it owns the actual document state), not this crate's. Row values are now
    /// obtained via `pack::infer_field::<QuerySnapshot, QueryResultField>` (routed through this
    /// `LiveQuery`'s own `InferenceCache`) instead of being read directly off `result.rows` — a
    /// row whose content is byte-identical to one already seen by this cache is served from the
    /// cache rather than re-materialized, per `QueryResultField`'s doc above.
    // 🔀️ `source: &impl QuerySource` (was `&dyn`) — same rationale as `execute`.
    pub async fn refresh(&mut self, source: &impl QuerySource, fulltext: Option<&impl FullTextLookup>, limits: &QueryLimits, control: &mut QueryCursorControl) -> Result<QueryDiff, DbError> {
        let mut result = execute(&self.spec.query, source, fulltext, limits, control).await?;
        let mut diff = QueryDiff::default();
        let mut next = [None; QUERY_ROW_SLOTS];
        let mut next_len = 0;
        while !result.rows.is_empty() {
            control.grant()?;
            let (row_id, hash, previous) = {
                let row = result.rows.get(0).ok_or_else(|| DbError::Internal("live query refresh lost source row".to_string()))?;
                let hash = query_value_hash(row.value());
                let previous = self.snapshot[..self.snapshot_len as usize].iter().flatten().find(|(id, _)| *id == row.id()).map(|(_, hash)| *hash);
                (row.id(), hash, previous)
            };
            match previous {
                None => diff.added.preflight_push()?,
                Some(previous) if previous != hash => diff.updated.preflight_push()?,
                Some(_) => {}
            }
            let retirement = if previous == Some(hash) { Some(reserve_query_rows_retirement().ok_or_else(|| DbError::Unavailable("query row retirement pressure refused refresh".to_string()))?) } else { None };
            let row = result.rows.take(0).ok_or_else(|| DbError::Internal("live query refresh lost preflighted row".to_string()))?;
            next[next_len] = Some((row_id, hash));
            next_len += 1;
            match previous {
                None => diff.added.push_preflighted(row),
                Some(previous) if previous != hash => diff.updated.push_preflighted(row),
                Some(_) => retire_query_row_with_reservation(row, retirement.ok_or_else(|| DbError::Internal("live query refresh lost unchanged-row retirement".to_string()))?),
            }
        }
        for (id, _) in self.snapshot[..self.snapshot_len as usize].iter().flatten() {
            if !next[..next_len].iter().flatten().any(|(next_id, _)| next_id == id) {
                diff.removed.push(id.0)?;
            }
        }
        self.snapshot = next;
        self.snapshot_len = next_len as u8;
        Ok(diff)
    }
}
//#endregion 🔖️LiveQuery

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
