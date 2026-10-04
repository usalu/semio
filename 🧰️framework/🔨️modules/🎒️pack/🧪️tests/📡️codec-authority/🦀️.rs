//! 🧪️ Actual retained parser syntax and owner lifecycle are distinct machine causes.
use crate::codec::*;
use semio_framework_value::ValueRefusalKind;

#[cfg(feature="deflate")]
fn close_authority_cursor(cursor:&mut DeflateRetainedCursor){
 for _ in 0..8{if cursor.close_step(8,usize::MAX)==RetainedInflateCloseStep::Complete{return}}
 panic!("bounded retained authority fixture failed to close")
}

#[cfg(feature="deflate")]
#[test]
fn actual_retained_deflate_syntax_lifecycle_and_requested_ceiling_are_distinct(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../⚠️error/🧫️fixtures/🧭️cause/📡️codec/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let mut grammar=DeflateRetainedCursor::try_new(0,0,0).unwrap();
  let mut terminal=None;
  for byte in row["stored"].as_array().unwrap(){
   grammar.admit_byte(u8::try_from(byte.as_u64().unwrap()).unwrap()).unwrap();
   for _ in 0..64{
    match grammar.grant(false){Ok(DeflateRetainedStep::NeedInput)=>{},Ok(DeflateRetainedStep::Complete)=>break,Ok(DeflateRetainedStep::Byte(_))=>panic!("empty grammar corpus produced a byte"),Err(error)=>{terminal=Some(error);break}}
    if grammar.can_admit(){break}
   }
   if terminal.is_some(){break}
  }
  if terminal.is_none(){
   let mut complete=false;
   for _ in 0..64{match grammar.grant(true){Ok(DeflateRetainedStep::Complete)=>{complete=true;break},Ok(DeflateRetainedStep::NeedInput)=>{},Ok(DeflateRetainedStep::Byte(_))=>panic!("empty grammar corpus produced a byte"),Err(error)=>{terminal=Some(error);break}}}
   assert!(complete||terminal.is_some(),"bounded grammar fixture did not terminate");
  }
  close_authority_cursor(&mut grammar);
  assert_eq!(terminal.and_then(|error|Some(error.kind()).map(ValueRefusalKind::as_str)),row["expectedKind"].as_str());
 }
 let mut lifecycle=DeflateRetainedCursor::try_new(0,0,0).unwrap();
 let reset=lifecycle.reset(0,0);
 close_authority_cursor(&mut lifecycle);
 assert_eq!(Some(reset.unwrap_err().kind()),Some(ValueRefusalKind::InvariantViolated));
 let mut syntax=DeflateRetainedCursor::try_new(0,0,0).unwrap();
 let admitted=syntax.admit_byte(7);
 let refusal=syntax.grant(true);
 let repeated=syntax.grant(true);
 close_authority_cursor(&mut syntax);
 assert!(admitted.is_ok());
 let refusal=refusal.unwrap_err();
 assert_eq!(Some(refusal.kind()),Some(ValueRefusalKind::InvalidValue));
 assert_eq!(repeated.unwrap_err(),refusal);
 let overlapping=match DeflateRetainedCursor::try_new(1,0,0){Ok(mut cursor)=>{close_authority_cursor(&mut cursor);panic!("declared logical limit accepted")},Err(error)=>error};
 assert_eq!(Some(overlapping.kind()),Some(ValueRefusalKind::WorkLimit));
 for name in ["physicalHistory","semanticLength"]{
  let row=&fixture["retainedAdmission"][name];
  let expected=row["expectedRawBytes"].as_u64().unwrap();
  let limit=row["semanticSegmentBytes"].as_u64().unwrap();
  let allocation=usize::try_from(row["maximumAllocationBytes"].as_u64().unwrap()).unwrap();
  let refusal=match DeflateRetainedCursor::try_new(expected,limit,allocation){Ok(mut cursor)=>{close_authority_cursor(&mut cursor);panic!("declared retained admission accepted")},Err(error)=>error};
  assert_eq!(refusal.kind().as_str(),row["expectedKind"].as_str().unwrap());
 }
 let ceiling=match DeflateRetainedCursor::try_new(1,1,0){Ok(mut cursor)=>{close_authority_cursor(&mut cursor);panic!("declared retained ceiling accepted")},Err(error)=>error};
 assert_eq!(Some(ceiling.kind()),Some(ValueRefusalKind::OwnershipLimit));
 eprintln!("[DEBUG] Pack retained actual syntax/lifecycle/ceiling producers remained distinct");
}

