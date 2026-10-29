use super::*;

#[test]
fn erased_original_identifier_source_preserves_unsigned_wire_bounds(){
 use semio_framework_value_native_owned::{native_decoding::NativeReceive,native_decoding::owned::tree::{NativeValueSource,NativeScalarValue}};
 fn receives<T:NativeReceive<dyn NativeValueSource>>(){}
 receives::<EntityId>();
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🆔️native.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){let original=semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(row["wire"].as_u64().unwrap()));let source:&dyn NativeValueSource=&original;let actual=<EntityId as NativeScalarValue<dyn NativeValueSource>>::read(source).map(|id|u64::from(id.0)).ok();assert_eq!(actual,row["expected"].as_u64());}
}
