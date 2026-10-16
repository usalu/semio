//! 📋️ Retained typed copying with distinct allocation and initialized-byte admission.

use crate::*;
use std::mem::size_of;

//#region 🧭️TypedCopy
pub const COPY_DEPTH: usize = 16;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiComponentCopyProgress {
    pub complete: bool,
    pub progressed: bool,
    pub allocated_bytes: usize,
    pub copied_bytes: usize,
}

pub fn error(reason: &'static str) -> PagedListAllocationError {
    PagedListAllocationError { allocated_bytes: 0, kind: PagedListRefusalKind::InvariantViolated, reason }
}
pub fn done() -> UiComponentCopyProgress {
    UiComponentCopyProgress { complete: true, progressed: true, ..Default::default() }
}
pub fn progress(bytes: usize) -> UiComponentCopyProgress {
    UiComponentCopyProgress { progressed: true, copied_bytes: bytes, ..Default::default() }
}
pub fn split(path: &mut [usize]) -> Result<(&mut usize, &mut [usize]), PagedListAllocationError> {
    path.split_first_mut().ok_or_else(|| error("typed copy exceeds schema depth"))
}
pub fn read_path(path: &[usize]) -> Result<(usize, &[usize]), PagedListError> {
    path.split_first().map(|(index, rest)| (*index, rest)).ok_or(PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "typed copy exceeds schema depth" })
}
pub const fn maximum(depths: &[usize]) -> usize {
    let mut result = 0;
    let mut index = 0;
    while index < depths.len() {
        if depths[index] > result {
            result = depths[index];
        }
        index += 1;
    }
    result
}

pub trait TypedCopy: Sized {
    const DEPTH: usize = 0;
    fn empty_like(&self) -> Self;
    fn allocation(&self, candidate: &Self, path: &[usize]) -> Result<usize, PagedListError>;
    fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usize) -> Result<UiComponentCopyProgress, PagedListAllocationError>;
}

#[derive(Clone, Copy)]
pub struct CopyGrant {
    pub allocation: usize,
    pub work: usize,
}

pub fn field<T: TypedCopy>(source: &T, candidate: &mut T, index: &mut usize, path: &mut [usize], count: usize, byte_candidate: &mut Vec<u8>, grant: CopyGrant) -> Result<UiComponentCopyProgress, PagedListAllocationError> {
    let mut step = source.copy_one(candidate, path, byte_candidate, grant.allocation, grant.work)?;
    if step.complete {
        *index += 1;
        path.fill(0);
    }
    step.complete = *index == count;
    Ok(step)
}

#[macro_export]
macro_rules! ui_copy_fields {
    ($type:ty {}) => {
        impl $crate::copy_protocol::TypedCopy for $type {
            const DEPTH: usize = 1;
            fn empty_like(&self) -> Self { Self {} }
            fn allocation(&self, _: &Self, path: &[usize]) -> Result<usize, $crate::PagedListError> { $crate::copy_protocol::read_path(path)?; Ok(0) }
            fn copy_one(&self, _: &mut Self, path: &mut [usize], _: &mut Vec<u8>, _: usize, _: usize) -> Result<$crate::copy_protocol::UiComponentCopyProgress, $crate::PagedListAllocationError> { $crate::copy_protocol::split(path)?; Ok($crate::copy_protocol::done()) }
        }
    };
    ($type:ty { $($index:literal => $field:tt : $field_type:ty),* $(,)? }) => {
        impl $crate::copy_protocol::TypedCopy for $type {
            const DEPTH: usize = 1 + $crate::copy_protocol::maximum(&[$(<$field_type as $crate::copy_protocol::TypedCopy>::DEPTH),*]);
            fn empty_like(&self) -> Self { Self { $($field: self.$field.empty_like()),* } }
            fn allocation(&self, candidate: &Self, path: &[usize]) -> Result<usize, $crate::PagedListError> {
                let (index, path) = $crate::copy_protocol::read_path(path)?;
                match index { $($index => self.$field.allocation(&candidate.$field, path),)* _ => Ok(0) }
            }
            fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usize) -> Result<$crate::copy_protocol::UiComponentCopyProgress, $crate::PagedListAllocationError> {
                let Self { $($field: _),* } = self;
                let (index, path) = $crate::copy_protocol::split(path)?;
                let count = 0 $(+ { let _ = stringify!($field); 1 })*;
                match *index { $($index => $crate::copy_protocol::field(&self.$field, &mut candidate.$field, index, path, count, byte_candidate, $crate::copy_protocol::CopyGrant { allocation, work }),)* _ => Ok($crate::copy_protocol::done()) }
            }
        }
    };
}

#[macro_export]
macro_rules! ui_copy_scalar {
    ($($type:ty),* $(,)?) => {$(
        impl $crate::copy_protocol::TypedCopy for $type {
            fn empty_like(&self) -> Self { *self }
            fn allocation(&self, _: &Self, _: &[usize]) -> Result<usize, $crate::PagedListError> { Ok(0) }
            fn copy_one(&self, candidate: &mut Self, _: &mut [usize], _: &mut Vec<u8>, _: usize, work: usize) -> Result<$crate::copy_protocol::UiComponentCopyProgress, $crate::PagedListAllocationError> {
                if work < ::std::mem::size_of::<Self>() { return Ok(Default::default()); }
                *candidate = *self;
                Ok($crate::copy_protocol::UiComponentCopyProgress { complete: true, ..$crate::copy_protocol::progress(::std::mem::size_of::<Self>()) })
            }
        }
    )*};
}
crate::ui_copy_scalar!(bool,u16,u32,u64,f64,UiNodeId,UiRevision);

impl TypedCopy for UiText {
    fn empty_like(&self) -> Self {
        Self::default()
    }
    fn allocation(&self, _: &Self, _: &[usize]) -> Result<usize, PagedListError> {
        Ok(0)
    }
    fn copy_one(&self, candidate: &mut Self, _: &mut [usize], _: &mut Vec<u8>, _: usize, work: usize) -> Result<UiComponentCopyProgress, PagedListAllocationError> {
        let (complete,bytes)=candidate.copy_prefix_from(self,work).map_err(error)?;
        Ok(UiComponentCopyProgress { complete, ..progress(bytes) })
    }
}

impl TypedCopy for UiFixedBytes {
    fn empty_like(&self) -> Self {
        Self { bytes: Box::default(), len: 0 }
    }
    fn allocation(&self, candidate: &Self, _: &[usize]) -> Result<usize, PagedListError> {
        Ok(if self.is_empty() || !candidate.bytes.is_empty() { 0 } else { UI_FIXED_BYTES })
    }
    fn copy_one(&self, candidate: &mut Self, _: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usize) -> Result<UiComponentCopyProgress, PagedListAllocationError> {
        if self.is_empty() || !candidate.bytes.is_empty() {
            return Ok(done());
        }
        if byte_candidate.capacity() == 0 {
            return reserve_byte_candidate(byte_candidate, allocation, |owner, requested| owner.try_reserve_exact(requested).map_err(|_| ()));
        }
        if byte_candidate.capacity() != UI_FIXED_BYTES {
            return Err(error("component byte candidate requires retirement after capacity rejection"));
        }
        let start = byte_candidate.len();
        if start < UI_FIXED_BYTES {
            let end = if start < self.len() { self.len() } else { UI_FIXED_BYTES };
            let bytes = (end - start).min(work);
            if start < self.len() {
                byte_candidate.extend_from_slice(&self.bytes[start..start + bytes]);
            } else {
                byte_candidate.resize(start + bytes, 0);
            }
            return Ok(progress(bytes));
        }
        if work < size_of::<Self>() {
            return Ok(Default::default());
        }
        if byte_candidate.len() != byte_candidate.capacity() {
            return Err(error("component byte candidate transfer would reallocate"));
        }
        candidate.bytes = std::mem::take(byte_candidate).into_boxed_slice();
        candidate.len = self.len;
        Ok(UiComponentCopyProgress { complete: true, ..progress(size_of::<Self>()) })
    }
}

pub fn reserve_byte_candidate(candidate: &mut Vec<u8>, grant: usize, allocate: impl FnOnce(&mut Vec<u8>, usize) -> Result<(), ()>) -> Result<UiComponentCopyProgress, PagedListAllocationError> {
    if grant < UI_FIXED_BYTES {
        return Ok(Default::default());
    }
    if candidate.capacity() != 0 || !candidate.is_empty() {
        return Err(error("component byte candidate allocation requires empty ownership"));
    }
    let allocated = allocate(candidate, UI_FIXED_BYTES);
    let actual = candidate.capacity();
    if allocated.is_err() || actual > grant || actual != UI_FIXED_BYTES {
        return Err(PagedListAllocationError { allocated_bytes: actual, kind: if allocated.is_err() { PagedListRefusalKind::AllocationFailed } else { PagedListRefusalKind::InvariantViolated }, reason: "component byte candidate capacity differs from exact admission" });
    }
    Ok(UiComponentCopyProgress { allocated_bytes: actual, ..progress(0) })
}

impl TypedCopy for UiValue {
    const DEPTH: usize = 1;
    fn empty_like(&self) -> Self {
        Self::Null
    }
    fn allocation(&self, _: &Self, _: &[usize]) -> Result<usize, PagedListError> {
        Ok(0)
    }
    fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, _: usize, work: usize) -> Result<UiComponentCopyProgress, PagedListAllocationError> {
        let (stage, _) = split(path)?;
        if let Self::Text(source) = self {
            if *stage == 0 {
                if work < size_of::<Self>() {
                    return Ok(Default::default());
                }
                *candidate = Self::Text(UiText::default());
                *stage = 1;
                return Ok(progress(size_of::<Self>()));
            }
            let Self::Text(target) = candidate else {
                return Err(error("typed text candidate changed variant"));
            };
            return source.copy_one(target, &mut [], byte_candidate, 0, work);
        }
        if work < size_of::<Self>() {
            return Ok(Default::default());
        }
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

impl TypedCopy for UiMap {
    const DEPTH: usize = 1;
    fn empty_like(&self) -> Self {
        Self::default()
    }
    fn allocation(&self, _: &Self, _: &[usize]) -> Result<usize, PagedListError> {
        Ok(0)
    }
    fn copy_one(&self, candidate: &mut Self, _: &mut [usize], _: &mut Vec<u8>, _: usize, work: usize) -> Result<UiComponentCopyProgress, PagedListAllocationError> {
        if work < size_of::<Self>() {
            return Ok(Default::default());
        }
        let mut arena = match UI_VALUE_ARENA.try_lock() {
            Ok(arena) => arena,
            Err(std::sync::TryLockError::WouldBlock) => return Ok(Default::default()),
            Err(std::sync::TryLockError::Poisoned(_)) => return Err(error("typed copy value arena is poisoned")),
        };
        *candidate = self.try_clone_in(&mut arena).ok_or_else(|| error("typed copy exact map alias admission failed"))?;
        Ok(UiComponentCopyProgress { complete: true, ..progress(size_of::<Self>()) })
    }
}

impl<T: TypedCopy> TypedCopy for Option<T> {
    const DEPTH: usize = 1 + T::DEPTH;
    fn empty_like(&self) -> Self {
        None
    }
    fn allocation(&self, candidate: &Self, path: &[usize]) -> Result<usize, PagedListError> {
        let (_, path) = read_path(path)?;
        match (self, candidate) {
            (Some(source), Some(target)) => source.allocation(target, path),
            _ => Ok(0),
        }
    }
    fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usize) -> Result<UiComponentCopyProgress, PagedListAllocationError> {
        let Some(source) = self else {
            return Ok(done());
        };
        let (stage, path) = split(path)?;
        if *stage == 0 {
            if work < size_of::<T>() {
                return Ok(Default::default());
            }
            *candidate = Some(source.empty_like());
            *stage = 1;
            return Ok(progress(size_of::<T>()));
        }
        source.copy_one(candidate.as_mut().ok_or_else(|| error("typed copy optional candidate is missing"))?, path, byte_candidate, allocation, work)
    }
}

