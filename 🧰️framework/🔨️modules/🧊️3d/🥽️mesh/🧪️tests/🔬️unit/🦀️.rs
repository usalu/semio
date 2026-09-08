
use super::*;

/// 🧬️ Additive `#[derive(ToValue, FromValue)]` round-trip (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
/// 26/09/01): `FromValue(ToValue(x)) == x` for the transparent newtypes, the `default`-field
/// structs, and the `u32_hashset_bridge`-covered `HalfedgeMesh` (a real, non-empty mesh, so
/// `vertices`/`halfedges`/`faces`/`uv_seams` are all actually exercised).
#[test]
fn value_round_trip_matches_serde_shape() {
    fn check<T: dsl_core::value::ToValue + dsl_core::value::FromValue + std::fmt::Debug + PartialEq>(value: T) {
        let round_tripped = <T as dsl_core::value::FromValue>::from_value(dsl_core::value::ToValue::to_value(&value)).expect("round-trip decode");
        assert_eq!(round_tripped, value);
    }

    check(Vec3::new(1.0, 2.0, 3.0));
    check(VertexId(7));
    check(WeldMode::ByDistance);
    check(MeshKernelError::InvalidInput("bad".to_string()));

    // `HalfedgeMesh` has no `PartialEq` (pre-existing — not added here), so round-trip fidelity
    // is checked by re-encoding the decoded mesh and comparing `DslValue`s, plus a direct
    // comparison against `serde_json`'s own encoding of the SAME mesh — the round-trip
    // contract's actual bar (byte-identical wire shape), not just "decodes to something".
    let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).expect("box_prim");
    let encoded = dsl_core::value::ToValue::to_value(&mesh);
    let decoded = <HalfedgeMesh as dsl_core::value::FromValue>::from_value(encoded.clone()).expect("round-trip decode");
    assert_eq!(dsl_core::value::ToValue::to_value(&decoded), encoded);

    let via_serde: serde_json::Value = serde_json::to_value(&mesh).expect("serde encode");
    assert_eq!(serde_json::Value::from(encoded), via_serde);
}

#[test]
fn public_kernel_api_is_synchronous() {
    let _: fn(f32, f32, f32) -> Vec3 = Vec3::new;
    let _: fn(f32, f32, f32) -> MeshResult<HalfedgeMesh> = HalfedgeMesh::box_prim;
    let _: fn(&str) -> MeshResult<HalfedgeMesh> = HalfedgeMesh::from_json;
    let _: fn(&HalfedgeMesh) -> MeshResult<String> = HalfedgeMesh::to_json;
    let _: fn(&mut HalfedgeMesh, Vec3) -> MeshResult<()> = HalfedgeMesh::translate;
}

#[semio_framework_async_macros::async_test]
async fn box_prim_has_six_faces() {
    let mesh = HalfedgeMesh::box_prim(2.0, 2.0, 2.0).unwrap();
    assert_eq!(mesh.face_count(), 6);
    assert_eq!(mesh.vertex_count(), 8);
}

#[semio_framework_async_macros::async_test]
async fn plane_prim_single_face() {
    let mesh = HalfedgeMesh::plane_prim(4.0, 4.0).unwrap();
    assert_eq!(mesh.face_count(), 1);
}

#[semio_framework_async_macros::async_test]
async fn translate_moves_vertices() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    mesh.translate(Vec3::new(1.0, 0.0, 0.0)).unwrap();
    let p = mesh.vertex_position(VertexId(0)).unwrap();
    assert!((p.x() - 0.5).abs() < 1e-5);
}

#[semio_framework_async_macros::async_test]
async fn from_indexed_triangles_builds_triangle_faces() {
    let positions = vec![
        0.0, 0.0, 0.0, //
        1.0, 0.0, 0.0, //
        0.0, 1.0, 0.0, //
    ];
    let indices = vec![0, 1, 2];
    let mesh = HalfedgeMesh::from_indexed_triangles(&positions, &indices).unwrap();
    assert_eq!(mesh.vertex_count(), 3);
    assert_eq!(mesh.face_count(), 1);
}

