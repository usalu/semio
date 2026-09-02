//! 📋️ Retained typed copying with distinct allocation and initialized-byte admission.

use super::*;
use crate::*;
use std::mem::{size_of, ManuallyDrop};

//#region 🧭️TypedCopy
const COPY_DEPTH: usize = 16;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiComponentCopyProgress {
    pub complete: bool,
    pub progressed: bool,
    pub allocated_bytes: usize,
    pub copied_bytes: usize,
}

fn error(reason: &'static str) -> UiFixedListAllocationError { UiFixedListAllocationError { allocated_bytes: 0, reason } }
fn done() -> UiComponentCopyProgress { UiComponentCopyProgress { complete: true, progressed: true, ..Default::default() } }
fn progress(bytes: usize) -> UiComponentCopyProgress { UiComponentCopyProgress { progressed: true, copied_bytes: bytes, ..Default::default() } }
fn split(path: &mut [usize]) -> Result<(&mut usize, &mut [usize]), UiFixedListAllocationError> { path.split_first_mut().ok_or_else(|| error("typed copy exceeds schema depth")) }
fn read_path(path: &[usize]) -> Result<(usize, &[usize]), &'static str> { path.split_first().map(|(index, rest)| (*index, rest)).ok_or("typed copy exceeds schema depth") }
const fn maximum(depths: &[usize]) -> usize {
    let mut result = 0;
    let mut index = 0;
    while index < depths.len() { if depths[index] > result { result = depths[index]; } index += 1; }
    result
}

trait TypedCopy: Sized {
    const DEPTH: usize = 0;
    fn empty_like(&self) -> Self;
    fn allocation(&self, candidate: &Self, path: &[usize]) -> Result<usize, &'static str>;
    fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError>;
}

fn field<T: TypedCopy>(source: &T, candidate: &mut T, index: &mut usize, path: &mut [usize], count: usize, byte_candidate: &mut Vec<u8>, allocation: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> {
    let mut step = source.copy_one(candidate, path, byte_candidate, allocation, work)?;
    if step.complete { *index += 1; path.fill(0); }
    step.complete = *index == count;
    Ok(step)
}

macro_rules! typed_fields {
    ($type:ty { $($index:literal => $field:tt : $field_type:ty),* $(,)? }) => {
        impl TypedCopy for $type {
            const DEPTH: usize = 1 + maximum(&[$(<$field_type as TypedCopy>::DEPTH),*]);
            fn empty_like(&self) -> Self { Self { $($field: self.$field.empty_like()),* } }
            fn allocation(&self, candidate: &Self, path: &[usize]) -> Result<usize, &'static str> {
                let (index, path) = read_path(path)?;
                match index { $($index => self.$field.allocation(&candidate.$field, path),)* _ => Ok(0) }
            }
            fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> {
                let Self { $($field: _),* } = self;
                let (index, path) = split(path)?;
                let count = 0 $(+ { let _ = stringify!($field); 1 })*;
                match *index { $($index => field(&self.$field, &mut candidate.$field, index, path, count, byte_candidate, allocation, work),)* _ => Ok(done()) }
            }
        }
    };
}

macro_rules! scalar {
    ($($type:ty),* $(,)?) => {$(
        impl TypedCopy for $type {
            fn empty_like(&self) -> Self { *self }
            fn allocation(&self, _: &Self, _: &[usize]) -> Result<usize, &'static str> { Ok(0) }
            fn copy_one(&self, candidate: &mut Self, _: &mut [usize], byte_candidate: &mut Vec<u8>, _: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> {
                if work < size_of::<Self>() { return Ok(Default::default()); }
                *candidate = *self;
                Ok(UiComponentCopyProgress { complete: true, ..progress(size_of::<Self>()) })
            }
        }
    )*};
}
scalar!(bool, u16, u64, f64, UiNodeId, UiRevision, Activity, TransitionHint, StyleSpec, Trigger, ContainerRole, InputKind, RowActionPlacement, SurfaceKind, Liveness, GridTrack, SpaceToken, Align, Justify, EdgeSpace, Axis, Anchor, ScrollAxes, Sizing);

impl TypedCopy for UiText {
    fn empty_like(&self) -> Self { Self::default() }
    fn allocation(&self, _: &Self, _: &[usize]) -> Result<usize, &'static str> { Ok(0) }
    fn copy_one(&self, candidate: &mut Self, _: &mut [usize], byte_candidate: &mut Vec<u8>, _: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> {
        let start = candidate.len();
        let bytes = self.len().saturating_sub(start).min(work);
        candidate.bytes[start..start + bytes].copy_from_slice(&self.bytes[start..start + bytes]);
        candidate.len += bytes as u16;
        Ok(UiComponentCopyProgress { complete: candidate.len == self.len, ..progress(bytes) })
    }
}

impl TypedCopy for UiFixedBytes {
    fn empty_like(&self) -> Self { Self { bytes: Box::default(), len: 0 } }
    fn allocation(&self, candidate: &Self, _: &[usize]) -> Result<usize, &'static str> { Ok(if self.is_empty() || !candidate.bytes.is_empty() { 0 } else { UI_FIXED_BYTES }) }
    fn copy_one(&self, candidate: &mut Self, _: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> {
        if self.is_empty() || !candidate.bytes.is_empty() { return Ok(done()); }
        if byte_candidate.capacity() == 0 {
            return reserve_byte_candidate(byte_candidate, allocation, |owner, requested| owner.try_reserve_exact(requested).map_err(|_| ()));
        }
        if byte_candidate.capacity() != UI_FIXED_BYTES { return Err(error("component byte candidate requires retirement after capacity rejection")); }
        let start = byte_candidate.len();
        if start < UI_FIXED_BYTES {
            let end = if start < self.len() { self.len() } else { UI_FIXED_BYTES };
            let bytes = (end - start).min(work);
            if start < self.len() { byte_candidate.extend_from_slice(&self.bytes[start..start + bytes]); } else { byte_candidate.resize(start + bytes, 0); }
            return Ok(progress(bytes));
        }
        if work < size_of::<Self>() { return Ok(Default::default()); }
        if byte_candidate.len() != byte_candidate.capacity() { return Err(error("component byte candidate transfer would reallocate")); }
        candidate.bytes = std::mem::take(byte_candidate).into_boxed_slice();
        candidate.len = self.len;
        Ok(UiComponentCopyProgress { complete: true, ..progress(size_of::<Self>()) })
    }
}

fn reserve_byte_candidate(candidate: &mut Vec<u8>, grant: usize, allocate: impl FnOnce(&mut Vec<u8>, usize) -> Result<(), ()>) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> {
    if grant < UI_FIXED_BYTES { return Ok(Default::default()); }
    if candidate.capacity() != 0 || !candidate.is_empty() { return Err(error("component byte candidate allocation requires empty ownership")); }
    let allocated = allocate(candidate, UI_FIXED_BYTES);
    let actual = candidate.capacity();
    if allocated.is_err() || actual > grant || actual != UI_FIXED_BYTES { return Err(UiFixedListAllocationError { allocated_bytes: actual, reason: "component byte candidate capacity differs from exact admission" }); }
    Ok(UiComponentCopyProgress { allocated_bytes: actual, ..progress(0) })
}

impl TypedCopy for UiValue {
    const DEPTH: usize = 1;
    fn empty_like(&self) -> Self { Self::Null }
    fn allocation(&self, _: &Self, _: &[usize]) -> Result<usize, &'static str> { Ok(0) }
    fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, _: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> {
        let (stage, _) = split(path)?;
        if let Self::Text(source) = self {
            if *stage == 0 {
                if work < size_of::<Self>() { return Ok(Default::default()); }
                *candidate = Self::Text(UiText::default());
                *stage = 1;
                return Ok(progress(size_of::<Self>()));
            }
            let Self::Text(target) = candidate else { return Err(error("typed text candidate changed variant")); };
            return source.copy_one(target, &mut [], byte_candidate, 0, work);
        }
        if work < size_of::<Self>() { return Ok(Default::default()); }
        let copied = match self {
            Self::Null => Self::Null,
            Self::Bool(value) => Self::Bool(*value),
            Self::Number(value) => Self::Number(*value),
            Self::List(_) | Self::Map(_) => {
                let mut arena = match UI_VALUE_ARENA.try_lock() {
                    Ok(arena) => arena,
                    Err(std::sync::TryLockError::WouldBlock) => return Ok(Default::default()),
                    Err(std::sync::TryLockError::Poisoned(_)) => return Err(error("typed copy value arena is poisoned")),
                };
                arena.try_clone_value(self).ok_or_else(|| error("typed copy exact value alias admission failed"))?
            }
            Self::Text(_) => unreachable!(),
        };
        *candidate = copied;
        Ok(UiComponentCopyProgress { complete: true, ..progress(size_of::<Self>()) })
    }
}

impl<T: TypedCopy> TypedCopy for Option<T> {
    const DEPTH: usize = 1 + T::DEPTH;
    fn empty_like(&self) -> Self { None }
    fn allocation(&self, candidate: &Self, path: &[usize]) -> Result<usize, &'static str> {
        let (_, path) = read_path(path)?;
        match (self, candidate) { (Some(source), Some(target)) => source.allocation(target, path), _ => Ok(0) }
    }
    fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> {
        let Some(source) = self else { return Ok(done()); };
        let (stage, path) = split(path)?;
        if *stage == 0 {
            if work < size_of::<T>() { return Ok(Default::default()); }
            *candidate = Some(source.empty_like());
            *stage = 1;
            return Ok(progress(size_of::<T>()));
        }
        source.copy_one(candidate.as_mut().ok_or_else(|| error("typed copy optional candidate is missing"))?, path, byte_candidate, allocation, work)
    }
}

impl<A: TypedCopy, B: TypedCopy> TypedCopy for (A, B) {
    const DEPTH: usize = 1 + maximum(&[A::DEPTH, B::DEPTH]);
    fn empty_like(&self) -> Self { (self.0.empty_like(), self.1.empty_like()) }
    fn allocation(&self, candidate: &Self, path: &[usize]) -> Result<usize, &'static str> {
        let (index, path) = read_path(path)?;
        match index { 0 => self.0.allocation(&candidate.0, path), 1 => self.1.allocation(&candidate.1, path), _ => Ok(0) }
    }
    fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> {
        let (index, path) = split(path)?;
        match *index { 0 => field(&self.0, &mut candidate.0, index, path, 2, byte_candidate, allocation, work), 1 => field(&self.1, &mut candidate.1, index, path, 2, byte_candidate, allocation, work), _ => Ok(done()) }
    }
}

impl<T: TypedCopy, const N: usize> TypedCopy for UiFixedList<T, N> {
    const DEPTH: usize = 1 + T::DEPTH;
    fn empty_like(&self) -> Self { Self::default() }
    fn allocation(&self, candidate: &Self, path: &[usize]) -> Result<usize, &'static str> {
        let (index, path) = read_path(path)?;
        let Some(source) = self.get(index) else { return Ok(0); };
        match candidate.get(index) { Some(target) => source.allocation(target, path), None => candidate.next_allocation_bytes() }
    }
    fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> {
        let (index, path) = split(path)?;
        let Some(source) = self.get(*index) else { return Ok(done()); };
        if candidate.get(*index).is_none() {
            if !candidate.has_reserved_slot() {
                let step = candidate.try_reserve_one(allocation)?;
                return Ok(UiComponentCopyProgress { progressed: step.progressed, allocated_bytes: step.allocated_bytes, ..Default::default() });
            }
            if work < size_of::<T>() { return Ok(Default::default()); }
            let mut empty = Some(source.empty_like());
            let step = candidate.try_place_reserved(&mut empty, work).map_err(error)?;
            if empty.is_some() { return Err(error("typed copy reserved placement rejected its exact empty owner")); }
            return Ok(UiComponentCopyProgress { progressed: step.progressed, copied_bytes: step.placed_bytes, ..Default::default() });
        }
        let mut step = source.copy_one(candidate.get_mut(*index).unwrap(), path, byte_candidate, allocation, work)?;
        if step.complete { *index += 1; path.fill(0); }
        step.complete = *index == self.len();
        Ok(step)
    }
}

impl<T: TypedCopy> TypedCopy for UiFixedMap<T> {
    const DEPTH: usize = <UiFixedList<(UiText, T)> as TypedCopy>::DEPTH;
    fn empty_like(&self) -> Self { Self { entries: UiFixedList::default() } }
    fn allocation(&self, candidate: &Self, path: &[usize]) -> Result<usize, &'static str> { self.entries.allocation(&candidate.entries, path) }
    fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> { self.entries.copy_one(&mut candidate.entries, path, byte_candidate, allocation, work) }
}

macro_rules! wrapper {
    ($($type:ty),*) => {$(impl TypedCopy for $type {
        fn empty_like(&self) -> Self { Self(self.0.empty_like()) }
        fn allocation(&self, candidate: &Self, path: &[usize]) -> Result<usize, &'static str> { self.0.allocation(&candidate.0, path) }
        fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> { self.0.copy_one(&mut candidate.0, path, byte_candidate, allocation, work) }
    })*};
}
wrapper!(Label, SurfaceId);
include!("../🧬️typed/🦀️.rs");
ui_typed_field_catalog!(typed_fields);

macro_rules! variants {
    ($type:ty { $($variant:ident : $props:ty),* }) => {
        impl TypedCopy for $type {
            const DEPTH: usize = maximum(&[$(<$props as TypedCopy>::DEPTH),*]);
            fn empty_like(&self) -> Self { match self { $(Self::$variant(source) => Self::$variant(source.empty_like()),)* } }
            fn allocation(&self, candidate: &Self, path: &[usize]) -> Result<usize, &'static str> {
                match (self, candidate) { $((Self::$variant(source), Self::$variant(target)) => source.allocation(target, path),)* _ => Err("typed copy candidate variant differs") }
            }
            fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> {
                match (self, candidate) { $((Self::$variant(source), Self::$variant(target)) => source.copy_one(target, path, byte_candidate, allocation, work),)* _ => Err(error("typed copy candidate variant differs")) }
            }
        }
    };
}
variants!(Component { Container:ContainerProps, Text:TextProps, Button:ButtonProps, Separator:SeparatorProps, Input:InputProps, Select:SelectProps, Toggle:ToggleProps, KeyValueList:KeyValueListProps, Slider:SliderProps, NumberStepper:NumberStepperProps, Ring:RingProps, IconSelect:IconSelectProps, Tree:TreeProps, TreeSection:TreeSectionProps, TreeItem:TreeItemProps, Image:ImageProps, Surface:SurfaceProps, Extension:ExtensionProps });
variants!(LayoutSpec { Leaf:LeafLayout, Stack:StackLayout, Grid:GridLayout, Overlay:OverlayLayout, Scroll:ScrollLayout, Absolute:AbsoluteLayout });
const _: () = assert!(<UiSnapshot as TypedCopy>::DEPTH <= COPY_DEPTH);
//#endregion 🧭️TypedCopy

