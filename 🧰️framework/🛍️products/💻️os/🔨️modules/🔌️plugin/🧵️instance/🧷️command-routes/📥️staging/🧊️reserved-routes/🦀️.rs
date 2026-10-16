//! 🏊️ Staged `reserved-routes` members of the `instance-commands` use cases, as methods of the borrowed bundle (WP7 of the instance family).
//!
//! 🧊️ Framework-reserved routes: the history, clipboard and shared host routes and their commit units.
use super::*;
use super::commands_instance_root::catalogue_snapshot_from_document;
use super::commands_instance_root::note_shell_command_label;
use super::docs_plugin_app_impl::history_changed_event;

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> CommandCtx<'ctx, A, M, X> {
    /// 🧮️ One reserved gesture's work, in the STAGED EDIT ROWS every other capacity in the
    /// runtime is counted in — never the bare item count the route used to hand its job, which
    /// no longer matched the two-row footprint its own config preflight declares.
    async fn framework_reserved_work_items(&mut self, action: &str, args: Option<&DslValue>) -> Result<usize, Fault> {
        let items = self.framework_reserved_route_items(action, args).await?;
        FRAMEWORK_RESERVED_ROUTE_CAPACITY.rows_for_items(items).ok_or_else(|| plugin_sdk_fault(format!("framework reserved route '{action}' declares {items} items beyond its exact work capacity")))
    }

    async fn framework_reserved_route_items(&mut self, action: &str, args: Option<&DslValue>) -> Result<usize, Fault> {
        match action {
            "commitCheckpoint" | "checkoutCheckpoint" => Ok(self.child_composition.children.len().saturating_add(1)),
            REVERT_TO_COMMAND_ACTION_ID => {
                self.as_render().refresh_cache().await?;
                let entry_seq = args.and_then(|value| value.get("entrySeq")).and_then(DslValue::as_f64).map(|seq| seq as u64);
                let target = entry_seq.and_then(|seq| self.history.cache.as_ref().and_then(|(_, _, _, history)| history.commands.iter().find(|entry| entry.seq == seq && entry.revertible)).and_then(|entry| entry.edit_id.clone()));
                match target {
                    Some(edit_id) => {
                        let applied = self.stores.store.applied_edit_ids();
                        Ok(applied.iter().position(|id| *id == edit_id).map_or(1, |position| applied.len().saturating_sub(position + 1).max(1)))
                    }
                    None => Ok(1),
                }
            }
            "copy" | "cut" => {
                let interaction = self.interaction.interaction_selection_snapshot(&*self.stores);
                Ok(interaction.selection.values().map(|selection| selection.ids.len()).sum::<usize>().max(1))
            }
            "paste" => {
                let fragment_bytes = args
                    .and_then(|value| value.get("fragment"))
                    .and_then(|value| <ClipboardFragment as semio_framework_value::FromValue>::from_value(value.clone()).ok())
                    .map(|fragment| fragment.pack_bytes.map_or(fragment.dsl_text.len(), |bytes| bytes.len()))
                    .unwrap_or_default();
                Ok(fragment_bytes.div_ceil(4_096).max(1))
            }
            _ => Ok(1),
        }
    }

    /// 🎰️ `pending_reserved` is DIRECT-MAPPED on `job % ARTIFACT_LIVE_OUTPUT_SLOTS`, and this caller
    /// MINTS the id — so a residue class already held by an unrelated reserved verb (a
    /// `noteShellCommand` the shell records for every user command, an in-flight `undo`) must be
    /// answered by minting into a vacant class, never by faulting the whole route. It used to fault:
    /// a canvas pick landed as `interactive-job.reserved-spawn-capacity` with 63 slots standing empty
    /// (ticket 26/09/02/PUZZLE-3D-END-TO-END battery #44-pre: `framework route 'interactionSelect'
    /// has no exact pending spawn slot`). `retire_pending_reserved_latest_wins` still runs first, so
    /// an interaction storm keeps its latest-wins retirement rather than filling the table.
    pub(super) async fn dispatch_framework_reserved_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
        if let Some(fault) = X::reserved_action_refusal(&*self.time_travel, &*self.tool_runs, action) {
            return Err(fault.into());
        }
        let contract = self.tools.qualified_tool_proof(&*self.stores, action)?.contract();
        let decoded_items = bounded_json_items(args, contract.max_decoded_items)?;
        let wire_args = args.cloned();
        let raw = semio_framework_pack_json::to_json_string(&(action, wire_args)).into_bytes();
        let work_items = self.framework_reserved_work_items(action, args).await?;
        if CLIPBOARD_ACTION_IDS.contains(&action) {
            let Some(authority)=self.as_dispatch().admit_reserved_emit_producer(action,meta,cx)?else{return Ok(None)};
            let producer = match self.as_render().build_artifact_reserved_action_job(action, args, raw.clone(), meta).await { Ok(producer) => producer, Err(fault) => { self.tools.finish_reserved_emit_producer(authority); return Err(fault.into()); } };
            let mut app_completion = None;
            let job = match producer {
                Some((job, completion)) => {
                    app_completion = Some(completion);
                    job
                }
                None => { self.tools.finish_reserved_emit_producer(authority); return Err(plugin_sdk_fault("clipboard producer has no exact installed reserved builder").into()); },
            };
            return self.as_dispatch().dispatch_reserved_original_producer(action,&raw,decoded_items,job,app_completion.expect("installed clipboard producer owns its original completion"),meta,None,authority,cx).await.map(Some).map_err(Into::into);
        }
        let Some(permit)=self.as_dispatch().admit_framework_reserved_spawn(action,&raw,decoded_items,meta,cx).await?else{return Ok(None)};
        let mut job = permit.operation.operation.0;
        self.pending.retire_pending_reserved_latest_wins(action, job);
        assert!(self.pending.pending_reserved.can_insert(job),"original reserved admission seals its exact vacant spawn slot");
        self.pending.pending_reserved.insert_admitted(job, PendingFrameworkReserved { action: action.to_string(), args: args.cloned(), meta: meta.clone(), permit });
        self.tools.release_reserved_emit_slot(job);
        let mut admitted = ToolOperations::<A>::empty_result(action, meta, vec![Effect::SpawnJob { job, kind: FRAMEWORK_RESERVED_JOB_KIND.into(), input: encode_framework_reserved_job_input(work_items as u64, &raw), placement: semio_framework::kernel::JobPlacement::Isolated }], Vec::new(), UiDirtyScope::None)
        .await;
        admitted.output = DslValue::Object(vec![("operationId".into(), DslValue::String(job.to_string())), ("generation".into(), DslValue::String(self.stores.store.generation().to_string()))]);
        Ok(Some(admitted))
    }

    /// 🪜️ Runs ONE bounded unit of the commit at the head of the queue. Commits are strictly FIFO,
    /// and the head waits while a finished outcome is still untaken, so at most one outcome is ever
    /// owed to the host. A finished commit leaves its result in `reserved_commit_outcome`.
    pub(super) async fn step_framework_reserved_commit(&mut self) -> Result<(), Fault> {
        if self.pending.reserved_commit_outcome.is_some() {
            return Ok(());
        }
        let Some(mut commit) = self.pending.reserved_commits.pop_front() else {
            return Ok(());
        };
        match self.run_framework_reserved_commit_unit(&mut commit).await {
            Ok(Some(result)) => {
                let result = self.as_render().finish_recorded(commit.log_generation_before, &commit.action, result).await;
                let _ = self.tools.typed_completion_outbox.push(TypedOperationCompletionWitness { operation: commit.permit.operation.operation.0, ui_scope: result.ui_scope.clone() });
                commit.permit.finish();
                self.pending.reserved_commit_outcome = Some(Ok(result));
            }
            Ok(None) => {
                commit.applied = commit.applied.saturating_add(1);
                self.pending.reserved_commits.push_front(commit);
            }
            Err(fault) => {
                commit.permit.finish();
                self.pending.reserved_commit_outcome = Some(Err(fault));
            }
        }
        Ok(())
    }

    async fn run_framework_reserved_commit_unit(&mut self, commit: &mut FrameworkReservedCommit) -> Result<Option<InvocationResult>, Fault> {
        let receipt = commit.permit.identity.take().ok_or_else(|| plugin_sdk_fault("reserved commit lost its authoring receipt"))?;
        let token = commit.permit.lease.token.clone();
        let publication = commit.permit.lease.publication_claim.clone();
        let mut progress = commit.permit.identity_progress;
        let mut observer = |next: semio_framework_value::native_encoding::NativeEncodeProgress| { progress = next; !token.is_cancelled_now() && !publication.is_cancelled() };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume(receipt, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>);
        let result = match self.run_framework_reserved_commit_unit_admitted(commit, &mut identity).await {
            Ok(output) => self.child_composition.follow_derivable_children(&mut *self.stores, &mut identity).await.map(|_| output),
            Err(fault) => Err(fault),
        };
        commit.permit.identity = Some(identity.pause().map_err(|error| plugin_sdk_fault(error.to_string()))?);
        drop(observer);
        commit.permit.identity_progress = progress;
        result
    }

    async fn run_framework_reserved_commit_unit_admitted(&mut self, commit: &mut FrameworkReservedCommit, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Option<InvocationResult>, Fault> {
        if commit.permit.is_cancelled().await {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancelled"), format!("framework route '{}' was cancelled between commit units", commit.action)));
        }
        if matches!(commit.stage, FrameworkReservedCommitStage::Validate) {
            self.stores.validate_framework_reserved_commit(X::reserved_action_refusal(&*self.time_travel, &*self.tool_runs, &commit.action), &commit.action, &commit.permit).await?;
            commit.stage = match history_command::<A>(&commit.action, commit.args.as_ref()).await {
                Some(ArtifactCommand::CommitCheckpoint { message, authors }) if !self.child_composition.children.is_empty() => {
                    let keys: Vec<MemberKey> = self.child_composition.children.keyed_entries().map(|(key, _)| key.to_key()).collect();
                    self.child_composition.admit_child_content_publication_span(keys.len())?;
                    commit.total = keys.len() as u64 + 1;
                    FrameworkReservedCommitStage::CheckpointChildren { message: message.unwrap_or_else(|| "checkpoint".to_string()), authors, keys, next: 0, pins: Vec::new() }
                }
                _ => {
                    commit.total = 1;
                    FrameworkReservedCommitStage::Route { pins: Vec::new() }
                }
            };
        }
        match std::mem::replace(&mut commit.stage, FrameworkReservedCommitStage::Validate) {
            FrameworkReservedCommitStage::Validate => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-commit-stage"), format!("framework route '{}' re-entered its validation unit", commit.action))),
            FrameworkReservedCommitStage::CheckpointChildren { message, authors, keys, next, mut pins } => {
                if let Some(key) = keys.get(next) {
                    if let Some(pin) = self.child_composition.checkpoint_child_unit(&message, &authors, key.borrowed(), identity).await? {
                        pins.push(pin);
                    }
                }
                commit.stage = if next + 1 < keys.len() {
                    FrameworkReservedCommitStage::CheckpointChildren { message, authors, keys, next: next + 1, pins }
                } else {
                    pins.sort_by(|left, right| left.child_ref.artifact_id.cmp(&right.child_ref.artifact_id));
                    FrameworkReservedCommitStage::Route { pins }
                };
                Ok(None)
            }
            FrameworkReservedCommitStage::Route { pins } => {
                let action = commit.action.clone();
                if HISTORY_ACTION_IDS.contains(&action.as_str()) {
                    let result = self.commit_framework_history_route(&action, commit.args.as_ref(), &commit.meta, &commit.permit, pins, identity).await?;
                    if action == "checkoutCheckpoint" && result.ui_scope != UiDirtyScope::None {
                        let pins = self.child_composition.checkout_cascade_pins(&mut *self.stores)?;
                        if !pins.is_empty() {
                            commit.total = commit.total.saturating_add(pins.len() as u64);
                            commit.stage = FrameworkReservedCommitStage::CheckoutChildren { pins, next: 0, result };
                            return Ok(None);
                        }
                    }
                    Ok(Some(result))
                } else if action == REVERT_TO_COMMAND_ACTION_ID {
                    match self.begin_framework_revert_route(commit.args.as_ref(), &commit.meta, identity).await? {
                        Ok(result) => Ok(Some(result)),
                        Err(edit_id) => {
                            commit.stage = FrameworkReservedCommitStage::Revert { edit_id };
                            Ok(None)
                        }
                    }
                } else {
                    self.commit_framework_shared_host_route(&action, commit.args.as_ref(), &commit.meta, &commit.permit, identity).await.map(Some)
                }
            }
            FrameworkReservedCommitStage::CheckoutChildren { pins, next, result } => {
                let Some(pin) = pins.get(next) else { return Ok(Some(result)) };
                self.child_composition.checkout_child_unit(pin, identity).await?;
                commit.stage = FrameworkReservedCommitStage::CheckoutChildren { pins, next: next + 1, result };
                Ok(None)
            }
            FrameworkReservedCommitStage::Revert { edit_id } => {
                if self.stores.framework_revert_unit(&edit_id, identity).await? {
                    commit.total = commit.total.saturating_add(1);
                    commit.stage = FrameworkReservedCommitStage::Revert { edit_id };
                    return Ok(None);
                }
                self.history.cache = None;
                self.tools.record_command(&mut *self.history, REVERT_TO_COMMAND_ACTION_ID, ActionKind::History, None, None, Vec::new(), None);
                Ok(Some(ToolOperations::<A>::empty_result(REVERT_TO_COMMAND_ACTION_ID, &commit.meta, Vec::new(), vec![history_changed_event().await], UiDirtyScope::Full).await))
            }
        }
    }

    /// 🧩️ A pending CHILD redo outranks a chrome replay for the same reason a parent one
    /// does: the document stack is reapplied before the shell rows stacked above it.
    ///
    /// 🧩️ A `Child`-lane row is a real undo target too, so chrome undo never jumps OVER
    /// the user's own composed-document edit to replay a panel toggle behind it.
    async fn dispatch_chrome_history_action(&mut self, action: &str, meta: &ActionMeta) -> Result<Option<InvocationResult>, Fault> {
        if action == "redo" {
            if !self.stores.store.redo_edit_ids().is_empty() || self.child_composition.child_history_tails().await.1 {
                return Ok(None);
            }
            let Some(replay) = self.history.shell_redo.pop() else {
                return Ok(None);
            };
            crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] chrome history action=redo seq={}", replay.seq));
            self.history.shell_undone.remove(&replay.seq);
            self.history.history_dirty_sequences.insert(replay.seq);
            self.tools.record_command(&mut *self.history, action, ActionKind::History, None, None, Vec::new(), None);
            return Ok(Some(ToolOperations::<A>::empty_result(action, meta, vec![Effect::ReplayShellCommand { action_id: replay.redo.action_id, args: replay.redo.args }], vec![history_changed_event().await], UiDirtyScope::Full).await));
        }
        if action != "undo" {
            return Ok(None);
        }
        let (child_applied_tails, _) = self.child_composition.child_history_tails().await;
        let applied: HashSet<&str> = self.stores.store.applied_edit_ids().iter().map(String::as_str).collect();
        let target = self.history.command_log.iter().rev().find(|entry| {
            let shell = entry.inverse.is_some() && entry.edit_id.is_none() && !self.history.shell_undone.contains(&entry.seq);
            let document = entry.edit_id.as_deref().is_some_and(|id| applied.contains(id));
            let child = entry.child_edit_ids.iter().any(|id| child_applied_tails.contains(id));
            shell || document || child
        });
        let Some(entry) = target else {
            return Ok(None);
        };
        if entry.edit_id.is_some() || entry.inverse.is_none() {
            return Ok(None);
        }
        let seq = entry.seq;
        let inverse = entry.inverse.clone().expect("chrome undo target has inverse");
        let redo = InverseAction { action_id: entry.action_id.clone(), args: None };
        crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] chrome history action=undo seq={seq} inverse={}", inverse.action_id));
        self.history.shell_undone.insert(seq);
        self.history.shell_redo.push(ShellHistoryReplay { seq, redo });
        self.history.history_dirty_sequences.insert(seq);
        self.tools.record_command(&mut *self.history, action, ActionKind::History, None, None, Vec::new(), None);
        Ok(Some(ToolOperations::<A>::empty_result(action, meta, vec![Effect::ReplayShellCommand { action_id: inverse.action_id, args: inverse.args }], vec![history_changed_event().await], UiDirtyScope::Full).await))
    }

    async fn commit_framework_history_route(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, permit: &FrameworkReservedCommitPermit, pins: Vec<vcs::CompositionPin>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] history route action={action}"));
        if permit.lease.is_cancelled().await {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancelled"), format!("framework route '{action}' was cancelled at commit")));
        }
        if action == "undo" || action == "redo" {
            if let Some(result) = self.as_time_travel().dispatch_supersede_history_action(action, meta, identity).await? {
                return Ok(result);
            }
            if let Some(result) = self.dispatch_chrome_history_action(action, meta).await? {
                return Ok(result);
            }
            let parent_group_id = if action == "undo" { self.stores.store.tail_group_id().await } else { self.stores.store.redo_tail().await.and_then(|(_, group_id)| group_id) };
            let group_id = match parent_group_id {
                Some(group_id) => Some(group_id),
                None => self.child_composition.child_group_history_target(&*self.history, action).await,
            };
            if let Some(group_id) = group_id {
                let grouped = self.as_dispatch().dispatch_group_history_action(action, &group_id, meta, identity).await?;
                if grouped.ui_scope != UiDirtyScope::None {
                    return Ok(grouped);
                }
            }
        }
        let command = history_command::<A>(action, args).await.ok_or_else(|| format!("history action {action} missing required argument"))?;
        match self.stores.store.dispatch(command, identity).await {
            Ok(_) => {
                self.stores.stamp_checkpoint_composition_pins(pins).await?;
                self.history.cache = None;
                if self.stores.store.local_step_pending() {
                    self.as_time_travel().defer_history_step_row(action);
                } else {
                    self.tools.record_command(&mut *self.history, action, ActionKind::History, None, None, Vec::new(), None);
                }
                Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), vec![history_changed_event().await], UiDirtyScope::Full).await)
            }
            Err(vcs::VcsError::NothingToUndo) | Err(vcs::VcsError::NothingToRedo) | Err(vcs::VcsError::ForeignEdit(_)) => Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await),
            Err(vcs::VcsError::ValidationFailed(ref message)) if message.as_str() == vcs::os_store::document_materialize::EMPTY_CHECKPOINT_MESSAGE => Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await),
            Err(error) => Err(error.into_fault()),
        }
    }

    /// ⏪️ Resolves a `revertToCommand` target. A document or configuration edit answers the lane and
    /// edit id [`Self::framework_revert_unit`] walks back one undo per commit unit; every other target
    /// answers its complete result.
    async fn begin_framework_revert_route(&mut self, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Result<InvocationResult, String>, Fault> {
        let action = REVERT_TO_COMMAND_ACTION_ID;
        self.as_render().refresh_cache().await?;
        let entry_seq = args.and_then(|value| value.get("entrySeq")).and_then(DslValue::as_f64).map(|seq| seq as u64);
        let transition_id = entry_seq.and_then(|seq| self.history.command_log.iter().find(|entry| entry.seq == seq)).and_then(|entry| entry.transition_id.clone());
        if let Some(transition_id) = transition_id {
            return Ok(Ok(self.as_time_travel().revert_history_edit_row(&transition_id, meta, identity).await?));
        }
        let target = entry_seq.and_then(|seq| self.history.cache.as_ref().and_then(|(_, _, _, history)| history.commands.iter().find(|entry| entry.seq == seq && entry.revertible)).map(|entry| (entry.edit_id.clone(), entry.kind, entry.inverse.clone())));
        let edit_id = match target {
            Some((Some(edit_id), _, _)) => edit_id,
            Some((None, ActionKind::Shell, Some(inverse))) => return Ok(Ok(ToolOperations::<A>::empty_result(action, meta, vec![Effect::ReplayShellCommand { action_id: inverse.action_id, args: inverse.args }], Vec::new(), UiDirtyScope::None).await)),
            _ => return Ok(Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await)),
        };
        if !self.stores.store.applied_edit_ids().iter().any(|id| *id == edit_id) {
            return Ok(Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await));
        }
        Ok(Err(edit_id))
    }

    async fn commit_framework_shared_host_route(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, permit: &FrameworkReservedCommitPermit, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        self.stores.validate_framework_reserved_commit(X::reserved_action_refusal(&*self.time_travel, &*self.tool_runs, action), action, permit).await?;
        if INTERACTION_ACTION_IDS.contains(&action) {
            return self.as_render().dispatch_interaction_action(action, args, meta, permit, identity).await;
        }
        if action == SET_HISTORY_COMMAND_FILTER_ACTION_ID {
            let value = args.and_then(|value| value.get("value")).and_then(DslValue::as_str).unwrap_or(semio_framework::HISTORY_COMMAND_FILTER_ALL);
            self.history.history_filter = HistoryCommandFilter::from_value(value).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("history-filter.unknown"), format!("'{value}' is no declared history filter option")))?;
            self.history.log_generation += 1;
            self.history.history_dirty_sequences.extend(self.history.command_log.iter().map(|entry| entry.seq));
            return Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: vec![FRAMEWORK_HISTORY_BODY_KEY.to_string()], utilities: false, tools: false, engagements: false, measures: false, labels: false })
            .await);
        }
        if action == NOTE_SHELL_COMMAND_ACTION_ID {
            let command_id = args.and_then(|value| value.get("commandId")).and_then(DslValue::as_str).ok_or_else(|| "noteShellCommand missing required commandId".to_string())?;
            let detail = args.and_then(|value| value.get("detail")).cloned();
            let label = note_shell_command_label(args.and_then(|value| value.get("label")), command_id, detail.as_ref());
            let inverse = args
                .and_then(|value| value.get("inverseCommandId"))
                .and_then(DslValue::as_str)
                .map(|inverse_command_id| InverseAction { action_id: inverse_command_id.to_string(), args: args.and_then(|value| value.get("inverseArgs")).cloned().or(detail) });
            self.stores.validate_framework_reserved_commit(X::reserved_action_refusal(&*self.time_travel, &*self.tool_runs, action), action, permit).await?;
            self.tools.record_command(&mut *self.history, command_id, ActionKind::Shell, Some(label), None, Vec::new(), inverse);
            return Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await);
        }
        if action == RECORD_TUTORIAL_ACTION_ID {
            return Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await);
        }
        Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unknown-shared-host-route"), format!("unknown shared host route '{action}'")))
    }

    /// 🕰️ The actual body of `PluginApp::handle_action` — renamed to an inherent method so
    /// `handle_action` itself can stay a thin `finish_recorded` wrapper (see `🔖️CommandLog`). B1:
    /// FRAMEWORK-reserved verbs only (history/revert/filter/noteShellCommand/clipboard/interaction) —
    /// an app's own behavior is dispatched exclusively through `dispatch_typed_command` now.
    pub(super) async fn dispatch_catalogue_example(&mut self, action: &str, body: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let snapshot = catalogue_snapshot_from_document::<A::Snapshot>(body)?;
        let pack = snapshot.encode_pack();
        let spr = store::os_store::text_format::empty_document_spr("", A::DOCUMENT_SCHEMA).await;
        self.as_dispatch().dispatch_emit(action, Emit { effects: vec![Effect::LoadDocument { pack, spr }], ui_scope: UiDirtyScope::Full, ..Emit::default() }, meta, identity, cx).await
    }
}
