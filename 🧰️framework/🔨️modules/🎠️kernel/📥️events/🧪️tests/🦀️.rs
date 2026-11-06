//! 📥️ Every inbound Event is received from a genuine original source and retired through its finite caller.
use semio_framework::kernel::Event;
use semio_framework_value::{DslValue,NativeDecodeControl,NativeReceive,NativeReceiver,OriginalReceivingPrimary,RetainedCloneGrant,RetainedCloneProgress,ValueError};
use semio_framework_value_testing_admission as harness;
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
#[test]fn original_event_receive_matches_serde_and_closes_every_partial_owner(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for(row,valid)in law["cases"].as_array().unwrap().iter().map(|row|(row,true)).chain(law["invalid"].as_array().unwrap().iter().map(|row|(row,false))){for cut in std::iter::once(None).chain(law["cancelCuts"].as_array().unwrap().iter().map(|n|Some(n.as_u64().unwrap()as usize))){
  let mut bank=harness::Wallet::from_profile(&law["stock"]);let initial=bank.remaining();let(input,seed)=observe(||DslValue::from(row["input"].clone()));
  let source=harness::capture(input,&mut bank,|b|b.loan(1,8192),|b,p,w,h,g|b.post(p,w,g,h));
  let(mut receiver,h)=observe(<Event as NativeReceive<DslValue>>::receiver);assert_eq!(h,(0,0));let mut callback=|_|true;let mut control=NativeDecodeControl::new(67108864,&mut callback);let mut failure=None;
  for _ in 0..cut.unwrap_or(100000){if receiver.output_ready(){break}let demand=receiver.demands(source.borrow(),8192).unwrap();let g=bank.loan(1,8192);
   for denied in harness::denials(g,demand){let(turn,h)=observe(||harness::receive(&mut control,denied,bank.work_loan(1),|cx|receiver.advance(source.borrow(),cx,8192,denied)));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(h,(0,0));}
   let(turn,h)=observe(||harness::receive(&mut control,g,bank.work_loan(1),|cx|receiver.advance(source.borrow(),cx,8192,g)));if let Err(error)=harness::post_step(&mut bank,turn,g,h,OriginalReceivingPrimary::retained_progress){failure=Some(error);break}
  }
  let mut output=None;if cut.is_none()&&!valid{assert!(failure.is_some(),"{}",row["variant"]);assert!(serde_json::from_value::<Event>(row["input"].clone()).is_err());}if cut.is_none()&&valid{assert!(receiver.output_ready(),"{}",row["variant"]);let g=bank.loan(1,8192);let(turn,h)=observe(||receiver.take(g));let p=turn.result.as_ref().unwrap().as_ref().unwrap().1;bank.post(p,turn.work,g,h);let(value,_)=turn.result.unwrap().unwrap();let oracle:Event=serde_json::from_value(row["input"].clone()).unwrap();assert_eq!(value,oracle);assert_eq!(serde_json::to_value(&value).unwrap(),row["input"]);output=Some(value);}
  receiver.begin_close();for _ in 0..100000{if receiver.terminal_is_empty(){break}let d=receiver.close_demands(8192).unwrap();let g=bank.loan(1,8192);for denied in harness::denials(g,d){let(turn,h)=observe(||receiver.close_step(denied));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(h,(0,0));}let(turn,h)=observe(||receiver.close_step(g));harness::post_step(&mut bank,turn,g,h,ValueError::retained_progress).unwrap();}
  assert!(receiver.terminal_is_empty());assert_eq!(observe(||drop(receiver)).1,(0,0));if let Some(failure)=failure{harness::close_owned(failure,&mut bank,8192,100000)}harness::close_owned(source,&mut bank,8192,100000);if let Some(value)=output{harness::close_owned(value,&mut bank,8192,100000)}let left=bank.remaining();assert_eq!(initial.maximum_capacity_bytes-left.maximum_capacity_bytes+seed.0,initial.maximum_release_bytes-left.maximum_release_bytes+seed.1);
 }}
}
