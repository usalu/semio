//! 📤️ Prepared original messages remain queued through failed transport attempts and bounded teardown.
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value_paged_list::PagedList;
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original outbox frontier differs from its queued message")}
fn error(e:semio_framework_value_paged_list::PagedListError)->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,e.reason)}
const LIMIT:usize=8192;
/// 📦️ Transport custody owns the original native paged message bytes.
pub type OriginalBackboneMessageBytes=semio_framework_value::paged::PagedBytes<{isize::MAX as usize}>;
/// 📮️ FIFO custody retains the exact prepared byte backing until acknowledged delivery or paid cancellation.
pub struct OriginalBackboneOutbox{rows:ManuallyDrop<PagedList<Option<OriginalBackboneMessageBytes>,{LIMIT}>>,head:usize,retiring:Option<ControlledRetirement<OriginalBackboneMessageBytes>>,closing:bool}
impl Default for OriginalBackboneOutbox{fn default()->Self{Self{rows:ManuallyDrop::new(PagedList::empty()),head:0,retiring:None,closing:false}}}
impl OriginalBackboneOutbox{
    pub fn len(&self)->usize{self.rows.len()-self.head}
    pub fn front(&self)->Option<&OriginalBackboneMessageBytes>{if self.closing{return None}self.rows.get(self.head).and_then(Option::as_ref)}
    pub fn append_demands(&self)->Result<RetirementDemand,ValueError>{if self.closing{return Err(invalid())}if self.rows.len()==LIMIT{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original outbox requires delivery before its authored row limit"))}let depth=self.rows.next_reserve_depth_demand().map_err(error)?;if self.rows.has_reserved_slot(){Ok(RetirementDemand{copy_bytes:size_of::<Option<OriginalBackboneMessageBytes>>(),depth:depth.max(1),..Default::default()})}else{Ok(RetirementDemand{capacity_bytes:self.rows.next_exact_capacity_allocation_bytes(LIMIT).map_err(error)?.ok_or_else(invalid)?,depth:depth.max(1),..Default::default()})}}
    pub fn reserve_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let d=self.append_demands()?;if !retained_clone_grant_admits(grant,d)||self.rows.has_reserved_slot(){return Ok(Default::default())}let step=self.rows.reserve_exact_capacity_one(LIMIT,grant.maximum_capacity_bytes).map_err(|e|error(e.refusal()).with_retained_progress(RetainedCloneProgress{copied_items:usize::from(e.allocated_bytes!=0),retained_capacity_bytes:e.allocated_bytes,..Default::default()}))?;Ok(RetainedCloneProgress{copied_items:usize::from(step.progressed),retained_capacity_bytes:step.allocated_bytes,..Default::default()})}
    pub fn can_append(&self,grant:RetainedCloneGrant)->Result<bool,ValueError>{Ok(self.rows.has_reserved_slot()&&retained_clone_grant_admits(grant,self.append_demands()?))}
    pub fn append_reserved(&mut self,bytes:&mut Option<OriginalBackboneMessageBytes>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{if bytes.is_none(){return Err(invalid())}if !self.can_append(grant)?{return Ok(None)}let d=self.append_demands()?;self.rows.push_reserved(bytes.take()).map_err(|_|invalid())?;Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
    /// 🚚️ Quotes the actual recipient reservation or one allocation-free original message donation.
    pub fn transfer_demands(&self,recipient:&Self)->Result<RetirementDemand,ValueError>{if self.closing||self.front().is_none(){return Err(invalid())}let mut d=recipient.append_demands()?;if recipient.rows.has_reserved_slot(){d.copy_bytes=d.copy_bytes.checked_add(size_of::<usize>()).ok_or_else(invalid)?;}Ok(d)}
    /// 📬️ Retains the source until its original backing moves into the admitted recipient slot.
    pub fn transfer_step(&mut self,recipient:&mut Self,grant:RetainedCloneGrant)->Result<Option<(bool,RetainedCloneProgress)>,ValueError>{
        let d=self.transfer_demands(recipient)?;if !retained_clone_grant_admits(grant,d){return Ok(None)}
        if !recipient.rows.has_reserved_slot(){let p=recipient.reserve_step(grant)?;return Ok((p!=Default::default()).then_some((false,p)))}
        let original=self.rows.get_mut(self.head).and_then(Option::take).ok_or_else(invalid)?;
        recipient.rows.push_reserved(Some(original)).unwrap_or_else(|_|panic!("original transport retains its admitted recipient slot"));self.head+=1;
        Ok(Some((true,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))
    }
    /// 🪟️ Quotes the actual channel's bounded header reservation before original byte custody moves.
    pub fn channel_demands(&self,queue:&std::collections::VecDeque<crate::BackboneMessage>)->Result<Option<RetirementDemand>,ValueError>{
        if self.closing{return Err(invalid())}if self.front().is_none()||queue.len()>=crate::BACKBONE_CHANNEL_MAXIMUM_MESSAGES{return Ok(None)}
        let width=size_of::<crate::BackboneMessage>();let d=if queue.len()==queue.capacity(){RetirementDemand{copy_bytes:queue.len()*width+size_of_val(queue),capacity_bytes:(queue.len()+1)*width,release_bytes:queue.capacity()*width,depth:1}}else{RetirementDemand{copy_bytes:width+size_of::<usize>(),depth:1,..Default::default()}};Ok(Some(d))
    }
    /// 🚇️ Donates directly into the canonical actor queue after its exact physical reservation succeeds.
    pub fn transfer_to_channel_step(&mut self,queue:&mut std::collections::VecDeque<crate::BackboneMessage>,grant:RetainedCloneGrant)->Result<Option<(bool,RetainedCloneProgress)>,ValueError>{
        let Some(d)=self.channel_demands(queue)?else{return Ok(None)};if !retained_clone_grant_admits(grant,d){return Ok(None)}
        if queue.len()==queue.capacity(){let released=queue.capacity()*size_of::<crate::BackboneMessage>();queue.try_reserve_exact(1).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original channel header reservation failed"))?;let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:queue.capacity()*size_of::<crate::BackboneMessage>(),released_bytes:released};if !p.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original channel reservation exceeds its physical quote").with_retained_progress(p))}return Ok(Some((false,p)))}
        let original=self.rows.get_mut(self.head).and_then(Option::take).ok_or_else(invalid)?;queue.push_back(crate::BackboneMessage::Mutations{envelopes:original});self.head+=1;
        Ok(Some((true,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))
    }
    pub fn acknowledgement_demands(&self)->Result<RetirementDemand,ValueError>{if self.closing||self.front().is_none()||self.retiring.is_some(){return Err(invalid())}Ok(RetirementDemand{copy_bytes:size_of::<Option<OriginalBackboneMessageBytes>>()+size_of::<ControlledRetirement<OriginalBackboneMessageBytes>>()+size_of::<usize>(),depth:1,..Default::default()})}
    pub fn acknowledge_delivered(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{let d=self.acknowledgement_demands()?;if !retained_clone_grant_admits(grant,d){return Ok(None)}let bytes=self.rows.get_mut(self.head).and_then(Option::take).ok_or_else(invalid)?;self.retiring=Some(ControlledRetirement::new(bytes).map_err(|(e,_)|e)?);self.head+=1;Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
    pub fn has_delivered_retirement(&self)->bool{self.retiring.is_some()}
    pub fn delivered_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{let owner=self.retiring.as_ref().ok_or_else(invalid)?;if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<OriginalBackboneMessageBytes>>>(),depth:1,..Default::default()})}else{owner.next_demand(body)}}
    pub fn close_delivered_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.delivered_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}let owner=self.retiring.as_mut().ok_or_else(invalid)?;if !owner.terminal_is_empty(){return owner.step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}self.retiring=None;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
    pub fn recycle_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.len()!=0{return Err(invalid())}self.close_demands(body)}
    pub fn recycle_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.recycle_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if self.retiring.is_some(){return self.close_delivered_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}if self.rows.pop().is_some(){self.head=self.rows.len();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}let step=self.rows.release_empty_page(grant.maximum_release_bytes).map_err(error)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes,..Default::default()})}else{RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes,..Default::default()})})}
    pub fn begin_close(&mut self){self.closing=true;}
    pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.retiring.is_some(){return self.delivered_demands(body)}if let Some(row)=self.rows.get(self.rows.len().saturating_sub(1)){return Ok(RetirementDemand{copy_bytes:size_of::<usize>()+size_of::<Option<OriginalBackboneMessageBytes>>()+if row.is_some(){size_of::<ControlledRetirement<OriginalBackboneMessageBytes>>()}else{0},depth:1,..Default::default()})}Ok(RetirementDemand{release_bytes:self.rows.next_release_allocation_bytes().map_err(error)?,depth:self.rows.next_release_depth_demand().map_err(error)?.max(1),..Default::default()})}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.closing{return Err(invalid())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if self.retiring.is_some(){return self.close_delivered_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}if let Some(row)=self.rows.pop(){if let Some(bytes)=row{self.retiring=Some(ControlledRetirement::new(bytes).map_err(|(e,_)|e)?)}self.head=self.head.min(self.rows.len());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}let step=self.rows.release_empty_page(grant.maximum_release_bytes).map_err(error)?;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes,..Default::default()}))}
    pub fn terminal_is_empty(&self)->bool{self.rows.terminal_is_empty()&&self.retiring.is_none()}
}
impl Drop for OriginalBackboneOutbox{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original outbox requires paid closure");}}

#[cfg(test)]
#[path="🧪️.rs"]
mod tests;