#[semio_framework_async_macros::async_test]
async fn from_indexed_triangles_by_face_id_merges_per_brep_face_without_filling_holes() {
    // Two quads on z=0 sharing no edge (a slab with a gap — like a face pair that must not be bridged),
    // plus a third vertical face that should stay separate. Face ids: 1 covers both coplanar quads'
    // triangles as two separate B-Rep faces (10 and 11), so the gap is never capped.
    let positions = vec![
        0.0, 0.0, 0.0, // 0
        1.0, 0.0, 0.0, // 1
        1.0, 1.0, 0.0, // 2
        0.0, 1.0, 0.0, // 3
        2.0, 0.0, 0.0, // 4
        3.0, 0.0, 0.0, // 5
        3.0, 1.0, 0.0, // 6
        2.0, 1.0, 0.0, // 7
        0.0, 0.0, 1.0, // 8
        1.0, 0.0, 1.0, // 9
    ];
    let indices = vec![
        0, 1, 2, 0, 2, 3, // face 10 — left quad
        4, 5, 6, 4, 6, 7, // face 11 — right quad (gap between x=1 and x=2)
        0, 1, 9, 0, 9, 8, // face 12 — vertical
    ];
    let face_ids = vec![10, 10, 11, 11, 12, 12];
    let mut mesh = HalfedgeMesh::from_indexed_triangles_by_face_id(&positions, &indices, &face_ids).unwrap();
    assert_eq!(mesh.face_count(), 3, "each B-Rep face becomes one n-gon; gap must not be filled");
    mesh.weld_coincident_vertices(1e-6).unwrap();
    let open = {
        let mut directed: HashMap<(u32, u32), u32> = HashMap::new();
        for fi in 0..mesh.face_count() {
            let verts = mesh.face_vertex_ids(FaceId(fi as u32)).unwrap();
            let n = verts.len();
            for i in 0..n {
                *directed.entry((verts[i].0, verts[(i + 1) % n].0)).or_insert(0) += 1;
            }
        }
        directed.keys().filter(|&&(a, b)| !directed.contains_key(&(b, a))).count()
    };
    assert!(open > 0, "gap between the two quads must remain an open boundary, not a filled face");
}

#[semio_framework_async_macros::async_test]
async fn orient_faces_consistently_fixes_same_winding_neighbors() {
    // Two quads sharing edge 1-2, both wound CCW in XY — shared edge has the same directed sense.
    let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [2.0, 0.0, 0.0], [2.0, 1.0, 0.0]];
    let faces = vec![vec![0, 1, 2, 3], vec![1, 2, 5, 4]];
    let mut mesh = HalfedgeMesh::from_faces(&positions, &faces).unwrap();
    let open_before = {
        let mut directed: HashMap<(u32, u32), u32> = HashMap::new();
        for fi in 0..mesh.face_count() {
            let verts = mesh.face_vertex_ids(FaceId(fi as u32)).unwrap();
            let n = verts.len();
            for i in 0..n {
                *directed.entry((verts[i].0, verts[(i + 1) % n].0)).or_insert(0) += 1;
            }
        }
        directed.keys().filter(|&&(a, b)| !directed.contains_key(&(b, a))).count()
    };
    assert!(open_before > 0);
    let flips = mesh.orient_faces_consistently().unwrap();
    assert!(flips >= 1);
    let open_after = {
        let mut directed: HashMap<(u32, u32), u32> = HashMap::new();
        for fi in 0..mesh.face_count() {
            let verts = mesh.face_vertex_ids(FaceId(fi as u32)).unwrap();
            let n = verts.len();
            for i in 0..n {
                *directed.entry((verts[i].0, verts[(i + 1) % n].0)).or_insert(0) += 1;
            }
        }
        directed.keys().filter(|&&(a, b)| !directed.contains_key(&(b, a))).count()
    };
    assert_eq!(open_after, 6, "only the outer boundary of the 2-quad strip should remain open");
}

#[semio_framework_async_macros::async_test]
async fn triangulate_produces_triangles_only() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    mesh.triangulate().unwrap();
    for fi in 0..mesh.face_count() {
        let verts = mesh.face_vertex_ids(FaceId(fi as u32)).unwrap();
        assert_eq!(verts.len(), 3);
    }
}

#[semio_framework_async_macros::async_test]
async fn extrude_increases_face_count() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    let before = mesh.face_count();
    mesh.extrude_faces(&[FaceId(0)], 0.5).unwrap();
    assert!(mesh.face_count() > before);
}

#[semio_framework_async_macros::async_test]
async fn tessellate_has_positions_and_indices() {
    let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    let transfer = mesh.tessellate().unwrap();
    assert!(!transfer.positions.is_empty());
    assert!(!transfer.indices.is_empty());
    assert!(!transfer.edge_positions.is_empty());
    assert_eq!(transfer.face_ids.len(), transfer.indices.len() / 3);
    assert_eq!(transfer.vertex_ids.len(), transfer.positions.len() / 3);
    assert_eq!(transfer.edge_ids.len() * 2, transfer.edge_positions.len() / 3);
    assert_eq!(transfer.uvs.len(), transfer.positions.len() / 3 * 2);
    assert_eq!(transfer.edge_uvs.len(), transfer.edge_ids.len() * 4);
    assert_eq!(transfer.edge_is_seam.len(), transfer.edge_ids.len());
}

#[semio_framework_async_macros::async_test]
async fn flip_faces_reverses_only_requested_normals() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    let before = mesh.face_normal(FaceId(0)).unwrap();
    let edge_ids = mesh.tessellate().unwrap().edge_ids;
    mesh.flip_faces(&[FaceId(0)]).unwrap();
    let after = mesh.face_normal(FaceId(0)).unwrap();
    assert!(before.dot(after) < -0.99);
    assert_eq!(mesh.tessellate().unwrap().edge_ids, edge_ids);
}

