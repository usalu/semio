use super::*;
use crate::schema::snapshot::{ObjFace, ObjFaceVertex, ObjGroup, ObjVertex};
use crate::STDIO_OBJ_DOCUMENT_SCHEMA;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn vertex(x: f64, y: f64, z: f64) -> ObjVertex {
    ObjVertex { x, y, z, w: None }
}

#[semio_framework_async_macros::async_test]
async fn bounds_matches_hand_built_vertex_extent() {
    let snapshot = ObjSnapshot {
        schema: STDIO_OBJ_DOCUMENT_SCHEMA.into(),
        vertices: vec![vertex(-1.0, 0.0, 2.0), vertex(3.0, -4.0, 1.0), vertex(0.0, 5.0, -2.0)],
        texcoords: Vec::new(),
        normals: Vec::new(),
        faces: vec![
            ObjFace { vertices: vec![ObjFaceVertex { vertex: 0, texcoord: None, normal: None }, ObjFaceVertex { vertex: 1, texcoord: None, normal: None }] },
            ObjFace { vertices: vec![ObjFaceVertex { vertex: 1, texcoord: None, normal: None }, ObjFaceVertex { vertex: 2, texcoord: None, normal: None }] },
        ],
        groups: vec![ObjGroup { name: "g1".into(), faces: vec![0, 1] }],
        objects: Vec::new(),
        mtllib: None,
        usemtl: Vec::new(),
        smoothing_groups: Vec::new(),
        unknown_statements: Vec::new(),
    };

    let bounds = compute_obj_bounds(&snapshot);
    assert_eq!(bounds.min, [-1.0, -4.0, -2.0]);
    assert_eq!(bounds.max, [3.0, 5.0, 2.0]);
    assert_eq!(bounds.vertex_count, 3);
    assert_eq!(bounds.face_count, 2);
    assert_eq!(bounds.group_count, 1);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = ObjSnapshot {
        schema: STDIO_OBJ_DOCUMENT_SCHEMA.into(),
        vertices: vec![vertex(1.0, 1.0, 1.0)],
        texcoords: Vec::new(),
        normals: Vec::new(),
        faces: Vec::new(),
        groups: Vec::new(),
        objects: Vec::new(),
        mtllib: None,
        usemtl: Vec::new(),
        smoothing_groups: Vec::new(),
        unknown_statements: Vec::new(),
    };
    assert_eq!(compute_obj_bounds(&snapshot), compute_obj_bounds(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_obj_bounds(&ObjSnapshot::default()), ObjBounds::default());
}
