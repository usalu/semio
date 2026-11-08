use super::*;
use crate::{DslValue,OriginalReceivingPrimary,shared_issuance::SealedShared};
use semio_framework_value_testing_admission::{self as fixture,BankWallet,EffectBank};
#[derive(crate::RetireOwned,crate::NativeReceive)]#[value(crate="crate",native_owned="crate")]struct Row{value:SealedShared<String>}
const BODY:usize=1<<16;
type Owner=NativeReceivingOriginal<DslValue,Row>;
type Turn=RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>;
type Close=RetainedWorkOutcome<RetainedCloneStep,crate::OriginalRetirementFailure>;
fn stock()->fixture::Wallet{fixture::Wallet::from_profile(&serde_json::from_str::<serde_json::Value>(include_str!("🎟️work/📐️.json")).unwrap())}
fn scratch()->EffectBank{EffectBank::new(BankWallet::from_profile(&serde_json::json!({"items":1u64<<40,"copyBytes":1u64<<40,"capacityBytes":1u64<<40,"releaseBytes":1u64<<40,"depth":1u64<<20,"fuel":1u64<<60})))}
fn observe<R>(run:impl FnOnce()->R)->(R,(usize,usize)){semio_framework_value_testing_allocator::observe_retirement_allocations(run)}
fn quoted(wallet:&fixture::Wallet,d:RetirementDemand)->RetainedCloneGrant{assert!(d.copy_bytes<=BODY);let loan=wallet.loan(1,BODY);assert!(retained_clone_grant_admits(loan,d));RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth,..loan}}
fn settled<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("synchronous original fixture turn stayed pending")}}
fn progress<E:RetainedProgressOf>(turn:&RetainedWorkOutcome<RetainedCloneStep,E>)->RetainedCloneProgress{match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.progress_of()}}
trait RetainedProgressOf{fn progress_of(&self)->RetainedCloneProgress;}
impl RetainedProgressOf for OriginalReceivingPrimary{fn progress_of(&self)->RetainedCloneProgress{self.retained_progress()}}
impl RetainedProgressOf for crate::OriginalRetirementFailure{fn progress_of(&self)->RetainedCloneProgress{self.retained_progress()}}
fn input<T:RetireOwned+Sync>(value:T,wallet:&mut fixture::Wallet)->RetainedCloneSource<T>{fixture::capture(value,wallet,|wallet|wallet.loan(1,BODY),|wallet|wallet.work_remaining(),|wallet,p,work,heap,g|wallet.post(p,u128::from(work),g,heap))}
fn row_source(wallet:&mut fixture::Wallet)->RetainedCloneSource<DslValue>{input(DslValue::Object(vec![("value".into(),DslValue::String("Original receiving owner🧱".into()))]),wallet)}
fn advance<I:Sync+'static,T:NativeReceive<I>>(owner:&mut NativeReceivingOriginal<I,T>,control:&mut NativeDecodeControl<'_>,wallet:&fixture::Wallet,grant:RetainedCloneGrant)->(Turn,(usize,usize)) where I:RetireOwned{
 let fuel=wallet.work_remaining();observe(||fixture::receive(control,grant,wallet.work_loan(fuel),|control|settled(owner.advance(control,BODY,fuel,grant))))
}
fn take<I:Sync+'static,T:NativeReceive<I>>(owner:&mut NativeReceivingOriginal<I,T>,bank:&mut EffectBank,grant:RetainedCloneGrant,output:&mut Option<T>)->(Turn,(usize,usize)) where I:RetireOwned{
 let((poll,_),heap)=observe(||bank.run(grant,1,|effect|owner.take(output,effect)));assert_ne!(bank.take_admitted(),Some(false));(settled(poll),heap)
}
fn close<I:Sync+'static,T:NativeReceive<I>>(owner:&mut NativeReceivingOriginal<I,T>,bank:&mut EffectBank,grant:RetainedCloneGrant)->(Close,(usize,usize)) where I:RetireOwned{
 let quote=owner.close_work_demand(BODY).unwrap();let((poll,actual),heap)=observe(||bank.run(grant,quote,|effect|owner.close_step(effect)));assert_ne!(bank.take_admitted(),Some(false));if let Some(bytes)=actual{assert_eq!(bytes,heap.0)}else{assert_eq!(heap.0,0)}(settled(poll),heap)
}
fn close_all<I:Sync+'static,T:NativeReceive<I>>(owner:&mut NativeReceivingOriginal<I,T>,wallet:&mut fixture::Wallet,bank:&mut EffectBank,released:&mut Vec<usize>) where I:RetireOwned{
 owner.begin_close();
 for _ in 0..10000{
  if owner.terminal_is_empty(){return}
  let d=owner.close_demands(BODY).unwrap();let g=quoted(wallet,d);
  for denied in fixture::denials(g,d){let(turn,heap)=close(owner,bank,denied);assert_eq!(turn.work,0);assert_eq!(progress(&turn),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.close_demands(BODY).unwrap(),d);}
  let(turn,heap)=close(owner,bank,g);let p=progress(&turn);wallet.post(p,turn.work,g,heap);released.push(p.released_bytes);turn.result.unwrap();
 }
 panic!("original receiving owner did not close in its turn bound")
}
#[test]fn native_receiving_original_keeps_cancellation_and_denied_transfers_owned(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();assert_eq!(law["closeOrder"],serde_json::json!(["receiver","output","original-source","frame"]));let mut finished=0;
 for cut in 0..128{
  let mut wallet=stock();let mut bank=scratch();let source=row_source(&mut wallet);let source_pointer=std::ptr::from_ref(source.borrow().get());let(mut owner,heap)=observe(||Owner::new(source));assert_eq!(heap,(0,0));assert_eq!(std::ptr::from_ref(owner.borrow_input().unwrap().get()),source_pointer);let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut callback);let mut turns=0;
  while turns<cut&&!owner.output_ready(){
   let fuel=wallet.work_remaining();let d=owner.demands(BODY,fuel).unwrap();let g=quoted(&wallet,d);
   for denied in fixture::denials(g,d){let(turn,heap)=advance(&mut owner,&mut control,&wallet,denied);assert_eq!(turn.work,0);assert_eq!(progress(&turn),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.demands(BODY,fuel).unwrap(),d);}
   let(turn,heap)=advance(&mut owner,&mut control,&wallet,g);let p=progress(&turn);wallet.post(p,turn.work,g,heap);turn.result.unwrap();assert_eq!(std::ptr::from_ref(owner.borrow_input().unwrap().get()),source_pointer);turns+=1;
  }
  if owner.output_ready(){
   finished=turns;let d=owner.take_demand().unwrap();let g=quoted(&wallet,d);
   for denied in fixture::denials(g,d){let mut denied_output=None;let(turn,heap)=take(&mut owner,&mut bank,denied,&mut denied_output);assert_eq!(turn.work,0);assert_eq!(progress(&turn),Default::default());assert!(denied_output.is_none());assert_eq!(heap,(0,0));assert!(owner.output_ready());}
   if cut%2==0{let mut taken=None;let(turn,heap)=take(&mut owner,&mut bank,g,&mut taken);let p=progress(&turn);wallet.post(p,turn.work,g,heap);turn.result.unwrap();let row=taken.expect("funded publication fills the caller slot");assert_eq!(row.value.as_str(),"Original receiving owner🧱");fixture::close_owned(row,&mut wallet,BODY,10000);}
  }
  close_all(&mut owner,&mut wallet,&mut bank,&mut Vec::new());assert!(owner.terminal_is_empty());if finished>0&&cut>finished{break}
 }
 assert!(finished>0);
}
#[test]fn native_receiving_original_shell_requires_funded_input_and_receiver_custody(){
 let mut wallet=stock();let mut bank=scratch();let source=row_source(&mut wallet);let pointer=std::ptr::from_ref(source.borrow().get());let(mut owner,heap)=observe(||Owner::new(source));assert_eq!(heap,(0,0));assert_eq!(std::ptr::from_ref(owner.borrow_input().unwrap().get()),pointer);assert!(!owner.receiver.reserved());let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut callback);
 for stage in 0..2{
  let fuel=wallet.work_remaining();let d=owner.demands(BODY,fuel).unwrap();let g=quoted(&wallet,d);
  for denied in fixture::denials(g,d){let(turn,heap)=advance(&mut owner,&mut control,&wallet,denied);assert_eq!(turn.work,0);assert_eq!(progress(&turn),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.demands(BODY,fuel).unwrap(),d);}
  let(turn,heap)=advance(&mut owner,&mut control,&wallet,g);let p=progress(&turn);assert_eq!(turn.work,1);wallet.post(p,turn.work,g,heap);turn.result.unwrap();
  assert!(owner.receiver.reserved());assert_eq!(owner.receiver.initialized(),stage==1);assert_eq!(p.retained_capacity_bytes,heap.0);
  assert_eq!(std::ptr::from_ref(owner.borrow_input().unwrap().get()),pointer);
 }
 close_all(&mut owner,&mut wallet,&mut bank,&mut Vec::new());assert!(owner.terminal_is_empty());
}
#[path="../🪞️projection/🧪️tests/🦀️.rs"]mod projection_laws;
#[test]fn native_persistent_list_original_vector_owner_fits_quantum_and_keeps_custody(){
 use semio_framework_value_persistent::list::persistent::PersistentList;
 type ListOwner=NativeReceivingOriginal<Vec<DslValue>,PersistentList<Row>>;
 for cut in [Some(0usize),Some(1),Some(8),Some(32),None]{
  let mut wallet=stock();let mut bank=scratch();let rows=(0..17).map(|index|DslValue::Object(vec![("value".into(),DslValue::String(format!("α🙂{index}")))])).collect::<Vec<_>>();let source=input(rows,&mut wallet);let pointer=std::ptr::from_ref(source.borrow().get());let(mut owner,heap)=observe(||ListOwner::new(source));assert_eq!(heap,(0,0));let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<30,&mut callback);let mut turns=0;
  while !owner.output_ready()&&cut.is_none_or(|cut|turns<cut){assert!(turns<4096,"original receiving stalled");let fuel=wallet.work_remaining();let d=owner.demands(BODY,fuel).unwrap();let g=quoted(&wallet,d);let(turn,heap)=advance(&mut owner,&mut control,&wallet,g);let p=progress(&turn);wallet.post(p,turn.work,g,heap);turn.result.unwrap();assert_eq!(std::ptr::from_ref(owner.borrow_input().unwrap().get()),pointer);turns+=1;}
  let mut output=None;
  if cut.is_none(){assert!(owner.output_ready());let g=quoted(&wallet,owner.take_demand().unwrap());let mut taken=None;let(turn,heap)=take(&mut owner,&mut bank,g,&mut taken);let p=progress(&turn);wallet.post(p,turn.work,g,heap);turn.result.unwrap();let value=taken.expect("funded publication fills the caller slot");assert_eq!(value.len(),17);for index in 0..17{assert_eq!(value.get(index).unwrap().value.as_str(),format!("α🙂{index}"));}output=Some(value);}
  close_all(&mut owner,&mut wallet,&mut bank,&mut Vec::new());assert!(owner.terminal_is_empty());if let Some(output)=output{fixture::close_owned(output,&mut wallet,BODY,100000);}
 }
}
