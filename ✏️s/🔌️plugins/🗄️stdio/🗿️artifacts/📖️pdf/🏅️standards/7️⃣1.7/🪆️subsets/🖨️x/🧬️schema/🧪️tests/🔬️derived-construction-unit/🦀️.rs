mod tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn new_requires_output_condition_and_builds_clean() {
            let snapshot =
                PdfXBuilderConstruction::new("FOGRA39").add_page(PdfPage::new(200.0, 200.0)).set_info(PdfInfo { title: Some("An X Test".into()), ..PdfInfo::default() }).build().expect("conforming construction must build");
            assert_eq!(snapshot.pages.len(), 1);
        }

        #[semio_framework_async_macros::async_test]
        async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
            let violating = PdfIndirectObject {
                id: ObjRef { num: 99, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Filter".into(), value: PdfObject::Name("Standard".into()) },
                    PdfDictEntry { key: "V".into(), value: PdfObject::Int(2) },
                    PdfDictEntry { key: "R".into(), value: PdfObject::Int(3) },
                    PdfDictEntry { key: "O".into(), value: PdfObject::Str(vec![0u8; 32]) },
                    PdfDictEntry { key: "U".into(), value: PdfObject::Str(vec![0u8; 32]) },
                ]),
            };
            let mut snapshot = PdfXBuilderConstruction::new("FOGRA39").add_page(PdfPage::new(100.0, 100.0)).build().unwrap();
            snapshot.objects.push(violating);
            let mutated = PdfXBuilderConstruction::from_snapshot(snapshot);
            let err = mutated.build().expect_err("an /Encrypt dict must fail build()");
            assert!(err.iter().any(|d| d.code.0 == crate::standards::v1_7::subsets::x::schema::CODE_ENCRYPT));
        }
    }
