use super::*;
use std::mem::size_of;
use semio_framework_value::{RetirementDemand, retained_clone::{RetainedCloneGrant, RetainedCloneSource}, retirement::{RetireOwned, controlled::ControlledRetirement},shared_issuance::SealedShared};
use semio_framework_value_native_owned::{native_decoding::NativeDecodeControl, native_decoding::NativeReceive, native_decoding::NativeReceiver, native_decoding::owned::NativeFlatRecord, native_decoding::owned::NativeRecordField};
#[test]
fn native_record_receive_borrows_actual_text_ordinals_and_payloads(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../../🌱️value/🛬️decode/🧳️owned/🧩️record/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let mut original=RecordValue::default();for pair in law["text"].as_array().unwrap(){let ordinal=u16::try_from(pair[0].as_u64().unwrap()).unwrap();original.fields.insert(ordinal,if let Some(text)=pair[1].as_str(){FieldValue::Text(text.into())}else{FieldValue::Int(pair[1].as_i64().unwrap())});}assert_eq!(original.field_count().unwrap(),3);
 for index in 0..3{let NativeRecordKey::Ordinal(ordinal)=original.field_key(index).unwrap()else{panic!("text field lost actual ordinal")};assert_eq!(usize::from(ordinal),index);let name=law["fields"][index]["name"].as_str().unwrap();if index<2{let text=original.text_field(index).unwrap();let FieldValue::Text(actual)=original.get(ordinal).unwrap()else{panic!("actual text changed")};assert_eq!(std::ptr::from_ref(text),std::ptr::from_ref(actual));assert_eq!(text,law["expected"][name].as_str().unwrap());}else{assert_eq!(original.number_field(index).unwrap().as_i64(),law["expected"][name].as_i64());}}
 assert!(original.field_key(3).is_err());assert!(original.text_field(2).is_err());assert!(original.number_field(0).is_err());
}

#[derive(semio_framework_value::RetireOwned)]
struct NativeRow{id:SealedShared<String>,name:SealedShared<String>,value:i64}
impl NativeFlatRecord<2,1> for NativeRow{
 const TEXT_FIELDS:[NativeRecordField;2]=[NativeRecordField{name:"id",ordinal:0},NativeRecordField{name:"name",ordinal:1}];
 const NUMBER_FIELDS:[NativeRecordField;1]=[NativeRecordField{name:"value",ordinal:2}];
 fn validation_demand(_: [&str;2],_:[Number;1])->RetirementDemand{RetirementDemand{copy_bytes:size_of::<i64>(),depth:1,..Default::default()}}
 fn validate_parts(_: [&str;2],numbers:[Number;1])->Result<(),ValueError>{numbers[0].as_i64().map(|_|()).ok_or_else(invalid)}
 fn from_parts([id,name]:[SealedShared<String>;2],numbers:[Number;1])->Self{Self{id,name,value:numbers[0].as_i64().unwrap()}}
}
semio_framework_value_native_owned::native_flat_record_receive!(NativeRow,2,1);
struct ConstructionTurn;
impl semio_framework_value_native_owned::native_decoding::NativeDecodeConstructionRecipient for ConstructionTurn{
 fn grant(&mut self,d:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{Ok(funded(d))}
 fn accept(&mut self,_:RetainedCloneGrant,_:semio_framework_value::RetainedCloneProgress)->Result<(),ValueError>{Ok(())}
}
fn funded(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096.max(d.copy_bytes),maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn drain<T:RetireOwned>(value:T){let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("native row original closure unsupported"));for _ in 0..10000{if owner.terminal_is_empty(){return;}let g=funded(owner.next_demand(4096).unwrap());assert!(owner.step(g).unwrap().progress().fits(g));}panic!("native original row closure stalled");}
fn original(value:RecordValue)->RetainedCloneSource<RecordValue>{let allocation=RetainedCloneSource::<RecordValue>::owned_constructor_demand::<()>();let d=RetirementDemand{copy_bytes:RetainedCloneSource::<RecordValue>::constructor_copy_bytes(),capacity_bytes:allocation.capacity_bytes,depth:allocation.depth,..Default::default()};RetainedCloneSource::admit_owned(value,(),funded(d)).unwrap_or_else(|_|panic!("text source admission")).0}
fn actual_input(law:&serde_json::Value)->RecordValue{let mut record=RecordValue::default();for pair in law["text"].as_array().unwrap(){record.fields.insert(u16::try_from(pair[0].as_u64().unwrap()).unwrap(),if let Some(text)=pair[1].as_str(){FieldValue::Text(text.into())}else{FieldValue::Int(pair[1].as_i64().unwrap())});}record}
#[test]
fn native_record_receive_constructs_original_text_members_and_closes_every_frontier(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../../🌱️value/🛬️decode/🧳️owned/🧩️record/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let mut finished=0;
 for cut in 0..128{
  let source=original(actual_input(&law));let foreign=original(actual_input(&law));let(mut receiver,p)=<NativeRow as NativeReceive<RecordValue>>::begin_receive(funded(<NativeRow as NativeReceive<RecordValue>>::receive_birth_demand())).unwrap();assert!(p.copied_bytes>0);let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut callback);let mut turns=0;
  while turns<cut&&!receiver.output_ready(){
   let d=receiver.demands(source.borrow(),4096).unwrap();let g=funded(d);if turns>0{assert!(receiver.demands(foreign.borrow(),4096).is_err());}
   for axis in 0..5{let mut denied=g;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes=d.copy_bytes-1,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes=d.capacity_bytes-1,3 if d.release_bytes>0=>denied.maximum_release_bytes=d.release_bytes-1,4 if d.depth>0=>denied.maximum_depth=d.depth-1,_=>continue}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||control.with_construction_recipient(&mut ConstructionTurn,|control|receiver.advance(source.borrow(),control,4096,denied)).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(receiver.demands(source.borrow(),4096).unwrap(),d);}
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||control.with_construction_recipient(&mut ConstructionTurn,|control|receiver.advance(source.borrow(),control,4096,g)).unwrap());let p=step.progress();assert!(p.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));turns+=1;
  }
  if receiver.output_ready(){finished=turns;let g=funded(receiver.take_demand().unwrap());let(row,p)=receiver.take(g).unwrap().unwrap();assert!(p.fits(g));assert_eq!(row.id.as_str(),law["expected"]["id"].as_str().unwrap());assert_eq!(row.name.as_str(),law["expected"]["name"].as_str().unwrap());assert_eq!(row.value,law["expected"]["value"].as_i64().unwrap());drain(row);}
  receiver.begin_close();for _ in 0..10000{if receiver.terminal_is_empty(){break;}let g=funded(receiver.close_demands(4096).unwrap());let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||receiver.close_step(g).unwrap());let p=step.progress();assert!(p.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));}assert!(receiver.terminal_is_empty());drain(source);drain(foreign);if finished>0&&cut>finished{break;}
 }assert!(finished>0);
}

