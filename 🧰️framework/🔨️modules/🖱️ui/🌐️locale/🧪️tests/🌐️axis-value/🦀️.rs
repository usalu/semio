use super::*;
use semio_framework_value::{FromValue, ToValue};

#[semio_framework_async_macros::async_test]
async fn locale_round_trips_through_to_value_and_from_value() {
    for locale in Locale::ALL {
        let decoded = Locale::from_value(locale.to_value()).expect("valid DslValue decodes");
        assert_eq!(decoded, locale);
    }
}

#[semio_framework_async_macros::async_test]
async fn terminology_round_trips_through_to_value_and_from_value() {
    for terminology in Terminology::ALL {
        let decoded = Terminology::from_value(terminology.to_value()).expect("valid DslValue decodes");
        assert_eq!(decoded, terminology);
    }
}

#[semio_framework_async_macros::async_test]
async fn to_value_matches_the_existing_serde_wire_string() {
    assert_eq!(Locale::En.to_value(), semio_framework_value::DslValue::String("en".to_string()));
    assert_eq!(Terminology::Native.to_value(), semio_framework_value::DslValue::String("native".to_string()));
}


use semio_framework_value::{OriginalRetirementFailure,RetirementDemand,ValueError,ValueRefusalKind,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,retained_clone::{OriginalEffectAuthority,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource,RetainedCloneStep,retained_clone_grant_admits},retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value_native_owned::native_decoding::{NativeReceive,NativeDecodeControl,NativeReceivingFailure,OriginalEffectReply,OriginalEffectReplyComplete,OriginalReceivingPrimary,owned::{NativeRecordKey,NativeSchemaKeySource,NativeSourceKeyReply,NativeSourceKeyReplyComplete,answer_schema_key,tree::{NativeValueSource,NativeTreeKind,NativeScalarQuery,NativeSourceScalar}}};
use semio_framework_value_native_receiving_original::NativeReceivingOriginal;
use semio_framework_value_testing_admission::{self as fixture,Authorities,BankWallet,EffectBank,FixtureRetirementProducer};
use crate::label::observe_locale_test_allocations as observed;
const BODY:usize=4096;
const MAXIMUM_TURNS:usize=4096;
/// ⛽️ Work ceiling of every receiving turn: constant so a resident action resumes under its unchanged request; the stock funds far more.
const TURN_WORK:u64=1<<20;
/// 📏️ Copy window of every quoted loan and the retirement body: large enough for the fixed receiver seeds and the metadata of the nested sealed close cursors, independent of the receiving law body.
const LOAN_BODY:usize=1<<16;
type Turn=RetainedWorkOutcome<RetainedCloneStep,NativeReceivingFailure>;
type Taken=RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>;
type Closer<'c,T>=OriginalEffectAuthority<FixtureRetirementProducer<'c,T>>;

#[derive(serde::Serialize,semio_framework_value::RetireOwned)]
struct OriginalLanguages{en:String,de:String}
#[derive(serde::Serialize,semio_framework_value::RetireOwned)]
struct OriginalLabels{native:OriginalLanguages,reuse:OriginalLanguages}
fn invalid_source()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"original locale source has no such field")}
/// 📬️ Answers an object source's scalar query and records its one unit of original work.
fn object_answer<'r>(query:NativeScalarQuery,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,NativeSourceScalar,ValueError>)->Result<OriginalEffectReplyComplete<'r,NativeSourceScalar,ValueError>,ValueError>{
 if effect.maximum_work_units()==0||!effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"locale source query exceeded its original work authority"))))}
 Ok(reply.complete(match query{NativeScalarQuery::Kind=>Ok(NativeSourceScalar::Kind(Some(NativeTreeKind::Object))),NativeScalarQuery::Null=>Ok(NativeSourceScalar::Null(false)),NativeScalarQuery::RecordLength=>Ok(NativeSourceScalar::Length(2)),_=>Err(invalid_source())}))
}
impl NativeValueSource for OriginalLanguages{
 fn query_scalar<'r>(&self,query:NativeScalarQuery,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,NativeSourceScalar,ValueError>)->Result<OriginalEffectReplyComplete<'r,NativeSourceScalar,ValueError>,ValueError>{object_answer(query,effect,reply)}
 fn recover_scalar<'r>(&self,_:NativeScalarQuery,_:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,NativeSourceScalar,ValueError>)->Result<OriginalEffectReplyComplete<'r,NativeSourceScalar,ValueError>,ValueError>{Ok(reply.pending())}
 fn query_schema_key<'reply,'source>(&'source self,query:NativeSchemaKeySource,effect:&mut RetainedAllocationEffect<'_>,reply:NativeSourceKeyReply<'reply,'source>)->Result<NativeSourceKeyReplyComplete<'reply>,ValueError>{answer_schema_key(self,query,effect,reply)}
 fn as_source(&self)->&dyn NativeValueSource{self}
 fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Object)}
 fn record_len(&self)->Result<usize,ValueError>{Ok(2)}
 fn record_key(&self,index:usize)->Result<NativeRecordKey<'_>,ValueError>{match index{0=>Ok(NativeRecordKey::Name("en")),1=>Ok(NativeRecordKey::Name("de")),_=>Err(invalid_source())}}
 fn record_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match index{0=>Ok(&self.en),1=>Ok(&self.de),_=>Err(invalid_source())}}
}
impl NativeValueSource for OriginalLabels{
 fn query_scalar<'r>(&self,query:NativeScalarQuery,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,NativeSourceScalar,ValueError>)->Result<OriginalEffectReplyComplete<'r,NativeSourceScalar,ValueError>,ValueError>{object_answer(query,effect,reply)}
 fn recover_scalar<'r>(&self,_:NativeScalarQuery,_:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,NativeSourceScalar,ValueError>)->Result<OriginalEffectReplyComplete<'r,NativeSourceScalar,ValueError>,ValueError>{Ok(reply.pending())}
 fn query_schema_key<'reply,'source>(&'source self,query:NativeSchemaKeySource,effect:&mut RetainedAllocationEffect<'_>,reply:NativeSourceKeyReply<'reply,'source>)->Result<NativeSourceKeyReplyComplete<'reply>,ValueError>{answer_schema_key(self,query,effect,reply)}
 fn as_source(&self)->&dyn NativeValueSource{self}
 fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Object)}
 fn record_len(&self)->Result<usize,ValueError>{Ok(2)}
 fn record_key(&self,index:usize)->Result<NativeRecordKey<'_>,ValueError>{match index{0=>Ok(NativeRecordKey::Name("native")),1=>Ok(NativeRecordKey::Name("reuse")),_=>Err(invalid_source())}}
 fn record_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match index{0=>Ok(&self.native),1=>Ok(&self.reuse),_=>Err(invalid_source())}}
}
fn original_label(input:&serde_json::Value)->OriginalLabels{
 let row=|name:&str|OriginalLanguages{en:input[name]["en"].as_str().unwrap().into(),de:input[name]["de"].as_str().unwrap().into()};
 let original=OriginalLabels{native:row("native"),reuse:row("reuse")};let source:&dyn NativeValueSource=&original;
 assert!(std::ptr::eq(source.record_child(0).unwrap().record_child(0).unwrap().text().unwrap(),&original.native.en));
 assert!(std::ptr::eq(source.record_child(1).unwrap().record_child(1).unwrap().text().unwrap(),&original.reuse.de));original
}
/// 🏦️ One explicit finite stock funds every turn of a case; nothing is borrowed from an ambient budget.
fn stock()->fixture::Wallet{fixture::Wallet::from_profile(&serde_json::json!({"items":1u64<<20,"copyBytes":1u64<<30,"capacityBytes":1u64<<30,"releaseBytes":1u64<<30,"depth":4096u64,"fuel":1u64<<40}))}
/// 🧾️ Scratch bank that admits the original source and runs publication effects.
fn scratch()->EffectBank{EffectBank::new(BankWallet::from_profile(&serde_json::json!({"items":1u64<<40,"copyBytes":1u64<<40,"capacityBytes":1u64<<40,"releaseBytes":1u64<<40,"depth":1u64<<20,"fuel":1u64<<60})))}
/// 🎯️ Lends the items of one schema metadata step and exactly the quoted demand under the loan copy window.
fn quoted(wallet:&fixture::Wallet,d:RetirementDemand)->RetainedCloneGrant{
 assert!(d.copy_bytes<=LOAN_BODY,"quoted copy {d:?} exceeds the loan window");let loan=wallet.loan(semio_framework_value_native_schema::NATIVE_SCHEMA_METADATA_ITEMS,LOAN_BODY);assert!(retained_clone_grant_admits(loan,d));RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth,..loan}
}
/// 🏁️ Every synchronous fixture turn completes; a pending turn is a law failure.
fn settled<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("synchronous locale fixture turn stayed pending")}}
/// ⏸️ A suspended receiving turn settles as zero progress carrying its work; its resident action resumes in a later turn.
fn suspended(poll:RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>)->Turn{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work}}}
/// 🧮️ Reads the physical progress a turn retained, whether it succeeded or refused.
trait RetainedProgressOf{fn progress_of(&self)->RetainedCloneProgress;}
impl RetainedProgressOf for OriginalReceivingPrimary{fn progress_of(&self)->RetainedCloneProgress{self.retained_progress()}}
impl RetainedProgressOf for NativeReceivingFailure{fn progress_of(&self)->RetainedCloneProgress{self.retained_progress()}}
fn progress<E:RetainedProgressOf>(turn:&RetainedWorkOutcome<RetainedCloneStep,E>)->RetainedCloneProgress{match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.progress_of()}}
fn input<S:RetireOwned+Sync+'static>(value:S,bank:&mut EffectBank)->RetainedCloneSource<S>{let g=bank.loan(1,BODY);match fixture::admit_owned(bank,value,(),g).result{Ok((source,_))=>source,Err(_)=>panic!("fixture source admission refused")}}
fn advance<S:NativeValueSource+RetireOwned+Sync,T:NativeReceive<S>>(owner:&mut NativeReceivingOriginal<S,T>,control:&mut NativeDecodeControl<'_>,payer:&mut Authorities,wallet:&fixture::Wallet,grant:RetainedCloneGrant)->(Turn,(usize,usize)){
 let fuel=TURN_WORK;let mut heap=(0,0);let turn=suspended(payer.nested(control,grant,|control,witness|{let(poll,observed_heap)=observed(||owner.advance(control,witness,BODY,fuel,grant));heap=observed_heap;poll}));(turn,heap)
}
fn settle_pending<S:NativeValueSource+RetireOwned+Sync,T:NativeReceive<S>>(owner:&mut NativeReceivingOriginal<S,T>,control:&mut NativeDecodeControl<'_>,payer:&mut Authorities,wallet:&mut fixture::Wallet){
 for _ in 0..MAXIMUM_TURNS{if !owner.advance_pending()||owner.output_ready(){return}let d=owner.demands(BODY,TURN_WORK).unwrap();let g=quoted(wallet,d);let(turn,heap)=advance(owner,control,payer,wallet,g);wallet.post(progress(&turn),turn.work,g,heap);turn.result.unwrap();}
 panic!("original locale receiving action did not settle")
}
fn take<S:NativeValueSource+RetireOwned+Sync,T:NativeReceive<S>>(owner:&mut NativeReceivingOriginal<S,T>,bank:&mut EffectBank,grant:RetainedCloneGrant,output:&mut Option<T>)->(Taken,(usize,usize)){
 let((poll,_),heap)=observed(||bank.run(grant,1,|effect|owner.take(output,effect)));assert_ne!(bank.take_admitted(),Some(false));(settled(poll),heap)
}
fn close_turn<T:RetireOwned>(authority:&mut Closer<'_,T>,payer:&mut Authorities,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant,work:u64)->(Result<RetainedCloneStep,OriginalRetirementFailure>,u128,(usize,usize)){
 let mut action=None;let mut heap_sum=(0usize,0usize);let mut work_sum=0u128;
 for _ in 0..MAXIMUM_TURNS{
  let(done,heap)=payer.effect(control,grant,work,|effect|{let before=effect.performed_work();let(ready,heap)=observed(||authority.drive(&mut (LOAN_BODY,None),effect,&mut action).unwrap());((ready,effect.performed_work()-before),heap)}).expect("close effect admitted");
  heap_sum.0+=heap.0;heap_sum.1+=heap.1;work_sum+=done.1;if done.0{break}
 }
 let demand=Closer::<T>::transfer_reply_demand().unwrap();let mut output=None;
 payer.effect(control,fixture::demand_grant(demand),1,|effect|{authority.transfer_reply_into(&(LOAN_BODY,None),&mut action,&mut output,effect).result.unwrap().unwrap()}).expect("reply transfer admitted");
 (output.take().expect("transferred close reply"),work_sum,heap_sum)
}
/// ♻️ Closes an owned value under quoted grants, proving every denied currency spends nothing and keeps the quote.
fn close<T:RetireOwned>(value:T,wallet:&mut fixture::Wallet,payer:&mut Authorities,control:&mut NativeDecodeControl<'_>){
 let mut cursor=ControlledRetirement::new(value).map_err(|(error,_)|error).unwrap();let mut authority=Some(OriginalEffectAuthority::new(FixtureRetirementProducer::new(&mut cursor,Some(payer.source_turn()))));
 for _ in 0..MAXIMUM_TURNS{
  let owner=authority.as_ref().unwrap().producer().owner();
  if owner.terminal_is_empty(){
   let mut producer=None;payer.effect(control,fixture::demand_grant(Closer::<T>::producer_transfer_demand().unwrap()),1,|effect|{Closer::<T>::transfer_producer(&mut authority,&(LOAN_BODY,None),&mut producer,effect).result.unwrap()}).expect("producer transfer admitted");producer.take();
   return
  }
  let d=owner.next_demand(LOAN_BODY).unwrap();let work=owner.next_computational_work(LOAN_BODY).unwrap();let g=quoted(wallet,d);
  for denied in fixture::denials(g,d){let(result,spent,heap)=close_turn(authority.as_mut().unwrap(),payer,control,denied,work);assert_eq!(spent,0);assert_eq!(heap,(0,0));assert_eq!(result.unwrap().progress(),RetainedCloneProgress::default());assert_eq!(authority.as_ref().unwrap().producer().owner().next_demand(LOAN_BODY).unwrap(),d);}
  let(result,spent,heap)=close_turn(authority.as_mut().unwrap(),payer,control,g,work);let p=result.unwrap().progress();assert!(p.fits(g));wallet.post(p,spent,g,heap);
 }
 panic!("original locale owner did not close in its turn bound")
}
fn receive_erased<S:NativeValueSource+RetireOwned+Sync+serde::Serialize,T:NativeReceive<S>+serde::Serialize>(value:S,expected:&serde_json::Value,cut:Option<usize>)->usize{
 let original=serde_json::to_value(&value).unwrap();let mut wallet=stock();let mut bank=scratch();let source=input(value,&mut bank);
 let(mut owner,heap)=observed(||NativeReceivingOriginal::<S,T>::new(source));assert_eq!(heap,(0,0));let mut payer=Authorities::new();let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<24,&mut callback);let mut turns=0;
 while turns<MAXIMUM_TURNS&&!owner.output_ready()&&cut.is_none_or(|cut|turns<cut){
  let fuel=TURN_WORK;let d=owner.demands(BODY,fuel).unwrap();let g=quoted(&wallet,d);
  for denied in fixture::denials(g,d){assert!(!retained_clone_grant_admits(denied,d));let(turn,heap)=advance(&mut owner,&mut control,&mut payer,&wallet,denied);assert_eq!(turn.work,0);assert_eq!(progress(&turn),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(owner.demands(BODY,fuel).unwrap(),d);assert_eq!(serde_json::to_value(owner.borrow_input().unwrap().get()).unwrap(),original);}
  let(turn,heap)=advance(&mut owner,&mut control,&mut payer,&wallet,g);let p=progress(&turn);assert!(p.fits(g),"turn {turns} posted {p:?} beyond its quote {d:?} under {g:?}");wallet.post(p,turn.work,g,heap);turn.result.unwrap();assert_eq!(serde_json::to_value(owner.borrow_input().unwrap().get()).unwrap(),original);turns+=1;
 }
 if let Some(cut)=cut{assert_eq!(turns,cut)}
 settle_pending(&mut owner,&mut control,&mut payer,&mut wallet);
 let output=if cut.is_none(){
  assert!(owner.output_ready());let d=owner.take_demand().unwrap();let g=quoted(&wallet,d);
  for denied in fixture::denials(g,d){assert!(!retained_clone_grant_admits(denied,d));let mut denied_output=None;let(turn,heap)=take(&mut owner,&mut bank,denied,&mut denied_output);assert_eq!(turn.work,0);assert_eq!(progress(&turn),RetainedCloneProgress::default());assert!(denied_output.is_none());assert_eq!(heap,(0,0));assert!(owner.output_ready());assert_eq!(owner.take_demand().unwrap(),d);}
  let mut taken=None;let(turn,heap)=take(&mut owner,&mut bank,g,&mut taken);let p=progress(&turn);assert!(p.fits(g));wallet.post(p,turn.work,g,heap);turn.result.unwrap();let value=taken.expect("funded publication fills the caller slot");assert_eq!(serde_json::to_value(&value).unwrap(),*expected);Some(value)
 }else{None};
 close(owner,&mut wallet,&mut payer,&mut control);
 if let Some(output)=output{close(output,&mut wallet,&mut payer,&mut control)}turns
}
fn every_prefix<S:NativeValueSource+RetireOwned+Sync+serde::Serialize,T:NativeReceive<S>+serde::Serialize>(row:&serde_json::Value,create:impl Fn()->S){
 let turns=receive_erased::<S,T>(create(),&row["expected"],None);assert!(turns>0);for cut in 0..=turns{receive_erased::<S,T>(create(),&row["expected"],Some(cut));}
}
#[test]
fn declared_axes_and_labels_receive_actual_erased_sources(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();assert_eq!(law["bodyBytes"],BODY);assert_eq!(law["maximumTurns"],MAXIMUM_TURNS);assert_eq!(law["cancellation"],"every-receiving-prefix");assert_eq!(law["deniedCurrencies"],serde_json::json!(["items","copy","capacity","release","depth"]));
 for row in law["cases"].as_array().unwrap(){match row["kind"].as_str().unwrap(){"locale"=>every_prefix::<String,Locale>(row,||row["input"].as_str().unwrap().to_string()),"terminology"=>every_prefix::<String,Terminology>(row,||row["input"].as_str().unwrap().to_string()),"label"=>every_prefix::<OriginalLabels,crate::LocalizedLabel>(row,||original_label(&row["input"])),_=>panic!("undeclared erased locale case")}}
}