#[semio_framework_async_macros::async_test]
async fn unwrap_uv_produces_bounded_coordinates() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    mesh.unwrap_uv().unwrap();
    let transfer = mesh.tessellate().unwrap();
    assert!(!transfer.uvs.is_empty());
    for chunk in transfer.uvs.chunks(2) {
        assert!(chunk[0].is_finite());
        assert!(chunk[1].is_finite());
        assert!(chunk[0] >= -0.01 && chunk[0] <= 1.01);
        assert!(chunk[1] >= -0.01 && chunk[1] <= 1.01);
    }
}

#[semio_framework_async_macros::async_test]
async fn obj_export_contains_vertices() {
    let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    let obj = mesh.to_obj().unwrap();
    assert!(obj.contains("v "));
    assert!(obj.contains("f "));
}

#[semio_framework_async_macros::async_test]
async fn ico_sphere_has_faces() {
    let mesh = HalfedgeMesh::ico_sphere_prim(1.0, 1).unwrap();
    assert!(mesh.face_count() > 20);
}

#[semio_framework_async_macros::async_test]
async fn decimate_reduces_vertices() {
    let mut mesh = HalfedgeMesh::ico_sphere_prim(1.0, 2).unwrap();
    let before = mesh.vertex_count();
    mesh.decimate(0.5).unwrap();
    assert!(mesh.vertex_count() <= before);
}

#[semio_framework_async_macros::async_test]
async fn json_roundtrip() {
    let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    let json = mesh.to_json().unwrap();
    let restored = HalfedgeMesh::from_json(&json).unwrap();
    assert_eq!(restored.vertex_count(), mesh.vertex_count());
}

#[semio_framework_async_macros::async_test]
async fn newell_normal_handles_collinear_first_corner() {
    // First three points (0,0,0)-(1,0,0)-(2,0,0) are collinear: the old first-triangle-cross
    // method degenerates to a zero vector here, Newell's method must not.
    let positions = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0], [2.0, 1.0, 0.0]];
    let mesh = HalfedgeMesh::from_faces(&positions, &[vec![0, 1, 2, 3]]).unwrap();
    let normal = mesh.face_normal(FaceId(0)).unwrap();
    assert!(normal.length() > 0.99 && normal.length() < 1.01);
    assert!(normal.dot(Vec3::new(0.0, 0.0, 1.0)).abs() > 0.99);
}

#[semio_framework_async_macros::async_test]
async fn ear_clipping_triangulates_concave_l_polygon() {
    // Concave L-shaped hexagon.
    let corners = [(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)];
    let mut positions: Vec<Vec3> = Vec::with_capacity(corners.len());
    for &(x, y) in &corners {
        positions.push(Vec3::new(x, y, 0.0));
    }
    let triangles = triangulate_polygon(&positions);
    assert_eq!(triangles.len(), corners.len() - 2);
    let shoelace = |pts: &[(f64, f64)]| -> f64 {
        let n = pts.len();
        let mut sum = 0.0;
        for i in 0..n {
            let (x0, y0) = pts[i];
            let (x1, y1) = pts[(i + 1) % n];
            sum += x0 * y1 - x1 * y0;
        }
        sum.abs() * 0.5
    };
    let polygon_area = shoelace(&corners.iter().map(|&(x, y)| (x as f64, y as f64)).collect::<Vec<_>>());
    let mut triangle_area_sum = 0.0f64;
    for tri in &triangles {
        let pts: Vec<(f64, f64)> = tri.iter().map(|&i| (corners[i].0 as f64, corners[i].1 as f64)).collect();
        triangle_area_sum += shoelace(&pts);
        // Nondegenerate.
        assert!(shoelace(&pts) > 1e-6);
    }
    assert!((triangle_area_sum - polygon_area).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn merge_coplanar_faces_reassembles_triangulated_cube_into_quads() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    mesh.triangulate().unwrap();
    assert_eq!(mesh.face_count(), 12);
    let merges = mesh.merge_coplanar_faces().unwrap();
    assert_eq!(merges, 6);
    assert_eq!(mesh.face_count(), 6);
    for fi in 0..mesh.face_count() {
        let verts = mesh.face_vertex_ids(FaceId(fi as u32)).unwrap();
        assert_eq!(verts.len(), 4);
    }
}

#[semio_framework_async_macros::async_test]
async fn dissolve_edges_merges_two_triangles_into_a_quad() {
    let positions = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]];
    let faces = vec![vec![0, 1, 2], vec![0, 2, 3]];
    let mut mesh = HalfedgeMesh::from_faces(&positions, &faces).unwrap();
    assert_eq!(mesh.face_count(), 2);
    // Halfedge index 2 is face 0's edge 2->0, the shared diagonal.
    mesh.dissolve_edges(&[EdgeId(2)]).unwrap();
    assert_eq!(mesh.face_count(), 1);
    let verts = mesh.face_vertex_ids(FaceId(0)).unwrap();
    assert_eq!(verts.len(), 4);
}

