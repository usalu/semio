//! 🧬️ PptxArtifact schema — full artifact state.

use crate::schema::snapshot::{PptxPresentation, PptxXmlPart};
use crate::PptxSnapshot;
use semio_s_artifact_stdio_zip::opc::OpcPackage;
use framework_schema::ArtifactSchema;

//#region Artifact
/// 🧬️ Full `stdio.pptx` artifact state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
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
    #[state(artifact)]
    #[value(default)]
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
pub fn pptx_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.stdio.pptx",
        artifact: framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: framework_schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::schema::snapshot::{PptxParagraph, PptxRun, PptxShape, PptxSlide, PptxTransform};
    use crate::{PptxDiff, PptxMutation, PptxSnapshot};
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
            let diff = crate::schema::mutations::apply_pptx_mutation(&mut self.snapshot, &mutation);
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
            self.snapshot = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_pptx(self.snapshot.presentation);
            self
        }
    }
    //#endregion 🔖️TypedConstructors
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::PptxSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.pptx` parts.
    #[derive(Clone, Debug, Default)]
    pub struct PptxParts {
        pub snapshot: Option<PptxSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.pptx` (ecma-376/🧱️base) sources.
    pub struct PptxAnalyzerAnalysis;

    impl ArtifactAnalysis for PptxAnalyzerAnalysis {
        type Parts = PptxParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            // 🕵️ Real sniff: OPC-shaped bytes whose root officeDocument relationship resolves under
            // `ppt/` — disambiguates from docx/xlsx, which share the same zip magic and OPC shape.
            match source {
                AnalyzeSource::Binary(bytes) if crate::standards::v_ecma_376::subsets::base::io::import::deserializers::sniff_pptx_bytes(bytes) => IoConfidence::High,
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
                        let result = if crate::standards::v_ecma_376::subsets::base::io::import::deserializers::sniff_pptx_bytes(bytes) {
                            crate::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_pptx(bytes).map_err(|err| err.to_string())
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
/// single source of truth for `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/
/// `🎒️.pack.semio` (both are literally this snapshot's `print_dsl`/`encode_pack` output,
/// asserted equal by `fixture_honesty_law` below) — same shape docx's own `demo_docx_snapshot()`
/// establishes.
pub async fn demo_pptx_snapshot() -> PptxSnapshot {
    use crate::schema::snapshot::{PptxParagraph, PptxRun, PptxShape, PptxSlide, PptxTransform};
    use crate::standards::v_ecma_376::subsets::base::io::export::serializers::{build_minimal_pptx, encode_pptx};
    use crate::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_pptx;
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
                        node: semio_s_artifact_stdio_xml::schema::snapshot::xml_document_from_text(r#"<p:graphicFrame><p:nvGraphicFramePr><p:cNvPr id="9" name="Table 1"/></p:nvGraphicFramePr></p:graphicFrame>"#)
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
