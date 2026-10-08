//! 🧬️ Bounded native-owner cloning with independent payload and capacity credits.

use crate::{ErasedSnapshotRetirement, retirement::RetireOwned};
use std::{
    mem::{size_of, size_of_val, ManuallyDrop},
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

#[path = "🗺️ordered-map/🦀️.rs"]
pub mod ordered_map;
#[path = "📦️paged/🦀️.rs"]
pub mod paged;
#[path = "📋️paged-list/🦀️.rs"]
pub mod paged_list;
#[path = "📋️field/🦀️.rs"]
mod field;
pub use field::RetainedFieldCursor;
#[path = "🔗️projection/🦀️.rs"]
mod owned_projection;
pub use owned_projection::RetainedOwnedProjection;

#[cfg(test)]
#[path = "🔗️source/🧪️tests/🔬️unit/🦀️.rs"]
mod source_custody_tests;

static RETAINED_CLONE_SOURCE_IDS: AtomicU64 = AtomicU64::new(1);

#[path = "🔗️source/🦀️.rs"]
mod source_custody;
pub use source_custody::RetainedCloneSource;
#[cfg(test)]
pub(crate) use source_custody::FixtureSource;
use source_custody::RetainedCloneLeaseOwner;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RetainedCloneProjection {
    parent: usize,
    address: usize,
    discriminator: usize,
}

pub struct RetainedCloneRef<'a, T: ?Sized> {
    value: &'a T,
    lease: &'a Arc<RetainedCloneLeaseOwner>,
    projection: RetainedCloneProjection,
}

impl<'a, T: ?Sized> Copy for RetainedCloneRef<'a, T> {}

impl<'a, T: ?Sized> Clone for RetainedCloneRef<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: ?Sized> RetainedCloneRef<'a, T> {
    pub fn get(self) -> &'a T {
        self.value
    }

    pub fn project<U: ?Sized, F>(self, discriminator: usize, project: F) -> RetainedCloneRef<'a, U>
    where
        F: for<'source> FnOnce(&'source T) -> &'source U,
    {
        let value = project(self.value);
        RetainedCloneRef { value, lease: self.lease, projection: RetainedCloneProjection { parent: self.projection.address, address: value as *const U as *const () as usize, discriminator } }
    }

    pub fn bind(self, binding: &mut Option<RetainedCloneBinding>) -> Result<(), crate::ValueError> {
        if let Some(expected) = binding {
            if expected.lease.as_ref().unwrap().id != self.lease.id || expected.projection != self.projection {
                return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained clone source lease or projected path changed"));
            }
        } else {
            *binding = Some(RetainedCloneBinding::new(Arc::clone(self.lease),self.projection));
        }
        Ok(())
    }
}

