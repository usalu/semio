//! 📜️ `trinity.graph` artifact — textual document grammar surface + laws (constitutional: dsl).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::jack::{Camera, JackSnapshot, Node, Port, PortDirection, PropertyBag, TrinityRamError};
use store::{DocumentDsl, PackDecodeOptions, PackEncodeOptions, PackError, TextError, TextSpan};

//#region 🔖️DslMirrors
/// 🔒️ Local twin of `PortDirection` (foreign, re-exported from `math::graph::manifest` and
/// consumed by the shared jack query kernel/`semio_s_plugin_trinity`/`framework::*` — this crate does
/// not own the freedom to reshape it) purely so the DSL engine's derive macros have something local to
/// bind: the orphan rule blocks `impl dsl::DslField for PortDirection` directly in this crate.
/// Converted at the `Port`/`PortDsl` boundary via `From`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar)]
enum PortDirectionDsl {
    In,
    Out,
}

impl From<PortDirection> for PortDirectionDsl {
    fn from(value: PortDirection) -> Self {
        match value {
            PortDirection::In => PortDirectionDsl::In,
            PortDirection::Out => PortDirectionDsl::Out,
        }
    }
}

impl From<PortDirectionDsl> for PortDirection {
    fn from(value: PortDirectionDsl) -> Self {
        match value {
            PortDirectionDsl::In => PortDirection::In,
            PortDirectionDsl::Out => PortDirection::Out,
        }
    }
}

/// 🔌️ Local mirror of `Port` for DSL round-tripping — `Port.direction: PortDirection` is foreign, so
/// `Port` itself cannot derive `dsl::DslRecord` (orphan rule); this twin swaps in `PortDirectionDsl`.
/// `pub(crate)` because `📡️spr`'s own `TrinityGraphOperationDsl` mirror (the `CreateNode.ports` field)
/// reuses this exact twin rather than redefining it.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub(crate) struct PortDsl {
    id: String,
    kind: String,
    direction: PortDirectionDsl,
    properties: PropertyBag,
}

pub(crate) fn port_to_port_dsl(port: &Port) -> PortDsl {
    PortDsl { id: port.id.clone(), kind: port.kind.clone(), direction: port.direction.into(), properties: port.properties.clone() }
}

pub(crate) fn port_dsl_to_port(port: PortDsl) -> Port {
    Port { id: port.id, kind: port.kind, direction: port.direction.into(), properties: port.properties }
}

/// 🧩️ Local mirror of `Node` — needed only because `Node.ports: Vec<Port>` transitively carries
/// `Port`'s foreign `direction` field; every other `Node` field is already DSL-ready directly.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct NodeDsl {
    id: String,
    kind: String,
    name: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    properties: PropertyBag,
    #[dsl(table)]
    ports: Vec<PortDsl>,
}

fn node_to_node_dsl(node: &Node) -> NodeDsl {
    NodeDsl { id: node.id.clone(), kind: node.kind.clone(), name: node.name.clone(), x: node.x, y: node.y, width: node.width, height: node.height, properties: node.properties.clone(), ports: node.ports.iter().map(port_to_port_dsl).collect() }
}

fn node_dsl_to_node(node: NodeDsl) -> Node {
    Node { id: node.id, kind: node.kind, name: node.name, x: node.x, y: node.y, width: node.width, height: node.height, properties: node.properties, ports: node.ports.into_iter().map(port_dsl_to_port).collect() }
}

/// 📦️ Local mirror of `JackSnapshot` for the `.trinity` document DSL. `manifest: Manifest` is
/// deliberately NOT a field here — the manifest is resolved from `manifestId` at load time (see
/// `JackSnapshot::resolve_manifest`), never round-tripped as text.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
#[dsl(extension = "trinity", layout = "lines")]
struct JackSnapshotDsl {
    schema: String,
    name: String,
    manifest_id: Option<String>,
    #[dsl(block)]
    camera: Camera,
    #[dsl(table)]
    nodes: Vec<NodeDsl>,
    #[dsl(table)]
    edges: Vec<crate::artifacts::jack::Edge>,
    root_node_id: Option<String>,
}
//#region 🔖️HandcraftedDocumentCodecs
/// ✉️ P6 handcrafted DocumentDsl/DocumentPack (derive no longer emits these traits).
impl store::DocumentDsl for JackSnapshotDsl {
    const EXTENSION: &'static str = "trinity";
    fn envelope_id() -> &'static str { "trinity.jack" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for JackSnapshotDsl {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️HandcraftedDocumentCodecs




fn jack_snapshot_to_dsl(fixture: &JackSnapshot) -> JackSnapshotDsl {
    JackSnapshotDsl {
        schema: fixture.schema.clone(),
        name: fixture.name.clone(),
        manifest_id: fixture.manifest_id.clone(),
        camera: fixture.camera.clone(),
        nodes: fixture.nodes.iter().map(node_to_node_dsl).collect(),
        edges: fixture.edges.clone(),
        root_node_id: fixture.root_node_id.clone(),
    }
}

/// 🔁️ Reconstructs the real `manifest` field via `resolve_manifest` (looked up from `manifest_id`).
fn jack_snapshot_dsl_to_jack_snapshot(parsed: JackSnapshotDsl) -> Result<JackSnapshot, TrinityRamError> {
    let mut fixture = JackSnapshot {
        schema: parsed.schema,
        name: parsed.name,
        manifest_id: parsed.manifest_id,
        manifest: crate::artifacts::jack::Manifest::default(),
        camera: parsed.camera,
        nodes: parsed.nodes.into_iter().map(node_dsl_to_node).collect(),
        edges: parsed.edges,
        root_node_id: parsed.root_node_id,
    };
    fixture.resolve_manifest()?;
    Ok(fixture)
}
//#endregion 🔖️DslMirrors

//#region 🔖️DslDocument
/// 📜️ `.trinity` textual notation for a whole [`JackSnapshot`] (`store::DocumentDsl`), delegating to
/// the derive-generated `JackSnapshotDsl` mirror. Also hand-implements `dsl::DslField` (normally
/// auto-emitted alongside `#[derive(dsl::DslRecord)]`) so `JackSnapshot` can be nested as an
/// ordinary field too — `TrinityGraphMutation::SetFixture` embeds a whole fixture snapshot.
impl DocumentDsl for JackSnapshot {
    const EXTENSION: &'static str = "trinity";
    fn envelope_id() -> &'static str { "trinity.jack" }

    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let parsed = <JackSnapshotDsl as DocumentDsl>::parse_dsl(text)?;
        jack_snapshot_dsl_to_jack_snapshot(parsed).map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        <JackSnapshotDsl as DocumentDsl>::print_dsl(&jack_snapshot_to_dsl(self))
    }
}

