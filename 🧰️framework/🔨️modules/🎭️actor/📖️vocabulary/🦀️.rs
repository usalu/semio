//! 📖️ Actor vocabulary and wire frames own nominal identities and original retained turn boundaries.
use std::collections::VecDeque;
use semio_framework_value_derive::{FromValue,ToValue};
pub use semio_framework_actor_pack as pack;
pub use semio_framework_job as job;
pub use semio_framework_actor_retained_turn as retained_turn;
use retained_turn::RetainedTurnReceipt;
#[path="../🚪️lifetime/🦀️.rs"]
pub mod instance_lifetime;
#[path="../📃️page/🦀️.rs"]
pub mod byte_page;
#[path="../📤️return/🦀️.rs"]
pub mod return_page;
#[path="../📥️cold-pair/🦀️.rs"]
pub mod cold_pair;
/// 🪪️ Exact fixed request identity retained from `Effect::SpawnJob` through publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct JobReplayRequest {
    pub controller: [u8; 32],
    pub tool: [u8; 32],
    pub schema: [u8; 32],
    pub version: u16,
    pub digest: [u8; 32],
}

impl JobReplayRequest {
    pub fn from_spawn(kind: &str, input: &[u8]) -> Self {
        let controller = kind.split_once('.').map_or(kind, |(controller, _)| controller);
        Self { controller: replay_digest32(controller.as_bytes()), tool: replay_digest32(kind.as_bytes()), schema: replay_digest32(b"semio.kernel.Effect.SpawnJob.v1"), version: 1, digest: replay_digest32_parts(kind.as_bytes(), input) }
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_hash32(out, &self.controller).await;
        pack::write_hash32(out, &self.tool).await;
        pack::write_hash32(out, &self.schema).await;
        pack::write_u16(out, self.version).await;
        pack::write_hash32(out, &self.digest).await;
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self {
            controller: pack::read_hash32(bytes, pos, "JobReplayRequest::controller").await?,
            tool: pack::read_hash32(bytes, pos, "JobReplayRequest::tool").await?,
            schema: pack::read_hash32(bytes, pos, "JobReplayRequest::schema").await?,
            version: pack::read_u16(bytes, pos, "JobReplayRequest::version").await?,
            digest: pack::read_hash32(bytes, pos, "JobReplayRequest::digest").await?,
        })
    }
}

/// 🧮️ Exact replay vocabulary mixing is shared by request identity and its retained publication header.
pub fn replay_mix(mut digest: u64, value: u64) -> u64 {
    digest ^= value;
    digest.wrapping_mul(0x0000_0100_0000_01b3)
}

fn replay_digest32(bytes: &[u8]) -> [u8; 32] {
    replay_digest32_parts(bytes, &[])
}

fn replay_digest32_parts(left: &[u8], right: &[u8]) -> [u8; 32] {
    let mut digest = [0u8; 32];
    for lane in 0..4 {
        let mut value = 0xcbf2_9ce4_8422_2325 ^ lane as u64;
        for byte in left.iter().copied().chain([0xff]).chain(right.iter().copied()) {
            value = replay_mix(value, u64::from(byte));
        }
        digest[lane * 8..lane * 8 + 8].copy_from_slice(&value.to_le_bytes());
    }
    digest
}

//#region 📦️PackageId
/// 📦️ Stable identity of an installed plugin or plugin+extension pair: `<plugin>` or `<plugin>/<extension>`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value", transparent)]
pub struct PackageId(pub String);

impl PackageId {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_str(out, &self.0).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self(pack::read_str(bytes, pos, "PackageId").await?))
    }
}

/// 🧬️ Blake3 hash of a compiled component's bytes — the compiled-cache key (`~/.semio/cache/wasmtime/...`).
#[derive(Clone, Copy, PartialEq, Eq, Hash, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value", transparent)]
pub struct PackageHash(pub [u8; 32]);

impl std::fmt::Debug for PackageHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PackageHash(")?;
        for byte in &self.0[..4] {
            write!(f, "{byte:02x}")?;
        }
        write!(f, "…)")
    }
}

impl PackageHash {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_hash32(out, &self.0).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self(pack::read_hash32(bytes, pos, "PackageHash").await?))
    }
}
//#endregion 📦️PackageId

//#region 🆔️ActorId
/// 🆔️ Bit-packed actor identifier: `plugin_ordinal:u16 | kind:u2 | ordinal:u32 | generation:u14`.
/// Generation makes restart-after-trap addressable without id reuse. The kernel re-exports this
/// type as `RuntimeActorId` (`kernel::ActorId` already names the presence/collab actor — never shadow).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value", transparent)]
pub struct ActorId(pub u64);

const ACTOR_ID_GENERATION_BITS: u32 = 14;
const ACTOR_ID_ORDINAL_BITS: u32 = 32;
const ACTOR_ID_KIND_BITS: u32 = 2;
const ACTOR_ID_GENERATION_MASK: u64 = (1 << ACTOR_ID_GENERATION_BITS) - 1;
const ACTOR_ID_ORDINAL_MASK: u64 = (1 << ACTOR_ID_ORDINAL_BITS) - 1;
const ACTOR_ID_KIND_MASK: u64 = (1 << ACTOR_ID_KIND_BITS) - 1;
const ACTOR_ID_ORDINAL_SHIFT: u32 = ACTOR_ID_GENERATION_BITS;
const ACTOR_ID_KIND_SHIFT: u32 = ACTOR_ID_ORDINAL_SHIFT + ACTOR_ID_ORDINAL_BITS;
const ACTOR_ID_PLUGIN_SHIFT: u32 = ACTOR_ID_KIND_SHIFT + ACTOR_ID_KIND_BITS;

impl ActorId {
    pub async fn new(plugin_ordinal: u16, kind_tag: u8, ordinal: u32, generation: u16) -> Self {
        let bits = ((plugin_ordinal as u64) << ACTOR_ID_PLUGIN_SHIFT) | (((kind_tag as u64) & ACTOR_ID_KIND_MASK) << ACTOR_ID_KIND_SHIFT) | ((ordinal as u64) << ACTOR_ID_ORDINAL_SHIFT) | ((generation as u64) & ACTOR_ID_GENERATION_MASK);
        Self(bits)
    }

    // 🚫️async: E1 pure accessor consumed by `Debug::fmt` (external trait impl below) — see R9
    pub fn plugin_ordinal(self) -> u16 {
        (self.0 >> ACTOR_ID_PLUGIN_SHIFT) as u16
    }

    // 🚫️async: E1 pure accessor consumed by `Debug::fmt` (external trait impl below) — see R9
    pub fn kind_tag(self) -> u8 {
        ((self.0 >> ACTOR_ID_KIND_SHIFT) & ACTOR_ID_KIND_MASK) as u8
    }

    // 🚫️async: E1 pure accessor consumed by `Debug::fmt` (external trait impl below) — see R9
    pub fn ordinal(self) -> u32 {
        ((self.0 >> ACTOR_ID_ORDINAL_SHIFT) & ACTOR_ID_ORDINAL_MASK) as u32
    }

