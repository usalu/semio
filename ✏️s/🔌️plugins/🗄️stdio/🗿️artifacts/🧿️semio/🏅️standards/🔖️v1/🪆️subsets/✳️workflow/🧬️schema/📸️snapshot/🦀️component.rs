//! 🧬️ SemioWorkflowSnapshot — id-keyed `nodes{kind,label,params,position:SemioPoint2}` +
//! `PortRef`-addressed `edges{from,to,kind}`, informed by OS `🔁️workflow` WorkflowNode +
//! `🌊️flow/🕸️dag` (see master-plan.md's "Subset snapshot cores" table). Complete per spec: no
//! `serde_json::Value`, no bare tuples (`PortRef`/`SemioPoint2` are named structs), no nested
//! fixed arrays.

use crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::engine::triples::{split_top_level, strip_brackets};

//#region 🔖️PortRef
/// 🔌️ Addresses one named port on one node — the endpoint shape `WorkflowEdge` connects through.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortRef {
    pub node: String,
    pub port: String,
}
//#endregion 🔖️PortRef

//#region 🔖️Param
/// 🎛️ One ordered key-value node parameter. String-valued is the honest boundary for a workflow
/// DAG's per-node config — a richer typed value graph is `object` subset's job (`SemioValue`), not
/// workflow's; see w1b-type-ownership.md's per-subset owned-types table.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowParam {
    pub key: String,
    pub value: String,
}
//#endregion 🔖️Param

//#region 🔖️Node
/// 🔁️ Owned by the `workflow` subset. DISTINCT from the OS kernel's own
/// `semio_framework::WorkflowNode` (a different crate, `semio-framework`, not
/// `semio-s-plugin-stdio`) — same name, zero collision risk, do not conflate the two (see
/// w1b-type-ownership.md).
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowNode {
    pub id: String,
    pub kind: String,
    pub label: String,
    #[serde(default)]
    pub params: Vec<WorkflowParam>,
    pub position: SemioPoint2,
}
//#endregion 🔖️Node

//#region 🔖️Edge
/// ➡️ Owned by the `workflow` subset. `id`-keyed (like `nodes`) so the sparse diff can address one
/// edge by identity rather than by its `(from,to,kind)` value, which is not guaranteed unique in a
/// real multigraph DAG.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowEdge {
    pub id: String,
    pub from: PortRef,
    pub to: PortRef,
    pub kind: String,
}
//#endregion 🔖️Edge

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_SEMIOWORKFLOW_DOCUMENT_SCHEMA: &str = "stdio.semio.workflow";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.workflow")]
pub struct SemioWorkflowSnapshot {
    #[state(persistent)]
    pub schema: String,
    /// 🆔️ Id-keyed strong collection — sparse-diffed via `🧰️triples::NamedTripleDiff`.
    #[state(persistent)]
    #[serde(default)]
    pub nodes: Vec<WorkflowNode>,
    /// 🆔️ Id-keyed strong collection — sparse-diffed via `🧰️triples::NamedTripleDiff`.
    #[state(persistent)]
    #[serde(default)]
    pub edges: Vec<WorkflowEdge>,
}

