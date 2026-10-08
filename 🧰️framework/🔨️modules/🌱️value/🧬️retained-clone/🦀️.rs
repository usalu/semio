//! 🧬️ Bounded native-owner cloning with independent payload and capacity credits.

use crate::{ErasedSnapshotRetirement, SnapshotRetirementStep, retirement::RetireOwned};
use std::{
    any::Any,
    mem::{size_of, size_of_val},
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

static RETAINED_CLONE_SOURCE_IDS: AtomicU64 = AtomicU64::new(1);

struct RetainedCloneLeaseOwner {
    id: u64,
    _source_alias: Box<dyn Any + Send + Sync>,
    _authority: Box<dyn Any + Send + Sync>,
}

pub struct RetainedCloneSource<T: Send + Sync + 'static> {
    owner: Arc<T>,
    lease: Arc<RetainedCloneLeaseOwner>,
}

impl<T: Send + Sync + 'static> RetainedCloneSource<T> {
    #[cfg(test)]
    pub(crate) fn from_owner(owner: T) -> Self {
        let owner = Arc::new(owner);
        let source_alias: Box<dyn Any + Send + Sync> = Box::new(Arc::clone(&owner));
        Self { owner, lease: Arc::new(RetainedCloneLeaseOwner { id: RETAINED_CLONE_SOURCE_IDS.fetch_add(1, Ordering::Relaxed), _source_alias: source_alias, _authority: Box::new(()) }) }
    }

    pub fn from_authority<A: Any + Send + Sync>(owner: Arc<T>, authority: A) -> Self {
        let source_alias: Box<dyn Any + Send + Sync> = Box::new(Arc::clone(&owner));
        Self { owner, lease: Arc::new(RetainedCloneLeaseOwner { id: RETAINED_CLONE_SOURCE_IDS.fetch_add(1, Ordering::Relaxed), _source_alias: source_alias, _authority: Box::new(authority) }) }
    }

    /// 📦️ Releases the source lease and transfers its immutable owner after child cursors close.
    pub fn into_owner(self) -> Arc<T> { self.owner }

    pub fn borrow(&self) -> RetainedCloneRef<'_, T> {
        RetainedCloneRef { value: self.owner.as_ref(), lease: &self.lease, projection: RetainedCloneProjection { parent: self.owner.as_ref() as *const T as usize, address: self.owner.as_ref() as *const T as usize, discriminator: 0 } }
    }

    /// 🔗️ Retains a projected native field through the actual immutable root lease.
    pub fn project_owned<U: ?Sized + Sync, F>(&self, discriminator: usize, project: F) -> RetainedOwnedProjection<U>
    where F: for<'source> FnOnce(&'source T) -> &'source U {
        unsafe { RetainedOwnedProjection::from_source(self.borrow().project(discriminator, project)) }
    }
}

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
        let candidate = RetainedCloneBinding { lease: Arc::clone(self.lease), projection: self.projection };
        if let Some(expected) = binding {
            if expected.lease.id != candidate.lease.id || expected.projection != candidate.projection {
                return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained clone source lease or projected path changed"));
            }
        } else {
            *binding = Some(candidate);
        }
        Ok(())
    }
}

pub struct RetainedCloneBinding {
    lease: Arc<RetainedCloneLeaseOwner>,
    projection: RetainedCloneProjection,
}

