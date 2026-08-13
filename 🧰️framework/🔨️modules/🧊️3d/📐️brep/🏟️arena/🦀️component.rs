//! 🗄️ A generic generational arena `Store<T, Id>` plus the [`define_id!`] macro that stamps out
//! one typed id per topology/geometry kind. Generational `(index, generation)` ids are chosen
//! over raw indices or `Rc`/`Arc` pointers because they are: serde-friendly (a `Body` round-trips
//! to plain JSON), deterministic (iteration walks slots in index order — required for
//! byte-identical output across runs of the same operation sequence), and self-detecting of stale
//! handles (a freed-and-reused slot's old id fails `get` instead of silently aliasing new data).

// #region 🔖️Ids

/// 🗄️ The (index, generation) pair every typed id newtype wraps. Implemented by [`define_id!`].
pub trait ArenaId: Copy + Eq + std::hash::Hash + std::fmt::Debug {
    fn from_raw(index: u32, generation: u32) -> Self;
    fn raw_index(self) -> u32;
    fn raw_generation(self) -> u32;
}

/// 🗄️ Declares a `Copy + Eq + Hash + Ord + Serialize` newtype id backed by `(u32, u32)`, with a
/// human-readable `"kind-index"` `Display`/`FromStr` pair (the textual encoding boundary layers —
/// flow dictionaries, document ids — key off of, per the plan's `EntityRef` design).
#[macro_export]
macro_rules! define_id {
    ($name:ident, $tag:literal) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            index: u32,
            generation: u32,
        }
        impl $crate::brep::arena::ArenaId for $name {
            fn from_raw(index: u32, generation: u32) -> Self {
                $name { index, generation }
            }
            fn raw_index(self) -> u32 {
                self.index
            }
            fn raw_generation(self) -> u32 {
                self.generation
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}-{}-{}", $tag, self.index, self.generation)
            }
        }
    };
}

define_id!(VertexId, "vertex");
define_id!(EdgeId, "edge");
define_id!(CoedgeId, "coedge");
define_id!(LoopId, "loop");
define_id!(FaceId, "face");
define_id!(ShellId, "shell");
define_id!(SolidId, "solid");
define_id!(Curve3Id, "curve3");
define_id!(Curve2Id, "curve2");
define_id!(SurfaceId, "surface");

// #endregion 🔖️Ids

// #region 🔖️Store

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct Slot<T> {
    generation: u32,
    value: Option<T>,
}

/// 🗄️ A generational arena: O(1) insert/get/remove, a LIFO free list so freed slots are reused
/// deterministically (identical operation sequences reuse slots in the same order, a precondition
/// for byte-identical serialized output), and index-ordered iteration. Serde bounds are pinned to
/// `T` only — `Id` never needs to be (de)serializable itself, it only appears inside a zero-sized
/// `PhantomData` marker.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(bound(serialize = "T: serde::Serialize", deserialize = "T: serde::de::DeserializeOwned"))]
pub struct Store<T, Id> {
    slots: Vec<Slot<T>>,
    free: Vec<u32>,
    _marker: std::marker::PhantomData<fn() -> Id>,
}

impl<T, Id: ArenaId> Default for Store<T, Id> {
    fn default() -> Self {
        Store::new()
    }
}

impl<T, Id: ArenaId> Store<T, Id> {
    pub fn new() -> Self {
        Store { slots: Vec::new(), free: Vec::new(), _marker: std::marker::PhantomData }
    }
    pub fn insert(&mut self, value: T) -> Id {
        if let Some(index) = self.free.pop() {
            let slot = &mut self.slots[index as usize];
            slot.value = Some(value);
            Id::from_raw(index, slot.generation)
        } else {
            let index = self.slots.len() as u32;
            self.slots.push(Slot { generation: 0, value: Some(value) });
            Id::from_raw(index, 0)
        }
    }
    pub fn get(&self, id: Id) -> Option<&T> {
        let slot = self.slots.get(id.raw_index() as usize)?;
        if slot.generation == id.raw_generation() {
            slot.value.as_ref()
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, id: Id) -> Option<&mut T> {
        let slot = self.slots.get_mut(id.raw_index() as usize)?;
        if slot.generation == id.raw_generation() {
            slot.value.as_mut()
        } else {
            None
        }
    }
    pub fn contains(&self, id: Id) -> bool {
        self.get(id).is_some()
    }
    pub fn remove(&mut self, id: Id) -> Option<T> {
        let slot = self.slots.get_mut(id.raw_index() as usize)?;
        if slot.generation != id.raw_generation() {
            return None;
        }
        let value = slot.value.take()?;
        slot.generation = slot.generation.wrapping_add(1);
        self.free.push(id.raw_index());
        Some(value)
    }
    pub fn len(&self) -> usize {
        self.slots.iter().filter(|s| s.value.is_some()).count()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// 🗄️ Deterministic index-order iteration over live entries.
    pub fn iter(&self) -> impl Iterator<Item = (Id, &T)> {
        self.slots.iter().enumerate().filter_map(|(i, slot)| slot.value.as_ref().map(|v| (Id::from_raw(i as u32, slot.generation), v)))
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Id, &mut T)> {
        self.slots.iter_mut().enumerate().filter_map(|(i, slot)| {
            let gen = slot.generation;
            slot.value.as_mut().map(|v| (Id::from_raw(i as u32, gen), v))
        })
    }
    pub fn ids(&self) -> impl Iterator<Item = Id> + '_ {
        self.iter().map(|(id, _)| id)
    }
}

// #endregion 🔖️Store

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    define_id!(TestId, "test");