pub struct RetainedCloneBinding {
    lease:ManuallyDrop<Option<Arc<RetainedCloneLeaseOwner>>>,
    close:ManuallyDrop<Option<crate::retirement::shared::SharedControlledRetirement<RetainedCloneLeaseOwner>>>,
    projection:RetainedCloneProjection,
}
impl RetainedCloneBinding {
    fn new(lease:Arc<RetainedCloneLeaseOwner>,projection:RetainedCloneProjection)->Self{Self{lease:ManuallyDrop::new(Some(lease)),close:ManuallyDrop::new(None),projection}}
    pub fn close_one(binding:&mut Option<Self>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>{
        let Some(owner)=binding.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()));};
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if grant.maximum_depth<owner.close.as_ref().map_or(Ok(1),|close|close.next_depth_demand())?{return Err(crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"binding close requires admitted depth"));}
        if owner.lease.is_some()&&grant.maximum_release_bytes<crate::retirement::shared::arc_bytes::<RetainedCloneLeaseOwner>(){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if owner.close.is_none(){if let Some(lease)=owner.lease.take(){*owner.close=Some(crate::retirement::shared::SharedControlledRetirement::lease(lease));}}
        let close=owner.close.as_mut().unwrap();let step=close.step(grant)?;
        if close.terminal_is_empty(){*owner.close=None;*binding=None;}
        Ok(step)
    }
    pub fn copy_demand(binding:&Option<Self>)->Result<usize,crate::ValueError>{binding.as_ref().and_then(|owner|owner.close.as_ref()).map_or(Ok(0),|owner|owner.next_copy_byte_demand())}
    pub fn capacity_demand(binding:&Option<Self>,body:usize)->Result<usize,crate::ValueError>{binding.as_ref().and_then(|owner|owner.close.as_ref()).map_or(Ok(0),|owner|owner.next_capacity_byte_demand(body))}
    pub fn release_demand(binding:&Option<Self>)->Result<usize,crate::ValueError>{binding.as_ref().map_or(Ok(0),|owner|owner.close.as_ref().map_or(Ok(crate::retirement::shared::arc_bytes::<RetainedCloneLeaseOwner>()),|owner|owner.next_release_byte_demand()))}
    pub fn depth_demand(binding:&Option<Self>)->Result<usize,crate::ValueError>{binding.as_ref().map_or(Ok(0),|owner|owner.close.as_ref().map_or(Ok(1),|owner|owner.next_depth_demand()))}
    fn terminal_is_empty(&self)->bool{self.lease.is_none()&&self.close.is_none()}
}
impl Drop for RetainedCloneBinding{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"binding must finish original source custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.lease);ManuallyDrop::drop(&mut self.close);}}}}

pub struct RetainedCloneBorrowAuthority{source:RetainedCloneSource<()>}
impl RetainedCloneBorrowAuthority{
    pub fn constructor_capacity_bytes<A:RetireOwned>()->usize{crate::retirement::shared::arc_bytes::<()>()+RetainedCloneSource::<()>::constructor_capacity_bytes::<A>()}
    pub fn admit<A:RetireOwned>(authority:A,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(crate::ValueError,A)>{
        let bytes=Self::constructor_capacity_bytes::<A>();
        if !A::controlled_retirement_supported(){return Err((crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"borrowed authority has no controlled retirement"),authority));}
        if grant.maximum_items==0||grant.maximum_depth==0||bytes>grant.maximum_capacity_bytes{return Err((crate::ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"borrowed authority requires full admitted constructor"),authority));}
        let root=Arc::new(());let root_bytes=crate::retirement::shared::arc_bytes::<()>();
        let(source,mut receipt)=RetainedCloneSource::admit(root,authority,RetainedCloneGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes-root_bytes,..grant}).unwrap_or_else(|(error,_,_)|panic!("preadmitted borrowed source refused: {error}"));
        receipt.retained_capacity_bytes+=root_bytes;Ok((Self{source},receipt))
    }
    pub fn borrow<'source,T:?Sized>(&'source self,value:&'source T)->RetainedCloneRef<'source,T>{let root=self.source.borrow();RetainedCloneRef{value,lease:root.lease,projection:RetainedCloneProjection{parent:value as*const T as*const()as usize,address:value as*const T as*const()as usize,discriminator:0}}}
    pub fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{self.source.next_close_copy_byte_demand()}
    pub fn next_close_capacity_byte_demand(&self,work:usize)->Result<usize,crate::ValueError>{self.source.next_close_capacity_byte_demand(work)}
    pub fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{self.source.next_close_release_byte_demand()}
    pub fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{self.source.next_close_depth_demand()}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>{self.source.close_step(grant)}
    pub fn terminal_is_empty(&self)->bool{self.source.terminal_is_empty()}
}
#[cfg(test)]
pub(crate) struct FixtureBorrowAuthority(source_custody::FixtureSource<()>);
#[cfg(test)]
impl FixtureBorrowAuthority{pub(crate) fn borrow<'source,T:?Sized>(&'source self,value:&'source T)->RetainedCloneRef<'source,T>{let root=self.0.borrow();RetainedCloneRef{value,lease:root.lease,projection:RetainedCloneProjection{parent:value as*const T as*const()as usize,address:value as*const T as*const()as usize,discriminator:0}}}}
#[cfg(test)]
impl RetainedCloneBorrowAuthority{pub(crate) fn new<A:RetireOwned>(authority:A)->FixtureBorrowAuthority{FixtureBorrowAuthority(RetainedCloneSource::fixture_from_authority(Arc::new(()),authority))}}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedCloneGrant {
    pub maximum_items: usize,
    /// 🧮️ Bounds payload copying within one clone turn.
    pub maximum_copy_bytes: usize,
    pub maximum_capacity_bytes: usize,
    /// ♻️ Bounds physical owner and scaffold release independently of copying.
    pub maximum_release_bytes: usize,
    pub maximum_depth: usize,
}

impl RetainedCloneGrant {
    /// 🎟️ Admits one structural allocation while reserving no payload-copy credit.
    pub fn one_capacity_turn(maximum_bytes: usize, maximum_depth: usize) -> Self {
        Self { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: maximum_bytes, maximum_release_bytes: 0, maximum_depth }
    }

    /// 🎟️ Admits one payload turn while reserving no allocation-capacity credit.
    pub fn one_payload_turn(maximum_bytes: usize, maximum_depth: usize) -> Self {
        Self { maximum_items: 1, maximum_copy_bytes: maximum_bytes, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth }
    }
    /// ♻️ Admits one release turn without copy or allocation credit.
    pub fn one_release_turn(maximum_bytes: usize, maximum_depth: usize) -> Self {
        Self { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: maximum_bytes, maximum_depth }
    }
}

/// 🌱️ Exact allocation-only constructor demand borrowed before original owner transfer.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct RetainedCloneBirthDemand {pub capacity_bytes:usize,pub depth:usize}
impl RetainedCloneBirthDemand {
    pub fn admit(self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,crate::ValueError>{
        if grant.maximum_items==0{return Err(crate::ValueError::literal(crate::ValueRefusalKind::WorkLimit,"constructor requires one admitted item"));}
        if grant.maximum_depth<self.depth{return Err(crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"constructor exceeds admitted depth"));}
        if grant.maximum_capacity_bytes<self.capacity_bytes{return Err(crate::ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"constructor exceeds admitted capacity"));}
        Ok(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:self.capacity_bytes,..Default::default()})
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedCloneProgress {
    pub copied_items: usize,
    pub copied_bytes: usize,
    pub retained_capacity_bytes: usize,
    pub released_bytes: usize,
}

impl RetainedCloneProgress {
    pub fn checked_add(self, other: Self) -> Result<Self, crate::ValueError> {
        Ok(Self {
            copied_items: self.copied_items.checked_add(other.copied_items).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained clone item progress overflow"))?,
            copied_bytes: self.copied_bytes.checked_add(other.copied_bytes).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained clone byte progress overflow"))?,
            released_bytes: self.released_bytes.checked_add(other.released_bytes).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained clone release progress overflow"))?,
            retained_capacity_bytes: self.retained_capacity_bytes.checked_add(other.retained_capacity_bytes).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained clone capacity progress overflow"))?,
        })
    }

