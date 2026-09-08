
use super::*;

fn assert_matches_oracle(mesh: &MeshData) {
    let ours = json::Value::from(mesh.clone());
    let oracle = serde_json::to_value(mesh).expect("oracle encode");
    let ours_text = json::to_string(&ours);
    let oracle_text = serde_json::to_string(&oracle).expect("oracle stringify");
    let ours_reparsed: serde_json::Value = serde_json::from_str(&ours_text).expect("reparse ours");
    assert_eq!(ours_reparsed, oracle, "first-party JSON diverged from serde_json:\n ours: {ours_text}\n serde: {oracle_text}");
}

#[test]
fn dense_mesh_matches_serde_json() {
    assert_matches_oracle(&mesh_box(1.0, 2.0, 3.0));
}

#[test]
fn default_mesh_omits_every_sparse_field() {
    let mesh = MeshData::default();
    assert_matches_oracle(&mesh);
    let ours = json::Value::from(mesh);
    for always in ["positions", "normals", "colors", "indices"] {
        assert!(ours.get(always).is_some(), "{always} must always be emitted");
    }
    for sparse in ["uvs", "faceIds", "vertexIds", "edgePositions", "edgeIds", "edgeUvs", "edgeIsSeam", "paintTextureBase64"] {
        assert!(ours.get(sparse).is_none(), "{sparse} must be omitted when empty");
    }
}

#[test]
fn populated_sparse_fields_are_emitted_camel_cased() {
    let mut mesh = mesh_box(1.0, 1.0, 1.0);
    mesh.uvs = vec![0.25, 0.5];
    mesh.face_ids = vec![7, 8];
    mesh.vertex_ids = vec![1];
    mesh.edge_positions = vec![0.0, 1.0, 2.0];
    mesh.edge_ids = vec![3];
    mesh.edge_uvs = vec![0.75];
    mesh.edge_is_seam = vec![1, 0];
    mesh.paint_texture_base64 = Some("abc".to_string());
    assert_matches_oracle(&mesh);
    let ours = json::Value::from(mesh);
    for sparse in ["uvs", "faceIds", "vertexIds", "edgePositions", "edgeIds", "edgeUvs", "edgeIsSeam", "paintTextureBase64"] {
        assert!(ours.get(sparse).is_some(), "{sparse} must be emitted when populated");
    }
}
