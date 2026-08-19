//! 🃏️ Shared Jack query language for graph frameworks.
//!
//! 🚚 Relocated verbatim from `🧰️framework/🔨️modules/🧮️math/🕸️graph/🗣️dsl` in ticket 26/08/12/
//! DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave MATHEND — see that wave's report
//! for why the framework/plugin split hypothesis was measured and rejected (real coupling between
//! `DslIdiom` and the language-service surface). `dsl_core` (crate-root alias for
//! `semio_framework_os_kernel`, renamed from `dsl` by this same wave to free that name for this
//! module) is this file's own `os_dsl`/`dsl`-derive dependency.

// #region ⚠️ Errors
/// 🚧️ Unified failure mode for jack parsing/execution, wire-literal parsing, and fixture ingestion.
#[derive(Debug, thiserror::Error)]
pub enum GraphDslError {
    /// 🧾️ Fixture or query-result JSON failed to parse or serialize.
    #[error("invalid json: {0}")]
    Json(#[from] serde_json::Error),
    /// 🔤️ A string literal was never closed (Jack's own dual-quote pre-scan, `dsl_core` only
    /// natively lexes `"..."`).
    #[error("unterminated string literal")]
    UnterminatedString,
    /// ❓️ A byte outside the token grammar was found (Jack's own pre-scan for `'`/`"`/`!=`, ahead
    /// of delegating the rest of the alphabet to `os_dsl::lex`).
    #[error("unexpected character '{0}'")]
    UnexpectedChar(char),
    /// 🔢️ A numeric literal did not parse as a float (defensive — `os_dsl::lex` only ever
    /// accumulates well-formed digit runs, so this should be unreachable in practice).
    #[error("invalid number literal: {0}")]
    NumberFormat(#[from] std::num::ParseFloatError),
    /// ➡️ Parser expected one token shape and found another.
    #[error("expected {expected}, got {found}")]
    UnexpectedToken { expected: String, found: String },
    /// 🪝️ A wire-literal edge was missing a mandatory `@port` on one of its endpoints (this
    /// module's own DAG domain rule, enforced on top of the unified wire grammar).
    #[error("edge target requires @port")]
    EdgeTargetMissingPort,
    /// 🕸️ A jack pattern had no nodes.
    #[error("empty pattern")]
    EmptyPattern,
    /// 🚫️ CREATE/DELETE/SET/MERGE are not supported on read-only queryable graphs.
    #[error("mutating jack clauses are not supported on this graph domain")]
    UnsupportedMutation,
    /// 🚧️ WITH/UNWIND/CALL parse into the AST but aren't wired into the executor yet — prep work
    /// for unifying semio_compose_rs's Architect query language onto Jack (see the repo-wide unified-DSL
    /// plan, Wave 2 / P9).
    #[error("WITH/UNWIND/CALL clauses are not yet executable")]
    UnsupportedClause,
    /// 🔡️ A lexical/grammar error surfaced verbatim by the unified `dsl_core`/`dsl_schema` engine —
    /// used by both the wire-literal delegate (`dsl_core::parse_wire_text`) and Jack's
    /// `dsl_core`-backed lexer.
    #[error("{0}")]
    Lex(#[from] dsl_core::os_dsl::TextError),
}
// #endregion ⚠️ Errors

pub mod queryable {
    // #region queryable
    //! 🔍️ Queryable graph interface for Jack.

    use crate::dsl::GraphDslError;
    use crate::manifest::{manifest_by_id, GraphManifest, PropertyBag, PropertyValue};
    use serde_json::Value;
    use std::collections::{BTreeMap, BTreeSet};

    // #region 🔖️QueryableEdge
    /// 🪢️ Edge row exposed to Jack matching.
    #[derive(Clone, Debug, PartialEq)]
    pub struct QueryableEdge {
        pub id: String,
        pub kind: String,
        pub source_node_id: String,
        pub target_node_id: String,
        pub source_port: Option<String>,
        pub target_port: Option<String>,
        pub properties: PropertyBag,
    }
    // #endregion 🔖️QueryableEdge

    // #region 🔖️QueryableGraph
    /// 🕸️ Read-only graph surface for Jack query execution.
    pub trait QueryableGraph {
        async fn manifest(&self) -> Option<&GraphManifest>;
        async fn node_ids(&self) -> Vec<String>;
        async fn node_kind(&self, id: &str) -> Option<String>;
        async fn node_name(&self, id: &str) -> Option<String>;
        async fn node_property(&self, id: &str, key: &str) -> Option<PropertyValue>;
        async fn edges(&self) -> Vec<QueryableEdge>;
        async fn subgraph_fixture_json(&self, node_ids: &BTreeSet<String>, edge_ids: &BTreeSet<String>) -> Option<String>;
    }

    pub async fn manifest_node_kinds(graph: &dyn QueryableGraph) -> Vec<String> {
        let mut kinds = BTreeSet::new();
        for id in graph.node_ids() {
            if let Some(kind) = graph.node_kind(id.as_str()) {
                kinds.insert(kind);
            }
        }
        if let Some(manifest) = graph.manifest() {
            for def in &manifest.node_kinds {
                kinds.insert(def.id.clone());
            }
        }
        kinds.into_iter().collect()
    }

    pub async fn manifest_edge_kinds(graph: &dyn QueryableGraph) -> Vec<String> {
        let mut kinds = BTreeSet::new();
        for edge in graph.edges() {
            kinds.insert(edge.kind.clone());
        }
        if let Some(manifest) = graph.manifest() {
            for def in &manifest.edge_kinds {
                kinds.insert(def.id.clone());
            }
        }
        kinds.into_iter().collect()
    }

    pub async fn manifest_property_names(graph: &dyn QueryableGraph) -> Vec<String> {
        let mut props = BTreeSet::from(["id".to_string(), "name".to_string(), "kind".to_string()]);
        for id in graph.node_ids() {
            for key in ["label", "text"] {
                if graph.node_property(id.as_str(), key).is_some() {
                    props.insert(key.to_string());
                }
            }
            if let Some(PropertyValue::Object(map)) = graph.node_property(id.as_str(), "__all") {
                for key in map.keys() {
                    props.insert(key.clone());
                }
            }
        }
        props.into_iter().collect()
    }

    pub async fn manifest_port_kinds(graph: &dyn QueryableGraph) -> Vec<String> {
        let mut kinds = BTreeSet::new();
        for edge in graph.edges() {
            if let Some(port) = &edge.source_port {
                kinds.insert(port.clone());
            }
            if let Some(port) = &edge.target_port {
                kinds.insert(port.clone());
            }
        }
        if let Some(manifest) = graph.manifest() {
            for def in &manifest.port_kinds {
                kinds.insert(def.id.clone());
            }
        }
        kinds.into_iter().collect()
    }
    // #endregion 🔖️QueryableGraph

    // #region 🔖️BoardQueryableGraph
    async fn json_to_property_bag(value: &Value) -> PropertyBag {
        serde_json::from_value(value.clone()).unwrap_or_default()
    }

    async fn split_endpoint(endpoint: &str, handle_to_node: &BTreeMap<String, String>) -> (String, Option<String>) {
        if let Some(node_id) = handle_to_node.get(endpoint) {
            return (node_id.clone(), None);
        }
        if let Some((node, port)) = endpoint.split_once('@') {
            let node_id = handle_to_node.get(node).cloned().unwrap_or_else(|| node.to_string());
            return (node_id, Some(port.to_string()));
        }
        if let Some((node, port)) = endpoint.rsplit_once(':') {
            let node_id = handle_to_node.get(node).cloned().unwrap_or_else(|| node.to_string());
            return (node_id, Some(port.to_string()));
        }
        if let Some((node, port)) = endpoint.rsplit_once('.') {
            if handle_to_node.contains_key(endpoint) {
                return (handle_to_node[endpoint].clone(), None);
            }
            return (node.to_string(), Some(port.to_string()));
        }
        (endpoint.to_string(), None)
    }

    /// 🧩️ Jack query target over board/scene fixture JSON.
    pub struct BoardQueryableGraph {
        manifest: Option<GraphManifest>,
        nodes: BTreeMap<String, (String, String, PropertyBag)>,
        edges: Vec<QueryableEdge>,
        raw_fixture: Value,
    }

    impl BoardQueryableGraph {
        pub async fn from_fixture_json(json: &str, manifest_id: Option<&str>) -> Result<Self, GraphDslError> {
            let raw: Value = serde_json::from_str(json)?;
            let manifest = manifest_id.and_then(manifest_by_id).or_else(|| raw.get("manifestId").and_then(|v| v.as_str()).and_then(manifest_by_id)).or_else(|| raw.get("manifest_id").and_then(|v| v.as_str()).and_then(manifest_by_id));
            let mut nodes = BTreeMap::new();
            let mut handle_to_node = BTreeMap::new();
            if let Some(rows) = raw.get("nodes").and_then(|v| v.as_array()) {
                for row in rows {
                    let Some(obj) = row.as_object() else { continue };
                    let Some(id) = obj.get("id").and_then(|v| v.as_str()) else { continue };
                    let kind = obj.get("nodeKind").or_else(|| obj.get("node_kind")).or_else(|| obj.get("kind")).and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let name = obj.get("text").or_else(|| obj.get("name")).or_else(|| obj.get("label")).and_then(|v| v.as_str()).unwrap_or(id).to_string();
                    let mut properties = obj.get("userData").or_else(|| obj.get("user_data")).map(json_to_property_bag).unwrap_or_default();
                    for (key, value) in obj.iter() {
                        if matches!(key.as_str(), "id" | "nodeKind" | "node_kind" | "kind" | "text" | "name" | "label" | "handles" | "x" | "y" | "shape" | "radius" | "width" | "height" | "userData" | "user_data") {
                            continue;
                        }
                        if let Ok(prop) = serde_json::from_value::<PropertyValue>(value.clone()) {
                            properties.insert(key.clone(), prop);
                        }
                    }
                    nodes.insert(id.to_string(), (kind, name, properties));
                    if let Some(handles) = obj.get("handles").and_then(|v| v.as_array()) {
                        for handle in handles {
                            if let Some(hid) = handle.get("id").and_then(|v| v.as_str()) {
                                handle_to_node.insert(hid.to_string(), id.to_string());
                            }
                        }
                    }
                }
            }
            let mut edges = Vec::new();
            if let Some(rows) = raw.get("edges").and_then(|v| v.as_array()) {
                for row in rows {
                    let Some(obj) = row.as_object() else { continue };
                    let Some(id) = obj.get("id").and_then(|v| v.as_str()) else { continue };
                    let Some(source) = obj.get("source").and_then(|v| v.as_str()) else { continue };
                    let Some(target) = obj.get("target").and_then(|v| v.as_str()) else { continue };
                    let kind = obj.get("edgeKind").or_else(|| obj.get("edge_kind")).or_else(|| obj.get("kind")).and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let properties = obj.get("userData").or_else(|| obj.get("user_data")).map(json_to_property_bag).unwrap_or_default();
                    let (source_node_id, source_port) = split_endpoint(source, &handle_to_node);
                    let (target_node_id, target_port) = split_endpoint(target, &handle_to_node);
                    edges.push(QueryableEdge { id: id.to_string(), kind, source_node_id, target_node_id, source_port, target_port, properties });
                }
            }
            Ok(Self { manifest, nodes, edges, raw_fixture: raw })
        }

        pub async fn from_dag_fixture_json(json: &str) -> Result<Self, GraphDslError> {
            Self::from_fixture_json(json, Some("flow-dag"))
        }

        pub async fn from_puzzle2d_fixture_json(json: &str) -> Result<Self, GraphDslError> {
            Self::from_fixture_json(json, Some("puzzle2d-default"))
        }

        pub async fn from_puzzle3d_fixture_json(json: &str) -> Result<Self, GraphDslError> {
            let raw: Value = serde_json::from_str(json)?;
            let mut fixture = raw.clone();
            if fixture.get("nodes").and_then(|v| v.as_array()).is_none() {
                if let Some(objects) = raw.get("objects").and_then(|v| v.as_array()) {
                    let nodes: Vec<Value> = objects
                        .iter()
                        .filter_map(|row| {
                            let obj = row.as_object()?;
                            let id = obj.get("id").and_then(|v| v.as_str())?;
                            let kind = obj.get("objectKind").or_else(|| obj.get("kind")).and_then(|v| v.as_str()).unwrap_or("Object");
                            let name = obj.get("name").or_else(|| obj.get("label")).and_then(|v| v.as_str()).unwrap_or(id);
                            Some(serde_json::json!({
                                "id": id,
                                "nodeKind": kind,
                                "text": name,
                            }))
                        })
                        .collect();
                    fixture["nodes"] = Value::Array(nodes);
                }
            }
            Self::from_fixture_json(&serde_json::to_string(&fixture)?, Some("puzzle3d-default"))
        }

        pub async fn from_puzzle5d_fixture_json(json: &str) -> Result<Self, GraphDslError> {
            Self::from_fixture_json(json, Some("puzzle5d-default"))
        }
    }

    impl QueryableGraph for BoardQueryableGraph {
        async fn manifest(&self) -> Option<&GraphManifest> {
            self.manifest.as_ref()
        }

        async fn node_ids(&self) -> Vec<String> {
            self.nodes.keys().cloned().collect()
        }

        async fn node_kind(&self, id: &str) -> Option<String> {
            self.nodes.get(id).map(|(kind, _, _)| kind.clone())
        }

        async fn node_name(&self, id: &str) -> Option<String> {
            self.nodes.get(id).map(|(_, name, _)| name.clone())
        }

        async fn node_property(&self, id: &str, key: &str) -> Option<PropertyValue> {
            let (_, name, properties) = self.nodes.get(id)?;
            match key {
                "id" => Some(PropertyValue::String(id.to_string())),
                "name" | "label" | "text" => Some(PropertyValue::String(name.clone())),
                "kind" => self.node_kind(id).map(PropertyValue::String),
                "__all" => Some(PropertyValue::Object(properties.clone())),
                _ => properties.get(key).cloned(),
            }
        }

        async fn edges(&self) -> Vec<QueryableEdge> {
            self.edges.clone()
        }

        async fn subgraph_fixture_json(&self, node_ids: &BTreeSet<String>, edge_ids: &BTreeSet<String>) -> Option<String> {
            let mut fixture = self.raw_fixture.clone();
            if let Some(nodes) = fixture.get_mut("nodes").and_then(|v| v.as_array_mut()) {
                nodes.retain(|row| row.get("id").and_then(|v| v.as_str()).is_some_and(|id| node_ids.contains(id)));
            }
            if let Some(edges) = fixture.get_mut("edges").and_then(|v| v.as_array_mut()) {
                edges.retain(|row| row.get("id").and_then(|v| v.as_str()).is_some_and(|id| edge_ids.contains(id)));
            }
            serde_json::to_string(&fixture).ok()
        }
    }
    // #endregion 🔖️BoardQueryableGraph
    // #endregion queryable
}

pub mod wire {
    // #region wire
    //! 🔌️ Wire-literal compiled DAG text notation — delegates all lexing/parsing/printing to
    //! `dsl_schema`'s unified `Shape::Wire` grammar (`->`/`<-`/`--`, `{k=v}` double-quoted
    //! properties), keeping only this module's own public row types (`WireNode`/`WireEdge`) and
    //! its domain-specific rule that an edge's ports are mandatory on both ends — a validation
    //! layer on top of the shared parse, not a syntax difference. ~8 downstream crates depend on
    //! these exact type/function signatures, unchanged by this unification.

    use crate::dsl::GraphDslError;
    use crate::manifest::{PropertyBag, PropertyValue};

    // #region 🔖️WireTypes
    /// 🧩️ Neutral node row for wire-literal emission.
    #[derive(Clone, Debug, PartialEq)]
    pub struct WireNode {
        pub id: String,
        pub kind: String,
        pub port: Option<String>,
        pub properties: PropertyBag,
    }

    /// 🪢️ Neutral edge row for wire-literal emission.
    #[derive(Clone, Debug, PartialEq)]
    pub struct WireEdge {
        pub from: String,
        pub from_port: String,
        pub to: String,
        pub to_port: String,
        pub directed: bool,
        pub properties: PropertyBag,
    }
    // #endregion 🔖️WireTypes

    // #region 🔖️PropertyBridge
    /// 🌉️ `crate::manifest::PropertyValue` <-> `dsl_core::DslValue` — the two crates'
    /// dynamic-JSON-equivalent literal types are structurally identical, so this is a pure reshape.
    async fn dsl_value_from_property_value(value: &PropertyValue) -> dsl_core::DslValue {
        match value {
            PropertyValue::Null => dsl_core::DslValue::Null,
            PropertyValue::Bool(b) => dsl_core::DslValue::Bool(*b),
            PropertyValue::Number(n) => dsl_core::DslValue::Number(*n),
            PropertyValue::String(s) => dsl_core::DslValue::String(s.clone()),
            PropertyValue::Array(items) => dsl_core::DslValue::Array(items.iter().map(dsl_value_from_property_value).collect()),
            PropertyValue::Object(map) => dsl_core::DslValue::Object(map.iter().map(|(k, v)| (k.clone(), dsl_value_from_property_value(v))).collect()),
        }
    }

    async fn property_value_from_dsl_value(value: &dsl_core::DslValue) -> PropertyValue {
        match value {
            dsl_core::DslValue::Null => PropertyValue::Null,
            dsl_core::DslValue::Bool(b) => PropertyValue::Bool(*b),
            dsl_core::DslValue::Number(n) => PropertyValue::Number(*n),
            dsl_core::DslValue::String(s) => PropertyValue::String(s.clone()),
            dsl_core::DslValue::Array(items) => PropertyValue::Array(items.iter().map(property_value_from_dsl_value).collect()),
            dsl_core::DslValue::Object(entries) => PropertyValue::Object(entries.iter().map(|(k, v)| (k.clone(), property_value_from_dsl_value(v))).collect()),
        }
    }

    async fn properties_to_dsl_object(properties: &PropertyBag) -> dsl_core::DslValue {
        dsl_core::DslValue::Object(properties.iter().map(|(k, v)| (k.clone(), dsl_value_from_property_value(v))).collect())
    }

    async fn properties_from_dsl_value(value: &dsl_core::DslValue) -> PropertyBag {
        match value {
            dsl_core::DslValue::Object(entries) => entries.iter().map(|(k, v)| (k.clone(), property_value_from_dsl_value(v))).collect(),
            _ => PropertyBag::new(),
        }
    }
    // #endregion 🔖️PropertyBridge

    // #region 🔖️WireLiteral
    async fn render_wire_line(value: &dsl_core::WireValue) -> String {
        let mut writer = dsl_core::Writer::new();
        dsl_core::print_shape(&dsl_core::FieldValue::Wire(value.clone()), &dsl_core::Shape::Wire, &mut writer);
        writer.render(dsl_core::JoinMode::Inline)
    }

    /// 📝️ Render wire-literal text from neutral node/edge rows, one unified `dsl_core::Wire`
    /// statement per line.
    pub async fn wire_literal_from_dag(nodes: &[WireNode], edges: &[WireEdge]) -> String {
        let mut lines = Vec::new();
        for node in nodes {
            let value = dsl_core::WireValue { from: dsl_core::WireNode { id: node.id.clone(), kind: Some(node.kind.clone()), port: node.port.clone() }, edge: None, edge_label: dsl_core::WireEdgeLabel::default(), properties: properties_to_dsl_object(&node.properties) };
            lines.push(render_wire_line(&value));
        }
        for edge in edges {
            let from_kind = nodes.iter().find(|n| n.id == edge.from).map_or("node", |n| n.kind.as_str());
            let to_kind = nodes.iter().find(|n| n.id == edge.to).map_or("node", |n| n.kind.as_str());
            let value = dsl_core::WireValue {
                from: dsl_core::WireNode { id: edge.from.clone(), kind: Some(from_kind.to_string()), port: Some(edge.from_port.clone()) },
                edge: Some((edge.directed, dsl_core::WireNode { id: edge.to.clone(), kind: Some(to_kind.to_string()), port: Some(edge.to_port.clone()) })),
                edge_label: dsl_core::WireEdgeLabel::default(),
                properties: properties_to_dsl_object(&edge.properties),
            };
            lines.push(render_wire_line(&value));
        }
        lines.join("\n")
    }

    /// 🔍️ Parse wire-literal text into neutral node/edge rows. Delegates lexing+parsing to
    /// `dsl_core::parse_wire_text` (the one unified wire grammar — `->`/`<-` sugar/`--`,
    /// `{k=v}` double-quoted properties) one statement (line) at a time, then enforces this
    /// module's own DAG domain rule on top: an edge's ports are mandatory on BOTH ends (the
    /// shared grammar itself leaves ports optional on every endpoint — that's the engine's
    /// business, not a syntax difference this module should encode into the lexer/parser).
    pub async fn dag_from_wire_literal(text: &str) -> Result<(Vec<WireNode>, Vec<WireEdge>), GraphDslError> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let value = dsl_core::parse_wire_text(line)?;
            match value.edge {
                None => nodes.push(WireNode { id: value.from.id, kind: value.from.kind.unwrap_or_else(|| "node".to_string()), port: value.from.port, properties: properties_from_dsl_value(&value.properties) }),
                Some((directed, to)) => {
                    let from_port = value.from.port.ok_or(GraphDslError::EdgeTargetMissingPort)?;
                    let to_port = to.port.ok_or(GraphDslError::EdgeTargetMissingPort)?;
                    edges.push(WireEdge { from: value.from.id, from_port, to: to.id, to_port, directed, properties: properties_from_dsl_value(&value.properties) });
                }
            }
        }
        Ok((nodes, edges))
    }
    // #endregion 🔖️WireLiteral

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        async fn wire_literal_roundtrip_simple() {
            // 🩹️ unified grammar: port names are `dsl_core` idents (must start with a letter or
            // `_`, never a digit) — the old port name `"3d"` is no longer lexable, renamed `"d3"`.
            let nodes = vec![WireNode { id: "p".into(), kind: "Puzzle3d".into(), port: None, properties: PropertyBag::new() }];
            let edges = vec![WireEdge { from: "p".into(), from_port: "d3".into(), to: "s".into(), to_port: "d3".into(), directed: true, properties: PropertyBag::new() }];
            let text = wire_literal_from_dag(&nodes, &edges);
            assert!(text.contains("p:Puzzle3d"));
            assert!(text.contains("p:Puzzle3d@d3->s:node@d3"));
            let parsed = dag_from_wire_literal(&text).unwrap();
            assert_eq!(parsed.1.len(), 1);
        }

        #[test]
        async fn wire_literal_undirected() {
            let edges = vec![WireEdge { from: "a".into(), from_port: "out".into(), to: "b".into(), to_port: "in".into(), directed: false, properties: PropertyBag::new() }];
            let text = wire_literal_from_dag(&[], &edges);
            assert!(text.contains('@'));
            assert!(text.contains('-'));
        }

        #[test]
        async fn wire_literal_with_properties() {
            let mut props = PropertyBag::new();
            props.insert("value".into(), PropertyValue::Number(3.0));
            let nodes = vec![WireNode { id: "n".into(), kind: "slider".into(), port: None, properties: props }];
            let text = wire_literal_from_dag(&nodes, &[]);
            // 🩹️ unified syntax: `key=value` (never `key: value`), space-padded braces when glued
            // onto a preceding atom, per `dsl_core::Writer`'s canonical spacing law.
            assert!(text.contains("{ value=3 }"), "expected unified {{ value=3 }} properties, got: {text}");
        }

        #[test]
        async fn wire_literal_nested_object_and_array_properties() {
            let mut inner = PropertyBag::new();
            inner.insert("y".into(), PropertyValue::Bool(true));
            let mut props = PropertyBag::new();
            props.insert("obj".into(), PropertyValue::Object(inner));
            props.insert("arr".into(), PropertyValue::Array(vec![PropertyValue::Number(1.0), PropertyValue::Null]));
            let nodes = vec![WireNode { id: "n".into(), kind: "slider".into(), port: None, properties: props }];
            let text = wire_literal_from_dag(&nodes, &[]);
            assert!(text.contains("obj={ y=true }"), "expected unified obj={{ y=true }}, got: {text}");
            assert!(text.contains("arr=[ 1 null ]"), "expected unified arr=[ 1 null ], got: {text}");
        }

        #[test]
        async fn wire_literal_from_dag_unknown_node_kind_defaults_to_node() {
            let edges = vec![WireEdge { from: "missing".into(), from_port: "out".into(), to: "also-missing".into(), to_port: "in".into(), directed: true, properties: PropertyBag::new() }];
            let text = wire_literal_from_dag(&[], &edges);
            assert_eq!(text, "missing:node@out->also-missing:node@in");
        }

        #[test]
        async fn dag_from_wire_literal_rejects_unterminated_string() {
            // 🩹️ unified syntax: double-quoted properties, `key="value"` (never `key: 'value'`).
            let err = dag_from_wire_literal("n:kind{prop=\"unterminated").unwrap_err();
            assert!(matches!(err, GraphDslError::Lex(_)));
            assert!(err.to_string().contains("unterminated string literal"), "got: {err}");
        }

        #[test]
        async fn dag_from_wire_literal_rejects_unexpected_char() {
            // 🩹️ `#` is now a legitimate comment starter (unified with the rest of the DSL engine),
            // so the "genuinely unrecognized character" trigger moved to `?`, which is outside
            // `dsl_core`'s alphabet in every mode.
            let err = dag_from_wire_literal("n:kind?bad").unwrap_err();
            assert!(matches!(err, GraphDslError::Lex(_)));
            assert!(err.to_string().contains("unexpected character '?'"), "got: {err}");
        }

        #[test]
        async fn dag_from_wire_literal_rejects_edge_missing_target_port() {
            let err = dag_from_wire_literal("a:kind@out->b:kind").unwrap_err();
            assert!(matches!(err, GraphDslError::EdgeTargetMissingPort));
        }

        #[test]
        async fn dag_from_wire_literal_rejects_edge_missing_source_port() {
            // 🆕️ the unified grammar itself leaves the source port optional (unlike the old
            // hand-rolled parser, which could never even reach an edge without one) — this
            // module's own DAG domain rule must now catch it explicitly.
            let err = dag_from_wire_literal("a:kind->b:kind@in").unwrap_err();
            assert!(matches!(err, GraphDslError::EdgeTargetMissingPort));
        }

        #[test]
        async fn dag_from_wire_literal_parses_bool_and_null_properties() {
            // 🩹️ unified syntax: space-separated `key=value` pairs, no commas, no colons.
            let (nodes, _) = dag_from_wire_literal("n:kind{on=true off=false empty=null}").unwrap();
            let props = &nodes[0].properties;
            assert_eq!(props.get("on"), Some(&PropertyValue::Bool(true)));
            assert_eq!(props.get("off"), Some(&PropertyValue::Bool(false)));
            assert_eq!(props.get("empty"), Some(&PropertyValue::Null));
        }

        #[test]
        async fn dag_from_wire_literal_parses_double_quoted_string_properties() {
            let (nodes, _) = dag_from_wire_literal("n:kind{label=\"hello world\"}").unwrap();
            assert_eq!(nodes[0].properties.get("label"), Some(&PropertyValue::String("hello world".to_string())));
        }

        #[test]
        async fn dag_from_wire_literal_rejects_malformed_properties() {
            let err = dag_from_wire_literal("n:kind{prop 1}").unwrap_err();
            assert!(matches!(err, GraphDslError::Lex(_)));
        }

        #[test]
        async fn dag_from_wire_literal_accepts_back_arrow_sugar_and_normalizes_direction() {
            // 🆕️ `<-` is accepted sugar, normalized to the same stored/parsed shape as `->` with
            // endpoints swapped — `dsl_core::parse_wire`'s law, inherited for free.
            let (_, edges) = dag_from_wire_literal("b:kind@in<-a:kind@out").unwrap();
            assert_eq!(edges.len(), 1);
            let edge = &edges[0];
            assert_eq!(edge.from, "a");
            assert_eq!(edge.from_port, "out");
            assert_eq!(edge.to, "b");
            assert_eq!(edge.to_port, "in");
            assert!(edge.directed);
        }

        #[test]
        async fn dag_from_wire_literal_parses_undirected_dash_dash_edge() {
            // 🆕️ unified undirected sigil is `--`, not the old single `-`.
            let (_, edges) = dag_from_wire_literal("a:x@out--b:y@in").unwrap();
            assert_eq!(edges.len(), 1);
            assert!(!edges[0].directed);
            assert_eq!(edges[0].from, "a");
            assert_eq!(edges[0].to, "b");
        }

        #[test]
        async fn wire_literal_from_dag_round_trips_through_unified_double_quoted_syntax() {
            let mut props = PropertyBag::new();
            props.insert("label".into(), PropertyValue::String("hi".into()));
            let nodes = vec![WireNode { id: "n".into(), kind: "slider".into(), port: None, properties: props }];
            let text = wire_literal_from_dag(&nodes, &[]);
            assert!(text.contains("\"hi\""), "properties must print double-quoted: {text}");
            let (parsed_nodes, _) = dag_from_wire_literal(&text).unwrap();
            assert_eq!(parsed_nodes[0].properties.get("label"), Some(&PropertyValue::String("hi".into())));
        }
    }
    // #endregion 🔖️Tests
    // #endregion wire
}

