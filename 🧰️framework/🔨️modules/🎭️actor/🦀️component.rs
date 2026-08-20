//! 🎭️ Pooled actor/plugin runtime kernel — domain-neutral, pure: no I/O, no clock (callers pass
//! `now_ms`), no `wasm_bindgen`/`web_sys`/`winit`/`tokio`/`std::thread` in this file. Transports are
//! injected via the [`ShardTransport`] trait; the one exception, [`ThreadTransport`], lives behind
//! `#[cfg(not(target_arch = "wasm32"))]` because it is built on `std::sync::mpsc`.
//!
//! 🪡 **Opaque seam**: `Effect`, `Event` and `UiPatch` are owned by a parallel packet in
//! `🎠️kernel` and are NOT defined here. This crate carries them as pack-encoded `Vec<u8>` inside
//! [`TurnResult`] (`ui_patches`, `effects`) and [`Payload::Event`] — the kernel crate encodes/decodes
//! the concrete typed collections; this crate only ever moves already-encoded bytes. The same seam
//! applies to `CapabilityGrant`: `kernel::CapabilityGrant` (`🛂️manifest/🦀️component.rs`) is an
//! application-layer type this framework-tier crate must not depend on (would invert the
//! framework → os-product layering), so [`CapabilityGrant`] here is a minimal, local, pack-codeable
//! stand-in reconciled by the `B1-host-native` packet at integration time.
//!
//! See `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️design-runtime.md` §1.

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::sync::Arc;

use serde::{Deserialize, Serialize};

//#region 🧵️Pack
/// 🧵️ Hand-rolled binary codec primitives shared by every type's `pack_encode`/`pack_decode` pair
/// below — LEB128 varints, length-prefixed bytes/strings, fixed 32-byte hashes, option/vec
/// combinators. Self-contained (no dependency on `🎒️pack`, which lives in the os-product layer).
pub mod pack {
    /// 🚨️ The one error type every `pack_decode` returns.
    #[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
    pub enum PackError {
        #[error("pack: truncated at offset {0} reading {1}")]
        Truncated(usize, &'static str),
        #[error("pack: invalid tag {tag} for {what} at offset {offset}")]
        InvalidTag { what: &'static str, tag: u8, offset: usize },
        #[error("pack: invalid utf8 in {0} at offset {1}")]
        InvalidUtf8(&'static str, usize),
        #[error("pack: overlong varint at offset {0}")]
        OverlongVarint(usize),
    }

    pub async fn write_u8(out: &mut Vec<u8>, v: u8) {
        out.push(v);
    }
    pub async fn read_u8(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<u8, PackError> {
        let byte = *bytes.get(*pos).ok_or(PackError::Truncated(*pos, what))?;
        *pos += 1;
        Ok(byte)
    }

    pub async fn write_bool(out: &mut Vec<u8>, v: bool) {
        out.push(v as u8);
    }
    pub async fn read_bool(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<bool, PackError> {
        Ok(read_u8(bytes, pos, what).await? != 0)
    }

    pub async fn write_u16(out: &mut Vec<u8>, v: u16) {
        out.extend_from_slice(&v.to_le_bytes());
    }
    pub async fn read_u16(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<u16, PackError> {
        let end = *pos + 2;
        let slice = bytes.get(*pos..end).ok_or(PackError::Truncated(*pos, what))?;
        *pos = end;
        Ok(u16::from_le_bytes(slice.try_into().expect("2 bytes")))
    }

    pub async fn write_u32(out: &mut Vec<u8>, v: u32) {
        out.extend_from_slice(&v.to_le_bytes());
    }
    pub async fn read_u32(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<u32, PackError> {
        let end = *pos + 4;
        let slice = bytes.get(*pos..end).ok_or(PackError::Truncated(*pos, what))?;
        *pos = end;
        Ok(u32::from_le_bytes(slice.try_into().expect("4 bytes")))
    }

    pub async fn write_u64(out: &mut Vec<u8>, v: u64) {
        out.extend_from_slice(&v.to_le_bytes());
    }
    pub async fn read_u64(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<u64, PackError> {
        let end = *pos + 8;
        let slice = bytes.get(*pos..end).ok_or(PackError::Truncated(*pos, what))?;
        *pos = end;
        Ok(u64::from_le_bytes(slice.try_into().expect("8 bytes")))
    }

    pub async fn write_f32(out: &mut Vec<u8>, v: f32) {
        out.extend_from_slice(&v.to_le_bytes());
    }
    pub async fn read_f32(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<f32, PackError> {
        let end = *pos + 4;
        let slice = bytes.get(*pos..end).ok_or(PackError::Truncated(*pos, what))?;
        *pos = end;
        Ok(f32::from_le_bytes(slice.try_into().expect("4 bytes")))
    }

    pub async fn write_varint_u64(out: &mut Vec<u8>, value: u64) {
        let mut remaining = value;
        loop {
            let byte = (remaining & 0x7F) as u8;
            remaining >>= 7;
            if remaining == 0 {
                out.push(byte);
                break;
            }
            out.push(byte | 0x80);
        }
    }
    pub async fn read_varint_u64(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<u64, PackError> {
        let start = *pos;
        let mut result: u64 = 0;
        for i in 0..10usize {
            let byte = *bytes.get(*pos).ok_or(PackError::Truncated(*pos, what))?;
            *pos += 1;
            let more = byte & 0x80 != 0;
            let payload = (byte & 0x7F) as u64;
            if i == 9 && (more || payload > 1) {
                return Err(PackError::OverlongVarint(start));
            }
            result |= payload << (i as u32 * 7);
            if !more {
                return Ok(result);
            }
        }
        Err(PackError::OverlongVarint(start))
    }

    pub async fn write_bytes(out: &mut Vec<u8>, v: &[u8]) {
        write_varint_u64(out, v.len() as u64).await;
        out.extend_from_slice(v);
    }
    pub async fn read_bytes(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<Vec<u8>, PackError> {
        let len = read_varint_u64(bytes, pos, what).await? as usize;
        let end = *pos + len;
        let slice = bytes.get(*pos..end).ok_or(PackError::Truncated(*pos, what))?;
        *pos = end;
        Ok(slice.to_vec())
    }

    pub async fn write_str(out: &mut Vec<u8>, v: &str) {
        write_bytes(out, v.as_bytes()).await;
    }
    pub async fn read_str(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<String, PackError> {
        let start = *pos;
        let raw = read_bytes(bytes, pos, what).await?;
        String::from_utf8(raw).map_err(|_| PackError::InvalidUtf8(what, start))
    }

    pub async fn write_hash32(out: &mut Vec<u8>, v: &[u8; 32]) {
        out.extend_from_slice(v);
    }
    pub async fn read_hash32(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<[u8; 32], PackError> {
        let end = *pos + 32;
        let slice = bytes.get(*pos..end).ok_or(PackError::Truncated(*pos, what))?;
        *pos = end;
        Ok(slice.try_into().expect("32 bytes"))
    }

    /// 🪡 Concrete (non-generic) `Option<Vec<u8>>` combinators — a closure-based generic
    /// `write_opt`/`read_opt` hits rustc's "implementation of `AsyncFnOnce` is not general enough"
    /// (the returned per-call future can't be proven to outlive a HRTB-quantified borrow of `out`/
    /// `bytes`, a known async-closures rough edge). Concrete types sidestep it entirely; every
    /// `Option<Vec<u8>>` field in this file's wire format goes through these two.
    pub async fn write_opt_bytes(out: &mut Vec<u8>, v: &Option<Vec<u8>>) {
        write_bool(out, v.is_some()).await;
        if let Some(x) = v {
            write_bytes(out, x).await;
        }
    }
    pub async fn read_opt_bytes(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<Option<Vec<u8>>, PackError> {
        if read_bool(bytes, pos, what).await? {
            Ok(Some(read_bytes(bytes, pos, what).await?))
        } else {
            Ok(None)
        }
    }

    /// 🪡 Same rationale as `write_opt_bytes`/`read_opt_bytes`, for `Option<u64>` (deadlines/wake times).
    pub async fn write_opt_u64(out: &mut Vec<u8>, v: &Option<u64>) {
        write_bool(out, v.is_some()).await;
        if let Some(x) = v {
            write_u64(out, *x).await;
        }
    }
    pub async fn read_opt_u64(bytes: &[u8], pos: &mut usize, what: &'static str) -> Result<Option<u64>, PackError> {
        if read_bool(bytes, pos, what).await? {
            Ok(Some(read_u64(bytes, pos, what).await?))
        } else {
            Ok(None)
        }
    }

    /// 🪡 `f` is a bare async fn item (`Type::pack_encode`), never a closure — fn items are
    /// automatically higher-ranked over their argument lifetimes, so they don't hit the
    /// `write_opt_bytes` doc's HRTB rough edge. `(item, out)` order (not `(out, item)`) is what
    /// lets `T::pack_encode`'s own `(&self, out)` signature line up without an adapter closure.
    pub async fn write_vec<T>(out: &mut Vec<u8>, v: &[T], mut f: impl AsyncFnMut(&T, &mut Vec<u8>)) {
        write_varint_u64(out, v.len() as u64).await;
        for item in v {
            f(item, out).await;
        }
    }
    pub async fn read_vec<T>(bytes: &[u8], pos: &mut usize, what: &'static str, mut f: impl AsyncFnMut(&[u8], &mut usize) -> Result<T, PackError>) -> Result<Vec<T>, PackError> {
        let len = read_varint_u64(bytes, pos, what).await? as usize;
        let mut out = Vec::with_capacity(len.min(1 << 20));
        for _ in 0..len {
            out.push(f(bytes, pos).await?);
        }
        Ok(out)
    }
}
//#endregion 🧵️Pack

//#region 📦️PackageId
/// 📦️ Stable identity of an installed plugin or plugin+extension pair: `<plugin>` or `<plugin>/<extension>`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct PackageHash(#[cfg_attr(feature = "typegen", ts(type = "number[]"))] pub [u8; 32]);

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
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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
        let bits = ((plugin_ordinal as u64) << ACTOR_ID_PLUGIN_SHIFT)
            | (((kind_tag as u64) & ACTOR_ID_KIND_MASK) << ACTOR_ID_KIND_SHIFT)
            | ((ordinal as u64) << ACTOR_ID_ORDINAL_SHIFT)
            | ((generation as u64) & ACTOR_ID_GENERATION_MASK);
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct Budget {
    pub fuel: u64,
    pub wall_ms: u32,
    pub memory_bytes: u64,
    pub ui_nodes: u32,
    pub mailbox_len: u16,
    pub max_effects: u32,
    pub max_patch_bytes: u32,
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
        }
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u64(out, self.fuel).await;
        pack::write_u32(out, self.wall_ms).await;
        pack::write_u64(out, self.memory_bytes).await;
        pack::write_u32(out, self.ui_nodes).await;
        pack::write_u16(out, self.mailbox_len).await;
        pack::write_u32(out, self.max_effects).await;
        pack::write_u32(out, self.max_patch_bytes).await;
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

    pub async fn budget_for(lane: Lane) -> Budget {
        match lane {
            Lane::Interactive => Budget { fuel: 2_000_000, wall_ms: 4, memory_bytes: 64 * 1024 * 1024, ui_nodes: 20_000, mailbox_len: 256, max_effects: 64, max_patch_bytes: 262_144 },
            Lane::UserVisible => Budget { fuel: 6_000_000, wall_ms: 16, memory_bytes: 96 * 1024 * 1024, ui_nodes: 20_000, mailbox_len: 256, max_effects: 128, max_patch_bytes: 524_288 },
            Lane::Background => Budget { fuel: 20_000_000, wall_ms: 50, memory_bytes: 192 * 1024 * 1024, ui_nodes: 8_000, mailbox_len: 512, max_effects: 256, max_patch_bytes: 1_048_576 },
            Lane::Maintenance => Budget { fuel: 80_000_000, wall_ms: 200, memory_bytes: 256 * 1024 * 1024, ui_nodes: 4_000, mailbox_len: 1024, max_effects: 512, max_patch_bytes: 2_097_152 },
        }
    }
}
//#endregion ⚖️LaneDefaults
//#endregion ⚖️Budget

//#region ✉️Envelope
/// 🪟 Local, opaque window identifier ([`Origin::Ui`]'s target) — the concrete `WindowHandle`
/// lives in the kernel crate; this crate only ever routes by this bare numeric id.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Origin {
    Ui { window: WindowId },
    /// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-shard-grants, Part A): struct variant, not
    /// the newtype `Actor(ActorId)` this used to be — serde's internal tagging (`#[serde(tag =
    /// "kind")]`) cannot serialize a newtype variant whose payload is not itself a map (`ActorId` is
    /// a bare `u64` tuple struct). Latent, not live (this crate has no `serde_json` dependency and
    /// the wire uses `pack_encode`/`pack_decode`), but the generated TS mirror rendered this as an
    /// impossible `{"kind":"actor"} & bigint` intersection — see `📓️luna-serde-newtype-audit.md`.
    Actor { id: ActorId },
    Kernel,
    Bus { topic: String },
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Payload {
    /// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-shard-grants, Part A): struct variant, not
    /// the newtype `Event(Vec<u8>)` this used to be — serde's internal tagging cannot serialize a
    /// newtype variant whose payload is a sequence (`serde_json` errors "cannot serialize tagged
    /// newtype variant ... containing a sequence", the exact defect `JobStep::Done`/`Failed` hit
    /// earlier this ticket). Latent, not live, on THIS crate's own pack wire — see [`Origin::Actor`]'s
    /// doc for the full context and `📓️luna-serde-newtype-audit.md`.
    Event { bytes: Vec<u8> },
    Suspend { checkpoint: bool },
    Resume { checkpoint: Option<Vec<u8>> },
    /// 🐛️ Same class of bug as [`Payload::Event`], payload is an integer this time (`serde_json`:
    /// "cannot serialize tagged newtype variant ... containing an integer") — struct variant, not
    /// the newtype `Cancel(u64)` this used to be.
    Cancel { seq: u64 },
    JobStep { job: u64 },
}

impl Payload {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            Payload::Event { bytes } => {
                pack::write_u8(out, 0).await;
                pack::write_bytes(out, bytes).await;
            }
            Payload::Suspend { checkpoint } => {
                pack::write_u8(out, 1).await;
                pack::write_bool(out, *checkpoint).await;
            }
            Payload::Resume { checkpoint } => {
                pack::write_u8(out, 2).await;
                pack::write_bool(out, checkpoint.is_some()).await;
                if let Some(v) = checkpoint {
                    pack::write_bytes(out, v).await;
                }
            }
            Payload::Cancel { seq } => {
                pack::write_u8(out, 3).await;
                pack::write_u64(out, *seq).await;
            }
            Payload::JobStep { job } => {
                pack::write_u8(out, 4).await;
                pack::write_u64(out, *job).await;
            }
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let tag = pack::read_u8(bytes, pos, "Payload").await?;
        match tag {
            0 => Ok(Payload::Event { bytes: pack::read_bytes(bytes, pos, "Payload::Event").await? }),
            1 => Ok(Payload::Suspend { checkpoint: pack::read_bool(bytes, pos, "Payload::Suspend").await? }),
            2 => Ok(Payload::Resume { checkpoint: pack::read_opt_bytes(bytes, pos, "Payload::Resume").await? }),
            3 => Ok(Payload::Cancel { seq: pack::read_u64(bytes, pos, "Payload::Cancel").await? }),
            4 => Ok(Payload::JobStep { job: pack::read_u64(bytes, pos, "Payload::JobStep").await? }),
            other => Err(pack::PackError::InvalidTag { what: "Payload", tag: other, offset: *pos }),
        }
    }
}

//#region 🔑️CoalesceKey
/// 🔑️ Latest-wins-per-`(actor, key)` coalescing key. Pointer-move, resize, presence, refresh all
/// coalesce under this — 200 stale mouse-moves must never queue.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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
        let coalesce = if pack::read_bool(bytes, pos, "Envelope::coalesce").await? {
            Some(CoalesceKey::pack_decode(bytes, pos).await?)
        } else {
            None
        };
        let cancel_of = pack::read_opt_u64(bytes, pos, "Envelope::cancel_of").await?;
        let payload = Payload::pack_decode(bytes, pos).await?;
        Ok(Self { to, from, lane, seq, deadline_ms, coalesce, cancel_of, payload })
    }
}
//#endregion ✉️Envelope

//#region 🔁️TurnResult
/// 🔁️ How a turn left the actor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum TurnStatus {
    Idle,
    MoreWork,
    CheckpointReady,
    /// 🐛️ Struct variant, not the newtype `Faulted(Vec<u8>)` this used to be — same sequence-payload
    /// serde defect as [`Payload::Event`]; see that variant's doc for the full explanation.
    Faulted { detail: Vec<u8> },
}

impl TurnStatus {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            TurnStatus::Idle => pack::write_u8(out, 0).await,
            TurnStatus::MoreWork => pack::write_u8(out, 1).await,
            TurnStatus::CheckpointReady => pack::write_u8(out, 2).await,
            TurnStatus::Faulted { detail } => {
                pack::write_u8(out, 3).await;
                pack::write_bytes(out, detail).await;
            }
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let tag = pack::read_u8(bytes, pos, "TurnStatus").await?;
        match tag {
            0 => Ok(TurnStatus::Idle),
            1 => Ok(TurnStatus::MoreWork),
            2 => Ok(TurnStatus::CheckpointReady),
            3 => Ok(TurnStatus::Faulted { detail: pack::read_bytes(bytes, pos, "TurnStatus::Faulted").await? }),
            other => Err(pack::PackError::InvalidTag { what: "TurnStatus", tag: other, offset: *pos }),
        }
    }
}

/// 📊️ What one turn actually spent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct TurnResult {
    pub ui_patches: Vec<u8>,
    pub effects: Vec<u8>,
    pub next_wake: Option<u64>,
    pub status: TurnStatus,
    pub usage: Usage,
}

impl TurnResult {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_bytes(out, &self.ui_patches).await;
        pack::write_bytes(out, &self.effects).await;
        pack::write_opt_u64(out, &self.next_wake).await;
        self.status.pack_encode(out).await;
        self.usage.pack_encode(out).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self {
            ui_patches: pack::read_bytes(bytes, pos, "TurnResult::ui_patches").await?,
            effects: pack::read_bytes(bytes, pos, "TurnResult::effects").await?,
            next_wake: pack::read_opt_u64(bytes, pos, "TurnResult::next_wake").await?,
            status: TurnStatus::pack_decode(bytes, pos).await?,
            usage: Usage::pack_decode(bytes, pos).await?,
        })
    }
}
//#endregion 🔁️TurnResult

//#region 📬️Mailbox
/// 🚦 What `Mailbox::enqueue` reports back — `Rejected` must always surface as a busy badge, never
/// a silent drop of a user action.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Backpressure {
    Accept,
    Coalesced,
    /// 🐛️ Struct variant, not the newtype `Dropped(Lane)` this used to be. `Lane` itself serializes
    /// fine alone (a plain enum, `#[serde(tag = "kind")]` with unit variants), but wrapping it as a
    /// newtype payload of ANOTHER internally-tagged enum hits the same "cannot serialize tagged
    /// newtype variant" defect as [`Payload::Event`] — the payload type does not matter, only that
    /// it is not itself a map.
    Dropped { lane: Lane },
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct Mailbox {
    pub capacity: u16,
    /// 🚫️ Excluded from the TS mirror: `ts-rs` has no `TS` impl for `VecDeque`, and this field is
    /// module-private — no TypeScript consumer can reach the per-lane rings, which cross to the web
    /// shard as pack bytes via [`Mailbox::pack_encode`], never as a structural JSON object. Emitting
    /// them would describe a shape the wire never carries. Found the first time typegen was ever run
    /// for this crate (MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, T1's lease).
    #[cfg_attr(feature = "typegen", ts(skip))]
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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
//#endregion 🔐️CapabilityGrant

//#region 🚑️FailurePolicy
/// 🚑️ What triggered a failure-ladder transition.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum FailureSignal {
    DeadlineOverrun { ratio: f32 },
    FuelExhausted,
    MemoryLimit,
    MailboxOverflow,
    UiQuota,
    /// 🐛️ Struct variant, not the newtype `Trap(String)` this used to be — `String` is a sequence
    /// of chars as far as serde's internal tagging is concerned, so it hits the exact same "cannot
    /// serialize tagged newtype variant ... containing a sequence" defect as [`Payload::Event`].
    Trap { detail: String },
    HeartbeatMissed { count: u32 },
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
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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

async fn quarantine_duration_ms(restart_count: u32) -> u64 {
    5_000u64.saturating_mul(1u64 << restart_count.min(10))
}
//#endregion 🚑️FailurePolicy

//#region 🗂️ActorRecord
/// 🗂️ Actor lifecycle state, driven by [`Kernel::activate`]/`suspend`/`resume` and the failure ladder.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
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

/// 🗂️ Full snapshot of one actor — assembled on demand by [`Kernel::actor_record`] from the
/// scheduler's live entry plus this actor's kind/capabilities/status/failure/metrics bookkeeping.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct ActorRecord {
    pub id: ActorId,
    pub kind: ActorKind,
    pub package: PackageId,
    pub shard: ShardId,
    pub capabilities: Vec<CapabilityGrant>,
    pub budget: Budget,
    pub mailbox: Mailbox,
    pub status: ActorStatus,
    pub failure: FailureState,
    pub metrics: ActorMetrics,
}

impl ActorRecord {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        self.id.pack_encode(out).await;
        self.kind.pack_encode(out).await;
        self.package.pack_encode(out).await;
        self.shard.pack_encode(out).await;
        pack::write_vec(out, &self.capabilities, CapabilityGrant::pack_encode).await;
        self.budget.pack_encode(out).await;
        self.mailbox.pack_encode(out).await;
        self.status.pack_encode(out).await;
        self.failure.pack_encode(out).await;
        self.metrics.pack_encode(out).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self {
            id: ActorId::pack_decode(bytes, pos).await?,
            kind: ActorKind::pack_decode(bytes, pos).await?,
            package: PackageId::pack_decode(bytes, pos).await?,
            shard: ShardId::pack_decode(bytes, pos).await?,
            capabilities: pack::read_vec(bytes, pos, "ActorRecord::capabilities", CapabilityGrant::pack_decode).await?,
            budget: Budget::pack_decode(bytes, pos).await?,
            mailbox: Mailbox::pack_decode(bytes, pos).await?,
            status: ActorStatus::pack_decode(bytes, pos).await?,
            failure: FailureState::pack_decode(bytes, pos).await?,
            metrics: ActorMetrics::pack_decode(bytes, pos).await?,
        })
    }
}
//#endregion 🗂️ActorRecord

//#region 🧩️ShardTable
/// 🧩️ Which host mechanism backs a shard.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub enum ShardKind {
    Thread,
    WebWorker,
    Process,
}

impl ShardKind {
    async fn tag(self) -> u8 {
        match self {
            ShardKind::Thread => 0,
            ShardKind::WebWorker => 1,
            ShardKind::Process => 2,
        }
    }
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u8(out, self.tag().await).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        match pack::read_u8(bytes, pos, "ShardKind").await? {
            0 => Ok(ShardKind::Thread),
            1 => Ok(ShardKind::WebWorker),
            2 => Ok(ShardKind::Process),
            other => Err(pack::PackError::InvalidTag { what: "ShardKind", tag: other, offset: *pos }),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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

/// 🧩️ Fixed shard pool. An actor is pinned to a shard; migration only happens at a quiescent point
/// via application-level checkpoint (never a raw linear-memory snapshot). The last `exclusive_reserve`
/// shards are reserved for [`ShardTable::request_exclusive`] leases.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct ShardTable {
    pub kind: ShardKind,
    shard_count: u16,
    exclusive_reserve: u16,
    assignment: BTreeMap<ActorId, ShardId>,
    exclusive_leases: BTreeMap<ShardId, ActorId>,
}

impl ShardTable {
    pub async fn new(kind: ShardKind, shard_count: u16, exclusive_reserve: u16) -> Self {
        let exclusive_reserve = exclusive_reserve.min(2).min(shard_count.saturating_sub(1));
        Self { kind, shard_count: shard_count.max(1), exclusive_reserve, assignment: BTreeMap::new(), exclusive_leases: BTreeMap::new() }
    }

