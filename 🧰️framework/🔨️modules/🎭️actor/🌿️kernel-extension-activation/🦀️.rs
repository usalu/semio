//! 🌿️ Kernel extension activation: the pinned activation, capabilities, extension links and the subtree deactivation, kill, suspend and resume cascades. See [`super`].

use super::*;

//#region 🏛️Kernel
impl Kernel {
    //#region 🔗️ExtensionActivation
    /// 📌️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): [`Self::activate`],
    /// widened for extension activation — pins to an EXACT `shard` via [`ShardTable::pin_to`] instead
    /// of the least-loaded heuristic (so an extension always lands on its parent's shard — see
    /// module design doc M6), and computes this actor's initial `capabilities` via
    /// [`intersect_capabilities`] against `parent`'s own already-granted set when `parent` is
    /// `Some` — the never-escalate-past-the-parent security property. `parent` is `None` for a
    /// top-level activation with a caller-supplied grant set (e.g. a `PluginApp` whose capabilities
    /// the host's own broker already resolved); passing `Some` is what makes this the extension path.
    /// This method does **not** itself record the [`Self::link_extension`] parent→child edge — call
    /// both from the host cascade so the two concerns (placement/capability vs. cascade topology)
    /// stay independently testable.
    #[allow(clippy::too_many_arguments)]
    pub async fn activate_pinned(
        &mut self,
        package: PackageId,
        plugin_ordinal: u16,
        kind: ActorKind,
        lane: Lane,
        window: Option<WindowId>,
        _event: ActivationEvent,
        shard: ShardId,
        parent: Option<ActorId>,
        requested_capabilities: Vec<CapabilityGrant>,
        retained: RetainedTurnInput,
        loan:OriginalActorTurnLoan,
    ) -> ActorId {
        let ordinal = self.next_ordinal.entry(package.clone()).or_insert(0);
        let id = ActorId::new(plugin_ordinal, kind.tag().await, *ordinal, 0).await;
        *ordinal += 1;
        let budget = lane_defaults::budget_for(lane);
        let shard = self.shards.pin_to(id, shard).await;
        self.scheduler.register_actor(id, package.clone(), lane, budget, shard, retained,loan).await;
        let capabilities = match parent {
            Some(parent_id) => {
                let granted = self.actors.get(&parent_id).map(|meta| meta.capabilities.clone()).unwrap_or_default();
                intersect_capabilities(&granted, &requested_capabilities).await
            }
            None => requested_capabilities,
        };
        self.actors.insert(id, ActorMeta { reservation: None, transport_key: None, kind, package, capabilities, budget, status: ActorStatus::Activating, failure: FailureState::new(), metrics: ActorMetrics::default(), window });
        id
    }

    /// 🔐️ Records/replaces an already-live actor's granted [`CapabilityGrant`] set — how a host's
    /// broker (or, in tests, the caller) attaches the capabilities a `PluginApp` was actually granted
    /// so that a later [`Self::activate_pinned`] can intersect an extension's requests against them.
    pub async fn set_capabilities(&mut self, actor: ActorId, grants: Vec<CapabilityGrant>) -> Result<(), KernelError> {
        let meta = self.actors.get_mut(&actor).ok_or(KernelError::UnknownActor)?;
        meta.capabilities = grants;
        Ok(())
    }

    /// 🧭️ Convenience: an actor's pinned shard without paying [`Self::actor_record`]'s full
    /// `ActorRecord` assembly cost (that allocates a fresh empty [`Mailbox`] every call).
    pub async fn shard_of(&self, actor: ActorId) -> Option<ShardId> {
        self.shards.shard_of(actor).await
    }

    /// 🔗️ Records a parent→child cascade edge. Both actors must already be live; the child is
    /// typically an `ActorKind::Extension` just returned by [`Self::activate_pinned`], but this is not
    /// itself enforced — cascade topology is a separate concern from actor kind (see this method's own
    /// region doc).
    pub async fn link_extension(&mut self, parent: ActorId, child: ActorId) -> Result<(), KernelError> {
        if !self.actors.contains_key(&parent) || !self.actors.contains_key(&child) {
            return Err(KernelError::UnknownActor);
        }
        if [parent, child].into_iter().any(|actor| self.actors.get(&actor).is_some_and(|meta| meta.reservation.is_some())) {
            return Err(KernelError::InvalidTransition);
        }
        self.links.entry(parent).or_default().push(child);
        Ok(())
    }

    /// 🔗️ The direct children [`Self::link_extension`] recorded for `parent` — empty for a leaf or an
    /// actor with no extensions.
    pub async fn children_of(&self, parent: ActorId) -> Vec<ActorId> {
        self.links.get(&parent).cloned().unwrap_or_default()
    }

    /// 🌳️ Iterative (never self-recursive — R10's "self-recursive async fn" residue shape applies to
    /// `async fn`s exactly as much as sync ones, and this crate's own rule bans the `Box::pin` escape
    /// hatch where a loop will do) post-order walk of `root`'s cascade subtree: every descendant
    /// appears strictly before its own parent, terminating with `root` itself last — i.e. leaves-first.
    /// Shared by every cascading lifecycle op below; [`Self::resume_cascade`] reverses this order to
    /// get the symmetric parent-first restore direction the design calls for.
    async fn subtree_leaves_first(&self, root: ActorId) -> Vec<ActorId> {
        let mut order = Vec::new();
        let mut stack = vec![(root, false)];
        while let Some((node, expanded)) = stack.pop() {
            if expanded {
                order.push(node);
                continue;
            }
            stack.push((node, true));
            for child in self.children_of(node).await {
                stack.push((child, false));
            }
        }
        order
    }

