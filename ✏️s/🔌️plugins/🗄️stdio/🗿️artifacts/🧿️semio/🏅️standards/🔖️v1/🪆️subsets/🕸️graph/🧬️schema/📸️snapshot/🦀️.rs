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
//! Modeled on `🔤️text`'s hand-rolled `ArtifactDsl`/`ArtifactPack` convention (real hex/bracket text
//! codec + real varint-length-prefixed binary codec, both wrapped in the shared
//! `store::semio_format` envelope). `position: SemioPoint2` and `properties: Vec<SemioValueEntry>`
//! are REAL reuse of the shared engine geometry type and the `🔢️value` subset's scalar-value
//! vocabulary (never redefined locally) — see `engine::geometry::SemioPoint2` and
//! `subsets::value::schema::snapshot::SemioValueEntry`.

use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;






use crate::standards::v1::subsets::value::schema::snapshot::SemioValueEntry;
use framework_schema::ArtifactSchema;

//#region 🔖️Ids
/// 🏷️ Document schema / DSL envelope id AND `ArtifactSchema` descriptor id — same literal for
/// both, per the master plan's "Schema descriptor ids `s.stdio.semio` + `s.stdio.semio.<subset>`"
/// convention, one per subset.
pub const STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA: &str = "s.stdio.semio.graph";
//#endregion 🔖️Ids

//#region 🔖️NodeId
/// 🪪 Stable identity for a graph node — a NAMED single-field struct, never a bare tuple newtype
/// (`dsl` has no blanket `DslField` impl for tuples of any arity — see `🔢️value`'s `ValueId` for
/// the precedent this follows).
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub enum SemioGraphPortKind {
    #[default]
    In,
    Out,
    InOut,
}
//#endregion 🔖️PortKind

//#region 🔖️Port
/// 🔌️ One named port on a node. Intrinsically ordered, anonymous, nested inside its owning node's
/// `ports` — the same shape `🔤️text`'s marks-inside-run pattern uses one level up
/// (`➕add-node-port`/`🔚remove-node-port`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct SemioGraphPort {
    pub name: String,
    pub kind: SemioGraphPortKind,
    pub category: String,
    pub properties: Vec<SemioValueEntry>,
}
//#endregion 🔖️Port

//#region 🔖️Node
/// 🔵 One id-keyed graph node, carrying its own ordered `ports` and `properties` (the latter REUSES
/// `🔢️value`'s `SemioValueEntry`, never a redefined parallel key/value type).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct SemioGraphNode {
    pub id: GraphNodeId,
    /// 🏷️ Freeform node-type tag, mirrors `flow`'s `FlowNode.kind`.
    #[value(default)]
    pub kind: String,
    #[value(default)]
    pub label: String,
    #[value(default)]
    pub position: SemioPoint2,
    pub width: f64,
    pub height: f64,
    #[value(default)]
    pub ports: Vec<SemioGraphPort>,
    #[value(default)]
    pub properties: Vec<SemioValueEntry>,
}
//#endregion 🔖️Node

//#region 🔖️Edge
/// ➡️ One id-keyed graph edge. `source`/`target` are ordinary data fields on this entity, not an
/// attach/detach relationship — see this file's module doc comment for the `create`/`delete` (not
/// `connect`/`disconnect`) ruling.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct SemioGraphEdge {
    pub id: GraphEdgeId,
    pub source: GraphNodeId,
    pub target: GraphNodeId,
    #[value(default)]
    pub kind: String,
    #[value(default)]
    pub label: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub source_port: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target_port: Option<String>,
    pub properties: Vec<SemioValueEntry>,
}
//#endregion 🔖️Edge

//#region 🔖️Snapshot
/// 🕸️ `nodes`/`edges` are both id-keyed sets with no user-meaningful display order, so there is no
/// `reorder-nodes`/`reorder-edges` mutation (`SEMANTIC-MUTATIONS-OVERHAUL`'s
/// `📓️derivation-rules.md` rule 2: "drop reorder for id-keyed sets with no display order").
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.graph")]
pub struct SemioGraphSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub nodes: Vec<SemioGraphNode>,
    #[state(artifact)]
    #[value(default)]
    pub edges: Vec<SemioGraphEdge>,
}

impl Default for SemioGraphSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA.into(), nodes: Vec::new(), edges: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️GraphPrimitives



































//#endregion 🔖️GraphPrimitives

//#region 🔖️BinaryPrimitives


























//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

#[path="🔣️json/🦀️.rs"]
mod declared_json;

//#region 🌉️ExternalCodecBridge



//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Wire







//#endregion 🔖️Wire

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.graph` document — two nodes (each with ≥1 port and ≥1 property,
/// non-default position) and one edge connecting them, exercising every leaf/collection shape at
/// least once. Single source of truth for `📚️examples/…/🖼️assets/🗣️.dsl.semio`/
/// `🎒️.pack.semio` and for the conformance-law tests in `🚪️io/🦀️.rs`.
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
                width: 0.0, height: 0.0,
                ports: vec![SemioGraphPort { name: "out".into(), kind: SemioGraphPortKind::Out, category: String::new(), properties: Vec::new() }],
                properties: vec![SemioValueEntry { key: "weight".into(), value: crate::standards::v1::subsets::value::schema::snapshot::SemioValue::Int { lexeme: "1".into() } }],
            },
            SemioGraphNode {
                id: GraphNodeId::new("n2"),
                kind: "sink".into(),
                label: "Sink".into(),
                position: SemioPoint2 { x: 120.5, y: -30.25 },
                width: 0.0, height: 0.0,
                ports: vec![SemioGraphPort { name: "in".into(), kind: SemioGraphPortKind::In, category: String::new(), properties: Vec::new() }],
                properties: vec![SemioValueEntry { key: "label".into(), value: crate::standards::v1::subsets::value::schema::snapshot::SemioValue::Str { value: "sink node".into() } }],
            },
        ],
        edges: vec![SemioGraphEdge { id: GraphEdgeId::new("e1"), source: GraphNodeId::new("n1"), target: GraphNodeId::new("n2"), kind: "flow".into(), label: "Main".into(), source_port: None, target_port: None, properties: Vec::new() }],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests







