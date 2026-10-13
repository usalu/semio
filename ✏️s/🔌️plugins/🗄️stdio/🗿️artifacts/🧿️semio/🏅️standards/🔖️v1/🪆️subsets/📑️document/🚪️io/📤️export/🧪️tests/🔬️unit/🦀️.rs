//! 🔁️ Round trips of the `document` subset through each format: import, export, import again.

use crate::serializers::artifacts::docx::v_ecma_376::any::SemioDocumentToDocx;
use crate::serializers::artifacts::md::v_commonmark::any::SemioDocumentToMd;
use crate::serializers::artifacts::pdf::v1_7::any::SemioDocumentToPdf;
use crate::serializers::artifacts::txt::v_utf_8::any::SemioDocumentToTxt;
use semio_s_artifact_stdio_semio_document_import::deserializers::artifacts::docx::v_ecma_376::any::SemioDocumentFromDocx;
use semio_s_artifact_stdio_semio_document_import::deserializers::artifacts::md::v_commonmark::any::SemioDocumentFromMd;
use semio_s_artifact_stdio_semio_document_import::deserializers::artifacts::pdf::v1_7::any::SemioDocumentFromPdf;
use semio_s_artifact_stdio_semio_document_import::deserializers::artifacts::txt::v_utf_8::any::SemioDocumentFromTxt;
use semio_framework_plugin::{ArtifactDeserializer, ArtifactSerializer};

#[semio_framework_async_macros::async_test]
async fn docx_round_trip_is_stable() {
    use semio_s_artifact_stdio_docx::schema::snapshot::{DocxBlock, DocxDocument, DocxParagraph, DocxRun, DocxStyle};
    use semio_s_artifact_stdio_docx::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_docx;

    let docx1 = build_minimal_docx(
        DocxDocument {
            styles: vec![DocxStyle { id: "Heading1".into(), name: "Heading 1".into(), based_on: None }],
            body: vec![
                DocxBlock::Paragraph(DocxParagraph {
                    runs: vec![DocxRun { text: "Title".into(), bold: true, italic: false, underline: false, extra_run_properties: Vec::new() }],
                    style: Some("Heading1".into()),
                    extra_paragraph_properties: Vec::new(),
                }),
                DocxBlock::paragraph("Body."),
            ],
        },
    );
    let semio1 = ::semio_framework_async::poll::resolve_ready(SemioDocumentFromDocx::deserialize(&docx1)).expect("deserialize");
    let docx2 = ::semio_framework_async::poll::resolve_ready(SemioDocumentToDocx::serialize(&semio1)).expect("serialize");
    let semio2 = ::semio_framework_async::poll::resolve_ready(SemioDocumentFromDocx::deserialize(&docx2)).expect("deserialize round 2");
    assert_eq!(semio1, semio2);
}
#[semio_framework_async_macros::async_test]
async fn md_round_trip_is_stable() {
    use semio_s_artifact_stdio_md::MdSnapshot;
    use semio_s_artifact_stdio_md::schema::snapshot::{MdBlock, MdInline};

    let md1 = MdSnapshot {
        schema: semio_s_artifact_stdio_md::STDIO_MD_DOCUMENT_SCHEMA.into(),
        blocks: vec![
            MdBlock::Heading { level: 1, inlines: vec![MdInline::Strong { inlines: vec![MdInline::Text { text: "Title".into() }] }] },
            MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "Body".into() }] },
            MdBlock::CodeBlock { info: Some("rust".into()), literal: "fn main() {}".into() },
        ],
    };
    let semio1 = ::semio_framework_async::poll::resolve_ready(SemioDocumentFromMd::deserialize(&md1)).expect("deserialize");
    let md2 = ::semio_framework_async::poll::resolve_ready(SemioDocumentToMd::serialize(&semio1)).expect("serialize");
    let semio2 = ::semio_framework_async::poll::resolve_ready(SemioDocumentFromMd::deserialize(&md2)).expect("deserialize round 2");
    assert_eq!(semio1, semio2);
}
#[semio_framework_async_macros::async_test]
async fn txt_round_trip_is_stable() {
    use semio_s_artifact_stdio_txt::TxtSnapshot;
    use semio_s_artifact_stdio_txt::schema::snapshot::LineEnding;

    let txt1 = TxtSnapshot { schema: semio_s_artifact_stdio_txt::STDIO_TXT_DOCUMENT_SCHEMA.into(), lines: vec!["First line.".into(), String::new(), "Third line.".into()], trailing_newline: true, line_ending: LineEnding::Lf };
    let semio1 = ::semio_framework_async::poll::resolve_ready(SemioDocumentFromTxt::deserialize(&txt1)).expect("deserialize");
    let txt2 = ::semio_framework_async::poll::resolve_ready(SemioDocumentToTxt::serialize(&semio1)).expect("serialize");
    let semio2 = ::semio_framework_async::poll::resolve_ready(SemioDocumentFromTxt::deserialize(&txt2)).expect("deserialize round 2");
    assert_eq!(semio1, semio2);
}
#[semio_framework_async_macros::async_test]
async fn pdf_round_trip_is_stable() {
    let pdf1 = semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::io::export::text_document(&[(612.0, 792.0, "Page one text."), (612.0, 792.0, "Page two text.")]);
    let semio1 = ::semio_framework_async::poll::resolve_ready(SemioDocumentFromPdf::deserialize(&pdf1)).expect("deserialize");
    let pdf2 = ::semio_framework_async::poll::resolve_ready(SemioDocumentToPdf::serialize(&semio1)).expect("serialize");
    let semio2 = ::semio_framework_async::poll::resolve_ready(SemioDocumentFromPdf::deserialize(&pdf2)).expect("deserialize round 2");
    assert_eq!(semio1, semio2);
}
