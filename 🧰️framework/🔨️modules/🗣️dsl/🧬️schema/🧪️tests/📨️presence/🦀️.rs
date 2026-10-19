use crate::{DslField,FieldValue,JoinMode,ParseOptions,Shape,parse_exact,parse_exact_controlled,print};
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl};

#[derive(Debug,PartialEq,serde::Serialize,semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword="presence")]
struct Presence {
    id:String,
    #[value(required)]
    factor:Option<f64>,
    #[value(default,deserialize_with="protocol::list_delta::supplied_value",skip_serializing_if="Option::is_none")]
    #[serde(skip_serializing_if="Option::is_none")]
    index:Option<usize>,
}

#[test]
fn value_presence_ordinary_controlled_and_borrowed_boundaries_share_intent() {
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🧪️fixtures/📨️presence/🔣️.json")).unwrap();
    let grammar=semio_framework_dsl::parse_grammar(corpus["grammar"].as_str().unwrap()).unwrap();let recognizer=semio_framework_dsl::Recognizer::compile(&grammar,&semio_framework_dsl::FragmentRegistry::new(),vec![crate::record_fields_matcher::<Presence>("presence-fields")]).unwrap();
    let spec=Presence::__dsl_spec();
    assert!(!spec.fields[1].optional);assert!(spec.fields[2].optional);assert!(matches!(spec.fields[1].shape,Shape::Block(_)));assert!(matches!(spec.fields[2].shape,Shape::UInt));
    let borrowed=<Presence as crate::BorrowedDslRecord>::RECORD;assert!(!borrowed.fields[1].optional);assert!(borrowed.fields[2].optional);
    for case in corpus["cases"].as_array().unwrap(){
        let text=case["text"].as_str().unwrap();let accepted=case["accepted"].as_bool().unwrap();assert_eq!(recognizer.recognize(text).unwrap(),accepted,"{}",case["id"]);
        let ordinary=parse_exact(text,&spec,&ParseOptions::default()).and_then(|record|Presence::__dsl_from_record(&record));assert_eq!(ordinary.is_ok(),accepted,"{}",case["id"]);
        let mut observe=|_:semio_framework_value::native_decoding::NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(32768,&mut observe);
        let controlled=parse_exact_controlled(text,&spec,&ParseOptions::default(),&mut control).map_err(|error|error.to_string()).and_then(|record|Presence::from_record_controlled(&record,&mut control).map_err(|error|error.to_string()));assert_eq!(controlled.is_ok(),accepted,"{}",case["id"]);
        if accepted{
            let ordinary=ordinary.unwrap();let controlled=controlled.unwrap();assert_eq!(ordinary,controlled);assert_eq!(serde_json::to_value(&ordinary).unwrap(),case["json"]);
            let record=ordinary.__dsl_to_record();let printed=print(&record,&spec,JoinMode::Inline);let decoded=Presence::__dsl_from_record(&parse_exact(&printed,&spec,&ParseOptions::default()).unwrap()).unwrap();assert_eq!(decoded,ordinary);
            let mut observe=|_:semio_framework_value::native_encoding::NativeEncodeProgress|true;let mut encode=NativeEncodeControl::new(32768,&mut observe);let producer=Presence::__dsl_spec_producer();let controlled_spec=(producer.encoding)(&mut encode).unwrap();assert!(!controlled_spec.fields[1].optional);assert!(controlled_spec.fields[2].optional);
            assert_eq!(ordinary.to_record_controlled(&mut encode).unwrap(),record);
            eprintln!("[DEBUG] DSL presence case={} requiredNullableFactor=true indexSupplied={} roundtrip=true",case["id"],ordinary.index.is_some());
        }
    }
    let mut record=parse_exact("presence id=\"c1\" factor { }",&spec,&ParseOptions::default()).unwrap();record.fields.insert(1,FieldValue::Absent);assert!(Presence::__dsl_from_record(&record).is_err());
    let mut observe=|_:semio_framework_value::native_decoding::NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(32768,&mut observe);assert!(Presence::from_record_controlled(&record,&mut control).is_err());
}

#[derive(semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword="batch")]
struct PresenceBatch { #[dsl(table)] rows:Vec<Presence> }

#[test]
fn value_presence_table_parser_preserves_required_nullable_fields() {
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🧪️fixtures/📨️presence/🔣️.json")).unwrap();let spec=PresenceBatch::__dsl_spec();
    for case in corpus["tableCases"].as_array().unwrap(){
        let text=case["text"].as_str().unwrap();let accepted=case["accepted"].as_bool().unwrap();let ordinary=parse_exact(text,&spec,&ParseOptions::default());assert_eq!(ordinary.is_ok(),accepted,"{}",case["id"]);
        let mut observe=|_:semio_framework_value::native_decoding::NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(32768,&mut observe);let controlled=parse_exact_controlled(text,&spec,&ParseOptions::default(),&mut control);assert_eq!(controlled.is_ok(),accepted,"{}",case["id"]);
        if accepted {let ordinary=PresenceBatch::__dsl_from_record(&ordinary.unwrap()).unwrap();let controlled=PresenceBatch::from_record_controlled(&controlled.unwrap(),&mut control).unwrap();assert_eq!(ordinary.rows,controlled.rows);assert_eq!(serde_json::to_value(&ordinary.rows[0]).unwrap(),case["json"]);}
    }
}
