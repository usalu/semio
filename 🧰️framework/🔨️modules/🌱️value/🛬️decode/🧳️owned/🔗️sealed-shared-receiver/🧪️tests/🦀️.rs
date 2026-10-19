use semio_framework_test_fixtures::fixture_text;
use super::*;
use crate::{DslValue,retained_clone::RetainedCloneSource,retirement::RetireOwned};
fn funded(d:RetirementDemand)->RetainedCloneGrant{assert!(d.copy_bytes<=4096);RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
struct OriginalTurnConstruction{grant:RetainedCloneGrant,issued:bool}
impl crate::native_decoding::NativeDecodeConstructionRecipient for OriginalTurnConstruction{
 fn grant(&mut self,d:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{assert!(!self.issued);assert!(retained_clone_grant_admits(self.grant,d));self.issued=true;Ok(self.grant)}
 fn accept(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),ValueError>{assert!(self.issued);assert_eq!(g,self.grant);assert!(p.fits(g));Ok(())}
}
fn advance_original<T,R:NativeReceiver<DslValue,T>>(receiver:&mut R,original:crate::retained_clone::RetainedCloneRef<'_,DslValue>,control:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let mut recipient=OriginalTurnConstruction{grant:g,issued:false};control.with_construction_recipient(&mut recipient,|control|receiver.advance(original,control,4096,g))}
fn drain<T:RetireOwned>(value:T){let mut owner=crate::retirement::controlled::ControlledRetirement::new(value).unwrap_or_else(|_|panic!("shared fixture supports original closure"));for _ in 0..100000{if owner.terminal_is_empty(){return;}let g=funded(owner.next_demand(4096).unwrap());let(step,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||owner.step(g).unwrap());assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}panic!("shared source retirement did not settle");}
fn source(value:DslValue)->RetainedCloneSource<DslValue>{let d=RetainedCloneSource::<DslValue>::owned_constructor_demand::<()>();RetainedCloneSource::admit_owned(value,(),RetainedCloneGrant{maximum_copy_bytes:RetainedCloneSource::<DslValue>::constructor_copy_bytes(),..funded(RetirementDemand{capacity_bytes:d.capacity_bytes,depth:d.depth,..Default::default()})}).unwrap_or_else(|_|panic!("original shared source admission")).0}
fn close(receiver:&mut NativeSharedReceiver<DslValue,String>){receiver.begin_close();for _ in 0..100000{if receiver.terminal_is_empty(){return;}let d=receiver.close_demands(4096).unwrap();let g=funded(d);let(step,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||receiver.close_step(g).unwrap());assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}panic!("shared receiver cancellation did not settle");}
#[test]
fn native_shared_original_receive_exact_prefix_cancellation_and_transfer(){
 let vectors:serde_json::Value=serde_json::from_str(fixture_text!("../../🧳️owned/🔗️sealed-shared-receiver/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for law in vectors["cases"].as_array().unwrap(){let mut complete=0;for cut in 0..1000{
  let original=source(DslValue::String(law["original"].as_str().unwrap().to_owned()));let foreign=source(DslValue::String(law["original"].as_str().unwrap().to_owned()));
  let d=<SealedShared<String> as NativeReceive<DslValue>>::receive_birth_demand();let g=funded(d);let((mut receiver,p),heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||<SealedShared<String> as NativeReceive<DslValue>>::begin_receive(g).unwrap());assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));
  let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut callback);let mut construction=crate::native_decoding::owned::OriginalConstructionFixture;let mut turns=0;
  while !receiver.output_ready()&&turns<cut{let d=receiver.demands(original.borrow(),4096).unwrap();let g=funded(d);
   for axis in 0..5{let mut denied=g;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes=d.copy_bytes-1,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes=d.capacity_bytes-1,3 if d.release_bytes>0=>denied.maximum_release_bytes=d.release_bytes-1,4 if d.depth>0=>denied.maximum_depth=d.depth-1,_=>continue};let(step,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||control.with_construction_recipient(&mut construction,|control|receiver.advance(original.borrow(),control,4096,denied)).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(receiver.demands(original.borrow(),4096).unwrap(),d);}
   let(step,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||control.with_construction_recipient(&mut construction,|control|receiver.advance(original.borrow(),control,4096,g)).unwrap());assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));turns+=1;
   if receiver.binding.is_some()&&!receiver.output_ready(){assert!(receiver.demands(foreign.borrow(),4096).is_err());}
  }
  if receiver.output_ready(){complete=turns;assert!(receiver.child.is_none());assert!(receiver.raw.is_none());let pointer=receiver.output.as_ref().unwrap().get().as_ptr();let d=receiver.take_demand().unwrap();let g=funded(d);let mut denied=g;denied.maximum_items=0;let(value,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||receiver.take(denied).unwrap());assert!(value.is_none());assert_eq!(heap,(0,0));let(value,p)=receiver.take(g).unwrap().unwrap();assert!(p.fits(g));assert_eq!(value.get().as_ptr(),pointer);assert_eq!(value.get(),law["expected"].as_str().unwrap());drain(value);}
  close(&mut receiver);drain(original);drain(foreign);if complete>0&&cut>complete{break;}
 }assert!(complete>0);eprintln!("[DEBUG] shared native original {} completed {} turns with all prefix cancellation and exact heap receipts",law["id"],complete);}
}
#[test]
fn native_shared_original_callback_refusal_retains_paid_child_authority(){
 let original=source(DslValue::String("authored original".to_owned()));let d=<SealedShared<String> as NativeReceive<DslValue>>::receive_birth_demand();let(mut receiver,_)=<SealedShared<String> as NativeReceive<DslValue>>::begin_receive(funded(d)).unwrap();let admitted=std::cell::Cell::new(true);let mut callback=|_|admitted.get();let mut control=NativeDecodeControl::new(1<<20,&mut callback);let mut construction=crate::native_decoding::owned::OriginalConstructionFixture;
 for _ in 0..100{if receiver.phase==Phase::Receive{break;}let g=funded(receiver.demands(original.borrow(),4096).unwrap());control.with_construction_recipient(&mut construction,|control|receiver.advance(original.borrow(),control,4096,g)).unwrap();}
 assert!(receiver.child.is_some());let pointer=receiver.child.as_deref().unwrap() as *const _;admitted.set(false);let g=funded(receiver.demands(original.borrow(),4096).unwrap());let(result,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||control.with_construction_recipient(&mut construction,|control|receiver.advance(original.borrow(),control,4096,g)));assert!(result.is_err());assert_eq!(heap,(0,0));assert_eq!(receiver.child.as_deref().unwrap() as *const _,pointer);assert!(!receiver.output_ready());close(&mut receiver);drain(original);eprintln!("[DEBUG] shared native callback refusal retained original child allocation until funded cancellation");
}

