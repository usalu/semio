//! ⚖️ Standard Result values use serde as the independent oracle and retain every partial owner.
use semio_framework_value_native_owned::*;
use semio_framework_value_native_owned::owned::NativeReceiver;
#[path="../../../../../🧪️testing/📥️admission/receiver-driver.rs"]
mod driver;
const BODY:usize=4096;
fn case(input:&serde_json::Value,valid:bool,cut:Option<u64>){
 let mut bank=driver::bank();let source=driver::admit(&mut bank,DslValue::from(input.clone()));let mut receiver=<Result<String,()>as owned::NativeReceive<DslValue>>::receiver();
 let mut refused=false;
 match cut{
  Some(cut)=>{let published=driver::receive_cut(&mut receiver,source.borrow(),&mut bank,BODY,Some(cut));assert!(matches!(published,Ok(None)|Err(_)),"{published:?}");}
  None=>{
   for _ in 0..100000{if receiver.output_ready(){break}if driver::receive_cut(&mut receiver,source.borrow(),&mut bank,BODY,Some(1)).is_err(){refused=true;break}}
   let expected=serde_json::from_value::<Result<String,()>>(input.clone());assert_eq!(expected.is_ok(),valid);assert_eq!(refused,expected.is_err());
   if let Ok(expected)=expected{
    assert!(receiver.output_ready());assert_eq!(receiver.take_demand().unwrap().copy_bytes,if expected.is_ok(){2*std::mem::size_of::<Option<String>>()}else{2*std::mem::size_of::<Option<()>>()});assert_eq!(receiver.take_work_demand().unwrap(),1);
    let actual=driver::receive_cut(&mut receiver,source.borrow(),&mut bank,BODY,None).unwrap().expect("uncut receive publishes");assert_eq!(actual,expected);driver::retire(actual,&mut bank);
   }
  }
 }
 driver::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());drop(receiver);driver::close_source(source,&mut bank);
}
#[test]
fn result_receives_standard_variants_and_closes_every_cancelled_partial_owner(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){for cut in std::iter::once(None).chain(law["cancelCuts"].as_array().unwrap().iter().map(|n|Some(n.as_u64().unwrap()))){
  case(&row["input"],row["valid"].as_bool().unwrap(),cut);
 }}
}
