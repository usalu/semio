//! 📄️ Real variant editor codecs preserve authored WordprocessingML and independently observed ZIP content.
use semio_plugin::{ArtifactBuilder, ArtifactEditor};
use semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::{DocxParagraph, DocxSnapshot};
use semio_s_artifact_stdio_docx_strict::standards::v_ecma_376::subsets::strict::io::DocxStrictBuilderConstruction;
use semio_s_artifact_stdio_docx_transitional::standards::v_ecma_376::subsets::transitional::io::DocxTransitionalBuilderConstruction;
use semio_s_artifact_stdio_docx_strict::editor::docx::standards::v_ecma_376::subsets::strict::DocxStrictEditor;
use semio_s_artifact_stdio_docx_transitional::editor::docx::standards::v_ecma_376::subsets::transitional::DocxTransitionalEditor;
use serde_json::Value;
use std::io::{Cursor, Read};

fn member(bytes: &[u8], name: &str) -> String {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).expect("independent ZIP reader accepts the actual editor output");
    let mut text = String::new();
    archive.by_name(name).unwrap().read_to_string(&mut text).unwrap();
    text
}

fn verify_package(bytes: &[u8], row: &Value) {
    use quick_xml::events::Event;
    let document = member(bytes, "word/document.xml");
    let mut reader = quick_xml::Reader::from_str(&document);
    let mut namespace = None;
    let mut texts = Vec::new();
    loop {
        match reader.read_event().unwrap() {
            Event::Start(element) if element.local_name().as_ref() == b"document" => {
                let qualified = element.name();
                let name = std::str::from_utf8(qualified.as_ref()).unwrap();
                let key = name.split_once(':').map_or_else(|| "xmlns".to_owned(), |(prefix, _)| format!("xmlns:{prefix}"));
                namespace = element.attributes().map(Result::unwrap).find(|attribute| attribute.key.as_ref() == key.as_bytes()).map(|attribute| String::from_utf8(attribute.value.into_owned()).unwrap());
            }
            Event::Text(text) => {
                let decoded = text.decode().unwrap();
                if !decoded.trim().is_empty() { texts.push(quick_xml::escape::unescape(&decoded).unwrap().into_owned()); }
            }
            Event::Eof => break,
            _ => {}
        }
    }
    assert_eq!(namespace.as_deref(), row["mainNamespace"].as_str());
    assert_eq!(texts, vec![row["paragraphs"]["en"].as_str().unwrap(), row["paragraphs"]["de"].as_str().unwrap()]);
    let relationships = member(bytes, "_rels/.rels");
    let mut reader = quick_xml::Reader::from_str(&relationships);
    let expected = format!("{}/officeDocument", row["relationshipBase"].as_str().unwrap());
    let mut office_document = false;
    loop {
        match reader.read_event().unwrap() {
            Event::Start(element) | Event::Empty(element) if element.local_name().as_ref() == b"Relationship" => {
                office_document |= element.attributes().map(Result::unwrap).any(|attribute| attribute.key.as_ref() == b"Type" && attribute.value.as_ref() == expected.as_bytes());
            }
            Event::Eof => break,
            _ => {}
        }
    }
    assert!(office_document, "the emitted package retains its variant relationship authority");
}

fn check<E: ArtifactEditor<Snapshot = DocxSnapshot>>(input: DocxSnapshot, row: &Value) {
    let codec = E::natural_file_codec().expect("actual variant declares its native codec");
    assert_eq!(E::DOCUMENT_SCHEMA, row["schema"].as_str().unwrap());
    assert_eq!(codec.format_kind, row["formatKind"].as_str().unwrap());
    assert_eq!(codec.extension, row["extension"].as_str().unwrap());
    assert_eq!(codec.media_type, row["mediaType"].as_str().unwrap());
    assert_eq!(codec.binary, row["binary"].as_bool().unwrap());
    let oracle: Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&input)).unwrap();
    let bytes = E::encode_natural_file(&input).expect("actual variant encodes its conformant authored snapshot");
    verify_package(&bytes, row);
    let restored = E::decode_natural_file(&bytes).expect("actual variant reads its own native document");
    assert_eq!(restored, input);
    let observed: Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&restored)).unwrap();
    assert_eq!(observed, oracle);
    verify_package(&E::encode_natural_file(&restored).unwrap(), row);
    assert!(E::decode_natural_file(b"invalid ZIP").is_err());
    assert!(E::decode_natural_file(&bytes[..bytes.len() / 2]).is_err());
}

#[test]
fn strict_and_transitional_editor_sdk_preserve_populated_native_packages() {
    let fixture: Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["rows"].as_array().unwrap() {
        let english = DocxParagraph::text(row["paragraphs"]["en"].as_str().unwrap().into());
        let german = DocxParagraph::text(row["paragraphs"]["de"].as_str().unwrap().into());
        match row["variant"].as_str().unwrap() {
            "strict" => check::<DocxStrictEditor>(DocxStrictBuilderConstruction::empty().add_paragraph(english).add_paragraph(german).build().expect("strict builder validates its populated native input"), row),
            "transitional" => check::<DocxTransitionalEditor>(DocxTransitionalBuilderConstruction::empty().add_paragraph(english).add_paragraph(german).build().expect("transitional builder validates its populated native input"), row),
            other => panic!("unknown DOCX variant {other}"),
        }
    }
}
