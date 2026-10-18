//! 📬️ Turn result and mailbox: the turn status, usage and result, the backpressure and the bounded mailbox. See [`super`].

use super::*;

//#region 🔁️TurnResult
/// 🔁️ How a turn left the actor.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::semio_framework_value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum TurnStatus {
    Idle,
    MoreWork,
    CheckpointReady {
        checkpoint: JobCheckpoint,
    },
    /// 🐛️ Struct variant, not the newtype `Faulted(Vec<u8>)` this used to be — same sequence-payload
    /// serde defect as [`Payload::Event`]; see that variant's doc for the full explanation.
    Faulted {
        detail: Vec<u8>,
    },
    PreviewReady {
        preview: Vec<u8>,
        sequence: u64,
    },
    CommitReady {
        candidate: JobCommitCandidate,
    },
    Cancelled,
}

impl TurnStatus {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            TurnStatus::Idle => semio_framework_actor_pack::write_u8(out, 0).await,
            TurnStatus::MoreWork => semio_framework_actor_pack::write_u8(out, 1).await,
            TurnStatus::CheckpointReady { checkpoint } => {
                semio_framework_actor_pack::write_u8(out, 2).await;
                checkpoint.pack_encode(out).await;
            }
            TurnStatus::Faulted { detail } => {
                semio_framework_actor_pack::write_u8(out, 3).await;
                semio_framework_actor_pack::write_bytes(out, detail).await;
            }
            TurnStatus::PreviewReady { preview, sequence } => {
                semio_framework_actor_pack::write_u8(out, 4).await;
                semio_framework_actor_pack::write_bytes(out, preview).await;
                semio_framework_actor_pack::write_u64(out, *sequence).await;
            }
            TurnStatus::CommitReady { candidate } => {
                semio_framework_actor_pack::write_u8(out, 5).await;
                candidate.pack_encode(out).await;
            }
            TurnStatus::Cancelled => semio_framework_actor_pack::write_u8(out, 6).await,
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        let tag = semio_framework_actor_pack::read_u8(bytes, pos, "TurnStatus").await?;
        match tag {
            0 => Ok(TurnStatus::Idle),
            1 => Ok(TurnStatus::MoreWork),
            2 => Ok(TurnStatus::CheckpointReady { checkpoint: JobCheckpoint::pack_decode(bytes, pos).await? }),
            3 => Ok(TurnStatus::Faulted { detail: semio_framework_actor_pack::read_bytes(bytes, pos, "TurnStatus::Faulted").await? }),
            4 => Ok(TurnStatus::PreviewReady { preview: semio_framework_actor_pack::read_bytes(bytes, pos, "TurnStatus::PreviewReady::preview").await?, sequence: semio_framework_actor_pack::read_u64(bytes, pos, "TurnStatus::PreviewReady::sequence").await? }),
            5 => Ok(TurnStatus::CommitReady { candidate: JobCommitCandidate::pack_decode(bytes, pos).await? }),
            6 => Ok(TurnStatus::Cancelled),
            other => Err(semio_framework_actor_pack::PackError::InvalidTag { what: "TurnStatus", tag: other, offset: *pos }),
        }
    }
}

/// 📊️ What one turn actually spent.
#[derive(Clone, Copy, Debug, Default, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct Usage {
    pub fuel: u64,
    pub wall_us: u64,
    pub memory_bytes: u64,
}

impl Usage {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        semio_framework_actor_pack::write_u64(out, self.fuel).await;
        semio_framework_actor_pack::write_u64(out, self.wall_us).await;
        semio_framework_actor_pack::write_u64(out, self.memory_bytes).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        Ok(Self { fuel: semio_framework_actor_pack::read_u64(bytes, pos, "Usage::fuel").await?, wall_us: semio_framework_actor_pack::read_u64(bytes, pos, "Usage::wall_us").await?, memory_bytes: semio_framework_actor_pack::read_u64(bytes, pos, "Usage::memory_bytes").await? })
    }
}

