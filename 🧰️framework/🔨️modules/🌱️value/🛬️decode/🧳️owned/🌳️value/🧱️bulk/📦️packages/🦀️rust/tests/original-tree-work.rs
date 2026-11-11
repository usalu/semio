//! 🌲️ Actual tree frontiers obey finite work loans and preserve every cancelled prefix.
use semio_framework_value_native_tree_engine::*;
use semio_framework_value_native_receiving_contract::NativeReceiver;
use semio_framework_value_native_decoding::native_decoding::NativeDecodeControl;
use semio_framework_value_control_contract::{RetainedCloneProgress,RetainedCloneStep,RetirementDemand};
use semio_framework_value_retained_clone::retained_clone::retained_clone_grant_admits;
use semio_framework_value_intrinsic::DslValue;
use semio_framework_value_testing_admission as source_fixture;
use source_fixture::Wallet;
fn observe<T>(f:impl FnOnce()->T)->(T,(usize,usize)){semio_framework_value_testing_allocator::observe_retirement_allocations(f)}
#[test]
fn actual_tree_zero_work_quotes_heap_receipts_and_cancelled_prefixes(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️work/🔣️.json")).unwrap();
 let oracle=serde_json::to_string(&fixture["input"]).unwrap();assert_eq!(serde_json::from_str::<serde_json::Value>(&oracle).unwrap(),fixture["input"]);
 for cut in std::iter::once(None).chain(fixture["cancelCuts"].as_array().unwrap().iter().map(|v|Some(v.as_u64().unwrap()))){
  let mut bank=Wallet::from_profile(&fixture["stock"]);let source=source_fixture::capture(DslValue::from(&fixture["input"]),&mut bank,|w|w.loan(1,4096),Wallet::work_remaining,|w,p,work,h,g|w.post(p,work,g,h));
  let(mut receiver,heap)=observe(NativeTreeReceiver::<DslValue,DslValue>::new);assert_eq!(heap,(0,0));
  let mut checkpoint=|_|true;let mut native=NativeDecodeControl::new(1<<24,&mut checkpoint);let mut complete=false;
  for turn in 0..10000u64{
   if receiver.output_ready(){complete=true;break}if cut.is_some_and(|cut|turn>=cut){break}
   for maximum in fixture["limits"].as_array().unwrap(){let maximum=maximum.as_u64().unwrap();let q=receiver.advance_work_demand(source.borrow(),4096,maximum).unwrap();assert!(q<=maximum)}
   let grant=bank.loan(1,4096);let before=bank.work_remaining();let(denied,heap)=observe(||source_fixture::receive(&mut native,grant,bank.work_loan(0),|native|receiver.advance(source.borrow(),native,4096,0,grant)));assert_eq!(denied.work,0);assert_eq!(denied.result.unwrap().progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(bank.work_remaining(),before);
   let maximum=128.min(bank.work_remaining());let q=receiver.advance_work_demand(source.borrow(),4096,maximum).unwrap();assert!(q>0&&q<=maximum);let demand=receiver.demands(source.borrow(),4096,maximum).unwrap();assert!(retained_clone_grant_admits(grant,demand));
   let(result,heap)=observe(||source_fixture::receive(&mut native,grant,bank.work_loan(maximum),|native|receiver.advance(source.borrow(),native,4096,maximum,grant)));assert!(result.work<=q);let p=result.result.as_ref().map(|s|s.progress()).unwrap_or_else(|e|e.retained_progress());bank.post(p,result.work,grant,heap);result.result.unwrap();
  }
  if cut.is_none(){assert!(complete);let grant=bank.loan(1,4096);let(turn,heap)=observe(||receiver.take(grant));let(value,p)=turn.result.unwrap().unwrap();bank.post(p,turn.work,grant,heap);assert_eq!(value,DslValue::from(&fixture["input"]));source_fixture::close_owned(value,&mut bank,4096,10000)}
  receiver.begin_close().unwrap();let mut actual=None;
  for _ in 0..10000{
   if actual.is_some(){let grant=bank.loan(1,4096);let demand=RetirementDemand{copy_bytes:std::mem::size_of::<Option<usize>>(),depth:1,..Default::default()};assert!(retained_clone_grant_admits(grant,demand));assert!(bank.work_remaining()>=1);actual=None;bank.post(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},1,grant,(0,0));continue}
   if receiver.terminal_is_empty(){break}let q=receiver.close_work_demand(4096).unwrap();assert!(q<=bank.work_remaining());let grant=bank.loan(1,4096);assert!(retained_clone_grant_admits(grant,receiver.close_demands(4096).unwrap()));let(turn,heap)=observe(||receiver.close_step(grant,&mut actual));assert!(turn.work<=q);if let Some(bytes)=actual{assert_eq!(bytes,heap.0)}else{assert_eq!(heap.0,0)}let p=turn.result.as_ref().map(|s|s.progress()).unwrap_or_else(|e|e.retained_progress());bank.post(p,turn.work,grant,heap);turn.result.unwrap();
  }
  assert!(actual.is_none());assert!(receiver.terminal_is_empty());assert_eq!(observe(||drop(receiver)).1,(0,0));source_fixture::close_owned(source,&mut bank,4096,10000);
 }
}
