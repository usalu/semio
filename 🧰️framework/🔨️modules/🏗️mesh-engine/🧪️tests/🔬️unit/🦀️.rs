
use super::*;

#[test]
fn box_has_triangles() {
    let mesh = mesh_box(1.0, 1.0, 1.0);
    assert_eq!(mesh.triangle_count(), 12);
    assert_eq!(mesh.normals.len(), mesh.positions.len());
}

#[test]
fn glb_round_trip() {
    let mesh = mesh_uv_sphere(1.0, 8, 6);
    let glb = mesh_to_glb(&mesh);
    let decoded = mesh_from_glb(&glb).expect("decode glb");
    assert_eq!(decoded.vertex_count(), mesh.vertex_count());
    assert_eq!(decoded.indices.len(), mesh.indices.len());
}

/// 🏙️ Puzzle GLBs may start with non-triangle guide geometry before their renderable surfaces.
#[test]
fn glb_import_collects_triangle_primitives_after_guides() {
    let decoded = mesh_from_glb(include_bytes!("../../../🖼️assets/🌱️metabolism/🎨️representation/💊️capsules/🪝️j/🧊️capsule_J.glb")).expect("decode Puzzle GLB");
    assert_eq!(decoded.vertex_count(), 1472);
    assert_eq!(decoded.triangle_count(), 1750);
    assert!(decoded.indices.iter().all(|index| (*index as usize) < 1472));
}

#[test]
fn obj_round_trip() {
    let mesh = mesh_uv_sphere(1.0, 8, 6);
    let obj = mesh_to_obj(&mesh, "sphere");
    let decoded = mesh_from_obj(&obj).expect("decode obj");
    assert_eq!(decoded.vertex_count(), mesh.vertex_count());
    assert_eq!(decoded.indices.len(), mesh.indices.len());
}

#[test]
fn stl_round_trip() {
    let mesh = mesh_box(1.0, 1.0, 1.0);
    let stl = mesh_to_stl(&mesh);
    assert_eq!(stl.len(), 80 + 4 + mesh.triangle_count() * 50);
    let decoded = mesh_from_stl(&stl).expect("decode stl");
    assert_eq!(decoded.triangle_count(), mesh.triangle_count());
    assert_eq!(decoded.positions.len(), mesh.triangle_count() * 9);
}

/// 🔺️ Small shared-vertex tetrahedron fixture (4 verts, 4 triangles) used by the format round-trip tests below — small enough to assert exact positions/indices, but with enough shared vertices to exercise indexed (not per-face-duplicated) geometry.
fn tetra_mesh_fixture() -> MeshData {
    let mut mesh = MeshData {
        positions: vec![
            0.0, 0.0, 0.0, // v0
            1.0, 0.0, 0.0, // v1
            0.0, 1.0, 0.0, // v2
            0.0, 0.0, 1.0, // v3
        ],
        indices: vec![0, 1, 2, 0, 1, 3, 0, 2, 3, 1, 2, 3],
        ..MeshData::default()
    };
    mesh.compute_normals();
    mesh
}

fn assert_positions_close(a: &[f32], b: &[f32]) {
    assert_eq!(a.len(), b.len(), "position array length mismatch");
    for (x, y) in a.iter().zip(b.iter()) {
        assert!((x - y).abs() < 1e-4, "position mismatch: {x} vs {y}");
    }
}

#[test]
fn obj_round_trip_preserves_positions_and_indices() {
    let mesh = tetra_mesh_fixture();
    let bytes = ObjExporter.export(&mesh).expect("export obj");
    let decoded = ObjImporter.import(&bytes).expect("import obj");
    assert_positions_close(&decoded.positions, &mesh.positions);
    assert_eq!(decoded.indices, mesh.indices);
}

#[test]
fn glb_round_trip_preserves_positions_and_indices() {
    let mesh = tetra_mesh_fixture();
    let bytes = GlbExporter.export(&mesh).expect("export glb");
    let decoded = GlbImporter.import(&bytes).expect("import glb");
    assert_positions_close(&decoded.positions, &mesh.positions);
    assert_eq!(decoded.indices, mesh.indices);
}

