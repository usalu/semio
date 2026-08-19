//! 📜️ `trinity.graph` artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `JackSnapshot.nodes`/`.edges` are gone —
//! replaced by a single composed `content: JackContentChild` slot. The old `JackSnapshotDsl`/
//! `NodeDsl` mirror existed only to give the derive engine a table-of-records path through `Node`'s
//! foreign-`direction`-bearing `ports`; since `nodes`/`edges` are now opaque (hidden inside the
//! composed child, never exposed on this struct), that half of the mirror is dead and removed — this
//! is a hand-rolled `ArtifactDsl`/`ArtifactPack` for `JackSnapshot` itself, matching `dag`'s own
//! precedent (`📓️wave4-reports/dag-report.md`) exactly.
//!
//! `PortDsl`/`PortDirectionDsl`/`port_to_port_dsl`/`port_dsl_to_port` are KEPT — they are also
//! consumed by `🧬️mutations/💾️binary` to encode raw `Node`/`Port` values carried directly on
//! mutation payloads (e.g. `CreateNode.node: Node`), an entirely separate concern from this
//! snapshot's own persisted shape.
//!
//! ⚠️ **The WIRE FORMAT still carries the real `nodes`/`edges` data** (JSON-blob-encoded), not just
//! the opaque handle — matching `dag`'s `<flow::FlowFixture as ArtifactDsl>::parse_dsl` precedent.
//! No `LinkResolver`/child-dispatch seam exists yet (see the artifact root's `🔖️WorkingScene`), so
//! the working-scene cache is only populated in-process, by whatever call SET the `content` field. A
//! codec that persisted only the bare handle would produce an UNRECOVERABLE snapshot the instant a
//! fresh process parses it. `parse_dsl`/`decode_pack` therefore mint+cache a FRESH content-addressed
//! handle from the decoded nodes/edges every time (deterministic — same data always re-derives the
//! same handle, so peers replaying the same bytes converge); `print_dsl`/`encode_pack` read the
//! CURRENT cached scene back out via `jack_working_scene`.
//!
//! ⚠️ Known gap, honestly documented rather than silently left: the committed
//! `📖️component.grammar.semio` grammar file still describes the OLD `nodes`/`edges`-table shape (it
//! backs `pilot_languages()`'s `jack.document` `LanguageSpec` registration for editor tooling, not
//! this codec) — it was not regenerated this pass since the actual wire format below no longer goes
//! through the `dsl::parse`/`dsl::print` engine at all (hand-rolled `schema=`/`name=`/…/`nodes=`
//! line format instead, mirroring `dag`'s `DagSnapshot` codec exactly). Follow-up work, flagged in
//! this ticket's report.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::jack::{Edge, JackSnapshot, Node, Port, PortDirection, PropertyBag};
use store::{ArtifactDsl, PackDecodeOptions, PackEncodeOptions, PackError, TextError, TextSpan};

//#region 🔖️DslMirrors
/// 🔒️ Local twin of `PortDirection` (foreign, re-exported from `graph::manifest` and
/// consumed by the shared jack query kernel/`semio_s_plugin_trinity`/`framework::*` — this crate does
/// not own the freedom to reshape it) purely so the DSL engine's derive macros have something local to
/// bind: the orphan rule blocks `impl dsl::DslField for PortDirection` directly in this crate.
/// Converted at the `Port`/`PortDsl` boundary via `From`. STILL USED by `🧬️mutations/💾️binary`'s
/// `TrinityGraphOperationDsl` mirror (`CreateNode`'s `ports` field) — not dead code.
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar)]
pub(crate) enum PortDirectionDsl {
    In,
    Out,
}

impl From<PortDirection> for PortDirectionDsl {
    async fn from(value: PortDirection) -> Self {
        match value {
            PortDirection::In => PortDirectionDsl::In,
            PortDirection::Out => PortDirectionDsl::Out,
        }
    }
}

impl From<PortDirectionDsl> for PortDirection {
    async fn from(value: PortDirectionDsl) -> Self {
        match value {
            PortDirectionDsl::In => PortDirection::In,
            PortDirectionDsl::Out => PortDirection::Out,
        }
    }
}

