//! ⚙️ `trinity.rewrite.rule` artifact — headless compute: parametric graph rewriting over the jack
//! query language (constitutional: engine).

use crate::artifacts::jack::{Graph, PropertyValue};
use crate::artifacts::rewrite::TrinityRewriteError;
use crate::ast::{Pattern, PatternEdge, PatternNode, QueryResult};
use crate::executor::execute;
use crate::language_service::parse;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Rewrite
/// ◀️ Left-hand side pattern for rewriting.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lhs {
    pub pattern: PatternJson,
    #[serde(default)]
    pub where_clause: Option<String>,
}

/// 🏷️ Parameter kind for parametric rewrite rules.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ParameterKind {
    String,
    Number,
    Boolean,
}

/// 🎛️ Parameter declaration on the right-hand side.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterSpec {
    pub name: String,
    pub kind: ParameterKind,
    pub default: PropertyValue,
}

/// ▶️ Right-hand side mutation for rewriting.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rhs {
    #[serde(default)]
    pub create: Vec<PatternJson>,
    #[serde(default)]
    pub delete: Vec<String>,
    #[serde(default)]
    pub set: Vec<AssignmentJson>,
    #[serde(default)]
    pub merge: Vec<PatternJson>,
    #[serde(default)]
    pub parameters: Vec<ParameterSpec>,
}

/// 📜️ Rewrite rule.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    pub name: String,
    pub lhs: Lhs,
    pub rhs: Rhs,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatternJson {
    pub left_var: String,
    pub left_kind: String,
    #[serde(default)]
    pub edge_var: Option<String>,
    #[serde(default)]
    pub edge_kind: Option<String>,
    #[serde(default)]
    pub right_var: Option<String>,
    #[serde(default)]
    pub right_kind: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    Ok(serde_json::from_str(bindings_json)?)
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
        PropertyValue::Array(_) | PropertyValue::Object(_) => serde_json::to_string(&resolved).unwrap_or_else(|_| "null".into()),
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
    let rule: Rule = serde_json::from_str(rule_json)?;
    let bindings = parse_bindings_json(bindings_json)?;
    let result = apply_rule(graph, &rule, &bindings)?;
    Ok(serde_json::to_string(&ApplyRuleResult { fixture: graph.fixture_json()?, query: result })?)
}

