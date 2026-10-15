//! 🔗️ Neutral durable link identity and canonical controlled record laws.
use semio_framework_os_store_link_schema::{ArtifactLink,LinkPin};
use semio_framework_value::{FromValue,ToValue,NativeDecodeControl,NativeEncodeControl};
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
    eprintln!("[DEBUG] Three exact durable link branches and six refusals agree with independent typed Serde");
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
    eprintln!("[DEBUG] Complete native link owner preserves record schema, exact u64, cancellation and unknown-field refusal");
}
