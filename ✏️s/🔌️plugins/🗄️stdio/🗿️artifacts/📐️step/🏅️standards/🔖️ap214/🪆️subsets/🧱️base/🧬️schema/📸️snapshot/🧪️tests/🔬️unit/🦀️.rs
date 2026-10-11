use super::*;

const FIXTURE: &str = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio.step','2026-08-10T00:00:00',('Ueli'),('semio'),'semio','','');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;\n#1=CARTESIAN_POINT('',(0.,0.,0.));\n#2=CARTESIAN_POINT('',(10.,0.,0.));\nENDSEC;\nEND-ISO-10303-21;\n";

#[semio_framework_async_macros::async_test]
async fn typed_snapshot_round_trips_through_native_text_and_external_part21() {
    let snapshot = StepSnapshot::from_part21_document(&semio_s_artifact_stdio_contract::part21::parse_part21(FIXTURE).expect("external Part21 fixture"));
    assert_eq!(snapshot.header.file_schema.schemas, vec!["AUTOMOTIVE_DESIGN".to_string()]);
    assert_eq!(snapshot.header.file_name.name, "semio.step");
    assert_eq!(snapshot.header.file_name.author, vec!["Ueli".to_string()]);
    assert_eq!(snapshot.entities.len(), 2);
    assert_eq!(snapshot.entities[0].id, 1);
    assert_eq!(snapshot.entities[0].name, "CARTESIAN_POINT");
    let text = store::ArtifactDsl::print_dsl(&snapshot);
    let reparsed = <StepSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("reparse");
    assert_eq!(snapshot, reparsed, "typed round trip must be lossless");
    let exchange=semio_s_artifact_stdio_contract::part21::write_part21(&snapshot.to_part21_document());
    assert_eq!(StepSnapshot::from_part21_document(&semio_s_artifact_stdio_contract::part21::parse_part21(&exchange).unwrap()),snapshot);
}

#[semio_framework_async_macros::async_test]
async fn typed_value_wrapper_round_trips() {
    let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n#1=IFCPROPERTYSINGLEVALUE('Height',$,IFCLENGTHMEASURE(3000.),$);\nENDSEC;\nEND-ISO-10303-21;\n";
    let snapshot = StepSnapshot::from_part21_document(&semio_s_artifact_stdio_contract::part21::parse_part21(text).expect("external Part21 fixture"));
    let args = &snapshot.entities[0].args;
    match &args[2] {
        StepValue::TypedValue(StepTypedValue { type_name, value }) => {
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
    let snapshot = StepSnapshot::from_part21_document(&semio_s_artifact_stdio_contract::part21::parse_part21(text).expect("external Part21 fixture"));
    let entity = &snapshot.entities[0];
    assert_eq!(entity.name, "IFCQUANTITYAREA");
    assert_eq!(entity.complex.len(), 1);
    assert_eq!(entity.complex[0].name, "IFCPHYSICALSIMPLEQUANTITY");
    let reparsed = <StepSnapshot as store::ArtifactDsl>::parse_dsl(&store::ArtifactDsl::print_dsl(&snapshot)).unwrap();
    assert_eq!(snapshot, reparsed);
}

#[semio_framework_async_macros::async_test]
async fn pack_codec_round_trip() {
    let snapshot = StepSnapshot::from_part21_document(&semio_s_artifact_stdio_contract::part21::parse_part21(FIXTURE).expect("external Part21 fixture"));
    let bytes = store::ArtifactPack::encode_pack(&snapshot);
    let decoded = <StepSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snapshot);
}

/// 🧪️ `codec_retention_law`: decode -> encode is byte-preserving for both the DSL (`.step`
/// text) and pack codecs on the real fixture.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law_decode_encode_is_stable() {
    let snapshot = StepSnapshot::from_part21_document(&semio_s_artifact_stdio_contract::part21::parse_part21(FIXTURE).expect("external Part21 fixture"));
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

/// 🧬️ The newtype payload record spells the typed value on the wire exactly as the former named variant did.
#[test]
fn typed_value_payload_record_keeps_the_former_named_variant_wire() {
    use semio_framework_value::ToValue;
    #[derive(value_derive::ToValue)]
    #[value(rename_all = "camelCase", rename_all_fields = "camelCase")]
    enum Former {
        Unset,
        Real(f64),
        TypedValue { type_name: String, value: Box<Former> },
    }
    let current = StepValue::TypedValue(StepTypedValue { type_name: "LENGTH_MEASURE".into(), value: Box::new(StepValue::Real(3000.0)) });
    let former = Former::TypedValue { type_name: "LENGTH_MEASURE".into(), value: Box::new(Former::Real(3000.0)) };
    assert_eq!(current.to_value(), former.to_value());
    assert_eq!(StepValue::Unset.to_value(), Former::Unset.to_value());
}
