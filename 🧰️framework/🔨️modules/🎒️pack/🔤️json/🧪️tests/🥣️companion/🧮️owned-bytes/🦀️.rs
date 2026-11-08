//! 🧮️ Every receiving turn charges exactly the heap it requested to the decode control, per receiver family.
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneSource};
use semio_framework_value::{DslValue,Number,RetainedAllocationEffect,RetainedWorkOutcome};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeReceive,OriginalReceivingPrimary};
use semio_framework_value_native_receiving_original::NativeReceivingOriginal;
use semio_framework_value_testing_admission::{capture,receive,BankWallet,EffectBank,RetainedWorkPoll,Wallet,WorkLoan};
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;

const WORK:u64=1<<20;
fn profile()->serde_json::Value{serde_json::json!({"items":1u64<<30,"copyBytes":1u64<<40,"capacityBytes":1u64<<40,"releaseBytes":1u64<<40,"depth":1u64<<20,"fuel":1u64<<60})}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64}}
fn source(value:DslValue)->RetainedCloneSource<DslValue>{
let mut wallet=Wallet::from_profile(&profile());
capture(value,&mut wallet,|wallet|wallet.loan(1,4096),|wallet|wallet.work_remaining(),|wallet,progress,work,heap,grant|wallet.post(progress,u128::from(work),grant,heap))
}
fn advance(owner:&mut NativeReceivingOriginal<DslValue,impl NativeReceive<DslValue>>,control:&mut NativeDecodeControl<'_>)->Result<(),OriginalReceivingPrimary>{
let remaining=u128::from(WORK)<<20;
let turn:RetainedWorkOutcome<(),OriginalReceivingPrimary>=receive(control,grant(),WorkLoan::new(&remaining,WORK),|control|match owner.advance(control,4096,WORK,grant()){RetainedWorkPoll::Complete(turn)=>RetainedWorkOutcome{result:turn.result.map(|_|()),work:turn.work},RetainedWorkPoll::Pending{..}=>panic!("receiving turn stayed pending")});
turn.result
}
fn close<T,E>(bank:&mut EffectBank,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<T,E>)->Result<T,E>{
let(poll,_)=bank.run(grant(),WORK,operation);
assert_ne!(bank.take_admitted(),Some(false),"the original Work ledger refused the close turn");
match poll{RetainedWorkPoll::Complete(turn)=>turn.result,RetainedWorkPoll::Pending{..}=>panic!("synchronous close stayed pending")}
}
fn law<T:NativeReceive<DslValue>+Sync>(value:DslValue){
let mut owner=NativeReceivingOriginal::<DslValue,T>::new(source(value));
let mut accept=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut accept);
for turn in 0..10000{
if owner.output_ready(){break}
let before=control.owned_bytes();
let(result,heap)=observe(||advance(&mut owner,&mut control));
result.unwrap_or_else(|error|panic!("receiving turn {turn} refused: {}",error.message()));
assert_eq!(control.owned_bytes()-before,heap.requested_bytes,"turn {turn} charged a different byte count than it allocated");
}
assert!(owner.output_ready(),"receiving did not finish");
let mut bank=EffectBank::new(BankWallet::from_profile(&profile()));
let mut taken=None;close(&mut bank,|effect|owner.take(&mut taken,effect)).ok().expect("funded take");let output=taken.expect("taken output");
let mut retirement=ControlledRetirement::new(output).unwrap_or_else(|_|panic!("output has controlled retirement"));
for _ in 0..10000{if retirement.terminal_is_empty(){break}close(&mut bank,|effect|retirement.step(effect)).ok().expect("output close step");}
owner.begin_close();
for _ in 0..10000{if owner.terminal_is_empty(){break}close(&mut bank,|effect|owner.close_step(effect)).ok().expect("receiver close step");}
assert!(owner.terminal_is_empty());
}
fn text(value:&str)->DslValue{DslValue::String(value.into())}
fn object(entries:Vec<(&str,DslValue)>)->DslValue{DslValue::Object(entries.into_iter().map(|(key,value)|(key.to_string(),value)).collect())}

#[derive(semio_framework_value::RetireOwned,semio_framework_value::NativeReceive)]
struct Record{first:String,second:String}
#[derive(semio_framework_value::RetireOwned,semio_framework_value::NativeReceive)]
enum Choice{Named{first:String},Other}

#[test]fn scalar_text_receiving_charges_its_allocations(){law::<String>(text("text"));}
#[test]fn sequence_receiving_charges_its_allocations(){law::<Vec<String>>(DslValue::Array(vec![text("a"),text("bc")]));}
#[test]fn bulk_sequence_receiving_charges_its_allocations(){law::<Vec<u64>>(DslValue::Array(vec![DslValue::Number(Number::UInt(1)),DslValue::Number(Number::UInt(2))]));}
#[test]fn tuple_receiving_charges_its_allocations(){law::<(String,String)>(DslValue::Array(vec![text("a"),text("b")]));}
#[test]fn array_receiving_charges_its_allocations(){law::<[String;2]>(DslValue::Array(vec![text("a"),text("b")]));}
#[test]fn persistent_list_receiving_charges_its_allocations(){law::<semio_framework_value_persistent::list::persistent::PersistentList<String>>(DslValue::Array(vec![text("a"),text("bc"),text("def")]));}
#[test]fn derived_record_receiving_charges_its_allocations(){law::<Record>(object(vec![("first",text("a")),("second",text("b"))]));}
#[test]fn derived_variant_receiving_charges_its_allocations(){law::<Choice>(object(vec![("Named",object(vec![("first",text("a"))]))]));}
