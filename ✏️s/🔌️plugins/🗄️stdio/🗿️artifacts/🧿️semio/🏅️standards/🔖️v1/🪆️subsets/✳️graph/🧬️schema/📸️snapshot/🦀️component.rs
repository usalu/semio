//! 🧬️ SemioGraphSnapshot — the neutral typed property graph: nodes and edges with ports and
//! properties. LEAF subset (no child slots, no link slots) per the master plan's stdio target
//! vocabulary — it is the shape under the reasoning-wires/dag/trinity-jack plugins, and `flow` will
//! compose it later for its own node/edge presentation.
//!
//! Edges are ID-KEYED ENTITIES, not relationships: `source`/`target` node-id fields are carried as
//! ordinary data on the edge entity, addressed/mutated via `create`/`delete` (entity lifecycle),
//! never `connect`/`disconnect` (reserved for an attach/detach handle with no independent identity
//! of its own — not the case here, since an edge needs its own referenceable id for future
//! property/port extension). See `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`'s binding brief for this
//! subset's full ruling.
//!
//! Modeled on `✳️text`'s hand-rolled `ArtifactDsl`/`ArtifactPack` convention (real hex/bracket text
//! codec + real varint-length-prefixed binary codec, both wrapped in the shared
//! `store::semio_format` envelope). `position: SemioPoint2` and `properties: Vec<SemioValueEntry>`
//! are REAL reuse of the shared engine geometry type and the `✳️value` subset's scalar-value
//! vocabulary (never redefined locally) — see `engine::geometry::SemioPoint2` and
//! `subsets::value::schema::snapshot::SemioValueEntry`.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets};
use crate::artifacts::semio::standards::v1::subsets::value::schema::diff::{dec_semio_value_bin, dec_semio_value_entry, enc_semio_value_bin, enc_semio_value_entry};
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueEntry;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
/// 🏷️ Document schema / DSL envelope id AND `ArtifactSchema` descriptor id — same literal for
/// both, per the master plan's "Schema descriptor ids `s.stdio.semio` + `s.stdio.semio.<subset>`"
/// convention, one per subset.
pub const STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA: &str = "s.stdio.semio.graph";
//#endregion 🔖️Ids

//#region 🔖️NodeId
/// 🪪 Stable identity for a graph node — a NAMED single-field struct, never a bare tuple newtype
/// (`dsl` has no blanket `DslField` impl for tuples of any arity — see `✳️value`'s `ValueId` for
/// the precedent this follows).
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphNodeId {
    pub value: String,
}

impl GraphNodeId {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
    }
}
//#endregion 🔖️NodeId

//#region 🔖️EdgeId
/// 🪪 Stable identity for a graph edge — same named-single-field convention as [`GraphNodeId`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphEdgeId {
    pub value: String,
}

impl GraphEdgeId {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
    }
}
//#endregion 🔖️EdgeId

//#region 🔖️PortKind
/// 🔌️ The direction a node port carries data in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SemioGraphPortKind {
    #[default]
    In,
    Out,
    InOut,
}
//#endregion 🔖️PortKind

//#region 🔖️Port
/// 🔌️ One named port on a node. Intrinsically ordered, anonymous, nested inside its owning node's
/// `ports` — the same shape `✳️text`'s marks-inside-run pattern uses one level up
/// (`➕add-node-port`/`🔚remove-node-port`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SemioGraphPort {
    pub name: String,
    pub kind: SemioGraphPortKind,
}
//#endregion 🔖️Port

//#region 🔖️Node
/// 🔵 One id-keyed graph node, carrying its own ordered `ports` and `properties` (the latter REUSES
/// `✳️value`'s `SemioValueEntry`, never a redefined parallel key/value type).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SemioGraphNode {
    pub id: GraphNodeId,
    /// 🏷️ Freeform node-type tag, mirrors `flow`'s `FlowNode.kind`.
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub position: SemioPoint2,
    #[serde(default)]
    pub ports: Vec<SemioGraphPort>,
    #[serde(default)]
    pub properties: Vec<SemioValueEntry>,
}
//#endregion 🔖️Node

