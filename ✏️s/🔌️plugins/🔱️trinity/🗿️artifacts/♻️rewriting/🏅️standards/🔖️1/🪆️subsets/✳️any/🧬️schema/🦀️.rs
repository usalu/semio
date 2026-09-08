//! 🧬️ Rewriting artifact schema — every field of the artifact with its state class.

use semio_s_artifact_trinity_jack::{Graph};
use crate::{TrinityRewritingError};
use semio_s_artifact_trinity_jack::ast::{Pattern, PatternEdge, PatternNode, QueryResult};
use semio_s_artifact_trinity_jack::executor::execute;
use semio_s_artifact_trinity_jack::language_service::parse;
use ::semio_framework_schema::ArtifactSchema;
use std::collections::BTreeMap;

//#region 🔖️Artifact
/// 🧬️ Full rewriting artifact state across the artifact and local config lanes.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.trinity.rewriting")]
pub struct RewritingArtifact {
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
    #[state(config)]
    pub lod_mode_by_window: BTreeMap<String, String>,
    #[state(config)]
    pub before_pane_camera: Camera,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for RewritingArtifact {
    fn default() -> Self {
        Self {
            before_fixture_json: String::new(),
            lhs_json: String::new(),
            rhs_json: String::new(),
            parameter_bindings: BTreeMap::new(),
            rule_layout: BTreeMap::new(),
            lod_mode_by_window: BTreeMap::new(),
            before_pane_camera: Camera::default(),
        }
    }
}

impl RewritingArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::RewritingSnapshot {
        crate::RewritingSnapshot {
            before_fixture_json: self.before_fixture_json.clone(),
            lhs_json: self.lhs_json.clone(),
            rhs_json: self.rhs_json.clone(),
            parameter_bindings: self.parameter_bindings.clone(),
            rule_layout: self.rule_layout.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::RewritingSnapshot) -> Self {
        Self { before_fixture_json: snapshot.before_fixture_json, lhs_json: snapshot.lhs_json, rhs_json: snapshot.rhs_json, parameter_bindings: snapshot.parameter_bindings, rule_layout: snapshot.rule_layout, ..Self::default() }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::RewritingSnapshot) {
        self.before_fixture_json = snapshot.before_fixture_json;
        self.lhs_json = snapshot.lhs_json;
        self.rhs_json = snapshot.rhs_json;
        self.parameter_bindings = snapshot.parameter_bindings;
        self.rule_layout = snapshot.rule_layout;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.trinity.rewriting` — twenty handcrafted schema leaves.
pub fn rewriting_artifact_schema_descriptor() -> ::semio_framework_schema::ArtifactSchemaDescriptor {
    ::semio_framework_schema::ArtifactSchemaDescriptor {
        id: "s.trinity.rewriting",
        artifact: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
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

pub(crate) fn parse_bindings_json(bindings_json: &str) -> Result<BTreeMap<String, PropertyValue>, TrinityRewritingError> {
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
pub fn apply_rule(graph: &mut Graph, rule: &Rule, bindings: &BTreeMap<String, PropertyValue>) -> Result<QueryResult, TrinityRewritingError> {
    let query = build_rule_query(rule, bindings);
    let parsed = parse(&query).map_err(TrinityRewritingError::Jack)?;
    let (result, operations) = execute(graph, &parsed).map_err(TrinityRewritingError::Jack)?;
    if !operations.is_empty() {
        let fixture = semio_s_artifact_trinity_jack::apply_trinity_graph_mutations(graph.to_fixture(), &operations)?;
        *graph = Graph::from_fixture(fixture)?;
    }
    Ok(result)
}

/// ♻️ Apply a rewrite rule from JSON.
pub fn apply_rule_json(graph: &mut Graph, rule_json: &str, bindings_json: &str) -> Result<String, TrinityRewritingError> {
    let rule: Rule = pack::from_json_str(rule_json)?;
    let bindings = parse_bindings_json(bindings_json)?;
    let result = apply_rule(graph, &rule, &bindings)?;
    Ok(pack::to_json_string(&ApplyRuleResult { fixture: graph.fixture_json()?, query: result }))
}

/// 🧵️ Build a rewrite rule Jack query from JSON without a graph.
pub fn rule_query_json(rule_json: &str, bindings_json: &str) -> Result<String, TrinityRewritingError> {
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
#[path = "🧪️tests/🔬️rule-application/🦀️.rs"]
mod rule_application_tests;
//#endregion 🧪️RuleApplicationTests

//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::{RewritingDiff, RewriteRuleMutation, RewritingSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct RewritingBuilderConstruction {
        snapshot: RewritingSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for RewritingBuilderConstruction {
        type Snapshot = RewritingSnapshot;
        type Mutation = RewriteRuleMutation;
        type Diff = RewritingDiff;
        fn empty() -> Self {
            Self { snapshot: RewritingSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<RewritingSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<RewritingSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
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
            let snapshot = <RewritingDiff as protocol::MutationDiff<RewritingSnapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::RewritingSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct RewritingParts {
        pub snapshot: Option<RewritingSnapshot>,
    }

    pub struct RewritingAnalyzerAnalysis;

    impl ArtifactAnalysis for RewritingAnalyzerAnalysis {
        type Parts = RewritingParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.trinity.rewriting", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = RewritingParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <RewritingSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <RewritingSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec RewritingBuilderFacets {
        construction: RewritingBuilderConstruction,
        analysis: RewritingAnalyzerAnalysis,
        composition: super::super::io::derived_composition::RewritingComposerComposition,
    }
    builder: RewritingBuilder,
    analyzer: RewritingAnalyzer,
    composer: RewritingComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use semio_s_artifact_trinity_jack::PropertyValue;
pub use crate::LayoutPoint;
pub use semio_s_artifact_trinity_jack::Camera;
//#endregion 🔁️Re-exports
