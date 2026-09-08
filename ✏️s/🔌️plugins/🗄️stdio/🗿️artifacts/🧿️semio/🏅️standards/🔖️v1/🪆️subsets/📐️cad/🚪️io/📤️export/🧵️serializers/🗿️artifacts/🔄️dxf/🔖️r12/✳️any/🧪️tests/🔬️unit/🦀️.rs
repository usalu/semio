
use super::*;
use crate::standards::v1::subsets::cad::schema::snapshot::CadLayer;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_cad() -> SemioCadSnapshot {
    SemioCadSnapshot {
        layers: vec![CadLayer { name: "0".into(), color_index: 7, line_type: "CONTINUOUS".into(), visible: true }],
        blocks: vec![CadBlock {
            name: "door".into(),
            base_point: SemioPoint2 { x: 0.0, y: 0.0 },
            entities: vec![CadEntityRecord { handle: "b1".into(), layer: "0".into(), entity: CadEntity::Line { a: SemioPoint2 { x: 0.0, y: 0.0 }, b: SemioPoint2 { x: 1.0, y: 0.0 } } }],
        }],
        entities: vec![
            CadEntityRecord { handle: "h1".into(), layer: "0".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 2.0, y: 2.0 }, radius: 1.5 } },
            CadEntityRecord { handle: "h2".into(), layer: "0".into(), entity: CadEntity::Ellipse { center: SemioPoint2 { x: 1.0, y: 1.0 }, major_axis_end: SemioPoint2 { x: 4.0, y: 1.0 }, ratio: 0.5, start_param: 0.0, end_param: 6.28 } },
        ],
        ..SemioCadSnapshot::default()
    }
}

/// 🧪️ Real round trip through dxf's own real Part-21-style ASCII writer/reader
/// (`print_dxf_document`/`parse_dxf_document`) AND the sibling import leaf's mapping.
#[semio_framework_async_macros::async_test]
async fn real_text_round_trip_through_dxf_codec() {
    let cad = sample_cad();
    let dxf = semio_framework_plugin::resolve_ready(SemioCadToDxf::serialize(&cad)).expect("serialize");
    assert_eq!(dxf.tables.layers.len(), 1);
    assert_eq!(dxf.blocks.len(), 1);
    assert_eq!(dxf.entities.len(), 2);

    let text = semio_s_artifact_stdio_dxf::schema::snapshot::print_dxf_document(&dxf);
    let reparsed = semio_s_artifact_stdio_dxf::schema::snapshot::parse_dxf_document(&text).expect("reparse real dxf text");
    assert_eq!(reparsed.tables.layers.len(), 1);
    assert_eq!(reparsed.tables.layers[0].name, "0");
    assert_eq!(reparsed.blocks.len(), 1);
    assert_eq!(reparsed.blocks[0].entities.len(), 1);
    assert_eq!(reparsed.entities.len(), 2);
    assert!(matches!(reparsed.entities[0], DxfEntity::Circle { .. }));
    match &reparsed.entities[1] {
        DxfEntity::Other { kind, .. } => assert_eq!(kind, "ELLIPSE"),
        other => panic!("expected raw-retained ELLIPSE, got {other:?}"),
    }
}