/// 🔌️ Local mirror of `Port` for DSL round-tripping — `Port.direction: PortDirection` is foreign, so
/// `Port` itself cannot derive `dsl::DslRecord` (orphan rule); this twin swaps in `PortDirectionDsl`.
/// `pub(crate)` because `🧬️mutations/💾️binary`'s own `TrinityGraphOperationDsl` mirror (the
/// `CreateNode.ports` field) reuses this exact twin rather than redefining it.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub(crate) struct PortDsl {
    id: String,
    kind: String,
    direction: PortDirectionDsl,
    properties: PropertyBag,
}

pub(crate) async fn port_to_port_dsl(port: &Port) -> PortDsl {
    PortDsl { id: port.id.clone(), kind: port.kind.clone(), direction: port.direction.into(), properties: port.properties.clone() }
}

pub(crate) async fn port_dsl_to_port(port: PortDsl) -> Port {
    Port { id: port.id, kind: port.kind, direction: port.direction.into(), properties: port.properties }
}
//#endregion 🔖️DslMirrors

//#region 🔖️CodecPrimitives
/// 🧪️ Real hex-encoded value primitives backing the hand-rolled `ArtifactDsl` below — same style
/// `dag`'s own facet establishes, duplicated locally (not imported across crates) to keep this facet
/// independently compilable.
async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) async fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) async fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
async fn enc_opt_str(s: &Option<String>) -> String {
    match s {
        Some(v) => enc_str(v),
        None => "-".to_string(),
    }
}
async fn dec_opt_str(s: &str) -> Result<Option<String>, String> {
    if s == "-" {
        Ok(None)
    } else {
        Ok(Some(dec_str(s)?))
    }
}

async fn print_jack_snapshot_body(s: &JackSnapshot) -> String {
    let scene = crate::artifacts::jack::jack_working_scene(s);
    let camera_json = serde_json::to_string(&s.camera).unwrap_or_default();
    let nodes_json = serde_json::to_string(&scene.nodes).unwrap_or_default();
    let edges_json = serde_json::to_string(&scene.edges).unwrap_or_default();
    format!(
        "schema={}\nname={}\nmanifestId={}\ncamera={}\nnodes={}\nedges={}\nrootNodeId={}",
        enc_str(&s.schema),
        enc_str(&s.name),
        enc_opt_str(&s.manifest_id),
        enc_str(&camera_json),
        enc_str(&nodes_json),
        enc_str(&edges_json),
        enc_opt_str(&s.root_node_id),
    )
}

