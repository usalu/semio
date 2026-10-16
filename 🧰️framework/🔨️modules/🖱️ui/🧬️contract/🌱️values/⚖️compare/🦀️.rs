//! ⚖️ Exact retained typed equality without whole-value clones or blocking arena access.

use crate::*;

//#region 🧭️TypedComparison
pub const COMPARE_DEPTH: usize = 16;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiComponentCompareProgress {
    pub complete: bool,
    pub progressed: bool,
    pub compared_bytes: usize,
    pub equal: Option<bool>,
}

pub fn result(equal: bool) -> UiComponentCompareProgress {
    UiComponentCompareProgress { complete: true, progressed: true, equal: Some(equal), compared_bytes: 0 }
}
pub fn progress() -> UiComponentCompareProgress {
    UiComponentCompareProgress { progressed: true, ..Default::default() }
}
pub fn split(path: &mut [usize]) -> Result<(&mut usize, &mut [usize]), &'static str> {
    path.split_first_mut().ok_or("typed comparison exceeds schema depth")
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

pub fn compare_bytes(left: &[u8], right: &[u8], position: &mut usize, remembered: &mut u8, grant: usize) -> UiComponentCompareProgress {
    if left.len() != right.len() {
        return result(false);
    }
    let mut compared = 0;
    while compared < grant && *position / 2 < left.len() {
        let offset = *position / 2;
        let equal = if (*position).is_multiple_of(2) {
            *remembered = left[offset];
            true
        } else {
            *remembered == right[offset]
        };
        *position += 1;
        compared += 1;
        if !equal {
            return UiComponentCompareProgress { compared_bytes: compared, ..result(false) };
        }
    }
    let complete = *position / 2 == left.len();
    UiComponentCompareProgress { complete, progressed: compared != 0 || complete, compared_bytes: compared, equal: complete.then_some(true) }
}

pub trait TypedCompare {
    const DEPTH: usize = 0;
    fn compare_one(&self, right: &Self, path: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiComponentCompareProgress, &'static str>;
}

pub fn field<T: TypedCompare>(left: &T, right: &T, index: &mut usize, path: &mut [usize], values: &mut ValueComparison, count: usize, bytes: usize) -> Result<UiComponentCompareProgress, &'static str> {
    let mut step = left.compare_one(right, path, values, bytes)?;
    if step.equal == Some(false) {
        return Ok(step);
    }
    if step.complete {
        *index += 1;
        path.fill(0);
    }
    step.complete = *index == count;
    step.equal = step.complete.then_some(true);
    Ok(step)
}

#[macro_export]
macro_rules! ui_compare_fields {
    ($type:ty {}) => {
        impl $crate::compare_protocol::TypedCompare for $type {
            const DEPTH: usize = 1;
            fn compare_one(&self, _: &Self, path: &mut [usize], _: &mut $crate::compare_protocol::ValueComparison, _: usize) -> Result<$crate::compare_protocol::UiComponentCompareProgress, &'static str> { $crate::compare_protocol::split(path)?; Ok($crate::compare_protocol::result(true)) }
        }
    };
    ($type:ty { $($index:literal => $field:tt : $field_type:ty),* $(,)? }) => {
        impl $crate::compare_protocol::TypedCompare for $type {
            const DEPTH: usize = 1 + $crate::compare_protocol::maximum(&[$(<$field_type as $crate::compare_protocol::TypedCompare>::DEPTH),*]);
            fn compare_one(&self, right: &Self, path: &mut [usize], values: &mut $crate::compare_protocol::ValueComparison, bytes: usize) -> Result<$crate::compare_protocol::UiComponentCompareProgress, &'static str> {
                let Self { $($field: _),* } = self;
                let (index, path) = $crate::compare_protocol::split(path)?;
                let count = 0 $(+ { let _ = stringify!($field); 1 })*;
                match *index { $($index => $crate::compare_protocol::field(&self.$field, &right.$field, index, path, values, count, bytes),)* _ => Ok($crate::compare_protocol::result(true)) }
            }
        }
    };
}