impl RetainedCloneBinding {
    /// 🧷️ Releases one alias while the external source authority remains retained.
    pub fn close_one(binding: &mut Option<Self>, maximum_items: usize) -> Result<SnapshotRetirementStep, crate::ValueError> {
        let Some(owner) = binding.as_ref() else { return Ok(SnapshotRetirementStep::Complete); };
        if maximum_items == 0 { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        if Arc::strong_count(&owner.lease) <= 1 {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained source authority must remain alive through binding close"));
        }
        *binding = None;
        Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
    }
}

fn retained_clone_exclusive_lease() -> Arc<RetainedCloneLeaseOwner> {
    Arc::new(RetainedCloneLeaseOwner { id: RETAINED_CLONE_SOURCE_IDS.fetch_add(1, Ordering::Relaxed), _source_alias: Box::new(()), _authority: Box::new(()) })
}

/// 🧷️ Binds borrowed fields to one externally retained immutable operation authority.
/// The caller retains the source owner until its cursors have reached terminal-empty closure.
pub struct RetainedCloneBorrowAuthority { lease: Arc<RetainedCloneLeaseOwner> }

impl RetainedCloneBorrowAuthority {
    pub fn new<A: Any + Send + Sync>(authority: A) -> Self {
        Self { lease: Arc::new(RetainedCloneLeaseOwner { id: RETAINED_CLONE_SOURCE_IDS.fetch_add(1, Ordering::Relaxed), _source_alias: Box::new(()), _authority: Box::new(authority) }) }
    }

    pub fn borrow<'source, T: ?Sized>(&'source self, source: &'source T) -> RetainedCloneRef<'source, T> {
        RetainedCloneRef { value: source, lease: &self.lease, projection: RetainedCloneProjection { parent: source as *const T as *const () as usize, address: source as *const T as *const () as usize, discriminator: 0 } }
    }
}

fn retained_clone_exclusive_ref<'a, T: ?Sized>(value: &'a T, lease: &'a Arc<RetainedCloneLeaseOwner>, discriminator: usize) -> RetainedCloneRef<'a, T> {
    RetainedCloneRef { value, lease, projection: RetainedCloneProjection { parent: lease.id as usize, address: value as *const T as *const () as usize, discriminator } }
}

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

pub fn admit_retained_clone_retirement(step: SnapshotRetirementStep, maximum_items: usize, maximum_bytes: usize, scope: &str) -> Result<SnapshotRetirementStep, crate::ValueError> {
    match step {
        SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > maximum_items || released_bytes > maximum_bytes => {
            Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, format!("{scope} exceeded its retained retirement item or byte grant")))
        }
        step => Ok(step),
    }
}

/// ♻️ Admits scaffold release while refusing a grant that cannot make observable progress.
pub fn admit_retained_clone_scaffold_retirement(step: SnapshotRetirementStep, maximum_items: usize, maximum_bytes: usize, scope: &str) -> Result<SnapshotRetirementStep, crate::ValueError> {
    match admit_retained_clone_retirement(step, maximum_items, maximum_bytes, scope)? {
        SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 } | SnapshotRetirementStep::Blocked if maximum_bytes == 0 => Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }),
        SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 } | SnapshotRetirementStep::Blocked => Err(crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, format!("{scope} cannot progress under its retained clone scaffold-release grant"))),
        step => Ok(step),
    }
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
/// The cursor owner retains it through `begin_close` and capacity-bearing `close_granted` turns until
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
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, crate::ValueError>;
    fn terminal_is_empty(&self) -> bool;
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
    fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "clone cursor has no capacity-bearing close authority"))
    }
}

trait ControlledCloneRetirement: Send {
    fn step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError>;
    fn terminal_is_empty(&self) -> bool;
    fn next_copy_byte_demand(&self) -> usize;
    fn next_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError>;
    fn next_release_byte_demand(&self) -> Result<usize, crate::ValueError>;
}

