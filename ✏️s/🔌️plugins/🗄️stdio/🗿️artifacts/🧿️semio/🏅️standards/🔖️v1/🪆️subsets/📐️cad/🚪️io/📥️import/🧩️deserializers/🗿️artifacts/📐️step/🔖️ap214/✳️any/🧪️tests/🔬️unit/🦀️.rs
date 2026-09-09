use super::*;

const FIXTURE: &str = concat!(
    "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\n",
    "FILE_NAME('semio.step','2026-08-11T00:00:00',('Ueli'),('semio'),'semio','','');\n",
    "FILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;\n",
    "#1=CARTESIAN_POINT('',(0.,0.,0.));\n",
    "#2=DIRECTION('',(1.,0.,0.));\n",
    "#3=VECTOR('',#2,5.);\n",
    "#4=LINE('',#1,#3);\n",
    "#5=CARTESIAN_POINT('',(2.,2.,0.));\n",
    "#6=AXIS2_PLACEMENT_3D('',#5,$,$);\n",
    "#7=CIRCLE('',#6,1.5);\n",
    "ENDSEC;\nEND-ISO-10303-21;\n",
);

#[semio_framework_async_macros::async_test]
async fn resolves_line_and_circle_through_the_real_entity_graph() {
    let step = <StepSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE).expect("parse real step text");
    let cad = semio_framework_plugin::resolve_ready(SemioCadFromStep::deserialize(&step)).expect("deserialize");
    assert_eq!(cad.entities.len(), 2);
    match &cad.entities[0].entity {
        CadEntity::Line { a, b } => {
            assert_eq!(*a, SemioPoint2 { x: 0.0, y: 0.0 });
            assert!((b.x - 5.0).abs() < 1e-9 && (b.y - 0.0).abs() < 1e-9);
        }
        other => panic!("expected Line, got {other:?}"),
    }
    match &cad.entities[1].entity {
        CadEntity::Circle { center, radius } => {
            assert_eq!(*center, SemioPoint2 { x: 2.0, y: 2.0 });
            assert_eq!(*radius, 1.5);
        }
        other => panic!("expected Circle, got {other:?}"),
    }
}
