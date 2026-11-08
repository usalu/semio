//! 🔗️ Neutral durable link identity and canonical controlled record laws.
use semio_os_store_link_schema::{ArtifactLink,LinkPin};
use semio_framework_value::{FromValue, ToValue, NativeEncodeControl};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_dsl_record::{DslField,FieldValue,Shape,NativeSchemaControl};

#[derive(serde::Serialize,serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
struct Reference {artifact_id:String,dialect:Dialect}
#[derive(serde::Serialize,serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
struct Dialect {artifact_kind:String,standard:String,subset:String}
#[derive(serde::Serialize,serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
struct Blob {hash:String,size:u64,media_type:String}
#[derive(serde::Serialize,serde::Deserialize)]
#[serde(tag="kind",rename_all="camelCase",deny_unknown_fields)]
enum Pin {Head{},Checkpoint{id:String},Snapshot{blob:Blob}}
#[derive(serde::Serialize,serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
struct Link {target:Reference,pin:Pin,role:String}

#[test]
fn neutral_link_codec_matches_independent_serde_and_all_pin_branches() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for (key,accepted) in [("valid",true),("invalid",false)] {
        for row in fixture[key].as_array().unwrap() {
            let oracle=serde_json::from_value::<Link>(row.clone()).ok();
            let input:semio_framework_value::DslValue=serde_json::from_value(row.clone()).unwrap();
            let actual=<ArtifactLink as FromValue>::from_value(input).ok();
            assert_eq!(oracle.is_some(),accepted,"{row}");
            assert_eq!(actual.is_some(),accepted,"{row}");
            if let (Some(oracle),Some(actual))=(oracle,actual) {
                assert_eq!(serde_json::to_value(ToValue::to_value(&actual)).unwrap(),serde_json::to_value(oracle).unwrap());
            }
        }
    }
}

#[test]
fn full_native_link_codec_preserves_schema_branch_control_and_u64() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for row in fixture["valid"].as_array().unwrap() {
        let input:semio_framework_value::DslValue=serde_json::from_value(row.clone()).unwrap();
        let mut link=<ArtifactLink as FromValue>::from_value(input).unwrap();
        if let LinkPin::Snapshot{blob}=&mut link.pin {blob.size=u64::MAX;}
        let field=<ArtifactLink as DslField>::to_value(&link);
        assert_eq!(<ArtifactLink as DslField>::from_value(&field).unwrap(),link);
        let mut allow=|_|true;
        let mut decode=NativeDecodeControl::new(65536,&mut allow);
        assert_eq!(<ArtifactLink as DslField>::from_value_controlled(&field,&mut decode).unwrap(),link);
        let mut allow=|_|true;
        let mut encode=NativeEncodeControl::new(65536,&mut allow);
        let output=<ArtifactLink as DslField>::to_record_controlled(&link,&mut encode).unwrap();
        assert_eq!(FieldValue::Record(output),field);
        let Shape::Record(producer)=<ArtifactLink as DslField>::shape()else{unreachable!()};
        let mut allow=|_|true;
        let spec=NativeDecodeControl::new(65536,&mut allow).produce(&producer).unwrap();
        assert_eq!(spec.fields.iter().map(|field|field.key.as_str()).collect::<Vec<_>>(),["target","pin","role"]);
        let FieldValue::Record(mut invalid)=field else{unreachable!()};
        invalid.fields.insert(99,FieldValue::Absent);
        assert!(<ArtifactLink as DslField>::from_value(&FieldValue::Record(invalid)).is_err());
        let mut deny=|_|false;
        assert!(<ArtifactLink as DslField>::from_value_controlled(&<ArtifactLink as DslField>::to_value(&link),&mut NativeDecodeControl::new(65536,&mut deny)).is_err());
        let mut deny=|_|false;
        assert!(<ArtifactLink as DslField>::to_value_controlled(&link,&mut NativeEncodeControl::new(65536,&mut deny)).is_err());
    }
}

#[test]
fn original_link_receiving_moves_every_pin_branch_and_agrees_with_the_codec_and_the_serde_corpus() {
    use semio_framework_dsl_record::receiving::owned::OwnedScalar;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for row in fixture["valid"].as_array().unwrap() {
        let oracle=serde_json::from_value::<Link>(row.clone()).unwrap();
        let input:semio_framework_value::DslValue=serde_json::from_value(row.clone()).unwrap();
        let mut link=<ArtifactLink as FromValue>::from_value(input).unwrap();
        if let LinkPin::Snapshot{blob}=&mut link.pin {blob.size=u64::MAX;}
        let field=<ArtifactLink as DslField>::to_value(&link);
        let mut source=Some(field.clone());
        let received=<ArtifactLink as OwnedScalar>::take_source(&mut source).unwrap();
        assert_eq!(received,link);
        assert_eq!(received,<ArtifactLink as DslField>::from_value(&field).unwrap());
        assert_eq!(received.role,oracle.role);
        let pin=<LinkPin as DslField>::to_value(&link.pin);
        let mut pin_source=Some(pin.clone());
        assert_eq!(<LinkPin as OwnedScalar>::take_source(&mut pin_source).unwrap(),link.pin);
    }
}

#[test]
fn original_link_receiving_refuses_a_foreign_branch_without_consuming_the_source() {
    use semio_framework_dsl_record::receiving::owned::OwnedScalar;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let input:semio_framework_value::DslValue=serde_json::from_value(fixture["valid"][2].clone()).unwrap();
    let link=<ArtifactLink as FromValue>::from_value(input).unwrap();
    let FieldValue::Record(whole)=<ArtifactLink as DslField>::to_value(&link) else {unreachable!()};
    let mut bad_pin=whole.clone();
    if let Some(FieldValue::Record(pin))=bad_pin.fields.get_mut(&1) {pin.fields.insert(2,FieldValue::Bool(true));}
    let mut bad_role=whole.clone();
    bad_role.fields.insert(2,FieldValue::UInt(1));
    let mut extra=whole.clone();
    extra.fields.insert(3,FieldValue::Absent);
    for foreign in [FieldValue::Bool(true),FieldValue::Record(bad_pin),FieldValue::Record(bad_role),FieldValue::Record(extra)] {
        let mut source=Some(foreign.clone());
        assert!(<ArtifactLink as OwnedScalar>::take_source(&mut source).is_err(),"refused: {foreign:?}");
        assert_eq!(format!("{source:?}"),format!("{:?}",Some(foreign)));
    }
    assert!(<ArtifactLink as OwnedScalar>::take_source(&mut None).is_err());
}