    // 🚫️async: E1 pure accessor consumed by `Debug::fmt` (external trait impl below) — see R9
    pub fn generation(self) -> u16 {
        (self.0 & ACTOR_ID_GENERATION_MASK) as u16
    }

    /// 🔁️ Restart-after-trap: same plugin/kind/ordinal, generation bumped (wraps at 14 bits — a
    /// restart storm beyond 16384 generations is itself a `Disabled`-worthy condition upstream).
    pub async fn next_generation(self) -> Self {
        Self::new(self.plugin_ordinal(), self.kind_tag(), self.ordinal(), self.generation().wrapping_add(1) & (ACTOR_ID_GENERATION_MASK as u16)).await
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u64(out, self.0).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self(pack::read_u64(bytes, pos, "ActorId").await?))
    }
}

impl std::fmt::Debug for ActorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ActorId(plugin={}, kind={}, ordinal={}, gen={})", self.plugin_ordinal(), self.kind_tag(), self.ordinal(), self.generation())
    }
}
//#endregion 🆔️ActorId

//#region 🎭️ActorKind
/// 🎭️ What an actor slot represents: a running app instance, an activated extension, or a
/// background job spawned by another actor. Discriminant order matches `ActorId`'s `kind:u2` tag.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::semio_framework_value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ActorKind {
    PluginApp { plugin: PackageId, app_id: String, instance_id: u32 },
    Extension { plugin: PackageId, extension_id: String },
    Job { owner: ActorId, job_id: u64 },
}

impl ActorKind {
    pub async fn tag(&self) -> u8 {
        match self {
            ActorKind::PluginApp { .. } => 0,
            ActorKind::Extension { .. } => 1,
            ActorKind::Job { .. } => 2,
        }
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            ActorKind::PluginApp { plugin, app_id, instance_id } => {
                pack::write_u8(out, 0).await;
                plugin.pack_encode(out).await;
                pack::write_str(out, app_id).await;
                pack::write_u32(out, *instance_id).await;
            }
            ActorKind::Extension { plugin, extension_id } => {
                pack::write_u8(out, 1).await;
                plugin.pack_encode(out).await;
                pack::write_str(out, extension_id).await;
            }
            ActorKind::Job { owner, job_id } => {
                pack::write_u8(out, 2).await;
                owner.pack_encode(out).await;
                pack::write_u64(out, *job_id).await;
            }
        }
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let tag = pack::read_u8(bytes, pos, "ActorKind").await?;
        match tag {
            0 => Ok(ActorKind::PluginApp { plugin: PackageId::pack_decode(bytes, pos).await?, app_id: pack::read_str(bytes, pos, "ActorKind::app_id").await?, instance_id: pack::read_u32(bytes, pos, "ActorKind::instance_id").await? }),
            1 => Ok(ActorKind::Extension { plugin: PackageId::pack_decode(bytes, pos).await?, extension_id: pack::read_str(bytes, pos, "ActorKind::extension_id").await? }),
            2 => Ok(ActorKind::Job { owner: ActorId::pack_decode(bytes, pos).await?, job_id: pack::read_u64(bytes, pos, "ActorKind::job_id").await? }),
            other => Err(pack::PackError::InvalidTag { what: "ActorKind", tag: other, offset: *pos }),
        }
    }
}
//#endregion 🎭️ActorKind

//#region 🛣️Lane
/// 🛣️ Scheduling priority class. Ordered highest-to-lowest priority by declaration order — see
/// [`Lane::priority_rank`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub enum Lane {
    Interactive,
    UserVisible,
    Background,
    Maintenance,
}

impl Lane {
    pub const ALL: [Lane; 4] = [Lane::Interactive, Lane::UserVisible, Lane::Background, Lane::Maintenance];

    /// 0 = highest priority (Interactive) .. 3 = lowest (Maintenance).
    pub async fn priority_rank(self) -> usize {
        match self {
            Lane::Interactive => 0,
            Lane::UserVisible => 1,
            Lane::Background => 2,
            Lane::Maintenance => 3,
        }
    }

    /// ⚖️ Level-2 DRR quantum weight — biases actor selection within a plugin toward interactive work.
    // 🚫️async: E1-adjacent pure computation — its one consumer, `Scheduler::actor_weight`, is
    // itself sync for the same reason (see that fn's own tag) — see R9 residue shape 1.
    pub fn weight(self) -> u32 {
        match self {
            Lane::Interactive => 8,
            Lane::UserVisible => 4,
            Lane::Background => 2,
            Lane::Maintenance => 1,
        }
    }

    async fn tag(self) -> u8 {
        self.priority_rank().await as u8
    }

    async fn from_tag(tag: u8) -> Result<Self, pack::PackError> {
        match tag {
            0 => Ok(Lane::Interactive),
            1 => Ok(Lane::UserVisible),
            2 => Ok(Lane::Background),
            3 => Ok(Lane::Maintenance),
            other => Err(pack::PackError::InvalidTag { what: "Lane", tag: other, offset: 0 }),
        }
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u8(out, self.tag().await).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let tag = pack::read_u8(bytes, pos, "Lane").await?;
        Self::from_tag(tag).await
    }
}
//#endregion 🛣️Lane

//#region ⚖️Budget
/// ⚖️ Per-turn resource ceilings enforced host-side. Replaces `PLUGIN_FUEL_BUDGET`.
#[derive(Clone, Copy, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct Budget {
    pub fuel: u64,
    pub wall_ms: u32,
    pub memory_bytes: u64,
    pub ui_nodes: u32,
    pub mailbox_len: u16,
    pub max_effects: u32,
    pub max_patch_bytes: u32,
    pub max_frames:u32,
}

impl Budget {
    /// ⚖️ Scales a `Throttled { factor }` failure stage onto this budget — fuel/wall/effects/patch
    /// bytes shrink; `mailbox_len`/`ui_nodes` ceilings are left alone (they bound queued/committed
    /// state, not this turn's spend).
    pub async fn scaled(self, factor: f32) -> Budget {
        let factor = factor.clamp(0.05, 1.0);
        Budget {
            fuel: ((self.fuel as f64) * factor as f64) as u64,
            wall_ms: ((self.wall_ms as f64) * factor as f64).max(1.0) as u32,
            memory_bytes: self.memory_bytes,
            ui_nodes: self.ui_nodes,
            mailbox_len: self.mailbox_len,
            max_effects: ((self.max_effects as f64) * factor as f64) as u32,
            max_patch_bytes: self.max_patch_bytes,
            max_frames:self.max_frames,
        }
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) -> Result<(), pack::PackError> {
        pack::write_u64(out, self.fuel).await;
        pack::write_u32(out, self.wall_ms).await;
        pack::write_u64(out, self.memory_bytes).await;
        pack::write_u32(out, self.ui_nodes).await;
        pack::write_u16(out, self.mailbox_len).await;
        pack::write_u32(out, self.max_effects).await;
        pack::write_u32(out, self.max_patch_bytes).await;
        pack::write_u32(out,self.max_frames).await;
        Ok(())
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self {
            fuel: pack::read_u64(bytes, pos, "Budget::fuel").await?,
            wall_ms: pack::read_u32(bytes, pos, "Budget::wall_ms").await?,
            memory_bytes: pack::read_u64(bytes, pos, "Budget::memory_bytes").await?,
            ui_nodes: pack::read_u32(bytes, pos, "Budget::ui_nodes").await?,
            mailbox_len: pack::read_u16(bytes, pos, "Budget::mailbox_len").await?,
            max_effects: pack::read_u32(bytes, pos, "Budget::max_effects").await?,
            max_patch_bytes: pack::read_u32(bytes, pos, "Budget::max_patch_bytes").await?,
            max_frames:pack::read_u32(bytes,pos,"Budget::max_frames").await?,
        })
    }
}

