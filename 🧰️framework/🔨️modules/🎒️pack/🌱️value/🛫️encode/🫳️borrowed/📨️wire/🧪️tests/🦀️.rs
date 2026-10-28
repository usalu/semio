//! 🧪️ The normal owned Pack wire uses one original finite caller and returns its exact source after scaffold closure.
use pack::record::{OwnedProjectedPackWireCursor,OwnedProjectedPackWireReady};
use semio_framework_dsl_record::native_encoding::{FieldProjectionSource,FieldProjectionView as View};
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneGrant as Grant,RetainedCloneProgress as Progress,RetainedCloneStep,RetirementDemand};
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};
use semio_framework_value::retained_clone::{RetainedCloneSource,RetainedCloneSourceTake};
use semio_framework_job::{OriginalJobFixtureCaller,OriginalTurnFailure,OperationId,Generation};
#[global_allocator]
static ORIGINAL_WIRE_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
fn observe<T>(run:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);(value,(heap.requested_bytes,heap.released_bytes))}
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("./🧫️fixtures/🔣️.json")).unwrap()}
fn stock()->Grant{Grant{maximum_items:100000,maximum_copy_bytes:1000000,maximum_capacity_bytes:4000000,maximum_release_bytes:4000000,maximum_depth:128}}
fn policy(copy:usize)->Grant{Grant{maximum_items:1,maximum_copy_bytes:copy,..stock()}}
fn funded(body:usize,d:RetirementDemand)->Grant{assert!(d.copy_bytes<=4096&&d.capacity_bytes<=4096);Grant{maximum_items:1,maximum_copy_bytes:body.max(d.copy_bytes),maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)}}
fn lease_election_copy_bytes()->usize{let law=fixture();let lease=&law["leaseElection"];let total=lease["handleBytes"].as_u64().unwrap()as usize+lease["payloadMoves"].as_u64().unwrap()as usize*lease["optionalPayloadBytes"].as_u64().unwrap()as usize;assert_eq!(total,lease["copyBytes"].as_u64().unwrap()as usize);total}
fn caller()->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OperationId(942),Generation(18),stock(),||Some(1000),1000000,100000)}
fn absent()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"original test projection ordinal is absent")}
#[derive(semio_framework_value::RetireOwned)]
struct OriginalSource{title:String,values:Vec<i64>,node_hash:u64,wide:[u64;64]}
impl OriginalSource{fn new()->Self{Self{title:"Original source 源".into(),values:vec![-17,0,9],node_hash:u64::MAX-2,wide:[73;64]}}}
impl FieldProjectionSource for OriginalSource{
 fn projection_identity(&self)->usize{self.title.as_ptr()as usize}
 fn projection_view(&self,path:&[usize])->Result<View<'_>,ValueError>{match path{[]=>Ok(View::IntrinsicObject(3)),[0]=>Ok(View::IntrinsicNumber(semio_framework_value::Number::UInt(self.node_hash))),[1]=>Ok(View::IntrinsicText(&self.title)),[2]=>Ok(View::IntrinsicArray(self.values.len())),[2,index]=>self.values.get(*index).map(|value|View::IntrinsicNumber(semio_framework_value::Number::Int(*value))).ok_or_else(absent),_=>Err(absent())}}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{if path.is_empty(){["nodeHash","title","values"].get(index).copied().ok_or_else(absent)}else{Err(absent())}}
}
fn close_physical<T:RetireOwned>(original:T,caller:&mut OriginalJobFixtureCaller,copy:usize,allocated:&mut usize,released:&mut usize){
 let mut owner=Some(original);let mut active=None;let demand=RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<T>(),depth:1,..Default::default()};
 let((closed,receipt),heap)=observe(||caller.turn(funded(copy,demand),|cx|{let grant=cx.retained_grant();assert!(grant.maximum_capacity_bytes>=demand.capacity_bytes);let source=owner.take().unwrap();let (closed,receipt)=semio_framework_value::retirement::admit_owned_retirement(source,grant).unwrap_or_else(|_|panic!("original test physical admission"));cx.consume_retained(receipt)?;cx.consume_fuel(1);Ok((closed,receipt))}).unwrap());assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));*allocated+=heap.0;*released+=heap.1;active=Some(closed);
 for _ in 0..100000{if active.is_none(){return}let(result,heap)=observe(||caller.turn(funded(copy,semio_framework_value::factory_ticket_demands(active.as_ref().unwrap(),copy).unwrap()),|cx|{let step=semio_framework_value::close_factory_ticket(&mut active,cx.retained_grant()).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);Ok(step)}));let step=result.unwrap();assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));*allocated+=heap.0;*released+=heap.1;}panic!("original physical source closure stalled")
}