    pub fn fits(self, grant: RetainedCloneGrant) -> bool {
        self.copied_items <= grant.maximum_items && self.copied_bytes <= grant.maximum_copy_bytes && self.retained_capacity_bytes <= grant.maximum_capacity_bytes && self.released_bytes <= grant.maximum_release_bytes
    }
}

pub fn admit_retained_clone_progress(grant: RetainedCloneGrant, progress: RetainedCloneProgress, scope: &str) -> Result<RetainedCloneProgress, crate::ValueError> {
    if progress.fits(grant) { Ok(progress) } else { Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, format!("{scope} exceeded its retained clone item, copy, capacity, or release grant"))) }
}

/// 🛡️ Checks both granted closure work and the child's exact terminal ownership witness.
pub fn admit_retained_clone_close(grant: RetainedCloneGrant, step: RetainedCloneStep, terminal: bool, scope: &str) -> Result<RetainedCloneStep, crate::ValueError> {
    admit_retained_clone_progress(grant, step.progress(), scope)?;
    if matches!(step, RetainedCloneStep::Complete(_)) && !terminal {
        return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, format!("{scope} completed with retained owners")));
    }
    Ok(step)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedCloneStep {
    Progress(RetainedCloneProgress),
    Complete(RetainedCloneProgress),
}

impl RetainedCloneStep {
    pub fn progress(self) -> RetainedCloneProgress {
        match self {
            Self::Progress(progress) | Self::Complete(progress) => progress,
        }
    }
}

/// 🧬️ Creates a one-shot cursor for an immutable owner retained at one stable address until completion or cancellation.
/// The cursor owner retains it through `begin_close` and capacity-bearing `close_step` turns until
/// `terminal_is_empty`; raw `close_step` is an explicit cold lifecycle, with no granted fallback.
pub trait RetainedClone: RetireOwned + Send + Sync + Sized + 'static {
    type Cursor: RetainedCloneCursor<Self>;
    fn retained_clone_cursor() -> Self::Cursor;
}

/// 🧷️ Transfers one native owner once; after `take`, callers must close the spent cursor before dropping it.
pub trait RetainedCloneCursor<T: RetainedClone>: Send {
    fn advance(&mut self, source: RetainedCloneRef<'_, T>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError>;
    fn take(&mut self) -> Option<T>;
    fn begin_close(&mut self) -> bool;
    
    fn terminal_is_empty(&self) -> bool;
    /// 🪜️ Observes the depth of the next controlled close transition without changing ownership.
    fn next_close_depth_demand(&self) -> Result<usize, crate::ValueError>;
    /// 🧮️ Observes the next payload-work grant without allocating or releasing retained owners.
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "clone cursor has no controlled close work demand"))
    }
    /// 📐️ Observes the next granted close birth; unsupported cursors retain their owners.
    fn next_close_capacity_byte_demand(&self, _maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "clone cursor has no controlled close birth demand"))
    }
    /// 📐️ Observes the next body or scaffold release independently of payload copying.
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "clone cursor has no controlled close release demand"))
    }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "clone cursor has no capacity-bearing close authority"))
    }
}

