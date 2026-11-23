//! 📝️ Actual original paged receiving keeps one explicit finite bank through source, output and closure.
use semio_framework_value::{paged::PagedUtf8,RetainedCloneGrant,DslValue};
use semio_framework_value_native_owned::native_decoding::{NativeReceive,NativeReceiver};
use semio_framework_value_testing_admission::{EffectBank,Wallet};
use semio_framework_value_testing_allocator as _;
#[path="../../../🧪️testing/📥️admission/receiver-driver.rs"]
pub(crate) mod driver;
#[path="../../../🧪️testing/📥️admission/default-receiver.rs"]
pub(crate) mod default_receiver;
pub(crate) fn bank()->EffectBank{let f:serde_json::Value=serde_json::from_str::<serde_json::Value>(include_str!("../../🎮️append/🎟️work/📐️.json")).unwrap()["receiveLaw"].clone();let s=&f["stock"];EffectBank::new(Wallet::new(RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:s["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize},1<<30))}
#[path="../🗂️map/🧪️tests/🦀️.rs"]mod maps;
#[test]
fn original_paged_text_admits_exact_unicode_and_closes_every_partial_owner(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){let text=row["text"].as_str().unwrap();assert_eq!(text.len(),row["bytes"].as_u64().unwrap()as usize);
  if row["maximum"]==1{let mut bank=bank();let source=driver::admit(&mut bank,DslValue::String(text.into()));let mut receiver=<PagedUtf8<1> as NativeReceive<DslValue>>::receiver();assert!(driver::receive_cut(&mut receiver,source.borrow(),&mut bank,4096,None).is_err());driver::close::<DslValue,PagedUtf8<1>,_>(&mut receiver,&mut bank,4096);assert!(receiver.terminal_is_empty());drop(receiver);driver::close_source(source,&mut bank);continue}
  for cut in [Some(0),Some(1),Some(4),Some(40),None]{let mut bank=bank();let source=driver::admit(&mut bank,DslValue::String(text.into()));let receiver=<PagedUtf8<4096> as NativeReceive<DslValue>>::receiver();
   let published=driver::lifecycle(receiver,source,&mut bank,4096,cut,|value:&PagedUtf8<4096>|{assert_eq!(value.to_string_owner(),text);assert_eq!(value.len(),text.len());assert_eq!(serde_json::to_string(&value.to_string_owner()).unwrap(),serde_json::to_string(text).unwrap())}).unwrap();assert_eq!(published,cut.is_none());
  }
 }
}
