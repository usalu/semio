//! 🧬️ SemioFlowSnapshot — id-keyed `nodes{kind,label,params,position:SemioPoint2}` +
//! `PortRef`-addressed `edges{from,to,kind}`, informed by OS `🔁️workflow` WorkflowNode +
//! `🌊️flow/🕸️dag` (see master-plan.md's "Subset snapshot cores" table). Complete per spec: no
//! `serde_json::Value`, no bare tuples (`PortRef`/`SemioPoint2` are named structs), no nested
//! fixed arrays.

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;



//#region 🔖️PortRef
/// 🔌️ Addresses one named port on one node — the endpoint shape `FlowEdge` connects through.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PortRef {
    pub node: String,
    pub port: String,
}
//#endregion 🔖️PortRef

//#region 🔖️Param
/// 🎛️ One ordered key-value node parameter. String-valued is the honest boundary for a flow
/// DAG's per-node config — a richer typed value graph is `value` subset's job (`SemioValue`), not
/// flow's; see w1b-type-ownership.md's per-subset owned-types table.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct FlowParam {
    pub key: String,
    pub value: String,
}
//#endregion 🔖️Param

//#region 🔖️Node
/// 🔁️ A node owned by the Flow subset.
/// The workflow artifact owns its separate `semio_framework_artifact_workflow_workflow::WorkflowNode`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct FlowNode {
    pub id: String,
    pub kind: String,
    pub label: String,
    #[value(default)]
    pub params: Vec<FlowParam>,
    pub position: SemioPoint2,
}
//#endregion 🔖️Node

//#region 🔖️Edge
/// ➡️ Owned by the `flow` subset. `id`-keyed (like `nodes`) so the sparse diff can address one
/// edge by identity rather than by its `(from,to,kind)` value, which is not guaranteed unique in a
/// real multigraph DAG.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct FlowEdge {
    pub id: String,
    pub from: PortRef,
    pub to: PortRef,
    pub kind: String,
}
//#endregion 🔖️Edge

use framework_schema::ArtifactSchema;

//#region 🔖️Ids
pub const STDIO_SEMIOFLOW_DOCUMENT_SCHEMA: &str = "stdio.semio.flow";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.flow")]
pub struct SemioFlowSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 🆔️ Id-keyed strong collection — sparse-diffed via `🧰️triples::NamedTripleDiff`.
    #[state(artifact)]
    #[value(default)]
    pub nodes: Vec<FlowNode>,
    /// 🆔️ Id-keyed strong collection — sparse-diffed via `🧰️triples::NamedTripleDiff`.
    #[state(artifact)]
    #[value(default)]
    pub edges: Vec<FlowEdge>,
}

impl Default for SemioFlowSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(), nodes: Default::default(), edges: Default::default() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextPrimitives



















//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives







//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🌉️ExternalCodecBridge











//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.flow` document — 2 nodes (one with 2 params, one with none, incl.
/// a negative coordinate) + 1 edge, exercising every collection/leaf shape at least once. Single
/// source of truth for `📚️examples/🌊️pipeline/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio`
/// and for the conformance-law tests in `🎹️composer/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_flow_snapshot() -> SemioFlowSnapshot {
    SemioFlowSnapshot {
        schema: STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(),
        nodes: vec![
            FlowNode {
                id: "n1".into(),
                kind: "source".into(),
                label: "Source".into(),
                params: vec![FlowParam { key: "count".into(), value: "3".into() }, FlowParam { key: "unit".into(), value: "items".into() }],
                position: SemioPoint2 { x: 0.0, y: 0.0 },
            },
            FlowNode { id: "n2".into(), kind: "sink".into(), label: "Sink".into(), params: Vec::new(), position: SemioPoint2 { x: 120.5, y: -30.25 } },
        ],
        edges: vec![FlowEdge { id: "e1".into(), from: PortRef { node: "n1".into(), port: "out".into() }, to: PortRef { node: "n2".into(), port: "in".into() }, kind: "data".into() }],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests






