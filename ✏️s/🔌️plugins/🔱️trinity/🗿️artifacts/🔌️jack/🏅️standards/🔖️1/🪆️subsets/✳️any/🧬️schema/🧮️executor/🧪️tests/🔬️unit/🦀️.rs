
use super::*;
use crate::ast::QueryResultKind;
use crate::language_service::{complete, format as format_source, hover, lint, semantic_tokens};
use crate::lexer::{Token, TokenClass, lex, tokenize};
use crate::{Camera, Manifest};

fn mini_graph() -> Graph {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🪜️resumable-query/🔣️.json")).unwrap();
    let graph = &fixture["graph"];
    let nodes = dsl::FromValue::from_value(dsl::DslValue::from(graph["nodes"].clone())).unwrap();
    let edges = dsl::FromValue::from_value(dsl::DslValue::from(graph["edges"].clone())).unwrap();
    let fixture = JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), graph["name"].as_str().unwrap().into(), Some(graph["manifestId"].as_str().unwrap().into()), Manifest::nakagin_default(), Camera::default(), nodes, edges, Some(graph["rootNodeId"].as_str().unwrap().into()));
    Graph::from_fixture(fixture).unwrap()
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
    assert_eq!(fixture.nodes().len(), 2);
    assert_eq!(fixture.edges().len(), 1);
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🪜️resumable-query/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let graph = mini_graph();
        let query = parse(case["query"].as_str().unwrap()).unwrap();
        let expected = execute(&graph, &query).unwrap();
        let mut execution = QueryExecution::new(graph, query);
        let mut steps = 0;
        let actual = loop {
            steps += 1;
            assert!(steps < 10_000);
            if let Some(result) = execution.step().unwrap() { break result; }
        };
        assert_eq!(actual, expected);
        let json: serde_json::Value = serde_json::from_str(&pack::to_json_string(&actual.0)).unwrap();
        assert_eq!(json["columns"], case["columns"]);
        assert_eq!(json["rows"], case["rows"]);
        if let Some(node_ids) = case.get("nodeIds") {
            let graph = actual.0.graph_fixture.as_ref().expect("typed graph result retains its local fixture owner");
            let mut actual_nodes: Vec<_> = graph.nodes().iter().map(|node| node.id.clone()).collect();
            let mut actual_edges: Vec<_> = graph.edges().iter().map(|edge| edge.id.clone()).collect();
            actual_nodes.sort();
            actual_edges.sort();
            assert_eq!(serde_json::to_value(actual_nodes).unwrap(), *node_ids);
            assert_eq!(serde_json::to_value(actual_edges).unwrap(), case["edgeIds"]);
        }
        assert_eq!(actual.1.len(), case["mutations"].as_u64().unwrap() as usize);
        assert!(execution.step().is_err());
        assert!(steps > 1);
        eprintln!("[DEBUG] resumable query completed in {steps} steps with {} document mutations", actual.1.len());
    }
}
