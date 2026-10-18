//! 🗂️ Actor record and shard table: the actor record with its shard map conversions and the shard table that pins actors to shards. See [`super`].

use super::*;

/// 🗂️ Full snapshot of one actor — assembled on demand by [`Kernel::actor_record`] from the
/// scheduler's live entry plus this actor's kind/capabilities/status/failure/metrics bookkeeping.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
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

/// 🔁️ `ToValue`/`FromValue` for `BTreeMap<ActorId, ShardId>` — the derive's generic `BTreeMap`
/// support is keyed on `String` only, and a bare-integer-newtype key (`ActorId`) needs its own
/// stringified-key `DslValue::Object` shape (mirrors what `serde_json` itself produces for a
/// newtype-struct map key: the inner integer, stringified).
fn actor_shard_map_to_value(map: &BTreeMap<ActorId, ShardId>) -> ::semio_framework_value::DslValue {
    ::semio_framework_value::DslValue::object(map.iter().map(|(id, shard)| (id.0.to_string(), ::semio_framework_value::ToValue::to_value(shard))))
}

fn actor_shard_map_from_value(value: ::semio_framework_value::DslValue) -> Result<BTreeMap<ActorId, ShardId>, ::semio_framework_value::ValueError> {
    let ::semio_framework_value::DslValue::Object(entries) = value else {
        return Err(::semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected object"));
    };
    entries
        .into_iter()
        .map(|(key, value)| {
            let id: u64 = key.parse().map_err(|_| ::semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid ActorId key `{key}`")))?;
            let shard: ShardId = ::semio_framework_value::FromValue::from_value(value).map_err(|error: ::semio_framework_value::ValueError| error.under(&key))?;
            Ok((ActorId(id), shard))
        })
        .collect()
}

/// 🔁️ `ToValue`/`FromValue` for `BTreeMap<ShardId, ActorId>` — same stringified-key shape as
/// [`actor_shard_map_to_value`], keyed the other way round.
fn shard_actor_map_to_value(map: &BTreeMap<ShardId, ActorId>) -> ::semio_framework_value::DslValue {
    ::semio_framework_value::DslValue::object(map.iter().map(|(shard, id)| (shard.0.to_string(), ::semio_framework_value::ToValue::to_value(id))))
}

fn shard_actor_map_from_value(value: ::semio_framework_value::DslValue) -> Result<BTreeMap<ShardId, ActorId>, ::semio_framework_value::ValueError> {
    let ::semio_framework_value::DslValue::Object(entries) = value else {
        return Err(::semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected object"));
    };
    entries
        .into_iter()
        .map(|(key, value)| {
            let id: u16 = key.parse().map_err(|_| ::semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid ShardId key `{key}`")))?;
            let actor: ActorId = ::semio_framework_value::FromValue::from_value(value).map_err(|error: ::semio_framework_value::ValueError| error.under(&key))?;
            Ok((ShardId(id), actor))
        })
        .collect()
}

/// 🧩️ Fixed shard pool. An actor is pinned to a shard; migration only happens at a quiescent point
/// via application-level checkpoint (never a raw linear-memory snapshot). The last `exclusive_reserve`
/// shards are reserved for [`ShardTable::request_exclusive`] leases.
#[derive(Clone, Debug, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct ShardTable {
    pub kind: ShardKind,
    pub(super) shard_count: u16,
    pub(super) exclusive_reserve: u16,
    #[value(serialize_with = "actor_shard_map_to_value", deserialize_with = "actor_shard_map_from_value")]
    pub(super) assignment: BTreeMap<ActorId, ShardId>,
    #[value(serialize_with = "shard_actor_map_to_value", deserialize_with = "shard_actor_map_from_value")]
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

    /// 📌️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): pins `actor` to an
    /// EXACT shard, bypassing `pin`/`pin_avoiding`'s least-loaded heuristic entirely. The extension-
    /// activation primitive: an extension actor pins to its PARENT's shard so parent↔extension
    /// `MessageEndpoint::Extension` traffic never crosses a transport (`Kernel::activate_pinned`'s own
    /// doc). Idempotent for an already-pinned actor, matching `pin`/`pin_avoiding`. `shard` is clamped
    /// into `[0, shard_count)` defensively — every real caller derives it from another actor's own
    /// live assignment, so an out-of-range value would only ever come from a caller bug, and clamping
    /// keeps that bug a misplacement rather than a silently-dropped actor.
    pub async fn pin_to(&mut self, actor: ActorId, shard: ShardId) -> ShardId {
        if let Some(existing) = self.assignment.get(&actor) {
            return *existing;
        }
        let shard = ShardId(shard.0.min(self.shard_count.saturating_sub(1)));
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