    pub async fn shard_count(&self) -> u16 {
        self.shard_count
    }

    /// 📌️ Pins an actor to the least-loaded shard of the non-exclusive pool, idempotently.
    ///
    /// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (V1b): this was `actor.0 % pool`, which reads the
    /// LOW bits of the packed id — but the layout is `plugin_ordinal:u16 | kind:u2 | ordinal:u32 |
    /// generation:u14`, so those low bits are `generation`, which is **0 for every freshly activated
    /// actor**. Every actor pinned to shard 0; the scale bench measured `perShardCounts {"0": 100}`
    /// against 8 configured shards. The pooled-shard mechanism this ticket exists to build was
    /// distributing nothing, and no test caught it because "pin returns a valid shard" passes
    /// perfectly when every answer is 0.
    ///
    /// Least-loaded rather than a hash, because the balance bound is a REQUIREMENT, not a
    /// preference: `📓️design-workforce.md` §4's budget 3 demands no shard exceed
    /// `ceil(actors/K)+1`, which a hash cannot guarantee (its bucket variance exceeds that bound
    /// well before 100 actors) and exact balancing gives by construction. Least-loaded also refills
    /// the gaps [`unpin`] leaves, which a round-robin counter would stride straight past. Ties break
    /// on lowest shard id, so placement is deterministic — no clock, no RNG, crate stays pure.
    pub async fn pin(&mut self, actor: ActorId) -> ShardId {
        if let Some(existing) = self.assignment.get(&actor) {
            return *existing;
        }
        let shard = self.least_loaded(&BTreeSet::new()).await;
        self.assignment.insert(actor, shard);
        shard
    }

    /// 🔥️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-interactive-isolation): same least-loaded
    /// placement as [`Self::pin`], factored so [`Self::pin_avoiding`] can reuse the identical
    /// count/tie-break arithmetic with a subset of shards excluded — this is why `pin`'s own budget-3
    /// balance guarantee (`pin_spreads_actors_of_one_plugin_across_the_pool`) is untouched by this
    /// packet: called with an empty `avoid` set, `pin`'s behaviour is byte-for-byte what it was before
    /// this method existed.
    async fn least_loaded(&self, avoid: &BTreeSet<ShardId>) -> ShardId {
        let pool = self.shard_count.saturating_sub(self.exclusive_reserve).max(1);
        let mut load = vec![0usize; pool as usize];
        for shard in self.assignment.values() {
            if (shard.0 as usize) < load.len() {
                load[shard.0 as usize] += 1;
            }
        }
        let chosen = load
            .iter()
            .enumerate()
            .filter(|(index, _)| !avoid.contains(&ShardId(*index as u16)))
            .min_by_key(|(index, count)| (**count, *index))
            .or_else(|| load.iter().enumerate().min_by_key(|(index, count)| (**count, *index)))
            .map_or(0, |(index, _)| index);
        ShardId(chosen as u16)
    }

    /// 🔥️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-interactive-isolation): [`Self::pin`],
    /// restricted to shards NOT in `avoid` — "reserve headroom so an interactive actor is never
    /// co-resident with N CPU-bound ones," concretely: `Kernel::activate` calls this instead of
    /// `pin` for `Lane::Interactive` actors, passing every shard `Kernel::saturated_shards` currently
    /// judges CPU-saturating (observed `ActorMetrics::is_saturating`, never a fixture/profile name).
    /// Falls back to the ordinary unrestricted least-loaded shard when EVERY general-pool shard is in
    /// `avoid` — a fully-saturated pool must still admit the actor; `Backpressure::Rejected` (mailbox
    /// capacity) already owns "no room" semantics elsewhere, this method is never the place a
    /// placement request goes unanswered. Idempotent for an already-pinned actor, exactly like `pin`.
    pub async fn pin_avoiding(&mut self, actor: ActorId, avoid: &BTreeSet<ShardId>) -> ShardId {
        if let Some(existing) = self.assignment.get(&actor) {
            return *existing;
        }
        let shard = self.least_loaded(avoid).await;
        self.assignment.insert(actor, shard);
        shard
    }

    pub async fn unpin(&mut self, actor: ActorId) {
        self.assignment.remove(&actor);
        self.exclusive_leases.retain(|_, held_by| *held_by != actor);
    }

    pub async fn shard_of(&self, actor: ActorId) -> Option<ShardId> {
        self.assignment.get(&actor).copied()
    }

    /// 🔒️ Leases one of the reserved exclusive shards to `actor` for the duration of foreground
    /// work — a lease, not a permanent per-plugin worker. `None` when none are free.
    pub async fn request_exclusive(&mut self, actor: ActorId) -> Option<ShardId> {
        if self.exclusive_reserve == 0 {
            return None;
        }
        let base = self.shard_count - self.exclusive_reserve;
        for offset in 0..self.exclusive_reserve {
            let candidate = ShardId(base + offset);
            if let std::collections::btree_map::Entry::Vacant(entry) = self.exclusive_leases.entry(candidate) {
                entry.insert(actor);
                self.assignment.insert(actor, candidate);
                return Some(candidate);
            }
        }
        None
    }