#[test]
fn stl_round_trip_preserves_triangle_geometry() {
    let mesh = tetra_mesh_fixture();
    let bytes = StlExporter.export(&mesh).expect("export stl");
    let decoded = StlImporter.import(&bytes).expect("import stl");
    assert_eq!(decoded.triangle_count(), mesh.triangle_count());
    // STL has no vertex sharing, so indices are trivially [0, 1, 2, 3, ...]; compare
    // per-triangle corner positions against the original indexed mesh instead.
    for (triangle, decoded_tri) in mesh.indices.as_chunks::<3>().0.iter().zip(decoded.indices.as_chunks::<3>().0) {
        for (&original_index, &decoded_index) in triangle.iter().zip(decoded_tri.iter()) {
            let original = &mesh.positions[original_index as usize * 3..original_index as usize * 3 + 3];
            let decoded_position = &decoded.positions[decoded_index as usize * 3..decoded_index as usize * 3 + 3];
            assert_positions_close(decoded_position, original);
        }
    }
}

#[test]
fn mesh_from_indexed_with_face_groups_stamps_per_triangle_face_ids() {
    let positions: Vec<f32> = (0..6 * 3 * 3).map(|i| i as f32).collect();
    let indices: Vec<u32> = (0..18).collect();
    let face_groups = [(101u32, 0u32, 6u32), (202u32, 6u32, 12u32)];
    let mesh = mesh_from_indexed_with_face_groups(&positions, &[], &indices, &face_groups);
    assert_eq!(mesh.face_ids.len(), 6);
    assert_eq!(&mesh.face_ids[0..2], &[101, 101]);
    assert_eq!(&mesh.face_ids[2..6], &[202, 202, 202, 202]);
}

#[test]
fn mesh_from_indexed_with_face_groups_empty_groups_leaves_face_ids_empty() {
    let positions: Vec<f32> = (0..9).map(|i| i as f32).collect();
    let indices: Vec<u32> = vec![0, 1, 2];
    let mesh = mesh_from_indexed_with_face_groups(&positions, &[], &indices, &[]);
    assert!(mesh.face_ids.is_empty());
}

#[test]
fn mesh_from_obj_rejects_malformed_v_and_vn_lines() {
    assert_eq!(mesh_from_obj("v 1.0 2.0\n").unwrap_err(), "obj: malformed v line");
    assert_eq!(mesh_from_obj("v 0 0 0\nvn 1.0\n").unwrap_err(), "obj: malformed vn line");
}

#[test]
fn mesh_from_obj_rejects_malformed_face_index() {
    let text = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf notanumber 2 3\n";
    assert_eq!(mesh_from_obj(text).unwrap_err(), "obj: malformed face index");
}

#[test]
fn mesh_from_obj_zero_and_out_of_range_negative_indices_error() {
    let text_zero = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 0 1 2\n";
    assert_eq!(mesh_from_obj(text_zero).unwrap_err(), "obj: zero vertex index");
    let text_negative = "v 0 0 0\nf -5 1 1\n";
    assert_eq!(mesh_from_obj(text_negative).unwrap_err(), "obj: negative vertex index out of range");
}

#[test]
fn mesh_from_obj_resolves_negative_relative_face_indices() {
    let text = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf -3 -2 -1\n";
    let mesh = mesh_from_obj(text).expect("negative indices resolve relative to the current vertex count");
    assert_eq!(mesh.indices, vec![0, 1, 2]);
}

#[test]
fn mesh_from_obj_triangulates_ngon_faces_and_skips_degenerate_faces() {
    let text = "v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nf 1 2 3 4\nf 1 2\n";
    let mesh = mesh_from_obj(text).expect("decode");
    assert_eq!(mesh.triangle_count(), 2, "quad fan-triangulates into 2 triangles; the 2-vertex face is skipped");
}

#[test]
fn mesh_from_stl_rejects_truncated_header_and_truncated_triangle_data() {
    assert_eq!(mesh_from_stl(&[0u8; 10]).unwrap_err(), "stl: truncated header");
    let mut bytes = vec![0u8; 84];
    bytes[80..84].copy_from_slice(&1u32.to_le_bytes());
    assert_eq!(mesh_from_stl(&bytes).unwrap_err(), "stl: truncated triangle data");
}

#[test]
fn mesh_from_glb_rejects_bytes_without_valid_glb_container() {
    assert!(mesh_from_glb(b"not a glb file").is_err());
}

