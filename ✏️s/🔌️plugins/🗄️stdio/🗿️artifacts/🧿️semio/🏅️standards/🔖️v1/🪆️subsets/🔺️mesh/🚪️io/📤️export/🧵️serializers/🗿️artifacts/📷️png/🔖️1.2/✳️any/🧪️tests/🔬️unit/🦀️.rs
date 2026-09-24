use super::*;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMesh;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn unit_cube() -> SemioMeshSnapshot {
    let positions: Vec<SemioPoint3> = (0..8).map(|i| SemioPoint3 { x: (i & 1) as f64, y: ((i >> 1) & 1) as f64, z: ((i >> 2) & 1) as f64 }).collect();
    let quads = [[0, 2, 3, 1], [4, 5, 7, 6], [0, 1, 5, 4], [2, 6, 7, 3], [0, 4, 6, 2], [1, 3, 7, 5]];
    let indices = quads.iter().flat_map(|q| [q[0], q[1], q[2], q[0], q[2], q[3]]).collect();
    SemioMeshSnapshot {
        meshes: vec![SemioMesh { id: "cube".into(), primitives: vec![SemioPrimitive { id: "cube".into(), topology: SemioTopology::Triangles, positions, normals: Vec::new(), uvs: Vec::new(), colors: Vec::new(), indices, material_id: None }] }],
        ..SemioMeshSnapshot::default()
    }
}

/// 📐️ Analytic oracle: a unit cube seen along its diagonal is a regular hexagon of area √3 whose
/// height is 2·√(2/3), so the fitted silhouette covers √3·s² pixels with s = (edge − 2·margin)/height.
#[test]
fn a_unit_cube_projects_to_the_analytic_hexagon() {
    let png = semio_framework_plugin::resolve_ready(SemioMeshToPng::serialize(&unit_cube())).expect("png");
    assert_eq!((png.width, png.height), (512, 512));
    let painted = png.pixels.chunks(4).map(|px| 1.0 - (px[0] as f64 + px[1] as f64 + px[2] as f64) / 765.0).map(|darkness| if darkness > 0.05 { 1.0 } else { 0.0 }).sum::<f64>();
    let scale = (MESH_VIEW_EDGE - 2.0 * MESH_VIEW_MARGIN) / (2.0 * (2.0f64 / 3.0).sqrt());
    let expected = 3.0f64.sqrt() * scale * scale;
    assert!((painted - expected).abs() / expected < 0.01, "painted {painted} vs analytic {expected}");
}

#[test]
fn three_visible_faces_get_three_distinct_shades() {
    let drawing = mesh_view_drawing(&unit_cube()).expect("view");
    assert_eq!(drawing.styles.len(), 3);
    let DrawNode::Group { children, .. } = &drawing.layers[0].root else { panic!("group root") };
    let front: Vec<&str> = children[children.len() - 6..].iter().filter_map(|node| if let DrawNode::Path { style: Some(style), .. } = node { Some(style.as_str()) } else { None }).collect();
    let distinct: std::collections::BTreeSet<&str> = front.iter().copied().collect();
    assert_eq!(distinct.len(), 3);
}

#[test]
fn strips_fans_and_indices_expand_to_the_same_triangles() {
    let quad = |topology: SemioTopology, indices: Vec<u32>| SemioPrimitive { id: "q".into(), topology, positions: vec![SemioPoint3::default(); 4], normals: Vec::new(), uvs: Vec::new(), colors: Vec::new(), indices, material_id: None };
    assert_eq!(primitive_triangles(&quad(SemioTopology::TriangleStrip, Vec::new())), vec![[0, 1, 2], [2, 1, 3]]);
    assert_eq!(primitive_triangles(&quad(SemioTopology::TriangleFan, Vec::new())), vec![[0, 1, 2], [0, 2, 3]]);
    assert_eq!(primitive_triangles(&quad(SemioTopology::Triangles, vec![0, 1, 2, 2, 3, 9])), vec![[0, 1, 2]]);
}

#[test]
fn an_empty_mesh_is_refused() {
    assert!(mesh_view_drawing(&SemioMeshSnapshot::default()).is_err());
}
