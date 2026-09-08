use super::*;

async fn node(id: &str) -> EdgeNode {
    EdgeNode { id: id.to_string(), kind: None, port: None }
}

#[semio_framework_async_macros::async_test]
async fn parses_a_directed_chain() {
    let chain = parse_chain_text("v1->v2->v3->v1").await.expect("parse_chain_text");
    assert_eq!(chain.nodes, vec![node("v1").await, node("v2").await, node("v3").await, node("v1").await]);
    assert!(chain.directed);
    assert_eq!(print_chain(&chain).await, "v1->v2->v3->v1");
}

#[semio_framework_async_macros::async_test]
async fn parses_an_undirected_chain() {
    let chain = parse_chain_text("v1--v2--v3").await.expect("parse_chain_text");
    assert!(!chain.directed);
    assert_eq!(print_chain(&chain).await, "v1--v2--v3");
}

#[semio_framework_async_macros::async_test]
async fn rejects_mixed_direction_chains() {
    let err = parse_chain_text("v1->v2--v3").await.unwrap_err();
    assert!(err.message.contains("cannot mix"), "unexpected message: {}", err.message);
}

#[semio_framework_async_macros::async_test]
async fn expand_lowers_a_chain_into_plain_edges() {
    let chain = parse_chain_text("a->b->c").await.expect("parse_chain_text");
    let edges = chain.expand().await;
    assert_eq!(edges.len(), 2);
    assert_eq!(edges[0], EdgeValue { from: node("a").await, link: Some(EdgeLink { directed: true, label: EdgeLabel::default(), to: node("b").await }) });
    assert_eq!(edges[1], EdgeValue { from: node("b").await, link: Some(EdgeLink { directed: true, label: EdgeLabel::default(), to: node("c").await }) });
}

#[semio_framework_async_macros::async_test]
async fn contract_reassembles_a_maximal_anonymous_run() {
    let chain = parse_chain_text("a->b->c->d").await.expect("parse_chain_text");
    let edges = chain.expand().await;
    let (contracted, consumed) = contract(&edges).expect("contract");
    assert_eq!(consumed, edges.len());
    assert_eq!(contracted, chain);
}

/// @emoji ⛓️‍💥️ A single unlabeled edge followed by a labeled one is NOT a 1-edge "chain of
/// one" — `contract` returns `None` so the caller prints `edges[0]` as a standalone statement
/// (never through `print_chain`) and retries `contract` from index 1. This matters for a
/// three-edge run where only the first edge is unlabeled: the run never reaches 2 chainable
/// edges, so it must fall back to two ordinary edge statements, not one bogus 1-edge chain.
#[semio_framework_async_macros::async_test]
async fn contract_returns_none_when_the_run_never_reaches_two_edges() {
    let mut edges = ChainValue { nodes: vec![node("a").await, node("b").await, node("c").await], directed: true }.expand().await;
    edges[1].link.as_mut().unwrap().label = EdgeLabel { id: Some("e1".to_string()), kind: None };
    assert_eq!(contract(&edges), None);
}

#[semio_framework_async_macros::async_test]
async fn contract_returns_none_when_endpoints_dont_thread() {
    let edges = vec![
        EdgeValue { from: node("a").await, link: Some(EdgeLink { directed: true, label: EdgeLabel::default(), to: node("b").await }) },
        EdgeValue { from: node("x").await, link: Some(EdgeLink { directed: true, label: EdgeLabel::default(), to: node("y").await }) },
    ];
    assert_eq!(contract(&edges), None);
}

/// @emoji 📖️ The fragment's `.grammar` file must at least parse under `dsl_grammar`'s parser
/// (the spec+conformance role decided for grammar files — this doesn't yet prove the
/// recognizer accepts every fixture, only that the spec itself is well-formed).
#[semio_framework_async_macros::async_test]
async fn grammar_file_is_syntactically_valid() {
    let source = include_str!("../../📖️.grammar.semio");
    let grammar = crate::os_dsl::grammar::parse_grammar(source).expect("family-graph.grammar must parse");
    assert_eq!(grammar.id, "family-graph");
    assert!(grammar.productions.len() > 6, "family-graph should expose edge/chain/label vocabulary");
}

#[semio_framework_async_macros::async_test]
async fn round_trip_matrix() {
    let sources = vec!["a->b", "a--b", "a->b->c->d->e", "v1:Vertex@p0->v2:Vertex@p1"];
    for source in sources {
        let chain = parse_chain_text(source).await.unwrap_or_else(|e| panic!("parse of {source:?} failed: {e:?}"));
        let printed = print_chain(&chain).await;
        assert_eq!(printed, source, "canonical print should match already-canonical input for {source:?}");
        let reparsed = parse_chain_text(&printed).await.unwrap_or_else(|e| panic!("reparse of {printed:?} failed: {e:?}"));
        assert_eq!(reparsed, chain);
    }
}
