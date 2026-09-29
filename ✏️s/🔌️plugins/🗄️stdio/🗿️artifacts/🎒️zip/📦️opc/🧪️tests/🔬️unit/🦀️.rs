use super::*;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlDoctype;

#[test]
fn compact_xml_preserves_epilog_and_checked_publication_rejects_invalid_doctype_position() {
    let root = XmlNode::Element { name: "root".into(), attrs: Vec::new(), children: Vec::new() };
    let doc = XmlDocument {
        prolog: vec![XmlNode::Comment { text: "before".into() }],
        root: Some(root.clone()),
        epilog: vec![XmlNode::ProcessingInstruction { target: "done".into(), data: "yes".into() }],
        ..Default::default()
    };
    assert_eq!(xml_document_to_opc_text_checked(&doc).unwrap(), "<!--before--><root/><?done yes?>");

    let invalid = XmlDocument { doctype: Some(XmlDoctype { prolog_position: 2, name: "root".into(), ..Default::default() }), ..doc };
    assert_eq!(xml_document_to_opc_text_checked(&invalid).unwrap_err(), "DOCTYPE prolog position 2 exceeds prolog length 1");
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_package() -> OpcPackage {
    let mut pkg = OpcPackage::empty();
    pkg.content_types.set_default("rels", RELS_CONTENT_TYPE);
    pkg.content_types.set_default("xml", "application/xml");
    pkg.set_part("word/document.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml", b"<w:document/>".to_vec());
    pkg.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "word/document.xml");
    pkg
}

fn publication_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🛡️publication/🔣️.json")).unwrap()
}

#[semio_framework_async_macros::async_test]
async fn publication_rejects_ambiguous_authored_metadata_before_writing_an_archive() {
    let fixture = publication_fixture();
    for case in fixture["invalidMetadata"].as_array().unwrap() {
        let mut package = sample_package();
        let identity = case["identity"].as_str().unwrap();
        let value = case["value"].as_str().unwrap();
        match case["kind"].as_str().unwrap() {
            "default" => package.content_types.defaults.push((identity.into(), value.into())),
            "override" => package.content_types.overrides.push((identity.into(), value.into())),
            "relationship" => package.add_relationship("", identity, REL_TYPE_OFFICE_DOCUMENT, value),
            "contentType" => package.parts.iter_mut().find(|part| part.path == identity).unwrap().content_type = value.into(),
            kind => panic!("unknown fixture kind {kind}"),
        }
        let before = package.clone();
        assert!(encode_opc(&package).is_err(), "case={case}");
        assert_eq!(package, before);
    }
    let package = sample_package();
    let bytes = encode_opc(&package).unwrap();
    assert_eq!(decode_opc(&bytes).unwrap(), package);
    let archive = zip::ZipArchive::new(std::io::Cursor::new(bytes)).unwrap();
    assert_eq!(archive.len(), fixture["paths"].as_array().unwrap().len());
}

#[semio_framework_async_macros::async_test]
async fn publication_rejects_ambiguous_part_paths_without_overwriting_payloads() {
    let fixture = publication_fixture();
    for path in fixture["conflictingPartPaths"].as_array().unwrap() {
        let mut package = sample_package();
        package.parts.push(OpcPart { path: path.as_str().unwrap().into(), content_type: "application/xml".into(), bytes: b"replacement".to_vec() });
        let before = package.clone();
        assert!(matches!(encode_opc(&package), Err(OpcError::Malformed(_))), "path={path}");
        assert_eq!(package, before);
    }
}

#[semio_framework_async_macros::async_test]
async fn publication_requires_a_complete_unique_path_permutation() {
    let fixture = publication_fixture();
    let package = sample_package();
    let before = package.clone();
    for order in fixture["invalidOrders"].as_array().unwrap() {
        let result = encode_opc_with_path_order(&package, |paths| *paths = order.as_array().unwrap().iter().map(|path| path.as_str().unwrap().into()).collect());
        assert!(matches!(result, Err(OpcError::Malformed(_))), "order={order}");
        assert_eq!(package, before);
    }
    let order: Vec<String> = fixture["validOrder"].as_array().unwrap().iter().map(|path| path.as_str().unwrap().into()).collect();
    let bytes = encode_opc_with_path_order(&package, |paths| *paths = order.clone()).unwrap();
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(&bytes)).unwrap();
    let actual: Vec<String> = (0..archive.len()).map(|index| archive.by_index(index).unwrap().name().into()).collect();
    assert_eq!(actual, order);
    assert_eq!(decode_opc(&bytes).unwrap(), package);
}

#[semio_framework_async_macros::async_test]
async fn publication_preserves_explicit_empty_relationship_parts() {
    let fixture = publication_fixture();
    let mut package = sample_package();
    package.relationships.insert(fixture["emptyRelationshipOwner"].as_str().unwrap().into(), Vec::new());
    let bytes = encode_opc(&package).unwrap();
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(&bytes)).unwrap();
    assert!(archive.by_name(fixture["emptyRelationshipPath"].as_str().unwrap()).is_ok());
    assert_eq!(decode_opc(&bytes).unwrap(), package);
}

