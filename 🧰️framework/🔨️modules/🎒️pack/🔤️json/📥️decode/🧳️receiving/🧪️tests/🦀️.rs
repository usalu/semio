use super::*;
use semio_framework_value::{retirement::RetireOwned,ArtifactOwnedValueRetirementFactory,DslValue,ErasedSnapshotRetirement,OwnedValueRetirementFactory,RetainedAllocationEffect,RetainedWorkOutcome,RetirementDemand,ValueError,factory_retire_owned,serde};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource,retained_clone_grant_admits};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeDecodeRetirementRecipient,NativeReceive};
use semio_framework_value_testing_admission::{capture,BankWallet,EffectBank,OriginalConstructionAccept,OriginalConstructionAcceptComplete,OriginalConstructionAuthority,OriginalConstructionGrant,OriginalConstructionGrantComplete,OriginalConstructionRecipient,OriginalSourceTurnIdentity,OriginalWorkAuthority,RetainedWorkPoll,Wallet,WorkLoan};
use std::sync::Arc;

const WORK:u64=1<<20;
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:8192,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}}
fn funded(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.max(4096),maximum_capacity_bytes:d.capacity_bytes.max(4096),maximum_release_bytes:d.release_bytes.max(4096),maximum_depth:d.depth.max(64),..grant()}}
fn profile()->serde_json::Value{serde_json::json!({"items":1u64<<30,"copyBytes":1u64<<40,"capacityBytes":1u64<<40,"releaseBytes":1u64<<40,"depth":1u64<<20,"fuel":1u64<<60})}
fn bank()->EffectBank{EffectBank::new(BankWallet::from_profile(&profile()))}
fn complete<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("synchronous original turn stayed pending")}}
fn run_poll<T,E>(bank:&mut EffectBank,g:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{let(poll,_)=bank.run(g,WORK,operation);assert_ne!(bank.take_admitted(),Some(false),"the original Work ledger refused the turn");complete(poll)}
fn run_outcome<T,E>(bank:&mut EffectBank,g:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<T,E>)->RetainedWorkOutcome<T,E>{run_poll(bank,g,|effect|RetainedWorkPoll::Complete(operation(effect)))}

struct TurnConstruction(RetainedCloneGrant,RetainedCloneProgress);
impl OriginalConstructionRecipient for TurnConstruction{
type Context<'turn,'source>=() where 'source:'turn;
fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError> where 'source:'turn{Ok(())}
fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity> where 'source:'turn{None}
fn grant<'a,'turn,'source>(&mut self,_:&mut (),quote:OriginalConstructionGrant<'a>)->OriginalConstructionGrantComplete<'a> where 'source:'turn{
if !retained_clone_grant_admits(self.0,quote.demand()){return quote.complete(Err(ValueError::invariant("original JSON test turn does not admit construction demand")));}
quote.complete(Ok(self.0))
}
fn accept<'a,'turn,'source>(&mut self,_:&mut (),receipt:OriginalConstructionAccept<'a>)->OriginalConstructionAcceptComplete<'a> where 'source:'turn{
let(g,p)=(receipt.grant(),receipt.progress());
let posted=self.1.checked_add(p).and_then(|total|{self.1=total;if g.maximum_items!=self.0.maximum_items||g.maximum_copy_bytes>self.0.maximum_copy_bytes||g.maximum_capacity_bytes>self.0.maximum_capacity_bytes||g.maximum_release_bytes>self.0.maximum_release_bytes||g.maximum_depth>self.0.maximum_depth||!p.fits(g){return Err(ValueError::invariant("original JSON construction receipt changed its original turn"));}Ok(())});
receipt.complete(posted)
}
fn submitted<'turn,'source>(&self,_:&())->RetainedCloneProgress where 'source:'turn{self.1}
}
fn turn_construction(g:RetainedCloneGrant)->OriginalConstructionAuthority<TurnConstruction>{OriginalConstructionAuthority::new(TurnConstruction(g,Default::default()))}
fn framed<R>(control:&mut NativeDecodeControl<'_>,construction:&mut OriginalConstructionAuthority<TurnConstruction>,work:&mut OriginalWorkAuthority<WorkLoan<'_>>,apply:impl FnOnce(&mut NativeDecodeControl<'_>)->R)->R{
work.with_loan(&mut(),|work_loan|construction.with_loan(&mut(),|construction_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(construction_loan,|control|Ok::<R,ValueError>(apply(control)))))).unwrap().unwrap().unwrap()
}
fn admitted<T,E:std::fmt::Debug,W>(result:Result<T,(E,W)>)->T{match result{Ok(value)=>value,Err((error,wire))=>{std::mem::forget(wire);panic!("original JSON admission refused: {error:?}")}}}
fn actual<R>(run:impl FnOnce()->R)->(R,semio_framework_trace::HeapAllocationObservation){semio_framework_trace::observe_heap_allocations_on_this_thread(run)}
fn receipt(p:RetainedCloneProgress,heap:semio_framework_trace::HeapAllocationObservation,g:RetainedCloneGrant){assert!(p.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));}
fn source<T:RetireOwned+Sync>(value:T)->RetainedCloneSource<T>{
let mut wallet=Wallet::from_profile(&profile());
capture(value,&mut wallet,|wallet|wallet.loan(1,1<<20),|wallet|wallet.work_remaining(),|wallet,progress,work,heap,grant|wallet.post(progress,u128::from(work),grant,heap))
}
fn close_source<T:RetireOwned+Sync>(source:&mut RetainedCloneSource<T>,bank:&mut EffectBank){
for _ in 0..10000{if source.terminal_is_empty(){return;}let g=funded(source.next_close_demand(4096).unwrap());let(turn,heap)=actual(||run_poll(bank,g,|effect|source.close_step(effect)));let step=turn.result.ok().expect("original JSON source close step");receipt(step.progress(),heap,g);}
panic!("original JSON source close stalled")
}
fn by_shape(row:&serde_json::Value,cut:Option<usize>,owned_cut:Option<Option<usize>>)->usize{if row["value"].is_string(){schedule_mode::<String>(row,cut,owned_cut)}else if row["value"].is_array(){schedule_mode::<Vec<String>>(row,cut,owned_cut)}else{schedule_mode::<u64>(row,cut,owned_cut)}}
fn schedule_mode<M>(row:&serde_json::Value,cut:Option<usize>,owned_cut:Option<Option<usize>>)->usize where M:NativeReceive<DslValue>+ArtifactCanonicalJsonTree+Sync+std::fmt::Debug+serde::Serialize{
let bytes=row["wire"].as_str().unwrap().as_bytes().to_vec();let pointer=bytes.as_ptr();let mut bank=bank();let mut root=source(bytes);let g=grant();
let wire=run_outcome(&mut bank,g,|effect|root.retain_original(effect)).result.unwrap().0;assert_eq!(wire.borrow().unwrap().get().as_ptr(),pointer);
let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<M>>=Arc::new(OwnedValueRetirementFactory::<M>::default());
let remaining=1u128<<60;let mut work=OriginalWorkAuthority::new(WorkLoan::new(&remaining,WORK));
let callbacks=std::cell::Cell::new(0usize);let mut accept=|_|{callbacks.set(callbacks.get()+1);true};
let mut original_retirement=NativeDecodeRetirementRecipient::new();let mut control=NativeDecodeControl::new_retained(&mut accept);control.install_retirement_recipient(&mut original_retirement).unwrap();
let shell=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes.max(OriginalJsonReceiving::<M>::constructor_demand().copy_bytes),..g};let(mut owner,p)=admitted(OriginalJsonReceiving::<M>::admit(wire,&factory,if row["header"]=="op"{b"op"}else{b""},|value|value,&control,shell));assert!(p.fits(shell));
let mut turns=0;let mut output=None;let mut owned=None;let mut faulted=false;let body=g.maximum_copy_bytes;
for _ in 0..100000{
if cut==Some(turns){break;}let g=RetainedCloneGrant{maximum_copy_bytes:body.max(owner.demands(body).unwrap().copy_bytes),..g};
let d=owner.demands(body).unwrap();
let mut foreign_accept=|_|true;let mut foreign=NativeDecodeControl::new_retained(&mut foreign_accept);
let(result,heap)=actual(||complete(owner.advance(&mut foreign,body,WORK,g)).result);
assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes,foreign.owned_bytes()),(0,0,0));
for axis in 0..5{
if axis==1&&d.copy_bytes==0||axis==2&&d.capacity_bytes==0||axis==3&&d.release_bytes==0{continue;}
let denied=match axis{0=>RetainedCloneGrant{maximum_items:0,..g},1=>RetainedCloneGrant{maximum_copy_bytes:0,..g},2=>RetainedCloneGrant{maximum_capacity_bytes:0,..g},3=>RetainedCloneGrant{maximum_release_bytes:0,..g},_=>RetainedCloneGrant{maximum_depth:0,..g}};
let admitted=control.owned_bytes();let calls=callbacks.get();let mut construction=turn_construction(denied);
let(step,heap)=actual(||framed(&mut control,&mut construction,&mut work,|control|complete(owner.advance(control,body,WORK,denied)).result.unwrap()));
receipt(step.progress(),heap,denied);assert_eq!(control.owned_bytes(),admitted);assert_eq!(callbacks.get(),calls);assert_eq!(step.progress(),Default::default());assert_eq!(owner.wire.as_ref().unwrap().borrow().unwrap().get().as_ptr(),pointer);
}
let admitted=control.owned_bytes();let mut construction=turn_construction(g);
let(result,heap)=actual(||framed(&mut control,&mut construction,&mut work,|control|complete(owner.advance(control,body,WORK,g)).result));
assert_eq!(control.owned_bytes()-admitted,heap.requested_bytes);
let p=match result{Ok(step)=>step.progress(),Err(error)=>{faulted=true;error.retained_progress()}};
receipt(p,heap,g);turns+=1;if faulted||owner.ready(){break;}
}
if cut.is_none(){
assert_eq!(faulted,!row["canonical"].as_bool().unwrap());
if !faulted{
assert!(owner.ready(),"original JSON receiving stalled case={} phase={:?} turns={} demand={:?}",row["name"],owner.phase,turns,owner.demands(body));
if let Some(cut)=owned_cut{
for at in 0..100000{
if cut==Some(at){break}
let funded=funded(owner.take_owned_demands(4096).unwrap());
let mut foreign_accept=|_|true;let mut foreign=NativeDecodeControl::new_retained(&mut foreign_accept);
let(turn,heap)=actual(||run_outcome(&mut bank,funded,|effect|owner.take_owned_into(&mut owned,&mut foreign,4096,effect)));
assert!(turn.result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
for denied in [RetainedCloneGrant{maximum_items:0,..funded},RetainedCloneGrant{maximum_depth:0,..funded}]{
let(turn,heap)=actual(||run_outcome(&mut bank,denied,|effect|owner.take_owned_into(&mut owned,&mut control,4096,effect)));
let step=turn.result.unwrap();receipt(step.progress(),heap,denied);assert_eq!(step.progress(),Default::default());assert!(owned.is_none())
}
let(turn,heap)=actual(||run_outcome(&mut bank,funded,|effect|owner.take_owned_into(&mut owned,&mut control,4096,effect)));
let step=turn.result.unwrap();receipt(step.progress(),heap,funded);if owned.is_some(){break}
}
if cut.is_none(){assert_eq!(serde_json::to_value(owned.as_ref().expect("unique original handoff must complete")).unwrap(),row["value"]);}
}else{
let(result,heap)=actual(||owner.take(g).result.unwrap());let(value,p)=result.unwrap();receipt(p,heap,g);
assert_eq!(serde_json::to_value(value.borrow().unwrap().get()).unwrap(),row["value"]);output=Some(value);
}
}
}
if let Some(value)=owned.take(){let mut owned=source(value);close_source(&mut owned,&mut bank)}
owner.begin_close();
if let Some(output)=output.as_mut(){
for _ in 0..10000{
if output.terminal_is_empty(){break;}
let g=funded(output.close_demand(4096).unwrap());let(turn,heap)=actual(||run_poll(&mut bank,g,|effect|output.close_step(effect)));let step=turn.result.ok().expect("original JSON output close step");receipt(step.progress(),heap,g);
}
assert!(output.terminal_is_empty());
}
for _ in 0..100000{
if owner.terminal_is_empty(){break;}
let g=funded(owner.close_demands(4096).unwrap());let admitted=control.owned_bytes();
let(turn,heap)=actual(||control.with_retirement(g,|retirement|Ok::<_,ValueError>(run_poll(&mut bank,g,|effect|owner.close_step(retirement,effect)))).unwrap());
let step=turn.result.ok().expect("original JSON owner close step");assert_eq!(control.owned_bytes()-admitted,heap.requested_bytes);receipt(step.progress(),heap,g);
}
assert!(owner.terminal_is_empty());let((),heap)=actual(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
close_source(&mut root,&mut bank);drop(control);assert!(original_retirement.terminal_is_empty());turns
}
#[test]
fn original_json_receiving_keeps_native_source_and_exact_physical_custody(){
let corpus:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();
for row in corpus["cases"].as_array().unwrap(){let turns=by_shape(row,None,None);for cut in[0,1,2,turns/2,turns.saturating_sub(1)]{by_shape(row,Some(cut),None);}}
}

#[test]
fn original_json_owned_terminal_transfers_one_native_owner_without_clone(){
let mut bank=bank();let mut bytes=source(b"op\"original-native-value\"".to_vec());let g=grant();
let(wire,_)=run_outcome(&mut bank,g,|effect|bytes.retain_original(effect)).result.unwrap();
let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<String>>=Arc::new(OwnedValueRetirementFactory::<String>::default());
let remaining=1u128<<60;let mut work=OriginalWorkAuthority::new(WorkLoan::new(&remaining,WORK));
let mut accept=|_|true;let mut recipient=NativeDecodeRetirementRecipient::new();let mut c=NativeDecodeControl::new_retained(&mut accept);c.install_retirement_recipient(&mut recipient).unwrap();
let shell=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes.max(OriginalJsonReceiving::<String>::constructor_demand().copy_bytes),..g};let(mut owner,_)=admitted(OriginalJsonReceiving::<String>::admit(wire,&factory,b"op",|s|s,&c,shell));
for _ in 0..10000{if owner.ready(){break}let mut construction=turn_construction(g);framed(&mut c,&mut construction,&mut work,|c|complete(owner.advance(c,4096,WORK,g)).result).unwrap();}
assert!(owner.ready());let pointer=owner.output.as_ref().unwrap().borrow().unwrap().get().as_ptr();let mut original=None;
for _ in 0..10000{
let paid=funded(owner.take_original_demand(4096).unwrap());
let(turn,heap)=actual(||run_outcome(&mut bank,paid,|effect|owner.take_original(&mut c,4096,effect)));
let(step,value)=turn.result.unwrap();receipt(step.progress(),heap,paid);
if let Some((value,issuer))=value{assert_eq!(value.as_ptr(),pointer);assert!(Arc::ptr_eq(&issuer,&factory));original=Some((value,issuer));break;}
}
assert!(original.is_some());assert!(!owner.ready());
for _ in 0..10000{
if owner.terminal_is_empty(){break}
let paid=funded(owner.close_demands(4096).unwrap());
let(turn,heap)=actual(||c.with_retirement(paid,|retirement|Ok::<_,ValueError>(run_poll(&mut bank,paid,|effect|owner.close_step(retirement,effect)))).unwrap());
receipt(turn.result.ok().expect("original JSON owner close step").progress(),heap,paid);
}
assert!(owner.terminal_is_empty());close_source(&mut bytes,&mut bank);
let(value,issuer)=original.take().unwrap();let mut slot=Some(value);let paid=funded(issuer.retire_owned_demands(slot.as_ref().unwrap()).unwrap());
let(mut retirement,_)=run_poll(&mut bank,paid,|effect|factory_retire_owned(&*issuer,&mut slot,effect)).result.ok().expect("typed factory retirement");
for _ in 0..10000{if retirement.terminal_is_empty(){break}let g=funded(retirement.next_demand(65536).unwrap());run_poll(&mut bank,g,|effect|retirement.close_step(effect)).result.ok().expect("typed retirement close step");}
assert!(retirement.terminal_is_empty());
}

#[test]
fn original_json_receiving_hands_back_unique_native_values_and_cancels_each_alias_close(){
let corpus:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();
for row in corpus["cases"].as_array().unwrap().iter().filter(|row|row["canonical"]==true){for cut in std::iter::once(None).chain(corpus["ownedHandoffCancelPrefixes"].as_array().unwrap().iter().map(|cut|Some(cut.as_u64().unwrap() as usize))){by_shape(row,None,Some(cut));}}
}