    pub async fn release_exclusive(&mut self, actor: ActorId) {
        self.exclusive_leases.retain(|_, held_by| *held_by != actor);
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        self.kind.pack_encode(out).await;
        pack::write_u16(out, self.shard_count).await;
        pack::write_u16(out, self.exclusive_reserve).await;
        pack::write_varint_u64(out, self.assignment.len() as u64).await;
        for (actor, shard) in &self.assignment {
            actor.pack_encode(out).await;
            shard.pack_encode(out).await;
        }
        pack::write_varint_u64(out, self.exclusive_leases.len() as u64).await;
        for (shard, actor) in &self.exclusive_leases {
            shard.pack_encode(out).await;
            actor.pack_encode(out).await;
        }
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let kind = ShardKind::pack_decode(bytes, pos).await?;
        let shard_count = pack::read_u16(bytes, pos, "ShardTable::shard_count").await?;
        let exclusive_reserve = pack::read_u16(bytes, pos, "ShardTable::exclusive_reserve").await?;
        let assignment_len = pack::read_varint_u64(bytes, pos, "ShardTable::assignment").await? as usize;
        let mut assignment_pairs = Vec::with_capacity(assignment_len.min(1 << 20));
        for _ in 0..assignment_len {
            assignment_pairs.push((ActorId::pack_decode(bytes, pos).await?, ShardId::pack_decode(bytes, pos).await?));
        }
        let lease_len = pack::read_varint_u64(bytes, pos, "ShardTable::exclusive_leases").await? as usize;
        let mut lease_pairs = Vec::with_capacity(lease_len.min(1 << 20));
        for _ in 0..lease_len {
            lease_pairs.push((ShardId::pack_decode(bytes, pos).await?, ActorId::pack_decode(bytes, pos).await?));
        }
        Ok(Self { kind, shard_count, exclusive_reserve, assignment: assignment_pairs.into_iter().collect(), exclusive_leases: lease_pairs.into_iter().collect() })
    }
}
//#endregion 🧩️ShardTable

//#region ⏱️Scheduler
/// ⏱️ One actor's scheduling entry inside [`Scheduler`]: everything the DRR algorithm needs that
/// isn't shared plugin-level bookkeeping.
#[derive(Clone, Debug)]
struct ScheduledActor {
    package: PackageId,
    lane: Lane,
    budget: Budget,
    shard: ShardId,
    mailbox: Mailbox,
    active: bool,
    throttle: f32,
    deficit: i64,
}

/// ⏱️ Result of one [`Scheduler::tick`] call: the turns granted this call, and (if nothing ran) the
/// earliest future timestamp worth ticking again for.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct Decision {
    pub run: Vec<TurnGrant>,
    pub wake_at: Option<u64>,
}

impl Decision {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_vec(out, &self.run, TurnGrant::pack_encode).await;
        pack::write_opt_u64(out, &self.wake_at).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { run: pack::read_vec(bytes, pos, "Decision::run", TurnGrant::pack_decode).await?, wake_at: pack::read_opt_u64(bytes, pos, "Decision::wake_at").await? })
    }
}

/// ⏱️ One granted turn: which actor, on which shard, with what (possibly throttle-scaled) budget,
/// and the envelopes drained from its mailbox for this turn.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct TurnGrant {
    pub actor: ActorId,
    pub shard: ShardId,
    pub budget: Budget,
    pub envelopes: Vec<Envelope>,
}

impl TurnGrant {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        self.actor.pack_encode(out).await;
        self.shard.pack_encode(out).await;
        self.budget.pack_encode(out).await;
        pack::write_vec(out, &self.envelopes, Envelope::pack_encode).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { actor: ActorId::pack_decode(bytes, pos).await?, shard: ShardId::pack_decode(bytes, pos).await?, budget: Budget::pack_decode(bytes, pos).await?, envelopes: pack::read_vec(bytes, pos, "TurnGrant::envelopes", Envelope::pack_decode).await? })
    }
}

/// ⏱️ How many envelopes a single granted turn drains from its actor's mailbox in one go.
const TURN_ENVELOPE_BATCH: usize = 8;

/// ⏱️ Hierarchical deficit round-robin: level 1 fairness across plugins (packages), level 2 across
/// each plugin's own actors. Without level 1, a plugin with 50 actors would take 50x the share of a
/// plugin with one. Owns every actor's mailbox — `submit`/`tick`/`complete` are its only entry points.
#[derive(Clone, Debug, Default)]
pub struct Scheduler {
    actors: BTreeMap<ActorId, ScheduledActor>,
    plugin_deficit: HashMap<PackageId, i64>,
    plugin_order: Vec<PackageId>,
    plugin_cursor: usize,
    grants_per_tick: u32,
}

impl Scheduler {
    pub async fn new(grants_per_tick: u32) -> Self {
        Self { actors: BTreeMap::new(), plugin_deficit: HashMap::new(), plugin_order: Vec::new(), plugin_cursor: 0, grants_per_tick: grants_per_tick.max(1) }
    }

    pub async fn register_actor(&mut self, actor: ActorId, package: PackageId, lane: Lane, budget: Budget, shard: ShardId) {
        if !self.plugin_order.contains(&package) {
            self.plugin_order.push(package.clone());
        }
        self.actors.insert(actor, ScheduledActor { package, lane, budget, shard, mailbox: Mailbox::new(budget.mailbox_len).await, active: true, throttle: 1.0, deficit: 0 });
    }

    pub async fn unregister_actor(&mut self, actor: ActorId) {
        self.actors.remove(&actor);
    }

    pub async fn set_active(&mut self, actor: ActorId, active: bool) {
        if let Some(entry) = self.actors.get_mut(&actor) {
            entry.active = active;
        }
    }

    pub async fn set_shard(&mut self, actor: ActorId, shard: ShardId) {
        if let Some(entry) = self.actors.get_mut(&actor) {
            entry.shard = shard;
        }
    }

    pub async fn set_throttle(&mut self, actor: ActorId, factor: f32) {
        if let Some(entry) = self.actors.get_mut(&actor) {
            entry.throttle = factor;
        }
    }

    pub async fn mailbox_pressure(&self, actor: ActorId) -> Option<f32> {
        self.actors.get(&actor).map(|e| e.mailbox.pressure())
    }

    /// 📈️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T1): the one piece of an actor's scheduling entry
    /// [`Kernel::actor_metrics_samples`] needs that [`ActorMeta`] doesn't itself carry — `lane` is
    /// fixed at [`Scheduler::register_actor`] and never changes, so this is a plain lookup.
    // 🚫️async: E1-adjacent pure accessor — its one consumer sits inside a sync `Iterator::map`
    // closure (`Kernel::actor_metrics_samples`) — see R9 residue shape 1.
    pub fn lane_of(&self, actor: ActorId) -> Option<Lane> {
        self.actors.get(&actor).map(|e| e.lane)
    }

    pub async fn submit(&mut self, envelope: Envelope) -> Backpressure {
        match self.actors.get_mut(&envelope.to) {
            Some(entry) => entry.mailbox.enqueue(envelope).await,
            None => Backpressure::Rejected,
        }
    }

    // 🚫️async: E1-adjacent pure computation — every consumer sits inside a sync `Iterator::map`/
    // `Option::map_or`/subtract-assign call (DRR quantum/deficit accounting), which cannot await —
    // see R9 residue shape 1.
    fn actor_weight(entry: &ScheduledActor) -> i64 {
        ((entry.lane.weight() as f32) * entry.throttle.max(0.05)).round().max(1.0) as i64
    }

    /// ⏱️ Pure DRR tick. `now_ms` gates deadline preemption and actor runnability (suspend/quarantine
    /// timers); all other state is internal and persists across calls.
    pub async fn tick(&mut self, now_ms: u64) -> Decision {
        let mut run = Vec::new();
        let mut granted_this_tick: BTreeSet<ActorId> = BTreeSet::new();
        let mut budget_left = self.grants_per_tick;

        //#region 🔖️DeadlinePreemption
        // 🚨️ Interactive envelopes past (or nearest) their deadline short-circuit ahead of DRR order.
        let mut overdue: Vec<(u64, ActorId)> = self
            .actors
            .iter()
            .filter(|(_, e)| e.active && !e.mailbox.is_empty())
            .filter_map(|(id, e)| e.mailbox.earliest_deadline().filter(|d| *d <= now_ms).map(|d| (d, *id)))
            .collect();
        overdue.sort();
        for (_, actor_id) in overdue {
            if budget_left == 0 {
                break;
            }
            if granted_this_tick.contains(&actor_id) {
                continue;
            }
            if let Some(grant) = self.drain_turn(actor_id).await {
                run.push(grant);
                granted_this_tick.insert(actor_id);
                budget_left -= 1;
            }
        }
        //#endregion 🔖️DeadlinePreemption

        //#region 🔖️DrrRounds
        if budget_left > 0 && !self.plugin_order.is_empty() {
            let plugin_count = self.plugin_order.len();
            let mut idle_streak = 0usize;
            while budget_left > 0 && idle_streak < plugin_count {
                let package = self.plugin_order[self.plugin_cursor % plugin_count].clone();
                self.plugin_cursor = (self.plugin_cursor + 1) % plugin_count;
                let pending_actor_ids: Vec<ActorId> = self.actors.iter().filter(|(id, e)| e.package == package && e.active && !granted_this_tick.contains(*id) && !e.mailbox.is_empty()).map(|(id, _)| *id).collect();
                if pending_actor_ids.is_empty() {
                    self.plugin_deficit.insert(package, 0);
                    idle_streak += 1;
                    continue;
                }
                idle_streak = 0;
                let quantum = pending_actor_ids.iter().filter_map(|id| self.actors.get(id)).map(Self::actor_weight).max().unwrap_or(1);
                let mut deficit = *self.plugin_deficit.entry(package.clone()).or_insert(0);
                deficit += quantum;
                while deficit > 0 && budget_left > 0 {
                    let mut candidates: Vec<ActorId> = self.actors.iter().filter(|(id, e)| e.package == package && e.active && !granted_this_tick.contains(*id) && !e.mailbox.is_empty()).map(|(id, _)| *id).collect();
                    candidates.sort();
                    let Some(actor_id) = self.pick_level2(&candidates).await else { break };
                    let weight = self.actors.get(&actor_id).map_or(1, Self::actor_weight);
                    if let Some(grant) = self.drain_turn(actor_id).await {
                        run.push(grant);
                        granted_this_tick.insert(actor_id);
                        budget_left -= 1;
                        deficit -= weight;
                    } else {
                        break;
                    }
                }
                self.plugin_deficit.insert(package.clone(), deficit);
            }
        }
        //#endregion 🔖️DrrRounds

        // ⏰️ Earliest deadline still sitting in any active actor's mailbox after this tick's grants
        // were drained — a hint for "check again by this time," not a guarantee of a grant then.
        let wake_at = self.actors.values().filter(|e| e.active && !e.mailbox.is_empty()).filter_map(|e| e.mailbox.earliest_deadline()).min();
        Decision { run, wake_at }
    }

    /// ⏱️ Level-2 DRR: rotate a per-actor deficit within the candidate set, weighted by lane (and
    /// throttle). Returns the next actor to grant, if any candidate has enough accrued deficit —
    /// otherwise grants everyone one quantum so the first call in a burst always makes progress.
    async fn pick_level2(&mut self, candidates: &[ActorId]) -> Option<ActorId> {
        if candidates.is_empty() {
            return None;
        }
        for &id in candidates {
            let weight = self.actors.get(&id).map_or(1, Self::actor_weight);
            let entry = self.actors.get_mut(&id).unwrap();
            entry.deficit += weight;
        }
        candidates.iter().copied().max_by_key(|id| self.actors.get(id).map_or(0, |e| e.deficit))
    }

    async fn drain_turn(&mut self, actor_id: ActorId) -> Option<TurnGrant> {
        let entry = self.actors.get_mut(&actor_id)?;
        let mut envelopes = Vec::new();
        for _ in 0..TURN_ENVELOPE_BATCH {
            match entry.mailbox.pop_next().await {
                Some(e) => envelopes.push(e),
                None => break,
            }
        }
        if envelopes.is_empty() {
            return None;
        }
        entry.deficit -= Self::actor_weight(entry);
        Some(TurnGrant { actor: actor_id, shard: entry.shard, budget: entry.budget.scaled(entry.throttle).await, envelopes })
    }
}
//#endregion ⏱️Scheduler

//#region 🖼️Scene
/// 🖼️ One committed, immutable frame of a window's UI. `patches` is an opaque pack-encoded
/// `Vec<UiPatch>` blob — see the module seam docstring; `node_count` is the host-tracked total used
/// only for [`Budget::ui_nodes`] quota accounting.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct SceneSnapshot {
    pub revision: u64,
    pub committed_ms: u64,
    pub patches: Vec<u8>,
    pub node_count: u32,
}

impl SceneSnapshot {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u64(out, self.revision).await;
        pack::write_u64(out, self.committed_ms).await;
        pack::write_bytes(out, &self.patches).await;
        pack::write_u32(out, self.node_count).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { revision: pack::read_u64(bytes, pos, "SceneSnapshot::revision").await?, committed_ms: pack::read_u64(bytes, pos, "SceneSnapshot::committed_ms").await?, patches: pack::read_bytes(bytes, pos, "SceneSnapshot::patches").await?, node_count: pack::read_u32(bytes, pos, "SceneSnapshot::node_count").await? })
    }
}

/// 🖼️ Per-window scene store: a builder accumulates patches between frame boundaries;
/// [`SceneStore::commit_frame`] publishes a new immutable snapshot once per boundary. If nothing
/// was pending, the previous snapshot is reused verbatim (same `Arc`, same revision) — the UI
/// thread never waits on a plugin that missed the deadline.
#[derive(Clone, Debug)]
pub struct SceneStore {
    current: Arc<SceneSnapshot>,
    pending: Vec<(ActorId, Vec<u8>, u32)>,
    pending_node_delta: u32,
}

impl Default for SceneStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneStore {
    // 🚫️async: E1 pure constructor consumed by `Default::default` (external trait impl above) —
    // see R9
    pub fn new() -> Self {
        Self { current: Arc::new(SceneSnapshot::default()), pending: Vec::new(), pending_node_delta: 0 }
    }

    pub async fn current(&self) -> Arc<SceneSnapshot> {
        self.current.clone()
    }

    /// 🖼️ Stages one actor's patch for the next commit, enforcing `budget.max_patch_bytes` and
    /// `budget.ui_nodes` host-side (no guest trust). Over quota: the node-count contribution is
    /// truncated to the remaining headroom (never counted past the ceiling) and `UiQuota` is
    /// returned so the caller can warn/truncate the typed patch itself.
    pub async fn apply_patch(&mut self, actor: ActorId, patch_bytes: Vec<u8>, node_delta: u32, budget: &Budget) -> Result<(), FailureSignal> {
        if patch_bytes.len() as u32 > budget.max_patch_bytes {
            return Err(FailureSignal::UiQuota);
        }
        let committed = self.current.node_count;
        let allowed = budget.ui_nodes.saturating_sub(committed + self.pending_node_delta);
        if node_delta > allowed {
            self.pending.push((actor, patch_bytes, allowed));
            self.pending_node_delta += allowed;
            return Err(FailureSignal::UiQuota);
        }
        self.pending.push((actor, patch_bytes, node_delta));
        self.pending_node_delta += node_delta;
        Ok(())
    }