impl dsl::DslField for JackSnapshot {
    fn shape() -> dsl::Shape {
        <JackSnapshotDsl as dsl::DslField>::shape()
    }

    fn to_value(&self) -> dsl::FieldValue {
        <JackSnapshotDsl as dsl::DslField>::to_value(&jack_snapshot_to_dsl(self))
    }

    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let parsed = <JackSnapshotDsl as dsl::DslField>::from_value(value)?;
        jack_snapshot_dsl_to_jack_snapshot(parsed).map_err(|error| error.to_string())
    }
}
//#endregion 🔖️DslDocument

//#region 🔖️Pack
/// 📦️ Binary pack notation for a whole [`JackSnapshot`] (`store::DocumentPack`), delegating through
/// the same mirror + `jack_snapshot_to_dsl`/`jack_snapshot_dsl_to_jack_snapshot` pair as the DSL impl
/// above (kept here, next to the mirror types it depends on, rather than in `🎒️pack` — the mirror is
/// private to this file).
impl store::DocumentPack for JackSnapshot {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        <JackSnapshotDsl as store::DocumentPack>::encode_pack_with(&jack_snapshot_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let parsed = <JackSnapshotDsl as store::DocumentPack>::decode_pack_with(bytes, options)?;
        jack_snapshot_dsl_to_jack_snapshot(parsed).map_err(|error| store::text_error_to_pack_error(TextError::new(error.to_string(), TextSpan::at(1, 1))))
    }
}
//#endregion 🔖️Pack

/// 📄️ The Nakagin Capsule Tower example fixture, handcrafted in the `.trinity` DSL.
pub const NAKAGIN_EXAMPLE_TEXT: &str = include_str!("../../../../../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.trinity` DSL text into a `JackSnapshot`.
pub fn parse_dsl(text: &str) -> Result<JackSnapshot, TextError> {
    <JackSnapshot as DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `JackSnapshot` back to `.trinity` DSL text.
pub fn print_dsl(document: &JackSnapshot) -> String {
    DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::jack::empty_trinity_graph_fixture;

    #[test]
    fn nakagin_example_dsl_round_trips() {
        let document = parse_dsl(NAKAGIN_EXAMPLE_TEXT).expect("parse nakagin example");
        ::store::os_store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn empty_document_dsl_round_trips() {
        ::store::os_store::test_support::assert_dsl_round_trip(&empty_trinity_graph_fixture());
    }

    #[test]
    fn parse_dsl_rejects_unknown_keyword() {
        let err = JackSnapshot::parse_dsl("bogus line").expect_err("unknown keyword");
        assert!(err.message.contains("expected"));
    }

    #[test]
    fn dsl_round_trip_mini_and_bundled_fixtures() {
        let nakagin = parse_dsl(NAKAGIN_EXAMPLE_TEXT).unwrap();
        ::store::os_store::test_support::assert_dsl_round_trip(&nakagin);
        ::store::os_store::test_support::assert_dsl_pack_equivalence(&nakagin);
        let branch = parse_dsl(include_str!("../../../../../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio")).unwrap();
        ::store::os_store::test_support::assert_dsl_round_trip(&branch);
        ::store::os_store::test_support::assert_dsl_pack_equivalence(&branch);
    }

    /// 🧩️ A hand-built fixture (not one of the bundled `.trinity` examples) with a nested `Object`-shaped
    /// node property (`position: {x,y,z}`) and `Number`-shaped edge properties (`u`/`v`) — exercises the
    /// `#[dsl(table)]` nested-row-within-a-row path (`NodeDsl.ports`) and `PropertyBag`'s `Object`/
    /// `Number` variants through the DSL round trip, distinct from what the bundled examples happen to
    /// contain.
    #[test]
    fn dsl_round_trip_mini_fixture() {
        use crate::artifacts::jack::{Camera, Edge, Manifest, Node, Port, PortDirection, PropertyBag, PropertyValue};
        use std::collections::BTreeMap;

        let fixture = JackSnapshot {
            schema: JackSnapshot::SCHEMA.into(),
            name: "mini".into(),
            manifest_id: Some("nakagin".into()),
            manifest: Manifest::nakagin_default(),
            camera: Camera::default(),
            root_node_id: Some("root".into()),
            nodes: vec![
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
            edges: vec![Edge {
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
        };
        ::store::os_store::test_support::assert_dsl_round_trip(&fixture);
        ::store::os_store::test_support::assert_dsl_pack_equivalence(&fixture);
    }
}
//#endregion 🧪️Tests
