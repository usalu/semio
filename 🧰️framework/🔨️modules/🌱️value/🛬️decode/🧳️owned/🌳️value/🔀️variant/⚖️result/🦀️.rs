//! ⚖️ The standard Result law uses serde as an independent oracle and pays for every partial owner.
use semio_framework_value_native_owned::*;
use semio_framework_value_testing_admission as harness;
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
use retained_clone::{RetainedCloneSource,retained_clone_grant_admits};
use retirement::{RetireOwned,controlled::ControlledRetirement};
fn post<E:std::fmt::Debug>(bank:&mut harness::Wallet,turn:RetainedWorkOutcome<RetainedCloneStep,E>,g:RetainedCloneGrant,h:(usize,usize),total:&mut(usize,usize),error:impl FnOnce(&E)->RetainedCloneProgress)->Result<RetainedCloneStep,E>{let p=turn.result.as_ref().map(|s|s.progress()).unwrap_or_else(error);assert_eq!(h,(p.retained_capacity_bytes,p.released_bytes));total.0+=h.0;total.1+=h.1;bank.post(p,turn.work,g,h);turn.result}
fn close<T:RetireOwned>(value:T,bank:&mut harness::Wallet,total:&mut(usize,usize)){let(mut owner,h)=observe(||ControlledRetirement::new(value).unwrap_or_else(|_|panic!("Result law retirement")));assert_eq!(h,(0,0));for _ in 0..100000{if owner.terminal_is_empty(){break}let g=bank.loan(1,4096);let(result,h)=observe(||owner.step(g));post(bank,RetainedWorkOutcome{result,work:owner.last_turn_work()},g,h,total,ValueError::retained_progress).unwrap();}assert!(owner.terminal_is_empty());assert_eq!(observe(||drop(owner)).1,(0,0));}
#[test]fn result_receives_standard_variants_and_closes_every_cancelled_partial_owner(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){for cut in std::iter::once(None).chain(law["cancelCuts"].as_array().unwrap().iter().map(|n|Some(n.as_u64().unwrap()as usize))){
  let mut bank=harness::Wallet::from_profile(&law["stock"]);let(input,mut total)=observe(||DslValue::from(row["input"].clone()));let mut wallet=(&mut bank,&mut total);
  let source:RetainedCloneSource<DslValue>=harness::capture(input,&mut wallet,|w|w.0.loan(1,4096),|w,p,work,h,g|{w.1.0+=h.0;w.1.1+=h.1;w.0.post(p,work,g,h)});
  let(mut receiver,h)=observe(<Result<String,()>as NativeReceive<DslValue>>::receiver);assert_eq!(h,(0,0));let mut callback=|_|true;let mut control=NativeDecodeControl::new(16777216,&mut callback);let mut refused=false;
  for _ in 0..cut.unwrap_or(100000){if receiver.output_ready(){break}let d=receiver.demands(source.borrow(),4096).unwrap();let g=bank.loan(1,4096);
   for denied in harness::denials(g,d){assert!(!retained_clone_grant_admits(denied,d));let(turn,h)=observe(||harness::receive(&mut control,denied,bank.work_loan(1),|cx|receiver.advance(source.borrow(),cx,4096,denied)));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(h,(0,0));}
   let(turn,h)=observe(||harness::receive(&mut control,g,bank.work_loan(1),|cx|receiver.advance(source.borrow(),cx,4096,g)));if post(&mut bank,turn,g,h,&mut total,OriginalReceivingPrimary::retained_progress).is_err(){refused=true;break}
  }
  let mut output=None;if cut.is_none(){let expected=serde_json::from_value::<Result<String,()>>(row["input"].clone());assert_eq!(expected.is_ok(),row["valid"].as_bool().unwrap());assert_eq!(refused,expected.is_err());if let Ok(expected)=expected{assert!(receiver.output_ready());let g=bank.loan(1,4096);let(turn,h)=observe(||receiver.take(g));let p=turn.result.as_ref().unwrap().as_ref().unwrap().1;total.0+=h.0;total.1+=h.1;bank.post(p,turn.work,g,h);let(actual,_)=turn.result.unwrap().unwrap();assert_eq!(actual,expected);output=Some(actual);}}
  receiver.begin_close();for _ in 0..100000{if receiver.terminal_is_empty(){break}let g=bank.loan(1,4096);let(turn,h)=observe(||receiver.close_step(g));post(&mut bank,turn,g,h,&mut total,ValueError::retained_progress).unwrap();}assert!(receiver.terminal_is_empty());assert_eq!(observe(||drop(receiver)).1,(0,0));close(source,&mut bank,&mut total);if let Some(output)=output{close(output,&mut bank,&mut total)}assert_eq!(total.0,total.1);
 }}
}
