//! 🧫️ Native text receiving retains every cancelled prefix and matches the independent UTF8 corpus.
use super::*;
use crate::native_receiving::TextRetirement;
use semio_framework_value::{ValueError,retirement::RetirementCursor};
use semio_framework_value::{DslValue,NativeDecodeControl,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome};
use semio_framework_value::native_decoding::{NativeReceive,NativeReceiver};
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
#[path="../../../../../🌱️value/🧪️testing/📥️admission/receiver-driver.rs"]mod driver;
fn run(text:&str,accepted:bool,cut:Option<usize>,_profile:&serde_json::Value){
 let mut bank=driver::bank();let(source,_,_)=semio_framework_value::testing::admit_owned_source(DslValue::String(text.into()),());
 let received=driver::lifecycle(<UiText as NativeReceive<DslValue>>::receiver(),source,&mut bank,1,cut.map(|n|n as u64),|value:&UiText|{assert_eq!(value.as_str(),text);assert_eq!(serde_json::to_value(value).unwrap(),serde_json::json!(text));});
 match cut{None=>{assert_eq!(received.is_ok(),accepted,"{received:?}");if accepted{assert_eq!(received,Ok(true));}},Some(_)=>assert!(matches!(received,Ok(false)|Err(_)),"a cancelled receive publishes nothing: {received:?}")}
}
#[test]
fn native_ui_text_receiving_preserves_denied_and_cancelled_prefixes(){let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🧱️foundation/🔣️.json")).unwrap();let profile:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();for row in corpus["texts"].as_array().unwrap(){let text=row["unit"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap()as usize);let accepted=row["valid"].as_bool().unwrap();assert_eq!(serde_json::from_value::<UiText>(serde_json::json!(text)).is_ok(),accepted);run(&text,accepted,None,&profile);for cut in profile["cancelPrefixes"].as_array().unwrap(){run(&text,accepted,Some(cut.as_u64().unwrap()as usize),&profile);}}}

#[test]
fn text_retirement_charges_its_quoted_work_to_the_calling_effect(){
 use semio_framework_value::retained_clone::OriginalEffectReply;
 use semio_framework_value::RetainedWorkPoll;
 use semio_framework_value::retirement::RetirementStep;
 use semio_framework_value_testing_admission::{EffectBank,Wallet};
 let law:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();
 for limit in [0u64,1]{
  let mut cursor=TextRetirement(Some(UiText::default()));let quote=cursor.next_computational_work(1).unwrap();assert_eq!(quote,law["retirementWork"]["occupied"].as_u64().unwrap());
  let stock=RetainedCloneGrant{maximum_items:4,maximum_copy_bytes:1<<16,maximum_capacity_bytes:1<<16,maximum_release_bytes:1<<16,maximum_depth:4};let mut bank=EffectBank::new(Wallet::new(stock,8));let grant=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..stock};let mut answer=None;
  let((poll,_),heap)=observe(||bank.run(grant,limit,|effect|{let mut stash=None;{let reply=OriginalEffectReply::borrow(&mut stash).unwrap();drop(cursor.close_into(1,effect,reply).unwrap());}answer=stash.take();RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work:effect.performed_work()})}));
  let RetainedWorkPoll::Complete(turn)=poll else{panic!("text retirement settles in its own action")};assert_ne!(bank.take_admitted(),Some(false));
  if limit==0{assert_eq!(turn.work,0);assert!(matches!(answer,Some(Ok(RetirementStep::BudgetExhausted))));assert!(!cursor.terminal_is_empty());continue}
  assert_eq!(turn.work,u128::from(quote));assert!(matches!(answer,Some(Ok(RetirementStep::Complete(_)))));assert!(cursor.terminal_is_empty());assert_eq!(cursor.next_computational_work(1).unwrap(),law["retirementWork"]["empty"].as_u64().unwrap());assert_eq!(heap,(0,0));
 }
}
