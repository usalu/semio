//! 🗄️ A generic generational arena `Store<T, Id>` plus the [`define_id!`] macro that stamps out
//! one typed id per topology/geometry kind. Generational `(index, generation)` ids are chosen
//! over raw indices or `Rc`/`Arc` pointers because they are: serde-friendly (a `Body` round-trips
//! to plain JSON), deterministic (iteration walks slots in index order — required for
//! byte-identical output across runs of the same operation sequence), and self-detecting of stale
//! handles (a freed-and-reused slot's old id fails `get` instead of silently aliasing new data).
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/🏟️arena` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL3.

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
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue)]
        pub struct $name {
            index: u32,
            generation: u32,
        }
        impl $crate::standards::v1::subsets::brep::schema::snapshot::arena::ArenaId for $name {
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

#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
struct Slot<T> {
    generation: u32,
    value: Option<T>,
}

/// 🗄️ A generational arena: O(1) insert/get/remove, a LIFO free list so freed slots are reused
/// deterministically (identical operation sequences reuse slots in the same order, a precondition
/// for byte-identical serialized output), and index-ordered iteration. Serde bounds are pinned to
/// `T` only — `Id` never needs to be (de)serializable itself, it only appears inside a zero-sized
/// `PhantomData` marker.
#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new() -> Self {
        Store { slots: Vec::new(), free: Vec::new(), _marker: std::marker::PhantomData }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn get(&self, id: Id) -> Option<&T> {
        let slot = self.slots.get(id.raw_index() as usize)?;
        if slot.generation == id.raw_generation() {
            slot.value.as_ref()
        } else {
            None
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn get_mut(&mut self, id: Id) -> Option<&mut T> {
        let slot = self.slots.get_mut(id.raw_index() as usize)?;
        if slot.generation == id.raw_generation() {
            slot.value.as_mut()
        } else {
            None
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn contains(&self, id: Id) -> bool {
        self.get(id).is_some()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn len(&self) -> usize {
        self.slots.iter().filter(|s| s.value.is_some()).count()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// 🗄️ Frees `id`'s slot without needing its value back — the arena-GC primitive [`Body::compact`]
    /// (`crate::standards::v1::subsets::brep::schema::snapshot::topology::Body`) calls per unreachable id: bumps the slot's generation
    /// (see [`Self::remove`]) so a still-held stale id self-detects instead of aliasing whatever
    /// reuses the slot next. Returns `false` (no-op) for an already-stale or out-of-range id.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn free(&mut self, id: Id) -> bool {
        self.remove(id).is_some()
    }
    /// 🗄️ Whether `id` currently resolves to a value — [`Self::contains`] under the name a
    /// liveness/registry call site reads more naturally.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_live(&self, id: Id) -> bool {
        self.contains(id)
    }
    /// 🗄️ Deterministic index-order iteration over live entries.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn iter(&self) -> impl Iterator<Item = (Id, &T)> {
        self.slots.iter().enumerate().filter_map(|(i, slot)| slot.value.as_ref().map(|v| (Id::from_raw(i as u32, slot.generation), v)))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Id, &mut T)> {
        self.slots.iter_mut().enumerate().filter_map(|(i, slot)| {
            let gen = slot.generation;
            slot.value.as_mut().map(|v| (Id::from_raw(i as u32, gen), v))
        })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn ids(&self) -> impl Iterator<Item = Id> + '_ {
        self.iter().map(|(id, _)| id)
    }
}

// #endregion 🔖️Store

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
