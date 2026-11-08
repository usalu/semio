//! 📦️ Actual native clone backing acquisition shared by field and boxed payload owners.
use semio_framework_value_control_contract::{RetainedAllocationEffect,ValueError,ValueRefusalKind};
/// 🧱️ Acquires only raw uninitialized storage under the original allocator marker.
pub fn allocate_clone_storage<T>(effect:&mut RetainedAllocationEffect<'_>)->Result<Box<std::mem::MaybeUninit<T>>,ValueError>{
 let layout=std::alloc::Layout::new::<T>();if layout.size()==0{return Ok(Box::new(std::mem::MaybeUninit::uninit()))}if effect.actual().is_some(){return Err(ValueError::invariant("original allocation attempt is occupied"))}*effect.actual()=Some(0);let pointer=unsafe{std::alloc::alloc(layout)};if !pointer.is_null(){*effect.actual()=Some(layout.size())}if pointer.is_null(){return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"retained clone storage allocation failed"))}Ok(unsafe{Box::from_raw(pointer.cast::<std::mem::MaybeUninit<T>>())})
}