impl<T: RetireOwned> ControlledCloneRetirement for crate::retirement::controlled::ControlledRetirement<T> {
    fn step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> { self.step(grant) }
    fn terminal_is_empty(&self) -> bool { self.terminal_is_empty() }
    fn next_copy_byte_demand(&self) -> usize { self.next_copy_byte_demand() }
    fn next_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> { self.next_capacity_byte_demand(maximum_release_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, crate::ValueError> { self.next_release_byte_demand() }
}

#[derive(Default)]
pub struct RetainedCloneClose {
    retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
    controlled: Option<Box<dyn ControlledCloneRetirement>>,
    controlled_bytes: usize,
}

impl RetainedCloneClose {
    /// 🔍️ Borrows the active cold owner's body or whole terminal Box release without moving it.
    pub fn next_cold_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if self.controlled.is_some() { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "controlled clone retirement cannot enter cold demand")); }
        Ok(self.retirement.as_ref().map_or(0, |owner| if owner.terminal_is_empty() { size_of_val(owner.as_ref()) } else { owner.next_close_byte_demand() }))
    }

    /// 🧮️ Borrows the active typed retirement's minimum payload work independently of physical release.
    pub fn next_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if self.retirement.is_some() { return Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "cold clone retirement has no controlled work demand")); }
        Ok(self.controlled.as_ref().map_or(0, |owner| owner.next_copy_byte_demand()))
    }
    /// 📐️ Observes the actual controlled owner Box required by the next handoff.
    pub fn next_owner_capacity_byte_demand<T: RetireOwned>(&self, present: bool, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.is_empty() { return self.next_capacity_byte_demand(maximum_release_bytes); }
        if present && !T::controlled_retirement_supported() { return Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "typed owner has no controlled close birth authority")); }
        Ok(if present { size_of::<crate::retirement::controlled::ControlledRetirement<T>>() } else { 0 })
    }
    /// 📐️ Observes the next controlled child birth without allocating or moving an owner.
    pub fn next_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if self.retirement.is_some() { return Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "cold clone retirement has no controlled birth demand")); }
        self.controlled.as_ref().map_or(Ok(0), |owner| owner.next_capacity_byte_demand(maximum_release_bytes))
    }

    /// 📐️ Observes the next physical body or terminal scaffold release independently of copying.
    pub fn next_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if self.retirement.is_some() { return Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "cold clone retirement has no controlled release demand")); }
        self.controlled.as_ref().map_or(Ok(0), |owner| if owner.terminal_is_empty() { Ok(self.controlled_bytes) } else { owner.next_release_byte_demand() })
    }

    pub fn begin<T: RetireOwned>(&mut self, value: T) -> Result<(), crate::ValueError> {
        if !self.is_empty() {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained clone retirement frontier is occupied"));
        }
        self.retirement = Some(super::retirement::owned_retirement(value));
        Ok(())
    }

    pub fn step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, crate::ValueError> {
        if self.controlled.is_some() { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "granted clone retirement cannot enter cold close")); }
        let Some(retirement) = self.retirement.as_mut() else { return Ok(SnapshotRetirementStep::Complete) };
        if retirement.terminal_is_empty() {
            let bytes = size_of_val(retirement.as_ref());
            if maximum_items == 0 || maximum_bytes < bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
            drop(self.retirement.take());
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes });
        }
        let step = admit_retained_clone_retirement(retirement.close_step(maximum_items, maximum_bytes)?, maximum_items, maximum_bytes, "retained clone owner retirement")?;
        if step == SnapshotRetirementStep::Complete {
            if !retirement.terminal_is_empty() {
                return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained clone retirement completed with a live owner"));
            }
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        Ok(step)
    }

    pub fn begin_option<T: RetireOwned>(&mut self, value: &mut Option<T>, maximum_items: usize) -> Result<Option<SnapshotRetirementStep>, crate::ValueError> {
        if !self.is_empty() {
            return Ok(None);
        }
        if value.is_none() {
            return Ok(None);
        }
        if maximum_items == 0 {
            return Ok(Some(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
        }
        self.begin(value.take().expect("checked retained clone retirement owner"))?;
        Ok(Some(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }))
    }

    pub fn begin_granted<T: RetireOwned>(&mut self, value: &mut Option<T>, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneStep>, crate::ValueError> {
        if !self.is_empty() || value.is_none() { return Ok(None); }
        let bytes = size_of::<crate::retirement::controlled::ControlledRetirement<T>>();
        let progress = RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: bytes, ..Default::default() };
        if !progress.fits(grant) { return Ok(Some(RetainedCloneStep::Progress(Default::default()))); }
        let owner = value.take().unwrap();
        let retirement = match crate::retirement::controlled::ControlledRetirement::new(owner) {
            Ok(retirement) => retirement,
            Err((error, owner)) => { *value = Some(owner); return Err(error); }
        };
        self.controlled = Some(Box::new(retirement));
        self.controlled_bytes = bytes;
        Ok(Some(RetainedCloneStep::Progress(progress)))
    }

    pub fn begin_default_granted<T: RetireOwned + Default>(&mut self, value: &mut T, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneStep>, crate::ValueError> {
        if !self.is_empty() { return Ok(None); }
        let bytes = size_of::<crate::retirement::controlled::ControlledRetirement<T>>();
        if grant.maximum_items == 0 || bytes > grant.maximum_capacity_bytes { return Ok(Some(RetainedCloneStep::Progress(Default::default()))); }
        let mut owner = Some(std::mem::take(value));
        match self.begin_granted(&mut owner, grant) {
            Ok(step) => { if let Some(owner) = owner { *value = owner; } Ok(step) }
            Err(error) => { if let Some(owner) = owner { *value = owner; } Err(error) }
        }
    }

    pub fn step_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.retirement.is_some() { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "cold clone retirement cannot enter granted close")); }
        let Some(retirement) = self.controlled.as_mut() else { return Ok(RetainedCloneStep::Complete(Default::default())); };
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if retirement.terminal_is_empty() {
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: self.controlled_bytes };
            if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.controlled = None;
            self.controlled_bytes = 0;
            return Ok(RetainedCloneStep::Progress(progress));
        }
        let step = retirement.step(grant)?;
        let progress = admit_retained_clone_progress(grant, step.progress(), "controlled clone retirement")?;
        if matches!(step, RetainedCloneStep::Complete(_)) && !retirement.terminal_is_empty() {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "controlled clone retirement completed with owners"));
        }
        Ok(RetainedCloneStep::Progress(progress))
    }

    pub fn is_empty(&self) -> bool {
        self.retirement.is_none() && self.controlled.is_none()
    }
}

