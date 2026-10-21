use super::*;
#[path="../🧾️source/🧪️tests/🦀️.rs"]
mod original_paged;
#[path="../⏱️timestamp/🧪️tests/🦀️.rs"]
mod timestamp;
#[path="../🔣️json-string/🧪️tests/🦀️.rs"]
mod json_string;
struct Heap{requested_bytes:usize,released_bytes:usize}
fn observe<T>(operation:impl FnOnce()->T)->(T,Heap){let(result,requested_bytes,released_bytes)=semio_framework_trace_memory_testing::observe_backing(operation);(result,Heap{requested_bytes,released_bytes})}
fn close_received(mut received:Option<Vec<u8>>,grant:RetainedCloneGrant)->usize{
 let original=received.as_ref().unwrap().as_ptr();for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(result,heap)=observe(||RetainedWireFieldDecode::admit_received(&mut received,denied).unwrap());assert!(result.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(received.as_ref().unwrap().as_ptr(),original);}
 let(result,heap)=observe(||RetainedWireFieldDecode::admit_received(&mut received,grant).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(received.is_none());let recipient=result.unwrap();assert_eq!(recipient.source_identity().0,original as usize);close_recipient(recipient,grant)
}
fn close_received_text(mut received:Option<String>,grant:RetainedCloneGrant)->usize{
 let original=received.as_ref().unwrap().as_ptr();for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(result,heap)=observe(||RetainedWireFieldDecode::admit_received_text(&mut received,denied).unwrap());assert!(result.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(received.as_ref().unwrap().as_ptr(),original);}
 let(result,heap)=observe(||RetainedWireFieldDecode::admit_received_text(&mut received,grant).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(received.is_none());let recipient=result.unwrap();assert_eq!(recipient.source_identity().0,original as usize);close_recipient(recipient,grant)
}
fn close_recipient(mut recipient:RetainedWireFieldDecode,grant:RetainedCloneGrant)->usize{
 assert_eq!(recipient.take_receipt().unwrap(),(grant,RetainedCloneProgress{copied_items:1,..Default::default()}));let demand=recipient.close_demands();if demand.release_bytes!=0{let(step,heap)=observe(||recipient.close_step(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant}).unwrap());assert_eq!(step,RetainedWireFieldStep::Blocked);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
 let(_,heap)=observe(||recipient.close_step(grant).unwrap());let(received,progress)=recipient.take_receipt().unwrap();assert_eq!(received,grant);assert!(progress.fits(grant));assert_eq!(progress.copied_bytes,0);assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert!(recipient.terminal_is_empty());let released=heap.released_bytes;let(_,heap)=observe(||drop(recipient));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));released
}

#[test]
fn original_retained_wire_field_native_receipts_keep_borrowed_payload_and_actual_backing(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let p=&law["grant"];let n=|key:&str|p[key].as_u64().unwrap()as usize;let grant=RetainedCloneGrant{maximum_items:n("items"),maximum_copy_bytes:n("copy"),maximum_capacity_bytes:n("capacity"),maximum_release_bytes:n("release"),maximum_depth:n("depth")};
 for field in law["fields"].as_array().unwrap(){for cancellation in [0,1,2,32]{
  let source=field["value"].as_str().unwrap().as_bytes();let original=source.as_ptr();let(mut decoder,heap)=observe(||RetainedWireFieldDecode::new(source,field["text"].as_bool().unwrap()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(mut birth,mut release)=(0,0);if !source.is_empty(){let changed=source.to_vec();let(result,heap)=observe(||decoder.advance(&changed,grant));assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(decoder.receipt().is_none());}
  for _ in 0..cancellation{
   if decoder.is_ready(){break}let demand=decoder.demands(source,grant.maximum_copy_bytes).unwrap();for(axis,denied)in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant}].into_iter().enumerate(){if axis==2&&demand.capacity_bytes==0{continue}let(step,heap)=observe(||decoder.advance(source,denied).unwrap());assert_eq!(step,RetainedWireFieldStep::Blocked);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
   let(_,heap)=observe(||decoder.advance(source,grant).unwrap());let(received,progress)=decoder.receipt().unwrap();assert_eq!(received,grant);assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));birth+=heap.requested_bytes;release+=heap.released_bytes;assert_eq!(decoder.source_identity(),(original as usize,source.len()));
   let(step,heap)=observe(||decoder.advance(source,grant).unwrap());assert_eq!(step,RetainedWireFieldStep::Blocked);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));decoder.take_receipt().unwrap();
  }
  if cancellation==32&&decoder.is_ready(){
   if field["text"].as_bool().unwrap(){let mut recipient=None;let(_,heap)=observe(||decoder.take_text_into(&mut recipient,grant).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!decoder.terminal_is_empty());assert_eq!(decoder.take_receipt().unwrap().1.copied_bytes,0);assert_eq!(recipient.as_ref().unwrap().as_bytes(),source);assert_eq!(serde_json::to_value(recipient.as_ref().unwrap()).unwrap(),field["value"]);release+=close_received_text(recipient,grant);}
   else{let mut recipient=None;let(_,heap)=observe(||decoder.take_into(&mut recipient,grant).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!decoder.terminal_is_empty());assert_eq!(decoder.take_receipt().unwrap().1.copied_bytes,0);assert_eq!(recipient.as_ref().unwrap(),source);release+=close_received(recipient,grant);}
  }
  else{decoder.cancel();let demand=decoder.demands(source,grant.maximum_copy_bytes).unwrap();if demand.release_bytes!=0{let(step,heap)=observe(||decoder.close_step(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant}).unwrap());assert_eq!(step,RetainedWireFieldStep::Blocked);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}let(_,heap)=observe(||decoder.close_step(grant).unwrap());let(received,progress)=decoder.receipt().unwrap();assert_eq!(received,grant);assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));birth+=heap.requested_bytes;release+=heap.released_bytes;assert!(!decoder.terminal_is_empty());decoder.take_receipt().unwrap();}
  assert!(decoder.terminal_is_empty());assert_eq!(birth,release);let(_,heap)=observe(||drop(decoder));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 }}
}

