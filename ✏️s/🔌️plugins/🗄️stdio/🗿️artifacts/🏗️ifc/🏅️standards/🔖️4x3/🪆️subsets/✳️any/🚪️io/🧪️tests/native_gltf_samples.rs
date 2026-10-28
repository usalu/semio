use super::*;

#[test]
fn original_khronos_dense_triangles_keep_neutral_counts_and_ifc_world_bounds() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("gltf-samples/cases.json")).unwrap();
    for row in cases["cases"].as_array().unwrap() {
        let file=row["file"].as_str().unwrap();
        let bytes:&[u8]=match file {
            "Triangle.gltf"=>include_bytes!("gltf-samples/Triangle.gltf"),
            "TriangleWithoutIndices.gltf"=>include_bytes!("gltf-samples/TriangleWithoutIndices.gltf"),
            "BoxInterleaved.glb"=>include_bytes!("gltf-samples/BoxInterleaved.glb"),
            "NegativeScaleTest.glb"=>include_bytes!("gltf-samples/NegativeScaleTest.glb"),
            _=>panic!("unknown official sample"),
        };
        let source=if file.ends_with(".glb") {decode_glb_checked(bytes).unwrap()} else {gltf_io::import::parse_gltf_document(bytes).unwrap()};
        let graph=gltf_to_ifc_controlled(&source,&mut |_|true).unwrap().snapshot;
        let mut vertices=0usize;let mut triangles=0usize;let mut primitives=0usize;
        let mut low=[f64::INFINITY;3];let mut high=[f64::NEG_INFINITY;3];
        for entity in &graph.entities {
            if entity.name=="IFCCARTESIANPOINTLIST3D" {
                let rows=entity.args[0].as_aggregate().unwrap();vertices+=rows.len();
                for point in rows {let xyz=point.as_aggregate().unwrap();for k in 0..3 {let IfcValue::Real(value)=&xyz[k] else {panic!("real coordinate required")};low[k]=low[k].min(*value);high[k]=high[k].max(*value);}}
            } else if entity.name=="IFCTRIANGULATEDFACESET" {primitives+=1;triangles+=entity.args[3].as_aggregate().unwrap().len();}
        }
        assert_eq!(vertices as u64,row["vertices"].as_u64().unwrap(),"{file}");
        assert_eq!(triangles as u64,row["triangles"].as_u64().unwrap(),"{file}");
        assert_eq!(primitives as u64,row["primitives"].as_u64().unwrap(),"{file}");
        for k in 0..3 {assert_eq!(low[k],row["ifcBounds"][0][k].as_f64().unwrap(),"{file}");assert_eq!(high[k],row["ifcBounds"][1][k].as_f64().unwrap(),"{file}");}
        let physical=crate::io::write_ifc4x3_step(&graph).unwrap();
        let restored=crate::io::parse_ifc4x3_step(&physical).unwrap();
        assert_eq!(restored.entities,graph.entities,"{file}");
    }
}
