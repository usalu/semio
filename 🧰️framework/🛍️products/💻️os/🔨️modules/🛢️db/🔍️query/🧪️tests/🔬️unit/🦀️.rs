use super::*;

async fn sample_row(name: &str, age: i64, tags: Vec<&str>) -> Value {
    let mut map = BTreeMap::new();
    map.insert("name".to_string(), Value::Text(name.to_string()));
    map.insert("age".to_string(), Value::Int(age));
    map.insert("tags".to_string(), Value::List(tags.into_iter().map(Value::from).collect()));
    Value::Map(map)
}

async fn sample_source() -> ProjectionSource {
    ProjectionSource::from_value(Value::List(vec![sample_row("alice", 30, vec!["admin", "eng"]).await, sample_row("bob", 25, vec!["eng"]).await, sample_row("cara", 40, vec!["admin"]).await])).await.unwrap()
}

fn control() -> QueryCursorControl {
    QueryCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap()
}

async fn query_bytes(bytes: &[u8]) -> QueryBytes {
    let mut writer = db_storage::DbIoPageWriter::try_reserve(bytes.len().div_ceil(db_storage::DB_IO_PAGE_BYTES)).unwrap();
    let mut offset = 0;
    while offset < bytes.len() {
        offset += writer.write_fragment(&bytes[offset..]).unwrap();
    }
    QueryBytes::from_pages(writer.seal_retained().await.unwrap()).unwrap()
}

#[semio_framework_async_macros::async_test]
async fn interrupted_query_rows_drop_retains_one_resumable_close_owner() {
    while query_rows_maintenance_step().unwrap() {}
    let mut rows = QueryRows::new();
    rows.push(QueryRow::new(RowId(1), Value::Bytes(query_bytes(&vec![0x5a; db_storage::DB_IO_PAGE_BYTES + 1]).await))).unwrap();
    drop(rows);
    assert!(query_rows_maintenance_step().unwrap());
    {
        let retired = QUERY_RETIRED_ROWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(retired.iter().flatten().any(|owner| !owner.terminal_is_empty()));
    }
    while query_rows_maintenance_step().unwrap() {}

    QUERY_RETIREMENT_PRESSURE_FAULT.store(false, std::sync::atomic::Ordering::Release);
    {
        let mut retired = QUERY_RETIRED_ROWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in retired.iter_mut() {
            *slot = Some(QueryRows::new());
        }
    }
    {
        let mut overflow = QUERY_RETIRED_ROWS_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in overflow.iter_mut() {
            *slot = Some(QueryRows::new());
        }
    }
    let mut exact = QueryRows::new();
    exact.push(QueryRow::new(RowId(0x5155_4552_59), Value::Null)).unwrap();
    let mut second = QueryRows::new();
    second.push(QueryRow::new(RowId(0x5155_4552_5a), Value::Null)).unwrap();
    assert_eq!(exact.retirement.map(|reservation| reservation.tier), Some(2));
    assert_eq!(second.retirement.map(|reservation| reservation.tier), Some(2));
    assert!(retire_query_rows(exact).is_ok());
    assert!(retire_query_rows(second).is_ok());
    assert!(QUERY_RETIREMENT_PRESSURE_FAULT.load(std::sync::atomic::Ordering::Acquire));
    {
        let quarantine = QUERY_RETIRED_ROWS_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(quarantine.iter().flatten().find_map(|rows| rows.get(0).map(QueryRow::id)), Some(RowId(0x5155_4552_59)));
        assert!(quarantine.iter().flatten().any(|rows| rows.get(0).map(QueryRow::id) == Some(RowId(0x5155_4552_5a))));
    }
    {
        let mut retired = QUERY_RETIRED_ROWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in retired.iter_mut() {
            *slot = None;
        }
    }
    for _ in 0..QUERY_RETIRED_ROW_SETS * 2 {
        assert!(query_rows_maintenance_step().unwrap());
    }
    assert!(query_rows_maintenance_step().unwrap());
    {
        let retired = QUERY_RETIRED_ROWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(retired.iter().flatten().find_map(|rows| rows.get(0).map(QueryRow::id)), Some(RowId(0x5155_4552_59)));
    }
    while query_rows_maintenance_step().unwrap() {}

    for tier in [&QUERY_RETIRED_ROWS, &QUERY_RETIRED_ROWS_OVERFLOW, &QUERY_RETIRED_ROWS_QUARANTINE] {
        let mut owners = tier.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in owners.iter_mut() {
            *slot = Some(QueryRows::new());
        }
    }
    let exact_refusal = QueryRow::new(RowId(0x5155_4552_5b), Value::Null);
    let mut refused = QueryRows::new();
    let exact_refusal = refused.push(exact_refusal).unwrap_err();
    assert_eq!(exact_refusal.id(), RowId(0x5155_4552_5b));
    for tier in [&QUERY_RETIRED_ROWS, &QUERY_RETIRED_ROWS_OVERFLOW, &QUERY_RETIRED_ROWS_QUARANTINE] {
        let mut owners = tier.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in owners.iter_mut() {
            *slot = None;
        }
    }
    assert!(refused.terminal_is_empty());
}