#[derive(Default)]
pub struct RetainedCloneClose {retirement:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>}
impl RetainedCloneClose {
    pub fn next_owner_depth_with_binding(&self,present:bool,binding:&Option<RetainedCloneBinding>)->Result<usize,crate::ValueError>{if !self.is_empty(){self.next_depth_demand()}else if present{Ok(1)}else{RetainedCloneBinding::depth_demand(binding)}}
    pub fn next_copy_with_binding(&self,binding:&Option<RetainedCloneBinding>)->Result<usize,crate::ValueError>{if self.is_empty(){RetainedCloneBinding::copy_demand(binding)}else{self.next_copy_byte_demand()}}
    pub fn next_release_with_binding(&self,binding:&Option<RetainedCloneBinding>)->Result<usize,crate::ValueError>{if self.is_empty(){RetainedCloneBinding::release_demand(binding)}else{self.next_release_byte_demand()}}
    pub fn next_owner_capacity_with_binding<T:RetireOwned>(&self,present:bool,body:usize,binding:&Option<RetainedCloneBinding>)->Result<usize,crate::ValueError>{if self.is_empty()&&!present{RetainedCloneBinding::capacity_demand(binding,body)}else{self.next_owner_capacity_byte_demand::<T>(present,body)}}
    pub fn next_copy_byte_demand(&self)->Result<usize,crate::ValueError> {self.retirement.as_ref().map_or(Ok(0),|owner|owner.next_copy_byte_demand())}
    pub fn next_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError> {self.retirement.as_ref().map_or(Ok(0),|owner|owner.next_capacity_byte_demand(body))}
    pub fn next_release_byte_demand(&self)->Result<usize,crate::ValueError> {self.retirement.as_ref().map_or(Ok(0),|owner|if owner.terminal_is_empty(){Ok(size_of_val(owner.as_ref()))}else{owner.next_release_byte_demand()})}
    pub fn next_depth_demand(&self)->Result<usize,crate::ValueError> {self.retirement.as_ref().map_or(Ok(0),|owner|if owner.terminal_is_empty(){Ok(1)}else{owner.next_depth_demand()})}
    pub fn next_owner_capacity_byte_demand<T:RetireOwned>(&self,present:bool,body:usize)->Result<usize,crate::ValueError> {
        if !self.is_empty(){return self.next_capacity_byte_demand(body);}
        if present&&!T::controlled_retirement_supported(){return Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"typed owner has no controlled close birth authority"));}
        Ok(if present{crate::owned_retirement_birth_bytes::<T>()}else{0})
    }
    pub fn begin_granted<T:RetireOwned>(&mut self,value:&mut Option<T>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,crate::ValueError> {
        if !self.is_empty()||value.is_none(){return Ok(None);}
        let bytes=crate::owned_retirement_birth_bytes::<T>();
        if grant.maximum_items==0||bytes>grant.maximum_capacity_bytes{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        match crate::admit_owned_retirement(value.take().unwrap(),grant) {
            Ok((owner,progress))=>{*self.retirement=Some(owner);Ok(Some(RetainedCloneStep::Progress(progress)))},
            Err((error,owner))=>{*value=Some(owner);Err(error)},
        }
    }
    pub fn begin_default_granted<T:RetireOwned+Default>(&mut self,value:&mut T,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,crate::ValueError> {
        if !self.is_empty(){return Ok(None);}
        let bytes=crate::owned_retirement_birth_bytes::<T>();
        if grant.maximum_items==0||bytes>grant.maximum_capacity_bytes{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        let mut owner=Some(std::mem::take(value));
        let step=self.begin_granted(&mut owner,grant);
        if let Some(owner)=owner{*value=owner;}
        step
    }
    pub fn step_granted(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError> {
        let step=crate::close_factory_ticket(&mut self.retirement,grant)?;
        if !step.progress().fits(grant){return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"clone retirement exceeded admitted full grant"));}
        Ok(step)
    }
    pub fn is_empty(&self)->bool {self.retirement.is_none()}
}

impl Drop for RetainedCloneClose {fn drop(&mut self){assert!(std::thread::panicking()||self.is_empty(),"clone retirement abandoned original ownership");if self.is_empty(){unsafe{ManuallyDrop::drop(&mut self.retirement);}}}}

pub fn close_retained_binding(binding: &mut Option<RetainedCloneBinding>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
    RetainedCloneBinding::close_one(binding, grant)
}

pub struct ScalarCursor<T> {
    value: Option<T>,
    source: Option<RetainedCloneBinding>,
    spent: bool,
    closing: bool,
    close: RetainedCloneClose,
}

impl<T> Default for ScalarCursor<T> {
    fn default() -> Self {
        Self { value: None, source: None, spent: false, closing: false, close: RetainedCloneClose::default() }
    }
}

impl<T> RetainedCloneCursor<T> for ScalarCursor<T>
where
    T: RetainedClone + Copy,
{
    fn advance(&mut self, source: RetainedCloneRef<'_, T>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained scalar clone cursor is closing"));
        }
        if self.spent {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained scalar clone cursor is spent"));
        }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        source.bind(&mut self.source)?;
        if self.value.is_some() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<T>(), retained_capacity_bytes: 0, released_bytes: 0 };
        if !progress.fits(grant) {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        self.value = Some(*source.get());
        Ok(RetainedCloneStep::Complete(progress))
    }

    fn take(&mut self) -> Option<T> {
        let value = self.value.take();
        if value.is_some() {
            self.spent = true;
        }
        value
    }

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "clone cursor must begin close before granted retirement")); }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.value, grant)? { return Ok(step); }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if !self.closing {return Ok(0);} self.close.next_owner_depth_with_binding(self.value.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_owner_capacity_with_binding::<T>(self.value.is_some(), maximum_release_bytes,&self.source)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_release_with_binding(&self.source)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.value.is_none() && self.close.is_empty() && self.source.is_none()
    }
}

macro_rules! retained_clone_scalar {
    ($($type:ty),+ $(,)?) => {$ (
        impl RetainedClone for $type {
            type Cursor = ScalarCursor<Self>;
            fn retained_clone_cursor() -> Self::Cursor { ScalarCursor::default() }
        }
    )+ };
}

retained_clone_scalar!((), bool, char, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

pub type ArrayCursor<T, const N: usize> = ScalarCursor<[T; N]>;

impl<T: Copy + Send + Sync + 'static, const N: usize> RetainedClone for [T; N] {
    type Cursor = ArrayCursor<T, N>;
    fn retained_clone_cursor() -> Self::Cursor {
        ScalarCursor::default()
    }
}

#[derive(Default)]
pub struct StringCursor {
    output: Option<String>,
    source: Option<RetainedCloneBinding>,
    phase: u8,
    closing: bool,
    close: RetainedCloneClose,
}

