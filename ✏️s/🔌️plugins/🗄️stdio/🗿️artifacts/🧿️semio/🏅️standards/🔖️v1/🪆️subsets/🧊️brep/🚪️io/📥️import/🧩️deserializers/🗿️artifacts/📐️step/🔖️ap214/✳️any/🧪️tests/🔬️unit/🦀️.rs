
use super::*;

/// 🧱️ Same single-triangular-planar-face box fixture used by step's own
/// `⚙️engine/🧱️brep` tests — a real-world-shaped AP214 exchange snippet (3 vertices, 3
/// straight edges, 1 planar face, 1 shell, 1 solid), not a synthetic degenerate case.
const FIXTURE: &str = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio.step','2026-08-10T00:00:00',('Ueli'),('semio'),'semio','','');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;\n#1=CARTESIAN_POINT('',(0.,0.,0.));\n#2=CARTESIAN_POINT('',(10.,0.,0.));\n#3=CARTESIAN_POINT('',(10.,10.,0.));\n#4=DIRECTION('',(0.,0.,1.));\n#5=VERTEX_POINT('',#1);\n#6=VERTEX_POINT('',#2);\n#7=VERTEX_POINT('',#3);\n#8=EDGE_CURVE('',#5,#6,#20,.T.);\n#9=EDGE_CURVE('',#6,#7,#21,.T.);\n#10=EDGE_CURVE('',#7,#5,#22,.T.);\n#20=LINE('',#1,#30);\n#21=LINE('',#2,#31);\n#22=LINE('',#3,#32);\n#30=VECTOR('',#4,1.);\n#31=VECTOR('',#4,1.);\n#32=VECTOR('',#4,1.);\n#11=ORIENTED_EDGE('',*,*,#8,.T.);\n#12=ORIENTED_EDGE('',*,*,#9,.T.);\n#13=ORIENTED_EDGE('',*,*,#10,.T.);\n#14=EDGE_LOOP('',(#11,#12,#13));\n#15=FACE_OUTER_BOUND('',#14,.T.);\n#16=PLANE('',#40);\n#40=AXIS2_PLACEMENT_3D('',#1,#4,$);\n#17=ADVANCED_FACE('',(#15),#16,.T.);\n#18=CLOSED_SHELL('',(#17));\n#19=MANIFOLD_SOLID_BREP('',#18);\nENDSEC;\nEND-ISO-10303-21;\n";

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fixture_step_snapshot() -> StepSnapshot {
    let doc = semio_s_artifact_stdio_step::engine::part21::parse_part21(FIXTURE).expect("parse real AP214 fixture");
    StepSnapshot::from_part21_document(&doc)
}

#[semio_framework_async_macros::async_test]
async fn deserializes_real_step_fixture_into_topologically_faithful_brep() {
    let step = fixture_step_snapshot();
    let brep = semio_framework_plugin::resolve_ready(SemioBrepFromStep::deserialize(&step)).expect("deserialize real fixture");

    assert_eq!(brep.vertices.len(), 3);
    assert_eq!(brep.edges.len(), 3);
    assert_eq!(brep.loops.len(), 1);
    assert_eq!(brep.faces.len(), 1);
    assert_eq!(brep.shells.len(), 1);
    assert_eq!(brep.solids.len(), 1);

    let v2 = brep.vertices.iter().find(|v| v.id == "v6").expect("VERTEX_POINT #6 imported as v6");
    assert_eq!(v2.point, SemioPoint3 { x: 10.0, y: 0.0, z: 0.0 });

    for e in &brep.edges {
        assert!(matches!(e.curve, BrepCurve::Line { .. }), "fixture edges are all straight LINEs, got {:?}", e.curve);
    }

    let face = &brep.faces[0];
    assert!(face.inner_loops.is_empty());
    match &face.surface {
        BrepSurface::Plane { normal, .. } => assert_eq!(*normal, SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }),
        other => panic!("expected Plane, got {other:?}"),
    }

    assert_eq!(brep.solids[0].shells.len(), 1);
    assert!(!brep.solids[0].shells[0].is_void);
}

#[semio_framework_async_macros::async_test]
async fn dangling_curve_reference_errors_rather_than_fabricating() {
    // A LINE whose `dir` points at a nonexistent VECTOR must fail loudly, not silently
    // produce a zero direction.
    let bad = FIXTURE.replace("#20=LINE('',#1,#30);", "#20=LINE('',#1,#999);");
    let doc = semio_s_artifact_stdio_step::engine::part21::parse_part21(&bad).expect("parse");
    let step = StepSnapshot::from_part21_document(&doc);
    let result = semio_framework_plugin::resolve_ready(SemioBrepFromStep::deserialize(&step));
    assert!(result.is_err(), "dangling VECTOR reference must surface as an error, not a fabricated direction");
}

#[semio_framework_async_macros::async_test]
async fn unsupported_surface_kind_errors_rather_than_fabricating() {
    // Swap PLANE for a surface kind outside this leaf's supported vocabulary.
    let bad = FIXTURE.replace("#16=PLANE('',#40);", "#16=SURFACE_OF_REVOLUTION('',#20,#40);");
    let doc = semio_s_artifact_stdio_step::engine::part21::parse_part21(&bad).expect("parse");
    let step = StepSnapshot::from_part21_document(&doc);
    let result = semio_framework_plugin::resolve_ready(SemioBrepFromStep::deserialize(&step));
    assert!(result.is_err(), "an unsupported surface entity must error, never silently become a Plane");
}
