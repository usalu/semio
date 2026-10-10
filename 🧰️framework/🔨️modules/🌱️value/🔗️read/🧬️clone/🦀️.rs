//! 🧬️ Captured original reads project immutable roots and retain unsupported registry bodies.
use super::{ReadLease,ErasedReadLease};
use crate::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use std::mem::ManuallyDrop;

pub struct OriginalReadSource<T:RetireOwned+Sync>{read:ManuallyDrop<Option<ReadLease<T>>>}
/// 🔐️ The private capsule exposes immutable root borrowing and consuming funded transfer only.
unsafe impl<T:RetireOwned+Sync> Sync for OriginalReadSource<T>{}
impl<T:RetireOwned+Sync> OriginalReadSource<T>{
 pub(super) fn new(read:ReadLease<T>)->Self{Self{read:ManuallyDrop::new(Some(read))}}
 pub fn get(&self)->&T{self.read.as_ref().expect("original read already transferred").get()}
 pub(super) fn take_refused(&mut self)->ReadLease<T>{self.read.take().expect("refused original read remains owned")}
 pub fn into_read(mut self,grant:RetainedCloneGrant)->Result<(ReadLease<T>,RetainedCloneProgress),(ValueError,Self)>{if grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<ReadLease<T>>()||grant.maximum_depth==0{return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"original read handoff is not funded"),self));}let read=self.read.take().expect("original read already transferred");Ok((read,RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<ReadLease<T>>(),..Default::default()}))}
}
impl<T:RetireOwned+Sync> Drop for OriginalReadSource<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.read.is_none(),"original standalone read still requires its actual registry issuer");if self.read.is_none(){unsafe{ManuallyDrop::drop(&mut self.read);}}}}

pub struct OriginalErasedReadSource<T:RetireOwned+Sync>{read:ManuallyDrop<Option<ErasedReadLease>>,marker:std::marker::PhantomData<T>}
/// 🧷️ The original erased frame is private and never exposes mutable or raw registry custody.
unsafe impl<T:RetireOwned+Sync> Sync for OriginalErasedReadSource<T>{}
impl<T:RetireOwned+Sync> OriginalErasedReadSource<T>{
 pub(super) fn new(read:ErasedReadLease)->Self{Self{read:ManuallyDrop::new(Some(read)),marker:std::marker::PhantomData}}
 pub fn get(&self)->&T{self.read.as_ref().and_then(|read|read.get::<T>()).expect("original erased source type changed")}
 pub(super) fn take_refused(&mut self)->ErasedReadLease{self.read.take().expect("refused original erased read remains owned")}
 pub fn into_read(mut self,grant:RetainedCloneGrant)->Result<(ErasedReadLease,RetainedCloneProgress),(ValueError,Self)>{if grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<ErasedReadLease>()||grant.maximum_depth==0{return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"original erased read handoff is not funded"),self));}let read=self.read.take().expect("original erased read already transferred");Ok((read,RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<ErasedReadLease>(),..Default::default()}))}
}
impl<T:RetireOwned+Sync> Drop for OriginalErasedReadSource<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.read.is_none(),"original erased standalone read still requires its actual registry issuer");if self.read.is_none(){unsafe{ManuallyDrop::drop(&mut self.read);}}}}

struct Cursor<S:Send+'static>{original:ManuallyDrop<S>}
fn unsupported()->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,"standalone read registry graph still exposes raw Arc/Weak authority; original read remains retained")}
impl<S:Send+'static> RetirementCursor for Cursor<S>{
 fn close_step(&mut self,_grant:RetainedCloneGrant)->RetirementStep{RetirementStep::Failure(unsupported())}
 fn terminal_is_empty(&self)->bool{false}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Err(unsupported())}
 fn next_birth_bytes(&self,_body:usize)->Option<usize>{None}
 fn next_close_byte_demand(&self)->Option<usize>{None}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Err(unsupported())}
 fn terminal_release_bytes(&self)->Option<usize>{None}
}
impl<S:Send+'static> Drop for Cursor<S>{fn drop(&mut self){assert!(std::thread::panicking(),"original standalone read cursor remains retained until its issuer is available");}}
impl<T:RetireOwned+Sync> RetireOwned for OriginalReadSource<T>{fn retirement(self)->Box<dyn RetirementCursor>{Box::new(Cursor{original:ManuallyDrop::new(self)})}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Cursor<Self>>())}fn controlled_retirement_supported()->bool{true}}
impl<T:RetireOwned+Sync> RetireOwned for OriginalErasedReadSource<T>{fn retirement(self)->Box<dyn RetirementCursor>{Box::new(Cursor{original:ManuallyDrop::new(self)})}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Cursor<Self>>())}fn controlled_retirement_supported()->bool{true}}
