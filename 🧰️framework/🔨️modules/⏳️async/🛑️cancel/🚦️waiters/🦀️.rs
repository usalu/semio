//! 🚦️ Original waiter backing has an inline atomic guard and a single bounded retirement admission.
use std::{cell::UnsafeCell,marker::PhantomData,ops::{Deref,DerefMut},sync::atomic::{AtomicBool,Ordering},task::Waker};
pub(super) struct CancelWaiters{gate:AtomicBool,entries:UnsafeCell<Vec<(u64,Waker)>>}
unsafe impl Sync for CancelWaiters{}
impl CancelWaiters{
    pub(super) fn new()->Self{Self{gate:AtomicBool::new(false),entries:UnsafeCell::new(Vec::new())}}
    pub(super) fn try_lock(&self)->Option<CancelWaitersGuard<'_>>{self.gate.compare_exchange(false,true,Ordering::Acquire,Ordering::Relaxed).ok().map(|_|CancelWaitersGuard{owner:self,local:PhantomData})}
    pub(super) fn lock(&self)->CancelWaitersGuard<'_>{loop{if let Some(guard)=self.try_lock(){return guard;}std::hint::spin_loop();}}
    pub(super) fn get_mut(&mut self)->&mut Vec<(u64,Waker)>{self.entries.get_mut()}
}
pub(super) struct CancelWaitersGuard<'a>{owner:&'a CancelWaiters,local:PhantomData<*mut()>}
impl Deref for CancelWaitersGuard<'_>{type Target=Vec<(u64,Waker)>;fn deref(&self)->&Self::Target{unsafe{&*self.owner.entries.get()}}}
impl DerefMut for CancelWaitersGuard<'_>{fn deref_mut(&mut self)->&mut Self::Target{unsafe{&mut*self.owner.entries.get()}}}
impl Drop for CancelWaitersGuard<'_>{fn drop(&mut self){self.owner.gate.store(false,Ordering::Release);}}
