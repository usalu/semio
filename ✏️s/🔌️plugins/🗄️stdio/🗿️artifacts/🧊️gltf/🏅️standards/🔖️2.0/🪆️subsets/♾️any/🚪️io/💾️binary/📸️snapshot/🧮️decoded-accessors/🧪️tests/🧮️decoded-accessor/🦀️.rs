//! 🧮️ Neutral decoded geometry fixture verifies the physical-to-semantic inference boundary.
use super::*;
#[test]
fn decoded_inference_projection_preserves_values_and_reprojects_mutations() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🧬️schema/📸️snapshot/🧫️fixtures/🧮️decoded-accessor/🔣️.json")).unwrap();
    let mut snapshot=crate::standards::v2_0::subsets::any::schema::demo_gltf_snapshot();
    let bytes:Vec<u8>=fixture["bytes"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u8).collect();
    snapshot.buffers=vec![bytes];
    let mut events=Vec::new();
    let input=project_gltf_inference(&snapshot,&mut |at,total|{events.push([at,total]);true}).unwrap();
    let expected:Vec<f64>=fixture["components"].as_array().unwrap().iter().map(|value|value.as_f64().unwrap()).collect();
    assert_eq!(input.accessor(0).unwrap().components,expected);
    let progress:Vec<[usize;2]>=fixture["expectedProgress"].as_array().unwrap().iter().map(|value|[value[0].as_u64().unwrap() as usize,value[1].as_u64().unwrap() as usize]).collect();
    assert_eq!(events,progress);
    let result=crate::standards::v2_0::subsets::any::schema::inferences::compute_gltf_inference(&input);
    assert_eq!(result.counts.vertex_count,fixture["expectedVertexCount"].as_u64().unwrap());
    assert_eq!(result.counts.triangle_count,fixture["expectedTriangleCount"].as_u64().unwrap());
    drop(input);
    snapshot.buffers[0][0..4].copy_from_slice(&2f32.to_le_bytes());
    let changed=project_gltf_inference(&snapshot,&mut |_,_|true).unwrap();
    assert_eq!(changed.accessor(0).unwrap().components[0],2.0);
    assert!(project_gltf_inference(&snapshot,&mut |_,_|false).is_err());
    eprintln!("[DEBUG] glTF projection decoded {} values, reprojected changed bytes, and refused cancellation",expected.len());
}