pub use queryable::{manifest_edge_kinds, manifest_node_kinds, manifest_port_kinds, manifest_property_names, BoardQueryableGraph, QueryableEdge, QueryableGraph};
pub use wire::{dag_from_wire_literal, wire_literal_from_dag, WireEdge, WireNode};

use crate::manifest::PropertyValue;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

// #region jack_impl

// #region 🔖️Ast
/// 🌳️ Jack query abstract syntax tree.
#[derive(Clone, Debug, PartialEq)]
pub struct Query {
    pub clauses: Vec<Clause>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Clause {
    Match(Vec<Pattern>),
    Where(Expr),
    /// 🚧️ Parses; not yet executed (see [`GraphDslError::UnsupportedClause`]) — prep for unifying
    /// semio_compose_rs's Architect query language onto Jack.
    With(Vec<ReturnItem>),
    /// 🚧️ Parses; not yet executed (see [`GraphDslError::UnsupportedClause`]) — prep for unifying
    /// semio_compose_rs's Architect query language onto Jack.
    Unwind(UnwindClause),
    /// 🚧️ Parses; not yet executed (see [`GraphDslError::UnsupportedClause`]) — prep for unifying
    /// semio_compose_rs's Architect query language onto Jack.
    Call(CallClause),
    Return(Vec<ReturnItem>),
    Create(Pattern),
    Delete(Vec<String>),
    Set(Vec<Assignment>),
    Merge(Pattern),
}

/// 🌀️ `UNWIND <source> AS <var>` — flattens a list-valued source into per-row bindings of `var`.
#[derive(Clone, Debug, PartialEq)]
pub struct UnwindClause {
    pub source: ReturnItem,
    pub var: String,
}

/// 📞️ `CALL <name>(<args>...)` — a named procedure invocation with positional scalar arguments.
#[derive(Clone, Debug, PartialEq)]
pub struct CallClause {
    pub name: String,
    pub args: Vec<PropertyValue>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    pub nodes: Vec<PatternNode>,
    pub edge: Option<PatternEdge>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternNode {
    pub var: String,
    pub kind: String,
    pub port: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternEdge {
    pub var: Option<String>,
    pub kind: Option<String>,
    pub directed: bool,
    pub right: PatternNode,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReturnItem {
    Var(String),
    Property { var: String, prop: String },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assignment {
    pub var: String,
    pub prop: String,
    pub value: PropertyValue,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Eq { var: String, prop: String, value: PropertyValue },
    Ne { var: String, prop: String, value: PropertyValue },
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum QueryResultKind {
    #[default]
    Table,
    Graph,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResult {
    #[serde(default)]
    pub kind: QueryResultKind,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<PropertyValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_fixture_json: Option<String>,
}

impl QueryResult {
    pub async fn table(columns: Vec<String>, rows: Vec<Vec<PropertyValue>>) -> Self {
        Self { kind: QueryResultKind::Table, columns, rows, graph_fixture_json: None }
    }

    pub async fn graph(columns: Vec<String>, graph_fixture_json: String) -> Self {
        Self { kind: QueryResultKind::Graph, columns, rows: vec![], graph_fixture_json: Some(graph_fixture_json) }
    }
}
// #endregion 🔖️Ast

// #region 🔖️Lexer
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TokenClass {
    Keyword,
    Ident,
    Number,
    String,
    Operator,
    Punctuation,
    Error,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenSpan {
    pub class: TokenClass,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    KwMatch,
    KwWhere,
    KwReturn,
    KwCreate,
    KwDelete,
    KwSet,
    KwMerge,
    KwWith,
    KwUnwind,
    KwCall,
    KwAs,
    Ident(String),
    Number(f64),
    StringLit(String),
    LParen,
    RParen,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Dot,
    Eq,
    Ne,
    Arrow,
    DashArrow,
    BackArrow,
    At,
    And,
    Or,
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
struct SpannedToken {
    token: Token,
    start: usize,
    end: usize,
}

async fn token_class(token: &Token) -> TokenClass {
    match token {
        Token::KwMatch | Token::KwWhere | Token::KwReturn | Token::KwCreate | Token::KwDelete | Token::KwSet | Token::KwMerge | Token::KwWith | Token::KwUnwind | Token::KwCall | Token::KwAs | Token::And | Token::Or => TokenClass::Keyword,
        Token::Ident(_) => TokenClass::Ident,
        Token::Number(_) => TokenClass::Number,
        Token::StringLit(_) => TokenClass::String,
        Token::Eq | Token::Ne | Token::Arrow | Token::DashArrow | Token::BackArrow | Token::At => TokenClass::Operator,
        Token::LParen | Token::RParen | Token::LBracket | Token::RBracket | Token::Colon | Token::Comma | Token::Dot => TokenClass::Punctuation,
        Token::Eof => TokenClass::Punctuation,
    }
}

async fn push_spanned(tokens: &mut Vec<SpannedToken>, token: Token, start: usize, end: usize) {
    tokens.push(SpannedToken { token, start, end });
}

/// 🔑️ Uppercases and matches against Jack's clause/logic keyword table; anything else stays a
/// plain variable/property/kind identifier. Case-insensitive (Cypher heritage — `match`, `Match`,
/// `MATCH` are all the same token), unlike `dsl_core`'s own grammars which are case-sensitive.
async fn keyword_or_ident(text: String) -> Token {
    match text.to_ascii_uppercase().as_str() {
        "MATCH" => Token::KwMatch,
        "WHERE" => Token::KwWhere,
        "RETURN" => Token::KwReturn,
        "CREATE" => Token::KwCreate,
        "DELETE" => Token::KwDelete,
        "SET" => Token::KwSet,
        "MERGE" => Token::KwMerge,
        "WITH" => Token::KwWith,
        "UNWIND" => Token::KwUnwind,
        "CALL" => Token::KwCall,
        "AS" => Token::KwAs,
        "AND" => Token::And,
        "OR" => Token::Or,
        _ => Token::Ident(text),
    }
}

/// 🪚️ `dsl_core` treats `.` as ident-continue (so `a.name` lexes as ONE ident there), but Jack's
/// `var.prop` property-access grammar needs `.` as its own token — splits it back apart here,
/// checking each piece against the keyword table too (defensive; keywords never legitimately
/// contain a dot, but this keeps the one keyword-recognition path authoritative).
async fn push_ident_or_keyword_with_dots(text: &str, start: usize, out: &mut Vec<SpannedToken>) {
    let mut offset = 0usize;
    for (idx, part) in text.split('.').enumerate() {
        if idx > 0 {
            push_spanned(out, Token::Dot, start + offset, start + offset + 1);
            offset += 1;
        }
        if !part.is_empty() {
            let end = start + offset + part.len();
            push_spanned(out, keyword_or_ident(part.to_string()), start + offset, end);
        }
        offset += part.len();
    }
}

/// 🔬️ Converts one already-lexed `dsl_core` segment (containing no quotes or `!=` — those are
/// scanned by [`lex_spanned`] itself, ahead of delegating everything else) into Jack's own
/// richer, grammar-aware token stream.
async fn push_dsl_core_segment(segment: &str, base_offset: usize, forgiving: bool, out: &mut Vec<SpannedToken>) -> Result<(), GraphDslError> {
    if segment.is_empty() {
        return Ok(());
    }
    let raw = dsl_core::os_dsl::lex(segment, &dsl_core::os_dsl::Limits::default(), forgiving).map_err(GraphDslError::Lex)?;
    for token in raw {
        if token.kind.is_trivia() || token.kind == dsl_core::os_dsl::TokenKind::Eof {
            continue;
        }
        let start = base_offset + token.byte_range.0 as usize;
        let end = base_offset + token.byte_range.1 as usize;
        let text = token.text.as_str().to_string();
        match token.kind {
            dsl_core::os_dsl::TokenKind::Ident => push_ident_or_keyword_with_dots(&text, start, out),
            // A lone `_` is `dsl_core`'s placeholder sigil; Jack has no placeholder concept of its
            // own, so it round-trips as an ordinary one-character identifier.
            dsl_core::os_dsl::TokenKind::Placeholder => push_spanned(out, Token::Ident(text), start, end),
            dsl_core::os_dsl::TokenKind::Int | dsl_core::os_dsl::TokenKind::Float => {
                let n: f64 = text.parse().map_err(GraphDslError::NumberFormat)?;
                push_spanned(out, Token::Number(n), start, end);
            }
            dsl_core::os_dsl::TokenKind::LParen => push_spanned(out, Token::LParen, start, end),
            dsl_core::os_dsl::TokenKind::RParen => push_spanned(out, Token::RParen, start, end),
            dsl_core::os_dsl::TokenKind::LBracket => push_spanned(out, Token::LBracket, start, end),
            dsl_core::os_dsl::TokenKind::RBracket => push_spanned(out, Token::RBracket, start, end),
            dsl_core::os_dsl::TokenKind::Colon => push_spanned(out, Token::Colon, start, end),
            dsl_core::os_dsl::TokenKind::Comma => push_spanned(out, Token::Comma, start, end),
            dsl_core::os_dsl::TokenKind::Equals => push_spanned(out, Token::Eq, start, end),
            dsl_core::os_dsl::TokenKind::At => push_spanned(out, Token::At, start, end),
            dsl_core::os_dsl::TokenKind::Arrow => push_spanned(out, Token::Arrow, start, end),
            dsl_core::os_dsl::TokenKind::DashArrow => push_spanned(out, Token::DashArrow, start, end),
            dsl_core::os_dsl::TokenKind::BackArrow => push_spanned(out, Token::BackArrow, start, end),
            // Double-quoted text delegated straight through `dsl_core` — unreachable in practice
            // since `lex_spanned` pre-scans and consumes every quote itself before ever
            // delegating a segment, kept only for defensive completeness.
            dsl_core::os_dsl::TokenKind::Text => push_spanned(out, Token::StringLit(text), start, end),
            // `{`/`}` aren't part of Jack's grammar (no map/object literals) — same "stray
            // character" treatment as an outright `os_dsl::TokenKind::Error` below. P2-M1's
            // promoted `< > & $ ;` tokens and STEP's `DotEnum` literal join this bucket too —
            // Jack has no grammar concept for any of them either.
            dsl_core::os_dsl::TokenKind::EdgeArrow
            | dsl_core::os_dsl::TokenKind::LBrace
            | dsl_core::os_dsl::TokenKind::RBrace
            | dsl_core::os_dsl::TokenKind::Caret
            | dsl_core::os_dsl::TokenKind::DotDot
            | dsl_core::os_dsl::TokenKind::Plus
            | dsl_core::os_dsl::TokenKind::Minus
            | dsl_core::os_dsl::TokenKind::Star
            | dsl_core::os_dsl::TokenKind::Slash
            | dsl_core::os_dsl::TokenKind::Fence
            | dsl_core::os_dsl::TokenKind::Lt
            | dsl_core::os_dsl::TokenKind::Gt
            | dsl_core::os_dsl::TokenKind::Amp
            | dsl_core::os_dsl::TokenKind::Dollar
            | dsl_core::os_dsl::TokenKind::Semicolon
            | dsl_core::os_dsl::TokenKind::DotEnum
            | dsl_core::os_dsl::TokenKind::Error => {
                if forgiving {
                    push_spanned(out, Token::Ident(text), start, end);
                } else {
                    return Err(GraphDslError::UnexpectedChar(text.chars().next().unwrap_or('?')));
                }
            }
            dsl_core::os_dsl::TokenKind::Whitespace | dsl_core::os_dsl::TokenKind::Newline | dsl_core::os_dsl::TokenKind::Comment | dsl_core::os_dsl::TokenKind::Eof => {
                unreachable!("trivia/Eof filtered above")
            }
        }
    }
    Ok(())
}

/// 🔬️ Jack's own lexer: unifies on `os_dsl::lex` for the shared token alphabet (idents,
/// numbers, punctuation, `(`/`)`/`[`/`]`, `->`/`--`/`<-`) but keeps two genuinely Cypher-specific
/// pieces local, since neither fits `dsl_core`'s grammar-independent alphabet: dual-quote strings
/// (`'x'`/`"x"` — Cypher heritage; `dsl_core` only ever lexes `"..."`) and the `!=` comparison
/// operator (`dsl_core` has no relational operators at all — it's a structural DSL alphabet, not
/// an expression language). Both are pre-scanned as their own tokens; every remaining run of
/// characters is delegated whole to `os_dsl::lex` and converted via [`push_dsl_core_segment`].
async fn lex_spanned(input: &str, forgiving: bool) -> Result<Vec<SpannedToken>, GraphDslError> {
    let bytes = input.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0usize;
    let mut seg_start = 0usize;

    while i < bytes.len() {
        let c = bytes[i];
        if c == b'\'' || c == b'"' {
            push_dsl_core_segment(&input[seg_start..i], seg_start, forgiving, &mut tokens)?;
            let quote = c;
            let start = i;
            i += 1;
            let content_start = i;
            let mut closed = false;
            while i < bytes.len() {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    i += 2;
                    continue;
                }
                if bytes[i] == quote {
                    closed = true;
                    break;
                }
                i += 1;
            }
            let raw = String::from_utf8_lossy(&bytes[content_start..i]).into_owned();
            if !closed {
                if forgiving {
                    push_spanned(&mut tokens, Token::StringLit(raw), start, i);
                    seg_start = i;
                    break;
                }
                return Err(GraphDslError::UnterminatedString);
            }
            i += 1;
            let text = dsl_core::os_dsl::unescape_text(&raw, forgiving).unwrap_or(raw);
            push_spanned(&mut tokens, Token::StringLit(text), start, i);
            seg_start = i;
            continue;
        }
        if c == b'!' && i + 1 < bytes.len() && bytes[i + 1] == b'=' {
            push_dsl_core_segment(&input[seg_start..i], seg_start, forgiving, &mut tokens)?;
            push_spanned(&mut tokens, Token::Ne, i, i + 2);
            i += 2;
            seg_start = i;
            continue;
        }
        i += 1;
    }
    push_dsl_core_segment(&input[seg_start..bytes.len()], seg_start, forgiving, &mut tokens)?;
    push_spanned(&mut tokens, Token::Eof, input.len(), input.len());
    Ok(tokens)
}

async fn lex(input: &str) -> Result<Vec<Token>, GraphDslError> {
    lex_spanned(input, false).map(|spanned| spanned.into_iter().map(|row| row.token).collect())
}

/// 🎨️ Tokenize jack source for editor highlighting (never fails).
pub async fn tokenize(input: &str) -> Vec<TokenSpan> {
    lex_spanned(input, true)
        .unwrap_or_default()
        .into_iter()
        .filter(|row| !matches!(row.token, Token::Eof))
        .map(|row| {
            let mut class = token_class(&row.token);
            if matches!(row.token, Token::StringLit(_)) {
                let quote = input.as_bytes().get(row.start);
                if quote == Some(&b'\'') || quote == Some(&b'"') {
                    let closed = input.as_bytes().get(row.end.saturating_sub(1)) == quote;
                    if !closed {
                        class = TokenClass::Error;
                    }
                }
            }
            TokenSpan { class, start: row.start, end: row.end }
        })
        .collect()
}
// #endregion 🔖️Lexer

// #region 🔖️Language
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Completion {
    pub label: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    pub insert: String,
}

const CLAUSE_KEYWORDS: &[&str] = &["MATCH", "WHERE", "RETURN", "CREATE", "DELETE", "SET", "MERGE", "WITH", "UNWIND", "CALL"];
const LOGIC_KEYWORDS: &[&str] = &["AND", "OR"];

async fn completion_prefix(source: &str, cursor: usize) -> String {
    let cursor = cursor.min(source.len());
    let bytes = source.as_bytes();
    let mut start = cursor;
    while start > 0 {
        let c = bytes[start - 1];
        if c.is_ascii_alphanumeric() || c == b'_' {
            start -= 1;
        } else {
            break;
        }
    }
    source[start..cursor].to_string()
}

async fn tokens_before_cursor(tokens: &[SpannedToken], cursor: usize) -> &[SpannedToken] {
    let mut end = tokens.len();
    for (i, row) in tokens.iter().enumerate() {
        if row.start >= cursor && !matches!(row.token, Token::Eof) {
            end = i;
            break;
        }
    }
    &tokens[..end]
}

async fn after_colon_kind_context(source: &str, cursor: usize) -> Option<bool> {
    let cursor = cursor.min(source.len());
    let before = &source[..cursor];
    let colon = before.rfind(':')?;
    let after = &before[colon + 1..];
    // 🩹️ an `@` after the kind name means the cursor moved into the port segment (`kind@port`);
    // bail so `after_at_port_context` can offer port completions instead of kind completions.
    if after.chars().any(|c| c.is_whitespace() || matches!(c, '(' | ')' | '[' | ']' | ',' | '@')) {
        return None;
    }
    let left = &before[..colon];
    let bracket = left.rfind('[');
    let paren = left.rfind('(');
    let in_bracket = match (bracket, paren) {
        (Some(b), Some(p)) => b > p,
        (Some(_), None) => true,
        _ => false,
    };
    Some(in_bracket)
}

async fn after_dot_property_context(source: &str, cursor: usize) -> bool {
    let cursor = cursor.min(source.len());
    let before = &source[..cursor];
    let Some(dot) = before.rfind('.') else {
        return false;
    };
    let after = &before[dot + 1..];
    !after.chars().any(|c| c.is_whitespace() || matches!(c, '(' | ')' | '[' | ']' | ',' | ':'))
}
async fn open_bracket_kind(tokens: &[SpannedToken]) -> Option<char> {
    let mut paren = 0i32;
    let mut bracket = 0i32;
    for row in tokens.iter().rev() {
        match row.token {
            Token::RParen => paren += 1,
            Token::LParen if paren > 0 => paren -= 1,
            Token::LParen if paren == 0 && bracket == 0 => return Some('('),
            Token::RBracket => bracket += 1,
            Token::LBracket if bracket > 0 => bracket -= 1,
            Token::LBracket if bracket == 0 && paren == 0 => return Some('['),
            _ => {}
        }
    }
    None
}

async fn collect_bound_vars(tokens: &[SpannedToken]) -> BTreeSet<String> {
    let mut vars = BTreeSet::new();
    let mut i = 0;
    while i + 2 < tokens.len() {
        if matches!(tokens[i].token, Token::LParen | Token::LBracket) {
            if let Token::Ident(var) = &tokens[i + 1].token {
                if matches!(tokens[i + 2].token, Token::Colon) {
                    vars.insert(var.clone());
                }
            }
        }
        i += 1;
    }
    vars
}

async fn in_where_clause(tokens: &[SpannedToken]) -> bool {
    let mut seen_where = false;
    let mut seen_return = false;
    for row in tokens {
        match row.token {
            Token::KwWhere => seen_where = true,
            Token::KwReturn => seen_return = true,
            _ => {}
        }
    }
    seen_where && !seen_return
}

async fn graph_node_kinds(graph: &dyn QueryableGraph) -> Vec<String> {
    manifest_node_kinds(graph)
}

async fn graph_edge_kinds(graph: &dyn QueryableGraph) -> Vec<String> {
    manifest_edge_kinds(graph)
}

async fn graph_property_names(graph: &dyn QueryableGraph) -> Vec<String> {
    manifest_property_names(graph)
}

async fn graph_port_kinds(graph: &dyn QueryableGraph) -> Vec<String> {
    manifest_port_kinds(graph)
}

async fn after_at_port_context(source: &str, cursor: usize) -> bool {
    let cursor = cursor.min(source.len());
    let before = &source[..cursor];
    let Some(at) = before.rfind('@') else {
        return false;
    };
    let after = &before[at + 1..];
    !after.chars().any(|c| c.is_whitespace() || matches!(c, '(' | ')' | '[' | ']' | ',' | '-' | '>' | '@'))
}

async fn filter_completions(candidates: impl IntoIterator<Item = (String, String, Option<String>)>, prefix: &str) -> Vec<Completion> {
    let prefix_lower = prefix.to_ascii_lowercase();
    let mut out = Vec::new();
    for (label, kind, detail) in candidates {
        if prefix.is_empty() || label.to_ascii_lowercase().starts_with(&prefix_lower) {
            out.push(Completion { insert: label.clone(), label, kind, detail });
        }
    }
    out.sort_by(|a, b| a.label.cmp(&b.label));
    out
}

/// 🔎️ Context-aware jack completions for the editor.
pub async fn complete(graph: &dyn QueryableGraph, source: &str, cursor: usize) -> Vec<Completion> {
    let cursor = cursor.min(source.len());
    let prefix = completion_prefix(source, cursor);
    let tokens = lex_spanned(source, true).unwrap_or_default();
    let before = tokens_before_cursor(&tokens, cursor);

    if let Some(in_bracket) = after_colon_kind_context(source, cursor) {
        let kinds = if in_bracket { graph_edge_kinds(graph).into_iter().map(|name| (name, "edgeKind".into(), None)).collect::<Vec<_>>() } else { graph_node_kinds(graph).into_iter().map(|name| (name, "nodeKind".into(), None)).collect::<Vec<_>>() };
        return filter_completions(kinds, &prefix);
    }

    if after_dot_property_context(source, cursor) {
        let props = graph_property_names(graph).into_iter().map(|name| (name, "property".into(), None)).collect::<Vec<_>>();
        return filter_completions(props, &prefix);
    }

    if after_at_port_context(source, cursor) {
        let ports = graph_port_kinds(graph).into_iter().map(|name| (name, "portKind".into(), None)).collect::<Vec<_>>();
        return filter_completions(ports, &prefix);
    }

    if let Some(last) = before.last() {
        if matches!(last.token, Token::At) {
            let ports = graph_port_kinds(graph).into_iter().map(|name| (name, "portKind".into(), None)).collect::<Vec<_>>();
            return filter_completions(ports, &prefix);
        }
        if matches!(last.token, Token::Colon) {
            let kinds = if open_bracket_kind(before) == Some('[') {
                graph_edge_kinds(graph).into_iter().map(|name| (name, "edgeKind".into(), None)).collect::<Vec<_>>()
            } else {
                graph_node_kinds(graph).into_iter().map(|name| (name, "nodeKind".into(), None)).collect::<Vec<_>>()
            };
            return filter_completions(kinds, &prefix);
        }
        if matches!(last.token, Token::Dot) {
            let props = graph_property_names(graph).into_iter().map(|name| (name, "property".into(), None)).collect::<Vec<_>>();
            return filter_completions(props, &prefix);
        }
    }

    if in_where_clause(before) {
        let logic = filter_completions(LOGIC_KEYWORDS.iter().map(|kw| (kw.to_string(), "keyword".into(), None)), &prefix);
        if !logic.is_empty() {
            return logic;
        }
    }

    let vars = collect_bound_vars(before);
    if !vars.is_empty() && prefix.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_') {
        let var_items = vars.into_iter().map(|name| (name, "variable".into(), None)).collect::<Vec<_>>();
        let filtered = filter_completions(var_items, &prefix);
        if !filtered.is_empty() {
            return filtered;
        }
    }

    filter_completions(CLAUSE_KEYWORDS.iter().map(|kw| (kw.to_string(), "keyword".into(), None)), &prefix)
}
// #endregion 🔖️Language

// #region 🔖️LanguageService
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Info,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    pub start: usize,
    pub end: usize,
    pub severity: DiagnosticSeverity,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hover {
    pub start: usize,
    pub end: usize,
    pub contents: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticToken {
    pub start: usize,
    pub end: usize,
    pub class: String,
}

async fn collect_pattern_vars(pattern: &Pattern, out: &mut BTreeSet<String>) {
    for node in &pattern.nodes {
        out.insert(node.var.clone());
    }
    if let Some(edge) = &pattern.edge {
        if let Some(var) = &edge.var {
            out.insert(var.clone());
        }
        out.insert(edge.right.var.clone());
    }
}

async fn collect_clause_bound_vars(clauses: &[Clause]) -> BTreeSet<String> {
    let mut vars = BTreeSet::new();
    for clause in clauses {
        match clause {
            Clause::Match(patterns) => {
                for pattern in patterns {
                    collect_pattern_vars(pattern, &mut vars);
                }
            }
            Clause::Create(pattern) | Clause::Merge(pattern) => collect_pattern_vars(pattern, &mut vars),
            _ => {}
        }
    }
    vars
}

async fn collect_referenced_vars(clauses: &[Clause]) -> Vec<(String, usize, usize)> {
    let mut refs = Vec::new();
    for clause in clauses {
        match clause {
            Clause::Return(items) => {
                for item in items {
                    match item {
                        ReturnItem::Var(v) => refs.push((v.clone(), 0, v.len())),
                        ReturnItem::Property { var, .. } => refs.push((var.clone(), 0, var.len())),
                    }
                }
            }
            Clause::Delete(vars) => {
                for var in vars {
                    refs.push((var.clone(), 0, var.len()));
                }
            }
            Clause::Set(assignments) => {
                for assignment in assignments {
                    refs.push((assignment.var.clone(), 0, assignment.var.len()));
                }
            }
            Clause::Where(expr) => collect_expr_vars(expr, &mut refs),
            _ => {}
        }
    }
    refs
}

async fn collect_expr_vars(expr: &Expr, refs: &mut Vec<(String, usize, usize)>) {
    match expr {
        Expr::Eq { var, .. } | Expr::Ne { var, .. } => refs.push((var.clone(), 0, var.len())),
        Expr::And(a, b) | Expr::Or(a, b) => {
            collect_expr_vars(a, refs);
            collect_expr_vars(b, refs);
        }
    }
}

async fn semantic_lints(graph: &dyn QueryableGraph, query: &Query, source: &str) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    let node_kinds = graph_node_kinds(graph).into_iter().collect::<BTreeSet<_>>();
    let edge_kinds = graph_edge_kinds(graph).into_iter().collect::<BTreeSet<_>>();
    let bound = collect_clause_bound_vars(&query.clauses);
    for clause in &query.clauses {
        match clause {
            Clause::Match(patterns) => {
                for pattern in patterns {
                    for node in &pattern.nodes {
                        if !node_kinds.contains(&node.kind) {
                            if let Some((start, end)) = find_kind_span(source, &node.kind) {
                                out.push(Diagnostic { start, end, severity: DiagnosticSeverity::Error, message: format!("unknown node kind '{}'", node.kind), code: Some("jack/unknown-node-kind".into()) });
                            }
                        }
                    }
                    if let Some(edge) = &pattern.edge {
                        if let Some(kind) = &edge.kind {
                            if !edge_kinds.contains(kind) {
                                if let Some((start, end)) = find_kind_span(source, kind) {
                                    out.push(Diagnostic { start, end, severity: DiagnosticSeverity::Error, message: format!("unknown edge kind '{}'", kind), code: Some("jack/unknown-edge-kind".into()) });
                                }
                            }
                        }
                    }
                }
            }
            Clause::Create(pattern) | Clause::Merge(pattern) => {
                for node in &pattern.nodes {
                    if !node_kinds.contains(&node.kind) {
                        if let Some((start, end)) = find_kind_span(source, &node.kind) {
                            out.push(Diagnostic { start, end, severity: DiagnosticSeverity::Error, message: format!("unknown node kind '{}'", node.kind), code: Some("jack/unknown-node-kind".into()) });
                        }
                    }
                }
            }
            _ => {}
        }
    }
    for (var, _, _) in collect_referenced_vars(&query.clauses) {
        if !bound.contains(&var) {
            if let Some((start, end)) = find_ident_span(source, &var) {
                out.push(Diagnostic { start, end, severity: DiagnosticSeverity::Error, message: format!("variable '{var}' is not bound by MATCH"), code: Some("jack/unbound-variable".into()) });
            }
        }
    }
    out
}

async fn find_kind_span(source: &str, kind: &str) -> Option<(usize, usize)> {
    let needle = format!(":{kind}");
    let start = source.find(&needle)?;
    Some((start + 1, start + needle.len()))
}

async fn find_ident_span(source: &str, ident: &str) -> Option<(usize, usize)> {
    let mut from = 0;
    while let Some(rel) = source[from..].find(ident) {
        let start = from + rel;
        let end = start + ident.len();
        let before = source.as_bytes().get(start.wrapping_sub(1));
        let after = source.as_bytes().get(end);
        let boundary_before = before.is_none_or(|c| !c.is_ascii_alphanumeric() && *c != b'_');
        let boundary_after = after.is_none_or(|c| !c.is_ascii_alphanumeric() && *c != b'_');
        if boundary_before && boundary_after {
            return Some((start, end));
        }
        from = end;
    }
    None
}

/// 🩺️ Lint jack source with syntax and semantic diagnostics.
pub async fn lint(graph: &dyn QueryableGraph, source: &str) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    for span in tokenize(source) {
        if span.class == TokenClass::Error {
            out.push(Diagnostic { start: span.start, end: span.end, severity: DiagnosticSeverity::Error, message: "unterminated string literal".into(), code: Some("jack/unterminated-string".into()) });
        }
    }
    match parse(source) {
        Ok(query) => out.extend(semantic_lints(graph, &query, source)),
        Err(err) => {
            let end = source.len().max(1);
            out.push(Diagnostic { start: 0, end, severity: DiagnosticSeverity::Error, message: err.to_string(), code: Some("jack/parse-error".into()) });
        }
    }
    out
}

async fn format_token(tok: &Token) -> String {
    match tok {
        Token::KwMatch => "MATCH".into(),
        Token::KwWhere => "WHERE".into(),
        Token::KwReturn => "RETURN".into(),
        Token::KwCreate => "CREATE".into(),
        Token::KwDelete => "DELETE".into(),
        Token::KwSet => "SET".into(),
        Token::KwMerge => "MERGE".into(),
        Token::KwWith => "WITH".into(),
        Token::KwUnwind => "UNWIND".into(),
        Token::KwCall => "CALL".into(),
        Token::KwAs => "AS".into(),
        Token::And => "AND".into(),
        Token::Or => "OR".into(),
        Token::Ident(s) => s.clone(),
        Token::Number(n) => {
            if n.fract() == 0.0 {
                format!("{}", *n as i64)
            } else {
                n.to_string()
            }
        }
        // 🩹️ unified syntax law: strings always PRINT double-quoted with `dsl_core`'s canonical
        // escape, regardless of which quote style the source used.
        Token::StringLit(s) => format!("\"{}\"", dsl_core::os_dsl::escape_text(s)),
        Token::LParen => "(".into(),
        Token::RParen => ")".into(),
        Token::LBracket => "[".into(),
        Token::RBracket => "]".into(),
        Token::Colon => ":".into(),
        Token::Comma => ",".into(),
        Token::Dot => ".".into(),
        Token::Eq => "=".into(),
        Token::Ne => "!=".into(),
        Token::Arrow => "->".into(),
        Token::DashArrow => "--".into(),
        Token::BackArrow => "<-".into(),
        Token::At => "@".into(),
        Token::Eof => String::new(),
    }
}

/// 🪞️ Format jack source canonically (idempotent).
pub async fn format(source: &str) -> Result<String, GraphDslError> {
    let tokens = lex_spanned(source, false)?;
    let mut out = String::new();
    let mut line_open = false;
    let mut i = 0;
    while i < tokens.len() {
        let row = &tokens[i];
        if matches!(row.token, Token::Eof) {
            break;
        }
        match &row.token {
            Token::KwMatch | Token::KwWhere | Token::KwReturn | Token::KwCreate | Token::KwDelete | Token::KwSet | Token::KwMerge | Token::KwWith | Token::KwUnwind | Token::KwCall => {
                if !out.is_empty() {
                    out.push('\n');
                }
                out.push_str(&format_token(&row.token));
                out.push(' ');
                line_open = true;
            }
            Token::Comma => {
                out.push_str(", ");
            }
            Token::Arrow => {
                out.push_str("->");
            }
            Token::DashArrow => {
                out.push_str("--");
            }
            Token::BackArrow => {
                out.push_str("<-");
            }
            Token::And | Token::Or | Token::KwAs => {
                out.push(' ');
                out.push_str(&format_token(&row.token));
                out.push(' ');
            }
            Token::Eq | Token::Ne => {
                out.push(' ');
                out.push_str(&format_token(&row.token));
                out.push(' ');
            }
            _ => {
                if line_open && !out.ends_with(' ') && !out.ends_with('\n') && !matches!(row.token, Token::RParen | Token::RBracket | Token::Comma | Token::Dot) {
                    let prev = tokens.get(i.saturating_sub(1)).map(|t| &t.token);
                    if !matches!(prev, Some(Token::LParen | Token::LBracket | Token::Colon | Token::Dot | Token::Arrow | Token::DashArrow | Token::BackArrow)) {
                        out.push(' ');
                    }
                }
                out.push_str(&format_token(&row.token));
            }
        }
        i += 1;
    }
    Ok(out.trim().to_string())
}

async fn hover_word_at(source: &str, cursor: usize) -> Option<(usize, usize, String)> {
    let cursor = cursor.min(source.len());
    if cursor > source.len() {
        return None;
    }
    let bytes = source.as_bytes();
    let mut start = cursor;
    while start > 0 {
        let c = bytes[start - 1];
        if c.is_ascii_alphanumeric() || c == b'_' || c == b':' || c == b'.' {
            start -= 1;
        } else {
            break;
        }
    }
    let mut end = cursor;
    while end < bytes.len() {
        let c = bytes[end];
        if c.is_ascii_alphanumeric() || c == b'_' || c == b':' || c == b'.' {
            end += 1;
        } else {
            break;
        }
    }
    if start == end {
        return None;
    }
    Some((start, end, source[start..end].to_string()))
}

/// 💬️ Hover information at cursor.
pub async fn hover(graph: &dyn QueryableGraph, source: &str, cursor: usize) -> Option<Hover> {
    let (start, end, word) = hover_word_at(source, cursor)?;
    let upper = word.to_ascii_uppercase();
    if CLAUSE_KEYWORDS.iter().any(|kw| *kw == upper) || LOGIC_KEYWORDS.iter().any(|kw| *kw == upper) {
        return Some(Hover { start, end, contents: format!("Jack keyword `{upper}`") });
    }
    if graph_node_kinds(graph).iter().any(|kind| kind == &word) {
        return Some(Hover { start, end, contents: format!("Node kind `{word}`") });
    }
    if graph_edge_kinds(graph).iter().any(|kind| kind == &word) {
        return Some(Hover { start, end, contents: format!("Edge kind `{word}`") });
    }
    if graph_property_names(graph).iter().any(|prop| prop == &word) {
        return Some(Hover { start, end, contents: format!("Property `{word}`") });
    }
    if collect_bound_vars(&lex_spanned(source, true).unwrap_or_default()).contains(&word) {
        return Some(Hover { start, end, contents: format!("Bound variable `{word}`") });
    }
    None
}

/// 🎨️ Semantic token classes for LSP highlighting.
pub async fn semantic_tokens(source: &str) -> Vec<SemanticToken> {
    tokenize(source)
        .into_iter()
        .map(|span| SemanticToken {
            start: span.start,
            end: span.end,
            class: match span.class {
                TokenClass::Keyword => "keyword",
                TokenClass::Ident => "ident",
                TokenClass::Number => "number",
                TokenClass::String => "string",
                TokenClass::Operator => "operator",
                TokenClass::Punctuation => "punctuation",
                TokenClass::Error => "error",
            }
            .into(),
        })
        .collect()
}
// #endregion 🔖️LanguageService

// #region 🔖️DslIdiom
/// 🔌️ Registers Jack as a `dsl_core::IdiomHooks` entry (the `DslIdiom` seam's Route B — an EMBEDDED
/// idiom hosted inside another document's `Shape::Embed("jack")` field) so `canonicalize`
/// normalizes embedded Jack text through this crate's own `format`/`tokenize`. Hand-built rather
/// than `dsl_core::hooks_for::<I: DslIdiom>()`: that helper needs `DslIdiom::print(ast) -> String`, and
/// Jack has no AST-to-text printer (`format` re-derives canonical text token-by-token from SOURCE,
/// not from a `Query`) — `IdiomHooks` itself only needs function pointers, so it's built directly
/// from the language-service surface Jack already has, no printer required.
pub async fn idiom_hooks() -> dsl_core::IdiomHooks {
    dsl_core::IdiomHooks { lang: "jack", canonicalize: idiom_canonicalize, classify: idiom_classify, complete: idiom_complete }
}

async fn idiom_canonicalize(text: &str) -> Result<String, dsl_core::TextError> {
    format(text).map_err(|e| dsl_core::TextError::new(e.to_string(), dsl_core::TextSpan::at(1, 1)))
}

async fn idiom_classify(text: &str) -> Vec<(dsl_core::TokenClass, dsl_core::TextSpan)> {
    tokenize(text)
        .into_iter()
        .map(|span| {
            let class = match span.class {
                TokenClass::Keyword => dsl_core::TokenClass::Keyword,
                TokenClass::Ident => dsl_core::TokenClass::Ident,
                TokenClass::Number => dsl_core::TokenClass::Number,
                TokenClass::String => dsl_core::TokenClass::String,
                TokenClass::Operator => dsl_core::TokenClass::Operator,
                TokenClass::Punctuation => dsl_core::TokenClass::Punctuation,
                TokenClass::Error => dsl_core::TokenClass::Error,
            };
            (class, byte_range_to_span(text, span.start, span.end))
        })
        .collect()
}

async fn idiom_complete(text: &str, offset: usize) -> Vec<dsl_core::CompletionItem> {
    // Jack's own `complete` needs a `&dyn QueryableGraph` for schema-aware suggestions (node/edge
    // kinds, property names) that the generic `DslIdiom`/embed-host seam has no graph to supply —
    // an empty graph still exercises the syntax-only completions (clause/logic keywords).
    struct EmptyGraph;
    impl QueryableGraph for EmptyGraph {
        async fn manifest(&self) -> Option<&crate::manifest::GraphManifest> {
            None
        }
        async fn node_ids(&self) -> Vec<String> {
            Vec::new()
        }
        async fn node_kind(&self, _id: &str) -> Option<String> {
            None
        }
        async fn node_name(&self, _id: &str) -> Option<String> {
            None
        }
        async fn node_property(&self, _id: &str, _key: &str) -> Option<PropertyValue> {
            None
        }
        async fn edges(&self) -> Vec<QueryableEdge> {
            Vec::new()
        }
        async fn subgraph_fixture_json(&self, _node_ids: &BTreeSet<String>, _edge_ids: &BTreeSet<String>) -> Option<String> {
            None
        }
    }
    complete(&EmptyGraph, text, offset).into_iter().map(|c| dsl_core::CompletionItem { label: c.label, detail: c.detail }).collect()
}

/// 📍️ Converts a byte-offset half-open range into `os_dsl::TextSpan`'s 1-based line/column/
/// length form — Jack's own spans are byte offsets (`TokenSpan`/`SemanticToken`), `dsl_core`'s are
/// line/column, so this is the one place that needs the source text to translate between them.
async fn byte_range_to_span(text: &str, start: usize, end: usize) -> dsl_core::TextSpan {
    let mut line = 1u32;
    let mut column = 1u32;
    for (i, ch) in text.char_indices() {
        if i >= start {
            break;
        }
        if ch == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    let length = text.get(start..end).map_or(0, |s| s.chars().count()) as u32;
    dsl_core::TextSpan::with_length(line, column, length)
}
// #endregion 🔖️DslIdiom

// #region 🔖️Parser
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    async fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    async fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    async fn bump(&mut self) -> Token {
        let t = self.peek().clone();
        if !matches!(t, Token::Eof) {
            self.pos += 1;
        }
        t
    }

    async fn expect_ident(&mut self) -> Result<String, GraphDslError> {
        match self.bump() {
            Token::Ident(s) => Ok(s),
            other => Err(GraphDslError::UnexpectedToken { expected: "ident".into(), found: format!("{other:?}") }),
        }
    }

    async fn parse_query(&mut self) -> Result<Query, GraphDslError> {
        let mut clauses = Vec::new();
        while !matches!(self.peek(), Token::Eof) {
            clauses.push(self.parse_clause()?);
        }
        Ok(Query { clauses })
    }

    async fn parse_clause(&mut self) -> Result<Clause, GraphDslError> {
        match self.peek() {
            Token::KwMatch => {
                self.bump();
                let mut patterns = vec![self.parse_pattern()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    patterns.push(self.parse_pattern()?);
                }
                Ok(Clause::Match(patterns))
            }
            Token::KwWhere => {
                self.bump();
                Ok(Clause::Where(self.parse_expr()?))
            }
            Token::KwReturn => {
                self.bump();
                let mut items = vec![self.parse_return_item()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    items.push(self.parse_return_item()?);
                }
                Ok(Clause::Return(items))
            }
            Token::KwCreate => {
                self.bump();
                Ok(Clause::Create(self.parse_pattern()?))
            }
            Token::KwDelete => {
                self.bump();
                let mut vars = vec![self.expect_ident()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    vars.push(self.expect_ident()?);
                }
                Ok(Clause::Delete(vars))
            }
            Token::KwSet => {
                self.bump();
                let mut items = vec![self.parse_assignment()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    items.push(self.parse_assignment()?);
                }
                Ok(Clause::Set(items))
            }
            Token::KwMerge => {
                self.bump();
                Ok(Clause::Merge(self.parse_pattern()?))
            }
            Token::KwWith => {
                self.bump();
                let mut items = vec![self.parse_return_item()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    items.push(self.parse_return_item()?);
                }
                Ok(Clause::With(items))
            }
            Token::KwUnwind => {
                self.bump();
                let source = self.parse_return_item()?;
                self.expect(&Token::KwAs)?;
                let var = self.expect_ident()?;
                Ok(Clause::Unwind(UnwindClause { source, var }))
            }
            Token::KwCall => {
                self.bump();
                let name = self.expect_ident()?;
                self.expect(&Token::LParen)?;
                let mut args = Vec::new();
                if !matches!(self.peek(), Token::RParen) {
                    args.push(self.parse_value()?);
                    while matches!(self.peek(), Token::Comma) {
                        self.bump();
                        args.push(self.parse_value()?);
                    }
                }
                self.expect(&Token::RParen)?;
                Ok(Clause::Call(CallClause { name, args }))
            }
            other => Err(GraphDslError::UnexpectedToken { expected: "clause start (MATCH/WHERE/RETURN/CREATE/DELETE/SET/MERGE/WITH/UNWIND/CALL)".into(), found: format!("{other:?}") }),
        }
    }

    /// 🕸️ Pattern grammar over the unified token alphabet — `dsl_core` has no standalone `-`
    /// token (only `->`/`--`/`<-`), so the leading connector before a bracketed edge label is
    /// always `--` or `<-`, never a bare dash (real Cypher's `-[r]->`/`<-[r]-` shape, adapted to
    /// this repo's alphabet). `<-` at the front means the edge points INTO `left`; represented by
    /// swapping which parsed node plays "left" so the stored `PatternEdge.right` is always the
    /// forward-direction target, mirroring `dsl_schema`'s own wire `<-` normalization.
    async fn parse_pattern(&mut self) -> Result<Pattern, GraphDslError> {
        self.expect(&Token::LParen)?;
        let left = self.parse_pattern_node()?;
        self.expect(&Token::RParen)?;
        match self.peek().clone() {
            Token::Arrow => {
                self.bump();
                let right = self.parse_bracketed_pattern_node()?;
                Ok(Pattern { nodes: vec![left], edge: Some(PatternEdge { var: None, kind: None, directed: true, right }) })
            }
            Token::DashArrow => {
                self.bump();
                if matches!(self.peek(), Token::LBracket) {
                    let (edge_var, edge_kind) = self.parse_edge_label()?;
                    let directed = match self.peek() {
                        Token::Arrow => {
                            self.bump();
                            true
                        }
                        Token::DashArrow => {
                            self.bump();
                            false
                        }
                        other => return Err(GraphDslError::UnexpectedToken { expected: "-> or --".into(), found: format!("{other:?}") }),
                    };
                    let right = self.parse_bracketed_pattern_node()?;
                    Ok(Pattern { nodes: vec![left], edge: Some(PatternEdge { var: edge_var, kind: edge_kind, directed, right }) })
                } else {
                    let right = self.parse_bracketed_pattern_node()?;
                    Ok(Pattern { nodes: vec![left], edge: Some(PatternEdge { var: None, kind: None, directed: false, right }) })
                }
            }
            Token::BackArrow => {
                self.bump();
                if matches!(self.peek(), Token::LBracket) {
                    let (edge_var, edge_kind) = self.parse_edge_label()?;
                    self.expect(&Token::DashArrow)?;
                    let right = self.parse_bracketed_pattern_node()?;
                    Ok(Pattern { nodes: vec![right], edge: Some(PatternEdge { var: edge_var, kind: edge_kind, directed: true, right: left }) })
                } else {
                    let right = self.parse_bracketed_pattern_node()?;
                    Ok(Pattern { nodes: vec![right], edge: Some(PatternEdge { var: None, kind: None, directed: true, right: left }) })
                }
            }
            _ => Ok(Pattern { nodes: vec![left], edge: None }),
        }
    }

    async fn parse_bracketed_pattern_node(&mut self) -> Result<PatternNode, GraphDslError> {
        self.expect(&Token::LParen)?;
        let node = self.parse_pattern_node()?;
        self.expect(&Token::RParen)?;
        Ok(node)
    }

    async fn parse_edge_label(&mut self) -> Result<(Option<String>, Option<String>), GraphDslError> {
        self.expect(&Token::LBracket)?;
        let edge_var = if matches!(self.peek(), Token::Ident(_)) { Some(self.expect_ident()?) } else { None };
        let edge_kind = if matches!(self.peek(), Token::Colon) {
            self.bump();
            Some(self.expect_ident()?)
        } else {
            None
        };
        self.expect(&Token::RBracket)?;
        Ok((edge_var, edge_kind))
    }

    async fn parse_pattern_node(&mut self) -> Result<PatternNode, GraphDslError> {
        let var = self.expect_ident()?;
        self.expect(&Token::Colon)?;
        let kind = self.expect_ident()?;
        let port = if matches!(self.peek(), Token::At) {
            self.bump();
            Some(self.expect_ident()?)
        } else {
            None
        };
        Ok(PatternNode { var, kind, port })
    }

    async fn parse_return_item(&mut self) -> Result<ReturnItem, GraphDslError> {
        let var = self.expect_ident()?;
        if matches!(self.peek(), Token::Dot) {
            self.bump();
            let prop = self.expect_ident()?;
            Ok(ReturnItem::Property { var, prop })
        } else {
            Ok(ReturnItem::Var(var))
        }
    }

    async fn parse_assignment(&mut self) -> Result<Assignment, GraphDslError> {
        let var = self.expect_ident()?;
        self.expect(&Token::Dot)?;
        let prop = self.expect_ident()?;
        self.expect(&Token::Eq)?;
        let value = self.parse_value()?;
        Ok(Assignment { var, prop, value })
    }

    async fn parse_expr(&mut self) -> Result<Expr, GraphDslError> {
        self.parse_or_expr()
    }

    async fn parse_or_expr(&mut self) -> Result<Expr, GraphDslError> {
        let mut left = self.parse_and_expr()?;
        while matches!(self.peek(), Token::Or) {
            self.bump();
            let right = self.parse_and_expr()?;
            left = Expr::Or(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    async fn parse_and_expr(&mut self) -> Result<Expr, GraphDslError> {
        let mut left = self.parse_cmp_expr()?;
        while matches!(self.peek(), Token::And) {
            self.bump();
            let right = self.parse_cmp_expr()?;
            left = Expr::And(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    async fn parse_cmp_expr(&mut self) -> Result<Expr, GraphDslError> {
        let var = self.expect_ident()?;
        self.expect(&Token::Dot)?;
        let prop = self.expect_ident()?;
        match self.bump() {
            Token::Eq => Ok(Expr::Eq { var, prop, value: self.parse_value()? }),
            Token::Ne => Ok(Expr::Ne { var, prop, value: self.parse_value()? }),
            other => Err(GraphDslError::UnexpectedToken { expected: "= or !=".into(), found: format!("{other:?}") }),
        }
    }

    async fn parse_value(&mut self) -> Result<PropertyValue, GraphDslError> {
        match self.bump() {
            Token::Number(n) => Ok(PropertyValue::Number(n)),
            Token::StringLit(s) => Ok(PropertyValue::String(s)),
            Token::Ident(s) if s.eq_ignore_ascii_case("true") => Ok(PropertyValue::Bool(true)),
            Token::Ident(s) if s.eq_ignore_ascii_case("false") => Ok(PropertyValue::Bool(false)),
            Token::Ident(s) if s.eq_ignore_ascii_case("null") => Ok(PropertyValue::Null),
            other => Err(GraphDslError::UnexpectedToken { expected: "value".into(), found: format!("{other:?}") }),
        }
    }

    async fn expect(&mut self, want: &Token) -> Result<(), GraphDslError> {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(want) {
            self.bump();
            Ok(())
        } else {
            Err(GraphDslError::UnexpectedToken { expected: format!("{want:?}"), found: format!("{:?}", self.peek()) })
        }
    }
}

/// 🔍️ Parse a jack query string.
pub async fn parse(query: &str) -> Result<Query, GraphDslError> {
    let tokens = lex(query)?;
    Parser::new(tokens).parse_query()
}
// #endregion 🔖️Parser

// #region 🔖️Executor
/// 🎯️ Variable binding in a match row.
#[derive(Clone, Debug, Default)]
pub struct Binding {
    pub nodes: BTreeMap<String, String>,
    pub edges: BTreeMap<String, String>,
}

/// ▶️ Execute a read-only jack query against a queryable graph.
pub async fn execute(graph: &dyn QueryableGraph, query: &Query) -> Result<QueryResult, GraphDslError> {
    let mut bindings: Vec<Binding> = vec![Binding::default()];
    let mut return_items: Option<Vec<ReturnItem>> = None;
    for clause in &query.clauses {
        match clause {
            Clause::Match(patterns) => bindings = match_patterns(graph, patterns)?,
            Clause::Where(expr) => bindings.retain(|b| eval_expr(graph, b, expr)),
            Clause::Return(items) => return_items = Some(items.clone()),
            Clause::Create(_) | Clause::Delete(_) | Clause::Set(_) | Clause::Merge(_) => {
                return Err(GraphDslError::UnsupportedMutation);
            }
            // TODO(unify-architect): wire up WITH/UNWIND/CALL execution once semio_compose_rs's Architect
            // query language unifies onto Jack (see plans/every-dsl-must-be-crispy-shell.md,
            // Wave 2 / P9). They already parse into the AST — this is prep work only.
            Clause::With(_) | Clause::Unwind(_) | Clause::Call(_) => {
                return Err(GraphDslError::UnsupportedClause);
            }
        }
    }
    if let Some(items) = return_items {
        return Ok(build_return(graph, &bindings, &items));
    }
    Ok(QueryResult::table(vec![], vec![]))
}

/// ▶️ Parse and execute jack in one step.
pub async fn run_query(graph: &dyn QueryableGraph, source: &str) -> Result<QueryResult, GraphDslError> {
    execute(graph, &parse(source)?)
}

/// ▶️ Execute jack and return JSON result.
pub async fn run_query_json(graph: &dyn QueryableGraph, source: &str) -> Result<String, GraphDslError> {
    Ok(serde_json::to_string(&run_query(graph, source)?)?)
}

async fn match_patterns(graph: &dyn QueryableGraph, patterns: &[Pattern]) -> Result<Vec<Binding>, GraphDslError> {
    let mut bindings = vec![Binding::default()];
    for pattern in patterns {
        let mut next = Vec::new();
        for binding in &bindings {
            next.extend(match_pattern(graph, pattern, binding)?);
        }
        bindings = next;
    }
    Ok(bindings)
}

async fn match_pattern(graph: &dyn QueryableGraph, pattern: &Pattern, base: &Binding) -> Result<Vec<Binding>, GraphDslError> {
    let left = pattern.nodes.first().ok_or(GraphDslError::EmptyPattern)?;
    if let Some(edge_pat) = &pattern.edge {
        let mut out = Vec::new();
        for node_id in graph.node_ids() {
            if graph.node_kind(node_id.as_str()).as_deref() != Some(left.kind.as_str()) {
                continue;
            }
            if binding_conflicts(base, &left.var, node_id.as_str()) {
                continue;
            }
            for edge in graph.edges() {
                if edge_pat.kind.as_ref().is_some_and(|k| *k != edge.kind) {
                    continue;
                }
                let pairs = if edge_pat.directed {
                    vec![(edge.source_node_id.as_str(), edge.target_node_id.as_str(), edge.source_port.as_deref(), edge.target_port.as_deref())]
                } else {
                    vec![
                        (edge.source_node_id.as_str(), edge.target_node_id.as_str(), edge.source_port.as_deref(), edge.target_port.as_deref()),
                        (edge.target_node_id.as_str(), edge.source_node_id.as_str(), edge.target_port.as_deref(), edge.source_port.as_deref()),
                    ]
                };
                for (src_id, tgt_id, src_port, tgt_port) in pairs {
                    if src_id != node_id {
                        continue;
                    }
                    if left.port.as_ref().is_some_and(|want| src_port != Some(want.as_str())) {
                        continue;
                    }
                    if graph.node_kind(tgt_id).as_deref() != Some(edge_pat.right.kind.as_str()) {
                        continue;
                    }
                    if edge_pat.right.port.as_ref().is_some_and(|want| tgt_port != Some(want.as_str())) {
                        continue;
                    }
                    let mut b = base.clone();
                    b.nodes.insert(left.var.clone(), node_id.clone());
                    if let Some(ev) = &edge_pat.var {
                        b.edges.insert(ev.clone(), edge.id.clone());
                    }
                    if binding_conflicts(base, &edge_pat.right.var, tgt_id) {
                        continue;
                    }
                    b.nodes.insert(edge_pat.right.var.clone(), tgt_id.to_string());
                    out.push(b);
                }
            }
        }
        return Ok(out);
    }
    let mut out = Vec::new();
    for node_id in graph.node_ids() {
        if graph.node_kind(node_id.as_str()).as_deref() != Some(left.kind.as_str()) {
            continue;
        }
        if binding_conflicts(base, &left.var, node_id.as_str()) {
            continue;
        }
        let mut b = base.clone();
        b.nodes.insert(left.var.clone(), node_id);
        out.push(b);
    }
    Ok(out)
}

async fn binding_conflicts(base: &Binding, var: &str, node_id: &str) -> bool {
    base.nodes.get(var).is_some_and(|existing| existing != node_id)
}

async fn eval_expr(graph: &dyn QueryableGraph, binding: &Binding, expr: &Expr) -> bool {
    match expr {
        Expr::Eq { var, prop, value } => binding_value(graph, binding, var, prop) == Some(value.clone()),
        Expr::Ne { var, prop, value } => binding_value(graph, binding, var, prop) != Some(value.clone()),
        Expr::And(a, b) => eval_expr(graph, binding, a) && eval_expr(graph, binding, b),
        Expr::Or(a, b) => eval_expr(graph, binding, a) || eval_expr(graph, binding, b),
    }
}

async fn binding_value(graph: &dyn QueryableGraph, binding: &Binding, var: &str, prop: &str) -> Option<PropertyValue> {
    let node_id = binding.nodes.get(var)?;
    graph.node_property(node_id, prop)
}

async fn binding_has_entity(binding: &Binding, var: &str) -> bool {
    binding.nodes.contains_key(var) || binding.edges.contains_key(var)
}

async fn return_items_want_graph(items: &[ReturnItem], bindings: &[Binding]) -> bool {
    items.iter().any(|item| {
        let ReturnItem::Var(v) = item else { return false };
        bindings.iter().any(|b| binding_has_entity(b, v))
    })
}

async fn collect_graph_entities(bindings: &[Binding], items: &[ReturnItem]) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut node_ids = BTreeSet::new();
    let mut edge_ids = BTreeSet::new();
    for binding in bindings {
        for item in items {
            if let ReturnItem::Var(v) = item {
                if let Some(id) = binding.nodes.get(v) {
                    node_ids.insert(id.clone());
                }
                if let Some(id) = binding.edges.get(v) {
                    edge_ids.insert(id.clone());
                }
            }
        }
    }
    (node_ids, edge_ids)
}

async fn build_return(graph: &dyn QueryableGraph, bindings: &[Binding], items: &[ReturnItem]) -> QueryResult {
    let columns: Vec<String> = items
        .iter()
        .map(|item| match item {
            ReturnItem::Var(v) => v.clone(),
            ReturnItem::Property { var, prop } => format!("{var}.{prop}"),
        })
        .collect();
    if return_items_want_graph(items, bindings) {
        let (node_ids, edge_ids) = collect_graph_entities(bindings, items);
        if let Some(json) = graph.subgraph_fixture_json(&node_ids, &edge_ids) {
            return QueryResult::graph(columns, json);
        }
    }
    let mut rows = Vec::new();
    for binding in bindings {
        let mut row = Vec::new();
        for item in items {
            let val = match item {
                ReturnItem::Var(v) => binding.nodes.get(v).and_then(|id| graph.node_name(id)).map_or(PropertyValue::Null, PropertyValue::String),
                ReturnItem::Property { var, prop } => binding_value(graph, binding, var, prop).unwrap_or(PropertyValue::Null),
            };
            row.push(val);
        }
        rows.push(row);
    }
    QueryResult::table(columns, rows)
}
// #endregion 🔖️Executor

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn parse_match_return() {
        let q = parse("MATCH (a:computation) RETURN a.name").unwrap();
        assert_eq!(q.clauses.len(), 2);
    }

    #[test]
    async fn idiom_hooks_canonicalize_and_classify_through_the_dsl_registry_seam() {
        let hooks = idiom_hooks();
        assert_eq!(hooks.lang, "jack");
        let canonical = (hooks.canonicalize)("MATCH   (a:computation)   RETURN   a.name").expect("canonicalize");
        assert_eq!(canonical, format("MATCH (a:computation) RETURN a.name").unwrap());
        assert!((hooks.canonicalize)("not jack at all $$$").is_err() || (hooks.canonicalize)("not jack at all $$$").is_ok(), "canonicalize must not panic on malformed input");
        let classes = (hooks.classify)("MATCH (a:computation) RETURN a.name");
        assert!(!classes.is_empty());
        assert!(classes.iter().any(|(class, _)| *class == dsl_core::TokenClass::Keyword), "MATCH/RETURN must classify as keywords");
        dsl_core::register_idiom(hooks);
        let resolved = dsl_core::idiom("jack").expect("jack must be resolvable by lang id after registration");
        assert_eq!((resolved.canonicalize)("MATCH (a:computation) RETURN a.name").unwrap(), format("MATCH (a:computation) RETURN a.name").unwrap());
    }

    #[test]
    async fn run_dag_fixture_query() {
        // 🩹️ Was `include_str!` of the dag technology's example fixture; that technology migrated its
        // fixture to a handcrafted DSL (`store::ArtifactDsl`) — inlined the same dag-fixture JSON this
        // test actually parses (`from_dag_fixture_json`), decoupled from its document format.
        let fixture = r#"{
  "schema": "dag.fixture",
  "camera": { "x": 0, "y": 0, "zoom": 1 },
  "nodes": [
    {
      "id": "slider",
      "name": "Amount",
      "abbreviation": "Amount",
      "icon": "emoji:🎚️",
      "kind": "slider",
      "x": -400,
      "y": -40,
      "width": 70,
      "height": 14,
      "min": 0,
      "max": 10,
      "step": 0.5,
      "value": 5,
      "output": { "id": "out", "label": "value", "cardinality": "!" }
    },
    {
      "id": "mode",
      "name": "Mode",
      "abbreviation": "Mode",
      "icon": "emoji:📋️",
      "kind": "select",
      "x": -400,
      "y": 80,
      "width": 56,
      "height": 28,
      "options": ["Add", "Multiply", "Max"],
      "selected": 0,
      "output": { "id": "out", "label": "mode", "cardinality": "!" }
    },
    {
      "id": "scale",
      "name": "Scale",
      "abbreviation": "Scale",
      "icon": "emoji:📐️",
      "kind": "computation",
      "x": -120,
      "y": -40,
      "width": 104,
      "height": 14,
      "inputs": [{ "id": "in", "label": "value", "cardinality": "!" }],
      "outputs": [{ "id": "out", "label": "scaled", "cardinality": "!" }]
    },
    {
      "id": "combine",
      "name": "Combine",
      "abbreviation": "Combine",
      "icon": "emoji:🔀️",
      "kind": "computation",
      "x": 120,
      "y": 0,
      "width": 104,
      "height": 28,
      "inputs": [
        { "id": "a", "label": "a", "cardinality": "!" },
        { "id": "b", "label": "b", "cardinality": "!" }
      ],
      "outputs": [{ "id": "out", "label": "merged", "cardinality": "!" }]
    },
    {
      "id": "screen",
      "name": "Preview",
      "abbreviation": "Preview",
      "icon": "emoji:🖥️",
      "kind": "screen",
      "x": 400,
      "y": 0,
      "width": 200,
      "height": 140,
      "media": {
        "kind": "svg",
        "src": "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 60'%3E%3Crect fill='%233c78d8' width='100' height='60'/%3E%3Ctext x='50' y='35' text-anchor='middle' fill='white' font-size='12'%3EDAG%3C/text%3E%3C/svg%3E"
      },
      "input": { "id": "in", "label": "result", "cardinality": "!" }
    }
  ],
  "edges": [
    { "id": "e1", "source": "slider:out", "target": "scale:in" },
    { "id": "e2", "source": "scale:out", "target": "combine:a" },
    { "id": "e3", "source": "mode:out", "target": "combine:b" },
    { "id": "e4", "source": "combine:out", "target": "screen:in" }
  ]
}
"#;
        let graph = BoardQueryableGraph::from_dag_fixture_json(fixture).unwrap();
        let result = run_query(&graph, "MATCH (n:computation) RETURN n.name").unwrap();
        assert!(!result.rows.is_empty());
    }

    #[test]
    async fn parse_match_with_port() {
        let q = parse("MATCH (a:computation@out) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &q.clauses[0] else { panic!("expected match") };
        assert_eq!(patterns[0].nodes[0].port.as_deref(), Some("out"));
    }

    #[test]
    async fn parse_undirected_edge() {
        // 🩹️ unified undirected sigil is `--`, not the old bare `-` (not even lexable in the
        // shared `dsl_core` alphabet, which has no standalone dash token).
        let q = parse("MATCH (a:computation)--(b:slider) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &q.clauses[0] else { panic!("expected match") };
        let edge = patterns[0].edge.as_ref().expect("edge");
        assert!(!edge.directed);
    }

    #[test]
    async fn parse_back_arrow_edge_swaps_left_and_right() {
        let q = parse("MATCH (a:computation)<-(b:slider) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &q.clauses[0] else { panic!("expected match") };
        // `<-` means the edge points INTO the parenthesized-first node — represented by swapping
        // which parsed node plays "left" so `edge.right` is always the forward-direction target.
        assert_eq!(patterns[0].nodes[0].kind, "slider");
        let edge = patterns[0].edge.as_ref().expect("edge");
        assert!(edge.directed);
        assert_eq!(edge.right.kind, "computation");
    }

    #[test]
    async fn parse_labeled_directed_and_undirected_edges_use_double_dash_connector() {
        let forward = parse("MATCH (a:computation)--[r:wire]->(b:slider) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &forward.clauses[0] else { panic!("expected match") };
        let edge = patterns[0].edge.as_ref().expect("edge");
        assert!(edge.directed);
        assert_eq!(edge.var.as_deref(), Some("r"));
        assert_eq!(edge.kind.as_deref(), Some("wire"));

        let undirected = parse("MATCH (a:computation)--[:wire]--(b:slider) RETURN a.name").unwrap();
        let Clause::Match(patterns) = &undirected.clauses[0] else { panic!("expected match") };
        let edge = patterns[0].edge.as_ref().expect("edge");
        assert!(!edge.directed);
    }

    #[test]
    async fn run_port_filtered_query() {
        // 🩹️ Was `include_str!` of the dag technology's example fixture; that technology migrated its
        // fixture to a handcrafted DSL (`store::ArtifactDsl`) — inlined the same dag-fixture JSON this
        // test actually parses (`from_dag_fixture_json`), decoupled from its document format.
        let fixture = r#"{
  "schema": "dag.fixture",
  "camera": { "x": 0, "y": 0, "zoom": 1 },
  "nodes": [
    {
      "id": "slider",
      "name": "Amount",
      "abbreviation": "Amount",
      "icon": "emoji:🎚️",
      "kind": "slider",
      "x": -400,
      "y": -40,
      "width": 70,
      "height": 14,
      "min": 0,
      "max": 10,
      "step": 0.5,
      "value": 5,
      "output": { "id": "out", "label": "value", "cardinality": "!" }
    },
    {
      "id": "mode",
      "name": "Mode",
      "abbreviation": "Mode",
      "icon": "emoji:📋️",
      "kind": "select",
      "x": -400,
      "y": 80,
      "width": 56,
      "height": 28,
      "options": ["Add", "Multiply", "Max"],
      "selected": 0,
      "output": { "id": "out", "label": "mode", "cardinality": "!" }
    },
    {
      "id": "scale",
      "name": "Scale",
      "abbreviation": "Scale",
      "icon": "emoji:📐️",
      "kind": "computation",
      "x": -120,
      "y": -40,
      "width": 104,
      "height": 14,
      "inputs": [{ "id": "in", "label": "value", "cardinality": "!" }],
      "outputs": [{ "id": "out", "label": "scaled", "cardinality": "!" }]
    },
    {
      "id": "combine",
      "name": "Combine",
      "abbreviation": "Combine",
      "icon": "emoji:🔀️",
      "kind": "computation",
      "x": 120,
      "y": 0,
      "width": 104,
      "height": 28,
      "inputs": [
        { "id": "a", "label": "a", "cardinality": "!" },
        { "id": "b", "label": "b", "cardinality": "!" }
      ],
      "outputs": [{ "id": "out", "label": "merged", "cardinality": "!" }]
    },
    {
      "id": "screen",
      "name": "Preview",
      "abbreviation": "Preview",
      "icon": "emoji:🖥️",
      "kind": "screen",
      "x": 400,
      "y": 0,
      "width": 200,
      "height": 140,
      "media": {
        "kind": "svg",
        "src": "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 60'%3E%3Crect fill='%233c78d8' width='100' height='60'/%3E%3Ctext x='50' y='35' text-anchor='middle' fill='white' font-size='12'%3EDAG%3C/text%3E%3C/svg%3E"
      },
      "input": { "id": "in", "label": "result", "cardinality": "!" }
    }
  ],
  "edges": [
    { "id": "e1", "source": "slider:out", "target": "scale:in" },
    { "id": "e2", "source": "scale:out", "target": "combine:a" },
    { "id": "e3", "source": "mode:out", "target": "combine:b" },
    { "id": "e4", "source": "combine:out", "target": "screen:in" }
  ]
}
"#;
        let graph = BoardQueryableGraph::from_dag_fixture_json(fixture).unwrap();
        let result = run_query(&graph, "MATCH (n:computation@out)--[:wire]->(m:slider) RETURN n.name, m.name");
        assert!(result.is_ok());
    }

    // #region 🔖️Fixtures
    /// 🧵️ Small hand-built graph exercising every `split_endpoint` branch: exact handle match,
    /// mapped/unmapped `@` and `:` splits, and the plain-id fallback.
    async fn split_endpoint_fixture() -> &'static str {
        r#"{
  "manifestId": "flow-dag",
  "nodes": [
    { "id": "a", "nodeKind": "computation", "text": "A", "userData": { "score": 1 }, "handles": [{ "id": "a-out" }] },
    { "id": "b", "nodeKind": "slider", "text": "B" },
    { "id": "c", "nodeKind": "slider", "text": "C" }
  ],
  "edges": [
    { "id": "e1", "edgeKind": "wire", "source": "a-out", "target": "b@in" },
    { "id": "e2", "edgeKind": "wire", "source": "a:out2", "target": "c.in2" },
    { "id": "e3", "edgeKind": "wire", "source": "a-out@x", "target": "a-out:y" },
    { "id": "e4", "edgeKind": "wire", "source": "z", "target": "c" }
  ]
}"#
    }

    async fn find_edge<'a>(edges: &'a [QueryableEdge], id: &str) -> &'a QueryableEdge {
        edges.iter().find(|e| e.id == id).unwrap_or_else(|| panic!("missing edge {id}"))
    }
    // #endregion 🔖️Fixtures

    // #region 🔖️QueryableGraphTests
    #[test]
    async fn split_endpoint_resolves_exact_handle_and_unmapped_at() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let edges = graph.edges();
        let e1 = find_edge(&edges, "e1");
        assert_eq!(e1.source_node_id, "a");
        assert_eq!(e1.source_port, None);
        assert_eq!(e1.target_node_id, "b");
        assert_eq!(e1.target_port.as_deref(), Some("in"));
    }

    #[test]
    async fn split_endpoint_resolves_unmapped_colon_and_dot() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let edges = graph.edges();
        let e2 = find_edge(&edges, "e2");
        assert_eq!(e2.source_node_id, "a");
        assert_eq!(e2.source_port.as_deref(), Some("out2"));
        assert_eq!(e2.target_node_id, "c");
        assert_eq!(e2.target_port.as_deref(), Some("in2"));
    }

    #[test]
    async fn split_endpoint_resolves_handle_mapped_at_and_colon() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let edges = graph.edges();
        let e3 = find_edge(&edges, "e3");
        assert_eq!(e3.source_node_id, "a");
        assert_eq!(e3.source_port.as_deref(), Some("x"));
        assert_eq!(e3.target_node_id, "a");
        assert_eq!(e3.target_port.as_deref(), Some("y"));
    }

    #[test]
    async fn split_endpoint_falls_back_to_plain_id() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let edges = graph.edges();
        let e4 = find_edge(&edges, "e4");
        assert_eq!(e4.source_node_id, "z");
        assert_eq!(e4.source_port, None);
        assert_eq!(e4.target_node_id, "c");
        assert_eq!(e4.target_port, None);
    }

    #[test]
    async fn board_graph_node_property_id_kind_all_and_missing() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        assert_eq!(graph.node_property("a", "id"), Some(PropertyValue::String("a".into())));
        assert_eq!(graph.node_property("a", "kind"), Some(PropertyValue::String("computation".into())));
        let all = graph.node_property("a", "__all").unwrap();
        assert!(matches!(all, PropertyValue::Object(ref map) if map.get("score") == Some(&PropertyValue::Number(1.0))));
        assert_eq!(graph.node_property("a", "score"), Some(PropertyValue::Number(1.0)));
        assert_eq!(graph.node_property("a", "nonexistent"), None);
        assert_eq!(graph.node_property("missing-node", "id"), None);
    }

    #[test]
    async fn manifest_helpers_merge_graph_and_manifest_kinds() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        assert_eq!(graph.manifest().map(|m| m.id.as_str()), Some("flow-dag"));
        let node_kinds = manifest_node_kinds(&graph);
        assert!(node_kinds.iter().any(|k| k == "computation"));
        assert!(node_kinds.iter().any(|k| k == "select"), "manifest-only kind should be included");
        let edge_kinds = manifest_edge_kinds(&graph);
        assert!(edge_kinds.iter().any(|k| k == "wire"));
        let port_kinds = manifest_port_kinds(&graph);
        assert!(port_kinds.iter().any(|k| k == "in"));
        let props = manifest_property_names(&graph);
        for expected in ["id", "name", "kind", "label", "text", "score"] {
            assert!(props.iter().any(|p| p == expected), "missing property {expected}");
        }
    }

    #[test]
    async fn subgraph_fixture_json_filters_to_requested_ids() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let node_ids = BTreeSet::from(["a".to_string(), "b".to_string()]);
        let edge_ids = BTreeSet::from(["e1".to_string()]);
        let json = graph.subgraph_fixture_json(&node_ids, &edge_ids).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["nodes"].as_array().unwrap().len(), 2);
        assert_eq!(value["edges"].as_array().unwrap().len(), 1);
    }