//#region ⚖️LaneDefaults
/// ⚖️ Default [`Budget`] per [`Lane`] — Interactive 4ms/2M fuel, UserVisible 16ms, Background 50ms,
/// Maintenance 200ms (the ladder the design spec fixes explicitly); the remaining ceilings scale
/// down the same tiers so background/maintenance work can never out-spend memory/UI/mailbox room
/// an interactive turn would need.
pub mod lane_defaults {
    use super::{Budget, Lane};

    pub fn budget_for(lane: Lane) -> Budget {
        match lane {
            Lane::Interactive => Budget { fuel: 2_000_000, wall_ms: 4, memory_bytes: 64 * 1024 * 1024, ui_nodes: 20_000, mailbox_len: 256, max_effects: 64, max_patch_bytes: 262_144,max_frames:8 },
            Lane::UserVisible => Budget { fuel: 6_000_000, wall_ms: 16, memory_bytes: 96 * 1024 * 1024, ui_nodes: 20_000, mailbox_len: 256, max_effects: 128, max_patch_bytes: 524_288,max_frames:8 },
            Lane::Background => Budget { fuel: 20_000_000, wall_ms: 50, memory_bytes: 192 * 1024 * 1024, ui_nodes: 8_000, mailbox_len: 512, max_effects: 256, max_patch_bytes: 1_048_576,max_frames:8 },
            Lane::Maintenance => Budget { fuel: 80_000_000, wall_ms: 200, memory_bytes: 256 * 1024 * 1024, ui_nodes: 4_000, mailbox_len: 1024, max_effects: 512, max_patch_bytes: 2_097_152,max_frames:8 },
        }
    }
}
//#endregion ⚖️LaneDefaults
//#endregion ⚖️Budget

//#region 🪪️JobBridge
/// 🪪️ Stable operation identity carried by every actor job turn and publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct JobOperation {
    pub operation: u64,
    pub base_revision: u64,
    pub generation: u64,
    pub preview_sequence: u64,
    pub seed: u64,
}

impl JobOperation {
    pub fn from_job(operation: job::Operation) -> Self {
        Self { operation: operation.operation.0, base_revision: operation.base_revision.0, generation: operation.generation.0, preview_sequence: operation.preview_sequence, seed: operation.seed }
    }

    pub fn into_job(self) -> job::Operation {
        job::Operation { operation: job::OperationId(self.operation), base_revision: job::RevisionId(self.base_revision), generation: job::Generation(self.generation), preview_sequence: self.preview_sequence, seed: self.seed }
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u64(out, self.operation).await;
        pack::write_u64(out, self.base_revision).await;
        pack::write_u64(out, self.generation).await;
        pack::write_u64(out, self.preview_sequence).await;
        pack::write_u64(out, self.seed).await;
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self {
            operation: pack::read_u64(bytes, pos, "JobOperation::operation").await?,
            base_revision: pack::read_u64(bytes, pos, "JobOperation::base_revision").await?,
            generation: pack::read_u64(bytes, pos, "JobOperation::generation").await?,
            preview_sequence: pack::read_u64(bytes, pos, "JobOperation::preview_sequence").await?,
            seed: pack::read_u64(bytes, pos, "JobOperation::seed").await?,
        })
    }
}

/// 📸️ Opaque resumable state plus the committed progress boundary it represents.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct JobCheckpoint {
    pub state: Vec<u8>,
    pub applied_progress: u64,
}

impl JobCheckpoint {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_bytes(out, &self.state).await;
        pack::write_u64(out, self.applied_progress).await;
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { state: pack::read_bytes(bytes, pos, "JobCheckpoint::state").await?, applied_progress: pack::read_u64(bytes, pos, "JobCheckpoint::applied_progress").await? })
    }
}

/// 🏁️ Final persisted job state and its authoritative output candidate.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct JobCommitCandidate {
    pub state: Vec<u8>,
    pub output: Vec<u8>,
}

impl JobCommitCandidate {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_bytes(out, &self.state).await;
        pack::write_bytes(out, &self.output).await;
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { state: pack::read_bytes(bytes, pos, "JobCommitCandidate::state").await?, output: pack::read_bytes(bytes, pos, "JobCommitCandidate::output").await? })
    }
}

/// 🚦️ Lossless actor-wire mirror of one universal `StepOutcome`.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::semio_framework_value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum JobStepOutcome {
    Yield,
    PreviewReady { preview: Vec<u8> },
    CheckpointReady { checkpoint: JobCheckpoint },
    Complete { candidate: JobCommitCandidate },
    Cancelled,
    Fault { detail: Vec<u8> },
}

impl JobStepOutcome {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            Self::Yield => pack::write_u8(out, 0).await,
            Self::PreviewReady { preview } => {
                pack::write_u8(out, 1).await;
                pack::write_bytes(out, preview).await;
            }
            Self::CheckpointReady { checkpoint } => {
                pack::write_u8(out, 2).await;
                checkpoint.pack_encode(out).await;
            }
            Self::Complete { candidate } => {
                pack::write_u8(out, 3).await;
                candidate.pack_encode(out).await;
            }
            Self::Cancelled => pack::write_u8(out, 4).await,
            Self::Fault { detail } => {
                pack::write_u8(out, 5).await;
                pack::write_bytes(out, detail).await;
            }
        }
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        match pack::read_u8(bytes, pos, "JobStepOutcome").await? {
            0 => Ok(Self::Yield),
            1 => Ok(Self::PreviewReady { preview: pack::read_bytes(bytes, pos, "JobStepOutcome::PreviewReady").await? }),
            2 => Ok(Self::CheckpointReady { checkpoint: JobCheckpoint::pack_decode(bytes, pos).await? }),
            3 => Ok(Self::Complete { candidate: JobCommitCandidate::pack_decode(bytes, pos).await? }),
            4 => Ok(Self::Cancelled),
            5 => Ok(Self::Fault { detail: pack::read_bytes(bytes, pos, "JobStepOutcome::Fault").await? }),
            tag => Err(pack::PackError::InvalidTag { what: "JobStepOutcome", tag, offset: *pos }),
        }
    }
}

