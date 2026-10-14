//! 🪢️ The original validation loop frontier pays source rows, diagnostic text and finding pages separately.
use super::*;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
use std::fmt::{self,Write};

#[derive(Clone,Copy)]
pub(super) struct LoopFinding {loop_id:LoopId,index:usize,next:usize,mismatch:bool}
semio_framework_value::artifact_retire_leaf!(LoopFinding);
impl LoopFinding {
 fn code(self)->&'static str{if self.mismatch{"next-prev-mismatch"}else{"loop-not-closed"}}
 fn text(self,entity:bool,target:&mut impl Write)->fmt::Result{if entity{write!(target,"loop-{}",self.loop_id.raw_index())}else if self.mismatch{write!(target,"coedge {}'s next/prev pointers are not symmetric with its ring neighbor",self.index)}else{write!(target,"coedge {} ends at a different vertex than coedge {} starts at",self.index,self.next)}}
 fn length(self,entity:bool)->Result<usize,ValueError>{let mut count=TextLength(0);self.text(entity,&mut count).map_err(|_|refusal())?;Ok(count.0)}
}
struct TextLength(usize);
impl Write for TextLength{fn write_str(&mut self,text:&str)->fmt::Result{self.0=self.0.checked_add(text.len()).ok_or(fmt::Error)?;Ok(())}}
struct TextSlice<'a>{target:&'a mut String,start:usize,end:usize,visited:usize}
impl Write for TextSlice<'_>{fn write_str(&mut self,text:&str)->fmt::Result{let end=self.visited.checked_add(text.len()).ok_or(fmt::Error)?;let from=self.start.saturating_sub(self.visited).min(text.len());let to=self.end.saturating_sub(self.visited).min(text.len());if to>from{self.target.push_str(&text[from..to]);}self.visited=end;Ok(())}}
fn refusal()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original validation loop source or frontier changed")}
fn allocation()->ValueError{ValueError::literal(ValueRefusalKind::AllocationFailed,"original validation loop backing allocation failed")}
fn metadata(depth:usize)->RetirementDemand{RetirementDemand{depth,..Default::default()}}

