use super::{apply_rule, apply_rule_json, assignment_value_jack, build_rule_query, parse_bindings_json, pattern_to_match_clause, plan_rule, resolve_parameter_value, rule_query_json};
use crate::standards::v1::subsets::any::schema::rule::*;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::graph::port_node_id;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::memory::Graph;
use semio_framework_graph_manifest::{PropertyBag, PropertyValue};
use semio_s_artifact_trinity_jack_semio_content::SemioJackContent;

fn nakagin_graph() -> Graph {
    Graph::from_snapshot(crate::standards::v1::subsets::any::schema::test_support::fixture_rule_state().working_graph).unwrap()
}

fn empty_rule() -> Rule {
    Rule {
        name: "r".into(),
        lhs: Lhs { pattern: Pattern { left_var: "a".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None }, where_clause: None },
        rhs: Rhs { create: vec![], delete: vec![], set: vec![], merge: vec![], parameters: vec![] },
    }
}

#[semio_framework_async_macros::async_test]
async fn jack_query_on_nakagin() {
    let mut g = nakagin_graph();
    let result = semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::runner::run(&mut g, "MATCH (a:Piece) WHERE a.name = 'b' RETURN a.name").unwrap();
    assert_eq!(result.rows.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn rewrite_rule_labels_core() {
    let mut g = nakagin_graph();
    let rule = Rule {
        name: "label-core".into(),
        lhs: Lhs { pattern: Pattern { left_var: "a".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None }, where_clause: Some("a.name = 'b'".into()) },
        rhs: Rhs { create: vec![], delete: vec![], set: vec![Assignment { var: "a".into(), prop: "label".into(), value: PropertyValue::String("nakagin-core".into()) }], merge: vec![], parameters: vec![] },
    };
    apply_rule(&mut g, &rule, &semio_framework_graph_manifest::PropertyBag::new()).unwrap();
    let core = g.node("7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
    assert_eq!(core.properties.get("label"), Some(&PropertyValue::String("nakagin-core".into())));
}

#[semio_framework_async_macros::async_test]
async fn rewrite_rule_parameter_substitution() {
    let mut g = nakagin_graph();
    let rule = Rule {
        name: "label-core".into(),
        lhs: Lhs { pattern: Pattern { left_var: "a".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None }, where_clause: Some("a.name = 'b'".into()) },
        rhs: Rhs {
            create: vec![],
            delete: vec![],
            set: vec![Assignment { var: "a".into(), prop: "label".into(), value: PropertyValue::String("$label".into()) }],
            merge: vec![],
            parameters: vec![ParameterSpec { name: "label".into(), kind: ParameterKind::String, default: PropertyValue::String("nakagin-core".into()) }],
        },
    };
    apply_rule(&mut g, &rule, &semio_framework_graph_manifest::PropertyBag::new()).unwrap();
    let core = g.node("7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
    assert_eq!(core.properties.get("label"), Some(&PropertyValue::String("nakagin-core".into())));

    let mut g2 = nakagin_graph();
    let mut bindings = semio_framework_graph_manifest::PropertyBag::new();
    bindings.insert("label".into(), PropertyValue::String("override-core".into()));
    apply_rule(&mut g2, &rule, &bindings).unwrap();
    let core2 = g2.node("7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
    assert_eq!(core2.properties.get("label"), Some(&PropertyValue::String("override-core".into())));

    let query = build_rule_query(&rule, &bindings);
    assert!(query.contains("SET a.label = \"override-core\""));
}

#[semio_framework_async_macros::async_test]
async fn rewriting_labeled_fixture_reloads() {
    let mut g = Graph::from_snapshot(crate::standards::v1::subsets::any::schema::test_support::fixture_rule_state().working_graph).unwrap();
    let rule = Rule {
        name: "label-core".into(),
        lhs: Lhs { pattern: Pattern { left_var: "a".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None }, where_clause: Some("a.name = 'b'".into()) },
        rhs: Rhs { create: vec![], delete: vec![], set: vec![Assignment { var: "a".into(), prop: "label".into(), value: PropertyValue::String("nakagin-core".into()) }], merge: vec![], parameters: vec![] },
    };
    apply_rule(&mut g, &rule, &semio_framework_graph_manifest::PropertyBag::new()).unwrap();
    let reloaded = Graph::from_snapshot(g.to_snapshot::<SemioJackContent>()).unwrap();
    let core = reloaded.node("7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
    assert_eq!(core.properties.get("label"), Some(&PropertyValue::String("nakagin-core".into())));
}

#[semio_framework_async_macros::async_test]
async fn pattern_to_match_clause_edge_variants() {
    let base = |edge_var: Option<&str>, edge_kind: Option<&str>| Pattern {
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
            pattern: Pattern { left_var: "a".into(), left_kind: "Piece".into(), edge_var: Some("e".into()), edge_kind: Some("Connection".into()), right_var: Some("b".into()), right_kind: Some("Piece".into()) },
            where_clause: Some("a.name = 'b'".into()),
        },
        rhs: Rhs {
            create: vec![Pattern { left_var: "c".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None }],
            delete: vec!["e".into()],
            set: vec![Assignment { var: "a".into(), prop: "label".into(), value: PropertyValue::String("x".into()) }],
            merge: vec![Pattern { left_var: "a".into(), left_kind: "Piece".into(), edge_var: Some("m".into()), edge_kind: None, right_var: Some("c".into()), right_kind: Some("Piece".into()) }],
            parameters: vec![],
        },
    };
    let query = build_rule_query(&rule, &semio_framework_graph_manifest::PropertyBag::new());
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
    let mut bindings = semio_framework_graph_manifest::PropertyBag::new();
    bindings.insert("label".to_string(), PropertyValue::String("bound-label".into()));

    assert_eq!(resolve_parameter_value(&rule, &bindings, &PropertyValue::String("$label".into())), PropertyValue::String("bound-label".into()));
    assert_eq!(resolve_parameter_value(&rule, &semio_framework_graph_manifest::PropertyBag::new(), &PropertyValue::String("$label".into())), PropertyValue::String("default-label".into()));
    assert_eq!(resolve_parameter_value(&rule, &semio_framework_graph_manifest::PropertyBag::new(), &PropertyValue::String("$unknown".into())), PropertyValue::String("$unknown".into()));
    assert_eq!(resolve_parameter_value(&rule, &semio_framework_graph_manifest::PropertyBag::new(), &PropertyValue::String("plain".into())), PropertyValue::String("plain".into()));
    assert_eq!(resolve_parameter_value(&rule, &semio_framework_graph_manifest::PropertyBag::new(), &PropertyValue::Number(5.0)), PropertyValue::Number(5.0));
    assert_eq!(resolve_parameter_value(&rule, &semio_framework_graph_manifest::PropertyBag::new(), &PropertyValue::String("$".into())), PropertyValue::String("$".into()));
}

#[semio_framework_async_macros::async_test]
async fn assignment_value_jack_formats_each_property_variant() {
    let rule = empty_rule();
    let bindings = semio_framework_graph_manifest::PropertyBag::new();
    assert_eq!(assignment_value_jack(&rule, &bindings, &PropertyValue::Null), "null");
    assert_eq!(assignment_value_jack(&rule, &bindings, &PropertyValue::Bool(true)), "true");
    assert_eq!(assignment_value_jack(&rule, &bindings, &PropertyValue::Number(4.5)), "4.5");
    assert_eq!(assignment_value_jack(&rule, &bindings, &PropertyValue::String("hi".into())), "\"hi\"");
    let arr = PropertyValue::Array(vec![PropertyValue::Number(1.0)]);
    assert_eq!(assignment_value_jack(&rule, &bindings, &arr), semio_framework_pack_json::to_json_string(&arr));
}

#[semio_framework_async_macros::async_test]
async fn parse_bindings_json_handles_empty_and_invalid() {
    assert_eq!(parse_bindings_json("").unwrap(), semio_framework_graph_manifest::PropertyBag::new());
    assert_eq!(parse_bindings_json("   ").unwrap(), semio_framework_graph_manifest::PropertyBag::new());
    assert!(parse_bindings_json("{not json").is_err());
    let mut expected = semio_framework_graph_manifest::PropertyBag::new();
    expected.insert("x".to_string(), PropertyValue::Number(1.0));
    assert_eq!(parse_bindings_json("{\"x\":1}").unwrap(), expected);
}

#[semio_framework_async_macros::async_test]
async fn apply_rule_json_and_rule_query_json_end_to_end() {
    let mut g = nakagin_graph();
    let mut rule = empty_rule();
    rule.name = "label-core".into();
    rule.lhs.where_clause = Some("a.name = 'b'".into());
    rule.rhs.set.push(Assignment { var: "a".into(), prop: "label".into(), value: PropertyValue::String("nakagin-core".into()) });
    let rule_json = semio_framework_pack_json::to_json_string(&rule);

    let query_out = rule_query_json(&rule_json, "{}").unwrap();
    let query_value: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&query_out, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(query_value["query"].as_str().unwrap().contains("SET a.label"));

    let apply_out = apply_rule_json(&mut g, &rule_json, "{}").unwrap();
    let apply_value: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&apply_out, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let reloaded = Graph::load_scene_json(apply_value["snapshotJson"].as_str().unwrap()).unwrap();
    assert_eq!(g, reloaded);
    let core = g.node("7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
    assert_eq!(core.properties.get("label"), Some(&PropertyValue::String("nakagin-core".into())));

    assert!(apply_rule_json(&mut g, "not json", "{}").is_err());
    assert!(rule_query_json("not json", "{}").is_err());
}

#[test]
fn create_then_assign_uses_exact_string_and_keeps_endpoint_identity() {
    let case: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let mut graph = nakagin_graph();
    let before = graph.clone();
    let query = rule_query_json(&case["rule"].to_string(), &case["bindings"].to_string()).unwrap();
    let query: serde_json::Value = serde_json::from_str(&query).unwrap();
    let query = query["query"].as_str().unwrap();
    assert!(query.find("CREATE").unwrap() < query.find("SET").unwrap());
    assert!(query.contains(&case["expectedName"].to_string()));
    let mut from_query = before.clone();
    semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::runner::run(&mut from_query, query).unwrap();
    apply_rule_json(&mut graph, &case["rule"].to_string(), &case["bindings"].to_string()).unwrap();
    assert_eq!(from_query, graph);
    let [node] = graph.nodes.values().filter(|node| !before.nodes.contains_key(&node.id)).collect::<Vec<_>>()[..] else {
        panic!("expected one created node");
    };
    let [edge] = graph.edges.values().filter(|edge| !before.edges.contains_key(&edge.id)).collect::<Vec<_>>()[..] else {
        panic!("expected one created edge");
    };
    assert_eq!(node.name, case["expectedName"].as_str().unwrap());
    assert_eq!(node.kind, case["expectedKind"].as_str().unwrap());
    assert_eq!(edge.kind, case["expectedEdgeKind"].as_str().unwrap());
    assert_eq!(port_node_id(&edge.source).unwrap_or(&edge.source), case["expectedSourceId"].as_str().unwrap());
    assert_eq!(port_node_id(&edge.target).unwrap_or(&edge.target), node.id);
}

#[test]
fn preview_preserves_puzzle_kind_names() {
    use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::{ast::Clause, language_service::parse};
    let mut rule = empty_rule();
    rule.lhs.pattern.left_kind = "part.beam".into();
    rule.lhs.pattern.edge_kind = Some("edge.link".into());
    rule.lhs.pattern.right_var = Some("b".into());
    rule.lhs.pattern.right_kind = Some("part.column".into());
    let query = build_rule_query(&rule, &PropertyBag::new());
    assert_eq!(parse(&query).unwrap().clauses, vec![Clause::Match(vec![rule.lhs.pattern.to_jack_pattern()])]);
}

#[test]
fn invalid_pattern_or_non_predicate_where_refuses_the_entire_rewrite() {
    let mut graph = nakagin_graph();
    let before = graph.clone();
    let mut rule = empty_rule();
    rule.rhs.set.push(Assignment { var: "a".into(), prop: "label".into(), value: PropertyValue::String("changed".into()) });
    rule.lhs.pattern.right_var = Some("b".into());
    assert!(apply_rule(&mut graph, &rule, &PropertyBag::new()).is_err());
    assert_eq!(graph, before);

    rule.lhs.pattern.right_var = None;
    rule.lhs.where_clause = Some("a.name = 'b' SET a.name = 'unexpected'".into());
    assert!(apply_rule(&mut graph, &rule, &PropertyBag::new()).is_err());
    assert_eq!(graph, before);
}

#[test]
fn merge_plan_keeps_structured_assignments_and_is_repeatable() {
    use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::graph::{EntityRef, GraphEffect};
    let mut graph = nakagin_graph();
    let original_edges = graph.edges.len();
    let mut rule = empty_rule();
    rule.lhs.where_clause = Some("a.name = 'b'".into());
    rule.rhs.merge.push(Pattern {
        left_var: "a".into(), left_kind: "Piece".into(),
        edge_var: Some("loopEdge".into()), edge_kind: Some("Connection".into()),
        right_var: Some("a".into()), right_kind: Some("Piece".into()),
    });
    let value = semio_framework_pack_json::from_json_str(r#"{"x":4,"y":3,"z":2}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    rule.rhs.set.push(Assignment { var: "a".into(), prop: "position".into(), value });
    let before = graph.clone();
    let (_, effects) = plan_rule(&graph, &rule, &PropertyBag::new()).unwrap();
    assert_eq!(graph, before);
    assert!(matches!(effects.as_slice(),
        [GraphEffect::CreateEdge(_), GraphEffect::SetProperty { entity: EntityRef::Node(_), key, value: PropertyValue::Object(_) }]
        if key == "position"
    ));
    apply_rule(&mut graph, &rule, &PropertyBag::new()).unwrap();
    assert_eq!(graph.edges.len(), original_edges + 1);
    let first = graph.clone();
    apply_rule(&mut graph, &rule, &PropertyBag::new()).unwrap();
    assert_eq!(graph, first);
}
