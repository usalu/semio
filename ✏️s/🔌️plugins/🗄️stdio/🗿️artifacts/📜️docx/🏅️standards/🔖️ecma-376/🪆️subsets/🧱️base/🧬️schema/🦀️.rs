//! 🧬️ DocxArtifact schema — full artifact state.

use crate::schema::snapshot::{DocxDocument, DocxXmlParts};
use crate::DocxSnapshot;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_zip::opc::retained::RetainedOpcPackage;

//#region Artifact
/// 🧬️ Full `stdio.docx` artifact state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.docx")]
pub struct DocxArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub opc: RetainedOpcPackage,
    #[state(artifact)]
    #[value(default)]
    pub xml_parts: DocxXmlParts,
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
    pub async fn to_snapshot(&self) -> DocxSnapshot {
        DocxSnapshot { schema: self.schema.clone(), opc: self.opc.clone(), xml_parts: self.xml_parts.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: DocxSnapshot) -> Self {
        Self { schema: snapshot.schema, opc: snapshot.opc, xml_parts: snapshot.xml_parts }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub async fn set_snapshot(&mut self, snapshot: DocxSnapshot) {
        self.schema = snapshot.schema;
        self.opc = snapshot.opc;
        self.xml_parts = snapshot.xml_parts;
    }
}
//#endregion Conversions

//#region Descriptor
/// 🧬️ Descriptor for `s.stdio.docx`.
pub fn docx_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.docx",
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
pub async fn empty_docx_snapshot() -> DocxSnapshot {
    DocxSnapshot::default()
}

/// 📄️ FG-wave: the demo `stdio.docx` document — a genuinely non-trivial `DocxSnapshot` exercising
/// a styled heading paragraph, a mixed-formatting run (bold/italic/plain), a 2x2 table (recursing
/// through `Table -> row -> cell -> Paragraph`), two named styles (one `based_on` the other), and
/// one unmodeled raw OPC part (`word/numbering.xml`, verbatim-retained). The single source of
/// truth for `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` (both are
/// literally this snapshot's `print_dsl`/`encode_pack` output, asserted equal by
/// `fixture_honesty_law` below) — same shape `📷️png/…/⚙️engine/🦀️.rs`'s own
/// `demo_png_snapshot()` establishes.
pub async fn demo_docx_snapshot() -> DocxSnapshot {
    use crate::schema::snapshot::{DocxBlock, DocxParagraph, DocxRun, DocxStyle, DocxTable, DocxTableCell, DocxTableRow};
    let document = DocxDocument {
        body: vec![
            DocxBlock::Paragraph(DocxParagraph { style: Some("Heading1".into()), ..DocxParagraph::text("Semio Demo") }),
            DocxBlock::Paragraph(DocxParagraph {
                runs: vec![DocxRun { text: "Bold and ".into(), bold: true, ..Default::default() }, DocxRun { text: "italic".into(), italic: true, ..Default::default() }, DocxRun { text: " text".into(), ..Default::default() }],
                style: None,
                extra_paragraph_properties: Vec::new(),
            }),
            DocxBlock::Table(DocxTable {
                rows: vec![
                    DocxTableRow { cells: vec![DocxTableCell { blocks: vec![DocxBlock::paragraph("R1C1")], ..Default::default() }, DocxTableCell { blocks: vec![DocxBlock::paragraph("R1C2")], ..Default::default() }], ..Default::default() },
                    DocxTableRow { cells: vec![DocxTableCell { blocks: vec![DocxBlock::paragraph("R2C1")], ..Default::default() }, DocxTableCell { blocks: vec![DocxBlock::paragraph("R2C2")], ..Default::default() }], ..Default::default() },
                ],
                ..Default::default()
            }),
        ],
        styles: vec![DocxStyle { id: "Normal".into(), name: "Normal".into(), based_on: None }, DocxStyle { id: "Heading1".into(), name: "heading 1".into(), based_on: Some("Normal".into()) }],
    };
    let mut snap = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_docx(document);
    snap.xml_parts
        .try_push(
            crate::schema::snapshot::DocxXmlPart::try_from_document(
                "word/numbering.xml".into(),
                "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml".into(),
                semio_s_artifact_stdio_xml::schema::snapshot::xml_document_from_text("<w:numbering/>").expect("valid demo numbering XML"),
            )
            .expect("valid retained demo numbering XML"),
        )
        .expect("demo XML part owner has capacity");
    snap.opc
        .content_types
        .set_override("word/numbering.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml")
        .expect("the demo content-type override fits retained OPC ownership");
    snap.xml_parts.sort_unstable_by(|left, right| left.path.cmp(&right.path));
    // 🔤️ Path-ascending is the package NORMAL FORM a decode hands back
    // (`semio_s_artifact_stdio_zip`'s decoder canonicalizes member order), so the demo has to be
    // stated in it: `📜️example.docx` IS `encode_docx(this)`, and `demo_subset_integrated_roundtrip`
    // re-encodes what a decode of that file yields and demands byte-identical output. Appending
    // `word/numbering.xml` after `build_minimal_docx`'s own parts left the demo one step off that
    // normal form, so every content byte round-tripped and the member SEQUENCE did not.
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