#[semio_framework_async_macros::async_test]
async fn publication_rejects_duplicate_archive_member_names() {
    use std::io::Write;
    let fixture = publication_fixture();
    let bytes = encode_opc(&sample_package()).unwrap();
    let source = crate::standards::v2_0::subsets::base::io::decode_zip(&bytes).unwrap();
    for path in fixture["duplicateArchivePaths"].as_array().unwrap() {
        let path = path.as_str().unwrap();
        let placeholder = "X".repeat(path.len());
        let duplicate = source.entries.iter().find(|entry| entry.name == path).unwrap();
        let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
        let options: zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        for entry in &source.entries {
            writer.start_file(&entry.name, options).unwrap();
            writer.write_all(&entry.data).unwrap();
        }
        writer.start_file(&placeholder, options).unwrap();
        writer.write_all(&duplicate.data).unwrap();
        let mut bytes = writer.finish().unwrap().into_inner();
        let offsets = {
            let mut archive = zip::ZipArchive::new(std::io::Cursor::new(&bytes)).unwrap();
            let member = archive.by_name(&placeholder).unwrap();
            [member.header_start() as usize + 30, member.central_header_start() as usize + 46]
        };
        for offset in offsets {
            bytes[offset..offset + path.len()].copy_from_slice(path.as_bytes());
        }
        assert!(matches!(decode_opc(&bytes), Err(OpcError::Zip(detail)) if detail.contains("names must be nonempty and unique")), "path={path}");
    }
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
    let snap = ZipSnapshot { schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(), entries: vec![ZipEntry { name: "word/document.xml".into(), data: b"<x/>".to_vec(), ..Default::default() }], comment: String::new(), ..Default::default() };
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

    let plain_zip = crate::standards::v2_0::subsets::base::io::encode_zip(&ZipSnapshot {
        schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
        entries: vec![ZipEntry { name: "a.txt".into(), data: b"hi".to_vec(), ..Default::default() }],
        comment: String::new(),
        ..Default::default()
    })
    .unwrap();
    assert!(!sniff_opc_bytes(&plain_zip), "a plain zip with no [Content_Types].xml must not sniff as OPC");
}

#[semio_framework_async_macros::async_test]
async fn metadata_namespaces_and_identity_constraints_survive_independent_zip_reopen() {
    use std::io::Read;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🏷️metadata-namespaces/🔣️.json")).unwrap();
    let mut zip = crate::standards::v2_0::subsets::base::io::decode_zip(&encode_opc(&sample_package()).unwrap()).unwrap();
    zip.entries.iter_mut().find(|entry| entry.name == CONTENT_TYPES_PART).unwrap().data = fixture["contentTypes"].as_str().unwrap().as_bytes().to_vec();
    zip.entries.iter_mut().find(|entry| entry.name == "_rels/.rels").unwrap().data = fixture["relationships"].as_str().unwrap().as_bytes().to_vec();
    let bytes = crate::standards::v2_0::subsets::base::io::encode_zip(&zip).unwrap();
    let package = decode_opc(&bytes).unwrap();
    assert_eq!(package.content_types.defaults[0].0, fixture["expectedExtension"].as_str().unwrap());
    assert_eq!(package.content_types.resolve("word/document.xml"), fixture["expectedContentType"].as_str());
    assert_eq!(package.relationships_for("")[0].target, fixture["expectedPart"].as_str().unwrap());
    let reopened = encode_opc(&package).unwrap();
    let mut oracle = zip::ZipArchive::new(std::io::Cursor::new(&reopened)).unwrap();
    let mut metadata = String::new();
    oracle.by_name(CONTENT_TYPES_PART).unwrap().read_to_string(&mut metadata).unwrap();
    assert!(metadata.contains("Extension=\"XML\""));
    assert_eq!(decode_opc(&reopened).unwrap(), package);
    for (field, path) in [("invalidContentTypes", CONTENT_TYPES_PART), ("invalidRelationships", "_rels/.rels")] {
        for invalid in fixture[field].as_array().unwrap() {
            let mut invalid_zip = zip.clone();
            invalid_zip.entries.iter_mut().find(|entry| entry.name == path).unwrap().data = invalid.as_str().unwrap().as_bytes().to_vec();
            let bytes = crate::standards::v2_0::subsets::base::io::encode_zip(&invalid_zip).unwrap();
            assert!(decode_opc(&bytes).is_err(), "invalid metadata: {invalid}");
        }
    }
}

#[test]
fn relationship_ids_are_reserved_uniquely_within_each_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🆔️relationship-identity/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut taken: Vec<String> = serde_json::from_value(case["taken"].clone()).unwrap();
        let mut oracle: std::collections::BTreeSet<String> = taken.iter().cloned().collect();
        for expected in case["expected"].as_array().unwrap() {
            let id = fresh_relationship_id(&mut taken);
            assert_eq!(id, expected.as_str().unwrap());
            assert!(oracle.insert(id.clone()));
            assert_eq!(taken.last(), Some(&id));
        }
        let mut package = OpcPackage::empty();
        for id in case["taken"].as_array().unwrap() {
            package.add_relationship("", id.as_str().unwrap(), "urn:preserved", "document.xml");
        }
        let selected = package.add_generated_relationship("", "urn:main", "document.xml");
        assert_eq!(selected, case["expected"][0].as_str().unwrap());
        assert_eq!(package.add_generated_relationship("document.xml", "urn:styles", "styles.xml"), "rId1");
    }
}
