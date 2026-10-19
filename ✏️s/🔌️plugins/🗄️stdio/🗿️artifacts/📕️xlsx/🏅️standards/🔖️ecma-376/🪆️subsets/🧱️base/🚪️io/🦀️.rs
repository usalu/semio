//! 🚪️ SpreadsheetML physical codecs and composition.

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_ecma_376::subsets::base::io::XlsxAnalyzer;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };

    pub struct XlsxComposerComposition;

    impl ArtifactComposition for XlsxComposerComposition {
        type Snapshot = XlsxSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_ZIP]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_ZIP)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "XlsxComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = XlsxAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "XlsxComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v_ecma_376::subsets::base::io::XlsxComposer as XlsxRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<XlsxRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry




pub mod derived_construction {
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxCell;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxCellValue;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSheet;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::diff::XlsxDiff;
    use semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::XlsxMutation;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.xlsx` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct XlsxBuilderConstruction {
        snapshot: XlsxSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for XlsxBuilderConstruction {
        type Snapshot = XlsxSnapshot;
        type Mutation = XlsxMutation;
        type Diff = XlsxDiff;
        fn empty() -> Self {
            Self { snapshot: XlsxSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<XlsxSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<XlsxSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let (next, diff) = store::os_store::replay::apply_outcome(&self.snapshot, protocol::Mutation::diff(&mutation, &self.snapshot));
            self.snapshot = next;
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = semio_framework_os_kernel::apply_diff(&diff, &self.snapshot)?;
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
    /// 🧱️ Typed content constructors — build a workbook from sheets and rows of cell values,
    /// auto-assigning `(row, col)` coordinates left-to-right (`col` 0-based).
    impl XlsxBuilderConstruction {
        /// ➕️ Appends a new (initially empty) sheet and makes it the active sheet for `add_row`.
        pub fn add_sheet(mut self, name: impl Into<String>) -> Self {
            let mut workbook = self.snapshot.project_workbook().unwrap_or_default();
            workbook.sheets.push(XlsxSheet { name: name.into(), cells: Vec::new() });
            self.snapshot = semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(workbook);
            self
        }

        /// ➕️ Appends a row of values to the active sheet (the most recently added one), assigning
        /// `(row: index, col: 0..)` coordinates left-to-right.
        pub fn add_row(mut self, index: u32, values: Vec<XlsxCellValue>) -> Self {
            let mut workbook = self.snapshot.project_workbook().unwrap_or_default();
            if let Some(sheet) = workbook.sheets.last_mut() {
                sheet.cells.extend(values.into_iter().enumerate().map(|(col, value)| XlsxCell { row: index, col: col as u32, value }));
            }
            self.snapshot = semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(workbook);
            self
        }
    }
    //#endregion 🔖️TypedConstructors
}
pub use derived_construction::*;

pub mod derived_analysis {
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.xlsx` parts.
    #[derive(Clone, Debug, Default)]
    pub struct XlsxParts {
        pub snapshot: Option<XlsxSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.xlsx` (ecma-376/🧱️base) sources.
    pub struct XlsxAnalyzerAnalysis;

    impl ArtifactAnalysis for XlsxAnalyzerAnalysis {
        type Parts = XlsxParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            match source {
                AnalyzeSource::Binary(bytes) if crate::standards::v_ecma_376::subsets::base::io::import::deserializers::sniff_xlsx_bytes(bytes) => semio_framework_plugin::io::Confidence::High,
                AnalyzeSource::Binary(_) | AnalyzeSource::Text(_) => semio_framework_plugin::io::Confidence::Low,
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = XlsxParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <XlsxSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <XlsxSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
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
    pub spec XlsxBuilderFacets {
        construction: XlsxBuilderConstruction,
        analysis: XlsxAnalyzerAnalysis,
        composition: XlsxComposerComposition,
    }
    builder: XlsxBuilder,
    analyzer: XlsxAnalyzer,
    composer: XlsxComposer,
);

impl semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot {
    pub fn part_text(&self, path: &str) -> Option<String> {
        let key = path.trim_start_matches('/');
        self.xml_part(key).map(|part| semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_to_text(&part.document)).or_else(|| self.opc.part_bytes(key).and_then(|bytes| String::from_utf8(bytes.to_vec()).ok()))
    }
}
