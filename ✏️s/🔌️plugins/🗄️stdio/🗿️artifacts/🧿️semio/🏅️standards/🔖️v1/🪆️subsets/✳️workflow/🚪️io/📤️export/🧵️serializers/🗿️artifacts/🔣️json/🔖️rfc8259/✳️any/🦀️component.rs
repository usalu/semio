//! 📤️ Serialize `s.stdio.semio/v1/workflow` into a real `s.stdio.json` (rfc8259) snapshot — the
//! mirror of this pair's deserializer. Lossless: every `WorkflowNode`/`WorkflowEdge` field has a
//! direct JSON member, so `serialize`+`deserialize` round-trips exactly.

use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use crate::artifacts::json::JsonSnapshot;
use crate::artifacts::json::schema::snapshot::{JsonMember, JsonValue};
use crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::workflow::schema::snapshot::{PortRef, SemioWorkflowSnapshot, WorkflowEdge, WorkflowNode, WorkflowParam};

//#region 🔖️FieldMapping
fn str_val(s: &str) -> JsonValue {
    JsonValue::String { value: s.to_string() }
}
fn num_val(n: f64) -> JsonValue {
    JsonValue::Number { lexeme: format!("{n}") }
}
fn member(key: &str, value: JsonValue) -> JsonMember {
    JsonMember { key: key.to_string(), value }
}
fn obj(members: Vec<JsonMember>) -> JsonValue {
    JsonValue::Object { members }
}

fn point_to_json(p: &SemioPoint2) -> JsonValue {
    obj(vec![member("x", num_val(p.x)), member("y", num_val(p.y))])
}

fn param_to_json(p: &WorkflowParam) -> JsonValue {
    obj(vec![member("key", str_val(&p.key)), member("value", str_val(&p.value))])
}

fn port_ref_to_json(p: &PortRef) -> JsonValue {
    obj(vec![member("node", str_val(&p.node)), member("port", str_val(&p.port))])
}

fn node_to_json(n: &WorkflowNode) -> JsonValue {
    obj(vec![
        member("id", str_val(&n.id)),
        member("kind", str_val(&n.kind)),
        member("label", str_val(&n.label)),
        member("params", JsonValue::Array { items: n.params.iter().map(param_to_json).collect() }),
        member("position", point_to_json(&n.position)),
    ])
}

fn edge_to_json(e: &WorkflowEdge) -> JsonValue {
    obj(vec![member("id", str_val(&e.id)), member("from", port_ref_to_json(&e.from)), member("to", port_ref_to_json(&e.to)), member("kind", str_val(&e.kind))])
}
//#endregion 🔖️FieldMapping

//#region 🔖️Serializer
pub struct SemioWorkflowToJson;

impl ArtifactSerializer for SemioWorkflowToJson {
    type From = SemioWorkflowSnapshot;
    type Into = JsonSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("workflow") };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

    fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let value = obj(vec![
            member("nodes", JsonValue::Array { items: from.nodes.iter().map(node_to_json).collect() }),
            member("edges", JsonValue::Array { items: from.edges.iter().map(edge_to_json).collect() }),
        ]);
        Ok(JsonSnapshot { schema: crate::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::workflow::schema::snapshot::STDIO_SEMIOWORKFLOW_DOCUMENT_SCHEMA;

    fn sample_semio() -> SemioWorkflowSnapshot {
        SemioWorkflowSnapshot {
            schema: STDIO_SEMIOWORKFLOW_DOCUMENT_SCHEMA.into(),
            nodes: vec![
                WorkflowNode { id: "n1".into(), kind: "source".into(), label: "Source".into(), params: vec![WorkflowParam { key: "count".into(), value: "3".into() }], position: SemioPoint2 { x: 0.0, y: 0.0 } },
                WorkflowNode { id: "n2".into(), kind: "sink".into(), label: "Sink".into(), params: Vec::new(), position: SemioPoint2 { x: 100.0, y: 50.0 } },
            ],
            edges: vec![WorkflowEdge { id: "e1".into(), from: PortRef { node: "n1".into(), port: "out".into() }, to: PortRef { node: "n2".into(), port: "in".into() }, kind: "data".into() }],
        }
    }

    #[test]
    fn maps_nodes_and_edges_to_json() {
        let json = SemioWorkflowToJson::serialize(&sample_semio()).expect("serialize");
        let root = match &json.value {
            JsonValue::Object { members } => members,
            other => panic!("expected object, got {other:?}"),
        };
        let nodes = root.iter().find(|m| m.key == "nodes").expect("nodes member");
        match &nodes.value {
            JsonValue::Array { items } => assert_eq!(items.len(), 2),
            other => panic!("expected array, got {other:?}"),
        }
    }

    /// 🔁️ Full round trip through THIS pair alone: serialize then encode/decode through the real
    /// `JsonSnapshot::ArtifactPack` byte codec and re-parse — proves the JSON shape this leaf emits
    /// is not just structurally right but genuinely re-parseable RFC8259 text.
    #[test]
    fn serialized_json_round_trips_through_the_real_json_text_codec() {
        let json1 = SemioWorkflowToJson::serialize(&sample_semio()).expect("serialize");
        let text = crate::artifacts::json::schema::snapshot::write_json_text(&json1.value);
        let reparsed = crate::artifacts::json::schema::snapshot::parse_json_text(&text).expect("re-parse emitted json text");
        assert_eq!(reparsed, json1.value);
    }
}
//#endregion 🔖️Tests
