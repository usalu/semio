
use super::*;
use pack::value::{DslValue, FromValue, Number, ToValue};

fn populated_mesh() -> MeshData {
    let mut mesh = mesh_box(1.0, 2.0, 3.0);
    mesh.uvs = vec![0.25, 0.5];
    mesh.face_ids = vec![7, 8];
    mesh.vertex_ids = vec![1];
    mesh.edge_positions = vec![0.0, 1.0, 2.0];
    mesh.edge_ids = vec![3];
    mesh.edge_uvs = vec![0.75];
    mesh.edge_is_seam = vec![1, 0];
    mesh.paint_texture_base64 = Some("abc".to_string());
    mesh
}

#[test]
fn default_mesh_round_trips() {
    let mesh = MeshData::default();
    let encoded = mesh.to_value();
    assert_eq!(MeshData::from_value(encoded), Ok(mesh));
}

#[test]
fn dense_mesh_round_trips() {
    let mesh = mesh_box(1.0, 2.0, 3.0);
    let encoded = mesh.to_value();
    assert_eq!(MeshData::from_value(encoded), Ok(mesh));
}

#[test]
fn fully_populated_mesh_round_trips() {
    let mesh = populated_mesh();
    let encoded = mesh.to_value();
    assert_eq!(MeshData::from_value(encoded), Ok(mesh));
}

/// 🎯️ The regression this whole ticket guards against: `indices`/`faceIds`/`vertexIds`/
/// `edgeIds`/`edgeIsSeam` must encode as `Number::UInt`, never `Number::Float` — a mesh index
/// silently becoming e.g. `3600.0` would corrupt geometry on decode. Position/normal/color/uv
/// fields must stay the reverse: always `Number::Float`, never crossed with the integer arm.
#[test]
fn index_and_count_fields_encode_as_uint_never_float() {
    let mesh = populated_mesh();
    let DslValue::Object(object) = mesh.to_value() else { panic!("expected an object") };
    for key in ["indices", "faceIds", "vertexIds", "edgeIds", "edgeIsSeam"] {
        let (_, value) = object.iter().find(|(k, _)| k == key).unwrap_or_else(|| panic!("missing {key}"));
        let DslValue::Array(items) = value else { panic!("{key} is not an array") };
        assert!(!items.is_empty(), "{key} fixture must be non-empty to exercise this");
        for item in items {
            assert!(matches!(item, DslValue::Number(Number::UInt(_))), "{key} element is not an integer: {item:?}");
        }
    }
    for key in ["positions", "normals", "uvs", "edgePositions", "edgeUvs"] {
        let (_, value) = object.iter().find(|(k, _)| k == key).unwrap_or_else(|| panic!("missing {key}"));
        let DslValue::Array(items) = value else { panic!("{key} is not an array") };
        assert!(!items.is_empty(), "{key} fixture must be non-empty to exercise this");
        for item in items {
            assert!(matches!(item, DslValue::Number(Number::Float(_))), "{key} element is not a float: {item:?}");
        }
    }
}

#[test]
fn decode_error_reports_the_offending_field_and_index() {
    let DslValue::Object(mut object) = mesh_box(1.0, 1.0, 1.0).to_value() else { panic!("expected an object") };
    let indices = object.iter_mut().find(|(k, _)| k == "indices").expect("indices present");
    indices.1 = DslValue::Array(vec![DslValue::Bool(true)]);
    let error = MeshData::from_value(DslValue::Object(object)).unwrap_err();
    assert_eq!(error.to_string(), "indices.0.expected a number, found Bool(true)");
}

/// 🔬️ Differential oracle: the JSON `pack::json::Value::from(mesh)` produces — proven
/// byte-identical to serde_json's own encoding by `mesh_data_json_oracle_tests` above — decodes
/// through serde_json's `Deserialize` to the SAME `MeshData` our first-party `FromValue` decodes
/// from the equivalent `DslValue` tree.
#[test]
fn from_value_agrees_with_serde_json_oracle_decode() {
    let mesh = populated_mesh();
    let json_value = json::Value::from(mesh.clone());
    let json_text = json::to_string(&json_value);
    let oracle: MeshData = serde_json::from_str(&json_text).expect("serde_json decode");
    let ours = MeshData::from_value(mesh.to_value()).expect("first-party decode");
    assert_eq!(ours, oracle);
    assert_eq!(ours, mesh);
}
