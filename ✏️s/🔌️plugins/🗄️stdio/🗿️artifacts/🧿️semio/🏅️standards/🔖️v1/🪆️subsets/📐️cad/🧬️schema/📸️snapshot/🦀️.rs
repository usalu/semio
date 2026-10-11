//! 🧬️ SemioCadSnapshot — layers/blocks/entities, informed by dxf r12's typed entity list, dwg's
//! `DwgDrawing`/`DwgEntity`/`DwgGeometry`, and the 📐️cad plugin's domain artifact (master plan
//! "Subset snapshot cores" table, `cad` row). `CadEntity` carries the full 9-variant vocabulary
//! (Line/Arc/Circle/Ellipse/Polyline/Text/Insert/Solid/Dimension).
use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use framework_schema::ArtifactSchema;
//#region 🔖️Ids
pub const STDIO_SEMIOCAD_DOCUMENT_SCHEMA: &str = "stdio.semio.cad";
//#endregion 🔖️Ids
//#region 🔖️Entity
/// 📐️ Owned by the `cad` subset — a WEAK value struct (see `🔺️diff`'s module doc comment): whole-
/// value replaced in diffs, never sub-diffed, same treatment as `BcfCamera`/`XlsxCellValue`.
///
/// 🧪️ `Default` (with `Line` as the zero-length degenerate default) is required here, not for any
/// domain reason, but because the shared `engine::triples::NamedTripleDiff<K,D,T>`'s
/// `#[derive(ToValue, FromValue)]` synthesizes a per-type-parameter bound (`T: ToValue`/
/// `FromValue`, see `🌱️value/✨️derive`'s module docs) plus this file's own `added: Vec<T>` field
/// needing `T: Default` wherever the missing-key fallback runs — bcf's local `NamedTripleDiff`
/// copy carries the identical requirement (see that file's own doc comment).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, value_derive::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum CadEntity {
    Line { a: SemioPoint2, b: SemioPoint2 },
    Arc { center: SemioPoint2, radius: f64, start_angle: f64, end_angle: f64 },
    Circle { center: SemioPoint2, radius: f64 },
    Ellipse { center: SemioPoint2, major_axis_end: SemioPoint2, ratio: f64, start_param: f64, end_param: f64 },
    Polyline { vertices: Vec<SemioPoint2>, closed: bool },
    Text { position: SemioPoint2, height: f64, rotation: f64, content: String },
    Insert { block_name: String, insertion_point: SemioPoint2, scale: SemioPoint2, rotation: f64 },
    Solid { p1: SemioPoint2, p2: SemioPoint2, p3: SemioPoint2, p4: SemioPoint2 },
    Dimension { def_point: SemioPoint2, text_position: SemioPoint2, measurement: f64, text: String },
}
/// 🧭️ Manual impl (not `#[derive(Default)]`) -- `Default` on an enum requires a UNIT default
/// variant, but every `CadEntity` variant carries fields, so the derive attribute is structurally
/// rejected here; hand-written zero-length `Line` matches what a derive-with-unit-variant would
/// have produced field-by-field if it were allowed. See this type's own doc comment above for WHY
/// `Default` is needed at all (the shared `engine::triples` spurious-bound workaround).
impl Default for CadEntity {
    fn default() -> Self {
        CadEntity::Line { a: SemioPoint2::default(), b: SemioPoint2::default() }
    }
}
//#endregion 🔖️Entity
//#region 🔖️Layer
/// 🗂️ Name-keyed (dxf `TABLES/LAYER`-style) — strong entity, own per-field diff. `Default` is the
/// same spurious-bound workaround `CadEntity` documents above.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct CadLayer {
    pub name: String,
    pub color_index: i32,
    pub line_type: String,
    pub visible: bool,
}
//#endregion 🔖️Layer
//#region 🔖️EntityRecord
/// 🏷️ One placed entity — `handle` is the id key (dxf group code 5); `layer` names the owning
/// `CadLayer` by reference. Referential invariants (dangling `layer`/`Insert.block_name`) are
/// checked by the composer's `SemioCadValidator`, not enforced structurally here.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct CadEntityRecord {
    pub handle: String,
    pub layer: String,
    pub entity: CadEntity,
}
//#endregion 🔖️EntityRecord
//#region 🔖️Block
/// 📦️ Name-keyed (dxf `BLOCKS` section) — strong entity; `entities` is its own nested id-keyed
/// collection (same shape as the top-level `entities`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct CadBlock {
    pub name: String,
    pub base_point: SemioPoint2,
    #[value(default)]
    pub entities: Vec<CadEntityRecord>,
}
//#endregion 🔖️Block
//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.cad")]
pub struct SemioCadSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub layers: Vec<CadLayer>,
    #[state(artifact)]
    #[value(default)]
    pub blocks: Vec<CadBlock>,
    #[state(artifact)]
    #[value(default)]
    pub entities: Vec<CadEntityRecord>,
}
impl Default for SemioCadSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(), layers: Vec::new(), blocks: Vec::new(), entities: Vec::new() }
    }
}
//#endregion 🔖️Snapshot
//#region 🔖️TextPrimitives
//#endregion 🔖️TextPrimitives
//#region 🔖️BinaryPrimitives
//#endregion 🔖️BinaryPrimitives
//#region 🔖️HandcraftedArtifactCodecs
//#endregion 🔖️HandcraftedArtifactCodecs
//#region 🌉️ExternalCodecBridge
//#endregion 🌉️ExternalCodecBridge
//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.cad` document — a small floor-plan-shaped drawing exercising every
/// collection AND every `CadEntity` variant at least once (a `door` block with a nested `Line`,
/// plus a top-level `Arc`/`Circle`/`Ellipse`/`Polyline`/`Text`/`Insert`/`Solid`/`Dimension`). Single
/// source of truth for `📚️examples/📐️drawing/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio`
/// and for the conformance-law tests in `🎹️composer/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_cad_snapshot() -> SemioCadSnapshot {
    SemioCadSnapshot {
        schema: STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
        layers: vec![CadLayer { name: "0".into(), color_index: 7, line_type: "CONTINUOUS".into(), visible: true }, CadLayer { name: "dim".into(), color_index: 1, line_type: "DASHED".into(), visible: true }],
        blocks: vec![CadBlock {
            name: "door".into(),
            base_point: SemioPoint2 { x: 0.0, y: 0.0 },
            entities: vec![CadEntityRecord { handle: "be1".into(), layer: "0".into(), entity: CadEntity::Line { a: SemioPoint2 { x: 0.0, y: 0.0 }, b: SemioPoint2 { x: 1.0, y: 0.0 } } }],
        }],
        entities: vec![
            CadEntityRecord { handle: "h1".into(), layer: "0".into(), entity: CadEntity::Arc { center: SemioPoint2 { x: 2.0, y: 2.0 }, radius: 1.0, start_angle: 0.0, end_angle: 180.0 } },
            CadEntityRecord { handle: "h2".into(), layer: "0".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 5.0, y: 5.0 }, radius: 2.0 } },
            CadEntityRecord { handle: "h3".into(), layer: "0".into(), entity: CadEntity::Ellipse { center: SemioPoint2 { x: 0.0, y: 0.0 }, major_axis_end: SemioPoint2 { x: 3.0, y: 0.0 }, ratio: 0.5, start_param: 0.0, end_param: 6.283 } },
            CadEntityRecord { handle: "h4".into(), layer: "0".into(), entity: CadEntity::Polyline { vertices: vec![SemioPoint2 { x: 0.0, y: 0.0 }, SemioPoint2 { x: 1.0, y: 1.0 }, SemioPoint2 { x: 2.0, y: 0.0 }], closed: true } },
            CadEntityRecord { handle: "h5".into(), layer: "0".into(), entity: CadEntity::Text { position: SemioPoint2 { x: 0.0, y: 0.0 }, height: 2.5, rotation: 0.0, content: "Room 101".into() } },
            CadEntityRecord { handle: "h6".into(), layer: "0".into(), entity: CadEntity::Insert { block_name: "door".into(), insertion_point: SemioPoint2 { x: 10.0, y: 10.0 }, scale: SemioPoint2 { x: 1.0, y: 1.0 }, rotation: 90.0 } },
            CadEntityRecord { handle: "h7".into(), layer: "0".into(), entity: CadEntity::Solid { p1: SemioPoint2 { x: 0.0, y: 0.0 }, p2: SemioPoint2 { x: 1.0, y: 0.0 }, p3: SemioPoint2 { x: 1.0, y: 1.0 }, p4: SemioPoint2 { x: 0.0, y: 1.0 } } },
            CadEntityRecord { handle: "h8".into(), layer: "dim".into(), entity: CadEntity::Dimension { def_point: SemioPoint2 { x: 0.0, y: 0.0 }, text_position: SemioPoint2 { x: 1.0, y: 1.0 }, measurement: 4.2, text: "4.20m".into() } },
        ],
    }
}
//#endregion 🔖️Demo
//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
                                                                