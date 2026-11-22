use super::*;
use semio_framework_3d_brep_construction::primitives::make_box;

#[semio_framework_async_macros::async_test]
async fn export_box_mesh_stl_nonempty() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let bytes = export_solid_stl(&body, solid, 0.1).unwrap();
    assert!(bytes.len() > 84, "binary STL must include header and triangles");
    let tri_count = u32::from_le_bytes(bytes[80..84].try_into().unwrap());
    assert!(tri_count > 0);
}

#[semio_framework_async_macros::async_test]
async fn import_stl_produces_faces() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let bytes = export_solid_stl(&body, solid, 0.1).unwrap();
    let mut imported_body = Body::new();
    let EntityRef::Solid(imported) = import_stl_to_body(&mut imported_body, &bytes, 1e-4).unwrap() else {panic!("closed STL requires solid")};
    assert!(!imported_body.solid_faces(imported).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn obj_export_import_round_trip() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let text = export_solid_obj(&body, solid, 0.1).unwrap();
    assert!(text.contains("v "));
    let mesh = import_obj(&text).unwrap();
    assert!(mesh.indices.len() >= 3);
}

#[semio_framework_async_macros::async_test]
async fn glb_export_import_round_trip() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let bytes = export_solid_glb(&body, solid, 0.1).unwrap();
    assert!(!bytes.is_empty());
    let mesh = import_glb(&bytes).unwrap();
    assert!(mesh.triangle_count() >= 1);
}

#[semio_framework_async_macros::async_test]
async fn import_glb_to_body_has_faces() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let bytes = export_solid_glb(&body, solid, 0.1).unwrap();
    let mut imported = Body::new();
    let EntityRef::Solid(imported_solid) = import_glb_to_body(&mut imported, &bytes, 1e-4).unwrap() else {panic!("closed GLB requires solid")};
    assert!(!imported.solid_faces(imported_solid).is_empty());
}

#[test]
fn synchronous_import_publishes_the_open_shell_after_retirement() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️import-rollback.json")).unwrap();
    let positions: Vec<[f64; 3]> = serde_json::from_value(fixture["points"].clone()).unwrap();
    let triangles: Vec<[u32; 3]> = serde_json::from_value(fixture["triangles"].clone()).unwrap();
    let mesh = TriangleMesh {
        positions: positions.into_iter().map(|[x, y, z]| Pnt3::new(x, y, z)).collect(),
        normals: Vec::new(),
        indices: triangles.into_iter().flatten().collect(),
    };
    let mut body = Body::new();
    let EntityRef::Shell(shell) = import_triangle_mesh_to_body(&mut body, &mesh, 1e-6).unwrap() else {
        panic!("open mesh requires a shell");
    };
    let expected = &fixture["openShell"];
    let count = |name| expected[name].as_u64().unwrap() as usize;
    let mut uses = std::collections::HashMap::new();
    for (_, coedge) in body.coedges.iter() {
        *uses.entry(coedge.edge).or_insert(0) += 1;
    }
    assert_eq!(body.shells.get(shell).unwrap().faces.len(), count("faces"));
    assert_eq!(body.edges.len(), count("edges"));
    assert_eq!(uses.values().filter(|&&uses| uses == 1).count(), count("boundaryEdges"));
    assert!(body.solids.is_empty());
}

