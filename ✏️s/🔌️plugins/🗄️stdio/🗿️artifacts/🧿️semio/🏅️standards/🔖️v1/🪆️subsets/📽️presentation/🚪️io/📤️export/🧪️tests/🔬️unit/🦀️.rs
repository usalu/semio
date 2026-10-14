//! 🔁️ Round trips of the `presentation` subset through each format: import, export, import again.

use crate::serializers::artifacts::pptx::v_ecma_376::any::SemioPresentationToPptx;
use semio_s_artifact_stdio_semio_presentation_import::deserializers::artifacts::pptx::v_ecma_376::any::SemioPresentationFromPptx;
use semio_framework_plugin::{ArtifactDeserializer, ArtifactSerializer};

/// 🔁️ W4 G6 fixture-backed round trip: pptx1 -(deserialize)-> semio1 -(serialize)-> pptx2
/// -(deserialize)-> semio2, asserting semio1 == semio2 (masters/layouts/slide id/notes/Table
/// shapes are the documented lossy fields — this fixture avoids them by construction, so the
/// comparison exercises TextBox/Picture/Placeholder shape fidelity end to end).
#[semio_framework_async_macros::async_test]
async fn pptx_round_trip_is_stable() {
    use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::PptxParagraph;
    use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::PptxPresentation;
    use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::PptxRun;
    use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::PptxShape;
    use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::PptxSlide;
    use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::PptxTransform;
    use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::construction::minimal::build_minimal_pptx;

    let pptx1 = build_minimal_pptx(
        PptxPresentation {
            slides: vec![PptxSlide {
                shapes: vec![
                    PptxShape::TextBox { text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "Hello".into(), bold: true, italic: false, font_size: Some(24) }] }], position: PptxTransform { x: 0, y: 0, cx: 100, cy: 20 } },
                    PptxShape::Picture { blip_rel_id: "rId2".into(), position: PptxTransform { x: 0, y: 30, cx: 50, cy: 50 } },
                    PptxShape::Placeholder { kind: "title".into(), text_frame: Vec::new(), position: PptxTransform { x: 0, y: 0, cx: 200, cy: 40 } },
                ],
            }],
        },
    );
    let semio1 = ::semio_framework_async::poll::resolve_ready(SemioPresentationFromPptx::deserialize(&pptx1)).expect("deserialize");
    let pptx2 = ::semio_framework_async::poll::resolve_ready(SemioPresentationToPptx::serialize(&semio1)).expect("serialize");
    let semio2 = ::semio_framework_async::poll::resolve_ready(SemioPresentationFromPptx::deserialize(&pptx2)).expect("deserialize round 2");
    assert_eq!(semio1, semio2);
}