pub fn close_retained_binding(binding: &mut Option<RetainedCloneBinding>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
    match RetainedCloneBinding::close_one(binding, grant.maximum_items)? {
        SnapshotRetirementStep::Complete => Ok(RetainedCloneStep::Complete(Default::default())),
        SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes })),
        SnapshotRetirementStep::Blocked => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained binding close blocked")),
    }
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

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, crate::ValueError> {
        if let Some(step) = self.close.begin_option(&mut self.value, maximum_items)? {
            return Ok(step);
        }
        let step = self.close.step(maximum_items, maximum_bytes)?;
        if step == SnapshotRetirementStep::Complete {
            self.source = None;
        }
        Ok(step)
    }

    fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "clone cursor must begin close before granted retirement")); }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.value, grant)? { return Ok(step); }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_copy_byte_demand()
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_owner_capacity_byte_demand::<T>(self.value.is_some(), maximum_release_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_release_byte_demand()
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

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, crate::ValueError> {
        if let Some(step) = self.close.begin_option(&mut self.output, maximum_items)? {
            return Ok(step);
        }
        let step = self.close.step(maximum_items, maximum_bytes)?;
        if step == SnapshotRetirementStep::Complete {
            self.source = None;
        }
        Ok(step)
    }

    fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "clone cursor must begin close before granted retirement")); }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_copy_byte_demand()
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_owner_capacity_byte_demand::<String>(self.output.is_some(), maximum_release_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_release_byte_demand()
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
                        let step = child.close_granted(remaining)?;
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

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, crate::ValueError> {
        if let Some(child) = self.child.as_mut() {
            if child.begin_close() {
                if maximum_items == 0 {
                    return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            let step = admit_retained_clone_retirement(child.close_step(maximum_items, maximum_bytes)?, maximum_items, maximum_bytes, "retained vector child close")?;
            if step != SnapshotRetirementStep::Complete {
                return Ok(step);
            }
            if !child.terminal_is_empty() {
                return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained vector child close completed with a live owner"));
            }
            if maximum_items == 0 {
                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.child = None;
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if !self.close.is_empty() {
            let step = self.close.step(maximum_items, maximum_bytes)?;
            return Ok(if step == SnapshotRetirementStep::Complete { SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 } } else { step });
        }
        if let Some(step) = self.close.begin_option(&mut self.child_value, maximum_items)? {
            return Ok(step);
        }
        if let Some(step) = self.close.begin_option(&mut self.output, maximum_items)? {
            return Ok(step);
        }
        if !self.values.is_empty() || (size_of::<T>() != 0 && self.values.capacity() != 0) {
            if maximum_items == 0 {
                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.close.begin(std::mem::take(&mut self.values))?;
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        self.source = None;
        Ok(SnapshotRetirementStep::Complete)
    }

    fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "native owner must begin close before granted retirement")); }
        if let Some(child) = self.child.as_mut() {
            if child.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            if !child.terminal_is_empty() { let step = child.close_granted(grant)?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained child close")?.progress())); }
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

    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child) = self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_copy_byte_demand() }; }
        self.close.next_copy_byte_demand()
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child)=self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_capacity_byte_demand(maximum_release_bytes) }; }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if self.child_value.is_some() { return self.close.next_owner_capacity_byte_demand::<T>(true,maximum_release_bytes); }
        self.close.next_owner_capacity_byte_demand::<Vec<T>>(self.output.is_some()||!self.values.is_empty()||(size_of::<T>()!=0&&self.values.capacity()!=0),maximum_release_bytes)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child)=self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_release_byte_demand() }; }
        self.close.next_release_byte_demand()
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
                        let step = self.child.close_granted(grant)?;
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

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, crate::ValueError> {
        if !self.child.terminal_is_empty() {
            if self.child.begin_close() {
                if maximum_items == 0 {
                    return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            let step = admit_retained_clone_retirement(self.child.close_step(maximum_items, maximum_bytes)?, maximum_items, maximum_bytes, "retained optional child close")?;
            if step == SnapshotRetirementStep::Complete {
                if !self.child.terminal_is_empty() {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained optional child completed close with a live owner"));
                }
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if !self.close.is_empty() {
            let step = self.close.step(maximum_items, maximum_bytes)?;
            return Ok(if step == SnapshotRetirementStep::Complete { SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 } } else { step });
        }
        if let Some(step) = self.close.begin_option(&mut self.child_value, maximum_items)? {
            return Ok(step);
        }
        if let Some(step) = self.close.begin_option(&mut self.output, maximum_items)? {
            return Ok(step);
        }
        self.source = None;
        Ok(SnapshotRetirementStep::Complete)
    }

    fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "native owner must begin close before granted retirement")); }
        if !self.child.terminal_is_empty() {
            if self.child.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            let step = self.child.close_granted(grant)?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.child.terminal_is_empty(), "retained child close")?.progress()));
        }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.child_value, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if !self.child.terminal_is_empty() { return self.child.next_close_copy_byte_demand(); }
        self.close.next_copy_byte_demand()
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if !self.child.terminal_is_empty() { return self.child.next_close_capacity_byte_demand(maximum_release_bytes); }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if self.child_value.is_some() { return self.close.next_owner_capacity_byte_demand::<T>(true,maximum_release_bytes); }
        self.close.next_owner_capacity_byte_demand::<Option<T>>(self.output.is_some(),maximum_release_bytes)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if !self.child.terminal_is_empty() { return self.child.next_close_release_byte_demand(); }
        self.close.next_release_byte_demand()
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
                    let step = child.close_granted(grant)?;
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

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, crate::ValueError> {
        if let Some(child) = self.child.as_mut() {
            if child.begin_close() {
                if maximum_items == 0 {
                    return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            let step = admit_retained_clone_retirement(child.close_step(maximum_items, maximum_bytes)?, maximum_items, maximum_bytes, "retained box child close")?;
            if step != SnapshotRetirementStep::Complete {
                return Ok(step);
            }
            if !child.terminal_is_empty() {
                return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained box child completed close with a live owner"));
            }
            if maximum_items == 0 {
                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.child = None;
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if !self.close.is_empty() {
            let step = self.close.step(maximum_items, maximum_bytes)?;
            return Ok(if step == SnapshotRetirementStep::Complete { SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 } } else { step });
        }
        if let Some(step) = self.close.begin_option(&mut self.value, maximum_items)? {
            return Ok(step);
        }
        if let Some(step) = self.close.begin_option(&mut self.output, maximum_items)? {
            return Ok(step);
        }
        self.source = None;
        Ok(SnapshotRetirementStep::Complete)
    }

    fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "native owner must begin close before granted retirement")); }
        if let Some(child) = self.child.as_mut() {
            if child.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            if !child.terminal_is_empty() { let step = child.close_granted(grant)?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained child close")?.progress())); }
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

    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child) = self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_copy_byte_demand() }; }
        self.close.next_copy_byte_demand()
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child)=self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_capacity_byte_demand(maximum_release_bytes) }; }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if self.value.is_some() { return self.close.next_owner_capacity_byte_demand::<T>(true,maximum_release_bytes); }
        self.close.next_owner_capacity_byte_demand::<Box<T>>(self.output.is_some(),maximum_release_bytes)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child)=self.child.as_ref() { return if child.terminal_is_empty() { Ok(size_of::<T::Cursor>()) } else { child.next_close_release_byte_demand() }; }
        self.close.next_release_byte_demand()
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
                                let step = self.$child.close_granted(grant)?;
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

            fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, crate::ValueError> {
                $(if !self.$child.terminal_is_empty() {
                    if self.$child.begin_close() {
                        if maximum_items == 0 { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
                        return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
                    }
                    let step = admit_retained_clone_retirement(
                        self.$child.close_step(maximum_items, maximum_bytes)?,
                        maximum_items,
                        maximum_bytes,
                        "retained tuple child close",
                    )?;
                    if step == SnapshotRetirementStep::Complete {
                        if !self.$child.terminal_is_empty() { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple child completed close with a live owner")); }
                        return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
                    }
                    return Ok(step);
                })+
                if !self.close.is_empty() {
                    let step = self.close.step(maximum_items, maximum_bytes)?;
                    return Ok(if step == SnapshotRetirementStep::Complete { SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 } } else { step });
                }
                $(if let Some(step) = self.close.begin_option(&mut self.$value, maximum_items)? { return Ok(step); })+
                if let Some(step) = self.close.begin_option(&mut self.output, maximum_items)? { return Ok(step); }
                self.source = None;
                Ok(SnapshotRetirementStep::Complete)
            }

            fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
                if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "tuple must begin close before granted retirement")); }
                $(if !self.$child.terminal_is_empty() {
                    if self.$child.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
                    let step = self.$child.close_granted(grant)?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.$child.terminal_is_empty(), "retained child close")?.progress()));
                })+
                if !self.close.is_empty() { return self.close.step_granted(grant); }
                $(if let Some(step) = self.close.begin_granted(&mut self.$value, grant)? { return Ok(step); })+
                if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
                close_retained_binding(&mut self.source, grant)
            }

            fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
                if !self.closing { return Ok(0); }
                $(if !self.$child.terminal_is_empty() { return self.$child.next_close_copy_byte_demand(); })+
                self.close.next_copy_byte_demand()
            }
            fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
                if !self.closing { return Ok(0); }
                $(if !self.$child.terminal_is_empty() { return self.$child.next_close_capacity_byte_demand(maximum_release_bytes); })+
                if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
                $(if self.$value.is_some() { return self.close.next_owner_capacity_byte_demand::<$type>(true,maximum_release_bytes); })+
                self.close.next_owner_capacity_byte_demand::<($($type,)+)>(self.output.is_some(),maximum_release_bytes)
            }
            fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
                if !self.closing { return Ok(0); }
                $(if !self.$child.terminal_is_empty() { return self.$child.next_close_release_byte_demand(); })+
                self.close.next_release_byte_demand()
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
