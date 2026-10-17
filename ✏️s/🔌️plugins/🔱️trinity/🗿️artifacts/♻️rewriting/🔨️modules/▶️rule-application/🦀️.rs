//! ▶️ Rewriting rule application: builds the Jack query of a rule, applies it to a graph and exposes both as JSON entry points.

use crate::standards::v1::subsets::any::schema::errors::TrinityRewritingError;
use crate::standards::v1::subsets::any::schema::rule::{Assignment, Pattern, Rule};
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::ast::QueryResult;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::executor::execute;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::language_service::parse;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::memory::Graph;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::operations::apply_graph_effects;
use semio_framework_graph_manifest::PropertyValue;
use semio_s_artifact_trinity_jack_semio_content::SemioJackContent;

/// ♻️ Apply a rewrite rule to a graph.
pub fn apply_rule(graph: &mut Graph, rule: &Rule, bindings: &semio_framework_graph_manifest::PropertyBag) -> Result<QueryResult, TrinityRewritingError> {
    let query = build_rule_query(rule, bindings);
    let parsed = parse(&query).map_err(TrinityRewritingError::Jack)?;
    let (result, effects) = execute(graph, &parsed).map_err(TrinityRewritingError::Jack)?;
    apply_graph_effects(graph, &effects)?;
    Ok(result)
}





#[derive(value_derive::ToValue)]
#[value(rename_all = "camelCase")]
pub struct ApplyRuleResult {
    pub snapshot_json: String,
    pub query: QueryResult,
}

#[derive(value_derive::ToValue)]
#[value(rename_all = "camelCase")]
pub struct RuleQueryResult {
    pub query: String,
}

pub fn parse_bindings_json(bindings_json: &str) -> Result<semio_framework_graph_manifest::PropertyBag, TrinityRewritingError> {
    if bindings_json.trim().is_empty() {
        return Ok(semio_framework_graph_manifest::PropertyBag::new());
    }
    Ok(semio_framework_pack_json::from_json_str(bindings_json, semio_framework_pack_json::JsonMemberPolicy::Reject)?)
}

pub fn pattern_to_match_clause(pattern: &Pattern) -> String {
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

pub fn parameter_defaults(rule: &Rule) -> semio_framework_graph_manifest::PropertyBag {
    let mut defaults = semio_framework_graph_manifest::PropertyBag::new();
    for param in &rule.rhs.parameters {
        defaults.insert(param.name.clone(), param.default.clone());
    }
    defaults
}

pub fn effective_bindings(rule: &Rule, bindings: &semio_framework_graph_manifest::PropertyBag) -> semio_framework_graph_manifest::PropertyBag {
    let mut merged = parameter_defaults(rule);
    for (key, value) in bindings {
        merged.insert(key.clone(), value.clone());
    }
    merged
}

pub fn resolve_parameter_value(rule: &Rule, bindings: &semio_framework_graph_manifest::PropertyBag, value: &PropertyValue) -> PropertyValue {
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
pub fn assignment_value_jack(rule: &Rule, bindings: &semio_framework_graph_manifest::PropertyBag, value: &PropertyValue) -> String {
    let resolved = resolve_parameter_value(rule, bindings, value);
    match resolved {
        PropertyValue::Null => "null".into(),
        PropertyValue::Bool(b) => b.to_string(),
        PropertyValue::Number(n) => n.to_string(),
        PropertyValue::String(s) => format!("\"{s}\""),
        PropertyValue::Array(_) | PropertyValue::Object(_) => semio_framework_pack_json::to_json_string(&resolved),
    }
}

/// 🧵️ Build the Jack query string for a rewrite rule without executing it.
pub fn build_rule_query(rule: &Rule, bindings: &semio_framework_graph_manifest::PropertyBag) -> String {
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

/// ♻️ Apply a rewrite rule from JSON.
pub fn apply_rule_json(graph: &mut Graph, rule_json: &str, bindings_json: &str) -> Result<String, TrinityRewritingError> {
    let rule: Rule = semio_framework_pack_json::from_json_str(rule_json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
    let bindings = parse_bindings_json(bindings_json)?;
    let result = apply_rule(graph, &rule, &bindings)?;
    Ok(semio_framework_pack_json::to_json_string(&ApplyRuleResult { snapshot_json: graph.host_snapshot_json::<SemioJackContent>()?, query: result }))
}

/// 🧵️ Build a rewrite rule Jack query from JSON without a graph.
pub fn rule_query_json(rule_json: &str, bindings_json: &str) -> Result<String, TrinityRewritingError> {
    let rule: Rule = semio_framework_pack_json::from_json_str(rule_json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
    let bindings = parse_bindings_json(bindings_json)?;
    let query = build_rule_query(&rule, &bindings);
    Ok(semio_framework_pack_json::to_json_string(&RuleQueryResult { query }))
}

#[cfg(test)]
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️rule-application/🦀️.rs"]
mod tests;
