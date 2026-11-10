//! 🔗️ Ordered aliases use the one actual issued shared owner.
//! 🔗️ Immutable first-party aliases with admitted allocation identity and explicit final-header release.
pub use crate::retirement::allocation_return::original::SharedOriginalStorage as SharedOwner;
/// 📤️ Cold explicit release returns payload custody separately from the allocation receipt.
/// 📤️ Moves the exact final payload separately from the physical header receipt.
pub struct SharedRelease<T>{pub value:Option<T>,pub progress:crate::RetainedCloneProgress}
pub(super) fn release<T>(owner:&mut SharedOwner<T>,grant:crate::RetainedCloneGrant)->SharedRelease<T>{
 if owner.terminal_is_empty()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_release_bytes<SharedOwner::<T>::allocation_bytes(){return SharedRelease{value:None,progress:Default::default()}}
 let mut elected=owner.elect_cold();let value=elected.as_mut().and_then(|original|{while !original.physical_is_unique(){std::hint::spin_loop()}original.take_value()});let released_bytes=if value.is_some(){SharedOwner::<T>::allocation_bytes()}else{0};SharedRelease{value,progress:crate::RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()}}
}
