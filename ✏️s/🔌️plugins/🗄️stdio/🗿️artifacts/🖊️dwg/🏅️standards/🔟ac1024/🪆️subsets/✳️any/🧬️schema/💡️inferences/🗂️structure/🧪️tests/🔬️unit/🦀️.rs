
use super::*;
use crate::standards::v_ac1024::subsets::any::schema::snapshot::{DwgEntityBody, DwgEntityCommon, DwgLineEntity, DwgLogicalDrawing, DwgLogicalLayer, DwgLogicalObject, DwgLogicalObjectBody, DwgObjectCategory};

#[semio_framework_async_macros::async_test]
async fn structure_matches_hand_built_logical_drawing() {
    let snapshot = DwgSnapshot {
        schema: "s.stdio.dwg".into(),
        version: "AC1024".into(),
        maintenance_version: 3,
        codepage: 30,
        drawing: DwgLogicalDrawing {
            layers: vec![DwgLogicalLayer { name: "0".into(), color: 7 }],
            objects: vec![DwgLogicalObject {
                handle: 1,
                type_code: 19,
                class_name: "LINE".into(),
                category: DwgObjectCategory::Entity,
                body: Some(DwgLogicalObjectBody::Entity(DwgEntityBody::Line(DwgLineEntity {
                    common: DwgEntityCommon { linetype_scale: 1.0, lineweight: 29, ..Default::default() },
                    start: vec![1.0, 2.0, 3.0],
                    end: vec![4.0, 5.0, 6.0],
                    thickness: 0.0,
                    extrusion: vec![0.0, 0.0, 1.0],
                }))),
                ..Default::default()
            }],
            ..Default::default()
        },
        ..Default::default()
    };
    let structure = compute_dwg_structure(&snapshot);
    assert_eq!(structure.layer_count, 1);
    assert_eq!(structure.entity_count, 1);
    assert_eq!(structure.geometry_value_count, 6);
    assert_eq!(structure.geometry_index_count, 0);
    assert_eq!(structure.text_character_count, 0);
    assert_eq!(structure.codepage, 30);
    assert_eq!(structure.version, "AC1024");
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = DwgSnapshot { schema: "s.stdio.dwg".into(), version: "AC1024".into(), codepage: 30, ..Default::default() };
    assert_eq!(compute_dwg_structure(&snapshot), compute_dwg_structure(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_dwg_structure(&DwgSnapshot::default()), DwgStructure::default());
}
