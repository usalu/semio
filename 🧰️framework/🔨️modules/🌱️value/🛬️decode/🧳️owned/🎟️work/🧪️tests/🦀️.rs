use crate::*;
use crate::owned::tree;
use crate::{DslValue,Number};
use crate::retained_clone::RetainedCloneSource;
use crate::driver::{self,Trace};
use semio_framework_value_testing_admission::{EffectBank,Wallet};
const BODY:usize=4096;
fn turn(receiver:&mut tree::NativeScalarReceiver<DslValue,u64>,source:RetainedCloneRef<'_,DslValue>,bank:&mut EffectBank,trace:&mut Trace)->Result<(),String>{driver::receive_traced(receiver,source,bank,BODY,Some(1),None,trace).map(|published|assert!(published.is_none()))}
fn close(mut receiver:tree::NativeScalarReceiver<DslValue,u64>,source:RetainedCloneSource<DslValue>,bank:&mut EffectBank){driver::close(&mut receiver,bank,BODY);assert!(receiver.terminal_is_empty());drop(receiver);driver::close_source(source,bank);}
#[test]fn actual_native_scalar_denial_read_error_and_close_keep_independent_work(){
 let row:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();let mut bank=EffectBank::new(Wallet::from_profile(&row["stock"]));
 let before=bank.wallet().remaining_work();let source=driver::admit(&mut bank,DslValue::Number(Number::UInt(row["input"].as_u64().unwrap())));assert_eq!(before-bank.wallet().remaining_work(),row["sourceAdmissionWork"].as_u64().unwrap());
 let pointer=std::ptr::from_ref(source.borrow().get());let(mut receiver,heap)=crate::observe_retirement_allocations(tree::NativeScalarReceiver::<DslValue,u64>::new);assert_eq!(heap,(0,0));let mut trace=Trace::default();assert_eq!(row["deniedWork"].as_u64().unwrap(),0);
 for expected in["bindingWork","readWork"]{
  if expected=="readWork"{assert_eq!(receiver.demands(source.borrow(),BODY,u64::MAX).unwrap().copy_bytes,std::mem::size_of::<Option<u64>>());assert_eq!(row["construction"],"optional-u64-placement");}
  turn(&mut receiver,source.borrow(),&mut bank,&mut trace).unwrap();assert_eq!(trace.advance.last().copied(),Some(u128::from(row[expected].as_u64().unwrap())),"{expected}");assert_eq!(std::ptr::from_ref(source.borrow().get()),pointer);
 }
 assert!(receiver.output_ready());let value=driver::receive_traced(&mut receiver,source.borrow(),&mut bank,BODY,None,None,&mut trace).unwrap().expect("uncut receive publishes");assert_eq!(serde_json::to_value(value).unwrap(),row["input"]);
 close(receiver,source,&mut bank);
 let source=driver::admit(&mut bank,DslValue::Bool(false));let mut receiver=tree::NativeScalarReceiver::<DslValue,u64>::new();let mut trace=Trace::default();
 turn(&mut receiver,source.borrow(),&mut bank,&mut trace).unwrap();let refused=turn(&mut receiver,source.borrow(),&mut bank,&mut trace).unwrap_err();
 assert_eq!(trace.advance.last().copied(),Some(u128::from(row["invalidReadWork"].as_u64().unwrap())));assert!(refused.contains("InvalidValue"),"{refused}");assert!(!receiver.output_ready());
 close(receiver,source,&mut bank);
}