impl BodyValidationJob {
 /// 🧮️ Quotes the next original validation event without allocating or mutating the source.
 pub fn funded_demands(&self,body:&Body)->Result<RetirementDemand,ValueError>{
  if self.source_body!=body as*const Body as usize{return Err(refusal())}
  match self.phase{BodyValidationPhase::CollectingTopology=>self.planning_demands(body),BodyValidationPhase::LoopRings=>self.loop_demands(body),BodyValidationPhase::Complete=>Ok(Default::default()),_=>Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original validation phase has no funded producer yet"))}
 }
 fn loop_demands(&self,_body:&Body)->Result<RetirementDemand,ValueError>{
  if let Some(finding)=self.loop_findings.get(self.loop_finding_cursor).copied().flatten(){
   let Some(output)=&self.loop_output else{return Ok(metadata(1))};
   if self.loop_text_stage<2{let entity=self.loop_text_stage==0;let text=if entity{&output.entity}else{&output.message};let length=finding.length(entity)?;return Ok(if text.len()==length{metadata(1)}else if text.capacity()==0{RetirementDemand{capacity_bytes:length,depth:1,..Default::default()}}else{RetirementDemand{copy_bytes:1,depth:1,..Default::default()}})}
   let capacity=self.issues.next_allocation_bytes().map_err(|_|refusal())?;
   return Ok(if capacity!=0{RetirementDemand{copy_bytes:self.issues.next_reserve_copy_byte_demand().map_err(|_|refusal())?,capacity_bytes:capacity,depth:self.issues.next_reserve_depth_demand().map_err(|_|refusal())?,..Default::default()}}else{metadata(self.issues.next_push_depth_demand().map_err(|_|refusal())?.max(1))})
  }
  if self.loop_id.is_some()&&self.ring_next.is_some(){if self.ring.capacity()==0{let extent=self.live_coedges.checked_add(1).ok_or_else(refusal)?;Ok(RetirementDemand{capacity_bytes:extent.checked_mul(std::mem::size_of::<CoedgeId>()).ok_or_else(refusal)?,depth:1,..Default::default()})}else if self.ring.len()<self.ring.capacity(){Ok(RetirementDemand{copy_bytes:std::mem::size_of::<CoedgeId>(),depth:1,..Default::default()})}else{Err(refusal())}}else{Ok(metadata(1))}
 }
 /// ⏱️ Advances one original planning or loop event under its full physical authority.
 pub fn step_funded(&mut self,body:&Body,units:usize,grant:RetainedCloneGrant)->Result<(RetainedCloneProgress,usize),ValueError>{
  let demand=self.funded_demands(body)?;
  if self.is_complete()||units==0||grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok((Default::default(),0))}
  if self.phase==BodyValidationPhase::CollectingTopology{return self.step_planning_funded(body,units,grant)}
  let mut receipt=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
  if let Some(finding)=self.loop_findings.get(self.loop_finding_cursor).copied().flatten(){
   if self.loop_output.is_none(){self.loop_output=Some(ValidationIssue{entity:String::new(),code:finding.code(),message:String::new()});}
   else if self.loop_text_stage<2{
    let entity=self.loop_text_stage==0;let output=self.loop_output.as_mut().unwrap();let text=if entity{&mut output.entity}else{&mut output.message};let length=finding.length(entity)?;
    if text.len()==length{self.loop_text_stage+=1;}
    else if text.capacity()==0{text.try_reserve_exact(length).map_err(|_|allocation())?;receipt.retained_capacity_bytes=text.capacity();}
    else{let from=text.len();let end=from.saturating_add(grant.maximum_copy_bytes).min(length);finding.text(entity,&mut TextSlice{target:text,start:from,end,visited:0}).map_err(|_|refusal())?;receipt.copied_bytes=text.len()-from;}
   }else if demand.capacity_bytes!=0{receipt=self.issues.reserve_one_funded(grant).map_err(|error|allocation().with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:error.allocated_bytes,..Default::default()}))?;}
   else{let output=self.loop_output.take().unwrap();if let Err(output)=self.issues.push_reserved(output){self.loop_output=Some(output);return Err(refusal())}self.loop_findings[self.loop_finding_cursor]=None;self.loop_finding_cursor+=1;self.loop_text_stage=0;}
  }else if self.loop_finding_cursor!=0{self.loop_finding_cursor=0;self.loop_findings=[None,None];self.ring_probe+=1;}
  else if self.loop_id.is_some()&&self.ring_next.is_some()&&self.ring.capacity()==0{let extent=self.live_coedges.checked_add(1).ok_or_else(refusal)?;self.ring.try_reserve_exact(extent).map_err(|_|allocation())?;receipt.retained_capacity_bytes=self.ring.capacity()*std::mem::size_of::<CoedgeId>();}
  else if let Some(loop_id)=self.loop_id{
   if let Some(current)=self.ring_next.take(){self.ring.push(current);if let Some(coedge)=body.coedges.get(current){if self.ring.len()<=self.live_coedges&&self.ring.first().copied()!=Some(coedge.next){self.ring_next=Some(coedge.next);}}}
   else if self.ring_probe<self.ring.len(){
    let index=self.ring_probe;let next=(index+1)%self.ring.len();let pair=body.coedge_endpoints(self.ring[index]).zip(body.coedge_endpoints(self.ring[next]));
    if let Some(((_,end),(start,_)))=pair{let a=body.coedges.get(self.ring[index]).ok_or_else(refusal)?;let b=body.coedges.get(self.ring[next]).ok_or_else(refusal)?;let mut found=0;if end!=start{self.loop_findings[found]=Some(LoopFinding{loop_id,index,next,mismatch:false});found+=1;}if a.next!=self.ring[next]||b.prev!=self.ring[index]{self.loop_findings[found]=Some(LoopFinding{loop_id,index,next,mismatch:true});found+=1;}if found==0{self.ring_probe+=1;}}
    else{self.ring_probe+=1;}
   }else{self.ring.clear();self.loop_id=None;}
  }else if self.loop_slot>=body.loops.slot_count(){self.phase=BodyValidationPhase::EdgeValence;}
  else{if let Some((id,lp))=body.loops.slot_at(self.loop_slot){self.loop_id=Some(id);self.ring_next=Some(lp.first);self.ring_probe=0;}self.loop_slot+=1;}
  self.units_done+=1;self.units_total=self.units_total.max(self.units_done.saturating_add(1));
  if !receipt.fits(grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original validation loop allocator exceeded admitted backing").with_retained_progress(receipt))}Ok((receipt,1))
 }
}
