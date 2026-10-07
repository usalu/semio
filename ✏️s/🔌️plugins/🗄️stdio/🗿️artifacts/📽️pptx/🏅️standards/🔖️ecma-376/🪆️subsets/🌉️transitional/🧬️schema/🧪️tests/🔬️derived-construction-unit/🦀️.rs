mod tests {
    use super::*;
    use semio_s_artifact_stdio_zip::opc::{OpcPackage, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};

    const TRANSITIONAL_PRESENTATION_XML: &str = concat!(
        r#"<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">"#,
        r#"<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst>"#,
        r#"<p:sldIdLst/>"#,
        "</p:presentation>",
    );

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn transitional_snapshot() -> PptxSnapshot {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", TRANSITIONAL_PRESENTATION_XML.as_bytes().to_vec());
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "ppt/presentation.xml");
        crate::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_pptx(&semio_s_artifact_stdio_zip::opc::encode_opc(&opc).expect("valid OPC package")).expect("valid presentation")
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_builder_has_no_office_document_relationship_and_fails_build() {
        let err = PptxTransitionalBuilderConstruction::empty().build().expect_err("an empty package has no officeDocument relationship, must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::standards::v_ecma_376::subsets::transitional::schema::conformance::CODE_MAIN_NS));
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_transitional_snapshot_builds_clean() {
        let snapshot = PptxTransitionalBuilderConstruction::from_snapshot(transitional_snapshot()).build().expect("conforming Transitional snapshot must build");
        assert!(snapshot.xml_parts.iter().any(|part| part.path == "ppt/presentation.xml"));
    }

    #[semio_framework_async_macros::async_test]
    async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let mut violating = transitional_snapshot();
        violating.xml_parts.push(crate::schema::snapshot::PptxXmlPart {
            path: "ppt/slides/slide1.xml".into(),
            content_type: "application/vnd.openxmlformats-officedocument.presentationml.slide+xml".into(),
            document: semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_from_text(r#"<p:sld xmlns:p="http://purl.oclc.org/ooxml/presentationml/main"/>"#).expect("valid XML"),
        });
        let (mutated, _diff) = PptxTransitionalBuilderConstruction::from_snapshot(PptxSnapshot::default()).mutate(PptxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: violating }));
        let err = mutated.build().expect_err("a Strict namespace anywhere must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::standards::v_ecma_376::subsets::transitional::schema::conformance::CODE_STRICT_NS_PRESENT));
    }
}
