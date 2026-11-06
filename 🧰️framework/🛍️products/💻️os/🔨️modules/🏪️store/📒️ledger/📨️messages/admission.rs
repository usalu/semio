//! 📨️ One original diagnostic candidate freezes mutation until bounded measurement and identity admission complete.
use super::*;
use semio_framework_value::{RetainedWorkOutcome,RetirementDemand,ValueRefusalKind};
use std::mem::ManuallyDrop;
pub(super) struct OriginalMessageAdmission {
 pub(super) candidate:ManuallyDrop<Option<protocol::EditMessages>>,
 hash:u64,index:usize,message:usize,target:usize,bytes:usize,probe:usize,compare:usize,tombstone:Option<usize>,bucket:usize,phase:u8,
}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
impl OriginalMessageAdmission {
 fn new(candidate:protocol::EditMessages)->Self{Self{candidate:ManuallyDrop::new(Some(candidate)),hash:0xcbf29ce484222325,index:0,message:0,target:0,bytes:0,probe:0,compare:0,tombstone:None,bucket:0,phase:0}}
 fn demand()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Option<Self>>()+size_of::<Self>()+2*size_of::<Option<protocol::EditMessages>>(),depth:1,..Default::default()}}
 fn advance(&mut self,ledger:&ArtifactEditMessageLedger)->Result<bool,ValueError>{
  let entry=self.candidate.as_ref().ok_or_else(||invalid("diagnostic admission lost its retained candidate"))?;
  match self.phase {
   0=>{
    if entry.edit_id.len()>ARTIFACT_EDIT_MESSAGE_ID_BYTES{return Err(invalid("diagnostic identity exceeds its declared byte policy"))}
    if self.index<entry.edit_id.len(){self.hash=(self.hash^u64::from(entry.edit_id.as_bytes()[self.index])).wrapping_mul(0x100000001b3);self.index+=1;}
    else{self.bytes=entry.edit_id.len();self.phase=1;}Ok(false)
   }
   1=>{
    if self.message==entry.messages.len(){self.phase=3;return Ok(false)}
    let message=&entry.messages[self.message];
    self.bytes=self.bytes.checked_add(ARTIFACT_EDIT_MESSAGE_OWNER_BYTES).and_then(|n|n.checked_add(message.code.0.len())).and_then(|n|n.checked_add(message.message.len())).ok_or_else(||invalid("diagnostic extent overflow"))?;
    self.target=0;self.phase=2;Ok(false)
   }
   2=>{
    let message=&entry.messages[self.message];
    if self.target<message.target.len(){self.bytes=self.bytes.checked_add(ARTIFACT_EDIT_MESSAGE_TARGET_BYTES).and_then(|n|n.checked_add(message.target[self.target].len())).ok_or_else(||invalid("diagnostic target extent overflow"))?;self.target+=1;}
    else{self.message+=1;self.phase=1;}
    if self.bytes>ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES{return Err(invalid("diagnostic candidate exceeds its declared byte policy"))}Ok(false)
   }
   3=>{
    if self.bytes>ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES{return Err(invalid("diagnostic candidate exceeds its declared byte policy"))}
    if self.probe==ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY {
     self.bucket=self.tombstone.ok_or_else(||invalid("diagnostic identity index is exhausted"))?;self.phase=5;return Ok(false)
    }
    let bucket=(self.hash as usize%ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY+self.probe)%ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY;
    let slot=ledger.buckets[bucket];
    if slot==ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY{self.bucket=self.tombstone.unwrap_or(bucket);self.phase=5;}
    else if slot==ARTIFACT_EDIT_MESSAGE_INDEX_TOMBSTONE{self.tombstone.get_or_insert(bucket);self.probe+=1;}
    else{self.bucket=bucket;self.compare=0;self.phase=4;}Ok(false)
   }
   4=>{
    let slot=usize::from(ledger.buckets[self.bucket]);
    if !ledger.occupied[slot]{return Err(invalid("diagnostic reserved lookup changed its occupied source"))}
    let established=unsafe{ledger.slots[slot].assume_init_ref()};
    if established.edit_id.len()!=entry.edit_id.len(){self.probe+=1;self.phase=3;}
    else if self.compare==entry.edit_id.len(){return Err(invalid("diagnostic candidate duplicates an existing identity"))}
    else if established.edit_id.as_bytes()[self.compare]!=entry.edit_id.as_bytes()[self.compare]{self.probe+=1;self.phase=3;}
    else{self.compare+=1;}Ok(false)
   }
   5=>Ok(true),
   _=>Err(invalid("diagnostic admission has an unknown frontier")),
  }
 }
}
impl ArtifactEditMessageLedger {
 /// 📏️ Prices complete retained candidate/header transfers, including the final fixed ticket writes.
 pub fn original_append_demand(&self)->RetirementDemand{
  let mut demand=OriginalMessageAdmission::demand();
  demand.copy_bytes+=size_of::<protocol::EditMessages>()+5*size_of::<u16>()+3*size_of::<usize>()+size_of::<u64>()+size_of::<bool>();demand
 }
 /// 🔒️ The actual candidate remains inside the fixed ledger while writers are fenced.
 pub fn original_append_pending(&self)->bool{self.original_admission.is_some()}
 /// 🪜️ Advances one hash byte, message extent, target extent, probe or identity byte.
 pub fn append_original_step(&mut self,source:&mut Option<protocol::EditMessages>,grant:RetainedCloneGrant,maximum_work:u64)->RetainedWorkOutcome<RetainedCloneProgress>{
  let demand=self.original_append_demand();
  if maximum_work==0||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  if !self.admission_is_complete(){return RetainedWorkOutcome{result:Err(invalid("diagnostic admission requires all actual fixed backing")),work:0}}
  if self.original_admission.is_none(){
   let Some(candidate)=source.take()else{return RetainedWorkOutcome{result:Ok(Default::default()),work:0}};
   self.original_admission=Some(OriginalMessageAdmission::new(candidate));
   return RetainedWorkOutcome{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:OriginalMessageAdmission::demand().copy_bytes,..Default::default()}),work:1}
  }
  if source.is_some(){return RetainedWorkOutcome{result:Err(invalid("diagnostic admission already owns its original donor")),work:0}}
  let mut cursor=self.original_admission.take().unwrap();
  let result=cursor.advance(self);let mut copied=OriginalMessageAdmission::demand().copy_bytes;
  let result=match result {
   Ok(true)=>{
    if self.free_len==0{Err(invalid("diagnostic fixed slots are exhausted"))}
    else{
     let slot=usize::from(self.free[self.free_len-1]);
     match self.generations[slot].checked_add(1){
      None=>Err(invalid("diagnostic ticket generation is exhausted")),
      Some(generation)=>{
       let entry=cursor.candidate.take().unwrap();self.free_len-=1;self.buckets[cursor.bucket]=slot as u16;self.slots[slot].write(entry);self.generations[slot]=generation;self.occupied[slot]=true;self.previous[slot]=self.tail;self.next[slot]=ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE;
       if self.tail==ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE{self.head=slot as u16}else{self.next[usize::from(self.tail)]=slot as u16}self.tail=slot as u16;self.len+=1;copied=demand.copy_bytes;Ok(())
      }
     }
    }
   },
   Ok(false)=>Ok(()),Err(error)=>Err(error),
  };
  if cursor.candidate.is_some(){self.original_admission=Some(cursor)}
  let progress=RetainedCloneProgress{copied_items:1,copied_bytes:copied,..Default::default()};
  RetainedWorkOutcome{result:result.map(|()|progress).map_err(|error|error.with_retained_progress(progress)),work:1}
 }
}

