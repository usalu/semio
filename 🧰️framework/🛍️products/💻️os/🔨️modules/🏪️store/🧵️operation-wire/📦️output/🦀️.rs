//! 📦️ Original operation octets retain each supplied-grant backing until paid handoff or closure.
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,ErasedSnapshotRetirement};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::list::{PagedList,PagedListError};
use std::mem::ManuallyDrop;
#[path="📋️operations/🦀️.rs"]
mod operations;
pub use operations::{ArtifactPreparedOperations,ArtifactPreparedOperationsOwner};

/// 🧱️ Payload extent and total physical capacity belong to the embedding operation owner.
pub struct ArtifactPreparedOperationOutput {
    bytes:ManuallyDrop<PagedList<u8,{isize::MAX as usize}>>,
    contiguous:ManuallyDrop<Option<Vec<u8>>>,
    contiguous_complete:bool,
    maximum_payload_bytes:usize,
    maximum_capacity_bytes:usize,
    closing:bool,
    closed:bool,
}

fn refusal(error:PagedListError)->ValueError {
    let kind=match error.kind {semio_framework_value::list::PagedListRefusalKind::AllocationFailed=>ValueRefusalKind::AllocationFailed,semio_framework_value::list::PagedListRefusalKind::OwnershipLimit=>ValueRefusalKind::OwnershipLimit,_=>ValueRefusalKind::InvariantViolated};
    ValueError::literal(kind,error.reason)
}

