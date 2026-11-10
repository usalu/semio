use super::*;
use crate::{DslValue,Number};
use crate::driver;
use semio_framework_value_testing_admission::{EffectBank,Wallet};
const BODY:usize=4096;
fn bank()->EffectBank{let f:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🌳️value/👥️tuple/🧪️tests/🧫️fixtures/📐️.json")).unwrap();EffectBank::new(Wallet::from_profile(&f["stock"]))}
fn original_prefixes<T:NativeReceive<DslValue>+crate::retirement::RetireOwned>(input:impl Fn()->DslValue,check:impl Fn(&T))where <T as NativeReceive<DslValue>>::Receiver:NativeReceiver<DslValue,T>{
 let mut finished=0;
 for cut in 0..500{
  let mut bank=bank();let original=driver::admit(&mut bank,input());let foreign=driver::admit(&mut bank,input());let(mut r,h)=crate::observe_retirement_allocations(T::receiver);assert_eq!(h,(0,0));
  for _ in 0..cut{if r.output_ready(){break}driver::none(driver::receive_cut(&mut r,original.borrow(),&mut bank,BODY,Some(1))).unwrap();if !r.output_ready(){assert!(r.demands(foreign.borrow(),BODY,1<<20).is_err())}}
  if r.output_ready(){let v=driver::receive_cut(&mut r,original.borrow(),&mut bank,BODY,None).unwrap().expect("uncut receive publishes");check(&v);driver::retire(v,&mut bank);finished=cut;}
  driver::close(&mut r,&mut bank,BODY);assert!(r.terminal_is_empty());drop(r);driver::close_source(original,&mut bank);driver::close_source(foreign,&mut bank);if finished>0{break}
 }
 assert!(finished>0);
}
#[test]fn original_heterogeneous_tuple_receiving_preserves_every_paid_prefix_and_wire_kind(){
 original_prefixes::<(u32,String)>(||DslValue::Array(vec![DslValue::Number(Number::UInt(17)),DslValue::String("β".into())]),|v|assert_eq!(v,&(17,"β".into())));
 original_prefixes::<(u16,u8,u8)>(||DslValue::Array(vec![DslValue::Number(Number::UInt(2024)),DslValue::Number(Number::UInt(2)),DslValue::Number(Number::UInt(29))]),|v|assert_eq!(v,&(2024,2,29)));
 for count in [1,3]{
  let mut bank=bank();let original=driver::admit(&mut bank,DslValue::Array((0..count).map(|_|DslValue::Number(Number::UInt(17))).collect()));let mut r=<(u32,String)as NativeReceive<DslValue>>::receiver();let mut refused=false;
  for _ in 0..500{if r.output_ready(){break}if driver::receive_cut(&mut r,original.borrow(),&mut bank,BODY,Some(1)).is_err(){refused=true;break}}
  assert!(refused);assert!(!r.output_ready());driver::close(&mut r,&mut bank,BODY);assert!(r.terminal_is_empty());drop(r);driver::close_source(original,&mut bank);
 }
}
