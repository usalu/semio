//! 🧪️ The original native wrapper publishes one independently admitted named root under its original ledger.
#![allow(dead_code)]
#[path="../../../../../🧪️tests/support.rs"]mod support;
#[path="../../../🧪️tests/helpers.rs"]mod helpers;
use helpers::{close_index,denies,heap,lookup,settle_reply,close_work_policy};
use support::effect;
/// 📏️ Authored retirement body: large enough for the fixed metadata of the nested sealed cursors plus one key page.
const BODY:usize=1<<16;
/// 📐️ Cross-language funding vectors fix the authored close body.
const LAW_BODY:usize=4096;
use semio_framework_value_native_receiving_original::NativeReceivingOriginal;
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value_control_contract::{OriginalEffectAuthority,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};
use semio_framework_value_edit::{AdmittedNamedKeyIndex,NamedPersistentRoot,PersistentRankedKeys,OriginalReceivingPrimary,OriginalRetirementFailure,OriginalRetirementCloseLoan,RetainedAllocationEffect,RetainedWorkOutcome};
use semio_framework_value_intrinsic::{DslValue,Number};
use semio_framework_value_native_decoding::NativeDecodeControl;
use semio_framework_value_native_owned::native_decoding;
use semio_framework_value_persistent::list::persistent::PersistentList;
use semio_framework_value_retained_clone::retained_clone::{self,RetainedOwnedProjection,RetainedOwnedProjectionAdmission,RetainedWorkPoll};
use semio_framework_value_retirement::{retirement::{RetireOwned,RetirementCursor,controlled::ControlledRetirement},RetirementDemand};
use semio_framework_value_shared_issuance::SealedShared;
#[derive(semio_framework_value_derive::RetireOwned,semio_framework_value_derive::NativeReceive)]#[value(crate="::semio_framework_value_native_owned",native_owned="::semio_framework_value_native_owned")]struct ReceivedRow{key:SealedShared<String>,payload:i64}
impl semio_framework_value_edit::NamedRecordKey for ReceivedRow{fn named_record_key(&self)->&str{self.key.as_str()}}
impl Clone for ReceivedRow{fn clone(&self)->Self{panic!("original named receiving cannot clone typed rows")}}

