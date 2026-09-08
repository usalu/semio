mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn new_requires_output_condition_and_builds_clean() {
        let snapshot = PdfVtBuilderConstruction::new("FOGRA39").add_page(PdfPage::new(200.0, 200.0)).set_info(PdfInfo { title: Some("A VT Test".into()), ..PdfInfo::default() }).build().expect("conforming construction must build");
        assert_eq!(snapshot.pages.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let mut snapshot = PdfVtBuilderConstruction::new("FOGRA39").add_page(PdfPage::new(100.0, 100.0)).build().unwrap();
        if let Some(catalog_obj) = snapshot.objects.iter_mut().find(|o| o.id.num == 1) {
            if let PdfObject::Dict(d) = &mut catalog_obj.value {
                d.retain(|e| e.key != "DPartRoot");
            }
        }
        let mutated = PdfVtBuilderConstruction::from_snapshot(snapshot);
        let err = mutated.build().expect_err("a Catalog missing /DPartRoot must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::standards::v1_7::subsets::vt::schema::CODE_DPART_ROOT));
    }
}