#[test]
fn original_prefixed_uuid_native_writes_only_funded_original_range_pieces(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🪪️id/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let prefix=law["prefix"].as_str().unwrap();let mut source=prefix.as_bytes().to_vec();let start=source.len();for pair in law["uuidHex"].as_str().unwrap().as_bytes().chunks_exact(2){source.push(u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap())}let p=&law["grant"];let n=|key:&str|p[key].as_u64().unwrap()as usize;let grant=RetainedCloneGrant{maximum_items:n("items"),maximum_copy_bytes:n("copy"),maximum_capacity_bytes:n("capacity"),maximum_release_bytes:n("release"),maximum_depth:n("depth")};
 for cancellation in [0,1,2,7,32]{let(result,heap)=observe(||RetainedWireFieldDecode::new_prefixed_uuid(&source,(0,start),(start,source.len())));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut decoder=result.unwrap();let(mut birth,mut release,mut copied)=(0,0,0);for _ in 0..cancellation{if decoder.is_ready(){break}let demand=decoder.demands(&source,grant.maximum_copy_bytes).unwrap();for(axis,denied)in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant},RetainedCloneGrant{maximum_copy_bytes:0,..grant}].into_iter().enumerate(){if(axis==2&&demand.capacity_bytes==0)||(axis==3&&demand.copy_bytes==0){continue}let(step,heap)=observe(||decoder.advance(&source,denied).unwrap());assert_eq!(step,RetainedWireFieldStep::Blocked);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}let(_,heap)=observe(||decoder.advance(&source,grant).unwrap());let(received,progress)=decoder.take_receipt().unwrap();assert_eq!(received,grant);assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));birth+=heap.requested_bytes;release+=heap.released_bytes;copied+=progress.copied_bytes;}
 if cancellation==32{assert!(decoder.is_ready());let mut recipient=None;let(_,heap)=observe(||decoder.take_into(&mut recipient,grant).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(decoder.take_receipt().unwrap().1.copied_bytes,0);assert_eq!(recipient.as_ref().unwrap(),law["expected"].as_str().unwrap().as_bytes());assert_eq!(copied,law["expected"].as_str().unwrap().len());release+=close_received(recipient,grant);}else{decoder.cancel();let(_,heap)=observe(||decoder.close_step(grant).unwrap());let(received,progress)=decoder.take_receipt().unwrap();assert_eq!(received,grant);assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));release+=heap.released_bytes;}
 assert!(decoder.terminal_is_empty());assert_eq!(birth,release);let(_,heap)=observe(||drop(decoder));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
}


