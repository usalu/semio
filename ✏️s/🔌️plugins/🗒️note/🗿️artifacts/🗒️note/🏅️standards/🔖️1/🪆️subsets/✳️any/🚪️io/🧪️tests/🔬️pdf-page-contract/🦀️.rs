
use super::*;
use semio_framework::io::io_mechanism::{Deserializer, Serializer};
use semio_framework::io_schema::IoPayload as ForeignPayload;
use semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::{
    io::{decode_pdf, encode_pdf},
    schema::snapshot::PdfSnapshot,
};

#[semio_framework_async_macros::async_test]
async fn note_pdf14_page_contract_matches_the_json_oracle() {
    let fixture: Value = serde_json::from_str(include_str!("../../🧪️fixtures/📖️pdf14-pages.json")).expect("neutral PDF vectors");
    for row in fixture["cases"].as_array().expect("cases") {
        let pdf: PdfSnapshot = dsl::os_pack::from_json_str(&serde_json::json!({"schema": semio_s_artifact_stdio_pdf::STDIO_PDF_DOCUMENT_SCHEMA, "pages": row["pages"]}).to_string()).expect("owned PDF snapshot");
        let bytes = encode_pdf(&pdf).expect("PDF 1.4 writer");
        assert!(bytes.starts_with(b"%PDF-1.4"));
        let note = crate::io::import::deserializers::artifacts::pdf::v1_4::base::PdfIntoNote::deserialize(&ForeignPayload::Binary(bytes)).await.expect("PDF import").value;
        let NoteBlockNode::Text { width, height, content, .. } = &note.blocks[0] else { panic!("PDF page text block") };
        let text: String = crate::note_block_text(content).iter().flat_map(|paragraph| paragraph.runs.iter().map(|run| run.text.as_str())).collect();
        let imported = serde_json::json!({"width": width, "height": height, "text": text});
        assert_eq!(imported, row["expectedImport"]);
        let exported = crate::io::export::serializers::artifacts::pdf::v1_4::base::NoteIntoPdf::serialize(&note).await.expect("PDF export").value;
        let ForeignPayload::Binary(bytes) = exported else { panic!("binary PDF export") };
        assert!(bytes.starts_with(b"%PDF-1.4"));
        let pdf = decode_pdf(&bytes).expect("exported PDF decode");
        assert_eq!(pdf.pages.len(), 1);
        let actual: Value = serde_json::from_str(&dsl::os_pack::to_json_string(&pdf.pages[0])).expect("independent page JSON oracle");
        assert_eq!(actual["text"], row["expectedExport"]["text"]);
        assert_eq!(actual["width"].as_f64(), row["expectedExport"]["width"].as_f64());
        assert_eq!(actual["height"].as_f64(), row["expectedExport"]["height"].as_f64());
    }
}
