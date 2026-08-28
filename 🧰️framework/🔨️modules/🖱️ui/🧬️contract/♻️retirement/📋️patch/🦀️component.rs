//! 🩹️ Separately admitted one-operation pages with exact typed descendant retirement.

use crate::{UiPatchOp, UiValueRetirement, UiValueRetirementStep, UI_DOCUMENT_PATCH_OPS};
use crate::action::{UiTypedRetire, UiTypedRetirementCursor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::mem::size_of;

#[path = "📨️pending/🦀️component.rs"]
mod pending;
pub use pending::UiPendingPatchOp;

//#region 📦️Storage
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiPatchAllocationError {
    pub allocated_bytes: usize,
    pub reason: &'static str,
}

#[derive(Default)]
struct PatchPages {
    pages: Vec<Vec<UiPatchOp>>,
    length: usize,
    allocated_bytes: usize,
}

#[derive(Default)]
pub struct UiPatchOps {
    storage: PatchPages,
    retirement: UiTypedRetirementCursor,
    closing: bool,
}

impl UiPatchOps {
    pub fn len(&self) -> usize { self.storage.length }
    pub fn is_empty(&self) -> bool { self.len() == 0 }
    pub fn capacity(&self) -> usize { self.storage.pages.len() }
    pub fn allocated_bytes(&self) -> usize { self.storage.allocated_bytes }
    pub fn has_reserved_slot(&self) -> bool { !self.closing && self.storage.pages.len() > self.storage.length }

    /// 🎟️ Returns physical backing required by the next separate allocation, before any mutation.
    pub fn next_allocation_bytes(&self) -> Result<usize, &'static str> {
        if self.closing { return Err("patch storage is closing"); }
        if self.len() == UI_DOCUMENT_PATCH_OPS { return Err("patch logical capacity exhausted"); }
        if self.has_reserved_slot() { return Ok(0); }
        if self.storage.pages.capacity() == 0 {
            return UI_DOCUMENT_PATCH_OPS.checked_mul(size_of::<Vec<UiPatchOp>>()).ok_or("patch directory allocation overflow");
        }
        Ok(size_of::<UiPatchOp>())
    }

    /// 🧾️ The caller must pre-admit physical bytes in its ledger; zero means no allocation occurred.
    pub fn try_reserve_one(&mut self, admitted_bytes: usize) -> Result<usize, UiPatchAllocationError> {
        let rejected = |reason| UiPatchAllocationError { allocated_bytes: 0, reason };
        let requested = self.next_allocation_bytes().map_err(rejected)?;
        if requested == 0 || admitted_bytes < requested { return Ok(0); }
        self.storage.allocated_bytes.checked_add(admitted_bytes).ok_or_else(|| rejected("patch allocation counter overflow"))?;
        if self.storage.pages.capacity() == 0 {
            self.storage.pages.try_reserve_exact(UI_DOCUMENT_PATCH_OPS).map_err(|_| rejected("patch directory allocation failed"))?;
            let actual = self.storage.pages.capacity() * size_of::<Vec<UiPatchOp>>();
            self.storage.allocated_bytes = actual;
            if actual > admitted_bytes { return Err(UiPatchAllocationError { allocated_bytes: actual, reason: "patch directory allocator exceeded admission; backing retained" }); }
            return Ok(actual);
        }
        let mut page = Vec::new();
        page.try_reserve_exact(1).map_err(|_| rejected("patch payload allocation failed"))?;
        let actual = page.capacity() * size_of::<UiPatchOp>();
        self.storage.pages.push(page);
        self.storage.allocated_bytes = self.storage.allocated_bytes.checked_add(actual).expect("admitted patch capacity fits byte counter");
        if actual > admitted_bytes { return Err(UiPatchAllocationError { allocated_bytes: actual, reason: "patch payload allocator exceeded admission; backing retained" }); }
        Ok(actual)
    }

    /// 📥️ Reports the whole inline placement; insufficient grants preserve the borrowed source exactly.
    pub fn try_push_reserved(&mut self, source: &mut Option<UiPatchOp>, physical_bytes: usize) -> Result<usize, &'static str> {
        if self.closing { return Err("patch storage is closing"); }
        if self.len() == UI_DOCUMENT_PATCH_OPS { return Err("patch logical capacity exhausted"); }
        if source.is_none() || physical_bytes < size_of::<UiPatchOp>() || !self.has_reserved_slot() { return Ok(0); }
        self.storage.pages[self.storage.length].push(source.take().expect("checked patch source"));
        self.storage.length += 1;
        Ok(size_of::<UiPatchOp>())
    }

    /// 🧊️ Synchronous cold builder only; retained callers separate ledger admission and placement.
    pub fn try_push(&mut self, value: UiPatchOp) -> Result<(), UiPatchOp> {
        let mut source = Some(value);
        while !self.has_reserved_slot() {
            let Ok(bytes) = self.next_allocation_bytes() else { return Err(source.take().unwrap()) };
            if self.try_reserve_one(bytes).is_err() { return Err(source.take().unwrap()); }
        }
        match self.try_push_reserved(&mut source, size_of::<UiPatchOp>()) { Ok(bytes) if bytes != 0 => Ok(()), _ => Err(source.take().unwrap()) }
    }

    pub fn get(&self, index: usize) -> Option<&UiPatchOp> {
        if self.closing || index >= self.len() { return None; }
        self.storage.pages.get(index)?.first()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut UiPatchOp> {
        if self.closing || index >= self.len() { return None; }
        self.storage.pages.get_mut(index)?.first_mut()
    }

    pub fn last_mut(&mut self) -> Option<&mut UiPatchOp> { self.get_mut(self.len().checked_sub(1)?) }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &UiPatchOp> {
        assert!(!self.closing, "closing patch payload cannot be read");
        self.storage.pages[..self.storage.length].iter().map(|page| &page[0])
    }

    /// 🧊️ Cold owned extraction may move the full operation and release its page; never a retained close step.
    pub fn pop(&mut self) -> Option<UiPatchOp> {
        assert!(!self.closing, "closing patch payload cannot be extracted");
        if self.storage.pages.len() > self.storage.length {
            let empty = self.storage.pages.pop().unwrap();
            self.storage.allocated_bytes -= empty.capacity() * size_of::<UiPatchOp>();
        }
        let mut page = self.storage.pages.pop()?;
        self.storage.length -= 1;
        self.storage.allocated_bytes -= page.capacity() * size_of::<UiPatchOp>();
        page.pop()
    }

    /// 🧊️ Cold empty-backing release; retained owners release one page through close_step instead.
    pub fn release_empty_allocation(&mut self) -> Result<bool, &'static str> {
        if !self.is_empty() { return Err("patch payload must retire before backing release"); }
        let released = self.storage.allocated_bytes != 0;
        self.storage.pages = Vec::new();
        self.storage.allocated_bytes = 0;
        Ok(released)
    }

    /// 📤️ Transfers page descriptors and the exact in-progress cursor, never an inline operation.
    pub fn take_all(&mut self) -> Self { std::mem::take(self) }

    /// ♻️ Semantic fields retire in place; backing owners release only after their exact descendants.
    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(UiValueRetirementStep::default()); }
        self.closing = true;
        self.retirement.advance(&mut self.storage, maximum_items, maximum_bytes)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.storage.allocated_bytes == 0 && self.storage.pages.is_empty() && (!self.closing || self.retirement.terminal_is_empty())
    }
}

