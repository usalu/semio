//! 🗺️ Planner, execute and live query: the query plan, execution, the result stream and the live query with its diffs. See [`super`].

use super::*;

//#region 🔖️Planner
/// 🗺️ The chosen execution strategy for a `Query`. `FullTextPushdown`'s `term` is always
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

/// 🔍️ Finds the first `Predicate::FullText` term reachable through a top-level conjunction
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

/// 🧠️ Chooses a `QueryPlan` for `query`. Currently a single heuristic (full-text pushdown);
/// the extension point for future pushdown kinds (indexed equality, etc.) is this one function.
pub fn plan(query: &Query) -> QueryPlan {
    match query.filter.as_ref().and_then(extract_full_text_term) {
        Some(term) => QueryPlan::FullTextPushdown { term },
        None => QueryPlan::FullScan,
    }
}
//#endregion 🔖️Planner

//#region 🔖️Execute
/// 📊️ What `execute` observed while producing a `QueryResult` — the contract's "diagnostics"
/// bullet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QueryDiagnostics {
    pub plan: QueryPlanKind,
    pub rows_scanned: u64,
    pub rows_matched: u64,
    pub rows_returned: u64,
}

/// 📦️ A fully materialized query result: the projected, sorted, paginated rows, plus
/// `QueryDiagnostics`. Convert to a `QueryStream` via `into_stream` for incremental consumption.
#[derive(Debug)]
pub struct QueryResult {
    pub rows: QueryRows,
    pub diagnostics: QueryDiagnostics,
}

/// ▶️ Plans and evaluates `query` against `source`. `fulltext` is only consulted if the
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
/// 🌊️ A `QueryResult`'s rows as an `Iterator`, for callers (`ArtifactHandle::query`'s
/// contract-frozen return type) that want to consume incrementally rather than hold the whole
/// `Vec`. Backed by an already-materialized `Vec::IntoIter` — see `db_state::PMap::iter`'s doc for
/// this crate family's established "eagerly materialize, simple to reason about" precedent; a
/// truly lazy pull-based evaluator is a straightforward future optimization.
pub struct QueryStream {
    pub(super) rows: QueryRows,
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
/// 📡️ A `Query` plus the `Consistency` it should be (re-)evaluated under — what a caller
/// hands to `ArtifactHandle::subscribe`. This crate only owns the diffing law (below); actor-level
/// registration/notification wiring belongs to `db_artifact`.
pub struct LiveQuerySpec {
    pub query: Query,
    pub consistency: Consistency,
}

/// 🔀️ The change between two successive evaluations of a `LiveQuery`'s `Query`: rows newly
/// present, rows no longer present, and rows present in both but with a changed `Value`.
pub struct QueryDiff {
    pub added: QueryRows,
    pub removed: db_io_core::DbIoU64List,
    pub updated: QueryRows,
}

impl Default for QueryDiff {
    fn default() -> Self {
        Self { added: QueryRows::new(), removed: db_io_core::DbIoU64List::new(), updated: QueryRows::new() }
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

pub(super) fn query_value_hash(value: &Value) -> [u8; 32] {
    let mut hash = semio_framework_hash::Hasher::new();
    hash_query_value(value, &mut hash);
    *hash.finalize().as_bytes()
}

/// 📺️ Tracks one live query's last-seen result set so `refresh` can emit a `QueryDiff`
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
    pub spec: LiveQuerySpec,
    pub snapshot: [Option<(RowId, [u8; 32])>; QUERY_ROW_SLOTS],
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

    /// 🔁️ Re-executes `self.spec.query` against `source` and diffs the result against the
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
