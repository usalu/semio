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
#[value(crate = "::protocol::value")]
        pub struct $name {
            index: u32,
            generation: u32,
        }
        impl $crate::brep::representation::arena::ArenaId for $name {
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
semio_framework_value::artifact_retire_leaf!(VertexId, EdgeId, CoedgeId, LoopId, FaceId, ShellId, SolidId, Curve3Id, Curve2Id, SurfaceId);

// #endregion 🔖️Ids

// #region 🔖️Store

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
struct Slot<T> {
    generation: u32,
    value: Option<T>,
    next_free: Option<u32>,
}

impl<T:semio_framework_value::retirement::RetireOwned> semio_framework_value::retirement::RetireOwned for Slot<T> {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.generation),semio_framework_value::retirement::deferred(self.value),semio_framework_value::retirement::deferred(self.next_free)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.generation),semio_framework_value::retirement::deferred_birth_bytes_for(&self.value),semio_framework_value::retirement::deferred_birth_bytes_for(&self.next_free)])}
    fn controlled_retirement_supported()->bool {T::controlled_retirement_supported()}
}

/// 🗄️ A generational arena: O(1) insert/get/remove, a LIFO free list so freed slots are reused
/// deterministically (identical operation sequences reuse slots in the same order, a precondition
/// for byte-identical serialized output), and index-ordered iteration. Serde bounds are pinned to
/// `T` only — `Id` never needs to be (de)serializable itself, it only appears inside a zero-sized
/// `PhantomData` marker.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct Store<T, Id> {
    slots: semio_framework_value::list::PagedList<Slot<T>,{usize::MAX}>,
    free_head: Option<u32>,
    _marker: std::marker::PhantomData<fn() -> Id>,
}

impl<T:semio_framework_value::retirement::RetireOwned,Id:ArenaId+'static> semio_framework_value::retirement::RetireOwned for Store<T,Id> {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.slots),semio_framework_value::retirement::deferred(self.free_head)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.slots),semio_framework_value::retirement::deferred_birth_bytes_for(&self.free_head)])}
    fn controlled_retirement_supported()->bool {T::controlled_retirement_supported()}
}

impl<T, Id: ArenaId> Default for Store<T, Id> {
    fn default() -> Self {
        Store::new()
    }
}

