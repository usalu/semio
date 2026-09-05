//! ♻️ Explicit, incremental typed-owner retirement shared by artifact factories and host codecs.

use super::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory, SnapshotRetirementStep};
use std::{marker::PhantomData, mem::ManuallyDrop, sync::Arc};

pub trait RetireOwned: Send + 'static {
    fn retirement(self) -> Box<dyn RetirementCursor>;
}

pub enum RetirementStep {
    Child(Box<dyn RetirementCursor>),
    Bytes(usize),
    Complete,
    BudgetExhausted,
}

pub trait RetirementCursor: Send {
    fn close_step(&mut self, maximum_bytes: usize) -> RetirementStep;
    fn terminal_is_empty(&self) -> bool;
}

struct Leaf<T: Copy + Send + 'static> {
    value: Option<T>,
    remaining: usize,
}
impl<T: Copy + Send + 'static> RetirementCursor for Leaf<T> {
    fn close_step(&mut self, maximum_bytes: usize) -> RetirementStep {
        if self.remaining > 0 {
            if maximum_bytes == 0 {
                return RetirementStep::BudgetExhausted;
            }
            let bytes = maximum_bytes.min(self.remaining);
            self.remaining -= bytes;
            return RetirementStep::Bytes(bytes);
        }
        self.value.take();
        RetirementStep::Complete
    }
    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.remaining == 0
    }
}

pub fn leaf<T: Copy + Send + 'static>(value: T) -> Box<dyn RetirementCursor> {
    Box::new(Leaf { value: Some(value), remaining: std::mem::size_of::<T>() })
}

#[macro_export]
macro_rules! artifact_retire_leaf {
    ($($type:ty),+ $(,)?) => {$ (
        impl $crate::os_store::retirement::RetireOwned for $type {
            fn retirement(self) -> Box<dyn $crate::os_store::retirement::RetirementCursor> { $crate::os_store::retirement::leaf(self) }
        }
    )+ };
}

#[macro_export]
macro_rules! artifact_retirement_sequence {
    ($($field:expr),* $(,)?) => { $crate::os_store::retirement::sequence(vec![$($crate::os_store::retirement::RetireOwned::retirement($field)),*]) };
}

#[macro_export]
macro_rules! artifact_retire_struct {
    ($type:ty { $($field:ident),+ $(,)? }) => {
        impl $crate::os_store::retirement::RetireOwned for $type {
            fn retirement(self) -> Box<dyn $crate::os_store::retirement::RetirementCursor> {
                let Self { $($field),+ } = self;
                $crate::artifact_retirement_sequence![$($field),+]
            }
        }
    };
}

artifact_retire_leaf!(bool, u8, u16, u32, u64, usize, i8, i16, i32, i64, f32, f64);

struct Bytes(ManuallyDrop<Vec<u8>>);
impl RetirementCursor for Bytes {
    fn close_step(&mut self, maximum_bytes: usize) -> RetirementStep {
        if self.0.is_empty() {
            return RetirementStep::Complete;
        }
        if maximum_bytes == 0 {
            return RetirementStep::BudgetExhausted;
        }
        let bytes = maximum_bytes.min(self.0.len());
        let next = self.0.len() - bytes;
        self.0.truncate(next);
        RetirementStep::Bytes(bytes)
    }
    fn terminal_is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl Drop for Bytes {
    fn drop(&mut self) {
        assert!(self.0.is_empty(), "owned bytes retired before terminal-empty");
        unsafe { ManuallyDrop::drop(&mut self.0) };
    }
}
impl RetireOwned for String {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        Box::new(Bytes(ManuallyDrop::new(self.into_bytes())))
    }
}

struct Collection<T: RetireOwned>(ManuallyDrop<Vec<T>>);
impl<T: RetireOwned> RetirementCursor for Collection<T> {
    fn close_step(&mut self, _: usize) -> RetirementStep {
        self.0.pop().map_or(RetirementStep::Complete, |value| RetirementStep::Child(value.retirement()))
    }
    fn terminal_is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl<T: RetireOwned> Drop for Collection<T> {
    fn drop(&mut self) {
        assert!(self.0.is_empty(), "owned collection retired before terminal-empty");
        unsafe { ManuallyDrop::drop(&mut self.0) };
    }
}
impl<T: RetireOwned> RetireOwned for Vec<T> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        Box::new(Collection(ManuallyDrop::new(self)))
    }
}
impl<T: RetireOwned> RetireOwned for Option<T> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        self.map_or_else(|| sequence(Vec::new()), RetireOwned::retirement)
    }
}
impl<T: RetireOwned> RetireOwned for Box<T> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        (*self).retirement()
    }
}
impl<T: RetireOwned, U: RetireOwned> RetireOwned for (T, U) {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        artifact_retirement_sequence![self.0, self.1]
    }
}
impl<T: RetireOwned, U: RetireOwned, V: RetireOwned> RetireOwned for (T, U, V) {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        artifact_retirement_sequence![self.0, self.1, self.2]
    }
}
impl RetireOwned for [u8; 32] {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        leaf(self)
    }
}

