//! 🔒️ An allocation-free exclusive slot for shared custody cells whose guarded fields must retire without an unreceipted lock allocation.
use std::{cell::UnsafeCell,ops::{Deref,DerefMut},sync::{LockResult,TryLockError,TryLockResult,atomic::{AtomicBool,Ordering}}};

/// 🔐️ Guards one value with an inline flag, because the host mutex may box a lazily created OS lock that no retirement receipt names; see [`std::sync::Mutex`].
pub struct Slot<T>{held:AtomicBool,value:UnsafeCell<T>}
unsafe impl<T:Send> Send for Slot<T>{}
unsafe impl<T:Send> Sync for Slot<T>{}
impl<T> Slot<T>{
    pub const fn new(value:T)->Self{Self{held:AtomicBool::new(false),value:UnsafeCell::new(value)}}
    pub fn try_lock(&self)->TryLockResult<SlotGuard<'_,T>>{if self.held.compare_exchange(false,true,Ordering::Acquire,Ordering::Relaxed).is_ok(){Ok(SlotGuard{slot:self})}else{Err(TryLockError::WouldBlock)}}
    pub fn lock(&self)->LockResult<SlotGuard<'_,T>>{loop{if let Ok(guard)=self.try_lock(){return Ok(guard)}std::thread::yield_now()}}
    pub fn into_inner(self)->T{self.value.into_inner()}
}
pub struct SlotGuard<'a,T>{slot:&'a Slot<T>}
impl<T> Deref for SlotGuard<'_,T>{type Target=T;fn deref(&self)->&T{unsafe{&*self.slot.value.get()}}}
impl<T> DerefMut for SlotGuard<'_,T>{fn deref_mut(&mut self)->&mut T{unsafe{&mut *self.slot.value.get()}}}
impl<T> Drop for SlotGuard<'_,T>{fn drop(&mut self){self.slot.held.store(false,Ordering::Release)}}
