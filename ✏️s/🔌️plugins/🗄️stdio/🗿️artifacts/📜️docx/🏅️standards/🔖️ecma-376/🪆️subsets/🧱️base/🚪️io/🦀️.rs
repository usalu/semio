//! 🚪️ IO stdio.docx (ecma-376/✳️any) — registration flows through `docx::declaration()`
//! (`🗄️stdio/🗿️artifacts/📜️docx/🦀️.rs`), not a side-effecting `register()`; `⚙️engine`
//! dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — its orphaned
//! `register()`/`register_artifact_inferences()`/`register_pilot_languages()` (zero callers,
//! superseded by `declaration()`) deleted outright; `DocxError` + shared OPC/XML constants below
//! (used by both `📥️import/🧩️deserializers` and `📤️export/🧵️serializers`); `io_registry` moved
//! here from `⚙️engine`, live (`docx::declaration()`'s `.composers(...)` and this artifact's own
//! root `io_registry` both reach it).
use crate::standards::v_ecma_376::subsets::base::schema::{vocabulary::*,refusal::*};


//#endregion 🔖️Constants

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_ecma_376::subsets::base::io::DocxAnalyzer;
    use crate::DocxSnapshot;
    use {semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::ComposeError,semio_framework_plugin::ComposeSource,semio_framework_plugin::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };

    pub struct DocxComposerComposition;

    impl ArtifactComposition for DocxComposerComposition {
        type Snapshot = DocxSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_ZIP]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_ZIP)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "DocxComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = DocxAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "DocxComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v_ecma_376::subsets::base::io::DocxComposer as DocxRawAnyComposer;
    use crate::standards::v_ecma_376::subsets::strict::io::DocxStrictComposer;
    use crate::standards::v_ecma_376::subsets::transitional::io::DocxTransitionalComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<DocxRawAnyComposer>(), composer_entry_of::<DocxStrictComposer>(), composer_entry_of::<DocxTransitionalComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::schema::snapshot::{DocxBlock, DocxParagraph, DocxRun, DocxStyle, DocxTable};
    use crate::{DocxDiff, DocxMutation, DocxSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.docx` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct DocxBuilderConstruction {
        snapshot: DocxSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
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
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<DocxSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<DocxSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::schema::mutations::apply_docx_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <DocxDiff as protocol::MutationDiff<DocxSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
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
            let mut document = self.snapshot.project_document().unwrap_or_default();
            document.body.push(DocxBlock::Paragraph(paragraph));
            self.snapshot = crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_docx(document);
            self
        }

        /// ➕️ Appends a single-run plain-text paragraph.
        pub async fn add_text_paragraph(self, text: impl Into<String>) -> Self {
            self.add_paragraph(DocxParagraph::text(text.into()))
        }

        /// ➕️ Appends a paragraph made of the given runs (basic bold/italic/underline formatting).
        pub async fn add_runs(self, runs: Vec<DocxRun>) -> Self {
            self.add_paragraph(DocxParagraph { runs, style: None, extra_paragraph_properties: Vec::new() })
        }

        /// ➕️ Appends a table.
        pub fn add_table(mut self, table: DocxTable) -> Self {
            let mut document = self.snapshot.project_document().unwrap_or_default();
            document.body.push(DocxBlock::Table(table));
            self.snapshot = crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_docx(document);
            self
        }

        /// ➕️ Appends (or replaces, by `id`) a named style.
        pub fn add_style(mut self, style: DocxStyle) -> Self {
            let mut document = self.snapshot.project_document().unwrap_or_default();
            if let Some(existing) = document.styles.iter_mut().find(|existing| existing.id == style.id) {
                *existing = style;
            } else {
                document.styles.push(style);
            }
            self.snapshot = crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_docx(document);
            self
        }
    }
    //#endregion 🔖️TypedConstructors
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::DocxSnapshot;
    use {semio_framework_plugin::Analysis,semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoConfidence,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

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
                AnalyzeSource::Binary(bytes) if crate::standards::v_ecma_376::subsets::base::io::import::deserializers::sniff_docx_bytes(bytes) => IoConfidence::High,
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
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <DocxSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
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

semio_framework_plugin::derive_artifact_facets!(
    pub spec DocxBuilderFacets {
        construction: DocxBuilderConstruction,
        analysis: DocxAnalyzerAnalysis,
        composition: crate::standards::v_ecma_376::subsets::base::io::derived_composition::DocxComposerComposition,
    }
    builder: DocxBuilder,
    analyzer: DocxAnalyzer,
    composer: DocxComposer,
);
