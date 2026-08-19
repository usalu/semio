//! 📥️ Deserialize `s.stdio.semio/v1/flow` from a real `s.stdio.json` (rfc8259) snapshot —
//! near-direct structural mapping (`{"nodes":[...],"edges":[...]}`), no lossy fields: every
//! `FlowNode`/`FlowEdge` field has a 1:1 JSON member. Malformed/missing members are real
//! errors (`store::PackError::Schema`), never silently defaulted away.

use crate::artifacts::json::schema::snapshot::{JsonMember, JsonValue};
use crate::artifacts::json::JsonSnapshot;
use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, FlowNode, FlowParam, PortRef, SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

//#region 🔖️JsonAccessors
async fn get<'a>(members: &'a [JsonMember], key: &str) -> Result<&'a JsonValue, store::PackError> {
    members.iter().find(|m| m.key == key).map(|m| &m.value).ok_or_else(|| store::PackError::Schema(format!("flow json: missing member {key:?}")))
}
async fn as_object(v: &JsonValue) -> Result<&[JsonMember], store::PackError> {
    match v {
        JsonValue::Object { members } => Ok(members),
        other => Err(store::PackError::Schema(format!("flow json: expected object, got {other:?}"))),
    }
}
async fn as_array(v: &JsonValue) -> Result<&[JsonValue], store::PackError> {
    match v {
        JsonValue::Array { items } => Ok(items),
        other => Err(store::PackError::Schema(format!("flow json: expected array, got {other:?}"))),
    }
}
async fn as_string(v: &JsonValue) -> Result<String, store::PackError> {
    match v {
        JsonValue::String { value } => Ok(value.clone()),
        other => Err(store::PackError::Schema(format!("flow json: expected string, got {other:?}"))),
    }
}
async fn as_f64(v: &JsonValue) -> Result<f64, store::PackError> {
    match v {
        JsonValue::Number { lexeme } => lexeme.parse::<f64>().map_err(|e| store::PackError::Schema(format!("flow json: bad number lexeme {lexeme:?}: {e}"))),
        other => Err(store::PackError::Schema(format!("flow json: expected number, got {other:?}"))),
    }
}
//#endregion 🔖️JsonAccessors

//#region 🔖️FieldMapping
async fn map_point(v: &JsonValue) -> Result<SemioPoint2, store::PackError> {
    let m = as_object(v)?;
    Ok(SemioPoint2 { x: as_f64(get(m, "x")?)?, y: as_f64(get(m, "y")?)? })
}

async fn map_param(v: &JsonValue) -> Result<FlowParam, store::PackError> {
    let m = as_object(v)?;
    Ok(FlowParam { key: as_string(get(m, "key")?)?, value: as_string(get(m, "value")?)? })
}

async fn map_port_ref(v: &JsonValue) -> Result<PortRef, store::PackError> {
    let m = as_object(v)?;
    Ok(PortRef { node: as_string(get(m, "node")?)?, port: as_string(get(m, "port")?)? })
}

async fn map_node(v: &JsonValue) -> Result<FlowNode, store::PackError> {
    let m = as_object(v)?;
    let params = match m.iter().find(|e| e.key == "params") {
        Some(e) => as_array(&e.value)?.iter().map(map_param).collect::<Result<Vec<_>, _>>()?,
        None => Vec::new(),
    };
    Ok(FlowNode { id: as_string(get(m, "id")?)?, kind: as_string(get(m, "kind")?)?, label: as_string(get(m, "label")?)?, params, position: map_point(get(m, "position")?)? })
}

async fn map_edge(v: &JsonValue) -> Result<FlowEdge, store::PackError> {
    let m = as_object(v)?;
    Ok(FlowEdge { id: as_string(get(m, "id")?)?, from: map_port_ref(get(m, "from")?)?, to: map_port_ref(get(m, "to")?)?, kind: as_string(get(m, "kind")?)? })
}
//#endregion 🔖️FieldMapping

//#region 🔖️Deserializer
pub struct SemioFlowFromJson;

impl ArtifactDeserializer for SemioFlowFromJson {
    type From = JsonSnapshot;
    type Into = SemioFlowSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("flow") };

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let root = as_object(&from.value)?;
        let nodes = as_array(get(root, "nodes")?)?.iter().map(map_node).collect::<Result<Vec<_>, _>>()?;
        let edges = as_array(get(root, "edges")?)?.iter().map(map_edge).collect::<Result<Vec<_>, _>>()?;
        Ok(SemioFlowSnapshot { schema: STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(), nodes, edges })
    }
}
//#endregion 🔖️Deserializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) async fn sample_json() -> JsonSnapshot {
        let text = r#"{"nodes":[{"id":"n1","kind":"source","label":"Source","params":[{"key":"count","value":"3"}],"position":{"x":0,"y":0}},{"id":"n2","kind":"sink","label":"Sink","params":[],"position":{"x":100,"y":50}}],"edges":[{"id":"e1","from":{"node":"n1","port":"out"},"to":{"node":"n2","port":"in"},"kind":"data"}]}"#;
        JsonSnapshot { schema: crate::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA.into(), value: crate::artifacts::json::schema::snapshot::parse_json_text(text).expect("valid json fixture") }
    }

    #[test]
    async fn maps_nodes_and_edges() {
        let semio = semio_framework_plugin::resolve_ready(SemioFlowFromJson::deserialize(&sample_json())).expect("deserialize");
        assert_eq!(semio.nodes.len(), 2);
        assert_eq!(semio.edges.len(), 1);
        assert_eq!(semio.nodes[0].id, "n1");
        assert_eq!(semio.nodes[0].params[0].key, "count");
        assert_eq!(semio.nodes[1].position.x, 100.0);
        assert_eq!(semio.edges[0].from.node, "n1");
        assert_eq!(semio.edges[0].to.port, "in");
    }

    #[test]
    async fn missing_required_member_is_a_real_error() {
        let bad = JsonSnapshot { schema: crate::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA.into(), value: crate::artifacts::json::schema::snapshot::parse_json_text("{}").unwrap() };
        assert!(semio_framework_plugin::resolve_ready(SemioFlowFromJson::deserialize(&bad)).is_err());
    }
}
//#endregion 🔖️Tests
