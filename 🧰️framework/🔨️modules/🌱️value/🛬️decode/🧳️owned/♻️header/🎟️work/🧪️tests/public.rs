//! ♻️ Public recursive receivers retain every cancelled prefix and exact serde output.
use super::*;
fn observe<R>(f:impl FnOnce()->R)->(R,(usize,usize)){crate::observe_retirement_allocations(f)}
#[derive(crate::RetireOwned,crate::NativeReceive,serde::Serialize)]#[value(crate="crate",native_owned="crate")]struct RecursiveNode{children:Vec<RecursiveNode>}
#[derive(serde::Deserialize,serde::Serialize)]struct OracleNode{children:Vec<OracleNode>}
#[test]fn actual_recursive_native_receivers_keep_finite_headers_exact_serde_output_and_cancelled_custody(){
 use semio_framework_value_testing_admission::{EffectBank,Wallet};
 let f:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();let oracle:OracleNode=serde_json::from_value(f["input"].clone()).unwrap();assert_eq!(serde_json::to_value(oracle).unwrap(),f["input"]);
 for cut in std::iter::once(None).chain(f["cancelCuts"].as_array().unwrap().iter().map(|v|Some(v.as_u64().unwrap()))){
  let mut bank=EffectBank::new(Wallet::from_profile(&f["stock"]));let source=crate::driver::admit(&mut bank,crate::DslValue::from(&f["input"]));
  let(mut receiver,heap)=observe(<RecursiveNode as NativeReceive<crate::DslValue>>::receiver);assert_eq!(heap,(0,0));
  let received=crate::driver::receive_cut(&mut receiver,source.borrow(),&mut bank,4096,cut).unwrap();
  match(cut,received){(None,Some(value))=>{assert_eq!(serde_json::to_value(&value).unwrap(),f["input"]);crate::driver::retire(value,&mut bank)},(Some(_),None)=>{},(cut,received)=>panic!("cut {cut:?} published {}",received.is_some())}
  crate::driver::close(&mut receiver,&mut bank,4096);assert!(receiver.terminal_is_empty());assert_eq!(observe(||drop(receiver)).1,(0,0));crate::driver::close_source(source,&mut bank);
 }
}