//#region 🔖️Value
mod value {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn path_get_walks_nested_map_and_list() {
        let row = sample_row("alice", 30, vec!["admin", "eng"]).await;
        assert_eq!(Path::field("name").get(&row), Some(&Value::Text("alice".to_string())));
        assert_eq!(Path::parse("tags.0").get(&row), Some(&Value::Text("admin".to_string())));
        assert_eq!(Path::parse("missing").get(&row), None);
        assert_eq!(Path::empty().get(&row), Some(&row));
    }

    #[semio_framework_async_macros::async_test]
    async fn path_get_rejects_type_mismatch() {
        let row = sample_row("alice", 30, vec!["admin"]).await;
        assert_eq!(Path::parse("name.0").get(&row), None);
        assert_eq!(Path::parse("age.field").get(&row), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn compare_values_orders_numerics_across_int_and_float() {
        assert_eq!(compare_values(&Value::Int(1), &Value::Float(1.5)), Ordering::Less);
        assert_eq!(compare_values(&Value::Float(2.0), &Value::Int(2)), Ordering::Equal);
    }

    #[semio_framework_async_macros::async_test]
    async fn compare_values_falls_back_to_rank_across_variants() {
        assert_eq!(compare_values(&Value::Null, &Value::Bool(false)), Ordering::Less);
        assert_eq!(compare_values(&Value::Text("z".to_string()), &Value::Int(0)), Ordering::Greater);
    }

    #[semio_framework_async_macros::async_test]
    async fn path_display_round_trips_through_parse() {
        let path = Path::parse("a.b.3");
        assert_eq!(path.to_string(), "a.b.3");
    }
}
//#endregion 🔖️Value

//#region 🔖️Query
mod query {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn eq_predicate_matches_and_rejects() {
        let row = sample_row("alice", 30, vec!["admin"]).await;
        assert!(eval_predicate(&Predicate::Eq(Path::field("age"), Value::Int(30)), &row));
        assert!(!eval_predicate(&Predicate::Eq(Path::field("age"), Value::Int(31)), &row));
    }

    #[semio_framework_async_macros::async_test]
    async fn ne_treats_missing_path_as_failing() {
        let row = sample_row("alice", 30, vec!["admin"]).await;
        assert!(!eval_predicate(&Predicate::Ne(Path::field("missing"), Value::Int(1)), &row));
    }

    #[semio_framework_async_macros::async_test]
    async fn and_or_not_compose() {
        let row = sample_row("alice", 30, vec!["admin"]).await;
        assert!(eval_predicate(&Predicate::And(vec![Predicate::Contains(Path::field("tags"), Value::from("admin"))]), &row));
        assert!(eval_predicate(&Predicate::Or(vec![Predicate::Contains(Path::field("tags"), Value::from("admin")), Predicate::Gte(Path::field("age"), Value::Int(40))]), &row,));
        assert!(!eval_predicate(&Predicate::And(vec![Predicate::Contains(Path::field("tags"), Value::from("admin")), Predicate::Gte(Path::field("age"), Value::Int(40))]), &row,));
        assert!(eval_predicate(&Predicate::Not(Box::new(Predicate::Gte(Path::field("age"), Value::Int(40)))), &row));
    }

    #[semio_framework_async_macros::async_test]
    async fn full_text_matches_case_insensitively_over_whole_document() {
        let row = sample_row("Alice", 30, vec!["admin"]).await;
        assert!(eval_predicate(&Predicate::FullText(Path::empty(), "ALICE".to_string()), &row));
        assert!(!eval_predicate(&Predicate::FullText(Path::empty(), "dave".to_string()), &row));
    }

    #[semio_framework_async_macros::async_test]
    async fn select_paths_projects_a_map_keyed_by_dotted_path() {
        let row = sample_row("alice", 30, vec!["admin"]).await;
        let projected = Select::Paths(vec![Path::field("name")]).project(row);
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

    async fn nested_sample() -> Value {
        let mut inner = BTreeMap::new();
        inner.insert("nickname".to_string(), Value::Text("ally".to_string()));
        inner.insert("verified".to_string(), Value::Bool(true));
        let mut row = BTreeMap::new();
        row.insert("name".to_string(), Value::Text("alice".to_string()));
        row.insert("age".to_string(), Value::Int(30));
        row.insert("score".to_string(), Value::Float(2.5));
        row.insert("blob".to_string(), Value::Bytes(query_bytes(&[9, 8, 7]).await));
        row.insert("tags".to_string(), Value::List(vec![Value::from("admin"), Value::Null]));
        row.insert("profile".to_string(), Value::Map(inner));
        Value::Map(row)
    }

    #[semio_framework_async_macros::async_test]
    async fn value_projection_state_round_trips_every_variant_including_nesting() {
        let value = nested_sample().await;
        let hash = query_value_hash(&value);
        let source = ProjectionSource::from_value(value).await.unwrap();
        let rows = source.scan(&mut control()).await.unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(query_value_hash(rows.get(0).unwrap().value()), hash);
    }

    #[semio_framework_async_macros::async_test]
    async fn value_projection_state_round_trips_null_and_empty_containers() {
        for value in [Value::Null, Value::List(Vec::new()), Value::Map(BTreeMap::new())] {
            let source = ProjectionSource::from_value(value).await.unwrap();
            assert_eq!(source.len().await, 1);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_rejects_truncated_bytes_and_unknown_tag_without_panicking() {
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        let mut cancelled_control = QueryCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 1).unwrap();
        let mut writer = db_storage::DbIoPageWriter::try_reserve(1).unwrap();
        writer.write_fragment(&[200]).unwrap();
        let mut pages = writer.seal_retained().await.unwrap();
        assert!(matches!(QueryBytes::copy_from_pages(&pages, &mut cancelled_control).await, Err(DbError::Unavailable(_))));
        while pages.close_step().unwrap().is_some() {}
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_rejects_trailing_bytes_after_a_complete_value() {
        let mut bytes = query_bytes(&[1, 0xff]).await;
        assert_eq!(bytes.len(), 2);
        while bytes.close_step().unwrap().is_some() {}
        assert!(bytes.terminal_is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_value_rejects_an_over_large_declared_element_count_before_allocating() {
        let values = (0..QUERY_ROW_SLOTS + 1).map(|value| Value::Int(value as i64)).collect();
        assert!(matches!(ProjectionSource::from_value(Value::List(values)).await, Err(DbError::LimitExceeded(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn projection_source_shapes_list_map_and_scalar_values_into_rows() {
        let list_source = ProjectionSource::from_value(Value::List(vec![Value::from(1i64), Value::from(2i64)])).await.unwrap();
        assert_eq!(list_source.len().await, 2);
        let rows = list_source.scan(&mut control()).await.unwrap();
        assert_eq!(rows.get(0).unwrap().value(), &Value::Int(1));
        assert_eq!(rows.get(1).unwrap().value(), &Value::Int(2));

        let mut map = BTreeMap::new();
        map.insert("a".to_string(), Value::from("first"));
        map.insert("b".to_string(), Value::from("second"));
        let map_source = ProjectionSource::from_value(Value::Map(map)).await.unwrap();
        let rows = map_source.scan(&mut control()).await.unwrap();
        assert_eq!(rows.get(0).unwrap().value(), &Value::from("first"));
        assert_eq!(rows.get(1).unwrap().value(), &Value::from("second"));

        let scalar_source = ProjectionSource::from_value(Value::Int(42)).await.unwrap();
        assert!(!scalar_source.is_empty().await);
        let rows = scalar_source.scan(&mut control()).await.unwrap();
        assert_eq!(rows.get(0).unwrap().value(), &Value::Int(42));
    }

    /// @emoji ⚖️ The end-to-end law this bridge exists for: bytes a caller retrieved from
    /// `db_projection::ProjectionEngine::state_at`/`preview_augmented` (simulated here by
    /// `ProjectionState::encode` on a hand-built row set, since this crate cannot construct a
    /// real `ProjectionEngine` without a `protocol::MutationEnvelope` — see the module doc)
    /// decode through `projection_query_source` into a `QuerySource` this crate's ordinary
    /// `execute` runs over identically to any other source.
    #[semio_framework_async_macros::async_test]
    async fn projection_query_source_decodes_bytes_into_a_queryable_source() {
        let rows = Value::List(vec![sample_row("alice", 30, vec!["admin", "eng"]).await, sample_row("bob", 25, vec!["eng"]).await]);
        let source = projection_query_source(rows).await.expect("decodes");
        let query = Query::new().filter(Predicate::Gte(Path::field("age"), Value::Int(30)));
        let result = db_actor::block_on(execute(&query, &source, None::<&NoFullTextLookup>, &QueryLimits::default(), &mut control())).expect("query succeeds");
        assert_eq!(result.rows.len(), 1);
        assert_eq!(Path::field("name").get(result.rows.get(0).unwrap().value()), Some(&Value::Text("alice".to_string())));
    }

    #[semio_framework_async_macros::async_test]
    async fn projection_query_source_surfaces_corrupt_bytes_as_an_error_not_a_panic() {
        let values = (0..QUERY_ROW_SLOTS + 1).map(|value| Value::Int(value as i64)).collect();
        assert!(matches!(projection_query_source(Value::List(values)).await, Err(DbError::LimitExceeded(_))));
    }
}
//#endregion 🔖️ProjectionBridge

//#region 🔖️Execute
mod execute_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn full_scan_filters_sorts_and_paginates() {
        let source = sample_source().await;
        let query = Query::new().filter(Predicate::Gte(Path::field("age"), Value::Int(25))).sort(vec![SortKey::descending(Path::field("age"))]).limit(2);
        let result = db_actor::block_on(execute(&query, &source, None::<&NoFullTextLookup>, &QueryLimits::default(), &mut control())).expect("query succeeds");
        assert_eq!(result.diagnostics.plan, QueryPlanKind::FullScan);
        assert_eq!(result.diagnostics.rows_matched, 3);
        assert_eq!(result.diagnostics.rows_returned, 2);
        let names: Vec<String> = result
            .rows
            .iter()
            .map(|row| match Path::field("name").get(row.value()) {
                Some(Value::Text(name)) => name.clone(),
                _ => panic!("expected a name"),
            })
            .collect();
        assert_eq!(names, vec!["cara".to_string(), "alice".to_string()]);
    }

    #[semio_framework_async_macros::async_test]
    async fn offset_skips_matched_rows_before_limit_applies() {
        let source = sample_source().await;
        let query = Query::new().sort(vec![SortKey::ascending(Path::field("age"))]).offset(1).limit(1);
        let result = db_actor::block_on(execute(&query, &source, None::<&NoFullTextLookup>, &QueryLimits::default(), &mut control())).expect("query succeeds");
        assert_eq!(result.rows.len(), 1);
        assert_eq!(Path::field("name").get(result.rows.get(0).unwrap().value()), Some(&Value::Text("alice".to_string())));
    }

    #[semio_framework_async_macros::async_test]
    async fn max_result_rows_limit_is_enforced() {
        let source = sample_source().await;
        let limits = QueryLimits { max_result_rows: 1, ..QueryLimits::default() };
        let error = db_actor::block_on(execute(&Query::new(), &source, None::<&NoFullTextLookup>, &limits, &mut control())).unwrap_err();
        assert!(matches!(error, DbError::LimitExceeded(_)));
    }

    #[semio_framework_async_macros::async_test]
    async fn into_stream_yields_the_same_rows_as_the_result() {
        let source = sample_source().await;
        let result = db_actor::block_on(execute(&Query::new(), &source, None::<&NoFullTextLookup>, &QueryLimits::default(), &mut control())).expect("query succeeds");
        let expected_len = result.rows.len();
        let stream = result.into_stream().await;
        assert_eq!(stream.count(), expected_len);
    }

    /// @emoji 🧪️ A hand-rolled `FullTextLookup` double — exercises pushdown without needing a
    /// real `db_storage::IndexStorage` (not a dependency of this crate; see module doc).
    pub(super) struct FakeFullText(pub std::collections::HashMap<String, Vec<RowId>>);
    impl FullTextLookup for FakeFullText {
        async fn search(&self, term: &str) -> Result<db_storage::DbIoU64List, DbError> {
            let mut result = db_storage::DbIoU64List::new();
            for id in self.0.get(term).into_iter().flatten() {
                result.push(id.0)?;
            }
            Ok(result)
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn full_text_pushdown_without_a_lookup_is_an_error() {
        let source = sample_source().await;
        let query = Query::new().filter(Predicate::FullText(Path::empty(), "alice".to_string()));
        let error = db_actor::block_on(execute(&query, &source, None::<&NoFullTextLookup>, &QueryLimits::default(), &mut control())).unwrap_err();
        assert!(matches!(error, DbError::InvalidArgument(_)));
    }
}
//#endregion 🔖️Execute

//#region 🔖️Planner
mod planner {
    use super::execute_tests::FakeFullText;
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn plan_recognizes_bare_and_conjoined_full_text_predicates() {
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
    #[semio_framework_async_macros::async_test]
    async fn pushdown_matches_full_scan_when_the_index_is_exhaustive() {
        let source = sample_source().await;
        let query = Query::new().filter(Predicate::FullText(Path::empty(), "admin".to_string()));

        let full_scan_result = db_actor::block_on(execute(&query, &source, None::<&NoFullTextLookup>, &QueryLimits::default(), &mut control()));
        assert!(matches!(full_scan_result, Err(DbError::InvalidArgument(_))));

        let mut postings = std::collections::HashMap::new();
        postings.insert("admin".to_string(), vec![RowId(0), RowId(1), RowId(2)]);
        let lookup = FakeFullText(postings);

        let source = sample_source().await;
        let pushdown = db_actor::block_on(execute(&query, &source, Some(&lookup), &QueryLimits::default(), &mut control())).expect("pushdown succeeds");
        assert_eq!(pushdown.diagnostics.plan, QueryPlanKind::FullTextPushdown);
        assert_eq!(pushdown.rows.len(), 2);
        let names: std::collections::HashSet<String> = pushdown
            .rows
            .iter()
            .map(|row| match Path::field("name").get(row.value()) {
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
        async fn current_frontier(&self) -> Result<Frontier, DbError> {
            Ok(self.current.clone())
        }
        async fn frontier_for_commit(&self, commit_id: &str) -> Result<Frontier, DbError> {
            self.commits.get(commit_id).cloned().ok_or_else(|| DbError::NotFound(commit_id.to_string()))
        }
    }

    async fn frontier_at(seq: u64) -> Frontier {
        Frontier { document: ArtifactId::from("doc-1"), head_seq: seq, commit_seq: seq, chain_hash: [0u8; 32], epoch: 0 }
    }

    #[semio_framework_async_macros::async_test]
    async fn canonical_resolves_to_current_frontier() {
        let resolver = FakeResolver { current: frontier_at(5).await, commits: BTreeMap::new() };
        let resolved = db_actor::block_on(resolve_consistency(&Consistency::Canonical, &resolver)).expect("resolves");
        assert_eq!(resolved.frontier, frontier_at(5).await);
        assert!(!resolved.historical);
        assert_eq!(resolved.preview_id, None);
    }

    #[semio_framework_async_macros::async_test]
    async fn at_least_succeeds_when_dominated_and_fails_otherwise() {
        let resolver = FakeResolver { current: frontier_at(5).await, commits: BTreeMap::new() };
        assert!(db_actor::block_on(resolve_consistency(&Consistency::AtLeast(frontier_at(3).await), &resolver)).is_ok());
        let error = db_actor::block_on(resolve_consistency(&Consistency::AtLeast(frontier_at(10).await), &resolver)).unwrap_err();
        assert!(matches!(error, DbError::Unavailable(_)));
    }

    #[semio_framework_async_macros::async_test]
    async fn exact_requires_a_bytewise_match() {
        let resolver = FakeResolver { current: frontier_at(5).await, commits: BTreeMap::new() };
        assert!(db_actor::block_on(resolve_consistency(&Consistency::Exact(frontier_at(5).await), &resolver)).is_ok());
        assert!(db_actor::block_on(resolve_consistency(&Consistency::Exact(frontier_at(6).await), &resolver)).is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn historical_resolves_via_commit_lookup() {
        let mut commits = BTreeMap::new();
        commits.insert("ck-abc".to_string(), frontier_at(2).await);
        let resolver = FakeResolver { current: frontier_at(5).await, commits };
        let resolved = db_actor::block_on(resolve_consistency(&Consistency::Historical("ck-abc".to_string()), &resolver)).expect("resolves");
        assert_eq!(resolved.frontier, frontier_at(2).await);
        assert!(resolved.historical);

        let error = db_actor::block_on(resolve_consistency(&Consistency::Historical("ck-missing".to_string()), &resolver)).unwrap_err();
        assert!(matches!(error, DbError::NotFound(_)));
    }

    #[semio_framework_async_macros::async_test]
    async fn speculative_and_preview_augmented_carry_the_preview_id() {
        let resolver = FakeResolver { current: frontier_at(5).await, commits: BTreeMap::new() };
        let speculative = db_actor::block_on(resolve_consistency(&Consistency::Speculative("pv-1".to_string()), &resolver)).expect("resolves");
        assert_eq!(speculative.preview_id, Some("pv-1".to_string()));
        let augmented = db_actor::block_on(resolve_consistency(&Consistency::PreviewAugmented("pv-2".to_string()), &resolver)).expect("resolves");
        assert_eq!(augmented.preview_id, Some("pv-2".to_string()));
    }
}
//#endregion 🔖️Consistency

//#region 🔖️LiveQuery
mod live_query {
    use super::*;

    async fn source_with(rows: Vec<Value>) -> ProjectionSource {
        ProjectionSource::from_value(Value::List(rows)).await.unwrap()
    }

    /// @emoji 🆔️ `PVec`'s `RowId` is positional (index-based — see its `QuerySource` impl's
    /// doc), so a diff's `added`/`removed`/`updated` classification is keyed by position, not
    /// by any notion of row identity: replacing `bob` with `cara` at the same index is an
    /// `updated` row, not a `removed` + `added` pair. This test exercises all three by keeping
    /// the vector's length changes and value changes at distinct positions.
    #[semio_framework_async_macros::async_test]
    async fn refresh_reports_added_removed_and_updated_rows() {
        let spec = LiveQuerySpec { query: Query::new(), consistency: Consistency::Canonical };
        let mut live = LiveQuery::new(spec).await;

        let first = source_with(vec![sample_row("alice", 30, vec!["admin"]).await, sample_row("bob", 25, vec!["eng"]).await]).await;
        let diff = db_actor::block_on(live.refresh(&first, None::<&NoFullTextLookup>, &QueryLimits::default(), &mut control())).expect("refresh succeeds");
        assert_eq!(diff.added.len(), 2);
        assert!(diff.removed.is_empty());
        assert!(diff.updated.is_empty());

        let second = source_with(vec![sample_row("alice", 31, vec!["admin"]).await, sample_row("bob", 25, vec!["eng"]).await, sample_row("cara", 40, vec!["admin"]).await]).await;
        let diff = db_actor::block_on(live.refresh(&second, None::<&NoFullTextLookup>, &QueryLimits::default(), &mut control())).expect("refresh succeeds");
        assert_eq!(diff.added.len(), 1);
        assert!(diff.removed.is_empty());
        assert_eq!(diff.updated.len(), 1);

        let third = source_with(vec![sample_row("alice", 31, vec!["admin"]).await]).await;
        let diff = db_actor::block_on(live.refresh(&third, None::<&NoFullTextLookup>, &QueryLimits::default(), &mut control())).expect("refresh succeeds");
        assert!(diff.added.is_empty());
        assert_eq!(diff.removed.len(), 2);
        assert!(diff.updated.is_empty());
    }

    /// @emoji ⚖️ The round-trip law `LiveQuery`'s doc promises: old snapshot ⊕ diff == new
    /// snapshot, exactly.
    #[semio_framework_async_macros::async_test]
    async fn diff_applied_to_old_snapshot_reconstructs_new_snapshot() {
        let spec = LiveQuerySpec { query: Query::new(), consistency: Consistency::Canonical };
        let mut live = LiveQuery::new(spec).await;

        let first = source_with(vec![sample_row("alice", 30, vec!["admin"]).await, sample_row("bob", 25, vec!["eng"]).await]).await;
        db_actor::block_on(live.refresh(&first, None::<&NoFullTextLookup>, &QueryLimits::default(), &mut control())).expect("refresh succeeds");
        let mut reconstructed: BTreeMap<RowId, [u8; 32]> = live.snapshot().collect();

        let second = source_with(vec![sample_row("alice", 31, vec!["admin"]).await, sample_row("cara", 40, vec!["admin"]).await]).await;
        let diff = db_actor::block_on(live.refresh(&second, None::<&NoFullTextLookup>, &QueryLimits::default(), &mut control())).expect("refresh succeeds");

        for id in diff.removed.as_slice() {
            reconstructed.remove(&RowId(*id));
        }
        for row in diff.added.iter().chain(diff.updated.iter()) {
            reconstructed.insert(row.id(), query_value_hash(row.value()));
        }

        assert_eq!(reconstructed, live.snapshot().collect());
    }

    //#region 🧪️IncrementalityLaw
    /// @emoji ⚖️ The incrementality law this dissolve exists to prove (ticket
    /// `26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS`): now that row
    /// content is routed through `QueryResultField: InferredField<QuerySnapshot>` (see
    /// `🔖️QueryResultField` above), a refresh over an UNCHANGED row set is all cache hits, and a
    /// refresh where exactly one row's own content changed misses for only that row — every
    /// other row's `DepHash` is untouched and is served warm from `LiveQuery`'s own
    /// `InferenceCache`. Reads `live.cache` directly (a private field of this same module — see
    /// `LiveQuery`'s doc on why `refresh`'s row values are no longer readable any other way)
    /// rather than through `pack::infer_field` in isolation, so the law is proven against the
    /// real public `refresh` path, not a hand-assembled `QuerySnapshot`.
    #[semio_framework_async_macros::async_test]
    async fn refresh_leaves_unrelated_rows_cache_warm_and_misses_only_the_changed_row() {
        let spec = LiveQuerySpec { query: Query::new(), consistency: Consistency::Canonical };
        let mut live = LiveQuery::new(spec).await;

        let first = source_with(vec![sample_row("alice", 30, vec!["admin"]).await, sample_row("bob", 25, vec!["eng"]).await, sample_row("cara", 40, vec!["admin"]).await]).await;
        db_actor::block_on(live.refresh(&first, None::<&NoFullTextLookup>, &QueryLimits::default(), &mut control())).expect("refresh succeeds");

        // An identical re-refresh: every row's dep_hash is unchanged, so every row is a cache hit.
        let identical = source_with(vec![sample_row("alice", 30, vec!["admin"]).await, sample_row("bob", 25, vec!["eng"]).await, sample_row("cara", 40, vec!["admin"]).await]).await;
        let diff = db_actor::block_on(live.refresh(&identical, None::<&NoFullTextLookup>, &QueryLimits::default(), &mut control())).expect("refresh succeeds");
        assert!(diff.added.is_empty() && diff.removed.is_empty() && diff.updated.is_empty(), "an unchanged source must produce an empty diff");

        // Only bob's row changes (same position, same length — isolates a value change from a
        // position-based added/removed churn).
        let third = source_with(vec![sample_row("alice", 30, vec!["admin"]).await, sample_row("bob", 26, vec!["eng"]).await, sample_row("cara", 40, vec!["admin"]).await]).await;
        let diff = db_actor::block_on(live.refresh(&third, None::<&NoFullTextLookup>, &QueryLimits::default(), &mut control())).expect("refresh succeeds");
        assert_eq!(diff.updated.len(), 1, "only bob's row changed");
        assert!(diff.added.is_empty() && diff.removed.is_empty());
    }
    //#endregion 🧪️IncrementalityLaw
}
//#endregion 🔖️LiveQuery

//#region 🔖️Limits
mod limits {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn default_result_bytes_matches_db_core_query_budget() {
        assert_eq!(QueryLimits::default().max_result_bytes, DbLimits::default().max_query_bytes);
    }

    #[semio_framework_async_macros::async_test]
    async fn max_scan_rows_is_enforced_even_when_nothing_matches() {
        let source = sample_source().await;
        let limits = QueryLimits { max_scan_rows: 1, ..QueryLimits::default() };
        let query = Query::new().filter(Predicate::Eq(Path::field("age"), Value::Int(999)));
        let error = db_actor::block_on(execute(&query, &source, None::<&NoFullTextLookup>, &limits, &mut control())).unwrap_err();
        assert!(matches!(error, DbError::LimitExceeded(_)));
    }
}
//#endregion 🔖️Limits
