use crate::JackWorkingScene;
use super::*;
use crate::ast::QueryResultKind;
use crate::language_service::{complete, format as format_source, hover, lint, semantic_tokens};
use crate::lexer::{lex, tokenize, Token, TokenClass};
use crate::{Camera, JackSnapshot, Manifest};

/// 🧸️ The retained content child of a standalone snapshot — what the run-query job reads from its child view.
fn content_of(snapshot: &JackSnapshot) -> semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot {
    crate::jack_content_for_handle(&snapshot.content).expect("retained content child").snapshot().clone()
}

fn mini_graph() -> Graph {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪜️resumable-query/🔣️.json")).unwrap();
    let graph = &fixture["graph"];
    let nodes = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(graph["nodes"].clone())).unwrap();
    let edges = semio_framework_value::FromValue::from_value(semio_framework_value::DslValue::from(graph["edges"].clone())).unwrap();
    let fixture = JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), graph["name"].as_str().unwrap().into(), Some(graph["manifestId"].as_str().unwrap().into()), Manifest::nakagin_default(), Camera::default(), JackWorkingScene { nodes: nodes, edges: edges }, Some(graph["rootNodeId"].as_str().unwrap().into()));
    Graph::from_snapshot(fixture).unwrap()
}

#[semio_framework_async_macros::async_test]
async fn parse_match_return() {
    let q = parse("MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a.name, b.name").unwrap();
    assert_eq!(q.clauses.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn run_match_return() {
    let mut g = mini_graph();
    let result = run(&mut g, "MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a.name, b.name").unwrap();
    assert_eq!(result.kind, QueryResultKind::Table);
    assert_eq!(result.rows.len(), 1);
    assert_eq!(result.rows[0][0], PropertyValue::String("core".into()));
}

#[semio_framework_async_macros::async_test]
async fn run_match_return_graph() {
    let mut g = mini_graph();
    let result = run(&mut g, "MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a, r, b").unwrap();
    assert_eq!(result.kind, QueryResultKind::Graph);
    let fixture = result.graph_fixture.expect("graph fixture");
    assert_eq!(fixture.nodes().expect("valid retained Jack child").len(), 2);
    assert_eq!(fixture.edges().expect("valid retained Jack child").len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn run_create() {
    let mut g = mini_graph();
    run(&mut g, "CREATE (n:Piece)").unwrap();
    assert_eq!(g.nodes.len(), 3);
}

#[semio_framework_async_macros::async_test]
async fn run_set() {
    let mut g = mini_graph();
    run(&mut g, "MATCH (a:Piece) WHERE a.name = 'core' SET a.label = 'root-core'").unwrap();
    let node = g.node("root").unwrap();
    assert_eq!(node.properties.get("label"), Some(&PropertyValue::String("root-core".into())));
}

#[semio_framework_async_macros::async_test]
async fn tokenize_keywords_and_strings() {
    let spans = tokenize("MATCH (a:Piece) WHERE a.name = 'core'");
    assert!(spans.iter().any(|s| s.class == TokenClass::Keyword && s.start == 0));
    assert!(spans.iter().any(|s| s.class == TokenClass::String));
}

#[semio_framework_async_macros::async_test]
async fn tokenize_unterminated_string_is_error() {
    let spans = tokenize("MATCH (a:Piece) WHERE a.name = 'core");
    assert!(spans.iter().any(|s| s.class == TokenClass::Error));
}

#[semio_framework_async_macros::async_test]
async fn complete_clause_keywords() {
    let g = mini_graph();
    let items = complete(&g, "MAT", 3);
    assert!(items.iter().any(|row| row.label == "MATCH"));
}

#[semio_framework_async_macros::async_test]
async fn complete_node_kinds_after_colon() {
    let g = mini_graph();
    let items = complete(&g, "MATCH (a:P", 11);
    assert!(items.iter().any(|row| row.label == "Piece"));
}

#[semio_framework_async_macros::async_test]
async fn complete_properties_after_dot() {
    let g = mini_graph();
    let items = complete(&g, "MATCH (a:Piece) WHERE a.n", 25);
    assert!(items.iter().any(|row| row.label == "name"));
}

#[semio_framework_async_macros::async_test]
async fn complete_bound_variables() {
    let g = mini_graph();
    let items = complete(&g, "MATCH (a:Piece) RETURN a", 24);
    assert!(items.iter().any(|row| row.label == "a"));
}

#[semio_framework_async_macros::async_test]
async fn lint_unterminated_string() {
    let g = mini_graph();
    let diags = lint(&g, "MATCH (a:Piece) WHERE a.name = 'core");
    assert!(diags.iter().any(|d| d.code.as_deref() == Some("jack/unterminated-string")));
}

#[semio_framework_async_macros::async_test]
async fn lint_unbound_variable() {
    let g = mini_graph();
    let diags = lint(&g, "RETURN a.name");
    assert!(diags.iter().any(|d| d.code.as_deref() == Some("jack/unbound-variable")));
}

#[semio_framework_async_macros::async_test]
async fn format_is_idempotent() {
    let source = "MATCH (a:Piece)--[r:Connection]->(b:Piece) RETURN a.name, b.name";
    let once = format_source(source).unwrap();
    let twice = format_source(&once).unwrap();
    assert_eq!(once, twice);
    assert!(once.contains("MATCH"));
    assert!(once.contains('\n'));
}

#[semio_framework_async_macros::async_test]
async fn hover_keyword() {
    let g = mini_graph();
    let info = hover(&g, "MATCH (a:Piece) RETURN a.name", 2).unwrap();
    assert!(info.contents.contains("MATCH"));
}

#[semio_framework_async_macros::async_test]
async fn semantic_tokens_cover_keywords() {
    let tokens = semantic_tokens("MATCH (a:Piece) RETURN a.name");
    assert!(tokens.iter().any(|t| t.class == "keyword"));
    assert!(tokens.iter().any(|t| t.class == "ident"));
}

#[semio_framework_async_macros::async_test]
async fn run_create_edge() {
    let mut g = mini_graph();
    while g.nodes.len() < 9 {
        run(&mut g, "CREATE (n:Piece)").unwrap();
    }
    run(&mut g, "CREATE (x:Piece)-[:Connection]->(y:Piece)").unwrap();
    assert_eq!(g.nodes.len(), 11);
    assert_eq!(g.edges.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn run_delete() {
    let mut g = mini_graph();
    run(&mut g, "MATCH (n:Piece) WHERE n.name = 'capsule' DELETE n").unwrap();
    assert_eq!(g.nodes.len(), 1);
    assert_eq!(g.edges.len(), 0);
}

#[semio_framework_async_macros::async_test]
async fn run_merge_noop_when_pattern_exists() {
    let mut g = mini_graph();
    run(&mut g, "MERGE (a:Piece)-[:Connection]->(b:Piece)").unwrap();
    assert_eq!(g.nodes.len(), 2);
    assert_eq!(g.edges.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn run_merge_creates_disconnected_pattern() {
    let mut g = mini_graph();
    g.edges.clear();
    run(&mut g, "MERGE (x:Piece)-[:Connection]->(y:Piece)").unwrap();
    assert_eq!(g.nodes.len(), 4);
    assert_eq!(g.edges.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn lex_not_equal() {
    let tokens = lex("WHERE a.name != 'core'").unwrap();
    assert!(tokens.iter().any(|t| matches!(t, Token::Ne)));
}

#[semio_framework_async_macros::async_test]
async fn query_ownership_resumable_matches_neutral_results_and_single_mutation_publication() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪜️resumable-query/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let graph = if case["inlineManifest"] == true {
            let mut snapshot = mini_graph().to_snapshot();
            snapshot.manifest_id = None;
            Graph::from_snapshot(snapshot).unwrap()
        } else {
            mini_graph()
        };
        let query = parse(case["query"].as_str().unwrap()).unwrap();
        let expected = execute(&graph, &query).unwrap();
        let snapshot = graph.to_snapshot();
        let content = content_of(&snapshot);
        let mut preparation = QueryExecutionPreparation::new(query);
        let mut preparation_turns = 0;
        let mut execution = loop {
            preparation_turns += 1;
            assert!(preparation_turns < 10_000);
            match preparation.step(&snapshot, &content, 4096).expect("retained query preparation") {
                QueryPreparationStep::Pending => {}
                QueryPreparationStep::Complete(execution) => break execution,
            }
        };
        assert!(preparation.terminal_is_empty());
        let mut steps = 0;
        let actual = loop {
            steps += 1;
            assert!(steps < 10_000);
            if let Some(result) = execution.step().unwrap() {
                break result;
            }
        };
        assert_eq!(actual.0.kind, expected.0.kind);
        assert_eq!(actual.0.columns, expected.0.columns);
        assert_eq!(actual.0.rows, expected.0.rows);
        assert_eq!(actual.1, expected.1);
        let graph_value = |snapshot: &Option<Box<JackSnapshot>>| snapshot.as_ref().map(|snapshot| serde_json::from_str::<serde_json::Value>(&snapshot.to_json().expect("materialized graph JSON")).expect("reference JSON"));
        assert_eq!(graph_value(&actual.0.graph_fixture), graph_value(&expected.0.graph_fixture));
        let packed = semio_framework_pack_json::to_json_string(&actual.0);
        assert!(packed.len() <= 1_048_576, "retained query result exceeded its emitted byte admission");
        let json: serde_json::Value = serde_json::from_str(&packed).unwrap();
        assert_eq!(json["columns"], case["columns"]);
        assert_eq!(json["rows"], case["rows"]);
        if let Some(node_ids) = case.get("nodeIds") {
            let graph = actual.0.graph_fixture.as_ref().expect("typed graph result retains its local fixture owner");
            let mut actual_nodes: Vec<_> = graph.nodes().expect("valid retained Jack child").iter().map(|node| node.id.clone()).collect();
            let mut actual_edges: Vec<_> = graph.edges().expect("valid retained Jack child").iter().map(|edge| edge.id.clone()).collect();
            actual_nodes.sort();
            actual_edges.sort();
            assert_eq!(serde_json::to_value(actual_nodes).unwrap(), *node_ids);
            assert_eq!(serde_json::to_value(actual_edges).unwrap(), case["edgeIds"]);
            assert_eq!(serde_json::to_value(&graph.manifest_id).unwrap(), *case.get("manifestId").unwrap_or(&fixture["graph"]["manifestId"]));
            let graph_json = graph.to_json().unwrap();
            let projected: serde_json::Value = serde_json::from_str(&graph_json).unwrap();
            assert_eq!(projected["manifest"], serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&graph.manifest)).unwrap());
            let decoded = JackSnapshot::from_json(&graph_json).unwrap();
            assert_eq!(decoded.manifest_id, graph.manifest_id);
            assert_eq!(decoded.manifest, graph.manifest);
        }
        assert_eq!(actual.1.len(), case["mutations"].as_u64().unwrap() as usize);
        assert!(execution.step().is_err());
        assert!(steps > 1);
        eprintln!("resumable query completed in {steps} steps with {} document mutations", actual.1.len());
    }
}

#[semio_framework_async_macros::async_test]
async fn query_ownership_artifact_contract_exposes_document_state_only() {
    for source in [include_str!("../../../🔣️.json"), include_str!("../../../🔺️diff/🔣️.json")] {
        let schema: serde_json::Value = serde_json::from_str(source).unwrap();
        for (name, field) in schema["properties"].as_object().unwrap() {
            assert_eq!(field["x-semio-state"], "artifact", "{name} belongs to a separate app or window owner");
        }
    }
}

#[test]
fn query_ownership_cancelled_preparation_closes_while_source_scene_remains_live() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪜️resumable-query/🔣️.json")).expect("neutral retained query fixture");
    let policy = &fixture["retainedExecution"];
    let cancel_after = policy["cancelAfterWorkUnits"].as_u64().expect("cancel units") as usize;
    let maximum_bytes = policy["retirementBytesPerStep"].as_u64().expect("retirement bytes") as usize;
    let source = mini_graph().to_snapshot();
    let source_owner = source.content.local_owner::<crate::JackContentOwner>().expect("source content owner");
    let query = parse("MATCH (a:Apartment) RETURN a.name").expect("query");
    let mut preparation = QueryExecutionPreparation::new(query);
    let content = content_of(&source);
    for _ in 0..cancel_after {
        assert!(matches!(preparation.step(&source, &content, maximum_bytes).expect("preparation step"), QueryPreparationStep::Pending));
    }
    preparation.begin_close();
    for _ in 0..100_000 {
        match preparation.close_step(policy["retirementItemsPerStep"].as_u64().expect("retirement items") as usize, maximum_bytes).expect("preparation close") {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= maximum_bytes);
            }
            store::SnapshotRetirementStep::Complete => break,
            store::SnapshotRetirementStep::Blocked => panic!("cancelled preparation waited for its live source"),
        }
    }
    assert!(preparation.terminal_is_empty());
    assert_eq!(source.content.local_owner::<crate::JackContentOwner>().expect("source survives cancellation").snapshot().nodes.len(), source_owner.snapshot().nodes.len());
    drop(preparation);
}

fn query_ownership_preparation_rejection(source: &JackSnapshot) -> semio_framework_value::ValueError {
    let mut preparation = QueryExecutionPreparation::new(parse("MATCH (a:Piece) RETURN a.name").expect("query"));
    let content = content_of(source);
    let error = (0..100_000)
        .find_map(|_| match preparation.step(source, &content, 4_096) {
            Ok(QueryPreparationStep::Pending) => None,
            Ok(QueryPreparationStep::Complete(_)) => panic!("oversized source entity was cloned"),
            Err(error) => Some(error),
        })
        .expect("oversized source entity is rejected");
    preparation.begin_close();
    let mut complete = false;
    for _ in 0..100_000 {
        match preparation.close_step(1, 4_096).expect("rejected preparation close") {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= 4_096);
            }
            store::SnapshotRetirementStep::Complete => {
                complete = true;
                break;
            }
            store::SnapshotRetirementStep::Blocked => panic!("rejected preparation failed to retire"),
        }
    }
    assert!(complete && preparation.terminal_is_empty());
    error
}

#[test]
fn query_ownership_preparation_rejects_oversized_node_and_edge_before_clone() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪜️resumable-query/🔣️.json")).expect("neutral retained query fixture");
    let property_bytes = fixture["retainedExecution"]["entityAdmission"]["oversizedPropertyBytes"].as_u64().expect("oversized property bytes") as usize;
    let mut node_graph = mini_graph();
    node_graph.nodes.get_mut("root").expect("root node").properties.insert("payload".into(), PropertyValue::String("n".repeat(property_bytes)));
    let node_source = node_graph.to_snapshot();
    let error=query_ownership_preparation_rejection(&node_source);assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);assert_eq!(error.message,"query entity exceeds its byte grant");
    assert_eq!(node_source.nodes().expect("valid retained Jack child").iter().find(|node| node.id == "root").expect("oversized source node remains live").properties.get("payload").expect("retained payload property").as_str().map(str::len), Some(property_bytes));
    let mut edge_graph = mini_graph();
    edge_graph.edges.get_mut("e1").expect("fixture edge").properties.insert("payload".into(), PropertyValue::String("e".repeat(property_bytes)));
    let edge_source = edge_graph.to_snapshot();
    let error=query_ownership_preparation_rejection(&edge_source);assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);assert_eq!(error.message,"query entity exceeds its byte grant");
    assert_eq!(edge_source.edges().expect("valid retained Jack child").iter().find(|edge| edge.id == "e1").expect("oversized source edge remains live").properties.get("payload").expect("retained payload property").as_str().map(str::len), Some(property_bytes));
}

