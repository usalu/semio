//! 🫙️ Original typed native prefix remains in its admitted caller-returned workspace.
use std::mem::ManuallyDrop;
use semio_framework_value::{ErasedSnapshotRetirement,ValueError,ValueRefusalKind};
use semio_framework_value::native_decoding::NativeDecodeControl;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::RetirementDemand;
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress};

pub(crate)struct Workspace<T:RetireOwned>{original:ManuallyDrop<Option<T>>,active:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>}
impl<T:RetireOwned>Workspace<T>{
fn new(original:T)->Self{Self{original:ManuallyDrop::new(Some(original)),active:ManuallyDrop::new(None)}}
fn get_mut(&mut self)->&mut T{self.original.as_mut().expect("live native prefix")}
fn take(&mut self)->T{self.original.take().expect("complete native prefix")}
fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
if let Some(active)=self.active.as_ref(){return store::artifact_retirement_box_demands(active,body)}
store::artifact_retirement_owned_birth_demands(&self.original)
}
}
impl<T:RetireOwned>ErasedSnapshotRetirement for Workspace<T>{
fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))}
if self.active.is_some(){return store::artifact_retirement_box_close_step(&mut self.active,grant)}
store::artifact_retirement_admit_owned(&mut self.original,&mut self.active,grant)
}
fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.active.is_none()}
fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.demands(body)?.capacity_bytes)}
fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}
fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
}
impl<T:RetireOwned>Drop for Workspace<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native prefix dropped before caller retirement");}}

/// 🛬️ Reserves the exact typed workspace before initializing any field and transfers its original body on every result.
pub(crate)fn decode<T:RetireOwned>(control:&mut NativeDecodeControl<'_>,empty:impl FnOnce()->T,read:impl FnOnce(&mut T,&mut NativeDecodeControl<'_>)->Result<(),ValueError>)->Result<T,ValueError>{
if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native prefix has no typed controlled retirement"))}
control.with_retirement_owner(std::mem::size_of::<Workspace<T>>(),|control|{
let mut owner=Box::new(Workspace::new(empty()));
let result=read(owner.get_mut(),control).and_then(|()|{control.checkpoint()?;Ok(owner.take())});
(result,Some(owner as Box<dyn ErasedSnapshotRetirement>))
})
}

/// 🧾️ Transfers a completed typed result while retaining its original auxiliary owners for caller retirement.
pub(crate)fn decode_projected<T:RetireOwned,R>(control:&mut NativeDecodeControl<'_>,empty:impl FnOnce()->T,read:impl FnOnce(&mut T,&mut NativeDecodeControl<'_>)->Result<(),ValueError>,take:impl FnOnce(&mut T)->R)->Result<R,ValueError>{
if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native prefix has no typed controlled retirement"))}
control.with_retirement_owner(std::mem::size_of::<Workspace<T>>(),|control|{
let mut owner=Box::new(Workspace::new(empty()));
let result=read(owner.get_mut(),control).and_then(|()|{control.checkpoint()?;Ok(take(owner.get_mut()))});
(result,Some(owner as Box<dyn ErasedSnapshotRetirement>))
})
}

/// 🏛️ Admits the original relational reconstruction workspace and returns every unfinished field to its caller.
pub(crate)fn reconstruct<T:RetireOwned>(control:&mut store::sqlite_snapshot::SqliteSnapshotControl<'_>,empty:impl FnOnce()->T,read:impl FnOnce(&mut T,&mut store::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<(),ValueError>)->Result<T,ValueError>{
if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"SQLite prefix has no typed controlled retirement"))}
control.with_retirement_owner(store::sqlite_snapshot::SqliteSnapshotPhase::ReconstructSnapshot,std::mem::size_of::<Workspace<T>>(),|control|{
let mut owner=Box::new(Workspace::new(empty()));
let result=read(owner.get_mut(),control).and_then(|()|{control.checkpoint(store::sqlite_snapshot::SqliteSnapshotPhase::ReconstructSnapshot,0,0)?;Ok(owner.take())});
(result,Some(owner as Box<dyn ErasedSnapshotRetirement>))
})
}

/// 🗂️ Moves completed relational output while preserving the original typed index and ordering scratch owners.
pub(crate)fn reconstruct_projected<T:RetireOwned,R>(control:&mut store::sqlite_snapshot::SqliteSnapshotControl<'_>,empty:impl FnOnce()->T,read:impl FnOnce(&mut T,&mut store::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<(),ValueError>,take:impl FnOnce(&mut T)->R)->Result<R,ValueError>{
if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"SQLite prefix has no typed controlled retirement"))}
control.with_retirement_owner(store::sqlite_snapshot::SqliteSnapshotPhase::ReconstructSnapshot,std::mem::size_of::<Workspace<T>>(),|control|{
let mut owner=Box::new(Workspace::new(empty()));
let result=read(owner.get_mut(),control).and_then(|()|{control.checkpoint(store::sqlite_snapshot::SqliteSnapshotPhase::ReconstructSnapshot,0,0)?;Ok(take(owner.get_mut()))});
(result,Some(owner as Box<dyn ErasedSnapshotRetirement>))
})
}
                                                      