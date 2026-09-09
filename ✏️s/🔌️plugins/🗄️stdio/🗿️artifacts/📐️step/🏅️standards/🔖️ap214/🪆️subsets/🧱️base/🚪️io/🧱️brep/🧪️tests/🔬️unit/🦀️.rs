use super::super::part21::parse_part21;
use super::*;

const FIXTURE: &str = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio.step','2026-08-10T00:00:00',('Ueli'),('semio'),'semio','','');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;\n#1=CARTESIAN_POINT('',(0.,0.,0.));\n#2=CARTESIAN_POINT('',(10.,0.,0.));\n#3=CARTESIAN_POINT('',(10.,10.,0.));\n#4=DIRECTION('',(0.,0.,1.));\n#5=VERTEX_POINT('',#1);\n#6=VERTEX_POINT('',#2);\n#7=VERTEX_POINT('',#3);\n#8=EDGE_CURVE('',#5,#6,#20,.T.);\n#9=EDGE_CURVE('',#6,#7,#21,.T.);\n#10=EDGE_CURVE('',#7,#5,#22,.T.);\n#20=LINE('',#1,#30);\n#21=LINE('',#2,#31);\n#22=LINE('',#3,#32);\n#30=VECTOR('',#4,1.);\n#31=VECTOR('',#4,1.);\n#32=VECTOR('',#4,1.);\n#11=ORIENTED_EDGE('',*,*,#8,.T.);\n#12=ORIENTED_EDGE('',*,*,#9,.T.);\n#13=ORIENTED_EDGE('',*,*,#10,.T.);\n#14=EDGE_LOOP('',(#11,#12,#13));\n#15=FACE_OUTER_BOUND('',#14,.T.);\n#16=PLANE('',#40);\n#40=AXIS2_PLACEMENT_3D('',#1,#4,$);\n#17=ADVANCED_FACE('',(#15),#16,.T.);\n#18=CLOSED_SHELL('',(#17));\n#19=MANIFOLD_SOLID_BREP('',#18);\nENDSEC;\nEND-ISO-10303-21;\n";

#[semio_framework_async_macros::async_test]
async fn analyzes_real_non_degenerate_mesh() {
    let doc = parse_part21(FIXTURE).expect("parse fixture");
    let view = analyze_brep_mesh(&doc);
    assert!(view.issues.is_empty(), "unexpected issues: {:?}", view.issues);
    assert_eq!(view.mesh.vertices.len(), 3);
    assert_eq!(view.mesh.faces.len(), 1);
    let face = &view.mesh.faces[0];
    assert_eq!(face.indices.len(), 3);
    for &idx in &face.indices {
        assert!(idx < view.mesh.vertices.len(), "face references an invalid vertex index");
    }
    assert_eq!(view.mesh.vertices[1].x, 10.0);
    assert_eq!(view.mesh.vertices[2].y, 10.0);
}

#[semio_framework_async_macros::async_test]
async fn curved_surface_flagged_unsupported_not_fabricated() {
    let text = FIXTURE.replace("#16=PLANE('',#40);", "#16=B_SPLINE_SURFACE_WITH_KNOTS('',3,3,((#1,#2,#3)),.UNSPECIFIED.,.F.,.F.,.F.,(4,4),(4,4),(0.,1.),(0.,1.),.UNSPECIFIED.);");
    let doc = parse_part21(&text).expect("parse");
    let view = analyze_brep_mesh(&doc);
    assert!(view.mesh.faces.is_empty(), "a curved face must not silently become a mesh face");
    assert!(view.issues.iter().any(|i| i.reason.contains("PLANE")));
}

#[semio_framework_async_macros::async_test]
async fn writer_round_trips_through_analyzer() {
    let mesh =
        BrepMesh { vertices: vec![BrepVertex { x: 0.0, y: 0.0, z: 0.0 }, BrepVertex { x: 4.0, y: 0.0, z: 0.0 }, BrepVertex { x: 4.0, y: 3.0, z: 0.0 }, BrepVertex { x: 0.0, y: 3.0, z: 0.0 }], faces: vec![BrepFace { indices: vec![0, 1, 2, 3] }] };
    let doc = brep_mesh_to_part21(&mesh);
    let text = super::super::part21::write_part21(&doc);
    let reparsed = parse_part21(&text).expect("reparse generated step text");
    let view = analyze_brep_mesh(&reparsed);
    assert!(view.issues.is_empty(), "unexpected issues: {:?}", view.issues);
    assert_eq!(view.mesh.vertices.len(), 4);
    assert_eq!(view.mesh.faces.len(), 1);
    assert_eq!(view.mesh.faces[0].indices.len(), 4);
}

#[semio_framework_async_macros::async_test]
async fn empty_mesh_writer_still_produces_valid_document() {
    let doc = brep_mesh_to_part21(&BrepMesh::default());
    let text = super::super::part21::write_part21(&doc);
    let reparsed = parse_part21(&text).expect("reparse empty mesh document");
    let view = analyze_brep_mesh(&reparsed);
    assert!(view.mesh.faces.is_empty());
    assert!(view.mesh.vertices.is_empty());
}