#[test]
fn synchronous_import_rollback_preserves_the_callers_live_geometry() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️import-rollback.json")).unwrap();
    let positions: Vec<[f64; 3]> = serde_json::from_value(fixture["points"].clone()).unwrap();
    let first: [u32; 3] = serde_json::from_value(fixture["triangles"][0].clone()).unwrap();
    let second: [u32; 3] = serde_json::from_value(fixture["lateRefusal"]["triangle"].clone()).unwrap();
    let mesh = TriangleMesh {
        positions: positions.into_iter().map(|[x, y, z]| Pnt3::new(x, y, z)).collect(),
        normals: Vec::new(),
        indices: first.into_iter().chain(second).collect(),
    };
    let mut body = Body::new();
    let solid = make_box(&mut body, 1.0, 2.0, 3.0, &mut OpRecorder::new()).unwrap();
    let entity_counts = body.entity_counts();
    let faces = body.solid_faces(solid);
    let obj = export_solid_obj(&body, solid, 0.1).unwrap();

    assert_eq!(
        import_triangle_mesh_to_body(&mut body, &mesh, 1e-6),
        Err(KernelError::InvalidInput(fixture["lateRefusal"]["message"].as_str().unwrap().into())),
    );
    assert_eq!(body.entity_counts(), entity_counts);
    assert_eq!(body.solid_faces(solid), faces);
    assert_eq!(export_solid_obj(&body, solid, 0.1).unwrap(), obj);
}

#[test]
fn original_import_rollback_debits_each_caller_funded_body_turn() {
    use semio_framework_value::AuthoringWallet;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️import-rollback.json")).unwrap();
    let positions: Vec<f32> = fixture["points"].as_array().unwrap().iter().flat_map(|p| p.as_array().unwrap()).map(|value| value.as_f64().unwrap() as f32).collect();
    let indices: Vec<u32> = fixture["triangles"].as_array().unwrap().iter().flat_map(|triangle| triangle.as_array().unwrap()).map(|value| value.as_u64().unwrap() as u32).collect();
    let before = fixture["facesBeforeCancellation"].as_u64().unwrap() as usize;
    let work = fixture["workPerTurn"].as_u64().unwrap();
    let mut cursor = MeshImportCursor::new(positions, Vec::new(), indices, 1e-6).unwrap();
    let mut pending = PendingMutations::default();
    cursor.activate(&mut pending);
    let other = pending.open();
    assert_eq!(pending.count(), fixture["pendingDuringInterleaving"].as_u64().unwrap() as usize);
    let mut body = Body::new();
    for _ in 0..20 {
        if body.faces.len() == before { break; }
        assert!(matches!(cursor.step(&mut body, 1).unwrap(), MeshImportStep::Working));
    }
    assert_eq!(body.faces.len(), before);
    let points: Vec<_> = body.vertices.iter().map(|(_, vertex)| vertex.position).collect();
    assert_eq!(points.len(), 3);
    let normal = (points[1] - points[0]).cross(points[2] - points[0]);
    assert_eq!(normal.dot(normal).sqrt() * 0.5, fixture["firstTriangleArea"].as_f64().unwrap());
    cursor.cancel();

    let mut removed_faces = 0;
    for _ in 0..1000 {
        if cursor.retirement_complete() { break; }
        let copy = cursor.next_close_copy_byte_demand(&body).unwrap();
        let grant = RetainedCloneGrant {
            maximum_items: 1, maximum_copy_bytes: copy,
            maximum_capacity_bytes: cursor.next_close_capacity_byte_demand(&body, copy).unwrap(),
            maximum_release_bytes: cursor.next_close_release_byte_demand(&body).unwrap(),
            maximum_depth: cursor.next_close_depth_demand(&body).unwrap(),
        };
        let start = body.faces.len();
        let mut wallet = AuthoringWallet::new(grant, u128::from(work));
        let turn = wallet.run_invoked(grant, work, |effect| {
            let mut effect = effect.reborrow(grant, work).unwrap();
            let result = cursor.close_original_step(&mut body, &mut effect);
            assert_eq!(result.work, effect.performed_work());
            result.result
        }).unwrap().unwrap();
        turn.output.unwrap();
        let removed = start - body.faces.len();
        assert!(removed <= 1);
        removed_faces += removed;
    }
    cursor.release();
    assert_eq!(pending.count(), fixture["pendingAfterCancellation"].as_u64().unwrap() as usize);
    drop(other);
    assert_eq!(pending.count(), 0);
    assert!(cursor.retirement_complete());
    assert_eq!(removed_faces, fixture["rollbackFaceDelta"].as_u64().unwrap() as usize);
    assert!(body.vertices.is_empty() && body.edges.is_empty() && body.faces.is_empty());
}
