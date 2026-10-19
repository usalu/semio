/// ⚙️ Parser and executor laws of the graph DSL: the clause parsers, the WITH, UNWIND and CALL execution, the executor semantics and the dash connector.

// #region 🔖️ParserAndExecutorTests
#[test]
fn parse_delete_set_merge_clauses() {
    block_on_test(async {
        let q = parse("MATCH (a:x) DELETE a").unwrap();
        assert!(matches!(q.clauses[1], Clause::Delete(ref vars) if vars == &vec!["a".to_string()]));
        let q = parse("MATCH (a:x) SET a.name = 'v'").unwrap();
        assert!(matches!(q.clauses[1], Clause::Set(ref items) if items.len() == 1 && items[0].prop == "name"));
        let q = parse("MERGE (a:x)").unwrap();
        assert!(matches!(q.clauses[0], Clause::Merge(_)));
    });
}

#[test]
fn parse_where_and_or_precedence() {
    block_on_test(async {
        let q = parse("MATCH (a:x) WHERE a.p = 1 AND a.q = 2 OR a.r != 3").unwrap();
        let Clause::Where(expr) = &q.clauses[1] else { panic!("expected where") };
        assert!(matches!(expr, Expr::Or(_, _)));
    });
}

// #region 🔖️WithUnwindCallTests
#[test]
fn parse_with_clause() {
    block_on_test(async {
        let q = parse("MATCH (a:x) WITH a, a.name RETURN a").unwrap();
        let Clause::With(items) = &q.clauses[1] else { panic!("expected with") };
        assert_eq!(items.len(), 2);
        assert!(matches!(&items[0], ReturnItem::Var(v) if v == "a"));
        assert!(matches!(&items[1], ReturnItem::Property { var, prop } if var == "a" && prop == "name"));
    });
}

#[test]
fn parse_unwind_clause() {
    block_on_test(async {
        let q = parse("MATCH (a:x) UNWIND a.items AS item RETURN item").unwrap();
        let Clause::Unwind(clause) = &q.clauses[1] else { panic!("expected unwind") };
        assert!(matches!(&clause.source, ReturnItem::Property { var, prop } if var == "a" && prop == "items"));
        assert_eq!(clause.var, "item");
    });
}

#[test]
fn parse_call_clause_with_positional_args() {
    block_on_test(async {
        let q = parse("CALL myProc(1, \"two\", true)").unwrap();
        let Clause::Call(clause) = &q.clauses[0] else { panic!("expected call") };
        assert_eq!(clause.name, "myProc");
        assert_eq!(clause.args, vec![PropertyValue::Number(1.0), PropertyValue::String("two".to_string()), PropertyValue::Bool(true)]);
    });
}

#[test]
fn execute_with_projects_named_vars_and_drops_the_rest() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation)--[:wire]--(b:slider) WITH a RETURN a.name, b.name").unwrap();
        assert!(!result.rows.is_empty());
        for row in &result.rows {
            assert_eq!(row[0], PropertyValue::String("A".to_string()));
            assert_eq!(row[1], PropertyValue::Null, "b was projected out of scope by WITH a, so b.name must be null");
        }
    });
}

#[test]
fn execute_with_where_filters_the_projected_bindings() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:slider) WITH a WHERE a.name = 'B' RETURN a.name").unwrap();
        assert_eq!(result.rows, vec![vec![PropertyValue::String("B".to_string())]]);
    });
}

#[test]
fn execute_with_property_item_keeps_its_source_var_resolvable() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation) WITH a.name RETURN a.name").unwrap();
        assert_eq!(result.rows, vec![vec![PropertyValue::String("A".to_string())]]);
    });
}

#[test]
fn execute_unwind_over_a_property_expression_producing_a_list() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(list_property_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation) UNWIND a.tags AS tag RETURN tag").unwrap();
        assert_eq!(result.rows, vec![vec![PropertyValue::String("x".to_string())], vec![PropertyValue::String("y".to_string())], vec![PropertyValue::String("z".to_string())],]);
    });
}

