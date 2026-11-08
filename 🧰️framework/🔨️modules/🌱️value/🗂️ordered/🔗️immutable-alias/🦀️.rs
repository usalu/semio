//! 🔗️ Immutable first-party aliases with admitted allocation identity and explicit final-header release.

use crate::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress}};
use std::{mem::ManuallyDrop, ops::Deref, sync::Arc};

/// 📥️ Owns a shared allocation whose aliases cannot create Weak or raw ownership.
#[must_use = "shared ownership requires transfer or explicit granted release"]
pub struct SharedOwner<T> { value: ManuallyDrop<Option<Arc<T>>> }

/// 📤️ Moves the exact final payload separately from the physical header receipt.
pub struct SharedRelease<T> { pub value: Option<T>, pub progress: RetainedCloneProgress }

impl<T> SharedOwner<T> {
    /// 🧮️ Measures the complete system Arc allocation, including alignment padding.
    pub fn allocation_bytes() -> usize { crate::retirement::arc_custody::shared_allocation_bytes::<T>() }

    /// 🛂️ Preserves an existing allocation only after exclusive strong and weak custody are proven together.
    pub fn admit_arc(mut value: Arc<T>) -> Result<Self, (ValueError, Arc<T>)> {
        if Arc::get_mut(&mut value).is_none() { return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"shared input retains foreign strong or weak custody"),value)); }
        Ok(Self { value: ManuallyDrop::new(Some(value)) })
    }

    /// 🎟️ Admits one complete allocation before moving the original inline payload into it.
    pub fn admit(value:T,grant:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome<(Self,RetainedCloneProgress),(ValueError,T)>{let turn=crate::retirement::arc_custody::admit_shared_allocation(value,grant,actual);crate::RetainedWorkOutcome{result:turn.result.map(|(value,progress)|(Self{value:ManuallyDrop::new(Some(value))},progress)),work:turn.work}}

    /// 📐️ Quotes the same original input and empty output before its shared allocation.
    pub fn admission_demand(input:&Option<T>,output:&Option<Self>)->Result<crate::RetirementDemand,ValueError>{if input.is_none()||output.is_some(){return Err(ValueError::invariant("shared admission requires its original occupied input and empty output"))}let copy_bytes=std::mem::size_of::<Option<T>>().checked_add(std::mem::size_of::<T>()).and_then(|n|n.checked_add(2*std::mem::size_of::<usize>())).and_then(|n|n.checked_add(std::mem::size_of::<Option<Self>>())).ok_or_else(||ValueError::invariant("shared admission header extent overflow"))?;Ok(crate::RetirementDemand{copy_bytes,capacity_bytes:Self::allocation_bytes(),depth:1,..Default::default()})}
    /// ⏱️ Quotes the actual one-step shared allocation producer.
    pub fn admission_work(input:&Option<T>,output:&Option<Self>)->Result<u64,ValueError>{Self::admission_demand(input,output).map(|_|1)}
    /// 📥️ Publishes the actual shared owner into its original slot after the real allocator succeeds.
    pub fn admit_into(input:&mut Option<T>,output:&mut Option<Self>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<RetainedCloneProgress>>{
        let demand=match Self::admission_demand(input,output){Ok(demand)=>demand,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};
        let grant=effect.grant();if !crate::retained_clone::retained_clone_grant_admits(grant,demand)||effect.maximum_work_units()==0{return crate::RetainedWorkOutcome{result:Ok(None),work:0}}
        if effect.actual().is_some(){return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("shared admission requires the original empty actual marker")),work:0}}
        let performed=match effect.performed(1){Ok(performed)=>performed,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:1}};if !performed{return crate::RetainedWorkOutcome{result:Ok(None),work:1}}
        let turn=crate::retirement::arc_custody::try_uninitialized_arc::<T>(grant,effect.actual());let mut storage=match turn.result{Ok((storage,_))=>storage,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:1}};
        *effect.actual()=Some(Self::allocation_bytes());let slot=Arc::get_mut(&mut storage).expect("new shared storage is uniquely owned");slot.write(input.take().expect("checked original shared input remains resident"));*output=Some(Self{value:ManuallyDrop::new(Some(unsafe{storage.assume_init()}))});
        crate::RetainedWorkOutcome{result:Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:Self::allocation_bytes(),..Default::default()})),work:1}
    }
    pub fn source_admission_demand()->Result<crate::RetirementDemand,ValueError>{
     let mut d=crate::retirement::arc_custody::shared_uninitialized_demand::<T>()?;
     d.copy_bytes=d.copy_bytes.checked_add(std::mem::size_of::<Option<T>>()).and_then(|n|n.checked_add(std::mem::size_of::<T>())).and_then(|n|n.checked_add(std::mem::size_of::<Option<Self>>())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"ordered shared original copy extent overflow"))?;Ok(d)
    }
    pub const fn source_admission_work_demand()->u64{4}
    pub fn admit_from(input:&mut Option<T>,output:&mut Option<Self>,grant:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome{
     use crate::retained_clone::RetainedCloneStep;
     if output.is_some()||actual.is_some(){return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("ordered shared original recipient or allocation marker occupied")),work:0}}
     if input.is_none(){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
     let d=match Self::source_admission_demand(){Ok(d)=>d,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};if !semio_framework_value_control_contract::retained_clone_grant_admits(grant,d){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
     let turn=crate::retirement::arc_custody::try_uninitialized_arc::<T>(grant,actual);let mut storage=match turn.result{Ok((storage,_))=>storage,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:turn.work}};
     let value=input.take().expect("ordered shared original validated before allocation");Arc::get_mut(&mut storage).expect("fresh shared allocation has one owner").write(value);*output=Some(Self{value:ManuallyDrop::new(Some(unsafe{storage.assume_init()}))});
     crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})),work:turn.work+3}
    }

    /// 🧊️ Constructs one shared allocation at an explicitly synchronous boundary.
    pub fn from_cold(value: T) -> Self { Self {value:ManuallyDrop::new(Some(Arc::new(value)))} }

    pub fn as_ptr(&self) -> *const T { Arc::as_ptr(self.value.as_ref().expect("shared ownership is present")) }
    pub fn terminal_is_empty(&self) -> bool { self.value.is_none() }
    pub fn next_release_byte_demand(&self) -> usize { if self.value.is_some() {Self::allocation_bytes()} else {0} }

    /// ♻️ Admits the possible complete header before atomically releasing one immutable alias.
    pub fn release_step(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<SharedRelease<T>>{
        let empty=||SharedRelease{value:None,progress:Default::default()};
        if self.value.is_none()||grant.maximum_items==0{return crate::RetainedWorkOutcome{result:Ok(empty()),work:0}}
        if grant.maximum_depth==0{return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::DepthLimit,"shared release requires admitted depth")),work:0}}
        let bytes=Self::allocation_bytes();if bytes>grant.maximum_release_bytes{return crate::RetainedWorkOutcome{result:Ok(empty()),work:0}}
        let value=Arc::into_inner(match self.value.take(){Some(v)=>v,None=>return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("value absent").into()),work:0}});let released_bytes=if value.is_some(){bytes}else{0};crate::RetainedWorkOutcome{result:Ok(SharedRelease{value,progress:RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()}}),work:1}
    }

}