//#region 🔖️Edge
/// ➡️ One id-keyed graph edge. `source`/`target` are ordinary data fields on this entity, not an
/// attach/detach relationship — see this file's module doc comment for the `create`/`delete` (not
/// `connect`/`disconnect`) ruling.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SemioGraphEdge {
    pub id: GraphEdgeId,
    pub source: GraphNodeId,
    pub target: GraphNodeId,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub label: String,
}
//#endregion 🔖️Edge

//#region 🔖️Snapshot
/// 🕸️ `nodes`/`edges` are both id-keyed sets with no user-meaningful display order, so there is no
/// `reorder-nodes`/`reorder-edges` mutation (`SEMANTIC-MUTATIONS-OVERHAUL`'s
/// `📓️derivation-rules.md` rule 2: "drop reorder for id-keyed sets with no display order").
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.graph")]
pub struct SemioGraphSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub nodes: Vec<SemioGraphNode>,
    #[state(artifact)]
    #[serde(default)]
    pub edges: Vec<SemioGraphEdge>,
}

impl Default for SemioGraphSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA.into(), nodes: Vec::new(), edges: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️GraphPrimitives
/// 🧪️ Real hex/bracket-encoded value primitives backing the hand-rolled `ArtifactDsl` below — same
/// style `✳️text`'s own `📸️snapshot`/`🔺️diff`/`🧬️mutations` facets already establish, duplicated
/// locally (not imported across facets) to keep each facet module independently compilable.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|it| enc(it)).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| dec(entry)).collect()
}

