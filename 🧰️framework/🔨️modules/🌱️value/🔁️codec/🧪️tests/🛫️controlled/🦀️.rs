use super::*;
use crate::native_encoding::{NativeEncodeControl,NativeEncodeProgress};

#[derive(crate::ToValue,serde::Serialize,serde::Deserialize)]
#[value(crate="crate",rename_all="camelCase")]
#[serde(rename_all="camelCase")]
struct OutputEntry{label:String,samples:Vec<i64>,optional:Option<bool>}

#[test]
fn controlled_value_encoding_declared_record_matches_independent_serde(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛫️controlled/🔣️.json")).unwrap();
 for input in fixture["entries"].as_array().unwrap(){let entry:OutputEntry=serde_json::from_value(input.clone()).unwrap();let reference=serde_json::to_value(&entry).unwrap();let mut callback=|_|true;let mut control=NativeEncodeControl::new(1_000_000,&mut callback);let actual=entry.to_value_controlled(&mut control).unwrap();assert_eq!(serde_json::Value::from(&actual),reference);let DslValue::Object(fields)=&actual else{panic!("record output")};assert_eq!(fields.iter().map(|(key,_)|key.as_str()).collect::<Vec<_>>(),fixture["fieldOrder"].as_array().unwrap().iter().map(|value|value.as_str().unwrap()).collect::<Vec<_>>());<DslValue as FromValue>::retire_decoded(actual);}
}
#[test]
fn controlled_value_encoding_primitive_and_intrinsic_outputs_are_admitted(){
 let input=DslValue::Object(vec![("nullable".into(),DslValue::Null),("octets".into(),DslValue::Bytes(vec![0,127,255]))]);let mut callback=|_|true;let mut control=NativeEncodeControl::new(1_000_000,&mut callback);assert_eq!(true.to_value_controlled(&mut control).unwrap(),DslValue::Bool(true));assert_eq!(input.to_value_controlled(&mut control).unwrap(),input);assert!(control.owned_bytes()>0);
}
#[test]
fn controlled_value_encoding_large_copy_and_collection_cancel_before_publication(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛫️controlled/🔣️.json")).unwrap();let count=fixture["collectionItems"].as_u64().unwrap() as usize;let canceled=fixture["cancelAfter"].as_u64().unwrap() as usize;
 let source=vec![String::from("child");count];let mut interior=false;let mut callback=|event:NativeEncodeProgress|{if event.total==count&&event.completed==canceled{interior=true;false}else{true}};let mut control=NativeEncodeControl::new(1_000_000,&mut callback);assert!(source.to_value_controlled(&mut control).is_err());drop(control);assert!(interior);
 let large="x".repeat(fixture["largeTextBytes"].as_u64().unwrap() as usize);let mut accepted=|_|true;let mut control=NativeEncodeControl::new(fixture["tinyBudgetBytes"].as_u64().unwrap() as usize,&mut accepted);assert!(large.to_value_controlled(&mut control).is_err());assert_eq!(control.owned_bytes(),0);
 let mut callback=|_:NativeEncodeProgress|false;let mut control=NativeEncodeControl::new(1_000_000,&mut callback);assert!(large.to_value_controlled(&mut control).is_err());assert_eq!(control.owned_bytes(),0);
}