    /// 🖼️ Publishes a new snapshot once per frame boundary. Empty pending set: returns the current
    /// snapshot unchanged (same revision) — reused because the frame had nothing new to show.
    pub async fn commit_frame(&mut self, now_ms: u64) -> Arc<SceneSnapshot> {
        if self.pending.is_empty() {
            return self.current.clone();
        }
        let mut merged = Vec::new();
        for (_, bytes, _) in &self.pending {
            merged.extend_from_slice(bytes);
        }
        let node_count = self.current.node_count + self.pending_node_delta;
        let snapshot = Arc::new(SceneSnapshot { revision: self.current.revision + 1, committed_ms: now_ms, patches: merged, node_count });
        self.pending.clear();
        self.pending_node_delta = 0;
        self.current = snapshot.clone();
        snapshot
    }
}
//#endregion 🖼️Scene

//#region 📈️Metrics
/// 📈️ Fixed-size ring of the last 64 per-turn wall-clock samples, for a p95 estimate cheap enough
/// to keep on every actor.
/// 📈️ Ring capacity for [`ActorMetrics`]'s wall-clock sample window — chosen as a plain `usize`
/// (not a const generic array) because serde's built-in array support tops out at 32 elements.
const WALL_US_RING_CAPACITY: usize = 64;

/// 🔥️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-interactive-isolation): minimum recorded turns
/// before [`ActorMetrics::is_saturating`] will answer `true` — one slow turn (a cold cache, a GC
/// pause) must never flip an actor "hot"; only a SUSTAINED pattern does.
const SATURATION_MIN_TURNS: u64 = 2;

/// 🔥️ [`ActorMetrics::is_saturating`] fires once [`ActorMetrics::wall_us_p95`] reaches this percent
/// of the actor's OWN declared [`Budget::wall_ms`] — a p95, not the latest sample, so one fast turn
/// cannot mask a sustained pattern of near-budget turns.
const SATURATION_THRESHOLD_PERCENT: u64 = 70;

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct ActorMetrics {
    pub turns: u64,
    pub fuel_total: u64,
    pub wall_us_total: u64,
    wall_us_ring: Vec<u32>,
    wall_us_ring_len: u8,
    wall_us_ring_pos: u8,
    pub memory_bytes: u64,
    pub mailbox_len: u16,
    pub mailbox_lag_ms: u32,
    pub coalesced: u64,
    pub dropped: u64,
    pub traps: u32,
    pub restarts: u32,
    pub stage: FailureStage,
    pub shard: ShardId,
}

impl std::fmt::Debug for ActorMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActorMetrics").field("turns", &self.turns).field("wall_us_p95", &self.wall_us_p95()).field("stage", &self.stage).finish()
    }
}

impl PartialEq for ActorMetrics {
    fn eq(&self, other: &Self) -> bool {
        self.turns == other.turns && self.fuel_total == other.fuel_total && self.wall_us_total == other.wall_us_total && self.wall_us_ring == other.wall_us_ring && self.memory_bytes == other.memory_bytes && self.stage == other.stage && self.shard == other.shard
    }
}

impl Default for ActorMetrics {
    fn default() -> Self {
        Self { turns: 0, fuel_total: 0, wall_us_total: 0, wall_us_ring: vec![0; WALL_US_RING_CAPACITY], wall_us_ring_len: 0, wall_us_ring_pos: 0, memory_bytes: 0, mailbox_len: 0, mailbox_lag_ms: 0, coalesced: 0, dropped: 0, traps: 0, restarts: 0, stage: FailureStage::Healthy, shard: ShardId(0) }
    }
}

impl ActorMetrics {
    pub async fn record_turn(&mut self, usage: &Usage) {
        self.turns += 1;
        self.fuel_total += usage.fuel;
        self.wall_us_total += usage.wall_us;
        self.wall_us_ring[self.wall_us_ring_pos as usize] = usage.wall_us.min(u32::MAX as u64) as u32;
        self.wall_us_ring_pos = (self.wall_us_ring_pos + 1) % WALL_US_RING_CAPACITY as u8;
        self.wall_us_ring_len = (self.wall_us_ring_len + 1).min(WALL_US_RING_CAPACITY as u8);
        self.memory_bytes = usage.memory_bytes;
    }

    // 🚫️async: E1 pure computation consumed by `Debug::fmt` (external trait impl above) — see R9
    pub fn wall_us_p95(&self) -> u32 {
        if self.wall_us_ring_len == 0 {
            return 0;
        }
        let mut samples: Vec<u32> = self.wall_us_ring[..self.wall_us_ring_len as usize].to_vec();
        samples.sort_unstable();
        let idx = ((samples.len() as f32) * 0.95).floor() as usize;
        samples[idx.min(samples.len() - 1)]
    }

    /// 🔥️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-interactive-isolation): observed-behaviour
    /// CPU-saturation signal — purely `record_turn`'s own tracked `wall_us_p95` against `budget`'s
    /// OWN declared `wall_ms` ceiling, never a fixture/profile name (this crate has no such concept
    /// and must not gain one — see the module doc's purity rule). Used by
    /// `Kernel::saturated_shards`/`ShardTable::pin_avoiding` to keep freshly-activated interactive
    /// actors off a shard a background actor is already monopolizing.
    pub async fn is_saturating(&self, budget: &Budget) -> bool {
        if self.turns < SATURATION_MIN_TURNS {
            return false;
        }
        let budget_us = (budget.wall_ms as u64) * 1000;
        if budget_us == 0 {
            return false;
        }
        (self.wall_us_p95() as u64) * 100 >= budget_us * SATURATION_THRESHOLD_PERCENT
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u64(out, self.turns).await;
        pack::write_u64(out, self.fuel_total).await;
        pack::write_u64(out, self.wall_us_total).await;
        for sample in &self.wall_us_ring {
            pack::write_u32(out, *sample).await;
        }
        pack::write_u8(out, self.wall_us_ring_len).await;
        pack::write_u8(out, self.wall_us_ring_pos).await;
        pack::write_u64(out, self.memory_bytes).await;
        pack::write_u16(out, self.mailbox_len).await;
        pack::write_u32(out, self.mailbox_lag_ms).await;
        pack::write_u64(out, self.coalesced).await;
        pack::write_u64(out, self.dropped).await;
        pack::write_u32(out, self.traps).await;
        pack::write_u32(out, self.restarts).await;
        self.stage.pack_encode(out).await;
        self.shard.pack_encode(out).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        let turns = pack::read_u64(bytes, pos, "ActorMetrics::turns").await?;
        let fuel_total = pack::read_u64(bytes, pos, "ActorMetrics::fuel_total").await?;
        let wall_us_total = pack::read_u64(bytes, pos, "ActorMetrics::wall_us_total").await?;
        let mut wall_us_ring = Vec::with_capacity(WALL_US_RING_CAPACITY);
        for _ in 0..WALL_US_RING_CAPACITY {
            wall_us_ring.push(pack::read_u32(bytes, pos, "ActorMetrics::wall_us_ring").await?);
        }
        let wall_us_ring_len = pack::read_u8(bytes, pos, "ActorMetrics::wall_us_ring_len").await?;
        let wall_us_ring_pos = pack::read_u8(bytes, pos, "ActorMetrics::wall_us_ring_pos").await?;
        Ok(Self {
            turns,
            fuel_total,
            wall_us_total,
            wall_us_ring,
            wall_us_ring_len,
            wall_us_ring_pos,
            memory_bytes: pack::read_u64(bytes, pos, "ActorMetrics::memory_bytes").await?,
            mailbox_len: pack::read_u16(bytes, pos, "ActorMetrics::mailbox_len").await?,
            mailbox_lag_ms: pack::read_u32(bytes, pos, "ActorMetrics::mailbox_lag_ms").await?,
            coalesced: pack::read_u64(bytes, pos, "ActorMetrics::coalesced").await?,
            dropped: pack::read_u64(bytes, pos, "ActorMetrics::dropped").await?,
            traps: pack::read_u32(bytes, pos, "ActorMetrics::traps").await?,
            restarts: pack::read_u32(bytes, pos, "ActorMetrics::restarts").await?,
            stage: FailureStage::pack_decode(bytes, pos).await?,
            shard: ShardId::pack_decode(bytes, pos).await?,
        })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct ShardMetrics {
    pub actors: u32,
    pub busy_ratio: f32,
    pub heartbeat_age_ms: u32,
}

impl ShardMetrics {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u32(out, self.actors).await;
        pack::write_f32(out, self.busy_ratio).await;
        pack::write_u32(out, self.heartbeat_age_ms).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { actors: pack::read_u32(bytes, pos, "ShardMetrics::actors").await?, busy_ratio: pack::read_f32(bytes, pos, "ShardMetrics::busy_ratio").await?, heartbeat_age_ms: pack::read_u32(bytes, pos, "ShardMetrics::heartbeat_age_ms").await? })
    }
}

/// 📈️ Sampled by `Kernel::metrics()`; the host publishes this as bus topic `os.runtime.metrics` at 2Hz.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct KernelMetrics {
    pub actors: u32,
    pub shards: u32,
    pub packages: u32,
}

impl KernelMetrics {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        pack::write_u32(out, self.actors).await;
        pack::write_u32(out, self.shards).await;
        pack::write_u32(out, self.packages).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { actors: pack::read_u32(bytes, pos, "KernelMetrics::actors").await?, shards: pack::read_u32(bytes, pos, "KernelMetrics::shards").await?, packages: pack::read_u32(bytes, pos, "KernelMetrics::packages").await? })
    }
}

//#region 🗒️RuntimeMetricsSnapshot
/// 🗒️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T1): one live actor's row for the `os.runtime.metrics`
/// publication — [`ActorMetrics`] joined with the kernel-level bookkeeping ([`PackageId`]/[`Lane`]/
/// [`ActorStatus`]) it doesn't itself carry. Built by [`Kernel::actor_metrics_samples`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct ActorMetricsSample {
    pub id: ActorId,
    pub package: PackageId,
    pub lane: Lane,
    pub status: ActorStatus,
    pub metrics: ActorMetrics,
}

impl ActorMetricsSample {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        self.id.pack_encode(out).await;
        self.package.pack_encode(out).await;
        self.lane.pack_encode(out).await;
        self.status.pack_encode(out).await;
        self.metrics.pack_encode(out).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { id: ActorId::pack_decode(bytes, pos).await?, package: PackageId::pack_decode(bytes, pos).await?, lane: Lane::pack_decode(bytes, pos).await?, status: ActorStatus::pack_decode(bytes, pos).await?, metrics: ActorMetrics::pack_decode(bytes, pos).await? })
    }
}

/// 🗒️ One shard's row for the `os.runtime.metrics` publication. `metrics.heartbeat_age_ms` is left at
/// its `Default` (0) by [`Kernel::shard_metrics_samples`] — the pure crate has no clock/transport of
/// its own (`important.md`'s purity rule), so a host overlays the real value from its own
/// `ShardTransport::heartbeat()` reading before publishing.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct ShardMetricsSample {
    pub shard: ShardId,
    pub metrics: ShardMetrics,
}

impl ShardMetricsSample {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        self.shard.pack_encode(out).await;
        self.metrics.pack_encode(out).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { shard: ShardId::pack_decode(bytes, pos).await?, metrics: ShardMetrics::pack_decode(bytes, pos).await? })
    }
}

/// 🗒️ The exact payload [`KernelMetrics`]'s own doc comment promises: "the host publishes this as bus
/// topic `os.runtime.metrics` at 2Hz." Built by [`Kernel::runtime_metrics_snapshot`], which takes
/// `sampled_at_ms` as a parameter rather than reading a clock — the crate core has none (transports
/// and time are injected, per this crate's own `Cargo.toml` description).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct RuntimeMetricsSnapshot {
    pub kernel: KernelMetrics,
    pub actors: Vec<ActorMetricsSample>,
    pub shards: Vec<ShardMetricsSample>,
    pub sampled_at_ms: u64,
}

impl RuntimeMetricsSnapshot {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        self.kernel.pack_encode(out).await;
        pack::write_vec(out, &self.actors, ActorMetricsSample::pack_encode).await;
        pack::write_vec(out, &self.shards, ShardMetricsSample::pack_encode).await;
        pack::write_u64(out, self.sampled_at_ms).await;
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self {
            kernel: KernelMetrics::pack_decode(bytes, pos).await?,
            actors: pack::read_vec(bytes, pos, "RuntimeMetricsSnapshot::actors", ActorMetricsSample::pack_decode).await?,
            shards: pack::read_vec(bytes, pos, "RuntimeMetricsSnapshot::shards", ShardMetricsSample::pack_decode).await?,
            sampled_at_ms: pack::read_u64(bytes, pos, "RuntimeMetricsSnapshot::sampled_at_ms").await?,
        })
    }
}

/// ⏱️ 2Hz, matching [`KernelMetrics`]'s doc comment. A `const`, not a config knob — no caller of
/// [`runtime_metrics_due`] currently needs a different cadence.
pub const RUNTIME_METRICS_PUBLISH_INTERVAL_MS: u64 = 500;

/// ⏱️ Pure cadence gate for a host's `os.runtime.metrics` publish loop — clock injected via `now_ms`
/// (never read internally), so a host can drive this off whatever tick source it already has (a
/// native thread's loop timer, a web `requestAnimationFrame`/`setInterval`). `None` (never published
/// yet) is always due.
pub async fn runtime_metrics_due(last_published_ms: Option<u64>, now_ms: u64) -> bool {
    match last_published_ms {
        None => true,
        Some(last) => now_ms.saturating_sub(last) >= RUNTIME_METRICS_PUBLISH_INTERVAL_MS,
    }
}
//#endregion 🗒️RuntimeMetricsSnapshot
//#endregion 📈️Metrics

//#region 🚚️ShardTransport
/// 🚚️ Injected duplex byte transport to one shard. `ThreadTransport` is the only impl living in
/// this crate (native, `std::sync::mpsc`); `WorkerTransport` (postMessage) and `ProcessTransport`
/// (stdio) are host-supplied — all three carry the same `Envelope`/`TurnResult` pack encoding.
pub trait ShardTransport: Send {
    async fn send(&self, bytes: &[u8]);
    async fn recv(&self) -> Option<Vec<u8>>;
    async fn heartbeat(&self) -> u64;
    async fn kill(&self);
}

//#region 🔖️ThreadTransport
#[cfg(not(target_arch = "wasm32"))]
mod thread_transport {
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
    use std::sync::mpsc::{Receiver, Sender};
    use std::sync::{Arc, Mutex};

    use super::ShardTransport;