struct Sequence(ManuallyDrop<Vec<Box<dyn RetirementCursor>>>);
impl RetirementCursor for Sequence {
    fn close_step(&mut self, _: usize) -> RetirementStep {
        self.0.pop().map_or(RetirementStep::Complete, RetirementStep::Child)
    }
    fn terminal_is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl Drop for Sequence {
    fn drop(&mut self) {
        assert!(self.0.is_empty(), "owned field sequence retired before terminal-empty");
        unsafe { ManuallyDrop::drop(&mut self.0) };
    }
}
pub fn sequence(fields: Vec<Box<dyn RetirementCursor>>) -> Box<dyn RetirementCursor> {
    Box::new(Sequence(ManuallyDrop::new(fields)))
}

struct CursorStack(ManuallyDrop<Vec<Box<dyn RetirementCursor>>>);
impl CursorStack {
    fn step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        let (mut turns, mut released_items, mut released_bytes) = (0, 0, 0);
        while turns < maximum_items {
            let Some(cursor) = self.0.last_mut() else { break };
            match cursor.close_step(maximum_bytes - released_bytes) {
                RetirementStep::Child(child) => self.0.push(child),
                RetirementStep::Bytes(bytes) if bytes <= maximum_bytes - released_bytes => released_bytes += bytes,
                RetirementStep::Bytes(_) => return Err("owned retirement exceeded its exact byte grant".into()),
                RetirementStep::Complete => {
                    if !cursor.terminal_is_empty() {
                        return Err("owned retirement reported Complete without a terminal-empty witness".into());
                    }
                    self.0.pop();
                    released_items += 1;
                }
                RetirementStep::BudgetExhausted => break,
            }
            turns += 1;
        }
        Ok(if self.0.is_empty() && released_items == 0 && released_bytes == 0 { SnapshotRetirementStep::Complete } else { SnapshotRetirementStep::Pending { released_items, released_bytes } })
    }
}
impl Drop for CursorStack {
    fn drop(&mut self) {
        assert!(self.0.is_empty(), "owned cursor stack retired before terminal-empty");
        unsafe { ManuallyDrop::drop(&mut self.0) };
    }
}

struct OwnedRetirement<T: RetireOwned> {
    value: ManuallyDrop<Option<T>>,
    cursors: CursorStack,
}
impl<T: RetireOwned> ErasedSnapshotRetirement for OwnedRetirement<T> {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(value) = self.value.take() {
            self.cursors.0.push(value.retirement());
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.cursors.step(maximum_items, maximum_bytes)
    }
    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.cursors.0.is_empty()
    }
}
impl<T: RetireOwned> Drop for OwnedRetirement<T> {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "owned value retired before terminal-empty");
    }
}
pub fn owned_retirement<T: RetireOwned>(value: T) -> Box<dyn ErasedSnapshotRetirement> {
    Box::new(OwnedRetirement { value: ManuallyDrop::new(Some(value)), cursors: CursorStack(ManuallyDrop::new(Vec::new())) })
}

struct SharedRetirement<T: RetireOwned + Sync> {
    value: ManuallyDrop<Option<Arc<T>>>,
    owned: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
}
impl<T: RetireOwned + Sync> ErasedSnapshotRetirement for SharedRetirement<T> {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(value) = self.value.take() {
            match Arc::try_unwrap(value) {
                Ok(value) => *self.owned = Some(owned_retirement(value)),
                Err(shared) => {
                    *self.value = Some(shared);
                    return Ok(SnapshotRetirementStep::Blocked);
                }
            }
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let Some(owned) = self.owned.as_mut() else { return Ok(SnapshotRetirementStep::Complete) };
        match owned.close_step(maximum_items, maximum_bytes)? {
            SnapshotRetirementStep::Complete => {
                if !owned.terminal_is_empty() {
                    return Err("shared retirement lacks its nested terminal witness".into());
                }
                self.owned.take();
                Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
            }
            step => Ok(step),
        }
    }
    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.owned.is_none()
    }
}
impl<T: RetireOwned + Sync> Drop for SharedRetirement<T> {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "shared value retired before terminal-empty");
    }
}
pub fn shared_retirement<T: RetireOwned + Sync>(value: Arc<T>) -> Box<dyn ErasedSnapshotRetirement> {
    Box::new(SharedRetirement { value: ManuallyDrop::new(Some(value)), owned: ManuallyDrop::new(None) })
}