/// 🎫️ One explicitly-addressed bounded job turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct JobTurn {
    pub job: u64,
    pub operation: JobOperation,
    pub step_sequence: u64,
}

impl JobTurn {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u64(out, self.job).await;
        self.operation.pack_encode(out).await;
        pack::write_u64(out, self.step_sequence).await;
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { job: pack::read_u64(bytes, pos, "JobTurn::job").await?, operation: JobOperation::pack_decode(bytes, pos).await?, step_sequence: pack::read_u64(bytes, pos, "JobTurn::step_sequence").await? })
    }
}

/// 📡️ One validated, replay-addressable publication from a bounded job turn.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct JobPublication {
    pub turn: JobTurn,
    pub outcome: JobStepOutcome,
}

impl JobPublication {
    pub fn turn_status(&self) -> TurnStatus {
        match &self.outcome {
            JobStepOutcome::Yield => TurnStatus::MoreWork,
            JobStepOutcome::PreviewReady { preview } => TurnStatus::PreviewReady { preview: preview.clone(), sequence: self.turn.operation.preview_sequence.saturating_sub(1) },
            JobStepOutcome::CheckpointReady { checkpoint } => TurnStatus::CheckpointReady { checkpoint: checkpoint.clone() },
            JobStepOutcome::Complete { candidate } => TurnStatus::CommitReady { candidate: candidate.clone() },
            JobStepOutcome::Cancelled => TurnStatus::Cancelled,
            JobStepOutcome::Fault { detail } => TurnStatus::Faulted { detail: detail.clone() },
        }
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        self.turn.pack_encode(out).await;
        self.outcome.pack_encode(out).await;
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { turn: JobTurn::pack_decode(bytes, pos).await?, outcome: JobStepOutcome::pack_decode(bytes, pos).await? })
    }
}


//#region ✉️Envelope
/// 🪟 Local, opaque window identifier ([`Origin::Ui`]'s target) — the concrete `WindowHandle`
/// lives in the kernel crate; this crate only ever routes by this bare numeric id.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value", transparent)]
pub struct WindowId(pub u32);