#[semio_framework_async_macros::async_test]
async fn merge_and_cleanup_collapses_seam_vertices_of_a_contiguous_strip() {
    // Three coplanar quads in a row sharing seam edges; the seam vertices are used by exactly
    // two faces each and lie on straight boundary lines, so they must be dropped after merge.
    let positions = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [2.0, 0.0, 0.0], [2.0, 1.0, 0.0], [3.0, 0.0, 0.0], [3.0, 1.0, 0.0]];
    let faces = vec![vec![0, 1, 2, 3], vec![1, 4, 5, 2], vec![4, 6, 7, 5]];
    let mut mesh = HalfedgeMesh::from_faces(&positions, &faces).unwrap();
    assert_eq!(mesh.face_count(), 3);
    let merges = mesh.merge_coplanar_faces().unwrap();
    assert_eq!(merges, 2);
    assert_eq!(mesh.face_count(), 1);
    let verts = mesh.face_vertex_ids(FaceId(0)).unwrap();
    assert_eq!(verts.len(), 4, "seam vertices 1,2,4,5 should have been dropped as collinear");
}

#[semio_framework_async_macros::async_test]
async fn tessellate_concave_face_tags_all_triangles_with_one_face_id() {
    let corners = [(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)];
    let positions: Vec<[f32; 3]> = corners.iter().map(|&(x, y)| [x, y, 0.0]).collect();
    let mesh = HalfedgeMesh::from_faces(&positions, &[(0..6).collect()]).unwrap();
    let transfer = mesh.tessellate().unwrap();
    assert_eq!(transfer.face_ids.len(), corners.len() - 2);
    assert!(transfer.face_ids.iter().all(|&id| id == 0));
    assert_eq!(transfer.indices.len(), (corners.len() - 2) * 3);
}

#[semio_framework_async_macros::async_test]
async fn weld_coincident_vertices_unifies_independently_tessellated_seam() {
    // Two quads sharing an edge but built with DUPLICATE (non-shared) vertices at that seam, as an
    // importer would produce when tessellating adjacent source faces independently.
    let positions = vec![
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0], // seam vertex, quad A's copy
        [1.0, 1.0, 0.0], // seam vertex, quad A's copy
        [0.0, 1.0, 0.0],
        [1.0, 0.0, 0.0], // seam vertex, quad B's copy (duplicate position, different id)
        [1.0, 1.0, 0.0], // seam vertex, quad B's copy (duplicate position, different id)
        [2.0, 0.0, 0.0],
        [2.0, 1.0, 0.0],
    ];
    let faces = vec![vec![0, 1, 2, 3], vec![4, 6, 7, 5]];
    let mut mesh = HalfedgeMesh::from_faces(&positions, &faces).unwrap();
    assert_eq!(mesh.vertex_count(), 8);
    let removed = mesh.weld_coincident_vertices(1e-4).unwrap();
    assert_eq!(removed, 2);
    assert_eq!(mesh.vertex_count(), 6);
    let merges = mesh.merge_coplanar_faces().unwrap();
    assert_eq!(merges, 1, "welding should have made the seam mergeable");
    assert_eq!(mesh.face_count(), 1);
}

#[semio_framework_async_macros::async_test]
async fn fill_holes_caps_a_missing_box_face() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    // Remove one face by rebuilding without it.
    let (positions, mut face_list) = mesh.polygon_soup();
    face_list.remove(0);
    assert_eq!(face_list.len(), 5);
    mesh = HalfedgeMesh::from_faces(&positions, &face_list).unwrap();
    let filled = mesh.fill_holes().unwrap();
    assert_eq!(filled, 1);
    assert_eq!(mesh.face_count(), 6);
    // Verify the resulting mesh is watertight: every directed boundary edge (position-keyed) now has
    // an opposite-winding counterpart.
    let mut directed: HashMap<(u32, u32), u32> = HashMap::new();
    for fi in 0..mesh.face_count() {
        let verts = mesh.face_vertex_ids(FaceId(fi as u32)).unwrap();
        let n = verts.len();
        for i in 0..n {
            *directed.entry((verts[i].0, verts[(i + 1) % n].0)).or_insert(0) += 1;
        }
    }
    for &(a, b) in directed.keys() {
        assert!(directed.contains_key(&(b, a)), "edge {a}->{b} has no opposite counterpart after fill_holes");
    }
}

#[semio_framework_async_macros::async_test]
async fn fill_holes_disambiguates_two_holes_sharing_one_vertex() {
    // 3x3 grid of vertices forming a 2x2 grid of quads; keep only the two DIAGONAL quads, so the two
    // missing (diagonally opposite) quads are separate holes that touch at exactly the shared center
    // vertex (index 4). A vertex-only "next" map cannot disambiguate this; proper halfedge rotation can.
    let mut positions = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            positions.push([i as f32, j as f32, 0.0]);
        }
    }
    let idx = |i: usize, j: usize| (i * 3 + j) as u32;
    let q00 = vec![idx(0, 0), idx(1, 0), idx(1, 1), idx(0, 1)];
    let q11 = vec![idx(1, 1), idx(2, 1), idx(2, 2), idx(1, 2)];
    let mut mesh = HalfedgeMesh::from_faces(&positions, &[q00, q11]).unwrap();
    assert_eq!(mesh.face_count(), 2);
    let filled = mesh.fill_holes().unwrap();
    assert_eq!(filled, 2, "expected both diagonal holes to be found and capped separately");
    assert_eq!(mesh.face_count(), 4);
    for fi in 0..mesh.face_count() {
        assert_eq!(mesh.face_vertex_ids(FaceId(fi as u32)).unwrap().len(), 4);
    }
}

