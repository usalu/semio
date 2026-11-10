use super::*;
use crate::{DslValue,Number};
use crate::driver;
use crate::owned::NativeReceiver;
use semio_framework_value_testing_admission::{EffectBank,Wallet};
const BODY:usize=4096;
#[test]fn actual_box_receiving_funds_real_backing_and_every_cancelled_partial_owner(){
 let row:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();let mut bank=EffectBank::new(Wallet::from_profile(&row["stock"]));assert_eq!(row["deniedWork"].as_u64().unwrap(),0);
 for cut in row["cancelPrefixes"].as_array().unwrap().iter().map(|x|Some(x.as_u64().unwrap())).chain([None]){
  let source=driver::admit(&mut bank,DslValue::Number(Number::UInt(row["value"].as_u64().unwrap())));
  let(mut receiver,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||<Box<u64> as NativeReceive<DslValue>>::receiver());assert_eq!(heap,(0,0));
  match cut{
   Some(cut)=>driver::none(driver::receive_cut(&mut receiver,source.borrow(),&mut bank,BODY,Some(cut))).unwrap(),
   None=>{let output=driver::receive_cut(&mut receiver,source.borrow(),&mut bank,BODY,None).unwrap().expect("uncut receive publishes");assert_eq!(serde_json::to_value(*output).unwrap(),row["value"]);driver::retire(output,&mut bank);}
  }
  driver::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());drop(receiver);driver::close_source(source,&mut bank);
 }
}
