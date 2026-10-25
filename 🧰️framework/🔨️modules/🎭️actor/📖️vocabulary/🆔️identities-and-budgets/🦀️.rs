//! 🆔️ Identities and budgets: the job replay request and digests, the package, actor and kind identities, the lanes and the lane budgets with their defaults. See [`super`].

use super::*;

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
        semio_framework_actor_pack::write_hash32(out, &self.controller).await;
        semio_framework_actor_pack::write_hash32(out, &self.tool).await;
        semio_framework_actor_pack::write_hash32(out, &self.schema).await;
        semio_framework_actor_pack::write_u16(out, self.version).await;
        semio_framework_actor_pack::write_hash32(out, &self.digest).await;
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        Ok(Self {
            controller: semio_framework_actor_pack::read_hash32(bytes, pos, "JobReplayRequest::controller").await?,
            tool: semio_framework_actor_pack::read_hash32(bytes, pos, "JobReplayRequest::tool").await?,
            schema: semio_framework_actor_pack::read_hash32(bytes, pos, "JobReplayRequest::schema").await?,
            version: semio_framework_actor_pack::read_u16(bytes, pos, "JobReplayRequest::version").await?,
            digest: semio_framework_actor_pack::read_hash32(bytes, pos, "JobReplayRequest::digest").await?,
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
        semio_framework_actor_pack::write_str(out, &self.0).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        Ok(Self(semio_framework_actor_pack::read_str(bytes, pos, "PackageId").await?))
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
        semio_framework_actor_pack::write_hash32(out, &self.0).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        Ok(Self(semio_framework_actor_pack::read_hash32(bytes, pos, "PackageHash").await?))
    }
}
//#endregion 📦️PackageId

//#region 🆔️ActorId
/// 🔢️ Bit-packed actor identifier: `plugin_ordinal:u16 | kind:u2 | ordinal:u32 | generation:u14`.
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

    pub fn plugin_ordinal(self) -> u16 {
        (self.0 >> ACTOR_ID_PLUGIN_SHIFT) as u16
    }

    pub fn kind_tag(self) -> u8 {
        ((self.0 >> ACTOR_ID_KIND_SHIFT) & ACTOR_ID_KIND_MASK) as u8
    }

    pub fn ordinal(self) -> u32 {
        ((self.0 >> ACTOR_ID_ORDINAL_SHIFT) & ACTOR_ID_ORDINAL_MASK) as u32
    }

    pub fn generation(self) -> u16 {
        (self.0 & ACTOR_ID_GENERATION_MASK) as u16
    }

    /// 🔁️ Restart-after-trap: same plugin/kind/ordinal, generation bumped (wraps at 14 bits — a
    /// restart storm beyond 16384 generations is itself a `Disabled`-worthy condition upstream).
    pub async fn next_generation(self) -> Self {
        Self::new(self.plugin_ordinal(), self.kind_tag(), self.ordinal(), self.generation().wrapping_add(1) & (ACTOR_ID_GENERATION_MASK as u16)).await
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        semio_framework_actor_pack::write_u64(out, self.0).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        Ok(Self(semio_framework_actor_pack::read_u64(bytes, pos, "ActorId").await?))
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
                semio_framework_actor_pack::write_u8(out, 0).await;
                plugin.pack_encode(out).await;
                semio_framework_actor_pack::write_str(out, app_id).await;
                semio_framework_actor_pack::write_u32(out, *instance_id).await;
            }
            ActorKind::Extension { plugin, extension_id } => {
                semio_framework_actor_pack::write_u8(out, 1).await;
                plugin.pack_encode(out).await;
                semio_framework_actor_pack::write_str(out, extension_id).await;
            }
            ActorKind::Job { owner, job_id } => {
                semio_framework_actor_pack::write_u8(out, 2).await;
                owner.pack_encode(out).await;
                semio_framework_actor_pack::write_u64(out, *job_id).await;
            }
        }
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        let tag = semio_framework_actor_pack::read_u8(bytes, pos, "ActorKind").await?;
        match tag {
            0 => Ok(ActorKind::PluginApp { plugin: PackageId::pack_decode(bytes, pos).await?, app_id: semio_framework_actor_pack::read_str(bytes, pos, "ActorKind::app_id").await?, instance_id: semio_framework_actor_pack::read_u32(bytes, pos, "ActorKind::instance_id").await? }),
            1 => Ok(ActorKind::Extension { plugin: PackageId::pack_decode(bytes, pos).await?, extension_id: semio_framework_actor_pack::read_str(bytes, pos, "ActorKind::extension_id").await? }),
            2 => Ok(ActorKind::Job { owner: ActorId::pack_decode(bytes, pos).await?, job_id: semio_framework_actor_pack::read_u64(bytes, pos, "ActorKind::job_id").await? }),
            other => Err(semio_framework_actor_pack::PackError::InvalidTag { what: "ActorKind", tag: other, offset: *pos }),
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

    /// 🥇️ 0 = highest priority (Interactive) .. 3 = lowest (Maintenance).
    pub async fn priority_rank(self) -> usize {
        match self {
            Lane::Interactive => 0,
            Lane::UserVisible => 1,
            Lane::Background => 2,
            Lane::Maintenance => 3,
        }
    }

    /// ⚖️ Level-2 DRR quantum weight — biases actor selection within a plugin toward interactive work.
    pub fn weight(self) -> u32 {
        match self {
            Lane::Interactive => 8,
            Lane::UserVisible => 4,
            Lane::Background => 2,
            Lane::Maintenance => 1,
        }
    }

    pub(super) async fn tag(self) -> u8 {
        self.priority_rank().await as u8
    }

    async fn from_tag(tag: u8) -> Result<Self, semio_framework_actor_pack::PackError> {
        match tag {
            0 => Ok(Lane::Interactive),
            1 => Ok(Lane::UserVisible),
            2 => Ok(Lane::Background),
            3 => Ok(Lane::Maintenance),
            other => Err(semio_framework_actor_pack::PackError::InvalidTag { what: "Lane", tag: other, offset: 0 }),
        }
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        semio_framework_actor_pack::write_u8(out, self.tag().await).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        let tag = semio_framework_actor_pack::read_u8(bytes, pos, "Lane").await?;
        Self::from_tag(tag).await
    }
}
//#endregion 🛣️Lane