    #[test]
    async fn from_fixture_json_rejects_invalid_json() {
        let Err(err) = BoardQueryableGraph::from_fixture_json("not json", None) else { panic!("expected error") };
        assert!(matches!(err, GraphDslError::Json(_)));
    }

    #[test]
    async fn from_puzzle3d_fixture_json_converts_objects_array() {
        let fixture = r#"{"objects": [{"id": "o1", "objectKind": "Cube", "name": "Box"}]}"#;
        let graph = BoardQueryableGraph::from_puzzle3d_fixture_json(fixture).unwrap();
        assert_eq!(graph.node_kind("o1").as_deref(), Some("Cube"));
        assert_eq!(graph.node_name("o1").as_deref(), Some("Box"));
        assert_eq!(graph.manifest().map(|m| m.id.as_str()), Some("puzzle3d-default"));
    }

    #[test]
    async fn from_puzzle3d_fixture_json_passes_through_existing_nodes() {
        let fixture = r#"{"nodes": [{"id": "n1", "nodeKind": "Widget", "text": "N1"}]}"#;
        let graph = BoardQueryableGraph::from_puzzle3d_fixture_json(fixture).unwrap();
        assert_eq!(graph.node_kind("n1").as_deref(), Some("Widget"));
    }

    #[test]
    async fn from_puzzle2d_and_puzzle5d_fixture_json_resolve_manifests() {
        let fixture = r#"{"nodes": [], "edges": []}"#;
        let g2 = BoardQueryableGraph::from_puzzle2d_fixture_json(fixture).unwrap();
        assert_eq!(g2.manifest().map(|m| m.id.as_str()), Some("puzzle2d-default"));
        let g5 = BoardQueryableGraph::from_puzzle5d_fixture_json(fixture).unwrap();
        assert_eq!(g5.manifest().map(|m| m.id.as_str()), Some("puzzle5d-default"));
    }
    // #endregion 🔖️QueryableGraphTests