impl RetainedCloneCursor<String> for StringCursor {
    fn advance(&mut self, source: RetainedCloneRef<'_, String>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone cursor is closing"));
        }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        source.bind(&mut self.source)?;
        let source = source.get();
        match self.phase {
            0 => {
                let planned = source.len();
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned, released_bytes: 0 };
                if !progress.fits(grant) {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                let mut output = String::new();
                output.try_reserve_exact(planned).map_err(|_| crate::ValueError::new(crate::ValueRefusalKind::AllocationFailed, "retained string clone capacity allocation failed"))?;
                let actual = output.capacity();
                if actual > grant.maximum_capacity_bytes {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone allocator exceeded its admitted capacity"));
                }
                self.output = Some(output);
                self.phase = 1;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { retained_capacity_bytes: actual, ..progress }))
            }
            1 => {
                let output = self.output.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone output owner is missing"))?;
                let start = output.len();
                if !source.is_char_boundary(start) {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone source content changed across a UTF-8 boundary"));
                }
                let remaining = source.len().checked_sub(start).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone source changed"))?;
                if remaining == 0 {
                    if grant.maximum_items == 0 {
                        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                    }
                    self.phase = 2;
                    return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
                }
                let maximum_end = start + remaining.min(grant.maximum_copy_bytes);
                let mut end = maximum_end;
                while end > start && !source.is_char_boundary(end) {
                    end -= 1;
                }
                if end == start {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                output.push_str(&source[start..end]);
                let progress = RetainedCloneProgress { copied_items: 0, copied_bytes: end - start, retained_capacity_bytes: 0, released_bytes: 0 };
                Ok(RetainedCloneStep::Progress(progress))
            }
            2 => Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
            3 => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone cursor is spent")),
            _ => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone state is invalid")),
        }
    }

    fn take(&mut self) -> Option<String> {
        if self.phase != 2 {
            return None;
        }
        self.phase = 3;
        self.output.take()
    }

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "clone cursor must begin close before granted retirement")); }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if !self.closing {return Ok(0);} self.close.next_owner_depth_with_binding(self.output.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_owner_capacity_with_binding::<String>(self.output.is_some(), maximum_release_bytes,&self.source)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_release_with_binding(&self.source)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.output.is_none() && self.close.is_empty() && self.source.is_none()
    }
}

impl RetainedClone for String {
    type Cursor = StringCursor;
    fn retained_clone_cursor() -> Self::Cursor {
        StringCursor::default()
    }
}

pub struct VecCursor<T: RetainedClone> {
    values: Vec<T>,
    child: Option<T::Cursor>,
    child_value: Option<T>,
    index: usize,
    source: Option<RetainedCloneBinding>,
    phase: u8,
    output: Option<Vec<T>>,
    closing: bool,
    close: RetainedCloneClose,
}

impl<T: RetainedClone> Default for VecCursor<T> {
    fn default() -> Self {
        Self { values: Vec::new(), child: None, child_value: None, index: 0, source: None, phase: 0, output: None, closing: false, close: RetainedCloneClose::default() }
    }
}

impl<T: RetainedClone> RetainedCloneCursor<Vec<T>> for VecCursor<T> {
    fn advance(&mut self, source: RetainedCloneRef<'_, Vec<T>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained vector clone cursor is closing"));
        }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        source.bind(&mut self.source)?;
        let source_value = source.get();
        if self.phase == 0 {
            let planned = source_value.len().checked_mul(size_of::<T>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained vector clone capacity overflow"))?;
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned, released_bytes: 0 };
            if !progress.fits(grant) {
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
            }
            self.values.try_reserve_exact(source_value.len()).map_err(|_| crate::ValueError::new(crate::ValueRefusalKind::AllocationFailed, "retained vector clone capacity allocation failed"))?;
            let actual = self.values.capacity().checked_mul(size_of::<T>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained vector clone actual capacity overflow"))?;
            if actual > grant.maximum_capacity_bytes {
                return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained vector clone allocator exceeded its admitted capacity"));
            }
            self.phase = 1;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { retained_capacity_bytes: actual, ..progress }));
        }
        if self.phase == 2 {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if self.phase == 3 {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained vector clone cursor is spent"));
        }
        let mut used = RetainedCloneProgress::default();
        loop {
            let remaining = RetainedCloneGrant {
                maximum_items: grant.maximum_items.saturating_sub(used.copied_items),
                maximum_copy_bytes: grant.maximum_copy_bytes.saturating_sub(used.copied_bytes),
                maximum_capacity_bytes: grant.maximum_capacity_bytes.saturating_sub(used.retained_capacity_bytes),
                maximum_depth: grant.maximum_depth, maximum_release_bytes: grant.maximum_release_bytes.saturating_sub(used.released_bytes) };
            if let Some(child) = self.child.as_mut() {
                if self.child_value.is_some() {
                    if !child.terminal_is_empty() {
                        if remaining.maximum_items == 0 {
                            return Ok(RetainedCloneStep::Progress(used));
                        }
                        let step = child.close_step(remaining)?;
                        let progress = admit_retained_clone_close(remaining, step, child.terminal_is_empty(), "retained vector child scaffold close")?.progress();
                        if progress == RetainedCloneProgress::default() { return Ok(RetainedCloneStep::Progress(used)); }
                        used = used.checked_add(progress)?;
                        continue;
                    }
                    if remaining.maximum_items == 0 {
                        return Ok(RetainedCloneStep::Progress(used));
                    }
                    self.child = None;
                    self.values.push(self.child_value.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained vector child owner is missing"))?);
                    self.index += 1;
                    used = used.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                    if used.copied_items == grant.maximum_items {
                        return Ok(RetainedCloneStep::Progress(used));
                    }
                    continue;
                }
            }
            if self.index == source_value.len() {
                if remaining.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(used));
                }
                self.output = Some(std::mem::take(&mut self.values));
                self.phase = 2;
                used = used.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                return Ok(RetainedCloneStep::Complete(used));
            }
            if remaining.maximum_items == 0 && remaining.maximum_copy_bytes == 0 && remaining.maximum_capacity_bytes == 0 && remaining.maximum_release_bytes == 0 {
                return Ok(RetainedCloneStep::Progress(used));
            }
            let child = self.child.get_or_insert_with(T::retained_clone_cursor);
            match child.advance(source.project(self.index + 1, |values| &values[self.index]), remaining)? {
                RetainedCloneStep::Progress(progress) => {
                    let progress = admit_retained_clone_progress(remaining, progress, "retained vector child")?;
                    if progress == RetainedCloneProgress::default() {
                        return Ok(RetainedCloneStep::Progress(used));
                    }
                    used = used.checked_add(progress)?;
                }
                RetainedCloneStep::Complete(progress) => {
                    let progress = admit_retained_clone_progress(remaining, progress, "retained vector child")?;
                    used = used.checked_add(progress)?;
                    if used.copied_items == grant.maximum_items {
                        return Ok(RetainedCloneStep::Progress(used));
                    }
                    self.child_value = Some(child.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained vector child completed without an owner"))?);
                    let _ = child.begin_close();
                    used = used.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                    continue;
                }
            }
        }
    }

    fn take(&mut self) -> Option<Vec<T>> {
        if self.phase != 2 {
            return None;
        }
        self.phase = 3;
        self.output.take()
    }

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "native owner must begin close before granted retirement")); }
        if let Some(child) = self.child.as_mut() {
            if child.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            if !child.terminal_is_empty() { let step = child.close_step(grant)?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained child close")?.progress())); }
            let progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
            self.child = None;
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.child_value, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        if self.values.capacity() > 0 { if let Some(step) = self.close.begin_default_granted(&mut self.values, grant)? { return Ok(step); } }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if !self.closing {return Ok(0);} if let Some(child)=self.child.as_ref(){return child.next_close_depth_demand();} self.close.next_owner_depth_with_binding(self.child_value.is_some()||self.output.is_some()||self.values.capacity()>0,&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child) = self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_copy_byte_demand() }; }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child)=self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_capacity_byte_demand(maximum_release_bytes) }; }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if self.child_value.is_some() { return self.close.next_owner_capacity_with_binding::<T>(true,maximum_release_bytes,&self.source); }
        self.close.next_owner_capacity_with_binding::<Vec<T>>(self.output.is_some()||!self.values.is_empty()||(size_of::<T>()!=0&&self.values.capacity()!=0),maximum_release_bytes,&self.source)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child)=self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_release_byte_demand() }; }
        self.close.next_release_with_binding(&self.source)
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.values.is_empty() && (size_of::<T>() == 0 || self.values.capacity() == 0) && self.child.is_none() && self.child_value.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none() }
}

