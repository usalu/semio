//! 🧬️ PptxArtifact schema — full artifact state.

use crate::artifacts::pptx::schema::snapshot::{PptxPresentation, PptxXmlPart};
use crate::artifacts::pptx::PptxSnapshot;
use crate::artifacts::zip::opc::OpcPackage;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region Artifact
/// 🧬️ Full `stdio.pptx` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pptx")]
pub struct PptxArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub opc: OpcPackage,
    #[state(artifact)]
    #[serde(default)]
    pub xml_parts: Vec<PptxXmlPart>,
    #[state(artifact)]
    #[serde(default)]
    pub presentation: PptxPresentation,
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
        PptxSnapshot { schema: self.schema.clone(), opc: self.opc.clone(), xml_parts: self.xml_parts.clone(), presentation: self.presentation.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: PptxSnapshot) -> Self {
        Self { schema: snapshot.schema, opc: snapshot.opc, xml_parts: snapshot.xml_parts, presentation: snapshot.presentation }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: PptxSnapshot) {
        self.schema = snapshot.schema;
        self.opc = snapshot.opc;
        self.xml_parts = snapshot.xml_parts;
        self.presentation = snapshot.presentation;
    }
}
//#endregion Conversions

//#region Descriptor
/// 🧬️ Descriptor for `s.stdio.pptx`.
pub fn pptx_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.pptx",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::pptx::schema::snapshot::{PptxParagraph, PptxRun, PptxShape, PptxSlide, PptxTransform};
    use crate::artifacts::pptx::{PptxDiff, PptxMutation, PptxSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.pptx` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct PptxBuilderConstruction {
        snapshot: PptxSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for PptxBuilderConstruction {
        type Snapshot = PptxSnapshot;
        type Mutation = PptxMutation;
        type Diff = PptxDiff;
        fn empty() -> Self {
            Self { snapshot: PptxSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<PptxSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<PptxSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::pptx::schema::mutations::apply_pptx_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <PptxDiff as protocol::MutationDiff<PptxSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder

    //#region 🔖️TypedConstructors
    /// 🧱️ Typed content constructors — build a presentation from slides of paragraphs/runs with
    /// basic formatting (bold/italic), the same shape as `docx::DocxBuilder`'s constructors.
    impl PptxBuilderConstruction {
        /// ➕️ Appends a new (initially empty) slide and makes it the active slide for `add_paragraph`.
        pub async fn add_slide(mut self) -> Self {
            self.snapshot.presentation.slides.push(PptxSlide::default());
            self.rebuild().await
        }

        /// ➕️ Appends a paragraph to the active slide's active `TextBox` shape (the most recently
        /// added one), creating a fresh `TextBox` shape first if the slide has none yet or its last
        /// shape isn't one.
        pub async fn add_paragraph(mut self, paragraph: PptxParagraph) -> Self {
            if let Some(slide) = self.snapshot.presentation.slides.last_mut() {
                match slide.shapes.last_mut() {
                    Some(PptxShape::TextBox { text_frame, .. }) => text_frame.push(paragraph),
                    _ => slide.shapes.push(PptxShape::TextBox { text_frame: vec![paragraph], position: PptxTransform::default() }),
                }
            }
            self.rebuild().await
        }

        /// ➕️ Appends a single-run plain-text paragraph to the active slide.
        pub async fn add_text_paragraph(self, text: impl Into<String>) -> Self {
            self.add_paragraph(PptxParagraph::text(text.into())).await
        }

        /// ➕️ Appends a paragraph made of the given runs (basic bold/italic formatting).
        pub async fn add_runs(self, runs: Vec<PptxRun>) -> Self {
            self.add_paragraph(PptxParagraph { runs }).await
        }

        async fn rebuild(mut self) -> Self {
            self.snapshot = crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_pptx(self.snapshot.presentation);
            self
        }
    }
    //#endregion 🔖️TypedConstructors
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::pptx::PptxSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.pptx` parts.
    #[derive(Clone, Debug, Default)]
    pub struct PptxParts {
        pub snapshot: Option<PptxSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.pptx` (ecma-376/✳️any) sources.
    pub struct PptxAnalyzerAnalysis;

    impl ArtifactAnalysis for PptxAnalyzerAnalysis {
        type Parts = PptxParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            // 🕵️ Real sniff: OPC-shaped bytes whose root officeDocument relationship resolves under
            // `ppt/` — disambiguates from docx/xlsx, which share the same zip magic and OPC shape.
            match source {
                AnalyzeSource::Binary(bytes) if crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::import::deserializers::sniff_pptx_bytes(bytes) => IoConfidence::High,
                AnalyzeSource::Binary(_) | AnalyzeSource::Text(_) => IoConfidence::Low,
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = PptxParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <PptxSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => {
                        let result = if crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::import::deserializers::sniff_pptx_bytes(bytes) {
                            crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::import::deserializers::decode_pptx(bytes).map_err(|err| err.to_string())
                        } else {
                            <PptxSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|err| err.to_string())
                        };
                        match result {
                            Ok(snapshot) => parts.snapshot = Some(snapshot),
                            Err(err) => {
                                confidence = IoConfidence::Low;
                                diagnostics.push(dsl::Diagnostic::error("stdio.analyze.binary", dsl::TextSpan::at(1, 1), err));
                            }
                        }
                    }
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🔖️DocumentHelpers
pub async fn empty_pptx_snapshot() -> PptxSnapshot {
    PptxSnapshot::default()
}

/// 📄️ FG-wave: the demo `stdio.pptx` presentation — a genuinely non-trivial `PptxSnapshot`
/// exercising a title `Placeholder` (bold run), a `Picture`, a `TextBox` with mixed bold/italic
/// runs across two paragraphs, and one logical `Other` shape (`p:graphicFrame`), plus one
/// semantically binary OPC part (`ppt/media/image1.png`). The
/// single source of truth for `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/
/// `🎒️example.pack.semio` (both are literally this snapshot's `print_dsl`/`encode_pack` output,
/// asserted equal by `fixture_honesty_law` below) — same shape docx's own `demo_docx_snapshot()`
/// establishes.
pub async fn demo_pptx_snapshot() -> PptxSnapshot {
    use crate::artifacts::pptx::schema::snapshot::{PptxParagraph, PptxRun, PptxShape, PptxSlide, PptxTransform};
    use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::export::serializers::{build_minimal_pptx, encode_pptx};
    use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::import::deserializers::decode_pptx;
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
                    // 🩹 Deliberately no `<a:graphic/>` child here: an UNATTRIBUTED self-closing
                    // element (real bytes `<a:graphic/>`, no space) would hit the SAME lexer
                    // identifier-fusion property this file's own grammar documents for `p:nvPr`/
                    // `p:grpSpPr`/etc (`"cNvGrpSpPr/"` fuses into ONE token) -- but the GENERIC
                    // `x-elem` logical fallback (unlike this artifact's own TYPED shape
                    // productions, which model every real fused case with an explicit literal
                    // token) has no way to disambiguate "bare self-close" from "open tag, more
                    // content follows" using only same-shape `LT x-name GT` lookahead -- a
                    // genuine, documented limitation of the x-elem restatement (same one docx's
                    // own snapshot grammar's `x-elem` inherits), not something this demo fixture
                    // should paper over by accident. Keeping every attr non-empty here keeps the
                    // conformance law honest without exercising that known gap.
                    PptxShape::Other {
                        node: crate::artifacts::xml::schema::snapshot::xml_document_from_text(r#"<p:graphicFrame><p:nvGraphicFramePr><p:cNvPr id="9" name="Table 1"/></p:nvGraphicFramePr></p:graphicFrame>"#)
                            .expect("valid logical fallback XML")
                            .root
                            .expect("fallback XML root"),
                    },
                ],
            },
        ],
    };
    let mut snap = build_minimal_pptx(presentation);
    snap.opc.set_part("ppt/media/image1.png", "image/png", b"\x89PNG\r\n\x1a\n".to_vec());
    // 🩹 Normalize the authored binary media plus logical XML through the same deterministic
    // materialization/deserialization boundary used by native I/O.
    let canonical_bytes = encode_pptx(&snap).expect("encode demo pptx for order canonicalization");
    decode_pptx(&canonical_bytes).expect("decode demo pptx for order canonicalization")
}
//#endregion 🔖️DocumentHelpers

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec PptxBuilderFacets {
        construction: PptxBuilderConstruction,
        analysis: PptxAnalyzerAnalysis,
        composition: super::super::io::derived_composition::PptxComposerComposition,
    }
    builder: PptxBuilder,
    analyzer: PptxAnalyzer,
    composer: PptxComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::pptx::schema::snapshot::{PptxParagraph, PptxRun, PptxShape, PptxSlide, PptxTransform};
    use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::export::serializers::{build_minimal_pptx, encode_pptx};
    use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::import::deserializers::{decode_pptx, sniff_pptx_bytes};
    use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::{
        PptxError, MINIMAL_SLIDE_MASTER_XML, PRESENTATION_CONTENT_TYPE, PRESENTATION_PART, REL_TYPE_OFFICE_DOCUMENT_STRICT, REL_TYPE_SLIDE, REL_TYPE_SLIDE_LAYOUT, REL_TYPE_SLIDE_MASTER, SLIDE_CONTENT_TYPE, SLIDE_LAYOUT_PART,
        SLIDE_MASTER_CONTENT_TYPE, SLIDE_MASTER_PART, THEME_PART,
    };
    use crate::artifacts::zip::opc::{self, OpcPackage, RELS_CONTENT_TYPE, REL_TYPE_OFFICE_DOCUMENT};

    async fn sample_presentation() -> PptxPresentation {
        PptxPresentation {
            slides: vec![
                PptxSlide {
                    shapes: vec![PptxShape::Placeholder {
                        kind: "title".into(),
                        text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "Title Slide".into(), bold: true, italic: false, font_size: Some(44) }] }],
                        position: PptxTransform { x: 100, y: 200, cx: 300, cy: 400 },
                    }],
                },
                PptxSlide {
                    shapes: vec![
                        PptxShape::TextBox { text_frame: vec![PptxParagraph::text("Second slide, plain.")], position: PptxTransform::default() },
                        PptxShape::TextBox { text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "italic note".into(), bold: false, italic: true, font_size: None }] }], position: PptxTransform { x: 1, y: 2, cx: 3, cy: 4 } },
                        PptxShape::Picture { blip_rel_id: "rId5".into(), position: PptxTransform { x: 10, y: 20, cx: 30, cy: 40 } },
                    ],
                },
            ],
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn builder_produces_minimal_valid_package_that_decodes_back() {
        let snap = build_minimal_pptx(sample_presentation().await);
        let bytes = encode_pptx(&snap).expect("encode minimal package");
        assert!(opc::sniff_opc_bytes(&bytes));
        assert!(sniff_pptx_bytes(&bytes));
        let decoded = decode_pptx(&bytes).expect("decode minimal package");
        assert_eq!(decoded.presentation, sample_presentation().await);
        // The synthesized boilerplate chain must actually be present — a real reader needs it.
        assert!(decoded.xml_parts.iter().any(|part| part.path == SLIDE_MASTER_PART));
        assert!(decoded.xml_parts.iter().any(|part| part.path == SLIDE_LAYOUT_PART));
        assert!(decoded.xml_parts.iter().any(|part| part.path == THEME_PART));
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_resolves_real_hand_built_package_with_shape_boundaries_and_position() {
        // Hand-built OOXML: a slide with TWO real shapes -- a positioned placeholder title and a
        // positioned picture -- exercising real shape-BOUNDARY recovery (not flattened text) and
        // real `a:xfrm` position decoding.
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");

        let slide_xml = concat!(
            r#"<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">"#,
            "<p:cSld><p:spTree>",
            r#"<p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr/>"#,
            "<p:sp><p:nvSpPr><p:cNvPr id=\"2\" name=\"Title\"/><p:cNvSpPr/><p:nvPr><p:ph type=\"title\"/></p:nvPr></p:nvSpPr>",
            r#"<p:spPr><a:xfrm><a:off x="111" y="222"/><a:ext cx="333" cy="444"/></a:xfrm></p:spPr>"#,
            r#"<p:txBody><a:bodyPr/><a:p><a:r><a:rPr b="1" i="1" sz="4400"/><a:t>Nested &amp; bold-italic</a:t></a:r></a:p></p:txBody>"#,
            "</p:sp>",
            r#"<p:pic><p:nvPicPr><p:cNvPr id="3" name="Pic"/><p:cNvPicPr/><p:nvPr/></p:nvPicPr>"#,
            r#"<p:blipFill><a:blip r:embed="rId9"/><a:stretch><a:fillRect/></a:stretch></p:blipFill>"#,
            r#"<p:spPr><a:xfrm><a:off x="5" y="6"/><a:ext cx="7" cy="8"/></a:xfrm></p:spPr></p:pic>"#,
            "</p:spTree></p:cSld></p:sld>",
        );
        opc.set_part("ppt/slides/slide1.xml", SLIDE_CONTENT_TYPE, slide_xml.as_bytes().to_vec());
        opc.add_relationship("ppt/slides/slide1.xml", "rId1", REL_TYPE_SLIDE_LAYOUT, "../slideLayouts/slideLayout1.xml");

        let presentation_xml = concat!(
            r#"<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">"#,
            r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
            r#"<p:sldIdLst><p:sldId id="256" r:id="rId2"/></p:sldIdLst>"#,
            "</p:presentation>",
        );
        opc.set_part(PRESENTATION_PART, PRESENTATION_CONTENT_TYPE, presentation_xml.as_bytes().to_vec());
        opc.add_relationship(PRESENTATION_PART, "rId1", REL_TYPE_SLIDE_MASTER, "slideMasters/slideMaster1.xml");
        opc.add_relationship(PRESENTATION_PART, "rId2", REL_TYPE_SLIDE, "slides/slide1.xml");
        opc.set_part(SLIDE_MASTER_PART, SLIDE_MASTER_CONTENT_TYPE, MINIMAL_SLIDE_MASTER_XML.as_bytes().to_vec());
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, PRESENTATION_PART);

        let bytes = opc::encode_opc(&opc).expect("encode hand-built package");
        let decoded = decode_pptx(&bytes).expect("decode hand-built pptx");

        assert_eq!(decoded.presentation.slides.len(), 1);
        let shapes = &decoded.presentation.slides[0].shapes;
        assert_eq!(shapes.len(), 2, "two DIRECT shapes must be recovered as two distinct PptxShape entries, not flattened");
        let PptxShape::Placeholder { kind, text_frame, position } = &shapes[0] else { panic!("expected placeholder shape") };
        assert_eq!(kind, "title");
        assert_eq!(*position, PptxTransform { x: 111, y: 222, cx: 333, cy: 444 });
        assert_eq!(text_frame[0].runs[0].text, "Nested & bold-italic");
        assert!(text_frame[0].runs[0].bold && text_frame[0].runs[0].italic);
        assert_eq!(text_frame[0].runs[0].font_size, Some(44));
        let PptxShape::Picture { blip_rel_id, position } = &shapes[1] else { panic!("expected picture shape") };
        assert_eq!(blip_rel_id, "rId9");
        assert_eq!(*position, PptxTransform { x: 5, y: 6, cx: 7, cy: 8 });
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_preserves_unmodeled_shape_kinds_as_logical_other() {
        // A `p:graphicFrame` (chart/table/SmartArt) direct child -- not typed by this layer --
        // must survive decode->encode->decode as a logical `PptxShape::Other` XML node.
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        let graphic_frame = r#"<p:graphicFrame><p:nvGraphicFramePr><p:cNvPr id="9" name="Table 1"/></p:nvGraphicFramePr><a:graphic/></p:graphicFrame>"#;
        let slide_xml = format!(
            concat!(
                r#"<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">"#,
                "<p:cSld><p:spTree>",
                r#"<p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr/>"#,
                "{}",
                "</p:spTree></p:cSld></p:sld>",
            ),
            graphic_frame,
        );
        opc.set_part("ppt/slides/slide1.xml", SLIDE_CONTENT_TYPE, slide_xml.into_bytes());
        opc.add_relationship("ppt/slides/slide1.xml", "rId1", REL_TYPE_SLIDE_LAYOUT, "../slideLayouts/slideLayout1.xml");
        let presentation_xml = concat!(
            r#"<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">"#,
            r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
            r#"<p:sldIdLst><p:sldId id="256" r:id="rId2"/></p:sldIdLst>"#,
            "</p:presentation>",
        );
        opc.set_part(PRESENTATION_PART, PRESENTATION_CONTENT_TYPE, presentation_xml.as_bytes().to_vec());
        opc.add_relationship(PRESENTATION_PART, "rId1", REL_TYPE_SLIDE_MASTER, "slideMasters/slideMaster1.xml");
        opc.add_relationship(PRESENTATION_PART, "rId2", REL_TYPE_SLIDE, "slides/slide1.xml");
        opc.set_part(SLIDE_MASTER_PART, SLIDE_MASTER_CONTENT_TYPE, MINIMAL_SLIDE_MASTER_XML.as_bytes().to_vec());
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, PRESENTATION_PART);

        let bytes = opc::encode_opc(&opc).expect("encode");
        let decoded = decode_pptx(&bytes).expect("decode");
        assert_eq!(decoded.presentation.slides[0].shapes.len(), 1);
        let PptxShape::Other { node } = &decoded.presentation.slides[0].shapes[0] else { panic!("expected Other shape") };
        let node_model = format!("{node:?}");
        assert!(node_model.contains("p:graphicFrame") && node_model.contains("Table 1"));

        // Re-encode -> re-decode: the logical XML node must survive the round trip.
        let re_encoded = encode_pptx(&decoded).expect("re-encode");
        let re_decoded = decode_pptx(&re_encoded).expect("re-decode");
        assert_eq!(re_decoded.presentation, decoded.presentation);
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_resolves_strict_office_document_relationship_too() {
        // 🏅️ A genuine ISO/IEC 29500-1 Strict package's root relationship carries
        // `REL_TYPE_OFFICE_DOCUMENT_STRICT`, never the Transitional type this engine's own writer
        // emits -- `decode_pptx`/`sniff_pptx_bytes` must still recognize it (ticket
        // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES, so the `✳️strict` subset's
        // analyzer can ever see real Strict bytes).
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        let presentation_xml = concat!(
            r#"<p:presentation xmlns:a="http://purl.oclc.org/ooxml/drawingml/main" xmlns:p="http://purl.oclc.org/ooxml/presentationml/main" xmlns:r="http://purl.oclc.org/ooxml/officeDocument/relationships">"#,
            r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
            r#"<p:sldIdLst/>"#,
            "</p:presentation>",
        );
        opc.set_part(PRESENTATION_PART, PRESENTATION_CONTENT_TYPE, presentation_xml.as_bytes().to_vec());
        opc.set_part(SLIDE_MASTER_PART, SLIDE_MASTER_CONTENT_TYPE, MINIMAL_SLIDE_MASTER_XML.as_bytes().to_vec());
        opc.add_relationship(PRESENTATION_PART, "rId1", REL_TYPE_SLIDE_MASTER, "slideMasters/slideMaster1.xml");
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT_STRICT, PRESENTATION_PART);

        let bytes = opc::encode_opc(&opc).expect("encode hand-built Strict package");
        assert!(sniff_pptx_bytes(&bytes), "Strict-relationship-typed package must still sniff as pptx");
        let decoded = decode_pptx(&bytes).expect("decode Strict-relationship-typed package");
        assert_eq!(decoded.presentation.slides.len(), 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_rejects_missing_presentation_relationship() {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        let bytes = opc::encode_opc(&opc).expect("encode");
        let err = decode_pptx(&bytes).expect_err("must reject a package with no officeDocument relationship");
        assert_eq!(err, PptxError::MissingPresentationRelationship);
    }

    #[semio_framework_async_macros::async_test]
    async fn unmodeled_slide_master_survives_decode_encode_logically() {
        let snap = build_minimal_pptx(sample_presentation().await);
        // Replace the synthesized slide master with a distinguishable "real" one before encoding.
        let mut opc = snap.opc.clone();
        opc.set_part(SLIDE_MASTER_PART, SLIDE_MASTER_CONTENT_TYPE, b"<p:sldMaster marker=\"real-file\"/>".to_vec());
        let bytes = opc::encode_opc(&opc).expect("encode");

        let decoded = decode_pptx(&bytes).expect("decode");
        assert_eq!(decoded.opc.part_bytes(SLIDE_MASTER_PART), Some(b"<p:sldMaster marker=\"real-file\"/>".as_slice()));
        let re_encoded = encode_pptx(&decoded).expect("re-encode must not clobber an already-present slide master");
        let re_decoded = decode_pptx(&re_encoded).expect("re-decode");
        assert_eq!(re_decoded.opc.part_bytes(SLIDE_MASTER_PART), Some(b"<p:sldMaster marker=\"real-file\"/>".as_slice()));
        assert_eq!(re_decoded.presentation, sample_presentation().await);
    }

    #[semio_framework_async_macros::async_test]
    async fn analyzer_builder_round_trip() {
        let original = build_minimal_pptx(sample_presentation().await);
        let bytes = encode_pptx(&original).expect("encode");
        let analyzed = decode_pptx(&bytes).expect("decode");
        let rebuilt = build_minimal_pptx(analyzed.presentation.clone());
        let rebuilt_bytes = encode_pptx(&rebuilt).expect("encode rebuilt");
        let reanalyzed = decode_pptx(&rebuilt_bytes).expect("decode rebuilt");
        assert_eq!(reanalyzed.presentation, analyzed.presentation);
    }

    #[semio_framework_async_macros::async_test]
    async fn shrinking_slide_count_drops_stale_slide_parts_and_relationships() {
        let mut wide = sample_presentation().await;
        let snap_wide = build_minimal_pptx(wide.clone());
        assert!(!snap_wide.opc.relationships_for("ppt/slides/slide2.xml").is_empty());

        wide.slides.truncate(1);
        let bytes = encode_pptx(&PptxSnapshot::from_parts(snap_wide.opc, snap_wide.xml_parts, wide)).expect("encode narrower presentation");
        let decoded = decode_pptx(&bytes).expect("decode");
        assert!(decoded.opc.relationships_for("ppt/slides/slide2.xml").is_empty(), "stale second slide's relationships must be dropped too");
        assert_eq!(decoded.presentation.slides.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn repeated_materialization_is_deterministic() {
        let snap = build_minimal_pptx(sample_presentation().await);
        let once = encode_pptx(&snap).expect("first materialization");
        let decoded = decode_pptx(&once).expect("decode first materialization");
        let twice = encode_pptx(&decoded).expect("second materialization");
        assert_eq!(once, twice);
    }

    //#region 🔖️ExactSourceRoundtrip
    async fn exact_pptx_bytes() -> Vec<u8> {
        std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../temp/domai-specific-programmaning-language-for-architects.pptx")).expect("read exact pptx fixture")
    }

    async fn local_member_names(bytes: &[u8]) -> Vec<String> {
        let mut names = Vec::new();
        let mut offset = 0usize;
        while bytes.get(offset..offset + 4) == Some(b"PK\x03\x04") {
            let compressed = u32::from_le_bytes(bytes[offset + 18..offset + 22].try_into().expect("compressed size")) as usize;
            let name_len = u16::from_le_bytes(bytes[offset + 26..offset + 28].try_into().expect("name length")) as usize;
            let extra_len = u16::from_le_bytes(bytes[offset + 28..offset + 30].try_into().expect("extra length")) as usize;
            let name_start = offset + 30;
            names.push(String::from_utf8(bytes[name_start..name_start + name_len].to_vec()).expect("UTF-8 member name"));
            offset = name_start + name_len + extra_len + compressed;
        }
        names
    }

    async fn local_compressed_members(bytes: &[u8]) -> Vec<(String, u16, u16, u16, u16, u16, u32, u32, Vec<u8>, Vec<u8>)> {
        let mut members = Vec::new();
        let mut offset = 0usize;
        while bytes.get(offset..offset + 4) == Some(b"PK\x03\x04") {
            let flags = u16::from_le_bytes(bytes[offset + 6..offset + 8].try_into().expect("flags"));
            let method = u16::from_le_bytes(bytes[offset + 8..offset + 10].try_into().expect("method"));
            let version = u16::from_le_bytes(bytes[offset + 4..offset + 6].try_into().expect("version"));
            let time = u16::from_le_bytes(bytes[offset + 10..offset + 12].try_into().expect("time"));
            let date = u16::from_le_bytes(bytes[offset + 12..offset + 14].try_into().expect("date"));
            let crc = u32::from_le_bytes(bytes[offset + 14..offset + 18].try_into().expect("crc"));
            let compressed = u32::from_le_bytes(bytes[offset + 18..offset + 22].try_into().expect("compressed size")) as usize;
            let uncompressed = u32::from_le_bytes(bytes[offset + 22..offset + 26].try_into().expect("uncompressed size"));
            let name_len = u16::from_le_bytes(bytes[offset + 26..offset + 28].try_into().expect("name length")) as usize;
            let extra_len = u16::from_le_bytes(bytes[offset + 28..offset + 30].try_into().expect("extra length")) as usize;
            let name_start = offset + 30;
            let payload_start = name_start + name_len + extra_len;
            members.push((
                String::from_utf8(bytes[name_start..name_start + name_len].to_vec()).expect("UTF-8 member name"),
                version,
                flags,
                method,
                time,
                date,
                crc,
                uncompressed,
                bytes[name_start + name_len..payload_start].to_vec(),
                bytes[payload_start..payload_start + compressed].to_vec(),
            ));
            offset = payload_start + compressed;
        }
        members
    }

    #[track_caller]
    async fn assert_exact_export(snapshot: &PptxSnapshot, expected: &[u8]) {
        let actual = encode_pptx(snapshot).expect("export exact fixture");
        if actual != expected {
            let first_diff = actual.iter().zip(expected).position(|(left, right)| left != right).unwrap_or(actual.len().min(expected.len()));
            let actual_names = local_member_names(&actual).await;
            let expected_names = local_member_names(expected).await;
            let first_order_diff = actual_names.iter().zip(&expected_names).position(|(left, right)| left != right).unwrap_or(actual_names.len().min(expected_names.len()));
            let actual_compressed = local_compressed_members(&actual).await;
            let expected_compressed = local_compressed_members(expected).await;
            let first_compressed = actual_compressed.iter().zip(&expected_compressed).position(|(left, right)| left != right).unwrap_or(actual_compressed.len().min(expected_compressed.len()));
            let compressed_detail = actual_compressed
                .get(first_compressed)
                .zip(expected_compressed.get(first_compressed))
                .map(|(left, right)| {
                    let prefix = left.9.iter().zip(&right.9).position(|(actual, expected)| actual != expected).unwrap_or(left.9.len().min(right.9.len()));
                    format!(" compressed_name={} actual_version={} expected_version={} actual_flags={} expected_flags={} actual_method={} expected_method={} actual_time={} expected_time={} actual_date={} expected_date={} actual_crc={} expected_crc={} actual_uncompressed={} expected_uncompressed={} actual_extra={:?} expected_extra={:?} actual_compressed={} expected_compressed={} compressed_prefix={prefix}", left.0, left.1, right.1, left.2, right.2, left.3, right.3, left.4, right.4, left.5, right.5, left.6, right.6, left.7, right.7, left.8, right.8, left.9.len(), right.9.len())
                })
                .unwrap_or_default();
            let actual_zip = crate::artifacts::zip::standards::v2_0::subsets::any::io::decode_zip(&actual).expect("decode actual mismatch");
            let expected_zip = crate::artifacts::zip::standards::v2_0::subsets::any::io::decode_zip(expected).expect("decode expected mismatch");
            let first_entry = actual_zip.entries.iter().zip(&expected_zip.entries).position(|(left, right)| left != right).unwrap_or(actual_zip.entries.len().min(expected_zip.entries.len()));
            let logical_mismatch_separator = ",";
            let logical_mismatches = actual_zip
                .entries
                .iter()
                .zip(&expected_zip.entries)
                .filter(|(left, right)| left != right)
                .take(16)
                .map(|(left, right)| {
                    let offset = left.data.iter().zip(&right.data).position(|(a, b)| a != b).unwrap_or(left.data.len().min(right.data.len()));
                    let start = offset.saturating_sub(12);
                    let actual_end = (offset + 44).min(left.data.len());
                    let expected_end = (offset + 44).min(right.data.len());
                    format!("{}:{}/{}@{}:{:?}!={:?}", left.name, left.data.len(), right.data.len(), offset, String::from_utf8_lossy(&left.data[start..actual_end]), String::from_utf8_lossy(&right.data[start..expected_end]),)
                })
                .collect::<Vec<_>>()
                .join(logical_mismatch_separator);
            let entry_detail = actual_zip
                .entries
                .get(first_entry)
                .zip(expected_zip.entries.get(first_entry))
                .map(|(left, right)| {
                    let data_diff = left.data.iter().zip(&right.data).position(|(a, b)| a != b).unwrap_or(left.data.len().min(right.data.len()));
                    let start = data_diff.saturating_sub(24);
                    let actual_end = (data_diff + 72).min(left.data.len());
                    let expected_end = (data_diff + 72).min(right.data.len());
                    let actual_snippet = String::from_utf8_lossy(&left.data[start..actual_end]);
                    let expected_snippet = String::from_utf8_lossy(&right.data[start..expected_end]);
                    format!(" name={} actual_data_len={} expected_data_len={} data_diff={data_diff} actual_snippet={actual_snippet:?} expected_snippet={expected_snippet:?}", left.name, left.data.len(), right.data.len())
                })
                .unwrap_or_default();
            panic!("PPTX exact export mismatch: actual_len={} expected_len={} first_diff={first_diff} first_order_diff={first_order_diff} first_compressed={first_compressed}{compressed_detail} first_entry={first_entry}{entry_detail} logical_mismatches=[{logical_mismatches}]", actual.len(), expected.len());
        }
    }

    fn first_positioned_shape(snapshot: &PptxSnapshot) -> (usize, usize, PptxTransform) {
        for (slide_index, slide) in snapshot.presentation.slides.iter().enumerate() {
            for (shape_index, shape) in slide.shapes.iter().enumerate() {
                let position = match shape {
                    PptxShape::TextBox { position, .. } | PptxShape::Picture { position, .. } | PptxShape::Placeholder { position, .. } => *position,
                    PptxShape::Other { .. } => continue,
                };
                return (slide_index, shape_index, position);
            }
        }
        panic!("exact fixture has no positioned shape");
    }

    #[semio_framework_async_macros::async_test]
    async fn fixture_survives_logical_io_persistence_diff_and_mutation_pipelines() {
        use crate::artifacts::pptx::schema::mutations::{set_shape_position, set_shape_text, set_snapshot};
        use crate::artifacts::pptx::{PptxDiff, PptxMutation};
        use protocol::{DiffAlgebra, DiffCodec, Mutation, MutationDiff, OpBinary, OpText};
        use semio_framework_plugin::{AnalyzeSource, ArtifactAnalysis, ArtifactComposition, ComposeSource};

        let exact_bytes = exact_pptx_bytes().await;
        let snapshot = decode_pptx(&exact_bytes).expect("import exact fixture");
        assert_eq!(snapshot.presentation.slides.len(), 62);
        assert!(!snapshot.xml_parts.is_empty());
        let xml_paths: std::collections::HashSet<&str> = snapshot.xml_parts.iter().map(|part| part.path.as_str()).collect();
        assert_eq!(xml_paths.len(), snapshot.xml_parts.len(), "every logical XML part must have one authority");
        assert!(snapshot.xml_parts.iter().any(|part| part.path == "ppt/drawings/vmlDrawing1.vml"), "VML must be parsed as logical XML");
        assert!(snapshot.opc.parts.iter().all(|part| !crate::artifacts::pptx::schema::snapshot::pptx_part_is_xml(&part.path, &part.content_type)), "OPC byte parts must contain no XML");
        assert!(snapshot.opc.parts.iter().all(|part| !xml_paths.contains(part.path.as_str())), "XML and binary authorities must be disjoint");
        assert!(
            snapshot.opc.parts.iter().all(|part| {
                let path = part.path.to_ascii_lowercase();
                [".png", ".jpg", ".jpeg", ".emf", ".bin"].iter().any(|extension| path.ends_with(extension))
            }),
            "fixture binary parts must be genuine media or embedded object payloads"
        );
        let relationship_parts = snapshot.opc.relationships.values().filter(|relationships| !relationships.is_empty()).count();
        assert_eq!(1 + relationship_parts + snapshot.xml_parts.len() + snapshot.opc.parts.len(), 211, "every native member must map to exactly one logical authority");
        let mut duplicate_authority = snapshot.clone();
        let duplicated = duplicate_authority.xml_parts.first().expect("fixture XML part");
        duplicate_authority.opc.parts.push(opc::OpcPart { path: duplicated.path.clone(), content_type: duplicated.content_type.clone(), bytes: b"<shadow/>".to_vec() });
        assert!(encode_pptx(&duplicate_authority).is_err(), "export must reject XML stored as opaque OPC bytes");
        assert_exact_export(&snapshot, &exact_bytes).await;

        let analysis = PptxAnalyzerAnalysis::analyze(&[AnalyzeSource::Binary(&exact_bytes)]);
        let analyzed = analysis.parts.snapshot.expect("analyze native PPTX fixture");
        assert_eq!(analyzed, snapshot);
        assert_exact_export(&analyzed, &exact_bytes).await;

        let dialect = <PptxAnalyzerAnalysis as ArtifactAnalysis>::DIALECT;
        let composition = crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::PptxComposerComposition::compose(&[ComposeSource { dialect, payload: AnalyzeSource::Binary(&exact_bytes) }]).expect("compose native PPTX fixture");
        assert_eq!(composition.snapshot, snapshot);
        assert_exact_export(&composition.snapshot, &exact_bytes).await;

        let zip = crate::artifacts::zip::standards::v2_0::subsets::any::io::decode_zip(&exact_bytes).expect("decode exact zip");
        assert_eq!(zip.entries.len(), 211);
        assert_eq!(zip.entries.iter().filter(|entry| entry.name.starts_with("ppt/slides/slide") && entry.name.ends_with(".xml")).count(), 62);
        assert_eq!(zip.entries.iter().filter(|entry| entry.name.ends_with(".rels")).count(), 78);

        let packed = store::ArtifactPack::encode_pack(&snapshot);
        let unpacked = <PptxSnapshot as store::ArtifactPack>::decode_pack(&packed).expect("unpack exact fixture");
        assert_eq!(unpacked, snapshot);
        assert_exact_export(&unpacked, &exact_bytes).await;

        let binary = crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::export::serializers::artifacts::zip::v2_0::any::serialize(&snapshot).expect("serialize exact fixture to binary");
        let from_binary = crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::import::deserializers::artifacts::zip::v2_0::any::deserialize(&binary).expect("deserialize exact fixture from binary");
        assert_eq!(from_binary, snapshot);
        assert_exact_export(&from_binary, &exact_bytes).await;

        let dsl = store::ArtifactDsl::print_dsl(&snapshot);
        let parsed = <PptxSnapshot as store::ArtifactDsl>::parse_dsl(&dsl).expect("parse exact fixture dsl");
        assert_eq!(parsed, snapshot);
        assert_exact_export(&parsed, &exact_bytes).await;

        let self_diff = PptxDiff::between(&snapshot, &snapshot);
        assert!(self_diff.is_empty());
        assert_exact_export(&self_diff.apply(&snapshot).unwrap(), &exact_bytes).await;

        // 🧭️ `NoMutation` was dropped by the mutation-leaf migration (26/08/29/S-END-TO-END); an
        // out-of-range `SetShapeText` is this subset's own documented no-op (`diff_set_shape_text`
        // returns `PptxDiff::default()` when the addressed slide doesn't exist), so it stands in
        // for the removed unit variant here.
        let mut no_op = snapshot.clone();
        let no_op_diff = crate::artifacts::pptx::schema::mutations::apply_pptx_mutation(&mut no_op, &PptxMutation::SetShapeText(set_shape_text::SetShapeText { slide_index: usize::MAX, shape_index: usize::MAX, text_frame: Vec::new() }));
        assert!(no_op_diff.diff().is_empty());
        assert_exact_export(&no_op, &exact_bytes).await;

        let (slide_index, shape_index, position) = first_positioned_shape(&snapshot);
        let changed_x = if position.x == i64::MAX { position.x - 1 } else { position.x + 1 };
        let mutation = PptxMutation::SetShapePosition(set_shape_position::SetShapePosition { slide_index, shape_index, position: PptxTransform { x: changed_x, ..position } });
        let mut changed = snapshot.clone();
        let forward = crate::artifacts::pptx::schema::mutations::apply_pptx_mutation(&mut changed, &mutation);
        assert_ne!(changed, snapshot);
        assert_ne!(encode_pptx(&changed).expect("encode mutated presentation"), exact_bytes);
        for inverse in mutation.inverse(&snapshot) {
            crate::artifacts::pptx::schema::mutations::apply_pptx_mutation(&mut changed, &inverse);
        }
        assert_eq!(changed, snapshot);
        assert_exact_export(&changed, &exact_bytes).await;

        let mid = forward.diff().apply(&snapshot).unwrap();
        let inverse = forward.diff().inverse(&snapshot);
        let mut absorbed = forward.diff().clone();
        absorbed.absorb(inverse.clone());
        let restored = inverse.apply(&mid).unwrap();
        assert_eq!(restored, snapshot);
        assert_eq!(absorbed.apply(&snapshot).unwrap(), snapshot);
        assert_exact_export(&restored, &exact_bytes).await;
        assert_exact_export(&absorbed.apply(&snapshot).unwrap(), &exact_bytes).await;

        let mut without_xml_parts = snapshot.clone();
        without_xml_parts.xml_parts.clear();
        let xml_parts_diff = PptxDiff::between(&without_xml_parts, &snapshot);
        let printed_diff = xml_parts_diff.print_diff();
        let parsed_diff = PptxDiff::parse_diff(&printed_diff).expect("parse logical XML parts diff");
        assert_exact_export(&parsed_diff.apply(&without_xml_parts).unwrap(), &exact_bytes).await;
        let encoded_diff = xml_parts_diff.encode_diff().expect("encode logical XML parts diff");
        let decoded_diff = PptxDiff::decode_diff(&encoded_diff).expect("decode logical XML parts diff");
        assert_exact_export(&decoded_diff.apply(&without_xml_parts).unwrap(), &exact_bytes).await;

        let set_snapshot = PptxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: snapshot.clone() });
        let printed_op = set_snapshot.print_op();
        let parsed_op = PptxMutation::parse_op(&printed_op).expect("parse exact set-snapshot");
        let mut via_text_op = without_xml_parts.clone();
        crate::artifacts::pptx::schema::mutations::apply_pptx_mutation(&mut via_text_op, &parsed_op);
        assert_exact_export(&via_text_op, &exact_bytes).await;
        let encoded_op = set_snapshot.encode_op().expect("encode exact set-snapshot");
        let decoded_op = PptxMutation::decode_op(&encoded_op).expect("decode exact set-snapshot");
        let mut via_binary_op = without_xml_parts;
        crate::artifacts::pptx::schema::mutations::apply_pptx_mutation(&mut via_binary_op, &decoded_op);
        assert_exact_export(&via_binary_op, &exact_bytes).await;
    }
    //#endregion 🔖️ExactSourceRoundtrip

    //#region 🔖️ConformanceLaws
    /// 🧪️ FG-wave: per-artifact conformance laws (`📖️grammar-recipe.md` §4's checklist item) --
    /// grammar/protocol parseability, `Recognizer` against real fixtures AND real `print_op`/
    /// `print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff`
    /// bytes, and the fixture-honesty round-trip. Lives beside the rest of this artifact's schema
    /// tests (moved out of `⚙️engine`, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) --
    /// this artifact's OWN early-warning, plus direct coverage of the mutations/diff facets the
    /// framework's `m5` auto-discovery does not reach at all.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::pptx::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect -- independent of, and cheaper than, the two
        /// `recognize`/`walk_protocol` laws below (a parse failure here fails fast with a clearer
        /// message).
        #[semio_framework_async_macros::async_test]
        async fn committed_facet_files_parse() {
            for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar models the real TEXT syntax of the
        /// XML parts a pptx OPC package carries (`📸️snapshot/📝️text/📖️component.grammar.semio`'s
        /// own doc comment explains why -- this artifact's `ArtifactDsl::print_dsl` hex-dumps the
        /// WHOLE binary OPC package, matching this facet's SIBLING binary protocol, not this text
        /// grammar; the two facets describe different LAYERS of the same real artifact, same as
        /// every OPC-family member's own container/contained-parts split). So this law decodes the
        /// REAL zip entries `encode_pptx` genuinely produces (via `zip::engine::decode_zip`, the
        /// same real codec `opc::decode_opc` itself delegates to) and recognizes EACH real
        /// modeled part's own text against the grammar -- direct proof the grammar matches this
        /// artifact's own real per-part XML bytes, not an invented approximation.
        #[semio_framework_async_macros::async_test]
        async fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);

            let demo = demo_pptx_snapshot().await;
            let bytes = encode_pptx(&demo).expect("encode demo pptx");
            let zip = crate::artifacts::zip::standards::v2_0::subsets::any::io::decode_zip(&bytes).expect("decode zip");

            let fixed_parts = ["[Content_Types].xml", "_rels/.rels", "ppt/presentation.xml"];
            let mut checked = 0;
            for entry in &zip.entries {
                let is_slide = entry.name.starts_with("ppt/slides/slide") && entry.name.ends_with(".xml");
                if !fixed_parts.contains(&entry.name.as_str()) && !is_slide {
                    continue;
                }
                let text = String::from_utf8(entry.data.clone()).unwrap_or_else(|e| panic!("part {:?}: not valid utf-8: {e}", entry.name));
                assert!(recognizer.recognize(&text).unwrap_or(false), "grammar did not recognize real part {:?}:\n{text}", entry.name);
                checked += 1;
            }
            assert_eq!(checked, fixed_parts.len() + demo.presentation.slides.len(), "not every modeled part was present in the real zip entries");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `PptxMutation` variant (`mutations::demo_mutation_cases()`).
        #[semio_framework_async_macros::async_test]
        async fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
        /// for every representative `PptxDiff` (`diff::demo_diff_cases()`).
        #[semio_framework_async_macros::async_test]
        async fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in diff::demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets --
        /// snapshot pack (`encode_pack`, envelope-unwrapped first, matching how
        /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
        /// mutation's `encode_op`, and every demo diff's `encode_diff`. The snapshot protocol
        /// declares `backward`/`jump` (restated from zip's own real ZIP layout), so `walk_protocol`
        /// correctly does NOT require landing on exactly `bytes.len()` (M2's own documented
        /// exception, `📖️grammar-recipe.md` §2.3) -- assert a sane in-range `consumed` there
        /// instead, same as zip's/docx's own `protocol_walk_law` does; the op/diff protocols have
        /// no such exception and must consume every byte.
        #[semio_framework_async_macros::async_test]
        async fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let demo = demo_pptx_snapshot().await;
            let packed = store::ArtifactPack::encode_pack(&demo);
            let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
            let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
            assert!(trace.consumed > 0 && trace.consumed <= inner.len(), "pack walk consumed an out-of-range span");

            let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
            for mutation in mutations::demo_mutation_cases() {
                let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
                let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
            }

            let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
            for d in diff::demo_diff_cases() {
                let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
                let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
            }
        }

        /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
        /// `print_dsl`/`encode_pack` output of `demo_pptx_snapshot().await` -- `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin -- so the
        /// fixtures can never silently drift back to a fake `"68656c6c6f"`-style placeholder again
        /// (see this ticket's own recon note on the pre-FG-wave state of these two files).
        #[semio_framework_async_macros::async_test]
        async fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_pptx_snapshot().await;

            let parsed = <PptxSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_pptx_snapshot().await");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_pptx_snapshot().await) drifted from the shipped .dsl.semio fixture");

            let decoded = <PptxSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_pptx_snapshot().await");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_pptx_snapshot().await) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
