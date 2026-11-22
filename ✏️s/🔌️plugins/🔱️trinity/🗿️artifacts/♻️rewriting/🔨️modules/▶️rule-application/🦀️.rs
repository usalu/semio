//! ▶️ Rewriting rule application: builds the Jack query of a rule, applies it to a graph and exposes both as JSON entry points.

use crate::standards::v1::subsets::any::schema::errors::TrinityRewritingError;
use crate::standards::v1::subsets::any::schema::rule::{default_parameter_bindings, Pattern, Rule};
use semio_framework_graph_manifest::{PropertyBag, PropertyValue};
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::ast::{Assignment, Clause, JackQueryGraphSnapshot, Pattern as JackPattern, Query, QueryResult};
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::executor::execute;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::graph::GraphEffect;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::language_service::parse;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::lexer::{Token, lex};
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::memory::Graph;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::operations::apply_graph_effects;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot;

/// 🧾️ Compiles a rule to the typed program used by Jack's stepped evaluator.
pub fn compile_rule(rule: &Rule, bindings: &PropertyBag) -> Result<Query, TrinityRewritingError> {
    for pattern in std::iter::once(&rule.lhs.pattern).chain(&rule.rhs.create).chain(&rule.rhs.merge) {
        validate_pattern(pattern)?;
    }
    let mut clauses = vec![Clause::Match(vec![rule.lhs.pattern.to_jack_pattern()])];
    if let Some(predicate) = rule_predicate(rule) {
        let mut filter = parse(&format!("WHERE {predicate}")).map_err(TrinityRewritingError::Jack)?.clauses;
        if !matches!(filter.as_slice(), [Clause::Where(_)]) {
            return Err(TrinityRewritingError::Jack("A rewrite filter must be a single WHERE predicate.".into()));
        }
        clauses.append(&mut filter);
    }
    clauses.extend(rhs_clauses(rule, bindings));
    Ok(Query { clauses })
}

/// 🗺️ Plan the ordered effects of a rewrite against the supplied graph.
pub fn plan_rule(graph: &Graph, rule: &Rule, bindings: &PropertyBag) -> Result<(QueryResult, Vec<GraphEffect>), TrinityRewritingError> {
    execute(graph, &compile_rule(rule, bindings)?).map_err(TrinityRewritingError::Jack)
}

/// ♻️ Apply a rewrite rule to a graph.
pub fn apply_rule(graph: &mut Graph, rule: &Rule, bindings: &PropertyBag) -> Result<QueryResult, TrinityRewritingError> {
    let (result, effects) = plan_rule(graph, rule, bindings)?;
    apply_graph_effects(graph, &effects)?;
    Ok(result)
}