#[cfg(test)]
mod tests{
 use super::*;
 fn row(id:String)->protocol::EditMessages{protocol::EditMessages{edit_id:id,messages:Vec::new()}}
 fn close(ledger:ArtifactEditMessageLedger){
  let mut owner=ArtifactEditMessageLedgerRetirement::new(ledger);
  for _ in 0..200000{
   if owner.terminal_is_empty(){return}
   let d=owner.demands(4096).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)};
   let(turn,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||owner.close_step(grant));
   let progress=turn.result.unwrap().progress();assert!(progress.fits(grant));assert_eq!((allocated,released),(progress.retained_capacity_bytes,progress.released_bytes));
  }panic!("original diagnostic retirement did not complete")
 }
 #[test]
 fn original_bounded_candidate_matches_sqlite_outcomes_and_retains_failed_owners_for_paid_close(){
  let law:serde_json::Value=serde_json::from_str(include_str!("admission.json")).unwrap();
  for case in law["cases"].as_array().unwrap(){
   let mut ledger=ArtifactEditMessageLedger::new();
   if case["expected"]=="full"{for i in 0..ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY{ledger.try_push(row(format!("existing-{i}"))).unwrap();}}
   else{for id in case["ids"].as_array().unwrap(){ledger.try_push(row(id.as_str().unwrap().into())).unwrap();}}
   let before=ledger.len();let mut donor=Some(row(case["candidate"].as_str().unwrap().into()));let pointer=donor.as_ref().unwrap().edit_id.as_ptr();let d=ledger.original_append_demand();
   let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
   for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..grant}]{
    let(turn,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||ledger.append_original_step(&mut donor,denied,1));assert_eq!(turn.result.unwrap(),Default::default());assert_eq!(turn.work,0);assert_eq!((allocated,released),(0,0));assert_eq!(donor.as_ref().unwrap().edit_id.as_ptr(),pointer);
   }
   let zero=ledger.append_original_step(&mut donor,grant,0);assert_eq!(zero.work,0);assert!(donor.is_some());
   let mut failed=false;
   for _ in 0..100000{
    let(turn,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||ledger.append_original_step(&mut donor,grant,1));assert_eq!((allocated,released),(0,0));assert!(turn.work<=1);
    match turn.result{Ok(progress)=>assert!(progress.fits(grant)),Err(error)=>{assert!(error.retained_progress().fits(grant));failed=true;break}}
    if !ledger.original_append_pending(){break}
    assert_eq!(ledger.original_admission.as_ref().unwrap().candidate.as_ref().unwrap().edit_id.as_ptr(),pointer);
    assert!(ledger.pop().is_none());assert!(ledger.get_mut_by_id(case["ids"][0].as_str().unwrap()).is_none());
   }
   assert!(donor.is_none());assert_eq!(failed,case["expected"]!="admitted");assert_eq!(ledger.len(),before+usize::from(!failed));
   if failed{assert!(ledger.original_append_pending())}else{assert_eq!(ledger.last().unwrap().edit_id.as_ptr(),pointer)}
   close(ledger);
  }
 }
}