#[macro_export]
macro_rules! ui_compare_scalar {
    ($($type:ty),* $(,)?) => {$(impl $crate::compare_protocol::TypedCompare for $type {
        fn compare_one(&self, right: &Self, _: &mut [usize], _: &mut $crate::compare_protocol::ValueComparison, _: usize) -> Result<$crate::compare_protocol::UiComponentCompareProgress, &'static str> { Ok($crate::compare_protocol::result(self == right)) }
    })*};
}
crate::ui_compare_scalar!(bool,u16,u32,u64,f64,UiNodeId,UiRevision);

macro_rules! byte_field {
    ($type:ty, $slice:ident) => {
        impl TypedCompare for $type {
            const DEPTH: usize = 2;
            fn compare_one(&self, right: &Self, path: &mut [usize], _: &mut ValueComparison, bytes: usize) -> Result<UiComponentCompareProgress, &'static str> {
                let (position, rest) = split(path)?;
                let remembered = rest.first_mut().ok_or("typed byte comparison lacks exact operand slot")?;
                let mut byte = *remembered as u8;
                let step = compare_bytes(self.$slice(), right.$slice(), position, &mut byte, bytes);
                *remembered = usize::from(byte);
                Ok(step)
            }
        }
    };
}
byte_field!(UiText, comparison_bytes);
byte_field!(UiFixedBytes, as_slice);

impl<T: TypedCompare> TypedCompare for Option<T> {
    const DEPTH: usize = 1 + T::DEPTH;
    fn compare_one(&self, right: &Self, path: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiComponentCompareProgress, &'static str> {
        let (_, path) = split(path)?;
        match (self, right) {
            (Some(left), Some(right)) => left.compare_one(right, path, values, bytes),
            (None, None) => Ok(result(true)),
            _ => Ok(result(false)),
        }
    }
}
impl<A: TypedCompare, B: TypedCompare> TypedCompare for (A, B) {
    const DEPTH: usize = 1 + maximum(&[A::DEPTH, B::DEPTH]);
    fn compare_one(&self, right: &Self, path: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiComponentCompareProgress, &'static str> {
        let (index, path) = split(path)?;
        match *index {
            0 => field(&self.0, &right.0, index, path, values, 2, bytes),
            1 => field(&self.1, &right.1, index, path, values, 2, bytes),
            _ => Ok(result(true)),
        }
    }
}
impl<T: TypedCompare, const N: usize> TypedCompare for UiFixedList<T, N> {
    const DEPTH: usize = 1 + T::DEPTH;
    fn compare_one(&self, right: &Self, path: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiComponentCompareProgress, &'static str> {
        if self.len() != right.len() {
            return Ok(result(false));
        }
        let (index, path) = split(path)?;
        match (self.get(*index), right.get(*index)) {
            (Some(left), Some(right)) => field(left, right, index, path, values, self.len(), bytes),
            (None, None) => Ok(result(true)),
            _ => Err("typed comparison lost an exact list ordinal"),
        }
    }
}
impl<T: TypedCompare> TypedCompare for UiFixedMap<T> {
    const DEPTH: usize = <UiFixedList<(UiText, T)> as TypedCompare>::DEPTH;
    fn compare_one(&self, right: &Self, path: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiComponentCompareProgress, &'static str> {
        self.entries.compare_one(&right.entries, path, values, bytes)
    }
}
macro_rules! wrapper {
    ($($type:ty),*) => {$(impl TypedCompare for $type {
        const DEPTH: usize = <UiText as TypedCompare>::DEPTH;
        fn compare_one(&self, right: &Self, path: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiComponentCompareProgress, &'static str> { self.0.compare_one(&right.0, path, values, bytes) }
    })*};
}
wrapper!(SurfaceId);
//#region 🌳️BorrowedValueTraversal
#[derive(Clone, Copy)]
pub(super) struct ValueFrame {
    left: u16,
    right: u16,
    position: u16,
    remembered: u8,
    value_phase: bool,
}
impl ValueFrame {
    const EMPTY: Self = Self { left: u16::MAX, right: u16::MAX, position: 0, remembered: 0, value_phase: false };
    pub(super) fn checked_page(index: usize) -> Result<u16, &'static str> {
        if index == UI_VALUE_NONE {
            return Ok(u16::MAX);
        }
        if index >= UI_VALUE_AGGREGATE_ITEMS {
            return Err("comparison page exceeds arena domain");
        }
        u16::try_from(index).map_err(|_| "comparison page exceeds fixed index")
    }
    pub(super) fn checked_position(position: usize) -> Result<u16, &'static str> {
        if position > 2 * UI_TEXT_MAX_BYTES {
            return Err("comparison position exceeds text domain");
        }
        u16::try_from(position).map_err(|_| "comparison position exceeds fixed offset")
    }
    fn new(left: usize, right: usize) -> Result<Self, &'static str> {
        Ok(Self { left: Self::checked_page(left)?, right: Self::checked_page(right)?, ..Self::EMPTY })
    }
}
const _: () = assert!(UI_VALUE_AGGREGATE_ITEMS < u16::MAX as usize);
const _: () = assert!(2 * UI_TEXT_MAX_BYTES <= u16::MAX as usize);

