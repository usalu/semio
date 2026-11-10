use super::*;
use crate::{DslValue,OriginalReceivingPrimary,shared_issuance::SealedShared};
use semio_framework_value_testing_admission::{self as fixture,BankWallet,EffectBank};
#[derive(crate::RetireOwned,crate::NativeReceive)]#[value(crate="crate",native_owned="crate")]struct Row{value:SealedShared<String>}
const BODY:usize=1<<16;
type Owner=NativeReceivingOriginal<DslValue,Row>;
type Turn=RetainedWorkOutcome<RetainedCloneStep,crate::native_decoding::NativeReceivingFailure>;
type Taken=RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>;
type Close=RetainedWorkOutcome<RetainedCloneStep,crate::OriginalRetirementFailure>;
fn stock()->fixture::Wallet{fixture::Wallet::from_profile(&serde_json::from_str::<serde_json::Value>(include_str!("🎟️work/📐️.json")).unwrap())}
fn scratch()->EffectBank{EffectBank::new(BankWallet::from_profile(&serde_json::json!({"items":1u64<<40,"copyBytes":1u64<<40,"capacityBytes":1u64<<40,"releaseBytes":1u64<<40,"depth":1u64<<20,"fuel":1u64<<60})))}
fn observe<R>(run:impl FnOnce()->R)->(R,(usize,usize)){semio_framework_value_testing_allocator::observe_retirement_allocations(run)}
fn quoted(wallet:&fixture::Wallet,d:RetirementDemand)->RetainedCloneGrant{assert!(d.copy_bytes<=BODY);let loan=wallet.loan(1,BODY);assert!(retained_clone_grant_admits(loan,d));RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth,..loan}}
fn settled<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("synchronous original fixture turn stayed pending")}}
fn progress<E:RetainedProgressOf>(turn:&RetainedWorkOutcome<RetainedCloneStep,E>)->RetainedCloneProgress{match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.progress_of()}}
trait RetainedProgressOf{fn progress_of(&self)->RetainedCloneProgress;}
impl RetainedProgressOf for OriginalReceivingPrimary{fn progress_of(&self)->RetainedCloneProgress{self.retained_progress()}}
impl RetainedProgressOf for crate::native_decoding::NativeReceivingFailure{fn progress_of(&self)->RetainedCloneProgress{self.retained_progress()}}
impl RetainedProgressOf for crate::OriginalRetirementFailure{fn progress_of(&self)->RetainedCloneProgress{self.retained_progress()}}
struct ConstructionTurn{original:std::sync::Arc<std::sync::Mutex<RetainedCloneGrant>>,accepted:std::sync::Arc<std::sync::atomic::AtomicBool>,submitted:RetainedCloneProgress,turn:crate::OriginalSourceTurnIdentity}
impl crate::native_decoding::OriginalConstructionRecipient for ConstructionTurn{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<crate::OriginalSourceTurnIdentity>where 'source:'turn{Some(self.turn)}
 fn grant<'a,'turn,'source>(&mut self,_:&mut (),quote:crate::OriginalConstructionGrant<'a>)->crate::OriginalConstructionGrantComplete<'a>where 'source:'turn{quote.complete(Ok(*self.original.lock().unwrap()))}
 fn accept<'a,'turn,'source>(&mut self,_:&mut (),receipt:crate::OriginalConstructionAccept<'a>)->crate::OriginalConstructionAcceptComplete<'a>where 'source:'turn{let p=receipt.progress();self.accepted.store(true,std::sync::atomic::Ordering::SeqCst);let result=self.submitted.checked_add(p).map(|sum|self.submitted=sum);receipt.complete(result)}
 fn submitted<'turn,'source>(&self,_:&())->RetainedCloneProgress where 'source:'turn{self.submitted}
}
struct ConstructionWork{turn:crate::OriginalSourceTurnIdentity}
impl crate::native_decoding::OriginalWorkRecipient for ConstructionWork{type Context<'turn,'source>=() where 'source:'turn;fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}fn source_turn<'turn,'source>(&self,_:&())->Option<crate::OriginalSourceTurnIdentity>where 'source:'turn{Some(self.turn)}fn settle_partial<'a,'turn,'source>(&mut self,_:&mut (),check:crate::native_decoding::OriginalWorkPartialCheck<'a>)->crate::native_decoding::OriginalWorkPartialCheckComplete<'a>where 'source:'turn{check.complete(Ok(()))}fn recover_partial<'a,'turn,'source>(&mut self,_:&mut (),check:crate::native_decoding::OriginalWorkPartialCheck<'a>)->crate::native_decoding::OriginalWorkPartialCheckComplete<'a>where 'source:'turn{check.complete(Ok(()))}fn admits<'a,'turn,'source>(&mut self,_:&mut (),quote:crate::native_decoding::OriginalWorkQuote<'a>,_:u64,_:u64)->crate::native_decoding::OriginalWorkQuoteComplete<'a>where 'source:'turn{quote.complete(Ok(true))}}
struct Authorities{turn:crate::OriginalSourceTurn,construction:crate::native_decoding::OriginalConstructionAuthority<ConstructionTurn>,work:crate::native_decoding::OriginalWorkAuthority<ConstructionWork>,grant:std::sync::Arc<std::sync::Mutex<RetainedCloneGrant>>,accepted:std::sync::Arc<std::sync::atomic::AtomicBool>}
impl Authorities{
 fn new()->Self{let issuer=crate::OriginalSourceTurnIssuer::new();let turn=issuer.issue().expect("fixture Source dispatch");let identity=turn.identity();let grant=std::sync::Arc::new(std::sync::Mutex::new(RetainedCloneGrant::default()));let accepted=std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));drop(grant.lock().unwrap());Self{turn,construction:crate::native_decoding::OriginalConstructionAuthority::new(ConstructionTurn{original:grant.clone(),accepted:accepted.clone(),submitted:Default::default(),turn:identity}),work:crate::native_decoding::OriginalWorkAuthority::new(ConstructionWork{turn:identity}),grant,accepted}}
 fn effect<R>(&mut self,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant,work:u64,f:impl FnOnce(&mut crate::RetainedAllocationEffect<'_>)->R)->Option<R>{
  *self.grant.lock().unwrap()=grant;self.accepted.store(false,std::sync::atomic::Ordering::SeqCst);
  self.construction.with_loan(&mut (),|loan|self.work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|control.with_effect(grant,work,|_,effect|f(effect)))))).unwrap().unwrap().unwrap()
 }
 fn receive<R>(&mut self,control:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant,f:impl FnOnce(&mut NativeDecodeControl<'_>,crate::OriginalSourceTurnWitness<'_>)->RetainedWorkOutcome<R,crate::native_decoding::NativeReceivingFailure>)->RetainedWorkOutcome<R,crate::native_decoding::NativeReceivingFailure>{
  *self.grant.lock().unwrap()=g;self.accepted.store(false,std::sync::atomic::Ordering::SeqCst);let witness=self.turn.witness();let mut work=0;
  let result=self.construction.with_loan(&mut (),|loan|self.work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|{let turn=f(control,witness);work=turn.work;turn.result}))).unwrap()).unwrap();RetainedWorkOutcome{result,work}
 }
}
fn input<T:RetireOwned+Sync+'static>(value:T,bank:&mut EffectBank)->RetainedCloneSource<T>{let g=bank.loan(1,BODY);match fixture::admit_owned(bank,value,(),g).result{Ok((source,_))=>source,Err(_)=>panic!("fixture source admission refused")}}
struct CloseProducer<'o,T:RetireOwned>{owner:&'o mut crate::retirement::controlled::ControlledRetirement<T>,turn:crate::OriginalSourceTurnIdentity}
impl<T:RetireOwned> CloseProducer<'_,T>{fn owner(&self)->&crate::retirement::controlled::ControlledRetirement<T>{self.owner}}
impl<T:RetireOwned> crate::OriginalEffectProducer for CloseProducer<'_,T>{
 type Output=RetainedCloneStep;type Failure=crate::OriginalRetirementFailure;
 type Context<'turn,'source>=(usize,Option<&'turn mut crate::OriginalRetirementCloseLoan<'source>>) where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Option<crate::OriginalSourceTurnIdentity>where 'source:'turn{Some(self.turn)}
 fn receive<'r,'turn,'source>(&mut self,c:&mut Self::Context<'turn,'source>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<crate::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'source:'turn{match c.1.as_deref_mut(){Some(original)=>self.owner.step_original_into(original,c.0,effect,reply),None=>self.owner.step_into(c.0,effect,reply)}}
 fn recover<'r,'turn,'source>(&mut self,c:&mut Self::Context<'turn,'source>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<crate::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'source:'turn{match c.1.as_deref_mut(){Some(original)=>self.owner.recover_step_original_into(original,c.0,effect,reply),None=>self.owner.recover_step_into(c.0,effect,reply)}}
}
impl<T:RetireOwned> crate::OriginalEffectBirthProducer for CloseProducer<'_,T>{
 type Original<'loan>=crate::OriginalRetirementCloseLoan<'loan>;
 fn requires_original_birth(&self)->bool{self.owner.requires_original_birth()}
 fn close_original_birth<'turn,'source,'loan>(&mut self,_:&mut Self::Context<'turn,'source>,original:&mut Self::Original<'loan>,grant:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,Self::Failure>where 'source:'turn{self.owner.step_original_birth(original,grant)}
}
type Closer<'c,T>=OriginalEffectAuthority<CloseProducer<'c,T>>;
fn close_turn<T:RetireOwned>(authority:&mut Closer<'_,T>,payer:&mut Authorities,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant,work:u64)->(Result<RetainedCloneStep,crate::OriginalRetirementFailure>,u128,(usize,usize)){
 let mut action=None;let mut heap_sum=(0usize,0usize);let mut work_sum=0u128;
 for _ in 0..10000{
  let(done,heap)=observe(||payer.effect(control,grant,work,|effect|{let before=effect.performed_work();let ready=authority.drive(&mut (BODY,None),effect,&mut action).unwrap();(ready,effect.performed_work()-before)}).expect("close effect admitted"));
  heap_sum.0+=heap.0;heap_sum.1+=heap.1;work_sum+=done.1;if done.0{break}
 }
 let recorded=action.as_ref().unwrap().recorded_reply().unwrap().is_ok();
 let demand=Closer::<T>::transfer_reply_demand().unwrap();let transfer_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let mut output=None;
 payer.effect(control,transfer_grant,1,|effect|{let context=(BODY,None);authority.transfer_reply_into(&context,&mut action,&mut output,effect).result.unwrap().unwrap()}).expect("reply transfer admitted");
 let result=output.take().expect("transferred close reply");assert_eq!(result.is_ok(),recorded);
 (result,work_sum,heap_sum)
}
fn finish_closer<T:RetireOwned>(authority:&mut Option<Closer<'_,T>>,payer:&mut Authorities,control:&mut NativeDecodeControl<'_>){
 let demand=Closer::<T>::producer_transfer_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let mut producer=None;
 payer.effect(control,grant,1,|effect|{let context=(BODY,None);Closer::<T>::transfer_producer(authority,&context,&mut producer,effect).result.unwrap()}).expect("producer transfer admitted");
 producer.take();
}
fn close_owned<T:RetireOwned>(value:T,payer:&mut Authorities,control:&mut NativeDecodeControl<'_>){
 let mut cursor=crate::retirement::controlled::ControlledRetirement::new(value).map_err(|(error,_)|error).unwrap();let mut authority=Some(OriginalEffectAuthority::new(CloseProducer{owner:&mut cursor,turn:payer.turn.identity()}));
 for _ in 0..100000{let owner=authority.as_ref().unwrap().producer().owner();if owner.terminal_is_empty(){finish_closer(&mut authority,payer,control);return}let d=owner.next_demand(BODY).unwrap();let work=owner.next_computational_work(BODY).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:BODY,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};close_turn(authority.as_mut().unwrap(),payer,control,g,work).0.unwrap();}
 panic!("owned value did not close in its turn bound")
}
fn row_source(bank:&mut EffectBank)->RetainedCloneSource<DslValue>{input(DslValue::Object(vec![("value".into(),DslValue::String("Original receiving owner🧱".into()))]),bank)}
fn advance<I:Sync+'static,T:NativeReceive<I>>(owner:&mut NativeReceivingOriginal<I,T>,control:&mut NativeDecodeControl<'_>,payer:&mut Authorities,wallet:&fixture::Wallet,grant:RetainedCloneGrant)->(Turn,(usize,usize)) where I:RetireOwned{
 let fuel=wallet.remaining_work();observe(||payer.receive(control,grant,|control,witness|settled(owner.advance(control,witness,BODY,fuel,grant))))
}
fn settle_pending<I:Sync+'static,T:NativeReceive<I>>(owner:&mut NativeReceivingOriginal<I,T>,control:&mut NativeDecodeControl<'_>,payer:&mut Authorities,wallet:&mut fixture::Wallet) where I:RetireOwned{
 for _ in 0..4096{if !owner.advance_pending()||owner.output_ready(){return}let fuel=wallet.remaining_work();let d=owner.demands(BODY,fuel).unwrap();let g=quoted(wallet,d);let(turn,heap)=advance(owner,control,payer,wallet,g);let p=progress(&turn);wallet.post(p,turn.work,g,heap);turn.result.unwrap();}
 panic!("original receiving action did not settle")
}
fn take<I:Sync+'static,T:NativeReceive<I>>(owner:&mut NativeReceivingOriginal<I,T>,bank:&mut EffectBank,grant:RetainedCloneGrant,output:&mut Option<T>)->(Taken,(usize,usize)) where I:RetireOwned{
 let((poll,_),heap)=observe(||bank.run(grant,1,|effect|owner.take(output,effect)));assert_ne!(bank.take_admitted(),Some(false));(settled(poll),heap)
}
fn close_all<I:Sync+'static,T:NativeReceive<I>>(owner:NativeReceivingOriginal<I,T>,wallet:&mut fixture::Wallet,payer:&mut Authorities,control:&mut NativeDecodeControl<'_>,released:&mut Vec<usize>) where I:RetireOwned{
 let mut cursor=crate::retirement::controlled::ControlledRetirement::new(owner).map_err(|(error,_)|error).unwrap();let mut authority=Some(OriginalEffectAuthority::new(CloseProducer{owner:&mut cursor,turn:payer.turn.identity()}));
 for _ in 0..10000{
  let owner=authority.as_ref().unwrap().producer().owner();if owner.terminal_is_empty(){finish_closer(&mut authority,payer,control);return}
  let d=owner.next_demand(BODY).unwrap();let work=owner.next_computational_work(BODY).unwrap();let g=quoted(wallet,d);
  for denied in fixture::denials(g,d){let(result,spent,heap)=close_turn(authority.as_mut().unwrap(),payer,control,denied,work);assert_eq!(spent,0);assert_eq!(heap,(0,0));assert_eq!(result.unwrap().progress(),Default::default());assert_eq!(authority.as_ref().unwrap().producer().owner().next_demand(BODY).unwrap(),d);}
  let(result,spent,heap)=close_turn(authority.as_mut().unwrap(),payer,control,g,work);let p=result.unwrap().progress();wallet.post(p,spent,g,heap);released.push(p.released_bytes);
 }
 panic!("original receiving owner did not close in its turn bound")
}
#[test]fn native_receiving_original_keeps_cancellation_and_denied_transfers_owned(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();assert_eq!(law["closeOrder"],serde_json::json!(["receiver","output","original-source","frame"]));let mut finished=0;
 for cut in 0..law["cutBound"].as_u64().unwrap() as usize{
  let mut wallet=stock();let mut bank=scratch();let source=row_source(&mut bank);let source_pointer=std::ptr::from_ref(source.borrow().get());let(mut owner,heap)=observe(||Owner::new(source));assert_eq!(heap,(0,0));assert_eq!(std::ptr::from_ref(owner.borrow_input().unwrap().get()),source_pointer);let mut payer=Authorities::new();let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut callback);let mut turns=0;
  while turns<cut&&!owner.output_ready(){
   let fuel=wallet.remaining_work();let d=owner.demands(BODY,fuel).unwrap();let g=quoted(&wallet,d);
   for denied in fixture::denials(g,d){let(turn,heap)=advance(&mut owner,&mut control,&mut payer,&wallet,denied);assert_eq!(turn.work,0);assert_eq!(progress(&turn),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.demands(BODY,fuel).unwrap(),d);}
   let(turn,heap)=advance(&mut owner,&mut control,&mut payer,&wallet,g);let p=progress(&turn);wallet.post(p,turn.work,g,heap);turn.result.unwrap();assert_eq!(std::ptr::from_ref(owner.borrow_input().unwrap().get()),source_pointer);turns+=1;
  }
  settle_pending(&mut owner,&mut control,&mut payer,&mut wallet);
  if owner.output_ready(){
   finished=turns;let d=owner.take_demand().unwrap();let g=quoted(&wallet,d);
   for denied in fixture::denials(g,d){let mut denied_output=None;let(turn,heap)=take(&mut owner,&mut bank,denied,&mut denied_output);assert_eq!(turn.work,0);assert_eq!(progress(&turn),Default::default());assert!(denied_output.is_none());assert_eq!(heap,(0,0));assert!(owner.output_ready());}
   if cut%2==0{let mut taken=None;let(turn,heap)=take(&mut owner,&mut bank,g,&mut taken);let p=progress(&turn);wallet.post(p,turn.work,g,heap);turn.result.unwrap();let row=taken.expect("funded publication fills the caller slot");assert_eq!(row.value.as_str(),"Original receiving owner🧱");close_owned(row,&mut payer,&mut control);}
  }
  close_all(owner,&mut wallet,&mut payer,&mut control,&mut Vec::new());if finished>0&&cut>finished{break}
 }
 assert!(finished>0);
}
#[test]fn native_receiving_original_shell_requires_funded_input_and_receiver_custody(){
 let mut wallet=stock();let mut bank=scratch();let source=row_source(&mut bank);let pointer=std::ptr::from_ref(source.borrow().get());let(mut owner,heap)=observe(||Owner::new(source));assert_eq!(heap,(0,0));assert_eq!(std::ptr::from_ref(owner.borrow_input().unwrap().get()),pointer);assert!(!owner.receiver.reserved());let mut payer=Authorities::new();let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut callback);
 for stage in 0..2{
  let fuel=wallet.remaining_work();let d=owner.demands(BODY,fuel).unwrap();let g=quoted(&wallet,d);
  for denied in fixture::denials(g,d){let(turn,heap)=advance(&mut owner,&mut control,&mut payer,&wallet,denied);assert_eq!(turn.work,0);assert_eq!(progress(&turn),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.demands(BODY,fuel).unwrap(),d);}
  let(turn,heap)=advance(&mut owner,&mut control,&mut payer,&wallet,g);let p=progress(&turn);assert_eq!(turn.work,1);wallet.post(p,turn.work,g,heap);turn.result.unwrap();
  assert!(owner.receiver.reserved());assert_eq!(owner.receiver.initialized(),stage==1);assert_eq!(p.retained_capacity_bytes,heap.0);
  assert_eq!(std::ptr::from_ref(owner.borrow_input().unwrap().get()),pointer);
 }
 close_all(owner,&mut wallet,&mut payer,&mut control,&mut Vec::new());
}
#[test]fn native_persistent_list_original_vector_owner_batches_actions_under_a_wide_grant(){
 use semio_framework_value_persistent::list::persistent::PersistentList;
 type ListOwner=NativeReceivingOriginal<Vec<DslValue>,PersistentList<Row>>;
 let law=serde_json::from_str::<serde_json::Value>(include_str!("🧫️fixtures/🔣️.json")).unwrap()["wideGrant"].clone();let rows=law["rows"].as_u64().unwrap() as usize;
 let mut wallet=fixture::Wallet::from_profile(&law["stock"]);let mut bank=scratch();
 let source=input(list_rows(rows,None),&mut bank);let(mut owner,heap)=observe(||ListOwner::new(source));assert_eq!(heap,(0,0));let mut payer=Authorities::new();let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<30,&mut callback);let mut turns=0u64;let mut actions=0u64;
 while !owner.output_ready(){assert!(turns<law["maximumTurns"].as_u64().unwrap(),"{rows} rows needed more than {turns} wide turns");let g=wallet.loan(law["turnItems"].as_u64().unwrap() as usize,law["turnCopyBytes"].as_u64().unwrap() as usize);let(turn,heap)=advance(&mut owner,&mut control,&mut payer,&wallet,g);let p=progress(&turn);actions+=p.copied_items as u64;wallet.post(p,turn.work,g,heap);turn.result.unwrap();turns+=1;settle_pending(&mut owner,&mut control,&mut payer,&mut wallet);}
 assert!(actions>=1000*turns,"{actions} actions in {turns} turns: the wide grant did not batch");let g=quoted(&wallet,owner.take_demand().unwrap());let mut taken=None;let(turn,heap)=take(&mut owner,&mut bank,g,&mut taken);let p=progress(&turn);wallet.post(p,turn.work,g,heap);turn.result.unwrap();let value=taken.expect("funded publication fills the caller slot");assert_eq!(value.len(),rows);
 close_all(owner,&mut wallet,&mut payer,&mut control,&mut Vec::new());close_owned(value,&mut payer,&mut control);
}
#[path="../🪞️projection/🧪️tests/🦀️.rs"]mod projection_laws;
#[test]fn native_persistent_list_original_vector_owner_fits_quantum_and_keeps_custody(){
 use semio_framework_value_persistent::list::persistent::PersistentList;
 type ListOwner=NativeReceivingOriginal<Vec<DslValue>,PersistentList<Row>>;
 for cut in [Some(0usize),Some(1),Some(8),Some(32),None]{
  let mut wallet=stock();let mut bank=scratch();let rows=(0..17).map(|index|DslValue::Object(vec![("value".into(),DslValue::String(format!("α🙂{index}")))])).collect::<Vec<_>>();let source=input(rows,&mut bank);let pointer=std::ptr::from_ref(source.borrow().get());let(mut owner,heap)=observe(||ListOwner::new(source));assert_eq!(heap,(0,0));let mut payer=Authorities::new();let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<30,&mut callback);let mut turns=0;
  while !owner.output_ready()&&cut.is_none_or(|cut|turns<cut){assert!(turns<serde_json::from_str::<serde_json::Value>(include_str!("🧫️fixtures/🔣️.json")).unwrap()["turnBound"].as_u64().unwrap() as usize,"original receiving stalled");let fuel=wallet.remaining_work();let d=owner.demands(BODY,fuel).unwrap();let g=quoted(&wallet,d);let(turn,heap)=advance(&mut owner,&mut control,&mut payer,&wallet,g);let p=progress(&turn);wallet.post(p,turn.work,g,heap);turn.result.unwrap();assert_eq!(std::ptr::from_ref(owner.borrow_input().unwrap().get()),pointer);turns+=1;}
  settle_pending(&mut owner,&mut control,&mut payer,&mut wallet);
  let mut output=None;
  if cut.is_none(){assert!(owner.output_ready());let g=quoted(&wallet,owner.take_demand().unwrap());let mut taken=None;let(turn,heap)=take(&mut owner,&mut bank,g,&mut taken);let p=progress(&turn);wallet.post(p,turn.work,g,heap);turn.result.unwrap();let value=taken.expect("funded publication fills the caller slot");assert_eq!(value.len(),17);for index in 0..17{assert_eq!(value.get(index).unwrap().value.as_str(),format!("α🙂{index}"));}output=Some(value);}
  close_all(owner,&mut wallet,&mut payer,&mut control,&mut Vec::new());if let Some(output)=output{close_owned(output,&mut payer,&mut control);}
 }
}

