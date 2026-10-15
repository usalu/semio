//! 🧷️ Command routes of the mounted artifact instance: framework-reserved routes, action dispatch and command dispatch.

use super::*;

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🧮️ One reserved gesture's work, in the STAGED EDIT ROWS every other capacity in the
    /// runtime is counted in — never the bare item count the route used to hand its job, which
    /// no longer matched the two-row footprint its own config preflight declares.
    async fn framework_reserved_work_items(&mut self, action: &str, args: Option<&DslValue>) -> Result<usize, Fault> {
        let items = self.framework_reserved_route_items(action, args).await?;
        FRAMEWORK_RESERVED_ROUTE_CAPACITY.rows_for_items(items).ok_or_else(|| plugin_sdk_fault(format!("framework reserved route '{action}' declares {items} items beyond its exact work capacity")))
    }

    async fn framework_reserved_route_items(&mut self, action: &str, args: Option<&DslValue>) -> Result<usize, Fault> {
        match action {
            "commitCheckpoint" | "checkoutCheckpoint" => Ok(self.children.len().saturating_add(1)),
            REVERT_TO_COMMAND_ACTION_ID => {
                self.refresh_cache().await?;
                let entry_seq = args.and_then(|value| value.get("entrySeq")).and_then(DslValue::as_f64).map(|seq| seq as u64);
                let target = entry_seq.and_then(|seq| self.cache.as_ref().and_then(|(_, _, _, history)| history.commands.iter().find(|entry| entry.seq == seq && entry.revertible)).and_then(|entry| entry.edit_id.clone()));
                match target {
                    Some(edit_id) => {
                        let applied = self.store.applied_edit_ids();
                        Ok(applied.iter().position(|id| *id == edit_id).map_or(1, |position| applied.len().saturating_sub(position + 1).max(1)))
                    }
                    None => Ok(1),
                }
            }
            "copy" | "cut" => {
                let interaction = self.interaction_selection_snapshot();
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

    /// 🖱️ Vortex pointermove is a storm of `interactionHover` (then a click `interactionSelect`)
    /// before the host finishes any Isolated reserved job. Exclusive `job % 64` slots made that
    /// storm `interactive-job.reserved-spawn-capacity`. Latest-wins: retire the same verb, and
    /// any other interaction occupying this hash slot, so hover preview and place can commit.
    fn retire_pending_reserved_latest_wins(&mut self, action: &str, job: u64) {
        if !INTERACTION_ACTION_IDS.contains(&action) {
            return;
        }
        let slot = job as usize % ARTIFACT_LIVE_OUTPUT_SLOTS;
        let mut ids = Vec::new();
        self.pending_reserved.each_id(|id| ids.push(id));
        let retire: Vec<u64> =
            ids.into_iter().filter(|&id| self.pending_reserved.get(id).is_some_and(|pending| pending.action == action || (id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS == slot && INTERACTION_ACTION_IDS.contains(&pending.action.as_str())))).collect();
        for id in retire {
            if let Some(pending) = self.pending_reserved.remove(id) {
                pending.permit.finish();
            }
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
    async fn dispatch_framework_reserved_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
        if self.time_travel.freezes_local_emits() && time_travel_freezes_reserved_action(action) {
            return Err(time_travel_frozen_fault(action).into());
        }
        let contract = self.qualified_tool_proof(action)?.contract();
        let decoded_items = bounded_json_items(args, contract.max_decoded_items)?;
        let wire_args = args.cloned();
        let raw = semio_framework_pack_json::to_json_string(&(action, wire_args)).into_bytes();
        let work_items = self.framework_reserved_work_items(action, args).await?;
        if CLIPBOARD_ACTION_IDS.contains(&action) {
            let Some(authority)=self.admit_reserved_emit_producer(action,meta,cx)?else{return Ok(None)};
            let producer = match self.build_artifact_reserved_action_job(action, args, raw.clone(), meta).await { Ok(producer) => producer, Err(fault) => { self.finish_reserved_emit_producer(authority); return Err(fault.into()); } };
            let mut app_completion = None;
            let job = match producer {
                Some((job, completion)) => {
                    app_completion = Some(completion);
                    job
                }
                None => { self.finish_reserved_emit_producer(authority); return Err(plugin_sdk_fault("clipboard producer has no exact installed reserved builder").into()); },
            };
            return self.dispatch_reserved_original_producer(action,&raw,decoded_items,job,app_completion.expect("installed clipboard producer owns its original completion"),meta,None,authority,cx).await.map(Some).map_err(Into::into);
        }
        let Some(permit)=self.admit_framework_reserved_spawn(action,&raw,decoded_items,meta,cx).await?else{return Ok(None)};
        let mut job = permit.operation.operation.0;
        self.retire_pending_reserved_latest_wins(action, job);
        assert!(self.pending_reserved.can_insert(job),"original reserved admission seals its exact vacant spawn slot");
        self.pending_reserved.insert_admitted(job, PendingFrameworkReserved { action: action.to_string(), args: args.cloned(), meta: meta.clone(), permit });
        self.release_reserved_emit_slot(job);
        let mut admitted = Self::empty_result(
            action,
            meta,
            vec![Effect::SpawnJob { job, kind: FRAMEWORK_RESERVED_JOB_KIND.into(), input: encode_framework_reserved_job_input(work_items as u64, &raw), placement: semio_framework::kernel::JobPlacement::Isolated }],
            Vec::new(),
            UiDirtyScope::None,
        )
        .await;
        admitted.output = DslValue::Object(vec![("operationId".into(), DslValue::String(job.to_string())), ("generation".into(), DslValue::String(self.store.generation().to_string()))]);
        Ok(Some(admitted))
    }

    async fn admit_framework_reserved_spawn(&mut self, verb: &str, raw: &[u8], decoded_items: usize, meta: &ActionMeta,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<FrameworkReservedCommitPermit>, semio_framework_job::OriginalTurnFailure> {
        let proof = self.qualified_tool_proof(verb)?;
        if !matches!(proof, QualifiedToolProof::FrameworkOwned(_) | QualifiedToolProof::AppOwned(_)) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-proof"), format!("framework route '{verb}' did not resolve its route-specific factory")).into());
        }
        let key = proof.key();
        let admission = self.tool_jobs.admit_exact_wire(key.controller_id.clone(), key.tool_id.clone(), proof.schema_id(), raw).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?;
        if decoded_items > proof.contract().max_decoded_items || !proof.admits::<A>(&admission) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-admission"), format!("framework route '{verb}' failed its exact owner/factory/schema/contract admission")).into());
        }
        let Some((operation,lease))=self.admit_original_tool_lease(verb,meta,false,cx)?else{return Ok(None)};
        FrameworkReservedCommitPermit::new(operation,lease,proof.contract().max_output_bytes).map(Some).map_err(Into::into)
    }

    /// 🕰️ Hands one finished framework-reserved spawn-job to the app's commit queue. Nothing here
    /// suspends: the commit itself runs unit by unit in [`Self::step_framework_reserved_commit`].
    /// Answers `false` for a job this app never admitted.
    pub(super) fn admit_framework_reserved_commit(&mut self, job: u64, output: Result<Vec<u8>, Fault>) -> Result<bool, Fault> {
        let Some(pending) = self.pending_reserved.remove(job) else {
            return Ok(false);
        };
        if let Err(fault) = output {
            pending.permit.finish();
            return Err(fault);
        }
        if self.reserved_commits.len() >= ARTIFACT_LIVE_OUTPUT_SLOTS {
            pending.permit.finish();
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-commit-capacity"), format!("framework route '{}' found the reserved commit queue full", pending.action)));
        }
        let PendingFrameworkReserved { action, args, meta, permit } = pending;
        self.reserved_commits.push_back(FrameworkReservedCommit { action, args, meta, permit, log_generation_before: self.log_generation, stage: FrameworkReservedCommitStage::Validate, applied: 0, total: 1 });
        Ok(true)
    }

    /// 📊️ Progress of the commit at the head of the reserved-commit queue.
    pub fn framework_reserved_commit_progress(&self) -> Option<FrameworkReservedCommitProgress> {
        self.reserved_commits.front().map(|commit| FrameworkReservedCommitProgress { operation: commit.permit.operation.operation.0, applied: commit.applied, total: commit.total })
    }

    /// 🪜️ Runs ONE bounded unit of the commit at the head of the queue. Commits are strictly FIFO,
    /// and the head waits while a finished outcome is still untaken, so at most one outcome is ever
    /// owed to the host. A finished commit leaves its result in `reserved_commit_outcome`.
    pub(super) async fn step_framework_reserved_commit(&mut self) -> Result<(), Fault> {
        if self.reserved_commit_outcome.is_some() {
            return Ok(());
        }
        let Some(mut commit) = self.reserved_commits.pop_front() else {
            return Ok(());
        };
        match self.run_framework_reserved_commit_unit(&mut commit).await {
            Ok(Some(result)) => {
                let result = self.finish_recorded(commit.log_generation_before, &commit.action, result).await;
                let _ = self.typed_completion_outbox.push(TypedOperationCompletionWitness { operation: commit.permit.operation.operation.0, ui_scope: result.ui_scope.clone() });
                commit.permit.finish();
                self.reserved_commit_outcome = Some(Ok(result));
            }
            Ok(None) => {
                commit.applied = commit.applied.saturating_add(1);
                self.reserved_commits.push_front(commit);
            }
            Err(fault) => {
                commit.permit.finish();
                self.reserved_commit_outcome = Some(Err(fault));
            }
        }
        Ok(())
    }

    /// 🧾️ Takes the result of the last finished reserved commit.
    pub(super) fn take_framework_reserved_commit_outcome(&mut self) -> Option<Result<InvocationResult, Fault>> {
        self.reserved_commit_outcome.take()
    }

    async fn run_framework_reserved_commit_unit(&mut self, commit: &mut FrameworkReservedCommit) -> Result<Option<InvocationResult>, Fault> {
        let receipt = commit.permit.identity.take().ok_or_else(|| plugin_sdk_fault("reserved commit lost its authoring receipt"))?;
        let token = commit.permit.lease.token.clone();
        let publication = commit.permit.lease.publication_claim.clone();
        let mut progress = commit.permit.identity_progress;
        let mut observer = |next: semio_framework_value::native_encoding::NativeEncodeProgress| { progress = next; !token.is_cancelled_now() && !publication.is_cancelled() };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume(receipt, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>);
        let result = match self.run_framework_reserved_commit_unit_admitted(commit, &mut identity).await {
            Ok(output) => self.follow_derivable_children(&mut identity).await.map(|_| output),
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
            self.validate_framework_reserved_commit(&commit.action, &commit.permit).await?;
            commit.stage = match Self::history_command(&commit.action, commit.args.as_ref()).await {
                Some(ArtifactCommand::CommitCheckpoint { message, authors }) if !self.children.is_empty() => {
                    let keys: Vec<MemberKey> = self.children.keyed_entries().map(|(key, _)| key.to_key()).collect();
                    self.admit_child_content_publication_span(keys.len())?;
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
                    if let Some(pin) = self.checkpoint_child_unit(&message, &authors, key.borrowed(), identity).await? {
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
                        let pins = self.checkout_cascade_pins()?;
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
                self.checkout_child_unit(pin, identity).await?;
                commit.stage = FrameworkReservedCommitStage::CheckoutChildren { pins, next: next + 1, result };
                Ok(None)
            }
            FrameworkReservedCommitStage::Revert { edit_id } => {
                if self.framework_revert_unit(&edit_id, identity).await? {
                    commit.total = commit.total.saturating_add(1);
                    commit.stage = FrameworkReservedCommitStage::Revert { edit_id };
                    return Ok(None);
                }
                self.cache = None;
                self.record_command(REVERT_TO_COMMAND_ACTION_ID, ActionKind::History, None, None, Vec::new(), None);
                Ok(Some(Self::empty_result(REVERT_TO_COMMAND_ACTION_ID, &commit.meta, Vec::new(), vec![history_changed_event().await], UiDirtyScope::Full).await))
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
            if !self.store.redo_edit_ids().is_empty() || self.child_history_tails().await.1 {
                return Ok(None);
            }
            let Some(replay) = self.shell_redo.pop() else {
                return Ok(None);
            };
            crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] chrome history action=redo seq={}", replay.seq));
            self.shell_undone.remove(&replay.seq);
            self.history_dirty_sequences.insert(replay.seq);
            self.record_command(action, ActionKind::History, None, None, Vec::new(), None);
            return Ok(Some(Self::empty_result(action, meta, vec![Effect::ReplayShellCommand { action_id: replay.redo.action_id, args: replay.redo.args }], vec![history_changed_event().await], UiDirtyScope::Full).await));
        }
        if action != "undo" {
            return Ok(None);
        }
        let (child_applied_tails, _) = self.child_history_tails().await;
        let applied: HashSet<&str> = self.store.applied_edit_ids().iter().map(String::as_str).collect();
        let target = self.command_log.iter().rev().find(|entry| {
            let shell = entry.inverse.is_some() && entry.edit_id.is_none() && !self.shell_undone.contains(&entry.seq);
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
        self.shell_undone.insert(seq);
        self.shell_redo.push(ShellHistoryReplay { seq, redo });
        self.history_dirty_sequences.insert(seq);
        self.record_command(action, ActionKind::History, None, None, Vec::new(), None);
        Ok(Some(Self::empty_result(action, meta, vec![Effect::ReplayShellCommand { action_id: inverse.action_id, args: inverse.args }], vec![history_changed_event().await], UiDirtyScope::Full).await))
    }

    async fn commit_framework_history_route(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, permit: &FrameworkReservedCommitPermit, pins: Vec<vcs::CompositionPin>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] history route action={action}"));
        if permit.lease.is_cancelled().await {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancelled"), format!("framework route '{action}' was cancelled at commit")));
        }
        if action == "undo" || action == "redo" {
            if let Some(result) = self.dispatch_supersede_history_action(action, meta, identity).await? {
                return Ok(result);
            }
            if let Some(result) = self.dispatch_chrome_history_action(action, meta).await? {
                return Ok(result);
            }
            let parent_group_id = if action == "undo" { self.store.tail_group_id().await } else { self.store.redo_tail().await.and_then(|(_, group_id)| group_id) };
            let group_id = match parent_group_id {
                Some(group_id) => Some(group_id),
                None => self.child_group_history_target(action).await,
            };
            if let Some(group_id) = group_id {
                let grouped = self.dispatch_group_history_action(action, &group_id, meta, identity).await?;
                if grouped.ui_scope != UiDirtyScope::None {
                    return Ok(grouped);
                }
            }
        }
        let command = Self::history_command(action, args).await.ok_or_else(|| format!("history action {action} missing required argument"))?;
        match self.store.dispatch(command, identity).await {
            Ok(_) => {
                self.stamp_checkpoint_composition_pins(pins).await?;
                self.cache = None;
                if self.store.local_step_pending() {
                    self.defer_history_step_row(action);
                } else {
                    self.record_command(action, ActionKind::History, None, None, Vec::new(), None);
                }
                Ok(Self::empty_result(action, meta, Vec::new(), vec![history_changed_event().await], UiDirtyScope::Full).await)
            }
            Err(vcs::VcsError::NothingToUndo) | Err(vcs::VcsError::NothingToRedo) | Err(vcs::VcsError::ForeignEdit(_)) => Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await),
            Err(vcs::VcsError::ValidationFailed(ref message)) if message.as_str() == vcs::os_store::document_materialize::EMPTY_CHECKPOINT_MESSAGE => Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await),
            Err(error) => Err(error.into_fault()),
        }
    }

    /// ⏪️ Resolves a `revertToCommand` target. A document or configuration edit answers the lane and
    /// edit id [`Self::framework_revert_unit`] walks back one undo per commit unit; every other target
    /// answers its complete result.
    async fn begin_framework_revert_route(&mut self, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Result<InvocationResult, String>, Fault> {
        let action = REVERT_TO_COMMAND_ACTION_ID;
        self.refresh_cache().await?;
        let entry_seq = args.and_then(|value| value.get("entrySeq")).and_then(DslValue::as_f64).map(|seq| seq as u64);
        let transition_id = entry_seq.and_then(|seq| self.command_log.iter().find(|entry| entry.seq == seq)).and_then(|entry| entry.transition_id.clone());
        if let Some(transition_id) = transition_id {
            return Ok(Ok(self.revert_history_edit_row(&transition_id, meta, identity).await?));
        }
        let target = entry_seq.and_then(|seq| self.cache.as_ref().and_then(|(_, _, _, history)| history.commands.iter().find(|entry| entry.seq == seq && entry.revertible)).map(|entry| (entry.edit_id.clone(), entry.kind, entry.inverse.clone())));
        let edit_id = match target {
            Some((Some(edit_id), _, _)) => edit_id,
            Some((None, ActionKind::Shell, Some(inverse))) => return Ok(Ok(Self::empty_result(action, meta, vec![Effect::ReplayShellCommand { action_id: inverse.action_id, args: inverse.args }], Vec::new(), UiDirtyScope::None).await)),
            _ => return Ok(Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await)),
        };
        if !self.store.applied_edit_ids().iter().any(|id| *id == edit_id) {
            return Ok(Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await));
        }
        Ok(Err(edit_id))
    }

    /// ⏪️ One revert unit: undoes the newest document edit while `edit_id` is not yet the document store's tail.
    /// Answers whether an undo was applied (`false` once the target is the tail or the store refuses).
    async fn framework_revert_unit(&mut self, edit_id: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<bool, Fault> {
        let applied = self.store.applied_edit_ids();
        if !(applied.iter().any(|id| id.as_str() == edit_id) && applied.last().is_some_and(|tail| tail.as_str() != edit_id)) {
            return Ok(false);
        }
        let undone = self.store.dispatch(ArtifactCommand::Undo, identity).await.map(|_| ());
        match undone {
            Ok(()) => Ok(true),
            Err(vcs::VcsError::NothingToUndo) | Err(vcs::VcsError::ForeignEdit(_)) => Ok(false),
            Err(error) => Err(error.into_fault()),
        }
    }

    pub(super) async fn commit_framework_clipboard_completion(&mut self,action:&str,completion:Option<ArtifactToolCompletion<A>>,meta:&ActionMeta,mut permit:FrameworkReservedCommitPermit, cx:&mut semio_framework_job::StepContext<'_>)->Result<InvocationResult,Fault>{
        let id=permit.operation.operation.0;
        let started=self.mount_original_reserved_owners(action,None,None,completion,meta,Some((permit.operation,permit.lease,permit.identity.take().ok_or_else(||plugin_sdk_fault("original reserved producer lost its authoring receipt"))?)), cx).await?;
        if let Some(fault)=permit.producer_fault{let mounted=self.tool_operations.get_mut(id).expect("original completion owns its admitted slot");mounted.terminal_fault.get_or_insert(fault);mounted.cancellation_lease.as_ref().expect("original completion lease").cancel();}
        self.finish_mounted_reserved_completion(id);
        Ok(started)
    }

    /// 📥️ Keeps the original producer and completion in their fixed slot through every fallible worker boundary.
    pub(super) async fn dispatch_reserved_original_producer(&mut self,verb:&str,raw:&[u8],decoded_items:usize,job:ArtifactReservedToolJob,completion:ArtifactToolCompletion<A>,meta:&ActionMeta,admission:Option<semio_framework::ToolWireAdmission>,mut permit:FrameworkReservedCommitPermit, cx:&mut semio_framework_job::StepContext<'_>)->Result<InvocationResult,Fault>{
        let id=permit.operation.operation.0;
        let started=self.mount_original_reserved_owners(verb,None,Some(job),Some(completion),meta,Some((permit.operation,permit.lease,permit.identity.take().ok_or_else(||plugin_sdk_fault("original reserved producer lost its authoring receipt"))?)), cx).await?;
        let proof=self.qualified_tool_proof(verb)?;
        let key=proof.key();
        let admitted=match admission{Some(admitted)=>admitted,None=>self.tool_jobs.admit_exact_wire(key.controller_id.clone(),key.tool_id.clone(),proof.schema_id(),raw).map_err(|error|plugin_sdk_fault(error.to_string()))?};
        if decoded_items>proof.contract().max_decoded_items||!proof.admits::<A>(&admitted){return Err(plugin_sdk_fault("original reserved producer failed its exact owner/factory/schema admission"));}
        let mounted=self.tool_operations.get_mut(id).expect("original producer owns its admitted slot");
        let job=mounted.reserved_producer.as_ref().expect("original reserved producer remains retained").clone();
        let dispatched=self.tool_jobs.dispatch(semio_framework::ToolOperationSpec::new(key.controller_id,key.tool_id,proof.schema_id(),job,mounted.operation)).map_err(|error|plugin_sdk_fault(error.to_string()))?;
        mounted.session_rejected=Some(semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(dispatched.job,mounted.source_identity));
        mounted.terminal_seen=false;
        mounted.stage=MountedTypedCommandFullOperationStage::Worker;
        Ok(started)
    }

    /// 📨️ Moves the original completion enum before validating its route-specific output kind.
    fn finish_mounted_reserved_completion(&mut self,id:u64){
        let mounted=self.tool_operations.get_mut(id).expect("original reserved completion retains its exact operation");
        let taken=mounted.completion.as_ref().map_or(Ok(None),ArtifactToolCompletion::take);
        let fault=match taken{
            Ok(Some(publication))=>{mounted.publication=Some(publication);match mounted.publication.as_ref().expect("original completion enum is retained"){
                ArtifactToolCompletionValue::Emit(Ok(_),_)=>None,
                ArtifactToolCompletionValue::Emit(Err(fault),_)=>Some(fault.bounded),
                ArtifactToolCompletionValue::Download(_,_)=>Some(ArtifactBoundedToolFault::from_fault(&plugin_sdk_fault("reserved producer completed with original download output"))),
            }},
            Ok(None)=>Some(ArtifactBoundedToolFault::from_fault(&plugin_sdk_fault("reserved producer has no completed original output"))),
            Err(fault)=>Some(ArtifactBoundedToolFault::from_fault(&fault)),
        };
        if let Some(fault)=fault{mounted.terminal_fault.get_or_insert(fault);mounted.cancellation_lease.as_ref().expect("original reserved completion lease").cancel();}
    }

    async fn commit_framework_shared_host_route(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, permit: &FrameworkReservedCommitPermit, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        self.validate_framework_reserved_commit(action, permit).await?;
        if INTERACTION_ACTION_IDS.contains(&action) {
            return self.dispatch_interaction_action(action, args, meta, permit, identity).await;
        }
        if action == SET_HISTORY_COMMAND_FILTER_ACTION_ID {
            let value = args.and_then(|value| value.get("value")).and_then(DslValue::as_str).unwrap_or(semio_framework::HISTORY_COMMAND_FILTER_ALL);
            self.history_filter = HistoryCommandFilter::from_value(value).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("history-filter.unknown"), format!("'{value}' is no declared history filter option")))?;
            self.log_generation += 1;
            self.history_dirty_sequences.extend(self.command_log.iter().map(|entry| entry.seq));
            return Ok(Self::empty_result(
                action,
                meta,
                Vec::new(),
                Vec::new(),
                UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: vec![FRAMEWORK_HISTORY_BODY_KEY.to_string()], utilities: false, tools: false, engagements: false, measures: false, labels: false },
            )
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
            self.validate_framework_reserved_commit(action, permit).await?;
            self.record_command(command_id, ActionKind::Shell, Some(label), None, Vec::new(), inverse);
            return Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await);
        }
        if action == RECORD_TUTORIAL_ACTION_ID {
            return Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await);
        }
        Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unknown-shared-host-route"), format!("unknown shared host route '{action}'")))
    }

    /// 🕰️ The actual body of `PluginApp::handle_action` — renamed to an inherent method so
    /// `handle_action` itself can stay a thin `finish_recorded` wrapper (see `🔖️CommandLog`). B1:
    /// FRAMEWORK-reserved verbs only (history/revert/filter/noteShellCommand/clipboard/interaction) —
    /// an app's own behavior is dispatched exclusively through `dispatch_typed_command` now.
    async fn dispatch_catalogue_example(&mut self, action: &str, body: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let snapshot = catalogue_snapshot_from_document::<A::Snapshot>(body)?;
        let pack = snapshot.encode_pack();
        let spr = store::os_store::text_format::empty_document_spr("", A::DOCUMENT_SCHEMA).await;
        self.dispatch_emit(action, Emit { effects: vec![Effect::LoadDocument { pack, spr }], ui_scope: UiDirtyScope::Full, ..Emit::default() }, meta, identity, cx).await
    }

    /// ⏳️ The `document.loading` refusal of `verb` while a whole-document load still runs (`📓️api-stepped-document-load.md`
    /// §3, gap N17): every verb but a view or interaction verb waits — nothing reads or edits the document being replaced;
    /// hover and pick read only the selection (audit W2A-13) — except `historyEditCancelReplay`, which cancels the load;
    /// `None` while no load runs.
    pub(crate) fn document_loading_refusal(&self, verb: &str) -> Option<Fault> {
        let mut loading = false;
        self.document_archive_loads.each_id(|operation| loading |= self.document_archive_loads.get(operation).is_some_and(ActiveDocumentArchiveLoad::loading));
        let exempt = matches!(self.declared_action_kind(verb), Some(ActionKind::View | ActionKind::Interaction)) || INTERACTION_ACTION_IDS.contains(&verb) || verb == semio_framework::HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID;
        (loading && !exempt).then(|| Fault::new(FaultOrigin::Framework, FaultCode::new(DOCUMENT_LOADING_CODE), format!("'{verb}' waits: the document is still loading")))
    }

    /// 🏷️ The kind an action or command `verb` is declared with; `None` for a verb this app does not declare.
    pub(super) fn declared_action_kind(&self, verb: &str) -> Option<ActionKind> {
        self.registry.get(verb).map(|definition| definition.kind).or_else(|| self.registry.get_command(verb).map(|command| command.kind))
    }

    pub(crate) async fn dispatch_action(&mut self, action: &str, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
        let Some(slot)=self.stage_action_source(action,source_args,original_fault,cx)?else{return Ok(None)};
        let Some(mut source)=self.action_ingress_sources.get_mut(slot).unwrap().loan_source(cx)?else{return Ok(None)};
        let result=self.dispatch_action_received(action,&mut source,meta,identity,cx).await;
        let terminal=!matches!(&result,Ok(None));
        self.action_ingress_sources.get_mut(slot).unwrap().return_source(std::mem::ManuallyDrop::into_inner(source),terminal);
        if terminal{self.pending_action_ingress_identity=None;}
        result
    }

    async fn dispatch_action_received(&mut self,action:&str,source:&mut crate::action_ingress::ActionIngressSource,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>{
        let args=source.assembled.as_ref().or(source.prepared.as_ref()).or(source.args.as_ref());
        if A::ROLE == AppRole::Viewer && viewer_rejects_action(action) {
            return Err((viewer_read_only_fault(action)).into());
        }
        if action == CANCEL_TYPED_OPERATION_ACTION_ID {
            return self.dispatch_operation_cancellation(args, meta).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        self.dismiss_refused_reprojection(action);
        if let Some(fault) = self.document_loading_refusal(action) {
            return Err((fault).into());
        }
        if self.time_travel.history_unavailable() && (HISTORY_ACTION_IDS.contains(&action) || action == REVERT_TO_COMMAND_ACTION_ID || is_time_travel_action_id(action)) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new(PURE_HISTORY_UNAVAILABLE_CODE), format!("'{action}' needs the document's history, which this head-only pure evaluation does not hold; use a live instance"))).into());
        }
        Box::pin(self.deliver_utility_host_events(meta, identity, cx)).await?;
        if action == semio_framework::HOST_EVENT_ACTION_ID {
            return Box::pin(self.dispatch_host_event_action(args, meta, identity, cx)).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        Box::pin(self.commit_typing_before(action, args, meta, identity, cx)).await?;
        if matches!(action, semio_framework::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID | semio_framework::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("framework.document-transfer.shell-owned"), format!("{action} is performed by the shell, never by the program"))).into());
        }
        if is_tool_run_action_id(action) {
            return self.dispatch_tool_run_action(action, args, meta).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if is_time_travel_action_id(action) {
            self.freeze_tool_machines();
            return Box::pin(self.dispatch_time_travel_action(action, args, meta, identity, cx)).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if is_framework_reserved_action_id(action) {
            return self.dispatch_framework_reserved_action(action,args,meta,cx).await;
        }
        if action == CATALOGUE_EXAMPLE_ACTION_ID && self.registry.get(action).is_none() {
            let example_id = args.and_then(|value| value.get("exampleId")).and_then(DslValue::as_str).unwrap_or("");
            if let Some(document) = A::catalogue_example_document(example_id) {
                let body = document?;
                return self.dispatch_catalogue_example(action, &body, meta, identity, cx).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
        }
        let definition = self.registry.get(action).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unknown-key"), format!("UI dispatch rejected unknown action key '{action}' before command construction")))?;
        validate_ui_dispatch_classification("action", action, definition.semantics.execution.interactive_job)?;
        let definition_kind = definition.kind;
        let tool = match Box::pin(self.admit_tool_dispatch(action, args, meta, identity, cx)).await {
            ToolDispatch::Plain => None,
            ToolDispatch::Tagged(tag) => Some(tag),
            ToolDispatch::Settled(result) => return result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from),
        };
        if source.assembled.is_none(){match self.import_staging.admit_args(args).map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), refusal.message()))? {
            semio_framework::kernel::ImportArguments::Staged { .. } => return Ok(Some(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await)),
            semio_framework::kernel::ImportArguments::Whole(whole) => source.assembled=Some(whole),
            semio_framework::kernel::ImportArguments::NotAnImport => (),
        }}
        let args=source.assembled.as_ref().or(source.prepared.as_ref()).or(source.args.as_ref());
        if let Some(config_mutation) = A::host_configuration_mutation(action, args)? {
            let admission = self.admit_host_configuration_json(action, args).await?;
            self.require_tool_operation_authority(&admission)?;
            let encoded = ::protocol::OpBinary::encode_op(&config_mutation).map_err(|error| error.into_fault())?;
            if encoded.len() > admission.proof.contract().max_output_bytes {
                return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.host-configuration-output"), format!("host configuration action '{action}' exceeds its exact output cap"))).into());
            }
            let mut emit = Emit::config(vec![config_mutation]);
            if let Some(ToolTag::Scrub(tag)) = &tool {
                let base = self.store.content_revision_now();
                let published = self.settle_press(tag, &base, &meta.actor, &mut emit)?;
                self.follow_tool_machines(true);
                if !published {
                    return Ok(Some(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::Full).await));
                }
            }
            let previous_kind = self.invocation_kind.replace(definition_kind);
            let result = self.dispatch_emit(action, emit, meta, identity, cx).await;
            self.invocation_kind = previous_kind;
            result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
        } else if matches!(action, SET_ACTIVE_TOOL_ACTION_ID | SET_ACTIVE_UTILITY_ACTION_ID) {
            let previous_kind = self.invocation_kind.replace(definition_kind);
            let result = self.dispatch_emit(action, Emit::default(), meta, identity, cx).await;
            self.invocation_kind = previous_kind;
            result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
        } else {
            let command=A::command_from_action(action,if source.assembled.is_some(){&mut source.assembled}else if source.prepared.is_some(){&mut source.prepared}else{&mut source.args},&mut source.fault).await?;
            let command=Box::new(command);
            let wire = <A::Command as ::protocol::OpBinary>::encode_op(&command).map_err(|error| error.into_fault())?;
            let admission = self.admit_command_wire(action, &wire, 1).await?;
            self.require_complete_tool_operation_pipeline(&admission)?;
            self.tool_machines.ingress = tool;
            let result = self.dispatch_typed_command_inner(command, admission, meta, identity, cx).await;
            self.tool_machines.ingress = None;
            result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
        }
    }

    /// 🎯️ Validates manifest structural app/mode ownership, converts the addressed
    /// invocation arguments once into the app's typed command, and enters the typed dispatch channel.
    pub(super) async fn dispatch_command(&mut self, address: &ManifestCommandAddress, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, active_mode_id: Option<&str>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let command_id = address.command_id.as_str();
        let owner_app_id = self.app.instance_id().await.to_string();
        let (owner, definition) = match &address.owner {
            ManifestCommandOwnerAddress::App { app_id, .. } => {
                if app_id != &owner_app_id {
                    return Err((Fault::from(format!("command '{command_id}' is not owned by app {owner_app_id}"))).into());
                }
                (format!("app {owner_app_id}"), self.registry.app_command(command_id).await.ok_or_else(|| Fault::from(format!("command '{command_id}' is not owned by app {owner_app_id}")))?)
            }
            ManifestCommandOwnerAddress::Mode { app_id, mode_id, .. } => {
                if app_id != &owner_app_id || active_mode_id != Some(mode_id.as_str()) {
                    return Err((Fault::from(format!("command '{command_id}' is not owned by active mode {mode_id} of app {owner_app_id}"))).into());
                }
                (format!("mode {mode_id}"), self.registry.mode_command(mode_id, command_id).await.ok_or_else(|| Fault::from(format!("command '{command_id}' is not owned by active mode {mode_id} of app {owner_app_id}")))?)
            }
            _ => return Err((Fault::from(format!("command '{command_id}' is not app- or mode-owned"))).into()),
        };
        validate_ui_dispatch_classification(&owner, command_id, definition.semantics.execution.interactive_job)?;
        let kind = definition.kind;
        let Some(slot)=self.stage_action_source(command_id,source_args,original_fault,cx)?else{return Ok(None)};
        let Some(mut source)=self.action_ingress_sources.get_mut(slot).unwrap().loan_source(cx)?else{return Ok(None)};
        let result=async {
        let args = source.args.as_ref();
        match self.import_staging.admit_args(args).map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), refusal.message()))? {
            semio_framework::kernel::ImportArguments::Staged { .. } => return Ok(Some(Self::empty_result(command_id, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await)),
            semio_framework::kernel::ImportArguments::Whole(whole) => source.assembled=Some(whole),
            semio_framework::kernel::ImportArguments::NotAnImport => (),
        };
        let args=source.assembled.as_ref().or(source.prepared.as_ref()).or(source.args.as_ref());
        if let Some(config_mutation) = A::host_configuration_mutation(command_id, args)? {
            let admission = self.admit_host_configuration_json(command_id, args).await?;
            self.require_tool_operation_authority(&admission)?;
            let encoded = ::protocol::OpBinary::encode_op(&config_mutation).map_err(|error| error.into_fault())?;
            if encoded.len() > admission.proof.contract().max_output_bytes {
                return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.host-configuration-output"), format!("host configuration command '{command_id}' exceeds its exact output cap"))).into());
            }
            let previous_kind = self.invocation_kind.replace(kind);
            let result = self.dispatch_emit(command_id, Emit::config(vec![config_mutation]), meta, identity, cx).await;
            self.invocation_kind = previous_kind;
            return result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        let crate::action_ingress::ActionIngressSource{assembled,prepared,args,fault,..}=&mut *source;
        let command=A::command_from_action(command_id,if assembled.is_some(){assembled}else if prepared.is_some(){prepared}else{args},fault).await?;
        let wire = <A::Command as ::protocol::OpBinary>::encode_op(&command).map_err(|error| error.into_fault())?;
        let admission = self.admit_command_wire(command_id, &wire, 1).await?;
        self.require_complete_tool_operation_pipeline(&admission)?;
        let previous_kind = self.invocation_kind.replace(kind);
        let result = self.dispatch_typed_command_inner(Box::new(command), admission, meta, identity, cx).await;
        self.invocation_kind = previous_kind;
        result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
        }.await;
        let terminal=!matches!(&result,Ok(None));
        self.action_ingress_sources.get_mut(slot).unwrap().return_source(std::mem::ManuallyDrop::into_inner(source),terminal);
        if terminal{self.pending_action_ingress_identity=None;}
        result
    }

    /// 🔐️ The agent lane's revision fill (ticket 26/09/23, G12 session 14c): a declared
    /// [`ActionArgDef::document_revision`] / [`ActionArgDef::target_revision`] argument the agent omitted is admitted against
    /// the token its rendered binding carries right now — the document's canonical revision (what every document-scoped
    /// verb compares against on this lane), or the app's own [`ArtifactApp::agent_target_revision`]; a target the app cannot
    /// resolve is refused by name. Only this lane fills: the shell lanes pass the binding's token and the app's parser
    /// refuses its absence there; the gateway admits an omission only behind a passed MCP `expectedRevision` (the
    /// document-level stale-write guard), and its prepare→invoke pair refuses a document that moved in between.
    async fn fill_agent_revisions(&mut self, address: &semio_framework::manifest::ActionAddress, arguments: &mut BTreeMap<String, DslValue>) -> Result<(), Fault> {
        let declared = match address.window_kind_id.as_str() {
            "*" => self.registry.get(&address.action_id).map(|action| action.args.clone()),
            kind => self.registry.window_action(kind, &address.action_id).await.map(|action| action.args.clone()),
        }
        .unwrap_or_default();
        let omitted: Vec<_> = declared.into_iter().filter(|arg| !arguments.contains_key(&arg.id)).collect();
        for arg in omitted.iter() {
            let semio_framework::manifest::ArgSchema::String { format: Some(format), .. } = &arg.schema else { continue };
            let token = match format {
                semio_framework::manifest::ArgFormat::DocumentRevision => self.store.content_revision().iter().map(|byte| format!("{byte:02x}")).collect::<String>(),
                semio_framework::manifest::ArgFormat::TargetRevision => {
                    self.refresh_cache().await?;
                    let render_operation = self.live_render_operation();
                    let parent_document_id = self.store.envelope().id.clone();
                    let staged = DslValue::Object(arguments.iter().map(|(key, value)| (key.clone(), value.clone())).collect());
                    let (_, snapshot, _, history) = self.cache.as_ref().expect("cache refreshed above");
                    let doc = ArtifactView::with_render_context(snapshot.as_ref(), history.as_ref(), ChildContentView::clone(&self.child_content_root), render_operation, parent_document_id, None).await.with_instance(self.instance_operation_owner.clone());
                    A::agent_target_revision(&address.action_id, &staged, &doc).await?.ok_or_else(|| {
                        Fault::new(
                            FaultOrigin::Framework,
                            FaultCode::new("agent.target-revision-unresolved"),
                            format!("action '{}' binds its `{}` to one addressed target and this app resolves no such target for an agent — pass the target's revision", address.action_id, arg.id),
                        )
                    })?
                }
                _ => continue,
            };
            arguments.insert(arg.id.clone(), DslValue::String(token));
        }
        Ok(())
    }

    /// 🧮️ PHASE ONE of the agent lane's two-phase contract: validate, price, and produce the ops
    /// **without applying any of them**.
    ///
    /// 🧭️ Why this cannot be `AppCommand::Command`. The shell's dispatch lane APPLIES: it runs
    /// `handle_action_invocation` → `dispatch_typed_command_inner`, which mounts a typed
    /// operation whose publication ladder commits into `self.store` and mints an undo group. An
    /// agent's `action_prepare` must not do that — its own protocol commits separately
    /// (`TransactionPrepare{ops}` → `TransactionCommit`), and `RoutingArtifactChannel` caches ONE
    /// guest instance per plugin, so a prepare that applied would leave the commit applying the
    /// same edit a second time. `history_undo` (which walks back exactly one application) would
    /// then leave half the edit standing — a GREEN gate that lies, which is worse than a red one.
    ///
    /// ✅️ What it shares with the shell lane, deliberately and by call rather than by copy:
    /// the owner-qualified `ManifestActionInvocation` address, `admit_addressed_action_view`'s
    /// window projection, the mode/window-kind ownership checks `handle_action_invocation` runs,
    /// the `windowId` argument it injects, `validate_ui_dispatch_classification`'s interactive-job
    /// discipline, `A::command_from_action`, the exact retained-wire admission `dispatch_action`
    /// begins (`admit_command_wire` + `require_complete_tool_operation_pipeline`) and the app's own
    /// job, built and dispatched through the same factory entry ([`Self::preview_typed_command_job`]).
    /// The one thing it does NOT share is the applying tail: the job runs to the completion it would
    /// publish, and nothing is published. It fails closed ([`agent_lane_carriage`]): a completion that publishes
    /// anything an agent transaction cannot carry is refused by name, never settled without it, and one that carries
    /// nothing but a notice or a selection is refused `app.command.no-effect`, its notices as structured causes.
    ///
    /// 🎟️ The admission it begins is consumed by the previewed job within this same call and retired
    /// through that job's own close protocol, so a prepared handle the caller never invokes leaves
    /// nothing behind.
    ///
    /// 🪟️ `"*"` is the catalog's own marker for an APP-scope verb — one that belongs to no
    /// particular window kind (`🗒️note` declares all 48 of its verbs this way). A concrete
    /// kind must own the verb; `"*"` addresses the app registry directly, exactly as
    /// `dispatch_action` does for every verb the shell sends.
    pub(crate) async fn preview_addressed_action(&mut self, invocation: &ManifestActionInvocation, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let address=&invocation.address;
        let owner_app_id = self.app.instance_id().await;
        if address.app_id != owner_app_id {
            return Err((plugin_sdk_fault(format!("action app owner {} does not match {owner_app_id}", address.app_id))).into());
        }
        if address.window_instance_id.trim().is_empty() {
            return Err((plugin_sdk_fault("action window instance id must be non-empty")).into());
        }
        let reserved_kind = framework_reserved_action_kind(&address.action_id);
        if reserved_kind.is_none() && !self.registry.has_mode(&address.mode_id).await {
            return Err((plugin_sdk_fault(format!("unknown action mode owner {}", address.mode_id))).into());
        }
        if address.window_kind_id != "*" && reserved_kind.is_none() && self.registry.window_action(&address.window_kind_id, &address.action_id).await.is_none() {
            return Err((plugin_sdk_fault(format!("window kind {} does not own action {}", address.window_kind_id, address.action_id))).into());
        }
        let context = meta.view_state.as_ref().ok_or_else(|| plugin_sdk_fault("addressed action requires the caller's explicit language and terminology context"))?;
        let view = crate::plugin_runtime::admit_addressed_action_view(context, &invocation.address)?;
        let definition = self
            .registry
            .get(&address.action_id)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unknown-key"), format!("UI dispatch rejected unknown action key '{}' before command construction", address.action_id)))?;
        let kind = definition.kind;
        validate_ui_dispatch_classification("action", &address.action_id, definition.semantics.execution.interactive_job)?;
        if self.agent_lane_preview.as_ref().is_none_or(|state|state.starting){
            if !self.admit_agent_lane_preview_key(meta,cx)?{return Ok(None)}
            if !self.action_source_capture_ready(&address.action_id,cx){return Ok(None)}
            let mut original_args=if self.action_source_pending(){None}else{let mut arguments=invocation.arguments.clone();arguments.insert("windowId".into(),DslValue::String(address.window_instance_id.clone()));self.fill_agent_revisions(address,&mut arguments).await?;
            Some(DslValue::Object(arguments.into_iter().collect()))};
            let Some(command)=self.receive_action_command(&address.action_id,&mut original_args,&mut None,cx).await?else{return Ok(None)};
            let wire=<A::Command as protocol::OpBinary>::encode_op(&command).map_err(|source|source.into_fault())?;
            let admission=self.admit_command_wire(&address.action_id,&wire,1).await?;self.require_complete_tool_operation_pipeline(&admission)?;
            self.begin_agent_lane_preview(Box::new(command),admission,&ActionMeta{view_state:Some(view),..meta.clone()},cx).await?;
            return Ok(None)
        }
        let maximum_bytes=self.agent_lane_preview.as_ref().unwrap().maximum_bytes;
        let Some((emit,(presence,transient,window_transient),job_steps))=self.advance_agent_lane_preview(&address.action_id,cx)?else{return Ok(None)};
        let carriage = agent_lane_carriage(&AgentLanePublication {
            carried_ops: emit.artifact_mutations.len() + emit.config_mutations.len() + emit.draft_mutations.len() + emit.child_emits.len(),
            effects: &emit.effects,
            window_config_ops: emit.window_config_mutations.len(),
            extension_calls: emit.extension_invocations.len(),
            events: emit.events.len(),
            selection_writes: emit.interaction_writes.len(),
            tasks: emit.tasks.len(),
            presence,
            transient,
            window_transient,
        });
        match carriage {
            AgentLaneCarriage::Carried => {}
            AgentLaneCarriage::NoEffect => return Err((agent_lane_no_effect_fault(&address.action_id, &emit.effects)).into()),
            AgentLaneCarriage::Uncarried(lanes) => return Err((agent_lane_uncarried_fault(&address.action_id, &lanes.join(", "))).into()),
        }
        if A::ROLE == AppRole::Viewer && (!emit.artifact_mutations.is_empty() || !emit.child_emits.is_empty()) {
            return Err((viewer_read_only_fault(&address.action_id)).into());
        }
        let mut artifact_op_bytes = Vec::with_capacity(emit.artifact_mutations.len());
        for op in emit.artifact_mutations.iter() {
            artifact_op_bytes.push(::protocol::OpBinary::encode_op(op).map_err(|error| error.into_fault())?);
        }
        for child in emit.child_emits.iter() {
            if !child.owner.is_empty() || self.children.get(&(child.slot.clone(), child.child_id.clone())).is_none() {
                return Err((Fault::new(
                    FaultOrigin::Framework,
                    FaultCode::new("interactive-job.agent-lane-child-missing"),
                    format!("action '{}' edits owned child {:?} in slot {:?}, which this instance does not hold", address.action_id, child.child_id, child.slot),
                )).into());
            }
        }
        let child_op_bytes = ChildEmit::encode_groups(&emit.child_emits);
        let mut config_op_bytes = Vec::with_capacity(emit.config_mutations.len());
        for op in emit.config_mutations.iter() {
            config_op_bytes.push(::protocol::OpBinary::encode_op(op).map_err(|error| error.into_fault())?);
        }
        let mut draft_op_bytes = Vec::with_capacity(emit.draft_mutations.len());
        for op in emit.draft_mutations.iter() {
            draft_op_bytes.push(::protocol::OpBinary::encode_op(op).map_err(|error| error.into_fault())?);
        }
        let priced = artifact_op_bytes.iter().chain(config_op_bytes.iter()).chain(draft_op_bytes.iter()).map(Vec::len).sum::<usize>();
        if priced > maximum_bytes {
            return Err((Fault::new(
                FaultOrigin::Framework,
                FaultCode::new("interactive-job.preview-output"),
                format!("previewed action '{}' produced {priced} op byte(s); its exact output cap is {}", address.action_id, maximum_bytes),
            )).into());
        }
        self.last_emit_wire = Some(EmitWire { document: protocol::encode_ops_vec(&artifact_op_bytes), config: protocol::encode_ops_vec(&config_op_bytes), draft: protocol::encode_ops_vec(&draft_op_bytes), children: child_op_bytes });
        let mut result = Self::empty_result(&address.action_id, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
        result.output = DslValue::Object(vec![
            ("previewedAction".into(), DslValue::String(address.action_id.clone())),
            ("actionKind".into(), DslValue::String(format!("{kind:?}"))),
            ("documentOps".into(), DslValue::String(artifact_op_bytes.len().to_string())),
            ("configOps".into(), DslValue::String(config_op_bytes.len().to_string())),
            ("draftOps".into(), DslValue::String(draft_op_bytes.len().to_string())),
            ("childGroups".into(), DslValue::String(emit.child_emits.len().to_string())),
            ("opBytes".into(), DslValue::String(priced.to_string())),
            ("jobSteps".into(), DslValue::String(job_steps.to_string())),
        ]);
        Ok(Some(result))
    }

    /// 🪪️ The identity every id one admitted command mints is scoped by ([`AppOperationContext::authoring_seed`]):
    /// content-addressed with [`store::content_id`] over the admitting actor and this document store's hybrid
    /// logical clock ticked for the admission — per author and moment, never a per-instance counter a fresh
    /// session restarts.
    pub(super) fn authoring_seed(&self, actor: &str) -> String {
        let mut clock = self.store.clock_now();
        clock.tick(semio_framework_job::default_now_ms().unwrap_or(clock.physical_ms));
        store::content_id("authoring-seed", format!("{actor}\u{1f}{}:{}:{}", clock.actor, clock.physical_ms, clock.logical).as_bytes())
    }

    /// 👁️ Builds the SAME app-owned job the shell lane builds for this admitted command (`A::build_tool_job` over
    /// [`Self::capture_typed_command_roots`]), dispatches it through the SAME retained-wire factory entry
    /// `start_typed_command_operation` uses, and drives it through its own steps on this turn
    /// ([`drive_agent_lane_preview`]) to the completion it would publish — publishing nothing. Every job shape an
    /// app owns previews this way: the SDK's retained command, a plugin's own tool-command job (puzzle, writer, …)
    /// and a download job, whose file an agent transaction cannot carry and is refused by name. The run holds a
    /// keyed cancellation lease, so cancelling the document cancels the preview. Answers the emit, how many presence,
    /// transient and window-transient mutations the job's completion would publish (lanes an agent
    /// transaction cannot carry, see [`agent_lane_uncarried_lanes`]) and how many of the job's own steps ran.
    fn admit_agent_lane_preview_key(&mut self,meta:&ActionMeta,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.agent_lane_preview.is_none(){
            let grant=cx.retained_grant();let bytes=size_of::<crate::tool_jobs::AgentLanePreview<A>>();if cx.should_yield()||grant.maximum_items<2||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
            let Some(operation_id)=self.admit_typed_operation_slot()else{return Ok(false)};
            let revision=self.store.content_revision_now();let key=ToolOperationKey{app_instance_id:meta.instance_id,document:ArtifactDocumentAuthority(meta.instance_id),operation_id,base_revision:semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().expect("revision lane width"))),generation:semio_framework_job::Generation(self.store.generation_now())};
            let Some(caller)=cx.original_source_parent()?.admit_loan(cx)?else{return Ok(false)};
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
            let config=semio_framework_job_worker_authority::BatchDriveConfig{retained:grant,site:"plugin_agent_lane_preview",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:0,step_budget_us:0};
            self.agent_lane_preview=Some(crate::tool_jobs::AgentLanePreview{key:Some(key),caller:std::mem::ManuallyDrop::new(Some(caller)),starting:true,producer:None,children:None,completion:None,completion_close:None,lease:None,lease_close:None,config,maximum_bytes:0,steps:0,publication:None,complete:false,closing:false,publication_close:None});
        }
        let state=self.agent_lane_preview.as_mut().unwrap();
        if state.closing{return Ok(false)}
        state.caller.as_ref().ok_or_else(||plugin_sdk_fault("original preview caller source is absent"))?.validate_context(cx)?;
        let key=state.key.as_ref().ok_or_else(||plugin_sdk_fault("original preview operation key is absent"))?;
        if key.app_instance_id!=meta.instance_id{return Err(plugin_sdk_fault("original preview operation belongs to another app instance").into())}
        if state.lease.is_none(){state.lease=self.tool_cancellations.begin_keyed(key,cx)?;}
        Ok(state.lease.is_some())
    }

    async fn begin_agent_lane_preview(&mut self, command: Box<A::Command>, admission: AdmittedToolCommand, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),semio_framework_job::OriginalTurnFailure> {
        let verb = admission.verb.clone();
        let roots = self.capture_typed_command_roots(command.as_ref(), meta).await?;
        let state=self.agent_lane_preview.as_ref().ok_or_else(||plugin_sdk_fault("original preview admission source is absent"))?;
        let key=state.key.as_ref().ok_or_else(||plugin_sdk_fault("original preview operation key is absent"))?;
        if !state.starting||state.lease.is_none()||key.base_revision!=roots.base_revision||key.generation!=roots.generation{return Err(plugin_sdk_fault("original preview roots no longer match their admitted operation key").into())}
        let operation_id=key.operation_id;
        let seed_handle = artifact_handle_of(&format!("{}/{}/{verb}/{}", meta.instance_id, self.tool_job_controller_id, roots.base_revision.0)).await;
        let operation = semio_framework_job::Operation::new(operation_id, roots.base_revision, roots.generation, (seed_handle.0 as u64) ^ ((seed_handle.0 >> 64) as u64));
        let completion = ArtifactToolCompletion::<A>::new();
        let presence_local = self.presence_store.local_read().map_err(Fault::from)?;
        let (context, _context_birth) = ArtifactOwnedContextHandle::admit(
            ArtifactOwnedToolJobContext::new(
                meta.instance_id,
                meta.view_state.clone(),
                roots.canonical_base_revision,
                roots.draft_generation,
                roots.transient_generation,
                ArtifactOwnedToolJobSnapshots {
                    children: std::sync::Arc::clone(&roots.children),
                    draft: std::sync::Arc::clone(&roots.draft_snapshot),
                    transient: std::sync::Arc::clone(&roots.transient),
                    window_config: roots.window_config_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
                    window_transient: roots.window_transient_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
                },
            )
            .with_source_issuers(&self.draft_store, &self.transient_store)
            .with_presence(presence_local, std::sync::Arc::clone(&roots.presence_peers), self.presence_store.peer_retirement_factory())
            .with_tool_run(self.tool_runs.view_for(meta.view_state.as_ref().and_then(|view| view.window_id.as_deref())))
            .with_provisional(self.tool_machines.provisional_values(), self.tool_machines.provisional_generation()),
            artifact_owned_tool_birth_grant(),
        );
        let key = admission.proof.key();
        let contract = admission.proof.contract();
        let spec = A::build_tool_job(ArtifactOwnedToolJobRequest {
                retained:self.mounted_policy.maintenance,
            command,
            raw_wire: ArtifactToolRawInput::transferred_to_factory(),
            operation,
            controller_id: key.controller_id,
            tool_id: key.tool_id,
            payload_schema_id: admission.proof.schema_id(),
            contract,
            decoded_items: admission.decoded_items,
            app_instance_id: meta.instance_id,
            parent_document_id: self.store.envelope().id.clone(),
            canonical_base_revision: roots.canonical_base_revision,
            authoring_seed: self.authoring_seed(&meta.actor),
            snapshot: roots.snapshot,
            snapshot_read: self.store.snapshot_read().map_err(FaultFrom::into_fault)?,
            config: roots.config,
            window_config: roots.window_config_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
            history: roots.history,
            interaction_state: roots.interaction_state,
            interaction_hover: std::sync::Arc::new(roots.interaction_hover),
            context,
            instance_operation_owner: self.instance_operation_owner.clone(),
            output_chunks: ArtifactOutputChunks::new(contract.max_output_bytes),
            completion: completion.clone(),
        })
        .await?
        .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-owned-builder"), format!("app-owned tool '{verb}' registered a factory but supplied no exact payload builder")))?;
        let wire_admission = admission.wire_admission.clone();
        let dispatch =
            self.tool_jobs.dispatch_wire_retained_with_spec(&wire_admission, admission.raw_wire, None, spec).map_err(|rejected| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), rejected.error.to_string()))?;
        let config=semio_framework_job_worker_authority::BatchDriveConfig{retained:cx.retained_grant(),site:"plugin_agent_lane_preview",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:contract.max_work_units_per_step,step_budget_us:u64::from(contract.max_step_micros)};
        let state=self.agent_lane_preview.as_mut().expect("original preview lease remains received");state.producer=Some(crate::tool_jobs::OriginalPreviewDrive::new(dispatch.job,config,cx));state.completion=Some(completion);state.config=config;state.maximum_bytes=contract.max_output_bytes;state.starting=false;
        Ok(())
    }

    fn advance_agent_lane_preview(&mut self,verb:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<(Emit<A::Mutation,A::ConfigMutation,A::DraftMutation>,(usize,usize,usize),u64)>,semio_framework_job::OriginalTurnFailure>{
        if cx.should_yield(){return Ok(None)}
        let state=self.agent_lane_preview.as_mut().ok_or_else(||plugin_sdk_fault("original preview source is absent"))?;
        if state.starting||state.closing{return Ok(None)}
        if !state.complete{state.caller.as_ref().ok_or_else(||plugin_sdk_fault("original preview caller source is absent"))?.validate_context(cx)?;}
        if let Some(producer)=state.producer.as_mut(){let Some(steps)=(match drive_agent_lane_preview(producer,verb,cx){Ok(result)=>result,Err(failure)=>{state.closing=true;return Err(failure)}})else{return Ok(None)};state.steps=steps;state.producer.take();return Ok(None)}
        if let Some(children)=state.children.as_mut(){let Some(steps)=(match drive_agent_lane_preview(children,verb,cx){Ok(result)=>result,Err(failure)=>{state.closing=true;return Err(failure)}})else{return Ok(None)};state.steps=state.steps.saturating_add(steps);state.children.take();state.complete=true;return Ok(None)}
        if !state.complete{
            let completion=state.completion.as_ref().ok_or_else(||plugin_sdk_fault("original preview completion source is absent"))?;
            match completion.take()?{
                Some(ArtifactToolCompletionValue::Emit(emit,ephemeral))=>{let emit=emit.map_err(ArtifactCompletionFault::into_fault)?;let job=ChildEmissionPreviewJob::<A>{emit:Some(emit),ephemeral:Some(ephemeral),completion:Some(completion.clone()),retirement:None,retirement_payload:None,retirement_fault:None,rejection:None,completion_retirement:None,maximum_bytes:state.maximum_bytes,wire_mode:true,fault_publication:semio_framework_job_fault_publication::RetainedFaultPublication::new(),closing:false};state.children=Some(crate::tool_jobs::OriginalPreviewDrive::new(job,state.config,cx));return Ok(None)},
                Some(value@ArtifactToolCompletionValue::Download(..))=>{state.publication=Some(value);state.complete=true;},
                None=>return Err(plugin_sdk_fault("preview producer completed without its original output").into()),
            }
        }
        if state.publication.is_none(){state.publication=state.completion.as_ref().ok_or_else(||plugin_sdk_fault("original preview completion source is absent"))?.take()?;return Ok(None)}
        if state.completion.is_some()||state.completion_close.is_some(){let step=semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut state.completion,&mut state.completion_close,cx.retained_grant())?;cx.consume_retained(step.progress())?;return Ok(None)}
        if let Some(owner)=state.lease_close.as_mut(){let step=if owner.terminal_is_empty(){crate::tool_cancellation::original_lease_cursor_release(&mut state.lease_close,cx.retained_grant()).map(RetainedCloneStep::Progress)?}else{owner.close_step(&self.tool_cancellations,cx.retained_grant())?};cx.consume_retained(step.progress())?;return Ok(None)}
        if state.lease.is_some(){let(owner,progress)=crate::tool_cancellation::original_lease_cursor_admit(&mut state.lease,cx.retained_grant())?;state.lease_close=owner;cx.consume_retained(progress)?;return Ok(None)}
        if let Some(caller)=state.caller.as_mut(){caller.close_step(cx.original_source_parent()?,cx)?;if caller.terminal_is_empty(){state.caller.take();}return Ok(None)}
        if cx.retained_grant().maximum_items<2||cx.retained_grant().maximum_depth==0{return Ok(None)}
        cx.consume_retained(RetainedCloneProgress{copied_items:2,..Default::default()})?;
        state.key.take();
        let state=self.agent_lane_preview.take().expect("original preview publication owner remains exact");
        match state.publication{
            Some(ArtifactToolCompletionValue::Emit(emit,ephemeral))=>emit.map(|emit|Some((emit,(ephemeral.presence.len(),ephemeral.transient.len(),ephemeral.window_transient.len()),state.steps))).map_err(|source|ArtifactCompletionFault::into_fault(source).into()),
            Some(ArtifactToolCompletionValue::Download(..))=>Err(agent_lane_uncarried_fault(verb,"a file download").into()),
            None=>Err(plugin_sdk_fault("preview closed without its original publication").into()),
        }
    }

    pub(crate) fn close_agent_lane_preview(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{
        let Some(state)=self.agent_lane_preview.as_mut()else{return Ok(PluginLifecycleStep::Complete(Default::default()))};
        if state.terminal_is_empty(){if grant.maximum_items==0||grant.maximum_depth==0||cx.should_yield(){return Ok(PluginLifecycleStep::Progress(Default::default()))}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;self.agent_lane_preview.take();return Ok(PluginLifecycleStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))}
        state.close_step(cx,self.tool_cancellations.handle())
    }

    /// 🧩️ A typed frame IS an owner-qualified invocation — the exact wire `AppCommand::Command`
    /// carries. Decoding it here is what gives the agent lane the same dispatch the shell's UI
    /// lane has; running it through [`Self::preview_addressed_action`] rather than
    /// `handle_action_invocation` is what makes it a PREPARE (see that method's own doc).
    pub(super) async fn dispatch_command_frame(&mut self, command_bytes: &[u8], meta: &ActionMeta, _identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let invocation: ManifestActionInvocation = crate::plugin_runtime::decode_wire_serialized(command_bytes).await.map_err(|_| {
            Fault::new(
                FaultOrigin::Framework,
                FaultCode::new("interactive-job.missing-exact-key"),
                "a typed command frame carries one owner-qualified ManifestActionInvocation — the same wire AppCommand::Command carries; this frame decoded as neither, so no command-specific pre-serde envelope can be selected",
            )
        })?;
        self.preview_addressed_action(&invocation, meta, cx).await
    }
}
