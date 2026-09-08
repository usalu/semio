
use super::*;

const FIXTURE: &str = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio.step','2026-08-10T00:00:00',('Ueli'),('semio'),'semio','','');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;\n#1=CARTESIAN_POINT('',(0.,0.,0.));\n#2=CARTESIAN_POINT('',(10.,0.,0.));\nENDSEC;\nEND-ISO-10303-21;\n";

#[semio_framework_async_macros::async_test]
async fn typed_snapshot_round_trips_through_part21_text() {
    let snapshot = <StepSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE).expect("parse");
    assert_eq!(snapshot.header.file_schema.schemas, vec!["AUTOMOTIVE_DESIGN".to_string()]);
    assert_eq!(snapshot.header.file_name.name, "semio.step");
    assert_eq!(snapshot.header.file_name.author, vec!["Ueli".to_string()]);
    assert_eq!(snapshot.entities.len(), 2);
    assert_eq!(snapshot.entities[0].id, 1);
    assert_eq!(snapshot.entities[0].name, "CARTESIAN_POINT");
    let text = store::ArtifactDsl::print_dsl(&snapshot);
    let reparsed = <StepSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("reparse");
    assert_eq!(snapshot, reparsed, "typed round trip must be lossless");
}

#[semio_framework_async_macros::async_test]
async fn typed_value_wrapper_round_trips() {
    let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n#1=IFCPROPERTYSINGLEVALUE('Height',$,IFCLENGTHMEASURE(3000.),$);\nENDSEC;\nEND-ISO-10303-21;\n";
    let snapshot = <StepSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("parse");
    let args = &snapshot.entities[0].args;
    match &args[2] {
        StepValue::TypedValue { type_name, value } => {
            assert_eq!(type_name, "IFCLENGTHMEASURE");
            assert_eq!(**value, StepValue::Real(3000.0));
        }
        other => panic!("expected TypedValue, got {other:?}"),
    }
    let reparsed = <StepSnapshot as store::ArtifactDsl>::parse_dsl(&store::ArtifactDsl::print_dsl(&snapshot)).unwrap();
    assert_eq!(snapshot, reparsed);
}

#[semio_framework_async_macros::async_test]
async fn complex_instance_keeps_every_type() {
    let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n#1=(IFCQUANTITYAREA($,$,$,10.5,$)IFCPHYSICALSIMPLEQUANTITY($,$,$,$));\nENDSEC;\nEND-ISO-10303-21;\n";
    let snapshot = <StepSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("parse");
    let entity = &snapshot.entities[0];
    assert_eq!(entity.name, "IFCQUANTITYAREA");
    assert_eq!(entity.complex.len(), 1);
    assert_eq!(entity.complex[0].name, "IFCPHYSICALSIMPLEQUANTITY");
    let reparsed = <StepSnapshot as store::ArtifactDsl>::parse_dsl(&store::ArtifactDsl::print_dsl(&snapshot)).unwrap();
    assert_eq!(snapshot, reparsed);
}

#[semio_framework_async_macros::async_test]
async fn pack_codec_round_trip() {
    let snapshot = <StepSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE).expect("parse");
    let bytes = store::ArtifactPack::encode_pack(&snapshot);
    let decoded = <StepSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snapshot);
}

/// 🧪️ `codec_retention_law`: decode -> encode is byte-preserving for both the DSL (`.step`
/// text) and pack codecs on the real fixture.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law_decode_encode_is_stable() {
    let snapshot = <StepSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE).expect("parse");
    let text_once = store::ArtifactDsl::print_dsl(&snapshot);
    let reparsed = <StepSnapshot as store::ArtifactDsl>::parse_dsl(&text_once).expect("reparse");
    let text_twice = store::ArtifactDsl::print_dsl(&reparsed);
    assert_eq!(text_once, text_twice, "print_dsl must be stable across a decode/encode cycle");
    assert_eq!(snapshot, reparsed);

    let bytes_once = store::ArtifactPack::encode_pack(&snapshot);
    let decoded = <StepSnapshot as store::ArtifactPack>::decode_pack(&bytes_once).expect("decode");
    let bytes_twice = store::ArtifactPack::encode_pack(&decoded);
    assert_eq!(bytes_once, bytes_twice, "encode_pack must be stable across a decode/encode cycle");
}