#[semio_framework_async_macros::async_test]
async fn vec3_dot_cross_length_lerp() {
    let a = Vec3::new(1.0, 0.0, 0.0);
    let b = Vec3::new(0.0, 1.0, 0.0);
    assert!(a.dot(b).abs() < 1e-6);
    assert!((a.cross(b).z() - 1.0).abs() < 1e-6);
    assert!((Vec3::new(3.0, 4.0, 0.0).length() - 5.0).abs() < 1e-6);
    let mid = a.lerp(b, 0.5);
    assert!((mid.x() - 0.5).abs() < 1e-6 && (mid.y() - 0.5).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn vec3_normalize_zero_vector_returns_zero() {
    assert_eq!(Vec3::ZERO.normalize(), Vec3::ZERO);
    let tiny = Vec3::new(1e-9, 0.0, 0.0);
    assert_eq!(tiny.normalize(), Vec3::ZERO);
}

#[semio_framework_async_macros::async_test]
async fn vertex_position_invalid_handle_returns_err() {
    let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    assert_eq!(mesh.vertex_position(VertexId(999)), Err(MeshKernelError::InvalidHandle));
}

#[semio_framework_async_macros::async_test]
async fn set_vertex_position_updates_and_rejects_invalid_handle() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    mesh.set_vertex_position(VertexId(0), Vec3::new(9.0, 9.0, 9.0)).unwrap();
    assert_eq!(mesh.vertex_position(VertexId(0)).unwrap(), Vec3::new(9.0, 9.0, 9.0));
    assert_eq!(mesh.set_vertex_position(VertexId(999), Vec3::ZERO), Err(MeshKernelError::InvalidHandle));
}

#[semio_framework_async_macros::async_test]
async fn edge_endpoints_returns_ordered_vertices() {
    let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    let (v0, v1) = mesh.edge_endpoints(EdgeId(0)).unwrap();
    assert_ne!(v0, v1);
    assert_eq!(mesh.edge_endpoints(EdgeId(9999)), Err(MeshKernelError::InvalidHandle));
}

#[semio_framework_async_macros::async_test]
async fn face_vertex_ids_invalid_handle_returns_err() {
    let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    assert_eq!(mesh.face_vertex_ids(FaceId(999)), Err(MeshKernelError::InvalidHandle));
}

#[semio_framework_async_macros::async_test]
async fn flip_faces_rejects_empty_selection_and_invalid_handle() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    assert_eq!(mesh.flip_faces(&[]), Err(MeshKernelError::EmptySelection));
    assert_eq!(mesh.flip_faces(&[FaceId(999)]), Err(MeshKernelError::InvalidHandle));
}

#[semio_framework_async_macros::async_test]
async fn from_indexed_triangles_rejects_malformed_lengths() {
    assert!(matches!(HalfedgeMesh::from_indexed_triangles(&[0.0, 0.0], &[0, 1, 2]), Err(MeshKernelError::InvalidInput(_))));
    assert!(matches!(HalfedgeMesh::from_indexed_triangles(&[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], &[0, 1]), Err(MeshKernelError::InvalidInput(_))));
}

#[semio_framework_async_macros::async_test]
async fn from_indexed_triangles_by_face_id_falls_back_when_empty() {
    let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
    let indices = vec![0, 1, 2];
    let mesh = HalfedgeMesh::from_indexed_triangles_by_face_id(&positions, &indices, &[]).unwrap();
    assert_eq!(mesh.face_count(), 1);
}

#[semio_framework_async_macros::async_test]
async fn from_indexed_triangles_by_face_id_rejects_length_mismatch() {
    let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
    let indices = vec![0, 1, 2];
    let err = HalfedgeMesh::from_indexed_triangles_by_face_id(&positions, &indices, &[10, 11]).unwrap_err();
    assert!(matches!(err, MeshKernelError::InvalidInput(_)));
}

#[semio_framework_async_macros::async_test]
async fn from_faces_rejects_degenerate_and_out_of_range() {
    let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
    assert!(matches!(HalfedgeMesh::from_faces(&positions, &[vec![0, 1]]), Err(MeshKernelError::DegenerateOperation)));
    assert!(matches!(HalfedgeMesh::from_faces(&positions, &[vec![0, 1, 99]]), Err(MeshKernelError::InvalidInput(_))));
}