#[test]
fn original_owned_source_header_returns_without_payload_copy(){
 for copy in [1,3,64]{let mut caller=caller();let((original,expected_title,expected_values),input)=observe(||{let original=OriginalSource::new();let title=original.title.as_ptr();let values=original.values.as_ptr();(original,title,values)});let mut allocated=input.0+caller.born_bytes();let mut released=input.1;let mut original=Some(original);
  let((mut source,receipt),heap)=observe(||caller.turn(policy(copy),|cx|{let(source,receipt)=RetainedCloneSource::admit_owned(original.take().unwrap(),(),cx.retained_grant()).unwrap_or_else(|_|panic!("original source admission"));cx.consume_retained(receipt)?;cx.consume_fuel(1);Ok((source,receipt))}).unwrap());assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));allocated+=heap.0;released+=heap.1;let mut returned=None;
  for _ in 0..100000{assert_eq!(source.original_authority().unwrap().title.as_ptr(),expected_title);assert_eq!(source.original_authority().unwrap().values.as_ptr(),expected_values);let(result,heap)=observe(||caller.turn(funded(copy,source.next_take_demand(copy).unwrap()),|cx|{let step=source.take_authority(cx.retained_grant())?;let receipt=match &step{RetainedCloneSourceTake::Pending(receipt)|RetainedCloneSourceTake::Ready(_,receipt)=>*receipt};cx.consume_retained(receipt)?;cx.consume_fuel(receipt.copied_items as u64);Ok(step)}));let step=result.unwrap();let receipt=caller.receipt();assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));allocated+=heap.0;released+=heap.1;match step{RetainedCloneSourceTake::Pending(_)=>{},RetainedCloneSourceTake::Ready(original,receipt)=>{assert_eq!(receipt.copied_bytes,0,"original owned authority donation copies no business header");returned=Some(original);break}}}
  let original=returned.expect("original owned source must return under fixed body1/3/64");assert!(source.terminal_is_empty());assert_eq!(original.title.as_ptr(),expected_title);assert_eq!(original.values.as_ptr(),expected_values);assert_eq!(original.wide,[73;64]);assert_eq!(observe(||drop(source)).1,(0,0));close_physical(original,&mut caller,copy,&mut allocated,&mut released);released+=caller.retire(policy(copy));assert_eq!(allocated,released);
 }
}

