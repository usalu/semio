//! 🔐️ Optional fixed telemetry borrows its original inline backing without native lock allocation.
use std::{cell::UnsafeCell,ops::{Deref,DerefMut},sync::atomic::{AtomicBool,Ordering}};

pub(super) struct StaticTryGate<T>{held:AtomicBool,original:UnsafeCell<T>}
unsafe impl<T:Send> Sync for StaticTryGate<T>{}
impl<T> StaticTryGate<T>{
 pub(super) const fn new(original:T)->Self{Self{held:AtomicBool::new(false),original:UnsafeCell::new(original)}}
 pub(super) fn try_lock(&self)->Option<StaticTryGuard<'_,T>>{self.held.compare_exchange(false,true,Ordering::Acquire,Ordering::Relaxed).ok().map(|_|StaticTryGuard{gate:self,borrow:std::marker::PhantomData})}
}
pub(super) struct StaticTryGuard<'a,T>{gate:&'a StaticTryGate<T>,borrow:std::marker::PhantomData<&'a mut T>}
impl<T>Deref for StaticTryGuard<'_,T>{type Target=T;fn deref(&self)->&T{unsafe{&*self.gate.original.get()}}}
impl<T>DerefMut for StaticTryGuard<'_,T>{fn deref_mut(&mut self)->&mut T{unsafe{&mut*self.gate.original.get()}}}
impl<T>Drop for StaticTryGuard<'_,T>{fn drop(&mut self){self.gate.held.store(false,Ordering::Release)}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
