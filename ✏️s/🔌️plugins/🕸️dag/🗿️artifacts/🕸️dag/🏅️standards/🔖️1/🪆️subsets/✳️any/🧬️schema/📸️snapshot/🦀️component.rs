//! 🧬️ DAG snapshot schema — persistent fields only.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `nodes`/`edges` are gone from this STRUCT —
//! replaced by a single composed `content: DagContentChild` slot (`s.stdio.semio.graph`). The old
//! `DagSnapshotDsl`/`DagNodeSpecDsl`/`DagNodeKindDsl` mirror existed only to give the derive engine a
//! `Box`-wrapped path through `DagNodeSpec.kind`; since that field is now opaque (hidden inside the
//! composed child, never exposed on this struct), the mirror and its derive are both gone — this is a
//! hand-rolled `ArtifactDsl`/`ArtifactPack`.
//!
//! ⚠️ **The WIRE FORMAT still carries the real `nodes`/`edges` data** (JSON-blob-encoded), not just
//! the opaque handle — matching flow's own `<flow::FlowFixture as ArtifactDsl>::parse_dsl(text).map(
//! Self::from_fixture)` precedent exactly. Reasoning: no `LinkResolver`/child-dispatch seam exists
//! yet (see `🔖️WorkingScene` in the artifact root), so the working-scene cache is only populated
//! in-process, by whatever call SET the `content` field (a mutation diff, `from_fixture`, …). A
//! codec that persisted only the bare handle would produce an UNRECOVERABLE snapshot the instant a
//! fresh process parses it (confirmed by a real test failure during this migration: `default_snapshot
//! ()` came back with an empty scene on every fresh run, silently vacuous-passing several inverse-law
//! tests). `parse_dsl`/`decode_pack` therefore mint+cache a FRESH content-addressed handle from the
//! decoded nodes/edges every time (deterministic — same data always re-derives the same handle, so
//! peers replaying the same bytes converge); `print_dsl`/`encode_pack` read the CURRENT cached scene
//! back out via `dag_working_scene`.

use crate::artifacts::dag::{DagContentChild, DagFixtureEdge, DagNodeSpec, DAG_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted DAG document snapshot — schema tag plus the composed `graph` content child.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.dag.dag")]
pub struct DagSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[child(kind = "s.stdio.semio.graph")]
    pub content: DagContentChild,
}

impl Default for DagSnapshot {
    fn default() -> Self {
        default_snapshot()
    }
}

/// 🌱 Canonical default document used by the play app and examples.
pub fn default_snapshot() -> DagSnapshot {
    crate::artifacts::dag::dsl::parse_dsl(crate::artifacts::dag::dsl::DAG_EXAMPLE_TEXT)
        .expect("bundled dag example DSL must parse")
}
//#endregion 🔖️Snapshot

//#region 🔖️CodecPrimitives
/// 🧪️ Real hex/bracket-encoded value primitives backing the hand-rolled `ArtifactDsl` below — same
/// style stdio's own `✳️graph`/`✳️text` facets already establish, duplicated locally (not imported
/// across crates) to keep this facet independently compilable.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

fn print_dag_snapshot_body(s: &DagSnapshot) -> String {
    let scene = crate::artifacts::dag::dag_working_scene(s);
    let nodes_json = serde_json::to_string(&scene.nodes).unwrap_or_default();
    let edges_json = serde_json::to_string(&scene.edges).unwrap_or_default();
    format!("schema={}\nnodes={}\nedges={}", enc_str(&s.schema), enc_str(&nodes_json), enc_str(&edges_json))
}
fn parse_dag_snapshot_body(body: &str) -> Result<DagSnapshot, String> {
    let mut schema = None;
    let mut nodes: Option<Vec<DagNodeSpec>> = None;
    let mut edges: Option<Vec<DagFixtureEdge>> = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("nodes=") {
            nodes = Some(serde_json::from_str(&dec_str(rest)?).map_err(|e| e.to_string())?);
        } else if let Some(rest) = line.strip_prefix("edges=") {
            edges = Some(serde_json::from_str(&dec_str(rest)?).map_err(|e| e.to_string())?);
        } else {
            return Err(format!("dag snapshot: unknown line {line:?}"));
        }
    }
    let schema = schema.ok_or_else(|| "dag snapshot: missing schema line".to_string())?;
    let nodes = nodes.ok_or_else(|| "dag snapshot: missing nodes line".to_string())?;
    let edges = edges.ok_or_else(|| "dag snapshot: missing edges line".to_string())?;
    let content = crate::artifacts::dag::dag_content_child_handle_and_cache(nodes, edges);
    Ok(DagSnapshot { schema, content })
}
//#endregion 🔖️CodecPrimitives

