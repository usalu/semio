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
    closing: bool,
    #[doc(hidden)] pub complete: bool,
    controlled_close: crate::retained_clone::RetainedCloneClose,
}

impl PagedUtf8AppendCursor {
    /// 📏️ Reads the exact next native page, UTF8 scalar or original chunk transfer without changing custody.
    fn advance_metadata_bytes()->usize{2*size_of::<Option<(usize,usize)>>()+2*size_of::<usize>()+size_of::<bool>()+size_of::<Option<String>>()+size_of::<usize>()}
    /// 🎟️ Original structural and cursor-state effects share the same caller grant before either owner advances.
    pub fn next_advance_demand<const N:usize>(&self,source:&str,destination:&PagedUtf8<N>)->Result<crate::RetirementDemand,ValueError>{let mut d=self.next_body_demand(source,destination)?;d.copy_bytes=d.copy_bytes.checked_add(Self::advance_metadata_bytes()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native UTF-8 append metadata overflow"))?;d.depth=d.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native UTF-8 append depth overflow"))?;Ok(d)}
    pub fn advance<const N:usize>(&mut self,source:&str,destination:&mut PagedUtf8<N>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.next_advance_demand(source,destination)?;if !crate::retained_clone::retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}let metadata=Self::advance_metadata_bytes();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-metadata,maximum_depth:grant.maximum_depth-1,..grant};match self.advance_body(source,destination,child){Ok(step)=>{let complete=matches!(step,RetainedCloneStep::Complete(_));let mut p=step.progress();p.copied_items=p.copied_items.max(1);p.copied_bytes=p.copied_bytes.checked_add(metadata).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native UTF-8 append receipt overflow"))?;Ok(if complete{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})},Err(error)=>{let mut p=error.retained_progress();p.copied_items=p.copied_items.max(1);p.copied_bytes=p.copied_bytes.saturating_add(metadata);Err(error.with_retained_progress(p))}}}
    fn next_body_demand<const N:usize>(&self,source:&str,destination:&PagedUtf8<N>)->Result<crate::RetirementDemand,ValueError>{
        let refusal=|message|ValueError::literal(ValueRefusalKind::InvariantViolated,message);
        if self.closing{return Err(refusal("native UTF-8 append cursor is closing"));}
        if self.source.is_some_and(|prior|prior!=(source.as_ptr()as usize,source.len())){return Err(refusal("native UTF-8 append source changed"));}
        if self.destination.is_some_and(|(prior,bytes)|prior!=destination as*const PagedUtf8<N>as usize||bytes!=destination.byte_len){return Err(refusal("native UTF-8 append destination changed"));}
        if self.source.is_none(){destination.byte_len.checked_add(source.len()).filter(|bytes|*bytes<=N).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native UTF-8 append exceeds its declared capacity"))?;}
        let mut demand=crate::RetirementDemand{depth:1,..Default::default()};
        if self.complete||self.position==source.len()&&self.pending.is_none(){return Ok(demand);}
        if !destination.chunks.has_reserved_slot(){demand.copy_bytes=destination.chunks.next_reserve_copy_byte_demand()?;demand.capacity_bytes=destination.chunks.next_allocation_bytes()?;demand.depth=destination.chunks.next_reserve_depth_demand()?;return Ok(demand);}
        if self.pending.is_none(){let mut end=(self.position+PAGED_UTF8_CHUNK_BYTES).min(source.len());while !source.is_char_boundary(end){end-=1;}demand.copy_bytes=size_of::<String>();demand.capacity_bytes=end-self.position;}
        else if self.position<self.end{demand.copy_bytes=source[self.position..self.end].chars().next().ok_or_else(||refusal("native UTF-8 append scalar is absent"))?.len_utf8();}
        else{demand.copy_bytes=size_of::<String>();}
        Ok(demand)
    }
    fn advance_body<const N: usize>(&mut self, source: &str, destination: &mut PagedUtf8<N>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "native UTF-8 append cursor is closing")); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
        let demand=self.next_body_demand(source,destination)?;
        if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
        let identity = (source.as_ptr() as usize, source.len());
        if self.source.is_some_and(|prior| prior != identity) { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "native UTF-8 append source changed")); }
        if self.source.is_none() { destination.byte_len.checked_add(source.len()).filter(|bytes| *bytes <= N).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native UTF-8 append exceeds its declared capacity"))?; }
        self.source = Some(identity);
        let destination_identity = destination as *const PagedUtf8<N> as usize;
        if self.destination.is_some_and(|(prior, bytes)| prior != destination_identity || bytes != destination.byte_len) { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "native UTF-8 append destination changed")); }
        self.destination = Some((destination_identity, destination.byte_len));
        if self.complete { return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())); }
        if self.position == source.len() && self.pending.is_none() {
            self.complete = true;
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if !destination.chunks.has_reserved_slot() {
            let demand = destination.chunks.next_allocation_bytes()?;
            if demand > grant.maximum_capacity_bytes { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
            let progress = destination.chunks.reserve_one_funded(grant).map_err(|error| ValueError::from(error.refusal()).with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:error.allocated_bytes,..Default::default()}))?;
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if self.pending.is_none() {
            let mut end = (self.position + PAGED_UTF8_CHUNK_BYTES).min(source.len());
            while !source.is_char_boundary(end) { end -= 1; }
            let bytes = end - self.position;
            destination.byte_len.checked_add(bytes).filter(|bytes| *bytes <= N).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native UTF-8 append exceeds its declared capacity"))?;
            if bytes > grant.maximum_capacity_bytes { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
            let mut pending = String::new();
            pending.try_reserve_exact(bytes).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "native UTF-8 append chunk allocation failed"))?;
            let capacity = pending.capacity();
            self.pending = Some(pending);
            if capacity > grant.maximum_capacity_bytes { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "native UTF-8 append allocator exceeded its admitted chunk").with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:capacity,..Default::default()})); }
            self.end = end;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: capacity, released_bytes: 0 }));
        }
        if self.position < self.end {
            let mut end = self.position.saturating_add(grant.maximum_copy_bytes).min(self.end);
            while end > self.position && !source.is_char_boundary(end) { end -= 1; }
            if end == self.position { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
            let bytes = end - self.position;
            self.pending.as_mut().expect("native append chunk owner").push_str(&source[self.position..end]);
            self.position = end;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0, released_bytes: 0 }));
        }
        let bytes = size_of::<String>();
        if bytes > grant.maximum_copy_bytes { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
        let pending = self.pending.take().expect("native append published chunk");
        let length = pending.len();
        if let Err(pending) = destination.chunks.push_reserved(pending) { self.pending = Some(pending); return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "native UTF-8 append lost its admitted slot")); }
        destination.byte_len += length;
        self.destination = Some((destination_identity, destination.byte_len));
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: 0, released_bytes: 0 }))
    }
    pub fn begin_close(&mut self) -> bool { if self.closing { return false; } self.closing = true; true }
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{
        if !self.controlled_close.is_empty()||self.pending.is_some(){return self.controlled_close.next_owner_demands::<String>(self.pending.is_some(),&None,body);}
        Ok(crate::RetirementDemand {copy_bytes:if self.source.is_some()||self.destination.is_some(){2*size_of::<Option<(usize,usize)>>() }else{0},depth:usize::from(!self.terminal_is_empty()),..Default::default()})
    }
    /// 🧮️ Reads the exact next pending chunk or metadata movement.
    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    /// 📐️ Reads the original pending chunk's typed retirement frame allocation.
    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    /// 🪙️ Reads the original pending chunk or terminal frame release.
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "native UTF-8 append must begin close before granted retirement")); }
        if !crate::retained_clone::retained_clone_grant_admits(grant,self.close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if !self.controlled_close.is_empty() { return self.controlled_close.step_granted(grant).map(|step|RetainedCloneStep::Progress(step.progress())); }
        if let Some(step) = self.controlled_close.begin_granted(&mut self.pending, grant)? { return Ok(step); }
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let copy=2*size_of::<Option<(usize,usize)>>();
        if grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return Ok(RetainedCloneStep::Progress(Default::default()));}
        self.source = None;
        self.destination = None;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes:copy, ..Default::default() }))
    }
    
    pub fn terminal_is_empty(&self) -> bool { self.closing && self.source.is_none() && self.destination.is_none() && self.pending.is_none() && self.controlled_close.is_empty() }
}

impl ErasedSnapshotRetirement for PagedUtf8AppendCursor {
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{self.close_demands(body)}
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Self::close_step(self, grant) }
    fn terminal_is_empty(&self) -> bool { PagedUtf8AppendCursor::terminal_is_empty(self) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.next_close_copy_byte_demand() }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { self.next_close_capacity_byte_demand(copy) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.next_close_release_byte_demand() }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.depth) }
}

impl Drop for PagedUtf8AppendCursor {
    fn drop(&mut self) { assert!((self.source.is_none() && self.destination.is_none() && self.pending.is_none() && self.controlled_close.is_empty()) || std::thread::panicking(), "native UTF-8 append cursor dropped before exact closure"); }
}

