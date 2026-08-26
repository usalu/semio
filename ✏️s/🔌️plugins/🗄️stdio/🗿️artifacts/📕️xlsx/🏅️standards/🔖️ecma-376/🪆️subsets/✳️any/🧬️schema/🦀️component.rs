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
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub opc: OpcPackage,
    #[state(artifact)]
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
    use crate::artifacts::xlsx::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet};
    use crate::artifacts::xlsx::{XlsxDiff, XlsxMutation, XlsxSnapshot};
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
            let diff = crate::artifacts::xlsx::schema::mutations::apply_xlsx_mutation(&mut self.snapshot, &mutation);
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
            self.snapshot = crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_xlsx(self.snapshot.workbook);
            self
        }
    }
    //#endregion 🔖️TypedConstructors
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::xlsx::XlsxSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

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
                AnalyzeSource::Binary(bytes) if crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::import::deserializers::sniff_xlsx_bytes(bytes) => IoConfidence::High,
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
/// `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` (both are literally
/// this snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law`
/// below) — same shape docx's own `demo_docx_snapshot()` establishes (this wave's OPC
/// pattern-setter).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_xlsx_snapshot() -> XlsxSnapshot {
    use crate::artifacts::xlsx::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet};
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::export::serializers::{build_minimal_xlsx, encode_xlsx};
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::import::deserializers::decode_xlsx;
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
mod tests {
    use super::*;
    use crate::artifacts::xlsx::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet};
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::export::serializers::{build_minimal_xlsx, encode_xlsx};
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::import::deserializers::{decode_xlsx, sniff_xlsx_bytes};
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::{
        column_index, column_letter, XlsxError, REL_TYPE_OFFICE_DOCUMENT_STRICT, REL_TYPE_SHARED_STRINGS, REL_TYPE_SHARED_STRINGS_STRICT, REL_TYPE_WORKSHEET, SHARED_STRINGS_CONTENT_TYPE, SHARED_STRINGS_PART, WORKBOOK_CONTENT_TYPE, WORKBOOK_PART,
        WORKSHEET_CONTENT_TYPE,
    };
    use crate::artifacts::xml::schema::snapshot::xml_document_from_text;
    use crate::artifacts::zip::opc::{self, OpcPackage, RELS_CONTENT_TYPE, REL_TYPE_OFFICE_DOCUMENT};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn cell(row: u32, col: u32, value: XlsxCellValue) -> XlsxCell {
        XlsxCell { row, col, value }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_workbook() -> XlsxWorkbook {
        XlsxWorkbook {
            sheets: vec![
                XlsxSheet {
                    name: "Numbers".into(),
                    cells: vec![
                        cell(1, 0, XlsxCellValue::SharedString(0)),
                        cell(1, 1, XlsxCellValue::SharedString(1)),
                        cell(2, 0, XlsxCellValue::SharedString(2)),
                        cell(2, 1, XlsxCellValue::Number(9.5)),
                        cell(3, 0, XlsxCellValue::SharedString(2)),
                        cell(3, 1, XlsxCellValue::Number(-3.0)),
                        cell(4, 0, XlsxCellValue::Boolean(true)),
                        cell(4, 1, XlsxCellValue::Empty),
                    ],
                },
                XlsxSheet { name: "Second".into(), cells: vec![cell(1, 0, XlsxCellValue::SharedString(2))] },
            ],
            shared_strings: vec!["Name".into(), "Score".into(), "Alice".into()],
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn column_letters_follow_spreadsheet_convention() {
        assert_eq!(column_letter(0), "A");
        assert_eq!(column_letter(25), "Z");
        assert_eq!(column_letter(26), "AA");
        assert_eq!(column_letter(27), "AB");
        assert_eq!(column_letter(51), "AZ");
        assert_eq!(column_letter(52), "BA");
    }

    #[semio_framework_async_macros::async_test]
    async fn column_index_is_the_real_inverse_of_column_letter() {
        for i in [0u32, 1, 25, 26, 27, 51, 52, 700] {
            assert_eq!(column_index(&column_letter(i)), Some(i), "round trip failed for {i}");
        }
        assert_eq!(column_index(""), None);
        assert_eq!(column_index("1A"), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn builder_produces_minimal_valid_package_that_decodes_back() {
        let snap = build_minimal_xlsx(sample_workbook());
        let bytes = encode_xlsx(&snap).expect("encode minimal package");
        assert!(opc::sniff_opc_bytes(&bytes));
        assert!(sniff_xlsx_bytes(&bytes));
        let decoded = decode_xlsx(&bytes).expect("decode minimal package");
        assert_eq!(decoded.workbook, sample_workbook());
    }

    #[semio_framework_async_macros::async_test]
    async fn shared_strings_are_carried_verbatim_never_resolved_or_deduped() {
        // 🎯️ The engine no longer resolves `SharedString(idx)` into literal text, nor dedupes on
        // encode -- `workbook.shared_strings` IS the SST, passed through directly. Confirms the
        // real bytes carry the table unchanged AND every cell keeps its own index (not a
        // resolved-text copy the old `Text` variant used to collapse into).
        let snap = build_minimal_xlsx(sample_workbook());
        let sst_bytes = snap.opc.part_bytes("xl/sharedStrings.xml").expect("sharedStrings.xml part present");
        let sst_xml = xml_document_from_text(std::str::from_utf8(sst_bytes).unwrap()).expect("parse sst");
        // 🩹 `shared_strings_from_xml` is module-private to the deserializers component; this test
        // re-derives the same strings via a full decode round trip instead of reaching in directly.
        let _ = sst_xml;
        let bytes = encode_xlsx(&snap).expect("encode");
        let re_decoded = decode_xlsx(&bytes).expect("decode");
        assert_eq!(re_decoded.workbook.shared_strings, vec!["Name".to_string(), "Score".to_string(), "Alice".to_string()]);
        assert_eq!(re_decoded.workbook.sheets[0].cells.iter().find(|c| c.row == 2 && c.col == 0).unwrap().value, XlsxCellValue::SharedString(2));
        assert_eq!(re_decoded.workbook.sheets[0].cells.iter().find(|c| c.row == 3 && c.col == 0).unwrap().value, XlsxCellValue::SharedString(2));
        assert_eq!(re_decoded.workbook.sheets[1].cells[0].value, XlsxCellValue::SharedString(2));
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_resolves_real_hand_built_package_with_every_cell_type() {
        // Hand-built OOXML: real workbook.xml + worksheet + sharedStrings.xml + all rels wired
        // by hand, not a generator shortcut. Exercises every `XlsxCellValue` variant: shared
        // string, number, boolean, inline string, and a formula with a cached numeric result.
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");

        let sst_xml = concat!(r#"<sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" count="2" uniqueCount="2">"#, "<si><t>Quarter</t></si>", "<si><t>Revenue &amp; Profit</t></si>", "</sst>",);
        opc.set_part(SHARED_STRINGS_PART, SHARED_STRINGS_CONTENT_TYPE, sst_xml.as_bytes().to_vec());

        let sheet_xml = concat!(
            r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">"#,
            "<sheetData>",
            r#"<row r="1"><c r="A1" t="s"><v>0</v></c><c r="B1" t="s"><v>1</v></c></row>"#,
            r#"<row r="2"><c r="A2"><v>4</v></c><c r="B2"><v>123.5</v></c></row>"#,
            r#"<row r="3"><c r="A3" t="b"><v>1</v></c><c r="B3" t="inlineStr"><is><t>literal</t></is></c></row>"#,
            r#"<row r="4"><c r="A4"><f>SUM(A2:B2)</f><v>127.5</v></c></row>"#,
            "</sheetData>",
            "</worksheet>",
        );
        opc.set_part("xl/worksheets/sheet1.xml", WORKSHEET_CONTENT_TYPE, sheet_xml.as_bytes().to_vec());

        let workbook_xml = concat!(
            r#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">"#,
            r#"<sheets><sheet name="Q1" sheetId="1" r:id="rId1"/></sheets>"#,
            "</workbook>",
        );
        opc.set_part(WORKBOOK_PART, WORKBOOK_CONTENT_TYPE, workbook_xml.as_bytes().to_vec());

        opc.add_relationship(WORKBOOK_PART, "rId1", REL_TYPE_WORKSHEET, "worksheets/sheet1.xml");
        opc.add_relationship(WORKBOOK_PART, "rId2", REL_TYPE_SHARED_STRINGS, "sharedStrings.xml");
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, WORKBOOK_PART);

        let bytes = opc::encode_opc(&opc).expect("encode hand-built package");
        let decoded = decode_xlsx(&bytes).expect("decode hand-built xlsx");

        assert_eq!(decoded.workbook.sheets.len(), 1);
        assert_eq!(decoded.workbook.sheets[0].name, "Q1");
        assert_eq!(decoded.workbook.shared_strings, vec!["Quarter".to_string(), "Revenue & Profit".to_string()]);
        let cells = &decoded.workbook.sheets[0].cells;
        let at = |row: u32, col: u32| cells.iter().find(|c| c.row == row && c.col == col).map(|c| &c.value);
        assert_eq!(at(1, 0), Some(&XlsxCellValue::SharedString(0)));
        assert_eq!(at(1, 1), Some(&XlsxCellValue::SharedString(1)));
        assert_eq!(at(2, 0), Some(&XlsxCellValue::Number(4.0)));
        assert_eq!(at(2, 1), Some(&XlsxCellValue::Number(123.5)));
        assert_eq!(at(3, 0), Some(&XlsxCellValue::Boolean(true)));
        assert_eq!(at(3, 1), Some(&XlsxCellValue::InlineString("literal".into())));
        assert_eq!(at(4, 0), Some(&XlsxCellValue::Formula { expr: "SUM(A2:B2)".into(), cached: Some(Box::new(XlsxCellValue::Number(127.5))) }));
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_rejects_out_of_range_shared_string_index() {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.set_part(SHARED_STRINGS_PART, SHARED_STRINGS_CONTENT_TYPE, br#"<sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" count="0" uniqueCount="0"></sst>"#.to_vec());
        opc.set_part("xl/worksheets/sheet1.xml", WORKSHEET_CONTENT_TYPE, br#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData><row r="1"><c r="A1" t="s"><v>7</v></c></row></sheetData></worksheet>"#.to_vec());
        opc.set_part(
            WORKBOOK_PART,
            WORKBOOK_CONTENT_TYPE,
            br#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="S" sheetId="1" r:id="rId1"/></sheets></workbook>"#
                .to_vec(),
        );
        opc.add_relationship(WORKBOOK_PART, "rId1", REL_TYPE_WORKSHEET, "worksheets/sheet1.xml");
        opc.add_relationship(WORKBOOK_PART, "rId2", REL_TYPE_SHARED_STRINGS, "sharedStrings.xml");
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, WORKBOOK_PART);
        let bytes = opc::encode_opc(&opc).expect("encode");

        let err = decode_xlsx(&bytes).expect_err("out-of-range shared-string index must be rejected, not silently empty");
        assert!(matches!(err, XlsxError::Malformed(_)));
    }

    #[semio_framework_async_macros::async_test]
    async fn unmodeled_parts_survive_decode_encode_verbatim() {
        let snap = build_minimal_xlsx(sample_workbook());
        let mut opc = snap.opc.clone();
        opc.set_part("xl/styles.xml", "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml", b"<styleSheet/>".to_vec());
        let bytes = opc::encode_opc(&opc).expect("encode");

        let decoded = decode_xlsx(&bytes).expect("decode");
        assert_eq!(decoded.opc.part_bytes("xl/styles.xml"), Some(b"<styleSheet/>".as_slice()));
        let re_encoded = encode_xlsx(&decoded).expect("re-encode");
        let re_decoded = decode_xlsx(&re_encoded).expect("re-decode");
        assert_eq!(re_decoded.opc.part_bytes("xl/styles.xml"), Some(b"<styleSheet/>".as_slice()));
        assert_eq!(re_decoded.workbook, sample_workbook());
    }

    #[semio_framework_async_macros::async_test]
    async fn analyzer_builder_round_trip() {
        let original = build_minimal_xlsx(sample_workbook());
        let bytes = encode_xlsx(&original).expect("encode");
        let analyzed = decode_xlsx(&bytes).expect("decode");
        let rebuilt = build_minimal_xlsx(analyzed.workbook.clone());
        let rebuilt_bytes = encode_xlsx(&rebuilt).expect("encode rebuilt");
        let reanalyzed = decode_xlsx(&rebuilt_bytes).expect("decode rebuilt");
        assert_eq!(reanalyzed.workbook, analyzed.workbook);
    }

    #[semio_framework_async_macros::async_test]
    async fn shrinking_sheet_count_drops_stale_worksheet_parts() {
        let mut wide = sample_workbook();
        let snap_wide = build_minimal_xlsx(wide.clone());
        assert!(snap_wide.opc.part("xl/worksheets/sheet2.xml").is_some());

        wide.sheets.truncate(1);
        let bytes = encode_xlsx(&XlsxSnapshot::from_parts(snap_wide.opc, wide)).expect("encode narrower workbook");
        let decoded = decode_xlsx(&bytes).expect("decode");
        assert!(decoded.opc.part("xl/worksheets/sheet2.xml").is_none(), "stale second sheet must be dropped, not left orphaned");
        assert_eq!(decoded.workbook.sheets.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_recognizes_strict_office_document_and_shared_strings_relationship_types() {
        // 🏅️ ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3: a genuinely
        // ISO/IEC 29500-1 Strict-shaped package uses the purl.oclc.org relationship TYPE URIs for
        // the package-root officeDocument pointer and the workbook's sharedStrings relationship --
        // without recognizing those (alongside the Transitional ones), decode would reject every
        // real Strict document outright.
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");

        let sst_xml = concat!(r#"<sst xmlns="http://purl.oclc.org/ooxml/spreadsheetml/main" count="1" uniqueCount="1">"#, "<si><t>Strict</t></si>", "</sst>",);
        opc.set_part(SHARED_STRINGS_PART, SHARED_STRINGS_CONTENT_TYPE, sst_xml.as_bytes().to_vec());

        let sheet_xml = concat!(r#"<worksheet xmlns="http://purl.oclc.org/ooxml/spreadsheetml/main">"#, "<sheetData>", r#"<row r="1"><c r="A1" t="s"><v>0</v></c></row>"#, "</sheetData>", "</worksheet>",);
        opc.set_part("xl/worksheets/sheet1.xml", WORKSHEET_CONTENT_TYPE, sheet_xml.as_bytes().to_vec());

        let workbook_xml = concat!(
            r#"<workbook xmlns="http://purl.oclc.org/ooxml/spreadsheetml/main" xmlns:r="http://purl.oclc.org/ooxml/officeDocument/relationships" conformance="strict">"#,
            r#"<sheets><sheet name="S" sheetId="1" r:id="rId1"/></sheets>"#,
            "</workbook>",
        );
        opc.set_part(WORKBOOK_PART, WORKBOOK_CONTENT_TYPE, workbook_xml.as_bytes().to_vec());

        opc.add_relationship(WORKBOOK_PART, "rId1", "http://purl.oclc.org/ooxml/officeDocument/relationships/worksheet", "worksheets/sheet1.xml");
        opc.add_relationship(WORKBOOK_PART, "rId2", REL_TYPE_SHARED_STRINGS_STRICT, "sharedStrings.xml");
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT_STRICT, WORKBOOK_PART);

        let bytes = opc::encode_opc(&opc).expect("encode strict-shaped package");
        assert!(sniff_xlsx_bytes(&bytes), "a Strict-shaped package must still sniff as xlsx");
        let decoded = decode_xlsx(&bytes).expect("decode Strict-shaped package");
        assert_eq!(decoded.workbook.sheets.len(), 1);
        assert_eq!(decoded.workbook.sheets[0].cells[0].value, XlsxCellValue::SharedString(0));
    }

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
        use crate::artifacts::xlsx::schema::{diff, mutations, snapshot};
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
        /// XML parts an xlsx OPC package carries (`📸️snapshot/📝️text/📖️component.grammar.semio`'s
        /// own doc comment explains why -- this artifact's `ArtifactDsl::print_dsl` hex-dumps the
        /// WHOLE binary OPC package, matching this facet's SIBLING binary protocol, not this text
        /// grammar; the two facets describe different LAYERS of the same real artifact, same as
        /// every OPC-family member's own container/contained-parts split). So, UNLIKE a
        /// binary-native pilot's `grammar_conformance_law` (which feeds `print_dsl` output
        /// straight to the recognizer), this law decodes the REAL zip entries `encode_xlsx`
        /// genuinely produces (via `zip::engine::decode_zip`, the same real codec `opc::decode_opc`
        /// itself delegates to) and recognizes EACH real part's own text against the grammar --
        /// direct proof the grammar matches this artifact's own real per-part XML bytes, not an
        /// invented approximation. `worksheet-part`'s own production is generic over the sheet
        /// index, so both `xl/worksheets/sheet1.xml` and `sheet2.xml` are checked against it.
        #[semio_framework_async_macros::async_test]
        async fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);

            let demo = demo_xlsx_snapshot();
            let bytes = encode_xlsx(&demo).expect("encode demo xlsx");
            let zip = crate::artifacts::zip::standards::v2_0::subsets::any::io::decode_zip(&bytes).expect("decode zip");

            let modeled_fixed = ["[Content_Types].xml", "_rels/.rels", "xl/workbook.xml", "xl/_rels/workbook.xml.rels", "xl/sharedStrings.xml"];
            let mut checked = 0;
            for entry in &zip.entries {
                let is_modeled = modeled_fixed.contains(&entry.name.as_str()) || (entry.name.starts_with("xl/worksheets/") && entry.name.ends_with(".xml"));
                if !is_modeled {
                    continue;
                }
                let text = String::from_utf8(entry.data.clone()).unwrap_or_else(|e| panic!("part {:?}: not valid utf-8: {e}", entry.name));
                assert!(recognizer.recognize(&text).unwrap_or(false), "grammar did not recognize real part {:?}:\n{text}", entry.name);
                checked += 1;
            }
            // 5 fixed parts + 2 worksheet parts (`demo_xlsx_snapshot()`'s own 2 sheets).
            assert_eq!(checked, modeled_fixed.len() + 2, "not every modeled part was present in the real zip entries");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `XlsxMutation` variant (`mutations::demo_mutation_cases()`).
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
        /// for every representative `XlsxDiff` (`diff::demo_diff_cases()`).
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
            let demo = demo_xlsx_snapshot();
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
        /// `print_dsl`/`encode_pack` output of `demo_xlsx_snapshot()` -- `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin -- so the
        /// fixtures can never silently drift back to a fake `"68656c6c6f"`-style placeholder again
        /// (see this ticket's own recon note on the pre-FG-wave state of these two files).
        #[semio_framework_async_macros::async_test]
        async fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_xlsx_snapshot();

            let parsed = <XlsxSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_xlsx_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_xlsx_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <XlsxSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_xlsx_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_xlsx_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