//#region ⚖️Budget
/// 💰️ Per-turn resource ceilings enforced host-side. Replaces `PLUGIN_FUEL_BUDGET`.
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
    /// 🎚️ Scales a `Throttled { factor }` failure stage onto this budget — fuel/wall/effects/patch
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

    pub async fn pack_encode(&self, out: &mut Vec<u8>) -> Result<(), semio_framework_actor_pack::PackError> {
        semio_framework_actor_pack::write_u64(out, self.fuel).await;
        semio_framework_actor_pack::write_u32(out, self.wall_ms).await;
        semio_framework_actor_pack::write_u64(out, self.memory_bytes).await;
        semio_framework_actor_pack::write_u32(out, self.ui_nodes).await;
        semio_framework_actor_pack::write_u16(out, self.mailbox_len).await;
        semio_framework_actor_pack::write_u32(out, self.max_effects).await;
        semio_framework_actor_pack::write_u32(out, self.max_patch_bytes).await;
        semio_framework_actor_pack::write_u32(out,self.max_frames).await;
        Ok(())
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        Ok(Self {
            fuel: semio_framework_actor_pack::read_u64(bytes, pos, "Budget::fuel").await?,
            wall_ms: semio_framework_actor_pack::read_u32(bytes, pos, "Budget::wall_ms").await?,
            memory_bytes: semio_framework_actor_pack::read_u64(bytes, pos, "Budget::memory_bytes").await?,
            ui_nodes: semio_framework_actor_pack::read_u32(bytes, pos, "Budget::ui_nodes").await?,
            mailbox_len: semio_framework_actor_pack::read_u16(bytes, pos, "Budget::mailbox_len").await?,
            max_effects: semio_framework_actor_pack::read_u32(bytes, pos, "Budget::max_effects").await?,
            max_patch_bytes: semio_framework_actor_pack::read_u32(bytes, pos, "Budget::max_patch_bytes").await?,
            max_frames:semio_framework_actor_pack::read_u32(bytes,pos,"Budget::max_frames").await?,
        })
    }
}

//#region ⚖️LaneDefaults
/// 🚦️ Default [`Budget`] per [`Lane`] — Interactive 4ms/2M fuel, UserVisible 16ms, Background 50ms,
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
