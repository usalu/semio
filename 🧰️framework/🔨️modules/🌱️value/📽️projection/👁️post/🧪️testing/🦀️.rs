//! 🧪️ Native central tests lend the actual post and restore it with the same original funded custody owner.
use semio_framework_value::{RetirementDemand,retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::{RetainedCloneRef,RetainedCloneGrant},snapshot_projection::post::{OriginalPostRead,PagedOneItemPostMode}};
use std::mem::ManuallyDrop;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
pub struct PostFixture<S:RetireOwned+Sync>{held_post:ManuallyDrop<Option<S>>,original_post_read:OriginalPostRead<S>}
impl<S:RetireOwned+Sync> PostFixture<S>{
 pub fn projected<O:semio_framework_value::snapshot_projection::ProjectionOutput<S>>(output:O)->Self{let d=O::placement_demand();let mut target=std::mem::MaybeUninit::<S>::uninit();let p=output.place(&mut target,grant(d)).unwrap_or_else(|_|panic!("test projected output original placement"));assert!(p.fits(grant(d)));Self::new(unsafe{target.assume_init()})}
 pub fn new(value:S)->Self{let mut owner=Self{held_post:ManuallyDrop::new(Some(value)),original_post_read:Default::default()};owner.prepare(PagedOneItemPostMode::Original);owner}
 fn prepare(&mut self,mode:PagedOneItemPostMode){for _ in 0..100000{let Some(d)=self.original_post_read.demand(&self.held_post,mode,4096).unwrap()else{return;};let p=self.original_post_read.prepare(&mut self.held_post,mode,4096,grant(d)).unwrap().expect("selected original post frontier");assert!(p.fits(grant(d)));}panic!("test post original handback stalled");}
 pub fn turn_demand(&self,mode:PagedOneItemPostMode,body:usize)->Result<Option<RetirementDemand>,semio_framework_value::ValueError>{self.original_post_read.demand(&self.held_post,mode,body)}
 pub fn read_mode(&self,mode:PagedOneItemPostMode)->Result<semio_framework_value::snapshot_projection::post::PagedOneItemPostRead<'_,S>,semio_framework_value::ValueError>{self.original_post_read.read(&self.held_post,mode)}
 pub fn prepare_turn(&mut self,mode:PagedOneItemPostMode,body:usize,grant:RetainedCloneGrant)->Result<Option<semio_framework_value::retained_clone::RetainedCloneProgress>,semio_framework_value::ValueError>{self.original_post_read.prepare(&mut self.held_post,mode,body,grant)}
 pub fn access(&mut self,mode:PagedOneItemPostMode)->Result<semio_framework_value::snapshot_projection::post::PagedOneItemPost<'_,S>,semio_framework_value::ValueError>{self.original_post_read.access(&mut self.held_post,mode)}
 pub fn borrow(&self)->RetainedCloneRef<'_,S>{self.original_post_read.read(&self.held_post,PagedOneItemPostMode::Original).unwrap().original().unwrap()}
 pub fn mutable_read(&mut self)->&S{self.prepare(PagedOneItemPostMode::Mutable);self.held_post.as_ref().unwrap()}
 pub fn into_inner(mut self)->S{self.prepare(PagedOneItemPostMode::Mutable);self.held_post.take().unwrap()}
}
impl<S:RetireOwned+Sync>std::ops::Deref for PostFixture<S>{type Target=S;fn deref(&self)->&S{if self.original_post_read.terminal_is_empty(){self.held_post.as_ref().unwrap()}else{self.borrow().get()}}}
impl<S:RetireOwned+Sync>std::ops::DerefMut for PostFixture<S>{fn deref_mut(&mut self)->&mut S{assert!(self.original_post_read.terminal_is_empty(),"mutable test post requires explicit funded handback before observed commit");self.held_post.as_mut().unwrap()}}
impl<S:RetireOwned+Sync+std::fmt::Debug>std::fmt::Debug for PostFixture<S>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{std::fmt::Debug::fmt(&**self,f)}}
impl<S:RetireOwned+Sync+PartialEq>PartialEq<S>for PostFixture<S>{fn eq(&self,other:&S)->bool{&**self==other}}
impl<S:RetireOwned+Sync>Drop for PostFixture<S>{fn drop(&mut self){if std::thread::panicking(){return;}if self.held_post.is_none()&&self.original_post_read.terminal_is_empty(){return;}self.prepare(PagedOneItemPostMode::Mutable);if let Some(value)=self.held_post.take(){let mut close=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("test post original typed retirement"));for _ in 0..100000{if close.terminal_is_empty(){return;}let d=close.next_demand(4096).unwrap();let p=close.step(grant(d)).unwrap().progress();assert!(p.fits(grant(d)));}panic!("test post original retirement stalled");}}}
