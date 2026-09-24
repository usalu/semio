//! 📜️ `trinity.graph` artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `JackSnapshot.nodes`/`.edges` are gone —
//! replaced by a single composed `content: JackContentChild` slot. The old `JackSnapshotDsl`/
//! `NodeDsl` mirror existed only to give the derive engine a table-of-records path through `Node`'s
//! foreign-`direction`-bearing `ports`; since `nodes`/`edges` are now opaque (hidden inside the
//! composed child, never exposed on this struct), that half of the mirror is dead and removed — text
//! and pack both encode `JackPackRecord`, the derived record of `JackSnapshot` plus its child content.
//!
//! `PortDsl`/`PortDirectionDsl`/`port_to_port_dsl`/`port_dsl_to_port` are KEPT — they are also
//! consumed by `🧬️mutations/💾️binary` to encode raw `Node`/`Port` values carried directly on
//! mutation payloads (e.g. `CreateNode.node: Node`), an entirely separate concern from this
//! snapshot's own persisted shape.
//!
//! ⚠️ **The WIRE FORMAT still carries the real `nodes`/`edges` data** (JSON-blob-encoded), not just
//! the opaque handle — matching `dag`'s `<semio_framework_artifact_flow_flow::FlowHostSnapshot as ArtifactDsl>::parse_dsl` precedent.
//! No `LinkResolver`/child-dispatch seam exists yet (see the artifact root's `🔖️WorkingScene`), so
//! the working-scene cache is only populated in-process, by whatever call SET the `content` field. A
//! codec that persisted only the bare handle would produce an UNRECOVERABLE snapshot the instant a
//! fresh process parses it. `parse_dsl`/`decode_pack` therefore mint+cache a FRESH content-addressed
//! handle from the decoded nodes/edges every time (deterministic — same data always re-derives the
//! same handle, so peers replaying the same bytes converge); `print_dsl`/`encode_pack` read the
//! CURRENT cached scene back out via `jack_working_scene`.
//!
//! ⚠️ Known gap, honestly documented rather than silently left: the committed
//! `📖️.grammar.semio` grammar file still describes the OLD `nodes`/`edges`-table shape (it
//! backs `pilot_languages()`'s `jack.document` `LanguageSpec` registration for editor tooling, not
//! this codec) — the snapshot text itself is the derived `dsl::print` of `JackPackRecord`.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::{Edge, JackSnapshot, Node, Port, PortDirection, PropertyBag};
use store::{ArtifactDsl, PackDecodeOptions, PackEncodeOptions, PackError, TextError, TextSpan};

//#region 🔖️DslMirrors
/// 🔒️ Local twin of `PortDirection` (foreign, re-exported from `semio_framework_graph::manifest` and
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
/// `pub(crate)` because `🧬️mutations/💾️binary`'s own `TrinityGraphOperationDsl` mirror (the
/// `CreateNode.ports` field) reuses this exact twin rather than redefining it.
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
//#endregion 🔖️DslMirrors

//#region 🔖️PackRecord
/// 🧬️ Derived pack record of a `JackSnapshot`: the persisted fields plus the composed graph child's
/// content (`nodes`/`edges`), from which decode re-mints the content-addressed child handle.
#[derive(dsl::DslRecord)]
#[dsl(extension = "trinity")]
struct JackPackRecord {
    schema: String,
    name: String,
    manifest_id: Option<String>,
    camera: dsl::DslValue,
    nodes: dsl::DslValue,
    edges: dsl::DslValue,
    root_node_id: Option<String>,
}

impl JackPackRecord {
    fn from_snapshot(snapshot: &JackSnapshot) -> Self {
        let scene = crate::jack_working_scene(snapshot);
        Self {
            schema: snapshot.schema.clone(),
            name: snapshot.name.clone(),
            manifest_id: snapshot.manifest_id.clone(),
            camera: dsl::ToValue::to_value(&snapshot.camera),
            nodes: dsl::ToValue::to_value(&scene.nodes),
            edges: dsl::ToValue::to_value(&scene.edges),
            root_node_id: snapshot.root_node_id.clone(),
        }
    }

    fn into_snapshot(self) -> Result<JackSnapshot, String> {
        let camera = dsl::FromValue::from_value(self.camera).map_err(|e| e.to_string())?;
        let nodes: Vec<Node> = dsl::FromValue::from_value(self.nodes).map_err(|e| e.to_string())?;
        let edges: Vec<Edge> = dsl::FromValue::from_value(self.edges).map_err(|e| e.to_string())?;
        let content = crate::jack_content_child_with_owner(nodes, edges);
        let mut fixture = JackSnapshot { schema: self.schema, name: self.name, manifest_id: self.manifest_id, manifest: crate::Manifest::default(), camera, content, root_node_id: self.root_node_id };
        fixture.resolve_manifest().map_err(|error| error.to_string())?;
        Ok(fixture)
    }
}

/// 🖨️ The derived text body: the same `JackPackRecord` the pack encodes, printed by the spec-driven engine.
pub(crate) fn print_pack_record_text(snapshot: &JackSnapshot) -> String {
    dsl::print(&JackPackRecord::from_snapshot(snapshot).__dsl_to_record(), &JackPackRecord::__dsl_spec(), dsl::JoinMode::Document)
}

/// 📖️ Parses a derived text body back through `JackPackRecord`, with the same decode steps as the pack.
pub(crate) fn parse_pack_record_text(body: &str) -> Result<JackSnapshot, store::TextError> {
    let record = dsl::parse(body, &JackPackRecord::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
    JackPackRecord::__dsl_from_record(&record)?.into_snapshot().map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1)))
}
//#endregion 🔖️PackRecord

//#region 🔖️HandcraftedArtifactCodecs
impl ArtifactDsl for JackSnapshot {
    const EXTENSION: &'static str = "trinity";
    fn envelope_id() -> &'static str {
        "trinity.jack"
    }

    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_pack_record_text(body)
    }

    fn print_dsl(&self) -> String {
        let body = print_pack_record_text(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for JackSnapshot {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = store::pack_rt::encode_document(&JackPackRecord::__dsl_spec(), &JackPackRecord::from_snapshot(self).__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }

    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &JackPackRecord::__dsl_spec(), options)?;
        JackPackRecord::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.into_snapshot().map_err(PackError::Schema)
    }

    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(JackPackRecord::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

/// 📄️ The Nakagin Capsule Tower example fixture, handcrafted in the `.trinity` DSL.
pub const NAKAGIN_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.trinity` DSL text into a `JackSnapshot`.
pub fn parse_dsl(text: &str) -> Result<JackSnapshot, TextError> {
    <JackSnapshot as ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `JackSnapshot` back to `.trinity` DSL text.
pub fn print_dsl(document: &JackSnapshot) -> String {
    ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type JackSnapshotText = String;
//#endregion 🚚️Carrier

