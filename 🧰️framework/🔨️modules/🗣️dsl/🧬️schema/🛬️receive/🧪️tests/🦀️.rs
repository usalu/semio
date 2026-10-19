use super::*;
use semio_framework_value::{NativeDecodeControl,RetirementDemand,native_decoding::{NativeReceive,NativeReceiver,owned::{NativeFlatRecord,NativeRecordField}},retained_clone::{RetainedCloneGrant,RetainedCloneSource},retirement::{RetireOwned,controlled::ControlledRetirement,shared::sealed::SealedShared}};
#[test]
fn native_record_receive_borrows_actual_text_ordinals_and_payloads(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../../🌱️value/🛬️decode/🧳️owned/🧩️record/🧫️fixtures/🔣️.json")).unwrap();let mut original=RecordValue::default();for pair in law["text"].as_array().unwrap(){let ordinal=u16::try_from(pair[0].as_u64().unwrap()).unwrap();original.fields.insert(ordinal,if let Some(text)=pair[1].as_str(){FieldValue::Text(text.into())}else{FieldValue::Int(pair[1].as_i64().unwrap())});}assert_eq!(original.field_count().unwrap(),3);
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
semio_framework_value::native_flat_record_receive!(NativeRow,2,1);
fn funded(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096.max(d.copy_bytes),maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn drain<T:RetireOwned>(value:T){let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("native row original closure unsupported"));for _ in 0..10000{if owner.terminal_is_empty(){return;}let g=funded(owner.next_demand(4096).unwrap());assert!(owner.step(g).unwrap().progress().fits(g));}panic!("native original row closure stalled");}
fn original(value:RecordValue)->RetainedCloneSource<RecordValue>{let allocation=RetainedCloneSource::<RecordValue>::owned_constructor_demand::<()>();let d=RetirementDemand{copy_bytes:RetainedCloneSource::<RecordValue>::constructor_copy_bytes(),capacity_bytes:allocation.capacity_bytes,depth:allocation.depth,..Default::default()};RetainedCloneSource::admit_owned(value,(),funded(d)).unwrap_or_else(|_|panic!("text source admission")).0}
fn actual_input(law:&serde_json::Value)->RecordValue{let mut record=RecordValue::default();for pair in law["text"].as_array().unwrap(){record.fields.insert(u16::try_from(pair[0].as_u64().unwrap()).unwrap(),if let Some(text)=pair[1].as_str(){FieldValue::Text(text.into())}else{FieldValue::Int(pair[1].as_i64().unwrap())});}record}
#[test]
fn native_record_receive_constructs_original_text_members_and_closes_every_frontier(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../../🌱️value/🛬️decode/🧳️owned/🧩️record/🧫️fixtures/🔣️.json")).unwrap();let mut finished=0;
 for cut in 0..128{
  let source=original(actual_input(&law));let foreign=original(actual_input(&law));let(mut receiver,p)=<NativeRow as NativeReceive<RecordValue>>::begin_receive(funded(<NativeRow as NativeReceive<RecordValue>>::receive_birth_demand())).unwrap();assert!(p.copied_bytes>0);let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut callback);let mut turns=0;
  while turns<cut&&!receiver.output_ready(){
   let d=receiver.demands(source.borrow(),4096).unwrap();let g=funded(d);if turns>0{assert!(receiver.demands(foreign.borrow(),4096).is_err());}
   for axis in 0..5{let mut denied=g;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes=d.copy_bytes-1,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes=d.capacity_bytes-1,3 if d.release_bytes>0=>denied.maximum_release_bytes=d.release_bytes-1,4 if d.depth>0=>denied.maximum_depth=d.depth-1,_=>continue}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||receiver.advance(source.borrow(),&mut control,4096,denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(receiver.demands(source.borrow(),4096).unwrap(),d);}
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||receiver.advance(source.borrow(),&mut control,4096,g).unwrap());let p=step.progress();assert!(p.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));turns+=1;
  }
  if receiver.output_ready(){finished=turns;let g=funded(receiver.take_demand().unwrap());let(row,p)=receiver.take(g).unwrap().unwrap();assert!(p.fits(g));assert_eq!(row.id.as_str(),law["expected"]["id"].as_str().unwrap());assert_eq!(row.name.as_str(),law["expected"]["name"].as_str().unwrap());assert_eq!(row.value,law["expected"]["value"].as_i64().unwrap());drain(row);}
  receiver.begin_close();for _ in 0..10000{if receiver.terminal_is_empty(){break;}let g=funded(receiver.close_demands(4096).unwrap());let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||receiver.close_step(g).unwrap());let p=step.progress();assert!(p.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));}assert!(receiver.terminal_is_empty());drain(source);drain(foreign);if finished>0&&cut>finished{break;}
 }assert!(finished>0);eprintln!("[DEBUG] text NativeReceive original record: {finished} normal turns, every frontier and all physical denial axes");
}