/// 🧵️ Build a rewrite rule Jack query from JSON without a graph.
pub fn rule_query_json(rule_json: &str, bindings_json: &str) -> Result<String, TrinityRewriteError> {
    let rule: Rule = serde_json::from_str(rule_json)?;
    let bindings = parse_bindings_json(bindings_json)?;
    let query = build_rule_query(&rule, &bindings);
    Ok(serde_json::to_string(&RuleQueryResult { query })?)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyRuleResult {
    pub fixture: String,
    pub query: QueryResult,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleQueryResult {
    pub query: String,
}
//#endregion 🔖️Rewrite

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::jack::dsl::NAKAGIN_EXAMPLE_TEXT;
    use store::DocumentDsl;

    fn nakagin_graph() -> Graph {
        let mut g = Graph::from_fixture(crate::artifacts::jack::GraphFixture::parse_dsl(NAKAGIN_EXAMPLE_TEXT).unwrap()).unwrap();
        g.recompute_derived();
        g
    }

    fn empty_rule() -> Rule {
        Rule {
            name: "r".into(),
            lhs: Lhs { pattern: PatternJson { left_var: "a".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None }, where_clause: None },
            rhs: Rhs { create: vec![], delete: vec![], set: vec![], merge: vec![], parameters: vec![] },
        }
    }

    #[test]
    fn jack_query_on_nakagin() {
        let mut g = nakagin_graph();
        let result = crate::executor::run(&mut g, "MATCH (a:Piece) WHERE a.name = 'b' RETURN a.name").unwrap();
        assert_eq!(result.rows.len(), 1);
    }

    #[test]
    fn rewrite_rule_labels_core() {
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

    #[test]
    fn rewrite_rule_parameter_substitution() {
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

    #[test]
    fn rewrite_labeled_fixture_reloads() {
        let mut g = Graph::from_fixture(crate::artifacts::jack::GraphFixture::parse_dsl(NAKAGIN_EXAMPLE_TEXT).unwrap()).unwrap();
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

    #[test]
    fn pattern_to_match_clause_edge_variants() {
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

    #[test]
    fn build_rule_query_edge_pattern_and_all_clauses() {
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

    #[test]
    fn resolve_parameter_value_variants() {
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

    #[test]
    fn assignment_value_jack_formats_each_property_variant() {
        let rule = empty_rule();
        let bindings = BTreeMap::new();
        assert_eq!(assignment_value_jack(&rule, &bindings, &PropertyValue::Null), "null");
        assert_eq!(assignment_value_jack(&rule, &bindings, &PropertyValue::Bool(true)), "true");
        assert_eq!(assignment_value_jack(&rule, &bindings, &PropertyValue::Number(4.5)), "4.5");
        assert_eq!(assignment_value_jack(&rule, &bindings, &PropertyValue::String("hi".into())), "\"hi\"");
        let arr = PropertyValue::Array(vec![PropertyValue::Number(1.0)]);
        assert_eq!(assignment_value_jack(&rule, &bindings, &arr), serde_json::to_string(&arr).unwrap());
    }

    #[test]
    fn parse_bindings_json_handles_empty_and_invalid() {
        assert_eq!(parse_bindings_json("").unwrap(), BTreeMap::new());
        assert_eq!(parse_bindings_json("   ").unwrap(), BTreeMap::new());
        assert!(parse_bindings_json("{not json").is_err());
        let mut expected = BTreeMap::new();
        expected.insert("x".to_string(), PropertyValue::Number(1.0));
        assert_eq!(parse_bindings_json("{\"x\":1}").unwrap(), expected);
    }

    #[test]
    fn apply_rule_json_and_rule_query_json_end_to_end() {
        let mut g = nakagin_graph();
        let mut rule = empty_rule();
        rule.name = "label-core".into();
        rule.lhs.where_clause = Some("a.name = 'b'".into());
        rule.rhs.set.push(AssignmentJson { var: "a".into(), prop: "label".into(), value: PropertyValue::String("nakagin-core".into()) });
        let rule_json = serde_json::to_string(&rule).unwrap();

        let query_out = rule_query_json(&rule_json, "{}").unwrap();
        let query_value: serde_json::Value = serde_json::from_str(&query_out).unwrap();
        assert!(query_value["query"].as_str().unwrap().contains("SET a.label"));

        let apply_out = apply_rule_json(&mut g, &rule_json, "{}").unwrap();
        let apply_value: serde_json::Value = serde_json::from_str(&apply_out).unwrap();
        assert!(apply_value.get("fixture").is_some());
        let core = g.node("7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
        assert_eq!(core.properties.get("label"), Some(&PropertyValue::String("nakagin-core".into())));

        assert!(apply_rule_json(&mut g, "not json", "{}").is_err());
        assert!(rule_query_json("not json", "{}").is_err());
    }
}
//#endregion 🧪️Tests


/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "rewrite.document",
        extension: Some("rewrite"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::rewrite::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::rewrite::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::rewrite::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::rewrite::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("rewrite.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "rewrite.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::rewrite::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::rewrite::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::rewrite::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::rewrite::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("rewrite.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "rewrite.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::rewrite::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::rewrite::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("rewrite.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "rewrite.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::rewrite::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::rewrite::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("rewrite.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "rewrite.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::rewrite::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::rewrite::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("rewrite.spr"),
    });
}


//#region 🔖️ArtifactEngine
pub struct RewriteRuleEngine {
    projection: crate::artifacts::rewrite::RewriteRuleDocument,
}

impl RewriteRuleEngine {
    pub fn new(projection: crate::artifacts::rewrite::RewriteRuleDocument) -> Self {
        Self { projection }
    }
}

impl protocol::ArtifactEngine for RewriteRuleEngine {
    type Projection = crate::artifacts::rewrite::RewriteRuleDocument;
    type Mutation = crate::artifacts::rewrite::mutations::RewriteRuleMutation;
    type Diff = crate::artifacts::rewrite::diff::RewriteRuleDiff;

    fn projection(&self) -> &Self::Projection {
        &self.projection
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Projection>>::diff(mutation, &self.projection);
        crate::artifacts::rewrite::mutations::apply_rewrite_rule_mutation(&mut self.projection, mutation);
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Projection>>::inverse(mutation, &self.projection)
    }
}
//#endregion 🔖️ArtifactEngine
