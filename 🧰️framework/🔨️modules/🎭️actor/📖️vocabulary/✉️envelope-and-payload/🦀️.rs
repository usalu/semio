//! ✉️ Envelope and payload: the window identity, the origin, the payload, the coalesce key and the envelope. See [`super`].

use super::*;

//#region ✉️Envelope
/// 🪟 Local, opaque window identifier ([`Origin::Ui`]'s target) — the concrete `WindowHandle`
/// lives in the kernel crate; this crate only ever routes by this bare numeric id.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value", transparent)]
pub struct WindowId(pub u32);

impl WindowId {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        semio_framework_actor_pack::write_u32(out, self.0).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        Ok(Self(semio_framework_actor_pack::read_u32(bytes, pos, "WindowId").await?))
    }
}

/// ✉️ Who sent an [`Envelope`].
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::semio_framework_value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Origin {
    Ui {
        window: WindowId,
    },
    /// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-shard-grants, Part A): struct variant, not
    /// the newtype `Actor(ActorId)` this used to be — serde's internal tagging (`#[serde(tag =
    /// "kind")]`) cannot serialize a newtype variant whose payload is not itself a map (`ActorId` is
    /// a bare `u64` tuple struct). Latent, not live (this crate has no `serde_json` dependency and
    /// the wire uses `pack_encode`/`pack_decode`), but the generated TS mirror rendered this as an
    /// impossible `{"kind":"actor"} & bigint` intersection — see `📓️luna-serde-newtype-audit.md`.
    Actor {
        id: ActorId,
    },
    Kernel,
    Bus {
        topic: String,
    },
}

impl Origin {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            Origin::Ui { window } => {
                semio_framework_actor_pack::write_u8(out, 0).await;
                window.pack_encode(out).await;
            }
            Origin::Actor { id } => {
                semio_framework_actor_pack::write_u8(out, 1).await;
                id.pack_encode(out).await;
            }
            Origin::Kernel => semio_framework_actor_pack::write_u8(out, 2).await,
            Origin::Bus { topic } => {
                semio_framework_actor_pack::write_u8(out, 3).await;
                semio_framework_actor_pack::write_str(out, topic).await;
            }
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        let tag = semio_framework_actor_pack::read_u8(bytes, pos, "Origin").await?;
        match tag {
            0 => Ok(Origin::Ui { window: WindowId::pack_decode(bytes, pos).await? }),
            1 => Ok(Origin::Actor { id: ActorId::pack_decode(bytes, pos).await? }),
            2 => Ok(Origin::Kernel),
            3 => Ok(Origin::Bus { topic: semio_framework_actor_pack::read_str(bytes, pos, "Origin::topic").await? }),
            other => Err(semio_framework_actor_pack::PackError::InvalidTag { what: "Origin", tag: other, offset: *pos }),
        }
    }
}

/// ✉️ The message body an [`Envelope`] carries. `Event` is an opaque pack-encoded blob of the
/// kernel crate's concrete `Event` type — see the module-level seam docstring.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::semio_framework_value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Payload {
    /// 🫴️ The Kernel supplies a new actual input for the Event held by the Shard.
    ContinueOriginalCall,
    /// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-shard-grants, Part A): struct variant, not
    /// the newtype `Event(Vec<u8>)` this used to be — serde's internal tagging cannot serialize a
    /// newtype variant whose payload is a sequence (`serde_json` errors "cannot serialize tagged
    /// newtype variant ... containing a sequence", the exact defect `JobStep::Done`/`Failed` hit
    /// earlier this ticket). Latent, not live, on THIS crate's own pack wire — see [`Origin::Actor`]'s
    /// doc for the full context and `📓️luna-serde-newtype-audit.md`.
    Event {
        bytes: Vec<u8>,
    },
    Suspend {
        operation: JobOperation,
        applied_progress: u64,
    },
    Resume {
        operation: JobOperation,
        checkpoint: JobCheckpoint,
    },
    /// 🐛️ Same class of bug as [`Payload::Event`], payload is an integer this time (`serde_json`:
    /// "cannot serialize tagged newtype variant ... containing an integer") — struct variant, not
    /// the newtype `Cancel(u64)` this used to be.
    Cancel {
        seq: u64,
    },
    JobStep {
        turn: JobTurn,
    },
    JobReplay {
        turn: JobTurn,
        request: JobReplayRequest,
        worker_count: u16,
        worker_slot: u16,
    },
}