impl Default for SemioWorkflowSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_SEMIOWORKFLOW_DOCUMENT_SCHEMA.into(),
            nodes: Default::default(),
            edges: Default::default(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextPrimitives
/// 🧪️ P2 pilot (workflow, the FIRST semio subset upgraded): real hex/bracket-encoded value
/// primitives backing the hand-rolled `ArtifactDsl` below — same style as this subset's own
/// `🔺️diff`/`🧬️mutations` facets (`GifDiff`/`SvgDiff`/`DocxDiff`'s established hand-rolled
/// convention), duplicated here (not imported from `schema::diff`) to keep `snapshot` — the base
/// type `diff`/`mutations` both depend ON — free of a reverse dependency on either sibling facet.
///
/// 🧩️ The `#[derive(dsl::DslArtifact)]` path was tried first per this ticket's brief and hits a
/// real mechanism gap: `position: SemioPoint2` would need `SemioPoint2` (`engine::geometry`,
/// OUTSIDE this ticket's `✳️workflow/`-only edit scope) to implement `dsl::DslField`/`DslRecord`,
/// which it does not. Hand-rolled instead — see this wave's report `mechanism_gaps`.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
fn enc_f64(v: f64) -> String {
    format!("{v}")
}
fn dec_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
fn enc_point2(p: &SemioPoint2) -> String {
    format!("[{},{}]", enc_f64(p.x), enc_f64(p.y))
}
fn dec_point2(s: &str) -> Result<SemioPoint2, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [x, y] = parts.as_slice() else { return Err(format!("point2: expected 2 fields, got {}", parts.len())) };
    Ok(SemioPoint2 { x: dec_f64(x)?, y: dec_f64(y)? })
}
fn enc_port_ref(p: &PortRef) -> String {
    format!("[{},{}]", enc_str(&p.node), enc_str(&p.port))
}
fn dec_port_ref(s: &str) -> Result<PortRef, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [node, port] = parts.as_slice() else { return Err(format!("port ref: expected 2 fields, got {}", parts.len())) };
    Ok(PortRef { node: dec_str(node)?, port: dec_str(port)? })
}
fn enc_param(p: &WorkflowParam) -> String {
    format!("[{},{}]", enc_str(&p.key), enc_str(&p.value))
}
fn dec_param(s: &str) -> Result<WorkflowParam, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [key, value] = parts.as_slice() else { return Err(format!("param: expected 2 fields, got {}", parts.len())) };
    Ok(WorkflowParam { key: dec_str(key)?, value: dec_str(value)? })
}
fn enc_node(n: &WorkflowNode) -> String {
    format!(
        "[{},{},{},{},{}]",
        enc_str(&n.id),
        enc_str(&n.kind),
        enc_str(&n.label),
        format!("[{}]", n.params.iter().map(enc_param).collect::<Vec<_>>().join(",")),
        enc_point2(&n.position)
    )
}
fn dec_node(s: &str) -> Result<WorkflowNode, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [id, kind, label, params, position] = parts.as_slice() else { return Err(format!("node: expected 5 fields, got {}", parts.len())) };
    let params = split_top_level(strip_brackets(params)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_param).collect::<Result<Vec<_>, String>>()?;
    Ok(WorkflowNode { id: dec_str(id)?, kind: dec_str(kind)?, label: dec_str(label)?, params, position: dec_point2(position)? })
}
fn enc_edge(e: &WorkflowEdge) -> String {
    format!("[{},{},{},{}]", enc_str(&e.id), enc_port_ref(&e.from), enc_port_ref(&e.to), enc_str(&e.kind))
}
fn dec_edge(s: &str) -> Result<WorkflowEdge, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [id, from, to, kind] = parts.as_slice() else { return Err(format!("edge: expected 4 fields, got {}", parts.len())) };
    Ok(WorkflowEdge { id: dec_str(id)?, from: dec_port_ref(from)?, to: dec_port_ref(to)?, kind: dec_str(kind)? })
}

/// 📄️ The real structured text body: three lines — `schema=<hex>`, `nodes=[<node>,...]`,
/// `edges=[<edge>,...]` — matching the grammar's `document = artifact-mark schema-line nodes-line
/// edges-line`. Newlines are pure lexer trivia in the shared dialect, so this is genuinely
/// recognizable by `dsl::Recognizer`, not merely readable.
fn print_workflow_snapshot_body(s: &SemioWorkflowSnapshot) -> String {
    format!(
        "schema={}\nnodes=[{}]\nedges=[{}]",
        enc_str(&s.schema),
        s.nodes.iter().map(enc_node).collect::<Vec<_>>().join(","),
        s.edges.iter().map(enc_edge).collect::<Vec<_>>().join(",")
    )
}
fn parse_workflow_snapshot_body(body: &str) -> Result<SemioWorkflowSnapshot, String> {
    let mut schema = None;
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("nodes=") {
            let inner = strip_brackets(rest)?;
            nodes = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_node).collect::<Result<Vec<_>, String>>()?;
        } else if let Some(rest) = line.strip_prefix("edges=") {
            let inner = strip_brackets(rest)?;
            edges = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_edge).collect::<Result<Vec<_>, String>>()?;
        } else {
            return Err(format!("workflow snapshot: unknown line {line:?}"));
        }
    }
    let schema = schema.ok_or_else(|| "workflow snapshot: missing schema line".to_string())?;
    Ok(SemioWorkflowSnapshot { schema, nodes, edges })
}
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers `stdio.json`'s upgraded `OpBinary`/`DiffCodec` reuse) backing
/// the real `ArtifactPack` below — replaces the old `serde_json::to_vec`-in-envelope shortcut.
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

