//! 🧬️ DocxArtifact schema — full artifact state.

use crate::artifacts::docx::schema::snapshot::DocxDocument;
use crate::artifacts::docx::DocxSnapshot;
use crate::artifacts::zip::opc::OpcPackage;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region Artifact
/// 🧬️ Full `stdio.docx` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.docx")]
pub struct DocxArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub opc: OpcPackage,
    #[state(persistent)]
    #[serde(default)]
    pub document: DocxDocument,
}
//#endregion Artifact

//#region Conversions
impl Default for DocxArtifact {
    fn default() -> Self {
        Self::from_snapshot(DocxSnapshot::default())
    }
}

impl DocxArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> DocxSnapshot {
        DocxSnapshot { schema: self.schema.clone(), opc: self.opc.clone(), document: self.document.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: DocxSnapshot) -> Self {
        Self { schema: snapshot.schema, opc: snapshot.opc, document: snapshot.document }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: DocxSnapshot) {
        self.schema = snapshot.schema;
        self.opc = snapshot.opc;
        self.document = snapshot.document;
    }
}
//#endregion Conversions

//#region Descriptor
/// 🧬️ Descriptor for `s.stdio.docx`.
pub fn docx_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.docx",
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
    use crate::artifacts::docx::schema::snapshot::{DocxBlock, DocxParagraph, DocxRun, DocxStyle, DocxTable};
    use crate::artifacts::docx::{DocxDiff, DocxMutation, DocxSnapshot};

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.docx` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct DocxBuilderConstruction {
        snapshot: DocxSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for DocxBuilderConstruction {
        type Snapshot = DocxSnapshot;
        type Mutation = DocxMutation;
        type Diff = DocxDiff;
        fn empty() -> Self {
            Self { snapshot: DocxSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<DocxSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<DocxSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = crate::artifacts::docx::schema::mutations::apply_docx_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <DocxDiff as protocol::MutationDiff<DocxSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
    //#endregion 🔖️Builder

    //#region 🔖️TypedConstructors
    /// 🧱️ Typed content constructors — build a `word/document.xml` document from paragraphs/runs
    /// with basic formatting (bold/italic), mirroring the svg artifact's "builder builds a full
    /// standard document" reference shape. Chainable; `build()` (from `ArtifactBuilder`) produces
    /// the final `DocxSnapshot`, whose OPC container is assembled fresh (see `io::export::serializers::build_minimal_docx`)
    /// the first time a paragraph is added to an otherwise-empty builder.
    impl DocxBuilderConstruction {
        /// ➕️ Appends a paragraph.
        pub fn add_paragraph(mut self, paragraph: DocxParagraph) -> Self {
            self.snapshot.document.body.push(DocxBlock::Paragraph(paragraph));
            self.snapshot = crate::artifacts::docx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_docx(self.snapshot.document);
            self
        }

        /// ➕️ Appends a single-run plain-text paragraph.
        pub fn add_text_paragraph(self, text: impl Into<String>) -> Self {
            self.add_paragraph(DocxParagraph::text(text.into()))
        }

        /// ➕️ Appends a paragraph made of the given runs (basic bold/italic/underline formatting).
        pub fn add_runs(self, runs: Vec<DocxRun>) -> Self {
            self.add_paragraph(DocxParagraph { runs, style: None, extra_paragraph_properties: Vec::new() })
        }

        /// ➕️ Appends a table.
        pub fn add_table(mut self, table: DocxTable) -> Self {
            self.snapshot.document.body.push(DocxBlock::Table(table));
            self.snapshot = crate::artifacts::docx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_docx(self.snapshot.document);
            self
        }

        /// ➕️ Appends (or replaces, by `id`) a named style.
        pub fn add_style(mut self, style: DocxStyle) -> Self {
            if let Some(existing) = self.snapshot.document.styles.iter_mut().find(|s| s.id == style.id) {
                *existing = style;
            } else {
                self.snapshot.document.styles.push(style);
            }
            self.snapshot = crate::artifacts::docx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_docx(self.snapshot.document);
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
    use crate::artifacts::docx::DocxSnapshot;

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.docx` parts.
    #[derive(Clone, Debug, Default)]
    pub struct DocxParts {
        pub snapshot: Option<DocxSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.docx` (ecma-376/✳️any) sources.
    pub struct DocxAnalyzerAnalysis;

    impl ArtifactAnalysis for DocxAnalyzerAnalysis {
        type Parts = DocxParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            // 🕵️ Real sniff: OPC-shaped bytes (real `[Content_Types].xml`) whose root officeDocument
            // relationship resolves under `word/` — disambiguates from xlsx/pptx, which share the
            // same zip magic and OPC shape but resolve under `xl/`/`ppt/` instead.
            match source {
                AnalyzeSource::Binary(bytes) if crate::artifacts::docx::standards::v_ecma_376::subsets::any::io::import::deserializers::sniff_docx_bytes(bytes) => IoConfidence::High,
                AnalyzeSource::Binary(_) | AnalyzeSource::Text(_) => IoConfidence::Low,
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = DocxParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <DocxSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                    AnalyzeSource::Binary(bytes) => match <DocxSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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

//#region 🔖️DocumentHelpers
pub fn empty_docx_snapshot() -> DocxSnapshot { DocxSnapshot::default() }

/// 📄️ FG-wave: the demo `stdio.docx` document — a genuinely non-trivial `DocxSnapshot` exercising
/// a styled heading paragraph, a mixed-formatting run (bold/italic/plain), a 2x2 table (recursing
/// through `Table -> row -> cell -> Paragraph`), two named styles (one `based_on` the other), and
/// one unmodeled raw OPC part (`word/numbering.xml`, verbatim-retained). The single source of
/// truth for `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` (both are
/// literally this snapshot's `print_dsl`/`encode_pack` output, asserted equal by
/// `fixture_honesty_law` below) — same shape `📷️png/…/⚙️engine/🦀️component.rs`'s own
/// `demo_png_snapshot()` establishes.
pub fn demo_docx_snapshot() -> DocxSnapshot {
    use crate::artifacts::docx::schema::snapshot::{DocxBlock, DocxParagraph, DocxRun, DocxStyle, DocxTable, DocxTableCell, DocxTableRow};
    let document = DocxDocument {
        body: vec![
            DocxBlock::Paragraph(DocxParagraph { style: Some("Heading1".into()), ..DocxParagraph::text("Semio Demo") }),
            DocxBlock::Paragraph(DocxParagraph {
                runs: vec![
                    DocxRun { text: "Bold and ".into(), bold: true, ..Default::default() },
                    DocxRun { text: "italic".into(), italic: true, ..Default::default() },
                    DocxRun { text: " text".into(), ..Default::default() },
                ],
                style: None,
                extra_paragraph_properties: Vec::new(),
            }),
            DocxBlock::Table(DocxTable {
                rows: vec![
                    DocxTableRow {
                        cells: vec![
                            DocxTableCell { blocks: vec![DocxBlock::paragraph("R1C1")], ..Default::default() },
                            DocxTableCell { blocks: vec![DocxBlock::paragraph("R1C2")], ..Default::default() },
                        ],
                        ..Default::default()
                    },
                    DocxTableRow {
                        cells: vec![
                            DocxTableCell { blocks: vec![DocxBlock::paragraph("R2C1")], ..Default::default() },
                            DocxTableCell { blocks: vec![DocxBlock::paragraph("R2C2")], ..Default::default() },
                        ],
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
        ],
        styles: vec![
            DocxStyle { id: "Normal".into(), name: "Normal".into(), based_on: None },
            DocxStyle { id: "Heading1".into(), name: "heading 1".into(), based_on: Some("Normal".into()) },
        ],
    };
    let mut snap = crate::artifacts::docx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_docx(document);
    snap.opc.set_part(
        "word/numbering.xml",
        "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml",
        b"<w:numbering/>".to_vec(),
    );
    snap
}
//#endregion 🔖️DocumentHelpers

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec DocxBuilderFacets {
        construction: derived_construction::DocxBuilderConstruction,
        analysis: derived_analysis::DocxAnalyzerAnalysis,
        composition: super::super::io::derived_composition::DocxComposerComposition,
    }
    builder: DocxBuilder,
    analyzer: DocxAnalyzer,
    composer: DocxComposer,
);
//#endregion 🧬️DerivedArtifactFacets