    #[test]
    fn insert_and_get_round_trips() {
        let mut store: Store<i32, TestId> = Store::new();
        let id = store.insert(42);
        assert_eq!(store.get(id), Some(&42));
    }

    #[test]
    fn remove_then_get_returns_none() {
        let mut store: Store<i32, TestId> = Store::new();
        let id = store.insert(1);
        assert_eq!(store.remove(id), Some(1));
        assert_eq!(store.get(id), None);
    }

    #[test]
    fn stale_handle_after_reuse_returns_none() {
        let mut store: Store<i32, TestId> = Store::new();
        let a = store.insert(1);
        store.remove(a);
        let b = store.insert(2);
        assert_eq!(b.raw_index(), a.raw_index(), "the freed slot should be reused (LIFO free list)");
        assert_ne!(b.raw_generation(), a.raw_generation());
        assert_eq!(store.get(a), None, "the stale handle must not alias the new value");
        assert_eq!(store.get(b), Some(&2));
    }

    #[test]
    fn iteration_is_index_ordered_and_skips_removed_slots() {
        let mut store: Store<i32, TestId> = Store::new();
        let a = store.insert(10);
        let _b = store.insert(20);
        let c = store.insert(30);
        store.remove(a);
        let collected: Vec<(TestId, i32)> = store.iter().map(|(id, v)| (id, *v)).collect();
        assert_eq!(collected.len(), 2);
        assert_eq!(collected[0].1, 20);
        assert_eq!(collected[1].1, 30);
        assert!(collected[1].0.raw_index() == c.raw_index());
    }

    #[test]
    fn len_reflects_only_live_entries() {
        let mut store: Store<i32, TestId> = Store::new();
        let a = store.insert(1);
        store.insert(2);
        assert_eq!(store.len(), 2);
        store.remove(a);
        assert_eq!(store.len(), 1);
        assert!(!store.is_empty());
    }

    #[test]
    fn display_uses_readable_tag_index_generation_format() {
        let mut store: Store<i32, TestId> = Store::new();
        let id = store.insert(1);
        assert_eq!(id.to_string(), format!("test-{}-{}", id.raw_index(), id.raw_generation()));
    }

    #[test]
    fn serde_round_trips_an_id() {
        let mut store: Store<i32, TestId> = Store::new();
        let id = store.insert(1);
        let json = serde_json::to_string(&id).unwrap();
        let back: TestId = serde_json::from_str(&json).unwrap();
        assert_eq!(back, id);
    }

    mod quick {
        use super::*;

        #[test]
        fn random_insert_remove_sequence_never_aliases_a_removed_id() {
            let mut rng = semio_framework_geometry::random::Rng::from_seed(83);
            let mut store: Store<u64, TestId> = Store::new();
            let mut live: Vec<(TestId, u64)> = Vec::new();
            let mut removed: Vec<TestId> = Vec::new();
            for i in 0..2000u64 {
                if !live.is_empty() && rng.next_bool(0.4) {
                    let idx = rng.next_range(0, live.len() as u64) as usize;
                    let (id, _) = live.remove(idx);
                    store.remove(id);
                    removed.push(id);
                } else {
                    let id = store.insert(i);
                    live.push((id, i));
                }
            }
            for (id, value) in &live {
                assert_eq!(store.get(*id), Some(value));
            }
            for id in &removed {
                if !live.iter().any(|(lid, _)| lid == id) {
                    assert_eq!(store.get(*id), None, "removed id {id:?} must not resolve");
                }
            }
        }
    }
}
// #endregion 🔖️Tests
