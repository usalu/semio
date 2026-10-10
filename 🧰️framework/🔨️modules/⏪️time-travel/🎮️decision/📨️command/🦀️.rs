//! 📨️ Original command custody separates a paid inline slot from its reserved native semantic frame.
use crate::{TimeTravelDiscardCursor,TimeTravelEvent};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retirement::{controlled::ControlledRetirement,queue::RetirementQueue},retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

/// 🎟️ Retains received native event fields before semantic construction or cancellation.
pub struct TimeTravelCommandCustody {
    original:Option<TimeTravelEvent>,
    retirement:Option<ControlledRetirement<TimeTravelEvent>>,
    done:bool,
}
impl TimeTravelCommandCustody {
    pub fn admission_copy_bytes()->usize{0}
    pub fn admit_original(original:&mut Option<TimeTravelEvent>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<Self::admission_copy_bytes(){return Ok(None)}
        if original.is_none(){return Err(refusal("pending command original is absent"))}
        Ok(Some((Self{original:original.take(),retirement:None,done:false},RetainedCloneProgress{copied_bytes:Self::admission_copy_bytes(),..item()})))
    }
    pub fn original(&self)->Option<&TimeTravelEvent>{self.original.as_ref()}
    /// 📏️ Names actual frame capacity only after the destination owns its separately paid slot.
    pub fn discard_admission_demand(&self,destination:&RetirementQueue)->Result<RetirementDemand,ValueError>{
        if !matches!(self.original,Some(TimeTravelEvent::Discard{..})){return Err(refusal("pending command has no Discard semantic authority"))}
        if !destination.has_reserved_slot(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"pending semantic frame requires an original reserved retirement slot"))}
        Ok(RetirementDemand{copy_bytes:TimeTravelDiscardCursor::admission_copy_bytes(),capacity_bytes:std::mem::size_of::<TimeTravelDiscardCursor>(),depth:2,..Default::default()})
    }
    /// 📦️ Transfers the same scalar event only after actual semantic-frame allocation is admitted.
    pub fn admit_discard(&mut self,destination:&RetirementQueue,grant:RetainedCloneGrant)->Result<Option<(Box<TimeTravelDiscardCursor>,RetainedCloneProgress)>,ValueError>{
        if grant.maximum_items==0{return Ok(None)}
        let demand=self.discard_admission_demand(destination)?;
        if grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes{return Ok(None)}
        let layout=std::alloc::Layout::new::<TimeTravelDiscardCursor>();
        let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<TimeTravelDiscardCursor>()) else{return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"pending semantic frame allocation failed"))};
        let parent_copy=0usize;let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,maximum_depth:grant.maximum_depth-1,..grant};
        let(owner,mut receipt)=TimeTravelDiscardCursor::admit_original(&mut self.original,child)?.expect("preflight retains the supported original command");
        assert!(receipt.fits(child));receipt.copied_bytes+=parent_copy;receipt.retained_capacity_bytes=layout.size();
        let owner=unsafe{pointer.as_ptr().write(owner);Box::from_raw(pointer.as_ptr())};
        Ok(Some((owner,receipt)))
    }
    pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.done{return Ok(Default::default())}
        if let Some(owner)=self.retirement.as_ref().filter(|owner|!owner.terminal_is_empty()){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"pending event retirement depth overflow"))?})}
        let copy_bytes=0;
        Ok(RetirementDemand{copy_bytes,depth:1,..Default::default()})
    }
    /// ♻️ Cancellation retains every original owning event field through the declared native authority.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.done{return Ok(RetainedCloneStep::Complete(Default::default()))}
        let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}
        if let Some(original)=self.original.take(){match ControlledRetirement::new(original){Ok(owner)=>self.retirement=Some(owner),Err((error,original))=>{self.original=Some(original);return Err(error)}}return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_bytes:demand.copy_bytes,..item()}))}
        if let Some(owner)=self.retirement.as_mut(){if !owner.terminal_is_empty(){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=owner.step(child)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original command event")?;return Ok(RetainedCloneStep::Progress(step.progress()))}self.retirement=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_bytes:demand.copy_bytes,..item()}))}
        self.done=true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_bytes:demand.copy_bytes,..item()}))
    }
    pub fn terminal_is_empty(&self)->bool{self.done&&self.original.is_none()&&self.retirement.is_none()}
}
impl Drop for TimeTravelCommandCustody{fn drop(&mut self){assert!(self.terminal_is_empty(),"pending command abandoned its original fields")}}
fn item()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
fn refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,message)}
