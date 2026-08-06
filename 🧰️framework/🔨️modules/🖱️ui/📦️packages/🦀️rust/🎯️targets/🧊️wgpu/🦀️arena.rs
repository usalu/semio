// #region arena
//! 🕳️ Hand-rolled generational arena for retained-mode tree nodes. No third-party slotmap dep: a
//! wrapper around an external crate's handle type would hide nothing and add a dependency for a
//! ~120-line data structure (repo rule: don't wrap external types without adding value).

/// 🪪️ Opaque handle into an `Arena`: a slot index plus a generation counter. A stale `NodeId`
/// (same index, old generation) never aliases a value inserted into a recycled slot.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NodeId {
    index: u32,
    generation: u32,
}

enum Slot<T> {
    Occupied { generation: u32, value: T },
    Free { generation: u32, next_free: Option<u32> },
}

/// 🌳️ Generational-index arena: O(1) insert/remove/get, freed slots recycled via an intrusive free
/// list threaded through `Slot::Free`.
pub struct Arena<T> {
    slots: Vec<Slot<T>>,
    free_head: Option<u32>,
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self { slots: Vec::new(), free_head: None }
    }
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, value: T) -> NodeId {
        match self.free_head {
            Some(index) => {
                let (generation, next_free) = match &self.slots[index as usize] {
                    Slot::Free { generation, next_free } => (*generation, *next_free),
                    Slot::Occupied { .. } => unreachable!("free list points at an occupied slot"),
                };
                self.free_head = next_free;
                self.slots[index as usize] = Slot::Occupied { generation, value };
                NodeId { index, generation }
            }
            None => {
                let index = self.slots.len() as u32;
                self.slots.push(Slot::Occupied { generation: 0, value });
                NodeId { index, generation: 0 }
            }
        }
    }

    pub fn remove(&mut self, id: NodeId) -> Option<T> {
        let slot = self.slots.get_mut(id.index as usize)?;
        match slot {
            Slot::Occupied { generation, .. } if *generation == id.generation => {
                let next_free = self.free_head;
                let freed_generation = generation.wrapping_add(1);
                let previous = std::mem::replace(slot, Slot::Free { generation: freed_generation, next_free });
                self.free_head = Some(id.index);
                match previous {
                    Slot::Occupied { value, .. } => Some(value),
                    Slot::Free { .. } => unreachable!(),
                }
            }
            _ => None,
        }
    }

    pub fn get(&self, id: NodeId) -> Option<&T> {
        match self.slots.get(id.index as usize)? {
            Slot::Occupied { generation, value } if *generation == id.generation => Some(value),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut T> {
        match self.slots.get_mut(id.index as usize)? {
            Slot::Occupied { generation, value } if *generation == id.generation => Some(value),
            _ => None,
        }
    }

    pub fn contains(&self, id: NodeId) -> bool {
        self.get(id).is_some()
    }

    /// 🚶️ Iterates every live `(NodeId, &T)` pair; freed slots are skipped.
    pub fn iter(&self) -> impl Iterator<Item = (NodeId, &T)> {
        self.slots.iter().enumerate().filter_map(|(index, slot)| match slot {
            Slot::Occupied { generation, value } => Some((NodeId { index: index as u32, generation: *generation }, value)),
            Slot::Free { .. } => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_get_round_trip() {
        let mut arena = Arena::new();
        let id = arena.insert(42);
        assert_eq!(arena.get(id), Some(&42));
    }

    #[test]
    fn remove_invalidates_the_old_node_id() {
        let mut arena = Arena::new();
        let id = arena.insert(1);
        assert_eq!(arena.remove(id), Some(1));
        assert_eq!(arena.get(id), None);
        assert_eq!(arena.remove(id), None);
    }

    #[test]
    fn reused_slot_bumps_generation_so_old_id_does_not_alias_new_value() {
        let mut arena = Arena::new();
        let a = arena.insert(1);
        arena.remove(a);
        let b = arena.insert(2);
        assert_eq!(b.index, a.index);
        assert_ne!(b.generation, a.generation);
        assert_eq!(arena.get(a), None);
        assert_eq!(arena.get(b), Some(&2));
    }

    #[test]
    fn iterates_over_live_slots_only() {
        let mut arena = Arena::new();
        let a = arena.insert(10);
        let b = arena.insert(20);
        arena.remove(a);
        let remaining: Vec<i32> = arena.iter().map(|(_, value)| *value).collect();
        assert_eq!(remaining, vec![20]);
        assert!(arena.contains(b));
        assert!(!arena.contains(a));
    }
}
// #endregion arena
