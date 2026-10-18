//! 🧮️ Value and path: the cursor control, the ordered query bytes, the `Value` model with its comparison and rendering, and the `Path` segments. See [`super`].

use super::*;

//#region 🔖️Value
pub struct QueryCursorControl {
    pub cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    pub deadline: std::time::Instant,
    pub fuel: usize,
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
    pub pages: db_io_core::DbIoPages,
}

impl QueryBytes {
    pub fn from_pages(pages: db_io_core::DbIoPages) -> Result<Self, (DbError, db_io_core::DbIoPages)> {
        if pages.operation() == 0 {
            return Err((DbError::InvalidArgument("query bytes require a nonzero operation".to_string()), pages));
        }
        Ok(Self { pages })
    }

    pub async fn copy_from_pages(source: &db_io_core::DbIoPages, control: &mut QueryCursorControl) -> Result<Self, DbError> {
        let mut writer = db_io_core::DbIoPageWriter::try_reserve(source.len().div_ceil(db_io_core::DB_IO_PAGE_BYTES)).map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
        for fragment in source.fragments() {
            control.grant()?;
            if writer.write_fragment(fragment)? != fragment.len() {
                return Err(DbError::Internal("query byte copy made partial progress".to_string()));
            }
            semio_framework_async::yield_once().await;
        }
        Ok(Self { pages: writer.seal_retained().await.map_err(db_io_core::DbIoPageWriterRejected::into_error)? })
    }

    pub fn len(&self) -> usize {
        self.pages.len()
    }

    pub fn fragments(&self) -> db_io_core::DbIoPageReader<'_> {
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

/// 🧬️ The dynamic value model every `Query` evaluates against. Deliberately this crate's
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

/// 🥇️ Cross-type ordering rank, used only as `compare_values`'s tie-breaker between values
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

/// ⚖️ A total order over `Value` (needed since `f64` alone isn't `Ord`): same-variant pairs
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

/// 🧵️ A best-effort text rendering of any `Value`, used only by `Predicate::FullText`'s
/// index-free fallback match (see that variant's doc).
// 🚫️async: E1 pure, self-recursive accessor consumed by a sync Iterator::map — see R9
pub(super) fn stringify_value(value: &Value) -> String {
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

/// 🧩️ One step of a `Path`: a named field into a `Value::Map`, or a positional index into a
/// `Value::List`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PathSegment {
    Field(String),
    Index(usize),
}

/// 🛤️ A field/index path into a `Value` tree — the addressing scheme every `Predicate`/
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

    /// 🔤️ Splits a dotted string into segments, treating any segment that parses as a plain
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

    /// 🔎️ Walks `value` through this path's segments. `None` on any type mismatch (e.g. an
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