#[semio_framework_async_macros::async_test]
async fn from_face_loops_bridges_hole_and_skips_degenerate_outer() {
    let mut positions = vec![[0.0, 0.0, 0.0], [4.0, 0.0, 0.0], [4.0, 4.0, 0.0], [0.0, 4.0, 0.0]];
    positions.extend_from_slice(&[[1.0, 1.0, 0.0], [3.0, 1.0, 0.0], [3.0, 3.0, 0.0], [1.0, 3.0, 0.0]]);
    let outer = vec![0, 1, 2, 3];
    let hole = vec![4, 5, 6, 7];
    let face_loops = vec![(outer, vec![hole]), (vec![0, 1], vec![])];
    let mesh = HalfedgeMesh::from_face_loops(&positions, &face_loops).unwrap();
    assert_eq!(mesh.face_count(), 8, "outer-with-hole bridges into a 10-vertex simple polygon (n-2 triangles); degenerate loop must be skipped");
    for fi in 0..mesh.face_count() {
        let verts = mesh.face_vertex_ids(FaceId(fi as u32)).unwrap();
        assert_eq!(verts.len(), 3);
    }
}

#[semio_framework_async_macros::async_test]
async fn cylinder_prim_has_expected_topology() {
    let mesh = HalfedgeMesh::cylinder_prim(1.0, 2.0, 8).unwrap();
    assert_eq!(mesh.face_count(), 8 * 3);
    assert_eq!(mesh.vertex_count(), 8 * 2 + 2);
}

#[semio_framework_async_macros::async_test]
async fn cone_prim_has_expected_topology() {
    let mesh = HalfedgeMesh::cone_prim(1.0, 2.0, 6).unwrap();
    assert_eq!(mesh.face_count(), 6 * 2);
    assert_eq!(mesh.vertex_count(), 6 + 2);
}

#[semio_framework_async_macros::async_test]
async fn rotate_mesh_rotates_vertices_about_axis() {
    let mut mesh = HalfedgeMesh::box_prim(2.0, 2.0, 2.0).unwrap();
    let before = mesh.vertex_position(VertexId(0)).unwrap();
    mesh.rotate(Vec3::new(0.0, 0.0, 1.0), std::f32::consts::FRAC_PI_2).unwrap();
    let after = mesh.vertex_position(VertexId(0)).unwrap();
    assert!((before.x() - after.y()).abs() < 1e-4);
}

#[semio_framework_async_macros::async_test]
async fn scale_mesh_scales_vertices() {
    let mut mesh = HalfedgeMesh::box_prim(2.0, 2.0, 2.0).unwrap();
    mesh.scale(Vec3::new(2.0, 1.0, 1.0)).unwrap();
    let p = mesh.vertex_position(VertexId(0)).unwrap();
    assert!((p.x() - (-2.0)).abs() < 1e-5);
}

#[semio_framework_async_macros::async_test]
async fn move_vertices_rejects_empty_and_moves_selected() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    assert_eq!(mesh.move_vertices(&[], Vec3::new(1.0, 0.0, 0.0)), Err(MeshKernelError::EmptySelection));
    let before = mesh.vertex_position(VertexId(0)).unwrap();
    mesh.move_vertices(&[VertexId(0)], Vec3::new(1.0, 0.0, 0.0)).unwrap();
    let after = mesh.vertex_position(VertexId(0)).unwrap();
    assert!((after.x() - before.x() - 1.0).abs() < 1e-5);
}

#[semio_framework_async_macros::async_test]
async fn rotate_vertices_rejects_empty_and_rotates_around_pivot() {
    let mut mesh = HalfedgeMesh::box_prim(2.0, 2.0, 2.0).unwrap();
    assert_eq!(mesh.rotate_vertices(&[], Vec3::new(0.0, 0.0, 1.0), 1.0, Vec3::ZERO), Err(MeshKernelError::EmptySelection));
    // Vertex 0 starts at (-1,-1,-1); rotating 90° about Z around the origin maps (x,y) -> (-y,x).
    mesh.rotate_vertices(&[VertexId(0)], Vec3::new(0.0, 0.0, 1.0), std::f32::consts::FRAC_PI_2, Vec3::ZERO).unwrap();
    let p = mesh.vertex_position(VertexId(0)).unwrap();
    assert!((p.x() - 1.0).abs() < 1e-4, "got x={}", p.x());
    assert!((p.y() - (-1.0)).abs() < 1e-4, "got y={}", p.y());
    assert!((p.z() - (-1.0)).abs() < 1e-4, "got z={}", p.z());
}

#[semio_framework_async_macros::async_test]
async fn scale_vertices_rejects_empty_and_scales_around_pivot() {
    let mut mesh = HalfedgeMesh::box_prim(2.0, 2.0, 2.0).unwrap();
    assert_eq!(mesh.scale_vertices(&[], Vec3::new(2.0, 2.0, 2.0), Vec3::ZERO), Err(MeshKernelError::EmptySelection));
    let before = mesh.vertex_position(VertexId(0)).unwrap();
    mesh.scale_vertices(&[VertexId(0)], Vec3::new(2.0, 1.0, 1.0), Vec3::ZERO).unwrap();
    let after = mesh.vertex_position(VertexId(0)).unwrap();
    assert!((after.x() - before.x() * 2.0).abs() < 1e-5);
}

