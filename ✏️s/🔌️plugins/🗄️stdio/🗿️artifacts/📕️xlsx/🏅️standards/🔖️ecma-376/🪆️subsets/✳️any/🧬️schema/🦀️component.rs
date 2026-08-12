//! 🧬️ XlsxArtifact schema — full artifact state.

use crate::artifacts::xlsx::schema::snapshot::XlsxWorkbook;
use crate::artifacts::xlsx::XlsxSnapshot;
use crate::artifacts::zip::opc::OpcPackage;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region Artifact
/// 🧬️ Full `stdio.xlsx` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xlsx")]
pub struct XlsxArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub opc: OpcPackage,
    #[state(persistent)]
    #[serde(default)]
    pub workbook: XlsxWorkbook,
}
//#endregion Artifact

//#region Conversions
impl Default for XlsxArtifact {
    fn default() -> Self {
        Self::from_snapshot(XlsxSnapshot::default())
    }
}

impl XlsxArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> XlsxSnapshot {
        XlsxSnapshot { schema: self.schema.clone(), opc: self.opc.clone(), workbook: self.workbook.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: XlsxSnapshot) -> Self {
        Self { schema: snapshot.schema, opc: snapshot.opc, workbook: snapshot.workbook }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: XlsxSnapshot) {
        self.schema = snapshot.schema;
        self.opc = snapshot.opc;
        self.workbook = snapshot.workbook;
    }
}
//#endregion Conversions

//#region Descriptor
/// 🧬️ Descriptor for `s.stdio.xlsx`.
pub fn xlsx_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.xlsx",
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
    use crate::artifacts::xlsx::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet};
    use crate::artifacts::xlsx::{XlsxDiff, XlsxMutation, XlsxSnapshot};

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.xlsx` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct XlsxBuilderConstruction {
        snapshot: XlsxSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
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
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<XlsxSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<XlsxSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = crate::artifacts::xlsx::schema::mutations::apply_xlsx_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <XlsxDiff as protocol::MutationDiff<XlsxSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
    //#endregion 🔖️Builder

    //#region 🔖️TypedConstructors
    /// 🧱️ Typed content constructors — build a workbook from sheets and rows of cell values,
    /// auto-assigning `(row, col)` coordinates left-to-right (`col` 0-based).
    impl XlsxBuilderConstruction {
        /// ➕️ Appends a new (initially empty) sheet and makes it the active sheet for `add_row`.
        pub fn add_sheet(mut self, name: impl Into<String>) -> Self {
            self.snapshot.workbook.sheets.push(XlsxSheet { name: name.into(), cells: Vec::new() });
            self.rebuild()
        }

        /// ➕️ Appends a row of values to the active sheet (the most recently added one), assigning
        /// `(row: index, col: 0..)` coordinates left-to-right.
        pub fn add_row(mut self, index: u32, values: Vec<XlsxCellValue>) -> Self {
            if let Some(sheet) = self.snapshot.workbook.sheets.last_mut() {
                sheet.cells.extend(values.into_iter().enumerate().map(|(col, value)| XlsxCell { row: index, col: col as u32, value }));
            }
            self.rebuild()
        }

        fn rebuild(mut self) -> Self {
            self.snapshot = crate::artifacts::xlsx::engine::build_minimal_xlsx(self.snapshot.workbook);
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
    use crate::artifacts::xlsx::XlsxSnapshot;

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.xlsx` parts.
    #[derive(Clone, Debug, Default)]
    pub struct XlsxParts {
        pub snapshot: Option<XlsxSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.xlsx` (ecma-376/✳️any) sources.
    pub struct XlsxAnalyzerAnalysis;

    impl ArtifactAnalysis for XlsxAnalyzerAnalysis {
        type Parts = XlsxParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            // 🕵️ Real sniff: OPC-shaped bytes whose root officeDocument relationship resolves under
            // `xl/` — disambiguates from docx/pptx, which share the same zip magic and OPC shape.
            match source {
                AnalyzeSource::Binary(bytes) if crate::artifacts::xlsx::engine::sniff_xlsx_bytes(bytes) => IoConfidence::High,
                AnalyzeSource::Binary(_) | AnalyzeSource::Text(_) => IoConfidence::Low,
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = XlsxParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <XlsxSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                    AnalyzeSource::Binary(bytes) => match <XlsxSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec XlsxBuilderFacets {
        construction: derived_construction::XlsxBuilderConstruction,
        analysis: derived_analysis::XlsxAnalyzerAnalysis,
        composition: super::super::io::derived_composition::XlsxComposerComposition,
    }
    builder: XlsxBuilder,
    analyzer: XlsxAnalyzer,
    composer: XlsxComposer,
);
//#endregion 🧬️DerivedArtifactFacets