    /// 🚚️ Native duplex transport over `std::sync::mpsc` — one end held by the kernel, the mirror
    /// end by the shard loop. Gated to non-wasm32 only; the pure crate core never spawns threads
    /// itself (the shard loop that reads/writes the mirror end lives in `plugin/host`, packet B1).
    pub struct ThreadTransport {
        outbound: Sender<Vec<u8>>,
        inbound: Mutex<Receiver<Vec<u8>>>,
        heartbeat_ms: Arc<AtomicU64>,
        killed: Arc<AtomicBool>,
    }

    impl ThreadTransport {
        /// 🔗️ Builds both ends of one duplex link, sharing the heartbeat/kill flags.
        pub async fn new_pair() -> (ThreadTransport, ThreadTransport) {
            let (tx_a, rx_a) = std::sync::mpsc::channel();
            let (tx_b, rx_b) = std::sync::mpsc::channel();
            let heartbeat_ms = Arc::new(AtomicU64::new(0));
            let killed = Arc::new(AtomicBool::new(false));
            let side_a = ThreadTransport { outbound: tx_a, inbound: Mutex::new(rx_b), heartbeat_ms: heartbeat_ms.clone(), killed: killed.clone() };
            let side_b = ThreadTransport { outbound: tx_b, inbound: Mutex::new(rx_a), heartbeat_ms, killed };
            (side_a, side_b)
        }

        /// 💓️ Records a heartbeat timestamp — called by whichever side is standing in for the
        /// shard loop's `Atomics.store` in the real web transport.
        pub async fn beat(&self, now_ms: u64) {
            self.heartbeat_ms.store(now_ms, Ordering::SeqCst);
        }

        /// ⏳️ Blocking receive with a timeout — `ShardExecutor` (terra-shard-grants,
        /// `🖥️host/🧵️shard/🏃️executor.rs`) parks here between pumps instead of a caller-driven
        /// busy-poll loop. `mpsc::Receiver::recv_timeout` blocks only the CALLING thread; it spawns
        /// no thread of its own, so the purity grep this crate's core is verified against (no
        /// `std::thread` in `🦀️component.rs`) still holds. Not part of the [`ShardTransport`]
        /// trait — `WorkerTransport`/`ProcessTransport` have their own, different ways of parking
        /// (a JS event loop, a blocking `read`), so this stays a `ThreadTransport`-only inherent
        /// method, the same shape `Self::beat` already uses for its own extra, non-trait surface.
        pub async fn recv_deadline(&self, timeout: std::time::Duration) -> Option<Vec<u8>> {
            if self.killed.load(Ordering::SeqCst) {
                return None;
            }
            self.inbound.lock().ok().and_then(|rx| rx.recv_timeout(timeout).ok())
        }
    }

    impl ShardTransport for ThreadTransport {
        async fn send(&self, bytes: &[u8]) {
            if self.killed.load(Ordering::SeqCst) {
                return;
            }
            let _ = self.outbound.send(bytes.to_vec());
        }

        async fn recv(&self) -> Option<Vec<u8>> {
            if self.killed.load(Ordering::SeqCst) {
                return None;
            }
            self.inbound.lock().ok().and_then(|rx| rx.try_recv().ok())
        }

        async fn heartbeat(&self) -> u64 {
            self.heartbeat_ms.load(Ordering::SeqCst)
        }

        async fn kill(&self) {
            self.killed.store(true, Ordering::SeqCst);
        }
    }
}
#[cfg(not(target_arch = "wasm32"))]
pub use thread_transport::ThreadTransport;
//#endregion 🔖️ThreadTransport
//#endregion 🚚️ShardTransport

//#region 🏛️Kernel
/// 🏛️ What activated an actor.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ActivationEvent {
    Manual,
    WindowOpen { window: WindowId },
    Restart,
}

/// 🚨️ Errors the [`Kernel`] façade's fallible operations return.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum KernelError {
    #[error("unknown actor")]
    UnknownActor,
    #[error("no exclusive shard available")]
    NoExclusiveShard,
    #[error("invalid status transition")]
    InvalidTransition,
}

/// 🏛️ Per-actor bookkeeping the [`Scheduler`] doesn't need to see: kind, capabilities, status,
/// failure ladder, metrics, and which window (if any) its scene patches target.
#[derive(Clone, Debug)]
struct ActorMeta {
    kind: ActorKind,
    package: PackageId,
    capabilities: Vec<CapabilityGrant>,
    budget: Budget,
    status: ActorStatus,
    failure: FailureState,
    metrics: ActorMetrics,
    window: Option<WindowId>,
}

/// 🏛️ The one-implementation, three-host runtime façade: `submit`/`tick`/`complete`/`activate`/
/// `suspend`/`resume`/`request_exclusive`/`commit_frame`/`metrics`. Composes [`Scheduler`],
/// [`ShardTable`], per-window [`SceneStore`]s and the failure ladder over one actor registry.
#[derive(Clone, Debug)]
pub struct Kernel {
    scheduler: Scheduler,
    shards: ShardTable,
    scenes: HashMap<WindowId, SceneStore>,
    actors: HashMap<ActorId, ActorMeta>,
    next_ordinal: HashMap<PackageId, u32>,
}

impl Kernel {
    pub async fn new(shard_kind: ShardKind, shard_count: u16, exclusive_reserve: u16, grants_per_tick: u32) -> Self {
        Self { scheduler: Scheduler::new(grants_per_tick).await, shards: ShardTable::new(shard_kind, shard_count, exclusive_reserve).await, scenes: HashMap::new(), actors: HashMap::new(), next_ordinal: HashMap::new() }
    }

    /// ▶️ Instantiates a fresh actor for `kind` under `package`/`lane`, pins it to a shard, and
    /// returns its freshly minted [`ActorId`] (generation 0).
    ///
    /// 🔥️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-interactive-isolation): `Lane::Interactive`
    /// alone pays [`Self::saturated_shards`]'s avoidance cost, via [`ShardTable::pin_avoiding`] rather
    /// than [`ShardTable::pin`] — every other lane pins exactly as before this packet (see
    /// `pin_avoiding`'s own doc for why: a wholesale avoid-saturated-shards policy for EVERY lane
    /// would just relocate the count imbalance `pin`'s own budget-3 guarantee depends on, not fix
    /// interactive latency).
    pub async fn activate(&mut self, package: PackageId, plugin_ordinal: u16, kind: ActorKind, lane: Lane, window: Option<WindowId>, _event: ActivationEvent) -> ActorId {
        let ordinal = self.next_ordinal.entry(package.clone()).or_insert(0);
        let id = ActorId::new(plugin_ordinal, kind.tag().await, *ordinal, 0).await;
        *ordinal += 1;
        let budget = lane_defaults::budget_for(lane).await;
        let shard = if lane == Lane::Interactive {
            let avoid = self.saturated_shards().await;
            self.shards.pin_avoiding(id, &avoid).await
        } else {
            self.shards.pin(id).await
        };
        self.scheduler.register_actor(id, package.clone(), lane, budget, shard).await;
        self.actors.insert(id, ActorMeta { kind, package, capabilities: Vec::new(), budget, status: ActorStatus::Activating, failure: FailureState::new(), metrics: ActorMetrics::default(), window });
        id
    }

    /// 🔥️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-interactive-isolation): every shard
    /// currently hosting at least one actor [`ActorMetrics::is_saturating`] judges CPU-saturating —
    /// purely from this kernel's own tracked state (turns already completed via [`Self::complete`]),
    /// never a fixture/profile name. `activate`'s own doc explains why only `Lane::Interactive`
    /// consults this.
    async fn saturated_shards(&self) -> BTreeSet<ShardId> {
        let mut out = BTreeSet::new();
        for (id, meta) in &self.actors {
            if meta.metrics.is_saturating(&meta.budget).await {
                if let Some(shard) = self.shards.shard_of(*id).await {
                    out.insert(shard);
                }
            }
        }
        out
    }

    pub async fn submit(&mut self, envelope: &Envelope) -> Backpressure {
        let backpressure = self.scheduler.submit(envelope.clone()).await;
        if let Some(meta) = self.actors.get_mut(&envelope.to) {
            match backpressure {
                Backpressure::Coalesced => meta.metrics.coalesced += 1,
                Backpressure::Dropped { .. } => meta.metrics.dropped += 1,
                _ => {}
            }
            if let Some(pressure) = self.scheduler.mailbox_pressure(envelope.to).await {
                meta.metrics.mailbox_len = (pressure * meta.budget.mailbox_len as f32) as u16;
            }
        }
        backpressure
    }

    pub async fn tick(&mut self, now_ms: u64) -> Decision {
        self.scheduler.tick(now_ms).await
    }

    /// ✅️ Records a turn's result against its actor: usage metrics, failure-ladder update (clean
    /// turn vs. `Faulted`), and status transition. Returns the escalation the caller (host) must
    /// act on for `Trapped`/`Quarantined` outcomes.
    pub async fn complete(&mut self, actor: ActorId, result: &TurnResult, now_ms: u64) -> Result<FailureEscalation, KernelError> {
        let meta = self.actors.get_mut(&actor).ok_or(KernelError::UnknownActor)?;
        meta.metrics.record_turn(&result.usage).await;
        let escalation = match &result.status {
            TurnStatus::Faulted { detail } => {
                let detail_string = String::from_utf8_lossy(detail).into_owned();
                meta.failure.on_signal(&FailureSignal::Trap { detail: detail_string }, Lane::Interactive, now_ms).await
            }
            _ => {
                meta.failure.on_clean_turn(now_ms).await;
                FailureEscalation::None
            }
        };
        meta.status = match &meta.failure.stage {
            FailureStage::Trapped { restarts } => {
                meta.metrics.traps += 1;
                meta.metrics.restarts = *restarts;
                ActorStatus::Trapped
            }
            FailureStage::Quarantined { .. } => ActorStatus::Quarantined,
            FailureStage::Disabled => ActorStatus::Disabled,
            FailureStage::Cancelled => ActorStatus::Draining,
            _ => ActorStatus::Active,
        };
        meta.metrics.stage = meta.failure.stage;
        let throttle = meta.failure.throttle_factor();
        let package = meta.package.clone();
        self.scheduler.set_throttle(actor, throttle.await).await;
        if let FailureEscalation::QuarantinePackage = escalation {
            self.quarantine_package(&package, now_ms).await;
        }
        Ok(escalation)
    }

    /// 🚑️ Package-wide quarantine: every actor sharing `package` is stopped, regardless of its own
    /// individual failure history.
    async fn quarantine_package(&mut self, package: &PackageId, now_ms: u64) {
        let affected: Vec<ActorId> = self.actors.iter().filter(|(_, m)| &m.package == package).map(|(id, _)| *id).collect();
        for actor in affected {
            if let Some(meta) = self.actors.get_mut(&actor) {
                meta.failure.stage = FailureStage::Quarantined { until: now_ms + quarantine_duration_ms(meta.failure.restart_count.max(1)).await };
                meta.status = ActorStatus::Quarantined;
                meta.metrics.stage = meta.failure.stage;
            }
            self.scheduler.set_active(actor, false).await;
        }
    }

    pub async fn suspend(&mut self, actor: ActorId, checkpoint: Option<Vec<u8>>) -> Result<(), KernelError> {
        let meta = self.actors.get_mut(&actor).ok_or(KernelError::UnknownActor)?;
        meta.status = ActorStatus::Suspended { checkpoint };
        self.scheduler.set_active(actor, false).await;
        Ok(())
    }

    pub async fn resume(&mut self, actor: ActorId) -> Result<(), KernelError> {
        let meta = self.actors.get_mut(&actor).ok_or(KernelError::UnknownActor)?;
        match meta.status {
            ActorStatus::Suspended { .. } => {
                meta.status = ActorStatus::Active;
                self.scheduler.set_active(actor, true).await;
                Ok(())
            }
            _ => Err(KernelError::InvalidTransition),
        }
    }

    pub async fn request_exclusive(&mut self, actor: ActorId) -> Result<ShardId, KernelError> {
        if !self.actors.contains_key(&actor) {
            return Err(KernelError::UnknownActor);
        }
        let shard = self.shards.request_exclusive(actor).await.ok_or(KernelError::NoExclusiveShard)?;
        self.scheduler.set_shard(actor, shard).await;
        Ok(shard)
    }

    pub async fn release_exclusive(&mut self, actor: ActorId) {
        self.shards.release_exclusive(actor).await;
        if let Some(shard) = self.shards.shard_of(actor).await {
            self.scheduler.set_shard(actor, shard).await;
        }
    }

    /// 🖼️ Routes an actor's `TurnResult.ui_patches`/estimated node-delta into its window's
    /// [`SceneStore`], enforcing quota. No-op (Ok) when the actor has no associated window.
    pub async fn apply_scene_patch(&mut self, actor: ActorId, patch_bytes: Vec<u8>, node_delta: u32) -> Result<(), FailureSignal> {
        let Some(meta) = self.actors.get(&actor) else { return Ok(()) };
        let Some(window) = meta.window else { return Ok(()) };
        let budget = meta.budget;
        self.scenes.entry(window).or_default().apply_patch(actor, patch_bytes, node_delta, &budget).await
    }

    /// 🖼️ Commits every window's pending patches at a frame boundary.
    pub async fn commit_frame(&mut self, now_ms: u64) -> HashMap<WindowId, Arc<SceneSnapshot>> {
        let mut out = HashMap::with_capacity(self.scenes.len());
        for (window, store) in self.scenes.iter_mut() {
            out.insert(*window, store.commit_frame(now_ms).await);
        }
        out
    }

    pub async fn scene_of(&self, window: WindowId) -> Option<Arc<SceneSnapshot>> {
        match self.scenes.get(&window) {
            Some(s) => Some(s.current().await),
            None => None,
        }
    }

    pub async fn metrics(&self) -> KernelMetrics {
        let packages: BTreeSet<&PackageId> = self.actors.values().map(|m| &m.package).collect();
        KernelMetrics { actors: self.actors.len() as u32, shards: self.shards.shard_count().await as u32, packages: packages.len() as u32 }
    }

    /// 🗂️ Assembles a full [`ActorRecord`] snapshot from the scheduler's live entry and this
    /// actor's own bookkeeping — a convenience for hosts/tests, not itself part of the required façade.
    pub async fn actor_record(&self, actor: ActorId) -> Option<ActorRecord> {
        let meta = self.actors.get(&actor)?;
        let shard = self.shards.shard_of(actor).await.unwrap_or(ShardId(0));
        let mailbox = Mailbox::new(meta.budget.mailbox_len).await;
        Some(ActorRecord { id: actor, kind: meta.kind.clone(), package: meta.package.clone(), shard, capabilities: meta.capabilities.clone(), budget: meta.budget, mailbox, status: meta.status.clone(), failure: meta.failure.clone(), metrics: meta.metrics.clone() })
    }

    pub async fn actor_status(&self, actor: ActorId) -> Option<&ActorStatus> {
        self.actors.get(&actor).map(|m| &m.status)
    }

    pub async fn actor_failure(&self, actor: ActorId) -> Option<&FailureState> {
        self.actors.get(&actor).map(|m| &m.failure)
    }