impl<T: RetainedClone> RetainedClone for Vec<T> {
    type Cursor = VecCursor<T>;
    fn retained_clone_cursor() -> Self::Cursor {
        VecCursor::default()
    }
}

pub struct OptionCursor<T: RetainedClone> {
    child: T::Cursor,
    child_value: Option<T>,
    output: Option<Option<T>>,
    source: Option<RetainedCloneBinding>,
    spent: bool,
    closing: bool,
    close: RetainedCloneClose,
}

impl<T: RetainedClone> Default for OptionCursor<T> {
    fn default() -> Self {
        Self { child: T::retained_clone_cursor(), child_value: None, output: None, source: None, spent: false, closing: false, close: RetainedCloneClose::default() }
    }
}

impl<T: RetainedClone> RetainedCloneCursor<Option<T>> for OptionCursor<T> {
    fn advance(&mut self, source: RetainedCloneRef<'_, Option<T>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained optional clone cursor is closing"));
        }
        if self.spent {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained optional clone cursor is spent"));
        }
        if self.output.is_some() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        let first = self.source.is_none();
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        source.bind(&mut self.source)?;
        let source_value = source.get();
        if first {
            if grant.maximum_items == 0 {
                self.source = None;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
            }
            if source_value.is_none() {
                self.output = Some(None);
                return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
            }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        match source_value {
            None => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained optional clone source shape changed")),
            Some(_) => {
                if self.child_value.is_some() {
                    if !self.child.terminal_is_empty() {
                        if grant.maximum_items == 0 {
                            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                        }
                        let step = self.child.close_step(grant)?;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.child.terminal_is_empty(), "retained optional child scaffold close")?.progress()));
                    }
                    if grant.maximum_items == 0 {
                        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                    }
                    self.output = Some(Some(self.child_value.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained optional child owner is missing"))?));
                    return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
                }
                match self.child.advance(source.project(1, |value| value.as_ref().expect("validated retained optional child")), grant)? {
                    RetainedCloneStep::Progress(progress) => Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant, progress, "retained optional child")?)),
                    RetainedCloneStep::Complete(progress) => {
                        let progress = admit_retained_clone_progress(grant, progress, "retained optional child")?;
                        if progress.copied_items >= grant.maximum_items {
                            return Ok(RetainedCloneStep::Progress(progress));
                        }
                        self.child_value = Some(self.child.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained optional child completed without an owner"))?);
                        let _ = self.child.begin_close();
                        Ok(RetainedCloneStep::Progress(progress.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?))
                    }
                }
            }
        }
    }

    fn take(&mut self) -> Option<Option<T>> {
        let output = self.output.take();
        if output.is_some() {
            self.spent = true;
        }
        output
    }

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "native owner must begin close before granted retirement")); }
        if !self.child.terminal_is_empty() {
            if self.child.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            let step = self.child.close_step(grant)?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.child.terminal_is_empty(), "retained child close")?.progress()));
        }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.child_value, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if !self.closing {return Ok(0);} if !self.child.terminal_is_empty(){return self.child.next_close_depth_demand();} self.close.next_owner_depth_with_binding(self.child_value.is_some()||self.output.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if !self.child.terminal_is_empty() { return self.child.next_close_copy_byte_demand(); }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if !self.child.terminal_is_empty() { return self.child.next_close_capacity_byte_demand(maximum_release_bytes); }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if self.child_value.is_some() { return self.close.next_owner_capacity_with_binding::<T>(true,maximum_release_bytes,&self.source); }
        self.close.next_owner_capacity_with_binding::<Option<T>>(self.output.is_some(),maximum_release_bytes,&self.source)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if !self.child.terminal_is_empty() { return self.child.next_close_release_byte_demand(); }
        self.close.next_release_with_binding(&self.source)
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.child.terminal_is_empty() && self.child_value.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none() }
}

