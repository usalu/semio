//! 🎚️ Consistency and query model: the consistency levels and resolvers, predicates, selects, sort keys and the `Query` builder. See [`super`].

use super::*;

//#region 🔖️Consistency
/// 🧭️ The six read-consistency modes `ArtifactHandle::query` accepts (frozen in the db
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

/// 🗺️ What a `Consistency` resolves to: the concrete `Frontier` a `QuerySource` should be
/// materialized at, which named preview (if any) participates, and whether this is a replay read
/// (informational — callers use it to decide whether write-side effects like live-query
/// registration are even meaningful for this read).
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedConsistency {
    pub frontier: Frontier,
    pub preview_id: Option<String>,
    pub historical: bool,
}

/// 🔌️ What `resolve_consistency` needs from its caller: the document's current frontier, and
/// a commit-id → frontier lookup for `Historical`. Kept minimal and storage-agnostic so this crate
/// never needs a concrete storage dependency of its own — `IndexConsistencyResolver` below is the
/// ready-made adapter over the sibling `db_index` crate.
pub trait ConsistencyResolver {
    async fn current_frontier(&self) -> Result<Frontier, DbError>;
    async fn frontier_for_commit(&self, commit_id: &str) -> Result<Frontier, DbError>;
}

/// 🧮️ Resolves `consistency` against `resolver` into a concrete `ResolvedConsistency`.
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

/// 🔗️ A `ConsistencyResolver` backed by real `db_index` typed indexes: `Historical`
/// resolution is exactly `CommitIndex::lookup` (commit id → command seq) followed by
/// `FrontierIndex::lookup` (command seq → frontier), matching `CommitIndex`'s own doc comment
/// ("for `Consistency::Historical(commit_id)` query resolution"). Construction needs a live
/// `db_storage::IndexStorage` on the caller's side — this crate stays storage-agnostic by only ever
/// holding the already-constructed typed index handles, never constructing them itself.
pub struct IndexConsistencyResolver<'a, S: db_io_core::IndexStorage> {
    pub commits: CommitIndex<'a, S>,
    pub frontiers: FrontierIndex<'a, S>,
}

impl<'resolver, S: db_io_core::IndexStorage> ConsistencyResolver for IndexConsistencyResolver<'resolver, S> {
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
/// 🎯️ A single filter condition. `And`/`Or`/`Not` semio_compose_rs the rest into an arbitrary boolean
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
    /// 🔤️ Case-insensitive substring match. `Path::empty()` matches against the whole
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

/// ✅️ Evaluates `predicate` against one materialized row `value`. A missing path is treated
/// as failing every comparison predicate (including `Ne`, deliberately: "the field isn't even
/// present" is not the same claim as "the field is present and differs").
pub(super) fn eval_predicate(predicate: &Predicate, value: &Value) -> bool {
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

/// 🗂️ Which fields a query returns: the whole materialized `Value`, or a projected `Map`
/// keyed by each requested `Path`'s dotted `Display` string.
#[derive(Debug, PartialEq, Default)]
pub enum Select {
    #[default]
    All,
    Paths(Vec<Path>),
}

impl Select {
    pub(super) fn project(&self, mut value: Value) -> Value {
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

/// 🔀️ One sort key: a `Path` to compare by, and its direction. A missing path sorts as
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

pub(super) fn compare_rows(a: &Value, b: &Value, sort: &[SortKey]) -> Ordering {
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

/// 📜️ A complete query: what to return (`select`), which rows qualify (`filter`), in what
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
