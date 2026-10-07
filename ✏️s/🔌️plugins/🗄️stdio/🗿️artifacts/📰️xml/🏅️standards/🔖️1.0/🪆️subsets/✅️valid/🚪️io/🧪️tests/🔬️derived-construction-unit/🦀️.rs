mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn conforming_snapshot_builds_clean() {
        let snapshot = XmlValidBuilderConstruction::from_text("<!DOCTYPE root>\n<root/>").expect("parses").build().expect("conforming construction must build");
        assert_eq!(snapshot.doc.doctype.as_ref().map(|doctype| doctype.name.as_str()), Some("root"));
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_doctype_fails_build() {
        let err = XmlValidBuilderConstruction::from_text("<root/>").expect("parses").build().expect_err("a document without a doctype must fail build()");
        assert!(err.iter().any(|d| d.code.0 == "stdio.xml.valid.doctype-missing"));
    }

    #[semio_framework_async_macros::async_test]
    async fn root_name_mismatch_injected_around_the_vocabulary_still_fails_build() {
        let built = XmlValidBuilderConstruction::from_text("<!DOCTYPE root>\n<root/>").expect("parses").build().expect("clean build");
        let mut mismatched = built;
        mismatched.doc.doctype = Some("<!DOCTYPE somethingElse>".into());
        let err = XmlValidBuilderConstruction::from_snapshot(mismatched).build().expect_err("a doctype/root name mismatch must fail build()");
        assert!(err.iter().any(|d| d.code.0 == "stdio.xml.valid.root-name-mismatch"));
    }
}
