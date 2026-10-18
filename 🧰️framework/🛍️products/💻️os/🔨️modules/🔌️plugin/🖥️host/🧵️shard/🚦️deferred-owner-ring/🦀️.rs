//! 🚦️ Deferred owner ring: the shard drive, the deferred item and byte limits, the owner keys, the admission limit, the fixed owner ring, the cancel cursor, the deferred authority and the frame admission and overflow. See [`super`].

use super::*;

pub enum ShardDrive {
    Idle { consumed_epoch: Option<u64> },
    MoreWork { consumed_epoch: Option<u64> },
    Blocked,
    Fault { error: PluginHostError, consumed_epoch: Option<u64>, work_remains: bool, terminal_frame: bool, terminal_overflow: bool },
}

//#region 🚦DeferredOwnerRing
pub const SHARD_DEFERRED_ITEMS: usize = 256;

pub const SHARD_DEFERRED_BYTES: usize = 16 * 1024 * 1024;

pub const SHARD_FRAME_MAX_BYTES: usize = SHARD_DEFERRED_BYTES;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct OwnerKey {
    pub(super) slot: usize,
    pub(super) generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdmissionLimit {
    Items,
    Bytes,
}

#[derive(Debug)]
pub(crate) struct AdmissionRejected<T> {
    pub limit: AdmissionLimit,
    pub owner: T,
}

pub struct OwnerSlot<T> {
    pub(super) generation: u64,
    pub(super) bytes: usize,
    pub owner: T,
}

pub struct FixedOwnerRing<T, const N: usize> {
    pub slots: Box<[Option<OwnerSlot<T>>]>,
    pub order: Box<[usize]>,
    tail: usize,
    pub len: usize,
    pub bytes: usize,
    byte_capacity: usize,
    next_generation: u64,
}

impl<T, const N: usize> FixedOwnerRing<T, N> {
    pub fn new(byte_capacity: usize) -> Self {
        Self { slots: std::iter::repeat_with(|| None).take(N).collect(), order: vec![0; N].into_boxed_slice(), tail: 0, len: 0, bytes: 0, byte_capacity, next_generation: 1 }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn can_admit(&self, items: usize, bytes: usize) -> Result<(), AdmissionLimit> {
        if items > N.saturating_sub(self.len) {
            return Err(AdmissionLimit::Items);
        }
        if bytes > self.byte_capacity.saturating_sub(self.bytes) {
            return Err(AdmissionLimit::Bytes);
        }
        Ok(())
    }

    pub fn try_push(&mut self, owner: T, bytes: usize) -> Result<OwnerKey, AdmissionRejected<T>> {
        if self.len == N {
            return Err(AdmissionRejected { limit: AdmissionLimit::Items, owner });
        }
        if bytes > self.byte_capacity.saturating_sub(self.bytes) {
            return Err(AdmissionRejected { limit: AdmissionLimit::Bytes, owner });
        }
        let generation = self.next_generation;
        self.next_generation = self.next_generation.wrapping_add(1).max(1);
        let slot = (0..N).map(|offset| (self.tail + offset) % N).find(|slot| self.slots[*slot].is_none()).expect("FixedOwnerRing: admitted item has one empty physical slot");
        debug_assert!(self.slots[slot].is_none());
        self.slots[slot] = Some(OwnerSlot { generation, bytes, owner });
        self.order[self.len] = slot;
        self.tail = (slot + 1) % N;
        self.len += 1;
        self.bytes += bytes;
        Ok(OwnerKey { slot, generation })
    }

    pub fn pop_front(&mut self) -> Option<(OwnerKey, T)> {
        self.pop_at(0)
    }

    pub fn get(&self, offset: usize) -> Option<&T> {
        if offset >= self.len {
            return None;
        }
        self.slots[self.order[offset]].as_ref().map(|slot| &slot.owner)
    }

    pub fn pop_at(&mut self, offset: usize) -> Option<(OwnerKey, T)> {
        if self.len == 0 {
            return None;
        }
        if offset >= self.len {
            return None;
        }
        let slot = self.order[offset];
        let entry = self.slots[slot].take().expect("FixedOwnerRing: occupied head invariant");
        self.order.copy_within(offset + 1..self.len, offset);
        self.len -= 1;
        self.bytes -= entry.bytes;
        Some((OwnerKey { slot, generation: entry.generation }, entry.owner))
    }

    pub fn front(&self) -> Option<&T> {
        self.get(0)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn contains(&self, key: OwnerKey) -> bool {
        self.slots.get(key.slot).and_then(Option::as_ref).is_some_and(|slot| slot.generation == key.generation)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CancelCursor {
    pub actor: u64,
    pub after_job: Option<u64>,
    pub owner_bytes: usize,
}

#[derive(Debug)]
#[expect(clippy::large_enum_variant, reason = "The fixed owner ring retains complete event authority in preallocated slots through admission, cancellation, and retirement.")]
pub enum DeferredAuthority {
    Register { actor: ActorId },
    Unregister { actor: ActorId },
    Event { actor: u64, event: Option<Event> },
    JobStep { actor: u64, turn: JobTurn },
    JobReplay { actor: u64, turn: JobTurn, request: JobReplayRequest, worker_count: u16, worker_slot: u16 },
    Cancel(CancelCursor),
    Suspend { actor: u64, operation: JobOperation, applied_progress: u64 },
    Resume { actor: u64, operation: JobOperation, checkpoint: JobCheckpoint },
}

pub fn split_frame_credit(raw_bytes: usize, items: usize, index: usize) -> usize {
    if items == 0 {
        return 0;
    }
    raw_bytes / items + usize::from(index < raw_bytes % items)
}

/// 📥️ What admitting one frame did: its authorities joined the deferred rings, or it named an actor
/// this shard does not own and was answered with a `Fault` outcome. An answered rejection is an
/// outcome on the wire, so it spends the drive's one opportunity exactly like a granted authority.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum FrameAdmission {
    Deferred,
    Answered,
}

pub(super) enum FrameAdmissionError {
    Full { bytes: Vec<u8> },
    TerminalCapacity { bytes: Vec<u8>, error: PluginHostError },
    Fault(PluginHostError),
}

#[derive(Debug)]
pub struct TerminalFrameOverflow {
    pub epoch: u64,
    pub bytes: Vec<u8>,
}

pub(super) fn defer_completion(
    interactive: &mut FixedOwnerRing<AdmittedAuthority, SHARD_DEFERRED_ITEMS>,
    background: &mut FixedOwnerRing<AdmittedAuthority, SHARD_DEFERRED_ITEMS>,
    terminal: &mut FixedOwnerRing<DeferredAuthority, SHARD_DEFERRED_ITEMS>,
    lane: semio_framework_actor::Lane,
    allocation: Option<ShardActorAllocation>,
    budget: Option<semio_framework_actor::Budget>,
    actor: u64,
    event: Event,
) -> Result<(), PluginHostError> {
    let bytes = size_of::<Event>()
        + match &event {
            Event::JobCompleted { result: RequestOutcome::Ok(bytes) | RequestOutcome::Err(bytes), .. } => bytes.capacity(),
            _ => unreachable!("ShardLoop: only job completions use generated authority admission"),
        };
    let ring = if ShardLoop::is_high_priority_lane(lane) { interactive } else { background };
    let owner = AdmittedAuthority::new(allocation, budget, DeferredAuthority::Event { actor, event:Some(event) }, lane, bytes);
    match ring.try_push(owner, bytes) {
        Ok(_) => Ok(()),
        Err(rejected) => {
            let _ = terminal.try_push(rejected.owner.authority, bytes).expect("ShardLoop: terminal completion ring owns every rejected completion");
            Err(PluginHostError::Plugin(format!("ShardLoop: completion {:?} capacity retained one terminal event for actor {actor}", rejected.limit)))
        }
    }
}
//#endregion 🚦DeferredOwnerRing
