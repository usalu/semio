use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_dxf() -> DxfSnapshot {
    DxfSnapshot {
        tables: semio_s_artifact_stdio_dxf::schema::snapshot::DxfTables { layers: vec![DxfLayer { name: "0".into(), color: 7, linetype: "CONTINUOUS".into(), flags: 0, ..Default::default() }], ..Default::default() },
        blocks: vec![DxfBlock { name: "door".into(), base_point: [0.0, 0.0, 0.0], entities: vec![DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 0.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] }], unknown_group_codes: vec![] }],
        entities: vec![
            DxfEntity::Circle { center: [2.0, 2.0, 0.0], radius: 1.5, layer: "0".into(), unknown_group_codes: vec![] },
            DxfEntity::Insert { block_name: "door".into(), position: [5.0, 5.0, 0.0], scale: [1.0, 1.0, 1.0], rotation: 90.0, layer: "0".into(), unknown_group_codes: vec![] },
            DxfEntity::Other {
                kind: "ELLIPSE".into(),
                group_codes: vec![
                    (10, DxfValue::Double { value: 1.0 }),
                    (20, DxfValue::Double { value: 1.0 }),
                    (11, DxfValue::Double { value: 3.0 }),
                    (21, DxfValue::Double { value: 0.0 }),
                    (40, DxfValue::Double { value: 0.5 }),
                    (41, DxfValue::Double { value: 0.0 }),
                    (42, DxfValue::Double { value: 6.28 }),
                ],
            },
            DxfEntity::Other { kind: "3DFACE".into(), group_codes: vec![] },
        ],
        ..DxfSnapshot::default()
    }
}

#[semio_framework_async_macros::async_test]
async fn maps_layers_blocks_and_entities() {
    let cad = semio_framework_plugin::resolve_ready(SemioCadFromDxf::deserialize(&sample_dxf())).expect("deserialize");
    assert_eq!(cad.layers.len(), 1);
    assert_eq!(cad.layers[0].visible, true);
    assert_eq!(cad.blocks.len(), 1);
    assert_eq!(cad.blocks[0].entities.len(), 1);
    assert!(matches!(cad.blocks[0].entities[0].entity, CadEntity::Line { .. }));
    // circle, insert, ellipse map; 3DFACE (Other, unmodeled kind) is honestly dropped
    assert_eq!(cad.entities.len(), 3);
    assert!(matches!(cad.entities[0].entity, CadEntity::Circle { .. }));
    assert!(matches!(cad.entities[1].entity, CadEntity::Insert { .. }));
    match &cad.entities[2].entity {
        CadEntity::Ellipse { center, major_axis_end, ratio, .. } => {
            assert_eq!(*center, SemioPoint2 { x: 1.0, y: 1.0 });
            assert_eq!(*major_axis_end, SemioPoint2 { x: 4.0, y: 1.0 });
            assert_eq!(*ratio, 0.5);
        }
        other => panic!("expected Ellipse, got {other:?}"),
    }
}