    /// 📈️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T1): one [`ActorMetricsSample`] per live actor —
    /// the per-actor rows of the `os.runtime.metrics` publication (`KernelMetrics`'s own doc comment).
    pub async fn actor_metrics_samples(&self) -> Vec<ActorMetricsSample> {
        self.actors.iter().map(|(id, meta)| ActorMetricsSample { id: *id, package: meta.package.clone(), lane: self.scheduler.lane_of(*id).unwrap_or(Lane::Background), status: meta.status.clone(), metrics: meta.metrics.clone() }).collect()
    }

    /// 📈️ One [`ShardMetricsSample`] per shard holding at least one actor. `busy_ratio` is the
    /// fraction of that shard's actors currently [`ActorStatus::Active`] — computable purely from
    /// kernel-tracked status, no clock needed. `heartbeat_age_ms` is left at 0; see
    /// [`ShardMetricsSample`]'s own doc comment for why a host must overlay it.
    pub async fn shard_metrics_samples(&self) -> Vec<ShardMetricsSample> {
        let mut per_shard: HashMap<ShardId, (u32, u32)> = HashMap::new();
        for (id, meta) in &self.actors {
            let shard = self.shards.shard_of(*id).await.unwrap_or(ShardId(0));
            let entry = per_shard.entry(shard).or_insert((0, 0));
            entry.1 += 1;
            if meta.status == ActorStatus::Active {
                entry.0 += 1;
            }
        }
        per_shard
            .into_iter()
            .map(|(shard, (active, total))| ShardMetricsSample { shard, metrics: ShardMetrics { actors: total, busy_ratio: if total > 0 { active as f32 / total as f32 } else { 0.0 }, heartbeat_age_ms: 0 } })
            .collect()
    }

    /// 📈️ Assembles the full `os.runtime.metrics` payload — [`Kernel::metrics`] plus every actor's and
    /// shard's sample — for a host to pack-encode and publish. `sampled_at_ms` is the caller's clock
    /// reading, never read internally (this crate's purity rule: transports and time are injected).
    pub async fn runtime_metrics_snapshot(&self, sampled_at_ms: u64) -> RuntimeMetricsSnapshot {
        RuntimeMetricsSnapshot { kernel: self.metrics().await, actors: self.actor_metrics_samples().await, shards: self.shard_metrics_samples().await, sampled_at_ms }
    }
}
//#endregion 🏛️Kernel

#[cfg(test)]
mod tests {
    mod quick {
        use crate::*;
        use std::sync::Arc;

        //#region 🔖️Helpers
        async fn env(to: ActorId, lane: Lane, seq: u64) -> Envelope {
            Envelope { to, from: Origin::Kernel, lane, seq, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: vec![1, 2, 3] } }
        }

        async fn ok_turn() -> TurnResult {
            TurnResult { ui_patches: vec![], effects: vec![], next_wake: None, status: TurnStatus::Idle, usage: Usage { fuel: 100, wall_us: 50, memory_bytes: 1024 } }
        }
        //#endregion 🔖️Helpers

        //#region 🔖️PackRoundTrips
        macro_rules! round_trip {
            ($name:ident, $ty:ty, $value:expr) => {
                #[semio_framework_async_macros::async_test]
                async fn $name() {
                    let value: $ty = $value;
                    let mut bytes = Vec::new();
                    value.pack_encode(&mut bytes).await;
                    let mut pos = 0usize;
                    let decoded = <$ty>::pack_decode(&bytes, &mut pos).await.unwrap();
                    assert_eq!(pos, bytes.len());
                    assert_eq!(decoded, value);
                }
            };
        }

        round_trip!(pack_round_trip_package_id, PackageId, PackageId("s.cad/extrude".into()));
        round_trip!(pack_round_trip_package_hash, PackageHash, PackageHash([7u8; 32]));
        round_trip!(pack_round_trip_actor_id, ActorId, ActorId::new(42, 1, 99, 3).await);
        round_trip!(pack_round_trip_actor_kind, ActorKind, ActorKind::PluginApp { plugin: PackageId("s.cad".into()), app_id: "s.cad.editor".into(), instance_id: 7 });
        round_trip!(pack_round_trip_lane, Lane, Lane::UserVisible);
        round_trip!(pack_round_trip_budget, Budget, lane_defaults::budget_for(Lane::Interactive).await);
        round_trip!(pack_round_trip_window_id, WindowId, WindowId(5));
        round_trip!(pack_round_trip_origin, Origin, Origin::Bus { topic: "os.runtime.metrics".into() });
        round_trip!(pack_round_trip_payload, Payload, Payload::Resume { checkpoint: Some(vec![9, 9, 9]) });
        round_trip!(pack_round_trip_coalesce_key, CoalesceKey, CoalesceKey("pointer-move".into()));
        round_trip!(pack_round_trip_envelope, Envelope, env(ActorId::new(1, 0, 1, 0).await, Lane::Interactive, 7).await);
        round_trip!(pack_round_trip_turn_status, TurnStatus, TurnStatus::Faulted { detail: vec![1, 2] });
        round_trip!(pack_round_trip_usage, Usage, Usage { fuel: 1, wall_us: 2, memory_bytes: 3 });
        round_trip!(pack_round_trip_turn_result, TurnResult, ok_turn().await);
        round_trip!(pack_round_trip_backpressure, Backpressure, Backpressure::Dropped { lane: Lane::Background });
        round_trip!(pack_round_trip_capability_grant, CapabilityGrant, CapabilityGrant { capability: "fs.read".into(), scope: Some(vec![1]) });
        round_trip!(pack_round_trip_failure_signal, FailureSignal, FailureSignal::HeartbeatMissed { count: 2 });
        round_trip!(pack_round_trip_failure_stage, FailureStage, FailureStage::Throttled { factor: 0.25 });
        round_trip!(pack_round_trip_failure_state, FailureState, FailureState { stage: FailureStage::Warned, clean_turns: 1, warn_count: 2, restart_count: 0, last_signal_ms: 500 });
        round_trip!(pack_round_trip_actor_status, ActorStatus, ActorStatus::Suspended { checkpoint: Some(vec![1, 2, 3]) });
        round_trip!(pack_round_trip_shard_id, ShardId, ShardId(3));
        round_trip!(pack_round_trip_shard_kind, ShardKind, ShardKind::WebWorker);
        round_trip!(pack_round_trip_decision, Decision, Decision { run: vec![TurnGrant { actor: ActorId::new(1, 0, 0, 0).await, shard: ShardId(0), budget: lane_defaults::budget_for(Lane::Background).await, envelopes: vec![] }], wake_at: Some(10) });
        round_trip!(pack_round_trip_turn_grant, TurnGrant, TurnGrant { actor: ActorId::new(2, 1, 3, 0).await, shard: ShardId(1), budget: lane_defaults::budget_for(Lane::Maintenance).await, envelopes: vec![env(ActorId::new(2, 1, 3, 0).await, Lane::Maintenance, 1).await] });
        round_trip!(pack_round_trip_scene_snapshot, SceneSnapshot, SceneSnapshot { revision: 3, committed_ms: 12, patches: vec![9, 9], node_count: 40 });
        round_trip!(pack_round_trip_shard_metrics, ShardMetrics, ShardMetrics { actors: 3, busy_ratio: 0.5, heartbeat_age_ms: 12 });
        round_trip!(pack_round_trip_kernel_metrics, KernelMetrics, KernelMetrics { actors: 3, shards: 4, packages: 2 });
        round_trip!(pack_round_trip_actor_metrics_sample, ActorMetricsSample, ActorMetricsSample { id: ActorId::new(1, 0, 2, 0).await, package: PackageId("s.cad".into()), lane: Lane::UserVisible, status: ActorStatus::Active, metrics: ActorMetrics::default() });
        round_trip!(pack_round_trip_shard_metrics_sample, ShardMetricsSample, ShardMetricsSample { shard: ShardId(2), metrics: ShardMetrics { actors: 5, busy_ratio: 0.4, heartbeat_age_ms: 8 } });
        round_trip!(
            pack_round_trip_runtime_metrics_snapshot,
            RuntimeMetricsSnapshot,
            RuntimeMetricsSnapshot {
                kernel: KernelMetrics { actors: 1, shards: 1, packages: 1 },
                actors: vec![ActorMetricsSample { id: ActorId::new(0, 0, 0, 0).await, package: PackageId("s.a".into()), lane: Lane::Interactive, status: ActorStatus::Active, metrics: ActorMetrics::default() }],
                shards: vec![ShardMetricsSample { shard: ShardId(0), metrics: ShardMetrics { actors: 1, busy_ratio: 1.0, heartbeat_age_ms: 0 } }],
                sampled_at_ms: 1234,
            }
        );

        #[semio_framework_async_macros::async_test]
        async fn pack_round_trip_mailbox() {
            let mut mailbox = Mailbox::new(4).await;
            mailbox.enqueue(env(ActorId::new(1, 0, 0, 0).await, Lane::Interactive, 1).await).await;
            mailbox.enqueue(env(ActorId::new(1, 0, 0, 0).await, Lane::Background, 2).await).await;
            let mut bytes = Vec::new();
            mailbox.pack_encode(&mut bytes).await;
            let mut pos = 0usize;
            let decoded = Mailbox::pack_decode(&bytes, &mut pos).await.unwrap();
            assert_eq!(pos, bytes.len());
            assert_eq!(decoded.len(), mailbox.len());
            assert_eq!(decoded.capacity, mailbox.capacity);
        }