#[test]
fn public_pack_primitives_and_replication_share_the_defining_error_identity(){
    use semio_framework_pack_error::PackRefusal as CanonicalRefusal;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⚠️error/🪪️identity/🔣️.json")).unwrap();
    for row in fixture["primitiveCases"].as_array().unwrap(){
        let bytes:Vec<u8>=row["bytes"].as_array().unwrap().iter().map(|value|u8::try_from(value.as_u64().unwrap()).unwrap()).collect();let mut position=0;
        let result:Result<u64,CanonicalRefusal>=crate::read_varint_u64(&bytes,&mut position);
        let error=result.unwrap_err();assert!(matches!(error,CanonicalRefusal::Truncated(_)));assert_eq!(position,row["position"].as_u64().unwrap()as usize);assert_eq!(error.to_string(),row["display"].as_str().unwrap());
        let container:crate::PackRefusal=error;let replica:protocol::PackRefusal=container;let canonical:CanonicalRefusal=replica;let converted=canonical.into_value_error();
        let oracle=serde_json::json!({"kind":row["convertedKind"],"message":row["display"]});assert_eq!(serde_json::json!({"kind":converted.kind.as_str(),"message":converted.message}),oracle);
    }
    eprintln!("[DEBUG] public Pack primitive/container/Replication errors retain one defining native type");
}

#[test]
fn public_pack_typed_causes_preserve_owned_messages_and_fault_identity(){
    use semio_framework_pack_error::PackRefusal as CanonicalRefusal;
    use semio_framework_diagnostic::{FaultFrom,FaultOrigin,Severity};
    use semio_framework_value::{ValueError,ValueRefusalKind};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../⚠️error/🧫️fixtures/⚠️refusal/🔣️.json")).unwrap();
    let identity:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⚠️error/🪪️identity/🔣️.json")).unwrap();
    fn kind(value:&str)->ValueRefusalKind{match value{"InvalidValue"=>ValueRefusalKind::InvalidValue,"Canceled"=>ValueRefusalKind::Canceled,"OwnershipLimit"=>ValueRefusalKind::OwnershipLimit,"AllocationFailed"=>ValueRefusalKind::AllocationFailed,"WorkLimit"=>ValueRefusalKind::WorkLimit,"DepthLimit"=>ValueRefusalKind::DepthLimit,"UnsupportedOwner"=>ValueRefusalKind::UnsupportedOwner,"InvariantViolated"=>ValueRefusalKind::InvariantViolated,_=>panic!("unknown authored kind")}}
    for row in fixture["cases"].as_array().unwrap(){
        let expected_kind=kind(row["kind"].as_str().unwrap());let mut cause=ValueError::new(expected_kind,row["message"].as_str().unwrap());for part in row["path"].as_array().unwrap(){cause=cause.under(part.as_str().unwrap());}let allocation=cause.message.as_ptr();
        let canonical=CanonicalRefusal::from(cause);let wrapper:crate::PackRefusal=canonical;let replica:protocol::PackRefusal=wrapper;let wrapper:CanonicalRefusal=replica;
        let CanonicalRefusal::ValueRefusal(refusal)=&wrapper else{panic!("typed cause")};assert_eq!(refusal.kind,expected_kind);assert_eq!(refusal.message.as_ptr(),allocation);assert_eq!(wrapper.to_string(),row["display"].as_str().unwrap());
        let source=std::error::Error::source(&wrapper).unwrap().downcast_ref::<ValueError>().unwrap();assert!(std::ptr::eq(source,refusal));assert_eq!(wrapper.fault_origin(),FaultOrigin::Module);assert_eq!(wrapper.fault_code().0,identity["fault"]["code"].as_str().unwrap());assert_eq!(wrapper.fault_severity(),Severity::Error);
        let converted=wrapper.into_value_error();assert_eq!(converted.kind,expected_kind);assert_eq!(converted.message.as_ptr(),allocation);assert_eq!(converted.message,row["display"].as_str().unwrap().strip_prefix("schema error: ").unwrap());
    }
    eprintln!("[DEBUG] public typed Pack cause eight vectors preserve allocation/path/kind/source and module.pack Fault; conversion retains raw owned cause");
}
