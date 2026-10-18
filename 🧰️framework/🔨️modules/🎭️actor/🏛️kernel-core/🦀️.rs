//! 🏛️ Kernel core: the kernel constructor, activation, submission, turn completion, suspension, exclusivity, scene, metrics and snapshot operations. See [`super`].

use super::*;

//#region 🏛️Kernel
impl Kernel {
    pub async fn new(shard_kind: ShardKind, shard_count: u16, exclusive_reserve: u16, grants_per_tick: u32) -> Self {
        Self { scheduler: Scheduler::new(grants_per_tick).await, shards: ShardTable::new(shard_kind, shard_count, exclusive_reserve).await, scenes: HashMap::new(), actors: HashMap::new(), next_ordinal: HashMap::new(), links: HashMap::new() }
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
    pub async fn activate(&mut self, package: PackageId, plugin_ordinal: u16, kind: ActorKind, lane: Lane, window: Option<WindowId>, _event: ActivationEvent, retained: RetainedTurnInput,loan:OriginalActorTurnLoan) -> ActorId {
        let ordinal = self.next_ordinal.entry(package.clone()).or_insert(0);
        let id = ActorId::new(plugin_ordinal, kind.tag().await, *ordinal, 0).await;
        *ordinal += 1;
        let budget = lane_defaults::budget_for(lane);
        let shard = if lane == Lane::Interactive {
            let avoid = self.saturated_shards().await;
            self.shards.pin_avoiding(id, &avoid).await
        } else {
            self.shards.pin(id).await
        };
        self.scheduler.register_actor(id, package.clone(), lane, budget, shard, retained,loan).await;
        self.actors.insert(id, ActorMeta { reservation: None, transport_key: None, kind, package, capabilities: Vec::new(), budget, status: ActorStatus::Activating, failure: FailureState::new(), metrics: ActorMetrics::default(), window });
        id
    }

    /// 🔥️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-interactive-isolation): every shard
    /// currently hosting at least one actor [`ActorMetrics::is_saturating`] judges CPU-saturating —
    /// purely from this kernel's own tracked state (turns already completed via [`Self::complete`]),
    /// never a fixture/profile name. `activate`'s own doc explains why only `Lane::Interactive`
    /// consults this.
    pub(super) async fn saturated_shards(&self) -> BTreeSet<ShardId> {
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

    /// 📮️ Borrows the scheduler's current original mailbox count without constructing a snapshot.
    pub fn mailbox_len(&self, actor: ActorId) -> Option<u16> {
        self.scheduler.actors.get(&actor).map(|entry| entry.mailbox.len())
    }

    /// 🎟️ Borrows the original remaining input independently of scheduling ceilings.
    pub fn retained_turn(&self, actor: ActorId) -> Option<RetainedTurnInput> {
        self.scheduler.actors.get(&actor).map(|entry| entry.retained)
    }

    pub fn retained_turn_loan(&self,actor:ActorId)->Option<OriginalActorTurnLoan>{self.scheduler.actors.get(&actor).map(|entry|entry.loan)}
    pub fn original_call_work_received(&self,actor:ActorId)->Option<u128>{self.scheduler.actors.get(&actor).map(|entry|entry.actual_received_work)}

    /// 📥️ Adopts one actual raw export receipt before another input or semantic result is admitted.
    pub fn receive_original_call(&mut self,actor:ActorId,returned:retained_turn::OriginalActorGuestReturn,continuation:bool)->Result<RetainedTurnReceipt,KernelError>{
        let entry=self.scheduler.actors.get_mut(&actor).ok_or(KernelError::UnknownActor)?;
        if entry.refused_return.is_some(){return Err(KernelError::InvalidRetainedReceipt)}
        let issued=entry.issued.ok_or(KernelError::TurnNotIssued)?;
        match entry.actual_received_spent.checked_add(returned.spent){Ok(spent)=>entry.actual_received_spent=spent,Err(_)=>{entry.refused_return=Some(returned);return Err(KernelError::InvalidRetainedReceipt)}}
        entry.actual_received_work=match entry.actual_received_work.checked_add(u128::from(returned.work_units_spent)){Some(work)=>work,None=>{entry.refused_return=Some(returned);return Err(KernelError::InvalidRetainedReceipt)}};
        let receipt=match issued.return_original(returned.spent,returned.work_units_spent){Ok(receipt)if returned.input==issued&&returned.claimed_remaining.is_none_or(|remaining|remaining==receipt.remaining)&&returned.claimed_remaining_fuel.is_none_or(|remaining|remaining==receipt.remaining_fuel)=>receipt,_=>{entry.refused_return=Some(returned);return Err(KernelError::InvalidRetainedReceipt)}};
        let loan=match entry.loan.remaining(returned.spent,returned.work_units_spent){Ok(loan)=>loan,Err(_)=>{entry.refused_return=Some(returned);return Err(KernelError::InvalidRetainedReceipt)}};
        entry.loan=loan;entry.retained=RetainedTurnInput{grant:receipt.remaining,work:OriginalActorTurnWork{fuel:receipt.remaining_fuel,..issued.work},..issued};entry.issued=None;entry.returned=true;entry.continuation=continuation;Ok(receipt)
    }

    /// 🧾️ Borrows actual raw spending and any refused owner before a scheduler continuation.
    pub fn original_call_receiving(&self,actor:ActorId)->Option<(semio_framework_value::RetainedCloneProgress,Option<&retained_turn::OriginalActorGuestReturn>)>{self.scheduler.actors.get(&actor).map(|entry|(entry.actual_received_spent,entry.refused_return.as_ref()))}

    /// ✅️ Records a turn's result against its actor: usage metrics, failure-ladder update (clean
    /// turn vs. `Faulted`), and status transition. Returns the escalation the caller (host) must
    /// act on for `Trapped`/`Quarantined` outcomes.
    pub async fn complete(&mut self, actor: ActorId, result: &TurnResult, now_ms: u64) -> Result<FailureEscalation, KernelError> {
        self.receive_original_call(actor,retained_turn::OriginalActorGuestReturn{input:result.retained_receipt.input,spent:result.retained_receipt.spent,work_units_spent:result.retained_receipt.work_units_spent,claimed_remaining:Some(result.retained_receipt.remaining),claimed_remaining_fuel:Some(result.retained_receipt.remaining_fuel)},matches!(result.status,TurnStatus::MoreWork))?;
        let meta = self.actors.get_mut(&actor).ok_or(KernelError::UnknownActor)?;
        if meta.reservation.is_some() {
            return Err(KernelError::InvalidTransition);
        }
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
    pub(super) async fn quarantine_package(&mut self, package: &PackageId, now_ms: u64) {
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
        if meta.reservation.is_some() {
            return Err(KernelError::InvalidTransition);
        }
        meta.status = ActorStatus::Suspended { checkpoint };
        self.scheduler.set_active(actor, false).await;
        Ok(())
    }

    pub async fn resume(&mut self, actor: ActorId) -> Result<(), KernelError> {
        let meta = self.actors.get_mut(&actor).ok_or(KernelError::UnknownActor)?;
        if meta.reservation.is_some() {
            return Err(KernelError::InvalidTransition);
        }
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
        if self.actors.get(&actor).is_some_and(|meta| meta.reservation.is_some() || meta.transport_key.is_some()) {
            return Err(KernelError::InvalidTransition);
        }
        let shard = self.shards.request_exclusive(actor).await.ok_or(KernelError::NoExclusiveShard)?;
        self.scheduler.set_shard(actor, shard).await;
        Ok(shard)
    }

    pub async fn release_exclusive(&mut self, actor: ActorId) {
        if self.actors.get(&actor).is_some_and(|meta| meta.reservation.is_some() || meta.transport_key.is_some()) {
            return;
        }
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
        Some(ActorRecord {
            id: actor,
            kind: meta.kind.clone(),
            package: meta.package.clone(),
            shard,
            capabilities: meta.capabilities.clone(),
            budget: meta.budget,
            mailbox,
            status: meta.status.clone(),
            failure: meta.failure.clone(),
            metrics: meta.metrics.clone(),
        })
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
        per_shard.into_iter().map(|(shard, (active, total))| ShardMetricsSample { shard, metrics: ShardMetrics { actors: total, busy_ratio: if total > 0 { active as f32 / total as f32 } else { 0.0 }, heartbeat_age_ms: 0 } }).collect()
    }

    /// 📈️ Assembles the full `os.runtime.metrics` payload — [`Kernel::metrics`] plus every actor's and
    /// shard's sample — for a host to pack-encode and publish. `sampled_at_ms` is the caller's clock
    /// reading, never read internally (this crate's purity rule: transports and time are injected).
    pub async fn runtime_metrics_snapshot(&self, sampled_at_ms: u64) -> RuntimeMetricsSnapshot {
        RuntimeMetricsSnapshot { kernel: self.metrics().await, actors: self.actor_metrics_samples().await, shards: self.shard_metrics_samples().await, sampled_at_ms }
    }

}
//#endregion 🏛️Kernel
