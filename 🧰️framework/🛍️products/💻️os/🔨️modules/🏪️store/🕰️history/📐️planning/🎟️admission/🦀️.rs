//! 🎟️ History cursor frames admit their actual allocation before original ownership moves.
use super::{ArtifactHistoryReadRetirement,ArtifactOwnedValueRetirementFactory,ErasedSnapshotRetirement,ValueError,admit_artifact_retirement};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_close};
use std::{mem::ManuallyDrop,sync::Arc};

/// 🧰️ Retains a paid empty history frame with its exact installed issuers before original field transfer.
pub struct PreparedHistoryReadRetirement<P,Mu:super::Mutation<P>> {owner:ManuallyDrop<Option<Box<ArtifactHistoryReadRetirement<P,Mu>>>>,closing:bool,registry:Option<usize>}
impl<P,Mu> PreparedHistoryReadRetirement<P,Mu> where P:Send+Sync+'static,Mu:super::Mutation<P>+Send+'static {
    pub(super) fn bind_registry(&mut self,identity:usize){self.registry=Some(identity)}
    /// 🪪️ Borrows original derivation authority before any prepared publication takes fields.
    pub fn accepts_derived_snapshot(&self,original:&super::ArtifactDerivedSnapshot<P>)->bool{!self.closing&&self.registry==Some(original.registry.identity())}
    /// 🪪️ Borrows original preview authority before any prepared publication takes fields.
    pub fn accepts_preview(&self,original:&super::ArtifactDerivedHistoryPreview<P,Mu>)->bool{!self.closing&&self.registry==Some(original.registry.identity())}
    /// 🪪️ Borrows original replay authority before any prepared publication takes fields.
    pub fn accepts_replay(&self,original:&super::ArtifactDerivedReplay<P,Mu>)->bool{!self.closing&&self.registry==Some(original.registry.identity())}
    fn install<T>(&mut self,original:&mut Option<T>,grant:RetainedCloneGrant,install:fn(&mut ArtifactHistoryReadRetirement<P,Mu>,T))->Result<Option<RetainedCloneProgress>,ValueError>{
        if original.is_none()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
        if self.closing{return Err(refusal("prepared history frame already entered cancellation"))}
        let owner=self.owner.as_mut().ok_or_else(||refusal("prepared history retirement frame is absent"))?;
        if owner.derived.is_some()||owner.derived_alias.is_some()||owner.preview.is_some()||owner.replay.is_some()||owner.finished.is_some(){return Err(refusal("prepared history frame already retains its original"))}
        install(owner,original.take().unwrap());Ok(Some(RetainedCloneProgress{copied_items:1,..Default::default()}))
    }
    /// 🖼️ Installs the same original preview without allocating or closing any owner.
    pub fn install_preview(&mut self,original:&mut Option<super::ArtifactDerivedHistoryPreview<P,Mu>>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
        if grant.maximum_items>0&&grant.maximum_depth>0&&original.as_ref().is_some_and(|owner|self.registry.is_some_and(|registry|registry!=owner.registry.identity())){return Err(refusal("foreign original preview cannot enter prepared Store retirement"))}
        self.install(original,grant,|owner,original|*owner.preview=Some(original))
    }
    /// ⏪️ Installs the same original replay without allocating or closing any owner.
    pub fn install_replay(&mut self,original:&mut Option<super::ArtifactDerivedReplay<P,Mu>>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
        if grant.maximum_items>0&&grant.maximum_depth>0&&original.as_ref().is_some_and(|owner|self.registry.is_some_and(|registry|registry!=owner.registry.identity())){return Err(refusal("foreign original replay cannot enter prepared Store retirement"))}
        self.install(original,grant,|owner,original|*owner.replay=Some(original))
    }
    /// 📋️ Installs the same original completed review without allocating or closing any owner.
    pub fn install_derived_snapshot(&mut self,original:&mut Option<super::ArtifactDerivedSnapshot<P>>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
        if grant.maximum_items>0&&grant.maximum_depth>0&&original.as_ref().is_some_and(|owner|self.registry.is_some_and(|registry|registry!=owner.registry.identity())){return Err(refusal("foreign original derived snapshot cannot enter prepared Store retirement"))}
        self.install(original,grant,|owner,original|*owner.derived=Some(original))
    }
    /// 📋️ Installs the same original completed review without allocating or closing any owner.
    pub fn install_finished(&mut self,original:&mut Option<super::EditReplayResult<P,Mu>>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{self.install(original,grant,|owner,original|*owner.finished=Some(original))}
    /// 🎟️ Hands the original allocated frame to already admitted retirement custody in its own turn.
    pub fn take_retirement(&mut self,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>{
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
        Ok(self.owner.take().map(|owner|(owner as Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress{copied_items:1,..Default::default()})))
    }
}
impl<P,Mu> ErasedSnapshotRetirement for PreparedHistoryReadRetirement<P,Mu> where P:Send+Sync+'static,Mu:super::Mutation<P>+Send+'static {
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.owner.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()))}
        if grant.maximum_items==0||grant.maximum_copy_bytes<self.next_copy_byte_demand()?||grant.maximum_capacity_bytes<self.next_capacity_byte_demand(grant.maximum_copy_bytes)?||grant.maximum_release_bytes<self.next_release_byte_demand()?||grant.maximum_depth<self.next_depth_demand()?{return Ok(RetainedCloneStep::Progress(Default::default()))}
        self.closing=true;
        let owner=self.owner.as_mut().unwrap();
        if owner.terminal_is_empty(){let released_bytes=std::mem::size_of::<ArtifactHistoryReadRetirement<P,Mu>>();drop(self.owner.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()}))}
        let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=owner.close_step(child)?;let step=admit_retained_clone_close(child,step,owner.terminal_is_empty(),"prepared original history frame")?;Ok(RetainedCloneStep::Progress(step.progress()))
    }
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.owner.as_ref().map_or(Ok(0),|owner|if owner.terminal_is_empty(){Ok(0)}else{owner.next_copy_byte_demand()})}
    fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.owner.as_ref().map_or(Ok(0),|owner|if owner.terminal_is_empty(){Ok(0)}else{owner.next_capacity_byte_demand(body)})}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.owner.as_ref().map_or(Ok(0),|owner|if owner.terminal_is_empty(){Ok(std::mem::size_of::<ArtifactHistoryReadRetirement<P,Mu>>())}else{owner.next_release_byte_demand()})}
    fn next_depth_demand(&self)->Result<usize,ValueError>{self.owner.as_ref().map_or(Ok(0),|owner|if owner.terminal_is_empty(){Ok(1)}else{owner.next_depth_demand()?.checked_add(1).ok_or_else(||refusal("prepared history depth overflow"))})}
    fn terminal_is_empty(&self)->bool{self.owner.is_none()}
}
impl<P,Mu:super::Mutation<P>> Drop for PreparedHistoryReadRetirement<P,Mu>{fn drop(&mut self){assert!(std::thread::panicking()||self.owner.is_none(),"prepared history frame abandoned its installed issuers");if self.owner.is_none(){unsafe{ManuallyDrop::drop(&mut self.owner)}}}}
fn refusal(message:&'static str)->ValueError{ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,message)}

pub(super) fn prepare_history_read_retirement<P,Mu>(snapshots:&Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,mutations:&Arc<dyn ArtifactOwnedValueRetirementFactory<Mu>>,grant:RetainedCloneGrant)->Result<Option<(PreparedHistoryReadRetirement<P,Mu>,RetainedCloneProgress)>,ValueError>
where P:Send+Sync+'static,Mu:super::Mutation<P>+Send+'static {
    let layout=std::alloc::Layout::new::<ArtifactHistoryReadRetirement<P,Mu>>();
    if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<layout.size(){return Ok(None)}
    let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<ArtifactHistoryReadRetirement<P,Mu>>())else{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"prepared history frame allocation failed"))};
    let owner=ArtifactHistoryReadRetirement{derived:ManuallyDrop::new(None),derived_alias:ManuallyDrop::new(None),preview:ManuallyDrop::new(None),replay:ManuallyDrop::new(None),finished:ManuallyDrop::new(None),loaded_replay:ManuallyDrop::new(None),active:ManuallyDrop::new(None),plan_retirement:ManuallyDrop::new(None),draft_retirement:ManuallyDrop::new(None),registry_retirement:ManuallyDrop::new(None),snapshots:ManuallyDrop::new(Some(Arc::clone(snapshots))),mutations:ManuallyDrop::new(Some(Arc::clone(mutations))),factory_retirement:std::array::from_fn(|_|None)};
    let owner=unsafe{pointer.as_ptr().write(owner);Box::from_raw(pointer.as_ptr())};
    Ok(Some((PreparedHistoryReadRetirement{owner:ManuallyDrop::new(Some(owner)),closing:false,registry:None},RetainedCloneProgress{copied_items:1,retained_capacity_bytes:layout.size(),..Default::default()})))
}