pub struct ValueComparison {
    frames: [ValueFrame; UI_VALUE_NESTING_DEPTH],
    length: usize,
    position: usize,
    remembered: u8,
}
impl Default for ValueComparison {
    fn default() -> Self {
        Self { frames: [ValueFrame::EMPTY; UI_VALUE_NESTING_DEPTH], length: 0, position: 0, remembered: 0 }
    }
}

enum ValueUnit {
    Progress(UiComponentCompareProgress),
    Children(usize, usize),
}

fn collection_heads(left: Option<UiCollectionHandle>, left_len: usize, right: Option<UiCollectionHandle>, right_len: usize, arena: &UiValueArena) -> Result<ValueUnit, &'static str> {
    if left_len != right_len {
        return Ok(ValueUnit::Progress(result(false)));
    }
    let head = |handle, len| -> Result<usize, &'static str> {
        match handle {
            None if len == 0 => Ok(UI_VALUE_NONE),
            Some(handle) => {
                let root = arena.collection(handle).ok_or("typed comparison collection authority is stale")?;
                if root.retiring || root.items != len {
                    return Err("typed comparison collection is not immutable");
                }
                Ok(root.head)
            }
            _ => Err("typed comparison collection root is missing"),
        }
    };
    Ok(ValueUnit::Children(head(left, left_len)?, head(right, right_len)?))
}

fn value_unit(left: &UiValue, right: &UiValue, position: &mut usize, remembered: &mut u8, arena: Option<&UiValueArena>, bytes: usize) -> Result<ValueUnit, &'static str> {
    let step = match (left, right) {
        (UiValue::Null, UiValue::Null) => result(true),
        (UiValue::Bool(left), UiValue::Bool(right)) => result(left == right),
        (UiValue::Number(left), UiValue::Number(right)) => result(left == right),
        (UiValue::Text(left), UiValue::Text(right)) => compare_bytes(left.comparison_bytes(), right.comparison_bytes(), position, remembered, bytes),
        (UiValue::List(left), UiValue::List(right)) => return collection_heads(left.handle, left.len, right.handle, right.len, arena.ok_or("typed comparison list requires arena authority")?),
        (UiValue::Map(left), UiValue::Map(right)) => return collection_heads(left.handle, left.len, right.handle, right.len, arena.ok_or("typed comparison map requires arena authority")?),
        _ => result(false),
    };
    Ok(ValueUnit::Progress(step))
}

impl ValueComparison {
    pub fn reads_empty(&self)->bool{self.length==0}

