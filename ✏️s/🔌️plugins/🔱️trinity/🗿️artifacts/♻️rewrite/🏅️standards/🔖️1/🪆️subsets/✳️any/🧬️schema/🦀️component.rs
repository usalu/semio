//! 🧬️ Rewrite artifact schema — every field of the artifact with its state class.

use crate::artifacts::jack::{Camera, Graph, PropertyValue};
use crate::artifacts::rewrite::{LayoutPoint, TrinityRewriteError};
use crate::ast::{Pattern, PatternEdge, PatternNode, QueryResult};
use crate::executor::execute;
use crate::language_service::parse;
use schema::ArtifactSchema;
use std::collections::BTreeMap;

//#region 🔖️Artifact
/// 🧬️ Full rewrite artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.rewrite")]
pub struct RewriteArtifact {
    #[state(artifact)]
    pub before_fixture_json: String,
    #[state(artifact)]
    pub lhs_json: String,
    #[state(artifact)]
    pub rhs_json: String,
    #[state(artifact)]
    pub parameter_bindings: BTreeMap<String, PropertyValue>,
    #[state(artifact)]
    pub rule_layout: BTreeMap<String, LayoutPoint>,
    #[state(presence)]
    pub lod_mode_by_window: BTreeMap<String, String>,
    #[state(config)]
    pub before_pane_camera: Camera,
    #[state(config)]
    pub reorganize_epoch: u64,
    #[state(config)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for RewriteArtifact {
    fn default() -> Self {
        Self {
            before_fixture_json: String::new(),
            lhs_json: String::new(),
            rhs_json: String::new(),
            parameter_bindings: BTreeMap::new(),
            rule_layout: BTreeMap::new(),
            lod_mode_by_window: BTreeMap::new(),
            before_pane_camera: Camera::default(),
            reorganize_epoch: 0,
            locale: "en-US".into(),
        }
    }
}