#[test]
fn execute_unwind_over_an_already_bound_list_value() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(list_property_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation) UNWIND a.matrix AS row UNWIND row AS cell RETURN cell").unwrap();
        assert_eq!(result.rows, vec![vec![PropertyValue::Number(1.0)], vec![PropertyValue::Number(2.0)], vec![PropertyValue::Number(3.0)], vec![PropertyValue::Number(4.0)]]);
    });
}

#[test]
fn execute_unwind_of_an_empty_list_yields_zero_rows() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(list_property_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation) UNWIND a.empty AS e RETURN e").unwrap();
        assert!(result.rows.is_empty());
    });
}

#[test]
fn execute_call_known_procedure_yields_its_registered_column() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(list_property_fixture(), None).unwrap();
        let result = run_query(&graph, "CALL nodeKinds() RETURN kind").unwrap();
        assert_eq!(result.rows, vec![vec![PropertyValue::String("computation".to_string())], vec![PropertyValue::String("slider".to_string())]]);
    });
}

#[test]
fn execute_call_unknown_procedure_reports_a_precise_error() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(list_property_fixture(), None).unwrap();
        let err = run_query(&graph, "CALL bogus()").unwrap_err();
        assert!(matches!(err, GraphDslError::UnknownProcedure(ref name) if name == "bogus"), "got {err:?}");
    });
}

#[test]
fn execute_call_procedure_arity_mismatch_reports_a_precise_error() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(list_property_fixture(), None).unwrap();
        let err = run_query(&graph, "CALL nodeKinds(1)").unwrap_err();
        assert!(matches!(err, GraphDslError::ProcedureArity { ref name, expected: 0, found: 1 } if name == "nodeKinds"), "got {err:?}");
    });
}
// #endregion 🔖️WithUnwindCallTests

#[test]
fn lexer_accepts_both_single_and_double_quoted_strings_and_always_prints_double_quoted() {
    block_on_test(async {
        let single = parse("MATCH (a:x) WHERE a.name = 'alpha' RETURN a").unwrap();
        let double = parse("MATCH (a:x) WHERE a.name = \"alpha\" RETURN a").unwrap();
        assert_eq!(single, double, "single- and double-quoted string literals must parse identically");
        let printed = format("MATCH (a:x) WHERE a.name = 'alpha' RETURN a").unwrap();
        assert!(printed.contains("\"alpha\""), "must always print double-quoted: {printed}");
        assert!(!printed.contains('\''), "must never print single-quoted: {printed}");
    });
}

#[test]
fn parse_unexpected_token_error_has_expected_and_found() {
    block_on_test(async {
        let err = parse("MATCH a:x)").unwrap_err();
        let GraphDslError::UnexpectedToken { expected, found } = err else { panic!("expected UnexpectedToken") };
        assert_eq!(expected, "LParen");
        assert!(found.contains("Ident"));
    });
}

#[test]
fn execute_where_clause_filters_bindings() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:slider) WHERE a.name = 'B' RETURN a.name").unwrap();
        assert_eq!(result.rows, vec![vec![PropertyValue::String("B".into())]]);
    });
}

#[test]
fn execute_and_or_expressions() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let and_result = run_query(&graph, "MATCH (a:slider) WHERE a.name = 'B' AND a.kind = 'slider' RETURN a.name").unwrap();
        assert_eq!(and_result.rows.len(), 1);
        let or_result = run_query(&graph, "MATCH (a:slider) WHERE a.name = 'B' OR a.name = 'C' RETURN a.name").unwrap();
        assert_eq!(or_result.rows.len(), 2);
    });
}

#[test]
fn execute_rejects_mutating_clauses() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        for query in ["CREATE (a:x)", "MATCH (a:x) DELETE a", "MATCH (a:x) SET a.p = 1", "MERGE (a:x)"] {
            let err = run_query(&graph, query).unwrap_err();
            assert!(matches!(err, GraphDslError::UnsupportedMutation), "query {query} should reject mutation");
        }
    });
}

