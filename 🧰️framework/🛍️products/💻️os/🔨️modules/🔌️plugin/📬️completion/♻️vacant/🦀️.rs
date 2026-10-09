//! 📦️ Vacant opaque collection backing remains original until a granted physical release.
use semio_framework_value::{retirement::{RetireOwned,RetirementCursor,RetirementStep},retained_clone::RetainedCloneGrant};
use std::{mem::ManuallyDrop,collections::VecDeque};
pub struct VacantVec<T:Send+'static>(pub Vec<T>);
pub struct VacantDeque<T:Send+'static>(pub VecDeque<T>);
impl<T:Send+'static> VacantVec<T>{pub fn original_birth(value:&Vec<T>)->Option<usize>{value.is_empty().then_some(std::mem::size_of::<Cursor<Self>>())}}
impl<T:Send+'static> VacantDeque<T>{pub fn original_birth(value:&VecDeque<T>)->Option<usize>{value.is_empty().then_some(std::mem::size_of::<Cursor<Self>>())}}
trait Vacant:Send+'static{fn empty(&self)->bool;fn bytes(&self)->Option<usize>;}
impl<T:Send+'static> Vacant for VacantVec<T>{fn empty(&self)->bool{self.0.is_empty()}fn bytes(&self)->Option<usize>{self.0.capacity().checked_mul(std::mem::size_of::<T>())}}
impl<T:Send+'static> Vacant for VacantDeque<T>{fn empty(&self)->bool{self.0.is_empty()}fn bytes(&self)->Option<usize>{self.0.capacity().checked_mul(std::mem::size_of::<T>())}}
struct Cursor<V:Vacant>{original:ManuallyDrop<Option<V>>}
impl<V:Vacant> RetirementCursor for Cursor<V>{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{let Some(original)=self.original.as_ref()else{return RetirementStep::Complete;};if !original.empty(){return RetirementStep::Failure(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"opaque collection still retains original live payloads"));}let Some(bytes)=original.bytes()else{return RetirementStep::BudgetExhausted;};if grant.maximum_items==0||grant.maximum_release_bytes<bytes{return RetirementStep::BudgetExhausted;}drop(self.original.take());RetirementStep::Bytes(bytes)}
 fn terminal_is_empty(&self)->bool{self.original.is_none()}
 fn next_close_byte_demand(&self)->Option<usize>{self.original.as_ref().map_or(Some(0),|value|if value.empty(){value.bytes()}else{None})}
 fn next_birth_bytes(&self,_copy:usize)->Option<usize>{Some(0)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl<V:Vacant> Drop for Cursor<V>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"vacant opaque backing reached Drop before actual release");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.original);}}}}
macro_rules! vacant{($type:ident)=>{impl<T:Send+'static> RetireOwned for $type<T>{fn retirement(self)->Box<dyn RetirementCursor>{Box::new(Cursor{original:ManuallyDrop::new(Some(self))})}fn retirement_birth_bytes(&self)->Option<usize>{self.empty().then_some(std::mem::size_of::<Cursor<Self>>())}fn controlled_retirement_supported()->bool{true}}};}
vacant!(VacantVec);vacant!(VacantDeque);

