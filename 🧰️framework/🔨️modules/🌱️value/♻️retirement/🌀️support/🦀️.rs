//! 🌀️ Synchronous conjunctive type queries close cycles without allocating or caching capabilities.
use std::{any::TypeId,cell::Cell,ptr};

struct Frame { owner: TypeId, previous: *const Frame }
thread_local! { static ACTIVE: Cell<*const Frame> = const { Cell::new(ptr::null()) }; }
struct Restore<'a> { head: &'a Cell<*const Frame>, previous: *const Frame }
impl Drop for Restore<'_> { fn drop(&mut self) { self.head.set(self.previous); } }

/// 🧬️ Evaluates a pure conjunction once per active type path; recursive edges inherit its pending truth.
///
/// # Safety
/// Every installed pointer names an initialized immutable stack-local frame in this thread. Its
/// restore guard drops before the frame on both return and unwind. Traversal completes before
/// invoking the synchronous callback, retaining no frame borrow across reentrant queries.
pub fn conjunctive_retirement_support<T: 'static>(check: impl FnOnce() -> bool) -> bool {
    ACTIVE.with(|head| {
        let owner = TypeId::of::<T>();
        let mut current = head.get();
        while !current.is_null() {
            let frame = unsafe { &*current };
            if frame.owner == owner { return true; }
            current = frame.previous;
        }
        let frame = Frame { owner, previous: head.get() };
        let restore = Restore { head, previous: frame.previous };
        head.set(ptr::from_ref(&frame));
        let result = check();
        drop(restore);
        result
    })
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
