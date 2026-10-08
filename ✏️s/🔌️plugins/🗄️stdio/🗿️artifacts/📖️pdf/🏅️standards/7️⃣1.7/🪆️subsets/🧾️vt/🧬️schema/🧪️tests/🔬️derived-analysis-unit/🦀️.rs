mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn conforming_x_snapshot() -> PdfSnapshot {
        use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfOutputIntent,PdfPage};
        let reference=|num|ObjRef{num,gen:0};
        let entry=|key:&str,value|PdfDictEntry::new(key,value);
        let object=|num,entries|PdfIndirectObject{id:reference(num),value:PdfObject::Dict(entries)};
        let profile=include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../🖼️assets/🌈️icc/🌈️sRGB2014.icc")).to_vec();
        let profile_reference=crate::standards::v1_7::subsets::base::io::foreign_artifacts::PdfArtifactResourcePort::admit(&mut crate::standards::v1_7::subsets::base::io::foreign_artifacts::NativePdfArtifactResources::default(),"s.stdio.icc",PdfObject::Stream{dict:vec![entry("N",PdfObject::Int(3))],data:profile.clone(),filters:Vec::new()}).unwrap();
        let mut page=PdfPage::new(100.0,100.0);
        page.trim_box=Some(page.media_box);
        PdfSnapshot{
            pages:vec![page],
            output_intents:vec![PdfOutputIntent{subtype:"GTS_PDFX".into(),condition_identifier:"sRGB2014".into(),condition:None,registry_name:None,info:None,profile:Some(profile_reference)}],
            catalog_extra:vec![entry("DPartRoot",PdfObject::Ref(reference(10)))],
            trailer:vec![entry("Root",PdfObject::Ref(reference(1)))],
            objects:vec![
                object(1,vec![entry("Type",PdfObject::name("Catalog")),entry("Pages",PdfObject::Ref(reference(4))),entry("OutputIntents",PdfObject::Array(vec![PdfObject::Ref(reference(2))])),entry("DPartRoot",PdfObject::Ref(reference(10)))]),
                object(2,vec![entry("Type",PdfObject::name("OutputIntent")),entry("S",PdfObject::name("GTS_PDFX")),entry("OutputConditionIdentifier",PdfObject::Str(b"sRGB2014".to_vec())),entry("DestOutputProfile",PdfObject::Ref(reference(9)))]),
                object(3,vec![entry("Type",PdfObject::name("Page")),entry("Parent",PdfObject::Ref(reference(4))),entry("MediaBox",PdfObject::numbers(&[0.0,0.0,100.0,100.0])),entry("TrimBox",PdfObject::numbers(&[0.0,0.0,100.0,100.0]))]),
                object(4,vec![entry("Type",PdfObject::name("Pages")),entry("Kids",PdfObject::Array(vec![PdfObject::Ref(reference(3))])),entry("Count",PdfObject::Int(1))]),
                PdfIndirectObject{id:reference(9),value:PdfObject::Stream{dict:vec![entry("N",PdfObject::Int(3))],data:profile,filters:Vec::new()}},
                object(10,vec![entry("Type",PdfObject::name("DPartRoot")),entry("DPartRootNode",PdfObject::Ref(reference(11)))]),
                object(11,vec![entry("Type",PdfObject::name("DPart")),entry("Parent",PdfObject::Ref(reference(10))),entry("Start",PdfObject::Ref(reference(3))),entry("DPM",PdfObject::Dict(Vec::new()))]),
            ],
            ..PdfSnapshot::default()
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn fully_conforming_vt_document_has_no_hard_diagnostics() {
        let snapshot = conforming_x_snapshot();
        let diagnostics = check_vt_conformance(&snapshot);
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_dpartroot_is_hard() {
        let mut snapshot = conforming_x_snapshot();
        let objects = &mut snapshot.objects;
        if let Some(catalog_obj) = objects.iter_mut().find(|o| o.id.num == 1) {
            if let PdfObject::Dict(d) = &mut catalog_obj.value {
                d.retain(|e| e.key != "DPartRoot");
            }
        }
        let diagnostics = check_vt_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DPART_ROOT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn x_violations_are_inherited_as_hard() {
        // No OutputIntent at all -- an X-4 violation must surface through vt too.
        let snapshot = PdfSnapshot::default();
        let diagnostics = check_vt_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == crate::standards::v1_7::subsets::x::io::CODE_OUTPUT_INTENT && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn dpart_missing_dpm_is_soft() {
        let mut snapshot = conforming_x_snapshot();
        let objects = &mut snapshot.objects;
        if let Some(dpart) = objects.iter_mut().find(|o| o.id.num == 11) {
            if let PdfObject::Dict(d) = &mut dpart.value {
                d.retain(|e| e.key != "DPM");
            }
        }
        let diagnostics = check_vt_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DPM && d.severity == Severity::Warning), "got {diagnostics:?}");
    }
}
