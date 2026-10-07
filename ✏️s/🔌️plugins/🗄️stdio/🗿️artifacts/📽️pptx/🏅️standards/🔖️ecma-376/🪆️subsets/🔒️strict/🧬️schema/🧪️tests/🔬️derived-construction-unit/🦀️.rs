mod tests {
    use super::*;
    use semio_s_artifact_stdio_zip::opc::{OpcPackage, RELS_CONTENT_TYPE};

    const STRICT_PRESENTATION_XML: &str = concat!(
        r#"<p:presentation xmlns:a="http://purl.oclc.org/ooxml/drawingml/main" xmlns:p="http://purl.oclc.org/ooxml/presentationml/main" xmlns:r="http://purl.oclc.org/ooxml/officeDocument/relationships" conformance="strict">"#,
        r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
        r#"<p:sldIdLst/>"#,
        "</p:presentation>",
    );

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn strict_snapshot() -> PptxSnapshot {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", STRICT_PRESENTATION_XML.as_bytes().to_vec());
        opc.add_relationship("", "rId1", "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument", "ppt/presentation.xml");
        crate::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_pptx(&semio_s_artifact_stdio_zip::opc::encode_opc(&opc).expect("valid OPC package")).expect("valid presentation")
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_builder_has_no_office_document_relationship_and_fails_build() {
        let err = PptxStrictBuilderConstruction::empty().build().expect_err("an empty package has no officeDocument relationship, must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::standards::v_ecma_376::subsets::strict::schema::conformance::CODE_MAIN_NS));
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_strict_snapshot_builds_clean() {
        let snapshot = PptxStrictBuilderConstruction::from_snapshot(strict_snapshot()).build().expect("conforming Strict snapshot must build");
        assert!(snapshot.xml_parts.iter().any(|part| part.path == "ppt/presentation.xml"));
    }

    #[semio_framework_async_macros::async_test]
    async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let mut violating = strict_snapshot();
        violating.xml_parts.push(crate::schema::snapshot::PptxXmlPart {
            path: "ppt/slides/slide1.xml".into(),
            content_type: "application/vnd.openxmlformats-officedocument.presentationml.slide+xml".into(),
            document: semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_from_text(r#"<v:shape xmlns:v="urn:schemas-microsoft-com:vml"/>"#).expect("valid XML"),
        });
        let (mutated, _diff) = PptxStrictBuilderConstruction::from_snapshot(PptxSnapshot::default()).mutate(PptxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: violating }));
        let err = mutated.build().expect_err("VML markup must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::standards::v_ecma_376::subsets::strict::schema::conformance::CODE_VML_PRESENT));
    }
}