impl RewriteArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::rewrite::RewriteSnapshot {
        crate::artifacts::rewrite::RewriteSnapshot {
            before_fixture_json: self.before_fixture_json.clone(),
            lhs_json: self.lhs_json.clone(),
            rhs_json: self.rhs_json.clone(),
            parameter_bindings: self.parameter_bindings.clone(),
            rule_layout: self.rule_layout.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::rewrite::RewriteSnapshot) -> Self {
        Self { before_fixture_json: snapshot.before_fixture_json, lhs_json: snapshot.lhs_json, rhs_json: snapshot.rhs_json, parameter_bindings: snapshot.parameter_bindings, rule_layout: snapshot.rule_layout, ..Self::default() }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::rewrite::RewriteSnapshot) {
        self.before_fixture_json = snapshot.before_fixture_json;
        self.lhs_json = snapshot.lhs_json;
        self.rhs_json = snapshot.rhs_json;
        self.parameter_bindings = snapshot.parameter_bindings;
        self.rule_layout = snapshot.rule_layout;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.trinity.rewrite` — twenty handcrafted schema leaves.
pub fn rewrite_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.trinity.rewrite",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

//#region 🔖️RuleApplication
/// ◀️ Left-hand side pattern for rewriting.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Lhs {
    pub pattern: PatternJson,
    #[value(default)]
    pub where_clause: Option<String>,
}

/// 🏷️ Parameter kind for parametric rewrite rules.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum ParameterKind {
    String,
    Number,
    Boolean,
}

/// 🎛️ Parameter declaration on the right-hand side.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ParameterSpec {
    pub name: String,
    pub kind: ParameterKind,
    pub default: PropertyValue,
}

/// ▶️ Right-hand side mutation for rewriting.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Rhs {
    #[value(default)]
    pub create: Vec<PatternJson>,
    #[value(default)]
    pub delete: Vec<String>,
    #[value(default)]
    pub set: Vec<AssignmentJson>,
    #[value(default)]
    pub merge: Vec<PatternJson>,
    #[value(default)]
    pub parameters: Vec<ParameterSpec>,
}

/// 📜️ Rewrite rule.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Rule {
    pub name: String,
    pub lhs: Lhs,
    pub rhs: Rhs,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PatternJson {
    pub left_var: String,
    pub left_kind: String,
    #[value(default)]
    pub edge_var: Option<String>,
    #[value(default)]
    pub edge_kind: Option<String>,
    #[value(default)]
    pub right_var: Option<String>,
    #[value(default)]
    pub right_kind: Option<String>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct AssignmentJson {
    pub var: String,
    pub prop: String,
    pub value: PropertyValue,
}

impl PatternJson {
    fn to_jack_pattern(&self) -> Pattern {
        let left = PatternNode { var: self.left_var.clone(), kind: self.left_kind.clone() };
        if let (Some(right_var), Some(right_kind)) = (&self.right_var, &self.right_kind) {
            Pattern { nodes: vec![left], edge: Some(PatternEdge { var: self.edge_var.clone(), kind: self.edge_kind.clone(), directed: true, right: PatternNode { var: right_var.clone(), kind: right_kind.clone() } }) }
        } else {
            Pattern { nodes: vec![left], edge: None }
        }
    }
}

fn pattern_to_match_clause(pattern: &PatternJson) -> String {
    let p = pattern.to_jack_pattern();
    let left = format!("({}:{} )", p.nodes[0].var, p.nodes[0].kind).replace(" )", ")");
    if let Some(edge) = &p.edge {
        let edge_mid = match (&edge.var, &edge.kind) {
            (Some(v), Some(k)) => format!("[{v}:{k}]"),
            (Some(v), None) => format!("[{v}]"),
            (None, Some(k)) => format!("[:{k}]"),
            (None, None) => "[]".into(),
        };
        format!("({}:{} )-{edge_mid}->({}:{} )", p.nodes[0].var, p.nodes[0].kind, edge.right.var, edge.right.kind).replace(" )", ")")
    } else {
        left
    }
}

pub(crate) fn parse_bindings_json(bindings_json: &str) -> Result<BTreeMap<String, PropertyValue>, TrinityRewriteError> {
    if bindings_json.trim().is_empty() {
        return Ok(BTreeMap::new());
    }
    Ok(pack::from_json_str(bindings_json)?)
}

fn parameter_defaults(rule: &Rule) -> BTreeMap<String, PropertyValue> {
    let mut defaults = BTreeMap::new();
    for param in &rule.rhs.parameters {
        defaults.insert(param.name.clone(), param.default.clone());
    }
    defaults
}

fn effective_bindings(rule: &Rule, bindings: &BTreeMap<String, PropertyValue>) -> BTreeMap<String, PropertyValue> {
    let mut merged = parameter_defaults(rule);
    for (key, value) in bindings {
        merged.insert(key.clone(), value.clone());
    }
    merged
}

fn resolve_parameter_value(rule: &Rule, bindings: &BTreeMap<String, PropertyValue>, value: &PropertyValue) -> PropertyValue {
    if let PropertyValue::String(s) = value {
        if let Some(name) = s.strip_prefix('$') {
            if !name.is_empty() {
                if let Some(resolved) = bindings.get(name) {
                    return resolved.clone();
                }
                for param in &rule.rhs.parameters {
                    if param.name == name {
                        return param.default.clone();
                    }
                }
            }
        }
    }
    value.clone()
}

/// 🩹️ unified syntax law: string literals PRINT double-quoted (never single-quoted) — matches the
/// shared `🫀️core` jack lexer/wire-literal printer, which accepts either quote style on parse but
/// always emits `"..."`.
fn assignment_value_jack(rule: &Rule, bindings: &BTreeMap<String, PropertyValue>, value: &PropertyValue) -> String {
    let resolved = resolve_parameter_value(rule, bindings, value);
    match resolved {
        PropertyValue::Null => "null".into(),
        PropertyValue::Bool(b) => b.to_string(),
        PropertyValue::Number(n) => n.to_string(),
        PropertyValue::String(s) => format!("\"{s}\""),
        PropertyValue::Array(_) | PropertyValue::Object(_) => pack::to_json_string(&resolved),
    }
}

/// 🧵️ Build the Jack query string for a rewrite rule without executing it.
pub fn build_rule_query(rule: &Rule, bindings: &BTreeMap<String, PropertyValue>) -> String {
    let effective = effective_bindings(rule, bindings);
    let mut query = format!("MATCH {}", pattern_to_match_clause(&rule.lhs.pattern));
    if let Some(where_clause) = &rule.lhs.where_clause {
        if !where_clause.trim().is_empty() {
            query.push_str(&format!(" WHERE {where_clause}"));
        }
    }
    for del in &rule.rhs.delete {
        query.push_str(&format!(" DELETE {del}"));
    }
    for set in &rule.rhs.set {
        let val = assignment_value_jack(rule, &effective, &set.value);
        query.push_str(&format!(" SET {}.{} = {val}", set.var, set.prop));
    }
    for create in &rule.rhs.create {
        query.push_str(&format!(" CREATE {}", pattern_to_match_clause(create)));
    }
    for merge in &rule.rhs.merge {
        query.push_str(&format!(" MERGE {}", pattern_to_match_clause(merge)));
    }
    query
}

/// ♻️ Apply a rewrite rule to a graph.
pub fn apply_rule(graph: &mut Graph, rule: &Rule, bindings: &BTreeMap<String, PropertyValue>) -> Result<QueryResult, TrinityRewriteError> {
    let query = build_rule_query(rule, bindings);
    let parsed = parse(&query).map_err(TrinityRewriteError::Jack)?;
    let (result, operations) = execute(graph, &parsed).map_err(TrinityRewriteError::Jack)?;
    if !operations.is_empty() {
        let fixture = crate::artifacts::jack::op::apply_trinity_graph_mutations(graph.to_fixture(), &operations)?;
        *graph = Graph::from_fixture(fixture)?;
    }
    Ok(result)
}

/// ♻️ Apply a rewrite rule from JSON.
pub fn apply_rule_json(graph: &mut Graph, rule_json: &str, bindings_json: &str) -> Result<String, TrinityRewriteError> {
    let rule: Rule = pack::from_json_str(rule_json)?;
    let bindings = parse_bindings_json(bindings_json)?;
    let result = apply_rule(graph, &rule, &bindings)?;
    Ok(pack::to_json_string(&ApplyRuleResult { fixture: graph.fixture_json()?, query: result }))
}

/// 🧵️ Build a rewrite rule Jack query from JSON without a graph.
pub fn rule_query_json(rule_json: &str, bindings_json: &str) -> Result<String, TrinityRewriteError> {
    let rule: Rule = pack::from_json_str(rule_json)?;
    let bindings = parse_bindings_json(bindings_json)?;
    let query = build_rule_query(&rule, &bindings);
    Ok(pack::to_json_string(&RuleQueryResult { query }))
}

#[derive(value_derive::ToValue)]
#[value(rename_all = "camelCase")]
pub struct ApplyRuleResult {
    pub fixture: String,
    pub query: QueryResult,
}

#[derive(value_derive::ToValue)]
#[value(rename_all = "camelCase")]
pub struct RuleQueryResult {
    pub query: String,
}
//#endregion 🔖️RuleApplication

//#region 🧪️RuleApplicationTests
#[cfg(test)]
mod rule_application_tests {
    use super::*;
    use crate::artifacts::jack::dsl::NAKAGIN_EXAMPLE_TEXT;
    use store::ArtifactDsl;

    fn nakagin_graph() -> Graph {
        Graph::from_fixture(crate::artifacts::jack::JackSnapshot::parse_dsl(NAKAGIN_EXAMPLE_TEXT).unwrap()).unwrap()
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
        let result = crate::executor::run(&mut g, "MATCH (a:Piece) WHERE a.name = 'b' RETURN a.name").unwrap();
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
    async fn rewrite_labeled_fixture_reloads() {
        let mut g = Graph::from_fixture(crate::artifacts::jack::JackSnapshot::parse_dsl(NAKAGIN_EXAMPLE_TEXT).unwrap()).unwrap();
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
}
//#endregion 🧪️RuleApplicationTests

//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::rewrite::{RewriteDiff, RewriteRuleMutation, RewriteSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct RewriteBuilderConstruction {
        snapshot: RewriteSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for RewriteBuilderConstruction {
        type Snapshot = RewriteSnapshot;
        type Mutation = RewriteRuleMutation;
        type Diff = RewriteDiff;
        fn empty() -> Self {
            Self { snapshot: RewriteSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<RewriteSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<RewriteSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <RewriteDiff as protocol::MutationDiff<RewriteSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::rewrite::RewriteSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct RewriteParts {
        pub snapshot: Option<RewriteSnapshot>,
    }

    pub struct RewriteAnalyzerAnalysis;

    impl ArtifactAnalysis for RewriteAnalyzerAnalysis {
        type Parts = RewriteParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.rewrite", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = RewriteParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <RewriteSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <RewriteSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec RewriteBuilderFacets {
        construction: RewriteBuilderConstruction,
        analysis: RewriteAnalyzerAnalysis,
        composition: super::super::io::derived_composition::RewriteComposerComposition,
    }
    builder: RewriteBuilder,
    analyzer: RewriteAnalyzer,
    composer: RewriteComposer,
);
//#endregion 🧬️DerivedArtifactFacets
