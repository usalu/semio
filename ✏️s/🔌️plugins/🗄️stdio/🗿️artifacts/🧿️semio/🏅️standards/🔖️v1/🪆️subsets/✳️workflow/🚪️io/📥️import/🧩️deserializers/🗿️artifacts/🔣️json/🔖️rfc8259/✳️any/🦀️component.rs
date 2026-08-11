//! 📥️ Deserialize `s.stdio.semio/v1/workflow` from a real `s.stdio.json` (rfc8259) snapshot —
//! near-direct structural mapping (`{"nodes":[...],"edges":[...]}`), no lossy fields: every
//! `WorkflowNode`/`WorkflowEdge` field has a 1:1 JSON member. Malformed/missing members are real
//! errors (`store::PackError::Schema`), never silently defaulted away.

use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};
use crate::artifacts::json::JsonSnapshot;
use crate::artifacts::json::schema::snapshot::{JsonMember, JsonValue};
use crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::workflow::schema::snapshot::{PortRef, STDIO_SEMIOWORKFLOW_DOCUMENT_SCHEMA, SemioWorkflowSnapshot, WorkflowEdge, WorkflowNode, WorkflowParam};

//#region 🔖️JsonAccessors
fn get<'a>(members: &'a [JsonMember], key: &str) -> Result<&'a JsonValue, store::PackError> {
    members.iter().find(|m| m.key == key).map(|m| &m.value).ok_or_else(|| store::PackError::Schema(format!("workflow json: missing member {key:?}")))
}
fn as_object(v: &JsonValue) -> Result<&[JsonMember], store::PackError> {
    match v {
        JsonValue::Object { members } => Ok(members),
        other => Err(store::PackError::Schema(format!("workflow json: expected object, got {other:?}"))),
    }
}
fn as_array(v: &JsonValue) -> Result<&[JsonValue], store::PackError> {
    match v {
        JsonValue::Array { items } => Ok(items),
        other => Err(store::PackError::Schema(format!("workflow json: expected array, got {other:?}"))),
    }
}
fn as_string(v: &JsonValue) -> Result<String, store::PackError> {
    match v {
        JsonValue::String { value } => Ok(value.clone()),
        other => Err(store::PackError::Schema(format!("workflow json: expected string, got {other:?}"))),
    }
}
fn as_f64(v: &JsonValue) -> Result<f64, store::PackError> {
    match v {
        JsonValue::Number { lexeme } => lexeme.parse::<f64>().map_err(|e| store::PackError::Schema(format!("workflow json: bad number lexeme {lexeme:?}: {e}"))),
        other => Err(store::PackError::Schema(format!("workflow json: expected number, got {other:?}"))),
    }
}
//#endregion 🔖️JsonAccessors

//#region 🔖️FieldMapping
fn map_point(v: &JsonValue) -> Result<SemioPoint2, store::PackError> {
    let m = as_object(v)?;
    Ok(SemioPoint2 { x: as_f64(get(m, "x")?)?, y: as_f64(get(m, "y")?)? })
}

fn map_param(v: &JsonValue) -> Result<WorkflowParam, store::PackError> {
    let m = as_object(v)?;
    Ok(WorkflowParam { key: as_string(get(m, "key")?)?, value: as_string(get(m, "value")?)? })
}

fn map_port_ref(v: &JsonValue) -> Result<PortRef, store::PackError> {
    let m = as_object(v)?;
    Ok(PortRef { node: as_string(get(m, "node")?)?, port: as_string(get(m, "port")?)? })
}

fn map_node(v: &JsonValue) -> Result<WorkflowNode, store::PackError> {
    let m = as_object(v)?;
    let params = match m.iter().find(|e| e.key == "params") {
        Some(e) => as_array(&e.value)?.iter().map(map_param).collect::<Result<Vec<_>, _>>()?,
        None => Vec::new(),
    };
    Ok(WorkflowNode { id: as_string(get(m, "id")?)?, kind: as_string(get(m, "kind")?)?, label: as_string(get(m, "label")?)?, params, position: map_point(get(m, "position")?)? })
}

fn map_edge(v: &JsonValue) -> Result<WorkflowEdge, store::PackError> {
    let m = as_object(v)?;
    Ok(WorkflowEdge { id: as_string(get(m, "id")?)?, from: map_port_ref(get(m, "from")?)?, to: map_port_ref(get(m, "to")?)?, kind: as_string(get(m, "kind")?)? })
}
//#endregion 🔖️FieldMapping

//#region 🔖️Deserializer
pub struct SemioWorkflowFromJson;

impl ArtifactDeserializer for SemioWorkflowFromJson {
    type From = JsonSnapshot;
    type Into = SemioWorkflowSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("workflow") };

    fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let root = as_object(&from.value)?;
        let nodes = as_array(get(root, "nodes")?)?.iter().map(map_node).collect::<Result<Vec<_>, _>>()?;
        let edges = as_array(get(root, "edges")?)?.iter().map(map_edge).collect::<Result<Vec<_>, _>>()?;
        Ok(SemioWorkflowSnapshot { schema: STDIO_SEMIOWORKFLOW_DOCUMENT_SCHEMA.into(), nodes, edges })
    }
}
//#endregion 🔖️Deserializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn sample_json() -> JsonSnapshot {
        let text = r#"{"nodes":[{"id":"n1","kind":"source","label":"Source","params":[{"key":"count","value":"3"}],"position":{"x":0,"y":0}},{"id":"n2","kind":"sink","label":"Sink","params":[],"position":{"x":100,"y":50}}],"edges":[{"id":"e1","from":{"node":"n1","port":"out"},"to":{"node":"n2","port":"in"},"kind":"data"}]}"#;
        JsonSnapshot { schema: crate::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA.into(), value: crate::artifacts::json::schema::snapshot::parse_json_text(text).expect("valid json fixture") }
    }

    #[test]
    fn maps_nodes_and_edges() {
        let semio = SemioWorkflowFromJson::deserialize(&sample_json()).expect("deserialize");
        assert_eq!(semio.nodes.len(), 2);
        assert_eq!(semio.edges.len(), 1);
        assert_eq!(semio.nodes[0].id, "n1");
        assert_eq!(semio.nodes[0].params[0].key, "count");
        assert_eq!(semio.nodes[1].position.x, 100.0);
        assert_eq!(semio.edges[0].from.node, "n1");
        assert_eq!(semio.edges[0].to.port, "in");
    }

    #[test]
    fn missing_required_member_is_a_real_error() {
        let bad = JsonSnapshot { schema: crate::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA.into(), value: crate::artifacts::json::schema::snapshot::parse_json_text("{}").unwrap() };
        assert!(SemioWorkflowFromJson::deserialize(&bad).is_err());
    }
}
//#endregion 🔖️Tests