impl WindowId {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u32(out, self.0).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self(pack::read_u32(bytes, pos, "WindowId").await?))
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
                pack::write_u8(out, 0).await;
                window.pack_encode(out).await;
            }
            Origin::Actor { id } => {
                pack::write_u8(out, 1).await;
                id.pack_encode(out).await;
            }
            Origin::Kernel => pack::write_u8(out, 2).await,
            Origin::Bus { topic } => {
                pack::write_u8(out, 3).await;
                pack::write_str(out, topic).await;
            }
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let tag = pack::read_u8(bytes, pos, "Origin").await?;
        match tag {
            0 => Ok(Origin::Ui { window: WindowId::pack_decode(bytes, pos).await? }),
            1 => Ok(Origin::Actor { id: ActorId::pack_decode(bytes, pos).await? }),
            2 => Ok(Origin::Kernel),
            3 => Ok(Origin::Bus { topic: pack::read_str(bytes, pos, "Origin::topic").await? }),
            other => Err(pack::PackError::InvalidTag { what: "Origin", tag: other, offset: *pos }),
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
            Payload::ContinueOriginalCall=>pack::write_u8(out,6).await,
            Payload::Event { bytes } => {
                pack::write_u8(out, 0).await;
                pack::write_bytes(out, bytes).await;
            }
            Payload::Suspend { operation, applied_progress } => {
                pack::write_u8(out, 1).await;
                operation.pack_encode(out).await;
                pack::write_u64(out, *applied_progress).await;
            }
            Payload::Resume { operation, checkpoint } => {
                pack::write_u8(out, 2).await;
                operation.pack_encode(out).await;
                checkpoint.pack_encode(out).await;
            }
            Payload::Cancel { seq } => {
                pack::write_u8(out, 3).await;
                pack::write_u64(out, *seq).await;
            }
            Payload::JobStep { turn } => {
                pack::write_u8(out, 4).await;
                turn.pack_encode(out).await;
            }
            Payload::JobReplay { turn, request, worker_count, worker_slot } => {
                pack::write_u8(out, 5).await;
                turn.pack_encode(out).await;
                request.pack_encode(out).await;
                pack::write_u16(out, *worker_count).await;
                pack::write_u16(out, *worker_slot).await;
            }
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let tag = pack::read_u8(bytes, pos, "Payload").await?;
        match tag {
            6 => Ok(Payload::ContinueOriginalCall),
            0 => Ok(Payload::Event { bytes: pack::read_bytes(bytes, pos, "Payload::Event").await? }),
            1 => Ok(Payload::Suspend { operation: JobOperation::pack_decode(bytes, pos).await?, applied_progress: pack::read_u64(bytes, pos, "Payload::Suspend::applied_progress").await? }),
            2 => Ok(Payload::Resume { operation: JobOperation::pack_decode(bytes, pos).await?, checkpoint: JobCheckpoint::pack_decode(bytes, pos).await? }),
            3 => Ok(Payload::Cancel { seq: pack::read_u64(bytes, pos, "Payload::Cancel").await? }),
            4 => Ok(Payload::JobStep { turn: JobTurn::pack_decode(bytes, pos).await? }),
            5 => Ok(Payload::JobReplay {
                turn: JobTurn::pack_decode(bytes, pos).await?,
                request: JobReplayRequest::pack_decode(bytes, pos).await?,
                worker_count: pack::read_u16(bytes, pos, "Payload::JobReplay::worker_count").await?,
                worker_slot: pack::read_u16(bytes, pos, "Payload::JobReplay::worker_slot").await?,
            }),
            other => Err(pack::PackError::InvalidTag { what: "Payload", tag: other, offset: *pos }),
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
        pack::write_str(out, &self.0).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self(pack::read_str(bytes, pos, "CoalesceKey").await?))
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
        pack::write_u64(out, self.seq).await;
        pack::write_opt_u64(out, &self.deadline_ms).await;
        pack::write_bool(out, self.coalesce.is_some()).await;
        if let Some(key) = &self.coalesce {
            key.pack_encode(out).await;
        }
        pack::write_opt_u64(out, &self.cancel_of).await;
        self.payload.pack_encode(out).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let to = ActorId::pack_decode(bytes, pos).await?;
        let from = Origin::pack_decode(bytes, pos).await?;
        let lane = Lane::pack_decode(bytes, pos).await?;
        let seq = pack::read_u64(bytes, pos, "Envelope::seq").await?;
        let deadline_ms = pack::read_opt_u64(bytes, pos, "Envelope::deadline_ms").await?;
        let coalesce = if pack::read_bool(bytes, pos, "Envelope::coalesce").await? { Some(CoalesceKey::pack_decode(bytes, pos).await?) } else { None };
        let cancel_of = pack::read_opt_u64(bytes, pos, "Envelope::cancel_of").await?;
        let payload = Payload::pack_decode(bytes, pos).await?;
        Ok(Self { to, from, lane, seq, deadline_ms, coalesce, cancel_of, payload })
    }
}
//#endregion ✉️Envelope

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
            TurnStatus::Idle => pack::write_u8(out, 0).await,
            TurnStatus::MoreWork => pack::write_u8(out, 1).await,
            TurnStatus::CheckpointReady { checkpoint } => {
                pack::write_u8(out, 2).await;
                checkpoint.pack_encode(out).await;
            }
            TurnStatus::Faulted { detail } => {
                pack::write_u8(out, 3).await;
                pack::write_bytes(out, detail).await;
            }
            TurnStatus::PreviewReady { preview, sequence } => {
                pack::write_u8(out, 4).await;
                pack::write_bytes(out, preview).await;
                pack::write_u64(out, *sequence).await;
            }
            TurnStatus::CommitReady { candidate } => {
                pack::write_u8(out, 5).await;
                candidate.pack_encode(out).await;
            }
            TurnStatus::Cancelled => pack::write_u8(out, 6).await,
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let tag = pack::read_u8(bytes, pos, "TurnStatus").await?;
        match tag {
            0 => Ok(TurnStatus::Idle),
            1 => Ok(TurnStatus::MoreWork),
            2 => Ok(TurnStatus::CheckpointReady { checkpoint: JobCheckpoint::pack_decode(bytes, pos).await? }),
            3 => Ok(TurnStatus::Faulted { detail: pack::read_bytes(bytes, pos, "TurnStatus::Faulted").await? }),
            4 => Ok(TurnStatus::PreviewReady { preview: pack::read_bytes(bytes, pos, "TurnStatus::PreviewReady::preview").await?, sequence: pack::read_u64(bytes, pos, "TurnStatus::PreviewReady::sequence").await? }),
            5 => Ok(TurnStatus::CommitReady { candidate: JobCommitCandidate::pack_decode(bytes, pos).await? }),
            6 => Ok(TurnStatus::Cancelled),
            other => Err(pack::PackError::InvalidTag { what: "TurnStatus", tag: other, offset: *pos }),
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
        pack::write_u64(out, self.fuel).await;
        pack::write_u64(out, self.wall_us).await;
        pack::write_u64(out, self.memory_bytes).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { fuel: pack::read_u64(bytes, pos, "Usage::fuel").await?, wall_us: pack::read_u64(bytes, pos, "Usage::wall_us").await?, memory_bytes: pack::read_u64(bytes, pos, "Usage::memory_bytes").await? })
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
    pub async fn pack_encode(&self, out: &mut Vec<u8>) -> Result<(), pack::PackError> {
        let mut receipt = [0; instance_lifetime::ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES];
        let length = match self.lifecycle_receipt {
            Some(value) => instance_lifetime::ActorInstanceLifecycleWire::Receipt(value).encode(&mut receipt).map_err(pack::PackError::InvalidLifecycle)?,
            None => 0,
        };
        self.cold_pair_ingress.validate().map_err(pack::PackError::InvalidColdPair)?;
        instance_lifetime::ActorUiPatchReceipt::validate_pairing(self.ui_patch_receipt, usize::from(!self.ui_patches.is_empty())).map_err(pack::PackError::InvalidUiPatchReceipt)?;
        let mut patch_receipt = [0; instance_lifetime::ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES];
        let patch_length = match self.ui_patch_receipt {
            Some(value) => value.encode(&mut patch_receipt).map_err(pack::PackError::InvalidUiPatchReceipt)?,
            None => 0,
        };
        self.retained_receipt.pack_encode(out).await?;
        pack::write_bytes(out, &self.ui_patches).await;
        pack::write_bytes(out, &self.effects).await;
        pack::write_bytes(out, &self.command_ingress).await;
        self.cold_pair_ingress.pack_encode(out).await?;
        pack::write_bytes(out, &receipt[..length]).await;
        pack::write_bytes(out, &patch_receipt[..patch_length]).await;
        pack::write_opt_u64(out, &self.next_wake).await;
        self.status.pack_encode(out).await;
        self.usage.pack_encode(out).await;
        Ok(())
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let result = Self {
            retained_receipt: RetainedTurnReceipt::pack_decode(bytes,pos).await?,
            ui_patches: pack::read_bytes(bytes, pos, "TurnResult::ui_patches").await?,
            effects: pack::read_bytes(bytes, pos, "TurnResult::effects").await?,
            command_ingress: pack::read_bytes(bytes, pos, "TurnResult::command_ingress").await?,
            cold_pair_ingress: cold_pair::ColdPairIngressStatus::pack_decode(bytes, pos).await?,
            lifecycle_receipt: Self::read_lifecycle_receipt(bytes, pos).await?,
            ui_patch_receipt: Self::read_ui_patch_receipt(bytes, pos).await?,
            next_wake: pack::read_opt_u64(bytes, pos, "TurnResult::next_wake").await?,
            status: TurnStatus::pack_decode(bytes, pos).await?,
            usage: Usage::pack_decode(bytes, pos).await?,
        };
        instance_lifetime::ActorUiPatchReceipt::validate_pairing(result.ui_patch_receipt, usize::from(!result.ui_patches.is_empty())).map_err(pack::PackError::InvalidUiPatchReceipt)?;
        Ok(result)
    }

    async fn read_ui_patch_receipt(bytes: &[u8], pos: &mut usize) -> Result<Option<instance_lifetime::ActorUiPatchReceipt>, pack::PackError> {
        let length = pack::read_u8(bytes, pos, "TurnResult::ui_patch_receipt.length").await? as usize;
        if length > instance_lifetime::ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES {
            return Err(pack::PackError::InvalidUiPatchReceipt("receipt length exceeds fixed authority"));
        }
        if length == 0 {
            return Ok(None);
        }
        let end = pos.checked_add(length).ok_or(pack::PackError::InvalidUiPatchReceipt("receipt offset overflow"))?;
        let receipt = instance_lifetime::ActorUiPatchReceipt::decode(bytes.get(*pos..end).ok_or(pack::PackError::Truncated(*pos, "TurnResult::ui_patch_receipt"))?).map_err(pack::PackError::InvalidUiPatchReceipt)?;
        *pos = end;
        Ok(Some(receipt))
    }

    async fn read_lifecycle_receipt(bytes: &[u8], pos: &mut usize) -> Result<Option<instance_lifetime::ActorInstanceLifecycleReceipt>, pack::PackError> {
        let length = pack::read_u8(bytes, pos, "TurnResult::lifecycle_receipt.length").await? as usize;
        if length > instance_lifetime::ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES {
            return Err(pack::PackError::InvalidLifecycle("receipt length exceeds fixed authority"));
        }
        if length == 0 {
            return Ok(None);
        }
        let end = pos.checked_add(length).ok_or(pack::PackError::InvalidLifecycle("receipt offset overflow"))?;
        let receipt = bytes.get(*pos..end).ok_or(pack::PackError::Truncated(*pos, "TurnResult::lifecycle_receipt"))?;
        let instance_lifetime::ActorInstanceLifecycleWire::Receipt(receipt) = instance_lifetime::ActorInstanceLifecycleWire::decode(receipt).map_err(pack::PackError::InvalidLifecycle)? else {
            return Err(pack::PackError::InvalidLifecycle("turn result requires a receipt"));
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
            Backpressure::Accept => pack::write_u8(out, 0).await,
            Backpressure::Coalesced => pack::write_u8(out, 1).await,
            Backpressure::Dropped { lane } => {
                pack::write_u8(out, 2).await;
                lane.pack_encode(out).await;
            }
            Backpressure::Rejected => pack::write_u8(out, 3).await,
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let tag = pack::read_u8(bytes, pos, "Backpressure").await?;
        match tag {
            0 => Ok(Backpressure::Accept),
            1 => Ok(Backpressure::Coalesced),
            2 => Ok(Backpressure::Dropped { lane: Lane::pack_decode(bytes, pos).await? }),
            3 => Ok(Backpressure::Rejected),
            other => Err(pack::PackError::InvalidTag { what: "Backpressure", tag: other, offset: *pos }),
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
    len: u16,
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
        pack::write_u16(out, self.capacity).await;
        pack::write_u16(out, self.len).await;
        for lane in &self.lanes {
            pack::write_varint_u64(out, lane.len() as u64).await;
            for envelope in lane.iter() {
                envelope.pack_encode(out).await;
            }
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let capacity = pack::read_u16(bytes, pos, "Mailbox::capacity").await?;
        let len = pack::read_u16(bytes, pos, "Mailbox::len").await?;
        let mut lanes: [VecDeque<Envelope>; 4] = Default::default();
        for lane in lanes.iter_mut() {
            *lane = pack::read_vec(bytes, pos, "Mailbox::lane", Envelope::pack_decode).await?.into();
        }
        Ok(Self { capacity, lanes, len })
    }
}
//#endregion 📬️Mailbox

//#region 🔐️CapabilityGrant
/// 🔐️ Minimal local stand-in for `kernel::CapabilityGrant` — see the module-level seam docstring.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct CapabilityGrant {
    pub capability: String,
    pub scope: Option<Vec<u8>>,
}

impl CapabilityGrant {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_str(out, &self.capability).await;
        pack::write_opt_bytes(out, &self.scope).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { capability: pack::read_str(bytes, pos, "CapabilityGrant::capability").await?, scope: pack::read_opt_bytes(bytes, pos, "CapabilityGrant::scope").await? })
    }
}
/// 🔐️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): the security property of
/// the whole extension-activation design — an extension must never hold a capability its host plugin
/// lacks. A REQUESTED grant survives only when `granted` (the parent's own already-granted set)
/// carries a grant of the SAME `capability` name; an ungranted request is silently dropped, never
/// escalated or substituted. This is an actual intersection, not "grant what was asked for": the
/// output is always a subset of `requested`, bounded by `granted`. Matched by capability name only
/// (this crate's `CapabilityGrant` is a minimal pack-codeable stand-in — see its own doc comment — so
/// `scope` reconciliation belongs to the real broker at integration time, not this pure function).
pub async fn intersect_capabilities(granted: &[CapabilityGrant], requested: &[CapabilityGrant]) -> Vec<CapabilityGrant> {
    requested.iter().filter(|request| granted.iter().any(|grant| grant.capability == request.capability)).cloned().collect()
}
//#endregion 🔐️CapabilityGrant

