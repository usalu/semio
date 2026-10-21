//! 🧭️ The DOF map: the node-major `(node, dof)` order and its paged open-addressing equation index.

use super::*;

const DOF_INDEX_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const DOF_INDEX_PRIME: u64 = 0x100_0000_01b3;

/// 🔢️ Numbers each node's active DOFs (the union of `dofs_per_node()` over elements touching it). The order and the equation index
/// are funded paged owners: a lookup is one hashed probe sequence over `slots`, never a scan of the whole order, and every owner
/// enters its reserved page one step at a time.
pub struct DofMap {
    pub order: AssemblyOrder<(String, Dof)>,
    pub slots: AssemblyOrder<Option<usize>>,
}

enum DofProbe {
    Found { slot: usize, index: usize },
    Vacant(usize),
    Exhausted,
}

impl DofMap {
    /// 📏️ The index size that keeps its load at most one half for `maximum_dofs` owners, or `None` when it overflows.
    pub fn slot_count(maximum_dofs: usize) -> Option<usize> {
        maximum_dofs.checked_mul(2)
    }

    /// #️⃣️ The index hash: FNV-1a, 64 bit, over the UTF-8 bytes of the node id followed by the one DOF index byte.
    pub fn hash(node_id: &str, dof: Dof) -> u64 {
        node_id.bytes().chain([dof.index() as u8]).fold(DOF_INDEX_OFFSET, |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(DOF_INDEX_PRIME))
    }

    /// 🧊️ Moves a cold ordered `(node, dof)` run into paged owners and indexes it, so a lookup is one probe sequence instead of a
    /// scan of the whole order — the scan made every assembly and recovery pass quadratic in the mesh (`elements × dofs × order`),
    /// which is what dominated a two-thousand-node continuum model before the factorization even started.
    pub fn from_order(order: Vec<(String, Dof)>) -> Self {
        let order: AssemblyOrder<(String, Dof)> = order.into_iter().collect();
        let slot_count = Self::slot_count(order.len()).expect("DOF index size fits the address space");
        let mut map = Self { order, slots: std::iter::repeat_n(None, slot_count).collect() };
        for index in 0..map.order.len() {
            let (node_id, dof) = &map.order[index];
            let slot = match map.probe(node_id, *dof) {
                DofProbe::Found { slot, .. } | DofProbe::Vacant(slot) => slot,
                DofProbe::Exhausted => unreachable!("a DOF index sized for its order always has a free slot"),
            };
            map.slots[slot] = Some(index);
        }
        map
    }

    fn probe(&self, node_id: &str, dof: Dof) -> DofProbe {
        let count = self.slots.len();
        if count == 0 {
            return DofProbe::Exhausted;
        }
        let start = (Self::hash(node_id, dof) % count as u64) as usize;
        for step in 0..count {
            let slot = (start + step) % count;
            let Some(Some(index)) = self.slots.get(slot).copied() else {
                return DofProbe::Vacant(slot);
            };
            if self.order.get(index).is_some_and(|(id, owner)| id == node_id && *owner == dof) {
                return DofProbe::Found { slot, index };
            }
        }
        DofProbe::Exhausted
    }

    pub fn get(&self, node_id: &str, dof: Dof) -> Option<usize> {
        match self.probe(node_id, dof) {
            DofProbe::Found { index, .. } => Some(index),
            DofProbe::Vacant(_) | DofProbe::Exhausted => None,
        }
    }

    /// 🔢️ Appends one `(node, dof)` owner into the reserved order page and its index slot — the mounted construction emits the
    /// order one owner per step. A missing reserved slot, a duplicate owner or a full index hands the owner back unchanged.
    pub fn push(&mut self, owner: (String, Dof)) -> Result<(), (String, Dof)> {
        if !self.order.has_reserved_slot() {
            return Err(owner);
        }
        let DofProbe::Vacant(slot) = self.probe(&owner.0, owner.1) else {
            return Err(owner);
        };
        let index = self.order.len();
        self.order.push_reserved(owner)?;
        self.slots[slot] = Some(index);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.order.len()
    }

    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }
}
