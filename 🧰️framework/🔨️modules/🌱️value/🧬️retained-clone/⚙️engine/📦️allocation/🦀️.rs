//! 📦️ Actual native clone backing acquisition shared by field and boxed payload owners.
use semio_framework_value_control_contract::{RetainedAllocationEffect,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,MaybeUninit,size_of};
/// 🧱️ Acquires only raw uninitialized storage under the original allocator marker.
pub fn allocate_clone_storage<T>(effect:&mut RetainedAllocationEffect<'_>)->Result<Box<std::mem::MaybeUninit<T>>,ValueError>{
 let layout=std::alloc::Layout::new::<T>();if layout.size()==0{return Ok(Box::new(std::mem::MaybeUninit::uninit()))}if effect.actual().is_some(){return Err(ValueError::invariant("original allocation attempt is occupied"))}*effect.actual()=Some(0);let pointer=unsafe{std::alloc::alloc(layout)};if !pointer.is_null(){*effect.actual()=Some(layout.size())}if pointer.is_null(){return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"retained clone storage allocation failed"))}Ok(unsafe{Box::from_raw(pointer.cast::<std::mem::MaybeUninit<T>>())})
}

/// 🧺️ Separates the paid reservation and initialization of one native cursor's backing.
pub struct CloneBacking<C>(ManuallyDrop<Option<Box<MaybeUninit<C>>>>);

impl<C> Default for CloneBacking<C>{
    fn default()->Self{Self(ManuallyDrop::new(None))}
}

impl<C> CloneBacking<C>{
    pub fn is_reserved(&self)->bool{self.0.is_some()}

    pub fn birth_demand(&self)->RetirementDemand{
        let pointer=size_of::<Option<Box<C>>>();
        if self.is_reserved(){
            RetirementDemand{copy_bytes:size_of::<C>()+2*pointer,depth:1,..Default::default()}
        }else{
            RetirementDemand{copy_bytes:pointer,capacity_bytes:size_of::<C>(),depth:1,..Default::default()}
        }
    }

    pub fn birth(&mut self,output:&mut Option<Box<C>>,seed:impl FnOnce()->C,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
        if output.is_some(){return Err(ValueError::invariant("retained cursor birth output occupied"))}
        let demand=self.birth_demand();
        if !admit(demand,effect)?{return Ok(RetainedCloneStep::Progress(Default::default()))}
        if let Some(storage)=self.0.as_mut(){
            storage.write(seed());
            let storage=self.0.take().ok_or_else(||ValueError::invariant("retained cursor backing absent"))?;
            *output=Some(unsafe{storage.assume_init()});
        }else{
            *self.0=Some(allocate_clone_storage(effect)?);
        }
        Ok(progress(demand))
    }

    pub fn release_demand(&self)->RetirementDemand{
        if self.is_reserved(){
            RetirementDemand{copy_bytes:size_of::<Option<Box<C>>>(),release_bytes:size_of::<C>(),depth:1,..Default::default()}
        }else{Default::default()}
    }

    pub fn release(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
        if !self.is_reserved(){return Ok(RetainedCloneStep::Complete(Default::default()))}
        let demand=self.release_demand();
        if !admit(demand,effect)?{return Ok(RetainedCloneStep::Progress(Default::default()))}
        drop(self.0.take());
        Ok(progress(demand))
    }
}

impl<C> Drop for CloneBacking<C>{
    fn drop(&mut self){
        assert!(std::thread::panicking()||!self.is_reserved(),"retained cursor backing reached Drop before paid release");
    }
}

fn admit(demand:RetirementDemand,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,ValueError>{
    if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?==0{return Ok(false)}
    if effect.performed(1)?{Ok(true)}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"retained cursor backing exceeded original Work"))}
}

fn progress(demand:RetirementDemand)->RetainedCloneStep{
    RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:demand.release_bytes})
}