/// 🔁️ What a `GuestRuntime::execute_turn` (packet B1) hands back to the kernel. `ui_patches`/
/// `effects` are opaque pack-encoded `Vec<UiPatch>`/`Vec<Effect>` blobs — see the module seam docstring.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct TurnResult {
    pub retained_receipt: RetainedTurnReceipt,
    pub ui_patches: Vec<u8>,
    pub effects: Vec<u8>,
    pub command_ingress: Vec<u8>,
    pub cold_pair_ingress: cold_pair::ColdPairIngressStatus,
    pub lifecycle_receipt: Option<instance_lifetime::ActorInstanceLifecycleReceipt>,
    pub ui_patch_receipt: Option<instance_lifetime::ActorUiPatchReceipt>,
    pub next_wake: Option<u64>,
    pub status: TurnStatus,
    pub usage: Usage,
}

impl TurnResult {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) -> Result<(), semio_framework_actor_pack::PackError> {
        let mut receipt = [0; instance_lifetime::ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES];
        let length = match self.lifecycle_receipt {
            Some(value) => instance_lifetime::ActorInstanceLifecycleWire::Receipt(value).encode(&mut receipt).map_err(semio_framework_actor_pack::PackError::InvalidLifecycle)?,
            None => 0,
        };
        self.cold_pair_ingress.validate().map_err(semio_framework_actor_pack::PackError::InvalidColdPair)?;
        instance_lifetime::ActorUiPatchReceipt::validate_pairing(self.ui_patch_receipt, usize::from(!self.ui_patches.is_empty())).map_err(semio_framework_actor_pack::PackError::InvalidUiPatchReceipt)?;
        let mut patch_receipt = [0; instance_lifetime::ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES];
        let patch_length = match self.ui_patch_receipt {
            Some(value) => value.encode(&mut patch_receipt).map_err(semio_framework_actor_pack::PackError::InvalidUiPatchReceipt)?,
            None => 0,
        };
        self.retained_receipt.pack_encode(out).await?;
        semio_framework_actor_pack::write_bytes(out, &self.ui_patches).await;
        semio_framework_actor_pack::write_bytes(out, &self.effects).await;
        semio_framework_actor_pack::write_bytes(out, &self.command_ingress).await;
        self.cold_pair_ingress.pack_encode(out).await?;
        semio_framework_actor_pack::write_bytes(out, &receipt[..length]).await;
        semio_framework_actor_pack::write_bytes(out, &patch_receipt[..patch_length]).await;
        semio_framework_actor_pack::write_opt_u64(out, &self.next_wake).await;
        self.status.pack_encode(out).await;
        self.usage.pack_encode(out).await;
        Ok(())
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        let result = Self {
            retained_receipt: RetainedTurnReceipt::pack_decode(bytes,pos).await?,
            ui_patches: semio_framework_actor_pack::read_bytes(bytes, pos, "TurnResult::ui_patches").await?,
            effects: semio_framework_actor_pack::read_bytes(bytes, pos, "TurnResult::effects").await?,
            command_ingress: semio_framework_actor_pack::read_bytes(bytes, pos, "TurnResult::command_ingress").await?,
            cold_pair_ingress: cold_pair::ColdPairIngressStatus::pack_decode(bytes, pos).await?,
            lifecycle_receipt: Self::read_lifecycle_receipt(bytes, pos).await?,
            ui_patch_receipt: Self::read_ui_patch_receipt(bytes, pos).await?,
            next_wake: semio_framework_actor_pack::read_opt_u64(bytes, pos, "TurnResult::next_wake").await?,
            status: TurnStatus::pack_decode(bytes, pos).await?,
            usage: Usage::pack_decode(bytes, pos).await?,
        };
        instance_lifetime::ActorUiPatchReceipt::validate_pairing(result.ui_patch_receipt, usize::from(!result.ui_patches.is_empty())).map_err(semio_framework_actor_pack::PackError::InvalidUiPatchReceipt)?;
        Ok(result)
    }

    async fn read_ui_patch_receipt(bytes: &[u8], pos: &mut usize) -> Result<Option<instance_lifetime::ActorUiPatchReceipt>, semio_framework_actor_pack::PackError> {
        let length = semio_framework_actor_pack::read_u8(bytes, pos, "TurnResult::ui_patch_receipt.length").await? as usize;
        if length > instance_lifetime::ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES {
            return Err(semio_framework_actor_pack::PackError::InvalidUiPatchReceipt("receipt length exceeds fixed authority"));
        }
        if length == 0 {
            return Ok(None);
        }
        let end = pos.checked_add(length).ok_or(semio_framework_actor_pack::PackError::InvalidUiPatchReceipt("receipt offset overflow"))?;
        let receipt = instance_lifetime::ActorUiPatchReceipt::decode(bytes.get(*pos..end).ok_or(semio_framework_actor_pack::PackError::Truncated(*pos, "TurnResult::ui_patch_receipt"))?).map_err(semio_framework_actor_pack::PackError::InvalidUiPatchReceipt)?;
        *pos = end;
        Ok(Some(receipt))
    }

    async fn read_lifecycle_receipt(bytes: &[u8], pos: &mut usize) -> Result<Option<instance_lifetime::ActorInstanceLifecycleReceipt>, semio_framework_actor_pack::PackError> {
        let length = semio_framework_actor_pack::read_u8(bytes, pos, "TurnResult::lifecycle_receipt.length").await? as usize;
        if length > instance_lifetime::ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES {
            return Err(semio_framework_actor_pack::PackError::InvalidLifecycle("receipt length exceeds fixed authority"));
        }
        if length == 0 {
            return Ok(None);
        }
        let end = pos.checked_add(length).ok_or(semio_framework_actor_pack::PackError::InvalidLifecycle("receipt offset overflow"))?;
        let receipt = bytes.get(*pos..end).ok_or(semio_framework_actor_pack::PackError::Truncated(*pos, "TurnResult::lifecycle_receipt"))?;
        let instance_lifetime::ActorInstanceLifecycleWire::Receipt(receipt) = instance_lifetime::ActorInstanceLifecycleWire::decode(receipt).map_err(semio_framework_actor_pack::PackError::InvalidLifecycle)? else {
            return Err(semio_framework_actor_pack::PackError::InvalidLifecycle("turn result requires a receipt"));
        };
        *pos = end;
        Ok(Some(receipt))
    }
}
//#endregion 🔁️TurnResult

