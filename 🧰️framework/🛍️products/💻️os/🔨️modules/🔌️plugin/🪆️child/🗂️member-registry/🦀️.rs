//! 🗂️ Member registry of the child members: the registry entries, admissions and registration errors and the child member registry with its drop. See [`super`].

use super::*;

#[doc(hidden)]
pub struct ChildMemberEntry<M> {
    pub reference: ArtifactRef,
    pub owner: store::OwnerRef,
    pub member: M,
}

#[doc(hidden)]
pub struct ChildMemberAdmission {
    pub index: usize,
    pub generation: u64,
    pub nested: bool,
}

pub struct ChildMemberRegistrationError<M> {
    pub member: Option<M>,
    pub fault: Fault,
}

impl<M> std::fmt::Debug for ChildMemberRegistrationError<M> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("ChildMemberRegistrationError").field("owner_returned", &self.member.is_some()).field("fault", &self.fault).finish()
    }
}

#[doc(hidden)]
pub struct ChildMemberRegistry<M> {
    pub slots: Box<[std::mem::MaybeUninit<ChildMemberEntry<M>>]>,
    pub occupied: [u64; CHILD_CONTENT_SLOTS / 64],
    pub reserved: [u64; CHILD_CONTENT_SLOTS / 64],
    pub nested: [u64; CHILD_CONTENT_SLOTS / 64],
    pub generations: [u64; CHILD_CONTENT_SLOTS],
    pub dense_slots: [usize; CHILD_CONTENT_SLOTS],
    ordinal_by_slot: [usize; CHILD_CONTENT_SLOTS],
    pub next_generation: u64,
    pub len: usize,
    pub allocation_admitted: bool,
}

impl<M> ChildMemberRegistry<M> {
    pub fn new() -> Self {
        let mut slots = Vec::new();
        let allocation_admitted = slots.try_reserve_exact(CHILD_CONTENT_SLOTS).is_ok();
        if allocation_admitted {
            slots.resize_with(CHILD_CONTENT_SLOTS, std::mem::MaybeUninit::uninit);
        }
        Self {
            slots: slots.into_boxed_slice(),
            occupied: [0; CHILD_CONTENT_SLOTS / 64],
            reserved: [0; CHILD_CONTENT_SLOTS / 64],
            nested: [0; CHILD_CONTENT_SLOTS / 64],
            generations: [0; CHILD_CONTENT_SLOTS],
            dense_slots: [usize::MAX; CHILD_CONTENT_SLOTS],
            ordinal_by_slot: [usize::MAX; CHILD_CONTENT_SLOTS],
            next_generation: 0,
            len: 0,
            allocation_admitted,
        }
    }

    pub(crate) fn hash(key: MemberKeyRef<'_>) -> Result<usize, Fault> {
        ChildContentView::hash(key)
    }

    pub fn occupied(&self, index: usize) -> bool {
        self.occupied[index / 64] & (1 << (index % 64)) != 0
    }

    pub fn reserved(&self, index: usize) -> bool {
        self.reserved[index / 64] & (1 << (index % 64)) != 0
    }

    pub fn entry(&self, index: usize) -> Option<&ChildMemberEntry<M>> {
        self.occupied(index).then(|| unsafe { self.slots[index].assume_init_ref() })
    }

    pub fn entry_mut(&mut self, index: usize) -> Option<&mut ChildMemberEntry<M>> {
        if !self.occupied(index) {
            return None;
        }
        Some(unsafe { self.slots[index].assume_init_mut() })
    }

    pub fn nested(&self, index: usize) -> bool {
        self.nested[index / 64] & (1 << (index % 64)) != 0
    }

