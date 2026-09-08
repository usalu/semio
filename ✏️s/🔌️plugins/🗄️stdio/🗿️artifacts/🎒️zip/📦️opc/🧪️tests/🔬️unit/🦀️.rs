
use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_package() -> OpcPackage {
    let mut pkg = OpcPackage::empty();
    pkg.content_types.set_default("rels", RELS_CONTENT_TYPE);
    pkg.content_types.set_default("xml", "application/xml");
    pkg.set_part("word/document.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml", b"<w:document/>".to_vec());
    pkg.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "word/document.xml");
    pkg
}

#[semio_framework_async_macros::async_test]
async fn round_trip_preserves_parts_and_relationships() {
    let pkg = sample_package();
    let bytes = encode_opc(&pkg).expect("encode");
    let decoded = decode_opc(&bytes).expect("decode");
    assert_eq!(decoded.part_bytes("word/document.xml"), Some(b"<w:document/>".as_slice()));
    assert_eq!(decoded.content_types.resolve("word/document.xml"), Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"));
    let root_rels = decoded.relationships_for("");
    assert_eq!(root_rels.len(), 1);
    assert_eq!(root_rels[0].target, "word/document.xml");
    assert_eq!(root_rels[0].rel_type, REL_TYPE_OFFICE_DOCUMENT);
}

#[semio_framework_async_macros::async_test]
async fn resolve_relationship_target_is_relative_to_owner_directory() {
    // A relationship owned by "word/document.xml" (rels file at
    // "word/_rels/document.xml.rels") targeting "media/image1.png" resolves against
    // "word/", not the package root — the #1 OPC relative-target gotcha.
    assert_eq!(resolve_relationship_target("word/document.xml", "media/image1.png"), "word/media/image1.png");
    assert_eq!(resolve_relationship_target("word/document.xml", "/media/image1.png"), "media/image1.png");
    assert_eq!(resolve_relationship_target("", "word/document.xml"), "word/document.xml");
}

#[semio_framework_async_macros::async_test]
async fn owner_and_rels_path_round_trip_including_root() {
    assert_eq!(rels_part_path_for(""), "_rels/.rels");
    assert_eq!(owner_for_rels_path("_rels/.rels"), Some(String::new()));
    assert_eq!(rels_part_path_for("word/document.xml"), "word/_rels/document.xml.rels");
    assert_eq!(owner_for_rels_path("word/_rels/document.xml.rels"), Some("word/document.xml".to_string()));
    assert_eq!(rels_part_path_for("xl/workbook.xml"), "xl/_rels/workbook.xml.rels");
    assert_eq!(owner_for_rels_path("xl/_rels/workbook.xml.rels"), Some("xl/workbook.xml".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn content_types_override_wins_over_default() {
    let mut ct = OpcContentTypes::default();
    ct.set_default("xml", "application/xml");
    ct.set_override("word/document.xml", "application/vnd.custom+xml");
    assert_eq!(ct.resolve("word/document.xml"), Some("application/vnd.custom+xml"));
    assert_eq!(ct.resolve("word/styles.xml"), Some("application/xml"));
    assert_eq!(ct.resolve("word/unknownext.bin"), None);
}

#[semio_framework_async_macros::async_test]
async fn decode_rejects_missing_content_types() {
    let snap = ZipSnapshot { schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(), entries: vec![ZipEntry { name: "word/document.xml".into(), data: b"<x/>".to_vec(), ..Default::default() }], comment: String::new() };
    let bytes = crate::standards::v2_0::subsets::base::io::encode_zip(&snap).unwrap();
    let err = decode_opc(&bytes).expect_err("must reject a zip with no [Content_Types].xml");
    assert_eq!(err, OpcError::MissingContentTypes);
}

#[semio_framework_async_macros::async_test]
async fn sniff_recognizes_content_types_entry() {
    let pkg = sample_package();
    let bytes = encode_opc(&pkg).unwrap();
    assert!(sniff_opc_bytes(&bytes));
    assert!(!sniff_opc_bytes(b"not a zip"));

    let plain_zip =
        crate::standards::v2_0::subsets::base::io::encode_zip(&ZipSnapshot { schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(), entries: vec![ZipEntry { name: "a.txt".into(), data: b"hi".to_vec(), ..Default::default() }], comment: String::new() }).unwrap();
    assert!(!sniff_opc_bytes(&plain_zip), "a plain zip with no [Content_Types].xml must not sniff as OPC");
}
