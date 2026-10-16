//! 📨️ Paged original edit message ledger, fixed tickets and paid physical retirement.
use std::mem::{size_of,size_of_val};
use semio_framework_value::{DslValue,ErasedSnapshotRetirement,ToValue,ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use protocol::mutation::MutationMessageLedgerRetirement as ArtifactStoreMessageLedgerRetirement;
use protocol::mutation::ArtifactMessageSource;
use semio_framework_value_retirement_owner::{retirement_owner_demands,retirement_owner_close};
pub const ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY: usize = 8_192;

pub const ARTIFACT_EDIT_MESSAGE_ID_BYTES: usize = 256;

pub const ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES: usize = protocol::mutation::MUTATION_MESSAGE_ENTRY_BYTES;

pub const ARTIFACT_EDIT_MESSAGE_OWNER_BYTES: usize = protocol::mutation::MUTATION_MESSAGE_OWNER_BYTES;

pub const ARTIFACT_EDIT_MESSAGE_TARGET_BYTES: usize = protocol::mutation::MUTATION_MESSAGE_TARGET_BYTES;

pub const ARTIFACT_EDIT_MESSAGE_LEDGER_BYTES: usize = ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY * ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES;

pub const ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY: u16 = u16::MAX;

pub const ARTIFACT_EDIT_MESSAGE_INDEX_TOMBSTONE: u16 = u16::MAX - 1;

pub const ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE: u16 = u16::MAX;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactEditMessageTicket {
    pub slot: u16,
    pub generation: u64,
}

pub const ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS: usize = 64;

pub const ARTIFACT_EDIT_MESSAGE_SLOT_PAGES: usize = ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY / ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS;

/// 🧱 Retains fixed message ticket slots in independently admitted physical pages.
pub struct ArtifactEditMessageSlots {
    pages: [Option<Box<[std::mem::MaybeUninit<protocol::EditMessages>]>>; ARTIFACT_EDIT_MESSAGE_SLOT_PAGES],
    admitted: usize,
}

impl ArtifactEditMessageSlots {
    fn empty() -> Self { Self { pages: std::array::from_fn(|_| None), admitted: 0 } }
    fn new() -> Self { Self { pages: std::array::from_fn(|_| Some(Box::<[protocol::EditMessages]>::new_uninit_slice(ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS))), admitted: ARTIFACT_EDIT_MESSAGE_SLOT_PAGES } }
    fn len(&self) -> usize { self.admitted * ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS }
    fn is_empty(&self) -> bool { self.pages.iter().all(Option::is_none) }
    fn admission_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.admitted == ARTIFACT_EDIT_MESSAGE_SLOT_PAGES { return Ok(Default::default()); }
        let capacity_bytes = size_of::<std::mem::MaybeUninit<protocol::EditMessages>>().checked_mul(ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "message page extent overflowed"))?;
        Ok(semio_framework_value::RetirementDemand { capacity_bytes, depth: 1, ..Default::default() })
    }
    fn admit_next(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        if self.admitted == ARTIFACT_EDIT_MESSAGE_SLOT_PAGES { return Ok(Default::default()); }
        let demand = self.admission_demands()?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "message page birth requires original depth")); }
        if grant.maximum_items == 0 || grant.maximum_capacity_bytes < demand.capacity_bytes { return Ok(Default::default()); }
        self.pages[self.admitted] = Some(Box::<[protocol::EditMessages]>::new_uninit_slice(ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS));
        self.admitted += 1;
        let progress = RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() };
        semio_framework_value::retained_clone::admit_retained_clone_progress(grant, progress, "original message slot page birth")?;
        Ok(progress)
    }
    fn release_demands(&self) -> semio_framework_value::RetirementDemand {
        self.pages.iter().rev().find_map(|page| page.as_ref()).map_or(Default::default(), |page| semio_framework_value::RetirementDemand { release_bytes: size_of_val(page.as_ref()), depth: 1, ..Default::default() })
    }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let demand = self.release_demands();
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "message page release requires original depth")); }
        if grant.maximum_items == 0 || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let index = self.pages.iter().rposition(Option::is_some).expect("original message page remains retained");
        drop(self.pages[index].take());
        self.admitted = index;
        let progress = RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() };
        Ok(if self.is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }
}

