//! 🧬️ SemioWorkflowSnapshot — id-keyed `nodes{kind,label,params,position:SemioPoint2}` +
//! `PortRef`-addressed `edges{from,to,kind}`, informed by OS `🔁️workflow` WorkflowNode +
//! `🌊️flow/🕸️dag` (see master-plan.md's "Subset snapshot cores" table). Complete per spec: no
//! `serde_json::Value`, no bare tuples (`PortRef`/`SemioPoint2` are named structs), no nested
//! fixed arrays.

use crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint2;

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

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁 JSON-pack round trip (honest, genuinely working — this subset's snapshot is a NEUTRAL semio
/// type, not an on-disk file format, so there is no "real" external byte layout to reproduce; the
/// same boundary the sibling W2a/W2b semio subsets use). Wrapped in the repo-wide
/// `store::semio_format` envelope.
impl store::ArtifactDsl for SemioWorkflowSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str { STDIO_SEMIOWORKFLOW_DOCUMENT_SCHEMA }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?;
            bytes.push(byte);
            i += 2;
        }
        serde_json::from_slice(&bytes).map_err(|e| store::TextError::new(format!("json decode: {e}"), dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = serde_json::to_vec(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
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
        let raw = serde_json::to_vec(self).map_err(|e| store::PackError::Schema(e.to_string()))?;
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
        serde_json::from_slice(&inner).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

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
