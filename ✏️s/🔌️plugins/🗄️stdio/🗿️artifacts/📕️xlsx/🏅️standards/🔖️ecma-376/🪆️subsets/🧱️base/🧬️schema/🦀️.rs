//! 🧬️ XlsxArtifact schema — full artifact state.

use crate::schema::snapshot::XlsxWorkbook;
use crate::XlsxSnapshot;
use semio_s_artifact_stdio_zip::opc::OpcPackage;
use framework_schema::ArtifactSchema;

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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> XlsxSnapshot {
        XlsxSnapshot { schema: self.schema.clone(), opc: self.opc.clone(), workbook: self.workbook.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: XlsxSnapshot) -> Self {
        Self { schema: snapshot.schema, opc: snapshot.opc, workbook: snapshot.workbook }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: XlsxSnapshot) {
        self.schema = snapshot.schema;
        self.opc = snapshot.opc;
        self.workbook = snapshot.workbook;
    }
}
//#endregion Conversions

//#region Descriptor
/// 🧬️ Descriptor for `s.stdio.xlsx`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn xlsx_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.stdio.xlsx",
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
            self.snapshot.workbook.sheets.push(XlsxSheet { name: name.into(), cells: Vec::new() });
            self.rebuild()
        }

        /// ➕️ Appends a row of values to the active sheet (the most recently added one), assigning
        /// `(row: index, col: 0..)` coordinates left-to-right.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_row(mut self, index: u32, values: Vec<XlsxCellValue>) -> Self {
            if let Some(sheet) = self.snapshot.workbook.sheets.last_mut() {
                sheet.cells.extend(values.into_iter().enumerate().map(|(col, value)| XlsxCell { row: index, col: col as u32, value }));
            }
            self.rebuild()
        }

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn rebuild(mut self) -> Self {
            self.snapshot = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(self.snapshot.workbook);
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

/// 📄️ FG-wave: the demo `stdio.xlsx` document — a genuinely non-trivial `XlsxSnapshot` exercising
/// every `XlsxCellValue` variant (`SharedString`, `Number`, `Boolean`, `Formula` with a cached
/// value, `InlineString`), two sheets, and one unmodeled raw OPC part (`xl/styles.xml`,
/// verbatim-retained). The single source of truth for
/// `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` (both are literally
/// this snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law`
/// below) — same shape docx's own `demo_docx_snapshot()` establishes (this wave's OPC
/// pattern-setter).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_xlsx_snapshot() -> XlsxSnapshot {
    use crate::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet};
    use crate::standards::v_ecma_376::subsets::base::io::export::serializers::{build_minimal_xlsx, encode_xlsx};
    use crate::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_xlsx;
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
    snap.opc.set_part("xl/styles.xml", "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml", b"<styleSheet/>".to_vec());
    // 🩹 Normalize `opc.parts`' ORDER to the canonical post-regeneration shape `encode_xlsx`
    // always produces (`regenerate_workbook_parts`'s `retain` keeps any unmodeled part -- here
    // `xl/styles.xml` -- in its CURRENT relative position, then re-appends `workbook.xml`/
    // `sharedStrings.xml`/every worksheet AFTER it; that shape is a fixed point of a further
    // `encode_xlsx`/`decode_xlsx` round trip, but the pre-round-trip in-memory order this
    // function would otherwise return is NOT). Without this, `fixture_honesty_law`'s direct
    // `parsed == demo()` comparison fails on part ORDER alone even though every part's CONTENT
    // round-trips correctly (`XlsxSnapshot`'s derived `PartialEq` is order-sensitive on
    // `opc.parts: Vec<OpcPart>`) -- a real, previously-undiscovered fixture-construction bug this
    // wave's own `fixture_honesty_law` caught live, not assumed.
    let bytes = encode_xlsx(&snap).expect("encode demo xlsx for part-order normalization");
    decode_xlsx(&bytes).expect("decode demo xlsx for part-order normalization")
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
