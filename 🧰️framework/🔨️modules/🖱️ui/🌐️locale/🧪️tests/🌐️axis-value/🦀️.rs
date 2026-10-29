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


use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneSource,RetainedCloneGrant,RetainedCloneProgress,retained_clone_grant_admits},retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value_native_owned::native_decoding::{NativeReceive,NativeReceiver,NativeDecodeControl,NativeDecodeConstructionRecipient,owned::{NativeRecordKey,tree::NativeValueSource}};
use crate::label::observe_locale_test_allocations as observed;
const BODY:usize=4096;
const MAXIMUM_TURNS:usize=4096;

#[derive(serde::Serialize,semio_framework_value::RetireOwned)]
struct OriginalLanguages{en:String,de:String}
#[derive(serde::Serialize,semio_framework_value::RetireOwned)]
struct OriginalLabels{native:OriginalLanguages,reuse:OriginalLanguages}
fn invalid_source()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"original locale source has no such field")}
impl NativeValueSource for OriginalLanguages{
 fn as_source(&self)->&dyn NativeValueSource{self}
 fn record_len(&self)->Result<usize,ValueError>{Ok(2)}
 fn record_key(&self,index:usize)->Result<NativeRecordKey<'_>,ValueError>{match index{0=>Ok(NativeRecordKey::Name("en")),1=>Ok(NativeRecordKey::Name("de")),_=>Err(invalid_source())}}
 fn record_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match index{0=>Ok(&self.en),1=>Ok(&self.de),_=>Err(invalid_source())}}
}
impl NativeValueSource for OriginalLabels{
 fn as_source(&self)->&dyn NativeValueSource{self}
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
fn funding(d:RetirementDemand)->RetainedCloneGrant{
 assert!(d.copy_bytes<=BODY);RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:BODY,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}
}
fn denials(d:RetirementDemand)->Vec<RetainedCloneGrant>{
 let g=funding(d);[(true,RetainedCloneGrant{maximum_items:0,..g}),(d.copy_bytes>0,RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..g}),(d.capacity_bytes>0,RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..g}),(d.release_bytes>0,RetainedCloneGrant{maximum_release_bytes:d.release_bytes.saturating_sub(1),..g}),(d.depth>0,RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..g})].into_iter().filter_map(|(positive,g)|positive.then_some(g)).collect()
}
struct Turn(RetainedCloneGrant);
impl NativeDecodeConstructionRecipient for Turn{
 fn grant(&mut self,d:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{assert!(retained_clone_grant_admits(self.0,d));Ok(self.0)}
 fn accept(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),ValueError>{assert!(p.fits(g));Ok(())}
}
fn measured<T>(run:impl FnOnce()->T,receipt:impl FnOnce(&T)->RetainedCloneProgress)->T{
 let(value,physical)=observed(run);let p=receipt(&value);assert_eq!(physical,(p.retained_capacity_bytes,p.released_bytes));value
}
fn retire<T:RetireOwned>(value:T){
 let(mut owner,physical)=observed(||ControlledRetirement::new(value).unwrap_or_else(|_|panic!("original locale test retirement")));
 assert_eq!(physical,(0,0));
 for _ in 0..MAXIMUM_TURNS{if owner.terminal_is_empty(){let(_,physical)=observed(||drop(owner));assert_eq!(physical,(0,0));return}let d=owner.next_demand(BODY).unwrap();let g=funding(d);let step=measured(||owner.step(g).unwrap(),|step|step.progress());assert!(step.progress().fits(g));}
 panic!("funded locale retirement did not settle");
}
struct Closing<'a,T,R:NativeReceiver<dyn NativeValueSource,T>>{receiver:&'a mut R,output:std::marker::PhantomData<fn()->T>}
impl<T,R:NativeReceiver<dyn NativeValueSource,T>> semio_framework_value::ErasedSnapshotRetirement for Closing<'_,T,R>{
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,ValueError>{self.receiver.close_step(g)}
 fn terminal_is_empty(&self)->bool{self.receiver.terminal_is_empty()}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.receiver.close_demands(BODY)?.copy_bytes)}
 fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.receiver.close_demands(body)?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.receiver.close_demands(BODY)?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.receiver.close_demands(BODY)?.depth)}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.receiver.close_demands(body)}
}
fn close_receiver<T,R:NativeReceiver<dyn NativeValueSource,T>>(receiver:&mut R){
 let mut loan=Closing{receiver,output:std::marker::PhantomData::<fn()->T>};
 for _ in 0..MAXIMUM_TURNS{if loan.receiver.terminal_is_empty(){return}let d=semio_framework_value::select_retirement_cursor(&mut loan,BODY).unwrap().demands();for denied in denials(d){let(step,physical)=observed(||semio_framework_value::select_retirement_cursor(&mut loan,BODY).unwrap().step(denied).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(physical,(0,0));assert_eq!(semio_framework_value::select_retirement_cursor(&mut loan,BODY).unwrap().demands(),d);}let g=funding(d);let step=measured(||semio_framework_value::select_retirement_cursor(&mut loan,BODY).unwrap().step(g).unwrap(),|step|step.progress());assert!(step.progress().fits(g));}panic!("selected native locale receiver closure did not settle");
}
fn receive_erased<S:NativeValueSource+RetireOwned+serde::Serialize,T:NativeReceive<dyn NativeValueSource>+serde::Serialize>(input:S,expected:&serde_json::Value,cut:Option<usize>)->usize{
 let original=serde_json::to_value(&input).unwrap();
 let birth=RetainedCloneSource::<S>::owned_constructor_demand::<()>();let d=RetirementDemand{copy_bytes:RetainedCloneSource::<S>::constructor_copy_bytes(),capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()};let g=funding(d);
 let(source,p)=measured(||RetainedCloneSource::admit_owned(input,(),g).unwrap_or_else(|_|panic!("original locale source admission")),|(_,p)|*p);assert!(p.fits(g));
 let d=T::receive_birth_demand();let g=funding(d);
 for denied in denials(d){assert!(!retained_clone_grant_admits(denied,d));let(result,physical)=observed(||T::begin_receive(denied));assert!(result.is_err());assert_eq!(physical,(0,0));}
 let(mut receiver,p)=measured(||T::begin_receive(g).unwrap(),|(_,p)|*p);assert!(p.fits(g));
 let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<24,&mut callback);let mut turns=0;
 while turns<MAXIMUM_TURNS&&!receiver.output_ready()&&cut.is_none_or(|cut|turns<cut){
  let source_ref=source.borrow().project(0,|value|value.as_source());let d=receiver.demands(source_ref,BODY).unwrap();let g=funding(d);let ready=receiver.output_ready();
  for denied in denials(d){assert!(!retained_clone_grant_admits(denied,d));let mut turn=Turn(denied);let(step,physical)=observed(||control.with_construction_recipient(&mut turn,|control|receiver.advance(source_ref,control,BODY,denied)).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(physical,(0,0));assert_eq!(receiver.output_ready(),ready);assert_eq!(receiver.demands(source_ref,BODY).unwrap(),d);assert_eq!(serde_json::to_value(source.borrow().get()).unwrap(),original);}
  let mut turn=Turn(g);let step=measured(||control.with_construction_recipient(&mut turn,|control|receiver.advance(source_ref,control,BODY,g)).unwrap(),|step|step.progress());assert!(step.progress().fits(g));assert_eq!(serde_json::to_value(source.borrow().get()).unwrap(),original);turns+=1;
 }
 let output=if cut.is_none(){assert!(receiver.output_ready());let d=receiver.take_demand().unwrap();for denied in denials(d){let(value,physical)=observed(||receiver.take(denied).unwrap());assert!(value.is_none());assert_eq!(physical,(0,0));assert!(receiver.output_ready());assert_eq!(receiver.take_demand().unwrap(),d);}let g=funding(d);let(value,p)=measured(||receiver.take(g).unwrap().unwrap(),|(_,p)|*p);assert!(p.fits(g));assert_eq!(serde_json::to_value(&value).unwrap(),*expected);Some(value)}else{assert_eq!(turns,cut.unwrap());None};
 receiver.begin_close();
 close_receiver::<T,_>(&mut receiver);
 assert!(receiver.terminal_is_empty());assert_eq!(serde_json::to_value(source.borrow().get()).unwrap(),original);let(_,physical)=observed(||drop(receiver));assert_eq!(physical,(0,0));if let Some(output)=output{retire(output)}retire(source);turns
}
fn every_prefix<S:NativeValueSource+RetireOwned+serde::Serialize,T:NativeReceive<dyn NativeValueSource>+serde::Serialize>(row:&serde_json::Value,create:impl Fn()->S){
 let turns=receive_erased::<S,T>(create(),&row["expected"],None);assert!(turns>0);for cut in 0..=turns{receive_erased::<S,T>(create(),&row["expected"],Some(cut));}
}
#[test]
fn declared_axes_and_labels_receive_actual_erased_sources(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();assert_eq!(law["bodyBytes"],BODY);assert_eq!(law["maximumTurns"],MAXIMUM_TURNS);assert_eq!(law["cancellation"],"every-receiving-prefix");assert_eq!(law["deniedCurrencies"],serde_json::json!(["items","copy","capacity","release","depth"]));
 for row in law["cases"].as_array().unwrap(){match row["kind"].as_str().unwrap(){"locale"=>every_prefix::<String,Locale>(row,||row["input"].as_str().unwrap().to_string()),"terminology"=>every_prefix::<String,Terminology>(row,||row["input"].as_str().unwrap().to_string()),"label"=>every_prefix::<OriginalLabels,crate::LocalizedLabel>(row,||original_label(&row["input"])),_=>panic!("undeclared erased locale case")}}
}