//#region 📬️Mailbox
/// 🚦 What `Mailbox::enqueue` reports back — `Rejected` must always surface as a busy badge, never
/// a silent drop of a user action.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::semio_framework_value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Backpressure {
    Accept,
    Coalesced,
    /// 🐛️ Struct variant, not the newtype `Dropped(Lane)` this used to be. `Lane` itself serializes
    /// fine alone (a plain enum, `#[serde(tag = "kind")]` with unit variants), but wrapping it as a
    /// newtype payload of ANOTHER internally-tagged enum hits the same "cannot serialize tagged
    /// newtype variant" defect as [`Payload::Event`] — the payload type does not matter, only that
    /// it is not itself a map.
    Dropped {
        lane: Lane,
    },
    Rejected,
}

impl Backpressure {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            Backpressure::Accept => semio_framework_actor_pack::write_u8(out, 0).await,
            Backpressure::Coalesced => semio_framework_actor_pack::write_u8(out, 1).await,
            Backpressure::Dropped { lane } => {
                semio_framework_actor_pack::write_u8(out, 2).await;
                lane.pack_encode(out).await;
            }
            Backpressure::Rejected => semio_framework_actor_pack::write_u8(out, 3).await,
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        let tag = semio_framework_actor_pack::read_u8(bytes, pos, "Backpressure").await?;
        match tag {
            0 => Ok(Backpressure::Accept),
            1 => Ok(Backpressure::Coalesced),
            2 => Ok(Backpressure::Dropped { lane: Lane::pack_decode(bytes, pos).await? }),
            3 => Ok(Backpressure::Rejected),
            other => Err(semio_framework_actor_pack::PackError::InvalidTag { what: "Backpressure", tag: other, offset: *pos }),
        }
    }
}

/// 📬️ Bounded ring per actor: one `VecDeque` per lane (so pop honors lane priority for free), a
/// coalescing scan on enqueue, and eviction of the lowest-priority nonempty lane before a hard reject.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct Mailbox {
    pub capacity: u16,
    /// 🚫️ Excluded from the TS mirror because this field is internal runtime state, and it is
    /// module-private — no TypeScript consumer can reach the per-lane rings, which cross to the web
    /// shard as pack bytes via [`Mailbox::pack_encode`], never as a structural JSON object. Emitting
    /// them would describe a shape the wire never carries. Found the first time typegen was ever run
    /// for this crate (MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, T1's lease).
    lanes: [VecDeque<Envelope>; 4],
    pub(super) len: u16,
}

