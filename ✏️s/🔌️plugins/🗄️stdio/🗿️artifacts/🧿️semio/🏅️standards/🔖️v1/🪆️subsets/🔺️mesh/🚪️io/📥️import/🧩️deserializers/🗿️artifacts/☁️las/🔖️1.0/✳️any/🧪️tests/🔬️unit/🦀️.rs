
use super::*;
use semio_s_artifact_stdio_las::schema::snapshot::{LasHeader, LasPoint};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_las() -> LasSnapshot {
    LasSnapshot {
        schema: "stdio.las".into(),
        header: LasHeader::default(),
        vlrs: Vec::new(),
        points: vec![
            LasPoint {
                x: 1.23,
                y: 4.56,
                z: 7.89,
                intensity: 100,
                return_number: 1,
                number_of_returns: 1,
                scan_direction_flag: false,
                edge_of_flight_line: false,
                classification: 2,
                scan_angle_rank: 0,
                user_data: 0,
                point_source_id: 0,
                gps_time: None,
                rgb: Some((65535, 0, 0)),
            },
            LasPoint {
                x: 2.34,
                y: 5.67,
                z: 8.90,
                intensity: 200,
                return_number: 1,
                number_of_returns: 1,
                scan_direction_flag: false,
                edge_of_flight_line: false,
                classification: 2,
                scan_angle_rank: 0,
                user_data: 0,
                point_source_id: 0,
                gps_time: None,
                rgb: Some((0, 65535, 0)),
            },
        ],
    }
}

#[semio_framework_async_macros::async_test]
async fn deserialize_maps_positions_and_uniform_rgb_as_points() {
    let semio = semio_framework_plugin::resolve_ready(SemioMeshFromLas::deserialize(&sample_las())).expect("deserialize");
    let prim = &semio.meshes[0].primitives[0];
    assert_eq!(prim.topology, SemioTopology::Points);
    assert_eq!(prim.positions.len(), 2);
    assert_eq!(prim.positions[0], SemioPoint3 { x: 1.23, y: 4.56, z: 7.89 });
    assert_eq!(prim.colors.len(), 2);
    assert_eq!(prim.colors[0], SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    assert!(prim.indices.is_empty() && prim.normals.is_empty() && prim.uvs.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn non_uniform_rgb_presence_drops_colors_rather_than_fabricating() {
    let mut las = sample_las();
    las.points[1].rgb = None;
    let semio = semio_framework_plugin::resolve_ready(SemioMeshFromLas::deserialize(&las)).expect("deserialize");
    assert!(semio.meshes[0].primitives[0].colors.is_empty());
}
