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
pub fn xlsx_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.xlsx",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
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

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

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
    use crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx;
    use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDocument, XmlNode};
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
        document: XmlDocument { root: Some(XmlNode::Element { name: "styleSheet".into(), attrs: vec![crate::schema::vocabulary::attr("xmlns", crate::schema::vocabulary::SML_NS)], children: Vec::new() }), ..XmlDocument::default() },
    });
    snap.xml_parts.sort_by(|left, right| left.path.cmp(&right.path));
    snap
}
//#endregion 🔖️DocumentHelpers

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