//#region 🔖️BinaryPrimitives
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

fn encode_dag_snapshot_binary(s: &DagSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let scene = crate::artifacts::dag::dag_working_scene(s);
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    write_str_lp(&mut out, &serde_json::to_string(&scene.nodes).unwrap_or_default());
    write_str_lp(&mut out, &serde_json::to_string(&scene.edges).unwrap_or_default());
    out
}
fn decode_dag_snapshot_binary(bytes: &[u8]) -> Result<DagSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let nodes: Vec<DagNodeSpec> = serde_json::from_str(&read_str_lp(&mut reader)?).map_err(|e| e.to_string())?;
    let edges: Vec<DagFixtureEdge> = serde_json::from_str(&read_str_lp(&mut reader)?).map_err(|e| e.to_string())?;
    let content = crate::artifacts::dag::dag_content_child_handle_and_cache(nodes, edges);
    Ok(DagSnapshot { schema, content })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for DagSnapshot {
    const EXTENSION: &'static str = "dag";
    fn envelope_id() -> &'static str {
        "dag.dag"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let mut snapshot = parse_dag_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
        snapshot.schema = DAG_DOCUMENT_SCHEMA.into();
        Ok(snapshot)
    }
    fn print_dsl(&self) -> String {
        let body = print_dag_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for DagSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_dag_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        let mut snapshot = decode_dag_snapshot_binary(&inner).map_err(store::PackError::Schema)?;
        snapshot.schema = DAG_DOCUMENT_SCHEMA.into();
        Ok(snapshot)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️FrameworkBridge
/// 🌉 `infinite_board_port_directed_dag::DagSnapshot` is the FRAMEWORK's own separate persisted
/// projection (backs `DagFixture`/`DagHost`), unrelated to and unaware of this plugin's composed
/// child — the bridge goes through the working-scene converter, never through `nodes`/`edges` fields
/// (this struct no longer has any).
impl From<DagSnapshot> for infinite_board_port_directed_dag::DagSnapshot {
    fn from(value: DagSnapshot) -> Self {
        let scene = crate::artifacts::dag::dag_working_scene(&value);
        Self { schema: value.schema, nodes: scene.nodes, edges: scene.edges }
    }
}

impl From<infinite_board_port_directed_dag::DagSnapshot> for DagSnapshot {
    fn from(value: infinite_board_port_directed_dag::DagSnapshot) -> Self {
        let content = crate::artifacts::dag::dag_content_child_handle_and_cache(value.nodes, value.edges);
        Self { schema: value.schema, content }
    }
}

impl From<&DagSnapshot> for infinite_board_port_directed_dag::DagSnapshot {
    fn from(value: &DagSnapshot) -> Self {
        value.clone().into()
    }
}

/// 🧾️ Node/edge accessors matching the OLD field-access call-site shape (`document.nodes`), now
/// reading through the working-scene cache. Kept as methods on `DagSnapshot` itself (rather than
/// forcing every call site to import `dag_working_scene`) to minimize the app-layer rewrite's blast
/// radius — see `crate::artifacts::dag::dag_working_scene` for the underlying cache.
impl DagSnapshot {
    pub fn nodes(&self) -> Vec<DagNodeSpec> {
        crate::artifacts::dag::dag_working_scene(self).nodes
    }
    pub fn edges(&self) -> Vec<DagFixtureEdge> {
        crate::artifacts::dag::dag_working_scene(self).edges
    }
}
//#endregion 🔖️FrameworkBridge