    /// ✂️ Shared removal primitive behind [`Self::deactivate`] and [`Self::kill`] — full teardown of
    /// `root`'s ENTIRE cascade subtree, leaves-first: `Scheduler::unregister_actor` (drops the DRR
    /// entry + mailbox), `ShardTable::unpin` (frees the shard slot/any exclusive lease), removes the
    /// `ActorMeta` and this actor's own outgoing link-table row. Also scrubs every removed id out of
    /// every OTHER actor's children list, so a subtree removal can never leave a dangling edge pointing
    /// at an id [`Self::actor_record`] can no longer resolve. Returns the removed ids leaves-first —
    /// the caller's evidence that the cascade actually walked the whole subtree, not just `root`.
    async fn cascade_remove(&mut self, root: ActorId) -> Vec<ActorId> {
        let order = self.subtree_leaves_first(root).await;
        for &id in &order {
            self.scheduler.unregister_actor(id).await;
            self.shards.unpin(id).await;
            self.actors.remove(&id);
            self.links.remove(&id);
        }
        for children in self.links.values_mut() {
            children.retain(|child| !order.contains(child));
        }
        order
    }

    /// ✂️ Graceful cascade teardown: `root` and every extension hanging off it (transitively),
    /// leaves-first, with **zero orphans** — no descendant is ever left registered after its ancestor
    /// is gone. The call site this ticket's own design names: a plugin closing, or an extension being
    /// uninstalled while live. Errors `UnknownActor` if `root` is not (or no longer) live.
    pub async fn deactivate(&mut self, root: ActorId) -> Result<Vec<ActorId>, KernelError> {
        if !self.actors.contains_key(&root) {
            return Err(KernelError::UnknownActor);
        }
        Ok(self.cascade_remove(root).await)
    }

    /// 🔪️ The failure ladder's cascade teardown — same leaves-first, zero-orphan removal as
    /// [`Self::deactivate`] (this crate's pure kernel state has no separate "abrupt vs. graceful"
    /// axis: both fully retire the subtree; a host distinguishes them only by WHEN it calls each —
    /// `deactivate` for user/uninstall-driven teardown, `kill` for the failure ladder giving up on an
    /// unrecoverable actor). Kept as its own named entry point rather than an alias so call sites read
    /// as intent, and so a future host-side distinction (e.g. a forced `ShardTransport::kill` versus a
    /// graceful drain) has an unambiguous kernel-level hook to attach to. "Parent kill takes its
    /// extensions down" (design doc M6's acceptance wording) is exactly this method called on the
    /// parent.
    pub async fn kill(&mut self, root: ActorId) -> Result<Vec<ActorId>, KernelError> {
        if !self.actors.contains_key(&root) {
            return Err(KernelError::UnknownActor);
        }
        Ok(self.cascade_remove(root).await)
    }

    /// 💤️ Cascading [`Self::suspend`], leaves-first: every descendant is suspended before `root`
    /// itself. This is ALSO this crate's "checkpoint cascade" — [`ActorStatus::Suspended`] already
    /// carries an optional checkpoint payload, so "checkpoint" and "suspend-with-bytes" are the same
    /// kernel operation; only `root` receives the caller's `checkpoint` bytes (a per-descendant
    /// checkpoint is a host/guest-runtime concern — the pure kernel only ever records the bytes it is
    /// handed, never produces them). Descendants suspend with `checkpoint: None`.
    pub async fn suspend_cascade(&mut self, root: ActorId, checkpoint: Option<Vec<u8>>) -> Result<Vec<ActorId>, KernelError> {
        if !self.actors.contains_key(&root) {
            return Err(KernelError::UnknownActor);
        }
        let order = self.subtree_leaves_first(root).await;
        for &id in &order {
            let bytes = if id == root { checkpoint.clone() } else { None };
            self.suspend(id, bytes).await?;
        }
        Ok(order)
    }

    /// ▶️ Cascading [`Self::resume`], PARENT-first — the symmetric restore direction (design doc M6's
    /// web-mirror doc: "symmetric cascade, restore: parent first"): a child's shard/mailbox is only
    /// useful once its parent is running again. Skips any descendant that is not currently
    /// [`ActorStatus::Suspended`] rather than erroring, since a partial cascade (only some children were
    /// ever suspended) must still resume everything that legitimately can.
    pub async fn resume_cascade(&mut self, root: ActorId) -> Result<Vec<ActorId>, KernelError> {
        if !self.actors.contains_key(&root) {
            return Err(KernelError::UnknownActor);
        }
        let mut order = self.subtree_leaves_first(root).await;
        order.reverse();
        let mut resumed = Vec::new();
        for &id in &order {
            if matches!(self.actor_status(id).await, Some(ActorStatus::Suspended { .. })) {
                self.resume(id).await?;
                resumed.push(id);
            }
        }
        Ok(resumed)
    }
    //#endregion 🔗️ExtensionActivation
}
//#endregion 🏛️Kernel
