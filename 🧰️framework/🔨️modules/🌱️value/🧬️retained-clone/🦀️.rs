//! 🧬️ Bounded native-owner cloning with independent payload and capacity credits.

use crate::{ErasedSnapshotRetirement, SnapshotRetirementStep, retirement::RetireOwned};
use std::{
    any::Any,
    mem::size_of,
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

    pub fn borrow(&self) -> RetainedCloneRef<'_, T> {
        RetainedCloneRef { value: self.owner.as_ref(), lease: &self.lease, projection: RetainedCloneProjection { parent: self.owner.as_ref() as *const T as usize, address: self.owner.as_ref() as *const T as usize, discriminator: 0 } }
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

fn retained_clone_exclusive_lease() -> Arc<RetainedCloneLeaseOwner> {
    Arc::new(RetainedCloneLeaseOwner { id: RETAINED_CLONE_SOURCE_IDS.fetch_add(1, Ordering::Relaxed), _source_alias: Box::new(()), _authority: Box::new(()) })
}

fn retained_clone_exclusive_ref<'a, T: ?Sized>(value: &'a T, lease: &'a Arc<RetainedCloneLeaseOwner>, discriminator: usize) -> RetainedCloneRef<'a, T> {
    RetainedCloneRef { value, lease, projection: RetainedCloneProjection { parent: lease.id as usize, address: value as *const T as *const () as usize, discriminator } }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedCloneGrant {
    pub maximum_items: usize,
    /// 🧮️ Bounds payload copying and completed-child scaffold release within one clone turn.
    pub maximum_copy_bytes: usize,
    pub maximum_capacity_bytes: usize,
    pub maximum_depth: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedCloneProgress {
    pub copied_items: usize,
    pub copied_bytes: usize,
    pub retained_capacity_bytes: usize,
}

impl RetainedCloneProgress {
    pub fn checked_add(self, other: Self) -> Result<Self, crate::ValueError> {
        Ok(Self {
            copied_items: self.copied_items.checked_add(other.copied_items).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained clone item progress overflow"))?,
            copied_bytes: self.copied_bytes.checked_add(other.copied_bytes).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained clone byte progress overflow"))?,
            retained_capacity_bytes: self.retained_capacity_bytes.checked_add(other.retained_capacity_bytes).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained clone capacity progress overflow"))?,
        })
    }

    pub fn fits(self, grant: RetainedCloneGrant) -> bool {
        self.copied_items <= grant.maximum_items && self.copied_bytes <= grant.maximum_copy_bytes && self.retained_capacity_bytes <= grant.maximum_capacity_bytes
    }
}

pub fn admit_retained_clone_progress(grant: RetainedCloneGrant, progress: RetainedCloneProgress, scope: &str) -> Result<RetainedCloneProgress, crate::ValueError> {
    if progress.fits(grant) { Ok(progress) } else { Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, format!("{scope} exceeded its retained clone item, copy, or capacity grant"))) }
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
/// The cursor owner must retain it through `begin_close` and bounded `close_step` calls until
/// `terminal_is_empty`; dropping an active cursor is outside this low-level ownership contract.
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
}

#[derive(Default)]
pub struct RetainedCloneClose {
    retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
}

impl RetainedCloneClose {
    pub fn begin<T: RetireOwned>(&mut self, value: T) -> Result<(), crate::ValueError> {
        if self.retirement.is_some() {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained clone retirement frontier is occupied"));
        }
        self.retirement = Some(super::retirement::owned_retirement(value));
        Ok(())
    }

    pub fn step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, crate::ValueError> {
        let Some(retirement) = self.retirement.as_mut() else { return Ok(SnapshotRetirementStep::Complete) };
        let step = admit_retained_clone_retirement(retirement.close_step(maximum_items, maximum_bytes)?, maximum_items, maximum_bytes, "retained clone owner retirement")?;
        if step == SnapshotRetirementStep::Complete {
            if !retirement.terminal_is_empty() {
                return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained clone retirement completed with a live owner"));
            }
            self.retirement.take();
            return Ok(SnapshotRetirementStep::Complete);
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

    pub fn is_empty(&self) -> bool {
        self.retirement.is_none()
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
        source.bind(&mut self.source)?;
        if self.value.is_some() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<T>(), retained_capacity_bytes: 0 };
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
        source.bind(&mut self.source)?;
        let source = source.get();
        match self.phase {
            0 => {
                let planned = source.len();
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned };
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
                let progress = RetainedCloneProgress { copied_items: 0, copied_bytes: end - start, retained_capacity_bytes: 0 };
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
        source.bind(&mut self.source)?;
        let source_value = source.get();
        if self.phase == 0 {
            let planned = source_value.len().checked_mul(size_of::<T>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained vector clone capacity overflow"))?;
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned };
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
                maximum_depth: grant.maximum_depth,
            };
            if let Some(child) = self.child.as_mut() {
                if self.child_value.is_some() {
                    if !child.terminal_is_empty() {
                        if remaining.maximum_items == 0 {
                            return Ok(RetainedCloneStep::Progress(used));
                        }
                        match admit_retained_clone_scaffold_retirement(child.close_step(remaining.maximum_items, remaining.maximum_copy_bytes)?, remaining.maximum_items, remaining.maximum_copy_bytes, "retained vector child scaffold close")? {
                            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                                let progress = admit_retained_clone_progress(remaining, RetainedCloneProgress { copied_items: released_items, copied_bytes: released_bytes, retained_capacity_bytes: 0 }, "retained vector child scaffold close")?;
                                if progress == RetainedCloneProgress::default() {
                                    return Ok(RetainedCloneStep::Progress(used));
                                }
                                used = used.checked_add(progress)?;
                                if used.copied_items == grant.maximum_items || used.copied_bytes == grant.maximum_copy_bytes {
                                    return Ok(RetainedCloneStep::Progress(used));
                                }
                                continue;
                            }
                            SnapshotRetirementStep::Blocked => return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained vector child scaffold close blocked")),
                            SnapshotRetirementStep::Complete => {
                                if !child.terminal_is_empty() {
                                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained vector child scaffold completed with a live owner"));
                                }
                                used = used.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                                continue;
                            }
                        }
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
            if remaining.maximum_items == 0 && remaining.maximum_copy_bytes == 0 && remaining.maximum_capacity_bytes == 0 {
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

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.values.is_empty() && (size_of::<T>() == 0 || self.values.capacity() == 0) && self.child.is_none() && self.child_value.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none()
    }
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
                        return match admit_retained_clone_scaffold_retirement(self.child.close_step(grant.maximum_items, grant.maximum_copy_bytes)?, grant.maximum_items, grant.maximum_copy_bytes, "retained optional child scaffold close")? {
                            SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, copied_bytes: released_bytes, retained_capacity_bytes: 0 })),
                            SnapshotRetirementStep::Blocked => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained optional child scaffold close blocked")),
                            SnapshotRetirementStep::Complete => {
                                if !self.child.terminal_is_empty() {
                                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained optional child scaffold completed with a live owner"));
                                }
                                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
                            }
                        };
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

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.child.terminal_is_empty() && self.child_value.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none()
    }
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
        source.bind(&mut self.source)?;
        if self.value.is_some() {
            if let Some(child) = self.child.as_mut() {
                if !child.terminal_is_empty() {
                    if grant.maximum_items == 0 {
                        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                    }
                    return match admit_retained_clone_scaffold_retirement(child.close_step(grant.maximum_items, grant.maximum_copy_bytes)?, grant.maximum_items, grant.maximum_copy_bytes, "retained box child scaffold close")? {
                        SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, copied_bytes: released_bytes, retained_capacity_bytes: 0 })),
                        SnapshotRetirementStep::Blocked => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained box child scaffold close blocked")),
                        SnapshotRetirementStep::Complete => {
                            if !child.terminal_is_empty() {
                                return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained box child scaffold completed with a live owner"));
                            }
                            Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
                        }
                    };
                }
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                self.child = None;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
            }
            let capacity = size_of::<T>();
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity };
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
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity };
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

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.child.is_none() && self.value.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none()
    }
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
                source.bind(&mut self.source)?;
                if self.draining {
                    match self.phase {
                        $($index => {
                            if !self.$child.terminal_is_empty() {
                                if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                                return match admit_retained_clone_scaffold_retirement(
                                    self.$child.close_step(grant.maximum_items, grant.maximum_copy_bytes)?,
                                    grant.maximum_items,
                                    grant.maximum_copy_bytes,
                                    "retained tuple child scaffold close",
                                )? {
                                    SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, copied_bytes: released_bytes, retained_capacity_bytes: 0 })),
                                    SnapshotRetirementStep::Blocked => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple child scaffold close blocked")),
                                    SnapshotRetirementStep::Complete => {
                                        if !self.$child.terminal_is_empty() { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple child scaffold completed with a live owner")); }
                                        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
                                    }
                                };
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
