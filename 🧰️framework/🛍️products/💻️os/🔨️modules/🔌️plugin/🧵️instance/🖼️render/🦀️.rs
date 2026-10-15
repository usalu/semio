//! 🖼️ Render side of the mounted artifact instance: composition graph, interaction state, render, media jobs and media export.

use super::*;

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🕸️ Re-syncs `self.composition`'s ownership/link graph from the parent's own live
    /// `ArtifactRefs` projection (`child_refs()`/`links()`) — the mechanism
    /// `store::os_store::composition_coordinator::CompositionGraph::sync_member`'s own doc comment names as the required follow-up
    /// "after any dispatch that might have changed an artifact's own `ArtifactRefs`". Deliberately
    /// NOT called automatically by `dispatch_emit`/`dispatch_emit_group`: doing so would force
    /// `A::Snapshot: store::ArtifactRefs` onto EVERY `ArtifactApp` impl in the workspace (today
    /// zero of them implement it — `ArtifactRefs`-bearing snapshots are new, later-wave-scoped
    /// schema work), breaking every existing plugin's compile for a graph no leaf artifact needs
    /// synced. A composed artifact's own app calls this explicitly wherever its `handle` might
    /// have changed a `#[child(...)]`/`#[link_slot(...)]` field — see B2's own
    /// `📓️wave1-reports/b2-store-composition-report.md` sharedFileRequest #6.
    pub async fn sync_composition_graph(&mut self) -> Result<(), Fault>
    where
        A::Snapshot: store::ArtifactRefs,
    {
        let snapshot = self.store.snapshot().map_err(|error| error.into_fault())?;
        let parent_id = self.store.envelope().id.clone();
        self.composition.graph_mut().await.sync_member(&parent_id, &snapshot).await.map_err(|error| plugin_sdk_fault(error.to_string()))
    }

    /// ↩️ Task 4: group-aware undo/redo. Routes through
    /// `store::os_store::composition_coordinator::CompositionCoordinator::undo_group`/`redo_group` across `self.store` (the parent)
    /// plus every LIVE child in `self.children` — the coordinator itself filters to members whose
    /// tail actually carries `group_id` (a child never touched by this particular gesture, or one
    /// whose own `undo()`/`redo()` call fails, is SKIPPED, not aborted — see `GroupUndoReport`'s
    /// own doc comment for why abort-all would be actively harmful: one collaborator's foreign or
    /// failed child edit must never permanently freeze this parent's undo stack). This generalizes
    /// the plain single-store path's existing benign `NothingToUndo`/`NothingToRedo`/`ForeignEdit`
    /// collapse to the multi-member case. `self.store` (the parent) and `self.children` (this
    /// composition's CHILD-kind type) are passed as SEPARATE arguments now — the same `Mp`/`Mc`
    /// split `dispatch_group` uses (see its own doc comment) — rather than one homogeneous list,
    /// so no more caller-side ordering dance either: `undo_group`/`redo_group` order parent vs.
    /// children internally to match `dispatch_group`'s apply order, and (per their own doc
    /// comments) treat every member independently regardless of order anyway.
    ///
    /// 🎯️ RESOLVED (was BLOCKED — see `📓️terra-dedyn-fw-os-spacemember-report.md`
    /// §undo_group/redo_group, and `📓️terra-dispatch-group-split-report.md` for the fix):
    /// `store::os_store::composition_coordinator::CompositionCoordinator::undo_group`/`redo_group<Mp: SpaceMember, Mc:
    /// SpaceMember>` now take `parent`/`children` as separate arguments, so `self.store`
    /// (`ArtifactStore<A::Snapshot, A::Mutation>`) unifies as `Mp` and `self.children`'s
    /// member type (`M`, this composition's CHILD-kind type) unifies as `Mc` independently.
    ///
    /// 🧾️ Nothing actually moved (every member was foreign/failed) — benign collapse,
    /// mirroring `NothingToUndo`/`NothingToRedo`/`ForeignEdit` just above: NOT logged
    /// (never touched any store), but the skip diagnostics still ride along so the caller
    /// can see WHY nothing happened instead of silently no-op'ing.
    ///
    /// 🧾️ Append-only, same as the plain path: undo/redo are pure cursor motion, never
    /// logged with an `edit_id` — `child_edit_ids` records every CHILD member this group call
    /// actually touched.
    pub(super) async fn dispatch_group_history_action(&mut self, action: &str, group_id: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        self.admit_child_content_publication_span(self.children.len())?;
        let parent_id = self.store.envelope().id.clone();
        let parent_dialect: ArtifactDialect = A::DIALECT.into();
        if self.store.envelope().dialect.as_ref() != Some(&parent_dialect) {
            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.group-history-dialect"), "composition history requires the parent's exact declared dialect"));
        }
        let parent_ref = ArtifactRef { artifact_id: parent_id.clone(), dialect: parent_dialect };
        let child_refs: Vec<ArtifactRef> = self.children.entries_physical().map(|entry| entry.reference.clone()).collect();

        let mut children: Vec<(&ArtifactRef, &mut M)> = Vec::with_capacity(child_refs.len());
        for (reference, entry) in child_refs.iter().zip(self.children.entries_mut_physical()) {
            children.push((reference, &mut entry.member));
        }

        let report = if action == "undo" { CompositionCoordinator::undo_group(&parent_ref, &mut self.store, &mut children, group_id, identity).await } else { CompositionCoordinator::redo_group(&parent_ref, &mut self.store, &mut children, group_id, identity).await };
        drop(children);
        self.cache = None;
        for (reference, _) in report.undone.iter().filter(|(reference, _)| reference.artifact_id != parent_id) {
            let Some(key) = self.children.keyed_entries().find(|(_, entry)| entry.reference.artifact_id == reference.artifact_id).map(|(key, _)| key.to_key()) else {
                return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.group-history-root"), "group history moved a child without exact immutable-root authority"));
            };
            let publication_generation = self.admit_child_content_publication()?;
            self.publish_member_content(publication_generation, key.borrowed()).await?;
        }

        let diagnostics: Vec<semio_framework_diagnostic::Diagnostic> = report
            .skipped
            .iter()
            .map(|(reference, error)| {
                semio_framework_diagnostic::Diagnostic::error("composition.group-history.skipped-member", semio_framework_diagnostic::TextSpan::default(), format!("{action} skipped member {} ({error})", reference.artifact_id))
            })
            .collect();

        if report.undone.is_empty() {
            let mut result = Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
            result.diagnostics = diagnostics;
            return Ok(result);
        }

        self.record_command(action, ActionKind::History, None, None, Vec::new(), None);
        let child_edit_ids: Vec<String> = report.undone.iter().filter(|(reference, _)| reference.artifact_id != parent_id).map(|(_, edit_id)| edit_id.clone()).collect();
        if let Some(last) = self.command_log.last_mut() {
            last.child_edit_ids = child_edit_ids;
        }

        let mut result = Self::empty_result(action, meta, Vec::new(), vec![history_changed_event().await], UiDirtyScope::Full).await;
        result.diagnostics = diagnostics;
        Ok(result)
    }

    /// 🧾️ `domain=granularity:id,id` for one `InteractionState`'s selection half — the compact witness
    /// the persist round-trip check below compares, so a lost selection names its own domain instead of
    /// reaching the user as a silently empty panel.
    fn interaction_selection_witness(state: &protocol::InteractionState) -> String {
        state.selection.iter().map(|(domain, selection)| format!("{domain}={}:{}", selection.granularity, selection.ids.join(","))).collect::<Vec<_>>().join(";")
    }

    /// 🕹️ Combines the persisted-local `interaction_store` snapshot with the ephemeral
    /// `interaction_hover` map into one `InteractionState` — the "app-side source" a host's presence
    /// heartbeat reads for THIS app instance before calling `assemble_presence_interaction`
    /// (`🏪️store/🔄️sync/🦀️.rs`, wave 2a) with it plus this app's declared hover/selection
    /// specs (from `AppDefinition.interactions`, already available wherever a heartbeat is built).
    pub async fn interaction_state(&self) -> protocol::InteractionState {
        let mut state = self.interaction_selection_snapshot();
        state.hover = self.interaction_hover.clone();
        state
    }

    /// 🕹️ The persisted-local selection half, with a store read failure NAMED instead of silently
    /// collapsing into an empty `InteractionState`. Every reader treats "no selection" and "the
    /// selection could not be read" identically, so a swallowed read reaches the user as a panel that
    /// renders the empty-document summary on a live pick — measured in the browser on
    /// `framework.panel.inspection` (ticket 26/09/02/PUZZLE-3D-END-TO-END, wave B6: 25 full refreshes
    /// over 73 s kept the pre-pick body hash while the same turn's leftover `InteractionView` carried
    /// the picked id).
    pub(super) fn interaction_selection_snapshot(&self) -> protocol::InteractionState {
        let mut state = match self.interaction_store.snapshot() {
            Ok(state) => state,
            Err(error) => {
                crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] interaction-store snapshot unavailable, selection read as empty: {error}"));
                protocol::InteractionState::default()
            }
        };
        if let Some(overlay) = &self.interaction_leftover_selection {
            for (domain, selection) in &overlay.selection {
                let empty = state.selection.get(domain).map(|current| current.ids.is_empty()).unwrap_or(true);
                if empty && !selection.ids.is_empty() {
                    state.selection.insert(domain.clone(), selection.clone());
                    if let Some(mode) = overlay.active_mode.get(domain) {
                        state.active_mode.insert(domain.clone(), *mode);
                    }
                    if let Some(granularity) = overlay.active_granularity.get(domain) {
                        state.active_granularity.insert(domain.clone(), granularity.clone());
                    }
                }
            }
            let leftover_ids: Vec<String> = overlay.selection.values().flat_map(|selection| selection.ids.iter().cloned()).collect();
            let vortex_empty = state.selection.get("vortex").map(|selection| selection.ids.is_empty()).unwrap_or(true);
            if vortex_empty && !leftover_ids.is_empty() {
                let granularity = overlay
                    .active_granularity
                    .get("vortex")
                    .cloned()
                    .or_else(|| overlay.selection.get("vortex").map(|selection| selection.granularity.clone()))
                    .filter(|granularity| !granularity.is_empty())
                    .unwrap_or_else(|| "object".to_string());
                state.selection.insert("vortex".to_string(), protocol::DomainSelection { granularity: granularity.clone(), ids: leftover_ids, anchor_id: None });
                if let Some(mode) = overlay.active_mode.get("vortex") {
                    state.active_mode.insert("vortex".to_string(), *mode);
                }
                state.active_granularity.entry("vortex".to_string()).or_insert(granularity);
            }
        }
        let vortex_empty = state.selection.get("vortex").map(|selection| selection.ids.is_empty()).unwrap_or(true);
        if vortex_empty && !self.interaction_leftover_ids.is_empty() {
            let granularity = state.active_granularity.get("vortex").cloned().filter(|granularity| !granularity.is_empty()).unwrap_or_else(|| "object".to_string());
            state.selection.insert("vortex".to_string(), protocol::DomainSelection { granularity: granularity.clone(), ids: self.interaction_leftover_ids.clone(), anchor_id: None });
            state.active_granularity.entry("vortex".to_string()).or_insert(granularity);
        }
        state
    }

    /// 🌳️ Resolves `def`'s `DomainTopology` for the current document — the CLOSURE/RANGE-ARITHMETIC
    /// use (always a concrete, possibly-empty topology; safe for `next_selection`/`next_hover`, which
    /// degrade gracefully to single-target over an empty one). `HierarchyProvider::Topology` calls the
    /// app's own `interaction_topology`; `HierarchyProvider::UiTree` reads the cache `render`'s
    /// post-processing pass populates from the last rendered tree; `HierarchyProvider::Flat` is
    /// trivially empty (no structure by declaration); `HierarchyProvider::PathDelimited` self-derives
    /// a topology from `known_ids` (the ids already involved in this dispatch — the current batch's
    /// targets plus whatever was already selected/hovered) by splitting each on `delimiter` — an
    /// INHERENT approximation (only ids the interaction machine has already seen are represented,
    /// since the wrapper has no document access of its own for an arbitrary id space); an app needing
    /// a real full-universe topology for a delimited id scheme declares `HierarchyProvider::Topology`
    /// and implements `interaction_topology` instead.
    async fn resolve_domain_topology(&mut self, def: &InteractionDefinition, known_ids: impl Iterator<Item = String>) -> Result<protocol::DomainTopology, Fault> {
        match &def.hierarchy {
            protocol::HierarchyProvider::Flat => Ok(protocol::DomainTopology::default()),
            protocol::HierarchyProvider::Topology => Ok(self.app_interaction_topology().await?.domains.get(&def.id).cloned().unwrap_or_default()),
            protocol::HierarchyProvider::UiTree => Ok(self.interaction_ui_topology.get(&def.id).cloned().unwrap_or_default()),
            protocol::HierarchyProvider::PathDelimited { delimiter } => Ok(derive_path_delimited_topology(def, delimiter, known_ids).await),
        }
    }

    /// 🗺️ `A::interaction_topology` for the LIVE document generation, built at most once per generation
    /// — see [`Self::interaction_topology_memo`] for why one pick used to build it twice.
    async fn app_interaction_topology(&mut self) -> Result<&protocol::InteractionTopology, Fault> {
        self.refresh_cache().await?;
        let key = (self.store.generation(), self.config_store.generation());
        if self.interaction_topology_memo.as_ref().map(|(cached, _)| *cached) != Some(key) {
            let render_operation = self.live_render_operation();
            let parent_document_id = self.store.envelope().id.clone();
            let (_, snapshot, config, history) = self.cache.as_ref().expect("cache refreshed above");
            let doc = ArtifactView::with_render_context(snapshot.as_ref(), history.as_ref(), ChildContentView::clone(&self.child_content_root), render_operation, parent_document_id, None).await.with_instance(self.instance_operation_owner.clone());
            let cfg = ConfigView { snapshot: config.as_ref(), window: None };
            let topology = A::interaction_topology(&doc, &cfg).await.map_err(|error| plugin_sdk_fault(error.into_message()))?;
            self.interaction_topology_memo = Some((key, topology));
        }
        Ok(&self.interaction_topology_memo.as_ref().expect("memo filled above").1)
    }

    /// 🧹 Builds the `InteractionTopology` `validate_state`'s PRUNING (existence-check) use needs —
    /// deliberately DIFFERENT from `resolve_domain_topology`'s closure/range use: a `Flat` domain is
    /// OMITTED from the map entirely (not inserted as an empty topology), because `validate_state`
    /// treats a missing domain entry as "no membership information, keep every id" while an EMPTY
    /// `DomainTopology` would mean "no id is a member" and prune every `Flat` selection on every
    /// dispatch — `Flat` genuinely has no structure to check staleness against, by declaration; an app
    /// wanting deleted-node pruning for a nominally-flat domain declares `HierarchyProvider::Topology`
    /// with one root `TopologyNode` per valid id instead.
    ///
    /// 🧹️ On the DOCUMENT-CHANGE pass, an EMPTY topology for a domain that still carries a
    /// selection is not "no information": it is the app answering that none of those ids
    /// exist any more — the exact case this prune exists for (every id deleted). Skipping it
    /// left `validate_state` with no existence set to check against, so the stale ids
    /// survived. A `Pick` pass keeps the old rule: an app that publishes no topology at all
    /// has not made a statement about existence, and its hover must not be pruned.
    async fn build_full_interaction_topology(&mut self, state: &protocol::InteractionState, origin: InteractionRevalidateOrigin) -> Result<protocol::InteractionTopology, Fault> {
        let defs: Vec<InteractionDefinition> = self.registry.interactions().await.cloned().collect();
        let mut domains = BTreeMap::new();
        for def in &defs {
            if matches!(def.hierarchy, protocol::HierarchyProvider::Flat) {
                continue;
            }
            let selected_ids = state.selection.get(&def.id).map(|selection| selection.ids.clone()).unwrap_or_default();
            let hovered_ids = state.hover.get(&def.id).map(|hover| hover.ids.clone()).unwrap_or_default();
            let checkable = origin == InteractionRevalidateOrigin::DocumentChange && !selected_ids.is_empty();
            let topology = self.resolve_domain_topology(def, selected_ids.into_iter().chain(hovered_ids)).await?;
            if topology.ordered.is_empty() && !checkable {
                continue;
            }
            domains.insert(def.id.clone(), topology);
        }
        Ok(protocol::InteractionTopology { domains })
    }

    /// 🧹 `validate_state` against fresh topology, then persists the result: hover into the ephemeral
    /// `interaction_hover` map, selection/mode/granularity into `interaction_store` via
    /// `ApplyInLane{lane: HistoryLane::Interaction}` — but ONLY when the persisted half actually
    /// changed, so a no-op revalidation (the common case for `revalidate_interaction_on_document_change`,
    /// called after every artifact dispatch even when nothing was selected) never mints an empty edit.
    ///
    /// 🕳️ `origin` decides whether a lost id is a DEFECT or the mechanism working: a
    /// [`InteractionRevalidateOrigin::DocumentChange`] pass exists precisely to prune ids the document
    /// no longer has, while a [`InteractionRevalidateOrigin::Pick`] pass carries ids a user or an app
    /// just named and may never drop them silently — see [`interaction_selection_loss_v1`].
    async fn revalidate_and_persist_interaction_state(&mut self, combined: protocol::InteractionState, _meta: &ActionMeta, origin: InteractionRevalidateOrigin, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<protocol::InteractionState, Fault> {
        let mut outlines: Vec<protocol::InteractionOutline> = Vec::new();
        for def in self.registry.interactions().await {
            outlines.push(def.outline().await);
        }
        let topology = self.build_full_interaction_topology(&combined, origin).await?;
        let validated = protocol::validate_state(&outlines, &topology, &combined).await;
        let validated = if origin == InteractionRevalidateOrigin::DocumentChange {
            let declared: Vec<String> = self.registry.interactions().await.map(|def| def.id.clone()).collect();
            let (pruned, overlay, leftover_ids) = interaction_after_document_change_v1(&validated, &declared, self.interaction_leftover_selection.as_ref(), &self.interaction_leftover_ids);
            self.interaction_leftover_selection = overlay;
            self.interaction_leftover_ids = leftover_ids;
            pruned
        } else {
            validated
        };
        self.interaction_hover = validated.hover.clone();
        let persisted_before = self.interaction_store.snapshot().unwrap_or_default();
        let persisted = protocol::InteractionState { selection: validated.selection, hover: BTreeMap::new(), active_mode: validated.active_mode, active_granularity: validated.active_granularity };
        let dispatched = Self::interaction_selection_witness(&combined);
        let requested = Self::interaction_selection_witness(&persisted);
        let minted = persisted != persisted_before;
        if minted {
            self.interaction_store.dispatch(ArtifactCommand::ApplyInLane { mutations: vec![InteractionConfigMutation::set_state(persisted.clone())], lane: HistoryLane::Interaction, transaction: None }, identity).await.map_err(|error| error.into_fault())?;
        }
        if origin == InteractionRevalidateOrigin::Pick {
            let readback = Self::interaction_selection_witness(&self.interaction_selection_snapshot());
            if let Some(reason) = interaction_selection_loss_v1(&dispatched, &requested, &readback, minted) {
                report_interaction_selection_loss(reason, &dispatched, &requested, &readback, &topology);
            }
        }
        Ok(persisted)
    }

    /// 🕹️ Task 4: called after every artifact (document) dispatch so an id deleted elsewhere in the
    /// document drops out of selection/hover automatically. Skips the topology/validate work
    /// entirely when the app declares no interaction domains at all (the overwhelming majority of
    /// dispatches, for an app mid-migration or with no interactions yet).
    /// 🧹️ The prune covers the leftover cache the guest lays over an empty store selection too
    /// ([`interaction_after_document_change_v1`]), so a whole-document replacement cannot leave the
    /// replaced document's pick standing in `interaction.selection(..)` — which is what every app
    /// command that reads a selection (`deleteSelection`, the transform verbs) would then act on.
    pub(super) async fn revalidate_interaction_on_document_change(&mut self, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        if self.registry.interactions().await.next().is_none() {
            return Ok(());
        }
        self.revalidate_and_persist_interaction_state(self.interaction_state().await, meta, InteractionRevalidateOrigin::DocumentChange, identity).await?;
        Ok(())
    }

    /// 🕹️ Applies an emit's [`InteractionWrite`]s through the SAME single-writer machine
    /// `dispatch_interaction_action` uses (`protocol::next_selection` over freshly resolved
    /// topology, then `revalidate_and_persist_interaction_state`), so an app-initiated re-select
    /// obeys every declared `SelectionSpec` rule — mode clamping, transitive closure, pruning —
    /// instead of poking `interaction_store` behind the state machine's back. Called only AFTER
    /// the triggering action's own mutations landed, so a just-created id is already in topology.
    /// An undeclared domain is a plugin bug, not a client input, so it faults rather than
    /// silently dropping the write.
    pub(super) async fn apply_interaction_writes(&mut self, writes: &[InteractionWrite], meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        if writes.is_empty() {
            return Ok(());
        }
        let mut state = self.interaction_selection_snapshot();
        for write in writes {
            let def = self.registry.interaction(&write.domain).await.cloned().ok_or_else(|| plugin_sdk_fault(format!("interaction write names undeclared domain {}", write.domain)))?;
            let mode = state.active_mode.get(&write.domain).copied().unwrap_or_else(|| def.selection.modes.first().copied().unwrap_or(protocol::SelectionMode::Multiple));
            let current = state.selection.get(&write.domain).cloned().unwrap_or_default();
            let known_ids = current.ids.iter().cloned().chain(write.targets.iter().map(|target| target.id.clone()));
            let topology = self.resolve_domain_topology(&def, known_ids).await?;
            let selection_input = protocol::SelectionInput { targets: write.targets.clone(), merge: write.merge, mode };
            let next = if write.targets.is_empty() && write.merge == protocol::MergeMode::Replace {
                protocol::DomainSelection { ids: Vec::new(), anchor_id: None, ..current }
            } else {
                protocol::next_selection(&def.selection, &current, &topology, &selection_input).await
            };
            state.selection.insert(write.domain.clone(), next);
            state.active_mode.insert(write.domain.clone(), mode);
        }
        state.hover = self.interaction_hover.clone();
        let domains: Vec<String> = writes.iter().map(|write| write.domain.clone()).collect();
        let (overlay, leftover_ids) = leftover_after_app_selection_write_v1(self.interaction_leftover_selection.as_ref(), &self.interaction_leftover_ids, &state, &domains);
        self.interaction_leftover_selection = overlay;
        self.interaction_leftover_ids = leftover_ids;
        self.revalidate_and_persist_interaction_state(state, meta, InteractionRevalidateOrigin::Pick, identity).await?;
        Ok(())
    }

    /// 🕹️ The actual body of `dispatch_action`'s interception for the six framework interaction verbs
    /// (`INTERACTION_ACTION_IDS`) — the ONE place any app's hover/selection ever mutates. Runs the
    /// pure os-kernel machine (`next_selection`/`next_hover`), re-validates+persists via
    /// `revalidate_and_persist_interaction_state`, records the command-log row under `ActionKind::Interaction`
    /// (kept out of the history panel — `finish_recorded`'s `skip_history_panel` check), and closes
    /// on the APP-DECLARED refresh scope for the dispatched verb and the domains it actually
    /// touched (`A::interaction_scope`, one entry per `InteractionVerb`), falling back to the scope
    /// DERIVED from the app's own surface declarations
    /// (`semio_framework::interaction_declared_refresh_scope` over `WindowKindDefinition.interactions`)
    /// and only then to `UiDirtyScope::Full`, for an app that declares neither. `touched` is the single domain the
    /// verb names, or every declared domain for the two whole-app verbs
    /// (`clearSelection`/`selectAll`) — the exact set the arms above wrote.
    ///
    /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C: the state transition itself lives in
    /// [`Self::apply_interaction_verb`], shared with the in-reactor fold of a GUEST-emitted verb
    /// ([`Self::fold_inline_interaction_verbs`]); this wrapper is the HOST-dispatched half, which
    /// alone owns a reserved-commit permit and shapes the result as its own `InvocationResult`.
    pub(super) async fn dispatch_interaction_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, permit: &FrameworkReservedCommitPermit, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        self.validate_framework_reserved_commit(action, permit).await?;
        let applied = self.apply_interaction_verb(action, args, meta, Some(permit), identity).await?;
        let mut result = Self::empty_result(action, meta, Vec::new(), Vec::new(), applied.scope).await;
        result.output = applied.leftover;
        Ok(result)
    }

    /// 🕹️ ONE interaction verb applied to this instance's session interaction state — the body
    /// `dispatch_interaction_action` (host-dispatched, `permit: Some`) and the guest-emitted fold
    /// (`fold_inline_interaction_verbs`, `permit: None`) share, so the six match arms exist once.
    ///
    /// 🔏️ The reserved-commit permit is a HOST-dispatch concern: it is the cancellation lease and
    /// revision witness of the reserved `FrameworkInteraction*Job` that admitted the host's
    /// verb, and `validate_framework_reserved_commit` refuses a commit against a document
    /// revision that moved while that job was queued. An inline verb has no such job: it runs
    /// inside an ALREADY-ADMITTED app command on the same instance, in the same turn, after that
    /// command's own store lanes landed, and it touches only the session interaction snapshot
    /// (`interaction_store` / `interaction_hover` / the leftover overlay — never the document, so
    /// there is no revision to go stale against). Skipping the permit there is therefore sound
    /// and the two `if let Some(permit)` checks below are exactly the host path's own two.
    async fn apply_interaction_verb(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, permit: Option<&FrameworkReservedCommitPermit>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<AppliedInteractionVerb, Fault> {
        self.refresh_cache().await?;
        let mut state = self.interaction_selection_snapshot();
        let verb = InteractionVerb::of_action(action).ok_or_else(|| plugin_sdk_fault(format!("{action} is not one of the six framework interaction verbs")))?;
        let mut retire_leftover_domains: Vec<String> = Vec::new();
        match verb {
            InteractionVerb::Select => {
                let domain_id = interaction_domain_id_arg(args, action).await?;
                let def = self.registry.interaction(&domain_id).await.cloned().ok_or_else(|| plugin_sdk_fault(format!("{action}: undeclared interaction domain {domain_id}")))?;
                let targets = parse_interaction_targets(args, action).await?;
                let merge = parse_merge_mode(args).await?;
                let mode = state.active_mode.get(&domain_id).copied().unwrap_or_else(|| def.selection.modes.first().copied().unwrap_or(protocol::SelectionMode::Single));
                let current = state.selection.get(&domain_id).cloned().unwrap_or_default();
                let known_ids = current.ids.iter().cloned().chain(targets.iter().map(|target| target.id.clone()));
                let topology = self.resolve_domain_topology(&def, known_ids).await?;
                if targets.is_empty() && merge == protocol::MergeMode::Replace {
                    let cleared_ids: HashSet<String> = state.selection.get(&domain_id).map(|selection| selection.ids.iter().cloned().collect()).unwrap_or_default();
                    retire_leftover_domains.push(domain_id.clone());
                    state.selection.remove(&domain_id);
                    if domain_id != "vortex" {
                        if let Some(vortex) = state.selection.get("vortex") {
                            if !vortex.ids.is_empty() && vortex.ids.iter().all(|id| cleared_ids.contains(id)) {
                                retire_leftover_domains.push("vortex".to_string());
                                state.selection.remove("vortex");
                            }
                        }
                    }
                    self.interaction_leftover_ids.retain(|id| !cleared_ids.contains(id));
                    state.active_mode.insert(domain_id, mode);
                } else {
                    let selection_input = protocol::SelectionInput { targets: targets.clone(), merge, mode };
                    let mut next = protocol::next_selection(&def.selection, &current, &topology, &selection_input).await;
                    if next.ids.is_empty() && !targets.is_empty() {
                        next.granularity = targets.last().map(|target| target.granularity.clone()).unwrap_or_else(|| if current.granularity.is_empty() { "object".to_string() } else { current.granularity.clone() });
                        next.ids = targets.iter().map(|target| target.id.clone()).collect();
                        next.anchor_id = targets.last().map(|target| target.id.clone());
                    }
                    state.selection.insert(domain_id.clone(), next);
                    state.active_mode.insert(domain_id, mode);
                }
            }
            InteractionVerb::Hover => {
                let domain_id = interaction_domain_id_arg(args, action).await?;
                let def = self.registry.interaction(&domain_id).await.cloned().ok_or_else(|| plugin_sdk_fault(format!("{action}: undeclared interaction domain {domain_id}")))?;
                let channel = args.and_then(|value| value.get("channel")).and_then(DslValue::as_str).unwrap_or("pointer").to_string();
                let targets = parse_interaction_targets(args, action).await?;
                let current = self.interaction_hover.get(&domain_id).cloned().unwrap_or_default();
                let known_ids = current.ids.iter().cloned().chain(targets.iter().map(|target| target.id.clone()));
                let topology = self.resolve_domain_topology(&def, known_ids).await?;
                let hover_input = protocol::HoverInput { channel, targets };
                let next = protocol::next_hover(&def.hover, &topology, &hover_input).await;
                if next.ids.is_empty() {
                    self.interaction_hover.remove(&domain_id);
                } else {
                    self.interaction_hover.insert(domain_id, next);
                }
            }
            InteractionVerb::ClearSelection => {
                retire_leftover_domains = self.registry.interactions().await.map(|def| def.id.clone()).collect();
                self.interaction_leftover_ids.clear();
                for domain_id in &retire_leftover_domains {
                    state.selection.remove(domain_id);
                }
            }
            InteractionVerb::SelectAll => {
                for def in self.registry.interactions().await.cloned().collect::<Vec<_>>() {
                    let granularity = state.active_granularity.get(&def.id).cloned().unwrap_or_else(|| def.granularities.first().map(|granularity| granularity.id.clone()).unwrap_or_default());
                    let mode = state.active_mode.get(&def.id).copied().unwrap_or_else(|| def.selection.modes.first().copied().unwrap_or(protocol::SelectionMode::Multiple));
                    let current = state.selection.get(&def.id).cloned().unwrap_or_default();
                    let hovered_ids = self.interaction_hover.get(&def.id).map(|hover| hover.ids.clone()).unwrap_or_default();
                    let known_ids = current.ids.iter().cloned().chain(hovered_ids);
                    let topology = self.resolve_domain_topology(&def, known_ids).await?;
                    let ids: Vec<String> = topology.ordered.iter().filter(|node| node.granularity == granularity).map(|node| node.id.clone()).collect();
                    if !ids.is_empty() {
                        state.selection.insert(def.id.clone(), protocol::DomainSelection { granularity, ids, anchor_id: None });
                    }
                    state.active_mode.insert(def.id.clone(), mode);
                }
            }
            InteractionVerb::SetSelectionMode => {
                let domain_id = interaction_domain_id_arg(args, action).await?;
                let def = self.registry.interaction(&domain_id).await.cloned().ok_or_else(|| plugin_sdk_fault(format!("{action}: undeclared interaction domain {domain_id}")))?;
                let raw = args.and_then(|value| value.get("mode")).and_then(DslValue::as_str).ok_or_else(|| plugin_sdk_fault(format!("{action} missing mode")))?;
                let mode = parse_selection_mode(raw).await?;
                if !def.selection.modes.contains(&mode) {
                    return Err(plugin_sdk_fault(format!("{action}: domain {domain_id} does not declare mode {raw}")));
                }
                state.active_mode.insert(domain_id, mode);
            }
            InteractionVerb::SetGranularity => {
                let domain_id = interaction_domain_id_arg(args, action).await?;
                let def = self.registry.interaction(&domain_id).await.cloned().ok_or_else(|| plugin_sdk_fault(format!("{action}: undeclared interaction domain {domain_id}")))?;
                let granularity_id = args.and_then(|value| value.get("granularityId")).and_then(DslValue::as_str).ok_or_else(|| plugin_sdk_fault(format!("{action} missing granularityId")))?.to_string();
                if !def.granularities.iter().any(|granularity| granularity.id == granularity_id) {
                    return Err(plugin_sdk_fault(format!("{action}: domain {domain_id} does not declare granularity {granularity_id}")));
                }
                state.active_granularity.insert(domain_id, granularity_id);
            }
        }
        let touched = match verb {
            InteractionVerb::ClearSelection | InteractionVerb::SelectAll => self.registry.interactions().await.map(|def| def.id.clone()).collect::<Vec<String>>(),
            _ => vec![interaction_domain_id_arg(args, action).await?],
        };
        state.hover = self.interaction_hover.clone();
        if !retire_leftover_domains.is_empty() {
            let (overlay, leftover_ids) = leftover_after_app_selection_write_v1(self.interaction_leftover_selection.as_ref(), &self.interaction_leftover_ids, &state, &retire_leftover_domains);
            self.interaction_leftover_selection = overlay;
            self.interaction_leftover_ids = leftover_ids;
        }
        if verb == InteractionVerb::Hover {
            let current_empty = state.selection.values().all(|selection| selection.ids.is_empty());
            if current_empty {
                if let Some(prior) = &self.interaction_leftover_selection {
                    if prior.selection.values().any(|selection| !selection.ids.is_empty()) {
                        state.selection = prior.selection.clone();
                        for (domain, mode) in &prior.active_mode {
                            state.active_mode.entry(domain.clone()).or_insert(*mode);
                        }
                        for (domain, granularity) in &prior.active_granularity {
                            state.active_granularity.entry(domain.clone()).or_insert_with(|| granularity.clone());
                        }
                    }
                }
            }
        }
        Self::overlay_leftover_ids_into_vortex(&mut state);
        let current_ids: Vec<String> = state.selection.values().flat_map(|selection| selection.ids.iter().cloned()).collect();
        if !current_ids.is_empty() {
            self.interaction_leftover_ids = current_ids;
        } else if !self.interaction_leftover_ids.is_empty() && retire_leftover_domains.is_empty() {
            let granularity =
                state.active_granularity.get("vortex").cloned().or_else(|| state.selection.get("vortex").map(|selection| selection.granularity.clone())).filter(|granularity| !granularity.is_empty()).unwrap_or_else(|| "object".to_string());
            state.selection.insert("vortex".to_string(), protocol::DomainSelection { granularity: granularity.clone(), ids: self.interaction_leftover_ids.clone(), anchor_id: None });
            state.active_granularity.entry("vortex".to_string()).or_insert(granularity);
        }
        let selection_cleared = !retire_leftover_domains.is_empty();
        let leftover = Self::leftover_interaction_view_from(&state, &self.interaction_hover, meta.view_state.as_ref().and_then(|view| view.window_id.as_deref()), selection_cleared);
        self.interaction_leftover_selection = Some(state.clone());
        if let Some(permit) = permit {
            self.validate_framework_reserved_commit(action, permit).await?;
        }
        self.revalidate_and_persist_interaction_state(state, meta, InteractionRevalidateOrigin::Pick, identity).await?;
        if let Some(permit) = permit {
            if permit.lease.is_cancelled().await {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancelled"), format!("framework route '{action}' was cancelled before interaction publication")));
            }
        }
        self.record_command(action, ActionKind::Interaction, None, None, Vec::new(), None);
        let declared: Vec<&str> = touched.iter().map(String::as_str).collect();
        let scope = A::interaction_scope(verb, &declared).or_else(|| self.registry.interaction_declared_refresh_scope(verb, &declared)).unwrap_or(UiDirtyScope::Full);
        Ok(AppliedInteractionVerb { scope, leftover })
    }

    /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — applies every interaction verb a guest
    /// `Emit` asked the host to replay (`Effect::ReplayShellCommand { action_id ∈
    /// INTERACTION_ACTION_IDS }` or `Effect::DispatchAction { action ∈ INTERACTION_ACTION_IDS }`,
    /// peeled by [`take_inline_interaction_verbs`]) INLINE, in this
    /// same turn, on this same instance, in emission order. Until now the host received that
    /// effect, re-entered `onAction` with it, and the verb landed one guest round trip later —
    /// queued BEHIND any pointer input the user issued meanwhile (design §0 "ordering hazard").
    ///
    /// 🧮️ The scope merge is `UiDirtyScope::merged_with` (widest wins: `Full` absorbs, `None`
    /// yields, two `Partial`s union) over the carrying emit's own scope and every applied verb's
    /// app-declared interaction refresh scope, so the one refresh the host runs for the carrying
    /// command already covers the surfaces the pick dirtied.
    ///
    /// 🩹 A verb that faults (undeclared domain, malformed targets, unknown merge …) must NOT fail
    /// the app command that carried it — its mutations already landed and the pick already
    /// happened — so the fault becomes a `dsl::Diagnostic` the caller attaches to its result and
    /// the remaining verbs still apply. Infallible by construction for that reason.
    pub(super) async fn fold_inline_interaction_verbs(&mut self, carrier: &str, folded: Vec<(String, Option<DslValue>)>, meta: &ActionMeta, scope: UiDirtyScope, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> InlineInteractionFold {
        let mut fold = InlineInteractionFold { scope, leftover: None, diagnostics: Vec::new() };
        for (action, args) in folded {
            match self.apply_interaction_verb(&action, args.as_ref(), meta, None, identity).await {
                Ok(applied) => {
                    fold.scope = fold.scope.merged_with(applied.scope);
                    fold.leftover = Some(applied.leftover);
                }
                Err(fault) => {
                    fold.diagnostics.push(semio_framework_diagnostic::Diagnostic::error(
                        INLINE_INTERACTION_VERB_DIAGNOSTIC_CODE,
                        semio_framework_diagnostic::TextSpan::default(),
                        format!("{carrier}: inline {action} was not applied ({}: {})", fault.code.0, fault.message),
                    ));
                }
            }
        }
        fold
    }

    /// 🕹️ Reserved leftover publication the host already peels (`leftoverShellInvocationFrames` →
    /// leftover `Invocation.output`). Built from the just-computed InteractionView, not the
    /// post-revalidate store snapshot, so a pick still lands on leftover when topology later prunes.
    /// 🕹️ leftoverInteractionStateV1 twin: leftover.ids become the vortex snapshot Inspection reads
    /// when persist `selection(vortex)` is empty after topology prune.
    fn overlay_leftover_ids_into_vortex(state: &mut protocol::InteractionState) {
        let leftover_ids: Vec<String> = state.selection.values().flat_map(|selection| selection.ids.iter().cloned()).collect();
        let vortex_empty = state.selection.get("vortex").map(|selection| selection.ids.is_empty()).unwrap_or(true);
        if vortex_empty && !leftover_ids.is_empty() {
            let granularity =
                state.active_granularity.get("vortex").cloned().or_else(|| state.selection.get("vortex").map(|selection| selection.granularity.clone())).filter(|granularity| !granularity.is_empty()).unwrap_or_else(|| "object".to_string());
            state.selection.insert("vortex".to_string(), protocol::DomainSelection { granularity: granularity.clone(), ids: leftover_ids, anchor_id: None });
            state.active_granularity.entry("vortex".to_string()).or_insert(granularity);
        }
    }

    /// 🪟️ `window_id` is the window INSTANCE the action that produced this leftover addressed, and
    /// it rides the encode so the host publishes the overlay under that pane rather than inferring one
    /// (`publishLeftoverWorldSelectionV1`, `🏛️ShellHost/🟦️.tsx`). `None` is the document scope of a
    /// windowless action — never a synthetic window (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B56).
    fn leftover_interaction_view_from(state: &protocol::InteractionState, hover: &InteractionHoverState, window_id: Option<&str>, selection_cleared: bool) -> DslValue {
        let selected_ids = leftover_selected_ids_of(state);
        let locked: Vec<(String, DslValue)> = selected_ids.iter().map(|id| (id.clone(), DslValue::Bool(false))).collect();
        let hover_target = hover.iter().find_map(|(domain, hover)| {
            hover.ids.first().map(|id| DslValue::object([("domain".to_string(), DslValue::String(domain.clone())), ("channel".to_string(), DslValue::String(hover.channel.clone())), ("id".to_string(), DslValue::String(id.clone()))]))
        });
        let gumball = DslValue::object([("active".to_string(), DslValue::Bool(leftover_gumball_active_v1())), ("anchorId".to_string(), selected_ids.first().cloned().map(DslValue::String).unwrap_or(DslValue::Null))]);
        DslValue::object([(
            "interactionView".to_string(),
            DslValue::object([
                ("selection".to_string(), semio_framework_value::ToValue::to_value(&state.selection)),
                ("hover".to_string(), semio_framework_value::ToValue::to_value(hover)),
                ("activeMode".to_string(), semio_framework_value::ToValue::to_value(&state.active_mode)),
                ("activeGranularity".to_string(), semio_framework_value::ToValue::to_value(&state.active_granularity)),
                ("selectedIds".to_string(), semio_framework_value::ToValue::to_value(&selected_ids)),
                ("locked".to_string(), DslValue::Object(locked)),
                ("gumball".to_string(), gumball),
                ("hoverTarget".to_string(), hover_target.unwrap_or(DslValue::Null)),
                ("windowId".to_string(), window_id.map(|id| DslValue::String(id.to_string())).unwrap_or(DslValue::Null)),
                ("selectionCleared".to_string(), DslValue::Bool(selection_cleared)),
            ]),
        )])
    }
    /// 🕹️ Task 5: recursively walks a just-presented `ComponentTree`, and for every `Component::Tree`
    /// carrying an `interaction_domain` caches a `DomainTopology` derived from its children (the
    /// `HierarchyProvider::UiTree` half of `resolve_domain_topology`) into `interaction_ui_topology`.
    /// Recursion is uniform over `TreeNode::children` now — every component nests the same way (no
    /// more per-variant `Stack`/`Section`/`Group`/`Field` match arms; a leaf is just a node with no
    /// children).
    ///
    /// 👥️ M2 (ticket 26/08/17 `design-unified.md`): ALSO derives one `PresenceUpdate` per node
    /// whose id is selected/hovered — own state off `state`/`self.interaction_hover`, peers off
    /// `InteractionView::peers_selecting`/`peers_hovering` — pushed onto `self.pending_presence`
    /// for `plugin_runtime::plugin_render_surface` to drain. This closes the gap the prior packet's
    /// own report named ("nothing in this function currently reads `state`" — see git history):
    /// selection/hover is `ui_contract::PresenceUpdate` now, never written back onto the tree
    /// itself (`TreeNode`/`Component::TreeItem` carry no presence field — the render-plane
    /// derivation lives entirely in this outbox, never in a document mutation).
    ///
    /// 🪟️ A windowed tree hands `ui_tree_domain_topology` only the rows this render
    /// materialised, never the container's full `TreeWindow::total` — see that function's own note.
    ///
    /// 🕹️ Materialized owned BEFORE the mutable walk — same "clone owned before the field-wise
    /// destructure" reasoning `dispatch_typed_command_inner` already uses: `pending_presence` is
    /// WRITTEN inside the walk while `interaction_hover`/`peer_presence` are only ever READ by
    /// it, so a snapshot avoids an aliasing conflict against `&mut self`.
    #[cfg(test)]
    pub(crate) async fn stamp_and_cache_interaction_ui(&mut self, tree: &ComponentTree, state: &protocol::InteractionState, body_key: &str) -> UiAssemblyResult<()> {
        OriginalRenderInteractionStamp { registry: &self.registry, interaction_hover: &self.interaction_hover, peer_presence: &self.peer_presence, own_color: self.own_color, local_interaction_authority: &mut self.local_interaction_authority, interaction_ui_topology: &mut self.interaction_ui_topology, pending_presence: &mut self.pending_presence, presence_marked_keys: &mut self.presence_marked_keys }.stamp_and_cache_interaction_ui(tree,state,body_key).await
    }

    pub(super) async fn stamp_pending_render_interaction_ui(&mut self, state: &protocol::InteractionState, body_key: &str) -> UiAssemblyResult<()> {
        let tree = self.pending_render_tree.as_ref().ok_or_else(||ui_assembly_error("render.source-absent"))?;
        OriginalRenderInteractionStamp { registry: &self.registry, interaction_hover: &self.interaction_hover, peer_presence: &self.peer_presence, own_color: self.own_color, local_interaction_authority: &mut self.local_interaction_authority, interaction_ui_topology: &mut self.interaction_ui_topology, pending_presence: &mut self.pending_presence, presence_marked_keys: &mut self.presence_marked_keys }.stamp_and_cache_interaction_ui(tree,state,body_key).await
    }

    pub(super) async fn build_artifact_reserved_action_job(&mut self, action: &str, args: Option<&DslValue>, raw_wire: Vec<u8>, meta: &ActionMeta) -> Result<Option<(ArtifactReservedToolJob, ArtifactToolCompletion<A>)>, Fault> {
        self.refresh_cache().await?;
        let (snapshot, config, history) = self.command_cache_inputs();
        let parent_document_id = self.store.envelope().id.clone();
        let canonical_base_revision = self.store.content_revision();
        let completion = ArtifactToolCompletion::<A>::new();
        let request = ArtifactReservedToolJobRequest {
                retained:self.mounted_policy.maintenance,
            controller_id: self.tool_job_controller_id.clone(),
            tool_id: action.to_string(),
            payload_schema_id: self.qualified_tool_proof(action)?.schema_id(),
            contract: self.qualified_tool_proof(action)?.contract(),
            app_instance_id: meta.instance_id,
            ui_axes: meta.view_state.as_ref().map(|view| (view.locale, view.terminology)),
            parent_document_id,
            canonical_base_revision,
            authoring_seed: self.authoring_seed(&meta.actor),
            snapshot,
            snapshot_read: self.store.snapshot_read().map_err(FaultFrom::into_fault)?,
            config,
            history,
            children: ChildContentView::clone(&self.child_content_root),
            raw_wire,
            input: ArtifactReservedToolInput::Action { args: args.cloned(), interaction: self.interaction_selection_snapshot(), hover: self.interaction_hover.clone() },
            completion: completion.clone(),
        };
        Ok(A::build_reserved_tool_job(request)?.map(|job| (job, completion)))
    }

    pub(super) async fn build_artifact_reserved_media_job(&mut self, port: &str, media: Media, meta: &ActionMeta) -> Result<(ArtifactReservedToolJob, ArtifactToolCompletion<A>), Fault> {
        let action = "import-media";
        self.refresh_cache().await?;
        let (snapshot, config, history) = self.command_cache_inputs();
        let completion = ArtifactToolCompletion::<A>::new();
        let request = ArtifactReservedToolJobRequest {
                retained:self.mounted_policy.maintenance,
            controller_id: self.tool_job_controller_id.clone(),
            tool_id: action.to_string(),
            payload_schema_id: self.qualified_tool_proof(action)?.schema_id(),
            contract: self.qualified_tool_proof(action)?.contract(),
            app_instance_id: meta.instance_id,
            ui_axes: meta.view_state.as_ref().map(|view| (view.locale, view.terminology)),
            parent_document_id: self.store.envelope().id.clone(),
            canonical_base_revision: self.store.content_revision(),
            authoring_seed: self.authoring_seed(&meta.actor),
            snapshot,
            snapshot_read: self.store.snapshot_read().map_err(FaultFrom::into_fault)?,
            config,
            history,
            children: ChildContentView::clone(&self.child_content_root),
            raw_wire: Vec::new(),
            input: ArtifactReservedToolInput::Media { port: port.to_string(), media },
            completion: completion.clone(),
        };
        let job = A::build_reserved_tool_job(request)?
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-reserved-builder"), format!("media port '{port}' is registered but has no concrete resumable importer")))?;
        Ok((job, completion))
    }

    pub(super) async fn receive_owned_media_poll(&mut self,operation_id:u64,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<ArtifactMediaExportPoll>,semio_framework_job::OriginalTurnFailure>{
        if self.receive_media_submission_refusal(cx)?||cx.should_yield(){return Ok(None)}
        match self.drive_owned_media_export(operation_id,cx).await{
            Ok(poll)=>Ok(Some(poll)),
            Err(crate::media_operation_refusal::MediaOperationFailure::Original(failure))=>Err(failure),
            Err(crate::media_operation_refusal::MediaOperationFailure::Domain(source))=>self.retain_media_submission_refusal(source,cx)
        }
    }

    pub(crate) fn render_failure_terminal_is_empty(&self)->bool{self.pending_render_tree.is_none()&&self.pending_render_tree_close.is_none()&&self.pending_render_tree_identity.is_none()&&self.pending_render_failure.is_none()&&self.pending_render_close_failure.is_none()&&self.pending_render_compound.is_none()&&self.pending_render_retirement.is_none()&&self.pending_render_identity.is_none()}
    pub(crate) fn render_failure_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.pending_render_tree.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        if let Some(tree)=self.pending_render_tree_close.as_ref(){return tree.next_demand(body)}
        if self.pending_render_close_failure.is_some()||self.pending_render_compound.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        if let Some(owner)=self.pending_render_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.selected_demand(body)}}
        if self.pending_render_failure.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        Ok(Default::default())
    }
    pub(crate) fn close_render_failure(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{
        let before=cx.retained_progress();
        if !self.prepare_render_closure_failure(cx)?{return Ok(PluginLifecycleStep::Progress(original_render_posted_since(before,cx.retained_progress())?))}
        match self.close_pending_render_tree_on_original(cx){Ok(Some(true))=>{},Ok(_)=>return Ok(PluginLifecycleStep::Progress(original_render_posted_since(before,cx.retained_progress())?)),Err(failure)=>{self.retain_render_closure_failure(failure,cx);return Ok(PluginLifecycleStep::Progress(original_render_posted_since(before,cx.retained_progress())?))}}
        let demand=self.render_failure_demands(grant.maximum_copy_bytes)?;
        semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
            if let Some(owner)=self.pending_render_retirement.as_mut(){
                if owner.terminal_is_empty(){drop(self.pending_render_retirement.take());self.pending_render_identity=None;self.pending_render_tree_identity=None;return Ok((PluginLifecycleStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}),true))}
                let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.step(grant))?;return Ok((PluginLifecycleStep::Progress(step.progress()),false));
            }
            if let Some(source)=self.pending_render_failure.take(){
                match semio_framework_value::retirement::controlled::ControlledRetirement::new(source){Ok(owner)=>{*self.pending_render_retirement=Some(owner);Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))},Err((failure,source))=>{*self.pending_render_failure=Some(source);Err(failure.into())}}
            }else{self.pending_render_identity=None;self.pending_render_tree_identity=None;Ok((PluginLifecycleStep::Complete(Default::default()),true))}
        })
    }

    fn close_pending_render_tree_on_original(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<bool>,semio_framework_job::OriginalTurnFailure>{
        if let Some(identity)=self.pending_render_tree_identity.as_mut(){identity.validate(cx)?;}
        if let Some(tree)=self.pending_render_tree.as_mut(){
            if let Some(resources)=tree.native_resources.as_mut(){
                if resources.close_step_on_original(cx)?!=Some(true){return Ok(None)}
                tree.native_resources=None;
                return Ok(Some(false));
            }
            let grant=cx.retained_grant();
            if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
            let tree=self.pending_render_tree.take().expect("original tree source remains stationary until funded close donation");
            *self.pending_render_tree_close=Some(semio_framework_ui_contract::OriginalBuiltTreeRetirement::new(tree.root));
            self.pending_render_tree_ready=false;
            cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;
            cx.consume_fuel(1);
            return Ok(Some(false));
        }
        if let Some(body)=self.pending_render_tree_close.as_mut(){
            if !body.terminal_is_empty(){
                let demand=body.next_demand(cx.retained_grant().maximum_copy_bytes)?;
                semio_framework_job::close_original_retained_step(cx,demand,|grant|body.step(grant))?;
                return Ok(Some(false));
            }
            let grant=cx.retained_grant();
            if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
            drop(self.pending_render_tree_close.take());
            cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;
            cx.consume_fuel(1);
            return Ok(Some(false));
        }
        self.pending_render_tree_ready=false;
        Ok(Some(true))
    }

    pub(super) fn retain_pending_render_tree(&mut self,tree:ComponentTree){
        assert!(self.pending_render_tree.is_none()&&self.pending_render_tree_identity.is_some(),"original render tree destination must be reserved before production");
        self.pending_render_tree=std::mem::ManuallyDrop::new(Some(tree));
        self.pending_render_tree_ready=false;
    }

    pub(super) fn retain_render_failure<T>(&mut self,source:crate::OriginalRenderFailure,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<T>,semio_framework_job::OriginalTurnFailure>{
        if self.pending_render_failure.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original pending render source cannot be overwritten").into())}
        *self.pending_render_failure=Some(source);self.pending_render_identity=Some((cx.operation(),cx.generation()));let _=self.receive_render_failure(cx)?;Ok(None)
    }

    fn retain_render_closure_failure(&mut self,failure:semio_framework_job::OriginalTurnFailure,cx:&semio_framework_job::StepContext<'_>){
        assert!(self.pending_render_compound.is_none()&&self.pending_render_close_failure.is_none(),"original render closure failure cannot overwrite a retained cause");
        if self.pending_render_failure.is_some(){*self.pending_render_close_failure=Some(failure);}else{*self.pending_render_failure=Some(crate::OriginalRenderFailure::Original(failure));self.pending_render_identity=Some((cx.operation(),cx.generation()));}
    }

    fn prepare_render_closure_failure(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.pending_render_compound.is_some(){
            if let Some(original)=semio_framework_job::admit_original_domain_failure(&mut self.pending_render_compound,cx)?{assert!(self.pending_render_failure.is_none());*self.pending_render_failure=Some(crate::OriginalRenderFailure::Original(original));}
            return Ok(self.pending_render_compound.is_none());
        }
        if self.pending_render_close_failure.is_none(){return Ok(true)}
        if !matches!(self.pending_render_failure.as_ref(),Some(crate::OriginalRenderFailure::Original(_))){
            if let Some(original)=semio_framework_job::admit_original_domain_failure(&mut self.pending_render_failure,cx)?{*self.pending_render_failure=Some(crate::OriginalRenderFailure::Original(original));}
            return Ok(false);
        }
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
        let Some(crate::OriginalRenderFailure::Original(initial))=self.pending_render_failure.take()else{unreachable!()};
        let closure=self.pending_render_close_failure.take().expect("original render closure cause remains stationary");
        *self.pending_render_compound=Some(semio_framework_job::OriginalClosureFailure{initial,closure});
        cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(false)
    }

    pub(super) fn receive_render_failure(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.pending_render_failure.is_none()&&self.pending_render_compound.is_none()&&self.pending_render_close_failure.is_none(){return Ok(false)}
        if !self.prepare_render_closure_failure(cx)?{return Ok(true)}
        match self.close_pending_render_tree_on_original(cx){Ok(Some(true))=>{},Ok(_)=>return Ok(true),Err(failure)=>{self.retain_render_closure_failure(failure,cx);return Ok(true)}}
        if matches!(self.pending_render_failure.as_ref(),Some(crate::OriginalRenderFailure::Original(_))){
            let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(true)}
            let Some(crate::OriginalRenderFailure::Original(failure))=self.pending_render_failure.take() else{unreachable!()};
            self.pending_render_identity=None;self.pending_render_tree_identity=None;
            cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Err(failure);
        }
        if self.pending_render_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"pending render domain requires its unchanged initiating source identity").into())}
        if let Some(failure)=semio_framework_job::admit_original_domain_failure(&mut self.pending_render_failure,cx)?{self.pending_render_identity=None;self.pending_render_tree_identity=None;return Err(failure)}
        Ok(true)
    }

    pub(crate) fn media_submission_refusal_terminal_is_empty(&self)->bool{self.media_download_preparation.is_none()&&self.media_export_preparation.is_none()&&self.media_submission_refusal.is_none()&&self.media_submission_refusal_retirement.is_none()&&self.media_submission_refusal_identity.is_none()}
    pub(crate) fn media_submission_refusal_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        use semio_framework_value::retirement::RetireOwned;
        if let Some(original)=self.media_download_preparation.as_ref(){return original.retirement_demands(body)}
        if let Some(original)=self.media_export_preparation.as_ref(){
            if original.producer.is_some()||original.body.is_some()||original.request.is_some()||original.request_close.is_some()||original.parts.is_some()||original.parts_close.is_some()||original.handle.is_some()||original.handle_close.is_some()||original.metadata.is_some()||original.metadata_close.is_some()||original.snapshot.is_some()||original.snapshot_lease.is_some()||original.completion.is_some()||original.completion_close.is_some()||original.credit.is_some()||original.credit_close.is_some()||original.chunks.is_some()||original.chunks_close.is_some()||original.operation.is_some(){return original.retirement_demands(body)}
            if let Some(owner)=original.retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<crate::tool_cancellation::ToolCancellationLeaseRetirement>(),depth:1,..Default::default()})}else{owner.retirement_demands(&self.tool_cancellations,body)}}
            if original.lease.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<ToolCancellationLease>(),capacity_bytes:size_of::<crate::tool_cancellation::ToolCancellationLeaseRetirement>(),depth:1,..Default::default()})}
            if let Some(owner)=original.snapshot_retention.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.retirement_demands(body)}}
            if !original.snapshot_source.terminal_is_empty(){return original.snapshot_source.retirement_demands(body)}
            return Ok(RetirementDemand{copy_bytes:size_of::<MediaExportPreparation<A>>(),depth:1,..Default::default()})
        }
        if let Some(owner)=self.media_submission_refusal_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of_val(&*self.media_submission_refusal_retirement),depth:1,..Default::default()})}else{owner.selected_demand(body)}}
        if self.media_submission_refusal.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        Ok(Default::default())
    }
    pub(crate) fn close_media_submission_refusal(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{
        if let Some(original)=self.media_download_preparation.as_ref(){self.snapshot_retirements.get(original.operation_id).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"pending download lost its original snapshot source"))?.validate_retirement(cx)?}
        if let Some(original)=self.media_export_preparation.as_ref(){
            if let Some(retention)=original.snapshot_retention.as_ref(){if !retention.terminal_is_empty(){retention.validate_retirement(cx)?}}
            else if let Some(retention)=self.snapshot_retirements.get(original.key.operation_id.0){retention.validate_retirement(cx)?}
            else if !original.snapshot_source.terminal_is_empty(){original.snapshot_source.validate_retirement(cx)?}
            else if original.source_identity!=(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"empty media preparation footer requires its unchanged original receiving source").into())}
        }
        let demand=self.media_submission_refusal_demands(grant.maximum_copy_bytes)?;
        semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
            if let Some(original)=self.media_download_preparation.as_mut(){if let Some(step)=original.close_step(grant)?{return Ok((PluginLifecycleStep::Progress(step.progress()),false))}self.media_download_preparation.take();return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}),false))}
            if let Some(original)=self.media_export_preparation.as_mut(){
                if let Some(progress)=original.close_material_step(cx,grant)?{return Ok((PluginLifecycleStep::Progress(progress),false))}
                let step=if let Some(owner)=original.retirement.as_mut(){if owner.terminal_is_empty(){RetainedCloneStep::Progress(crate::tool_cancellation::original_lease_cursor_release(&mut original.retirement,grant)?)}else{owner.close_step(&self.tool_cancellations,grant)?}}else if original.lease.is_some(){let(owner,progress)=crate::tool_cancellation::original_lease_cursor_admit(&mut original.lease,grant)?;original.retirement=owner;RetainedCloneStep::Progress(progress)}
                else if let Some(owner)=original.snapshot_retention.as_mut(){if owner.terminal_is_empty(){original.snapshot_retention.take();RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}else{return owner.close_step(cx,grant).map(|step|(PluginLifecycleStep::Progress(step.progress()),false))}}
                else if !original.snapshot_source.terminal_is_empty(){return original.snapshot_source.close_step(cx).map(|step|(PluginLifecycleStep::Progress(step.progress()),false))}
                else{self.media_export_preparation.take();RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<MediaExportPreparation<A>>(),..Default::default()})};
                return Ok((PluginLifecycleStep::Progress(step.progress()),false))
            }
            if let Some(owner)=self.media_submission_refusal_retirement.as_mut(){
                if owner.terminal_is_empty(){drop(self.media_submission_refusal_retirement.take());self.media_submission_refusal_identity=None;return Ok((PluginLifecycleStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}),true))}
                let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.step(grant))?;return Ok((PluginLifecycleStep::Progress(step.progress()),false));
            }
            if let Some(source)=self.media_submission_refusal.take(){
                match semio_framework_value::retirement::controlled::ControlledRetirement::new(source){Ok(owner)=>{*self.media_submission_refusal_retirement=Some(owner);Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}),false))},Err((failure,source))=>{*self.media_submission_refusal=Some(source);Err(failure.into())}}
            }else{self.media_submission_refusal_identity=None;Ok((PluginLifecycleStep::Complete(Default::default()),true))}
        })
    }

    pub(super) fn retain_media_submission_refusal<T>(&mut self,source:impl Into<crate::media_operation_refusal::MediaOperationSource>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<T>,semio_framework_job::OriginalTurnFailure>{
        if self.media_submission_refusal.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original pending media source cannot be overwritten").into())}
        *self.media_submission_refusal=Some(crate::media_operation_refusal::MediaOperationRefusal{source:source.into()});self.media_submission_refusal_identity=Some((cx.operation(),cx.generation()));let _=self.receive_media_submission_refusal(cx)?;Ok(None)
    }

    pub(super) fn receive_media_submission_refusal(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.media_submission_refusal.is_none(){return Ok(false)}
        if self.media_submission_refusal_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"pending media domain requires its unchanged initiating source identity").into())}
        if let Some(failure)=semio_framework_job::admit_original_domain_failure(&mut self.media_submission_refusal,cx)?{self.media_submission_refusal_identity=None;return Err(failure)}
        Ok(true)
    }

    pub(super) async fn receive_owned_media_export(&mut self,port:&str,app_instance_id:u32,receiver:Option<&mut Option<ArtifactMediaExportHandle>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.receive_media_submission_refusal(cx)?||cx.should_yield(){return Ok(false)}
        match self.submit_owned_media_export(port,app_instance_id,receiver,cx).await{
            Ok(handle)=>Ok(handle),
            Err(crate::media_operation_refusal::MediaOperationFailure::Domain(source))=>self.retain_media_submission_refusal::<()>(source,cx).map(|_|false),
            Err(crate::media_operation_refusal::MediaOperationFailure::Original(failure))=>Err(failure)
        }
    }

    async fn submit_owned_media_export(&mut self,port:&str,app_instance_id:u32,mut receiver:Option<&mut Option<ArtifactMediaExportHandle>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,crate::media_operation_refusal::MediaOperationFailure>{
        if self.media_export_preparation.is_none(){if let Some(handle)=receiver.as_deref().and_then(|slot|slot.as_ref()){
            let active=self.media_exports.get(handle.operation_id.0).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"received media handle lost its live operation"))?;
            if active.handle.original_loan()!=handle.original_loan()||active.port!=port{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"received media handle belongs to another operation").into())}
            self.snapshot_retirements.get(handle.operation_id.0).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"received media handle lost its original source"))?.validate(cx)?;return Ok(true)
        }}
        let source_identity=(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?);let port_identity=(port.as_ptr()as usize,port.len());let grant=cx.retained_grant();
        if let Some(original)=self.media_export_preparation.as_ref(){if original.source_identity!=source_identity||original.port_identity!=port_identity||original.key.app_instance_id!=app_instance_id{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media submission requires its unchanged original source and borrowed port").into())}}
        if cx.should_yield()||cx.is_cancelled(){return Ok(false)}
        if self.media_export_preparation.is_none(){
            if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<size_of::<MediaExportPreparation<A>>(){return Ok(false)}
            let Some(slot)=(0..ARTIFACT_LIVE_OUTPUT_SLOTS).find(|slot|self.media_exports.slot_is_vacant(*slot)&&self.media_closures.slot_is_vacant(*slot)&&self.snapshot_retirements.slot_is_vacant(*slot))else{return Ok(false)};
            let revision=self.store.content_revision_now();let operation_id=semio_framework_job::allocate_operation_id_in_slot(ARTIFACT_LIVE_OUTPUT_SLOTS as u64,slot as u64);let key=ToolOperationKey{app_instance_id,document:ArtifactDocumentAuthority(app_instance_id),operation_id,base_revision:semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap())),generation:semio_framework_job::Generation(self.store.generation_now())};
            *self.media_export_preparation=Some(MediaExportPreparation{canonical_revision:revision,metadata:ManuallyDrop::new(Some(std::array::from_fn(|_|Vec::new()))),metadata_stage:0,metadata_close:ManuallyDrop::new(None),request_close:ManuallyDrop::new(None),parts:ManuallyDrop::new(None),parts_close:ManuallyDrop::new(None),handle:ManuallyDrop::new(None),handle_close:ManuallyDrop::new(None),operation:None,body_ready:false,operation_bound:false,assembled:false,published:false,snapshot_source:semio_framework_plugin_ownership::store_lifecycle::ArtifactMediaSnapshotSource::empty(),snapshot:ManuallyDrop::new(None),snapshot_lease:ManuallyDrop::new(None),snapshot_retention:ManuallyDrop::new(None),request:ManuallyDrop::new(None),body:ManuallyDrop::new(None),producer:ManuallyDrop::new(None),source_identity,port_identity,key,lease:None,retirement:None,completion:None,completion_close:None,credit:None,credit_close:None,chunks:None,chunks_close:None});
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<MediaExportPreparation<A>>(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)
        }
        let original=self.media_export_preparation.as_mut().unwrap();if original.retirement.is_some(){return Ok(false)}
        if original.published{
            self.snapshot_retirements.get(original.key.operation_id.0).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"published media lost its original supplier"))?.validate(cx)?;
            let Some(receiver)=receiver.as_deref_mut()else{return Ok(true)};
            if original.handle.is_some(){let _=ArtifactMediaExportHandle::receive_original(&mut original.handle,receiver,cx)?;return Ok(false)}
            if receiver.is_none(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"published media lost its retained receiving handle").into())}
            let bytes=size_of::<MediaExportPreparation<A>>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
            self.media_export_preparation.take();
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(true)
        }
        if self.store.content_revision_now()!=original.canonical_revision||self.store.generation_now()!=original.key.generation.0{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media preparation requires its exact captured canonical revision and generation").into())}
        let(id,registration)=self.app_tool_registrations.iter().find(|(id,_)|id.strip_prefix("export-media:")==Some(port)).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"media output lacks its published application factory"))?;
        if registration.owner!=crate::tool_contract::ToolOwnerWitness::of::<A>()||registration.key.tool_id!=*id{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media output requires its exact registered concrete application owner").into())}let contract=registration.contract;
        if original.snapshot.is_none()&&!original.assembled{
            if original.snapshot_source.ready(){let _=original.snapshot_source.handoff(&mut original.snapshot,&mut original.snapshot_lease,&mut original.snapshot_retention,cx)?;return Ok(false)}
            let factory=self.store.owned_snapshot_retirement_factory()?;let store=&self.store;let _=original.snapshot_source.admit_step(factory,||store.snapshot_owner(),cx)?;return Ok(false)
        }
        if let Some(retention)=original.snapshot_retention.as_ref(){retention.validate(cx)?}else{self.snapshot_retirements.get(original.key.operation_id.0).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media producer lost its retained original source and installed supplier"))?.validate(cx)?}
        if original.lease.is_none(){let Some(lease)=self.tool_cancellations.begin(&original.key,cx)?else{return Ok(false)};original.lease=Some(lease);return Ok(false)}
        if !original.assembled{
            if original.completion.is_none(){let _=ArtifactMediaExportCompletion::original_birth_admit(&mut original.completion,cx)?;return Ok(false)}
            if original.credit.is_none(){let _=ArtifactMediaExportCredit::original_birth_admit(contract.max_output_bytes,&mut original.credit,cx)?;return Ok(false)}
            if original.chunks.is_none(){let _=ArtifactOutputChunks::original_birth_admit(contract.max_output_bytes,&mut original.chunks,cx)?;return Ok(false)}
            if original.metadata_stage<6{
                let index=original.metadata_stage;let input=match index{0=>id.as_bytes(),1|3|5=>self.store.envelope().id.as_bytes(),2|4=>port.as_bytes(),_=>unreachable!()};let output=&mut original.metadata.as_mut().unwrap()[index];
                if output.capacity()==0&&!input.is_empty(){if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<input.len(){return Ok(false)}output.try_reserve_exact(input.len()).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"media original metadata allocation refused"))?;cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:output.capacity(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
                if output.len()<input.len(){let count=(input.len()-output.len()).min(grant.maximum_copy_bytes);if grant.maximum_items==0||grant.maximum_depth==0||count==0{return Ok(false)}let start=output.len();output.extend_from_slice(&input[start..start+count]);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
                if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<size_of::<usize>(){return Ok(false)}original.metadata_stage+=1;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<usize>(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)
            }
            let copy=size_of::<ArtifactMediaExportJobRequest<A>>()+size_of::<crate::media_export::MediaExportOwnedParts>()+size_of::<ArtifactMediaExportHandle>()+size_of::<ArtifactMediaExportCredit>()+size_of::<ArtifactOutputChunks>()+size_of::<ArtifactMediaExportCompletion>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return Ok(false)}
            let text=|bytes:Vec<u8>|String::from_utf8(bytes).expect("original source metadata retains its complete UTF-8 bytes");let fields=original.metadata.as_mut().unwrap();let tool_id=text(std::mem::take(&mut fields[0]));let parent_document_id=text(std::mem::take(&mut fields[1]));let request_port=text(std::mem::take(&mut fields[2]));let parts_document=text(std::mem::take(&mut fields[3]));let parts_port=text(std::mem::take(&mut fields[4]));let published_document=text(std::mem::take(&mut fields[5]));
            let operation=semio_framework_job::Operation::new(original.key.operation_id,original.key.base_revision,original.key.generation,original.key.operation_id.0^original.key.base_revision.0);
            let completion=original.completion.take().unwrap();let output_credit=original.credit.take().unwrap();let output_chunks=original.chunks.take().unwrap();
            *original.parts=Some(crate::media_export::MediaExportOwnedParts{handle:ArtifactMediaExportHandle{app_instance_id,parent_document_id:parts_document,operation_id:operation.operation,base_revision:operation.base_revision,generation:operation.generation},port:parts_port,retained_result:None,output_credit:output_credit.clone(),output_chunks:output_chunks.clone(),completion:completion.clone()});
            *original.handle=Some(ArtifactMediaExportHandle{app_instance_id,parent_document_id:published_document,operation_id:operation.operation,base_revision:operation.base_revision,generation:operation.generation});
            *original.request=Some(ArtifactMediaExportJobRequest{source_identity:(cx.operation(),cx.generation()),retained:cx.retained_grant(),operation,tool_id,parent_document_id,canonical_base_revision:original.canonical_revision,snapshot:original.snapshot.take().unwrap(),snapshot_close:original.snapshot_lease.take().unwrap(),port:request_port,output_credit,output_chunks,completion});original.operation=Some(operation);original.assembled=true;
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
        }
        if !original.body_ready{original.body_ready=A::build_media_export_job(&mut original.request,&mut original.body,cx)?;if original.body_ready&&(original.request.is_some()||original.body.is_none()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media factory readiness must retain one actual body and consume its exact request").into())}return Ok(false)}
        if original.producer.is_none(){let _=ArtifactReservedToolJob::admit_boxed_original(&mut original.body,&mut original.producer,cx)?;return Ok(false)}
        if !original.operation_bound{original.operation_bound=original.producer.as_mut().unwrap().bind_original_operation(&mut original.operation,cx)?;return Ok(false)}
        let bytes=size_of::<crate::media_export::ActiveMediaExport>()+size_of::<ArtifactSnapshotCloseRetention>()+size_of::<bool>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
        let operation=original.request.as_ref().map_or_else(||semio_framework_job::Operation::new(original.key.operation_id,original.key.base_revision,original.key.generation,original.key.operation_id.0^original.key.base_revision.0),|request|request.operation);let operation_id=operation.operation;
        let retention=original.snapshot_retention.take().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media publication lost its actual snapshot supplier"))?;
        self.snapshot_retirements.insert_admitted(operation_id.0,retention);
        let active=ActiveMediaExport{parts:ManuallyDrop::new(original.parts.take()),parts_retirement:ManuallyDrop::new(None),cancellation_retirement:None,operation,contract,session:None,session_rejected:Some(semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(original.producer.take().unwrap(),(cx.operation(),cx.generation()))),outcome_pending:false,resume_pending:false,lease:ManuallyDrop::new(original.lease.take()),checkpoint_available:false,applied_progress:0,download_pending:false,close_stage:ActiveMediaExportCloseStage::Live};
        self.media_exports.insert_admitted(operation_id.0,active);self.current_media_export=Some(operation_id.0);original.published=true;
        cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);
        Ok(false)
    }

    fn quarantine_media_snapshot(&self, operation_id: u64, active: &ActiveMediaExport) -> Result<(), MediaError> {
        self.snapshot_retirements.get(operation_id).map(|_| ()).ok_or_else(|| MediaError::Payload(active.port.clone(), "media export lost its pre-admitted app-owned snapshot retirement authority".into()))
    }

    fn finish_media_poll(&mut self, operation_id: u64, active: ActiveMediaExport, poll: ArtifactMediaExportPoll) -> Result<ArtifactMediaExportPoll,crate::media_operation_refusal::MediaOperationFailure> {
        if let Err(error) = self.quarantine_media_snapshot(operation_id, &active) {
            debug_assert!(self.media_exports.can_insert(operation_id));
            self.media_exports.insert_admitted(operation_id, active);
            return Err(error.into());
        }
        debug_assert!(self.media_closures.can_insert(operation_id));
        self.media_closures.insert_admitted(operation_id, active);
        Ok(poll)
    }

    async fn drive_owned_media_export(&mut self, operation_id:u64,cx:&mut semio_framework_job::StepContext<'_>) -> Result<ArtifactMediaExportPoll,crate::media_operation_refusal::MediaOperationFailure> {
        self.snapshot_retirements.get(operation_id).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media step lost its original retained source and snapshot supplier"))?.validate(cx)?;
        if !self.media_closures.can_insert(operation_id) {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media close authority is saturated or already owns this operation").into());
        }
        if self.media_exports.get(operation_id).is_none() {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media registry lost its admitted operation").into());
        }
        let mut active = self.media_exports.remove(operation_id).expect("checked exact media export owner");
        let grant=cx.retained_grant();
        if active.resume_pending || active.outcome_pending {
            if let Err(failure)=active.retire_nonterminal_outcome(cx){self.media_exports.insert_admitted(operation_id,active);return Err(failure.into());}
            let poll=active.running_poll();self.media_exports.insert_admitted(operation_id,active);return Ok(poll);
        }
        if let Some(rejected) = active.session_rejected.as_mut() {
            if rejected.terminal_is_empty() {
                if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<size_of_val(rejected){let poll=active.running_poll();self.media_exports.insert_admitted(operation_id,active);return Ok(poll);}
                let cancelled=cx.is_cancelled()||active.lease.as_ref().expect("live media lease").token.is_cancelled_now();drop(active.session_rejected.take());
                return self.finish_media_poll(operation_id,active,if cancelled{ArtifactMediaExportPoll::Cancelled}else{ArtifactMediaExportPoll::Failed("media export mounted session capacity is exhausted".into())});
            }
            if rejected.admission_pending()&&!cx.is_cancelled()&&!active.lease.as_ref().expect("live media lease").token.is_cancelled_now(){
                let config=semio_framework_job_worker_authority::BatchDriveConfig{retained:grant,site:"plugin_media_export_submission",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:cx.fuel_remaining(),step_budget_us:0};
                let result=rejected.admit_pending(config,cx);
                match result{Ok(Some(session))=>{active.session=Some(session);drop(active.session_rejected.take());},Ok(None)=>{},Err(failure)=>{self.media_exports.insert_admitted(operation_id,active);return Err(failure.into());}}
            }else{
                let demand=rejected.retirement_demands(grant.maximum_copy_bytes)?;
                let result=crate::document_ingress::original_rejection_lifecycle(cx,demand,grant,"media rejection awaits its original worker",|cx|{let step=rejected.close_step(cx)?;Ok((step,rejected.terminal_is_empty()))});
                if let Err(failure)=result{self.media_exports.insert_admitted(operation_id,active);return Err(failure.into());}
            }
            let poll = active.running_poll();
            self.media_exports.insert_admitted(operation_id, active);
            return Ok(poll);
        }
        if grant.maximum_items==0||grant.maximum_depth==0{let poll=active.running_poll();self.media_exports.insert_admitted(operation_id,active);return Ok(poll);}
        let pump = active.session.as_mut().map(|session| session.pump_one_for_interactive_turn(cx));
        match pump {
            Some(Ok(semio_framework_job_worker::WorkerJobPoll::Outcome | semio_framework_job_worker::WorkerJobPoll::Terminal)) => {}
            Some(Err(failure))=>{self.media_exports.insert_admitted(operation_id,active);return Err(failure.into());},
            Some(Ok(_)) => {
                let poll = active.running_poll();
                self.media_exports.insert_admitted(operation_id, active);
                return Ok(poll);
            }
            None => return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed("media export lost its mounted session".into())),
        }
        let receipt = active.session.as_mut().and_then(semio_framework_job_worker::MountedWorkerJobSession::take_checked_out_retained_step_receipt);
        if !receipt.is_some_and(|(issued, progress)| progress.fits(issued) && progress.fits(grant)) {
            return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed("media export mounted worker receipt is absent or exceeded its issued grant".into()));
        }
        enum SeenMediaOutcome { Progress, Checkpoint { applied_progress: u64, state_len: usize }, Complete { output_len: usize }, Cancelled, Fault(String), Lost }
        let seen = match active.session.as_ref().map(semio_framework_job_worker::MountedWorkerJobSession::checked_out_outcome) {
            Some(Ok(Some(semio_framework_job::JobOutcomeView::Yield { .. } | semio_framework_job::JobOutcomeView::PreviewReady { .. }))) => SeenMediaOutcome::Progress,
            Some(Ok(Some(semio_framework_job::JobOutcomeView::CheckpointReady { state, applied_progress, .. }))) => SeenMediaOutcome::Checkpoint { applied_progress, state_len: state.len() },
            Some(Ok(Some(semio_framework_job::JobOutcomeView::Complete { output, .. }))) => SeenMediaOutcome::Complete { output_len: output.map_or(0, semio_framework_job::RetainedJobPayload::len) },
            Some(Ok(Some(semio_framework_job::JobOutcomeView::Cancelled { .. }))) => SeenMediaOutcome::Cancelled,
            Some(Ok(Some(semio_framework_job::JobOutcomeView::Fault { detail, .. }))) => SeenMediaOutcome::Fault(detail.single_page().and_then(|bytes| std::str::from_utf8(bytes).ok()).filter(|detail| !detail.is_empty()).unwrap_or("media export job faulted").to_string()),
            _ => SeenMediaOutcome::Lost,
        };
        match seen {
            SeenMediaOutcome::Progress => {
                active.outcome_pending = true;
                if let Err(failure) = active.retire_nonterminal_outcome(cx) {self.media_exports.insert_admitted(operation_id,active);return Err(failure.into());}
                let poll = active.running_poll();
                self.media_exports.insert_admitted(operation_id, active);
                Ok(poll)
            }
            SeenMediaOutcome::Checkpoint { applied_progress, state_len } => {
                if applied_progress < active.applied_progress || state_len > active.contract.max_output_bytes {
                    return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed("media export checkpoint is non-monotone or oversized".into()));
                }
                active.applied_progress = applied_progress;
                active.checkpoint_available = true;
                active.outcome_pending = true;
                if let Err(failure) = active.retire_nonterminal_outcome(cx) {self.media_exports.insert_admitted(operation_id,active);return Err(failure.into());}
                let poll = active.running_poll();
                self.media_exports.insert_admitted(operation_id, active);
                Ok(poll)
            }
            SeenMediaOutcome::Complete { output_len } => {
                if output_len > active.contract.max_output_bytes || active.lease.as_ref().expect("live media lease").is_cancelled().await {
                    return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed("media export was cancelled or its commit candidate exceeded the output cap".into()));
                }
                let live_revision_bytes = self.store.content_revision();
                let live_revision = semio_framework_job::RevisionId(u64::from_be_bytes(live_revision_bytes[..8].try_into().expect("revision lane width")));
                let live_generation = semio_framework_job::Generation(self.store.generation());
                if !matches!(semio_framework_job::validate_commit(&active.operation, live_revision, live_generation), semio_framework_job::CommitValidation::Accepted) {
                    return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed("media export completed against a stale revision or generation".into()));
                }
                if active.retained_result.is_none(){match active.completion.take(){Ok(Some(result))=>active.retained_result=Some(result),Ok(None)=>return self.finish_media_poll(operation_id,active,ArtifactMediaExportPoll::Failed("media export completed without producer output".into())),Err(error)=>return self.finish_media_poll(operation_id,active,ArtifactMediaExportPoll::Failed(error.message))};}
                let result=match active.retained_result.as_ref(){Some(Ok(result))=>result,Some(Err(error))=>{let Some(Err(source))=active.retained_result.take()else{unreachable!("observed exact original media domain result")};self.media_closures.insert_admitted(operation_id,active);return Err(source.into());},None=>return self.finish_media_poll(operation_id,active,ArtifactMediaExportPoll::Failed("media export lost its original completion result".into()))};
                let output_bytes = match active.output_credit.validate_terminal() {
                    Ok(output_bytes) => output_bytes,
                    Err(error) => return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed(error.message)),
                };
                if let Err(error) = validate_media_export_structure(result, &active.output_chunks, output_bytes, active.contract.max_output_bytes) {
                    return self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed(error.message));
                }
                active.lease.as_mut().expect("live media lease").finish_in_place();
                let Some(Ok(result))=active.retained_result.take()else{return self.finish_media_poll(operation_id,active,ArtifactMediaExportPoll::Failed("media export original completion changed during publication".into()));};
                if self.current_media_export == Some(operation_id) {
                    self.current_media_export = None;
                }
                self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Complete(result))
            }
            SeenMediaOutcome::Cancelled => self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Cancelled),
            SeenMediaOutcome::Fault(detail) => self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed(detail)),
            SeenMediaOutcome::Lost => self.finish_media_poll(operation_id, active, ArtifactMediaExportPoll::Failed("media export original outcome changed during publication".into())),
        }
    }

    pub(super) async fn cancel_owned_media_export(&mut self, handle: &ArtifactMediaExportHandle) -> Result<(), MediaError> {
        let operation_id = handle.operation_id.0;
        if let Some(output) = self.segmented_downloads.get(operation_id) {
            if !output.owns_media_export(handle) {
                return Err(MediaError::Payload(handle.parent_document_id.clone(), "media export download handle authority mismatch".into()));
            }
            if !self.segmented_closures.can_insert(operation_id) {
                return Err(MediaError::Payload(handle.parent_document_id.clone(), "media export download cleanup authority is saturated".into()));
            }
            let output = self.segmented_downloads.remove(operation_id).ok_or_else(|| MediaError::Payload(handle.parent_document_id.clone(), "media export download changed before cancellation".into()))?;
            self.segmented_closures.insert_admitted(operation_id,crate::media_export::MediaDownloadRetirement::pending(output));
            return Ok(());
        }
        if !self.media_closures.can_insert(operation_id) {
            return Err(MediaError::Payload(handle.parent_document_id.clone(), "media close authority is saturated or already owns this operation".into()));
        }
        let Some(active) = self.media_exports.get(operation_id) else {
            return Err(MediaError::Payload(handle.parent_document_id.clone(), "unknown or completed media export handle".into()));
        };
        if active.handle != *handle {
            return Err(MediaError::Payload(handle.parent_document_id.clone(), "media export handle authority mismatch".into()));
        }
        self.quarantine_media_snapshot(operation_id, active)?;
        let active = self.media_exports.remove(operation_id).expect("checked exact media export owner");
        self.media_closures.insert_admitted(operation_id, active);
        if self.current_media_export == Some(operation_id) {
            self.current_media_export = None;
        }
        Ok(())
    }
}