#[test]
fn execute_undirected_edge_matches_both_directions() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let forward = run_query(&graph, "MATCH (a:computation)--[:wire]--(b:slider) RETURN a.name, b.name").unwrap();
        let reverse = run_query(&graph, "MATCH (b:slider)--[:wire]--(a:computation) RETURN a.name, b.name").unwrap();
        assert!(!forward.rows.is_empty());
        assert!(!reverse.rows.is_empty());
    });
}

#[test]
fn execute_multiple_match_patterns_join_bindings() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation), (b:slider) RETURN a.name, b.name").unwrap();
        assert_eq!(result.rows.len(), 2);
    });
}

#[test]
fn execute_returns_graph_kind_when_returning_bound_entities() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation)--[e:wire]--(b:slider) RETURN a, e, b").unwrap();
        assert_eq!(result.kind, QueryResultKind::Graph);
        assert!(result.graph_snapshot_json.is_some());
    });
}

#[test]
fn execute_returns_table_kind_for_property_projection() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation) RETURN a.name").unwrap();
        assert_eq!(result.kind, QueryResultKind::Table);
    });
}

#[test]
fn execute_with_no_return_clause_yields_empty_table() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation)").unwrap();
        assert!(result.columns.is_empty());
        assert!(result.rows.is_empty());
    });
}

#[test]
fn run_query_json_serializes_result() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let json = run_query_json(&graph, "MATCH (a:computation) RETURN a.name").unwrap();
        let value: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&json,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(value["columns"][0], "a.name");
    });
}

#[test]
fn empty_pattern_error_is_reachable_via_pattern_construction() {
    block_on_test(async {
        let pattern = Pattern { nodes: vec![], edge: None };
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let err = match_patterns(&graph, std::slice::from_ref(&pattern)).unwrap_err();
        assert!(matches!(err, GraphDslError::EmptyPattern));
    });
}
// #endregion 🔖️ParserAndExecutorTests

// #region ➖️CypherDashConnector
/// ➖️ A bare `-` is the pattern connector every Cypher-shaped query writes, and it parses to exactly
/// the same `Pattern` as this grammar's own `--` spelling.
///
/// 🐛️ `-` used to fall into the lexer's stray-character bucket, so the ONLY spelling this grammar
/// accepted was `--`. Since `lint`, `complete`, `hover`, `semantic_tokens` and `format` are the
/// language service every consumer of this dialect delegates to, a perfectly runnable query was
/// reported `unexpected character '-'` in the editor: measured live on 2026-09-22 in semio-tech play's
/// `trinity-jack` pane, whose own default query is
/// `MATCH (a:Piece)-[r:Connection]->(b:Piece) …` and whose executor parses that shape in its own
/// green unit tests.
#[test]
fn a_bare_dash_is_the_same_pattern_connector_as_a_double_dash() {
    block_on_test(async {
        let single = parse("MATCH (a:x)-[r:wire]->(b:y) RETURN a.p").unwrap();
        let double = parse("MATCH (a:x)--[r:wire]->(b:y) RETURN a.p").unwrap();
        assert_eq!(single, double, "`-` and `--` must reach the same parsed query");
    });
}

/// ➖️ The undirected tail and the reversed form take a single dash too.
#[test]
fn a_bare_dash_also_spells_the_undirected_and_reversed_connectors() {
    block_on_test(async {
        assert_eq!(parse("MATCH (a:x)-[r:wire]-(b:y) RETURN a.p").unwrap(), parse("MATCH (a:x)--[r:wire]--(b:y) RETURN a.p").unwrap());
        assert_eq!(parse("MATCH (a:x)<-[r:wire]-(b:y) RETURN a.p").unwrap(), parse("MATCH (a:x)<-[r:wire]--(b:y) RETURN a.p").unwrap());
    });
}

/// ➖️ Formatting a single-dash query stays idempotent — whichever spelling the formatter settles on,
/// running it twice must be a fixed point, and the result must still parse.
#[test]
fn formatting_a_single_dash_pattern_is_idempotent_and_reparses() {
    block_on_test(async {
        let once = format("match(a:x)-[r:wire]->(b:y) return a.p").unwrap();
        assert_eq!(format(&once).unwrap(), once);
        parse(&once).unwrap();
    });
}
// #endregion ➖️CypherDashConnector