    /// 🧷️ The owner edge of the member at one occupied slot.
    pub fn key_at(&self, index: usize) -> Option<MemberKeyRef<'_>> {
        let entry = self.entry(index)?;
        Some(MemberKeyRef { owner: if self.nested(index) { entry.owner.parent.artifact_id.as_str() } else { "" }, slot: &entry.owner.slot, child_id: &entry.owner.child_id })
    }

    pub fn locate(&self, key: MemberKeyRef<'_>) -> Result<Result<usize, usize>, Fault> {
        if !self.allocation_admitted {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-member-allocation"), "fixed child-member registry allocation was not pre-admitted"));
        }
        let start = Self::hash(key)?;
        for offset in 0..CHILD_CONTENT_SLOTS {
            let index = (start + offset) % CHILD_CONTENT_SLOTS;
            match self.key_at(index) {
                Some(candidate) if candidate == key => return Ok(Ok(index)),
                Some(_) => {}
                None if self.reserved(index) => {}
                None => return Ok(Err(index)),
            }
        }
        Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-member-capacity"), "fixed child-member registry is saturated"))
    }

    pub fn admit_identity(&mut self, key: MemberKeyRef<'_>) -> Result<ChildMemberAdmission, Fault> {
        if !self.allocation_admitted {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-member-allocation"), "fixed child-member registry allocation was not pre-admitted"));
        }
        match self.locate(key)? {
            Ok(_) => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-member-duplicate"), "fixed child-member identity is already occupied")),
            Err(index) => {
                let generation = self.next_generation.checked_add(1).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-member-generation"), "fixed child-member admission generation exhausted"))?;
                self.next_generation = generation;
                self.generations[index] = generation;
                self.reserved[index / 64] |= 1 << (index % 64);
                Ok(ChildMemberAdmission { index, generation, nested: !key.owner.is_empty() })
            }
        }
    }

    pub fn admit(&mut self, key: &(String, String)) -> Result<ChildMemberAdmission, Fault> {
        self.admit_identity(MemberKeyRef::root(&key.0, &key.1))
    }

    #[expect(clippy::needless_pass_by_value, reason = "This non-Copy token conveys one-use admission authority and must be consumed by the commit, even though its identity fields are scalar.")]
    pub fn insert_admitted(&mut self, admission: ChildMemberAdmission, reference: ArtifactRef, owner: store::OwnerRef, member: M) {
        assert!(self.reserved(admission.index) && self.generations[admission.index] == admission.generation && !self.occupied(admission.index), "exact child-member admission changed while its exclusive app owner was suspended");
        self.reserved[admission.index / 64] &= !(1 << (admission.index % 64));
        self.slots[admission.index].write(ChildMemberEntry { reference, owner, member });
        self.occupied[admission.index / 64] |= 1 << (admission.index % 64);
        if admission.nested {
            self.nested[admission.index / 64] |= 1 << (admission.index % 64);
        }
        self.dense_slots[self.len] = admission.index;
        self.ordinal_by_slot[admission.index] = self.len;
        self.len += 1;
    }

    /// 🌻️ The member the document itself owns at one `(slot, child_id)` step.
    pub fn get(&self, key: &(String, String)) -> Option<&ChildMemberEntry<M>> {
        self.member(MemberKeyRef::root(&key.0, &key.1))
    }

    pub fn get_mut(&mut self, key: &(String, String)) -> Option<&mut ChildMemberEntry<M>> {
        self.member_mut(MemberKeyRef::root(&key.0, &key.1))
    }

    /// 🪆️ The member at one exact owner edge, whatever its depth.
    pub fn member(&self, key: MemberKeyRef<'_>) -> Option<&ChildMemberEntry<M>> {
        let Ok(Ok(index)) = self.locate(key) else { return None };
        self.entry(index)
    }

    pub fn member_mut(&mut self, key: MemberKeyRef<'_>) -> Option<&mut ChildMemberEntry<M>> {
        let Ok(Ok(index)) = self.locate(key) else { return None };
        self.entry_mut(index)
    }

    /// 🧾️ Every live member with its owner edge, in admission order.
    pub fn keyed_entries(&self) -> impl Iterator<Item = (MemberKeyRef<'_>, &ChildMemberEntry<M>)> {
        (0..self.len).filter_map(|ordinal| self.keyed_entry_by_ordinal(ordinal))
    }

    pub fn keyed_entry_by_ordinal(&self, ordinal: usize) -> Option<(MemberKeyRef<'_>, &ChildMemberEntry<M>)> {
        let index = *self.dense_slots[..self.len].get(ordinal)?;
        Some((self.key_at(index)?, self.entry(index)?))
    }

    /// 🧭️ The member an owner path names: one lookup per step, each under the member the step before resolved.
    pub fn resolve(&self, path: &MemberPath) -> Option<(MemberKey, &ChildMemberEntry<M>)> {
        let mut owner = "";
        let mut resolved = None;
        for step in path.steps() {
            let entry = self.member(MemberKeyRef { owner, slot: &step.slot, child_id: &step.child_id })?;
            resolved = Some((MemberKey { owner: owner.to_string(), slot: step.slot.clone(), child_id: step.child_id.clone() }, entry));
            owner = entry.reference.artifact_id.as_str();
        }
        resolved
    }

    /// 🗺️ The owner path of the member at `key`: its own step under the steps of every member that owns it; `None`
    /// while a member on the way up is not live.
    pub fn path_of(&self, key: MemberKeyRef<'_>) -> Option<MemberPath> {
        let mut steps = vec![MemberStep { slot: key.slot.to_string(), child_id: key.child_id.to_string() }];
        let mut owner = key.owner;
        while !owner.is_empty() {
            if steps.len() > self.len {
                return None;
            }
            let (above, _) = self.keyed_entries().find(|(_, entry)| entry.reference.artifact_id == owner)?;
            steps.push(MemberStep { slot: above.slot.to_string(), child_id: above.child_id.to_string() });
            owner = above.owner;
        }
        steps.reverse();
        Some(MemberPath(steps))
    }

    /// 🏷️ Every live member with the text of its owner path — the store id the history wire names it by.
    pub fn addressed_entries(&self) -> impl Iterator<Item = (String, &ChildMemberEntry<M>)> {
        self.keyed_entries().filter_map(|(key, entry)| Some((self.path_of(key)?.to_string(), entry)))
    }

    pub fn entries(&self) -> impl Iterator<Item = &ChildMemberEntry<M>> {
        self.dense_slots[..self.len].iter().map(|index| self.entry(*index).expect("dense child-member slot remains occupied"))
    }

    pub fn entries_physical(&self) -> impl Iterator<Item = &ChildMemberEntry<M>> {
        let occupied = self.occupied;
        self.slots.iter().enumerate().filter(move |(index, _)| occupied[*index / 64] & (1 << (*index % 64)) != 0).map(|(_, slot)| unsafe { slot.assume_init_ref() })
    }

    pub fn entries_mut_physical(&mut self) -> impl Iterator<Item = &mut ChildMemberEntry<M>> {
        let occupied = self.occupied;
        self.slots.iter_mut().enumerate().filter_map(move |(index, slot)| (occupied[index / 64] & (1 << (index % 64)) != 0).then(|| unsafe { slot.assume_init_mut() }))
    }

    pub(super) fn entry_by_ordinal(&self, ordinal: usize) -> Option<&ChildMemberEntry<M>> {
        (ordinal < self.len).then(|| self.entry(self.dense_slots[ordinal]).expect("dense child-member slot remains occupied"))
    }

    pub fn take_at(&mut self, index: usize) -> Option<ChildMemberEntry<M>> {
        if !self.occupied(index) {
            return None;
        }
        self.occupied[index / 64] &= !(1 << (index % 64));
        self.nested[index / 64] &= !(1 << (index % 64));
        self.len -= 1;
        let ordinal = self.ordinal_by_slot[index];
        let moved = self.dense_slots[self.len];
        self.dense_slots[ordinal] = moved;
        self.ordinal_by_slot[moved] = ordinal;
        self.dense_slots[self.len] = usize::MAX;
        self.ordinal_by_slot[index] = usize::MAX;
        Some(unsafe { self.slots[index].assume_init_read() })
    }

    pub fn cancel_admission(&mut self, admission: &ChildMemberAdmission) -> bool {
        if !self.reserved(admission.index) || self.generations[admission.index] != admission.generation {
            return false;
        }
        self.reserved[admission.index / 64] &= !(1 << (admission.index % 64));
        true
    }

    pub fn has_admission_in_flight(&self) -> bool {
        self.reserved.iter().any(|word| *word != 0)
    }

    /// 🍂️ Removes one exact member while no admission is in flight and re-seats every later entry of its probe
    /// run (backward shift), so every other member stays addressable by [`Self::locate`].
    pub fn remove(&mut self, key: MemberKeyRef<'_>) -> Result<Option<ChildMemberEntry<M>>, Fault> {
        if self.has_admission_in_flight() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-member-removal-admission"), "a child member cannot leave the fixed registry while an admission is in flight"));
        }
        let Ok(mut hole) = self.locate(key)? else { return Ok(None) };
        let entry = self.take_at(hole).expect("located child member remains occupied");
        let mut cursor = (hole + 1) % CHILD_CONTENT_SLOTS;
        while let Some(moved) = self.key_at(cursor) {
            let home = Self::hash(moved)?;
            if (cursor + CHILD_CONTENT_SLOTS - home) % CHILD_CONTENT_SLOTS >= (cursor + CHILD_CONTENT_SLOTS - hole) % CHILD_CONTENT_SLOTS {
                self.relocate(cursor, hole);
                hole = cursor;
            }
            cursor = (cursor + 1) % CHILD_CONTENT_SLOTS;
        }
        Ok(Some(entry))
    }

    fn relocate(&mut self, from: usize, to: usize) {
        let entry = unsafe { self.slots[from].assume_init_read() };
        self.slots[to].write(entry);
        self.occupied[from / 64] &= !(1 << (from % 64));
        self.occupied[to / 64] |= 1 << (to % 64);
        if self.nested(from) {
            self.nested[to / 64] |= 1 << (to % 64);
        } else {
            self.nested[to / 64] &= !(1 << (to % 64));
        }
        self.nested[from / 64] &= !(1 << (from % 64));
        self.generations[to] = self.generations[from];
        let ordinal = self.ordinal_by_slot[from];
        self.dense_slots[ordinal] = to;
        self.ordinal_by_slot[to] = ordinal;
        self.ordinal_by_slot[from] = usize::MAX;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<M> Drop for ChildMemberRegistry<M> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.is_empty(), "fixed child-member registry reached Drop before every exact member was transferred to bounded retirement");
        assert!(std::thread::panicking() || self.reserved.iter().all(|word| *word == 0), "fixed child-member registry reached Drop with a reserved owner slot outside exact admission");
        assert!(
            std::thread::panicking() || self.dense_slots.iter().all(|slot| *slot == usize::MAX) && self.ordinal_by_slot.iter().all(|ordinal| *ordinal == usize::MAX),
            "fixed child-member registry reached Drop with a stale dense owner coordinate"
        );
    }
}
