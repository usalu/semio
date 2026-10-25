//! 🔱️ Jack language engine: the shared graph query language service over the graph of a loaded Jack snapshot.

use crate::server::{graphless_suppressed_code_prefix, LanguageEngine};
use semio_framework_graph_dsl::{BoardQueryableGraph, Completion, Diagnostic, Hover, SemanticToken};
use semio_framework_graph_manifest::{GraphManifest, KindDef};
use semio_framework_pack_json::{JsonMemberPolicy, Value};

/// 🔱️ Without a loaded snapshot the engine checks syntax and variables; kind checks need the manifest of a snapshot.
pub struct JackEngine {
    graph: BoardQueryableGraph,
    loaded: bool,
}

impl Default for JackEngine {
    fn default() -> Self {
        Self { graph: BoardQueryableGraph::from_host_snapshot_json("{}", None).expect("an empty snapshot is a valid graph"), loaded: false }
    }
}

/// 🏷️ The kind rows of one manifest list (`nodeKinds`, `edgeKinds`, `portKinds`); a row is named by `name`, or by `id` in the shared graph manifest shape.
fn kinds(manifest: &Value, list: &str) -> Result<Vec<KindDef>, String> {
    let Some(rows) = manifest.get(list) else { return Ok(Vec::new()) };
    let rows = rows.as_array().ok_or_else(|| format!("invalid snapshot manifest: {list} must be an array"))?;
    rows.iter()
        .map(|row| -> Result<KindDef, String> {
            let name = row.get("name").or_else(|| row.get("id")).and_then(Value::as_str).ok_or_else(|| format!("invalid snapshot manifest: a row of {list} needs a name"))?;
            Ok(KindDef { id: name.to_string(), name: name.to_string(), properties: Vec::new(), ports: Vec::new(), direction: None, presentation: None })
        })
        .collect()
}

/// 📜️ The kinds a Jack snapshot declares, as the graph manifest the shared language service checks queries against.
fn graph_manifest(raw: &Value) -> Result<Option<GraphManifest>, String> {
    let Some(manifest) = raw.get("manifest").filter(|row| !row.is_null()) else { return Ok(None) };
    if manifest.as_object().is_none() {
        return Err("invalid snapshot manifest: an object is required".to_string());
    }
    let id = manifest.get("id").or_else(|| raw.get("manifestId")).and_then(Value::as_str).unwrap_or("snapshot").to_string();
    let schema = manifest.get("schema").and_then(Value::as_str).unwrap_or("s.trinity.manifest").to_string();
    let name = manifest.get("name").and_then(Value::as_str).unwrap_or(&id).to_string();
    Ok(Some(GraphManifest {
        schema,
        id,
        name,
        axes: Default::default(),
        node_kinds: kinds(manifest, "nodeKinds")?,
        edge_kinds: kinds(manifest, "edgeKinds")?,
        port_kinds: kinds(manifest, "portKinds")?,
        wire_kinds: Vec::new(),
        layer_kinds: Vec::new(),
        language_kinds: Vec::new(),
        surface_kinds: Vec::new(),
        window_kinds: Vec::new(),
        file_node_kinds: Vec::new(),
        descriptor_kinds: Vec::new(),
        edge_tips: Vec::new(),
        kind_compatibility: Vec::new(),
    }))
}

impl JackEngine {
    /// 📸️ Replaces the graph with the manifest kinds, nodes and edges of one Jack snapshot document.
    pub fn load_snapshot(&mut self, json: &str) -> Result<(), String> {
        let raw = semio_framework_pack_json::parse(json, JsonMemberPolicy::Reject).map_err(|error| format!("invalid snapshot json: {error}"))?;
        let manifest = graph_manifest(&raw)?;
        self.graph = BoardQueryableGraph::from_host_snapshot_json(json, manifest).map_err(|error| error.to_string())?;
        self.loaded = true;
        Ok(())
    }

    pub fn has_snapshot(&self) -> bool {
        self.loaded
    }
}

impl LanguageEngine for JackEngine {
    fn diagnostics(&self, source: &str) -> Vec<Diagnostic> {
        let mut rows = semio_framework_graph_dsl::lint(&self.graph, source);
        if !self.loaded {
            let prefix = graphless_suppressed_code_prefix();
            rows.retain(|row| !row.code.as_deref().is_some_and(|code| code.starts_with(prefix)));
        }
        rows
    }

    fn complete(&self, source: &str, offset: usize) -> Vec<Completion> {
        semio_framework_graph_dsl::complete(&self.graph, source, offset)
    }

    fn hover(&self, source: &str, offset: usize) -> Option<Hover> {
        semio_framework_graph_dsl::hover(&self.graph, source, offset)
    }

    fn semantic_tokens(&self, source: &str) -> Vec<SemanticToken> {
        semio_framework_graph_dsl::semantic_tokens(source)
    }

    fn format(&self, source: &str) -> Result<String, String> {
        semio_framework_graph_dsl::format(source).map_err(|error| error.to_string())
    }

    fn extension(&mut self, method: &str, params: &Value) -> Option<Result<Value, String>> {
        if method != "jack/loadSnapshot" {
            return None;
        }
        let Some(json) = params.get("json").and_then(Value::as_str) else {
            return Some(Err("params.json, a Jack snapshot document, is required".to_string()));
        };
        Some(self.load_snapshot(json).map(|()| Value::Null))
    }
}