impl<T: RetainedClone> RetainedClone for Option<T> {
    type Cursor = OptionCursor<T>;
    fn retained_clone_cursor() -> Self::Cursor {
        OptionCursor::default()
    }
}

pub struct BoxCursor<T: RetainedClone> {
    child: Option<Box<T::Cursor>>,
    value: Option<T>,
    output: Option<Box<T>>,
    source: Option<RetainedCloneBinding>,
    spent: bool,
    closing: bool,
    close: RetainedCloneClose,
}

impl<T: RetainedClone> Default for BoxCursor<T> {
    fn default() -> Self {
        Self { child: None, value: None, output: None, source: None, spent: false, closing: false, close: RetainedCloneClose::default() }
    }
}

impl<T: RetainedClone> RetainedCloneCursor<Box<T>> for BoxCursor<T> {
    fn advance(&mut self, source: RetainedCloneRef<'_, Box<T>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained box clone cursor is closing"));
        }
        if self.spent {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained box clone cursor is spent"));
        }
        if self.output.is_some() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        source.bind(&mut self.source)?;
        if self.value.is_some() {
            if let Some(child) = self.child.as_mut() {
                if !child.terminal_is_empty() {
                    if grant.maximum_items == 0 {
                        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                    }
                    let step = child.close_step(grant)?;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained box child scaffold close")?.progress()));
                }
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                let bytes = size_of::<T::Cursor>();
                if bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.child = None;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: bytes }));
            }
            let capacity = size_of::<T>();
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity, released_bytes: 0 };
            if !progress.fits(grant) {
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
            }
            let value = self.value.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained box child owner is missing"))?;
            self.output = Some(Box::new(value));
            return Ok(RetainedCloneStep::Complete(progress));
        }
        if self.child.is_none() {
            if grant.maximum_depth == 0 {
                return Err(crate::ValueError::new(crate::ValueRefusalKind::DepthLimit, "retained box clone structural depth limit exceeded"));
            }
            let capacity = size_of::<T::Cursor>();
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity, released_bytes: 0 };
            if !progress.fits(grant) {
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
            }
            self.child = Some(Box::new(T::retained_clone_cursor()));
            return Ok(RetainedCloneStep::Progress(progress));
        }
        let child = self.child.as_mut().expect("initialized retained box child");
        let child_grant = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        match child.advance(source.project(1, |boxed| boxed.as_ref()), child_grant)? {
            RetainedCloneStep::Progress(progress) => Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(child_grant, progress, "retained box child")?)),
            RetainedCloneStep::Complete(progress) => {
                let progress = admit_retained_clone_progress(child_grant, progress, "retained box child")?;
                if progress.copied_items >= grant.maximum_items {
                    return Ok(RetainedCloneStep::Progress(progress));
                }
                self.value = Some(child.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained box child completed without an owner"))?);
                let _ = child.begin_close();
                Ok(RetainedCloneStep::Progress(progress.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?))
            }
        }
    }

    fn take(&mut self) -> Option<Box<T>> {
        let output = self.output.take();
        if output.is_some() {
            self.spent = true;
        }
        output
    }

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "native owner must begin close before granted retirement")); }
        if let Some(child) = self.child.as_mut() {
            if child.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            if !child.terminal_is_empty() { let step = child.close_step(grant)?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained child close")?.progress())); }
            if grant.maximum_depth==0 { return Err(crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"boxed clone scaffold release requires depth")); }
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: size_of::<T::Cursor>() };
            if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.child = None;
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.value, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if !self.closing {return Ok(0);} if let Some(child)=self.child.as_ref(){return if child.terminal_is_empty(){Ok(1)}else{child.next_close_depth_demand()};} self.close.next_owner_depth_with_binding(self.value.is_some()||self.output.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child) = self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_copy_byte_demand() }; }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child)=self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_capacity_byte_demand(maximum_release_bytes) }; }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if self.value.is_some() { return self.close.next_owner_capacity_with_binding::<T>(true,maximum_release_bytes,&self.source); }
        self.close.next_owner_capacity_with_binding::<Box<T>>(self.output.is_some(),maximum_release_bytes,&self.source)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child)=self.child.as_ref() { return if child.terminal_is_empty() { Ok(size_of::<T::Cursor>()) } else { child.next_close_release_byte_demand() }; }
        self.close.next_release_with_binding(&self.source)
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.child.is_none() && self.value.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none() }
}

impl<T: RetainedClone> RetainedClone for Box<T> {
    type Cursor = BoxCursor<T>;
    fn retained_clone_cursor() -> Self::Cursor {
        BoxCursor::default()
    }
}

