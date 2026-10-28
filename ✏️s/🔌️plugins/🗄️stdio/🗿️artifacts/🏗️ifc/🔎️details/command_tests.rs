//! 🧪️ Original command wire parsing and field movement preserve every cancellation owner.
use super::*;
use semio_framework_plugin::app_testing::OriginalWorkFixture;
fn grant()->RetainedCloneGrant {RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:262144,maximum_release_bytes:1048576,maximum_depth:256}}
fn close(caller:&mut OriginalWorkFixture,reader:&mut CommandWire){
 reader.begin_close();
 for _ in 0..100_000 {if reader.terminal_is_empty(){return;}caller.turn(grant(),|cx|cx.with_original_retirement(|cx|{semio_framework_job::close_original_retained_step(cx,reader.close_demands()?,|g|reader.close_step(g))?;Ok(())})).unwrap();}
 panic!("original wire receiver close did not drain")
}
fn run(wire:&[u8],stop:Option<usize>,expected:Option<&editing::SnapshotEditEvent>)->usize {
 let mut caller=OriginalWorkFixture::new();let mut reader=CommandWire::default();let original=wire.as_ptr();let mut turns=0;let mut ready=false;let mut moved_text=None;
 for turn in 1..100_000 {turns=turn;
  let demand=reader.demands(wire).unwrap();let before=(reader.position(),reader.ready());
  let short=RetainedCloneGrant{maximum_items:0,..grant()};
  let (unchanged,heap)=caller.work_turn(short,1,|cx|{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||reader.advance(wire,cx));Ok((result?,heap))}).unwrap();
  assert!(!unchanged);assert_eq!((reader.position(),reader.ready()),before);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(reader.demands(wire).unwrap(),demand);
  if reader.phase==Phase::Construct {if let Some(DslValue::String(value))=&reader.fields[1]{moved_text=Some(value.as_ptr());}}
  if stop==Some(turn){caller.cancel().cancel_now();}
  let result=caller.work_turn(grant(),1,|cx|{let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||reader.advance(wire,cx));let after=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));assert!(!heap.overflowed);Ok(result)}).unwrap();
  match result{Ok(true)=>{ready=true;break;},Ok(false)=>{},Err(_)=>{assert!(stop.is_some()||expected.is_none());break;}}
  assert_eq!(wire.as_ptr(),original);
 }
 assert_eq!(ready,stop.is_none()&&expected.is_some());
 if ready {let value=reader.output.as_ref().unwrap();assert_eq!(value,expected.unwrap());if let editing::SnapshotEditEvent::SetValue{value:DslValue::String(value),..}=value{assert_eq!(Some(value.as_ptr()),moved_text);}}
 close(&mut caller,&mut reader);caller.finish();turns
}
#[test]
fn retained_wire_refuses_a_rebound_original_before_parsing_or_losing_custody(){
 let wire=b"{\"EditSnapshot\":{\"event\":{\"operation\":\"removeValue\",\"path\":\"/entities/0\"}}}".to_vec();let foreign=wire.clone();let mut caller=OriginalWorkFixture::new();let mut reader=CommandWire::default();
 assert!(!caller.work_turn(grant(),1,|cx|reader.advance(&wire,cx)).unwrap());let before=(reader.position(),reader.source,reader.phase as u8);
 let (result,heap)=caller.work_turn(grant(),1,|cx|{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||reader.advance(&foreign,cx));Ok((result,heap))}).unwrap();
 assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!((reader.position(),reader.source,reader.phase as u8),before);
 caller.cancel().cancel_now();close(&mut caller,&mut reader);caller.finish();
}
#[test]
fn neutral_original_command_wire_receives_exact_events_and_closes_every_cancelled_prefix(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/command-wire.json")).unwrap();
 for row in fixture["valid"].as_array().unwrap(){let wire=row.to_string();let event=editing::SnapshotEditEvent::from_value(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&row["EditSnapshot"]["event"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())).unwrap();let turns=run(wire.as_bytes(),None,Some(&event));for prefix in 1..=turns{run(wire.as_bytes(),Some(prefix),Some(&event));}}
 for row in fixture["invalid"].as_array().unwrap(){run(row.to_string().as_bytes(),None,None);}
 for wire in fixture["invalidWire"].as_array().unwrap(){run(wire.as_str().unwrap().as_bytes(),None,None);}
}
