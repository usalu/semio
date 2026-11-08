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


use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits},retirement::RetireOwned};
use semio_framework_value_native_owned::native_decoding::{NativeReceive,NativeDecodeControl,OriginalEffectReply,OriginalEffectReplyComplete,OriginalReceivingPrimary,owned::{NativeRecordKey,tree::{NativeValueSource,NativeTreeKind,NativeScalarQuery,NativeSourceScalar}}};
use semio_framework_value::native_decoding::NativePreparedDecodeControl;
use semio_framework_value_native_receiving_original::NativeReceivingOriginal;
use semio_framework_value_testing_admission as fixture;
use crate::label::observe_locale_test_allocations as observed;
#[path="../../../../🌱️value/♻️retirement/📦️owner/🧪️work/effect.rs"]
mod effect;
const BODY:usize=4096;
const MAXIMUM_TURNS:usize=4096;

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
 fn as_source(&self)->&dyn NativeValueSource{self}
 fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Object)}
 fn record_len(&self)->Result<usize,ValueError>{Ok(2)}
 fn record_key(&self,index:usize)->Result<NativeRecordKey<'_>,ValueError>{match index{0=>Ok(NativeRecordKey::Name("en")),1=>Ok(NativeRecordKey::Name("de")),_=>Err(invalid_source())}}
 fn record_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match index{0=>Ok(&self.en),1=>Ok(&self.de),_=>Err(invalid_source())}}
}
impl NativeValueSource for OriginalLabels{
 fn query_scalar<'r>(&self,query:NativeScalarQuery,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,NativeSourceScalar,ValueError>)->Result<OriginalEffectReplyComplete<'r,NativeSourceScalar,ValueError>,ValueError>{object_answer(query,effect,reply)}
 fn recover_scalar<'r>(&self,_:NativeScalarQuery,_:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,NativeSourceScalar,ValueError>)->Result<OriginalEffectReplyComplete<'r,NativeSourceScalar,ValueError>,ValueError>{Ok(reply.pending())}
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
/// 🎯️ Lends one item and exactly the quoted demand under the test copy body.
fn quoted(wallet:&fixture::Wallet,d:RetirementDemand)->RetainedCloneGrant{
 assert!(d.copy_bytes<=BODY);let loan=wallet.loan(1,BODY);assert!(retained_clone_grant_admits(loan,d));RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth,..loan}
}
/// 🏁️ A suspended turn settles as zero progress carrying its work; host recipients here are synchronous.
fn outcome(poll:RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work}}}
fn receive_turn<S:NativeValueSource+RetireOwned+Sync,T:NativeReceive<S>>(owner:&mut NativeReceivingOriginal<S,T>,control:&mut NativeDecodeControl<'_>,wallet:&fixture::Wallet,grant:RetainedCloneGrant)->(RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>,(usize,usize)){
 let fuel=wallet.work_remaining();observed(||fixture::receive(control,grant,wallet.work_loan(fuel),|control|{outcome(owner.advance(control,BODY,fuel,grant))}))
}
fn close_turn<S:NativeValueSource+RetireOwned+Sync,T:NativeReceive<S>>(owner:&mut NativeReceivingOriginal<S,T>,grant:RetainedCloneGrant)->(RetainedWorkOutcome<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>,(usize,usize)){
 let(poll,heap)=effect::with_effect(grant,1<<20,&mut None,|effect|observed(||owner.close_step(effect)));(effect::settled(poll),heap)
}
fn take_turn<S:NativeValueSource+RetireOwned+Sync,T:NativeReceive<S>>(owner:&mut NativeReceivingOriginal<S,T>,grant:RetainedCloneGrant,output:&mut Option<T>)->(RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>,(usize,usize)){
 let(poll,heap)=effect::with_effect(grant,1<<20,&mut None,|effect|observed(||owner.take(output,effect)));(outcome(poll),heap)
}
fn receive_erased<S:NativeValueSource+RetireOwned+Sync+serde::Serialize,T:NativeReceive<S>+serde::Serialize>(input:S,expected:&serde_json::Value,cut:Option<usize>)->usize{
 let original=serde_json::to_value(&input).unwrap();let mut wallet=stock();
 let source=fixture::capture(input,&mut wallet,|wallet|wallet.loan(1,BODY),|wallet|wallet.work_remaining(),|wallet,p,work,heap,g|wallet.post(p,u128::from(work),g,heap));
 let mut owner=NativeReceivingOriginal::<S,T>::new(source);let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<24,&mut callback);let mut turns=0;
 while turns<MAXIMUM_TURNS&&!owner.output_ready()&&cut.is_none_or(|cut|turns<cut){
  let fuel=wallet.work_remaining();let d=owner.demands(BODY,fuel).unwrap();let g=quoted(&wallet,d);
  for denied in fixture::denials(g,d){assert!(!retained_clone_grant_admits(denied,d));let(turn,heap)=receive_turn(&mut owner,&mut control,&wallet,denied);assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(owner.demands(BODY,fuel).unwrap(),d);assert_eq!(serde_json::to_value(owner.borrow_input().unwrap().get()).unwrap(),original);}
  let(turn,heap)=receive_turn(&mut owner,&mut control,&wallet,g);let work=turn.work;let p=turn.result.unwrap().progress();assert!(p.fits(g));wallet.post(p,work,g,heap);assert_eq!(serde_json::to_value(owner.borrow_input().unwrap().get()).unwrap(),original);turns+=1;
 }
 let output=if cut.is_none(){
  assert!(owner.output_ready());let d=owner.take_demand().unwrap();let g=quoted(&wallet,d);
  for denied in fixture::denials(g,d){assert!(!retained_clone_grant_admits(denied,d));let mut denied_output=None;let(turn,heap)=take_turn(&mut owner,denied,&mut denied_output);assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),RetainedCloneProgress::default());assert!(denied_output.is_none());assert_eq!(heap,(0,0));assert!(owner.output_ready());assert_eq!(owner.take_demand().unwrap(),d);}
  let mut taken=None;let(turn,heap)=take_turn(&mut owner,g,&mut taken);let work=turn.work;let p=turn.result.unwrap().progress();assert!(p.fits(g));wallet.post(p,work,g,heap);let value=taken.expect("funded publication fills the caller slot");assert_eq!(serde_json::to_value(&value).unwrap(),*expected);Some(value)
 }else{assert_eq!(turns,cut.unwrap());None};
 owner.begin_close();
 for _ in 0..MAXIMUM_TURNS{
  if owner.terminal_is_empty(){break}let d=owner.close_demands(BODY).unwrap();let g=quoted(&wallet,d);
  for denied in fixture::denials(g,d){let(turn,heap)=close_turn(&mut owner,denied);assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(owner.close_demands(BODY).unwrap(),d);}
  let(turn,heap)=close_turn(&mut owner,g);let work=turn.work;let p=turn.result.unwrap().progress();assert!(p.fits(g));wallet.post(p,work,g,heap);
 }
 assert!(owner.terminal_is_empty());let(_,heap)=observed(||drop(owner));assert_eq!(heap,(0,0));
 if let Some(output)=output{fixture::close_owned(output,&mut wallet,BODY,MAXIMUM_TURNS)}turns
}
fn every_prefix<S:NativeValueSource+RetireOwned+Sync+serde::Serialize,T:NativeReceive<S>+serde::Serialize>(row:&serde_json::Value,create:impl Fn()->S){
 let turns=receive_erased::<S,T>(create(),&row["expected"],None);assert!(turns>0);for cut in 0..=turns{receive_erased::<S,T>(create(),&row["expected"],Some(cut));}
}
#[test]
fn declared_axes_and_labels_receive_actual_erased_sources(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();assert_eq!(law["bodyBytes"],BODY);assert_eq!(law["maximumTurns"],MAXIMUM_TURNS);assert_eq!(law["cancellation"],"every-receiving-prefix");assert_eq!(law["deniedCurrencies"],serde_json::json!(["items","copy","capacity","release","depth"]));
 for row in law["cases"].as_array().unwrap(){match row["kind"].as_str().unwrap(){"locale"=>every_prefix::<String,Locale>(row,||row["input"].as_str().unwrap().to_string()),"terminology"=>every_prefix::<String,Terminology>(row,||row["input"].as_str().unwrap().to_string()),"label"=>every_prefix::<OriginalLabels,crate::LocalizedLabel>(row,||original_label(&row["input"])),_=>panic!("undeclared erased locale case")}}
}