pub struct OwnedValueRetirementFactory<T>(PhantomData<fn() -> T>);
impl<T> Default for OwnedValueRetirementFactory<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<T: RetireOwned> ArtifactOwnedValueRetirementFactory<T> for OwnedValueRetirementFactory<T> {
    fn retire_owned(&self, value: T) -> Box<dyn ErasedSnapshotRetirement> {
        owned_retirement(value)
    }
}
pub struct SharedValueRetirementFactory<T>(PhantomData<fn() -> T>);
impl<T> Default for SharedValueRetirementFactory<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<T: RetireOwned + Sync> SnapshotRetirementFactory<T> for SharedValueRetirementFactory<T> {
    fn retire(&self, value: Arc<T>) -> Box<dyn ErasedSnapshotRetirement> {
        shared_retirement(value)
    }
}

artifact_retire_struct!(crate::os_io::ArtifactDialect { artifact_kind, standard, subset });
artifact_retire_struct!(crate::os_io::ArtifactRef { artifact_id, dialect });
artifact_retire_struct!(super::BlobRef { hash, size, media_type });
artifact_retire_struct!(super::ArtifactLink { target, pin, role });
artifact_retire_struct!(super::OwnerRef { parent, slot, child_id });
impl<S: Send + 'static> RetireOwned for super::ArtifactChild<S> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        artifact_retirement_sequence![self.child_id, self.target]
    }
}
impl RetireOwned for super::LinkPin {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Head => sequence(Vec::new()),
            Self::Checkpoint { id } => id.retirement(),
            Self::Snapshot { blob } => blob.retirement(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drain(mut retirement: Box<dyn ErasedSnapshotRetirement>, items: usize, bytes: usize) -> usize {
        let mut released = 0;
        for _ in 0..100_000 {
            match retirement.close_step(items, bytes).unwrap() {
                SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= items);
                    assert!(released_bytes <= bytes);
                    released += released_bytes;
                }
                SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    return released;
                }
                SnapshotRetirementStep::Blocked => panic!("unshared fixture unexpectedly blocked"),
            }
        }
        panic!("bounded fixture retirement did not finish");
    }

    #[test]
    fn owned_retirement_matches_neutral_exact_byte_grants() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixture/🔣️.json")).unwrap();
        assert_eq!(fixture["cases"].as_array().unwrap().len(), 7);
        for row in fixture["cases"].as_array().unwrap() {
            for budget in fixture["budgets"].as_array().unwrap() {
                let retirement = match row["kind"].as_str().unwrap() {
                    "string" => owned_retirement(row["value"].as_str().unwrap().to_owned()),
                    "strings" => owned_retirement(serde_json::from_value::<Vec<String>>(row["value"].clone()).unwrap()),
                    "optionalString" => owned_retirement(serde_json::from_value::<Option<String>>(row["value"].clone()).unwrap()),
                    "pair" => owned_retirement(serde_json::from_value::<(String, String)>(row["value"].clone()).unwrap()),
                    _ => panic!("unknown neutral case"),
                };
                assert_eq!(drain(retirement, budget["items"].as_u64().unwrap() as usize, budget["bytes"].as_u64().unwrap() as usize), row["bytes"].as_u64().unwrap() as usize, "{}", row["id"]);
            }
        }
    }

    #[test]
    fn owned_retirement_rejects_false_terminal_and_preserves_shared_roots() {
        let root = Arc::new("owned".to_string());
        let mut shared = shared_retirement(Arc::clone(&root));
        assert!(matches!(shared.close_step(0, 0).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
        assert!(matches!(shared.close_step(1, 1).unwrap(), SnapshotRetirementStep::Blocked));
        assert_eq!(Arc::strong_count(&root), 2);
        drop(root);
        assert_eq!(drain(shared, 1, 1), 5);
        let mut value = owned_retirement("zero".to_string());
        for _ in 0..4 {
            assert!(matches!(value.close_step(1, 0).unwrap(), SnapshotRetirementStep::Pending { released_bytes: 0, .. }));
        }
        assert!(!value.terminal_is_empty());
        assert_eq!(drain(value, 1, 2), 4);
        struct Hostile {
            mode: u8,
        }
        impl RetirementCursor for Hostile {
            fn close_step(&mut self, maximum_bytes: usize) -> RetirementStep {
                match self.mode {
                    1 => RetirementStep::Bytes(maximum_bytes + 1),
                    _ => RetirementStep::Complete,
                }
            }
            fn terminal_is_empty(&self) -> bool {
                self.mode == 2
            }
        }
        for mode in [0, 1] {
            let mut stack = CursorStack(ManuallyDrop::new(vec![Box::new(Hostile { mode })]));
            assert!(stack.step(1, 1).is_err());
            assert_eq!(stack.0.len(), 1);
            stack.0.pop();
            assert!(matches!(stack.step(1, 1).unwrap(), SnapshotRetirementStep::Complete));
        }
    }
}