struct ConstructionTurn{original:std::sync::Arc<std::sync::Mutex<RetainedCloneGrant>>,accepted:std::sync::Arc<std::sync::atomic::AtomicBool>,submitted:RetainedCloneProgress,turn:semio_framework_value_control_contract::OriginalSourceTurnIdentity}
impl native_decoding::OriginalConstructionRecipient for ConstructionTurn{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<semio_framework_value_control_contract::OriginalSourceTurnIdentity>where 'source:'turn{Some(self.turn)}
 fn grant<'a,'turn,'source>(&mut self,_:&mut (),quote:semio_framework_value_control_contract::OriginalConstructionGrant<'a>)->semio_framework_value_control_contract::OriginalConstructionGrantComplete<'a>where 'source:'turn{quote.complete(Ok(*self.original.lock().unwrap()))}
 fn accept<'a,'turn,'source>(&mut self,_:&mut (),receipt:semio_framework_value_control_contract::OriginalConstructionAccept<'a>)->semio_framework_value_control_contract::OriginalConstructionAcceptComplete<'a>where 'source:'turn{let p=receipt.progress();self.accepted.store(true,std::sync::atomic::Ordering::SeqCst);let result=self.submitted.checked_add(p).map(|sum|self.submitted=sum);receipt.complete(result)}
 fn submitted<'turn,'source>(&self,_:&())->RetainedCloneProgress where 'source:'turn{self.submitted}
}
struct ConstructionWork{turn:semio_framework_value_control_contract::OriginalSourceTurnIdentity}
impl native_decoding::OriginalWorkRecipient for ConstructionWork{type Context<'turn,'source>=() where 'source:'turn;fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}fn source_turn<'turn,'source>(&self,_:&())->Option<semio_framework_value_control_contract::OriginalSourceTurnIdentity>where 'source:'turn{Some(self.turn)}fn settle_partial<'a,'turn,'source>(&mut self,_:&mut (),check:native_decoding::OriginalWorkPartialCheck<'a>)->native_decoding::OriginalWorkPartialCheckComplete<'a>where 'source:'turn{check.complete(Ok(()))}fn recover_partial<'a,'turn,'source>(&mut self,_:&mut (),check:native_decoding::OriginalWorkPartialCheck<'a>)->native_decoding::OriginalWorkPartialCheckComplete<'a>where 'source:'turn{check.complete(Ok(()))}fn admits<'a,'turn,'source>(&mut self,_:&mut (),quote:native_decoding::OriginalWorkQuote<'a>,_:u64,_:u64)->native_decoding::OriginalWorkQuoteComplete<'a>where 'source:'turn{quote.complete(Ok(true))}}
struct Authorities{turn:semio_framework_value_control_contract::OriginalSourceTurn,construction:native_decoding::OriginalConstructionAuthority<ConstructionTurn>,work:native_decoding::OriginalWorkAuthority<ConstructionWork>,grant:std::sync::Arc<std::sync::Mutex<RetainedCloneGrant>>,accepted:std::sync::Arc<std::sync::atomic::AtomicBool>}
impl Authorities{
 fn new()->Self{let issuer=semio_framework_value_control_contract::OriginalSourceTurnIssuer::new();let turn=issuer.issue().expect("fixture Source dispatch");let identity=turn.identity();let grant=std::sync::Arc::new(std::sync::Mutex::new(RetainedCloneGrant::default()));let accepted=std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));drop(grant.lock().unwrap());Self{turn,construction:native_decoding::OriginalConstructionAuthority::new(ConstructionTurn{original:grant.clone(),accepted:accepted.clone(),submitted:Default::default(),turn:identity}),work:native_decoding::OriginalWorkAuthority::new(ConstructionWork{turn:identity}),grant,accepted}}
 fn effect<R>(&mut self,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant,work:u64,f:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->Option<R>{
  *self.grant.lock().unwrap()=grant;self.accepted.store(false,std::sync::atomic::Ordering::SeqCst);
  self.construction.with_loan(&mut (),|loan|self.work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|control.with_effect(grant,work,|_,effect|f(effect)))))).unwrap().unwrap().unwrap()
 }
 fn receive<R>(&mut self,control:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant,f:impl FnOnce(&mut NativeDecodeControl<'_>,semio_framework_value_control_contract::OriginalSourceTurnWitness<'_>)->RetainedWorkOutcome<R,native_decoding::NativeReceivingFailure>)->RetainedWorkOutcome<R,native_decoding::NativeReceivingFailure>{
  *self.grant.lock().unwrap()=g;self.accepted.store(false,std::sync::atomic::Ordering::SeqCst);let witness=self.turn.witness();let mut work=0;
  let result=self.construction.with_loan(&mut (),|loan|self.work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|{let turn=f(control,witness);work=turn.work;turn.result}))).unwrap()).unwrap();RetainedWorkOutcome{result,work}
 }
}
struct CloseProducer<'o,T:RetireOwned>{owner:&'o mut ControlledRetirement<T>,turn:semio_framework_value_control_contract::OriginalSourceTurnIdentity}
impl<T:RetireOwned> CloseProducer<'_,T>{fn owner(&self)->&ControlledRetirement<T>{self.owner}}
impl<T:RetireOwned> semio_framework_value_control_contract::OriginalEffectProducer for CloseProducer<'_,T>{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;
 type Context<'turn,'source>=(usize,Option<&'turn mut OriginalRetirementCloseLoan<'source>>) where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Option<semio_framework_value_control_contract::OriginalSourceTurnIdentity>where 'source:'turn{Some(self.turn)}
 fn receive<'r,'turn,'source>(&mut self,c:&mut Self::Context<'turn,'source>,effect:&mut RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'source:'turn{match c.1.as_deref_mut(){Some(original)=>self.owner.step_original_into(original,c.0,effect,reply),None=>self.owner.step_into(c.0,effect,reply)}}
 fn recover<'r,'turn,'source>(&mut self,c:&mut Self::Context<'turn,'source>,effect:&mut RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'source:'turn{match c.1.as_deref_mut(){Some(original)=>self.owner.recover_step_original_into(original,c.0,effect,reply),None=>self.owner.recover_step_into(c.0,effect,reply)}}
}
impl<T:RetireOwned> semio_framework_value_control_contract::OriginalEffectBirthProducer for CloseProducer<'_,T>{
 type Original<'loan>=OriginalRetirementCloseLoan<'loan>;
 fn requires_original_birth(&self)->bool{self.owner.requires_original_birth()}
 fn close_original_birth<'turn,'source,'loan>(&mut self,_:&mut Self::Context<'turn,'source>,original:&mut Self::Original<'loan>,grant:RetainedCloneGrant)->retained_clone::RetainedWorkPoll<RetainedCloneStep,Self::Failure>where 'source:'turn{self.owner.step_original_birth(original,grant)}
}
type Closer<'c,T>=OriginalEffectAuthority<CloseProducer<'c,T>>;
fn close_turn<T:RetireOwned>(authority:&mut Closer<'_,T>,payer:&mut Authorities,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant,work:u64)->(Result<RetainedCloneStep,OriginalRetirementFailure>,u128,(usize,usize)){
 let mut action=None;let mut heap_sum=(0usize,0usize);let mut work_sum=0u128;
 for _ in 0..10000{
  let(done,heap)=heap(||payer.effect(control,grant,work,|effect|{let before=effect.performed_work();let ready=authority.drive(&mut (BODY,None),effect,&mut action).unwrap();(ready,effect.performed_work()-before)}).expect("close effect admitted"));
  heap_sum.0+=heap.0;heap_sum.1+=heap.1;work_sum+=done.1;if done.0{break}RECOVERED.fetch_add(1,std::sync::atomic::Ordering::SeqCst);
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
static RECOVERED:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
/// ♻️ Closes any owner through its actual original close authority, redriving every pending reply through its recovery entry and balancing the physical ledger.
fn close_retire<T:RetireOwned>(value:T,payer:&mut Authorities,control:&mut NativeDecodeControl<'_>,total:&mut(usize,usize),mut prepared:Option<&mut usize>){
 let mut cursor=ControlledRetirement::new(value).map_err(|(error,_)|error).unwrap();let mut authority=Some(OriginalEffectAuthority::new(CloseProducer{owner:&mut cursor,turn:payer.turn.identity()}));let mut close_work=close_work_policy::Work::new();
 for _ in 0..100000{
  let owner=authority.as_ref().unwrap().producer().owner();
  if owner.terminal_is_empty(){finish_closer(&mut authority,payer,control);drop(authority);assert_eq!(heap(||drop(cursor)).1,(0,0));return}
  let d=owner.next_demand(BODY).unwrap();assert!(d.copy_bytes<=BODY,"selected close quote exceeds its authored body: {d:?}");let work=owner.next_computational_work(BODY).unwrap();
  for denied in denies(d){let(result,spent,h)=close_turn(authority.as_mut().unwrap(),payer,control,denied,work);assert_eq!(spent,0);assert_eq!(h,(0,0));assert_eq!(result.unwrap().progress(),Default::default());assert_eq!(authority.as_ref().unwrap().producer().owner().next_demand(BODY).unwrap(),d);}
  let g=close_fund(d);if prepared.is_some(){control.charge(d.capacity_bytes).unwrap();}
  let(result,_,h)=close_turn(authority.as_mut().unwrap(),payer,control,g,work);let step=result.unwrap();account(step.progress(),g,h,total);
  if let Some(prepared)=prepared.as_deref_mut(){*prepared+=h.0;assert_eq!(control.owned_bytes(),*prepared);}
  let owner=authority.as_ref().unwrap().producer().owner();close_work.observe(step.progress()==Default::default()&&!owner.terminal_is_empty()&&owner.next_demand(BODY).unwrap()==d);
 }
 panic!("owner did not close in its turn bound")
}

fn fund(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:257,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn close_fund(d:RetirementDemand)->RetainedCloneGrant{close_fund_to(d,BODY)}
fn close_fund_to(d:RetirementDemand,body:usize)->RetainedCloneGrant{assert!(d.copy_bytes<=body,"selected close quote exceeds its authored body: {d:?}");RetainedCloneGrant{maximum_copy_bytes:body,..fund(d)}}
fn funding_vectors(f:&serde_json::Value){for row in f["copyFunding"].as_array().unwrap(){let d=RetirementDemand{copy_bytes:row["quotedCopy"].as_u64().unwrap()as usize,capacity_bytes:7,release_bytes:11,depth:3};let g=if row["purpose"]=="forward"{fund(d)}else{close_fund_to(d,LAW_BODY)};assert_eq!(g.maximum_copy_bytes,row["maximumCopy"].as_u64().unwrap()as usize);assert_eq!((g.maximum_capacity_bytes,g.maximum_release_bytes,g.maximum_depth),(d.capacity_bytes,d.release_bytes,d.depth));}}
/// 📤️ Takes the received root through one actual original effect and reply slot.
fn take_root(owner:&mut NativeReceivingOriginal<DslValue,NamedPersistentRoot<ReceivedRow>>,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<(NamedPersistentRoot<ReceivedRow>,RetainedCloneProgress)>,OriginalReceivingPrimary>{let mut output=None;let turn=effect::settle(g,|effect|owner.take(&mut output,effect));RetainedWorkOutcome{result:turn.result.map(|step|output.map(|root|(root,step.progress()))),work:turn.work}}
fn account(p:RetainedCloneProgress,g:RetainedCloneGrant,h:(usize,usize),total:&mut(usize,usize)){assert!(p.fits(g));assert_eq!(h,(p.retained_capacity_bytes,p.released_bytes));total.0+=h.0;total.1+=h.1;}
fn close_value<T:RetireOwned>(value:T,payer:&mut Authorities,control:&mut NativeDecodeControl<'_>,total:&mut(usize,usize)){close_retire(value,payer,control,total,None)}
fn run_received(keys:&[String],admitted:bool,cut:Option<usize>){
 type Owner=NativeReceivingOriginal<DslValue,NamedPersistentRoot<ReceivedRow>>;
 let(value,mut total)=heap(||DslValue::Array(keys.iter().enumerate().map(|(rank,key)|DslValue::Object(vec![("key".into(),DslValue::String(key.clone())),("payload".into(),DslValue::Number(Number::Int(rank as i64)))])).collect()));let mut original={let mut bank=helpers::fixture_bank();let request=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};let(admitted,progress)=semio_framework_value_clone_source_testing::admit_owned(&mut bank,value,(),request).result.unwrap_or_else(|_|panic!("fixture source admission refused"));total.0+=progress.retained_capacity_bytes;Some(admitted)};
 let pointer=std::ptr::from_ref(original.as_ref().unwrap().borrow().get());let expected=serde_json::Value::from(original.as_ref().unwrap().borrow().get());let mut owner=Owner::new(original.take().unwrap());assert!(original.is_none());assert_eq!(std::ptr::from_ref(owner.borrow_input().unwrap().get()),pointer);
 let mut payer=Authorities::new();let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<30,&mut callback);let mut prepared=0;assert_eq!(control.owned_bytes(),prepared);let mut turns=0;let mut refused=None;
 while !owner.output_ready()&&cut.is_none_or(|cut|turns<cut){let d=owner.demands(BODY,effect::WORK).unwrap();let g=fund(d);for denied in denies(d){let(s,h)=heap(||payer.receive(&mut control,denied,|control,witness|effect::settled(owner.advance(control,witness,BODY,effect::WORK,denied))).result.unwrap());assert_eq!(h,(0,0));assert_eq!(s.progress(),Default::default());assert_eq!(owner.demands(BODY,effect::WORK).unwrap(),d);}let(turn,h)=heap(||payer.receive(&mut control,g,|control,witness|effect::settled(owner.advance(control,witness,BODY,effect::WORK,g))));let result=turn.result;let p=result.as_ref().map(|s|s.progress()).unwrap_or_else(|e|e.retained_progress());account(p,g,h,&mut total);prepared+=h.0;assert_eq!(control.owned_bytes(),prepared,"turn {turns} progress {p:?} heap {h:?} result {:?}",result.as_ref().err().map(|e|format!("{e:?}")));assert_eq!(std::ptr::from_ref(owner.borrow_input().unwrap().get()),pointer);assert_eq!(serde_json::Value::from(owner.borrow_input().unwrap().get()),expected);turns+=1;assert!(turns<1000000);if let Err(e)=result{refused=Some(format!("{e:?} after {turns} turns, owned {}",control.owned_bytes()));assert!(!admitted&&matches!(&e,native_decoding::NativeReceivingFailure::Primary(p)|native_decoding::NativeReceivingFailure::ReceivingEffect(native_decoding::NativeReceivingEffectFailure{child:Some(p),..})if p.primary.as_ref().is_some_and(|p|p.kind==ValueRefusalKind::InvalidValue)),"{refused:?}");break;}}
 let mut output=None;if cut.is_none(){assert_eq!(owner.output_ready(),admitted,"{refused:?}");assert_eq!(refused.is_some(),!admitted,"{refused:?}");if admitted{let g=fund(owner.take_demand().unwrap());let((root,p),h)=heap(||take_root(&mut owner,g).result.unwrap().unwrap());account(p,g,h,&mut total);assert_eq!(root.len(),keys.len());let ranked=root.ranked().unwrap();for(rank,key)in keys.iter().enumerate(){let row=ranked.original.get().get(rank).unwrap();assert_eq!(row.key.as_str(),key);assert_eq!(row.payload,rank as i64);}output=Some(root);}}
 close_retire(owner,&mut payer,&mut control,&mut total,Some(&mut prepared));
 if let Some(root)=output{let d=root.project_index_demand().unwrap();let mut projected=None;for _ in 0..2{let(turn,h)=heap(||effect::settle(fund(d),|effect|root.project_index(&mut projected,effect)));let p=turn.result.unwrap().progress();account(p,fund(d),h,&mut total);}let mut alias=projected.unwrap();assert!(alias.ready());let ranked=root.ranked().unwrap();let original_key=ranked.key;let mut admission=RetainedOwnedProjectionAdmission::<PersistentList<ReceivedRow>>::default();let d=root.project_rows_demand(&admission).unwrap();for denied in denies(d){let(turn,h)=heap(||effect::settle(denied,|effect|settle_reply(effect,|effect,reply|root.project_rows_into(&mut admission,effect,reply))));assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(h,(0,0));assert!(admission.terminal_is_empty());}
 let(turn,h)=heap(||effect::settle(fund(d),|effect|settle_reply(effect,|effect,reply|root.project_rows_into(&mut admission,effect,reply))));let p=turn.result.unwrap().progress();account(p,fund(d),h,&mut total);
 let mut reader=None;let td=admission.take_demand().unwrap();let(turn,h)=heap(||effect::settle(fund(td),|effect|settle_reply(effect,|effect,reply|admission.take(&mut reader,effect,reply))));let p=turn.result.unwrap().progress();account(p,fund(td),h,&mut total);assert!(admission.terminal_is_empty());drop(admission);let mut reader=reader.unwrap();assert!(reader.matches_source(ranked.original));
 close_value(root,&mut payer,&mut control,&mut total);let live=PersistentRankedKeys{original:reader.borrow().unwrap(),key:original_key};for(rank,key)in keys.iter().enumerate(){assert_eq!(lookup(&live,&alias,key).0,Some(rank));}let(b,r)=close_index(&mut alias);total.0+=b;total.1+=r;let(b,r)=helpers::close_loop(&mut reader,BODY,true,|x|x.terminal_is_empty(),|x|x.close_demand(BODY).unwrap(),|x,recover,effect,reply|if recover{x.recover_close_into(BODY,effect,reply)}else{x.close_into(BODY,effect,reply)});total.0+=b;total.1+=r;assert_eq!(heap(||drop(reader)).1,(0,0));}
 assert_eq!(total.0,total.1,"original input, typed pages, admission directory and independent source must settle on the same physical ledger");
}
#[test]fn original_named_receiving_publishes_independent_roots_and_closes_every_paid_cut(){let f:serde_json::Value=serde_json::from_str(fixture_text!("../../🔑️named/📇️index/🌳️root/🛬️receive/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();funding_vectors(&f);for row in f["cases"].as_array().unwrap(){let keys=row["keys"].as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_owned()).collect::<Vec<_>>();for cut in f["cuts"].as_array().unwrap().iter().map(|v|Some(v.as_u64().unwrap()as usize)).chain(std::iter::once(None)){run_received(&keys,row["admitted"].as_bool().unwrap(),cut);}}}
#[test]fn original_named_receiving_checks_full_utf8_under_original_decoder_quantum(){let f:serde_json::Value=serde_json::from_str(fixture_text!("../../🔑️named/📇️index/🌳️root/🛬️receive/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();funding_vectors(&f);let prefix=f["prefix"].as_str().unwrap().repeat(f["repeat"].as_u64().unwrap()as usize);assert_eq!(prefix.len(),65538);for cut in f["cuts"].as_array().unwrap().iter().map(|v|Some(v.as_u64().unwrap()as usize)).chain(std::iter::once(None)){run_received(&[prefix.clone(),format!("{prefix}x")],true,cut);}}
