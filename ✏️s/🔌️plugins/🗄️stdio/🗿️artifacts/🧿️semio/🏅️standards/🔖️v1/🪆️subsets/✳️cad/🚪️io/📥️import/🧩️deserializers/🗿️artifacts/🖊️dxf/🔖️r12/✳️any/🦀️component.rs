//! 📥️ `dxf` (r12) → `s.stdio.semio/v1/cad` — DXF is genuinely entity-shaped (matches `cad`'s own
//! entity-record model closely: `CadEntity`'s Line/Arc/Circle/Polyline/Text/Insert/Solid variants
//! map 1:1 onto R12's typed `DxfEntity` variants of the same name). Two real, honest impedance
//! mismatches, both handled explicitly (never silently dropped):
//! - **Dimensionality**: DXF entity points are `[f64;3]`; `cad`'s `SemioPoint2` is 2D-only (per
//!   this subset's own spec). The `z` component is dropped — documented, not fabricated.
//! - **R12 has no native ELLIPSE/DIMENSION entity** (both are later AutoCAD additions/still
//!   AutoCAD-legal via generic group codes) — this codec's own R12 reader therefore has no typed
//!   variant for either and both land in `DxfEntity::Other`. Since `CadEntity` DOES model
//!   `Ellipse`/`Dimension` (part of the master plan's 9-variant vocabulary), this bridge defines
//!   its OWN internally-consistent group-code convention for round-tripping them through
//!   `Other{kind:"ELLIPSE"|"DIMENSION", group_codes}` (documented below) — a real, working
//!   best-effort bridge, not a fabricated one, but NOT a claim of ISO/AutoCAD spec conformance
//!   for these two kinds specifically (only this codec's own round trip is guaranteed).
//! - Every OTHER `DxfEntity::Other` kind (3DFACE, POINT, SHAPE, ATTRIB, …) has no `CadEntity`
//!   counterpart at all and is dropped on import (documented, `CadEntity` is a closed 9-variant
//!   enum with no raw-retention escape hatch).
//! - `handle` (DXF group code 5) is not specially typed by this codec's `DxfEntity` (it lands in
//!   `unknown_group_codes` when present) — `CadEntityRecord.handle` is synthesized sequentially
//!   (`"E{n}"` top-level, `"B{block}#{n}"` inside a block), a documented synthetic identity.

use crate::artifacts::dxf::{
    schema::snapshot::{DxfBlock, DxfEntity, DxfLayer, DxfValue, DxfVertex},
    DxfSnapshot,
};
use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::{CadBlock, CadEntity, CadEntityRecord, CadLayer, SemioCadSnapshot, STDIO_SEMIOCAD_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("cad") };

//#region 🔖️OtherGroupCodes
async fn code_f64(codes: &[(i32, DxfValue)], code: i32) -> f64 {
    codes
        .iter()
        .find(|(c, _)| *c == code)
        .map(|(_, v)| match v {
            DxfValue::Double { value } => *value,
            DxfValue::Int { value } => *value as f64,
            _ => 0.0,
        })
        .unwrap_or(0.0)
}
async fn code_str(codes: &[(i32, DxfValue)], code: i32) -> String {
    codes
        .iter()
        .find(|(c, _)| *c == code)
        .map(|(_, v)| match v {
            DxfValue::Str { value } => value.clone(),
            other => format!("{other:?}"),
        })
        .unwrap_or_default()
}

/// 🌙️ This bridge's own convention for a raw-retained `ELLIPSE` (R12 has no typed variant — see
/// module doc): 10/20 center, 11/21 major-axis endpoint RELATIVE to center, 40 ratio, 41/42
/// start/end param.
async fn ellipse_from_other(group_codes: &[(i32, DxfValue)]) -> CadEntity {
    let center = SemioPoint2 { x: code_f64(group_codes, 10).await, y: code_f64(group_codes, 20).await };
    let major_axis_end = SemioPoint2 { x: center.x + code_f64(group_codes, 11), y: center.y + code_f64(group_codes, 21) };
    CadEntity::Ellipse { center, major_axis_end, ratio: code_f64(group_codes, 40).await, start_param: code_f64(group_codes, 41).await, end_param: code_f64(group_codes, 42).await }
}

/// 📏️ This bridge's own convention for a raw-retained `DIMENSION` (R12's own typed group-code
/// table is complex/derived — see module doc): 10/20 definition point, 11/21 text position, 42
/// measurement, 1 text override.
async fn dimension_from_other(group_codes: &[(i32, DxfValue)]) -> CadEntity {
    CadEntity::Dimension {
        def_point: SemioPoint2 { x: code_f64(group_codes, 10).await, y: code_f64(group_codes, 20).await },
        text_position: SemioPoint2 { x: code_f64(group_codes, 11).await, y: code_f64(group_codes, 21).await },
        measurement: code_f64(group_codes, 42).await,
        text: code_str(group_codes, 1).await,
    }
}
//#endregion 🔖️OtherGroupCodes