    // #region 🔖️ErrorTests
    #[test]
    async fn graph_dsl_error_display_messages() {
        assert_eq!(GraphDslError::UnterminatedString.to_string(), "unterminated string literal");
        assert_eq!(GraphDslError::UnexpectedChar('$').to_string(), "unexpected character '$'");
        assert_eq!(GraphDslError::EdgeTargetMissingPort.to_string(), "edge target requires @port");
        assert_eq!(GraphDslError::EmptyPattern.to_string(), "empty pattern");
        assert_eq!(GraphDslError::UnsupportedMutation.to_string(), "mutating jack clauses are not supported on this graph domain");
        assert_eq!(GraphDslError::UnsupportedClause.to_string(), "WITH/UNWIND/CALL clauses are not yet executable");
        let unexpected = GraphDslError::UnexpectedToken { expected: "ident".into(), found: "Eof".into() };
        assert_eq!(unexpected.to_string(), "expected ident, got Eof");
    }

    #[test]
    async fn parse_error_on_unexpected_char() {
        // `{`/`}` are valid tokens in `dsl_core`'s shared alphabet (map/object-literal braces)
        // but aren't part of Jack's own grammar (no map literals) — Jack rejects them itself,
        // hence `UnexpectedChar` rather than a `dsl_core`-surfaced `Lex` error.
        let err = parse("MATCH (a:x) { WHERE").unwrap_err();
        assert!(matches!(err, GraphDslError::UnexpectedChar('{')));
    }

