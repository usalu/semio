//! 🔮️ Complete portable binary64 corpus runs through the original neutral geometry algorithms.

use super::*;
use serde_json::{json, Value};
use std::time::{Duration, Instant};

#[test]
fn complete_portable_region_surface_corpus() {
    let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let rows = fixture["vectors"].as_array().unwrap();
    assert_eq!(rows.len(), 9);
    let started = Instant::now();
    let mut observed = Vec::new();
    for vector in rows {
        assert!(started.elapsed() < Duration::from_secs(60));
        let inputs = vector["regions"].as_array().unwrap();
        assert!(inputs.len() <= 128);
        let mut regions = Vec::new();
        for input in inputs {
            assert!(started.elapsed() < Duration::from_secs(60));
            let outer: Vec<[f64; 2]> = serde_json::from_value(input["domain"]["outer"].clone()).unwrap();
            let holes: Vec<Vec<[f64; 2]>> = serde_json::from_value(input["domain"]["holes"].clone()).unwrap();
            assert!(outer.len() <= 128 && holes.len() <= 32 && holes.iter().all(|hole| hole.len() <= 128));
            let domain = PlanarDomain { outer, holes };
            let options = MeshOpts { max_edge: input["options"]["maxEdge"].as_f64().unwrap(), min_angle_deg: input["options"]["minAngleDeg"].as_f64().unwrap() };
            let Ok(triangulated) = triangulate(&domain, &options) else { continue };
            let extruded = extrude_tri_mesh(&triangulated, input["thickness"].as_f64().unwrap(), 1);
            let volume = split_to_tets(&extruded);
            let triangles = boundary_faces(&volume);
            let tetrahedra: Vec<[u32; 4]> = volume.cells.iter().map(|cell| match cell { Cell::Tet4(tet) => *tet, _ => panic!("split_to_tets retained a non-tetrahedral cell") }).collect();
            assert!(volume.points.len() <= 65_536 && tetrahedra.len() <= 65_536 && triangles.len() <= 65_536);
            let expected = vector["expected"].as_array().unwrap().iter().find(|row| row["regionId"] == input["regionId"]).unwrap();
            let mut bounds = [[f64::INFINITY; 3], [f64::NEG_INFINITY; 3]];
            for point in &volume.points {
                assert!(point.iter().all(|value| value.is_finite()));
                for axis in 0..3 { bounds[0][axis] = bounds[0][axis].min(point[axis]); bounds[1][axis] = bounds[1][axis].max(point[axis]); }
            }
            assert_eq!(bounds, serde_json::from_value::<[[f64; 3]; 2]>(expected["bounds"].clone()).unwrap());
            let expected_bits: [[String; 3]; 2] = serde_json::from_value(expected["boundsBits"].clone()).unwrap();
            for side in 0..2 { for axis in 0..3 { assert_eq!(format!("{:016x}", bounds[side][axis].to_bits()), expected_bits[side][axis]); } }
            for point in &domain.outer { assert!(volume.points.iter().any(|position| position[0].to_bits() == point[0].to_bits() && position[1].to_bits() == point[1].to_bits())); }
            assert!(triangles.len() >= expected["minimumTriangles"].as_u64().unwrap() as usize);
            assert!(tetrahedra.iter().flatten().all(|index| (*index as usize) < volume.points.len()));
            assert!(triangles.iter().flatten().all(|index| (*index as usize) < volume.points.len()));
            regions.push(json!({"regionId":input["regionId"],"points":volume.points,"tetrahedra":tetrahedra,"triangles":triangles}));
        }
        assert_eq!(regions.iter().map(|row| row["regionId"].clone()).collect::<Vec<_>>(), vector["expected"].as_array().unwrap().iter().map(|row| row["regionId"].clone()).collect::<Vec<_>>());
        observed.push(json!({"id":vector["id"],"neutral":{"regions":regions}}));
        println!("[DEBUG] Real neutral region surface vector {} retained binary64 bounds and identities", vector["id"]);
    }
    let path = std::env::var_os("SEMIO_FEM_NEUTRAL_RESULTS").expect("owning neutral corpus command must declare its output path");
    let bytes = serde_json::to_vec(&observed).unwrap();
    assert!(bytes.len() <= 1_048_576);
    let path = std::path::Path::new(&path);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, bytes).unwrap();
    println!("[DEBUG] Complete nine-vector neutral receiver wrote its explicit bounded artifact");
}