#[test]
fn original_fallible_wire_copy_retains_exact_performed_prefix_and_same_physical_backing(){
 use std::cell::Cell;
 struct Fault<'a>{bytes:&'a[u8],reads:Cell<usize>,fail_at:usize}
 impl RetainedWireByteSource for Fault<'_>{
  fn original_source_identity(&self)->OriginalWireSourceIdentity{self.bytes.original_source_identity()}
  fn byte_at(&self,index:usize)->Option<u8>{let turn=self.reads.get();self.reads.set(turn+1);if turn==self.fail_at{None}else{self.bytes.get(index).copied()}}
 }
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/🛑️byte.json")).unwrap();let policy=&law["grant"];let n=|key:&str|policy[key].as_u64().unwrap()as usize;let grant=RetainedCloneGrant{maximum_items:n("maximumItems"),maximum_copy_bytes:n("maximumCopyBytes"),maximum_capacity_bytes:n("maximumCapacityBytes"),maximum_release_bytes:n("maximumReleaseBytes"),maximum_depth:n("maximumDepth")};let hex=law["sourceHex"].as_str().unwrap();let bytes=hex.as_bytes().chunks_exact(2).map(|pair|u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()).collect::<Vec<_>>();let wire=bytes.as_ptr();
 for row in law["failures"].as_array().unwrap(){let offset=row["copyOffset"].as_u64().unwrap()as usize;let source=Fault{bytes:&bytes,reads:Cell::new(0),fail_at:bytes.len()+offset};let mut decoder=RetainedWireFieldDecode::new(&source,false);let(_,birth)=observe(||decoder.advance(&source,grant).unwrap());let(received,progress)=decoder.take_receipt().unwrap();assert_eq!(received,grant);assert_eq!((birth.requested_bytes,birth.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));let retained=decoder.output.as_ref().unwrap().as_ptr();
 let(result,heap)=observe(||decoder.advance(&source,grant));let error=result.unwrap_err();assert_eq!(error.kind,ValueRefusalKind::InvalidValue);assert_eq!(error.message,"original wire byte is absent");assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let expected=RetainedCloneProgress{copied_items:usize::from(offset!=0),copied_bytes:offset,..Default::default()};assert_eq!(error.retained_progress(),expected);assert_eq!(decoder.receipt(),Some((grant,expected)));assert_eq!(decoder.output.as_ref().unwrap().as_ptr(),retained);assert_eq!(decoder.output.as_ref().unwrap().as_slice(),&bytes[..offset]);assert_eq!(bytes.as_ptr(),wire);assert_eq!(decoder.advance(&source,grant).unwrap(),RetainedWireFieldStep::Blocked);assert_eq!(decoder.take_receipt(),Some((grant,expected)));
 decoder.cancel();let(_,close)=observe(||decoder.close_step(grant).unwrap());let(received,progress)=decoder.take_receipt().unwrap();assert_eq!(received,grant);assert_eq!((close.requested_bytes,close.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert_eq!(birth.requested_bytes,close.released_bytes);assert!(decoder.terminal_is_empty());let(_,drop)=observe(||std::mem::drop(decoder));assert_eq!((drop.requested_bytes,drop.released_bytes),(0,0));
 }
}

#[test]
fn original_wire_retirement_selects_exact_buffer_release_and_terminal_drop(){
 use semio_framework_value::retirement::{RetireOwned,RetirementStep};
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let rule=&law["retirement"];let p=&law["grant"];let n=|key:&str|p[key].as_u64().unwrap()as usize;let forward=RetainedCloneGrant{maximum_items:n("items"),maximum_copy_bytes:n("copy"),maximum_capacity_bytes:n("capacity"),maximum_release_bytes:n("release"),maximum_depth:n("depth")};
 for field in law["fields"].as_array().unwrap(){for cut in rule["cuts"].as_array().unwrap(){
  let source=field["value"].as_str().unwrap().as_bytes();let mut decoder=RetainedWireFieldDecode::new(source,field["text"].as_bool().unwrap());
  for _ in 0..cut.as_u64().unwrap(){decoder.take_receipt();if decoder.is_ready(){break;}decoder.advance(source,forward).unwrap();}
  let capacity=decoder.output.as_ref().map_or(0,Vec::capacity);let expected=RetirementDemand{copy_bytes:rule["copyBytes"].as_u64().unwrap()as usize,capacity_bytes:rule["capacityBytes"].as_u64().unwrap()as usize,release_bytes:capacity,depth:rule["liveDepth"].as_u64().unwrap()as usize};let wrapper=decoder.retirement_birth_bytes().unwrap();
  let(mut cursor,heap)=observe(||decoder.retirement());assert_eq!((heap.requested_bytes,heap.released_bytes),(wrapper,0));assert!(!cursor.terminal_is_empty());
  for body in rule["bodies"].as_array().unwrap(){let(d,heap)=observe(||cursor.next_demand(body.as_u64().unwrap()as usize).unwrap());assert_eq!(d,expected);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:expected.copy_bytes,maximum_capacity_bytes:expected.capacity_bytes,maximum_release_bytes:expected.release_bytes,maximum_depth:expected.depth};
  for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),Some(RetainedCloneGrant{maximum_depth:0,..grant}),(capacity>0).then(||RetainedCloneGrant{maximum_release_bytes:capacity-1,..grant})].into_iter().flatten(){let(step,heap)=observe(||cursor.close_step(denied));assert!(matches!(step,RetirementStep::BudgetExhausted));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cursor.next_demand(0).unwrap(),expected);}
  let(step,heap)=observe(||cursor.close_step(grant));let RetirementStep::Progress(progress)=step else{panic!("funded original wire retirement must progress")};assert!(progress.fits(grant));assert_eq!(progress.released_bytes,capacity);assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert!(cursor.terminal_is_empty());assert_eq!(cursor.next_demand(4096).unwrap(),RetirementDemand::default());
  assert_eq!(cursor.terminal_release_bytes(),Some(wrapper));let(_,heap)=observe(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,wrapper));
 }}
}