#[test]
fn original_owned_pack_wire_returns_same_source_and_matches_normal_decoder(){
 let law=fixture();for copy in law["copyGrants"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){let mut caller=caller();let((original,title,values),input)=observe(||{let original=OriginalSource::new();let title=original.title.as_ptr();let values=original.values.as_ptr();(original,title,values)});let mut allocated=input.0+caller.born_bytes();let mut released=input.1;let mut writer=OwnedProjectedPackWireCursor::new(original,1);let mut ready=None;
  for turn in 0..100000{if let Some(original)=writer.original(){assert_eq!(original.title.as_ptr(),title);assert_eq!(original.values.as_ptr(),values)}let demand=writer.next_demand(copy).unwrap();let before=caller.cumulative_receipt();let fuel=caller.fuel_remaining();for denied in [Grant{maximum_items:0,..policy(copy)},Grant{maximum_depth:0,..policy(copy)}]{let(result,heap)=observe(||caller.turn(denied,|cx|{let step=writer.advance(cx.retained_grant()).unwrap();assert!(step.ready.is_none());assert_eq!(step.progress,Default::default());Ok(())}));result.unwrap();assert_eq!(heap,(0,0));assert_eq!(caller.cumulative_receipt(),before);assert_eq!(caller.fuel_remaining(),fuel)}
   if demand.capacity_bytes>0{let(result,heap)=observe(||caller.turn(Grant{maximum_capacity_bytes:demand.capacity_bytes-1,..policy(copy)},|cx|{let step=writer.advance(cx.retained_grant()).unwrap();assert!(step.ready.is_none());assert_eq!(step.progress,Default::default());Ok(())}));result.unwrap();assert_eq!(heap,(0,0));assert_eq!(caller.cumulative_receipt(),before)}
   let(result,heap)=observe(||caller.turn(funded(copy,writer.next_demand(copy).unwrap()),|cx|{let step=writer.advance(cx.retained_grant()).unwrap();cx.consume_retained(step.progress)?;cx.consume_fuel(step.progress.copied_items as u64);Ok(step)}));let step=result.unwrap();assert_eq!(heap,(step.progress.retained_capacity_bytes,step.progress.released_bytes));allocated+=heap.0;released+=heap.1;if let Some(original)=step.ready{ready=Some(original);break}assert!(turn<99999,"original owned Pack writer stalled")
  }
  let ready:OwnedProjectedPackWireReady<OriginalSource>=ready.unwrap();assert!(writer.terminal_is_empty());assert_eq!(ready.source.title.as_ptr(),title);assert_eq!(ready.source.values.as_ptr(),values);assert_eq!(ready.source.wide,[73;64]);let decoded=pack::record::decode_value_record_body_exact(&ready.wire.to_vec_owner(),1,&pack::PackLimits::default()).unwrap();assert_eq!(decoded,semio_framework_value::DslValue::Object(vec![("nodeHash".into(),semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(u64::MAX-2))),("title".into(),semio_framework_value::DslValue::String("Original source 源".into())),("values".into(),semio_framework_value::DslValue::Array([-17,0,9].into_iter().map(|value|semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(value))).collect()))]));assert_eq!(observe(||drop(writer)).1,(0,0));close_physical(ready,&mut caller,copy,&mut allocated,&mut released);released+=caller.retire(policy(copy));assert_eq!(allocated,released);
 }
}

#[test]
fn original_owned_pack_wire_cancellation_closes_all_original_backing(){
 for copy in [1,3,64]{for stop in [0,1,3,17,83]{let mut caller=caller();let(original,input)=observe(OriginalSource::new);let mut allocated=input.0+caller.born_bytes();let mut released=input.1;let mut writer=OwnedProjectedPackWireCursor::new(original,1);for _ in 0..stop{let(step,heap)=observe(||caller.turn(funded(copy,writer.next_demand(copy).unwrap()),|cx|{let step=writer.advance(cx.retained_grant()).unwrap();assert!(step.ready.is_none());cx.consume_retained(step.progress)?;cx.consume_fuel(step.progress.copied_items as u64);Ok(step)}).unwrap());assert_eq!(heap,(step.progress.retained_capacity_bytes,step.progress.released_bytes));allocated+=heap.0;released+=heap.1;}
   semio_framework_async::poll::resolve_ready(caller.cancel().cancel());writer.begin_close();for turn in 0..100000{if writer.terminal_is_empty(){break}let demand=writer.next_demand(copy).unwrap();if demand.release_bytes>0{let(result,heap)=observe(||caller.turn(Grant{maximum_release_bytes:demand.release_bytes-1,..policy(copy)},|cx|{let step=writer.close_step(cx.retained_grant()).unwrap();assert_eq!(step.progress(),Default::default());Ok(())}));result.unwrap();assert_eq!(heap,(0,0));}
    let(step,heap)=observe(||caller.turn(funded(copy,writer.next_demand(copy).unwrap()),|cx|{let step=writer.close_step(cx.retained_grant()).unwrap();cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);Ok(step)}).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));allocated+=heap.0;released+=heap.1;assert!(turn<99999,"original cancellation closure stalled");}
   assert!(writer.terminal_is_empty());assert_eq!(observe(||drop(writer)).1,(0,0));released+=caller.retire(policy(copy));assert_eq!(allocated,released);
  }}
}

