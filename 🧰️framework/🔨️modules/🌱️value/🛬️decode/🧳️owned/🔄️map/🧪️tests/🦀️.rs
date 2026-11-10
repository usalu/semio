use crate::*;
use crate::driver;
use crate::owned::{NativeReceiveMap,NativeMappedReceiver,NativeReceiver};
use crate::DslValue;
use semio_framework_value_testing_admission::{EffectBank,Wallet};
#[derive(crate::RetireOwned)]#[value(crate="crate")]
struct Wrapped(String);
impl NativeReceiveMap<String> for Wrapped{fn from_native(value:String)->Self{Self(value)}}
const BODY:usize=4096;
fn bank()->EffectBank{EffectBank::new(Wallet::from_profile(&serde_json::from_str::<serde_json::Value>(include_str!("🎟️work/📐️.json")).unwrap()))}
fn check_mapping<I:crate::owned::tree::NativeValueSource+?Sized+'static>(project:for<'a>fn(RetainedCloneRef<'a,DslValue>)->RetainedCloneRef<'a,I>){
 let law:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🔄️map/🧪️tests/🔣️.json")).unwrap();let text=law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap()as usize);let oracle:serde_json::Value=serde_json::from_str(&serde_json::to_string(&text).unwrap()).unwrap();
 for cut in law["cancelPrefixes"].as_array().unwrap().iter().map(|v|Some(v.as_u64().unwrap())).chain([None]){
  let mut bank=bank();let source=driver::admit(&mut bank,DslValue::String(text.clone()));
  let(mut receiver,h)=crate::observe_retirement_allocations(NativeMappedReceiver::<I,String,Wrapped>::new);assert_eq!(h,(0,0));
  match cut{
   Some(cut)=>driver::none(driver::receive_cut(&mut receiver,project(source.borrow()),&mut bank,BODY,Some(cut))).unwrap(),
   None=>{
    while !receiver.output_ready(){driver::none(driver::receive_cut(&mut receiver,project(source.borrow()),&mut bank,BODY,Some(1))).unwrap()}
    assert_eq!(receiver.take_demand().unwrap().copy_bytes,2*std::mem::size_of::<Option<Wrapped>>());
    let output=driver::receive_cut(&mut receiver,project(source.borrow()),&mut bank,BODY,None).unwrap().expect("uncut receive publishes");assert_eq!(serde_json::Value::String(output.0.clone()),oracle);driver::retire(output,&mut bank);
   }
  }
  driver::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());drop(receiver);driver::close_source(source,&mut bank);
 }
}
#[test]fn native_mapping_preserves_completed_original_text_and_every_cancelled_child(){check_mapping::<DslValue>(|input|input);check_mapping::<dyn crate::owned::tree::NativeValueSource>(|input|input.project(0,|source|source as &dyn crate::owned::tree::NativeValueSource));}

#[test]fn native_mapped_close_intent_matches_the_shared_finite_corpus_without_closing_its_child(){
 let law:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../✨️derive/🛬️receive/🎟️frontier/🧪️tests/🔣️.json")).unwrap();
 for row in law["closeIntent"].as_array().unwrap(){
  let mut bank=bank();let(mut receiver,h)=crate::observe_retirement_allocations(NativeMappedReceiver::<DslValue,String,Wrapped>::new);assert_eq!(h,(0,0));assert!(!receiver.child_is_closing());
  let full=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:std::mem::size_of::<bool>(),maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
  let begin=|bank:&mut EffectBank,receiver:&mut NativeMappedReceiver<DslValue,String,Wrapped>,grant:RetainedCloneGrant,maximum:u64|{let((poll,_),heap)=crate::observe_retirement_allocations(||bank.run(grant,maximum,|effect|driver::settle::<OriginalRetirementFailure>(effect,|effect,reply|receiver.begin_close_into(effect,reply))));assert_ne!(bank.take_admitted(),Some(false));match poll{RetainedWorkPoll::Complete(turn)=>(turn,heap),RetainedWorkPoll::Pending{..}=>panic!("close start stayed pending")}};
  if row["initiallyClosing"].as_bool().unwrap(){let(t,h)=begin(&mut bank,&mut receiver,full,1);assert_eq!(t.work,1);assert!(t.result.is_ok());assert_eq!(h,(0,0));}
  let grant=RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_copy_bytes:row["copyBytes"].as_u64().unwrap()as usize,..full};
  let(t,h)=begin(&mut bank,&mut receiver,grant,row["maximumWork"].as_u64().unwrap());let work=t.work;let p=t.result.unwrap().progress();assert_eq!(work,u128::from(row["work"].as_u64().unwrap()));assert_eq!(p.copied_items,work as usize);assert_eq!(p.copied_bytes,work as usize*std::mem::size_of::<bool>());assert_eq!(h,(0,0));
  assert_eq!(receiver.is_closing(),row["ownClosing"].as_bool().unwrap());assert_eq!(receiver.child_is_closing(),row["childClosing"].as_bool().unwrap());assert!(receiver.terminal_is_empty());
 }
}