fn project_child(source:&RetainedCloneSource<DslValue>,index:Option<usize>,wallet:&mut fixture::Wallet,bank:&mut EffectBank)->crate::retained_clone::RetainedOwnedProjection<DslValue>{
 use crate::retained_clone::RetainedOwnedProjectionAdmission;
 let target=||{let root=source.borrow();match index{Some(index)=>root.project(index,move|value|match value{DslValue::Array(rows)=>&rows[index],value=>value}),None=>root}};
 let mut capture=RetainedOwnedProjectionAdmission::<DslValue>::default();let mut reply=None;
 let demand=capture.demand(target()).unwrap();let work=capture.work_demand(target()).unwrap();let g=quoted(wallet,demand);
 let(poll,heap)=observe(||bank.run(g,work,|effect|{let before=effect.performed_work();drop(capture.admit(target(),effect,crate::OriginalEffectReply::borrow(&mut reply).unwrap()).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:reply.take().unwrap(),work:effect.performed_work()-before})}).0);assert_ne!(bank.take_admitted(),Some(false));let turn:RetainedWorkOutcome<RetainedCloneStep,ValueError>=settled(poll);wallet.post(turn.result.unwrap().progress(),turn.work,g,heap);
 let demand=capture.take_demand().unwrap();let work=capture.take_work_demand().unwrap();let g=quoted(wallet,demand);let mut output=None;
 let(poll,heap)=observe(||bank.run(g,work,|effect|{let before=effect.performed_work();drop(capture.take(&mut output,effect,crate::OriginalEffectReply::borrow(&mut reply).unwrap()).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:reply.take().unwrap(),work:effect.performed_work()-before})}).0);assert_ne!(bank.take_admitted(),Some(false));let turn:RetainedWorkOutcome<RetainedCloneStep,ValueError>=settled(poll);wallet.post(turn.result.unwrap().progress(),turn.work,g,heap);
 assert!(capture.terminal_is_empty());output.expect("projection published")
}