impl std::fmt::Debug for UiPatchOps {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("UiPatchOps").field("length", &self.len()).field("allocated_bytes", &self.allocated_bytes()).field("closing", &self.closing).finish()
    }
}

impl PartialEq for UiPatchOps {
    fn eq(&self, other: &Self) -> bool { !self.closing && !other.closing && self.iter().eq(other.iter()) }
}

impl std::ops::Index<usize> for UiPatchOps {
    type Output = UiPatchOp;
    fn index(&self, index: usize) -> &Self::Output { self.get(index).expect("patch index must identify a readable initialized operation") }
}

impl<'a> IntoIterator for &'a UiPatchOps {
    type Item = &'a UiPatchOp;
    type IntoIter = std::iter::Map<std::slice::Iter<'a, Vec<UiPatchOp>>, fn(&'a Vec<UiPatchOp>) -> &'a UiPatchOp>;
    fn into_iter(self) -> Self::IntoIter {
        assert!(!self.closing, "closing patch payload cannot be read");
        self.storage.pages[..self.len()].iter().map(|page| &page[0])
    }
}

/// 🧊️ Owned iteration is synchronous cold traversal, including any unconsumed descendants on Drop.
impl IntoIterator for UiPatchOps {
    type Item = UiPatchOp;
    type IntoIter = std::iter::Flatten<std::vec::IntoIter<Vec<UiPatchOp>>>;
    fn into_iter(self) -> Self::IntoIter { assert!(!self.closing, "closing patch payload cannot be extracted"); self.storage.pages.into_iter().flatten() }
}

impl Serialize for UiPatchOps {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        if self.closing { return Err(serde::ser::Error::custom("closing patch payload cannot be serialized")); }
        let mut sequence = serializer.serialize_seq(Some(self.len()))?;
        for operation in self { sequence.serialize_element(operation)?; }
        sequence.end()
    }
}

/// 🧊️ Cold decoding admits each page synchronously; it is not an interactive decoder.
impl<'de> Deserialize<'de> for UiPatchOps {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UiPatchOps;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str("a bounded patch operation sequence") }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let mut operations = UiPatchOps::default();
                while let Some(operation) = access.next_element()? {
                    if operations.try_push(operation).is_err() { return Err(serde::de::Error::custom("patch allocation or logical capacity exhausted")); }
                }
                Ok(operations)
            }
        }
        deserializer.deserialize_seq(Visitor)
    }
}
//#endregion 📦️Storage

//#region ♻️InPlaceRetirement
impl UiTypedRetire for PatchPages {
    const DEPTH: usize = 1 + <UiPatchOp as UiTypedRetire>::DEPTH;
    fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        let (phase, child_path) = path.split_first_mut().ok_or("patch retirement exceeds schema depth")?;
        let Some(page) = self.pages.last_mut() else {
            let released = self.allocated_bytes != 0;
            self.pages = Vec::new();
            self.allocated_bytes = 0;
            return Ok(UiValueRetirementStep { complete: true, progressed: true, released_items: usize::from(released), released_bytes: 0 });
        };
        if page.is_empty() {
            let allocated = page.capacity() * size_of::<UiPatchOp>();
            self.pages.pop();
            self.allocated_bytes -= allocated;
            return Ok(UiValueRetirementStep { progressed: true, released_items: 1, ..UiValueRetirementStep::default() });
        }
        if *phase == 1 {
            page.truncate(0);
            self.length -= 1;
            *phase = 0;
            child_path.fill(0);
            return Ok(UiValueRetirementStep { progressed: true, released_items: 1, ..UiValueRetirementStep::default() });
        }
        let mut step = page[0].retire_typed(child_path, value, bytes)?;
        if step.complete { *phase = 1; child_path.fill(0); }
        step.complete = false;
        Ok(step)
    }
}

impl UiTypedRetire for UiPatchOps {
    const DEPTH: usize = <PatchPages as UiTypedRetire>::DEPTH;
    fn retire_typed(&mut self, _: &mut [u8], _: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str> { self.close_step(1, bytes) }
}
//#endregion ♻️InPlaceRetirement
