//! 🎮️ Admits and fills one native UTF-8 chunk under separate capacity and payload grants.

use super::{PagedUtf8, PAGED_UTF8_CHUNK_BYTES};
use crate::{retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}, ErasedSnapshotRetirement, ValueError, ValueRefusalKind};

#[derive(Default)]
pub struct PagedUtf8AppendCursor {
    source: Option<(usize, usize)>,
    destination: Option<(usize, usize)>,
    position: usize,
    end: usize,
    pending: Option<String>,
    bytes: Option<Vec<u8>>,
    advance_fault: Option<&'static str>,
    closing: bool,
    close_faulted: bool,
    #[doc(hidden)] pub complete: bool,
    controlled_close: crate::retained_clone::RetainedCloneClose,
}

impl PagedUtf8AppendCursor {
    /// 📏️ Quotes the same original byte body before any backing, payload or owner transition.
    pub fn next_advance_demand<const N:usize>(&self,source:&str,destination:&PagedUtf8<N>,body:usize)->Result<crate::RetirementDemand,ValueError>{
        if let Some(message)=self.advance_fault{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,message))}if self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native UTF-8 append cursor is closing"))}if self.source.is_some_and(|prior|prior!=(source.as_ptr()as usize,source.len())){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native UTF-8 append source changed"))}if self.destination.is_some_and(|(prior,bytes)|prior!=destination as*const PagedUtf8<N>as usize||bytes!=destination.byte_len){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native UTF-8 append destination changed"))}if self.source.is_none(){destination.byte_len.checked_add(source.len()).filter(|bytes|*bytes<=N).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native UTF-8 append exceeds its declared capacity"))?;return Ok(crate::RetirementDemand{depth:1,..Default::default()})}let mut d=crate::RetirementDemand{depth:1,..Default::default()};if self.complete||self.position==source.len()&&self.bytes.is_none()&&self.pending.is_none(){return Ok(d)}if self.pending.is_some(){d.copy_bytes=size_of::<String>();d.depth=destination.chunks.next_push_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"native UTF-8 append publication depth overflow"))?;return Ok(d)}if self.bytes.is_some(){if self.position<self.end{d.copy_bytes=body.max(1).min(self.end-self.position)}return Ok(d)}if !destination.chunks.has_reserved_slot(){d.copy_bytes=destination.chunks.next_reserve_copy_byte_demand()?;d.capacity_bytes=destination.chunks.next_allocation_bytes()?;d.depth=destination.chunks.next_reserve_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"native UTF-8 append reserve depth overflow"))?;return Ok(d)}let mut end=self.position.saturating_add(PAGED_UTF8_CHUNK_BYTES).min(source.len());while !source.is_char_boundary(end){end-=1}d.capacity_bytes=end-self.position;Ok(d)
    }
    /// 🎮️ Every outcome retains actual allocator, byte store, validation or metadata work before any refusal.
    pub fn advance<const N:usize>(&mut self,source:&str,destination:&mut PagedUtf8<N>,body:usize,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{
        if let Some(message)=self.advance_fault{return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,message)),work:0}}let d=match self.next_advance_demand(source,destination,body){Ok(d)=>d,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};if self.complete{return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}if !crate::retained_clone::retained_clone_grant_admits(grant,d){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}let control=||RetainedCloneProgress{copied_items:1,..Default::default()};
        if self.source.is_none(){self.source=Some((source.as_ptr()as usize,source.len()));self.destination=Some((destination as*const PagedUtf8<N>as usize,destination.byte_len));return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(control())),work:1}}
        if self.position==source.len()&&self.bytes.is_none()&&self.pending.is_none(){self.complete=true;return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(control())),work:1}}
        if self.pending.is_some(){let length=self.pending.as_ref().unwrap().len();let Some(total)=destination.byte_len.checked_add(length).filter(|n|*n<=N)else{return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native UTF-8 append exceeds its declared capacity")),work:0}};let value=self.pending.take().unwrap();if let Err(value)=destination.chunks.push_reserved(value){self.pending=Some(value);self.advance_fault=Some("native UTF-8 append lost its admitted slot");return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native UTF-8 append lost its admitted slot").with_retained_progress(control())),work:1}}destination.byte_len=total;self.destination=Some((destination as*const PagedUtf8<N>as usize,total));return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<String>(),..Default::default()})),work:1}}
        if self.bytes.is_some(){if self.position<self.end{if body==0{return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}let end=self.position+d.copy_bytes;let bytes=self.bytes.as_mut().unwrap();if bytes.capacity()-bytes.len()<d.copy_bytes{self.advance_fault=Some("native UTF-8 append lost its admitted chunk capacity");return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native UTF-8 append lost its admitted chunk capacity").with_retained_progress(control())),work:1}}bytes.extend_from_slice(&source.as_bytes()[self.position..end]);self.position=end;return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}}let bytes=self.bytes.take().unwrap();return match String::from_utf8(bytes){Ok(value)=>{self.pending=Some(value);crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(control())),work:1}},Err(error)=>{self.bytes=Some(error.into_bytes());self.advance_fault=Some("native UTF-8 append source changed");crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native UTF-8 append source changed").with_retained_progress(control())),work:1}}}}
        if !destination.chunks.has_reserved_slot(){return match destination.chunks.reserve_one(grant.maximum_capacity_bytes){Ok(p)=>crate::RetainedWorkOutcome{work:p.work,result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(p.progressed),copied_bytes:if p.progressed{d.copy_bytes}else{0},retained_capacity_bytes:p.allocated_bytes,released_bytes:p.released_allocation_bytes}))},Err(error)=>crate::RetainedWorkOutcome{work:error.work,result:Err(ValueError::from(error.refusal()).with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:error.allocated_bytes,..Default::default()}))}}}
        let mut end=self.position.saturating_add(PAGED_UTF8_CHUNK_BYTES).min(source.len());while !source.is_char_boundary(end){end-=1}let mut bytes=Vec::new();if bytes.try_reserve_exact(end-self.position).is_err(){return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"native UTF-8 append chunk allocation failed")),work:1}}let capacity=bytes.capacity();self.bytes=Some(bytes);self.end=end;let p=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()};if capacity>grant.maximum_capacity_bytes{return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native UTF-8 append allocator exceeded its admitted chunk").with_retained_progress(p)),work:1}}crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(p)),work:1}
    }
    pub fn begin_close(&mut self) -> bool { if self.closing { return false; } self.closing = true; self.close_faulted = false; true }
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{
        if !self.controlled_close.is_empty(){return self.controlled_close.demands(body)}if self.pending.is_some(){return self.controlled_close.next_owner_demands::<String>(true,&None,body)}if self.bytes.is_some(){return self.controlled_close.next_owner_demands::<Vec<u8>>(true,&None,body)}
        Ok(crate::RetirementDemand {depth:usize::from(!self.terminal_is_empty()),..Default::default()})
    }
    /// 🧮️ Reads the exact next pending chunk or metadata movement.
    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    /// 📐️ Reads the original pending chunk's typed retirement frame allocation.
    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    /// 🪙️ Reads the original pending chunk or terminal frame release.
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> crate::RetainedWorkOutcome {
        if self.terminal_is_empty() { return crate::RetainedWorkOutcome { result: Ok(RetainedCloneStep::Complete(Default::default())), work: 0 }; }
        if grant.maximum_items == 0 { return crate::RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(Default::default())), work: 0 }; }
        if !self.closing {
            let work = u64::from(!self.close_faulted);
            self.close_faulted = true;
            return crate::RetainedWorkOutcome { result: Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "native UTF-8 append must begin close before granted retirement")), work };
        }
        let demand = match self.close_demands(grant.maximum_copy_bytes) {
            Ok(demand) => demand,
            Err(error) => return crate::RetainedWorkOutcome { result: Err(error), work: 0 },
        };
        if !crate::retained_clone::retained_clone_grant_admits(grant, demand) { return crate::RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(Default::default())), work: 0 }; }
        if !self.controlled_close.is_empty() { let turn=self.controlled_close.step_granted(grant);return crate::RetainedWorkOutcome{result:turn.result.map(|s|RetainedCloneStep::Progress(s.progress())),work:turn.work}; }
        if self.pending.is_some() {
            let turn = self.controlled_close.begin_granted(&mut self.pending, grant);
            return crate::RetainedWorkOutcome { result: turn.result.and_then(|step|step.map(|s|RetainedCloneStep::Progress(s.progress())).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "native UTF-8 append retirement omitted its retained pending chunk"))), work: turn.work };
        }
        if self.bytes.is_some(){let turn=self.controlled_close.begin_granted(&mut self.bytes,grant);return crate::RetainedWorkOutcome{result:turn.result.map(|s|RetainedCloneStep::Progress(s.unwrap_or(RetainedCloneStep::Progress(Default::default())).progress())),work:turn.work};}
        self.source = None;
        self.destination = None;
        self.advance_fault = None;
        crate::RetainedWorkOutcome { result: Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() })), work: 1 }
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.source.is_none() && self.destination.is_none() && self.pending.is_none() && self.bytes.is_none() && self.advance_fault.is_none() && self.controlled_close.is_empty() }
}

impl ErasedSnapshotRetirement for PagedUtf8AppendCursor {
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{self.close_demands(body)}
    fn close_step(&mut self, grant: RetainedCloneGrant) -> crate::RetainedWorkOutcome { Self::close_step(self, grant) }
    fn terminal_is_empty(&self) -> bool { PagedUtf8AppendCursor::terminal_is_empty(self) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.next_close_copy_byte_demand() }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { self.next_close_capacity_byte_demand(copy) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.next_close_release_byte_demand() }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.depth) }
}

impl Drop for PagedUtf8AppendCursor {
    fn drop(&mut self) { assert!((self.source.is_none() && self.destination.is_none() && self.pending.is_none() && self.bytes.is_none() && self.advance_fault.is_none() && self.controlled_close.is_empty()) || std::thread::panicking(), "native UTF-8 append cursor dropped before exact closure"); }
}


#[cfg(test)]
#[path="♻️close/🧪️tests/🦀️.rs"]
mod close_work_tests;

#[cfg(test)]#[path="🎟️work/🧪️tests/🦀️.rs"]mod normal_work_tests;