    pub fn reset(&mut self) {
        self.length = 0;
        self.position = 0;
        self.remembered = 0;
    }
    fn push(&mut self, left: usize, right: usize) -> Result<(), &'static str> {
        let slot = self.frames.get_mut(self.length).ok_or("typed comparison exceeds admitted collection depth")?;
        *slot = ValueFrame::new(left, right)?;
        self.length += 1;
        Ok(())
    }
    fn advance(&mut self, left: &UiValue, right: &UiValue, bytes: usize) -> Result<UiComponentCompareProgress, &'static str> {
        let collection = matches!((left, right), (UiValue::List(_), UiValue::List(_)) | (UiValue::Map(_), UiValue::Map(_)));
        self.advance_from(collection, |position, remembered, arena| value_unit(left, right, position, remembered, arena, bytes), bytes)
    }
    /// 🗺️ The same traversal rooted at two bare [`UiMap`] handles (a [`RowTarget`]'s argument map).
    fn advance_map(&mut self, left: &UiMap, right: &UiMap, bytes: usize) -> Result<UiComponentCompareProgress, &'static str> {
        self.advance_from(true, |_, _, arena| collection_heads(left.handle, left.len, right.handle, right.len, arena.ok_or("typed comparison map requires arena authority")?), bytes)
    }
    fn advance_from(&mut self, collection: bool, root: impl FnOnce(&mut usize, &mut u8, Option<&UiValueArena>) -> Result<ValueUnit, &'static str>, bytes: usize) -> Result<UiComponentCompareProgress, &'static str> {
        if self.length == 0 && !collection {
            return match root(&mut self.position, &mut self.remembered, None)? {
                ValueUnit::Progress(step) => Ok(step),
                _ => Err("typed comparison unexpected collection"),
            };
        }
        let arena = match UI_VALUE_ARENA.try_lock() {
            Ok(arena) => arena,
            Err(std::sync::TryLockError::WouldBlock) => return Ok(Default::default()),
            Err(std::sync::TryLockError::Poisoned(_)) => return Err("typed comparison arena is poisoned"),
        };
        if self.length == 0 {
            return match root(&mut self.position, &mut self.remembered, Some(&arena))? {
                ValueUnit::Progress(step) => Ok(step),
                ValueUnit::Children(left, right) => {
                    self.push(left, right)?;
                    Ok(progress())
                }
            };
        }
        let index = self.length - 1;
        let mut frame = self.frames[index];
        if frame.left == u16::MAX || frame.right == u16::MAX {
            if frame.left != frame.right {
                return Ok(result(false));
            }
            self.length -= 1;
            return Ok(if self.length == 0 { result(true) } else { progress() });
        }
        let left_page = arena.pages.get(usize::from(frame.left)).ok_or("typed comparison left page missing")?;
        let right_page = arena.pages.get(usize::from(frame.right)).ok_or("typed comparison right page missing")?;
        let (left, right) = match (&left_page.value, &right_page.value) {
            (Some(UiPageValue::List(left)), Some(UiPageValue::List(right))) => (left, right),
            (Some(UiPageValue::Map(left_key, left)), Some(UiPageValue::Map(right_key, right))) => {
                if !frame.value_phase {
                    let mut position = usize::from(frame.position);
                    let mut step = compare_bytes(left_key.comparison_bytes(), right_key.comparison_bytes(), &mut position, &mut frame.remembered, bytes);
                    frame.position = ValueFrame::checked_position(position)?;
                    if step.equal == Some(true) {
                        frame.value_phase = true;
                        frame.position = 0;
                        frame.remembered = 0;
                        step.complete = false;
                        step.equal = None;
                    }
                    self.frames[index] = frame;
                    return Ok(step);
                }
                (left, right)
            }
            _ => return Err("typed comparison page kind changed under retained root"),
        };
        let mut position = usize::from(frame.position);
        let unit = value_unit(left, right, &mut position, &mut frame.remembered, Some(&arena), bytes)?;
        frame.position = ValueFrame::checked_position(position)?;
        let (step, children) = match unit {
            ValueUnit::Progress(step) if step.equal != Some(true) => {
                self.frames[index] = frame;
                return Ok(step);
            }
            ValueUnit::Progress(mut step) => {
                step.complete = false;
                step.equal = None;
                (step, None)
            }
            ValueUnit::Children(left, right) => (progress(), Some((left, right))),
        };
        self.frames[index] = ValueFrame::new(left_page.next, right_page.next)?;
        if let Some((left, right)) = children {
            self.push(left, right)?;
        }
        Ok(step)
    }
}

impl TypedCompare for UiValue {
    fn compare_one(&self, right: &Self, _: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiComponentCompareProgress, &'static str> {
        let step = values.advance(self, right, bytes)?;
        if step.complete {
            values.reset();
        }
        Ok(step)
    }
}

impl TypedCompare for UiMap {
    fn compare_one(&self, right: &Self, _: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiComponentCompareProgress, &'static str> {
        let step = values.advance_map(self, right, bytes)?;
        if step.complete {
            values.reset();
        }
        Ok(step)
    }
}
//#endregion 🌳️BorrowedValueTraversal


#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