#[semio_framework_async_macros::async_test]
async fn move_vertices_proportional_rejects_empty_and_applies_falloff() {
    let mut mesh = HalfedgeMesh::box_prim(2.0, 2.0, 2.0).unwrap();
    assert_eq!(mesh.move_vertices_proportional(&[], Vec3::new(1.0, 0.0, 0.0), Vec3::ZERO, 1.0), Err(MeshKernelError::EmptySelection));
    let all: Vec<VertexId> = (0..mesh.vertex_count() as u32).map(VertexId).collect();
    let before = mesh.vertex_position(VertexId(0)).unwrap();
    mesh.move_vertices_proportional(&all, Vec3::new(1.0, 0.0, 0.0), before, 0.001).unwrap();
    let moved = mesh.vertex_position(VertexId(0)).unwrap();
    assert!((moved.x() - before.x() - 1.0).abs() < 1e-4, "vertex at the pivot itself should get full falloff");
}

#[semio_framework_async_macros::async_test]
async fn snap_vertices_to_grid_rejects_non_positive_and_snaps() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    assert_eq!(mesh.snap_vertices_to_grid(&[VertexId(0)], 0.0), Err(MeshKernelError::InvalidInput("grid must be positive".into())));
    mesh.set_vertex_position(VertexId(0), Vec3::new(0.44, 0.0, 0.0)).unwrap();
    mesh.snap_vertices_to_grid(&[VertexId(0)], 0.5).unwrap();
    let p = mesh.vertex_position(VertexId(0)).unwrap();
    assert!((p.x() - 0.5).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn inset_faces_rejects_empty_and_adds_inner_face() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    assert_eq!(mesh.inset_faces(&[], 0.1), Err(MeshKernelError::EmptySelection));
    let before = mesh.face_count();
    mesh.inset_faces(&[FaceId(0)], 0.1).unwrap();
    assert!(mesh.face_count() > before);
}

#[semio_framework_async_macros::async_test]
async fn bevel_edges_rejects_empty_and_runs_on_selection() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    assert_eq!(mesh.bevel_edges(&[], 0.1, 1), Err(MeshKernelError::EmptySelection));
    let before_verts = mesh.vertex_count();
    mesh.bevel_edges(&[EdgeId(0)], 0.1, 1).unwrap();
    assert_eq!(mesh.vertex_count(), before_verts + 2, "bevel_edges appends two offset points per edge");
}

#[semio_framework_async_macros::async_test]
async fn loop_cut_rejects_zero_cuts_and_adds_rings() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    assert_eq!(mesh.loop_cut(&[], 0), Err(MeshKernelError::InvalidInput("cuts must be > 0".into())));
    let before = mesh.face_count();
    mesh.loop_cut(&[], 1).unwrap();
    assert!(mesh.face_count() > before);
}

#[semio_framework_async_macros::async_test]
async fn knife_cut_on_quad_face_adds_split_triangles() {
    // Quad lies in the XZ plane (y=0); the cut plane (x from cut_dir, z=1 from cut_a/cut_b) crosses
    // both z-varying edges of the quad transversally, so knife_cut must find two hits and add faces.
    let positions = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [2.0, 0.0, 2.0], [0.0, 0.0, 2.0]];
    let mut mesh = HalfedgeMesh::from_faces(&positions, &[vec![0, 1, 2, 3]]).unwrap();
    let before = mesh.face_count();
    mesh.knife_cut(FaceId(0), Vec3::new(0.0, -1.0, 1.0), Vec3::new(1.0, -1.0, 1.0)).unwrap();
    assert!(mesh.face_count() > before, "two valid plane hits on the quad must add new split faces");
}

#[semio_framework_async_macros::async_test]
async fn knife_cut_rejects_invalid_face_handle() {
    let mut mesh = HalfedgeMesh::empty();
    assert_eq!(mesh.knife_cut(FaceId(0), Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0)), Err(MeshKernelError::InvalidHandle));
}

#[semio_framework_async_macros::async_test]
async fn merge_vertices_rejects_too_few_and_merges_first_and_center_modes() {
    let mut mesh = HalfedgeMesh::box_prim(2.0, 2.0, 2.0).unwrap();
    assert_eq!(mesh.merge_vertices(&[VertexId(0)], WeldMode::First, 0.0), Err(MeshKernelError::EmptySelection));

    let mut first_mesh = HalfedgeMesh::box_prim(2.0, 2.0, 2.0).unwrap();
    let p0 = first_mesh.vertex_position(VertexId(0)).unwrap();
    first_mesh.merge_vertices(&[VertexId(0), VertexId(1)], WeldMode::First, 0.0).unwrap();
    assert_eq!(first_mesh.vertex_position(VertexId(0)).unwrap(), p0);

    let mut center_mesh = HalfedgeMesh::box_prim(2.0, 2.0, 2.0).unwrap();
    let a = center_mesh.vertex_position(VertexId(0)).unwrap();
    let b = center_mesh.vertex_position(VertexId(1)).unwrap();
    center_mesh.merge_vertices(&[VertexId(0), VertexId(1)], WeldMode::Center, 0.0).unwrap();
    let merged = center_mesh.vertex_position(VertexId(0)).unwrap();
    assert!((merged.x() - a.lerp(b, 0.5).x()).abs() < 1e-5);
}