impl ArtifactPreparedOperationOutput {
    /// 🎟️ Installs original caller limits without constructing any heap backing.
    pub fn empty(maximum_payload_bytes:usize,maximum_capacity_bytes:usize)->Result<Self,ValueError> {
        if maximum_payload_bytes>isize::MAX as usize||maximum_capacity_bytes>isize::MAX as usize{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"operation output limits exceed addressable ownership"));}
        Ok(Self{bytes:ManuallyDrop::new(PagedList::empty()),contiguous:ManuallyDrop::new(None),contiguous_complete:false,maximum_payload_bytes,maximum_capacity_bytes,closing:false,closed:false})
    }
    pub fn begin_close(&mut self){self.closing=true;}
    pub fn len(&self)->usize{self.bytes.len()}
    pub fn allocated_bytes(&self)->usize{self.bytes.allocated_bytes()+self.contiguous.as_ref().map_or(0,Vec::capacity)}
    pub fn byte_at(&self,index:usize)->Option<u8>{self.bytes.get(index).copied()}
    pub fn iter(&self)->impl ExactSizeIterator<Item=u8>+'_ {self.bytes.iter().copied()}

    /// 🛂️ Admits one original page birth or at most sixty-four initialized octets.
    pub fn append(&mut self,source:&[u8],grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        if self.closing||self.closed||self.contiguous.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"operation output is closing or transferring"));}
        if source.is_empty()||grant.maximum_items==0{return Ok(Default::default());}
        if source.len()>self.maximum_payload_bytes-self.bytes.len(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"operation output payload extent exhausted"));}
        let depth=self.bytes.next_reserve_depth_demand().map_err(refusal)?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"operation output admission depth overflow"))?;
        if grant.maximum_depth<depth{return Ok(Default::default());}
        if !self.bytes.has_reserved_slot(){
            let required=self.bytes.next_exact_capacity_allocation_bytes(self.maximum_payload_bytes).map_err(refusal)?.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"operation output lost its unfunded payload page"))?;
            if self.bytes.allocated_bytes().checked_add(required).is_none_or(|total|total>self.maximum_capacity_bytes){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"operation output total backing authority exhausted"));}
            if grant.maximum_capacity_bytes<required{return Ok(Default::default());}
            let capacity=grant.maximum_capacity_bytes.min(self.maximum_capacity_bytes-self.bytes.allocated_bytes());
            let step=self.bytes.reserve_exact_capacity_one(self.maximum_payload_bytes,capacity).map_err(|error|refusal(error.refusal()).with_retained_progress(RetainedCloneProgress{copied_items:usize::from(error.allocated_bytes!=0),retained_capacity_bytes:error.allocated_bytes,..Default::default()}))?;
            let progress=RetainedCloneProgress{copied_items:usize::from(step.progressed),retained_capacity_bytes:step.allocated_bytes,..Default::default()};
            if self.bytes.allocated_bytes()>self.maximum_capacity_bytes||!progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"operation output retained actual backing beyond supplied authority").with_retained_progress(progress));}
            return Ok(progress);
        }
        let count=source.len().min(64).min(grant.maximum_copy_bytes).min(self.bytes.capacity()-self.bytes.len());
        if count==0{return Ok(Default::default());}
        let mut progress=RetainedCloneProgress{copied_items:1,..Default::default()};
        for byte in &source[..count]{
            self.bytes.push_reserved(*byte).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"operation output admitted slot rejected original octet").with_retained_progress(progress))?;
            progress.copied_bytes+=1;
        }
        Ok(progress)
    }

    /// 🤝️ Moves the exact original byte backing into the canonical Value payload without allocation.
    pub fn take_bytes(&mut self,grant:RetainedCloneGrant)->Result<Option<(semio_framework_value::paged::PagedBytes<{isize::MAX as usize}>,RetainedCloneProgress)>,ValueError>{
        if self.closing||self.closed||self.contiguous.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"operation output handoff requires its live paged owner"));}
        let copy=std::mem::size_of::<PagedList<u8,{isize::MAX as usize}>>();
        if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(None);}
        let original=std::mem::replace(&mut *self.bytes,PagedList::empty());self.closed=true;
        Ok(Some((semio_framework_value::paged::PagedBytes::from_retained_bytes(original),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})))
    }

    /// 🧾️ Retains the exact contiguous proposal prefix alongside its original paid pages.
    pub fn contiguous_prefix(&self)->Option<&[u8]>{self.contiguous.as_ref().map(Vec::as_slice)}

    /// 📬️ Pays the genuine contiguous protocol backing and every copied byte before releasing original pages.
    pub fn prepare_contiguous(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.closing||self.closed{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"contiguous operation transfer requires its live original owner"));}
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.contiguous.is_none(){
            let capacity=self.bytes.len();let copied_bytes=std::mem::size_of::<Vec<u8>>();
            if self.bytes.allocated_bytes().checked_add(capacity).is_none_or(|total|total>self.maximum_capacity_bytes){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"contiguous operation backing exceeds original total capacity authority"));}
            if grant.maximum_capacity_bytes<capacity||grant.maximum_copy_bytes<copied_bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
            let output=if capacity==0{Vec::new()}else{
                let layout=std::alloc::Layout::array::<u8>(capacity).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"contiguous operation backing exceeds addressable ownership"))?;
                let pointer=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}).ok_or_else(||ValueError::literal(ValueRefusalKind::AllocationFailed,"contiguous operation backing allocation refused"))?;
                unsafe{Vec::from_raw_parts(pointer.as_ptr(),0,capacity)}
            };
            *self.contiguous=Some(output);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes,retained_capacity_bytes:capacity,..Default::default()}));
        }
        if !self.contiguous_complete{
            let output=self.contiguous.as_mut().expect("contiguous transfer retains its actual buffer");
            let count=(self.bytes.len()-output.len()).min(64).min(grant.maximum_copy_bytes);
            if output.len()==self.bytes.len(){
                if grant.maximum_copy_bytes<std::mem::size_of::<bool>(){return Ok(RetainedCloneStep::Progress(Default::default()));}
                self.contiguous_complete=true;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<bool>(),..Default::default()}));
            }
            if count==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
            let mut progress=RetainedCloneProgress{copied_items:1,..Default::default()};
            for _ in 0..count{
                let byte=self.bytes.get(output.len()).copied().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"contiguous operation lost its original initialized byte").with_retained_progress(progress))?;
                output.push(byte);progress.copied_bytes+=1;
            }
            return Ok(RetainedCloneStep::Progress(progress));
        }
        self.close_paged_one(grant)
    }

    /// 🤝️ Hands the exact already funded contiguous buffer to the actual protocol recipient.
    pub fn take_contiguous(&mut self,grant:RetainedCloneGrant)->Result<Option<(Vec<u8>,RetainedCloneProgress)>,ValueError>{
        if self.closing||self.closed{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"contiguous operation handoff requires its live original owner"));}
        let copied_bytes=std::mem::size_of::<Vec<u8>>();
        if !self.contiguous_complete||!self.bytes.terminal_is_empty()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copied_bytes{return Ok(None);}
        let original=self.contiguous.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"contiguous operation handoff lost its original backing"))?;self.closed=true;
        Ok(Some((original,RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()})))
    }

    fn paged_demands(&self)->Result<RetirementDemand,ValueError>{
        let(copy_bytes,release_bytes,depth)=if !self.bytes.is_empty(){(1,0,self.bytes.next_pop_depth_demand().map_err(refusal)?)}else{(0,self.bytes.next_release_allocation_bytes().map_err(refusal)?,self.bytes.next_release_depth_demand().map_err(refusal)?)};
        Ok(RetirementDemand{copy_bytes,release_bytes,depth:depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"operation output retirement depth overflow"))?,..Default::default()})
    }

    fn close_paged_one(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.bytes.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        let demand=self.paged_demands()?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
        let mut progress=RetainedCloneProgress{copied_items:1,..Default::default()};
        if !self.bytes.is_empty(){self.bytes.pop();progress.copied_bytes=1;}
        else{let step=self.bytes.release_empty_page(grant.maximum_release_bytes).map_err(|error|refusal(error).with_retained_progress(progress))?;progress.released_bytes=step.released_allocation_bytes;}
        Ok(if self.bytes.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }

    /// 📏️ The original retained frontier refuses authority rather than enlarging any currency.
    pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{
        if self.closed{return Ok(Default::default());}
        if let Some(output)=self.contiguous.as_ref(){return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Vec<u8>>(),release_bytes:output.capacity(),depth:1,..Default::default()});}
        self.paged_demands()
    }
}

impl ErasedSnapshotRetirement for ArtifactPreparedOperationOutput {
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.retirement_demands()?.copy_bytes)}
    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.retirement_demands()?.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.retirement_demands()?.depth)}
    fn terminal_is_empty(&self)->bool{self.closed&&self.contiguous.is_none()&&self.bytes.terminal_is_empty()}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.closed{return Ok(RetainedCloneStep::Complete(Default::default()));}
        let demand=self.retirement_demands()?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
        self.closing=true;
        if self.contiguous.is_some(){drop(self.contiguous.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}));}
        let step=self.close_paged_one(grant)?;
        if self.bytes.terminal_is_empty(){self.closed=true;}
        Ok(if self.closed{RetainedCloneStep::Complete(step.progress())}else{step})
    }
}

impl Drop for ArtifactPreparedOperationOutput {
    fn drop(&mut self){assert!(self.bytes.terminal_is_empty()&&self.contiguous.is_none(),"operation output original backings require paid handoff or terminal closure");unsafe{ManuallyDrop::drop(&mut self.bytes);ManuallyDrop::drop(&mut self.contiguous)};}
}
