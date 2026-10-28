//! ✏️ A completed replay moves its actual draft inputs into private effective rows after canonical wire capture finishes.
use semio_framework_os_spr::{HistoryInputDrafts,MutationEnvelope,MutationId,EffectiveSupersession,HybridLogicalTimestamp};
use crate::lifted_replay::EffectiveSupersessions;
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneClose,retained_clone_grant_admits}};
use std::mem::{ManuallyDrop,size_of};
type Row=(MutationId,EffectiveSupersession);
type Source=(usize,usize,[(usize,usize);3],HybridLogicalTimestamp);
/// 🧳️ Canonical incoming and displaced rows retain every original payload until insertion or paid cancellation.
pub struct OriginalReplaySupersessions{row:ManuallyDrop<Option<Row>>,displaced:ManuallyDrop<Option<EffectiveSupersession>>,retirement:RetainedCloneClose,source:Option<Source>,epochs:(u64,u64),phase:u8,complete:bool,closing:bool}
impl Default for OriginalReplaySupersessions{fn default()->Self{Self{row:ManuallyDrop::new(None),displaced:ManuallyDrop::new(None),retirement:Default::default(),source:None,epochs:(0,0),phase:0,complete:false,closing:false}}}
fn invalid(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,text)}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn stamp(drafts:&HistoryInputDrafts,map:&EffectiveSupersessions,event:&MutationEnvelope,scope:Option<&str>)->Source{(drafts as*const _ as usize,map as*const _ as usize,[(event.mutation_id.0.as_ptr()as usize,event.mutation_id.0.len()),(event.actor.0.as_ptr()as usize,event.actor.0.len()),scope.map_or((0,0),|scope|(scope.as_ptr()as usize,scope.len()))],event.timestamp)}
fn append(target:&mut String,source:&str){let start=target.len();let mut end=(start+64).min(source.len());while !source.is_char_boundary(end){end-=1;}target.push_str(&source[start..end]);}
impl OriginalReplaySupersessions{
 fn validate(&self,drafts:&HistoryInputDrafts,map:&EffectiveSupersessions,event:&MutationEnvelope,scope:Option<&str>)->Result<(),ValueError>{if self.source.is_some_and(|source|source!=stamp(drafts,map,event,scope)||self.epochs!=(drafts.mutation_epoch(),map.mutation_epoch())){return Err(invalid("replay effective overlay changed its original draft, private index or authored transition"))}Ok(())}
 /// 📏️ Quotes exactly one original row move, bounded metadata copy, index birth or displaced-row close.
 pub fn demands(&self,drafts:&HistoryInputDrafts,map:&EffectiveSupersessions,event:&MutationEnvelope,scope:Option<&str>,body:usize)->Result<RetirementDemand,ValueError>{self.validate(drafts,map,event,scope)?;if self.closing{return self.close_demands(body)}if !self.retirement.is_empty()||self.displaced.is_some(){return self.close_child_demands(body)}if self.complete{return Ok(Default::default())}let d=match self.phase{0 if drafts.is_empty()=>header::<bool>(),0=>drafts.take_first_demands()?,1=>RetirementDemand{capacity_bytes:event.mutation_id.0.len(),..header::<String>()},2|5=>RetirementDemand{copy_bytes:64+size_of::<u8>(),depth:1,..Default::default()},3=>header::<semio_framework_os_spr::ActorId>(),4=>RetirementDemand{capacity_bytes:scope.map_or(0,str::len),..header::<Option<String>>()},7=>{let key=&self.row.as_ref().unwrap().0;let capacity=map.next_insert_capacity_byte_demand(key,body)?;RetirementDemand{copy_bytes:if capacity==0{size_of::<Row>()+size_of::<Option<EffectiveSupersession>>()}else{0},capacity_bytes:capacity,depth:map.next_insert_depth_demand(key)?,..Default::default()}},_=>return Err(invalid("replay effective overlay phase changed"))};Ok(d)}
 /// 🪡️ Every payload Vec moves unchanged; only actual provenance strings and the original actor lease are prepared.
 pub fn advance(&mut self,drafts:&mut HistoryInputDrafts,map:&mut EffectiveSupersessions,event:&MutationEnvelope,scope:Option<&str>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if self.closing{return Err(invalid("closing replay effective overlay cannot resume"))}let d=self.demands(drafts,map,event,scope,grant.maximum_copy_bytes)?;if self.complete||!retained_clone_grant_admits(grant,d){return Ok(Default::default())}self.source=Some(stamp(drafts,map,event,scope));if !self.retirement.is_empty()||self.displaced.is_some(){return self.close_child(grant).map(|step|step.progress())}let mut p=receipt(d);match self.phase{
  0 if drafts.is_empty()=>self.complete=true,
  0=>{let Some(((id,replacement),moved))=drafts.take_first(grant)?else{return Ok(Default::default())};*self.row=Some((id,EffectiveSupersession{transition_id:String::new(),actor:Default::default(),timestamp:event.timestamp,scope:None,replacement}));self.phase=1;p=moved;},
  1=>{self.row.as_mut().unwrap().1.transition_id.try_reserve_exact(event.mutation_id.0.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"replay transition identity reservation failed"))?;self.phase=2;},
  2=>{let text=&mut self.row.as_mut().unwrap().1.transition_id;if text.len()==event.mutation_id.0.len(){self.phase=3;}else{append(text,&event.mutation_id.0);}},
  3=>{self.row.as_mut().unwrap().1.actor=event.actor.0.admit_clone(grant)?.0;self.phase=4;},
  4=>{if let Some(scope)=scope{let mut text=String::new();text.try_reserve_exact(scope.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"replay alternative identity reservation failed"))?;self.row.as_mut().unwrap().1.scope=Some(text);self.phase=5;}else{self.phase=7;}},
  5=>{let text=self.row.as_mut().unwrap().1.scope.as_mut().unwrap();let original=scope.ok_or_else(||invalid("replay effective overlay original scope disappeared"))?;if text.len()==original.len(){self.phase=7;}else{append(text,original);}},
  7 if d.capacity_bytes!=0=>{p=map.reserve_insert_step(&self.row.as_ref().unwrap().0,grant).map_err(|(error,p)|error.with_retained_progress(p))?;},
  7=>{let (key,row)=self.row.take().unwrap();match map.insert_reserved(key,row,grant){Ok((old,_))=>{*self.displaced=old;self.phase=0;},Err((error,key,row))=>{*self.row=Some((key,row));return Err(error)}}},
  _=>return Err(invalid("replay effective overlay action changed")),
 }self.epochs=(drafts.mutation_epoch(),map.mutation_epoch());Ok(p)}
 /// 🏁️ All actual draft payloads have reached their canonical private effective rows.
 pub fn prepared(&self)->bool{self.complete&&!self.closing&&self.row.is_none()&&self.displaced.is_none()&&self.retirement.is_empty()}
 fn close_child_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty(){return self.retirement.demands(body)}self.retirement.next_owner_demands::<EffectiveSupersession>(true,&None,body)}
 fn close_child(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.retirement.is_empty(){return self.retirement.step_granted(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}Ok(self.retirement.begin_granted(&mut self.displaced,grant)?.map_or(RetainedCloneStep::Progress(Default::default()),|step|RetainedCloneStep::Progress(step.progress())))}
 /// 🛑️ Cancellation retains actual incoming and displaced rows independently of the enclosing replay result.
 pub fn begin_close(&mut self){self.closing=true;}
 /// 📏️ Quotes one canonical row's actual typed retirement frame and backing effects.
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty()||self.displaced.is_some(){return self.close_child_demands(body)}if self.row.is_some(){return self.retirement.next_owner_demands::<Row>(true,&None,body)}Ok(header::<Option<Source>>())}
 /// ♻️ Original payloads remain in their real canonical rows until a funded typed close consumes them.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.begin_close();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.retirement.is_empty()||self.displaced.is_some(){return self.close_child(grant)}if self.row.is_some(){return Ok(self.retirement.begin_granted(&mut self.row,grant)?.map_or(RetainedCloneStep::Progress(Default::default()),|step|RetainedCloneStep::Progress(step.progress())))}self.source=None;Ok(RetainedCloneStep::Progress(receipt(d)))}
 /// 🫙️ The enclosing result owns inserted rows; this cursor keeps no remaining original payload or borrowed coordinates.
 pub fn terminal_is_empty(&self)->bool{self.row.is_none()&&self.displaced.is_none()&&self.retirement.is_empty()&&self.source.is_none()}
}
impl Drop for OriginalReplaySupersessions{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"replay effective overlay requires funded original row closure");}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