type ListRows=NativeReceivingOriginal<Vec<DslValue>,semio_framework_value_persistent::list::persistent::PersistentList<Row>>;
fn list_rows(count:usize,bad:Option<usize>)->Vec<DslValue>{(0..count).map(|index|DslValue::Object(vec![("value".into(),if bad==Some(index){DslValue::Array(Vec::new())}else{DslValue::String(format!("α🙂{index}"))})])).collect()}
fn refuse_list(rows:Vec<DslValue>,allow:Option<usize>)->String{
 let mut wallet=stock();let mut bank=scratch();let source=input(rows,&mut bank);let before=format!("{:?}",source.borrow().get());let pointer=std::ptr::from_ref(source.borrow().get());
 let(mut owner,heap)=observe(||ListRows::new(source));assert_eq!(heap,(0,0));let mut payer=Authorities::new();let calls=std::cell::Cell::new(0usize);let open=std::cell::Cell::new(false);let mut callback=|_|{let n=calls.get();calls.set(n+1);open.get()||allow.is_none_or(|allow|n<allow)};let mut control=NativeDecodeControl::new(1<<30,&mut callback);
 let bound=serde_json::from_str::<serde_json::Value>(include_str!("🧫️fixtures/🔣️.json")).unwrap()["turnBound"].as_u64().unwrap() as usize;let mut refusal=None;
 for _ in 0..bound{if owner.output_ready(){break}let fuel=wallet.remaining_work();let d=owner.demands(BODY,fuel).unwrap();let g=quoted(&wallet,d);let(turn,heap)=advance(&mut owner,&mut control,&mut payer,&wallet,g);let p=progress(&turn);wallet.post(p,turn.work,g,heap);if let Err(error)=turn.result{refusal=Some(format!("{error:?}"));break}assert_eq!(std::ptr::from_ref(owner.borrow_input().unwrap().get()),pointer);}
 let refusal=refusal.expect("the authored refusal must stop the receiving turn");open.set(true);settle_pending(&mut owner,&mut control,&mut payer,&mut wallet);assert_eq!(format!("{:?}",owner.borrow_input().unwrap().get()),before);
 close_all(owner,&mut wallet,&mut payer,&mut control,&mut Vec::new());refusal
}
#[test]fn native_persistent_list_later_row_refusal_retains_original_partial_owner(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../📋️list/🧪️tests/🧫️fixtures/📐️.json")).unwrap();
 for bad in law["invalidRows"].as_array().unwrap().iter().map(|bad|bad.as_u64().unwrap() as usize).filter(|bad|*bad<17){let refusal=refuse_list(list_rows(17.max(bad+1),Some(bad)),None);assert!(refusal.contains("InvalidValue"),"row {bad}: {refusal}");}
}
#[test]fn native_persistent_list_original_callback_refusal_preserves_partial_receiving(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../📋️list/🧪️tests/🧫️fixtures/📐️.json")).unwrap();
 for allow in [0usize,1,3,8,31,256]{let refusal=refuse_list(list_rows(17,None),Some(allow));assert!(refusal.contains("Canceled"),"allow {allow}: {refusal}");}
 assert_eq!(law["version"],1);
}
