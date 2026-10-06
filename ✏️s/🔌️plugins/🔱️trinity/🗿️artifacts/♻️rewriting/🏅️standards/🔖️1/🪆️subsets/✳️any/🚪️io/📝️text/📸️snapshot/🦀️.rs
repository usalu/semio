//! 📜️ `trinity.rewrite.rule` artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::RewritingSnapshot;
use store::ArtifactDsl;

/// 📄️ The bundled Nakagin `label-core` rewrite rule, handcrafted in the `.rewriting` DSL — mirrors the
/// `trinity-rewriting` app's own real default rule over a trimmed two-node/one-edge slice of the
/// bundled `🔱️nakagin-capsule-tower.trinity` working-graph.
pub const NAKAGIN_LABEL_CORE_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.rewriting` DSL text into a `RewritingSnapshot`.
pub fn parse_dsl(text: &str) -> Result<RewritingSnapshot, semio_framework_diagnostic::TextError> {
    <RewritingSnapshot as ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `RewritingSnapshot` back to `.rewriting` DSL text.
pub fn print_dsl(document: &RewritingSnapshot) -> String {
    ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type RewritingSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::LayoutPoint;
use ::semio_framework_schema::ArtifactSchema;
use std::collections::BTreeMap;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use semio_framework_graph::manifest::PropertyValue;

/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for RewritingSnapshot {
    const EXTENSION: &'static str = "rewriting";
    fn envelope_id() -> &'static str {
        "trinity.rewriting"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📤️ Renders the declared typed rule, Jack child handle, exact words and keyed maps as JSON.
pub fn encode_rewriting_snapshot_json(snapshot: &RewritingSnapshot) -> Result<String, semio_framework_value::ValueError> {
    let value = json::convert(semio_framework_value::ToValue::to_value(snapshot), false)?;
    Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&value)))
}

/// 📥️ The inverse of [`encode_rewriting_snapshot_json`] — decodes those committed specification
/// vectors into real [`RewritingSnapshot`] values, so `mutate-rewriting-1`'s adapter reads the committed
/// snapshot rather than re-declaring it as a Rust literal beside it. Reaching a JSON library from that
/// adapter is impossible: the generated test host links only this crate and `semio-repo-test-host`.
pub fn decode_rewriting_snapshot_json(text: &str) -> Result<RewritingSnapshot, semio_framework_value::ValueError> {
    let value = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))?;
    let value = json::convert(semio_framework_pack_json::to_dsl_value(&value), true)?;
    <RewritingSnapshot as semio_framework_value::FromValue>::from_value(value)
}

/// 📝️ Parses `.rewriting.dsl.semio` text into a [`RewritingSnapshot`] — a named, non-async pass-through
/// of this type's own handcrafted `store::ArtifactDsl` impl above, whose trait and error type are
/// both unnameable outside this crate, so `mutate-rewriting-1`'s `identity-round-trip` scenario
/// reaches the real committed artifact
/// (`../../🖼️assets/🎬️demo/🗣️.dsl.semio`) through this instead.
pub fn parse_rewriting_dsl(text: &str) -> Result<RewritingSnapshot, String> {
    <RewritingSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 📝️ Renders the declared typed Rewriting document through its owned DSL facet.
pub fn print_rewriting_dsl(snapshot: &RewritingSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::*;
use crate::TrinityRewritingError;
use ::semio_framework_schema::ArtifactSchema;
use semio_s_artifact_trinity_jack::ast::{Pattern as JackPattern, PatternEdge, PatternNode, QueryResult};
use semio_s_artifact_trinity_jack::executor::execute;
use semio_s_artifact_trinity_jack::language_service::parse;
use semio_s_artifact_trinity_jack::Graph;
use std::collections::BTreeMap;
use layout::RuleLayout;
use rule::{Pattern,Lhs,Rhs,Assignment,ParameterKind,ParameterSpec,Rule};
use derived_construction::*;
use derived_analysis::*;
use crate::LayoutPoint;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use semio_framework_graph::manifest::PropertyValue;
use semio_s_artifact_trinity_jack::Camera;

pub(crate) fn parse_bindings_json(bindings_json: &str) -> Result<semio_framework_graph::manifest::PropertyBag, TrinityRewritingError> {
    if bindings_json.trim().is_empty() {
        return Ok(semio_framework_graph::manifest::PropertyBag::new());
    }
    Ok(semio_framework_pack_json::from_json_str(bindings_json, semio_framework_pack_json::JsonMemberPolicy::Reject)?)
}
}
pub use snapshot_wire_codec::*;

#[allow(unused_imports)]
mod snapshot_wire2_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::*;
use crate::TrinityRewritingError;
use ::semio_framework_schema::ArtifactSchema;
use semio_s_artifact_trinity_jack::ast::{Pattern as JackPattern, PatternEdge, PatternNode, QueryResult};
use semio_s_artifact_trinity_jack::executor::execute;
use semio_s_artifact_trinity_jack::language_service::parse;
use semio_s_artifact_trinity_jack::Graph;
use std::collections::BTreeMap;
use layout::RuleLayout;
use rule::{Pattern,Lhs,Rhs,Assignment,ParameterKind,ParameterSpec,Rule};
use crate::LayoutPoint;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use semio_framework_graph::manifest::PropertyValue;
use semio_s_artifact_trinity_jack::Camera;

pub(crate) fn pattern_to_match_clause(pattern: &Pattern) -> String {
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

pub(crate) fn parameter_defaults(rule: &Rule) -> semio_framework_graph::manifest::PropertyBag {
    let mut defaults = semio_framework_graph::manifest::PropertyBag::new();
    for param in &rule.rhs.parameters {
        defaults.insert(param.name.clone(), param.default.clone());
    }
    defaults
}

pub(crate) fn effective_bindings(rule: &Rule, bindings: &semio_framework_graph::manifest::PropertyBag) -> semio_framework_graph::manifest::PropertyBag {
    let mut merged = parameter_defaults(rule);
    for (key, value) in bindings {
        merged.insert(key.clone(), value.clone());
    }
    merged
}

pub(crate) fn resolve_parameter_value(rule: &Rule, bindings: &semio_framework_graph::manifest::PropertyBag, value: &PropertyValue) -> PropertyValue {
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
pub(crate) fn assignment_value_jack(rule: &Rule, bindings: &semio_framework_graph::manifest::PropertyBag, value: &PropertyValue) -> String {
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
pub fn build_rule_query(rule: &Rule, bindings: &semio_framework_graph::manifest::PropertyBag) -> String {
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
    Ok(semio_framework_pack_json::to_json_string(&ApplyRuleResult { snapshot_json: graph.host_snapshot_json()?, query: result }))
}

/// 🧵️ Build a rewrite rule Jack query from JSON without a graph.
pub fn rule_query_json(rule_json: &str, bindings_json: &str) -> Result<String, TrinityRewritingError> {
    let rule: Rule = semio_framework_pack_json::from_json_str(rule_json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
    let bindings = parse_bindings_json(bindings_json)?;
    let query = build_rule_query(&rule, &bindings);
    Ok(semio_framework_pack_json::to_json_string(&RuleQueryResult { query }))
}
}
pub use snapshot_wire2_codec::*;