pub(super) fn history_read_retirement_birth_bytes<P,Mu:super::Mutation<P>>()->usize {std::mem::size_of::<ArtifactHistoryReadRetirement<P,Mu>>()}

pub(super) fn admit_history_read_retirement<P,Mu,T>(original:&mut Option<T>,snapshots:&Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,mutations:&Arc<dyn ArtifactOwnedValueRetirementFactory<Mu>>,grant:RetainedCloneGrant,install:fn(&mut ArtifactHistoryReadRetirement<P,Mu>,T))->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>
where P:Send+Sync+'static,Mu:super::Mutation<P>+Send+'static {
    if original.is_none(){return Ok(None);}
    admit_artifact_retirement((),grant,|()|{
        let mut owner=ArtifactHistoryReadRetirement {derived:ManuallyDrop::new(None),derived_alias:ManuallyDrop::new(None),preview:ManuallyDrop::new(None),replay:ManuallyDrop::new(None),finished:ManuallyDrop::new(None),loaded_replay:ManuallyDrop::new(None),active:ManuallyDrop::new(None),plan_retirement:ManuallyDrop::new(None),draft_retirement:ManuallyDrop::new(None),registry_retirement:ManuallyDrop::new(None),snapshots:ManuallyDrop::new(Some(Arc::clone(snapshots))),mutations:ManuallyDrop::new(Some(Arc::clone(mutations))),factory_retirement:std::array::from_fn(|_|None)};
        install(&mut owner,original.take().expect("observed original history cursor"));owner
    }).map(Some).map_err(|(error,())|error)
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