#[derive(crate::ToValue,serde::Serialize)]
#[value(crate="crate",rename_all="camelCase")]
#[serde(rename_all="camelCase")]
struct OutputShape{#[value(rename="title")]#[serde(rename="title")]name:String,#[value(flatten)]#[serde(flatten)]entry:OutputEntry,#[value(skip)]#[serde(skip)]hidden:std::marker::PhantomData<()>,#[value(skip_serializing_if="Option::is_none")]#[serde(skip_serializing_if="Option::is_none")]absent:Option<String>}
#[derive(crate::ToValue,serde::Serialize)]
#[value(crate="crate",tag="kind",content="payload",rename_all="snake_case")]
#[serde(tag="kind",content="payload",rename_all="snake_case")]
enum OutputAdjacent{Empty,Item(OutputEntry),Named{value:String}}
#[derive(crate::ToValue,serde::Serialize)]
#[value(crate="crate",tag="kind",rename_all="snake_case")]
#[serde(tag="kind",rename_all="snake_case")]
enum OutputInternal{Empty,Item(OutputEntry),Named{value:String}}
#[derive(crate::ToValue,serde::Serialize)]
#[value(crate="crate")]
enum OutputExternal{Empty,Item(OutputEntry),Named{value:String}}
#[derive(crate::ToValue,serde::Serialize)]
#[value(crate="crate",transparent)]
#[serde(transparent)]
struct OutputTransparent{value:Vec<String>}
fn assert_output_reference<T:ToValue+serde::Serialize>(source:&T){let mut callback=|_|true;let mut control=NativeEncodeControl::new(10_000_000,&mut callback);let actual=source.to_value_controlled(&mut control).unwrap();let independent=serde_json::to_value(source).unwrap();let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛫️controlled/🔣️.json")).unwrap();assert!(fixture["representations"].as_array().unwrap().iter().any(|row|row["output"]==independent));assert_eq!(serde_json::Value::from(&actual),independent);<DslValue as FromValue>::retire_decoded(actual);}
#[test]
fn controlled_value_encoding_derived_representations_match_independent_serde(){
 assert_output_reference(&OutputShape{name:"title".into(),entry:OutputEntry{label:"label".into(),samples:vec![1,2],optional:None},hidden:std::marker::PhantomData,absent:None});
 assert_output_reference(&OutputAdjacent::Empty);assert_output_reference(&OutputAdjacent::Item(OutputEntry{label:"item".into(),samples:vec![],optional:Some(true)}));assert_output_reference(&OutputAdjacent::Named{value:"named".into()});
 assert_output_reference(&OutputInternal::Empty);assert_output_reference(&OutputInternal::Item(OutputEntry{label:"item".into(),samples:vec![],optional:None}));assert_output_reference(&OutputInternal::Named{value:"named".into()});
 assert_output_reference(&OutputExternal::Empty);assert_output_reference(&OutputExternal::Item(OutputEntry{label:"item".into(),samples:vec![],optional:None}));assert_output_reference(&OutputExternal::Named{value:"named".into()});
 assert_output_reference(&OutputTransparent{value:vec!["one".into(),"two".into()]});
}
#[test]
fn controlled_value_encoding_wide_nullable_frontier_admits_slots_and_cancels(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛫️controlled/🔣️.json")).unwrap();let input=DslValue::Array(vec![DslValue::Null;fixture["frontier"]["nullableSlots"].as_u64().unwrap() as usize]);let mut accepted=|_|true;let mut control=NativeEncodeControl::new(128,&mut accepted);assert!(input.to_value_controlled(&mut control).is_err());
 let mut saw=false;let mut callback=|event:NativeEncodeProgress|{if event.completed>=256{ saw=true;false}else{true}};let mut control=NativeEncodeControl::new(10_000_000,&mut callback);assert!(input.to_value_controlled(&mut control).is_err());drop(control);assert!(saw);
}

static ORDINARY_OUTPUT:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
fn ordinary_output(value:&String)->DslValue{ORDINARY_OUTPUT.fetch_add(1,std::sync::atomic::Ordering::SeqCst);DslValue::String(value.clone())}
fn explicit_output(value:&String,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{value.to_value_controlled(control)}
#[derive(crate::ToValue)]
#[value(crate="crate")]
struct MissingOutputBridge{#[value(serialize_with="ordinary_output")]value:String}
#[derive(crate::ToValue)]
#[value(crate="crate")]
struct ExplicitOutputBridge{#[value(serialize_with="ordinary_output",serialize_controlled_with="explicit_output")]value:String}
#[derive(crate::ToValue)]
#[value(crate="crate")]
struct OctetOutputBridge{#[value(with="crate::bytes",serialize_controlled_with="crate::bytes::to_value_controlled")]value:Vec<u8>,#[value(with="crate::bytes::optional",serialize_controlled_with="crate::bytes::optional::to_value_controlled")]optional:Option<Vec<u8>>}
#[test]
fn controlled_value_encoding_custom_functions_never_invoke_ordinary_fallback(){
 ORDINARY_OUTPUT.store(0,std::sync::atomic::Ordering::SeqCst);let mut callback=|_|true;let mut control=NativeEncodeControl::new(10_000,&mut callback);assert!(MissingOutputBridge{value:"bridge".into()}.to_value_controlled(&mut control).is_err());let actual=ExplicitOutputBridge{value:"bridge".into()}.to_value_controlled(&mut control).unwrap();assert_eq!(actual,DslValue::Object(vec![("value".into(),DslValue::String("bridge".into()))]));assert_eq!(ORDINARY_OUTPUT.load(std::sync::atomic::Ordering::SeqCst),0);
 let source=OctetOutputBridge{value:vec![0,128,255],optional:Some(vec![])};assert_eq!(source.to_value_controlled(&mut control).unwrap(),DslValue::Object(vec![("value".into(),DslValue::Bytes(vec![0,128,255])),("optional".into(),DslValue::Bytes(vec![]))]));
}
#[test]
fn controlled_value_encoding_scalar_words_and_container_fields_preserve_exact_output(){
 let mut accepted=|_|true;let mut zero=NativeEncodeControl::new(0,&mut accepted);assert_eq!(DslValue::Null.to_value_controlled(&mut zero).unwrap(),DslValue::Null);assert_eq!(false.to_value_controlled(&mut zero).unwrap(),DslValue::Bool(false));assert_eq!(zero.owned_bytes(),0);
 let mut callback=|_|true;let mut control=NativeEncodeControl::new(1_000_000,&mut callback);
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛫️controlled/🔣️.json")).unwrap();for word in fixture["integerWords"].as_array().unwrap(){let value=word.as_str().unwrap().parse::<i64>().unwrap();assert_eq!(value.to_value_controlled(&mut control).unwrap(),DslValue::Number(Number::Int(value)));}assert_eq!(u64::MAX.to_value_controlled(&mut control).unwrap(),DslValue::Number(Number::UInt(u64::MAX)));
 let source=(true,Some("field".to_string()),[1u8,2,3]);assert_eq!(source.to_value_controlled(&mut control).unwrap(),source.to_value());
 let mut map=std::collections::BTreeMap::new();map.insert("first".to_string(),vec![1,2]);map.insert("second".to_string(),vec![3]);assert_eq!(map.to_value_controlled(&mut control).unwrap(),map.to_value());
 let mut map=std::collections::HashMap::new();map.insert(i64::MIN,"low".to_string());map.insert(i64::MAX,"high".to_string());assert_eq!(map.to_value_controlled(&mut control).unwrap(),map.to_value());
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛬️controlled/🔣️.json")).unwrap();for word in fixture["floatWords"].as_array().unwrap(){let bits=u64::from_str_radix(word.as_str().unwrap(),16).unwrap();let DslValue::Number(Number::Float(actual))=f64::from_bits(bits).to_value_controlled(&mut control).unwrap() else{panic!("float")};assert_eq!(actual.to_bits(),bits);}
 for word in fixture["binary32Words"].as_array().unwrap(){let bits=u32::from_str_radix(word.as_str().unwrap(),16).unwrap();let value=f32::from_bits(bits);let actual=value.to_value_controlled(&mut control).unwrap();assert_eq!(f32::from_value(actual).unwrap().to_bits(),bits);}
}

struct RefusedOutput;
impl ToValue for RefusedOutput{fn to_value(&self)->DslValue{panic!("ordinary output must not execute")}}
#[derive(crate::ToValue)]
#[value(crate="crate")]
struct PartialOutput{first:DslValue,later:RefusedOutput}
#[derive(crate::ToValue)]
#[value(crate="crate")]
enum NestedOutput{End,Next(Box<NestedOutput>)}
#[test]
fn controlled_value_encoding_partial_deep_output_is_retired_without_recursive_drop(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛫️controlled/🔣️.json")).unwrap();let mut root=DslValue::Null;for _ in 0..fixture["frontier"]["intrinsicDepth"].as_u64().unwrap(){root=DslValue::Array(vec![root]);}let source=PartialOutput{first:root,later:RefusedOutput};let mut callback=|_|true;let mut control=NativeEncodeControl::new(10_000_000,&mut callback);assert!(source.to_value_controlled(&mut control).unwrap_err().message.contains("no controlled native encoding"));<DslValue as FromValue>::retire_decoded(source.first);
 let mut source=NestedOutput::End;for _ in 0..fixture["frontier"]["typedDepth"].as_u64().unwrap(){source=NestedOutput::Next(Box::new(source));}let mut callback=|_|true;let mut control=NativeEncodeControl::new(10_000_000,&mut callback);assert!(source.to_value_controlled(&mut control).unwrap_err().message.contains("depth limit"));while let NestedOutput::Next(child)=source{source=*child;}
}
#[test]
fn controlled_value_encoding_paths_match_platform_lossy_primitive_without_unadmitted_copies(){
 let mut callback=|_|true;let mut control=NativeEncodeControl::new(1_000_000,&mut callback);let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛫️controlled/🔣️.json")).unwrap();let path=std::path::PathBuf::from(fixture["path"]["utf8"].as_str().unwrap());assert_eq!(path.to_value_controlled(&mut control).unwrap(),path.to_value());
 #[cfg(unix)]{use std::os::unix::ffi::OsStringExt;let path=std::path::PathBuf::from(std::ffi::OsString::from_vec(fixture["path"]["octets"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u8).collect()));assert_eq!(path.to_value_controlled(&mut control).unwrap(),path.to_value());}
 #[cfg(windows)]{use std::os::windows::ffi::OsStringExt;let path=std::path::PathBuf::from(std::ffi::OsString::from_wide(&[97,0xd800,98,0xdc00]));assert_eq!(path.to_value_controlled(&mut control).unwrap(),path.to_value());}
 let path=std::path::PathBuf::from("x".repeat(100_000));let mut callback=|event:NativeEncodeProgress|event.completed<256;let mut control=NativeEncodeControl::new(1_000_000,&mut callback);assert!(path.to_value_controlled(&mut control).is_err());assert_eq!(control.owned_bytes(),0);
}

#[derive(crate::ToValue,serde::Serialize)]
#[value(crate="crate")]
struct OutputSkipOnly{#[value(skip)]#[serde(skip)]field:RefusedOutput}
#[test]
fn controlled_value_encoding_skipped_fields_allocate_no_output_slots(){let source=OutputSkipOnly{field:RefusedOutput};let mut callback=|_|true;let mut control=NativeEncodeControl::new(0,&mut callback);assert_eq!(source.to_value_controlled(&mut control).unwrap(),DslValue::Object(vec![]));assert_eq!(control.owned_bytes(),0);assert_output_reference(&source);}

#[derive(crate::ToValue,serde::Serialize)]
#[value(crate="crate")]
enum OutputBindingExternal{Named{control:String,__output:String,__wrapper:String,__payload:String,__tag:String,__semio_field_control:String,__semio_encoding_field_0:String,__source_field_0:String}}
#[derive(crate::ToValue,serde::Serialize)]
#[value(crate="crate",tag="kind")]
#[serde(tag="kind")]
enum OutputBindingInternal{Named{control:String,__output:String,__wrapper:String,__payload:String,__tag:String,__semio_field_control:String,__semio_encoding_field_0:String,__source_field_0:String}}
#[derive(crate::ToValue,serde::Serialize)]
#[value(crate="crate",tag="kind",content="payload")]
#[serde(tag="kind",content="payload")]
enum OutputBindingAdjacent{Named{control:String,__output:String,__wrapper:String,__payload:String,__tag:String,__semio_field_control:String,__semio_encoding_field_0:String,__source_field_0:String}}
#[test]
fn controlled_value_encoding_field_bindings_preserve_independent_serde_identity(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛫️controlled/🔣️.json")).unwrap();
 for row in fixture["bindingHygiene"].as_array().unwrap(){
  let fields=&row["fields"];macro_rules! source{($owner:ident)=>{$owner::Named{control:fields["control"].as_str().unwrap().into(),__output:fields["__output"].as_str().unwrap().into(),__wrapper:fields["__wrapper"].as_str().unwrap().into(),__payload:fields["__payload"].as_str().unwrap().into(),__tag:fields["__tag"].as_str().unwrap().into(),__semio_field_control:fields["__semio_field_control"].as_str().unwrap().into(),__semio_encoding_field_0:fields["__semio_encoding_field_0"].as_str().unwrap().into(),__source_field_0:fields["__source_field_0"].as_str().unwrap().into()}}}
  fn check<T:ToValue+serde::Serialize>(source:T,expected:&serde_json::Value){let reference=serde_json::to_value(&source).unwrap();assert_eq!(&reference,expected);let ordinary=source.to_value();assert_eq!(serde_json::Value::from(&ordinary),*expected);let mut callback=|_|true;let mut control=NativeEncodeControl::new(10000,&mut callback);let encoded=source.to_value_controlled(&mut control).unwrap();assert_eq!(encoded,ordinary);assert_eq!(serde_json::Value::from(&encoded),reference);let mut refuse=|_|false;let mut canceled=NativeEncodeControl::new(10000,&mut refuse);assert!(source.to_value_controlled(&mut canceled).is_err());let mut zero=NativeEncodeControl::new(0,&mut callback);assert!(source.to_value_controlled(&mut zero).is_err());}
  match row["id"].as_str().unwrap(){"external"=>check(source!(OutputBindingExternal),&row["output"]),"internal"=>check(source!(OutputBindingInternal),&row["output"]),"adjacent"=>check(source!(OutputBindingAdjacent),&row["output"]),_=>panic!("unknown binding representation")}
 }
}

#[derive(crate::ToValue,serde::Serialize)]
#[value(crate="crate")]
enum OutputReserved{Reserved{__output:String,__wrapper:String,entries:String}}
#[test]
fn controlled_value_encoding_named_fields_do_not_shadow_producer_bindings(){assert_output_reference(&OutputReserved::Reserved{__output:"first".into(),__wrapper:"second".into(),entries:"third".into()});}

mod output_namespace_identity{
 struct Result;
 #[derive(crate::ToValue,serde::Serialize)]
 #[value(crate="crate",tag="kind",content="payload")]
 #[serde(tag="kind",content="payload")]
 enum Output{Named{control:String}}
 #[test]
 fn controlled_value_encoding_caller_result_type_does_not_shadow_owned_return(){
  use crate::{ToValue,FromValue,DslValue,NativeEncodeControl};let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🛫️controlled/🔣️.json")).unwrap();assert_eq!(fixture["callerTypeNames"],serde_json::json!(["Result"]));let _identity=Result;let source=Output::Named{control:"controller-field".into()};let reference=serde_json::to_value(&source).unwrap();let mut callback=|_|true;let mut control=NativeEncodeControl::new(10000,&mut callback);let encoded=source.to_value_controlled(&mut control).unwrap();assert_eq!(encoded,DslValue::from(&reference));assert_eq!(source.to_value(),encoded);<DslValue as FromValue>::retire_decoded(encoded);
 }
}
