//! 🛰️ Server frames: the apply outcome, the ack stage, the snapshot chunk bytes and the server frame enum. See [`super`].

use super::*;

//#region 🔖️ServerFrame
/// ⚖️ How the semio_hub resolved one submitted operation against concurrent history.
#[derive(Clone, Debug, PartialEq)]
pub enum ApplyOutcome {
    Accepted,
    Transformed {
        envelope: Box<semio_framework_replication_causal::MutationEnvelope>,
    },
    /// 🧾 `messages` (trailing addition, tag unchanged) is one packed `Vec<MutationMessage>` blob —
    /// opaque here (this crate stays decoupled from `os_spr::command`'s concrete type, matching
    /// `ArtifactDiff`/`InverseMutation`'s opaque-bytes convention above), packed by the caller with
    /// `pack::encode_record_body` before construction. See contract-freeze.md §C8 of
    /// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/`.
    Rejected {
        reason: String,
        messages: Vec<u8>,
    },
}

/// 🪜️ One stage of a submitted batch's lifecycle, from `Received` to `Applied`.
#[derive(Clone, Debug, PartialEq)]
pub enum AckStage {
    Received,
    Persisted,
    Applied { outcome: Box<ApplyOutcome> },
}

pub const SNAPSHOT_CHUNK_BACKING_BYTES: usize = 4 * 1024;

/// 🧱️ One fixed-capacity snapshot frame backing whose owned allocation can never exceed one wire page.
#[derive(Clone)]
pub struct SnapshotChunkBytes {
    backing: Option<Box<[u8; SNAPSHOT_CHUNK_BACKING_BYTES]>>,
    pub(super) len: u16,
}

impl SnapshotChunkBytes {
    /// 🆕️ Allocates exactly one fixed snapshot backing.
    pub fn allocate_fixed() -> Self {
        Self { backing: Some(Box::new([0; SNAPSHOT_CHUNK_BACKING_BYTES])), len: 0 }
    }

    /// 📥️ Copies one bounded source into a fixed snapshot backing.
    pub fn try_from_slice(source: &[u8]) -> Option<Self> {
        if source.len() > SNAPSHOT_CHUNK_BACKING_BYTES {
            return None;
        }
        let mut owner = Self::allocate_fixed();
        owner.try_extend_from_slice(source).then_some(owner)
    }

    /// ➕️ Appends bytes only while the fixed backing can retain the complete source.
    pub fn try_extend_from_slice(&mut self, source: &[u8]) -> bool {
        let start = usize::from(self.len);
        let Some(end) = start.checked_add(source.len()).filter(|end| *end <= SNAPSHOT_CHUNK_BACKING_BYTES) else { return false };
        let Some(backing) = self.backing.as_mut() else { return false };
        backing[start..end].copy_from_slice(source);
        self.len = end as u16;
        true
    }

    /// 📏️ Returns the initialized snapshot byte count.
    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    /// 🩹️ Reports whether the initialized snapshot range is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// 🧩️ Borrows the initialized snapshot byte range.
    pub fn as_slice(&self) -> &[u8] {
        self.backing.as_ref().map_or(&[], |backing| &backing[..usize::from(self.len)])
    }

    /// 🧮️ Returns the exact retained fixed backing debit.
    pub fn backing_bytes(&self) -> usize {
        self.backing.as_ref().map_or(0, |backing| size_of_val(backing.as_ref()))
    }

    /// 🧹️ Retires the single fixed backing in one explicit close opportunity.
    pub fn close_one(&mut self) -> bool {
        if self.backing.take().is_none() {
            return false;
        }
        self.len = 0;
        true
    }

    /// 🏁️ Reports whether the fixed backing owner has been retired.
    pub fn terminal_is_empty(&self) -> bool {
        self.backing.is_none()
    }
}

impl std::fmt::Debug for SnapshotChunkBytes {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_tuple("SnapshotChunkBytes").field(&self.as_slice()).finish()
    }
}

impl PartialEq for SnapshotChunkBytes {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for SnapshotChunkBytes {}

/// 📦️ Artifact-bootstrap name for the same fixed-capacity wire backing, without changing private snapshot bytes.
pub type ArtifactBootstrapChunkBytes = SnapshotChunkBytes;

/// 📬️ One frame the semio_hub sends to a client.
#[derive(Clone, Debug, PartialEq)]
pub enum ServerFrame {
    Welcome {
        session_id: String,
        resume_token: String,
        server_frontier: semio_framework_replication_causal::FrontierSummary,
        bootstrap: Bootstrap,
    },
    SnapshotChunk {
        seq: u32,
        bytes: SnapshotChunkBytes,
    },
    SnapshotDone {
        seq_count: u32,
    },
    Commands {
        envelopes: Vec<semio_framework_replication_causal::MutationEnvelope>,
        origin: semio_framework_replication_ids::ActorId,
        frontier: semio_framework_replication_causal::FrontierSummary,
    },
    Ack {
        batch_id: u64,
        stages: Vec<AckStage>,
        frontier: semio_framework_replication_causal::FrontierSummary,
    },
    Preview {
        actor: semio_framework_replication_ids::ActorId,
        key: String,
        seq: u64,
        payload: Vec<u8>,
    },
    Presence {
        peers: Vec<Vec<u8>>,
    },
    CreditGrant {
        n: u32,
    },
    Error {
        code: String,
        message: String,
    },
    /// 🎨️ The hub's one-time session assignment for this connection: `color` is the
    /// hub-assigned palette index (`HubState.session_colors`, §C7.3), leased per `(space, actor)` and
    /// stamped by the client actor onto every outbound `PresencePeer` (never filled by a shell). Sent
    /// exactly once per connection, after `Welcome` (and its follow-up bootstrap frames) and before
    /// any `Presence` frame.
    Session {
        actor: String,
        color: u8,
    },
    ArtifactBootstrapChunk {
        descriptor_hash: [u8; 32],
        index: u32,
        bytes: ArtifactBootstrapChunkBytes,
    },
    ArtifactBootstrapDone {
        descriptor_hash: [u8; 32],
        chunk_count: u32,
    },
    RebootstrapRequired {
        control: RebootstrapRequired,
    },
}
//#endregion 🔖️ServerFrame
