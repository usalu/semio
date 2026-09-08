//! 📤️ Serialize `s.stdio.semio/v1/flow` into a real `s.stdio.json` (rfc8259) snapshot — the
//! mirror of this pair's deserializer. Lossless: every `FlowNode`/`FlowEdge` field has a
//! direct JSON member, so `serialize`+`deserialize` round-trips exactly.

use semio_s_artifact_stdio_json::schema::snapshot::{JsonMember, JsonValue};
use semio_s_artifact_stdio_json::JsonSnapshot;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, FlowNode, FlowParam, PortRef, SemioFlowSnapshot};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

//#region 🔖️FieldMapping
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn str_val(s: &str) -> JsonValue {
    JsonValue::String { value: s.to_string() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn num_val(n: f64) -> JsonValue {
    JsonValue::Number { lexeme: format!("{n}") }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn member(key: &str, value: JsonValue) -> JsonMember {
    JsonMember { key: key.to_string(), value }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn obj(members: Vec<JsonMember>) -> JsonValue {
    JsonValue::Object { members }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_to_json(p: &SemioPoint2) -> JsonValue {
    obj(vec![member("x", num_val(p.x)), member("y", num_val(p.y))])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn param_to_json(p: &FlowParam) -> JsonValue {
    obj(vec![member("key", str_val(&p.key)), member("value", str_val(&p.value))])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn port_ref_to_json(p: &PortRef) -> JsonValue {
    obj(vec![member("node", str_val(&p.node)), member("port", str_val(&p.port))])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn node_to_json(n: &FlowNode) -> JsonValue {
    obj(vec![
        member("id", str_val(&n.id)),
        member("kind", str_val(&n.kind)),
        member("label", str_val(&n.label)),
        member("params", JsonValue::Array { items: n.params.iter().map(param_to_json).collect() }),
        member("position", point_to_json(&n.position)),
    ])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn edge_to_json(e: &FlowEdge) -> JsonValue {
    obj(vec![member("id", str_val(&e.id)), member("from", port_ref_to_json(&e.from)), member("to", port_ref_to_json(&e.to)), member("kind", str_val(&e.kind))])
}
//#endregion 🔖️FieldMapping

//#region 🔖️Serializer
pub struct SemioFlowToJson;

impl ArtifactSerializer for SemioFlowToJson {
    type From = SemioFlowSnapshot;
    type Into = JsonSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("flow") };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let value = obj(vec![member("nodes", JsonValue::Array { items: from.nodes.iter().map(node_to_json).collect() }), member("edges", JsonValue::Array { items: from.edges.iter().map(edge_to_json).collect() })]);
        Ok(JsonSnapshot { schema: semio_s_artifact_stdio_json::STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