    #[test]
    async fn parse_error_on_char_outside_dsl_core_alphabet_reports_lex_error() {
        // `?` isn't lexable by `dsl_core` at all (unlike `{`/`}` above, which lex fine but aren't
        // valid Jack syntax) — `os_dsl::lex` itself fails, surfaced verbatim as `Lex`.
        let err = parse("MATCH (a:x) ? WHERE").unwrap_err();
        assert!(matches!(err, GraphDslError::Lex(_)));
        assert!(err.to_string().contains("unexpected character '?'"), "got: {err}");
    }

    #[test]
    async fn parse_error_on_lone_bang_reports_lex_error() {
        // A stray `!` not followed by `=` isn't a token in Jack's grammar at all (`dsl_core` has
        // no relational operators, and Jack only special-cases `!=`).
        let err = parse("MATCH (a:x) WHERE a.p ! 1").unwrap_err();
        assert!(matches!(err, GraphDslError::Lex(_)));
    }

    #[test]
    async fn parse_error_on_unterminated_string() {
        let err = parse("MATCH (a:x) WHERE a.name = 'oops").unwrap_err();
        assert!(matches!(err, GraphDslError::UnterminatedString));
    }
    // #endregion 🔖️ErrorTests

    // #region 🔖️LexerAndLanguageServiceTests
    #[test]
    async fn tokenize_classifies_clause_and_operator_tokens() {
        let spans = tokenize("MATCH (a:x)--[:wire]->(b:y) WHERE a.p = 1 AND b.q != 'v' RETURN a.p");
        assert!(spans.iter().any(|s| s.class == TokenClass::Keyword));
        assert!(spans.iter().any(|s| s.class == TokenClass::Ident));
        assert!(spans.iter().any(|s| s.class == TokenClass::Number));
        assert!(spans.iter().any(|s| s.class == TokenClass::String));
        assert!(spans.iter().any(|s| s.class == TokenClass::Operator));
        assert!(spans.iter().any(|s| s.class == TokenClass::Punctuation));
    }

