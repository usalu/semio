//! 🚪️ IO stdio.docx (ecma-376/✳️any) — registration flows through `docx::declaration()`
//! (`🗄️stdio/🗿️artifacts/📜️docx/🦀️.rs`), not a side-effecting `register()`; `⚙️engine`
//! dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — its orphaned
//! `register()`/`register_artifact_inferences()`/`register_pilot_languages()` (zero callers,
//! superseded by `declaration()`) deleted outright; `DocxError` + shared OPC/XML constants below
//! (used by both `📥️import/🧩️deserializers` and `📤️export/🧵️serializers`); `io_registry` moved
//! here from `⚙️engine`, live (`docx::declaration()`'s `.composers(...)` and this artifact's own
//! root `io_registry` both reach it).
//#region 🔖️Error
/// ⚠️ Typed docx decode/encode failure — a package this engine cannot honestly interpret is
/// never fabricated into a partial/empty document.
#[derive(Clone, Debug, PartialEq)]
pub enum DocxError {
    Opc(semio_s_artifact_stdio_zip::opc::OpcError),
    Ownership(semio_framework_value::ValueError),
    MissingMainDocumentRelationship,
    MissingPart(String),
    Xml { part: String, detail: String },
    Malformed(String),
}

impl std::fmt::Display for DocxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Opc(e) => write!(f, "docx: {e}"),
            Self::Ownership(detail) => write!(f, "docx: retained ownership: {}",detail.message),
            Self::MissingMainDocumentRelationship => write!(f, "docx: package root has no officeDocument relationship"),
            Self::MissingPart(p) => write!(f, "docx: missing required part {p}"),
            Self::Xml { part, detail } => write!(f, "docx: xml in {part}: {detail}"),
            Self::Malformed(detail) => write!(f, "docx: {detail}"),
        }
    }
}

impl std::error::Error for DocxError {}
/// 🪢️ Package and ownership layers keep their own kind; every document-structure refusal is invalid input.
impl From<DocxError> for semio_framework_value::ValueError {
    fn from(error: DocxError) -> Self {
        let kind = match &error { DocxError::Opc(error) => error.refusal_kind(), DocxError::Ownership(error) => error.kind, _ => semio_framework_value::ValueRefusalKind::InvalidValue };
        Self::new(kind, error.to_string())
    }
}

impl From<semio_s_artifact_stdio_zip::opc::OpcError> for DocxError {
    fn from(e: semio_s_artifact_stdio_zip::opc::OpcError) -> Self {
        Self::Opc(e)
    }
}

impl From<semio_framework_value::ValueError> for DocxError {
    fn from(error: semio_framework_value::ValueError) -> Self {
        Self::Ownership(error)
    }
}
impl DocxError {
    /// 🧭️ Retains actual owned causes while classifying authored package validation failures.
    pub fn into_value_error(self) -> semio_framework_value::ValueError {
        match self {
            Self::Ownership(error) => error,
            Self::Opc(error) => error.into_value_error(),
            error @ (Self::MissingMainDocumentRelationship | Self::MissingPart(_) | Self::Xml { .. } | Self::Malformed(_)) => semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string()),
        }
    }
}
//#endregion 🔖️Error

//#region 🔖️Constants
/// 🏅️ ISO/IEC 29500-1 Strict's officeDocument relationship type (`📏️strict`'s
/// `STRICT_REL_BASE`/`officeDocument`) — decode must recognize this alongside the transitional
/// `REL_TYPE_OFFICE_DOCUMENT`, since this `✳️any`-level decoder is shared by every subset
/// including `📏️strict`, which legitimately never uses the transitional relationship type.
pub const STRICT_REL_TYPE_OFFICE_DOCUMENT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument";
pub const W_NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
pub const R_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
pub const MAIN_DOCUMENT_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
pub const MAIN_DOCUMENT_PART: &str = "word/document.xml";
pub const STYLES_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml";
pub const STYLES_PART: &str = "word/styles.xml";
/// 🧭️ The styles relationship's `Target`, RELATIVE TO ITS OWNER'S DIRECTORY (`word/`) per OPC
/// §9.3 -- NOT `STYLES_PART` verbatim, which is package-root-relative and would resolve (via
/// `resolve_relationship_target("word/document.xml", "word/styles.xml")`) to the wrong path
/// `word/word/styles.xml`. This is the OPC module's own documented "#1 relative-target gotcha".
pub const STYLES_REL_TARGET: &str = "styles.xml";
pub const REL_TYPE_STYLES: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";
/// 🏅️ ISO/IEC 29500-1 Strict's styles relationship type — the exact counterpart of
/// [`STRICT_REL_TYPE_OFFICE_DOCUMENT`], and needed for the same reason. A package that has been
/// stamped Strict (`📏️strict`'s `set-relationship-base`/`set-snapshot`) carries THIS type on its
/// styles relationship and never the transitional one, so a writer that recognizes only
/// [`REL_TYPE_STYLES`] concludes the package has no styles relationship and appends a second,
/// transitional-typed one beside the strict one it just failed to see — real package corruption,
/// caught by `📏️mutate-docx-ecma-376-strict`'s differential rows the moment that case first ran a
/// subject half.
pub const STRICT_REL_TYPE_STYLES: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/styles";
//#endregion 🔖️Constants

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_ecma_376::subsets::base::io::DocxAnalyzer;
    use crate::DocxSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

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
            self.snapshot = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_docx(document);
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
            self.snapshot = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_docx(document);
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
            self.snapshot = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_docx(document);
            self
        }
    }
    //#endregion 🔖️TypedConstructors
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::DocxSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

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