fn validate_pattern(pattern: &Pattern) -> Result<(), TrinityRewritingError> {
    let has_edge = pattern.edge_var.is_some() || pattern.edge_kind.is_some();
    match (&pattern.right_var, &pattern.right_kind) {
        (Some(var), Some(_)) if pattern.edge_var.as_ref().is_some_and(|edge| edge == &pattern.left_var || edge == var) => {
            Err(TrinityRewritingError::Jack("A rewrite edge and its endpoints must have distinct variables.".into()))
        }
        (Some(_), Some(_)) => Ok(()),
        (None, None) if !has_edge => Ok(()),
        _ => Err(TrinityRewritingError::Jack("A rewrite edge requires both a right variable and a right kind.".into())),
    }
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

pub fn parse_bindings_json(bindings_json: &str) -> Result<PropertyBag, TrinityRewritingError> {
    if bindings_json.trim().is_empty() {
        return Ok(PropertyBag::new());
    }
    Ok(semio_framework_pack_json::from_json_str(bindings_json, semio_framework_pack_json::JsonMemberPolicy::Reject)?)
}

pub fn pattern_to_match_clause(pattern: &Pattern) -> String {
    format_pattern(&pattern.to_jack_pattern())
}

fn format_pattern(pattern: &JackPattern) -> String {
    let left = &pattern.nodes[0];
    let mut text = format!("({}:{})", left.var, format_kind(&left.kind));
    if let Some(edge) = &pattern.edge {
        let edge_text = match (&edge.var, &edge.kind) {
            (Some(var), Some(kind)) => format!("[{var}:{}]", format_kind(kind)),
            (Some(var), None) => format!("[{var}]"),
            (None, Some(kind)) => format!("[:{}]", format_kind(kind)),
            (None, None) => "[]".to_owned(),
        };
        text.push_str(&format!("-{edge_text}->({}:{})", edge.right.var, format_kind(&edge.right.kind)));
    }
    text
}

fn format_kind(kind: &str) -> String {
    if matches!(lex(kind).as_deref(), Ok([Token::Ident(identifier), Token::Eof]) if identifier == kind) {
        return kind.to_owned();
    }
    semio_framework_pack_json::to_json_string(&kind)
}

pub fn parameter_defaults(rule: &Rule) -> PropertyBag {
    default_parameter_bindings(&rule.rhs)
}

pub fn effective_bindings(rule: &Rule, bindings: &PropertyBag) -> PropertyBag {
    let mut merged = parameter_defaults(rule);
    for (key, value) in bindings {
        merged.insert(key.clone(), value.clone());
    }
    merged
}

pub fn resolve_parameter_value(rule: &Rule, bindings: &PropertyBag, value: &PropertyValue) -> PropertyValue {
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
pub fn assignment_value_jack(rule: &Rule, bindings: &PropertyBag, value: &PropertyValue) -> String {
    format_value(&resolve_parameter_value(rule, bindings, value))
}

fn format_value(value: &PropertyValue) -> String {
    match value {
        PropertyValue::Number(n) => n.to_string(),
        _ => semio_framework_pack_json::to_json_string(value),
    }
}

fn rule_predicate(rule: &Rule) -> Option<&str> {
    rule.lhs.where_clause.as_deref().filter(|predicate| !predicate.trim().is_empty())
}

fn rhs_clauses(rule: &Rule, bindings: &PropertyBag) -> Vec<Clause> {
    let effective = effective_bindings(rule, bindings);
    let mut clauses = Vec::new();
    if !rule.rhs.delete.is_empty() {
        clauses.push(Clause::Delete(rule.rhs.delete.clone()));
    }
    clauses.extend(rule.rhs.create.iter().map(|pattern| Clause::Create(pattern.to_jack_pattern())));
    clauses.extend(rule.rhs.merge.iter().map(|pattern| Clause::Merge(pattern.to_jack_pattern())));
    if !rule.rhs.set.is_empty() {
        clauses.push(Clause::Set(rule.rhs.set.iter().map(|item| Assignment {
            var: item.var.clone(),
            prop: item.prop.clone(),
            value: resolve_parameter_value(rule, &effective, &item.value),
        }).collect()));
    }
    clauses
}

/// 🧵️ Build the Jack query string for a rewrite rule without executing it.
pub fn build_rule_query(rule: &Rule, bindings: &PropertyBag) -> String {
    let mut query = format!("MATCH {}", pattern_to_match_clause(&rule.lhs.pattern));
    if let Some(predicate) = rule_predicate(rule) {
        query.push_str(&format!(" WHERE {predicate}"));
    }
    for clause in rhs_clauses(rule, bindings) {
        match clause {
            Clause::Delete(vars) => query.push_str(&format!(" DELETE {}", vars.join(", "))),
            Clause::Create(pattern) => query.push_str(&format!(" CREATE {}", format_pattern(&pattern))),
            Clause::Merge(pattern) => query.push_str(&format!(" MERGE {}", format_pattern(&pattern))),
            Clause::Set(items) => {
                let assignments = items.iter().map(|item| format!("{}.{} = {}", item.var, item.prop, format_value(&item.value))).collect::<Vec<_>>();
                query.push_str(&format!(" SET {}", assignments.join(", ")));
            }
            _ => unreachable!("rewrite RHS contains only mutation clauses"),
        }
    }
    query
}

/// ♻️ Apply a rewrite rule from JSON.
pub fn apply_rule_json(graph: &mut Graph, rule_json: &str, bindings_json: &str) -> Result<String, TrinityRewritingError> {
    let rule: Rule = semio_framework_pack_json::from_json_str(rule_json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
    let bindings = parse_bindings_json(bindings_json)?;
    let result = apply_rule(graph, &rule, &bindings)?;
    let snapshot = JackQueryGraphSnapshot {
        schema: JackSnapshot::SCHEMA.into(),
        name: graph.name.clone(),
        manifest_id: graph.manifest_id.clone(),
        camera: graph.camera.clone(),
        nodes: graph.nodes.values().cloned().collect(),
        edges: graph.edges.values().cloned().collect(),
        root_node_id: graph.root_node_id.clone(),
        query: graph.query.clone(),
    };
    Ok(semio_framework_pack_json::to_json_string(&ApplyRuleResult { snapshot_json: snapshot.to_json()?, query: result }))
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