#[test]
fn query_ownership_output_admission_rejects_oversized_table_before_publication() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪜️resumable-query/🔣️.json")).expect("neutral retained query fixture");
    let policy = &fixture["retainedExecution"]["outputAdmission"];
    let row_count = policy["rowCount"].as_u64().expect("row count") as usize;
    let cell_bytes = policy["cellBytes"].as_u64().expect("cell bytes") as usize;
    let maximum_bytes = policy["maximumBytes"].as_u64().expect("maximum bytes") as usize;
    let encoded_cells = 2 + row_count * (cell_bytes + 2) + row_count.saturating_sub(1);
    assert!(encoded_cells > maximum_bytes, "neutral table oracle must exceed the result admission");
    let mut graph = mini_graph();
    let template = graph.nodes.get("root").expect("root node").clone();
    graph.nodes.clear();
    graph.edges.clear();
    graph.root_node_id = None;
    for index in 0..row_count {
        let mut node = template.clone();
        node.id = format!("n-{index:04}");
        node.name = "x".repeat(cell_bytes);
        while let Some((key,value))=node.properties.pop_last(){drop(key);<PropertyValue as semio_framework_dsl_record::DslField>::retire_decoded(value);}
        node.ports.clear();
        graph.nodes.insert(node.id.clone(), node);
    }
    let mut execution = QueryExecution::new(graph, parse("MATCH (a:Piece) RETURN a.name").expect("query"));
    let error = (0..100_000)
        .find_map(|_| match execution.step() {
            Ok(None) => None,
            Ok(Some(_)) => panic!("oversized query result was published"),
            Err(error) => Some(error),
        })
        .expect("oversized query result is rejected");
    assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);assert_eq!(error.message, "query result exceeds its output admission");
    execution.begin_close();
    let mut complete = false;
    for _ in 0..100_000 {
        match execution.close_step(1, 4_096).expect("rejected execution close") {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= 4_096);
            }
            store::SnapshotRetirementStep::Complete => {
                complete = true;
                break;
            }
            store::SnapshotRetirementStep::Blocked => panic!("rejected execution failed to retire"),
        }
    }
    assert!(complete && execution.terminal_is_empty());
    eprintln!("query output admission rejected a {encoded_cells}-byte table before publication");
}

#[test]
fn query_preparation_layout_matches_neutral_budget() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪜️resumable-query/🔣️.json")).expect("neutral query ownership fixture");
    let maximum = fixture["retainedExecution"]["maximumPreparationStepInlineBytes"].as_u64().expect("inline preparation budget") as usize;
    let actual = size_of::<QueryPreparationStep>();
    assert!(actual <= maximum, "query preparation must transfer a compact execution owner");
}
