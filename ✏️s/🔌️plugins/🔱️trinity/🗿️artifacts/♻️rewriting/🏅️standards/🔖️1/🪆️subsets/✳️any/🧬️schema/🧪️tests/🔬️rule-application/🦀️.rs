
use super::*;
use semio_s_artifact_trinity_jack::dsl::NAKAGIN_EXAMPLE_TEXT;
use store::ArtifactDsl;

fn nakagin_graph() -> Graph {
    Graph::from_fixture(semio_s_artifact_trinity_jack::JackSnapshot::parse_dsl(NAKAGIN_EXAMPLE_TEXT).unwrap()).unwrap()
}

fn empty_rule() -> Rule {
    Rule {
        name: "r".into(),
        lhs: Lhs { pattern: PatternJson { left_var: "a".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None }, where_clause: None },
        rhs: Rhs { create: vec![], delete: vec![], set: vec![], merge: vec![], parameters: vec![] },
    }
}

#[semio_framework_async_macros::async_test]
async fn jack_query_on_nakagin() {
    let mut g = nakagin_graph();
    let result = semio_s_artifact_trinity_jack::executor::run(&mut g, "MATCH (a:Piece) WHERE a.name = 'b' RETURN a.name").unwrap();
    assert_eq!(result.rows.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn rewrite_rule_labels_core() {
    let mut g = nakagin_graph();
    let rule = Rule {
        name: "label-core".into(),
        lhs: Lhs { pattern: PatternJson { left_var: "a".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None }, where_clause: Some("a.name = 'b'".into()) },
        rhs: Rhs { create: vec![], delete: vec![], set: vec![AssignmentJson { var: "a".into(), prop: "label".into(), value: PropertyValue::String("nakagin-core".into()) }], merge: vec![], parameters: vec![] },
    };
    apply_rule(&mut g, &rule, &BTreeMap::new()).unwrap();
    let core = g.node("7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
    assert_eq!(core.properties.get("label"), Some(&PropertyValue::String("nakagin-core".into())));
}

#[semio_framework_async_macros::async_test]
async fn rewrite_rule_parameter_substitution() {
    let mut g = nakagin_graph();
    let rule = Rule {
        name: "label-core".into(),
        lhs: Lhs { pattern: PatternJson { left_var: "a".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None }, where_clause: Some("a.name = 'b'".into()) },
        rhs: Rhs {
            create: vec![],
            delete: vec![],
            set: vec![AssignmentJson { var: "a".into(), prop: "label".into(), value: PropertyValue::String("$label".into()) }],
            merge: vec![],
            parameters: vec![ParameterSpec { name: "label".into(), kind: ParameterKind::String, default: PropertyValue::String("nakagin-core".into()) }],
        },
    };
    apply_rule(&mut g, &rule, &BTreeMap::new()).unwrap();
    let core = g.node("7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
    assert_eq!(core.properties.get("label"), Some(&PropertyValue::String("nakagin-core".into())));

    let mut g2 = nakagin_graph();
    let mut bindings = BTreeMap::new();
    bindings.insert("label".into(), PropertyValue::String("override-core".into()));
    apply_rule(&mut g2, &rule, &bindings).unwrap();
    let core2 = g2.node("7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
    assert_eq!(core2.properties.get("label"), Some(&PropertyValue::String("override-core".into())));

    let query = build_rule_query(&rule, &bindings);
    assert!(query.contains("SET a.label = \"override-core\""));
}

#[semio_framework_async_macros::async_test]
async fn rewriting_labeled_fixture_reloads() {
    let mut g = Graph::from_fixture(semio_s_artifact_trinity_jack::JackSnapshot::parse_dsl(NAKAGIN_EXAMPLE_TEXT).unwrap()).unwrap();
    let rule = Rule {
        name: "label-core".into(),
        lhs: Lhs { pattern: PatternJson { left_var: "a".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None }, where_clause: Some("a.name = 'b'".into()) },
        rhs: Rhs { create: vec![], delete: vec![], set: vec![AssignmentJson { var: "a".into(), prop: "label".into(), value: PropertyValue::String("nakagin-core".into()) }], merge: vec![], parameters: vec![] },
    };
    apply_rule(&mut g, &rule, &BTreeMap::new()).unwrap();
    let fixture_json = g.fixture_json().unwrap();
    let reloaded = Graph::load_json(&fixture_json).unwrap();
    let core = reloaded.node("7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
    assert_eq!(core.properties.get("label"), Some(&PropertyValue::String("nakagin-core".into())));
}

#[semio_framework_async_macros::async_test]
async fn pattern_to_match_clause_edge_variants() {
    let base = |edge_var: Option<&str>, edge_kind: Option<&str>| PatternJson {
        left_var: "a".into(),
        left_kind: "Piece".into(),
        edge_var: edge_var.map(String::from),
        edge_kind: edge_kind.map(String::from),
        right_var: Some("b".into()),
        right_kind: Some("Piece".into()),
    };
    assert_eq!(pattern_to_match_clause(&base(Some("e"), Some("Connection"))), "(a:Piece)-[e:Connection]->(b:Piece)");
    assert_eq!(pattern_to_match_clause(&base(Some("e"), None)), "(a:Piece)-[e]->(b:Piece)");
    assert_eq!(pattern_to_match_clause(&base(None, Some("Connection"))), "(a:Piece)-[:Connection]->(b:Piece)");
    assert_eq!(pattern_to_match_clause(&base(None, None)), "(a:Piece)-[]->(b:Piece)");
}

#[semio_framework_async_macros::async_test]
async fn build_rule_query_edge_pattern_and_all_clauses() {
    let rule = Rule {
        name: "full".into(),
        lhs: Lhs {
            pattern: PatternJson { left_var: "a".into(), left_kind: "Piece".into(), edge_var: Some("e".into()), edge_kind: Some("Connection".into()), right_var: Some("b".into()), right_kind: Some("Piece".into()) },
            where_clause: Some("a.name = 'b'".into()),
        },
        rhs: Rhs {
            create: vec![PatternJson { left_var: "c".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None }],
            delete: vec!["e".into()],
            set: vec![AssignmentJson { var: "a".into(), prop: "label".into(), value: PropertyValue::String("x".into()) }],
            merge: vec![PatternJson { left_var: "a".into(), left_kind: "Piece".into(), edge_var: Some("m".into()), edge_kind: None, right_var: Some("c".into()), right_kind: Some("Piece".into()) }],
            parameters: vec![],
        },
    };
    let query = build_rule_query(&rule, &BTreeMap::new());
    assert!(query.starts_with("MATCH (a:Piece)-[e:Connection]->(b:Piece) WHERE a.name = 'b'"));
    assert!(query.contains("DELETE e"));
    assert!(query.contains("SET a.label = \"x\""));
    assert!(query.contains("CREATE (c:Piece)"));
    assert!(query.contains("MERGE (a:Piece)-[m]->(c:Piece)"));
}

#[semio_framework_async_macros::async_test]
async fn resolve_parameter_value_variants() {
    let mut rule = empty_rule();
    rule.rhs.parameters.push(ParameterSpec { name: "label".into(), kind: ParameterKind::String, default: PropertyValue::String("default-label".into()) });
    let mut bindings = BTreeMap::new();
    bindings.insert("label".to_string(), PropertyValue::String("bound-label".into()));

    assert_eq!(resolve_parameter_value(&rule, &bindings, &PropertyValue::String("$label".into())), PropertyValue::String("bound-label".into()));
    assert_eq!(resolve_parameter_value(&rule, &BTreeMap::new(), &PropertyValue::String("$label".into())), PropertyValue::String("default-label".into()));
    assert_eq!(resolve_parameter_value(&rule, &BTreeMap::new(), &PropertyValue::String("$unknown".into())), PropertyValue::String("$unknown".into()));
    assert_eq!(resolve_parameter_value(&rule, &BTreeMap::new(), &PropertyValue::String("plain".into())), PropertyValue::String("plain".into()));
    assert_eq!(resolve_parameter_value(&rule, &BTreeMap::new(), &PropertyValue::Number(5.0)), PropertyValue::Number(5.0));
    assert_eq!(resolve_parameter_value(&rule, &BTreeMap::new(), &PropertyValue::String("$".into())), PropertyValue::String("$".into()));
}

#[semio_framework_async_macros::async_test]
async fn assignment_value_jack_formats_each_property_variant() {
    let rule = empty_rule();
    let bindings = BTreeMap::new();
    assert_eq!(assignment_value_jack(&rule, &bindings, &PropertyValue::Null), "null");
    assert_eq!(assignment_value_jack(&rule, &bindings, &PropertyValue::Bool(true)), "true");
    assert_eq!(assignment_value_jack(&rule, &bindings, &PropertyValue::Number(4.5)), "4.5");
    assert_eq!(assignment_value_jack(&rule, &bindings, &PropertyValue::String("hi".into())), "\"hi\"");
    let arr = PropertyValue::Array(vec![PropertyValue::Number(1.0)]);
    assert_eq!(assignment_value_jack(&rule, &bindings, &arr), pack::to_json_string(&arr));
}

#[semio_framework_async_macros::async_test]
async fn parse_bindings_json_handles_empty_and_invalid() {
    assert_eq!(parse_bindings_json("").unwrap(), BTreeMap::new());
    assert_eq!(parse_bindings_json("   ").unwrap(), BTreeMap::new());
    assert!(parse_bindings_json("{not json").is_err());
    let mut expected = BTreeMap::new();
    expected.insert("x".to_string(), PropertyValue::Number(1.0));
    assert_eq!(parse_bindings_json("{\"x\":1}").unwrap(), expected);
}

#[semio_framework_async_macros::async_test]
async fn apply_rule_json_and_rule_query_json_end_to_end() {
    let mut g = nakagin_graph();
    let mut rule = empty_rule();
    rule.name = "label-core".into();
    rule.lhs.where_clause = Some("a.name = 'b'".into());
    rule.rhs.set.push(AssignmentJson { var: "a".into(), prop: "label".into(), value: PropertyValue::String("nakagin-core".into()) });
    let rule_json = pack::to_json_string(&rule);

    let query_out = rule_query_json(&rule_json, "{}").unwrap();
    let query_value: pack::JsonValue = pack::parse_json(&query_out).unwrap();
    assert!(query_value["query"].as_str().unwrap().contains("SET a.label"));

    let apply_out = apply_rule_json(&mut g, &rule_json, "{}").unwrap();
    let apply_value: pack::JsonValue = pack::parse_json(&apply_out).unwrap();
    assert!(apply_value.get("fixture").is_some());
    let core = g.node("7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
    assert_eq!(core.properties.get("label"), Some(&PropertyValue::String("nakagin-core".into())));

    assert!(apply_rule_json(&mut g, "not json", "{}").is_err());
    assert!(rule_query_json("not json", "{}").is_err());
}