impl<T, Id: ArenaId> Store<T, Id> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new() -> Self {
        Store { slots: semio_framework_value::list::PagedList::empty(), free_head: None, _marker: std::marker::PhantomData }
    }
    /// 🪙️ Original insertion transfers ownership without copying any prior slot or incoming payload.
    pub fn next_insert_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    /// 🏗️ Quotes one original page allocation before an incoming value transfers.
    pub fn next_insert_capacity_byte_demand(&self,_copy:usize)->Result<usize,semio_framework_value::ValueError>{if self.free_head.is_some(){Ok(0)}else{self.slots.next_allocation_bytes().map_err(Into::into)}}
    /// 🍂️ Insertion retains every admitted original arena page.
    pub fn next_insert_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    /// 🪜️ Quotes the actual original reuse, reserved placement or prospective page path.
    pub fn next_insert_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{
        if let Some(index)=self.free_head{return self.slots.next_get_depth_demand(index as usize).map_err(Into::into);}
        if self.slots.has_reserved_slot(){self.slots.next_push_depth_demand().map_err(Into::into)}else{self.slots.next_reserve_depth_demand().map_err(Into::into)}
    }
    /// 🎟️ Reserves one original backing allocation while every existing original value stays in place.
    pub fn reserve_insert_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneProgress,(semio_framework_value::ValueError,semio_framework_value::retained_clone::RetainedCloneProgress)>{
        use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::RetainedCloneProgress};
        let empty=RetainedCloneProgress::default();let bytes=self.next_insert_capacity_byte_demand(grant.maximum_copy_bytes).map_err(|error|(error,empty))?;if bytes==0{return Ok(empty);}
        if grant.maximum_items==0{return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"arena reservation requires one admitted item"),empty));}
        if grant.maximum_depth<self.next_insert_depth_demand().map_err(|error|(error,empty))?{return Err((ValueError::literal(ValueRefusalKind::DepthLimit,"arena reservation exceeds admitted depth"),empty));}
        if grant.maximum_capacity_bytes<bytes{return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"arena backing exceeds admitted capacity"),empty));}
        match self.slots.reserve_one(grant.maximum_capacity_bytes){Ok(progress)=>Ok(RetainedCloneProgress {copied_items:usize::from(progress.progressed),retained_capacity_bytes:progress.allocated_bytes,..empty}),Err(error)=>{let receipt=RetainedCloneProgress {copied_items:usize::from(error.allocated_bytes!=0),retained_capacity_bytes:error.allocated_bytes,..empty};let error=ValueError::from(error.refusal()).with_retained_progress(receipt);Err((error,receipt))}}
    }
    /// 🫴️ Transfers the exact original incoming value into independently admitted original backing.
    pub fn insert_reserved(&mut self,value:T,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(Id,semio_framework_value::retained_clone::RetainedCloneProgress),(semio_framework_value::ValueError,T)>{
        use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::RetainedCloneProgress};
        if grant.maximum_items==0{return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"arena insertion requires one admitted item"),value));}
        let depth=match self.next_insert_depth_demand(){Ok(depth)=>depth,Err(error)=>return Err((error,value))};
        if grant.maximum_depth<depth{return Err((ValueError::literal(ValueRefusalKind::DepthLimit,"arena insertion exceeds admitted depth"),value));}
        if let Some(index)=self.free_head{let slot=&mut self.slots[index as usize];self.free_head=slot.next_free.take();slot.value=Some(value);return Ok((Id::from_raw(index,slot.generation),RetainedCloneProgress {copied_items:1,..Default::default()}));}
        if !self.slots.has_reserved_slot(){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"arena insertion requires its admitted original slot"),value));}
        let Ok(index)=u32::try_from(self.slots.len())else{return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"arena original slot identity is exhausted"),value));};
        match self.slots.push_reserved(Slot {generation:0,value:Some(value),next_free:None}){Ok(())=>Ok((Id::from_raw(index,0),RetainedCloneProgress {copied_items:1,..Default::default()})),Err(slot)=>Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"arena original reserved slot changed"),slot.value.unwrap()))}
    }
    /// 🧊️ Explicitly drains the same original reserve and transfer frontier for cold construction.
    pub fn insert(&mut self, value: T) -> Id {
        use semio_framework_value::retained_clone::RetainedCloneGrant;
        let copy=self.next_insert_copy_byte_demand().expect("original arena cold copy authority");
        loop{let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:self.next_insert_capacity_byte_demand(copy).expect("original arena cold capacity authority"),maximum_release_bytes:self.next_insert_release_byte_demand().expect("original arena cold release authority"),maximum_depth:self.next_insert_depth_demand().expect("original arena cold depth authority")};if grant.maximum_capacity_bytes==0{return self.insert_reserved(value,grant).unwrap_or_else(|(error,_)|panic!("original arena cold transfer: {error}")).0;}self.reserve_insert_step(grant).unwrap_or_else(|(error,_)|panic!("original arena cold reserve: {error}"));}
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
        slot.next_free=self.free_head;
        self.free_head=Some(id.raw_index());
        Some(value)
    }
    /// 🎟️ The intrusive original free head changes no backing allocation.
    pub(crate) fn remove_backing_demand(&self,_id:Id)->(usize,usize) {(0,0)}
    /// 🧭️ Quotes the original existing slot path without changing its generation or payload.
    pub fn next_remove_depth_demand(&self,id:Id)->Result<usize,semio_framework_value::ValueError>{self.next_slot_depth_demand(id.raw_index() as usize).map(|depth|depth.max(1))}
    /// 🎟️ Removes one original slot through its inline free head after scalar work admission.
    pub(crate) fn remove_granted(&mut self,id:Id,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(Option<T>,semio_framework_value::retained_clone::RetainedCloneProgress),semio_framework_value::ValueError> {
        use semio_framework_value::retained_clone::RetainedCloneProgress;
        if grant.maximum_items==0 || grant.maximum_depth<self.next_remove_depth_demand(id)? {return Ok((None,RetainedCloneProgress::default()));}
        let value=self.remove(id);
        Ok((value,RetainedCloneProgress {copied_items:1,..Default::default()}))
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
    /// (`crate::brep::representation::topology::Body`) calls per unreachable id: bumps the slot's generation
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
    /// 🎟️ The original arena slot count, including holes, without a live-entry scan.
    pub(crate) fn slot_count(&self) -> usize { self.slots.len() }
    /// 📏️ Borrows the exact original arena page path for one source slot.
    pub(crate) fn next_slot_depth_demand(&self,index:usize)->Result<usize,semio_framework_value::ValueError>{self.slots.next_get_depth_demand(index).map_err(Into::into)}
    /// 🎟️ Borrows one original slot; a hole consumes its caller's scan turn.
    pub(crate) fn slot_at(&self, index: usize) -> Option<(Id, &T)> {
        let slot = self.slots.get(index)?;
        slot.value.as_ref().map(|value| (Id::from_raw(index as u32, slot.generation), value))
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