macro_rules! retained_clone_tuple {
    ($cursor:ident, $count:expr, $(($index:tt, $type:ident, $child:ident, $value:ident)),+ $(,)?) => {
        pub struct $cursor<$($type: RetainedClone),+> {
            $($child: $type::Cursor, $value: Option<$type>,)+
            phase: usize,
            source: Option<RetainedCloneBinding>,
            output: Option<($($type,)+)>,
            spent: bool,
            draining: bool,
            closing: bool,
            close: RetainedCloneClose,
        }

        impl<$($type: RetainedClone),+> Default for $cursor<$($type),+> {
            fn default() -> Self {
                Self {
                    $($child: $type::retained_clone_cursor(), $value: None,)+
                    phase: 0,
                    source: None,
                    output: None,
                    spent: false,
                    draining: false,
                    closing: false,
                    close: RetainedCloneClose::default(),
                }
            }
        }

        impl<$($type: RetainedClone),+> RetainedCloneCursor<($($type,)+)> for $cursor<$($type),+> {
            fn advance(&mut self, source: RetainedCloneRef<'_, ($($type,)+)>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
                if self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple clone cursor is closing")); }
                if self.spent { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple clone cursor is spent")); }
                if self.output.is_some() { return Ok(RetainedCloneStep::Complete(Default::default())); }
                if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        source.bind(&mut self.source)?;
                if self.draining {
                    match self.phase {
                        $($index => {
                            if !self.$child.terminal_is_empty() {
                                if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                                let step = self.$child.close_step(grant)?;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.$child.terminal_is_empty(), "retained tuple child scaffold close")?.progress()));
                            }
                            if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                            self.draining = false;
                            self.phase += 1;
                            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
                        },)+
                        _ => return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple drain state is invalid")),
                    }
                }
                match self.phase {
                    $($index => match self.$child.advance(source.project($index + 1, |value| &value.$index), grant)? {
                        RetainedCloneStep::Progress(progress) => Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant, progress, "retained tuple child")?)),
                        RetainedCloneStep::Complete(progress) => {
                            let progress = admit_retained_clone_progress(grant, progress, "retained tuple child")?;
                            if progress.copied_items >= grant.maximum_items {
                                return Ok(RetainedCloneStep::Progress(progress));
                            }
                            self.$value = Some(self.$child.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple child completed without an owner"))?);
                            let _ = self.$child.begin_close();
                            self.draining = true;
                            Ok(RetainedCloneStep::Progress(progress.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?))
                        }
                    },)+
                    $count => {
                        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                        self.output = Some(($(self.$value.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple owner is missing"))?,)+));
                        self.phase += 1;
                        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
                    }
                    _ => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple clone state is invalid")),
                }
            }

            fn take(&mut self) -> Option<($($type,)+)> {
                let output = self.output.take();
                if output.is_some() { self.spent = true; }
                output
            }

            fn begin_close(&mut self) -> bool {
                if self.closing { return false; }
                self.closing = true;
                true
            }

            

            fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
                if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "tuple must begin close before granted retirement")); }
                $(if !self.$child.terminal_is_empty() {
                    if self.$child.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
                    let step = self.$child.close_step(grant)?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.$child.terminal_is_empty(), "retained child close")?.progress()));
                })+
                if !self.close.is_empty() { return self.close.step_granted(grant); }
                $(if let Some(step) = self.close.begin_granted(&mut self.$value, grant)? { return Ok(step); })+
                if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
                close_retained_binding(&mut self.source, grant)
            }

            fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if !self.closing {return Ok(0);} $(if !self.$child.terminal_is_empty(){return self.$child.next_close_depth_demand();})+ self.close.next_owner_depth_with_binding(self.output.is_some()$(||self.$value.is_some())+,&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
                if !self.closing { return Ok(0); }
                $(if !self.$child.terminal_is_empty() { return self.$child.next_close_copy_byte_demand(); })+
                self.close.next_copy_with_binding(&self.source)
            }
            fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
                if !self.closing { return Ok(0); }
                $(if !self.$child.terminal_is_empty() { return self.$child.next_close_capacity_byte_demand(maximum_release_bytes); })+
                if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
                $(if self.$value.is_some() { return self.close.next_owner_capacity_with_binding::<$type>(true,maximum_release_bytes,&self.source); })+
                self.close.next_owner_capacity_with_binding::<($($type,)+)>(self.output.is_some(),maximum_release_bytes,&self.source)
            }
            fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
                if !self.closing { return Ok(0); }
                $(if !self.$child.terminal_is_empty() { return self.$child.next_close_release_byte_demand(); })+
                self.close.next_release_with_binding(&self.source)
            }
            fn terminal_is_empty(&self) -> bool {
                self.closing && self.output.is_none() && self.close.is_empty() && self.source.is_none() $(&& self.$value.is_none() && self.$child.terminal_is_empty())+
            }
        }

        impl<$($type: RetainedClone),+> RetainedClone for ($($type,)+) {
            type Cursor = $cursor<$($type),+>;
            fn retained_clone_cursor() -> Self::Cursor { Default::default() }
        }
    };
}

retained_clone_tuple!(Tuple2Cursor, 2, (0, A, first_cursor, first_value), (1, B, second_cursor, second_value));
retained_clone_tuple!(Tuple3Cursor, 3, (0, A, first_cursor, first_value), (1, B, second_cursor, second_value), (2, C, third_cursor, third_value));