#[test]
fn original_owned_pack_wire_physical_retirement_closes_actual_native_fields(){
 for copy in [1,3,64]{for stop in [0,1,3,17,83]{let mut caller=caller();let(original,input)=observe(OriginalSource::new);let mut allocated=input.0+caller.born_bytes();let mut released=input.1;let mut writer=OwnedProjectedPackWireCursor::new(original,1);
  for _ in 0..stop{let(step,heap)=observe(||caller.turn(funded(copy,writer.next_demand(copy).unwrap()),|cx|{let step=writer.advance(cx.retained_grant()).unwrap();assert!(step.ready.is_none());cx.consume_retained(step.progress)?;cx.consume_fuel(step.progress.copied_items as u64);Ok(step)}).unwrap());assert_eq!(heap,(step.progress.retained_capacity_bytes,step.progress.released_bytes));allocated+=heap.0;released+=heap.1;}
  semio_framework_async::poll::resolve_ready(caller.cancel().cancel());close_physical(writer,&mut caller,copy,&mut allocated,&mut released);released+=caller.retire(policy(copy));assert_eq!(allocated,released,"original physical copy{copy} stop{stop}");
 }}
}

#[derive(semio_framework_value::RetireOwned)]
struct OriginalOctets{bytes:semio_framework_value::paged::PagedBytes<{isize::MAX as usize}>}
impl FieldProjectionSource for OriginalOctets{
 fn projection_view(&self,path:&[usize])->Result<View<'_>,ValueError>{if path.is_empty(){Ok(View::IntrinsicBytesSource(&self.bytes))}else{Err(absent())}}
}
#[test]
fn original_paged_octets_wire_reads_original_pages_and_closes_every_cancel_prefix(){
 let law=fixture();let input=law["pagedBytes"]["pattern"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect::<Vec<_>>();let length=law["pagedBytes"]["length"].as_u64().unwrap()as usize;
 let expected=(0..length).map(|index|input[index%input.len()]).collect::<Vec<_>>();
 for stop in [Some(0),Some(1),Some(17),Some(83),None]{let mut caller=caller();let(original,heap)=observe(||OriginalOctets{bytes:semio_framework_value::paged::PagedBytes::try_from_slice(&expected).unwrap()});let pointer=original.bytes.retained_bytes().backing_ptr(0);let mut allocated=heap.0+caller.born_bytes();let mut released=heap.1;let mut writer=OwnedProjectedPackWireCursor::new(original,1);let mut ready=None;
  for turn in 0..200000{if stop==Some(turn){break}if let Some(original)=writer.original(){assert_eq!(original.bytes.retained_bytes().backing_ptr(0),pointer)}let(result,heap)=observe(||caller.turn(funded(64,writer.next_demand(64).unwrap()),|cx|{let step=writer.advance(cx.retained_grant()).unwrap();cx.consume_retained(step.progress)?;cx.consume_fuel(step.progress.copied_items as u64);Ok(step)}));let step=result.unwrap();assert_eq!(heap,(step.progress.retained_capacity_bytes,step.progress.released_bytes));allocated+=heap.0;released+=heap.1;if let Some(value)=step.ready{assert!(turn<4096,"original paged bytes must emit admitted fragments rather than one byte per turn");ready=Some(value);break}assert!(turn<199999);}
  if let Some(ready)=ready{assert!(stop.is_none());assert_eq!(ready.source.bytes.retained_bytes().backing_ptr(0),pointer);let decoded=pack::record::decode_value_record_body_exact(&ready.wire.to_vec_owner(),1,&pack::PackLimits::default()).unwrap();assert_eq!(decoded,semio_framework_value::DslValue::Bytes(expected.clone()));close_physical(ready,&mut caller,64,&mut allocated,&mut released);}else{assert!(stop.is_some());semio_framework_async::poll::resolve_ready(caller.cancel().cancel());writer.begin_close();for turn in 0..200000{if writer.terminal_is_empty(){break}let(step,heap)=observe(||caller.turn(funded(64,writer.next_demand(64).unwrap()),|cx|{let step=writer.close_step(cx.retained_grant()).unwrap();cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);Ok(step)}).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));allocated+=heap.0;released+=heap.1;assert!(turn<199999);}}
  assert!(writer.terminal_is_empty());assert_eq!(observe(||drop(writer)).1,(0,0));released+=caller.retire(policy(64));assert_eq!(allocated,released);
 }
}