/// 🆔️ `GraphNodeId`/`GraphEdgeId` encode as a bare hex token directly — same convention a run's
/// `language` field uses in `✳️text`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_node_id(id: &GraphNodeId) -> String {
    enc_str(&id.value)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_node_id(s: &str) -> Result<GraphNodeId, String> {
    Ok(GraphNodeId::new(dec_str(s)?))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_edge_id(id: &GraphEdgeId) -> String {
    enc_str(&id.value)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_edge_id(s: &str) -> Result<GraphEdgeId, String> {
    Ok(GraphEdgeId::new(dec_str(s)?))
}

/// 🔢 `SemioPoint2`'s `x`/`y` are `f64`; encoded as `hex(x.to_string())`/`hex(y.to_string())`
/// (text-lexeme style — never round-tripped through a binary float type in the TEXT codec), parsed
/// back with `.parse::<f64>()`. Two flat comma-separated tokens, no wrapping brackets, so they slot
/// directly into an outer bracketed field list (matches this facet's committed grammar).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point2_fields(p: &SemioPoint2) -> String {
    format!("{},{}", enc_str(&p.x.to_string()), enc_str(&p.y.to_string()))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_f64_hex(s: &str) -> Result<f64, String> {
    dec_str(s)?.parse::<f64>().map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_port_kind(k: SemioGraphPortKind) -> char {
    match k {
        SemioGraphPortKind::In => 'i',
        SemioGraphPortKind::Out => 'o',
        SemioGraphPortKind::InOut => 'x',
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_port_kind(s: &str) -> Result<SemioGraphPortKind, String> {
    match s {
        "i" => Ok(SemioGraphPortKind::In),
        "o" => Ok(SemioGraphPortKind::Out),
        "x" => Ok(SemioGraphPortKind::InOut),
        other => Err(format!("bad port kind {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_port(p: &SemioGraphPort) -> String {
    format!("[{},{}]", enc_str(&p.name), enc_port_kind(p.kind))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_port(s: &str) -> Result<SemioGraphPort, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, kind] = parts.as_slice() else { return Err(format!("port: expected 2 fields, got {}", parts.len())) };
    Ok(SemioGraphPort { name: dec_str(name)?, kind: dec_port_kind(kind)? })
}

/// 🍃️ A property list element is `enc_semio_value_entry(&p)`'s raw output (`hexkey:value`),
/// embedded directly as one comma-separated list element — its own internal `:` never collides
/// with the outer `,`/`[]` delimiters, so no extra wrapping brackets are needed (REUSE of
/// `✳️value`'s diff-facet helpers, not a locally reinvented codec).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_property(p: &SemioValueEntry) -> String {
    enc_semio_value_entry(p)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_property(s: &str) -> Result<SemioValueEntry, String> {
    dec_semio_value_entry(s)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_node(n: &SemioGraphNode) -> String {
    format!("[{},{},{},{},{},{}]", enc_node_id(&n.id), enc_str(&n.kind), enc_str(&n.label), enc_point2_fields(&n.position), enc_list(&n.ports, enc_port), enc_list(&n.properties, enc_property),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_node(s: &str) -> Result<SemioGraphNode, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, kind, label, x, y, ports, properties] = parts.as_slice() else {
        return Err(format!("node: expected 7 fields, got {}", parts.len()));
    };
    Ok(SemioGraphNode { id: dec_node_id(id)?, kind: dec_str(kind)?, label: dec_str(label)?, position: SemioPoint2 { x: dec_f64_hex(x)?, y: dec_f64_hex(y)? }, ports: dec_list(ports, dec_port)?, properties: dec_list(properties, dec_property)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_edge(e: &SemioGraphEdge) -> String {
    format!("[{},{},{},{},{}]", enc_edge_id(&e.id), enc_node_id(&e.source), enc_node_id(&e.target), enc_str(&e.kind), enc_str(&e.label))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_edge(s: &str) -> Result<SemioGraphEdge, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, source, target, kind, label] = parts.as_slice() else {
        return Err(format!("edge: expected 5 fields, got {}", parts.len()));
    };
    Ok(SemioGraphEdge { id: dec_edge_id(id)?, source: dec_node_id(source)?, target: dec_node_id(target)?, kind: dec_str(kind)?, label: dec_str(label)? })
}

/// 📄️ The real structured graph body: three lines — `schema=<hex>`, `nodes=[<node>,...]`,
/// `edges=[<edge>,...]` — matching the grammar's `document = artifact-mark schema-line nodes-line
/// edges-line`. Newlines are pure lexer trivia in the shared dialect.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_graph_snapshot_body(s: &SemioGraphSnapshot) -> String {
    format!("schema={}\nnodes={}\nedges={}", enc_str(&s.schema), enc_list(&s.nodes, enc_node), enc_list(&s.edges, enc_edge))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_graph_snapshot_body(body: &str) -> Result<SemioGraphSnapshot, String> {
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
            nodes = dec_list(rest, dec_node)?;
        } else if let Some(rest) = line.strip_prefix("edges=") {
            edges = dec_list(rest, dec_edge)?;
        } else {
            return Err(format!("semio graph snapshot: unknown line {line:?}"));
        }
    }
    Ok(SemioGraphSnapshot { schema: schema.ok_or_else(|| "semio graph snapshot: missing schema line".to_string())?, nodes, edges })
}
//#endregion 🔖️GraphPrimitives

//#region 🔖️BinaryPrimitives
/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers every other real semio codec in this standard uses).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn port_kind_tag(k: SemioGraphPortKind) -> u8 {
    match k {
        SemioGraphPortKind::In => 0,
        SemioGraphPortKind::Out => 1,
        SemioGraphPortKind::InOut => 2,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn port_kind_from_tag(tag: u8) -> Result<SemioGraphPortKind, String> {
    match tag {
        0 => Ok(SemioGraphPortKind::In),
        1 => Ok(SemioGraphPortKind::Out),
        2 => Ok(SemioGraphPortKind::InOut),
        other => Err(format!("unsupported port kind tag {other}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_port(out: &mut Vec<u8>, p: &SemioGraphPort) {
    write_str_lp(out, &p.name);
    out.push(port_kind_tag(p.kind));
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_port(reader: &mut store::ByteReader<'_>) -> Result<SemioGraphPort, String> {
    let name = read_str_lp(reader)?;
    let kind = port_kind_from_tag(reader.read_u8().map_err(|e| e.to_string())?)?;
    Ok(SemioGraphPort { name, kind })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_property(out: &mut Vec<u8>, p: &SemioValueEntry) {
    write_str_lp(out, &p.key);
    enc_semio_value_bin(&p.value, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_property(reader: &mut store::ByteReader<'_>) -> Result<SemioValueEntry, String> {
    let key = read_str_lp(reader)?;
    let value = dec_semio_value_bin(reader)?;
    Ok(SemioValueEntry { key, value })
}

/// 🔢 `SemioPoint2`'s `x`/`y` written raw (8+8 bytes, no length prefix needed for a fixed-size
/// float) via `f64::to_le_bytes()`/`f64::from_le_bytes()`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_point2(out: &mut Vec<u8>, p: &SemioPoint2) {
    out.extend_from_slice(&p.x.to_le_bytes());
    out.extend_from_slice(&p.y.to_le_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_point2(reader: &mut store::ByteReader<'_>) -> Result<SemioPoint2, String> {
    let x = f64::from_le_bytes(reader.read_bytes(8).map_err(|e| e.to_string())?.try_into().map_err(|_| "point2: short x".to_string())?);
    let y = f64::from_le_bytes(reader.read_bytes(8).map_err(|e| e.to_string())?.try_into().map_err(|_| "point2: short y".to_string())?);
    Ok(SemioPoint2 { x, y })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_node(out: &mut Vec<u8>, n: &SemioGraphNode) {
    write_str_lp(out, &n.id.value);
    write_str_lp(out, &n.kind);
    write_str_lp(out, &n.label);
    write_point2(out, &n.position);
    store::pack_rt::write_varint_u64(out, n.ports.len() as u64);
    for p in &n.ports {
        write_port(out, p);
    }
    store::pack_rt::write_varint_u64(out, n.properties.len() as u64);
    for p in &n.properties {
        write_property(out, p);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_node(reader: &mut store::ByteReader<'_>) -> Result<SemioGraphNode, String> {
    let id = GraphNodeId::new(read_str_lp(reader)?);
    let kind = read_str_lp(reader)?;
    let label = read_str_lp(reader)?;
    let position = read_point2(reader)?;
    let port_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut ports = Vec::with_capacity(port_count as usize);
    for _ in 0..port_count {
        ports.push(read_port(reader)?);
    }
    let property_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut properties = Vec::with_capacity(property_count as usize);
    for _ in 0..property_count {
        properties.push(read_property(reader)?);
    }
    Ok(SemioGraphNode { id, kind, label, position, ports, properties })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_edge(out: &mut Vec<u8>, e: &SemioGraphEdge) {
    write_str_lp(out, &e.id.value);
    write_str_lp(out, &e.source.value);
    write_str_lp(out, &e.target.value);
    write_str_lp(out, &e.kind);
    write_str_lp(out, &e.label);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_edge(reader: &mut store::ByteReader<'_>) -> Result<SemioGraphEdge, String> {
    let id = GraphEdgeId::new(read_str_lp(reader)?);
    let source = GraphNodeId::new(read_str_lp(reader)?);
    let target = GraphNodeId::new(read_str_lp(reader)?);
    let kind = read_str_lp(reader)?;
    let label = read_str_lp(reader)?;
    Ok(SemioGraphEdge { id, source, target, kind, label })
}

/// 🎁 `format u8` + varint-length-prefixed `schema` UTF-8 — both genuinely, individually
/// protocol-walkable — then `nodes`/`edges` (varint count + per-record fields) as the honest opaque
/// `payload` tail (`protocol-array-of-records` gap — homogeneous, variable-length repeated records).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_graph_snapshot_binary(s: &SemioGraphSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.nodes.len() as u64);
    for n in &s.nodes {
        write_node(&mut out, n);
    }
    store::pack_rt::write_varint_u64(&mut out, s.edges.len() as u64);
    for e in &s.edges {
        write_edge(&mut out, e);
    }
    out
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_graph_snapshot_binary(bytes: &[u8]) -> Result<SemioGraphSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = semio_framework_plugin::resolve_ready(store::ByteReader::new(bytes));
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let node_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut nodes = Vec::with_capacity(node_count as usize);
    for _ in 0..node_count {
        nodes.push(read_node(&mut reader)?);
    }
    let edge_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut edges = Vec::with_capacity(edge_count as usize);
    for _ in 0..edge_count {
        edges.push(read_edge(&mut reader)?);
    }
    Ok(SemioGraphSnapshot { schema, nodes, edges })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁 Real structured text/binary codecs, wrapped in the repo-wide `store::semio_format` envelope.
impl store::ArtifactDsl for SemioGraphSnapshot {
    const EXTENSION: &'static str = "semio";
    async fn envelope_id() -> &'static str {
        STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA
    }

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_graph_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    async fn print_dsl(&self) -> String {
        let body = print_graph_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioGraphSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_graph_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_graph_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.graph` document — two nodes (each with ≥1 port and ≥1 property,
/// non-default position) and one edge connecting them, exercising every leaf/collection shape at
/// least once. Single source of truth for `📚️examples/…/🖼️assets/🗣️example.dsl.semio`/
/// `🎒️example.pack.semio` and for the conformance-law tests in `🚪️io/🦀️component.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_graph_snapshot() -> SemioGraphSnapshot {
    SemioGraphSnapshot {
        schema: STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA.into(),
        nodes: vec![
            SemioGraphNode {
                id: GraphNodeId::new("n1"),
                kind: "source".into(),
                label: "Source".into(),
                position: SemioPoint2 { x: 0.0, y: 0.0 },
                ports: vec![SemioGraphPort { name: "out".into(), kind: SemioGraphPortKind::Out }],
                properties: vec![SemioValueEntry { key: "weight".into(), value: crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue::Int { lexeme: "1".into() } }],
            },
            SemioGraphNode {
                id: GraphNodeId::new("n2"),
                kind: "sink".into(),
                label: "Sink".into(),
                position: SemioPoint2 { x: 120.5, y: -30.25 },
                ports: vec![SemioGraphPort { name: "in".into(), kind: SemioGraphPortKind::In }],
                properties: vec![SemioValueEntry { key: "label".into(), value: crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue::Str { value: "sink node".into() } }],
            },
        ],
        edges: vec![SemioGraphEdge { id: GraphEdgeId::new("e1"), source: GraphNodeId::new("n1"), target: GraphNodeId::new("n2"), kind: "flow".into(), label: "Main".into() }],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn populated() -> SemioGraphSnapshot {
        demo_graph_snapshot()
    }

    #[semio_framework_async_macros::async_test]
    async fn json_pack_round_trips() {
        let snap = SemioGraphSnapshot::default();
        let bytes = <SemioGraphSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioGraphSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_text_round_trips() {
        let snap = SemioGraphSnapshot::default();
        let text = <SemioGraphSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioGraphSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    /// 🧪️ codec_retention_law: decode(encode(snapshot)) is byte-for-byte structurally identical
    /// on a fully-populated snapshot (nodes/edges/ports/properties non-empty), not just the default.
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        let snap = populated();
        let bytes = <SemioGraphSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioGraphSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
        let text = <SemioGraphSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back_text = <SemioGraphSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back_text);
    }
}
//#endregion 🔖️Tests