impl Payload {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            Payload::ContinueOriginalCall=>semio_framework_actor_pack::write_u8(out,6).await,
            Payload::Event { bytes } => {
                semio_framework_actor_pack::write_u8(out, 0).await;
                semio_framework_actor_pack::write_bytes(out, bytes).await;
            }
            Payload::Suspend { operation, applied_progress } => {
                semio_framework_actor_pack::write_u8(out, 1).await;
                operation.pack_encode(out).await;
                semio_framework_actor_pack::write_u64(out, *applied_progress).await;
            }
            Payload::Resume { operation, checkpoint } => {
                semio_framework_actor_pack::write_u8(out, 2).await;
                operation.pack_encode(out).await;
                checkpoint.pack_encode(out).await;
            }
            Payload::Cancel { seq } => {
                semio_framework_actor_pack::write_u8(out, 3).await;
                semio_framework_actor_pack::write_u64(out, *seq).await;
            }
            Payload::JobStep { turn } => {
                semio_framework_actor_pack::write_u8(out, 4).await;
                turn.pack_encode(out).await;
            }
            Payload::JobReplay { turn, request, worker_count, worker_slot } => {
                semio_framework_actor_pack::write_u8(out, 5).await;
                turn.pack_encode(out).await;
                request.pack_encode(out).await;
                semio_framework_actor_pack::write_u16(out, *worker_count).await;
                semio_framework_actor_pack::write_u16(out, *worker_slot).await;
            }
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        let tag = semio_framework_actor_pack::read_u8(bytes, pos, "Payload").await?;
        match tag {
            6 => Ok(Payload::ContinueOriginalCall),
            0 => Ok(Payload::Event { bytes: semio_framework_actor_pack::read_bytes(bytes, pos, "Payload::Event").await? }),
            1 => Ok(Payload::Suspend { operation: JobOperation::pack_decode(bytes, pos).await?, applied_progress: semio_framework_actor_pack::read_u64(bytes, pos, "Payload::Suspend::applied_progress").await? }),
            2 => Ok(Payload::Resume { operation: JobOperation::pack_decode(bytes, pos).await?, checkpoint: JobCheckpoint::pack_decode(bytes, pos).await? }),
            3 => Ok(Payload::Cancel { seq: semio_framework_actor_pack::read_u64(bytes, pos, "Payload::Cancel").await? }),
            4 => Ok(Payload::JobStep { turn: JobTurn::pack_decode(bytes, pos).await? }),
            5 => Ok(Payload::JobReplay {
                turn: JobTurn::pack_decode(bytes, pos).await?,
                request: JobReplayRequest::pack_decode(bytes, pos).await?,
                worker_count: semio_framework_actor_pack::read_u16(bytes, pos, "Payload::JobReplay::worker_count").await?,
                worker_slot: semio_framework_actor_pack::read_u16(bytes, pos, "Payload::JobReplay::worker_slot").await?,
            }),
            other => Err(semio_framework_actor_pack::PackError::InvalidTag { what: "Payload", tag: other, offset: *pos }),
        }
    }
}

//#region 🔑️CoalesceKey
/// 🔑️ Latest-wins-per-`(actor, key)` coalescing key. Pointer-move, resize, presence, refresh all
/// coalesce under this — 200 stale mouse-moves must never queue.
#[derive(Clone, Debug, PartialEq, Eq, Hash, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value", transparent)]
pub struct CoalesceKey(pub String);

impl CoalesceKey {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        semio_framework_actor_pack::write_str(out, &self.0).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        Ok(Self(semio_framework_actor_pack::read_str(bytes, pos, "CoalesceKey").await?))
    }
}
//#endregion 🔑️CoalesceKey

/// ✉️ One routed message: destination, sender, scheduling lane, an optional deadline that
/// short-circuits DRR ordering, an optional coalescing key, an optional envelope-seq this cancels,
/// and its payload.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct Envelope {
    pub to: ActorId,
    pub from: Origin,
    pub lane: Lane,
    pub seq: u64,
    pub deadline_ms: Option<u64>,
    pub coalesce: Option<CoalesceKey>,
    pub cancel_of: Option<u64>,
    pub payload: Payload,
}

impl Envelope {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        self.to.pack_encode(out).await;
        self.from.pack_encode(out).await;
        self.lane.pack_encode(out).await;
        semio_framework_actor_pack::write_u64(out, self.seq).await;
        semio_framework_actor_pack::write_opt_u64(out, &self.deadline_ms).await;
        semio_framework_actor_pack::write_bool(out, self.coalesce.is_some()).await;
        if let Some(key) = &self.coalesce {
            key.pack_encode(out).await;
        }
        semio_framework_actor_pack::write_opt_u64(out, &self.cancel_of).await;
        self.payload.pack_encode(out).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        let to = ActorId::pack_decode(bytes, pos).await?;
        let from = Origin::pack_decode(bytes, pos).await?;
        let lane = Lane::pack_decode(bytes, pos).await?;
        let seq = semio_framework_actor_pack::read_u64(bytes, pos, "Envelope::seq").await?;
        let deadline_ms = semio_framework_actor_pack::read_opt_u64(bytes, pos, "Envelope::deadline_ms").await?;
        let coalesce = if semio_framework_actor_pack::read_bool(bytes, pos, "Envelope::coalesce").await? { Some(CoalesceKey::pack_decode(bytes, pos).await?) } else { None };
        let cancel_of = semio_framework_actor_pack::read_opt_u64(bytes, pos, "Envelope::cancel_of").await?;
        let payload = Payload::pack_decode(bytes, pos).await?;
        Ok(Self { to, from, lane, seq, deadline_ms, coalesce, cancel_of, payload })
    }
}
//#endregion ✉️Envelope