impl Mailbox {
    pub async fn new(capacity: u16) -> Self {
        Self { capacity, lanes: Default::default(), len: 0 }
    }

    // 🚫️async: E1-adjacent pure accessor — every consumer sits inside a sync `Iterator`/`Option`
    // combinator closure or a test assertion — see R9 residue shape 1.
    pub fn len(&self) -> u16 {
        self.len
    }

    // 🚫️async: E1-adjacent pure accessor — same rationale as `len` above — see R9 residue shape 1.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // 🚫️async: E1-adjacent pure accessor — its one consumer, `Kernel::mailbox_pressure` (below),
    // sits inside a sync `Option::map` closure — see R9 residue shape 1.
    pub fn pressure(&self) -> f32 {
        if self.capacity == 0 {
            return 1.0;
        }
        self.len as f32 / self.capacity as f32
    }

    /// 📬️ `latest-wins` coalescing (replaces in place, preserving queue position — coalescing must
    /// not let a hot key jump the line), then bounded-ring enqueue with lowest-priority eviction.
    pub async fn enqueue(&mut self, envelope: Envelope) -> Backpressure {
        if let Some(key) = envelope.coalesce.clone() {
            let lane_idx = envelope.lane.priority_rank().await;
            if let Some(existing) = self.lanes[lane_idx].iter_mut().find(|e| e.coalesce.as_ref() == Some(&key)) {
                *existing = envelope;
                return Backpressure::Coalesced;
            }
        }
        if self.len >= self.capacity {
            let incoming_rank = envelope.lane.priority_rank().await;
            let victim_rank = (incoming_rank + 1..4).rev().find(|&rank| !self.lanes[rank].is_empty());
            match victim_rank {
                Some(rank) => {
                    self.lanes[rank].pop_front();
                    self.len -= 1;
                    let dropped_lane = Lane::ALL[rank];
                    self.lanes[incoming_rank].push_back(envelope);
                    self.len += 1;
                    Backpressure::Dropped { lane: dropped_lane }
                }
                None => Backpressure::Rejected,
            }
        } else {
            self.lanes[envelope.lane.priority_rank().await].push_back(envelope);
            self.len += 1;
            Backpressure::Accept
        }
    }

    /// ⏭️ Highest lane-priority envelope first, FIFO within a lane.
    pub async fn pop_next(&mut self) -> Option<Envelope> {
        for lane in self.lanes.iter_mut() {
            if let Some(envelope) = lane.pop_front() {
                self.len -= 1;
                return Some(envelope);
            }
        }
        None
    }

    /// 👀️ Earliest `deadline_ms` among all queued envelopes, for scheduler preemption checks.
    // 🚫️async: E1-adjacent pure accessor — both consumers sit inside sync `Iterator::filter_map`
    // closures (`Scheduler::tick`'s deadline-preemption and wake-at scans), which cannot await —
    // see R9 residue shape 1.
    pub fn earliest_deadline(&self) -> Option<u64> {
        self.lanes.iter().flatten().filter_map(|e| e.deadline_ms).min()
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        semio_framework_actor_pack::write_u16(out, self.capacity).await;
        semio_framework_actor_pack::write_u16(out, self.len).await;
        for lane in &self.lanes {
            semio_framework_actor_pack::write_varint_u64(out, lane.len() as u64).await;
            for envelope in lane.iter() {
                envelope.pack_encode(out).await;
            }
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        let capacity = semio_framework_actor_pack::read_u16(bytes, pos, "Mailbox::capacity").await?;
        let len = semio_framework_actor_pack::read_u16(bytes, pos, "Mailbox::len").await?;
        let mut lanes: [VecDeque<Envelope>; 4] = Default::default();
        for lane in lanes.iter_mut() {
            *lane = semio_framework_actor_pack::read_vec(bytes, pos, "Mailbox::lane", Envelope::pack_decode).await?.into();
        }
        Ok(Self { capacity, lanes, len })
    }
}
//#endregion 📬️Mailbox
