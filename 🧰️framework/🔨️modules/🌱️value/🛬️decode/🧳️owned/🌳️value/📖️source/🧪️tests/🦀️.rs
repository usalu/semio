//! 👓️ Genuine heterogeneous records lend original text and numeric field owners through one object-safe source facet.
use super::*;
struct Original{text:String,count:Number,sequence:Vec<DslValue>}
impl NativeValueSource for Original{fn query_scalar<'r>(&self,query:crate::owned::tree::NativeScalarQuery,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_decoding::native_decoding::OriginalEffectReply<'r,crate::owned::tree::NativeSourceScalar,ValueError>)->Result<semio_framework_value_native_decoding::native_decoding::OriginalEffectReplyComplete<'r,crate::owned::tree::NativeSourceScalar,ValueError>,ValueError>{use crate::owned::tree::{NativeScalarQuery as Q,NativeSourceScalar as S};if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"test Source query exceeded its work authority"))))}Ok(reply.complete(match query{Q::Kind=>Ok(S::Kind(Some(crate::owned::tree::NativeTreeKind::Object))),Q::Null=>Ok(S::Null(false)),Q::RecordLength=>Ok(S::Length(3)),_=>Err(invalid())}))}fn recover_scalar<'r>(&self,_:crate::owned::tree::NativeScalarQuery,_:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_decoding::native_decoding::OriginalEffectReply<'r,crate::owned::tree::NativeSourceScalar,ValueError>)->Result<semio_framework_value_native_decoding::native_decoding::OriginalEffectReplyComplete<'r,crate::owned::tree::NativeSourceScalar,ValueError>,ValueError>{Ok(reply.pending())}fn as_source(&self)->&dyn NativeValueSource{self}fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Object)}fn record_len(&self)->Result<usize,ValueError>{Ok(3)}fn record_key(&self,index:usize)->Result<crate::owned::NativeRecordKey<'_>,ValueError>{["text","count","sequence"].get(index).map(|key|crate::owned::NativeRecordKey::Name(key)).ok_or_else(invalid)}fn record_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match index{0=>Ok(&self.text),1=>Ok(&self.count),2=>Ok(&self.sequence),_=>Err(invalid())}}}
#[test]fn actual_original_children_are_not_wire_proxy_owners(){let original=Original{text:"original".into(),count:Number::UInt(7),sequence:vec![DslValue::Bool(true)]};let source:&dyn NativeValueSource=&original;assert!(std::ptr::eq(source.record_child(0).unwrap().text().unwrap(),&original.text));assert_eq!(source.record_child(1).unwrap().number().unwrap(),original.count);let sequence=source.record_child(2).unwrap();assert_eq!(sequence.sequence_len().unwrap(),1);let actual=sequence.sequence_child(0).unwrap();assert!(std::ptr::eq(actual as *const dyn NativeValueSource as *const (),&original.sequence[0] as *const DslValue as *const()));assert!(actual.boolean().unwrap());assert!(source.record_child(3).is_err());}

#[test]
fn erased_original_sources_keep_typed_receivers_and_ranked_children(){
 fn receiver<T:NativeReceive<dyn NativeValueSource>>(){}
 receiver::<crate::shared_issuance::SealedShared<u64>>();receiver::<Vec<u64>>();receiver::<[u64;2]>();receiver::<semio_framework_value_persistent::list::PagedList<u64,2>>();receiver::<crate::list::persistent::PersistentList<u64>>();
 let fixture:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🌳️value/📖️source/🧪️tests/🧫️fixtures/📐️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){let original=DslValue::from(&row["input"]);let source:&dyn NativeValueSource=&original;let actual=if row["kind"]=="sequence"{let count=<SequenceRows as NativeSequenceInput<dyn NativeValueSource>>::length(source).unwrap();serde_json::Value::Array((0..count).map(|index|serde_json::to_value(<SequenceRows as NativeSequenceInput<dyn NativeValueSource>>::child(source,index).unwrap().number().unwrap().as_u64().unwrap()).unwrap()).collect())}else{let count=<RecordEntries as NativeSequenceInput<dyn NativeValueSource>>::length(source).unwrap();serde_json::Value::Array((0..count).map(|index|{let child=<RecordEntries as NativeSequenceInput<dyn NativeValueSource>>::child(source,index).unwrap();serde_json::json!([child.sequence_child(0).unwrap().text().unwrap(),child.sequence_child(1).unwrap().number().unwrap().as_u64().unwrap()])}).collect())};assert_eq!(actual,row["expected"]);assert_eq!(serde_json::Value::from(&original),row["input"]);}
}


