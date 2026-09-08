mod tests {
        use super::*;
        use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject};

        #[semio_framework_async_macros::async_test]
        async fn empty_builder_builds_clean() {
            let snapshot = PdfEBuilderConstruction::new().add_page(PdfPage::new(200.0, 200.0)).set_info(PdfInfo { title: Some("An E Test".into()), ..PdfInfo::default() }).build().expect("no hard violations by default");
            assert_eq!(snapshot.pages.len(), 1);
        }

        #[semio_framework_async_macros::async_test]
        async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
            let violating = PdfIndirectObject { id: ObjRef { num: 99, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Subtype".into(), value: PdfObject::Name("Movie".into()) }]) };
            let mut snapshot = PdfEBuilderConstruction::new().add_page(PdfPage::new(100.0, 100.0)).build().unwrap();
            snapshot.objects.push(violating);
            let mutated = PdfEBuilderConstruction::from_snapshot(snapshot);
            let err = mutated.build().expect_err("a Movie annotation must fail build()");
            assert!(err.iter().any(|d| d.code.0 == crate::standards::v1_7::subsets::e::schema::CODE_MOVIE_OR_SOUND));
        }
    }
