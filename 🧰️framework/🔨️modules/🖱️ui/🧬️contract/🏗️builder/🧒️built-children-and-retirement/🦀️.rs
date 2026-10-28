//! 🧒️ Built children and retirement: the credited bindings, the built child retire authority and slots and the `BuiltChildren` paged child list with its iterators and serde. See [`super`].

use super::*;

//#region 🔖️Builder
//#region 🧱️BuiltNode
/// 🔗️ Aliases one node's action bindings for [`BuiltNode::credited_clone`], each embedded
/// [`crate::UiValue`] paying its own arena alias credit and refusing rather than overdrawing.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(super) fn credited_bindings(bindings: &crate::UiNodeBindings) -> Option<crate::UiNodeBindings> {
    let mut aliased = crate::UiNodeBindings::default();
    for binding in bindings.iter() {
        aliased.try_push(binding.credited_clone()?).ok()?;
    }
    Some(aliased)
}

type BuiltChildBacking = Box<[Option<Box<BuiltNode>>]>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct BuiltChildRetireKey {
    pub(super) slot: usize,
    pub(super) epoch: u64,
}

#[derive(Default)]
pub(super) struct BuiltChildRetireSlot {
    pub(super) epoch: u64,
    pub(super) reserved: bool,
    pub(super) owner: Option<BuiltChildRetireOwner>,
}

pub(super) struct BuiltChildRetireOwner {
    pub(super) backing: BuiltChildBacking,
    pub(super) len: usize,
    pub(super) cursor: usize,
}

pub(super) struct BuiltChildRetireAuthority {
    pub(super) slots: [BuiltChildRetireSlot; UI_BUILT_CHILD_RETIRE_SLOTS],
    reserve_cursor: usize,
    close_cursor: usize,
}

impl Default for BuiltChildRetireAuthority {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| BuiltChildRetireSlot::default()), reserve_cursor: 0, close_cursor: 0 }
    }
}

pub(super) static BUILT_CHILD_RETIRE_AUTHORITY: LazyLock<Mutex<BuiltChildRetireAuthority>> = LazyLock::new(|| Mutex::new(BuiltChildRetireAuthority::default()));
static BUILT_CHILD_AUTHORITY_PUBLISHED:std::sync::atomic::AtomicBool=std::sync::atomic::AtomicBool::new(false);

fn with_built_child_retire_authority<T>(f: impl FnOnce(&mut BuiltChildRetireAuthority) -> T) -> T {
    let source=&*BUILT_CHILD_RETIRE_AUTHORITY;
    BUILT_CHILD_AUTHORITY_PUBLISHED.store(true,std::sync::atomic::Ordering::Release);
    let mut authority = source.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    f(&mut authority)
}

impl BuiltChildRetireAuthority {
    fn reserve(&mut self) -> Option<BuiltChildRetireKey> {
        for offset in 0..UI_BUILT_CHILD_RETIRE_SLOTS {
            let slot = (self.reserve_cursor + offset) % UI_BUILT_CHILD_RETIRE_SLOTS;
            let entry = &mut self.slots[slot];
            if entry.reserved || entry.owner.is_some() {
                continue;
            }
            let epoch = entry.epoch.checked_add(1)?;
            entry.epoch = epoch;
            entry.reserved = true;
            self.reserve_cursor = (slot + 1) % UI_BUILT_CHILD_RETIRE_SLOTS;
            return Some(BuiltChildRetireKey { slot, epoch });
        }
        None
    }

    pub(super) fn release(&mut self, key: BuiltChildRetireKey) {
        let entry = &mut self.slots[key.slot];
        if entry.epoch == key.epoch && entry.owner.is_none() {
            entry.reserved = false;
        }
    }

    fn publish(&mut self, key: BuiltChildRetireKey, owner: BuiltChildRetireOwner) {
        let entry = &mut self.slots[key.slot];
        entry.epoch = key.epoch;
        entry.reserved = true;
        entry.owner = Some(owner);
    }

    fn take_close_page(&mut self) -> Option<BuiltNode> {
        for offset in 0..UI_BUILT_CHILD_RETIRE_SLOTS {
            let slot = (self.close_cursor + offset) % UI_BUILT_CHILD_RETIRE_SLOTS;
            let entry = &mut self.slots[slot];
            let Some(owner) = entry.owner.as_mut() else { continue };
            while owner.cursor < owner.len {
                let cursor = owner.cursor;
                let next = owner.cursor.checked_add(1)?;
                owner.cursor = next;
                if let Some(node) = owner.backing[cursor].take() {
                    self.close_cursor = slot;
                    return Some(*node);
                }
            }
            entry.owner = None;
            entry.reserved = false;
            self.close_cursor = (slot + 1) % UI_BUILT_CHILD_RETIRE_SLOTS;
            return None;
        }
        None
    }

