/// 🔤️ Lexer and language service laws of the graph DSL: the tokenizer, the formatter, completion, hover, lint and semantic tokens.

// #region 🔖️LexerAndLanguageServiceTests
#[test]
fn tokenize_classifies_clause_and_operator_tokens() {
    block_on_test(async {
        let spans = tokenize("MATCH (a:x)--[:wire]->(b:y) WHERE a.p = 1 AND b.q != 'v' RETURN a.p");
        assert!(spans.iter().any(|s| s.class == TokenClass::Keyword));
        assert!(spans.iter().any(|s| s.class == TokenClass::Ident));
        assert!(spans.iter().any(|s| s.class == TokenClass::Number));
        assert!(spans.iter().any(|s| s.class == TokenClass::String));
        assert!(spans.iter().any(|s| s.class == TokenClass::Operator));
        assert!(spans.iter().any(|s| s.class == TokenClass::Punctuation));
    });
}

#[test]
fn tokenize_marks_unterminated_string_as_error_class() {
    block_on_test(async {
        let spans = tokenize("MATCH (a:x) WHERE a.p = 'unterminated");
        assert!(spans.iter().any(|s| s.class == TokenClass::Error));
    });
}

#[test]
fn tokenize_never_panics_on_stray_symbols() {
    block_on_test(async {
        let spans = tokenize("MATCH (a:x) ~ ^ RETURN a");
        assert!(spans.iter().any(|s| s.class == TokenClass::Ident && s.end - s.start == 1));
    });
}

#[test]
fn tokenize_treats_hash_as_a_comment_to_end_of_line() {
    block_on_test(async {
        let source = "MATCH (a:x) # a trailing comment\nRETURN a";
        let comment_start = source.find('#').unwrap();
        let line_end = source.find('\n').unwrap();
        let spans = tokenize(source);
        assert!(!spans.iter().any(|s| s.start >= comment_start && s.start < line_end), "no token should start inside the comment body: {spans:?}");
        assert!(spans.iter().any(|s| s.class == TokenClass::Keyword));
    });
}

#[test]
fn format_query_is_idempotent_and_normalizes_whitespace() {
    block_on_test(async {
        let once = format("match(a:x)--[:wire]->(b:y) where a.p=1 and b.q!='v' return a.p,b.q").unwrap();
        assert!(once.contains("MATCH"));
        assert!(once.contains(" AND "));
        assert!(once.contains(" = "));
        let twice = format(&once).unwrap();
        assert_eq!(once, twice);
    });
}

#[test]
fn format_rejects_unterminated_string() {
    block_on_test(async {
        let err = format("MATCH (a:x) WHERE a.p = 'oops").unwrap_err();
        assert!(matches!(err, GraphDslError::UnterminatedString));
    });
}

#[test]
fn complete_after_colon_suggests_node_then_edge_kinds() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let node_source = "MATCH (a:c";
        let node_completions = complete(&graph, node_source, node_source.len());
        assert!(node_completions.iter().any(|c| c.label == "computation"));
        let edge_source = "MATCH (a:computation)--[:w";
        let edge_completions = complete(&graph, edge_source, edge_source.len());
        assert!(edge_completions.iter().any(|c| c.label == "wire"));
    });
}

#[test]
fn complete_after_at_suggests_port_kinds() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let source = "MATCH (a:computation@i";
        let completions = complete(&graph, source, source.len());
        assert!(completions.iter().any(|c| c.label == "in"));
    });
}

#[test]
fn complete_after_dot_suggests_property_names() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let source = "MATCH (a:computation) RETURN a.sc";
        let completions = complete(&graph, source, source.len());
        assert!(completions.iter().any(|c| c.label == "score"));
    });
}

#[test]
fn complete_suggests_bound_variable_when_prefix_does_not_match_logic_keywords() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let source = "MATCH (abc:computation) WHERE ab";
        let completions = complete(&graph, source, source.len());
        assert!(completions.iter().any(|c| c.label == "abc" && c.kind == "variable"));
    });
}

#[test]
fn complete_in_where_clause_suggests_logic_keywords() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let source = "MATCH (a:computation) WHERE a.score = 1 AN";
        let completions = complete(&graph, source, source.len());
        assert!(completions.iter().any(|c| c.label == "AND"));
    });
}

#[test]
fn complete_at_start_suggests_clause_keywords() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let completions = complete(&graph, "MA", 2);
        assert!(completions.iter().any(|c| c.label == "MATCH"));
    });
}

#[test]
fn hover_reports_keyword_and_bound_variable() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let source = "MATCH (a:computation) WHERE a.score = 1 RETURN a";
        let match_pos = source.find("MATCH").unwrap();
        assert!(hover(&graph, source, match_pos + 1).unwrap().contents.contains("keyword"));
        let var_pos = source.rfind('a').unwrap();
        assert!(hover(&graph, source, var_pos + 1).unwrap().contents.contains("Bound variable"));
    });
}

#[test]
fn hover_matches_bare_node_kind_edge_kind_and_property_words() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let source = "computation wire score";
        assert!(hover(&graph, source, 3).unwrap().contents.contains("Node kind"));
        let edge_pos = source.find("wire").unwrap();
        assert!(hover(&graph, source, edge_pos + 1).unwrap().contents.contains("Edge kind"));
        let prop_pos = source.find("score").unwrap();
        assert!(hover(&graph, source, prop_pos + 1).unwrap().contents.contains("Property"));
    });
}

#[test]
fn hover_returns_none_for_whitespace() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        assert!(hover(&graph, "MATCH (a:x)   RETURN a", 12).is_none());
    });
}

#[test]
fn lint_flags_unknown_node_kind() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let diags = lint(&graph, "MATCH (a:nonexistentKind) RETURN a");
        assert!(diags.iter().any(|d| d.code.as_deref() == Some("jack/unknown-node-kind")));
    });
}

#[test]
fn lint_flags_unbound_variable() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let diags = lint(&graph, "MATCH (a:computation) RETURN b");
        assert!(diags.iter().any(|d| d.code.as_deref() == Some("jack/unbound-variable")));
    });
}

#[test]
fn lint_reports_parse_errors() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let diags = lint(&graph, "MATCH (a:computation");
        assert!(diags.iter().any(|d| d.code.as_deref() == Some("jack/parse-error")));
    });
}

#[test]
fn lint_clean_query_has_no_diagnostics() {
    block_on_test(async {
        let graph = BoardQueryableGraph::from_host_snapshot_json(split_endpoint_fixture(), None).unwrap();
        let diags = lint(&graph, "MATCH (a:computation) RETURN a.name");
        assert!(diags.is_empty());
    });
}

#[test]
fn semantic_tokens_mirror_tokenize_classes() {
    block_on_test(async {
        let tokens = semantic_tokens("MATCH (a:x) RETURN a");
        assert!(tokens.iter().any(|t| t.class == "keyword"));
        assert!(tokens.iter().any(|t| t.class == "ident"));
    });
}
// #endregion 🔖️LexerAndLanguageServiceTests
