//! 🔌️ Query source and projection: the `QuerySource` and full-text lookup traits and the projection bridge. See [`super`].

use super::*;

//#region 🔖️QuerySource
/// 🚰️ What the planner/evaluator need from a materialized document: every row (for a full
/// scan), or one row by id (for a pushdown candidate list). No `Send + Sync` bound — see the module
/// doc's note on `db_state`'s `Rc`-based structures.
pub trait QuerySource {
    async fn scan(&self, control: &mut QueryCursorControl) -> Result<QueryRows, DbError>;

    /// 🎯️ Default: linear `scan` + find. Override when a cheaper direct lookup exists (e.g.
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

/// 🧵️ The natural `QuerySource` over a `db_state::PVec`: row id = element index. This is the
/// crate's one built-in `QuerySource`, demonstrating the intended wiring to `db_state`'s persistent
/// structures — a caller with a richer per-document schema (`db_artifact`) supplies its own
/// `QuerySource` over whatever `PMap`/`PTree`/overlay shape it actually stores.
/// 🔌️ What a full-text pushdown needs: term → candidate row ids. `db_index::FullTextIndex`
/// implements this directly below (its `doc_ref` postings are exactly this trait's `RowId`s).
pub trait FullTextLookup {
    async fn search(&self, term: &str) -> Result<db_io_core::DbIoU64List, DbError>;
}

/// 🚫️ The phantom `FullTextLookup` type for `execute`/`refresh`'s `fulltext: None` call
/// sites — replaces the old `Option<&dyn FullTextLookup>` (per ruling **O1**, `dyn FullTextLookup`
/// stopped being object-safe the moment `search` became a real `async fn`), so the generic
/// `fulltext: Option<&impl FullTextLookup>` parameter needs a concrete type on every `None` call
/// site for inference to succeed. Never actually constructed or called.
pub enum NoFullTextLookup {}

impl FullTextLookup for NoFullTextLookup {
    async fn search(&self, _term: &str) -> Result<db_io_core::DbIoU64List, DbError> {
        match *self {}
    }
}

impl<'index, S: db_io_core::IndexStorage> FullTextLookup for FullTextIndex<'index, S> {
    async fn search(&self, term: &str) -> Result<db_io_core::DbIoU64List, DbError> {
        FullTextIndex::search(self, term).await
    }
}
//#endregion 🔖️QuerySource

//#region 🔖️ProjectionBridge
/// 🛡️ Ceiling on a decoded `Value::List`/`Value::Map`'s declared element count, checked via
/// `check_len` BEFORE `decode_value` allocates its `Vec`/`BTreeMap` — the same
/// "validate before allocating" invariant `QueryLimits` and every decoder across the family holds to.
/// ✍️ `Value`'s own canonical binary encoding — this crate's own choice (the
/// `db_projection::ProjectionState` trait leaves the exact byte shape unspecified): a tag byte per
/// variant followed by the variant's payload, `List`/`Map` recursing depth-first. `Map` is
/// `BTreeMap`-backed, so its entries are already emitted in ascending key order.
/// 🔌️ `Value`'s `db_projection::ProjectionState` impl — lets any `db_projection::ProjectionClass`
/// (registered by a higher layer, e.g. `db_artifact`, which owns the `protocol::MutationEnvelope`
/// interpretation this crate deliberately never touches — see the module doc) declare `State = Value`
/// and get this crate's query/planner/live-diff machinery for free over its checkpointed state, via
/// `projection_query_source` below.
/// 📽️ A `QuerySource` over one decoded projection state `Value`, row-shaped so `execute`/
/// `LiveQuery` can run over it exactly like any other source: a `List` becomes one row per element
/// (positional `RowId`, matching `PVec<Value>`'s convention above), a `Map` becomes one row per
/// entry (`RowId` assigned by ascending key order — `BTreeMap`'s natural iteration, so it's stable
/// across calls for the same map shape), and any other `Value` becomes a single `RowId(0)` row.
pub struct ProjectionSource(pub(super) std::cell::RefCell<QueryRows>);

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

/// 🌉️ The bridge the module doc's `db_projection`-integration note describes: decodes
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
