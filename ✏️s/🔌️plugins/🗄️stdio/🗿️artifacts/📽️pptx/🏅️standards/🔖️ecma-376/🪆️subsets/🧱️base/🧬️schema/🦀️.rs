//! 🧬️ PptxArtifact schema — full artifact state.

use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{PptxPresentation, PptxXmlPart};
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::PptxSnapshot;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_zip::opc::OpcPackage;


//#region Artifact
/// 🧬️ Full `stdio.pptx` artifact state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pptx")]
pub struct PptxArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub opc: OpcPackage,
    #[state(artifact)]
    #[value(default)]
    pub xml_parts: Vec<PptxXmlPart>,
}
//#endregion Artifact

//#region Conversions
impl Default for PptxArtifact {
    fn default() -> Self {
        Self::from_snapshot(PptxSnapshot::default())
    }
}

impl PptxArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> PptxSnapshot {
        PptxSnapshot { schema: self.schema.clone(), opc: self.opc.clone(), xml_parts: self.xml_parts.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: PptxSnapshot) -> Self {
        Self { schema: snapshot.schema, opc: snapshot.opc, xml_parts: snapshot.xml_parts }
    }
}
//#endregion Conversions

//#region Descriptor
//#endregion Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🔖️DocumentHelpers
/// 🆕️ A new pptx document: the minimal presentation package the real writer builds (`build_minimal_pptx`, docx's
/// precedent) — the empty `Default` package saved as a materialized package and reopened as a different document.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn blank_pptx_snapshot() -> PptxSnapshot {
    construction::minimal::build_minimal_pptx(PptxPresentation::default())
}

/// 📄️ FG-wave: the demo `stdio.pptx` presentation — a genuinely non-trivial `PptxSnapshot`
/// exercising a title `Placeholder` (bold run), a `Picture`, a `TextBox` with mixed bold/italic
/// runs across two paragraphs, and one logical `Other` shape (`p:graphicFrame`), plus one
/// semantically binary OPC part (`ppt/media/image1.png`). The
/// single source of truth for `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/
/// `🎒️.pack.semio` (both are literally this snapshot's `print_dsl`/`encode_pack` output,
/// asserted equal by `fixture_honesty_law` below) — same shape docx's own `demo_docx_snapshot()`
/// establishes.
pub async fn demo_pptx_snapshot() -> PptxSnapshot {
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{PptxParagraph, PptxRun, PptxShape, PptxSlide, PptxTransform};
    use crate::standards::v_ecma_376::subsets::base::schema::construction::minimal::build_minimal_pptx;
        let presentation = PptxPresentation {
        slides: vec![
            PptxSlide {
                shapes: vec![
                    PptxShape::Placeholder {
                        kind: "title".into(),
                        text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "Semio Demo".into(), bold: true, italic: false, font_size: Some(44) }] }],
                        position: PptxTransform { x: 685800, y: 457200, cx: 7772400, cy: 1143000 },
                    },
                    PptxShape::Picture { blip_rel_id: "rId2".into(), position: PptxTransform { x: 685800, y: 1600200, cx: 2286000, cy: 1714500 } },
                ],
            },
            PptxSlide {
                shapes: vec![
                    PptxShape::TextBox {
                        text_frame: vec![
                            PptxParagraph { runs: vec![PptxRun { text: "Bold and ".into(), bold: true, italic: false, font_size: None }, PptxRun { text: "italic".into(), bold: false, italic: true, font_size: None }] },
                            PptxParagraph::text("second paragraph"),
                        ],
                        position: PptxTransform { x: 685800, y: 457200, cx: 7772400, cy: 2286000 },
                    },
                    PptxShape::Other {
                        node: semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { name: "p:graphicFrame".into(), attrs: Vec::new(), children: vec![semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { name: "p:nvGraphicFramePr".into(), attrs: Vec::new(), children: vec![semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { name: "p:cNvPr".into(), attrs: vec![semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr { name: "id".into(), value: "9".into() }, semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr { name: "name".into(), value: "Table 1".into() }], children: Vec::new() }] }] },
                    },
                ],
            },
        ],
    };
    let mut snap = build_minimal_pptx(presentation);
    snap.opc.set_part("ppt/media/image1.png", "image/png", b"\x89PNG\r\n\x1a\n".to_vec());
    snap
}
//#endregion 🔖️DocumentHelpers

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
//#endregion 🧪️Tests

