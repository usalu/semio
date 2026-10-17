//! 🖼️ Preview: 4 tests covering brep preview component reference tables refuse missing picking buffers; brep preview component ranges refuse incomplete or overlapping groups; brep merged preview preserves original component ranges; a…
use super::*;


/// 🛂️ Analytic reference tables cannot claim geometry without matching picking buffers.
#[test]
fn brep_preview_component_reference_tables_refuse_missing_picking_buffers() {
    use semio_framework_value::FromValue;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🎯️component-picking/🔣️.json")).unwrap();
    for row in fixture["referenceRefusals"].as_array().unwrap() {
        let input=serde_json::json!({"componentReferences":row["references"]});
        let input=semio_framework_pack_json::parse(&input.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert!(semio_framework_mesh_engine::MeshData::from_value(semio_framework_pack_json::to_dsl_value(&input)).is_err(),"{}",row["id"]);
    }
}


/// 🚧️ Malformed analytic ranges are refused before allocating picking buffers.
#[test]
fn brep_preview_component_ranges_refuse_incomplete_or_overlapping_groups() {
    use semio_framework_value::FromValue;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🎯️component-picking/🔣️.json")).unwrap();
    for row in fixture["refusals"].as_array().unwrap() {
        let mut source=fixture["transfer"].clone();
        source[format!("{}_groups",row["domain"].as_str().unwrap())][row["row"].as_u64().unwrap() as usize][row["field"].as_str().unwrap()]=row["value"].clone();
        let source=semio_framework_pack_json::parse(&source.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        if let Ok(transfer)=MeshTransfer::from_value(semio_framework_pack_json::to_dsl_value(&source)) {assert!(mesh_data_from_mesh_transfer(&transfer).is_err());}
    }
}


/// 🔗️ Merged preview topology resolves the same neutral exact component references.
#[test]
fn brep_merged_preview_preserves_original_component_ranges() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🔗️mesh-transfer-merge/🔣️.json")).unwrap();
    let wire = semio_framework_pack_json::parse(&fixture["merged"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let transfer = <MeshTransfer as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&wire)).unwrap();
    let mesh = mesh_data_from_mesh_transfer(&transfer).unwrap();
    assert_eq!(serde_json::json!(mesh.face_ids),fixture["expected"]["faceIds"]);
    assert_eq!(serde_json::json!(mesh.edge_ids),fixture["expected"]["edgeIds"]);
    assert_eq!(serde_json::json!(mesh.vertex_ids),fixture["expected"]["vertexIds"]);
    let encoded: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::Value::from(mesh.clone()).to_string()).unwrap();
    assert_eq!(encoded["componentReferences"],fixture["expected"]["componentReferences"]);
    let area: f32 = mesh.indices.chunks_exact(3).map(|indices| {
        let points = [indices[0],indices[1],indices[2]].map(|index| {let at=index as usize*3;parry3d::math::Point::new(mesh.positions[at],mesh.positions[at+1],mesh.positions[at+2])});
        parry3d::shape::Triangle::new(points[0],points[1],points[2]).area()
    }).sum();
    assert_eq!(area as f64,fixture["expected"]["area"].as_f64().unwrap());
}


/// 🚫️ Original vertex ranges obey the same neutral refusal contract before mesh publication.
#[test]
fn brep_original_vertex_ranges_refuse_malformed_coverage_and_labels() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🎯️vertex-provenance/🔣️.json")).unwrap();
    let points = fixture["cases"][0]["points"].as_array().unwrap();
    let raw = serde_json::json!({"position":[0,0,0,1,0,0,0,1,0],"normal":[0,0,1,0,0,1,0,0,1],"index":[0,1,2],"edges":[],"points":points.iter().flat_map(|point|point.as_array().unwrap().iter().cloned()).collect::<Vec<_>>(),"face_groups":[],"edge_groups":[],"vertex_groups":points.iter().enumerate().map(|(index,_)|serde_json::json!({"start":index,"count":1,"entity_id":(index+1).to_string()})).collect::<Vec<_>>()});
    for refusal in fixture["refusals"].as_array().unwrap() {
        let mut source = raw.clone();
        source["vertex_groups"][refusal["row"].as_u64().unwrap() as usize][refusal["field"].as_str().unwrap()] = refusal["value"].clone();
        let source = semio_framework_pack_json::parse(&source.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        if let Ok(transfer) = <MeshTransfer as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&source)) { assert!(mesh_data_from_mesh_transfer(&transfer).is_err(),"refusal {refusal}"); }
    }
}