    fn is_terminal_empty(&self) -> bool {
        self.slots.iter().all(|slot| !slot.reserved && slot.owner.is_none())
    }

    fn is_close_queue_empty(&self) -> bool {
        self.slots.iter().all(|slot| slot.owner.is_none())
    }
}

/// 🧹️ Retires one queued built-child page and reports whether the close queue is now empty. Live
/// reservations (trees still mounted) do not count: a closing owner waits only for the pages handed
/// back for retirement, never for every other surface to unmount.
pub fn close_built_node_page_one() -> bool {
    let node = with_built_child_retire_authority(BuiltChildRetireAuthority::take_close_page);
    drop(node);
    with_built_child_retire_authority(|authority| authority.is_close_queue_empty())
}

/// 🏁️ True once no built-child page is reserved or queued anywhere — the shutdown witness.
pub fn built_node_pages_are_terminal_empty() -> bool {
    with_built_child_retire_authority(|authority| authority.is_terminal_empty())
}

/// ♻️ Reclaims ONE retirement slot by draining the queue in place, for a builder that would otherwise
/// be refused while reclaimable credit sits unpumped.
///
/// [`BUILT_CHILD_RETIRE_AUTHORITY`] is a credit pool, not a free list: a dropped page publishes its
/// backing into the slot it already holds and the slot is released only when a pump
/// ([`close_built_node_page_one`]) drains that owner. The pump is owned by the reactor turn and the
/// reconciler, so every OTHER owner of built pages — a panel kit assembling a probe tree, an
/// out-of-turn assembly helper, a second plugin runtime sharing the process — abandons its slots for
/// good, and the refusal then lands on an unrelated later builder as `ui.fixed-capacity`, at whatever
/// stage happened to run once the 384th slot went. An admission must never refuse against credit it
/// can itself reclaim, so exhaustion drains before it refuses and only a pool whose every slot backs
/// a LIVE tree still says no.
///
/// Draining is bounded by exactly one page: a fully drained owner releases its slot, and the nodes it
/// releases republish into the slots they already hold — [`BuiltChildRetireAuthority::reserve`] is the
/// only claimant, so the pool cannot grow while it is being drained. Every node is dropped outside the
/// authority lock, because dropping one republishes its own children through the same non-reentrant
/// mutex.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn reclaim_built_child_retire_slot() -> Option<BuiltChildRetireKey> {
    loop {
        let drained = with_built_child_retire_authority(BuiltChildRetireAuthority::take_close_page);
        let handed_back = drained.is_some();
        drop(drained);
        if let Some(handback) = with_built_child_retire_authority(BuiltChildRetireAuthority::reserve) {
            return Some(handback);
        }
        if !handed_back && with_built_child_retire_authority(|authority| authority.is_close_queue_empty()) {
            return None;
        }
    }
}

#[derive(Default)]
pub struct BuiltChildren {
    pub(super) backing: Option<BuiltChildBacking>,
    pub(super) len: usize,
    pub(super) handback: Option<BuiltChildRetireKey>,
}

impl fmt::Debug for BuiltChildren {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("BuiltChildren").field("len", &self.len).finish()
    }
}

