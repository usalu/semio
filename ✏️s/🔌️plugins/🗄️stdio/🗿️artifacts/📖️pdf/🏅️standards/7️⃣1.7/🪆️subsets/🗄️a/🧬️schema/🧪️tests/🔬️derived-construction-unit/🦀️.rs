mod tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn new_requires_output_intent_and_builds_clean() {
            let snapshot = PdfABuilderConstruction::new("sRGB IEC61966-2.1")
                
                .add_page(PdfPage::new(200.0, 200.0))
                
                .set_info(PdfInfo { title: Some("A Test".into()), ..PdfInfo::default() })
                
                .build()
                
                .expect("conforming construction must build");
            assert_eq!(snapshot.pages.len(), 1);
            assert_eq!(snapshot.info.title.as_deref(), Some("A Test"));
        }

        #[semio_framework_async_macros::async_test]
        async fn hard_violation_injected_via_snapshot_still_fails_build() {
            let violating = PdfIndirectObject { id: ObjRef { num: 99, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "S".into(), value: PdfObject::Name("Launch".into()) }]) };
            let mut snapshot = PdfABuilderConstruction::new("sRGB IEC61966-2.1").add_page(PdfPage::new(100.0, 100.0)).build().unwrap();
            snapshot.objects.push(violating);
            let mutated = PdfABuilderConstruction::from_snapshot(snapshot);
            let err = mutated.build().expect_err("a /Launch action must fail build()");
            assert!(err.iter().any(|d| d.code.0 == crate::standards::v1_7::subsets::a::schema::CODE_LAUNCH));
        }
    }
