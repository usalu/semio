
use super::*;
use crate::standards::v1::subsets::brep::schema::diff::primitives::make_box;

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
    let imported = import_stl_to_body(&mut imported_body, &bytes, 1e-4).unwrap();
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
    let imported_solid = import_glb_to_body(&mut imported, &bytes, 1e-4).unwrap();
    assert!(!imported.solid_faces(imported_solid).is_empty());
}