impl<A: TypedCopy, B: TypedCopy> TypedCopy for (A, B) {
    const DEPTH: usize = 1 + maximum(&[A::DEPTH, B::DEPTH]);
    fn empty_like(&self) -> Self {
        (self.0.empty_like(), self.1.empty_like())
    }
    fn allocation(&self, candidate: &Self, path: &[usize]) -> Result<usize, PagedListError> {
        let (index, path) = read_path(path)?;
        match index {
            0 => self.0.allocation(&candidate.0, path),
            1 => self.1.allocation(&candidate.1, path),
            _ => Ok(0),
        }
    }
    fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usize) -> Result<UiComponentCopyProgress, PagedListAllocationError> {
        let (index, path) = split(path)?;
        match *index {
            0 => field(&self.0, &mut candidate.0, index, path, 2, byte_candidate, CopyGrant { allocation, work }),
            1 => field(&self.1, &mut candidate.1, index, path, 2, byte_candidate, CopyGrant { allocation, work }),
            _ => Ok(done()),
        }
    }
}

impl<T: TypedCopy, const N: usize> TypedCopy for UiFixedList<T, N> {
    const DEPTH: usize = 1 + T::DEPTH;
    fn empty_like(&self) -> Self {
        Self::default()
    }
    fn allocation(&self, candidate: &Self, path: &[usize]) -> Result<usize, PagedListError> {
        let (index, path) = read_path(path)?;
        let Some(source) = self.get(index) else {
            return Ok(0);
        };
        match candidate.get(index) {
            Some(target) => source.allocation(target, path),
            None => candidate.next_allocation_bytes(),
        }
    }
    fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usize) -> Result<UiComponentCopyProgress, PagedListAllocationError> {
        let (index, path) = split(path)?;
        let Some(source) = self.get(*index) else {
            return Ok(done());
        };
        if candidate.get(*index).is_none() {
            if !candidate.has_reserved_slot() {
                let step = candidate.try_reserve_one(allocation)?;
                return Ok(UiComponentCopyProgress { progressed: step.progressed, allocated_bytes: step.allocated_bytes, ..Default::default() });
            }
            if work < size_of::<T>() {
                return Ok(Default::default());
            }
            let mut empty = Some(source.empty_like());
            let step = candidate.try_place_reserved(&mut empty, work).map_err(|refusal| PagedListAllocationError { allocated_bytes: 0, kind: refusal.kind, reason: refusal.reason })?;
            if empty.is_some() {
                return Err(error("typed copy reserved placement rejected its exact empty owner"));
            }
            return Ok(UiComponentCopyProgress { progressed: step.progressed, copied_bytes: step.placed_bytes, ..Default::default() });
        }
        let mut step = source.copy_one(candidate.get_mut(*index).unwrap(), path, byte_candidate, allocation, work)?;
        if step.complete {
            *index += 1;
            path.fill(0);
        }
        step.complete = *index == self.len();
        Ok(step)
    }
}

impl<T: TypedCopy> TypedCopy for UiFixedMap<T> {
    const DEPTH: usize = <UiFixedList<(UiText, T)> as TypedCopy>::DEPTH;
    fn empty_like(&self) -> Self {
        Self { entries: UiFixedList::default() }
    }
    fn allocation(&self, candidate: &Self, path: &[usize]) -> Result<usize, PagedListError> {
        self.entries.allocation(&candidate.entries, path)
    }
    fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usize) -> Result<UiComponentCopyProgress, PagedListAllocationError> {
        self.entries.copy_one(&mut candidate.entries, path, byte_candidate, allocation, work)
    }
}

macro_rules! wrapper {
    ($($type:ty),*) => {$(impl TypedCopy for $type {
        fn empty_like(&self) -> Self { Self(self.0.empty_like()) }
        fn allocation(&self, candidate: &Self, path: &[usize]) -> Result<usize, PagedListError> { self.0.allocation(&candidate.0, path) }
        fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usize) -> Result<UiComponentCopyProgress, PagedListAllocationError> { self.0.copy_one(&mut candidate.0, path, byte_candidate, allocation, work) }
    })*};
}
wrapper!(SurfaceId);
