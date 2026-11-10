use semio_framework_test_fixtures::fixture_text;
use super::*;
use crate::{DslValue,Number,ValueRefusalKind,list::persistent::PersistentList,shared_issuance::SealedShared};
use semio_framework_value_testing_admission::{EffectBank,Wallet};
use driver::Trace;
const BODY:usize=512;
#[derive(crate::RetireOwned,crate::NativeReceive)]#[value(crate="crate",native_owned="crate")]
struct Row{label:SealedShared<String>,rank:i64}
impl std::fmt::Debug for Row{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("Row").field("rank",&self.rank).finish()}}
impl Clone for Row{fn clone(&self)->Self{panic!("persistent native receiving must never clone an owned typed row")}}
fn observe<T>(run:impl FnOnce()->T)->(T,(usize,usize)){crate::observe_retirement_allocations(run)}
fn fixture()->serde_json::Value{serde_json::from_str(fixture_text!("../../📋️list/🧪️tests/🧫️fixtures/📐️.json")).unwrap()}
fn input(count:usize,prefix:&str,stride:i64,repeat:usize,bad:Option<usize>)->DslValue{DslValue::Array((0..count).map(|i|DslValue::Object(vec![("label".into(),DslValue::String(if i==0{prefix.repeat(repeat)}else{format!("{prefix}{i}")})),("rank".into(),if bad==Some(i){DslValue::String("refused".into())}else{DslValue::Number(Number::Int(i as i64*stride))})])).collect())}
fn bank(f:&serde_json::Value)->EffectBank{EffectBank::new(Wallet::from_profile(&f["stock"]))}
fn rows(root:&PersistentList<Row>,count:usize)->Vec<serde_json::Value>{(0..count).map(|i|{let row=root.get(i).unwrap();serde_json::json!({"label":row.label.as_str(),"rank":row.rank})}).collect()}
type Receiver=<PersistentList<Row> as NativeReceive<DslValue>>::Receiver;
fn finish(mut receiver:Receiver,bank:&mut EffectBank){driver::close(&mut receiver,bank,BODY);assert!(receiver.terminal_is_empty());assert_eq!(observe(||drop(receiver)).1,(0,0));}
#[test]fn native_persistent_list_receives_original_rows_and_preserves_all_cancelled_custody(){
 let f=fixture();let prefix=f["prefix"].as_str().unwrap();let stride=f["stride"].as_i64().unwrap();let repeat=f["firstRepeat"].as_u64().unwrap()as usize;
 for count in f["cases"].as_array().unwrap(){let count=count.as_u64().unwrap()as usize;for cut in std::iter::once(None).chain(f["cuts"].as_array().unwrap().iter().map(|v|Some(v.as_u64().unwrap()))){
  let mut bank=bank(&f);let original=driver::admit(&mut bank,input(count,prefix,stride,repeat,None));let foreign=driver::admit(&mut bank,input(count,prefix,stride,repeat,None));
  let pointer=std::ptr::from_ref(original.borrow().get());let expected=serde_json::Value::from(original.borrow().get());
  let(mut receiver,h)=observe(<PersistentList<Row> as NativeReceive<DslValue>>::receiver);assert_eq!(h,(0,0));
  let received=driver::receive_cut(&mut receiver,original.borrow(),&mut bank,BODY,cut).unwrap();
  if !receiver.output_ready()&&cut.is_some_and(|cut|cut>0){assert!(receiver.demands(foreign.borrow(),BODY,1<<20).is_err())}
  assert_eq!(std::ptr::from_ref(original.borrow().get()),pointer);
  let output=match(cut,received){(None,Some(root))=>{assert_eq!(root.len(),count);assert_eq!(serde_json::Value::Array(rows(&root,count)),expected);Some(root)},(Some(_),None)=>None,(cut,received)=>panic!("cut {cut:?} published {}",received.is_some())};
  finish(receiver,&mut bank);driver::close_source(foreign,&mut bank);driver::close_source(original,&mut bank);
  if let Some(root)=output{for i in 0..count{assert_eq!(root.get(i).unwrap().rank,i as i64*stride)}driver::retire(root,&mut bank)}
 }}
}
#[test]fn native_persistent_list_later_row_refusal_retains_original_partial_owner(){
 let f=fixture();for bad in f["invalidRows"].as_array().unwrap(){
  let mut bank=bank(&f);let original=driver::admit(&mut bank,input(257,"α🙂",2,1,Some(bad.as_u64().unwrap()as usize)));let before=serde_json::Value::from(original.borrow().get());
  let mut receiver=<PersistentList<Row> as NativeReceive<DslValue>>::receiver();
  let error=driver::receive_cut(&mut receiver,original.borrow(),&mut bank,BODY,None).unwrap_err();assert!(error.contains("InvalidValue"),"row refusal kind: {error}");
  assert_eq!(serde_json::Value::from(original.borrow().get()),before);finish(receiver,&mut bank);driver::close_source(original,&mut bank);
 }
}
#[test]
fn native_persistent_list_original_callback_refusal_preserves_partial_receiving(){
 let f=fixture();for allow in[0u64,1,3,8,31,256]{
  let mut bank=bank(&f);let original=driver::admit(&mut bank,input(17,"α🙂",2,10923,None));let before=serde_json::Value::from(original.borrow().get());
  let mut receiver=<PersistentList<Row> as NativeReceive<DslValue>>::receiver();let flag=std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));let mut trace=Trace{admit:Some(flag.clone()),..Default::default()};
  driver::none(driver::receive_traced(&mut receiver,original.borrow(),&mut bank,BODY,Some(allow),None,&mut trace)).unwrap();
  if !receiver.output_ready(){flag.store(false,std::sync::atomic::Ordering::SeqCst);let error=driver::receive_traced(&mut receiver,original.borrow(),&mut bank,BODY,None,None,&mut trace).unwrap_err();assert!(error.contains("Canceled"),"cancellation kind: {error}");}
  assert_eq!(serde_json::Value::from(original.borrow().get()),before);finish(receiver,&mut bank);driver::close_source(original,&mut bank);
 }
}
