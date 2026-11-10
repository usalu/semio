use crate::{DslValue,NativeReceive,NativeReceiver};
use crate::driver;
use semio_framework_value_testing_admission::{EffectBank,Wallet};
#[test]fn actual_fixed_array_receiving_has_no_per_element_birth_or_take_callback(){
 let f:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();
 fn case<const N:usize>(f:&serde_json::Value,values:Vec<bool>,cut:Option<u64>){
  let s=&f["stock"];let mut bank=EffectBank::new(Wallet::from_profile(s));let source=driver::admit(&mut bank,DslValue::Array(values.iter().copied().map(DslValue::Bool).collect()));
  let(mut receiver,heap)=crate::observe_retirement_allocations(||<[bool;N]as NativeReceive<DslValue>>::receiver());assert_eq!(heap,(0,0));
  match cut{
   Some(cut)=>driver::none(driver::receive_cut(&mut receiver,source.borrow(),&mut bank,4096,Some(cut))).unwrap(),
   None=>{
    while !receiver.output_ready(){driver::none(driver::receive_cut(&mut receiver,source.borrow(),&mut bank,4096,Some(1))).unwrap()}
    assert_eq!(receiver.take_demand().unwrap().copy_bytes,f["publicationFields"].as_array().unwrap().len()*std::mem::size_of::<Option<[bool;N]>>());assert_eq!(receiver.take_work_demand().unwrap(),1);
    let value=driver::receive_cut(&mut receiver,source.borrow(),&mut bank,4096,None).unwrap().expect("uncut receive publishes");
    assert_eq!(serde_json::to_value(value.as_slice()).unwrap(),serde_json::to_value(&values).unwrap());assert!(s["fuel"].as_u64().unwrap()-bank.wallet().remaining_work()>=N as u64);
    driver::retire(value,&mut bank);
   }
  }
  driver::close(&mut receiver,&mut bank,4096);assert!(receiver.terminal_is_empty());drop(receiver);driver::close_source(source,&mut bank);
 }
 case::<0>(&f,vec![],None);case::<3>(&f,serde_json::from_value(f["cases"][1]["values"].clone()).unwrap(),None);case::<4097>(&f,vec![true;4097],None);for cut in f["cancelCuts"].as_array().unwrap(){case::<3>(&f,vec![true,false,true],Some(cut.as_u64().unwrap()));}
}