#[derive(crate::RetireOwned,crate::NativeReceive)]
struct WideRecord{
 a:SealedShared<String>,b:SealedShared<String>,c:SealedShared<String>,d:SealedShared<String>,e:SealedShared<String>,
 f:SealedShared<String>,g:SealedShared<String>,h:SealedShared<String>,i:SealedShared<String>,j:SealedShared<String>,
 k:SealedShared<String>,l:SealedShared<String>,m:SealedShared<String>,n:SealedShared<String>,o:SealedShared<String>,
 p:SealedShared<String>,q:SealedShared<String>,r:SealedShared<String>,s:SealedShared<String>,t:SealedShared<String>,u:SealedShared<String>,
 v:SealedShared<String>,w:SealedShared<String>,x:SealedShared<String>,y:SealedShared<String>,z:SealedShared<String>,
 aa:SealedShared<String>,ab:SealedShared<String>,ac:SealedShared<String>,ad:SealedShared<String>,ae:SealedShared<String>,af:SealedShared<String>,
}
#[test]
fn native_record32_single_active_child_fits4096_and_preserves_every_original_prefix(){
 let mut finished=0;
 for cut in 0..10000{
  let keys=["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z","aa","ab","ac","ad","ae","af"];let original=source(DslValue::Object(keys.into_iter().map(|key|(key.to_owned(),DslValue::String(format!("original {key} 🏠️")))).collect()));
  let birth=<WideRecord as NativeReceive<DslValue>>::receive_birth_demand();let(mut receiver,p)=<WideRecord as NativeReceive<DslValue>>::begin_receive(funded(birth)).unwrap();assert!(p.fits(funded(birth)));
  let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<24,&mut callback);let mut construction=crate::native_decoding::owned::OriginalConstructionFixture;let mut turns=0;
  while !receiver.output_ready()&&turns<cut{
   let d=receiver.demands(original.borrow(),4096).unwrap();let g=funded(d);let mut denied=g;denied.maximum_items=0;let(step,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||control.with_construction_recipient(&mut construction,|control|receiver.advance(original.borrow(),control,4096,denied)).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(receiver.demands(original.borrow(),4096).unwrap(),d);
   let(step,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||control.with_construction_recipient(&mut construction,|control|receiver.advance(original.borrow(),control,4096,g)).unwrap());assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
   assert!(receiver.native_field_owner_count()<=1);turns+=1;
  }
  if receiver.output_ready(){finished=turns;let g=funded(receiver.take_demand().unwrap());let(row,p)=receiver.take(g).unwrap().unwrap();assert!(p.fits(g));assert_eq!(row.a.get(),"original a 🏠️");assert_eq!(row.q.get(),"original q 🏠️");assert_eq!(row.af.get(),"original af 🏠️");drain(row);}
  receiver.begin_close();for _ in 0..100000{if receiver.terminal_is_empty(){break;}let g=funded(receiver.close_demands(4096).unwrap());let(step,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||receiver.close_step(g).unwrap());assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}assert!(receiver.terminal_is_empty());drain(original);if finished>0&&cut>finished{break;}
 }
 assert!(finished>0);eprintln!("[DEBUG] original32-field shared record completed {finished} paid turns; one active child, every4096 prefix closed with exact physical receipts");
}

