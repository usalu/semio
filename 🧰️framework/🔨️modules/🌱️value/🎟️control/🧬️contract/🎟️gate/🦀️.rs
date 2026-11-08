//! 🚦️ One non-allocating retained-owner gate shared by every layer that must not box a lock lazily.
/// 🚦️ Keeps retained-owner exclusion inline and refuses contention without parking or allocating.
pub struct RetainedOwnerGate<T> { occupied:std::sync::atomic::AtomicBool,value:std::cell::UnsafeCell<T> }
unsafe impl<T:Send> Send for RetainedOwnerGate<T> {}
unsafe impl<T:Send> Sync for RetainedOwnerGate<T> {}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct RetainedOwnerGateBusy;
pub struct RetainedOwnerGateGuard<'a,T> {gate:&'a RetainedOwnerGate<T>,exclusive:std::marker::PhantomData<&'a mut T>}
impl<T> RetainedOwnerGate<T> {
    pub const fn new(value:T)->Self {Self{occupied:std::sync::atomic::AtomicBool::new(false),value:std::cell::UnsafeCell::new(value)}}
    pub fn try_lock(&self)->Result<RetainedOwnerGateGuard<'_,T>,RetainedOwnerGateBusy> {
        self.occupied.compare_exchange(false,true,std::sync::atomic::Ordering::Acquire,std::sync::atomic::Ordering::Relaxed).map(|_|RetainedOwnerGateGuard{gate:self,exclusive:std::marker::PhantomData}).map_err(|_|RetainedOwnerGateBusy)
    }
    pub fn get_mut(&mut self)->&mut T {self.value.get_mut()}
    pub fn into_inner(self)->T {self.value.into_inner()}
}
impl<T> std::ops::Deref for RetainedOwnerGateGuard<'_,T> {type Target=T;fn deref(&self)->&T {unsafe{&*self.gate.value.get()}}}
impl<T> std::ops::DerefMut for RetainedOwnerGateGuard<'_,T> {fn deref_mut(&mut self)->&mut T {unsafe{&mut*self.gate.value.get()}}}
impl<T> Drop for RetainedOwnerGateGuard<'_,T> {fn drop(&mut self){self.gate.occupied.store(false,std::sync::atomic::Ordering::Release)}}
