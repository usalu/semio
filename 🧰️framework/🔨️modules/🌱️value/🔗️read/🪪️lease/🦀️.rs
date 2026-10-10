//! 🪪️ Erased read custody preserves the original typed lease and its independently funded frame.
use super::{ReadLease,ReadLeaseId};
use crate::{ErasedSnapshotRetirement,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use std::{any::Any,mem::ManuallyDrop};

trait LeaseView:ErasedSnapshotRetirement {
    fn as_any(&self)->&dyn Any;
    fn root(&self)->Option<&dyn Any>;
    fn id(&self)->ReadLeaseId;
    fn matches(&self,generation:u64,revision:[u8;32])->bool;
}
impl<T:RetireOwned+Sync> LeaseView for ReadLease<T> {
    fn as_any(&self)->&dyn Any {self}
    fn root(&self)->Option<&dyn Any> {self.owner.as_deref().map(|owner|owner as &dyn Any)}
    fn id(&self)->ReadLeaseId {self.id()}
    fn matches(&self,generation:u64,revision:[u8;32])->bool {self.commit_authority_matches(generation,revision)}
}

/// 🪪️ One admitted concrete lease frame retains its original root, registry and closing cursor.
pub struct ErasedReadLease {lease:ManuallyDrop<Option<Box<dyn LeaseView>>>}
impl ErasedReadLease {
    pub fn terminal_is_empty(&self)->bool {self.lease.is_none()}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {ErasedSnapshotRetirement::close_step(self,grant)}
    pub fn next_depth_demand(&self)->Result<usize,ValueError> {Ok(self.demands(0)?.depth)}
    pub const fn constructor_capacity_bytes<T:RetireOwned+Sync>()->usize {size_of::<ReadLease<T>>()}
    pub fn admit<T:RetireOwned+Sync>(lease:ReadLease<T>,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,ReadLease<T>)> {
        if grant.maximum_items==0{return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"erased read admission requires an admitted item"),lease));}
        if grant.maximum_depth==0{return Err((ValueError::literal(ValueRefusalKind::DepthLimit,"erased read admission requires admitted depth"),lease));}
        let bytes=Self::constructor_capacity_bytes::<T>();
        if grant.maximum_capacity_bytes<bytes{return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"erased read frame exceeds admitted capacity"),lease));}
        Ok((Self {lease:ManuallyDrop::new(Some(Box::new(lease)))},RetainedCloneProgress {copied_items:1,retained_capacity_bytes:bytes,..Default::default()}))
    }
    pub fn get<T:'static>(&self)->Option<&T> {self.lease.as_ref()?.root()?.downcast_ref()}
    pub fn id(&self)->Option<ReadLeaseId> {self.lease.as_ref().map(|lease|lease.id())}
    pub fn commit_authority_matches(&self,generation:u64,revision:[u8;32])->bool {self.lease.as_ref().is_some_and(|lease|lease.matches(generation,revision))}
    fn typed<T:RetireOwned+Sync>(&self)->Result<&ReadLease<T>,ValueError> {self.lease.as_ref().and_then(|lease|lease.as_any().downcast_ref()).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"erased read has a different concrete root type or is closed"))}
    pub fn source_capacity_bytes<T:RetireOwned+Sync>(&self)->Result<usize,ValueError> {self.typed::<T>()?;Ok(RetainedCloneSource::<T>::borrowed_constructor_capacity_bytes::<super::OriginalErasedReadSource<T>>())}
    pub fn source_copy_bytes<T:RetireOwned+Sync>(&self)->usize{RetainedCloneSource::<T>::borrowed_constructor_copy_bytes::<super::OriginalErasedReadSource<T>>()}
    pub fn admit_source<T:RetireOwned+Sync>(self,grant:RetainedCloneGrant)->Result<(RetainedCloneSource<T,super::OriginalErasedReadSource<T>>,RetainedCloneProgress),(ValueError,Self)>{
        if self.get::<T>().is_none(){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"original erased read source has a different type or is closed"),self));}
        RetainedCloneSource::admit_borrowed(super::OriginalErasedReadSource::new(self),|read:&super::OriginalErasedReadSource<T>|read.get(),grant).map_err(|(error,mut read)|(error,read.take_refused()))
    }
    pub fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError> {
        let Some(lease)=self.lease.as_ref()else{return Ok(Default::default());};
        if lease.terminal_is_empty(){return Ok(RetirementDemand {release_bytes:size_of_val(lease.as_ref()),depth:1,..Default::default()});}
        Ok(RetirementDemand {copy_bytes:lease.next_copy_byte_demand()?,capacity_bytes:lease.next_capacity_byte_demand(copy)?,release_bytes:lease.next_release_byte_demand()?,depth:lease.next_depth_demand()?})
    }
}
impl ErasedSnapshotRetirement for ErasedReadLease {
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let Some(lease)=self.lease.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()));};
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if lease.terminal_is_empty(){
            if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"erased read frame release requires admitted depth"));}
            let bytes=size_of_val(lease.as_ref());
            if grant.maximum_release_bytes<bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
            drop(self.lease.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:bytes,..Default::default()}));
        }
        let step=lease.close_step(grant)?;
        let step=crate::retained_clone::admit_retained_clone_close(grant,step,lease.terminal_is_empty(),"erased read lease")?;
        Ok(RetainedCloneStep::Progress(step.progress()))
    }
    fn terminal_is_empty(&self)->bool {self.lease.is_none()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {Ok(self.demands(0)?.copy_bytes)}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {Ok(self.demands(copy)?.capacity_bytes)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError> {Ok(self.demands(0)?.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError> {Ok(self.demands(0)?.depth)}
    fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError> {self.demands(body)}
}
impl RetirementCursor for ErasedReadLease {
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep {match ErasedSnapshotRetirement::close_step(self,grant){Ok(RetainedCloneStep::Complete(p))if p==RetainedCloneProgress::default()=>RetirementStep::Complete,Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}}
    fn terminal_is_empty(&self)->bool {ErasedSnapshotRetirement::terminal_is_empty(self)}
    fn next_work_byte_demand(&self)->Result<usize,ValueError> {self.next_copy_byte_demand()}
    fn next_birth_bytes(&self,copy:usize)->Option<usize> {self.next_capacity_byte_demand(copy).ok()}
    fn next_close_byte_demand(&self)->Option<usize> {self.next_release_byte_demand().ok()}
    fn next_depth_demand(&self)->Result<usize,ValueError> {ErasedSnapshotRetirement::next_depth_demand(self)}
    fn allows_admitted_narrow_work(&self)->bool {true}
    fn terminal_release_bytes(&self)->Option<usize> {ErasedSnapshotRetirement::terminal_is_empty(self).then_some(size_of::<Self>())}
}
impl RetireOwned for ErasedReadLease {
    fn retirement(self)->Box<dyn RetirementCursor> {Box::new(self)}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<Self>())}
    fn controlled_retirement_supported()->bool {true}
}
impl Drop for ErasedReadLease {fn drop(&mut self){assert!(std::thread::panicking()||self.lease.is_none(),"erased read must close its original lease and funded frame");if self.lease.is_none(){unsafe{ManuallyDrop::drop(&mut self.lease);}}}}
