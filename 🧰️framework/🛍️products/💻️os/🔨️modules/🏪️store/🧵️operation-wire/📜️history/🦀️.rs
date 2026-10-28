//! 📜️ Original sorted drafts stream into canonical transition pages and their content-addressed identity.
use super::ArtifactPreparedOperationOutput;
use semio_framework_os_spr::{HistoryInputDrafts,HistoryTransition,HistoryTransitionWireCursor,HistoryTransitionBorrowedIdentity,HistoryTransitionCommitView,HistoryTransitionBranchView,HistoryTransitionEditMode,HistoryTransitionEditsView,HistoryTransitionRepinView,HybridLogicalTimestamp,CausalPayload,MutationId};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::retained_clone::RetainedCloneClose;
use semio_framework_hash::Hasher;
use semio_framework_value::ErasedSnapshotRetirement;
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,PartialEq,Eq)]
enum HistoryPayloadSource{Drafts{address:usize,epoch:u64,scope:Option<(usize,usize)>},Transition{address:usize},Commit{identity:HistoryTransitionBorrowedIdentity,epoch:u64},Branch{identity:HistoryTransitionBorrowedIdentity,epoch:u64},Edits{identity:HistoryTransitionBorrowedIdentity,epoch:u64},Repin{identity:HistoryTransitionBorrowedIdentity,epoch:u64}}
/// 🪡️ The enclosing publisher retains its original draft owner while this pure cursor reborrows its fields.
pub struct OriginalHistoryTransitionPayload{cursor:HistoryTransitionWireCursor,output:ArtifactPreparedOperationOutput,hash:Hasher,id:ManuallyDrop<Option<String>>,retirement:RetainedCloneClose,source:Option<(HistoryPayloadSource,HybridLogicalTimestamp)>,pending:[u8;64],length:usize,offset:usize,measured:usize,phase:u8,closing:bool}
fn invalid(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,text)}
fn permits(g:RetainedCloneGrant,d:RetirementDemand)->bool{semio_framework_value::retained_clone::retained_clone_grant_admits(g,d)}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn hash_varint(hash:&mut Hasher,mut value:u64){let mut bytes=[0;10];let mut length=0;loop{bytes[length]=(value as u8&127)|if value>=128{128}else{0};length+=1;value>>=7;if value==0{break}}hash.update(&bytes[..length]);}
impl OriginalHistoryTransitionPayload{
 /// 🌱️ Empty original output pages perform no source scan or heap allocation.
 pub fn new(maximum_payload:usize,maximum_capacity:usize)->Result<Self,ValueError>{Ok(Self{cursor:Default::default(),output:ArtifactPreparedOperationOutput::empty(maximum_payload,maximum_capacity)?,hash:Hasher::new(),id:ManuallyDrop::new(None),retirement:Default::default(),source:None,pending:[0;64],length:0,offset:0,measured:0,phase:0,closing:false})}
 /// 📏️ Quotes the next measurement, actual page birth, bounded copy or fixed identifier birth.
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.closing{return self.close_demands(body)}let metadata=size_of::<Option<(HistoryPayloadSource,HybridLogicalTimestamp)>>()+3*size_of::<usize>()+size_of::<u8>();let mut d=match self.phase{0=>RetirementDemand{copy_bytes:64+size_of::<HistoryTransitionWireCursor>(),depth:1,..Default::default()},1=>header::<Hasher>(),2 if self.offset<self.length=>self.output.append_demands((self.length-self.offset).min(body))?,2=>RetirementDemand{copy_bytes:64+size_of::<HistoryTransitionWireCursor>(),depth:1,..Default::default()},3=>RetirementDemand{copy_bytes:27+size_of::<String>()+size_of::<Hasher>(),capacity_bytes:27,depth:1,..Default::default()},_=>header::<(CausalPayload,MutationId)>()};d.copy_bytes=d.copy_bytes.checked_add(metadata).ok_or_else(||invalid("original transition quote overflow"))?;Ok(d)}
 /// ✍️ One actual output action preserves source identity, epoch, scope and the exact authored clock.
 pub fn advance_supersession(&mut self,drafts:&HistoryInputDrafts,scope:Option<&str>,timestamp:HybridLogicalTimestamp,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let source=(HistoryPayloadSource::Drafts{address:drafts as*const _ as usize,epoch:drafts.mutation_epoch(),scope:scope.map(|text|(text.as_ptr()as usize,text.len()))},timestamp);self.advance_source(source,grant,|cursor,output|cursor.write_supersession(drafts,scope,output))}
 /// 🌿️ Streams an original Commit or Branch using the same canonical pages, hash and independently supplied currencies.
 pub fn advance_transition(&mut self,transition:&HistoryTransition,timestamp:HybridLogicalTimestamp,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let source=(HistoryPayloadSource::Transition{address:transition as*const _ as usize},timestamp);self.advance_source(source,grant,|cursor,output|cursor.write(transition,output))}
 /// 🏛️ Reborrows every actual canonical Commit field through measurement and payload append phases.
 pub fn advance_commit_view(&mut self,view:&HistoryTransitionCommitView<'_>,epoch:u64,timestamp:HybridLogicalTimestamp,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let source=(HistoryPayloadSource::Commit{identity:view.identity(),epoch},timestamp);self.advance_source(source,grant,|cursor,output|cursor.write_commit_view(view,epoch,output))}
 /// 🌿️ Reborrows the actual canonical Alternative fields without reconstructing an owned transition.
 pub fn advance_branch_view(&mut self,view:&HistoryTransitionBranchView<'_>,epoch:u64,timestamp:HybridLogicalTimestamp,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let source=(HistoryPayloadSource::Branch{identity:view.identity(),epoch},timestamp);self.advance_source(source,grant,|cursor,output|cursor.write_branch_view(view,epoch,output))}
 /// ↩️ Preserves true Revert/Reinstate identity while streaming the original mutation IDs and authored clock.
 pub fn advance_edits_view(&mut self,view:&HistoryTransitionEditsView<'_>,epoch:u64,timestamp:HybridLogicalTimestamp,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let source=(HistoryPayloadSource::Edits{identity:view.identity(),epoch},timestamp);self.advance_source(source,grant,|cursor,output|cursor.write_edits_view(view,epoch,output))}
 /// 📌️ Streams actual typed pin URI segments and canonical checkpoint fields without an owned Repin mirror.
 pub fn advance_repin_view(&mut self,view:&HistoryTransitionRepinView<'_>,epoch:u64,timestamp:HybridLogicalTimestamp,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let source=(HistoryPayloadSource::Repin{identity:view.identity(),epoch},timestamp);self.advance_source(source,grant,|cursor,output|cursor.write_repin_view(view,epoch,output))}
 fn advance_source(&mut self,source:(HistoryPayloadSource,HybridLogicalTimestamp),grant:RetainedCloneGrant,mut write:impl FnMut(&mut HistoryTransitionWireCursor,&mut[u8])->Result<usize,ValueError>)->Result<RetainedCloneProgress,ValueError>{if self.closing{return Err(invalid("closing transition payload cannot resume"))}if self.source.is_some_and(|original|original!=source){return Err(invalid("transition payload changed its original source, scope or clock"))}if self.ready(){return Ok(Default::default())}let timestamp=source.1;let d=self.demands(grant.maximum_copy_bytes)?;if !permits(grant,d){return Ok(Default::default())}self.source=Some(source);let metadata=size_of::<Option<(HistoryPayloadSource,HybridLogicalTimestamp)>>()+3*size_of::<usize>()+size_of::<u8>();let mut receipt=progress(d);
  match self.phase{
   0=>{let written=write(&mut self.cursor,&mut self.pending)?;self.measured=self.measured.checked_add(written).ok_or_else(||invalid("original transition extent overflow"))?;if self.cursor.is_finished(){self.phase=1;}},
   1=>{self.cursor=Default::default();hash_varint(&mut self.hash,timestamp.actor);hash_varint(&mut self.hash,timestamp.physical_ms);hash_varint(&mut self.hash,timestamp.logical);hash_varint(&mut self.hash,self.measured as u64);self.length=0;self.offset=0;self.phase=2;},
   2 if self.offset<self.length=>{let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-metadata,..grant};let p=self.output.append(&self.pending[self.offset..self.length],child)?;if p.copied_bytes!=0{self.hash.update(&self.pending[self.offset..self.offset+p.copied_bytes]);self.offset+=p.copied_bytes;}receipt=p.checked_add(RetainedCloneProgress{copied_items:usize::from(p.copied_items==0),copied_bytes:metadata,..Default::default()})?;},
   2 if self.cursor.is_finished()=>{if self.output.len()!=self.measured{return Err(invalid("original transition measurement changed during streaming"))}self.phase=3;},
   2=>{self.length=write(&mut self.cursor,&mut self.pending)?;self.offset=0;},
   3=>{let hash=self.hash.finalize();let mut id=String::with_capacity(27);id.push_str("transition-");const HEX:&[u8;16]=b"0123456789abcdef";for byte in &hash.as_bytes()[..8]{id.push(HEX[(byte>>4)as usize]as char);id.push(HEX[(byte&15)as usize]as char);}*self.id=Some(id);self.phase=4;},
   _=>return Err(invalid("original transition payload phase changed")),
  }if !receipt.fits(grant){return Err(invalid("original transition exceeded its original grant").with_retained_progress(receipt))}Ok(receipt)
 }
 /// 🏁️ Complete bytes and their exact content identity remain owned until the same handoff.
 pub fn ready(&self)->bool{self.phase==4&&!self.closing}
 /// 🤝️ Moves the actual paid payload pages and identifier without copying any original input bytes.
 pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(CausalPayload,MutationId,RetainedCloneProgress)>,ValueError>{if !self.ready(){return Err(invalid("original transition payload is not ready"))}let copy=size_of::<CausalPayload>()+size_of::<MutationId>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return Ok(None)}let Some((bytes,_))=self.output.take_bytes(grant)?else{return Ok(None)};let id=MutationId(self.id.take().unwrap());self.phase=5;Ok(Some((CausalPayload::from_retained_bytes(bytes.into_retained_bytes()),id,RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})))}
 /// 🛑️ Stops output while preserving every actual admitted payload and identifier owner.
 pub fn begin_close(&mut self){self.closing=true;self.output.begin_close();}
 /// 📏️ Prices original page return, typed identifier close and the final source-coordinate reset.
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.output.terminal_is_empty(){return self.output.retirement_demands()}if !self.retirement.is_empty(){return self.retirement.demands(body)}if self.id.is_some(){return self.retirement.next_owner_demands::<String>(true,&None,body)}Ok(header::<Option<(HistoryPayloadSource,HybridLogicalTimestamp)>>())}
 /// ♻️ Physically closes all native pages and partial identifiers before forgetting source coordinates.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.begin_close();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demands(grant.maximum_copy_bytes)?;if !permits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.output.terminal_is_empty(){return self.output.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}if !self.retirement.is_empty(){return self.retirement.select(grant.maximum_copy_bytes)?.step(grant)}if self.id.is_some(){return Ok(self.retirement.begin_granted(&mut self.id,grant)?.unwrap_or(RetainedCloneStep::Progress(Default::default())))}self.source=None;Ok(RetainedCloneStep::Progress(progress(d)))}
 /// 🫙️ Source coordinates, original pages and every typed close frame must all return.
 pub fn terminal_is_empty(&self)->bool{self.output.terminal_is_empty()&&self.id.is_none()&&self.retirement.is_empty()&&self.source.is_none()}
}
impl Drop for OriginalHistoryTransitionPayload{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original transition payload must transfer or physically close");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.id);}}}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