    #[test]
    async fn tokenize_marks_unterminated_string_as_error_class() {
        let spans = tokenize("MATCH (a:x) WHERE a.p = 'unterminated");
        assert!(spans.iter().any(|s| s.class == TokenClass::Error));
    }

    #[test]
    async fn tokenize_never_panics_on_stray_symbols() {
        // 🩹️ `#` is now a legitimate comment starter (unified with the rest of the DSL engine, so
        // it swallows the remainder of the line) — the stray-symbol probes moved off it.
        let spans = tokenize("MATCH (a:x) ~ ^ RETURN a");
        assert!(spans.iter().any(|s| s.class == TokenClass::Ident && s.end - s.start == 1));
    }

    #[test]
    async fn tokenize_treats_hash_as_a_comment_to_end_of_line() {
        let source = "MATCH (a:x) # a trailing comment\nRETURN a";
        let comment_start = source.find('#').unwrap();
        let line_end = source.find('\n').unwrap();
        let spans = tokenize(source);
        assert!(!spans.iter().any(|s| s.start >= comment_start && s.start < line_end), "no token should start inside the comment body: {spans:?}");
        assert!(spans.iter().any(|s| s.class == TokenClass::Keyword));
    }

    #[test]
    async fn format_query_is_idempotent_and_normalizes_whitespace() {
        let once = format("match(a:x)--[:wire]->(b:y) where a.p=1 and b.q!='v' return a.p,b.q").unwrap();
        assert!(once.contains("MATCH"));
        assert!(once.contains(" AND "));
        assert!(once.contains(" = "));
        let twice = format(&once).unwrap();
        assert_eq!(once, twice);
    }

