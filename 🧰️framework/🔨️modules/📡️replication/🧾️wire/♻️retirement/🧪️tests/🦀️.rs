//! 🧪️ Exact original Protocol causes retire without invented copying or hidden physical release.
use super::*;
use semio_framework_trace::observe_heap_allocations_on_this_thread;
use semio_framework_pack_error::{PackError,PackRefusal,PackRetryDisposition,PackTransportCategory,TransportAdmission,reserve_transport};
use semio_framework_diagnostic::{TextError,TextSpan};
fn owned(row:&serde_json::Value,index:usize)->String{
 let mut text=String::with_capacity(row["capacities"][index].as_u64().unwrap() as usize);
 text.push_str(row["texts"][index].as_str().unwrap());text
}
fn cause(row:&serde_json::Value)->ProtocolError{
 match row["kind"].as_str().unwrap(){
  "io"=>ProtocolError::Io(owned(row,0)),
  "malformed"=>ProtocolError::Malformed{what:"wire",offset:7,detail:owned(row,0)},
  "packMalformed"=>PackRefusal::Malformed{kind:ValueRefusalKind::InvalidValue,what:"body",offset:9,detail:owned(row,0)}.into(),
  "value"=>PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::InvalidValue,owned(row,0))).into(),
  "text"=>PackRefusal::TextRefusal(TextError{kind:ValueRefusalKind::InvalidValue,message:owned(row,0),span:TextSpan::with_length(2,3,4),expected:Some(owned(row,1))}).into(),
  "borrowed"=>PackRefusal::ValueRefusal(ValueError::literal(ValueRefusalKind::Canceled,"original borrowed cause")).into(),
  "scalar"=>ProtocolError::DictOutOfOrder{expected:3,actual:2},
  _=>panic!("unregistered neutral error variant"),
 }
}
fn exact(demand:semio_framework_value::RetirementDemand)->RetainedCloneGrant{
 RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}
}
#[test]
fn protocol_original_cause_retirement_preserves_refusal_and_exact_physical_axes(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){for pause in law["pauses"].as_array().unwrap(){
  let mut original=Some(cause(row));let before=original.as_ref().unwrap().to_string();
  let escaped=serde_json::to_string(&before).unwrap();assert_eq!(serde_json::from_str::<String>(&escaped).unwrap(),before);
  let mut released=0usize;let expected:usize=row["capacities"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as usize).sum();
  for turn in 0..law["maximumTurns"].as_u64().unwrap(){
   let (demand,heap)=observe_heap_allocations_on_this_thread(||protocol_error_retirement_demand(&original).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   assert!(demand.copy_bytes+demand.capacity_bytes<=4096);
   if original.is_some(){
    let (zero,heap)=observe_heap_allocations_on_this_thread(||close_protocol_error_one(&mut original,RetainedCloneGrant{maximum_items:0,..exact(demand)}).unwrap());assert_eq!(zero.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    for axis in 0..3{
     let mut grant=exact(demand);let bytes=match axis{0=>&mut grant.maximum_copy_bytes,1=>&mut grant.maximum_release_bytes,_=>&mut grant.maximum_depth};if *bytes==0{continue;}*bytes-=1;
     let (blocked,heap)=observe_heap_allocations_on_this_thread(||close_protocol_error_one(&mut original,grant).unwrap());assert_eq!(blocked.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(protocol_error_retirement_demand(&original).unwrap(),demand);
    }
   }
   let (step,heap)=observe_heap_allocations_on_this_thread(||close_protocol_error_one(&mut original,exact(demand)).unwrap());let progress=step.progress();
   assert_eq!(heap.requested_bytes,progress.retained_capacity_bytes);assert_eq!(heap.released_bytes,progress.released_bytes);assert_eq!(progress.released_bytes,demand.release_bytes);assert!(progress.fits(exact(demand)));released+=progress.released_bytes;
   if turn==pause.as_u64().unwrap(){assert_eq!(released+remaining_capacity(&original),expected);}
   if matches!(step,RetainedCloneStep::Complete(_)){assert!(original.is_none());assert_eq!(released,expected);break;}
   assert!(turn+1<law["maximumTurns"].as_u64().unwrap());
  }
  let (_,heap)=observe_heap_allocations_on_this_thread(||drop(original));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 }}
 println!("[DEBUG] original ProtocolError/Pack/Value/Text slots match Serde Unicode/NUL; exact scalar copy and spare8192 releases admit zero/below/pause without hidden heap");
}
fn remaining_capacity(original:&Option<ProtocolError>)->usize{
 match original{
  Some(ProtocolError::Io(text)|ProtocolError::Malformed{detail:text,..})=>text.capacity(),
  Some(ProtocolError::Pack(PackError::Refusal(refusal)))=>match refusal{
   PackRefusal::Malformed{detail,..}=>detail.capacity(),
   PackRefusal::ValueRefusal(error)|PackRefusal::Io{error,..}=>match &error.message{std::borrow::Cow::Borrowed(_)=>0,std::borrow::Cow::Owned(text)=>text.capacity()},
   PackRefusal::TextRefusal(error)=>error.message.capacity()+error.expected.as_ref().map_or(0,String::capacity),_=>0,
  },_=>0,
 }
}
#[derive(Debug)]
struct Provider;
impl std::fmt::Display for Provider{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str("provider-owned cause")}}
impl std::error::Error for Provider{}
#[test]
fn protocol_transport_cause_refuses_without_dropping_original_provider(){
 let mut admission=TransportAdmission::new(4096,0);
 let reserve=reserve_transport::<Provider,_>(&mut admission,PackTransportCategory::NativeIo,PackRetryDisposition::Never,|_,_|true).unwrap();
 let mut original=Some(ProtocolError::Pack(PackError::TransportFailure(reserve.publish(Provider))));
 let source=match original.as_ref().unwrap(){ProtocolError::Pack(PackError::TransportFailure(error))=>error.source_error() as *const _,_=>unreachable!()};
 let (zero,heap)=observe_heap_allocations_on_this_thread(||close_protocol_error_one(&mut original,RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:64}).unwrap());assert_eq!(zero.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 let (refusal,heap)=observe_heap_allocations_on_this_thread(||protocol_error_retirement_demand(&original).unwrap_err());assert_eq!(refusal.kind,ValueRefusalKind::UnsupportedOwner);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 let (refusal,heap)=observe_heap_allocations_on_this_thread(||close_protocol_error_one(&mut original,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:64}).unwrap_err());assert_eq!(refusal.kind,ValueRefusalKind::UnsupportedOwner);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 match original.as_ref().unwrap(){ProtocolError::Pack(PackError::TransportFailure(error))=>assert!(std::ptr::eq(source,error.source_error())),_=>panic!("original provider replaced")}
 println!("[DEBUG] foreign provider retains exact source identity on zero/unsupported protocol cleanup; no manufactured release or fallback");
}
