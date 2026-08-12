//! 🧬️ PptxArtifact schema — full artifact state.

use crate::artifacts::pptx::schema::snapshot::PptxPresentation;
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
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub opc: OpcPackage,
    #[state(persistent)]
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
        PptxSnapshot { schema: self.schema.clone(), opc: self.opc.clone(), presentation: self.presentation.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: PptxSnapshot) -> Self {
        Self { schema: snapshot.schema, opc: snapshot.opc, presentation: snapshot.presentation }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: PptxSnapshot) {
        self.schema = snapshot.schema;
        self.opc = snapshot.opc;
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
            rust: include_str!("🧬️mutations/🦀️component.rs"),
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
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::pptx::schema::snapshot::{PptxParagraph, PptxRun, PptxShape, PptxSlide, PptxTransform};
    use crate::artifacts::pptx::{PptxDiff, PptxMutation, PptxSnapshot};

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
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = crate::artifacts::pptx::schema::mutations::apply_pptx_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <PptxDiff as protocol::MutationDiff<PptxSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
    //#endregion 🔖️Builder

    //#region 🔖️TypedConstructors
    /// 🧱️ Typed content constructors — build a presentation from slides of paragraphs/runs with
    /// basic formatting (bold/italic), the same shape as `docx::DocxBuilder`'s constructors.
    impl PptxBuilderConstruction {
        /// ➕️ Appends a new (initially empty) slide and makes it the active slide for `add_paragraph`.
        pub fn add_slide(mut self) -> Self {
            self.snapshot.presentation.slides.push(PptxSlide::default());
            self.rebuild()
        }

        /// ➕️ Appends a paragraph to the active slide's active `TextBox` shape (the most recently
        /// added one), creating a fresh `TextBox` shape first if the slide has none yet or its last
        /// shape isn't one.
        pub fn add_paragraph(mut self, paragraph: PptxParagraph) -> Self {
            if let Some(slide) = self.snapshot.presentation.slides.last_mut() {
                match slide.shapes.last_mut() {
                    Some(PptxShape::TextBox { text_frame, .. }) => text_frame.push(paragraph),
                    _ => slide.shapes.push(PptxShape::TextBox { text_frame: vec![paragraph], position: PptxTransform::default() }),
                }
            }
            self.rebuild()
        }

        /// ➕️ Appends a single-run plain-text paragraph to the active slide.
        pub fn add_text_paragraph(self, text: impl Into<String>) -> Self {
            self.add_paragraph(PptxParagraph::text(text.into()))
        }

        /// ➕️ Appends a paragraph made of the given runs (basic bold/italic formatting).
        pub fn add_runs(self, runs: Vec<PptxRun>) -> Self {
            self.add_paragraph(PptxParagraph { runs })
        }

        fn rebuild(mut self) -> Self {
            self.snapshot = crate::artifacts::pptx::engine::build_minimal_pptx(self.snapshot.presentation);
            self
        }
    }
    //#endregion 🔖️TypedConstructors
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::pptx::PptxSnapshot;

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
                AnalyzeSource::Binary(bytes) if crate::artifacts::pptx::engine::sniff_pptx_bytes(bytes) => IoConfidence::High,
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
                            diagnostics.push(dsl::Diagnostic::error(
                                "stdio.analyze.text",
                                dsl::TextSpan::at(1, 1),
                                err.to_string(),
                            ));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <PptxSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error(
                                "stdio.analyze.binary",
                                dsl::TextSpan::at(1, 1),
                                err.to_string(),
                            ));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec PptxBuilderFacets {
        construction: derived_construction::PptxBuilderConstruction,
        analysis: derived_analysis::PptxAnalyzerAnalysis,
        composition: super::super::io::derived_composition::PptxComposerComposition,
    }
    builder: PptxBuilder,
    analyzer: PptxAnalyzer,
    composer: PptxComposer,
);
//#endregion 🧬️DerivedArtifactFacets
