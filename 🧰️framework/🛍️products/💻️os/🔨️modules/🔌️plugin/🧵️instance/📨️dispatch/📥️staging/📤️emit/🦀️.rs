//! 🎿️ Staged `emit` members of the `instance-dispatch` use cases, as methods of the borrowed bundle (WP7 of the instance family).
//!
//! 📤️ `dispatch_emit` and the emit transaction groups: how one app emit lands as commands, effects and history rows.
use semio_framework::{ActionKind, AppRole, Fault, FaultCode, FaultFrom, FaultOrigin};
use semio_framework_plugin_decl_emission::{ActionMeta, ChildEmit, Emit, TransactionPhase};
use semio_framework::kernel::{ActorId, AppEvent, ArtifactDiff, ArtifactHandle, ArtifactVersion, EditRef, Effect, HybridLogicalTimestamp, InverseMutation, InvocationId, InvocationResult, KernelMutation, MutationId, SchemaId, UiDirtyScope, UndoGroup, UndoPolicy};
use semio_framework_plugin_decl_app::ArtifactApp;
use store::os_store::store_schema::ArtifactCommand;
use semio_framework_artifact_reference::{ArtifactDialect, ArtifactRef};
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_child::ChildContentRetirement;
use semio_framework_plugin_decl_views::child_view::{ChildContentView, MemberKeyRef};
use store::os_store::composition_coordinator::{ChildDispatch, CompositionCoordinator, GroupMeta};
use crate::dispatch_bundle::DispatchCtx;
use semio_framework_value::DslValue;
use semio_framework_plugin_job::{EmitWire, TransactionProposalDraft};
use std::future::Future;
use std::collections::HashSet;
use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework_plugin_instance_docs::docs_components::MediaTransfers;
use store::{ColdMutation, MemberFactory, Mutation, ReferenceSpaceMember};
use semio_framework_plugin_retained_command::typed_operation::MountedTypedCommandFullOperation;
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_instance_jobs::jobs_components::ToolOperations;
use crate::dispatch_emit;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_plugin_interaction::interaction_topology::take_inline_interaction_verbs;
use semio_framework_ui_contract::*;
use semio_framework_plugin_time_travel as time_travel;
use semio_framework_plugin_instance_docs::docs_instance_root::artifact_handle_of;
use semio_framework_plugin_instance_docs::docs_instance_root::tool_transaction_shape_fault;
use semio_framework_plugin_instance_docs::docs_instance_root::viewer_read_only_fault;

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> DispatchCtx<'ctx, A, M, X> {
    /// 🧪️ Lands one app `Emit` through the production `dispatch_emit` seam under a named
    /// verb — the harness for laws about what the reactor does with an emit's effect list before
    /// the host sees it (ticket 26/09/16/INPUT-CAUSALITY-LEDGER: the inline interaction fold),
    /// which no registry-less typed command can reach and no reserved route carries.
    #[cfg(test)]
    pub async fn test_dispatch_emit(&mut self, verb: &str, emit: Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        self.dispatch_emit(verb, emit, meta, identity, cx).await
    }

    /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — the reactor-side fold of a guest emit's
    /// interaction verbs: every `Effect::ReplayShellCommand { action_id ∈ INTERACTION_ACTION_IDS }`
    /// and every `Effect::DispatchAction { action ∈ INTERACTION_ACTION_IDS }` (the same intent in
    /// `📏️layout`/`💡️reasoning`'s emission shape, any `delay_ms`) is peeled BEFORE the effects
    /// are handed out ([`take_inline_interaction_verbs`]), the emit
    /// lands exactly as before ([`Self::dispatch_emit_inner`]), and then — after this command's
    /// own store lanes and `interaction_writes` landed, so a just-created id is already in
    /// topology — each peeled verb is applied inline ([`Self::fold_inline_interaction_verbs`]).
    /// The result leaves with the verbs' scopes merged into its `ui_scope`, the last verb's
    /// leftover `InteractionView` as its `output` when the carrier produced none (the SAME
    /// `Invocation.output.interactionView` lane a host-dispatched verb publishes on, so the
    /// world overlay still lands), and one diagnostic per verb that could not be applied.
    /// A failed carrier applies no verb: its pick did not happen. The typed-operation ladder,
    /// which never enters here, has its own twin ([`Self::publish_mounted_typed_inline_interaction_unit`]).
    pub async fn dispatch_emit(&mut self, verb: &str, mut emit: Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        if !emit.owned_child_emits.is_empty()||emit.owned_child_emits.capacity()!=0||!emit.child_preparations.is_empty()||emit.child_preparations.capacity()!=0{
            return self.mount_original_emit_publication(verb, emit, meta, None, cx).await;
        }
        let folded = take_inline_interaction_verbs(&mut emit.effects);
        let mut result = self.dispatch_emit_inner(verb, emit, meta, identity).await?;
        if folded.is_empty() {
            return Ok(result);
        }
        let scope = std::mem::replace(&mut result.ui_scope, UiDirtyScope::None);
        let fold = self.as_render().fold_inline_interaction_verbs(verb, folded, meta, scope, identity).await;
        result.ui_scope = fold.scope;
        if let (DslValue::Null, Some(leftover)) = (&result.output, fold.leftover) {
            result.output = leftover;
        }
        result.diagnostics.extend(fold.diagnostics);
        Ok(result)
    }

    /// 🧵️ The task lane belongs to the MIGRATED publication ladder
    /// (`publish_mounted_typed_operation_unit`), which owns the per-operation cancellation
    /// lease a spawned task must be cancellable under and spends exactly one task per
    /// bounded publication unit. This unmigrated route has neither, so a task emitted here
    /// is refused by name rather than spawned unowned.
    ///
    /// 🔒️ Contract §2.3 clause 2 — hard backstop, unreachable through `ViewerApp` (its `handle`
    /// builds `Emit` solely from `ViewEmit`'s three fields, contract §2.2), but this is what
    /// makes a hand-written runtime `ArtifactApp` impl that sets `ROLE = Viewer` safe too.
    ///
    /// 🔀️ Transaction freeze (contract §5.10): while a transaction is pending on this
    /// instance, reject anything that would emit artifact mutations rather than applying or
    /// queuing it — config/draft-only and op-less dispatches are unaffected. Read-only
    /// commands (RefreshUi/ReadDocument/ContextMenu/ephemeral lanes) never reach
    /// `dispatch_emit` at all, so they are unaffected by construction.
    ///
    /// 🔀️ Composite-mutation proposal (contract §5.1): when any artifact mutation carries
    /// foreign steps (folded forward exactly like `ArtifactStore::replay_mutations` folds its
    /// own snapshot, via the same public `Mutation::diff`/`protocol::apply_diff` pair — the
    /// private `apply_mutation` helper lives across the crate boundary in `store`), apply
    /// NOTHING — stash a `TransactionProposalDraft` instead, drained once by `plugin_exchange`
    /// (`take_pending_transaction_proposal`) and framed as `AppFrame::TransactionProposal`
    /// rather than the ordinary `Invocation`/`Done` frame.
    ///
    /// 📝️ Draft lane — ephemeral; applied without command-log rows (never checkpoints).
    ///
    /// 🚦️ Kind discipline, enforced where the operations actually arrive: a migrated dispatch hands
    /// the reducer to a worker, so the `View`/`🐚️Shell`-kind refusal `ArtifactApp::command_id`'s own
    /// doc promises can only be made against the published emit, never against the admission receipt.
    ///
    /// 🧮️ Config side dispatches first, independent of whether this verb ALSO touches the document
    /// — captures the resulting (possibly amended) config edit id for the command-log row below.
    ///
    /// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1): a non-empty `child_emits` means this
    /// gesture must land as ONE atomic multi-document group — routed entirely through
    /// `dispatch_emit_group` (parent ops included), never through the solitary single-store
    /// path below. See that method's own doc comment for why the two paths stay genuinely
    /// separate rather than being unified into one (the group protocol has no streamed open edit).
    ///
    /// 🧾️ An app-declared action logs under its declared kind (so an `Mutation`-kind action
    /// that happened to produce zero document operations — e.g. paste with no clipboard
    /// fragment, or a pure config-op dispatch — still gets a real row, `edit_id: None`,
    /// correctly filed under "Without Operations"); a declared command logs under its own
    /// independently declared kind; anything unresolved (registry-less test construction, an ad-hoc
    /// verb like an import-media port id) logs as `View`.
    ///
    /// 🪟️ A `View`-kind dispatch whose ONLY emission is the per-window config lane is session
    /// view state of one window (its camera pose, grid/LOD/selectable-kind options), not a
    /// command: the window-config store keeps its own per-window lane and nothing here is
    /// revertible, so a row would only pad the artifact command history with one entry per
    /// orbit tick. A `View` action that reaches the document or the shared config store still
    /// logs — `select`, whose emission is a config op, is unaffected.
    ///
    /// 🪢️ Captured before dispatch so `result_from_last_edit` can report only the operations THIS dispatch
    /// added — if a streamed tick grows the same open edit (`before_edit_id` unchanged after dispatch), these
    /// are the tail offsets into that edit's now-longer forwards/inverse; if a new edit was created
    /// instead, the offsets are moot (checked via edit identity, not reused blindly).
    ///
    /// 🕹️ Task 4: after EVERY artifact (document) dispatch, re-derive fresh topology and prune
    /// any selection/hover id no longer present — a document edit that deleted a node must not
    /// leave it lingering in another window's selection.
    ///
    /// 🧾️ One command-log entry per VCS edit — a coalesced gesture (`amended_same_edit`) grows the
    /// existing entry's `op_lines` live (see `build_history_view`), it never appends a new entry. The
    /// entry is logged BEFORE the interaction state is revalidated: a `Topology` domain refreshes the
    /// history cache, whose backfill would otherwise file the new edit under `apply` first and this
    /// dispatch would log it a second time (a committed typing run showed as two rows of one edit).
    ///
    /// ⏪️ No memory `inverse` for an edit-linked row — the VCS edit's own `Mutation::inverse`
    /// is already the real inverse, and `revertToCommand`'s edit_id branch replays that.
    async fn dispatch_emit_inner(&mut self, verb: &str, mut emit: Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        MediaTransfers::<A>::mint_extension_invocations(meta.instance_id, &mut emit)?;
        let Emit { artifact_mutations, config_mutations, window_config_mutations, draft_mutations, transaction, transaction_phase, effects, extension_invocations, events, ui_scope, child_emits, owned_child_emits, child_preparations, interaction_writes, tasks } = emit;
        assert!(owned_child_emits.is_empty()&&owned_child_emits.capacity()==0,"applying retained children require their staged publication owner");
        assert!(child_preparations.is_empty()&&child_preparations.capacity()==0,"dispatch owns only completely prepared child operation groups");
        if let Some(fault) = tool_transaction_shape_fault(verb, transaction.as_ref(), transaction_phase, !child_emits.is_empty(), !artifact_mutations.is_empty(), artifact_mutations.iter().any(Mutation::may_emit_foreign_steps)) {
            return Err(fault);
        }
        let effects = self.stores.stamp_load_document_effects(effects)?;
        debug_assert!(extension_invocations.is_empty(), "mint_extension_invocations drains the lane before destructuring");

        if !tasks.is_empty() {
            return Err(Fault::new(
                FaultOrigin::Framework,
                FaultCode::new("interactive-job.task-lane-unmigrated-route"),
                format!(
                    "verb {verb:?} emitted {} async task(s) on the unmigrated dispatch route; the task lane is published by the migrated typed-operation ladder, which owns the cancellation lease every spawned task is retired under",
                    tasks.len()
                ),
            ));
        }

        if A::ROLE == AppRole::Viewer && !artifact_mutations.is_empty() {
            return Err(viewer_read_only_fault(verb));
        }

        if !artifact_mutations.is_empty() {
            if let Some(fault) = X::mutation_refusal(&*self.time_travel, &*self.tool_runs, verb) {
                return Err(fault);
            }
        }

        if !artifact_mutations.is_empty() || !child_emits.is_empty() {
            if let Some(fault) = X::document_refusal(&*self.time_travel, &*self.tool_runs, verb) {
                return Err(fault);
            }
        }

        if !artifact_mutations.is_empty() && self.txn.pending_transaction.is_some() {
            let pending_txn_id = self.txn.pending_transaction.as_ref().map(|pending| pending.txn_id.clone()).unwrap_or_default();
            return Err(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.instance-busy", format!("verb {verb:?} would emit artifact mutations while transaction {pending_txn_id:?} is pending on this instance")));
        }

        if artifact_mutations.iter().any(Mutation::may_emit_foreign_steps) {
            let mut running = self.stores.store.snapshot().map_err(|error| error.into_fault())?;
            let foreign = Vec::new();
            for op in &artifact_mutations {
                if ColdMutation::foreign_step_source(op, &running, 0).map_err(|error| error.into_fault())?.is_some() { return Err(plugin_sdk_fault("foreign transaction output awaits its original staged preparation recipient")); }
                let outcome = op.diff(&running);
                if !outcome.is_applicable(protocol::MergePolicy::default()) {
                    return Err(MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", format!("mutation outcome was rejected: {:?}", outcome.messages())));
                }
                running = protocol::apply_diff_cold(outcome.diff(), &running).map_err(|error| MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string()))?;
            }
            if !foreign.is_empty() {
                let mut local_ops = Vec::with_capacity(artifact_mutations.len());
                for op in artifact_mutations.iter() {
                    local_ops.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
                }
                self.txn.pending_transaction_proposal = Some(TransactionProposalDraft { local_ops, foreign });
                return Ok(ToolOperations::<A>::empty_result(verb, meta, effects, events, ui_scope).await);
            }
        }

        let mut artifact_op_bytes = Vec::with_capacity(artifact_mutations.len());
        for op in artifact_mutations.iter() {
            artifact_op_bytes.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
        }
        let mut config_op_bytes = Vec::with_capacity(config_mutations.len());
        for op in config_mutations.iter() {
            config_op_bytes.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
        }
        let mut draft_op_bytes = Vec::with_capacity(draft_mutations.len());
        for op in draft_mutations.iter() {
            draft_op_bytes.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
        }
        self.pending.last_emit_wire = Some(EmitWire { document: protocol::encode_ops_vec(&artifact_op_bytes), config: protocol::encode_ops_vec(&config_op_bytes), draft: protocol::encode_ops_vec(&draft_op_bytes), children: Vec::new() });

        if !draft_mutations.is_empty() {
            self.stores.draft_store.dispatch(ArtifactCommand::Apply { mutations: draft_mutations, transaction: None }, identity).await.map_err(|error| error.into_fault())?;
        }

        let declared_kind = self.tools.declared_dispatch_kind(&*self.host_events, verb);
        if !artifact_mutations.is_empty() {
            self.media.require_operation_emitting_kind(verb, declared_kind)?;
        }

        let config_edited = !config_mutations.is_empty();
        if config_edited {
            let config_command = ArtifactCommand::Apply { mutations: config_mutations, transaction: None };
            self.stores.config_store.set_authoring_verb(Some(verb.to_string()));
            let dispatched = self.stores.config_store.dispatch(config_command, identity).await;
            self.stores.config_store.set_authoring_verb(None);
            dispatched.map_err(|error| error.into_fault())?;
        }

        if !window_config_mutations.is_empty() {
            let authority = self
                .stores.window_config_store
                .capture(meta.view_state.as_ref())
                .await?
                .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.window-context"), "window config emission requires one exact registered ViewModel window authority"))?;
            for mutation in window_config_mutations {
                self.stores.window_config_store.dispatch(&authority, &meta.actor, mutation, identity).await?;
            }
            X::window_config_published(&mut *self.time_travel, &mut *self.tool_runs);
        }
        #[cfg(test)]
        debug_assert!(tasks.is_empty());

        if !child_emits.is_empty() {
            let result = self.dispatch_emit_group(verb, &artifact_mutations, &child_emits, effects, events, ui_scope, meta, None, transaction, identity).await?;
            self.as_render().apply_interaction_writes(&interaction_writes, meta, identity).await?;
            return Ok(result);
        }

        let in_transaction = transaction.is_some();
        let log_label = self.tools.verb_label(verb);
        let vcs_commands = artifact_lane_commands(artifact_mutations, transaction, transaction_phase, self.stores.store.open_transaction());
        if vcs_commands.is_empty() {
            let kind = match self.host_events.invocation_kind {
                Some(kind) => kind,
                None => match self.tools.registry.get(verb).map(|definition| definition.kind) {
                    Some(kind) => kind,
                    None => match self.tools.registry.get_command(verb) {
                        Some(command) => command.kind,
                        None => ActionKind::View,
                    },
                },
            };
            if !in_transaction && !config_edited {
                self.tools.record_command(&mut *self.history, verb, kind, log_label, None, Vec::new(), None);
            }
            self.as_render().apply_interaction_writes(&interaction_writes, meta, identity).await?;
            return Ok(ToolOperations::<A>::empty_result(verb, meta, effects, events, ui_scope).await);
        }
        let aborting = matches!(vcs_commands.as_slice(), [ArtifactCommand::AbortTransaction { .. }]);
        let before_edit_id = self.stores.store.envelope().vcs.edits.last().map(|edit| edit.id.clone());
        let (before_forwards_len, before_backwards_len) = self.stores.store.edit_mutations().map_or((0, 0), |(f, b, _)| (f.len(), b.len()));
        for vcs_command in vcs_commands {
            self.stores.store.set_authoring_verb(Some(verb.to_string()));
            let dispatched = self.stores.store.dispatch(vcs_command, identity).await;
            self.stores.store.set_authoring_verb(None);
            match dispatched {
                Ok(receipt) => self.pending.record_dispatch_receipt(&mut *self.stores, receipt),
                Err(vcs::VcsError::Rejected { policy, messages }) => return Err(self.pending.record_rejected_dispatch(policy, messages).await),
                Err(error) => return Err(error.into_fault()),
            }
            .await;
        }
        if aborting {
            self.as_render().revalidate_interaction_on_document_change(meta, identity).await?;
            self.as_render().apply_interaction_writes(&interaction_writes, meta, identity).await?;
            self.history.retire_displaced_document_rows();
            return Ok(ToolOperations::<A>::empty_result(verb, meta, effects, events, ui_scope).await);
        }
        let amended_same_edit = before_edit_id.is_some() && self.stores.store.envelope().vcs.edits.last().map(|edit| &edit.id) == before_edit_id.as_ref();
        if !amended_same_edit {
            if let Some(edit_id) = self.stores.store.envelope().vcs.edits.last().map(|edit| edit.id.clone()) {
                let kind = match self.host_events.invocation_kind {
                    Some(kind) => kind,
                    None => match self.tools.registry.get(verb) {
                        Some(def) => def.kind,
                        None => match self.tools.registry.get_command(verb) {
                            Some(def) => def.kind,
                            None => ActionKind::Mutation,
                        },
                    },
                };
                self.tools.record_command(&mut *self.history, verb, kind, log_label, Some(edit_id), Vec::new(), None);
            }
        }
        self.as_render().revalidate_interaction_on_document_change(meta, identity).await?;
        self.as_render().apply_interaction_writes(&interaction_writes, meta, identity).await?;
        let tail_offset = if amended_same_edit { (before_forwards_len, before_backwards_len) } else { (0, 0) };
        Ok(self.stores.result_from_last_edit(verb, meta, effects, events, ui_scope, tail_offset).await)
    }

    /// 🧩️ Commits an agent transaction that carries owned-child op groups as ONE composite gesture
    /// (`dispatch_emit_group`, the same path the shell lane's child-group verbs take) whose group identity is
    /// `txn_id` on every member it touches, so the gateway's `TransactionUndo{group_id: txn_id}` moves them all.
    /// The parent's tail edit (when the group touched the parent) carries the prepared origin like a solitary
    /// commit does. Returns the parent's edit id, else the first touched child's.
    pub async fn commit_transaction_group(&mut self, txn_id: &str, ops: Vec<A::Mutation>, children: Vec<ChildEmit>, origin: protocol::MutationOrigin, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, Fault> {
        let parent_edits_before = self.stores.store.envelope().vcs.edits.len();
        let result = self
            .dispatch_emit_group(&format!("transaction:{txn_id}"), &ops, &children, Vec::new(), Vec::new(), UiDirtyScope::Full, meta, Some(txn_id.to_string()), None, identity)
            .await
            .map_err(|fault| MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", fault.message))?;
        if self.stores.store.envelope().vcs.edits.len() > parent_edits_before {
            self.stores.store.stamp_tail_origin(origin).await.map_err(|error| MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", format!("{error:?}")))?;
            return Ok(self.stores.store.envelope().vcs.edits.last().map(|edit| edit.id.clone()).unwrap_or_default());
        }
        Ok(result.inverse_group.member_edits.first().map(|edit| edit.edit_id.clone()).unwrap_or_default())
    }

    /// ↩️ `transaction_undo`/`transaction_redo` for a composing instance: moves every member (parent and
    /// owned children) whose tail carries `group_id` through `CompositionCoordinator::undo_group`/`redo_group`,
    /// republishing each moved child's content like the shell lane's group history does. A group no member
    /// carries is refused by name, exactly like the solitary path.
    pub async fn transaction_group_history(&mut self, action: &str, group_id: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        self.child_composition.admit_child_content_publication_span(self.child_composition.children.len())?;
        let parent_id = self.stores.store.envelope().id.clone();
        let parent_dialect: ArtifactDialect = A::DIALECT.into();
        if self.stores.store.envelope().dialect.as_ref() != Some(&parent_dialect) {
            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.group-history-dialect"), "composition history requires the parent's exact declared dialect"));
        }
        let parent_ref = ArtifactRef { artifact_id: parent_id.clone(), dialect: parent_dialect };
        let child_refs: Vec<ArtifactRef> = self.child_composition.children.entries_physical().map(|entry| entry.reference.clone()).collect();
        let mut members: Vec<(&ArtifactRef, &mut M)> = Vec::with_capacity(child_refs.len());
        for (reference, entry) in child_refs.iter().zip(self.child_composition.children.entries_mut_physical()) {
            members.push((reference, &mut entry.member));
        }
        let report = if action == "undo" { CompositionCoordinator::undo_group(&parent_ref, &mut self.stores.store, &mut members, group_id, identity).await } else { CompositionCoordinator::redo_group(&parent_ref, &mut self.stores.store, &mut members, group_id, identity).await };
        drop(members);
        self.history.cache = None;
        for (reference, _) in report.undone.iter().filter(|(reference, _)| reference.artifact_id != parent_id) {
            let Some(key) = self.child_composition.children.keyed_entries().find(|(_, entry)| entry.reference.artifact_id == reference.artifact_id).map(|(key, _)| key.to_key()) else {
                return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.group-history-root"), "group history moved a child without exact immutable-root authority"));
            };
            let publication_generation = self.child_composition.admit_child_content_publication()?;
            self.child_composition.publish_member_content(publication_generation, key.borrowed()).await?;
        }
        if report.undone.is_empty() {
            let skipped = report.skipped.iter().map(|(reference, error)| format!("{} ({error})", reference.artifact_id)).collect::<Vec<_>>().join(", ");
            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.group-history-tail"), format!(
                "transaction_{action}: no member of this instance carries group {group_id:?} at its {} tail{}",
                if action == "undo" { "applied" } else { "redo" },
                if skipped.is_empty() { String::new() } else { format!(" (skipped: {skipped})") }
            )));
        }
        self.tools.record_command(&mut *self.history, action, ActionKind::History, None, None, Vec::new(), None);
        Ok(())
    }

    /// 🚧️ `ArtifactEnvelope.dialect` stays `Option<ArtifactDialect>` per B2's own DEFERRED
    /// scope decision (`📓️wave1-reports/b2-store-composition-report.md`) — this fallback (a
    /// synthetic "native" dialect from the app's own `DOCUMENT_SCHEMA`) is only ever consulted
    /// when no real dialect was ever threaded through `create_document_envelope`, and only
    /// matters for the OWNERSHIP-GRAPH bookkeeping `dispatch_group` needs a real `ArtifactRef`
    /// for, not for any wire/codec decision.
    ///
    /// 🛡️ Keys are unique and `self.children` is not resized until after `dispatch_group`, so
    /// these entry pointers stay valid and non-aliasing for the simultaneous child borrows
    /// `dispatch_group` requires.
    ///
    /// 🎛️ `meta.actor` is honored; a group is always one plain edit per member.
    ///
    /// 🎯️ RESOLVED (was BLOCKED — see `📓️terra-dedyn-fw-os-spacemember-report.md` §dispatch_group,
    /// and `📓️terra-dispatch-group-split-report.md` for the fix): `store::os_store::composition_coordinator::CompositionCoordinator::
    /// dispatch_group<Mp: SpaceMember, Mc: ReferenceSpaceMember + MemberFactory>` now takes SEPARATE type
    /// parameters for `parent` and `children` — `self.store` (`ArtifactStore<A::Snapshot,
    /// A::Mutation>`, this app's own fixed type, only ever mutated) unifies as `Mp`; `dispatches`
    /// (`Vec<(&mut M, ChildDispatch<'_>)>`, this composition's CHILD-kind enum, the only side ever
    /// genesis-constructed) unifies as `Mc`. Dyn-free (O1-compliant) and now type-checks.
    ///
    /// 🌱️ Absorb any freshly-created children into the live map — extracted into its own
    /// method (below) both so it is directly unit-testable and so it needs no revisiting once
    /// a genesis-emitting `Emit` constructor lands (`genesis` is always `Vec::new()` on THIS
    /// call today — a `ChildEmit` only ever targets an ALREADY-live child).
    ///
    /// 🪆️ Child-lane replication: a composed member's edit is a real event of a real document,
    /// and the parent's backbone is the only endpoint it can cross. Announced here, at the one
    /// choke point every child edit lands at, so a replica folds it into ITS member of the
    /// same lane instead of diverging (ticket 26/09/19 `📓️flow.md` §5.3).
    ///
    /// 🪪️ The PARENT's handle must be the same value its own `KernelMutation.document` carries
    /// (`ArtifactHandle(meta.instance_id)`), not `artifact_handle_of(parent_id)` — otherwise one
    /// `InvocationResult` identifies the same document two different ways and any consumer
    /// correlating `mutations` with `member_edits` by handle silently fails to match the parent.
    /// Children keep the content-addressed `artifact_handle_of`, which is what their own
    /// `KernelMutation.document` uses.
    ///
    /// 🧱️ Parent side: `dispatch_wire` landed a REAL edit on `self.store` above (whenever
    /// `parent_ops` was non-empty), so its tail edit's real forward/inverse mutations are
    /// available exactly like the solitary path's `result_from_last_edit`.
    ///
    /// 🧩️ Child side: real per-op inverse bytes are not retrievable through the object-safe
    /// `SpaceMember` seam (see this fn's own doc comment) — each touched child's edit surfaces
    /// as ONE best-effort `KernelMutation` bundling all of that child's ops as its diff
    /// payload, with an empty (but validly-encoded, decodes to zero ops) inverse payload.
    /// `dispatch_group`'s phase 2 walks `children` (built here in the SAME order as
    /// `child_emits`) by index, skipping empty-ops entries — so the non-empty-ops subsequence
    /// of `child_emits`, in order, lines up 1:1 with the non-parent tail of `member_edits`.
    ///
    /// 🧾️ One command-log row for the whole group, carrying every touched child's edit in `child_edit_ids` (see
    /// `CommandLogEntry::child_edit_ids`'s own doc comment); a config-lane edit never joins it (design §20.13, L4).
    #[allow(clippy::too_many_arguments)]
    pub async fn dispatch_emit_group(
        &mut self,
        verb: &str,
        artifact_mutations: &[A::Mutation],
        child_emits: &[ChildEmit],
        effects: Vec<Effect>,
        events: Vec<AppEvent>,
        ui_scope: UiDirtyScope,
        meta: &ActionMeta,
        group_id: Option<String>,
        transaction: Option<protocol::TransactionRef>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        if !artifact_mutations.is_empty() {
            self.media.require_operation_emitting_kind(verb, self.tools.declared_dispatch_kind(&*self.host_events, verb))?;
        }
        let touched_child_count = child_emits.iter().filter(|child| !child.ops.is_empty()).count();
        let child_publication_generation = (touched_child_count != 0).then(|| self.child_composition.admit_child_content_publication()).transpose()?;
        let parent_id = self.stores.store.envelope().id.clone();
        let parent_dialect: ArtifactDialect = A::DIALECT.into();
        if self.stores.store.envelope().dialect.as_ref() != Some(&parent_dialect) {
            return Err(plugin_sdk_fault("composition requires the parent's exact declared dialect"));
        }
        let parent_ref = ArtifactRef { artifact_id: parent_id.clone(), dialect: parent_dialect };
        let parent_ops=artifact_mutations.iter().map(::protocol::OpBinary::encode_op).collect::<Result<Vec<_>,_>>().map_err(|error|plugin_sdk_fault(error.to_string()))?;

        let mut seen_keys = ::std::collections::HashSet::with_capacity(child_emits.len());
        let mut child_ptrs: Vec<(*mut M, ChildDispatch<'_>)> = Vec::with_capacity(child_emits.len());
        for child_emit in child_emits {
            let key = (child_emit.slot.clone(), child_emit.child_id.clone());
            if !seen_keys.insert(key.clone()) {
                return Err(plugin_sdk_fault(format!("dispatch_emit: verb {verb:?} emitted duplicate ChildEmit for slot {:?} child {:?}", child_emit.slot, child_emit.child_id)));
            }
            let entry = self.child_composition.children.get_mut(&key).ok_or_else(|| {
                plugin_sdk_fault(format!("dispatch_emit: verb {verb:?} emitted a ChildEmit for slot {:?} child {:?} with no live child store — call VcsArtifactApp::open_child/register_child first", child_emit.slot, child_emit.child_id))
            })?;
            let target = entry.reference.clone();
            let member_ptr: *mut M = &mut entry.member;
            child_ptrs.push((member_ptr, ChildDispatch::borrowed(target, &child_emit.ops, &child_emit.op_schema, &child_emit.labels)));
        }
        let child_refs: Vec<ArtifactRef> = child_ptrs.iter().map(|(_, dispatch)| dispatch.child.clone()).collect();
        let child_member_ptrs: Vec<*mut M> = child_ptrs.iter().map(|(member, _)| *member).collect();
        let mut dispatches: Vec<(&mut M, ChildDispatch<'_>)> = child_ptrs.into_iter().map(|(ptr, dispatch)| (unsafe { &mut *ptr }, dispatch)).collect();

        let group_meta = GroupMeta { actor: Some(meta.actor.clone()), group_id, transaction };
        self.stores.store.set_authoring_verb(Some(verb.to_string()));
        let receipt = self.child_composition.composition.dispatch_group(&parent_ref, &mut self.stores.store, &mut dispatches, parent_ops, Vec::new(), group_meta, identity).await;
        self.stores.store.set_authoring_verb(None);
        let receipt = receipt.map_err(|error| plugin_sdk_fault(error.to_string()))?;
        drop(dispatches);
        self.history.cache = None;
        if let Some(publication_generation) = child_publication_generation {
            let mut next = ChildContentView::clone(&self.child_composition.child_content_root);
            for child_emit in child_emits.iter().filter(|child| !child.ops.is_empty()) {
                let captured = match self.child_composition.children.get(&(child_emit.slot.clone(), child_emit.child_id.clone())) {
                    Some(entry) => next.with_member(MemberKeyRef::root(&child_emit.slot, &child_emit.child_id), &entry.reference, &entry.member).await,
                    None => Err(plugin_sdk_fault("committed child group lost its exact live member before immutable-root capture")),
                };
                match captured {
                    Ok(captured) => next = captured,
                    Err(fault) => {
                        if !next.is_empty() {
                            self.child_composition.child_content_retirements.insert_admitted(publication_generation, ChildContentRetirement::new(next, false));
                        }
                        let mut members: Vec<(&ArtifactRef, &mut M)> = child_refs.iter().zip(child_member_ptrs.iter().copied()).map(|(reference, member)| (reference, unsafe { &mut *member })).collect();
                        let compensation = CompositionCoordinator::undo_group(&parent_ref, &mut self.stores.store, &mut members, &receipt.invocation_id, identity).await;
                        drop(members);
                        if compensation.undone.len() != receipt.member_edits.len() {
                            return Err(plugin_sdk_fault(format!("child root capture failed ({}) and group compensation moved {}/{} members", fault.message, compensation.undone.len(), receipt.member_edits.len())));
                        }
                        return Err(fault);
                    }
                }
            }
            let previous = std::mem::replace(&mut *self.child_composition.child_content_root, next);
            if !previous.is_empty() {
                self.child_composition.child_content_retirements.insert_admitted(publication_generation, ChildContentRetirement::new(previous, false));
            }
            self.child_composition.child_content_generation = publication_generation;
        }

        self.child_composition.absorb_created_children(&mut *self.stores, receipt.created_children).await?;

        let announced_lanes: Vec<(String, String)> = child_emits.iter().filter(|child_emit| !child_emit.ops.is_empty()).map(|child_emit| (child_emit.slot.clone(), child_emit.child_id.clone())).collect();
        self.child_composition.announce_member_tail_edits(&mut *self.stores, &announced_lanes).await?;

        let invocation_id = InvocationId(receipt.invocation_id.clone());
        let mut member_edits: Vec<EditRef> = Vec::with_capacity(receipt.member_edits.len());
        for (reference, edit_id) in receipt.member_edits.iter() {
            let document = if reference.artifact_id == parent_id { ArtifactHandle(meta.instance_id as u128) } else { artifact_handle_of(&reference.artifact_id).await };
            member_edits.push(EditRef { document, edit_id: edit_id.clone() });
        }

        let mut mutations: Vec<KernelMutation> = Vec::new();

        let parent_touched = receipt.member_edits.iter().any(|(reference, _)| reference.artifact_id == parent_id);
        if parent_touched {
            let operation_wires = self.stores.store.authored_edit_operation_bytes(identity).map_err(|error| plugin_sdk_fault(error.to_string()))?;
            if let (Some((forward_wires, inverse_wires)), Some((forwards, _, mutation_meta))) = (operation_wires, self.stores.store.edit_mutations()) {
                let inverse_payload = identity.encode(|control| ::protocol::os_spr::io::binary::operation_sequence::encode(&inverse_wires, control)).map_err(|error| plugin_sdk_fault(error.to_string()))?;
                let document = ArtifactHandle(meta.instance_id as u128);
                let schema = A::DOCUMENT_SCHEMA.to_string();
                for ((index, _forward), forward_wire) in forwards.iter().enumerate().zip(forward_wires) {
                    let entry = mutation_meta.get(index);
                    let mutation_id = entry.and_then(|entry_meta| entry_meta.mutation_id.clone()).unwrap_or_else(|| MutationId(format!("{}:{index}", invocation_id.0)));
                    let base_version = ArtifactVersion(entry.map_or(0, |entry_meta| entry_meta.base_version));
                    let undo_policy = entry.map_or(UndoPolicy::ExactBaseOnly, |entry_meta| entry_meta.undo_policy);
                    let author = entry.and_then(|entry_meta| entry_meta.author_id.clone()).unwrap_or_else(|| ActorId(meta.actor.clone()));
                    let timestamp = entry.map_or_else(|| HybridLogicalTimestamp::new(0, 0), |entry_meta| entry_meta.timestamp);
                    mutations.push(KernelMutation {
                        id: mutation_id.clone(),
                        document,
                        base_version,
                        invocation_id: invocation_id.clone(),
                        diff: ArtifactDiff { schema: SchemaId(format!("{schema}.operation")), payload: forward_wire },
                        inverse: InverseMutation {
                            target_mutation: mutation_id,
                            inverse_diff: ArtifactDiff { schema: SchemaId(format!("{schema}.operation.inverse")), payload: identity.encode(|control| control.copy_bytes(&inverse_payload)).map_err(|error| plugin_sdk_fault(error.to_string()))? },
                            base_version,
                            dependencies: Vec::new(),
                            undo_policy,
                        },
                        dependencies: Vec::new(),
                        author,
                        timestamp,
                    });
                }
            }
        }

        let child_member_edits: Vec<&(ArtifactRef, String)> = receipt.member_edits.iter().filter(|(reference, _)| reference.artifact_id != parent_id).collect();
        let touched_child_emits: Vec<&ChildEmit> = child_emits.iter().filter(|child_emit| !child_emit.ops.is_empty()).collect();
        debug_assert_eq!(touched_child_emits.len(), child_member_edits.len(), "dispatch_group's per-child edit order must match the non-empty-ops subsequence of child_emits");
        let mut child_edit_ids: Vec<String> = Vec::with_capacity(child_member_edits.len());
        for (child_emit, (reference, edit_id)) in touched_child_emits.into_iter().zip(child_member_edits) {
            child_edit_ids.push(edit_id.clone());
            let document = artifact_handle_of(&reference.artifact_id).await;
            let mutation_id = MutationId(edit_id.clone());
            mutations.push(KernelMutation {
                id: mutation_id.clone(),
                document,
                base_version: ArtifactVersion(0),
                invocation_id: invocation_id.clone(),
                diff: ArtifactDiff { schema: child_emit.op_schema.clone(), payload: protocol::encode_ops_vec(&child_emit.ops) },
                inverse: InverseMutation {
                    target_mutation: mutation_id,
                    inverse_diff: ArtifactDiff { schema: SchemaId(format!("{}.inverse", child_emit.op_schema.0)), payload: protocol::encode_ops_vec(&[]) },
                    base_version: ArtifactVersion(0),
                    dependencies: Vec::new(),
                    undo_policy: UndoPolicy::ExactBaseOnly,
                },
                dependencies: Vec::new(),
                author: ActorId(meta.actor.clone()),
                timestamp: HybridLogicalTimestamp::new(0, 0),
            });
        }

        let mutation_ids: Vec<MutationId> = mutations.iter().map(|mutation| mutation.id.clone()).collect();
        let inverse_mutations: Vec<InverseMutation> = mutations.iter().map(|mutation| mutation.inverse.clone()).collect();

        let parent_edit_id = receipt.member_edits.iter().find(|(reference, _)| reference.artifact_id == parent_id).map(|(_, edit_id)| edit_id.clone());
        let kind = self.tools.registry.get(verb).map_or(ActionKind::Mutation, |def| def.kind);
        let label = self.tools.verb_label(verb);
        self.tools.record_command(&mut *self.history, verb, kind, label, parent_edit_id, child_edit_ids, None);

        Ok(InvocationResult {
            output: DslValue::Null,
            mutations,
            inverse_group: UndoGroup { invocation_id, mutations: mutation_ids, inverse_mutations, member_edits },
            diagnostics: Vec::new(),
            requested_effects: effects,
            events,
            ui_scope,
            history_patch: None,
        })
    }

    /// ♻️ The ONE release site of an admitted typed-operation slot. Every outcome an operation can reach
    /// — settled, faulted, refused, replaced, cancelled — funnels its mounted owner into
    /// [`MountedTypedCommandFullOperationStage::Retiring`], and this is the only code that may remove it
    /// from `tool_operations` and so free its residue class for [`Self::admit_typed_operation_slot`]: one
    /// bounded retirement unit, the terminal-empty witness, then the exact removal. The turn driver
    /// ([`Self::advance_typed_operation_publication_one`]), the cooperative maintenance rotation and the
    /// app close ladder all walk THIS function, so no authority can admit a slot and leave its release to
    /// another (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B8).
    /// 🧾️ The migrated twin of [`Self::dispatch_emit`]'s own command-log append. A typed command's
    /// emit reaches its stores through the mounted publication ladder, which never walks
    /// `dispatch_emit`, so before this every migrated dispatch reached the history panel only as
    /// [`Self::backfill_command_log`]'s anonymous `apply` row — losing the verb, its
    /// declared [`ActionKind`], its own label and its revert target — and a dispatch that published
    /// no durable lane at all (a `Mutation` verb whose reducer emitted nothing, every `View` verb
    /// such as `select`) reached it not at all.
    ///
    /// It runs on the durable RECEIPT, not at retirement, because the operation mints its `Ui` page
    /// between the two and rendering the history body backfills: an edit this row owns would already
    /// be filed under `apply` by the time the slot retired. Only the document lane opens or amends a row: a config-lane
    /// edit is never a history row (design §20.13, L4). A coalesced gesture amends the edit its first dispatch already
    /// logged, so later dispatches append no second row — and they mark that row dirty, so the admitting completion
    /// re-upserts it.
    pub fn record_typed_operation_lane(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>) {
        if mounted.terminal_fault.is_some() {
            return;
        }
        let Some(edit_id) = self.stores.store.envelope().vcs.edits.last().map(|edit| edit.id.clone()) else { return };
        let logged_seq = self.history.command_log.iter().find(|entry| entry.edit_id.as_deref() == Some(edit_id.as_str())).map(|entry| entry.seq);
        if let Some(seq) = logged_seq {
            mounted.command_logged = true;
            self.history.history_dirty_sequences.insert(seq);
            return;
        }
        if mounted.command_logged {
            if let Some(entry) = self.history.command_log.last_mut().filter(|entry| entry.action_id == mounted.verb) {
                entry.edit_id = Some(edit_id);
                self.history.history_dirty_sequences.insert(entry.seq);
                self.history.log_generation += 1;
            }
            return;
        }
        mounted.command_logged = true;
        let kind = self.tools.typed_operation_command_kind(&*self.host_events, &mounted.verb, true);
        let verb = mounted.verb.clone();
        let label = self.tools.verb_label(&verb);
        self.tools.record_command(&mut *self.history, &verb, kind, label, Some(edit_id), Vec::new(), None);
    }
}

/// 🎞️ The document-store commands an emission's artifact lane dispatches (design §15): a streamed tick appends to its
/// transaction's open edit (opening it), a commit closes the open edit of its transaction (appending its last operations
/// first) or, with none open, records its operations as one edit carrying the ref, an abort reverts the open edit, and an
/// emission outside a transaction applies as one plain edit. An empty list publishes nothing.
pub(crate) fn artifact_lane_commands<M>(mutations: Vec<M>, transaction: Option<protocol::TransactionRef>, phase: TransactionPhase, open: Option<&store::os_store::store_schema::OpenToolTransaction>) -> Vec<ArtifactCommand<M>> {
    let open = open.zip(transaction.as_ref()).is_some_and(|(open, transaction)| open.transaction.id == transaction.id);
    let append = |mutations: Vec<M>, transaction: protocol::TransactionRef| (!mutations.is_empty()).then(|| ArtifactCommand::AppendTransaction { mutations, transaction });
    match (transaction, phase) {
        (Some(transaction), TransactionPhase::Stream) => append(mutations, transaction).into_iter().collect(),
        (Some(transaction), TransactionPhase::Commit) if open => {
            let transaction_id = transaction.id.clone();
            append(mutations, transaction).into_iter().chain([ArtifactCommand::CommitTransaction { transaction_id }]).collect()
        }
        (Some(transaction), TransactionPhase::Abort) => open.then(|| ArtifactCommand::AbortTransaction { transaction_id: transaction.id }).into_iter().collect(),
        (_, _) if mutations.is_empty() => Vec::new(),
        (transaction, _) => vec![ArtifactCommand::Apply { mutations, transaction }],
    }
}
