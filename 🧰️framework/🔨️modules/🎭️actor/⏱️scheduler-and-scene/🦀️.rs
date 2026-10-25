//! 🎬️ Scheduler and scene: the scheduled actor, the decision and turn grant records, the deficit round robin scheduler and the scene snapshot and store. See [`super`].

use super::*;

//#region ⏱️Scheduler
/// 📋️ One actor's scheduling entry inside [`Scheduler`]: everything the DRR algorithm needs that
/// isn't shared plugin-level bookkeeping.
#[derive(Clone, Debug)]
pub(super) struct ScheduledActor {
    pub(super) retained: RetainedTurnInput,
    pub(super) loan:OriginalActorTurnLoan,
    pub(super) actual_received_work:u128,
    pub(super) issued: Option<RetainedTurnInput>,
    pub(super) returned: bool,
    pub(super) actual_received_spent:semio_framework_value::RetainedCloneProgress,
    pub(super) refused_return:Option<retained_turn::OriginalActorGuestReturn>,
    pub(super) continuation:bool,
    pub(super) package: PackageId,
    pub(super) lane: Lane,
    pub(super) budget: Budget,
    pub(super) shard: ShardId,
    pub(super) mailbox: Mailbox,
    pub(super) active: bool,
    pub(super) throttle: f32,
    deficit: i64,
}

impl ScheduledActor {
    fn can_issue(&self) -> bool {
        self.active && self.issued.is_none() && self.refused_return.is_none() && (self.continuation||!self.mailbox.is_empty()) && self.retained.validate().is_ok() && self.loan.validate_for(self.retained).is_ok() && self.loan.retained_policy.maximum_items!=0 && self.loan.work_fuel!=0 && self.retained.grant.maximum_items != 0 && (!self.returned || self.retained.epoch.checked_add(1).is_some())
    }
}

/// ✔️ Result of one [`Scheduler::tick`] call: the turns granted this call, and (if nothing ran) the
/// earliest future timestamp worth ticking again for.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct Decision {
    pub run: Vec<TurnGrant>,
    pub wake_at: Option<u64>,
}

impl Decision {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) -> Result<(), pack::PackError> {
        for grant in &self.run { grant.retained.validate().map_err(|_| pack::PackError::InvalidRetainedTurn("operation identity is absent"))?; }
        pack::write_varint_u64(out, self.run.len() as u64).await;
        for grant in &self.run { grant.pack_encode(out).await?; }
        pack::write_opt_u64(out, &self.wake_at).await;
        Ok(())
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self { run: pack::read_vec(bytes, pos, "Decision::run", TurnGrant::pack_decode).await?, wake_at: pack::read_opt_u64(bytes, pos, "Decision::wake_at").await? })
    }
}

/// 🎫️ One granted turn: which actor, on which shard, with what (possibly throttle-scaled) budget,
/// and the envelopes drained from its mailbox for this turn.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct TurnGrant {
    pub actor: ActorId,
    pub shard: ShardId,
    pub budget: Budget,
    pub retained: RetainedTurnInput,
    pub loan:OriginalActorTurnLoan,
    pub envelopes: Vec<Envelope>,
}

impl TurnGrant {
    /// 🎟️ Borrows the scheduler's exact original input without changing its wallet or epoch.
    pub fn original_input(&self) -> &RetainedTurnInput { &self.retained }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) -> Result<(), pack::PackError> {
        self.retained.validate().map_err(|_| pack::PackError::InvalidRetainedTurn("operation identity is absent"))?;
        self.actor.pack_encode(out).await;
        self.shard.pack_encode(out).await;
        self.budget.pack_encode(out).await?;
        self.retained.pack_encode(out).await?;self.loan.pack_encode(out).await;
        pack::write_vec(out, &self.envelopes, Envelope::pack_encode).await;
        Ok(())
    }
    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, pack::PackError> {
        Ok(Self {
            actor: ActorId::pack_decode(bytes, pos).await?,
            shard: ShardId::pack_decode(bytes, pos).await?,
            budget: Budget::pack_decode(bytes, pos).await?,
            retained: RetainedTurnInput::pack_decode(bytes, pos).await?,
            loan:OriginalActorTurnLoan::pack_decode(bytes,pos).await?,
            envelopes: pack::read_vec(bytes, pos, "TurnGrant::envelopes", Envelope::pack_decode).await?,
        })
    }
}

/// 🔂️ How many envelopes a single granted turn drains from its actor's mailbox in one go.

/// ⚖️ Hierarchical deficit round-robin: level 1 fairness across plugins (packages), level 2 across
/// each plugin's own actors. Without level 1, a plugin with 50 actors would take 50x the share of a
/// plugin with one. Owns every actor's mailbox — `submit`/`tick`/`complete` are its only entry points.
#[derive(Clone, Debug, Default)]
pub struct Scheduler {
    pub(super) actors: BTreeMap<ActorId, ScheduledActor>,
    plugin_deficit: HashMap<PackageId, i64>,
    plugin_order: Vec<PackageId>,
    plugin_cursor: usize,
    pub(super) grants_per_tick: u32,
}

impl Scheduler {
    pub async fn new(grants_per_tick: u32) -> Self {
        Self { actors: BTreeMap::new(), plugin_deficit: HashMap::new(), plugin_order: Vec::new(), plugin_cursor: 0, grants_per_tick: grants_per_tick.max(1) }
    }

    pub async fn register_actor(&mut self, actor: ActorId, package: PackageId, lane: Lane, budget: Budget, shard: ShardId, retained: RetainedTurnInput,loan:OriginalActorTurnLoan) {
        self.register_actor_state(actor, package, lane, budget, shard, true, retained,loan).await;
    }

