use super::*;
#[test]
fn font_pair_rejects_malformed_borrowed_sources_without_effects() {
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for value in law["queries"]["accepted"].as_array().unwrap(){assert_eq!(serde_json::to_value(serde_json::from_value::<FontPairQuery>(value.clone()).unwrap()).unwrap(),*value);}
    for value in law["queries"]["refused"].as_array().unwrap(){assert!(serde_json::from_value::<FontPairQuery>(value.clone()).is_err());}
    for value in law["adjustments"]["accepted"].as_array().unwrap(){assert_eq!(serde_json::to_value(serde_json::from_value::<FontPairAdjustment>(value.clone()).unwrap()).unwrap(),*value);}
    for value in law["adjustments"]["refused"].as_array().unwrap(){assert!(serde_json::from_value::<FontPairAdjustment>(value.clone()).is_err());}
    for row in law["malformedSources"].as_array().unwrap(){
        let original:Vec<u8>=serde_json::from_value(row["bytes"].clone()).unwrap();let pointer=original.as_ptr();
        let(result,births,frees)=crate::wgpu::host::physical_job_close_tests::measured(||BorrowedFontPairSource::read(&original,*b"latn"));
        let error=result.err().unwrap_or_else(||panic!("original malformed font accepted: {}",row["name"]));
        assert_eq!(serde_json::to_value(error.kind).unwrap(),row["kind"]);assert!(matches!(error.message,std::borrow::Cow::Borrowed(_)));
        assert_eq!((births,frees),(0,0));assert_eq!(original.as_ptr(),pointer);
    }
    eprintln!("[DEBUG] font pair original typed malformedSources5 query/output schema vectors14 independentSerde=true denialBirth0 denialRelease0 borrowedRefusals=true originalPointers=true");
}