        #[semio_framework_async_macros::async_test]
        async fn pack_round_trip_actor_record() {
            let mut kernel = Kernel::new(ShardKind::Thread, 4, 1, 4).await;
            let id = kernel.activate(PackageId("s.cad".into()), 1, ActorKind::PluginApp { plugin: PackageId("s.cad".into()), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
            let record = kernel.actor_record(id).await.unwrap();
            let mut bytes = Vec::new();
            record.pack_encode(&mut bytes).await;
            let mut pos = 0usize;
            let decoded = ActorRecord::pack_decode(&bytes, &mut pos).await.unwrap();
            assert_eq!(pos, bytes.len());
            assert_eq!(decoded, record);
        }

        /// 📌️ The property the old `actor.0 % pool` silently violated: 100 actors of one plugin
        /// must SPREAD across the pool, not all land on shard 0. Every actor `activate` mints has
        /// `generation == 0`, and generation occupied the low bits, so the old modulo returned 0 for
        /// all of them — a bench-measured `perShardCounts {"0": 100}`. Asserts distribution, not
        /// mere validity: a "pin returns a shard in range" test passes when every answer is 0.
        #[semio_framework_async_macros::async_test]
        async fn pin_spreads_actors_of_one_plugin_across_the_pool() {
            let mut table = ShardTable::new(ShardKind::Thread, 8, 0).await;
            let mut counts: std::collections::BTreeMap<u16, usize> = std::collections::BTreeMap::new();
            for ordinal in 0..100u32 {
                *counts.entry(table.pin(ActorId::new(7, 0, ordinal, 0).await).await.0).or_default() += 1;
            }
            assert_eq!(counts.values().sum::<usize>(), 100);
            assert_eq!(counts.len(), 8, "all 8 shards must receive actors, got {counts:?}");
            assert!(*counts.values().max().unwrap() <= 100 / 8 + 1, "no shard may exceed ceil(100/8)+1: {counts:?}");
        }

        /// 🔁️ Pinning the same actor twice is idempotent — a re-pin must not consume a second slot
        /// and skew the balance.
        #[semio_framework_async_macros::async_test]
        async fn pin_is_idempotent_for_the_same_actor() {
            let mut table = ShardTable::new(ShardKind::Thread, 8, 0).await;
            let actor = ActorId::new(3, 0, 11, 0).await;
            assert_eq!(table.pin(actor).await, table.pin(actor).await);
        }

        /// 🕳️ `unpin` leaves a gap; the next `pin` must refill it rather than stride past, which is
        /// exactly what a round-robin counter would do and why placement is least-loaded.
        #[semio_framework_async_macros::async_test]
        async fn pin_refills_the_gap_left_by_unpin() {
            let mut table = ShardTable::new(ShardKind::Thread, 4, 0).await;
            let mut actors: Vec<ActorId> = Vec::new();
            for ordinal in 0..8u32 {
                actors.push(ActorId::new(1, 0, ordinal, 0).await);
            }
            for actor in &actors {
                table.pin(*actor).await;
            }
            let freed = table.shard_of(actors[2]).await.unwrap();
            table.unpin(actors[2]).await;
            assert_eq!(table.pin(ActorId::new(1, 0, 99, 0).await).await, freed);
        }

        /// 🔥️ PROPERTY (terra-interactive-isolation): the mechanism this packet's mission asked for —
        /// N CPU-saturating actors sharing a shard must not receive a freshly-activated interactive
        /// actor as a co-resident. Pure/deterministic: injected `Usage`s stand in for real wall-clock
        /// turns (`ActorMetrics::is_saturating`'s whole point is never needing a clock/bench of its
        /// own), no thread/bench needed.
        #[semio_framework_async_macros::async_test]
        async fn interactive_actor_avoids_a_shard_saturated_by_cpu_bound_actors() {
            let mut kernel = Kernel::new(ShardKind::Thread, 3, 0, 64).await;
            let background_package = PackageId("cpu-hog".into());
            // 🔢️ 6 Background actors round-robin 2-per-shard across 3 shards under plain least-loaded
            // `pin` (proven deterministic by `pin_spreads_actors_of_one_plugin_across_the_pool`'s own
            // reasoning) — read each one's ACTUAL shard back via `actor_record` rather than assuming
            // the exact order, so this test stays correct even if that internal tie-break ever changes.
            let mut by_shard: std::collections::BTreeMap<u16, Vec<ActorId>> = std::collections::BTreeMap::new();
            for ordinal in 0..6u32 {
                let id = kernel.activate(background_package.clone(), 1, ActorKind::PluginApp { plugin: background_package.clone(), app_id: "hog".into(), instance_id: ordinal }, Lane::Background, None, ActivationEvent::Manual).await;
                let shard = kernel.actor_record(id).await.unwrap().shard;
                by_shard.entry(shard.0).or_default().push(id);
            }
            assert_eq!(by_shard.len(), 3, "expected all 3 shards to receive background actors: {by_shard:?}");

            // 🔥️ Drive every actor on shards OTHER than the last one over its own Background budget's
            // wall_ms ceiling for 2 turns (SATURATION_MIN_TURNS) — crossing `is_saturating`'s
            // threshold. The last shard's actors are left untouched (default `ActorMetrics`, zero
            // turns), making it the one and only "clean" shard.
            let shard_ids: Vec<u16> = by_shard.keys().copied().collect();
            let (safe_shard, hot_shards) = shard_ids.split_last().unwrap();
            let hot_turn = TurnResult { ui_patches: vec![], effects: vec![], next_wake: None, status: TurnStatus::Idle, usage: Usage { fuel: 100, wall_us: 40_000, memory_bytes: 1024 } };
            for shard in hot_shards {
                for actor in &by_shard[shard] {
                    kernel.complete(*actor, &hot_turn, 0).await.unwrap();
                    kernel.complete(*actor, &hot_turn, 0).await.unwrap();
                }
            }

            let interactive_id = kernel.activate(PackageId("editor".into()), 2, ActorKind::PluginApp { plugin: PackageId("editor".into()), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
            let interactive_shard = kernel.actor_record(interactive_id).await.unwrap().shard;
            assert_eq!(interactive_shard.0, *safe_shard, "interactive actor must land on the one shard with no CPU-saturating co-resident, got {interactive_shard:?} (hot shards: {hot_shards:?})");
        }

        #[semio_framework_async_macros::async_test]
        async fn pack_round_trip_shard_table() {
            let mut table = ShardTable::new(ShardKind::Thread, 4, 1).await;
            let actor = ActorId::new(1, 0, 0, 0).await;
            table.pin(actor).await;
            table.request_exclusive(ActorId::new(2, 0, 0, 0).await).await;
            let mut bytes = Vec::new();
            table.pack_encode(&mut bytes).await;
            let mut pos = 0usize;
            let decoded = ShardTable::pack_decode(&bytes, &mut pos).await.unwrap();
            assert_eq!(pos, bytes.len());
            assert_eq!(decoded.shard_of(actor).await, table.shard_of(actor).await);
        }

        #[semio_framework_async_macros::async_test]
        async fn pack_round_trip_actor_metrics() {
            let mut metrics = ActorMetrics::default();
            for i in 0..70u64 {
                metrics.record_turn(&Usage { fuel: i, wall_us: i * 3, memory_bytes: 10 }).await;
            }
            let mut bytes = Vec::new();
            metrics.pack_encode(&mut bytes).await;
            let mut pos = 0usize;
            let decoded = ActorMetrics::pack_decode(&bytes, &mut pos).await.unwrap();
            assert_eq!(pos, bytes.len());
            assert_eq!(decoded, metrics);
            assert_eq!(decoded.wall_us_p95(), metrics.wall_us_p95());
        }
        //#endregion 🔖️PackRoundTrips

        //#region 🔖️SerdeRoundTrips
        /// 🎯️ terra-shard-grants, Part A. `📌️important.md` rule 12: "after fixing one variant of a
        /// serde-shape defect, sweep every sibling" — the `JobStep::Done`/`Failed` fix recorded this
        /// instruction and nobody executed it for the six siblings sitting in this crate. Each of
        /// these SERIALIZES TO BYTES AND BACK (`serde_json`, not an in-process `assert_eq!` on the
        /// original value) — that distinction is the entire point: a plain equality check on the
        /// original Rust value would pass even if `serde_json::to_vec` panics or errors partway,
        /// exactly how the `JobStep` bug hid for a full wave (its tests only ever asserted on
        /// in-process values, never on bytes that had crossed a serde boundary).
        macro_rules! serde_round_trip {
            ($name:ident, $ty:ty, $value:expr) => {
                #[semio_framework_async_macros::async_test]
                async fn $name() {
                    let value: $ty = $value;
                    let bytes = serde_json::to_vec(&value).expect("serde_json::to_vec must not error — this is the exact defect this test exists to catch");
                    let decoded: $ty = serde_json::from_slice(&bytes).expect("serde_json::from_slice must round-trip what to_vec produced");
                    assert_eq!(decoded, value);
                }
            };
        }

        serde_round_trip!(serde_round_trip_payload_event, Payload, Payload::Event { bytes: vec![1, 2, 3] });
        serde_round_trip!(serde_round_trip_payload_cancel, Payload, Payload::Cancel { seq: 42 });
        serde_round_trip!(serde_round_trip_origin_actor, Origin, Origin::Actor { id: ActorId::new(1, 0, 2, 0).await });
        serde_round_trip!(serde_round_trip_turn_status_faulted, TurnStatus, TurnStatus::Faulted { detail: b"boom".to_vec() });
        serde_round_trip!(serde_round_trip_failure_signal_trap, FailureSignal, FailureSignal::Trap { detail: "trapped".to_string() });
        serde_round_trip!(serde_round_trip_backpressure_dropped, Backpressure, Backpressure::Dropped { lane: Lane::Background });
        //#endregion 🔖️SerdeRoundTrips

        //#region 🔖️ActorIdBitPacking
        #[semio_framework_async_macros::async_test]
        async fn actor_id_bit_packing_round_trips_all_fields() {
            let id = ActorId::new(0xBEEF, 2, 0xC0FFEE, 0x1234 & 0x3FFF).await;
            assert_eq!(id.plugin_ordinal(), 0xBEEF);
            assert_eq!(id.kind_tag(), 2);
            assert_eq!(id.ordinal(), 0xC0FFEE);
            assert_eq!(id.generation(), 0x1234 & 0x3FFF);
        }

        #[semio_framework_async_macros::async_test]
        async fn actor_id_next_generation_bumps_only_generation() {
            let id = ActorId::new(3, 1, 9, 5).await;
            let restarted = id.next_generation().await;
            assert_eq!(restarted.plugin_ordinal(), id.plugin_ordinal());
            assert_eq!(restarted.kind_tag(), id.kind_tag());
            assert_eq!(restarted.ordinal(), id.ordinal());
            assert_eq!(restarted.generation(), id.generation() + 1);
        }
        //#endregion 🔖️ActorIdBitPacking

        //#region 🔖️MailboxTests
        #[semio_framework_async_macros::async_test]
        async fn mailbox_coalesces_latest_wins_older_dropped() {
            let mut mailbox = Mailbox::new(10).await;
            let actor = ActorId::new(1, 0, 0, 0).await;
            for i in 0..200u64 {
                let mut e = env(actor, Lane::Interactive, i).await;
                e.coalesce = Some(CoalesceKey("pointer-move".into()));
                e.payload = Payload::Event { bytes: vec![i as u8] };
                let bp = mailbox.enqueue(e).await;
                assert!(matches!(bp, Backpressure::Accept | Backpressure::Coalesced));
            }
            assert_eq!(mailbox.len(), 1, "200 coalesced moves must never queue more than the latest");
            let popped = mailbox.pop_next().await.unwrap();
            assert_eq!(popped.payload, Payload::Event { bytes: vec![199] });
        }

        #[semio_framework_async_macros::async_test]
        async fn mailbox_backpressure_rejected_when_full_and_nothing_lower_priority() {
            let mut mailbox = Mailbox::new(2).await;
            let actor = ActorId::new(1, 0, 0, 0).await;
            assert_eq!(mailbox.enqueue(env(actor, Lane::Maintenance, 1).await).await, Backpressure::Accept);
            assert_eq!(mailbox.enqueue(env(actor, Lane::Maintenance, 2).await).await, Backpressure::Accept);
            let bp = mailbox.enqueue(env(actor, Lane::Maintenance, 3).await).await;
            assert_eq!(bp, Backpressure::Rejected, "no lower-priority lane to evict — must reject, never silently drop");
        }

        #[semio_framework_async_macros::async_test]
        async fn mailbox_backpressure_drops_lower_priority_lane_to_admit_interactive() {
            let mut mailbox = Mailbox::new(2).await;
            let actor = ActorId::new(1, 0, 0, 0).await;
            assert_eq!(mailbox.enqueue(env(actor, Lane::Maintenance, 1).await).await, Backpressure::Accept);
            assert_eq!(mailbox.enqueue(env(actor, Lane::Background, 2).await).await, Backpressure::Accept);
            let bp = mailbox.enqueue(env(actor, Lane::Interactive, 3).await).await;
            assert_eq!(bp, Backpressure::Dropped { lane: Lane::Maintenance });
            assert_eq!(mailbox.len(), 2);
        }

        #[semio_framework_async_macros::async_test]
        async fn mailbox_pop_next_honors_lane_priority_over_fifo() {
            let mut mailbox = Mailbox::new(10).await;
            let actor = ActorId::new(1, 0, 0, 0).await;
            mailbox.enqueue(env(actor, Lane::Maintenance, 1).await).await;
            mailbox.enqueue(env(actor, Lane::Background, 2).await).await;
            mailbox.enqueue(env(actor, Lane::Interactive, 3).await).await;
            assert_eq!(mailbox.pop_next().await.unwrap().lane, Lane::Interactive);
            assert_eq!(mailbox.pop_next().await.unwrap().lane, Lane::Background);
            assert_eq!(mailbox.pop_next().await.unwrap().lane, Lane::Maintenance);
        }
        //#endregion 🔖️MailboxTests

        //#region 🔖️SchedulerFairness
        #[semio_framework_async_macros::async_test]
        async fn drr_fairness_plugin_with_50_actors_does_not_starve_plugin_with_1() {
            // 🧪️ Both plugins are given abundant backlog up front (nobody runs dry mid-test), so the
            // measured split reflects level-1 DRR's PLUGIN-level fairness, not which plugin happened
            // to have offered work left when the other ran out. Without level 1, `s.busy` (50 actors)
            // would swamp `s.quiet` (1 actor) roughly 50:1 — see design §Scheduler.
            let mut scheduler = Scheduler::new(4).await;
            let busy_package = PackageId("s.busy".into());
            let quiet_package = PackageId("s.quiet".into());
            let budget = lane_defaults::budget_for(Lane::Background).await;
            let mut busy_actors = Vec::new();
            for i in 0..50u32 {
                let id = ActorId::new(1, 0, i, 0).await;
                scheduler.register_actor(id, busy_package.clone(), Lane::Background, budget, ShardId(0)).await;
                busy_actors.push(id);
            }
            let quiet_actor = ActorId::new(2, 0, 0, 0).await;
            scheduler.register_actor(quiet_actor, quiet_package, Lane::Background, budget, ShardId(1)).await;

            for &id in &busy_actors {
                for seq in 0..30u64 {
                    scheduler.submit(Envelope { to: id, from: Origin::Kernel, lane: Lane::Background, seq, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: vec![] } }).await;
                }
            }
            for seq in 0..500u64 {
                scheduler.submit(Envelope { to: quiet_actor, from: Origin::Kernel, lane: Lane::Background, seq, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: vec![] } }).await;
            }

            let mut busy_grants = 0u32;
            let mut quiet_grants = 0u32;
            for now in 0..100u64 {
                let decision = scheduler.tick(now).await;
                for grant in &decision.run {
                    if grant.actor == quiet_actor {
                        quiet_grants += 1;
                    } else {
                        busy_grants += 1;
                    }
                }
            }
            assert!(quiet_grants > 0, "the 1-actor plugin must never starve");
            assert!(busy_grants > 0);
            let ratio = busy_grants as f64 / quiet_grants as f64;
            assert!(ratio < 10.0, "level-1 DRR must keep PLUGIN-level share roughly comparable regardless of actor count (busy={busy_grants} quiet={quiet_grants} ratio={ratio}, naive per-actor scheduling would give ~50x)");
        }

        #[semio_framework_async_macros::async_test]
        async fn deadline_preemption_runs_before_background_drr_deficit() {
            let mut scheduler = Scheduler::new(1).await;
            let package = PackageId("s.mixed".into());
            let budget = lane_defaults::budget_for(Lane::Background).await;
            let bg_actor = ActorId::new(1, 0, 0, 0).await;
            let interactive_actor = ActorId::new(1, 0, 1, 0).await;
            scheduler.register_actor(bg_actor, package.clone(), Lane::Background, budget, ShardId(0)).await;
            scheduler.register_actor(interactive_actor, package, Lane::Interactive, lane_defaults::budget_for(Lane::Interactive).await, ShardId(0)).await;
            scheduler.submit(Envelope { to: bg_actor, from: Origin::Kernel, lane: Lane::Background, seq: 1, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: vec![] } }).await;
            scheduler.submit(Envelope { to: interactive_actor, from: Origin::Kernel, lane: Lane::Interactive, seq: 2, deadline_ms: Some(5), coalesce: None, cancel_of: None, payload: Payload::Event { bytes: vec![] } }).await;
            let decision = scheduler.tick(10).await;
            assert_eq!(decision.run.len(), 1);
            assert_eq!(decision.run[0].actor, interactive_actor, "an overdue interactive deadline must preempt DRR ordering");
        }
        //#endregion 🔖️SchedulerFairness

        //#region 🔖️FailureLadder
        #[semio_framework_async_macros::async_test]
        async fn failure_ladder_escalates_and_decays_back_to_healthy() {
            let mut state = FailureState::new();
            assert_eq!(state.stage, FailureStage::Healthy);

            let esc = state.on_signal(&FailureSignal::DeadlineOverrun { ratio: 1.2 }, Lane::Interactive, 0).await;
            assert_eq!(esc, FailureEscalation::None);
            assert_eq!(state.stage, FailureStage::Warned);

            let esc = state.on_signal(&FailureSignal::DeadlineOverrun { ratio: 1.4 }, Lane::Interactive, 10).await;
            assert_eq!(esc, FailureEscalation::None);
            assert!(matches!(state.stage, FailureStage::Throttled { .. }), "second interactive warn must throttle (threshold=2)");

            let esc = state.on_signal(&FailureSignal::MailboxOverflow, Lane::Interactive, 20).await;
            assert_eq!(esc, FailureEscalation::None);
            assert!(matches!(state.stage, FailureStage::Suspended { .. }), "third interactive warn must suspend (threshold=2 -> suspend_at)");

            for i in 0..200u64 {
                state.on_clean_turn(1_000_000 + i).await;
            }
            assert_eq!(state.stage, FailureStage::Healthy, "sustained clean turns must decay all the way back to Healthy, got {:?}", state.stage);
        }

        #[semio_framework_async_macros::async_test]
        async fn failure_ladder_trap_then_quarantine_is_package_wide() {
            let mut kernel = Kernel::new(ShardKind::Thread, 4, 1, 4).await;
            let package = PackageId("s.flaky".into());
            let a = kernel.activate(package.clone(), 1, ActorKind::PluginApp { plugin: package.clone(), app_id: "a".into(), instance_id: 0 }, Lane::Background, None, ActivationEvent::Manual).await;
            let b = kernel.activate(package.clone(), 1, ActorKind::PluginApp { plugin: package, app_id: "b".into(), instance_id: 1 }, Lane::Background, None, ActivationEvent::Manual).await;

            for i in 0..FAILURE_QUARANTINE_RESTART_THRESHOLD {
                let faulted = TurnResult { ui_patches: vec![], effects: vec![], next_wake: None, status: TurnStatus::Faulted { detail: b"boom".to_vec() }, usage: Usage::default() };
                kernel.complete(a, &faulted, (i as u64) * 100).await.unwrap();
            }
            assert_eq!(kernel.actor_status(a).await, Some(&ActorStatus::Quarantined));
            assert_eq!(kernel.actor_status(b).await, Some(&ActorStatus::Quarantined), "quarantine must be package-wide, not just the trapping actor");
        }

        #[semio_framework_async_macros::async_test]
        async fn failure_ladder_manual_reset_returns_to_healthy_immediately() {
            let mut state = FailureState::new();
            state.on_signal(&FailureSignal::FuelExhausted, Lane::Background, 0).await;
            assert_ne!(state.stage, FailureStage::Healthy);
            state.on_signal(&FailureSignal::ManualReset, Lane::Background, 1).await;
            assert_eq!(state.stage, FailureStage::Healthy);
            assert_eq!(state.warn_count, 0);
        }
        //#endregion 🔖️FailureLadder

        //#region 🔖️Scene
        #[semio_framework_async_macros::async_test]
        async fn scene_revision_is_monotonic_and_reuses_snapshot_on_empty_commit() {
            let mut store = SceneStore::new();
            let budget = lane_defaults::budget_for(Lane::Interactive).await;
            let actor = ActorId::new(1, 0, 0, 0).await;
            let first = store.commit_frame(0).await;
            assert_eq!(first.revision, 0, "no pending patches yet -> initial empty snapshot, revision 0");

            store.apply_patch(actor, vec![1, 2, 3], 10, &budget).await.unwrap();
            let second = store.commit_frame(16).await;
            assert_eq!(second.revision, 1);
            assert!(second.committed_ms >= first.committed_ms);

            let third = store.commit_frame(32).await;
            assert_eq!(third.revision, second.revision, "nothing pending -> previous snapshot reused, same revision");
            assert!(Arc::ptr_eq(&second, &third));
        }

        #[semio_framework_async_macros::async_test]
        async fn scene_ui_node_quota_truncates_and_signals() {
            let mut store = SceneStore::new();
            let mut budget = lane_defaults::budget_for(Lane::Interactive).await;
            budget.ui_nodes = 100;
            let actor = ActorId::new(1, 0, 0, 0).await;
            let err = store.apply_patch(actor, vec![1], 150, &budget).await.unwrap_err();
            assert_eq!(err, FailureSignal::UiQuota);
            let snapshot = store.commit_frame(0).await;
            assert_eq!(snapshot.node_count, 100, "node count must be truncated to the budget ceiling, never exceed it");
        }

        #[semio_framework_async_macros::async_test]
        async fn scene_max_patch_bytes_rejects_oversized_patch() {
            let mut store = SceneStore::new();
            let mut budget = lane_defaults::budget_for(Lane::Interactive).await;
            budget.max_patch_bytes = 4;
            let actor = ActorId::new(1, 0, 0, 0).await;
            let err = store.apply_patch(actor, vec![0; 100], 1, &budget).await.unwrap_err();
            assert_eq!(err, FailureSignal::UiQuota);
        }
        //#endregion 🔖️Scene

        //#region 🔖️ThreadTransport
        #[semio_framework_async_macros::async_test]
        #[cfg(not(target_arch = "wasm32"))]
        async fn thread_transport_duplex_send_recv_and_heartbeat() {
            let (kernel_side, shard_side) = ThreadTransport::new_pair().await;
            kernel_side.send(b"to-shard").await;
            assert_eq!(shard_side.recv().await, Some(b"to-shard".to_vec()));
            shard_side.send(b"to-kernel").await;
            assert_eq!(kernel_side.recv().await, Some(b"to-kernel".to_vec()));
            shard_side.beat(42).await;
            assert_eq!(kernel_side.heartbeat().await, 42);
        }

        #[semio_framework_async_macros::async_test]
        #[cfg(not(target_arch = "wasm32"))]
        async fn thread_transport_kill_stops_recv() {
            let (kernel_side, shard_side) = ThreadTransport::new_pair().await;
            kernel_side.send(b"queued-before-kill").await;
            kernel_side.kill().await;
            assert_eq!(shard_side.recv().await, None, "a killed transport must never yield a stale message");
        }

        /// 🎯️ terra-shard-grants requirement: `recv_deadline` must return `None` on a genuine
        /// timeout (nothing ever sent) rather than blocking forever — proven with a short, bounded
        /// deadline so the test itself cannot hang. "Spawns no thread" is a property of the
        /// IMPLEMENTATION (`mpsc::Receiver::recv_timeout`, which blocks only the calling thread —
        /// see [`ThreadTransport::recv_deadline`]'s own doc), verified by code review plus the
        /// crate-wide purity grep this ticket already runs (`std::thread` must match only the
        /// header doc comment across the whole file) — deliberately NOT re-proven here via
        /// `std::thread::current()`, which would itself add a real (non-doc-comment) `std::thread`
        /// use to this file and defeat the very grep this test is meant to keep passing.
        #[semio_framework_async_macros::async_test]
        #[cfg(not(target_arch = "wasm32"))]
        async fn recv_deadline_returns_none_on_timeout() {
            let (_kernel_side, shard_side) = ThreadTransport::new_pair().await;
            let result = shard_side.recv_deadline(std::time::Duration::from_millis(20)).await;
            assert_eq!(result, None, "nothing was ever sent — a timeout must yield None, not block forever");
        }

        /// 🎯️ The complementary case: a message sent before the deadline must still be delivered —
        /// `recv_deadline` is a bounded wait, not merely a disguised `recv()` that always returns
        /// `None` until timeout.
        #[semio_framework_async_macros::async_test]
        #[cfg(not(target_arch = "wasm32"))]
        async fn recv_deadline_returns_the_message_when_one_arrives_before_the_timeout() {
            let (kernel_side, shard_side) = ThreadTransport::new_pair().await;
            kernel_side.send(b"before-deadline").await;
            assert_eq!(shard_side.recv_deadline(std::time::Duration::from_millis(200)).await, Some(b"before-deadline".to_vec()));
        }

        /// 🛑️ Mirrors `thread_transport_kill_stops_recv` for the blocking variant — a killed
        /// transport must return `None` immediately, never wait out the full deadline.
        #[semio_framework_async_macros::async_test]
        #[cfg(not(target_arch = "wasm32"))]
        async fn recv_deadline_returns_none_immediately_on_a_killed_transport() {
            let (kernel_side, shard_side) = ThreadTransport::new_pair().await;
            kernel_side.send(b"queued-before-kill").await;
            kernel_side.kill().await;
            assert_eq!(shard_side.recv_deadline(std::time::Duration::from_millis(20)).await, None, "a killed transport must never yield a stale message");
        }
        //#endregion 🔖️ThreadTransport

        //#region 🔖️KernelFacade
        #[semio_framework_async_macros::async_test]
        async fn kernel_activate_submit_tick_complete_round_trip() {
            let mut kernel = Kernel::new(ShardKind::Thread, 4, 1, 4).await;
            let window = WindowId(1);
            let actor = kernel.activate(PackageId("s.cad".into()), 1, ActorKind::PluginApp { plugin: PackageId("s.cad".into()), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, Some(window), ActivationEvent::WindowOpen { window }).await;
            let bp = kernel.submit(&env(actor, Lane::Interactive, 1).await).await;
            assert_eq!(bp, Backpressure::Accept);
            let decision = kernel.tick(0).await;
            assert_eq!(decision.run.len(), 1);
            assert_eq!(decision.run[0].actor, actor);
            let escalation = kernel.complete(actor, &ok_turn().await, 1).await.unwrap();
            assert_eq!(escalation, FailureEscalation::None);
            assert_eq!(kernel.actor_status(actor).await, Some(&ActorStatus::Active));
        }

        #[semio_framework_async_macros::async_test]
        async fn kernel_suspend_resume_round_trip() {
            let mut kernel = Kernel::new(ShardKind::Thread, 4, 1, 4).await;
            let actor = kernel.activate(PackageId("s.cad".into()), 1, ActorKind::Extension { plugin: PackageId("s.cad".into()), extension_id: "e1".into() }, Lane::Background, None, ActivationEvent::Manual).await;
            kernel.suspend(actor, Some(vec![1, 2, 3])).await.unwrap();
            assert_eq!(kernel.actor_status(actor).await, Some(&ActorStatus::Suspended { checkpoint: Some(vec![1, 2, 3]) }));
            kernel.resume(actor).await.unwrap();
            assert_eq!(kernel.actor_status(actor).await, Some(&ActorStatus::Active));
        }

        #[semio_framework_async_macros::async_test]
        async fn kernel_request_exclusive_then_release() {
            let mut kernel = Kernel::new(ShardKind::Thread, 4, 1, 4).await;
            let actor = kernel.activate(PackageId("s.cad".into()), 1, ActorKind::Job { owner: ActorId::new(0, 0, 0, 0).await, job_id: 1 }, Lane::Background, None, ActivationEvent::Manual).await;
            let shard = kernel.request_exclusive(actor).await.unwrap();
            assert!(shard.0 >= 3, "exclusive shards must come from the reserved tail of the pool");
            kernel.release_exclusive(actor).await;
        }

        #[semio_framework_async_macros::async_test]
        async fn kernel_metrics_counts_actors_shards_packages() {
            let mut kernel = Kernel::new(ShardKind::Thread, 4, 0, 4).await;
            kernel.activate(PackageId("s.a".into()), 1, ActorKind::Extension { plugin: PackageId("s.a".into()), extension_id: "e".into() }, Lane::Background, None, ActivationEvent::Manual).await;
            kernel.activate(PackageId("s.b".into()), 2, ActorKind::Extension { plugin: PackageId("s.b".into()), extension_id: "e".into() }, Lane::Background, None, ActivationEvent::Manual).await;
            let metrics = kernel.metrics().await;
            assert_eq!(metrics.actors, 2);
            assert_eq!(metrics.packages, 2);
            assert_eq!(metrics.shards, 4);
        }
        //#endregion 🔖️KernelFacade

        //#region 🔖️RuntimeMetricsSnapshot
        /// 📈️ T1 runtime evidence: drives two real actors (different packages/lanes) through
        /// `activate`/`submit`/`tick`/`complete`, then asserts `runtime_metrics_snapshot`'s rows —
        /// package, lane, status, turns, shard — match what the kernel actually did, not a fake.
        #[semio_framework_async_macros::async_test]
        async fn runtime_metrics_snapshot_reflects_real_kernel_activity() {
            let mut kernel = Kernel::new(ShardKind::Thread, 2, 0, 8).await;
            let cad = kernel.activate(PackageId("s.cad".into()), 1, ActorKind::PluginApp { plugin: PackageId("s.cad".into()), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
            let stdio = kernel.activate(PackageId("s.stdio".into()), 2, ActorKind::Extension { plugin: PackageId("s.stdio".into()), extension_id: "e".into() }, Lane::Background, None, ActivationEvent::Manual).await;

            kernel.submit(&env(cad, Lane::Interactive, 1).await).await;
            let decision = kernel.tick(0).await;
            assert_eq!(decision.run.len(), 1, "only `cad` has a pending envelope this tick");
            kernel.complete(cad, &ok_turn().await, 5).await.unwrap();

            let snapshot = kernel.runtime_metrics_snapshot(5).await;
            assert_eq!(snapshot.sampled_at_ms, 5);
            assert_eq!(snapshot.kernel.actors, 2);
            assert_eq!(snapshot.kernel.packages, 2);
            assert_eq!(snapshot.actors.len(), 2);

            let cad_row = snapshot.actors.iter().find(|row| row.id == cad).expect("cad row present");
            assert_eq!(cad_row.package, PackageId("s.cad".into()));
            assert_eq!(cad_row.lane, Lane::Interactive);
            assert_eq!(cad_row.status, ActorStatus::Active);
            assert_eq!(cad_row.metrics.turns, 1, "the completed turn must be counted");

            let stdio_row = snapshot.actors.iter().find(|row| row.id == stdio).expect("stdio row present");
            assert_eq!(stdio_row.package, PackageId("s.stdio".into()));
            assert_eq!(stdio_row.lane, Lane::Background);
            assert_eq!(stdio_row.metrics.turns, 0, "stdio never got a turn");

            assert!(!snapshot.shards.is_empty(), "at least one shard row for the two pinned actors");
            let total_shard_actors: u32 = snapshot.shards.iter().map(|row| row.metrics.actors).sum();
            assert_eq!(total_shard_actors, 2, "every actor is counted on exactly one shard");
        }

        #[semio_framework_async_macros::async_test]
        async fn runtime_metrics_due_gates_at_the_2hz_interval_and_always_fires_once() {
            assert!(runtime_metrics_due(None, 0).await, "never published yet must always be due");
            assert!(!runtime_metrics_due(Some(1_000), 1_200).await, "200ms since last publish is inside the 500ms window");
            assert!(runtime_metrics_due(Some(1_000), 1_500).await, "exactly the 500ms interval must fire");
            assert!(runtime_metrics_due(Some(1_000), 2_000).await, "well past the interval must fire");
        }
        //#endregion 🔖️RuntimeMetricsSnapshot

        //#region 🔖️ShardTable
        #[semio_framework_async_macros::async_test]
        async fn shard_sizing_policy_clamps_native_and_web() {
            assert_eq!(clamp_native_shard_count(1).await, 2);
            assert_eq!(clamp_native_shard_count(9).await, 8);
            assert_eq!(clamp_native_shard_count(5).await, 4);
            assert_eq!(clamp_web_shard_count(1).await, 1);
            assert_eq!(clamp_web_shard_count(9).await, 4);
        }
        //#endregion 🔖️ShardTable
    }

    //#region 🔖️Typegen
    #[cfg(feature = "typegen")]
    #[semio_framework_async_macros::async_test]
    async fn exports_typescript_bindings() {
        use ts_rs::TS;
        use crate::*;
        PackageId::export().unwrap();
        PackageHash::export().unwrap();
        ActorId::export().unwrap();
        ActorKind::export().unwrap();
        Lane::export().unwrap();
        Budget::export().unwrap();
        WindowId::export().unwrap();
        Origin::export().unwrap();
        Payload::export().unwrap();
        CoalesceKey::export().unwrap();
        Envelope::export().unwrap();
        TurnStatus::export().unwrap();
        Usage::export().unwrap();
        TurnResult::export().unwrap();
        Backpressure::export().unwrap();
        Mailbox::export().unwrap();
        CapabilityGrant::export().unwrap();
        FailureSignal::export().unwrap();
        FailureStage::export().unwrap();
        FailureState::export().unwrap();
        ActorStatus::export().unwrap();
        ActorRecord::export().unwrap();
        ShardKind::export().unwrap();
        ShardId::export().unwrap();
        ShardTable::export().unwrap();
        Decision::export().unwrap();
        TurnGrant::export().unwrap();
        SceneSnapshot::export().unwrap();
        ActorMetrics::export().unwrap();
        ShardMetrics::export().unwrap();
        KernelMetrics::export().unwrap();
        ActivationEvent::export().unwrap();
        ActorMetricsSample::export().unwrap();
        ShardMetricsSample::export().unwrap();
        RuntimeMetricsSnapshot::export().unwrap();
    }
    //#endregion 🔖️Typegen
}