#[derive(semio_framework_value::RetireOwned,semio_framework_value::NativeReceive)]
#[value(rename_all="camelCase")]
struct OriginalNested{label:String,count:u32}
#[derive(semio_framework_value::RetireOwned,semio_framework_value::NativeReceive)]
#[value(tag="kind",rename_all="camelCase")]
enum OriginalStatement{Nested(OriginalNested),Mark{label:String,count:u32}}
#[test]fn genuine_tagged_records_receive_original_ordinal_payloads_without_wire_conversion(){
 for tag in ["nested","mark"]{let mut record=RecordValue::default();record.fields.insert(0,FieldValue::Text("original nested".into()));record.fields.insert(1,FieldValue::UInt(7));let authority=(tag.to_string(),record);let d=RetainedCloneSource::<(String,RecordValue)>::owned_constructor_demand::<()>();let source=RetainedCloneSource::admit_owned(authority,(),funded(RetirementDemand{copy_bytes:RetainedCloneSource::<(String,RecordValue)>::constructor_copy_bytes(),capacity_bytes:d.capacity_bytes,depth:d.depth,..Default::default()})).unwrap_or_else(|_|panic!("actual statement source")).0;let input=source.borrow().project(0,|value|value as &dyn NativeValueSource);assert_eq!(input.get().variant_tag(),Some(tag));let payload=input.get().variant_payload().unwrap();assert!(matches!(payload.record_key(0).unwrap(),NativeRecordKey::Ordinal(0)));let(mut receiver,_)=<OriginalStatement as NativeReceive<dyn NativeValueSource>>::begin_receive(funded(<OriginalStatement as NativeReceive<dyn NativeValueSource>>::receive_birth_demand())).unwrap();let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut callback);for _ in 0..1024{if receiver.output_ready(){break}let d=receiver.demands(input,4096).unwrap();let denied=RetainedCloneGrant{maximum_items:0,..funded(d)};assert_eq!(receiver.advance(input,&mut control,4096,denied).unwrap().progress(),Default::default());let g=funded(d);let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||receiver.advance(input,&mut control,4096,g).unwrap());assert!(step.progress().fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));}assert!(receiver.output_ready());let(value,_)=receiver.take(funded(receiver.take_demand().unwrap())).unwrap().unwrap();match &value{OriginalStatement::Nested(row)=>{assert_eq!(row.label,"original nested");assert_eq!(row.count,7)},OriginalStatement::Mark{label,count}=>{assert_eq!(label,"original nested");assert_eq!(*count,7)}}drain(value);receiver.begin_close();for _ in 0..1024{if receiver.terminal_is_empty(){break}let d=receiver.close_demands(4096).unwrap();receiver.close_step(funded(d)).unwrap();}assert!(receiver.terminal_is_empty());drain(source);}
}
