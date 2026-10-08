//! ♻️ Original protocol cause slots separate logical owner movement and physical backing release.
use super::ProtocolError;
use semio_framework_pack_error::{PackError,PackRefusal};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
#[derive(Clone,Copy)]
enum TextSlot{Protocol,Pack,Value,DiagnosticMessage,DiagnosticExpected}
fn slot(cause:&ProtocolError)->Result<Option<(TextSlot,usize)>,ValueError>{
 let present=|slot,text:&String|if text.capacity()==0{None}else{Some((slot,text.capacity()))};
 Ok(match cause{
  ProtocolError::Malformed{detail,..}|ProtocolError::Io(detail)=>present(TextSlot::Protocol,detail),
  ProtocolError::Pack(error)=>match error{
   PackError::TransportFailure(_)=>return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"protocol transport cause retains its genuine provider retirement authority")),
   PackError::Refusal(refusal)=>match refusal{
    PackRefusal::Malformed{detail,..}=>present(TextSlot::Pack,detail),
    PackRefusal::ValueRefusal(error)|PackRefusal::Io{error,..}=>match &error.message{std::borrow::Cow::Borrowed(_)=>None,std::borrow::Cow::Owned(text)=>present(TextSlot::Value,text)},
    PackRefusal::TextRefusal(error)=>present(TextSlot::DiagnosticMessage,&error.message).or_else(||error.expected.as_ref().and_then(|text|present(TextSlot::DiagnosticExpected,text))),
    PackRefusal::BadMagic|PackRefusal::UnsupportedVersion{..}|PackRefusal::UnknownRequiredFlags(_)|PackRefusal::Truncated(_)|PackRefusal::ChecksumMismatch{..}|PackRefusal::ContentHashMismatch|PackRefusal::LimitExceeded{..}|PackRefusal::RetainedMalformed{..}|PackRefusal::RetainedAllocation{..}|PackRefusal::NonCanonical(_)|PackRefusal::UnsupportedCodec(_)|PackRefusal::TransportAdmission{..}=>None,
   },
  },
  ProtocolError::ChainMismatch{..}|ProtocolError::TornTail(_)|ProtocolError::UnknownCriticalRecord(_)|ProtocolError::DictMiss(_)|ProtocolError::DictOutOfOrder{..}|ProtocolError::VerifierRequired|ProtocolError::SignatureInvalid{..}|ProtocolError::FrameFraming(_)|ProtocolError::LimitExceeded(_)=>None,
 })
}
fn slot_mut(cause:&mut ProtocolError,slot:TextSlot)->&mut String{
 match (slot,cause){
  (TextSlot::Protocol,ProtocolError::Malformed{detail,..}|ProtocolError::Io(detail))=>detail,
  (TextSlot::Pack,ProtocolError::Pack(PackError::Refusal(PackRefusal::Malformed{detail,..})))=>detail,
  (TextSlot::Value,ProtocolError::Pack(PackError::Refusal(PackRefusal::ValueRefusal(error)|PackRefusal::Io{error,..})))=>match &mut error.message{std::borrow::Cow::Owned(text)=>text,_=>unreachable!("checked original owned cause")},
  (TextSlot::DiagnosticMessage,ProtocolError::Pack(PackError::Refusal(PackRefusal::TextRefusal(error))))=>&mut error.message,
  (TextSlot::DiagnosticExpected,ProtocolError::Pack(PackError::Refusal(PackRefusal::TextRefusal(error))))=>error.expected.as_mut().expect("checked original expected cause"),
  _=>unreachable!("original protocol slot remains exclusively borrowed"),
 }
}
/// 🔎️ Reads the first original owned cause slot without allocating, copying its text, or erasing provider custody.
pub fn protocol_error_retirement_demand(cause:&Option<ProtocolError>)->Result<RetirementDemand,ValueError>{
 let Some(cause)=cause.as_ref()else{return Ok(RetirementDemand::default())};
 Ok(match slot(cause)?{
  Some((_,capacity))=>RetirementDemand{copy_bytes:2*std::mem::size_of::<String>(),release_bytes:capacity,depth:1,..Default::default()},
  None=>RetirementDemand{copy_bytes:std::mem::size_of::<Option<ProtocolError>>(),depth:1,..Default::default()},
 })
}
/// 🎟️ Moves one admitted native descriptor and releases only its actual original backing, preserving refused owners in place.
pub fn close_protocol_error_one(cause:&mut Option<ProtocolError>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
 if cause.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()));}
 if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
 let demand=protocol_error_retirement_demand(cause)?;
 if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
 let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()};
 let error=cause.as_mut().expect("original protocol cause retained through admission");
 if let Some((slot,_))=slot(error)?{
  let text=std::mem::take(slot_mut(error,slot));drop(text);
  Ok(RetainedCloneStep::Progress(progress))
 }else{drop(cause.take());Ok(RetainedCloneStep::Complete(progress))}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