impl std::ops::Index<usize> for ArtifactEditMessageSlots {
    type Output = std::mem::MaybeUninit<protocol::EditMessages>;
    fn index(&self, index: usize) -> &Self::Output { &self.pages[index / ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS].as_ref().expect("message ticket page remains admitted")[index % ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS] }
}

impl std::ops::IndexMut<usize> for ArtifactEditMessageSlots {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.pages[index / ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS].as_mut().expect("message ticket page remains admitted")[index % ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS] }
}

pub struct ArtifactEditMessageLedger {
    slots: ArtifactEditMessageSlots,
    generations: Box<[u64]>,
    occupied: Box<[bool]>,
    next: Box<[u16]>,
    previous: Box<[u16]>,
    buckets: Box<[u16]>,
    free: Box<[u16]>,
    free_len: usize,
    head: u16,
    tail: u16,
    len: usize,
}

/// 🗃️ Retains every original sparse message array until its exact physical release turn.
pub struct ArtifactEditMessageLedgerRetirement {
    original: std::mem::ManuallyDrop<ArtifactEditMessageLedger>,
    active: Option<ArtifactStoreMessageLedgerRetirement>,
}

impl ArtifactEditMessageLedgerRetirement {
    pub fn new(original: ArtifactEditMessageLedger) -> Self { Self { original: std::mem::ManuallyDrop::new(original), active: None } }
    pub fn demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.active.is_some() { return retirement_owner_demands(&self.active, body); }
        if self.original.len != 0 { return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<protocol::EditMessages>(), depth: 1, ..Default::default() }); }
        macro_rules! extent { ($field:ident) => { if !self.original.$field.is_empty() { return Ok(semio_framework_value::RetirementDemand { release_bytes: std::mem::size_of_val(self.original.$field.as_ref()), depth: 1, ..Default::default() }); } }; }
        if !self.original.slots.is_empty() { return Ok(self.original.slots.release_demands()); }
        extent!(generations); extent!(occupied); extent!(next); extent!(previous); extent!(buckets); extent!(free);
        Ok(Default::default())
    }
}

impl ErasedSnapshotRetirement for ArtifactEditMessageLedgerRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "message directory requires admitted original depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if self.active.is_some() { return retirement_owner_close(&mut self.active, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(entry) = self.original.pop() {
            self.active = Some(ArtifactStoreMessageLedgerRetirement::new(entry.edit_id, entry.messages));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        macro_rules! release { ($field:ident) => {
            if !self.original.$field.is_empty() {
                drop(std::mem::take(&mut self.original.$field));
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty }));
            }
        }; }
        if !self.original.slots.is_empty() { return self.original.slots.close_step(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        release!(generations); release!(occupied); release!(next); release!(previous); release!(buckets);
        if !self.original.free.is_empty() {
            drop(std::mem::take(&mut self.original.free));
            self.original.free_len = 0;
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty }));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }
    fn terminal_is_empty(&self) -> bool { self.active.is_none() && self.original.len == 0 && self.original.backing_is_empty() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl Drop for ArtifactEditMessageLedgerRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "original message ledger reached Drop before every backing array was physically released");
        if self.terminal_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.original); } }
    }
}

pub struct ArtifactEditMessageLedgerIter<'a> {
    ledger: &'a ArtifactEditMessageLedger,
    next: u16,
}

impl<'a> Iterator for ArtifactEditMessageLedgerIter<'a> {
    type Item = &'a protocol::EditMessages;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE {
            return None;
        }
        let slot = usize::from(self.next);
        self.next = self.ledger.next[slot];
        Some(unsafe { self.ledger.slots[slot].assume_init_ref() })
    }
}

