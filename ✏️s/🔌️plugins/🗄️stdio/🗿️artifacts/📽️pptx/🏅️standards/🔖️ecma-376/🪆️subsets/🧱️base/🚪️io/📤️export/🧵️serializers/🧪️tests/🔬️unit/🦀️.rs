use super::*;
use quick_xml::{events::Event, reader::Reader, XmlVersion};
use semio_s_artifact_stdio_xml::schema::snapshot::xml_document_from_text;
use std::io::{Cursor, Read};

fn literal_export_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../../🧫️fixtures/🔤️literal-xml-export/🔣️.json")).unwrap()
}

fn literal_export_snapshot(fixture: &serde_json::Value) -> PptxSnapshot {
    let mut snapshot = build_minimal_pptx(PptxPresentation::default());
    for case in fixture["cases"].as_array().unwrap() {
        snapshot.xml_parts.push(crate::schema::snapshot::PptxXmlPart {
            path: case["path"].as_str().unwrap().into(),
            content_type: case["contentType"].as_str().unwrap().into(),
            document: xml_document_from_text(case["xml"].as_str().unwrap()).unwrap(),
        });
    }
    snapshot
}

#[test]
fn literal_xml_export_preserves_attributes_with_independent_zip_and_xml_readers() {
    let fixture = literal_export_fixture();
    let snapshot = literal_export_snapshot(&fixture);
    let bytes = encode_pptx(&snapshot).unwrap();
    let mut archive = zip::ZipArchive::new(Cursor::new(&bytes)).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let path = case["path"].as_str().unwrap();
        let mut text = String::new();
        archive.by_name(path).unwrap().read_to_string(&mut text).unwrap();
        let mut reader = Reader::from_str(&text);
        let mut values = Vec::new();
        loop {
            match reader.read_event().unwrap_or_else(|error| panic!("{}: {error}: {text}", case["id"])) {
                Event::Start(node) | Event::Empty(node) => {
                    for attribute in node.attributes() {
                        let attribute = attribute.unwrap_or_else(|error| panic!("{}: {error}: {text}", case["id"]));
                        if attribute.key.as_ref() == case["attribute"].as_str().unwrap() {
                            values.push(attribute.normalized_value(XmlVersion::Explicit1_0).unwrap().into_owned());
                        }
                    }
                }
                Event::Eof => break,
                _ => {}
            }
        }
        assert_eq!(values, vec![case["value"].as_str().unwrap()], "{}", case["id"]);
        println!("[DEBUG] PPTX literal XML {} preserved through independent ZIP/XML readers", case["id"]);
    }
    let reopened = crate::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_pptx(&bytes).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let path = case["path"].as_str().unwrap();
        assert_eq!(reopened.xml_parts.iter().find(|part| part.path == path).unwrap().document.root, snapshot.xml_parts.iter().find(|part| part.path == path).unwrap().document.root);
    }
}

#[test]
fn literal_xml_export_preserves_authored_content_order() {
    let fixture = literal_export_fixture();
    let snapshot = literal_export_snapshot(&fixture);
    let expected: Vec<&str> = fixture["cases"].as_array().unwrap().iter().map(|case| case["path"].as_str().unwrap()).collect();
    let bytes = encode_pptx(&snapshot).unwrap();
    let archive = zip::ZipArchive::new(Cursor::new(&bytes)).unwrap();
    let actual: Vec<&str> = archive.file_names().filter(|name| expected.contains(name)).collect();
    assert_eq!(actual, expected);
    println!("[DEBUG] PPTX ZIP members follow authored part order for custom paths");
}