//#region 🎟️ExactRootOwner
struct OwnedComponent { candidate: Option<Component>, source: Option<Component>, byte_candidate: Vec<u8> }

pub struct UiComponentCopy {
    owned: ManuallyDrop<OwnedComponent>,
    path: [usize; COPY_DEPTH],
    retirement: UiTypedRetirementCursor,
    complete: bool,
    closing: bool,
}

impl UiComponentCopy {
    pub fn new(source: Component) -> Self { Self { owned: ManuallyDrop::new(OwnedComponent { source: Some(source), candidate: None, byte_candidate: Vec::new() }), path: [0; COPY_DEPTH], retirement: Default::default(), complete: false, closing: false } }
    pub fn source(&self) -> Option<&Component> { if self.closing { None } else { self.owned.source.as_ref() } }
    pub fn candidate(&self) -> Option<&Component> { if self.closing || !self.complete { None } else { self.owned.candidate.as_ref() } }
    pub fn next_allocation_bytes(&self) -> Result<usize, &'static str> {
        if self.closing { return Err("component copy is closing"); }
        if self.owned.byte_candidate.capacity() != 0 { return Ok(0); }
        match (&self.owned.source, &self.owned.candidate) { (Some(source), Some(target)) if !self.complete => source.allocation(target, &self.path), _ => Ok(0) }
    }
    /// 🎟️ Admits only the next backing allocation; no initialized payload bytes share this turn.
    pub fn reserve_next(&mut self, allocation: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> {
        let requested = self.next_allocation_bytes().map_err(error)?;
        if requested == 0 || allocation < requested { return Ok(Default::default()); }
        let owned = &mut *self.owned;
        let source = owned.source.as_ref().ok_or_else(|| error("component allocation source is missing"))?;
        let candidate = owned.candidate.as_mut().ok_or_else(|| error("component allocation candidate is missing"))?;
        source.copy_one(candidate, &mut self.path, &mut owned.byte_candidate, allocation, 0)
    }
    pub fn advance(&mut self, items: usize, allocation: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> {
        if self.closing { return Err(error("component copy is closing")); }
        if self.complete { return Ok(done()); }
        if items == 0 || work == 0 { return Ok(Default::default()); }
        let owned = &mut *self.owned;
        let source = owned.source.as_ref().ok_or_else(|| error("component copy source is missing"))?;
        if owned.candidate.is_none() {
            if work < size_of::<Component>() { return Ok(Default::default()); }
            owned.candidate = Some(source.empty_like());
            return Ok(progress(size_of::<Component>()));
        }
        let step = source.copy_one(owned.candidate.as_mut().unwrap(), &mut self.path, &mut owned.byte_candidate, allocation, work)?;
        self.complete = step.complete;
        Ok(step)
    }
    pub fn take_completed(&mut self) -> Option<(Component, Component)> {
        if !self.complete || self.closing || self.owned.source.is_none() || self.owned.candidate.is_none() { return None; }
        let owned = &mut *self.owned;
        Some((owned.source.take()?, owned.candidate.take()?))
    }
    /// 📤️ One exact completed root is transferred only after its independent move grant.
    pub fn take_completed_source_with_grant(&mut self, bytes: usize) -> Option<Component> {
        if !self.complete || self.closing || bytes < size_of::<Component>() { return None; }
        self.owned.source.take()
    }
    /// 📤️ Candidate transfer leaves the source owner available for a separate return or retirement.
    pub fn take_completed_candidate_with_grant(&mut self, bytes: usize) -> Option<Component> {
        if !self.complete || self.closing || bytes < size_of::<Component>() { return None; }
        self.owned.candidate.take()
    }
    pub fn close_step(&mut self, items: usize, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        if self.terminal_is_empty() { return Ok(UiValueRetirementStep { complete: true, ..Default::default() }); }
        if items == 0 || bytes == 0 { return Ok(Default::default()); }
        self.closing = true;
        self.retirement.advance(&mut *self.owned, items, bytes)
    }
    pub fn terminal_is_empty(&self) -> bool { self.owned.source.is_none() && self.owned.candidate.is_none() && self.owned.byte_candidate.capacity() == 0 && (!self.closing || self.retirement.terminal_is_empty()) }
}

impl UiTypedRetire for OwnedComponent {
    const DEPTH: usize = 1 + <Option<Component> as UiTypedRetire>::DEPTH;
    fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        let (field, path) = path.split_first_mut().ok_or("component copy retirement exceeds schema depth")?;
        if *field == 0 {
            if !self.byte_candidate.is_empty() {
                let released = self.byte_candidate.len().min(bytes);
                self.byte_candidate.truncate(self.byte_candidate.len() - released);
                return Ok(UiValueRetirementStep { progressed: true, released_bytes: released, ..Default::default() });
            }
            let released = self.byte_candidate.capacity() != 0;
            drop(std::mem::take(&mut self.byte_candidate));
            *field = 1;
            return Ok(UiValueRetirementStep { progressed: true, released_items: usize::from(released), ..Default::default() });
        }
        let mut step = match *field { 1 => self.candidate.retire_typed(path, value, bytes)?, 2 => self.source.retire_typed(path, value, bytes)?, _ => return Ok(UiValueRetirementStep { complete: true, progressed: true, ..Default::default() }) };
        if step.complete { *field += 1; path.fill(0); }
        step.complete = *field == 3;
        Ok(step)
    }
}

impl Drop for UiComponentCopy {
    fn drop(&mut self) { if !self.terminal_is_empty() && !std::thread::panicking() { panic!("component copy requires exact source and candidate retirement"); } }
}
//#endregion 🎟️ExactRootOwner

#[cfg(test)]
#[path = "🧪️bytes.rs"]
mod byte_tests;