//#region 🚑️FailurePolicy
/// 🚑️ What triggered a failure-ladder transition.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::semio_framework_value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum FailureSignal {
    DeadlineOverrun {
        ratio: f32,
    },
    FuelExhausted,
    MemoryLimit,
    MailboxOverflow,
    UiQuota,
    /// 🐛️ Struct variant, not the newtype `Trap(String)` this used to be — `String` is a sequence
    /// of chars as far as serde's internal tagging is concerned, so it hits the exact same "cannot
    /// serialize tagged newtype variant ... containing a sequence" defect as [`Payload::Event`].
    Trap {
        detail: String,
    },
    HeartbeatMissed {
        count: u32,
    },
    ManualReset,
}

impl FailureSignal {
    /// 🚨️ Whether this signal is severe enough to skip the warn/throttle rungs and trap outright.
    async fn is_fatal(&self) -> bool {
        matches!(self, FailureSignal::Trap { .. })
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            FailureSignal::DeadlineOverrun { ratio } => {
                pack::write_u8(out, 0).await;
                pack::write_f32(out, *ratio).await;
            }
            FailureSignal::FuelExhausted => pack::write_u8(out, 1).await,
            FailureSignal::MemoryLimit => pack::write_u8(out, 2).await,
            FailureSignal::MailboxOverflow => pack::write_u8(out, 3).await,
            FailureSignal::UiQuota => pack::write_u8(out, 4).await,
            FailureSignal::Trap { detail } => {
                pack::write_u8(out, 5).await;
                pack::write_str(out, detail).await;
            }
            FailureSignal::HeartbeatMissed { count } => {
                pack::write_u8(out, 6).await;
                pack::write_u32(out, *count).await;
            }
            FailureSignal::ManualReset => pack::write_u8(out, 7).await,
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let tag = pack::read_u8(bytes, pos, "FailureSignal").await?;
        match tag {
            0 => Ok(FailureSignal::DeadlineOverrun { ratio: pack::read_f32(bytes, pos, "FailureSignal::ratio").await? }),
            1 => Ok(FailureSignal::FuelExhausted),
            2 => Ok(FailureSignal::MemoryLimit),
            3 => Ok(FailureSignal::MailboxOverflow),
            4 => Ok(FailureSignal::UiQuota),
            5 => Ok(FailureSignal::Trap { detail: pack::read_str(bytes, pos, "FailureSignal::Trap").await? }),
            6 => Ok(FailureSignal::HeartbeatMissed { count: pack::read_u32(bytes, pos, "FailureSignal::count").await? }),
            7 => Ok(FailureSignal::ManualReset),
            other => Err(pack::PackError::InvalidTag { what: "FailureSignal", tag: other, offset: *pos }),
        }
    }
}

/// 🪜️ Rungs of the failure ladder, worst-consequence order.
#[derive(Clone, Copy, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::semio_framework_value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum FailureStage {
    Healthy,
    Warned,
    Throttled { factor: f32 },
    Suspended { until: u64 },
    Cancelled,
    Trapped { restarts: u32 },
    Quarantined { until: u64 },
    Disabled,
}