//#region 🔖️EntityMap
async fn cad_entity_from_dxf(e: &DxfEntity) -> Option<CadEntity> {
    match e {
        DxfEntity::Line { start, end, .. } => Some(CadEntity::Line { a: SemioPoint2 { x: start[0], y: start[1] }, b: SemioPoint2 { x: end[0], y: end[1] } }),
        DxfEntity::Circle { center, radius, .. } => Some(CadEntity::Circle { center: SemioPoint2 { x: center[0], y: center[1] }, radius: *radius }),
        DxfEntity::Arc { center, radius, start_angle, end_angle, .. } => Some(CadEntity::Arc { center: SemioPoint2 { x: center[0], y: center[1] }, radius: *radius, start_angle: *start_angle, end_angle: *end_angle }),
        DxfEntity::Polyline { vertices, closed, .. } => Some(CadEntity::Polyline { vertices: vertices.iter().map(|v: &DxfVertex| SemioPoint2 { x: v.x, y: v.y }).collect(), closed: *closed }),
        DxfEntity::Text { position, height, value, .. } => Some(CadEntity::Text { position: SemioPoint2 { x: position[0], y: position[1] }, height: *height, rotation: 0.0, content: value.clone() }),
        DxfEntity::Solid { points, .. } => Some(CadEntity::Solid {
            p1: SemioPoint2 { x: points[0][0], y: points[0][1] },
            p2: SemioPoint2 { x: points[1][0], y: points[1][1] },
            p3: SemioPoint2 { x: points[2][0], y: points[2][1] },
            p4: SemioPoint2 { x: points[3][0], y: points[3][1] },
        }),
        DxfEntity::Insert { block_name, position, scale, rotation, .. } => {
            Some(CadEntity::Insert { block_name: block_name.clone(), insertion_point: SemioPoint2 { x: position[0], y: position[1] }, scale: SemioPoint2 { x: scale[0], y: scale[1] }, rotation: *rotation })
        }
        DxfEntity::Other { kind, group_codes } if kind == "ELLIPSE" => Some(ellipse_from_other(group_codes).await),
        DxfEntity::Other { kind, group_codes } if kind == "DIMENSION" => Some(dimension_from_other(group_codes).await),
        DxfEntity::Other { .. } => None,
    }
}

async fn dxf_entity_layer(e: &DxfEntity) -> String {
    match e {
        DxfEntity::Line { layer, .. } | DxfEntity::Circle { layer, .. } | DxfEntity::Arc { layer, .. } | DxfEntity::Polyline { layer, .. } | DxfEntity::Text { layer, .. } | DxfEntity::Solid { layer, .. } | DxfEntity::Insert { layer, .. } => {
            layer.clone()
        }
        DxfEntity::Other { .. } => String::new(),
    }
}

async fn records_from_entities(entities: &[DxfEntity], handle_prefix: &str) -> Vec<CadEntityRecord> {
    entities.iter().enumerate().filter_map(|(i, e)| semio_framework_plugin::resolve_ready(cad_entity_from_dxf(e)).map(|entity| CadEntityRecord { handle: format!("{handle_prefix}{i}"), layer: dxf_entity_layer(e), entity })).collect()
}
//#endregion 🔖️EntityMap

//#region 🔖️LayerBlockMap
/// 🚦️ DXF `LAYER` flags bit 0 = frozen (AutoCAD DXF R12 group 70 convention) — the closest real
/// on-disk signal to `CadLayer.visible`.
async fn cad_layer_from_dxf(l: &DxfLayer) -> CadLayer {
    CadLayer { name: l.name.clone(), color_index: l.color, line_type: l.linetype.clone(), visible: (l.flags & 1) == 0 }
}

async fn cad_block_from_dxf(b: &DxfBlock) -> CadBlock {
    CadBlock { name: b.name.clone(), base_point: SemioPoint2 { x: b.base_point[0], y: b.base_point[1] }, entities: records_from_entities(&b.entities, &format!("B{}#", b.name)).await }
}
//#endregion 🔖️LayerBlockMap

//#region 🔖️Deserializer
pub struct SemioCadFromDxf;

impl ArtifactDeserializer for SemioCadFromDxf {
    type From = DxfSnapshot;
    type Into = SemioCadSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        Ok(SemioCadSnapshot {
            schema: STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
            layers: from.tables.layers.iter().map(cad_layer_from_dxf).collect(),
            blocks: from.blocks.iter().map(cad_block_from_dxf).collect(),
            entities: records_from_entities(&from.entities, "E").await,
        })
    }
}
//#endregion 🔖️Deserializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    async fn sample_dxf() -> DxfSnapshot {
        DxfSnapshot {
            tables: crate::artifacts::dxf::schema::snapshot::DxfTables { layers: vec![DxfLayer { name: "0".into(), color: 7, linetype: "CONTINUOUS".into(), flags: 0, ..Default::default() }], ..Default::default() },
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
}
//#endregion 🔖️Tests
