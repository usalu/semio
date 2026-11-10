//! 🔗️ Shared records, defaults and flattened schemas exercise public owners from the same package.
pub use semio_framework_value_native_owned::*;
pub use semio_framework_value_native_owned::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,RetainedCloneClose,RetainedCloneCursor,retained_clone_grant_admits};
pub use semio_framework_value_native_owned::retirement::RetireOwned;
pub use semio_framework_value_derive::{RetireOwned,NativeReceive,ToValue};
pub use semio_framework_value_intrinsic::{ToValue,NativeEncodeControl,ValueShape};
pub fn observe_retirement_allocations<R>(run:impl FnOnce()->R)->(R,(usize,usize)){semio_framework_value_testing_allocator::observe_retirement_allocations(run)}
pub use semio_framework_value_native_owned::shared_issuance::SealedShared;
use semio_framework_test_fixtures::fixture_text;
#[path="../../../../../🧪️testing/📥️admission/receiver-driver.rs"]pub(crate) mod driver;
#[path="../../../../../🧪️testing/📥️admission/default-receiver.rs"]pub(crate) mod default_receiver;
#[path="../📥️driver/🦀️.rs"]mod shared_driver;
use shared_driver::*;
#[derive(crate::RetireOwned,crate::NativeReceive)]#[value(crate="crate")]
struct WideRecord{
 a:SealedShared<String>,b:SealedShared<String>,c:SealedShared<String>,d:SealedShared<String>,e:SealedShared<String>,
 f:SealedShared<String>,g:SealedShared<String>,h:SealedShared<String>,i:SealedShared<String>,j:SealedShared<String>,
 k:SealedShared<String>,l:SealedShared<String>,m:SealedShared<String>,n:SealedShared<String>,o:SealedShared<String>,
 p:SealedShared<String>,q:SealedShared<String>,r:SealedShared<String>,s:SealedShared<String>,t:SealedShared<String>,u:SealedShared<String>,
 v:SealedShared<String>,w:SealedShared<String>,x:SealedShared<String>,y:SealedShared<String>,z:SealedShared<String>,
 aa:SealedShared<String>,ab:SealedShared<String>,ac:SealedShared<String>,ad:SealedShared<String>,ae:SealedShared<String>,af:SealedShared<String>,
}
#[test]
fn native_record32_single_active_child_fits4096_and_preserves_every_original_prefix(){
 let keys=["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z","aa","ab","ac","ad","ae","af"];
 sampled_prefixes::<WideRecord>(65536,256,BODY,||DslValue::Object(keys.into_iter().map(|key|(key.to_owned(),DslValue::String(format!("original {key} 🏠️")))).collect()),|_,receiver,_,_|assert!(receiver.native_field_owner_count()<=1),|row|{assert_eq!(row.a.get(),"original a 🏠️");assert_eq!(row.q.get(),"original q 🏠️");assert_eq!(row.af.get(),"original af 🏠️");});
}
#[derive(crate::RetireOwned,crate::NativeReceive)]#[value(crate="crate")]
struct GroupName{name:SealedShared<String>}
#[derive(crate::RetireOwned,crate::NativeReceive)]#[value(crate="crate")]
struct GroupVolume{volume:f64}