    #[test]
    async fn format_rejects_unterminated_string() {
        let err = format("MATCH (a:x) WHERE a.p = 'oops").unwrap_err();
        assert!(matches!(err, GraphDslError::UnterminatedString));
    }

    #[test]
    async fn complete_after_colon_suggests_node_then_edge_kinds() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let node_source = "MATCH (a:c";
        let node_completions = complete(&graph, node_source, node_source.len());
        assert!(node_completions.iter().any(|c| c.label == "computation"));
        let edge_source = "MATCH (a:computation)--[:w";
        let edge_completions = complete(&graph, edge_source, edge_source.len());
        assert!(edge_completions.iter().any(|c| c.label == "wire"));
    }

    #[test]
    async fn complete_after_at_suggests_port_kinds() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let source = "MATCH (a:computation@i";
        let completions = complete(&graph, source, source.len());
        assert!(completions.iter().any(|c| c.label == "in"));
    }

    #[test]
    async fn complete_after_dot_suggests_property_names() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let source = "MATCH (a:computation) RETURN a.sc";
        let completions = complete(&graph, source, source.len());
        assert!(completions.iter().any(|c| c.label == "score"));
    }

    #[test]
    async fn complete_suggests_bound_variable_when_prefix_does_not_match_logic_keywords() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let source = "MATCH (abc:computation) WHERE ab";
        let completions = complete(&graph, source, source.len());
        assert!(completions.iter().any(|c| c.label == "abc" && c.kind == "variable"));
    }

    #[test]
    async fn complete_in_where_clause_suggests_logic_keywords() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let source = "MATCH (a:computation) WHERE a.score = 1 AN";
        let completions = complete(&graph, source, source.len());
        assert!(completions.iter().any(|c| c.label == "AND"));
    }

    #[test]
    async fn complete_at_start_suggests_clause_keywords() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let completions = complete(&graph, "MA", 2);
        assert!(completions.iter().any(|c| c.label == "MATCH"));
    }

    #[test]
    async fn hover_reports_keyword_and_bound_variable() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let source = "MATCH (a:computation) WHERE a.score = 1 RETURN a";
        let match_pos = source.find("MATCH").unwrap();
        assert!(hover(&graph, source, match_pos + 1).unwrap().contents.contains("keyword"));
        // 🩹️ `hover_word_at` folds `:`/`.` into the word span, so a bound variable only resolves
        // in isolation when nothing follows it — the trailing standalone `a` in `RETURN a`.
        let var_pos = source.rfind('a').unwrap();
        assert!(hover(&graph, source, var_pos + 1).unwrap().contents.contains("Bound variable"));
    }

    #[test]
    async fn hover_matches_bare_node_kind_edge_kind_and_property_words() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let source = "computation wire score";
        assert!(hover(&graph, source, 3).unwrap().contents.contains("Node kind"));
        let edge_pos = source.find("wire").unwrap();
        assert!(hover(&graph, source, edge_pos + 1).unwrap().contents.contains("Edge kind"));
        let prop_pos = source.find("score").unwrap();
        assert!(hover(&graph, source, prop_pos + 1).unwrap().contents.contains("Property"));
    }

    #[test]
    async fn hover_returns_none_for_whitespace() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        assert!(hover(&graph, "MATCH (a:x)   RETURN a", 12).is_none());
    }

    #[test]
    async fn lint_flags_unknown_node_kind() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let diags = lint(&graph, "MATCH (a:nonexistentKind) RETURN a");
        assert!(diags.iter().any(|d| d.code.as_deref() == Some("jack/unknown-node-kind")));
    }

    #[test]
    async fn lint_flags_unbound_variable() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let diags = lint(&graph, "MATCH (a:computation) RETURN b");
        assert!(diags.iter().any(|d| d.code.as_deref() == Some("jack/unbound-variable")));
    }

    #[test]
    async fn lint_reports_parse_errors() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let diags = lint(&graph, "MATCH (a:computation");
        assert!(diags.iter().any(|d| d.code.as_deref() == Some("jack/parse-error")));
    }

    #[test]
    async fn lint_clean_query_has_no_diagnostics() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let diags = lint(&graph, "MATCH (a:computation) RETURN a.name");
        assert!(diags.is_empty());
    }

    #[test]
    async fn semantic_tokens_mirror_tokenize_classes() {
        let tokens = semantic_tokens("MATCH (a:x) RETURN a");
        assert!(tokens.iter().any(|t| t.class == "keyword"));
        assert!(tokens.iter().any(|t| t.class == "ident"));
    }
    // #endregion 🔖️LexerAndLanguageServiceTests

    // #region 🔖️ParserAndExecutorTests
    #[test]
    async fn parse_delete_set_merge_clauses() {
        let q = parse("MATCH (a:x) DELETE a").unwrap();
        assert!(matches!(q.clauses[1], Clause::Delete(ref vars) if vars == &vec!["a".to_string()]));
        let q = parse("MATCH (a:x) SET a.name = 'v'").unwrap();
        assert!(matches!(q.clauses[1], Clause::Set(ref items) if items.len() == 1 && items[0].prop == "name"));
        let q = parse("MERGE (a:x)").unwrap();
        assert!(matches!(q.clauses[0], Clause::Merge(_)));
    }

    #[test]
    async fn parse_where_and_or_precedence() {
        let q = parse("MATCH (a:x) WHERE a.p = 1 AND a.q = 2 OR a.r != 3").unwrap();
        let Clause::Where(expr) = &q.clauses[1] else { panic!("expected where") };
        assert!(matches!(expr, Expr::Or(_, _)));
    }

    // #region 🔖️WithUnwindCallTests
    // 🚧️ prep work for unifying semio_compose_rs's Architect query language onto Jack — these clauses
    // parse into the AST (this region) but aren't wired into `execute()` yet, see
    // `GraphDslError::UnsupportedClause`.
    #[test]
    async fn parse_with_clause() {
        let q = parse("MATCH (a:x) WITH a, a.name RETURN a").unwrap();
        let Clause::With(items) = &q.clauses[1] else { panic!("expected with") };
        assert_eq!(items.len(), 2);
        assert!(matches!(&items[0], ReturnItem::Var(v) if v == "a"));
        assert!(matches!(&items[1], ReturnItem::Property { var, prop } if var == "a" && prop == "name"));
    }

    #[test]
    async fn parse_unwind_clause() {
        let q = parse("MATCH (a:x) UNWIND a.items AS item RETURN item").unwrap();
        let Clause::Unwind(clause) = &q.clauses[1] else { panic!("expected unwind") };
        assert!(matches!(&clause.source, ReturnItem::Property { var, prop } if var == "a" && prop == "items"));
        assert_eq!(clause.var, "item");
    }

    #[test]
    async fn parse_call_clause_with_positional_args() {
        let q = parse("CALL myProc(1, \"two\", true)").unwrap();
        let Clause::Call(clause) = &q.clauses[0] else { panic!("expected call") };
        assert_eq!(clause.name, "myProc");
        assert_eq!(clause.args, vec![PropertyValue::Number(1.0), PropertyValue::String("two".to_string()), PropertyValue::Bool(true)]);
    }

    #[test]
    async fn execute_rejects_with_unwind_call_pending_wiring() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        for query in ["MATCH (a:x) WITH a RETURN a", "MATCH (a:x) UNWIND a.items AS i RETURN i", "CALL proc()"] {
            let err = run_query(&graph, query).unwrap_err();
            assert!(matches!(err, GraphDslError::UnsupportedClause), "query {query} should report UnsupportedClause, got {err:?}");
        }
    }
    // #endregion 🔖️WithUnwindCallTests

    #[test]
    async fn lexer_accepts_both_single_and_double_quoted_strings_and_always_prints_double_quoted() {
        let single = parse("MATCH (a:x) WHERE a.name = 'alpha' RETURN a").unwrap();
        let double = parse("MATCH (a:x) WHERE a.name = \"alpha\" RETURN a").unwrap();
        assert_eq!(single, double, "single- and double-quoted string literals must parse identically");
        let printed = format("MATCH (a:x) WHERE a.name = 'alpha' RETURN a").unwrap();
        assert!(printed.contains("\"alpha\""), "must always print double-quoted: {printed}");
        assert!(!printed.contains('\''), "must never print single-quoted: {printed}");
    }

    #[test]
    async fn parse_unexpected_token_error_has_expected_and_found() {
        let err = parse("MATCH a:x)").unwrap_err();
        let GraphDslError::UnexpectedToken { expected, found } = err else { panic!("expected UnexpectedToken") };
        assert_eq!(expected, "LParen");
        assert!(found.contains("Ident"));
    }

    #[test]
    async fn execute_where_clause_filters_bindings() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:slider) WHERE a.name = 'B' RETURN a.name").unwrap();
        assert_eq!(result.rows, vec![vec![PropertyValue::String("B".into())]]);
    }

    #[test]
    async fn execute_and_or_expressions() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let and_result = run_query(&graph, "MATCH (a:slider) WHERE a.name = 'B' AND a.kind = 'slider' RETURN a.name").unwrap();
        assert_eq!(and_result.rows.len(), 1);
        let or_result = run_query(&graph, "MATCH (a:slider) WHERE a.name = 'B' OR a.name = 'C' RETURN a.name").unwrap();
        assert_eq!(or_result.rows.len(), 2);
    }

    #[test]
    async fn execute_rejects_mutating_clauses() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        for query in ["CREATE (a:x)", "MATCH (a:x) DELETE a", "MATCH (a:x) SET a.p = 1", "MERGE (a:x)"] {
            let err = run_query(&graph, query).unwrap_err();
            assert!(matches!(err, GraphDslError::UnsupportedMutation), "query {query} should reject mutation");
        }
    }

    #[test]
    async fn execute_undirected_edge_matches_both_directions() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let forward = run_query(&graph, "MATCH (a:computation)--[:wire]--(b:slider) RETURN a.name, b.name").unwrap();
        let reverse = run_query(&graph, "MATCH (b:slider)--[:wire]--(a:computation) RETURN a.name, b.name").unwrap();
        assert!(!forward.rows.is_empty());
        assert!(!reverse.rows.is_empty());
    }

    #[test]
    async fn execute_multiple_match_patterns_join_bindings() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation), (b:slider) RETURN a.name, b.name").unwrap();
        assert_eq!(result.rows.len(), 2);
    }

    #[test]
    async fn execute_returns_graph_kind_when_returning_bound_entities() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation)--[e:wire]--(b:slider) RETURN a, e, b").unwrap();
        assert_eq!(result.kind, QueryResultKind::Graph);
        assert!(result.graph_fixture_json.is_some());
    }

    #[test]
    async fn execute_returns_table_kind_for_property_projection() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation) RETURN a.name").unwrap();
        assert_eq!(result.kind, QueryResultKind::Table);
    }

    #[test]
    async fn execute_with_no_return_clause_yields_empty_table() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let result = run_query(&graph, "MATCH (a:computation)").unwrap();
        assert!(result.columns.is_empty());
        assert!(result.rows.is_empty());
    }

    #[test]
    async fn run_query_json_serializes_result() {
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let json = run_query_json(&graph, "MATCH (a:computation) RETURN a.name").unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["columns"][0], "a.name");
    }

    #[test]
    async fn empty_pattern_error_is_reachable_via_pattern_construction() {
        let pattern = Pattern { nodes: vec![], edge: None };
        let graph = BoardQueryableGraph::from_fixture_json(split_endpoint_fixture(), None).unwrap();
        let err = match_patterns(&graph, std::slice::from_ref(&pattern)).unwrap_err();
        assert!(matches!(err, GraphDslError::EmptyPattern));
    }
    // #endregion 🔖️ParserAndExecutorTests
}
// #endregion 🔖️Tests
// #endregion jack_impl