pub struct ArtifactEditMessageLedgerRejected {
    inner: protocol::mutation::EditMessageLedgerRetirement,
    reason: &'static str,
}

impl ArtifactEditMessageLedger {
    /// 🪶 Starts allocation-free fixed ticket admission without transferring any payload.
    pub fn empty() -> Self {
        Self { slots: ArtifactEditMessageSlots::empty(), generations: Box::default(), occupied: Box::default(), next: Box::default(), previous: Box::default(), buckets: Box::default(), free: Box::default(), free_len: 0, head: ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE, tail: ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE, len: 0 }
    }
    /// 🧮 Quotes only the next original backing under independent caller policy.
    pub fn admission_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.slots.len() != ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY { return self.slots.admission_demands(); }
        let width = if self.generations.is_empty() { size_of::<u64>() } else if self.occupied.is_empty() { size_of::<bool>() } else if self.next.is_empty() || self.previous.is_empty() || self.buckets.is_empty() || self.free.is_empty() { size_of::<u16>() } else { return Ok(Default::default()); };
        let capacity_bytes = width.checked_mul(ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "message index backing extent overflowed"))?;
        Ok(semio_framework_value::RetirementDemand { capacity_bytes, depth: 1, ..Default::default() })
    }
    /// 🎟️ True only after every original ticket page and identity backing is retained.
    pub fn admission_is_complete(&self) -> bool {
        self.slots.len() == ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY && self.generations.len() == ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY && self.occupied.len() == ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY && self.next.len() == ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY && self.previous.len() == ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY && self.buckets.len() == ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY && self.free.len() == ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY
    }
    /// 📥 Admits one physical backing, retaining all previous pages on refusal.
    pub fn admit_next(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        if self.admission_is_complete() { return Ok(Default::default()); }
        if self.slots.len() != ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY { return self.slots.admit_next(grant); }
        let demand = self.admission_demands()?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "message index birth requires original depth")); }
        if grant.maximum_items == 0 || grant.maximum_capacity_bytes < demand.capacity_bytes { return Ok(Default::default()); }
        if self.generations.is_empty() { self.generations = vec![0; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice(); }
        else if self.occupied.is_empty() { self.occupied = vec![false; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice(); }
        else if self.next.is_empty() { self.next = vec![ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice(); }
        else if self.previous.is_empty() { self.previous = vec![ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice(); }
        else if self.buckets.is_empty() { self.buckets = vec![ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice(); }
        else { self.free = (0..ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY).rev().map(|slot| slot as u16).collect::<Vec<_>>().into_boxed_slice(); self.free_len = ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY; }
        let progress = RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() };
        semio_framework_value::retained_clone::admit_retained_clone_progress(grant, progress, "original message identity backing birth")?;
        Ok(progress)
    }

    pub fn new() -> Self {
        Self {
            slots: ArtifactEditMessageSlots::new(),
            generations: vec![0; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice(),
            occupied: vec![false; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice(),
            next: vec![ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice(),
            previous: vec![ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice(),
            buckets: vec![ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice(),
            free: (0..ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY).rev().map(|slot| slot as u16).collect::<Vec<_>>().into_boxed_slice(),
            free_len: ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY,
            head: ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE,
            tail: ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE,
            len: 0,
        }
    }

    pub fn try_from_entries(entries: Vec<protocol::EditMessages>) -> Result<Self, ArtifactEditMessageLedgerRejected> {
        if let Err(reason) = Self::preflight_entries(&entries) {
            return Err(ArtifactEditMessageLedgerRejected::new(entries, reason));
        }
        let mut ledger = Self::new();
        for entry in entries {
            ledger.push_reserved(entry).expect("preflighted artifact edit-message ledger slot remains admissible");
        }
        Ok(ledger)
    }

    pub fn from_preflighted_entries(entries: Vec<protocol::EditMessages>) -> Self {
        assert!(Self::preflight_entries(&entries).is_ok(), "preflighted artifact edit-message entries changed before fixed-ledger adoption");
        let mut ledger = Self::new();
        for entry in entries {
            ledger.push_reserved(entry).unwrap_or_else(|_| panic!("preflighted artifact edit-message slot remains admissible"));
        }
        ledger
    }

    fn preflight_entries(entries: &[protocol::EditMessages]) -> Result<(), &'static str> {
        if entries.len() > ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY {
            return Err("artifact edit-message ledger exceeds its exact fixed item authority");
        }
        let mut preflight_buckets = vec![ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY; ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY].into_boxed_slice();
        let mut admitted_bytes = 0usize;
        for (entry_index, entry) in entries.iter().enumerate() {
            let Some(entry_bytes) = Self::entry_bytes(entry) else {
                return Err("artifact edit-message ledger byte authority overflowed");
            };
            if entry.edit_id.len() > ARTIFACT_EDIT_MESSAGE_ID_BYTES || entry_bytes > ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES {
                return Err("artifact edit-message ledger contains an entry outside its fixed byte authority");
            }
            admitted_bytes = match admitted_bytes.checked_add(entry_bytes) {
                Some(bytes) if bytes <= ARTIFACT_EDIT_MESSAGE_LEDGER_BYTES => bytes,
                _ => return Err("artifact edit-message ledger exceeds its exact aggregate byte authority"),
            };
            let start = Self::hash(&entry.edit_id) % ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY;
            let mut admitted = false;
            for probe in 0..ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY {
                let bucket = (start + probe) % ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY;
                let established = preflight_buckets[bucket];
                if established == ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY {
                    preflight_buckets[bucket] = entry_index as u16;
                    admitted = true;
                    break;
                }
                if entries[usize::from(established)].edit_id == entry.edit_id {
                    return Err("artifact edit-message ledger contains a duplicate identity");
                }
            }
            if !admitted {
                return Err("artifact edit-message ledger fixed identity table is saturated");
            }
        }
        Ok(())
    }

    fn entry_bytes(entry: &protocol::EditMessages) -> Option<usize> {
        entry.messages.iter().try_fold(entry.edit_id.len(), |bytes, message| message.target.iter().try_fold(bytes.checked_add(ARTIFACT_EDIT_MESSAGE_OWNER_BYTES)?.checked_add(message.code.0.len())?.checked_add(message.message.len())?, |bytes, segment| bytes.checked_add(ARTIFACT_EDIT_MESSAGE_TARGET_BYTES)?.checked_add(segment.len())))
    }

    pub fn hash(id: &str) -> usize {
        id.as_bytes().iter().fold(0xcbf29ce484222325u64, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)) as usize
    }

    pub fn find_ticket(&self, id: &str) -> Option<ArtifactEditMessageTicket> {
        if !self.admission_is_complete() || id.len() > ARTIFACT_EDIT_MESSAGE_ID_BYTES {
            return None;
        }
        let start = Self::hash(id) % ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY;
        for probe in 0..ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY {
            let slot = self.buckets[(start + probe) % ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY];
            if slot == ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY {
                return None;
            }
            if slot == ARTIFACT_EDIT_MESSAGE_INDEX_TOMBSTONE {
                continue;
            }
            let slot = usize::from(slot);
            if self.occupied[slot] && unsafe { self.slots[slot].assume_init_ref() }.edit_id == id {
                return Some(ArtifactEditMessageTicket { slot: slot as u16, generation: self.generations[slot] });
            }
        }
        None
    }

    fn reserve_identity(&mut self, slot: u16, id: &str) {
        let start = Self::hash(id) % ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY;
        let mut tombstone = None;
        for probe in 0..ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY {
            let bucket = (start + probe) % ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY;
            match self.buckets[bucket] {
                ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY => {
                    self.buckets[tombstone.unwrap_or(bucket)] = slot;
                    return;
                }
                ARTIFACT_EDIT_MESSAGE_INDEX_TOMBSTONE => {
                    tombstone.get_or_insert(bucket);
                }
                _ => {}
            }
        }
        self.buckets[tombstone.expect("preflighted artifact edit-message identity has a deterministic bucket")] = slot;
    }

    fn reserved_identity_bucket(&self, slot: u16, id: &str) -> Option<usize> {
        let start = Self::hash(id) % ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY;
        for probe in 0..ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY {
            let bucket = (start + probe) % ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY;
            if self.buckets[bucket] == slot {
                return Some(bucket);
            }
            if self.buckets[bucket] == ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY {
                return None;
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> ArtifactEditMessageLedgerIter<'_> {
        ArtifactEditMessageLedgerIter { ledger: self, next: self.head }
    }

    pub fn first(&self) -> Option<&protocol::EditMessages> {
        (self.head != ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE).then(|| unsafe { self.slots[usize::from(self.head)].assume_init_ref() })
    }

    pub fn last(&self) -> Option<&protocol::EditMessages> {
        (self.tail != ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE).then(|| unsafe { self.slots[usize::from(self.tail)].assume_init_ref() })
    }

    pub fn get_by_id(&self, id: &str) -> Option<&protocol::EditMessages> {
        self.find_ticket(id).and_then(|ticket| self.get(ticket))
    }

    pub fn get_mut_by_id(&mut self, id: &str) -> Option<&mut protocol::EditMessages> {
        let ticket = self.find_ticket(id)?;
        self.get_mut(ticket)
    }

    pub fn get(&self, ticket: ArtifactEditMessageTicket) -> Option<&protocol::EditMessages> {
        let slot = usize::from(ticket.slot);
        (slot < ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY && self.occupied[slot] && self.generations[slot] == ticket.generation).then(|| unsafe { self.slots[slot].assume_init_ref() })
    }

    fn get_mut(&mut self, ticket: ArtifactEditMessageTicket) -> Option<&mut protocol::EditMessages> {
        let slot = usize::from(ticket.slot);
        (slot < ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY && self.occupied[slot] && self.generations[slot] == ticket.generation).then(|| unsafe { self.slots[slot].assume_init_mut() })
    }

    pub fn try_push(&mut self, entry: protocol::EditMessages) -> Result<ArtifactEditMessageTicket, protocol::EditMessages> {
        if !self.admission_is_complete() || self.free_len == 0 || entry.edit_id.len() > ARTIFACT_EDIT_MESSAGE_ID_BYTES || Self::entry_bytes(&entry).is_none_or(|bytes| bytes > ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES) || self.find_ticket(&entry.edit_id).is_some() {
            return Err(entry);
        }
        self.push_reserved(entry)
    }

    pub fn admit(&mut self, entry: protocol::EditMessages) -> Result<(), protocol::EditMessages> {
        self.try_push(entry).map(|_| ())
    }

    fn push_reserved(&mut self, entry: protocol::EditMessages) -> Result<ArtifactEditMessageTicket, protocol::EditMessages> {
        assert!(self.free_len > 0, "artifact edit-message ledger push lacked its exact fixed-slot admission");
        self.free_len -= 1;
        let slot = usize::from(self.free[self.free_len]);
        let Some(generation) = self.generations[slot].checked_add(1) else {
            self.free[self.free_len] = slot as u16;
            self.free_len += 1;
            return Err(entry);
        };
        self.reserve_identity(slot as u16, &entry.edit_id);
        self.slots[slot].write(entry);
        self.generations[slot] = generation;
        self.occupied[slot] = true;
        self.previous[slot] = self.tail;
        self.next[slot] = ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE;
        if self.tail == ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE {
            self.head = slot as u16;
        } else {
            self.next[usize::from(self.tail)] = slot as u16;
        }
        self.tail = slot as u16;
        self.len += 1;
        Ok(ArtifactEditMessageTicket { slot: slot as u16, generation: self.generations[slot] })
    }

    pub fn pop(&mut self) -> Option<protocol::EditMessages> {
        (self.tail != ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE).then(|| {
            let slot = usize::from(self.tail);
            self.remove(ArtifactEditMessageTicket { slot: slot as u16, generation: self.generations[slot] }).expect("validated artifact edit-message tail remains live")
        })
    }

    pub fn remove(&mut self, ticket: ArtifactEditMessageTicket) -> Option<protocol::EditMessages> {
        self.get(ticket)?;
        let slot = usize::from(ticket.slot);
        let identity_bucket = self.reserved_identity_bucket(ticket.slot, &unsafe { self.slots[slot].assume_init_ref() }.edit_id).expect("artifact edit-message identity remains reserved while its slot is live");
        let previous = self.previous[slot];
        let next = self.next[slot];
        if previous == ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE {
            self.head = next;
        } else {
            self.next[usize::from(previous)] = next;
        }
        if next == ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE {
            self.tail = previous;
        } else {
            self.previous[usize::from(next)] = previous;
        }
        self.occupied[slot] = false;
        self.buckets[identity_bucket] = ARTIFACT_EDIT_MESSAGE_INDEX_TOMBSTONE;
        self.next[slot] = ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE;
        self.previous[slot] = ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE;
        self.free[self.free_len] = ticket.slot;
        self.free_len += 1;
        self.len -= 1;
        Some(unsafe { self.slots[slot].assume_init_read() })
    }

    pub fn remove_id(&mut self, id: &str) -> Option<protocol::EditMessages> {
        let ticket = self.find_ticket(id)?;
        self.remove(ticket)
    }

    pub fn backing_is_empty(&self) -> bool { self.slots.is_empty() && self.generations.is_empty() && self.occupied.is_empty() && self.next.is_empty() && self.previous.is_empty() && self.buckets.is_empty() && self.free.is_empty() }
    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && (self.free_len == ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY || (self.free_len == 0 && self.backing_is_empty()))
    }
}

impl Default for ArtifactEditMessageLedger {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for &'a ArtifactEditMessageLedger {
    type Item = &'a protocol::EditMessages;
    type IntoIter = ArtifactEditMessageLedgerIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl std::fmt::Debug for ArtifactEditMessageLedger {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}

impl PartialEq for ArtifactEditMessageLedger {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.iter().zip(other.iter()).all(|(left, right)| left == right)
    }
}

impl Eq for ArtifactEditMessageLedger {}

impl ToValue for ArtifactEditMessageLedger {
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(ToValue::to_value).collect())
    }
}

impl ArtifactEditMessageLedgerRejected {
    /// 🔎️ Keeps the canonical rejected row owner visible through an immutable borrow.
    pub fn retained_entries(&self) -> &[protocol::EditMessages] { self.inner.retained_entries() }
    fn new(entries: Vec<protocol::EditMessages>, reason: &'static str) -> Self {
        Self { inner: protocol::mutation::EditMessageLedgerRetirement::new(entries), reason }
    }
}

impl ErasedSnapshotRetirement for ArtifactEditMessageLedgerRejected {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.inner.close_step(grant) }
    fn terminal_is_empty(&self) -> bool { self.inner.terminal_is_empty() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.inner.next_copy_byte_demand() }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { self.inner.next_capacity_byte_demand(body) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.inner.next_release_byte_demand() }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { self.inner.next_depth_demand() }
}

impl Drop for ArtifactEditMessageLedgerRejected {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty()), "rejected artifact edit-message authority reached Drop before every exact payload owner was cursor-retired: {}", self.reason);
    }
}

impl Drop for ArtifactEditMessageLedger {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty()), "artifact edit-message fixed ledger reached Drop before every exact payload owner was cursor-retired");
    }
}



#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER:semio_framework_trace_memory_testing::RequestedAllocator=semio_framework_trace_memory_testing::RequestedAllocator;
