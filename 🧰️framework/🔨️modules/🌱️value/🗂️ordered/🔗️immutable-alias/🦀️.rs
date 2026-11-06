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
