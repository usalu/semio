//! 🖼️ Render side of the mounted artifact instance: composition graph, interaction state, render, media jobs and media export.

use super::*;

use super::docs_plugin_app_impl::{history_changed_event};
use super::jobs_instance_root::{OriginalRenderInteractionStamp};

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
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
        self.child_composition.admit_child_content_publication_span(self.child_composition.children.len())?;
        let parent_id = self.stores.store.envelope().id.clone();
        let parent_dialect: ArtifactDialect = A::DIALECT.into();
        if self.stores.store.envelope().dialect.as_ref() != Some(&parent_dialect) {
            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.group-history-dialect"), "composition history requires the parent's exact declared dialect"));
        }
        let parent_ref = ArtifactRef { artifact_id: parent_id.clone(), dialect: parent_dialect };
        let child_refs: Vec<ArtifactRef> = self.child_composition.children.entries_physical().map(|entry| entry.reference.clone()).collect();

        let mut children: Vec<(&ArtifactRef, &mut M)> = Vec::with_capacity(child_refs.len());
        for (reference, entry) in child_refs.iter().zip(self.child_composition.children.entries_mut_physical()) {
            children.push((reference, &mut entry.member));
        }

        let report = if action == "undo" { CompositionCoordinator::undo_group(&parent_ref, &mut self.stores.store, &mut children, group_id, identity).await } else { CompositionCoordinator::redo_group(&parent_ref, &mut self.stores.store, &mut children, group_id, identity).await };
        drop(children);
        self.history.cache = None;
        for (reference, _) in report.undone.iter().filter(|(reference, _)| reference.artifact_id != parent_id) {
            let Some(key) = self.child_composition.children.keyed_entries().find(|(_, entry)| entry.reference.artifact_id == reference.artifact_id).map(|(key, _)| key.to_key()) else {
                return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.group-history-root"), "group history moved a child without exact immutable-root authority"));
            };
            let publication_generation = self.child_composition.admit_child_content_publication()?;
            self.child_composition.publish_member_content(publication_generation, key.borrowed()).await?;
        }

        let diagnostics: Vec<semio_framework_diagnostic::Diagnostic> = report
            .skipped
            .iter()
            .map(|(reference, error)| {
                semio_framework_diagnostic::Diagnostic::error("composition.group-history.skipped-member", semio_framework_diagnostic::TextSpan::default(), format!("{action} skipped member {} ({error})", reference.artifact_id))
            })
            .collect();

        if report.undone.is_empty() {
            let mut result = ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
            result.diagnostics = diagnostics;
            return Ok(result);
        }

        self.tools.record_command(&mut self.history, action, ActionKind::History, None, None, Vec::new(), None);
        let child_edit_ids: Vec<String> = report.undone.iter().filter(|(reference, _)| reference.artifact_id != parent_id).map(|(_, edit_id)| edit_id.clone()).collect();
        if let Some(last) = self.history.command_log.last_mut() {
            last.child_edit_ids = child_edit_ids;
        }

        let mut result = ToolOperations::<A>::empty_result(action, meta, Vec::new(), vec![history_changed_event().await], UiDirtyScope::Full).await;
        result.diagnostics = diagnostics;
        Ok(result)
    }

    /// 🧾️ `domain=granularity:id,id` for one `InteractionState`'s selection half — the compact witness
    /// the persist round-trip check below compares, so a lost selection names its own domain instead of
    /// reaching the user as a silently empty panel.
    fn interaction_selection_witness(state: &protocol::InteractionState) -> String {
        state.selection.iter().map(|(domain, selection)| format!("{domain}={}:{}", selection.granularity, selection.ids.join(","))).collect::<Vec<_>>().join(";")
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
            protocol::HierarchyProvider::UiTree => Ok(self.interaction.interaction_ui_topology.get(&def.id).cloned().unwrap_or_default()),
            protocol::HierarchyProvider::PathDelimited { delimiter } => Ok(derive_path_delimited_topology(def, delimiter, known_ids).await),
        }
    }

    /// 🗺️ `A::interaction_topology` for the LIVE document generation, built at most once per generation
    /// — see [`Self::interaction_topology_memo`] for why one pick used to build it twice.
    async fn app_interaction_topology(&mut self) -> Result<&protocol::InteractionTopology, Fault> {
        self.refresh_cache().await?;
        let key = (self.stores.store.generation(), self.stores.config_store.generation());
        if self.interaction.interaction_topology_memo.as_ref().map(|(cached, _)| *cached) != Some(key) {
            let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
            let parent_document_id = self.stores.store.envelope().id.clone();
            let (_, snapshot, config, history) = self.history.cache.as_ref().expect("cache refreshed above");
            let doc = ArtifactView::with_render_context(snapshot.as_ref(), history.as_ref(), ChildContentView::clone(&self.child_composition.child_content_root), render_operation, parent_document_id, None).await.with_instance(self.tools.instance_operation_owner.clone());
            let cfg = ConfigView { snapshot: config.as_ref(), window: None };
            let topology = A::interaction_topology(&doc, &cfg).await.map_err(|error| plugin_sdk_fault(error.into_message()))?;
            self.interaction.interaction_topology_memo = Some((key, topology));
        }
        Ok(&self.interaction.interaction_topology_memo.as_ref().expect("memo filled above").1)
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
        let defs: Vec<InteractionDefinition> = self.tools.registry.interactions().await.cloned().collect();
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
        for def in self.tools.registry.interactions().await {
            outlines.push(def.outline().await);
        }
        let topology = self.build_full_interaction_topology(&combined, origin).await?;
        let validated = protocol::validate_state(&outlines, &topology, &combined).await;
        let validated = if origin == InteractionRevalidateOrigin::DocumentChange {
            let declared: Vec<String> = self.tools.registry.interactions().await.map(|def| def.id.clone()).collect();
            let (pruned, overlay, leftover_ids) = interaction_after_document_change_v1(&validated, &declared, self.interaction.interaction_leftover_selection.as_ref(), &self.interaction.interaction_leftover_ids);
            self.interaction.interaction_leftover_selection = overlay;
            self.interaction.interaction_leftover_ids = leftover_ids;
            pruned
        } else {
            validated
        };
        self.interaction.interaction_hover = validated.hover.clone();
        let persisted_before = self.stores.interaction_store.snapshot().unwrap_or_default();
        let persisted = protocol::InteractionState { selection: validated.selection, hover: BTreeMap::new(), active_mode: validated.active_mode, active_granularity: validated.active_granularity };
        let dispatched = Self::interaction_selection_witness(&combined);
        let requested = Self::interaction_selection_witness(&persisted);
        let minted = persisted != persisted_before;
        if minted {
            self.stores.interaction_store.dispatch(ArtifactCommand::ApplyInLane { mutations: vec![InteractionConfigMutation::set_state(persisted.clone())], lane: HistoryLane::Interaction, transaction: None }, identity).await.map_err(|error| error.into_fault())?;
        }
        if origin == InteractionRevalidateOrigin::Pick {
            let readback = Self::interaction_selection_witness(&self.interaction.interaction_selection_snapshot(&self.stores));
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
    pub(crate) async fn revalidate_interaction_on_document_change(&mut self, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        if self.tools.registry.interactions().await.next().is_none() {
            return Ok(());
        }
        self.revalidate_and_persist_interaction_state(self.interaction.interaction_state(&self.stores).await, meta, InteractionRevalidateOrigin::DocumentChange, identity).await?;
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
        let mut state = self.interaction.interaction_selection_snapshot(&self.stores);
        for write in writes {
            let def = self.tools.registry.interaction(&write.domain).await.cloned().ok_or_else(|| plugin_sdk_fault(format!("interaction write names undeclared domain {}", write.domain)))?;
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
        state.hover = self.interaction.interaction_hover.clone();
        let domains: Vec<String> = writes.iter().map(|write| write.domain.clone()).collect();
        let (overlay, leftover_ids) = leftover_after_app_selection_write_v1(self.interaction.interaction_leftover_selection.as_ref(), &self.interaction.interaction_leftover_ids, &state, &domains);
        self.interaction.interaction_leftover_selection = overlay;
        self.interaction.interaction_leftover_ids = leftover_ids;
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
        self.stores.validate_framework_reserved_commit(action, permit).await?;
        let applied = self.apply_interaction_verb(action, args, meta, Some(permit), identity).await?;
        let mut result = ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), applied.scope).await;
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
        let mut state = self.interaction.interaction_selection_snapshot(&self.stores);
        let verb = InteractionVerb::of_action(action).ok_or_else(|| plugin_sdk_fault(format!("{action} is not one of the six framework interaction verbs")))?;
        let mut retire_leftover_domains: Vec<String> = Vec::new();
        match verb {
            InteractionVerb::Select => {
                let domain_id = interaction_domain_id_arg(args, action).await?;
                let def = self.tools.registry.interaction(&domain_id).await.cloned().ok_or_else(|| plugin_sdk_fault(format!("{action}: undeclared interaction domain {domain_id}")))?;
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
                    self.interaction.interaction_leftover_ids.retain(|id| !cleared_ids.contains(id));
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
                let def = self.tools.registry.interaction(&domain_id).await.cloned().ok_or_else(|| plugin_sdk_fault(format!("{action}: undeclared interaction domain {domain_id}")))?;
                let channel = args.and_then(|value| value.get("channel")).and_then(DslValue::as_str).unwrap_or("pointer").to_string();
                let targets = parse_interaction_targets(args, action).await?;
                let current = self.interaction.interaction_hover.get(&domain_id).cloned().unwrap_or_default();
                let known_ids = current.ids.iter().cloned().chain(targets.iter().map(|target| target.id.clone()));
                let topology = self.resolve_domain_topology(&def, known_ids).await?;
                let hover_input = protocol::HoverInput { channel, targets };
                let next = protocol::next_hover(&def.hover, &topology, &hover_input).await;
                if next.ids.is_empty() {
                    self.interaction.interaction_hover.remove(&domain_id);
                } else {
                    self.interaction.interaction_hover.insert(domain_id, next);
                }
            }
            InteractionVerb::ClearSelection => {
                retire_leftover_domains = self.tools.registry.interactions().await.map(|def| def.id.clone()).collect();
                self.interaction.interaction_leftover_ids.clear();
                for domain_id in &retire_leftover_domains {
                    state.selection.remove(domain_id);
                }
            }
            InteractionVerb::SelectAll => {
                for def in self.tools.registry.interactions().await.cloned().collect::<Vec<_>>() {
                    let granularity = state.active_granularity.get(&def.id).cloned().unwrap_or_else(|| def.granularities.first().map(|granularity| granularity.id.clone()).unwrap_or_default());
                    let mode = state.active_mode.get(&def.id).copied().unwrap_or_else(|| def.selection.modes.first().copied().unwrap_or(protocol::SelectionMode::Multiple));
                    let current = state.selection.get(&def.id).cloned().unwrap_or_default();
                    let hovered_ids = self.interaction.interaction_hover.get(&def.id).map(|hover| hover.ids.clone()).unwrap_or_default();
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
                let def = self.tools.registry.interaction(&domain_id).await.cloned().ok_or_else(|| plugin_sdk_fault(format!("{action}: undeclared interaction domain {domain_id}")))?;
                let raw = args.and_then(|value| value.get("mode")).and_then(DslValue::as_str).ok_or_else(|| plugin_sdk_fault(format!("{action} missing mode")))?;
                let mode = parse_selection_mode(raw).await?;
                if !def.selection.modes.contains(&mode) {
                    return Err(plugin_sdk_fault(format!("{action}: domain {domain_id} does not declare mode {raw}")));
                }
                state.active_mode.insert(domain_id, mode);
            }
            InteractionVerb::SetGranularity => {
                let domain_id = interaction_domain_id_arg(args, action).await?;
                let def = self.tools.registry.interaction(&domain_id).await.cloned().ok_or_else(|| plugin_sdk_fault(format!("{action}: undeclared interaction domain {domain_id}")))?;
                let granularity_id = args.and_then(|value| value.get("granularityId")).and_then(DslValue::as_str).ok_or_else(|| plugin_sdk_fault(format!("{action} missing granularityId")))?.to_string();
                if !def.granularities.iter().any(|granularity| granularity.id == granularity_id) {
                    return Err(plugin_sdk_fault(format!("{action}: domain {domain_id} does not declare granularity {granularity_id}")));
                }
                state.active_granularity.insert(domain_id, granularity_id);
            }
        }
        let touched = match verb {
            InteractionVerb::ClearSelection | InteractionVerb::SelectAll => self.tools.registry.interactions().await.map(|def| def.id.clone()).collect::<Vec<String>>(),
            _ => vec![interaction_domain_id_arg(args, action).await?],
        };
        state.hover = self.interaction.interaction_hover.clone();
        if !retire_leftover_domains.is_empty() {
            let (overlay, leftover_ids) = leftover_after_app_selection_write_v1(self.interaction.interaction_leftover_selection.as_ref(), &self.interaction.interaction_leftover_ids, &state, &retire_leftover_domains);
            self.interaction.interaction_leftover_selection = overlay;
            self.interaction.interaction_leftover_ids = leftover_ids;
        }
        if verb == InteractionVerb::Hover {
            let current_empty = state.selection.values().all(|selection| selection.ids.is_empty());
            if current_empty {
                if let Some(prior) = &self.interaction.interaction_leftover_selection {
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
            self.interaction.interaction_leftover_ids = current_ids;
        } else if !self.interaction.interaction_leftover_ids.is_empty() && retire_leftover_domains.is_empty() {
            let granularity =
                state.active_granularity.get("vortex").cloned().or_else(|| state.selection.get("vortex").map(|selection| selection.granularity.clone())).filter(|granularity| !granularity.is_empty()).unwrap_or_else(|| "object".to_string());
            state.selection.insert("vortex".to_string(), protocol::DomainSelection { granularity: granularity.clone(), ids: self.interaction.interaction_leftover_ids.clone(), anchor_id: None });
            state.active_granularity.entry("vortex".to_string()).or_insert(granularity);
        }
        let selection_cleared = !retire_leftover_domains.is_empty();
        let leftover = Self::leftover_interaction_view_from(&state, &self.interaction.interaction_hover, meta.view_state.as_ref().and_then(|view| view.window_id.as_deref()), selection_cleared);
        self.interaction.interaction_leftover_selection = Some(state.clone());
        if let Some(permit) = permit {
            self.stores.validate_framework_reserved_commit(action, permit).await?;
        }
        self.revalidate_and_persist_interaction_state(state, meta, InteractionRevalidateOrigin::Pick, identity).await?;
        if let Some(permit) = permit {
            if permit.lease.is_cancelled().await {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancelled"), format!("framework route '{action}' was cancelled before interaction publication")));
            }
        }
        self.tools.record_command(&mut self.history, action, ActionKind::Interaction, None, None, Vec::new(), None);
        let declared: Vec<&str> = touched.iter().map(String::as_str).collect();
        let scope = A::interaction_scope(verb, &declared).or_else(|| self.tools.registry.interaction_declared_refresh_scope(verb, &declared)).unwrap_or(UiDirtyScope::Full);
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

    pub(super) async fn stamp_pending_render_interaction_ui(&mut self, state: &protocol::InteractionState, body_key: &str) -> UiAssemblyResult<()> {
        let tree = self.pending.pending_render_tree.as_ref().ok_or_else(||ui_assembly_error("render.source-absent"))?;
        OriginalRenderInteractionStamp { registry: &self.tools.registry, interaction_hover: &self.interaction.interaction_hover, peer_presence: &self.roster.peer_presence, own_color: self.roster.own_color, local_interaction_authority: &mut self.interaction.local_interaction_authority, interaction_ui_topology: &mut self.interaction.interaction_ui_topology, pending_presence: &mut self.roster.pending_presence, presence_marked_keys: &mut self.roster.presence_marked_keys }.stamp_and_cache_interaction_ui(tree,state,body_key).await
    }

    pub(super) async fn build_artifact_reserved_action_job(&mut self, action: &str, args: Option<&DslValue>, raw_wire: Vec<u8>, meta: &ActionMeta) -> Result<Option<(ArtifactReservedToolJob, ArtifactToolCompletion<A>)>, Fault> {
        self.refresh_cache().await?;
        let (snapshot, config, history) = self.history.command_cache_inputs();
        let parent_document_id = self.stores.store.envelope().id.clone();
        let canonical_base_revision = self.stores.store.content_revision();
        let completion = ArtifactToolCompletion::<A>::new();
        let request = ArtifactReservedToolJobRequest {
                retained:self.mounted_policy.maintenance,
            controller_id: self.tools.tool_job_controller_id.clone(),
            tool_id: action.to_string(),
            payload_schema_id: self.tools.qualified_tool_proof(&self.stores, action)?.schema_id(),
            contract: self.tools.qualified_tool_proof(&self.stores, action)?.contract(),
            app_instance_id: meta.instance_id,
            ui_axes: meta.view_state.as_ref().map(|view| (view.locale, view.terminology)),
            parent_document_id,
            canonical_base_revision,
            authoring_seed: self.stores.authoring_seed(&meta.actor),
            snapshot,
            snapshot_read: self.stores.store.snapshot_read().map_err(FaultFrom::into_fault)?,
            config,
            history,
            children: ChildContentView::clone(&self.child_composition.child_content_root),
            raw_wire,
            input: ArtifactReservedToolInput::Action { args: args.cloned(), interaction: self.interaction.interaction_selection_snapshot(&self.stores), hover: self.interaction.interaction_hover.clone() },
            completion: completion.clone(),
        };
        Ok(A::build_reserved_tool_job(request)?.map(|job| (job, completion)))
    }

    pub(super) async fn build_artifact_reserved_media_job(&mut self, port: &str, media: Media, meta: &ActionMeta) -> Result<(ArtifactReservedToolJob, ArtifactToolCompletion<A>), Fault> {
        let action = "import-media";
        self.refresh_cache().await?;
        let (snapshot, config, history) = self.history.command_cache_inputs();
        let completion = ArtifactToolCompletion::<A>::new();
        let request = ArtifactReservedToolJobRequest {
                retained:self.mounted_policy.maintenance,
            controller_id: self.tools.tool_job_controller_id.clone(),
            tool_id: action.to_string(),
            payload_schema_id: self.tools.qualified_tool_proof(&self.stores, action)?.schema_id(),
            contract: self.tools.qualified_tool_proof(&self.stores, action)?.contract(),
            app_instance_id: meta.instance_id,
            ui_axes: meta.view_state.as_ref().map(|view| (view.locale, view.terminology)),
            parent_document_id: self.stores.store.envelope().id.clone(),
            canonical_base_revision: self.stores.store.content_revision(),
            authoring_seed: self.stores.authoring_seed(&meta.actor),
            snapshot,
            snapshot_read: self.stores.store.snapshot_read().map_err(FaultFrom::into_fault)?,
            config,
            history,
            children: ChildContentView::clone(&self.child_composition.child_content_root),
            raw_wire: Vec::new(),
            input: ArtifactReservedToolInput::Media { port: port.to_string(), media },
            completion: completion.clone(),
        };
        let job = A::build_reserved_tool_job(request)?
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-reserved-builder"), format!("media port '{port}' is registered but has no concrete resumable importer")))?;
        Ok((job, completion))
    }
}