impl FailureStage {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            FailureStage::Healthy => pack::write_u8(out, 0).await,
            FailureStage::Warned => pack::write_u8(out, 1).await,
            FailureStage::Throttled { factor } => {
                pack::write_u8(out, 2).await;
                pack::write_f32(out, *factor).await;
            }
            FailureStage::Suspended { until } => {
                pack::write_u8(out, 3).await;
                pack::write_u64(out, *until).await;
            }
            FailureStage::Cancelled => pack::write_u8(out, 4).await,
            FailureStage::Trapped { restarts } => {
                pack::write_u8(out, 5).await;
                pack::write_u32(out, *restarts).await;
            }
            FailureStage::Quarantined { until } => {
                pack::write_u8(out, 6).await;
                pack::write_u64(out, *until).await;
            }
            FailureStage::Disabled => pack::write_u8(out, 7).await,
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let tag = pack::read_u8(bytes, pos, "FailureStage").await?;
        match tag {
            0 => Ok(FailureStage::Healthy),
            1 => Ok(FailureStage::Warned),
            2 => Ok(FailureStage::Throttled { factor: pack::read_f32(bytes, pos, "FailureStage::factor").await? }),
            3 => Ok(FailureStage::Suspended { until: pack::read_u64(bytes, pos, "FailureStage::until").await? }),
            4 => Ok(FailureStage::Cancelled),
            5 => Ok(FailureStage::Trapped { restarts: pack::read_u32(bytes, pos, "FailureStage::restarts").await? }),
            6 => Ok(FailureStage::Quarantined { until: pack::read_u64(bytes, pos, "FailureStage::until").await? }),
            7 => Ok(FailureStage::Disabled),
            other => Err(pack::PackError::InvalidTag { what: "FailureStage", tag: other, offset: *pos }),
        }
    }
}

/// ⏳️ Number of consecutive clean turns before `FailureState::on_clean_turn` decays one rung.
pub const FAILURE_DECAY_CLEAN_TURNS: u32 = 10;
/// 🔁️ Consecutive traps before an actor's package is quarantined.
pub const FAILURE_QUARANTINE_RESTART_THRESHOLD: u32 = 3;
/// 💓️ Consecutive missed heartbeats before a shard is treated as dead (see design §Watchdog).
pub const FAILURE_HEARTBEAT_TRAP_THRESHOLD: u32 = 3;

/// ⚖️ Exponential per-lane warn-count thresholds `[warned→throttled, throttled→suspended]` —
/// interactive actors escalate fastest (they are the most visible), maintenance actors get the
/// most slack.
async fn lane_escalation_thresholds(lane: Lane) -> [u32; 2] {
    match lane {
        Lane::Interactive => [2, 3],
        Lane::UserVisible => [3, 5],
        Lane::Background => [5, 9],
        Lane::Maintenance => [9, 17],
    }
}

/// 🚑️ Per-actor failure ladder state: current [`FailureStage`], warn/restart counters, and the
/// clean-turn counter that drives decay.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct FailureState {
    pub stage: FailureStage,
    pub clean_turns: u32,
    pub warn_count: u32,
    pub restart_count: u32,
    pub last_signal_ms: u64,
}

impl Default for FailureState {
    fn default() -> Self {
        Self::new()
    }
}

/// 🪜️ Outcome of `FailureState::on_signal` the kernel must act on beyond updating this actor's own state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FailureEscalation {
    /// No cross-actor action required.
    None,
    /// This actor trapped — kernel must drop + re-instantiate with `ActorId::next_generation` and restore its last checkpoint.
    Restart,
    /// Restart count crossed the threshold — kernel must quarantine every actor sharing this actor's `PackageId`.
    QuarantinePackage,
}

impl FailureState {
    // 🚫️async: E1 pure constructor consumed by `Default::default` (external trait impl above) —
    // see R9
    pub fn new() -> Self {
        Self { stage: FailureStage::Healthy, clean_turns: 0, warn_count: 0, restart_count: 0, last_signal_ms: 0 }
    }

    /// 🚑️ Applies one failure signal, returning what the kernel must do beyond this actor's own bookkeeping.
    pub async fn on_signal(&mut self, signal: &FailureSignal, lane: Lane, now_ms: u64) -> FailureEscalation {
        self.clean_turns = 0;
        self.last_signal_ms = now_ms;
        if let FailureSignal::ManualReset = signal {
            *self = Self::new();
            return FailureEscalation::None;
        }
        if signal.is_fatal().await {
            self.restart_count += 1;
            self.stage = FailureStage::Trapped { restarts: self.restart_count };
            if self.restart_count >= FAILURE_QUARANTINE_RESTART_THRESHOLD {
                self.stage = FailureStage::Quarantined { until: now_ms + quarantine_duration_ms(self.restart_count).await };
                return FailureEscalation::QuarantinePackage;
            }
            return FailureEscalation::Restart;
        }
        if let FailureSignal::HeartbeatMissed { count } = signal {
            if *count >= FAILURE_HEARTBEAT_TRAP_THRESHOLD {
                self.restart_count += 1;
                self.stage = FailureStage::Trapped { restarts: self.restart_count };
                if self.restart_count >= FAILURE_QUARANTINE_RESTART_THRESHOLD {
                    self.stage = FailureStage::Quarantined { until: now_ms + quarantine_duration_ms(self.restart_count).await };
                    return FailureEscalation::QuarantinePackage;
                }
                return FailureEscalation::Restart;
            }
        }
        self.warn_count += 1;
        let [throttle_at, suspend_at] = lane_escalation_thresholds(lane).await;
        self.stage = if self.warn_count >= suspend_at {
            FailureStage::Suspended { until: now_ms + suspend_backoff_ms(self.warn_count).await }
        } else if self.warn_count >= throttle_at {
            FailureStage::Throttled { factor: throttle_factor(self.warn_count, throttle_at).await }
        } else {
            FailureStage::Warned
        };
        FailureEscalation::None
    }

    /// ⏳️ One clean (no-signal) turn elapsed — decays one rung after `FAILURE_DECAY_CLEAN_TURNS`
    /// consecutive clean turns, or immediately promotes an expired `Suspended`/`Quarantined` timer
    /// into the next rung down.
    pub async fn on_clean_turn(&mut self, now_ms: u64) {
        self.clean_turns += 1;
        match self.stage {
            FailureStage::Warned if self.clean_turns >= FAILURE_DECAY_CLEAN_TURNS => {
                self.stage = FailureStage::Healthy;
                self.warn_count = 0;
                self.clean_turns = 0;
            }
            FailureStage::Throttled { .. } if self.clean_turns >= FAILURE_DECAY_CLEAN_TURNS => {
                self.stage = FailureStage::Warned;
                self.clean_turns = 0;
            }
            FailureStage::Suspended { until } if now_ms >= until => {
                self.stage = FailureStage::Throttled { factor: 0.5 };
                self.clean_turns = 0;
            }
            FailureStage::Quarantined { until } if now_ms >= until => {
                self.stage = FailureStage::Warned;
                self.clean_turns = 0;
                self.restart_count = 0;
            }
            _ => {}
        }
    }

    /// ▶️ Whether the scheduler may currently grant this actor a turn.
    pub async fn runnable(&self, now_ms: u64) -> bool {
        match self.stage {
            FailureStage::Healthy | FailureStage::Warned | FailureStage::Throttled { .. } => true,
            FailureStage::Suspended { until } => now_ms >= until,
            FailureStage::Cancelled | FailureStage::Trapped { .. } | FailureStage::Disabled => false,
            FailureStage::Quarantined { until } => now_ms >= until,
        }
    }

