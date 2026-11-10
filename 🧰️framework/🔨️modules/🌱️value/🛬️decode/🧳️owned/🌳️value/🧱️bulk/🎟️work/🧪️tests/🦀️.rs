use crate::*;
use crate::owned::tree::{NativeValueSource,NativeTreeKind};
use crate::owned::tree::bulk::*;
use crate::driver;
use semio_framework_value_testing_admission::{EffectBank,Wallet};
const BODY:usize=4096;
fn law()->serde_json::Value{serde_json::from_str(include_str!("../📐️.json")).unwrap()}
fn run(input:DslValue,cut:Option<u64>,profile:&serde_json::Value,expected_error:Option<&str>){
 let expected=serde_json::Value::from(&input);let mut bank=EffectBank::new(Wallet::from_profile(profile));let source=driver::admit(&mut bank,input);let pointer=std::ptr::from_ref(source.borrow().get());
 let(mut receiver,heap)=crate::observe_retirement_allocations(||<DslValue as NativeReceive<DslValue>>::receiver());assert_eq!(heap,(0,0));
 let received=driver::receive_faulted(&mut receiver,source.borrow(),&mut bank,BODY,cut,Some(1));assert_eq!(std::ptr::from_ref(source.borrow().get()),pointer);
 match received{
  Ok(Some(output))=>{assert!(cut.is_none()&&expected_error.is_none());assert_eq!(serde_json::Value::from(&output),expected);driver::retire(output,&mut bank)},
  Ok(None)=>assert!(cut.is_some()),
  Err(error)=>assert!(cut.is_none()&&expected_error.is_some_and(|message|error.contains(message)),"{error}"),
 }
 driver::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());drop(receiver);driver::close_source(source,&mut bank);
}
#[test]fn actual_native_tree_matches_independent_serde_under_one_stock_and_every_cancel_cut(){let row=law();for cut in std::iter::once(None).chain(row["cancelCuts"].as_array().unwrap().iter().map(|v|Some(v.as_u64().unwrap()))){run(DslValue::from(row["input"].clone()),cut,&row["stock"],None);}}
#[test]fn actual_native_tree_original_depth_boundary_retains_rejected_input_until_paid_close(){let row=law();assert_eq!(NATIVE_TREE_MAXIMUM_DEPTH,row["productionMaximumDepth"].as_u64().unwrap()as usize);for depth in row["depthCases"].as_array().unwrap(){let depth=depth.as_u64().unwrap()as usize;let mut original=DslValue::Null;let mut independent=serde_json::Value::Null;for _ in 0..depth{original=DslValue::Array(vec![original]);independent=serde_json::Value::Array(vec![independent]);}assert_eq!(serde_json::Value::from(&original),independent);let rejected=depth>NATIVE_TREE_MAXIMUM_DEPTH;run(original,None,&row[if rejected{"rejectedInputStock"}else{"stock"}],rejected.then(||row["depthFailure"].as_str().unwrap()));}}

#[test]fn native_tree_kind_is_a_pure_allocation_free_shape_header(){let row=law();let values=[DslValue::Null,DslValue::Bool(true),DslValue::Number(crate::Number::UInt(1)),DslValue::String(String::new()),DslValue::Array(Vec::new()),DslValue::Object(Vec::new())];let(kinds,heap)=crate::observe_retirement_allocations(||values.each_ref().map(|v|v.tree_kind().unwrap().as_str()));assert_eq!(heap,(0,0));assert_eq!(serde_json::to_value(kinds).unwrap(),row["expectedKinds"]);let mut bank=EffectBank::new(Wallet::from_profile(&row["stock"]));driver::retire(values,&mut bank);}