    pub(super) async fn register_actor_state(&mut self, actor: ActorId, package: PackageId, lane: Lane, budget: Budget, shard: ShardId, active: bool, retained: RetainedTurnInput,loan:OriginalActorTurnLoan) {
        if !self.plugin_order.contains(&package) {
            self.plugin_order.push(package.clone());
        }
        self.actors.insert(actor, ScheduledActor { retained,loan,actual_received_work:0, issued: None, returned: false, actual_received_spent:Default::default(),refused_return:None,continuation:false,package, lane, budget, shard, mailbox: Mailbox::new(budget.mailbox_len).await, active, throttle: 1.0, deficit: 0 });
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
    pub fn lane_of(&self, actor: ActorId) -> Option<Lane> {
        self.actors.get(&actor).map(|e| e.lane)
    }

    pub async fn submit(&mut self, envelope: Envelope) -> Backpressure {
        match self.actors.get_mut(&envelope.to) {
            Some(entry) => entry.mailbox.enqueue(envelope).await,
            None => Backpressure::Rejected,
        }
    }

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
        let mut overdue: Vec<(u64, ActorId)> = self.actors.iter().filter(|(_, e)| e.can_issue()).filter_map(|(id, e)| e.mailbox.earliest_deadline().filter(|d| *d <= now_ms).map(|d| (d, *id))).collect();
        overdue.sort();
        for (_, actor_id) in overdue {
            if budget_left == 0 {
                break;
            }
            if granted_this_tick.contains(&actor_id) {
                continue;
            }
            if let Some(grant) = self.drain_original_call(actor_id).await {
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
                let pending_actor_ids: Vec<ActorId> = self.actors.iter().filter(|(id, e)| e.package == package && e.can_issue() && !granted_this_tick.contains(*id)).map(|(id, _)| *id).collect();
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
                    let mut candidates: Vec<ActorId> = self.actors.iter().filter(|(id, e)| e.package == package && e.can_issue() && !granted_this_tick.contains(*id)).map(|(id, _)| *id).collect();
                    candidates.sort();
                    let Some(actor_id) = self.pick_level2(&candidates).await else { break };
                    let weight = self.actors.get(&actor_id).map_or(1, Self::actor_weight);
                    if let Some(grant) = self.drain_original_call(actor_id).await {
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

        let wake_at = self.actors.values().filter(|e| e.can_issue()).filter_map(|e| e.mailbox.earliest_deadline()).min();
        Decision { run, wake_at }
    }

    /// 🪜️ Level-2 DRR: rotate a per-actor deficit within the candidate set, weighted by lane (and
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

    async fn drain_original_call(&mut self, actor_id: ActorId) -> Option<TurnGrant> {
        let entry = self.actors.get_mut(&actor_id)?;
        if entry.issued.is_some() || entry.refused_return.is_some() || (!entry.continuation&&entry.mailbox.is_empty()) || entry.retained.validate().is_err() || entry.loan.validate_for(entry.retained).is_err() || entry.loan.retained_policy.maximum_items==0 || entry.loan.work_fuel==0 || entry.retained.grant.maximum_items == 0 { return None; }
        let epoch = if entry.returned { entry.retained.epoch.checked_add(1)? } else { entry.retained.epoch };
        let retained = RetainedTurnInput { epoch, ..entry.retained };
        let mut envelopes = Vec::new();
        if entry.continuation{envelopes.push(Envelope{to:actor_id,from:Origin::Kernel,lane:entry.lane,seq:retained.epoch,deadline_ms:None,coalesce:None,cancel_of:None,payload:Payload::ContinueOriginalCall})}else if let Some(original)=entry.mailbox.pop_next().await{envelopes.push(original)}
        if envelopes.is_empty() {
            return None;
        }
        entry.retained = retained;
        entry.issued = Some(retained);
        entry.deficit -= Self::actor_weight(entry);
        Some(TurnGrant { actor: actor_id, shard: entry.shard, budget: entry.budget.scaled(entry.throttle).await, retained, loan:entry.loan,envelopes })
    }
}
//#endregion ⏱️Scheduler

//#region 🖼️Scene
/// 📸️ One committed, immutable frame of a window's UI. `patches` is an opaque pack-encoded
/// `Vec<UiPatch>` blob — see the module seam docstring; `node_count` is the host-tracked total used
/// only for [`Budget::ui_nodes`] quota accounting.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
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
        Ok(Self {
            revision: pack::read_u64(bytes, pos, "SceneSnapshot::revision").await?,
            committed_ms: pack::read_u64(bytes, pos, "SceneSnapshot::committed_ms").await?,
            patches: pack::read_bytes(bytes, pos, "SceneSnapshot::patches").await?,
            node_count: pack::read_u32(bytes, pos, "SceneSnapshot::node_count").await?,
        })
    }
}

/// 🗃️ Per-window scene store: a builder accumulates patches between frame boundaries;
/// [`SceneStore::commit_frame`] publishes a new immutable snapshot once per boundary. If nothing
/// was pending, the previous snapshot is reused verbatim (same `Arc`, same revision) — the UI
/// thread never waits on a plugin that missed the deadline.
#[derive(Clone, Debug)]
pub struct SceneStore {
    pub(super) current: Arc<SceneSnapshot>,
    pending: Vec<(ActorId, Vec<u8>, u32)>,
    pending_node_delta: u32,
}

impl Default for SceneStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneStore {
    pub fn new() -> Self {
        Self { current: Arc::new(SceneSnapshot::default()), pending: Vec::new(), pending_node_delta: 0 }
    }

    pub async fn current(&self) -> Arc<SceneSnapshot> {
        self.current.clone()
    }

    /// 🩹️ Stages one actor's patch for the next commit, enforcing `budget.max_patch_bytes` and
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
