//! 📜️ Raster artifact — textual document grammar surface + laws (constitutional: dsl).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::raster::RasterSnapshot;

/// 📄️ The `semio` example document, handcrafted in the `.raster` DSL.
pub const SEMIO_RASTER_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.raster` DSL text into a `RasterSnapshot`.
pub fn parse_dsl(text: &str) -> Result<RasterSnapshot, store::TextError> {
    <RasterSnapshot as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `RasterSnapshot` back to `.raster` DSL text.
pub fn print_dsl(document: &RasterSnapshot) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::raster::{RasterImageAsset, RasterLayerMask, RasterLayerNode, RasterTransform, RASTER_DOCUMENT_SCHEMA};
    use std::collections::BTreeMap;

    /// 📄️ Handcrafted document exercising every layer kind/field, shared with the `pack`/`op`
    /// taxonomy nodes' own copies (each node keeps its own private copy, per §7 test isolation).
    fn representative_raster_document() -> RasterSnapshot {
        let mut assets = BTreeMap::new();
        assets.insert("asset-1".into(), RasterImageAsset { mime: "image/png".into(), data: b"abc".to_vec() });
        let mut params = BTreeMap::new();
        params.insert("brightness".into(), dsl::to_dsl_value(&serde_json::json!(0.06)).expect("dsl value"));
        params.insert("label".into(), dsl::to_dsl_value(&serde_json::json!("Warm \"Curve\"")).expect("dsl value"));
        params.insert("enabled".into(), dsl::to_dsl_value(&serde_json::json!(true)).expect("dsl value"));
        params.insert("fallback".into(), dsl::DslValue::Null);
        params.insert("curves".into(), dsl::to_dsl_value(&serde_json::json!([[0.0, 0.0], [0.25, 0.2], [1.0, 1.0]])).expect("dsl value"));
        params.insert("nested".into(), dsl::to_dsl_value(&serde_json::json!({ "inner": 1.5 })).expect("dsl value"));
        RasterSnapshot {
            schema: RASTER_DOCUMENT_SCHEMA.into(),
            id: "doc-1".into(),
            title: Some("Representative \"Doc\"".into()),
            assets,
            layers: vec![
                RasterLayerNode::Pixel {
                    id: "pixel-1".into(),
                    name: "Pixel One".into(),
                    visible: true,
                    opacity: 1.0,
                    blend_mode: "normal".into(),
                    transform: RasterTransform::default(),
                    mask: Some(RasterLayerMask { enabled: true, linked: false, invert: true, width: Some(64), height: None }),
                    width: Some(256),
                    height: Some(256),
                    image_key: Some("asset-1".into()),
                },
                RasterLayerNode::Group {
                    id: "group-1".into(),
                    name: "Group / Nested".into(),
                    visible: false,
                    opacity: 0.5,
                    blend_mode: "screen".into(),
                    transform: RasterTransform { x: 1.0, y: -2.0, scale_x: 1.5, scale_y: 0.5, rotation: 12.0 },
                    mask: None,
                    children: vec![
                        RasterLayerNode::Pixel {
                            id: "pixel-2".into(),
                            name: "Child Pixel".into(),
                            visible: true,
                            opacity: 0.75,
                            blend_mode: "multiply".into(),
                            transform: RasterTransform::default(),
                            mask: None,
                            width: None,
                            height: None,
                            image_key: None,
                        },
                        RasterLayerNode::Group { id: "group-2".into(), name: "Nested Group".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: Vec::new() },
                    ],
                },
                RasterLayerNode::Adjustment { id: "adjust-1".into(), name: "Curves & Co".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "curves".into(), params },
            ],
        }
    }

    #[test]
    fn semio_example_dsl_round_trips() {
        let fixture = crate::artifacts::raster::engine::semio_fixture_snapshot();
        store::os_store::test_support::assert_dsl_round_trip(&fixture);
        let printed = print_dsl(&fixture);
        let reparsed = parse_dsl(&printed).expect("parse printed semio fixture");
        assert_eq!(reparsed.id, fixture.id);
    }

    #[test]
    fn raster_dsl_round_trips_representative_document() {
        store::os_store::test_support::assert_dsl_round_trip(&representative_raster_document());
    }
}
//#endregion 🧪️Tests