    /// ⚖️ The DRR weight/budget scale factor for the current stage — `1.0` unless `Throttled`.
    pub async fn throttle_factor(&self) -> f32 {
        match self.stage {
            FailureStage::Throttled { factor } => factor,
            _ => 1.0,
        }
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        self.stage.pack_encode(out).await;
        pack::write_u32(out, self.clean_turns).await;
        pack::write_u32(out, self.warn_count).await;
        pack::write_u32(out, self.restart_count).await;
        pack::write_u64(out, self.last_signal_ms).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self {
            stage: FailureStage::pack_decode(bytes, pos).await?,
            clean_turns: pack::read_u32(bytes, pos, "FailureState::clean_turns").await?,
            warn_count: pack::read_u32(bytes, pos, "FailureState::warn_count").await?,
            restart_count: pack::read_u32(bytes, pos, "FailureState::restart_count").await?,
            last_signal_ms: pack::read_u64(bytes, pos, "FailureState::last_signal_ms").await?,
        })
    }
}

async fn throttle_factor(warn_count: u32, throttle_at: u32) -> f32 {
    let steps_in = warn_count.saturating_sub(throttle_at);
    (0.5f32).powi(1 + steps_in as i32).max(0.05)
}

async fn suspend_backoff_ms(warn_count: u32) -> u64 {
    1_000u64.saturating_mul(1u64 << warn_count.min(10))
}

/// 🚑️ The failure policy supplies the same quarantine duration to package-wide Kernel escalation.
pub async fn quarantine_duration_ms(restart_count: u32) -> u64 {
    5_000u64.saturating_mul(1u64 << restart_count.min(10))
}
//#endregion 🚑️FailurePolicy


//#region 🗂️ActorRecord
/// 🗂️ Actor lifecycle state, driven by [`Kernel::activate`]/`suspend`/`resume` and the failure ladder.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::semio_framework_value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ActorStatus {
    Cold,
    Activating,
    Active,
    Suspended { checkpoint: Option<Vec<u8>> },
    Draining,
    Trapped,
    Quarantined,
    Disabled,
}

impl ActorStatus {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            ActorStatus::Cold => pack::write_u8(out, 0).await,
            ActorStatus::Activating => pack::write_u8(out, 1).await,
            ActorStatus::Active => pack::write_u8(out, 2).await,
            ActorStatus::Suspended { checkpoint } => {
                pack::write_u8(out, 3).await;
                pack::write_opt_bytes(out, checkpoint).await;
            }
            ActorStatus::Draining => pack::write_u8(out, 4).await,
            ActorStatus::Trapped => pack::write_u8(out, 5).await,
            ActorStatus::Quarantined => pack::write_u8(out, 6).await,
            ActorStatus::Disabled => pack::write_u8(out, 7).await,
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let tag = pack::read_u8(bytes, pos, "ActorStatus").await?;
        match tag {
            0 => Ok(ActorStatus::Cold),
            1 => Ok(ActorStatus::Activating),
            2 => Ok(ActorStatus::Active),
            3 => Ok(ActorStatus::Suspended { checkpoint: pack::read_opt_bytes(bytes, pos, "ActorStatus::Suspended").await? }),
            4 => Ok(ActorStatus::Draining),
            5 => Ok(ActorStatus::Trapped),
            6 => Ok(ActorStatus::Quarantined),
            7 => Ok(ActorStatus::Disabled),
            other => Err(pack::PackError::InvalidTag { what: "ActorStatus", tag: other, offset: *pos }),
        }
    }
}


//#region 🧩️ShardTable
/// 🧩️ Which host EXECUTION SURFACE a shard runs on — `design-runtime.md` §2's "thread, worker, or
/// process" trio, unchanged by the P1c one-pool-worker-runtime refactor EXCEPT for what `Native`
/// itself now means. A shard was never "a thread" as an identity here — `ShardTable` (below) only
/// ever pinned an [`ActorId`] to a [`ShardId`], a bookkeeping integer; this crate's own purity rule
/// means it never spawned anything. What changed in the HOST crates (`semio-framework-plugin-host`'s
/// `ShardExecutor`, `semio-framework-os`'s `NativeKernelRuntime`) is that `Native` no longer implies
/// "and therefore gets one dedicated OS thread" — a `Native` shard is now a logical affinity unit
/// (its actors' `wasmtime::Store`s are pinned to it so guest instance state stays coherent) whose
/// turns are submitted as jobs onto one process-wide `semio_framework_async::WorkerPool`, mutually
/// exclusive PER SHARD (so affinity is enforced by a lock, not by thread identity) rather than
/// concurrent by construction. `WebWorker`/`Process` were already real boundary crossings (a browser
/// worker, an OS process) unaffected by this distinction — `Native` is the one variant whose meaning
/// narrows from "a thread" to "this process, pool-scheduled."
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub enum ShardKind {
    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (P1c): renamed from `Thread` — same process,
    /// same address space, `wasmtime::Store` instances pinned by `ShardTable::pin`'s least-loaded
    /// placement, but no OS thread of its own; the host schedules its turns onto a shared
    /// `WorkerPool`. Wire tag unchanged (`0`) — this is a rename, not a new wire variant.
    Native,
    WebWorker,
    Process,
}

impl ShardKind {
    async fn tag(self) -> u8 {
        match self {
            ShardKind::Native => 0,
            ShardKind::WebWorker => 1,
            ShardKind::Process => 2,
        }
    }
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u8(out, self.tag().await).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        match pack::read_u8(bytes, pos, "ShardKind").await? {
            0 => Ok(ShardKind::Native),
            1 => Ok(ShardKind::WebWorker),
            2 => Ok(ShardKind::Process),
            other => Err(pack::PackError::InvalidTag { what: "ShardKind", tag: other, offset: *pos }),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value", transparent)]
pub struct ShardId(pub u16);

impl ShardId {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u16(out, self.0).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self(pack::read_u16(bytes, pos, "ShardId").await?))
    }
}

/// 🧮️ Host-side sizing policy: native `available_parallelism()-1` clamped `[2,8]` — pure arithmetic,
/// the actual OS query happens in the (non-pure) host binary.
pub async fn clamp_native_shard_count(available_parallelism: u16) -> u16 {
    available_parallelism.saturating_sub(1).clamp(2, 8)
}

/// 🧮️ Host-side sizing policy: web `min(hardwareConcurrency-1, 4)`.
pub async fn clamp_web_shard_count(hardware_concurrency: u16) -> u16 {
    hardware_concurrency.saturating_sub(1).clamp(1, 4)
}


#[cfg(test)]
#[path="../📃️page/🧪️tests/📃️page/🦀️.rs"]
mod byte_page_tests;
#[cfg(test)]
#[path="../📤️return/🧪️tests/📤️return/🦀️.rs"]
mod return_page_tests;
#[cfg(test)]
#[path="../📥️cold-pair/🧪️tests/📥️cold-pair/🦀️.rs"]
mod cold_pair_tests;