#[semio_framework_async_macros::async_test]
async fn merge_vertices_by_distance_only_merges_within_threshold() {
    let mut mesh = HalfedgeMesh::box_prim(2.0, 2.0, 2.0).unwrap();
    let before_verts = mesh.vertex_count();
    mesh.merge_vertices(&[VertexId(0), VertexId(1)], WeldMode::ByDistance, 0.01).unwrap();
    assert_eq!(mesh.vertex_count(), before_verts, "vertices farther apart than threshold must not be remapped");
}

#[semio_framework_async_macros::async_test]
async fn dissolve_vertices_rejects_empty_and_removes_incident_faces() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    assert_eq!(mesh.dissolve_vertices(&[]), Err(MeshKernelError::EmptySelection));
    let before = mesh.face_count();
    mesh.dissolve_vertices(&[VertexId(0)]).unwrap();
    assert!(mesh.face_count() < before);
}

#[semio_framework_async_macros::async_test]
async fn subdivide_faces_rejects_empty_and_quadruples_selected_face() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    assert_eq!(mesh.subdivide_faces(&[]), Err(MeshKernelError::EmptySelection));
    let before = mesh.face_count();
    mesh.subdivide_faces(&[FaceId(0)]).unwrap();
    assert_eq!(mesh.face_count(), before - 1 + 8, "a quad face fans 4 edge-midpoint pairs to the centroid into 8 triangles");
}

#[semio_framework_async_macros::async_test]
async fn set_shading_rejects_invalid_handle_and_marks_smooth() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    assert_eq!(mesh.set_shading(&[FaceId(999)], true), Err(MeshKernelError::InvalidHandle));
    mesh.set_shading(&[FaceId(0)], true).unwrap();
    mesh.recompute_normals().unwrap();
}

#[semio_framework_async_macros::async_test]
async fn mirror_doubles_geometry_and_welds_seam() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    let before_faces = mesh.face_count();
    mesh.mirror(MirrorAxis::X, 1e-4).unwrap();
    assert_eq!(mesh.face_count(), before_faces * 2);
}

#[semio_framework_async_macros::async_test]
async fn mark_uv_seam_toggles_and_is_uv_seam_reports_state() {
    let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    let mut mesh = mesh;
    assert!(!mesh.is_uv_seam(EdgeId(0)));
    mesh.mark_uv_seam(&[EdgeId(0)], true);
    assert!(mesh.is_uv_seam(EdgeId(0)));
    mesh.mark_uv_seam(&[EdgeId(0)], false);
    assert!(!mesh.is_uv_seam(EdgeId(0)));
}

#[semio_framework_async_macros::async_test]
async fn unwrap_uv_splits_islands_across_seam() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    mesh.mark_uv_seam(&[EdgeId(0), EdgeId(2), EdgeId(4), EdgeId(6), EdgeId(8), EdgeId(10)], true);
    mesh.unwrap_uv().unwrap();
    let transfer = mesh.tessellate().unwrap();
    assert!(!transfer.uvs.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn decimate_no_op_when_ratio_at_max_and_clamps_below_min() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    let before = mesh.vertex_count();
    mesh.decimate(1.0).unwrap();
    assert_eq!(mesh.vertex_count(), before);

    let mut sphere = HalfedgeMesh::ico_sphere_prim(1.0, 2).unwrap();
    let before_sphere = sphere.vertex_count();
    sphere.decimate(0.0).unwrap();
    assert!(sphere.vertex_count() < before_sphere, "ratio below 0.1 must clamp to 0.1, not become a no-op");
}

#[semio_framework_async_macros::async_test]
async fn decimate_converges_near_target_ratio_without_emptying_mesh() {
    let mut mesh = HalfedgeMesh::ico_sphere_prim(1.0, 2).unwrap();
    let before = mesh.vertex_count();
    mesh.decimate(0.5).unwrap();
    let target = ((before as f32) * 0.5).ceil() as usize;
    assert!(mesh.vertex_count() <= target + 1, "decimate must converge on roughly the requested vertex count, not merge forever");
    assert!(mesh.face_count() > 0, "a 50% decimation must not leave zero faces");
}

#[semio_framework_async_macros::async_test]
async fn to_obj_includes_uv_coordinates_when_present() {
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    mesh.unwrap_uv().unwrap();
    let obj = mesh.to_obj().unwrap();
    assert!(obj.contains("vt "));
}

#[semio_framework_async_macros::async_test]
async fn from_json_rejects_invalid_input() {
    let err = HalfedgeMesh::from_json("not json").unwrap_err();
    assert!(matches!(err, MeshKernelError::InvalidInput(_)));
}