/// 🧊️ Loads the committed language-agnostic single-triangle fixture's expected decoded
/// output (`✅️expected-single-triangle.json`, comparable by any implementation) via
/// `pack::json`, not hardcoded twice.
fn expected_single_triangle() -> (Vec<f32>, Vec<f32>, Vec<u32>) {
    let text = include_str!("../../🧫️fixtures/🧊️gltf-codec/✅️expected-single-triangle.json");
    let value = json::parse(text).expect("expected fixture json parses");
    let floats = |key: &str| -> Vec<f32> { value.get(key).and_then(json::Value::as_array).unwrap().iter().map(|v| v.as_f64().unwrap() as f32).collect() };
    let indices = value.get("indices").and_then(json::Value::as_array).unwrap().iter().map(|v| v.as_u64().unwrap() as u32).collect();
    (floats("positions"), floats("normals"), indices)
}

#[test]
fn mesh_from_gltf_decodes_embedded_base64_buffer() {
    let text = include_str!("../../🧫️fixtures/🧊️gltf-codec/🔺️single-triangle-embedded.gltf");
    let mesh = mesh_from_glb(text.as_bytes()).expect("decode embedded-buffer .gltf");
    let (positions, normals, indices) = expected_single_triangle();
    assert_eq!(mesh.positions, positions);
    assert_eq!(mesh.normals, normals);
    assert_eq!(mesh.indices, indices);
}

#[test]
fn mesh_from_glb_decodes_embedded_bin_chunk() {
    let bytes = include_bytes!("../../🧫️fixtures/🧊️gltf-codec/🧊️single-triangle-embedded.glb");
    let mesh = mesh_from_glb(bytes).expect("decode embedded-buffer .glb");
    let (positions, normals, indices) = expected_single_triangle();
    assert_eq!(mesh.positions, positions);
    assert_eq!(mesh.normals, normals);
    assert_eq!(mesh.indices, indices);
}

/// 🚫️ External (file-path) buffer uris are left unresolved by design — this engine has no
/// filesystem/network access, matching the stdio gltf artifact's own `resolve_document_buffers`
/// contract — a clear typed error, never fabricated geometry.
#[test]
fn mesh_from_gltf_reports_a_clear_error_for_unresolved_external_buffer() {
    let text = include_str!("../../🧫️fixtures/🧊️gltf-codec/🔗️external-buffer.gltf");
    let error = mesh_from_glb(text.as_bytes()).unwrap_err();
    assert!(error.contains("buffer 0 bytes unavailable"), "unexpected error: {error}");
}

#[test]
fn mesh_from_kind_maps_known_kinds_and_falls_back_to_box() {
    assert_eq!(mesh_from_kind("plane").triangle_count(), mesh_plane(1.0, 1.0).triangle_count());
    assert_eq!(mesh_from_kind("cylinder").triangle_count(), mesh_cylinder(0.5, 1.0, 16).triangle_count());
    assert_eq!(mesh_from_kind("cone").triangle_count(), mesh_cone(0.5, 1.0, 16).triangle_count());
    assert_eq!(mesh_from_kind("torus").triangle_count(), mesh_torus(0.5, 0.15, 16, 12).triangle_count());
    assert_eq!(mesh_from_kind("vortex-marker").triangle_count(), mesh_ico_sphere(0.12, 1).triangle_count());
    assert_eq!(mesh_from_kind("totally-unknown-kind").triangle_count(), mesh_box(1.0, 1.0, 1.0).triangle_count());
}

#[test]
fn mesh_data_aabb_and_merge() {
    let mut mesh = mesh_box(2.0, 4.0, 6.0);
    let (min, max) = mesh.aabb();
    assert!((min[0] - -1.0).abs() < 1e-5 && (max[0] - 1.0).abs() < 1e-5);
    assert!((min[1] - -2.0).abs() < 1e-5 && (max[1] - 2.0).abs() < 1e-5);

    let base_vertex_count = mesh.vertex_count();
    let extra = mesh_plane(1.0, 1.0);
    let extra_vertex_count = extra.vertex_count();
    mesh.merge(&extra);
    assert_eq!(mesh.vertex_count(), base_vertex_count + extra_vertex_count);
    assert_eq!(*mesh.indices.last().unwrap(), (base_vertex_count + extra_vertex_count - 1) as u32, "merged indices are offset by the base vertex count");
}

#[test]
fn mesh_exporter_and_importer_use_short_format_kind_ids_not_media_format() {
    assert_eq!(ObjExporter.format_kind(), "obj");
    assert_eq!(ObjImporter.format_kind(), "obj");
    assert_eq!(GlbExporter.format_kind(), "glb");
    assert_eq!(GlbImporter.format_kind(), "glb");
    assert_eq!(StlExporter.format_kind(), "stl");
    assert_eq!(StlImporter.format_kind(), "stl");
}