impl SharedOwner<()>{
 pub fn unit_admission_demand()->Result<crate::RetirementDemand,ValueError>{let mut d=crate::retirement::arc_custody::shared_uninitialized_demand::<()>()?;d.copy_bytes=d.copy_bytes.checked_add(std::mem::size_of::<Option<Self>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"shared unit recipient extent overflow"))?;Ok(d)}
 pub const fn unit_admission_work_demand()->u64{3}
 pub fn admit_unit(output:&mut Option<Self>,grant:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome{
  use crate::retained_clone::RetainedCloneStep;
  if output.is_some()||actual.is_some(){return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("shared unit recipient or allocation marker occupied")),work:0}}
  let d=match Self::unit_admission_demand(){Ok(d)=>d,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};if !semio_framework_value_control_contract::retained_clone_grant_admits(grant,d){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  let turn=crate::retirement::arc_custody::try_uninitialized_arc::<()>(grant,actual);let mut storage=match turn.result{Ok((storage,_))=>storage,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:turn.work}};Arc::get_mut(&mut storage).expect("fresh shared unit has one owner").write(());*output=Some(Self{value:ManuallyDrop::new(Some(unsafe{storage.assume_init()}))});
  crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})),work:turn.work+2}
 }
}

impl<T> Clone for SharedOwner<T> {
    fn clone(&self) -> Self { Self {value:ManuallyDrop::new(Some(Arc::clone(self.value.as_ref().expect("shared clone requires live ownership"))))} }
}
impl<T> Deref for SharedOwner<T> {type Target=T;fn deref(&self)->&T {self.value.as_ref().expect("shared borrow requires live ownership").as_ref()} }
impl<T:std::fmt::Debug> std::fmt::Debug for SharedOwner<T> {fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {self.deref().fmt(f)} }
impl<T> Drop for SharedOwner<T> {
    fn drop(&mut self) {
        if self.value.is_some() {assert!(std::thread::panicking(),"shared owner must complete granted release before drop");return;}
        unsafe {ManuallyDrop::drop(&mut self.value);}
    }
}

#[cfg(test)]#[path="🎟️work/🧪️tests/🦀️.rs"]mod work_tests;