use crate::driver;
use semio_framework_value_testing_admission::{EffectBank,Wallet};
const BODY:usize=4096;
fn receive_original<T:NativeReceive<dyn NativeValueSource>+crate::retirement::RetireOwned>(input:&serde_json::Value,expected:&[u64],observe:impl Fn(&T)->Vec<u64>)where <T as NativeReceive<dyn NativeValueSource>>::Receiver:NativeReceiver<dyn NativeValueSource,T>{
 let fixture:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🌳️value/📖️source/🧪️tests/🧫️fixtures/📐️.json")).unwrap();let mut bank=EffectBank::new(Wallet::from_profile(&fixture["stock"]));
 let original=driver::admit(&mut bank,DslValue::from(input));let foreign=driver::admit(&mut bank,DslValue::from(input));let(mut receiver,h)=crate::observe_retirement_allocations(T::receiver);assert_eq!(h,(0,0));
 let source=||original.borrow().project(0,|value|value.as_source());let foreign_ref=||foreign.borrow().project(0,|value|value.as_source());
 for _ in 0..10000{
  if receiver.output_ready(){break}driver::none(driver::receive_cut(&mut receiver,source(),&mut bank,BODY,Some(1))).unwrap();
  if !receiver.output_ready(){assert!(receiver.demands(foreign_ref(),BODY,1<<20).is_err());assert!(receiver.select_advance(foreign_ref(),BODY,1<<20).is_err());}
  assert_eq!(serde_json::Value::from(original.borrow().get()),*input);assert_eq!(serde_json::Value::from(foreign.borrow().get()),*input);
 }
 assert!(receiver.output_ready());let output=driver::receive_cut(&mut receiver,source(),&mut bank,BODY,None).unwrap().expect("uncut receive publishes");assert_eq!(observe(&output),expected);
 driver::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());assert_eq!(serde_json::Value::from(original.borrow().get()),*input);assert_eq!(crate::observe_retirement_allocations(||drop(receiver)).1,(0,0));
 driver::retire(output,&mut bank);driver::close_source(original,&mut bank);driver::close_source(foreign,&mut bank);
}
#[test]
fn funded_erased_sources_receive_shared_vectors_arrays_paged_and_persistent_owners(){
 let fixture:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🌳️value/📖️source/🧪️tests/🧫️fixtures/📐️.json")).unwrap();let row=&fixture["cases"][0];let expected:Vec<u64>=serde_json::from_value(row["expected"].clone()).unwrap();let input=&row["input"];
 receive_original::<Vec<u64>>(input,&expected,|value|value.clone());
 receive_original::<[u64;2]>(input,&expected,|value|value.to_vec());
 receive_original::<semio_framework_value_persistent::list::PagedList<u64,2>>(input,&expected,|value|(0..value.len()).map(|index|u64::clone(value.get(index).unwrap())).collect());
 receive_original::<crate::list::persistent::PersistentList<u64>>(input,&expected,|value|(0..value.len()).map(|index|u64::clone(value.get(index).unwrap())).collect());
 receive_original::<crate::shared_issuance::SealedShared<u64>>(&input[0],&expected[..1],|value|vec![*value.get()]);
}