#[derive(Default,crate::RetireOwned,crate::NativeReceive,crate::ToValue)]#[value(crate="crate")]
enum OriginalDefaultMedia { #[default] Empty,Text(String) }
#[derive(crate::RetireOwned,crate::NativeReceive,crate::ToValue)]#[value(crate="crate")]
struct OriginalDeclaredDefaults {
 id:String,
 #[value(default)]values:Vec<String>,
 #[value(default)]scale:f64,
 #[value(default)]enabled:bool,
 #[value(default)]media:OriginalDefaultMedia,
 optional:Option<String>,
}
#[test]
fn native_declared_defaults_keep_missing_fields_original_and_heapless(){
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../📥️default/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 prefixes::<OriginalDeclaredDefaults>(2048,||DslValue::from(&law["input"]),|_,_,_,_|{},|output|{assert_eq!(output.scale.to_bits(),0.0_f64.to_bits());assert_eq!(serde_json::Value::from(crate::ToValue::to_value(&output.scale)),serde_json::to_value(0.0_f64).unwrap());assert_eq!(serde_json::Value::from(crate::ToValue::to_value(output)),law["expected"]);});
 use crate::owned::NativeDefault;
 let mut bank=bank();let source=driver::admit(&mut bank,DslValue::Null);let(cursor,heap)=crate::observe_retirement_allocations(Vec::<String>::default_cursor);assert_eq!(heap,(0,0));let mut cursor=crate::default_receiver::DefaultReceiver(cursor);
 let before=bank.wallet().remaining();while !NativeReceiver::<DslValue,Vec<String>>::output_ready(&cursor){driver::none(driver::receive_cut(&mut cursor,source.borrow(),&mut bank,BODY,Some(1))).unwrap()}let value:Vec<String>=driver::receive_cut(&mut cursor,source.borrow(),&mut bank,BODY,None).unwrap().expect("uncut receive publishes");let after=bank.wallet().remaining();
 assert_eq!((before.maximum_capacity_bytes,before.maximum_release_bytes),(after.maximum_capacity_bytes,after.maximum_release_bytes));assert_eq!(value.capacity(),0);driver::retire(value,&mut bank);driver::close::<DslValue,Vec<String>,_>(&mut cursor,&mut bank,BODY);assert!(NativeReceiver::<DslValue,Vec<String>>::terminal_is_empty(&cursor));drop(cursor);driver::close_source(source,&mut bank);
}
#[derive(crate::RetireOwned,crate::NativeReceive)]#[value(crate="crate")]
struct FlattenedRecord{
 #[value(flatten)]name:SealedShared<GroupName>,
 #[value(flatten)]volume:SealedShared<GroupVolume>,
}
#[test]
fn native_flattened_original_record_preserves_source_schema_and_all_partial_custody(){
 let input=||DslValue::Object(vec![("name".into(),DslValue::String("original β".into())),("volume".into(),DslValue::Number(crate::Number::Float(123.5)))]);
 let binding_work={let mut bank=bank();let source=driver::admit(&mut bank,input());let work=source.borrow().binding_work_demand(&None).unwrap();driver::close_source(source,&mut bank);u128::from(work)};
 sampled_prefixes::<FlattenedRecord>(16384,32,1<<16,input,|turn,_,_,trace|if turn==0{assert_eq!(trace.advance.first().copied(),Some(binding_work));},|row|{assert_eq!(row.name.get().name.get(),"original β");assert_eq!(row.volume.get().volume,123.5);});
 for entries in [vec![("name",DslValue::String("a".into())),("volume",DslValue::Number(crate::Number::Float(1.0))),("unknown",DslValue::Null)],vec![("name",DslValue::String("a".into())),("name",DslValue::String("b".into())),("volume",DslValue::Number(crate::Number::Float(1.0)))],vec![("name",DslValue::String("a".into()))]]{
  let mut bank=bank();let original=driver::admit(&mut bank,DslValue::Object(entries.into_iter().map(|(key,value)|(key.to_owned(),value)).collect()));let mut receiver=<FlattenedRecord as NativeReceive<DslValue>>::receiver();let mut refused=false;
  for _ in 0..16384{if receiver.output_ready(){break}if driver::receive_cut(&mut receiver,original.borrow(),&mut bank,BODY,Some(1)).is_err(){refused=true;break}}
  assert!(refused);assert!(!receiver.output_ready());driver::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());drop(receiver);driver::close_source(original,&mut bank);
 }
}
#[path="../🫙️empty/🦀️.rs"]mod empty_record;
#[test]
fn native_flattened_ordinals_refuse_ambiguous_declared_ids(){
 use crate::native_decoding::owned::{NativeRecordFields,NativeRecordMatcher,NativeSchemaKeySource,NativeFieldMatch};
 for(input,key,expected)in [(DslValue::String("name".into()),NativeSchemaKeySource::Text,NativeFieldMatch::Field(0)),(DslValue::String("volume".into()),NativeSchemaKeySource::Text,NativeFieldMatch::Field(1)),(DslValue::Number(crate::Number::UInt(0)),NativeSchemaKeySource::EnumOrdinal,NativeFieldMatch::Ambiguous),(DslValue::Number(crate::Number::UInt(1)),NativeSchemaKeySource::EnumOrdinal,NativeFieldMatch::Missing)]{
  let mut bank=bank();let original=driver::admit(&mut bank,input);let mut matcher=MatcherReceiver::new(NativeRecordMatcher::new(FlattenedRecord::native_schema_fields(),FlattenedRecord::native_schema_count(),key));let mut trace=driver::Trace::default();
  while !matcher.output_ready(){driver::none(driver::receive_traced(&mut matcher,original.borrow(),&mut bank,BODY,Some(1),None,&mut trace)).unwrap();}
  assert_eq!(matcher.inner.result(),Some(expected));assert!(trace.advance.iter().any(|work|*work>1));
  driver::close(&mut matcher,&mut bank,BODY);assert!(matcher.terminal_is_empty());drop(matcher);driver::close_source(original,&mut bank);
 }
}
fn ordinal_schema(index:usize)->Option<crate::owned::NativeRecordSchema>{[(0u32,"zero"),(7,"seven"),(u32::MAX,"maximum")].get(index).map(|(ordinal,name)|crate::owned::NativeRecordSchema::Leaf(crate::owned::NativeSchemaLeaf{name,ordinal:*ordinal}))}
#[test]
fn native_dsl_enum_ordinals_accept_only_non_negative_u32_numbers(){
 use crate::native_decoding::owned::{NativeFieldMatch,NativeRecordMatcher,NativeSchemaKeySource};
 let cases=[(DslValue::Number(crate::Number::UInt(0)),Some(0)),(DslValue::Number(crate::Number::UInt(u64::from(u32::MAX))),Some(2)),(DslValue::Number(crate::Number::UInt(u64::from(u32::MAX)+1)),None),(DslValue::Number(crate::Number::Int(7)),Some(1)),(DslValue::Number(crate::Number::Int(-1)),None),(DslValue::Number(crate::Number::Int(i64::MAX)),None),(DslValue::Number(crate::Number::Float(1.0)),None),(DslValue::String("0".into()),None)];
 for(value,expected)in cases{
  let mut bank=bank();let original=driver::admit(&mut bank,value);let mut matcher=MatcherReceiver::new(NativeRecordMatcher::new(unsafe{crate::owned::NativeSchemaFields::native(ordinal_schema)},3,NativeSchemaKeySource::EnumOrdinal));let mut refused=false;
  for _ in 0..10000{if matcher.output_ready(){break}if driver::receive_cut(&mut matcher,original.borrow(),&mut bank,BODY,Some(1)).is_err(){refused=true;break}}
  match expected{Some(field)=>{assert!(!refused);assert_eq!(matcher.inner.result(),Some(NativeFieldMatch::Field(field)))},None=>{assert!(refused||matcher.inner.result()==Some(NativeFieldMatch::Missing))}}
  driver::close(&mut matcher,&mut bank,BODY);assert!(matcher.terminal_is_empty());drop(matcher);driver::close_source(original,&mut bank);
 }
}
