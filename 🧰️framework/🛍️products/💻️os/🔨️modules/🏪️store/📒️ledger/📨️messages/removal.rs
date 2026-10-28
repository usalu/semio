//! 🪃️ A bounded original bucket probe prepares one exact message removal before publication.
use super::*;
use semio_framework_value::{RetirementDemand,ValueRefusalKind};
/// 🧭️ The original ledger and text backing authenticate every fixed bucket visit.
#[derive(Default)]
pub struct OriginalEditMessageRemoval{source:Option<(usize,usize,usize,usize)>,start:usize,probe:usize,ticket:Option<ArtifactEditMessageTicket>,bucket:Option<usize>,ready:bool}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original message removal changed its ledger, text or exact ticket")}
impl OriginalEditMessageRemoval{
 /// 📏️ One probe compares at most the defining identity extent and changes fixed cursor metadata.
 pub fn preparation_demands()->RetirementDemand{RetirementDemand{copy_bytes:ARTIFACT_EDIT_MESSAGE_ID_BYTES+size_of::<Self>(),depth:1,..Default::default()}}
 /// 🏁️ Absence is a prepared result authenticated by the same original ledger.
 pub fn prepared(&self)->bool{self.ready}
 /// 🫙️ A cancelled probe returns its original source fence before its enclosing frame retires.
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()}
 /// 📐️ The complete inline cursor returns in one fixed funded action without touching the ledger.
 pub fn close_demands()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>(),depth:1,..Default::default()}}
 /// ♻️ Clearing the source witness leaves every original row and bucket unchanged.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=Self::close_demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}*self=Self::default();Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 /// 🪡️ Visits one real identity bucket without gathering or traversing a ledger suffix.
 pub fn advance(&mut self,ledger:&ArtifactEditMessageLedger,id:&str,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  let source=(ledger as*const _ as usize,ledger.len(),id.as_ptr()as usize,id.len());if id.len()>ARTIFACT_EDIT_MESSAGE_ID_BYTES||self.source.is_some_and(|original|original!=source){return Err(invalid())}if self.ready{return Ok(Default::default())}let d=Self::preparation_demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(Default::default())}
  if self.source.is_none(){self.source=Some(source);self.start=ArtifactEditMessageLedger::hash(id)%ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY;self.ready=!ledger.admission_is_complete();}else if self.probe==ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY{self.ready=true;}else{let bucket=(self.start+self.probe)%ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY;let slot=ledger.buckets[bucket];if slot==ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY{self.ready=true;}else if slot!=ARTIFACT_EDIT_MESSAGE_INDEX_TOMBSTONE{let index=usize::from(slot);if ledger.occupied[index]&&unsafe{ledger.slots[index].assume_init_ref()}.edit_id==id{self.ticket=Some(ArtifactEditMessageTicket{slot,generation:ledger.generations[index]});self.bucket=Some(bucket);self.ready=true;}}self.probe+=1;}
  Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})
 }
 /// 📏️ Publication unlinks only the prepared original row and transfers its unchanged payload.
 pub fn publication_demands()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<protocol::EditMessages>()+12*size_of::<usize>(),depth:1,..Default::default()}}
 /// 🪟️ Every validation happens before the prepared ledger links change.
 pub fn can_publish(&self,ledger:&ArtifactEditMessageLedger)->bool{self.ready&&self.source.is_some_and(|source|source.0==ledger as*const _ as usize&&source.1==ledger.len())&&match(self.ticket,self.bucket){(Some(ticket),Some(bucket))=>ledger.get(ticket).is_some()&&ledger.buckets.get(bucket).copied()==Some(ticket.slot),(None,None)=>true,_=>false}}
 /// 📤️ The caller retains the original removed messages for paid retirement after publication.
 pub fn publish(&mut self,ledger:&mut ArtifactEditMessageLedger,output:&mut Option<protocol::EditMessages>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if output.is_some()||!self.can_publish(ledger){return Err(invalid())}let d=Self::publication_demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(Default::default())}if let Some(ticket)=self.ticket{*output=Some(ledger.remove_at_bucket(ticket,self.bucket.unwrap()).ok_or_else(invalid)?);}self.source=None;self.ticket=None;self.bucket=None;self.ready=false;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})}
}
