//! 📤️ `s.stdio.semio/v1/cad` → `dxf` (r12) — mirrors the import leaf's entity map, using the SAME
//! bridge-owned `ELLIPSE`/`DIMENSION` group-code convention documented there for the two
//! `CadEntity` variants R12 has no typed entity for. `z` is always written `0.0` (cad is 2D-only —
//! see the import leaf's dimensionality note). Emits a minimal, real, spec-legal R12 header
//! (`$ACADVER AC1009`) plus a `LAYER` table built from `layers` — `print_dxf_document` (dxf's own
//! writer) needs no MORE than that for a structurally valid file.

use crate::artifacts::dxf::{
    schema::snapshot::{DxfBlock, DxfEntity, DxfHeaderVar, DxfLayer, DxfTables, DxfValue},
    DxfSnapshot,
};
use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::{CadBlock, CadEntity, CadEntityRecord, SemioCadSnapshot};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("cad") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId::ANY };

//#region 🔖️OtherGroupCodes
fn ellipse_to_other(center: &SemioPoint2, major_axis_end: &SemioPoint2, ratio: f64, start_param: f64, end_param: f64) -> DxfEntity {
    DxfEntity::Other {
        kind: "ELLIPSE".into(),
        group_codes: vec![
            (10, DxfValue::Double { value: center.x }),
            (20, DxfValue::Double { value: center.y }),
            (11, DxfValue::Double { value: major_axis_end.x - center.x }),
            (21, DxfValue::Double { value: major_axis_end.y - center.y }),
            (40, DxfValue::Double { value: ratio }),
            (41, DxfValue::Double { value: start_param }),
            (42, DxfValue::Double { value: end_param }),
        ],
    }
}
fn dimension_to_other(def_point: &SemioPoint2, text_position: &SemioPoint2, measurement: f64, text: &str) -> DxfEntity {
    DxfEntity::Other {
        kind: "DIMENSION".into(),
        group_codes: vec![
            (10, DxfValue::Double { value: def_point.x }),
            (20, DxfValue::Double { value: def_point.y }),
            (11, DxfValue::Double { value: text_position.x }),
            (21, DxfValue::Double { value: text_position.y }),
            (42, DxfValue::Double { value: measurement }),
            (1, DxfValue::Str { value: text.to_string() }),
        ],
    }
}
//#endregion 🔖️OtherGroupCodes

//#region 🔖️EntityMap
fn dxf_entity_from_cad(rec: &CadEntityRecord) -> DxfEntity {
    let layer = rec.layer.clone();
    match &rec.entity {
        CadEntity::Line { a, b } => DxfEntity::Line { start: [a.x, a.y, 0.0], end: [b.x, b.y, 0.0], layer, unknown_group_codes: vec![] },
        CadEntity::Circle { center, radius } => DxfEntity::Circle { center: [center.x, center.y, 0.0], radius: *radius, layer, unknown_group_codes: vec![] },
        CadEntity::Arc { center, radius, start_angle, end_angle } => DxfEntity::Arc { center: [center.x, center.y, 0.0], radius: *radius, start_angle: *start_angle, end_angle: *end_angle, layer, unknown_group_codes: vec![] },
        CadEntity::Ellipse { center, major_axis_end, ratio, start_param, end_param } => ellipse_to_other(center, major_axis_end, *ratio, *start_param, *end_param),
        CadEntity::Polyline { vertices, closed } => DxfEntity::Polyline {
            vertices: vertices.iter().map(|v| crate::artifacts::dxf::schema::snapshot::DxfVertex { x: v.x, y: v.y, z: 0.0, bulge: 0.0, unknown_group_codes: vec![] }).collect(),
            closed: *closed,
            layer,
            unknown_group_codes: vec![],
        },
        CadEntity::Text { position, height, content, .. } => DxfEntity::Text { position: [position.x, position.y, 0.0], height: *height, value: content.clone(), layer, unknown_group_codes: vec![] },
        CadEntity::Insert { block_name, insertion_point, scale, rotation } => {
            DxfEntity::Insert { block_name: block_name.clone(), position: [insertion_point.x, insertion_point.y, 0.0], scale: [scale.x, scale.y, 1.0], rotation: *rotation, layer, unknown_group_codes: vec![] }
        }
        CadEntity::Solid { p1, p2, p3, p4 } => DxfEntity::Solid { points: [[p1.x, p1.y, 0.0], [p2.x, p2.y, 0.0], [p3.x, p3.y, 0.0], [p4.x, p4.y, 0.0]], layer, unknown_group_codes: vec![] },
        CadEntity::Dimension { def_point, text_position, measurement, text } => dimension_to_other(def_point, text_position, *measurement, text),
    }
}
//#endregion 🔖️EntityMap

//#region 🔖️LayerBlockMap
fn dxf_layer_from_cad(l: &crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::CadLayer) -> DxfLayer {
    DxfLayer { name: l.name.clone(), color: l.color_index, linetype: l.line_type.clone(), flags: if l.visible { 0 } else { 1 }, unknown_group_codes: vec![] }
}
fn dxf_block_from_cad(b: &CadBlock) -> DxfBlock {
    DxfBlock { name: b.name.clone(), base_point: [b.base_point.x, b.base_point.y, 0.0], entities: b.entities.iter().map(dxf_entity_from_cad).collect(), unknown_group_codes: vec![] }
}
//#endregion 🔖️LayerBlockMap

//#region 🔖️Serializer
pub struct SemioCadToDxf;

impl ArtifactSerializer for SemioCadToDxf {
    type From = SemioCadSnapshot;
    type Into = DxfSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        Ok(DxfSnapshot {
            schema: crate::artifacts::dxf::STDIO_DXF_DOCUMENT_SCHEMA.into(),
            header_vars: vec![DxfHeaderVar { name: "$ACADVER".into(), group_code: 1, value: DxfValue::Str { value: "AC1009".into() }, extra_group_codes: vec![] }],
            tables: DxfTables { layers: from.layers.iter().map(dxf_layer_from_cad).collect(), ..DxfTables::default() },
            other_tables: vec![],
            blocks: from.blocks.iter().map(dxf_block_from_cad).collect(),
            entities: from.entities.iter().map(dxf_entity_from_cad).collect(),
        })
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::CadLayer;

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
    #[test]
    fn real_text_round_trip_through_dxf_codec() {
        let cad = sample_cad();
        let dxf = semio_framework_plugin::resolve_ready(SemioCadToDxf::serialize(&cad)).expect("serialize");
        assert_eq!(dxf.tables.layers.len(), 1);
        assert_eq!(dxf.blocks.len(), 1);
        assert_eq!(dxf.entities.len(), 2);

        let text = crate::artifacts::dxf::schema::snapshot::print_dxf_document(&dxf);
        let reparsed = crate::artifacts::dxf::schema::snapshot::parse_dxf_document(&text).expect("reparse real dxf text");
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
}
//#endregion 🔖️Tests
