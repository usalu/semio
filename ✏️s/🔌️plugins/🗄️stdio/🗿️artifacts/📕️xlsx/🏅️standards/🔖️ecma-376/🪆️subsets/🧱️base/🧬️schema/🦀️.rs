//! 🧬️ XlsxArtifact schema — full artifact state.

use crate::schema::snapshot::{XlsxWorkbook, XlsxXmlPart};
use crate::XlsxSnapshot;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_zip::opc::OpcPackage;

//#region Artifact
/// 🧬️ Full `stdio.xlsx` artifact state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xlsx")]
pub struct XlsxArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub opc: OpcPackage,
    #[state(artifact)]
    #[value(default)]
    pub xml_parts: Vec<XlsxXmlPart>,
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> XlsxSnapshot {
        XlsxSnapshot { schema: self.schema.clone(), opc: self.opc.clone(), xml_parts: self.xml_parts.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: XlsxSnapshot) -> Self {
        Self { schema: snapshot.schema, opc: snapshot.opc, xml_parts: snapshot.xml_parts }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: XlsxSnapshot) {
        self.schema = snapshot.schema;
        self.opc = snapshot.opc;
        self.xml_parts = snapshot.xml_parts;
    }
}
//#endregion Conversions

//#region Descriptor
/// 🧬️ Descriptor for `s.stdio.xlsx`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn xlsx_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.stdio.xlsx",
        artifact: framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
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
    use crate::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet};
    use crate::{XlsxDiff, XlsxMutation, XlsxSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

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
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::schema::mutations::apply_xlsx_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <XlsxDiff as protocol::MutationDiff<XlsxSnapshot>>::apply(&diff, &self.snapshot)?;
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
    /// 🧱️ Typed content constructors — build a workbook from sheets and rows of cell values,
    /// auto-assigning `(row, col)` coordinates left-to-right (`col` 0-based).
    impl XlsxBuilderConstruction {
        /// ➕️ Appends a new (initially empty) sheet and makes it the active sheet for `add_row`.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_sheet(mut self, name: impl Into<String>) -> Self {
            let mut workbook = self.snapshot.project_workbook().unwrap_or_default();
            workbook.sheets.push(XlsxSheet { name: name.into(), cells: Vec::new() });
            self.snapshot = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(workbook);
            self
        }

        /// ➕️ Appends a row of values to the active sheet (the most recently added one), assigning
        /// `(row: index, col: 0..)` coordinates left-to-right.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_row(mut self, index: u32, values: Vec<XlsxCellValue>) -> Self {
            let mut workbook = self.snapshot.project_workbook().unwrap_or_default();
            if let Some(sheet) = workbook.sheets.last_mut() {
                sheet.cells.extend(values.into_iter().enumerate().map(|(col, value)| XlsxCell { row: index, col: col as u32, value }));
            }
            self.snapshot = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(workbook);
            self
        }
    }
    //#endregion 🔖️TypedConstructors
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::XlsxSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

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

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            // 🕵️ Real sniff: OPC-shaped bytes whose root officeDocument relationship resolves under
            // `xl/` — disambiguates from docx/pptx, which share the same zip magic and OPC shape.
            match source {
                AnalyzeSource::Binary(bytes) if crate::standards::v_ecma_376::subsets::base::io::import::deserializers::sniff_xlsx_bytes(bytes) => IoConfidence::High,
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
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <XlsxSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_xlsx_snapshot() -> XlsxSnapshot {
    XlsxSnapshot::default()
}

/// 📄️ The demo `stdio.xlsx` document — a genuinely non-trivial `XlsxSnapshot` with `SharedString`, `Number`, `Boolean`,
/// `Formula` (with a cached value) and `InlineString` cells on two sheets, plus one unmodeled XML part (`xl/styles.xml`)
/// carried verbatim in the XML authority lane. Stated in the package
/// normal form (XML parts path-ascending), so it is a fixed point of `encode_xlsx`/`decode_xlsx`. The single source of truth
/// for `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` (literally this snapshot's `print_dsl`/`encode_pack`
/// output, asserted by `fixture_honesty_law`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_xlsx_snapshot() -> XlsxSnapshot {
    use crate::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet};
    use crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx;
    use semio_s_artifact_stdio_xml::schema::snapshot::xml_document_from_text;
    const STYLES_PART: &str = "xl/styles.xml";
    const STYLES_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml";
    let workbook = XlsxWorkbook {
        sheets: vec![
            XlsxSheet {
                name: "Sheet1".into(),
                cells: vec![
                    XlsxCell { row: 1, col: 0, value: XlsxCellValue::SharedString(0) },
                    XlsxCell { row: 1, col: 1, value: XlsxCellValue::SharedString(1) },
                    XlsxCell { row: 2, col: 0, value: XlsxCellValue::SharedString(2) },
                    XlsxCell { row: 2, col: 1, value: XlsxCellValue::Number(95.5) },
                    XlsxCell { row: 3, col: 0, value: XlsxCellValue::Boolean(true) },
                    XlsxCell { row: 3, col: 1, value: XlsxCellValue::Formula { expr: "SUM(B2:B2)".into(), cached: Some(Box::new(XlsxCellValue::Number(95.5))) } },
                ],
            },
            XlsxSheet { name: "Totals".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::InlineString("Total Score".into()) }] },
        ],
        shared_strings: vec!["Name".into(), "Score".into(), "Alice".into()],
    };
    let mut snap = build_minimal_xlsx(workbook);
    snap.opc.content_types.set_override(STYLES_PART, STYLES_CONTENT_TYPE);
    snap.xml_parts.push(XlsxXmlPart {
        path: STYLES_PART.into(),
        content_type: STYLES_CONTENT_TYPE.into(),
        document: xml_document_from_text("<styleSheet xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\"/>").expect("valid demo styles XML"),
    });
    snap.xml_parts.sort_by(|left, right| left.path.cmp(&right.path));
    snap
}
//#endregion 🔖️DocumentHelpers

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec XlsxBuilderFacets {
        construction: XlsxBuilderConstruction,
        analysis: XlsxAnalyzerAnalysis,
        composition: super::super::io::derived_composition::XlsxComposerComposition,
    }
    builder: XlsxBuilder,
    analyzer: XlsxAnalyzer,
    composer: XlsxComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