async fn parse_jack_snapshot_body(body: &str) -> Result<JackSnapshot, String> {
    let mut schema = None;
    let mut name = None;
    let mut manifest_id: Option<Option<String>> = None;
    let mut camera = None;
    let mut nodes: Option<Vec<Node>> = None;
    let mut edges: Option<Vec<Edge>> = None;
    let mut root_node_id: Option<Option<String>> = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("name=") {
            name = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("manifestId=") {
            manifest_id = Some(dec_opt_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("camera=") {
            camera = Some(serde_json::from_str(&dec_str(rest)?).map_err(|e| e.to_string())?);
        } else if let Some(rest) = line.strip_prefix("nodes=") {
            nodes = Some(serde_json::from_str(&dec_str(rest)?).map_err(|e| e.to_string())?);
        } else if let Some(rest) = line.strip_prefix("edges=") {
            edges = Some(serde_json::from_str(&dec_str(rest)?).map_err(|e| e.to_string())?);
        } else if let Some(rest) = line.strip_prefix("rootNodeId=") {
            root_node_id = Some(dec_opt_str(rest)?);
        } else {
            return Err(format!("jack snapshot: unknown line {line:?}"));
        }
    }
    let schema = schema.ok_or_else(|| "jack snapshot: missing schema line".to_string())?;
    let name = name.ok_or_else(|| "jack snapshot: missing name line".to_string())?;
    let manifest_id = manifest_id.ok_or_else(|| "jack snapshot: missing manifestId line".to_string())?;
    let camera = camera.ok_or_else(|| "jack snapshot: missing camera line".to_string())?;
    let nodes = nodes.ok_or_else(|| "jack snapshot: missing nodes line".to_string())?;
    let edges = edges.ok_or_else(|| "jack snapshot: missing edges line".to_string())?;
    let root_node_id = root_node_id.ok_or_else(|| "jack snapshot: missing rootNodeId line".to_string())?;
    let content = crate::artifacts::jack::jack_content_child_handle_and_cache(nodes, edges);
    let mut fixture = JackSnapshot { schema, name, manifest_id, manifest: crate::artifacts::jack::Manifest::default(), camera, content, root_node_id };
    fixture.resolve_manifest().map_err(|error| error.to_string())?;
    Ok(fixture)
}
//#endregion 🔖️CodecPrimitives

//#region 🔖️BinaryPrimitives
/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`), matching `dag`'s own `write_str_lp`/`read_str_lp` convention.
async fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
async fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
async fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
async fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
impl ArtifactDsl for JackSnapshot {
    const EXTENSION: &'static str = "trinity";
    async fn envelope_id() -> &'static str { "trinity.jack" }

    async fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_jack_snapshot_body(body).map_err(|e| TextError::new(e, TextSpan::at(1, 1)))
    }

    async fn print_dsl(&self) -> String {
        let body = print_jack_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for JackSnapshot {
    async fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let _ = options;
        let scene = crate::artifacts::jack::jack_working_scene(self);
        let mut out = Vec::new();
        const PACK_BINARY_FORMAT: u8 = 1;
        out.push(PACK_BINARY_FORMAT);
        write_str_lp(&mut out, &self.schema);
        write_str_lp(&mut out, &self.name);
        write_str_lp(&mut out, self.manifest_id.as_deref().unwrap_or(""));
        write_str_lp(&mut out, &self.manifest_id.is_some().to_string());
        write_str_lp(&mut out, &serde_json::to_string(&self.camera).unwrap_or_default());
        write_str_lp(&mut out, &serde_json::to_string(&scene.nodes).unwrap_or_default());
        write_str_lp(&mut out, &serde_json::to_string(&scene.edges).unwrap_or_default());
        write_str_lp(&mut out, self.root_node_id.as_deref().unwrap_or(""));
        write_str_lp(&mut out, &self.root_node_id.is_some().to_string());
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &out))
    }

    async fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as ArtifactDsl>::envelope_id() {
            return Err(PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        let mut reader = store::ByteReader::new(&inner);
        const PACK_BINARY_FORMAT: u8 = 1;
        let format = reader.read_u8().map_err(|e| PackError::Schema(e.to_string()))?;
        if format != PACK_BINARY_FORMAT {
            return Err(PackError::Schema(format!("unsupported pack format {format}")));
        }
        let schema = read_str_lp(&mut reader).map_err(PackError::Schema)?;
        let name = read_str_lp(&mut reader).map_err(PackError::Schema)?;
        let manifest_id_raw = read_str_lp(&mut reader).map_err(PackError::Schema)?;
        let manifest_id_present: bool = read_str_lp(&mut reader).map_err(PackError::Schema)?.parse().unwrap_or(false);
        let manifest_id = manifest_id_present.then_some(manifest_id_raw);
        let camera_json = read_str_lp(&mut reader).map_err(PackError::Schema)?;
        let camera = serde_json::from_str(&camera_json).map_err(|e| PackError::Schema(e.to_string()))?;
        let nodes_json = read_str_lp(&mut reader).map_err(PackError::Schema)?;
        let nodes: Vec<Node> = serde_json::from_str(&nodes_json).map_err(|e| PackError::Schema(e.to_string()))?;
        let edges_json = read_str_lp(&mut reader).map_err(PackError::Schema)?;
        let edges: Vec<Edge> = serde_json::from_str(&edges_json).map_err(|e| PackError::Schema(e.to_string()))?;
        let root_node_id_raw = read_str_lp(&mut reader).map_err(PackError::Schema)?;
        let root_node_id_present: bool = read_str_lp(&mut reader).map_err(PackError::Schema)?.parse().unwrap_or(false);
        let root_node_id = root_node_id_present.then_some(root_node_id_raw);
        let content = crate::artifacts::jack::jack_content_child_handle_and_cache(nodes, edges);
        let mut fixture = JackSnapshot { schema, name, manifest_id, manifest: crate::artifacts::jack::Manifest::default(), camera, content, root_node_id };
        fixture.resolve_manifest().map_err(|error| store::text_error_to_pack_error(TextError::new(error.to_string(), TextSpan::at(1, 1))))?;
        Ok(fixture)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

/// 📄️ The Nakagin Capsule Tower example fixture, handcrafted in the `.trinity` DSL.
pub const NAKAGIN_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.trinity` DSL text into a `JackSnapshot`.
pub async fn parse_dsl(text: &str) -> Result<JackSnapshot, TextError> {
    <JackSnapshot as ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `JackSnapshot` back to `.trinity` DSL text.
pub async fn print_dsl(document: &JackSnapshot) -> String {
    ArtifactDsl::print_dsl(document)
}


//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::jack::empty_trinity_graph_fixture;

    #[test]
    async fn nakagin_example_dsl_round_trips() {
        let document = parse_dsl(NAKAGIN_EXAMPLE_TEXT).expect("parse nakagin example");
        ::store::os_store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    async fn empty_document_dsl_round_trips() {
        ::store::os_store::test_support::assert_dsl_round_trip(&empty_trinity_graph_fixture());
    }

    #[test]
    async fn parse_dsl_rejects_unknown_keyword() {
        let err = JackSnapshot::parse_dsl("bogus line").expect_err("unknown keyword");
        assert!(err.message.contains("jack snapshot"));
    }

    #[test]
    async fn dsl_round_trip_mini_and_bundled_fixtures() {
        let nakagin = parse_dsl(NAKAGIN_EXAMPLE_TEXT).unwrap();
        ::store::os_store::test_support::assert_dsl_round_trip(&nakagin);
        ::store::os_store::test_support::assert_dsl_pack_equivalence(&nakagin);
    }

    /// 🧩️ A hand-built fixture (not one of the bundled `.trinity` examples) with a nested `Object`-shaped
    /// node property (`position: {x,y,z}`) and `Number`-shaped edge properties (`u`/`v`) — exercises the
    /// JSON-blob content codec round trip on non-trivial `PropertyBag`'s `Object`/`Number` variants.
    #[test]
    async fn dsl_round_trip_mini_fixture() {
        use crate::artifacts::jack::{Camera, Edge, JackSnapshot, Manifest, Node, Port, PortDirection, PropertyBag, PropertyValue};
        use std::collections::BTreeMap;

        let fixture = JackSnapshot::with_content(
            JackSnapshot::SCHEMA.into(),
            "mini".into(),
            Some("nakagin".into()),
            Manifest::nakagin_default(),
            Camera::default(),
            vec![
                Node {
                    id: "root".into(),
                    kind: "Piece".into(),
                    name: "core".into(),
                    x: 0.0,
                    y: 0.0,
                    width: 80.0,
                    height: 40.0,
                    properties: {
                        let mut p = PropertyBag::new();
                        let mut pos = BTreeMap::new();
                        pos.insert("x".into(), PropertyValue::Number(0.0));
                        pos.insert("y".into(), PropertyValue::Number(0.0));
                        pos.insert("z".into(), PropertyValue::Number(0.0));
                        p.insert("position".into(), PropertyValue::Object(pos));
                        p
                    },
                    ports: vec![Port { id: "out-a".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
                },
                Node {
                    id: "child".into(),
                    kind: "Piece".into(),
                    name: "capsule".into(),
                    x: 120.0,
                    y: 0.0,
                    width: 80.0,
                    height: 40.0,
                    properties: PropertyBag::new(),
                    ports: vec![Port { id: "in-a".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
                },
            ],
            vec![Edge {
                id: "e1".into(),
                kind: "Connection".into(),
                source: "root@out-a".into(),
                target: "child@in-a".into(),
                properties: {
                    let mut p = PropertyBag::new();
                    p.insert("u".into(), PropertyValue::Number(1.2));
                    p.insert("v".into(), PropertyValue::Number(-0.6));
                    p
                },
            }],
            Some("root".into()),
        );
        ::store::os_store::test_support::assert_dsl_round_trip(&fixture);
        ::store::os_store::test_support::assert_dsl_pack_equivalence(&fixture);
    }
}
//#endregion 🧪️Tests
