use super::*;
use semio_s_artifact_stdio_obj::schema::snapshot::{ObjFaceVertex, ObjNormal, ObjTexCoord, ObjVertex};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_obj() -> ObjSnapshot {
    ObjSnapshot {
        schema: "stdio.obj".into(),
        vertices: vec![ObjVertex { x: 0.0, y: 0.0, z: 0.0, w: None }, ObjVertex { x: 1.0, y: 0.0, z: 0.0, w: None }, ObjVertex { x: 1.0, y: 1.0, z: 0.0, w: None }, ObjVertex { x: 0.0, y: 1.0, z: 0.0, w: None }],
        texcoords: vec![ObjTexCoord { u: 0.0, v: 0.0, w: None }, ObjTexCoord { u: 1.0, v: 0.0, w: None }, ObjTexCoord { u: 1.0, v: 1.0, w: None }, ObjTexCoord { u: 0.0, v: 1.0, w: None }],
        normals: vec![ObjNormal { x: 0.0, y: 0.0, z: 1.0 }],
        faces: vec![ObjFace {
            vertices: vec![
                ObjFaceVertex { vertex: 0, texcoord: Some(0), normal: Some(0) },
                ObjFaceVertex { vertex: 1, texcoord: Some(1), normal: Some(0) },
                ObjFaceVertex { vertex: 2, texcoord: Some(2), normal: Some(0) },
                ObjFaceVertex { vertex: 3, texcoord: Some(3), normal: Some(0) },
            ],
        }],
        groups: Vec::new(),
        objects: Vec::new(),
        mtllib: None,
        usemtl: Vec::new(),
        smoothing_groups: Vec::new(),
        unknown_statements: Vec::new(),
    }
}

#[semio_framework_async_macros::async_test]
async fn deserialize_fan_triangulates_the_quad_into_two_triangles() {
    let semio = semio_framework_plugin::resolve_ready(SemioMeshFromObj::deserialize(&sample_obj())).expect("deserialize");
    assert_eq!(semio.meshes.len(), 1);
    assert_eq!(semio.meshes[0].id, "mesh-0");
    let prim = &semio.meshes[0].primitives[0];
    assert_eq!(prim.positions.len(), 6, "quad (4 corners) fan-triangulates to 2 triangles = 6 corners");
    assert_eq!(prim.normals.len(), 6);
    assert_eq!(prim.uvs.len(), 6);
    assert!(prim.indices.is_empty());
    assert_eq!(prim.positions[0], SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 });
    assert_eq!(prim.positions[1], SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 });
    assert_eq!(prim.positions[2], SemioPoint3 { x: 1.0, y: 1.0, z: 0.0 });
}

#[semio_framework_async_macros::async_test]
async fn objects_partition_into_separate_semio_meshes() {
    let mut obj = sample_obj();
    obj.faces.push(ObjFace { vertices: vec![ObjFaceVertex { vertex: 0, texcoord: Some(0), normal: Some(0) }, ObjFaceVertex { vertex: 1, texcoord: Some(1), normal: Some(0) }, ObjFaceVertex { vertex: 2, texcoord: Some(2), normal: Some(0) }] });
    obj.objects = vec![semio_s_artifact_stdio_obj::schema::snapshot::ObjObject { name: "quad".into(), faces: vec![0] }, semio_s_artifact_stdio_obj::schema::snapshot::ObjObject { name: "tri".into(), faces: vec![1] }];
    let semio = semio_framework_plugin::resolve_ready(SemioMeshFromObj::deserialize(&obj)).expect("deserialize");
    assert_eq!(semio.meshes.len(), 2);
    assert_eq!(semio.meshes[0].id, "quad");
    assert_eq!(semio.meshes[1].id, "tri");
    assert_eq!(semio.meshes[1].primitives[0].positions.len(), 3);
}

#[semio_framework_async_macros::async_test]
async fn out_of_range_vertex_reference_is_a_hard_error() {
    let mut obj = sample_obj();
    obj.faces[0].vertices[0].vertex = 999;
    let err = semio_framework_plugin::resolve_ready(SemioMeshFromObj::deserialize(&obj)).expect_err("out-of-range vertex must error");
    assert!(format!("{err:?}").contains("out-of-range"), "got {err:?}");
}