#[derive(crate::RetireOwned,crate::NativeReceive)]
struct GroupName{name:SealedShared<String>}
#[derive(crate::RetireOwned,crate::NativeReceive)]
struct GroupVolume{volume:f64}

#[derive(Default,crate::RetireOwned,crate::NativeReceive,crate::ToValue)]
enum OriginalDefaultMedia { #[default] Empty,Text(String) }
#[derive(crate::RetireOwned,crate::NativeReceive,crate::ToValue)]
struct OriginalDeclaredDefaults {
 id:String,
 #[value(default)]values:Vec<String>,
 #[value(default)]scale:f64,
 #[value(default)]enabled:bool,
 #[value(default)]media:OriginalDefaultMedia,
 optional:Option<String>,
}
#[test]
fn native_declared_defaults_keep_missing_fields_original_and_heapless() {
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🧳️owned/📥️default/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let mut finished=0;
 for cut in 0..2048 {
  let source=source(DslValue::from(&law["input"]));let demand=<OriginalDeclaredDefaults as NativeReceive<DslValue>>::receive_birth_demand();let(mut receiver,_)=<OriginalDeclaredDefaults as NativeReceive<DslValue>>::begin_receive(funded(demand)).unwrap();let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<24,&mut callback);let mut turns=0;
  while !receiver.output_ready()&&turns<cut {
   let demand=receiver.demands(source.borrow(),4096).unwrap();let grant=funded(demand);
   let mut denied=grant;denied.maximum_items=0;let(step,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||advance_original(&mut receiver,source.borrow(),&mut control,denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(receiver.demands(source.borrow(),4096).unwrap(),demand);
   let(step,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||advance_original(&mut receiver,source.borrow(),&mut control,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));turns+=1;
  }
  if receiver.output_ready() { finished=turns;let(output,p)=receiver.take(funded(receiver.take_demand().unwrap())).unwrap().unwrap();assert!(p.copied_items>0);assert_eq!(serde_json::Value::from(crate::ToValue::to_value(&output)),law["expected"]);drain(output); }
  receiver.begin_close();for _ in 0..100000 { if receiver.terminal_is_empty(){break;}let grant=funded(receiver.close_demands(4096).unwrap());let(step,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||receiver.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes)); }assert!(receiver.terminal_is_empty());drain(source);if finished>0&&cut>finished{break;}
 }
 assert!(finished>0);
 use crate::native_decoding::owned::NativeDefault;
 let demand=Vec::<String>::native_default_demand();let grant=funded(demand);let((value,p),heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||Vec::<String>::native_default(grant).unwrap());assert_eq!(heap,(0,0));assert_eq!(value.capacity(),0);assert_eq!(p.retained_capacity_bytes,0);assert!(p.fits(grant));
 eprintln!("[DEBUG] native declared defaults completed={finished} turns; every prefix/zero grant retains original, schema defaults construct no heap backing");
}
#[derive(crate::RetireOwned,crate::NativeReceive)]
struct FlattenedRecord{
 #[value(flatten)]name:SealedShared<GroupName>,
 #[value(flatten)]volume:SealedShared<GroupVolume>,
}
#[test]
fn native_flattened_original_record_preserves_source_schema_and_all_partial_custody(){
 let input=||DslValue::Object(vec![("name".into(),DslValue::String("original β".into())),("volume".into(),DslValue::Number(crate::Number::Float(123.5)))]);let mut finished=0;
 for cut in 0..2000{
  let original=source(input());let foreign=source(input());let d=<FlattenedRecord as NativeReceive<DslValue>>::receive_birth_demand();let(mut receiver,_)=<FlattenedRecord as NativeReceive<DslValue>>::begin_receive(funded(d)).unwrap();let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<24,&mut callback);let mut turns=0;
  while !receiver.output_ready()&&turns<cut{let d=receiver.demands(original.borrow(),4096).unwrap();let g=funded(d);let mut denied=g;denied.maximum_items=0;let(step,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||advance_original(&mut receiver,original.borrow(),&mut control,denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(receiver.demands(original.borrow(),4096).unwrap(),d);let(step,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||advance_original(&mut receiver,original.borrow(),&mut control,g).unwrap());assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));turns+=1;if receiver.binding.is_some()&&!receiver.output_ready(){assert!(receiver.demands(foreign.borrow(),4096).is_err());}}
  if receiver.output_ready(){finished=turns;let d=receiver.take_demand().unwrap();let g=funded(d);let((row,p),heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||receiver.take(g).unwrap().unwrap());assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert_eq!(row.name.get().name.get(),"original β");assert_eq!(row.volume.get().volume,123.5);drain(row);}
  receiver.begin_close();for _ in 0..100000{if receiver.terminal_is_empty(){break;}let g=funded(receiver.close_demands(4096).unwrap());let(step,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||receiver.close_step(g).unwrap());assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}assert!(receiver.terminal_is_empty());drain(original);drain(foreign);if finished>0&&cut>finished{break;}
 }assert!(finished>0);
 for entries in [vec![("name",DslValue::String("a".into())),("volume",DslValue::Number(crate::Number::Float(1.0))),("unknown",DslValue::Null)],vec![("name",DslValue::String("a".into())),("name",DslValue::String("b".into())),("volume",DslValue::Number(crate::Number::Float(1.0)))],vec![("name",DslValue::String("a".into()))]]{
  let original=source(DslValue::Object(entries.into_iter().map(|(key,value)|(key.to_owned(),value)).collect()));let d=<FlattenedRecord as NativeReceive<DslValue>>::receive_birth_demand();let(mut receiver,_)=<FlattenedRecord as NativeReceive<DslValue>>::begin_receive(funded(d)).unwrap();let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<24,&mut callback);let mut refused=false;for _ in 0..2000{if receiver.output_ready(){break;}let g=funded(receiver.demands(original.borrow(),4096).unwrap());if advance_original(&mut receiver,original.borrow(),&mut control,g).is_err(){refused=true;break;}}assert!(refused);assert!(!receiver.output_ready());receiver.begin_close();for _ in 0..100000{if receiver.terminal_is_empty(){break;}let g=funded(receiver.close_demands(4096).unwrap());let(step,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||receiver.close_step(g).unwrap());assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}assert!(receiver.terminal_is_empty());drain(original);
 }eprintln!("[DEBUG] original flattened record completed {finished} bounded turns; full schema rejects unknown, duplicate and missing fields, every prefix closes original typed custody");
}