impl BuiltChildren {
    /// 🎟️ Declares the next exact native backing or child box before original receiving may construct it.
    pub fn initialized_push_demand(&self)->semio_framework_value::RetirementDemand{
        use std::mem::size_of;
        let backing=UI_BUILT_CHILDREN_MAX*size_of::<Option<Box<BuiltNode>>>();
        semio_framework_value::RetirementDemand{copy_bytes:if self.backing.is_none(){backing+size_of::<Self>()}else{2*size_of::<BuiltNode>()+size_of::<Option<Box<BuiltNode>>>()},capacity_bytes:if self.backing.is_none(){backing}else{size_of::<BuiltNode>()},depth:2,..Default::default()}
    }
    /// 🧷️ Retains every born owner before returning its original receipt; never reclaims unrelated children.
    pub fn push_initialized(&mut self,donor:&mut Option<BuiltNode>,native:&mut semio_framework_value::NativeDecodeControl<'_>,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneProgress,semio_framework_value::ValueError>{
        use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
        let invalid=|reason|ValueError::literal(ValueRefusalKind::OwnershipLimit,reason);
        if donor.is_none()||self.len==UI_BUILT_CHILDREN_MAX{return Err(invalid("staging child source capacity refused"));}
        let demand=self.initialized_push_demand();
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(Default::default());}
        if self.backing.is_none(){
            if !BUILT_CHILD_AUTHORITY_PUBLISHED.load(std::sync::atomic::Ordering::Acquire){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"staging child source is not initialized"));}
            let mut authority=match BUILT_CHILD_RETIRE_AUTHORITY.try_lock(){Ok(authority)=>authority,Err(std::sync::TryLockError::WouldBlock)=>return Ok(Default::default()),Err(std::sync::TryLockError::Poisoned(_))=>return Err(invalid("staging child source is poisoned"))};
            let key=authority.reserve().ok_or_else(||invalid("staging child retirement capacity refused"))?;
            let mut backing=match native.allocate_vec::<Option<Box<BuiltNode>>>(UI_BUILT_CHILDREN_MAX){Ok(backing)=>backing,Err(error)=>{authority.release(key);return Err(error);}};
            let capacity=backing.capacity()*std::mem::size_of::<Option<Box<BuiltNode>>>();backing.resize_with(UI_BUILT_CHILDREN_MAX,||None);
            self.backing=Some(backing.into_boxed_slice());self.handback=Some(key);
            return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:capacity,..Default::default()});
        }
        let mut owner=native.allocate_vec::<BuiltNode>(1)?;let capacity=owner.capacity()*std::mem::size_of::<BuiltNode>();
        owner.push(donor.take().unwrap());let raw=Box::into_raw(owner.into_boxed_slice())as*mut BuiltNode;
        let node=unsafe{Box::from_raw(raw)};self.backing.as_mut().unwrap()[self.len]=Some(node);self.len+=1;
        Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:capacity,..Default::default()})
    }
    #[expect(clippy::result_large_err, reason = "A refused child admission returns the exact unboxed node before obtaining backing or retirement credit.")]
    pub fn try_push(&mut self, node: BuiltNode) -> Result<(), BuiltNode> {
        if self.len == UI_BUILT_CHILDREN_MAX {
            return Err(node);
        }
        if self.backing.is_none() {
            let reserved = with_built_child_retire_authority(BuiltChildRetireAuthority::reserve);
            let Some(handback) = reserved.or_else(reclaim_built_child_retire_slot) else { return Err(node) };
            let mut backing = Vec::with_capacity(UI_BUILT_CHILDREN_MAX);
            backing.resize_with(UI_BUILT_CHILDREN_MAX, || None);
            self.backing = Some(backing.into_boxed_slice());
            self.handback = Some(handback);
        }
        let Some(backing) = self.backing.as_mut() else { return Err(node) };
        let Some(slot) = backing.get_mut(self.len) else { return Err(node) };
        if slot.is_some() {
            return Err(node);
        }
        let Some(len) = self.len.checked_add(1) else { return Err(node) };
        *slot = Some(Box::new(node));
        self.len = len;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<BuiltNode> {
        let index = self.len.checked_sub(1)?;
        self.len = index;
        self.backing.as_mut()?.get_mut(index)?.take().map(|node| *node)
    }

    /// 🧾️ A second owner of the same children, each level paying its own retirement reservation and
    /// every embedded `UiValue` its own arena alias credit — never `Clone`, which would duplicate one
    /// [`BuiltChildRetireKey`] into two owners and publish the same slot twice. Refuses when either
    /// credit is exhausted, so an alias is a bounded admission and never a silent overdraft.
    pub fn credited_clone(&self) -> Option<Self> {
        let mut aliased = Self::default();
        for child in self.iter() {
            aliased.try_push(child.credited_clone()?).ok()?;
        }
        Some(aliased)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.backing.as_ref().map_or(0, |_| UI_BUILT_CHILDREN_MAX)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: usize) -> Option<&BuiltNode> {
        (index < self.len).then(|| self.backing.as_ref()?.get(index)?.as_deref()).flatten()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut BuiltNode> {
        if index >= self.len {
            return None;
        }
        self.backing.as_mut()?.get_mut(index)?.as_deref_mut()
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &BuiltNode> {
        self.backing.iter().flat_map(|backing| backing[..self.len].iter()).filter_map(Option::as_deref)
    }

    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut BuiltNode> {
        let len = self.len;
        self.backing.iter_mut().flat_map(move |backing| backing[..len].iter_mut()).filter_map(Option::as_deref_mut)
    }
}

impl Index<usize> for BuiltChildren {
    type Output = BuiltNode;

    fn index(&self, index: usize) -> &Self::Output {
        self.backing.as_ref().and_then(|backing| backing.get(index)).and_then(Option::as_deref).expect("built child index must be admitted")
    }
}

impl IndexMut<usize> for BuiltChildren {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.backing.as_mut().and_then(|backing| backing.get_mut(index)).and_then(Option::as_deref_mut).expect("built child index must be admitted")
    }
}

impl<'a> IntoIterator for &'a BuiltChildren {
    type Item = &'a BuiltNode;
    type IntoIter = std::iter::FilterMap<std::slice::Iter<'a, Option<Box<BuiltNode>>>, fn(&Option<Box<BuiltNode>>) -> Option<&BuiltNode>>;

    fn into_iter(self) -> Self::IntoIter {
        fn present(value: &Option<Box<BuiltNode>>) -> Option<&BuiltNode> {
            value.as_deref()
        }
        self.backing.as_deref().map_or([].as_slice(), |backing| &backing[..self.len]).iter().filter_map(present)
    }
}

impl<'a> IntoIterator for &'a mut BuiltChildren {
    type Item = &'a mut BuiltNode;
    type IntoIter = std::iter::FilterMap<std::slice::IterMut<'a, Option<Box<BuiltNode>>>, fn(&mut Option<Box<BuiltNode>>) -> Option<&mut BuiltNode>>;

    fn into_iter(self) -> Self::IntoIter {
        fn present(value: &mut Option<Box<BuiltNode>>) -> Option<&mut BuiltNode> {
            value.as_deref_mut()
        }
        let len = self.len;
        self.backing.as_deref_mut().map_or([].as_mut_slice(), |backing| &mut backing[..len]).iter_mut().filter_map(present)
    }
}

pub struct BuiltChildrenIntoIter {
    pub(super) backing: Option<BuiltChildBacking>,
    pub(super) len: usize,
    pub(super) cursor: usize,
    pub(super) handback: Option<BuiltChildRetireKey>,
}

impl Iterator for BuiltChildrenIntoIter {
    type Item = BuiltNode;

    fn next(&mut self) -> Option<Self::Item> {
        while self.cursor < self.len {
            let cursor = self.cursor;
            let next = self.cursor.checked_add(1)?;
            self.cursor = next;
            if let Some(node) = self.backing.as_mut()?.get_mut(cursor)?.take() {
                return Some(*node);
            }
        }
        if let Some(handback) = self.handback.take() {
            with_built_child_retire_authority(|authority| authority.release(handback));
        }
        self.backing = None;
        None
    }
}

impl ExactSizeIterator for BuiltChildrenIntoIter {
    fn len(&self) -> usize {
        self.len.saturating_sub(self.cursor)
    }
}

impl Drop for BuiltChildrenIntoIter {
    fn drop(&mut self) {
        if let (Some(backing), Some(handback)) = (self.backing.take(), self.handback.take()) {
            with_built_child_retire_authority(|authority| authority.publish(handback, BuiltChildRetireOwner { backing, len: self.len, cursor: self.cursor }));
        }
    }
}

impl IntoIterator for BuiltChildren {
    type Item = BuiltNode;
    type IntoIter = BuiltChildrenIntoIter;

    fn into_iter(mut self) -> Self::IntoIter {
        BuiltChildrenIntoIter { backing: self.backing.take(), len: self.len, cursor: 0, handback: self.handback.take() }
    }
}

impl Drop for BuiltChildren {
    fn drop(&mut self) {
        if let (Some(backing), Some(handback)) = (self.backing.take(), self.handback.take()) {
            with_built_child_retire_authority(|authority| authority.publish(handback, BuiltChildRetireOwner { backing, len: self.len, cursor: 0 }));
        }
    }
}

impl Serialize for BuiltChildren {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::{Error, SerializeSeq};
        if !self.is_empty() {
            return Err(S::Error::custom("BuiltChildren requires retained page transport"));
        }
        serializer.serialize_seq(Some(0))?.end()
    }
}

impl<'de> Deserialize<'de> for BuiltChildren {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{SeqAccess, Visitor};
        struct BuiltChildrenVisitor;
        impl<'de> Visitor<'de> for BuiltChildrenVisitor {
            type Value = BuiltChildren;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("empty built children; populated pages require retained transport")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                if access.next_element::<BuiltNode>()?.is_some() {
                    return Err(serde::de::Error::custom("BuiltChildren requires retained page transport"));
                }
                Ok(BuiltChildren::default())
            }
        }
        deserializer.deserialize_seq(BuiltChildrenVisitor)
    }
}
//#endregion 🧱️BuiltNode
//#endregion 🔖️Builder