fn encode_workflow_snapshot_binary(s: &SemioWorkflowSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.nodes.len() as u64);
    for n in &s.nodes {
        write_str_lp(&mut out, &n.id);
        write_str_lp(&mut out, &n.kind);
        write_str_lp(&mut out, &n.label);
        store::pack_rt::write_varint_u64(&mut out, n.params.len() as u64);
        for p in &n.params {
            write_str_lp(&mut out, &p.key);
            write_str_lp(&mut out, &p.value);
        }
        out.extend_from_slice(&n.position.x.to_le_bytes());
        out.extend_from_slice(&n.position.y.to_le_bytes());
    }
    store::pack_rt::write_varint_u64(&mut out, s.edges.len() as u64);
    for e in &s.edges {
        write_str_lp(&mut out, &e.id);
        write_str_lp(&mut out, &e.from.node);
        write_str_lp(&mut out, &e.from.port);
        write_str_lp(&mut out, &e.to.node);
        write_str_lp(&mut out, &e.to.port);
        write_str_lp(&mut out, &e.kind);
    }
    out
}
fn decode_workflow_snapshot_binary(bytes: &[u8]) -> Result<SemioWorkflowSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let node_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut nodes = Vec::with_capacity(node_count as usize);
    for _ in 0..node_count {
        let id = read_str_lp(&mut reader)?;
        let kind = read_str_lp(&mut reader)?;
        let label = read_str_lp(&mut reader)?;
        let param_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
        let mut params = Vec::with_capacity(param_count as usize);
        for _ in 0..param_count {
            let key = read_str_lp(&mut reader)?;
            let value = read_str_lp(&mut reader)?;
            params.push(WorkflowParam { key, value });
        }
        let x = reader.read_f64_le().map_err(|e| e.to_string())?;
        let y = reader.read_f64_le().map_err(|e| e.to_string())?;
        nodes.push(WorkflowNode { id, kind, label, params, position: SemioPoint2 { x, y } });
    }
    let edge_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut edges = Vec::with_capacity(edge_count as usize);
    for _ in 0..edge_count {
        let id = read_str_lp(&mut reader)?;
        let from_node = read_str_lp(&mut reader)?;
        let from_port = read_str_lp(&mut reader)?;
        let to_node = read_str_lp(&mut reader)?;
        let to_port = read_str_lp(&mut reader)?;
        let kind = read_str_lp(&mut reader)?;
        edges.push(WorkflowEdge { id, from: PortRef { node: from_node, port: from_port }, to: PortRef { node: to_node, port: to_port }, kind });
    }
    Ok(SemioWorkflowSnapshot { schema, nodes, edges })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁 Real structured text/binary codecs (P2 pilot — first semio subset upgraded off the old
/// hex-dump-of-`serde_json` shortcut). Wrapped in the repo-wide `store::semio_format` envelope,
/// unchanged.
impl store::ArtifactDsl for SemioWorkflowSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str { STDIO_SEMIOWORKFLOW_DOCUMENT_SCHEMA }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_workflow_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_workflow_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioWorkflowSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_workflow_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        decode_workflow_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.workflow` document — 2 nodes (one with 2 params, one with none, incl.
/// a negative coordinate) + 1 edge, exercising every collection/leaf shape at least once. Single
/// source of truth for `📚️examples/🌊️pipeline/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio`
/// and for the conformance-law tests in `🎹️composer/🦀️component.rs`.
#[cfg(test)]
pub(crate) fn demo_workflow_snapshot() -> SemioWorkflowSnapshot {
    SemioWorkflowSnapshot {
        schema: STDIO_SEMIOWORKFLOW_DOCUMENT_SCHEMA.into(),
        nodes: vec![
            WorkflowNode {
                id: "n1".into(),
                kind: "source".into(),
                label: "Source".into(),
                params: vec![WorkflowParam { key: "count".into(), value: "3".into() }, WorkflowParam { key: "unit".into(), value: "items".into() }],
                position: SemioPoint2 { x: 0.0, y: 0.0 },
            },
            WorkflowNode {
                id: "n2".into(),
                kind: "sink".into(),
                label: "Sink".into(),
                params: Vec::new(),
                position: SemioPoint2 { x: 120.5, y: -30.25 },
            },
        ],
        edges: vec![WorkflowEdge {
            id: "e1".into(),
            from: PortRef { node: "n1".into(), port: "out".into() },
            to: PortRef { node: "n2".into(), port: "in".into() },
            kind: "data".into(),
        }],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> SemioWorkflowSnapshot {
        SemioWorkflowSnapshot {
            schema: STDIO_SEMIOWORKFLOW_DOCUMENT_SCHEMA.into(),
            nodes: vec![
                WorkflowNode {
                    id: "n1".into(),
                    kind: "source".into(),
                    label: "Source".into(),
                    params: vec![WorkflowParam { key: "count".into(), value: "3".into() }],
                    position: SemioPoint2 { x: 0.0, y: 0.0 },
                },
                WorkflowNode {
                    id: "n2".into(),
                    kind: "sink".into(),
                    label: "Sink".into(),
                    params: Vec::new(),
                    position: SemioPoint2 { x: 100.0, y: 50.0 },
                },
            ],
            edges: vec![WorkflowEdge {
                id: "e1".into(),
                from: PortRef { node: "n1".into(), port: "out".into() },
                to: PortRef { node: "n2".into(), port: "in".into() },
                kind: "data".into(),
            }],
        }
    }

    #[test]
    fn json_pack_round_trips() {
        let snap = sample();
        let bytes = <SemioWorkflowSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioWorkflowSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[test]
    fn dsl_text_round_trips() {
        let snap = sample();
        let text = <SemioWorkflowSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioWorkflowSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    #[test]
    fn default_snapshot_has_no_nodes_or_edges() {
        let snap = SemioWorkflowSnapshot::default();
        assert!(snap.nodes.is_empty());
        assert!(snap.edges.is_empty());
    }
}
//#endregion 🔖️Tests
