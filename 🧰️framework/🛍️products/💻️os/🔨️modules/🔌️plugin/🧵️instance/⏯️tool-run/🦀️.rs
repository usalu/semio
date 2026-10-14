//! ⏯️ The instance's tool-run turns: ledger driving, tick folding and finalize publication.
use crate::tool_jobs::tool_run::*;
use crate::tool_jobs::tool_run::{selected_entry, selected_entry_mut};
use semio_framework_tool_run::{
    tool_run_pointer_value, ToolRunAction, ToolRunEffect, ToolRunEvent, ToolRunId, ToolRunIdentity, ToolRunLane, ToolRunMachine, ToolRunRebasePolicy, ToolRunReconfigurePolicy,
    ToolRunRejection, ToolRunSettingsReads, ToolRunState, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTick, ToolRunTraceStore, TOOL_RUN_ARG_GENERATION, TOOL_RUN_ARG_RUN_ID, TOOL_RUN_ARG_TOOL_ID, TOOL_RUN_ARG_WINDOW_ID, TOOL_RUN_REASON_CONFLICT, TOOL_RUN_REASON_REBASING, TOOL_RUN_TICK_BYTES_MAX,
};
use std::sync::Arc;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_close}};
use super::*;

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🏃️ Ledger work plus an unobserved store or settings generation change the watch still owes.
    pub(crate) fn tool_run_has_pending_work(&self) -> bool {
        self.tool_runs.has_pending_work()
            || self.tool_runs.is_ui_dirty()
            || self.tool_runs.entries.iter().any(|entry| {
                matches!(entry.slot.state, ToolRunState::Running | ToolRunState::Paused | ToolRunState::Complete | ToolRunState::Finalizing)
                    && (entry.base_generation != self.store.generation()
                        || entry.settings_generation != self.tool_run_settings_generation()
                        || entry.member.as_ref().is_some_and(|member| self.children.get(&(member.slot.clone(), member.child_id.clone())).is_some_and(|child| child.member.visit_member(ToolRunMemberGeneration) != member.base_generation)))
            })
    }

    /// 🎚️ The committed settings generations the driver watches (config store, window-config publications).
    fn tool_run_settings_generation(&self) -> (u64, u64) {
        (self.config_store.generation(), self.tool_runs.driver.window_config_publications)
    }

    /// ⏯️ Host-driven routing of the §2.5 actions: applied to the ledger now, never queued behind the run.
    ///
    /// 🧯️ A rejection REPAINTS the panel. The panel's buttons carry the run identity captured when
    /// the panel last rendered, and a settings change bumps the live generation — so a stale press
    /// was rejected, published `UiDirtyScope::None`, and therefore left the same stale identity on
    /// the button: every later press was silently rejected too, forever. Measured 2026-09-17 on a
    /// ◻️2d fill that stayed `Paused · Searching an open handle (2/5)` for 30 s after Abort.
    pub(crate) async fn dispatch_tool_run_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
        let outcome = self.apply_tool_run_action(action, args, meta).await?;
        let output = match outcome {
            ToolRunActionOutcome::Applied(effect) => DslValue::Object(vec![("toolRun".into(), DslValue::String(effect.as_str().into()))]),
            ToolRunActionOutcome::Rejected(rejection) => DslValue::Object(vec![("rejected".into(), DslValue::String(rejection.code().into()))]),
        };
        let ui_scope = match outcome {
            ToolRunActionOutcome::Applied(ToolRunEffect::SpawnJob) => UiDirtyScope::Full,
            ToolRunActionOutcome::Applied(_) => self.tool_runs.dirty_scope(),
            ToolRunActionOutcome::Rejected(_) => self.tool_runs.dirty_scope(),
        };
        let mut result = Self::empty_result(action, meta, Vec::new(), Vec::new(), ui_scope).await;
        result.output = output;
        Ok(result)
    }

    /// ⚖️ Applies one §2.5 action to the ledger and performs the transition's immediate effect. An action that
    /// names no identity at all (neither `runId` nor `generation`, as a keyboard chord dispatches it) targets
    /// the instance's live run at its generation of this dispatch; an identity it does name is checked as is.
    pub async fn apply_tool_run_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta) -> Result<ToolRunActionOutcome, Fault> {
        let outcome = self.apply_selected_tool_run_action(action, args, meta).await;
        self.tool_runs.select_primary();
        outcome
    }

    /// ⚖️ [`Self::apply_tool_run_action`] on the run the action addresses: its `runId`, else the primary run.
    ///
    /// 🛑️ An abort is idempotent in the generation: it ends the run whatever that run was last
    /// reconfigured to, exactly as `Dismiss` above already ignores generation entirely. Checking
    /// the button's captured generation made Abort unreachable after any settings change — the
    /// one control whose whole job is to get out of a run it cannot otherwise leave.
    async fn apply_selected_tool_run_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta) -> Result<ToolRunActionOutcome, Fault> {
        let Some(action) = ToolRunAction::from_id(action) else {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.unknown-action"), format!("'{action}' is not a tool run action")));
        };
        if action == ToolRunAction::Start {
            return self.start_tool_run(args, meta).await;
        }
        let names_identity = args.is_some_and(|args| args.get(TOOL_RUN_ARG_RUN_ID).is_some() || args.get(TOOL_RUN_ARG_GENERATION).is_some());
        match tool_run_arg_u64(args, TOOL_RUN_ARG_RUN_ID) {
            Some(run) => {
                self.tool_runs.select_run(run);
            }
            None => self.tool_runs.select_primary(),
        }
        let live = if names_identity { None } else { self.tool_runs.slot() };
        let run = live.map_or_else(|| tool_run_arg_u64(args, TOOL_RUN_ARG_RUN_ID).unwrap_or(0), |slot| slot.run);
        let generation = live.map(|slot| slot.generation).or_else(|| tool_run_arg_u64(args, TOOL_RUN_ARG_GENERATION).and_then(|value| u32::try_from(value).ok()));
        let event = match (action, generation) {
            (ToolRunAction::Dismiss, _) => ToolRunEvent::Dismiss { run },
            (_, None) => return Ok(ToolRunActionOutcome::Rejected(ToolRunRejection::Stale)),
            (ToolRunAction::Pause, Some(generation)) if self.tool_runs.state() == Some(ToolRunState::Paused) => ToolRunEvent::Resume { run, generation },
            (ToolRunAction::Pause, Some(generation)) => ToolRunEvent::Pause { run, generation },
            (ToolRunAction::Resume, Some(generation)) => ToolRunEvent::Resume { run, generation },
            (ToolRunAction::Step, Some(generation)) => ToolRunEvent::Step { run, generation },
            (ToolRunAction::Finalize, Some(generation)) => ToolRunEvent::Finalize { run, generation },
            (ToolRunAction::Abort, Some(generation)) => {
                let generation = self.tool_runs.slot().filter(|slot| slot.run == run).map_or(generation, |slot| slot.generation);
                let publishing = selected_entry!(self.tool_runs).and_then(|entry| entry.finalize.as_ref()).is_some_and(|finalize| {
                    finalize.published
                        || finalize.publication.as_ref().is_some_and(|publication| {
                            !matches!(publication.phase(), store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::Preparing | store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::PreparingCursor | store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::PreflightingCommit)
                        })
                });
                ToolRunEvent::Abort { run, generation, publishing }
            }
            (ToolRunAction::Start, _) => unreachable!("start handled above"),
        };
        if let Some(entry) = selected_entry_mut!(self.tool_runs).filter(|entry| action == ToolRunAction::Finalize && entry.slot.run == run) {
            entry.actor = meta.actor.clone();
        }
        let effect = match self.tool_runs.apply_event(event) {
            Ok(effect) => effect,
            Err(rejection) => return Ok(ToolRunActionOutcome::Rejected(rejection)),
        };
        match (effect, selected_entry_mut!(self.tool_runs)) {
            (ToolRunEffect::DriveOneUnit, Some(entry)) => entry.pending_step = true,
            (ToolRunEffect::StopScheduling, Some(entry)) => entry.pending_step = false,
            (ToolRunEffect::BeginFinalize, Some(entry)) => entry.finalize = Some(ToolRunFinalize { phase: ToolRunFinalizePhase::Pending, publication: None, ingress:Default::default(),rejected:false,retracted: 0, published: false }),
            (ToolRunEffect::CloseJob, _) => self.tool_runs.close_current_job(),
            (ToolRunEffect::CancelBatch, _) => {
                self.tool_runs.close_current_job();
                if let Some(publication) = selected_entry_mut!(self.tool_runs).and_then(|entry| entry.finalize.as_mut()).and_then(|finalize| finalize.publication.as_mut()) {
                    self.store.cancel_apply_batch(publication);
                }
            }
            _ => {}
        }
        Ok(ToolRunActionOutcome::Applied(effect))
    }

    async fn start_tool_run(&mut self, args: Option<&DslValue>, meta: &ActionMeta) -> Result<ToolRunActionOutcome, Fault> {
        let view_state = meta.view_state.as_ref();
        let tool_id = args
            .and_then(|args| args.get(TOOL_RUN_ARG_TOOL_ID))
            .and_then(DslValue::as_str)
            .map(str::to_string)
            .or_else(|| view_state.and_then(|view| view.active_utility_id.clone().filter(|id| self.registry.tool_run(id).is_some()).or_else(|| view.active_tool_id.clone())))
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.tool-id"), "toolRunStart needs a toolId argument or an active tool"))?;
        let definition =
            self.registry.tool_run(&tool_id).map(|(_, definition)| definition.clone()).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.unknown-tool"), format!("tool '{tool_id}' declares no ToolRunDefinition")))?;
        let window_id = args.and_then(|args| args.get(TOOL_RUN_ARG_WINDOW_ID)).and_then(DslValue::as_str).map(str::to_string).or_else(|| view_state.and_then(|view| view.window_id.clone().or_else(|| view.focused_window_id.clone())));
        if definition.mutating && self.time_travel.freezes_local_emits() {
            return Ok(ToolRunActionOutcome::Rejected(ToolRunRejection::Busy));
        }
        let member = definition.member.as_deref().map(|slot| self.tool_run_member_target(slot)).transpose()?;
        let lane = ToolRunLane { mutating: definition.mutating, tool_id: tool_id.clone(), window_id: if definition.mutating { None } else { window_id.clone() } };
        let live: Vec<(ToolRunLane, ToolRunState)> = self.tool_runs.entries.iter().map(|entry| (entry.lane(), entry.slot.state)).collect();
        let replaced = match lane.admit(&live) {
            Ok(replaced) => replaced,
            Err(rejection) => return Ok(ToolRunActionOutcome::Rejected(rejection)),
        };
        let run = self.tool_runs.next_run;
        if replaced.iter().any(|index|self.tool_runs.entries[*index].finalize.as_ref().is_some_and(|finalize|!finalize.ingress.terminal_is_empty()||finalize.publication.is_some())){return Ok(ToolRunActionOutcome::Rejected(ToolRunRejection::Busy));}
        let transition = match ToolRunMachine::apply(None, ToolRunEvent::Start { run }) {
            Ok(transition) => transition,
            Err(rejection) => return Ok(ToolRunActionOutcome::Rejected(rejection)),
        };
        let slot = transition.slot.expect("start yields a slot");
        self.tool_runs.next_run = run.wrapping_add(1);
        for index in replaced.into_iter().rev() {
            let previous = self.tool_runs.entries.remove(index);
            self.tool_runs.retire_entry(*previous);
        }
        self.refresh_cache().await?;
        let (base, _, _) = self.command_cache_inputs();
        let identity = ToolRunIdentity::new(ToolRunId { app_instance_id: self.live_runtime_instance_id.unwrap_or(meta.instance_id), run }, self.store.content_revision());
        let settings_generation = self.tool_run_settings_generation();
        let window = window_id.and_then(|window_id| view_state?.window_instances.iter().find(|window| window.id == window_id).map(|window| (window.id.clone(), window.window_kind_id.clone())));
        let settings_values = self.tool_run_settings_values(&definition.settings, window.as_ref());
        let window_bodies = definition.windows.iter().filter_map(|window_kind_id| self.registry.window_body_key(window_kind_id).map(str::to_string)).collect();
        self.tool_runs.entries.push(Box::new(ToolRunEntry {
            tool_id,
            actor: meta.actor.clone(),
            settings_values,
            window,
            window_bodies,
            payload: None,
            trace_keys: ToolRunTraceKeys::default(),
            definition,
            slot,
            identity,
            overlay: Arc::clone(&base),
            base,
            base_generation: self.store.generation(),
            settings_generation,
            provisional: Vec::new(),
            entity_marks: Vec::new(),
            entities: Arc::new(BTreeSet::new()),
            refold: None,
            trace: ToolRunTraceStore::new(identity),
            recent_trace: std::collections::VecDeque::with_capacity(TOOL_RUN_PANEL_TRACE_ROWS),
            stage: 0,
            completed: 0,
            total: None,
            counters: Vec::new(),
            steps: ToolRunStepRing::new(),
            step_sequence: 0,
            sequence: 0,
            conflicts: 0,
            rate_origin: None,
            units_per_second: 0.0,
            job: None,
            checkpoint: None,
            pending_step: false,
            finalize: None,
            announced: None,
            port: ToolRunJobPort::default(),
            displaced: Vec::new(),
            member,
        }));
        Ok(ToolRunActionOutcome::Applied(transition.effect))
    }

    /// 🧵️ Builds the run's job for `purpose`: the app's retargetable job when it builds one, else its plain job.
    fn build_tool_run_job_slot(&self, entry: &ToolRunEntry<A>, purpose: ToolRunJobPurpose) -> Result<Option<ToolRunJobSlot<A::Config>>, Fault> {
        let request = || ToolRunJobRequest::<A> {
            receiving: &entry.definition.receiving,
            tool_id: &entry.tool_id,
            definition: &entry.definition,
            purpose,
            identity: entry.identity,
            snapshot: match purpose {
                ToolRunJobPurpose::Run => Arc::clone(&entry.base),
                ToolRunJobPurpose::Revalidate => self.store.snapshot_owner(),
            },
            config: self.config_store.snapshot_owner(),
            window_id: entry.window.as_ref().map(|(window_id, _)| window_id.as_str()),
            window_config: entry.window.as_ref().and_then(|(window_id, window_kind_id)| self.window_config_store.snapshot(window_kind_id, window_id)),
            checkpoint: entry.checkpoint.as_deref(),
            provisional: &entry.provisional,
            instance_owner: self.instance_operation_owner.clone(),
            port: entry.port.clone(),
            trace_keys: entry.trace_keys.clone(),
            entity_marks: &entry.entity_marks,
            children: ChildContentView::clone(&self.child_content_root),
            member_ops: entry.member.as_ref().map_or(&[], |member| member.ops.as_slice()),
        };
        let generation = entry.identity.generation;
        if let Some(job) = A::build_retargetable_tool_run_job(request())? {
            return Ok(Some(ToolRunJobSlot::new(ToolRunJobHandle::Retargetable(job), purpose, generation, entry.definition.receiving)));
        }
        Ok(A::build_tool_run_job(request())?.map(|job| ToolRunJobSlot::new(ToolRunJobHandle::Plain(job), purpose, generation, entry.definition.receiving)))
    }

    /// 🎚️ What the run's declared settings reads name right now. A declared window kind that is the starting
    /// window's reads only that window's config, the one its jobs receive; any other declared kind reads every
    /// partition of that kind.
    fn tool_run_settings_values(&self, reads: &ToolRunSettingsReads, window: Option<&(String, String)>) -> ToolRunSettingsValues {
        let config = if reads.config.is_empty() {
            Vec::new()
        } else {
            let document = semio_framework_value::ToValue::to_value(self.config_store.snapshot_owner().as_ref());
            reads.config.iter().map(|pointer| tool_run_pointer_value(&document, pointer).cloned()).collect()
        };
        let window_config = reads
            .window_config
            .iter()
            .flat_map(|(window_kind_id, pointers)| {
                let started = window.filter(|(_, kind)| kind == window_kind_id).map(|(window_id, _)| window_id.as_str());
                self.window_config_store.pointer_values(window_kind_id, pointers).into_iter().filter(move |(window_id, _)| started.is_none_or(|started| started == window_id)).map(move |(window_id, values)| (window_kind_id.clone(), window_id, values))
            })
            .collect();
        ToolRunSettingsValues { config, window_config }
    }

    fn apply_tool_run_driver_event(&mut self, event: ToolRunEvent) -> Option<ToolRunEffect> {
        self.tool_runs.apply_event(event).ok()
    }

    /// 📮️ Hands the host every effect jobs queued on the port, in order, while the typed effect outbox has room.
    fn drain_tool_run_port(&mut self) {
        for index in 0..self.tool_runs.entries.len() {
            while let Some(effect) = self.tool_runs.entries[index].port.pop_effect() {
                if let Err(effect) = self.typed_effect_outbox.push(effect) {
                    self.tool_runs.entries[index].port.restore_effect(effect);
                    return;
                }
            }
        }
    }

    /// 🎯️ A tick-sized change: progress, steps, trace and provisional appends reach the panel and the scene
    /// windows only (`ToolRunLedger::dirty_scope`).
    fn mark_tool_run_ui_dirty(&mut self) {
        self.tool_runs.ui_dirty = true;
        self.flush_tool_run_ui_dirty();
    }

    /// 📄️ The document every window reads was swapped wholesale (overlay refolded, released or discarded), so
    /// every body is dirty once.
    fn mark_tool_run_document_dirty(&mut self) {
        self.tool_runs.document_dirty = true;
        self.flush_tool_run_ui_dirty();
    }

    /// 📮️ Hands the owed scope to the typed UI outbox once it has room; until then the flags stay set, so a
    /// queued unrelated scope never swallows a run change.
    pub(crate) fn flush_tool_run_ui_dirty(&mut self) {
        if !self.tool_runs.is_ui_dirty() || self.typed_ui_outbox.len() != 0 {
            return;
        }
        let scope = if self.tool_runs.document_dirty { UiDirtyScope::Full } else { self.tool_runs.dirty_scope() };
        if self.typed_ui_outbox.push(scope).is_ok() {
            self.tool_runs.ui_dirty = false;
            self.tool_runs.document_dirty = false;
        }
    }

    /// 🎬️ Scene lane injection (§3.2, §4.1 layer 2): the first World3d, Canvas2d or Board2d scene surface a window body
    /// renders carries the `toolRunTrace` lane answered for the cursor that window's renderer echoed.
    /// Panel bodies, override renders and bodies without a scene surface carry nothing.
    pub(crate) fn inject_tool_run_trace_lane(&mut self, tree: &mut ComponentTree, body_key: &str, view_state: &ViewModel) -> Result<(), Fault> {
        let Some(window_id) = view_state.window_id.as_deref() else { return Ok(()) };
        self.tool_runs.retain_trace_windows(|live| view_state.window_instances.iter().any(|window| window.id == live));
        let Some(surface) = tool_run_scene_surface(&mut tree.root) else {
            self.tool_runs.trace_windows.remove(window_id);
            return Ok(());
        };
        let cursor = view_state.tool_run_trace_cursor_by_window_id.get(window_id).copied();
        let Some(lane) = self.tool_runs.trace_lane(window_id, body_key, cursor, TOOL_RUN_TRACE_DELTA_BYTES) else { return Ok(()) };
        inject_tool_run_trace_lane_into(surface, &lane).map_err(|error| Fault::new(FaultOrigin::Plugin, FaultCode::new("toolRun.trace-lane"), error.to_string()))
    }

    /// 👀️ Store and settings watch (§3.3): emits `baseChanged` / `settingsChanged` with their policies.
    ///
    /// - A base change rebinds a retargetable job to the new identity in place; any other job is closed and
    ///   rebuilt under the new identity (`restart` also discards the provisional ops), so no tick of the old
    ///   identity is ever produced.
    /// - A config or window-config publication is only a settings change when a value behind one of the run's
    ///   declared `settings` pointers changed; `resume` then retargets a retargetable job in place and rebuilds
    ///   any other from its checkpoint.
    fn watch_tool_run_generations(&mut self) {
        let store_generation = self.store.generation();
        let settings_generation = self.tool_run_settings_generation();
        let member_generation = self.tool_run_member_generation();
        let Some(entry) = selected_entry!(self.tool_runs) else { return };
        let (run, state, rebase, reconfigure) = (entry.slot.run, entry.slot.state, entry.definition.rebase, entry.definition.reconfigure);
        let parent_moved = entry.base_generation != store_generation;
        let base_changed = match (entry.member.as_ref(), member_generation) {
            (Some(member), Some(generation)) => member.base_generation != generation,
            (Some(_), None) => false,
            (None, _) => parent_moved,
        };
        let settings_moved = entry.settings_generation != settings_generation;
        if !matches!(state, ToolRunState::Running | ToolRunState::Paused | ToolRunState::Complete) {
            return;
        }
        if parent_moved && entry.member.is_some() {
            let head = self.store.snapshot_owner();
            let entry = selected_entry_mut!(self.tool_runs).expect("the watch keeps the slot");
            entry.replace_base(head);
            let overlay = Arc::clone(&entry.base);
            entry.replace_overlay(overlay);
            entry.base_generation = store_generation;
        }
        if let (true, Some(generation)) = (base_changed, member_generation.filter(|_| selected_entry!(self.tool_runs).is_some_and(|entry| entry.member.is_some()))) {
            if self.apply_tool_run_driver_event(ToolRunEvent::BaseChanged { run }) == Some(ToolRunEffect::Refold) {
                let restart = rebase == ToolRunRebasePolicy::Restart;
                if restart {
                    self.tool_runs.close_current_job();
                    self.tool_runs.discard_provisional();
                }
                let entry = selected_entry_mut!(self.tool_runs).expect("refold keeps the slot");
                if let Some(member) = entry.member.as_mut() {
                    member.base_generation = generation;
                    member.rebase = true;
                }
                entry.framework_step(ToolRunStepKind::Warning, TOOL_RUN_REASON_REBASING, &[]);
                if !restart {
                    self.tool_runs.retarget_current_job(|job, identity| {
                        job.rebind(identity);
                        true
                    });
                }
                self.mark_tool_run_document_dirty();
            }
        } else if base_changed && self.apply_tool_run_driver_event(ToolRunEvent::BaseChanged { run }) == Some(ToolRunEffect::Refold) {
            let head = self.store.snapshot_owner();
            let restart = rebase == ToolRunRebasePolicy::Restart;
            if restart {
                self.tool_runs.close_current_job();
                self.tool_runs.discard_provisional();
            }
            let entry = selected_entry_mut!(self.tool_runs).expect("refold keeps the slot");
            entry.replace_base(head);
            entry.base_generation = store_generation;
            entry.identity.base_revision = self.store.content_revision();
            entry.trace.rebind(entry.identity);
            if restart {
                let overlay = Arc::clone(&entry.base);
                entry.replace_overlay(overlay);
            } else {
                entry.begin_refold(true);
            }
            entry.framework_step(ToolRunStepKind::Warning, TOOL_RUN_REASON_REBASING, &[]);
            if !restart {
                self.tool_runs.retarget_current_job(|job, identity| {
                    job.rebind(identity);
                    true
                });
            }
            self.mark_tool_run_document_dirty();
        }
        if !settings_moved {
            return;
        }
        let entry = selected_entry!(self.tool_runs).expect("the watch keeps the slot");
        let values = self.tool_run_settings_values(&entry.definition.settings, entry.window.as_ref());
        let entry = selected_entry_mut!(self.tool_runs).expect("the watch keeps the slot");
        entry.settings_generation = settings_generation;
        if values == entry.settings_values {
            return;
        }
        entry.settings_values = values;
        if self.apply_tool_run_driver_event(ToolRunEvent::SettingsChanged { run }) != Some(ToolRunEffect::Reconfigure) {
            return;
        }
        match reconfigure {
            ToolRunReconfigurePolicy::Restart => {
                self.tool_runs.close_current_job();
                self.tool_runs.discard_provisional();
                self.mark_tool_run_document_dirty();
            }
            ToolRunReconfigurePolicy::Resume => {
                let config = self.config_store.snapshot_owner();
                self.tool_runs.retarget_current_job(|job, identity| job.reconfigure(identity, config));
                self.mark_tool_run_ui_dirty();
            }
        }
    }

    /// ⏯️ One bounded driver turn (≤ `ToolRunDriver::turn_wall_us`): owed dirty scope, the generation watch of every
    /// run, retirement within the wall budget, then — while nothing retires — refold, job steps, finalize. The watch never
    /// waits behind retirement: a settings or base change reaches its run in the turn it becomes visible.
    pub(crate) async fn drive_tool_run_turn(&mut self,cx:&mut semio_framework_job::StepContext<'_>,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
        let grant=cx.retained_grant();if cx.should_yield(){return Ok(())}
        if self.tool_runs.has_original_receipt(){return Ok(());}
        self.flush_tool_run_ui_dirty();
        self.drain_tool_run_port();
        let deadline=cx.deadline_us();
        if self.tool_runs.entries.is_empty(){self.retire_tool_runs_until(cx)?;return Ok(());}
        let index=self.tool_runs.driver_cursor%self.tool_runs.entries.len();
        let run=self.tool_runs.entries[index].slot.run;
        self.tool_runs.driver_cursor=index.saturating_add(1);
        self.tool_runs.selected=Some(index);
        self.watch_tool_run_generations();
        if self.retire_tool_runs_until(cx)?{self.tool_runs.select_primary();return Ok(());}
        let outcome=if self.tool_runs.select_run(run){self.drive_selected_tool_run(deadline,cx.retained_grant(),identity).await.map_err(semio_framework_job::OriginalTurnFailure::from)}else{Ok(())};
        self.tool_runs.select_primary();
        outcome
    }

    /// 🚰️ Retirement units until nothing retires, a retirement blocks or the turn deadline. `true` while owners still
    /// retire or a unit released nothing: the runs wait for the next turn instead of displacing more.
    fn retire_tool_runs_until(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let grant=cx.retained_grant();let Some(step)=self.tool_run_retire_step(cx)?else{return Ok(false)};self.tool_runs.retain_original_receipt(grant,step.progress())?;Ok(true)
    }

    /// ⏯️ The selected run's share of one driver turn.
    async fn drive_selected_tool_run(&mut self, deadline: u64,grant:RetainedCloneGrant,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let Some(state) = self.tool_runs.state() else { return Ok(()) };
        match state {
            ToolRunState::Aborting => {
                if self.tool_runs.retired_jobs.is_empty() && self.tool_runs.retired_publications.is_empty() {
                    let (run, generation) = self.tool_runs.slot().map(|slot| (slot.run, slot.generation)).expect("aborting slot");
                    if selected_entry!(self.tool_runs).is_some_and(|entry|entry.finalize.is_some()) {
                        if self.close_tool_run_finalize_owner(grant)?{let copy=size_of::<Option<ToolRunFinalize<A>>>();if tool_grant_funds(grant,RetirementDemand{copy_bytes:copy,depth:1,..Default::default()}){drop(selected_entry_mut!(self.tool_runs).unwrap().finalize.take());self.tool_runs.retain_original_receipt(grant,RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;}}
                        return Ok(());
                    }
                    if self.apply_tool_run_driver_event(ToolRunEvent::AbortComplete { run, generation }) == Some(ToolRunEffect::RetireProvisional) {
                        self.tool_runs.discard_provisional();
                        self.mark_tool_run_document_dirty();
                    }
                }
                Ok(())
            }
            ToolRunState::Starting => self.admit_tool_run_job().await,
            ToolRunState::Running | ToolRunState::Paused | ToolRunState::Complete => {
                if self.refold_tool_run(deadline) || self.refresh_tool_run_member(deadline)? {
                    return Ok(());
                }
                let entry = selected_entry!(self.tool_runs).expect("live slot");
                let wants_job = match entry.slot.state {
                    ToolRunState::Running => true,
                    ToolRunState::Paused => entry.pending_step,
                    _ => false,
                };
                if wants_job && entry.job.is_none() {
                    return self.admit_tool_run_job().await;
                }
                if wants_job {
                    self.step_tool_run_job(deadline,grant)?;
                }
                Ok(())
            }
            ToolRunState::Finalizing => {
                if self.refresh_tool_run_member(deadline)? {
                    return Ok(());
                }
                self.finalize_tool_run_turn(deadline,grant,identity).await
            }
            ToolRunState::Finalized | ToolRunState::Aborted | ToolRunState::Faulted => self.settle_tool_run_member(deadline),
        }
    }

    /// 🔁️ `true` while a refold still owns this turn; a refold waiting only for its job's boundary lets the job step.
    fn refold_tool_run(&mut self, deadline: u64) -> bool {
        let Some(entry) = selected_entry_mut!(self.tool_runs).filter(|entry| entry.refold.is_some()) else { return false };
        if entry.job.is_none() {
            entry.settle_refold();
        }
        match entry.refold_turn(deadline) {
            ToolRunRefoldTurn::Pending => true,
            ToolRunRefoldTurn::AwaitingBoundary => false,
            ToolRunRefoldTurn::Swapped => {
                self.mark_tool_run_document_dirty();
                false
            }
        }
    }

    async fn admit_tool_run_job(&mut self) -> Result<(), Fault> {
        let Some(entry) = selected_entry!(self.tool_runs) else { return Ok(()) };
        let (run, generation, state) = (entry.slot.run, entry.slot.generation, entry.slot.state);
        entry.port.wake();
        match self.build_tool_run_job_slot(entry, ToolRunJobPurpose::Run) {
            Ok(Some(job)) => {
                selected_entry_mut!(self.tool_runs).expect("live slot").job = Some(job);
                if state == ToolRunState::Starting {
                    self.apply_tool_run_driver_event(ToolRunEvent::JobAdmitted { run, generation });
                }
            }
            Ok(None) | Err(_) => self.fault_tool_run(run, generation),
        }
        self.mark_tool_run_ui_dirty();
        Ok(())
    }

    fn fault_tool_run(&mut self, run: u64, generation: u32) {
        if self.apply_tool_run_driver_event(ToolRunEvent::JobFault { run, generation }) == Some(ToolRunEffect::DiscardProvisional) {
            self.tool_runs.close_current_job();
            self.tool_runs.discard_provisional();
            if let Some(entry) = selected_entry_mut!(self.tool_runs) {
                entry.framework_step(ToolRunStepKind::Danger, TOOL_RUN_REASON_CONFLICT, &[ToolRunStepArg::Unsigned(0)]);
            }
            self.mark_tool_run_document_dirty();
        }
    }

    /// 🦶️ Drives the current job until the turn deadline, a pause, a single step or a terminal outcome.
    fn step_tool_run_job(&mut self,deadline:u64,grant:RetainedCloneGrant)->Result<(),Fault>{
        if semio_framework_job::default_now_us().is_none_or(|now|now>=deadline){return Ok(())}
        if self.tool_runs.has_original_receipt(){return Ok(())}
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(())}
        let Some(entry)=selected_entry_mut!(self.tool_runs)else{return Ok(())};
        let (run,generation)=(entry.slot.run,entry.slot.generation);let single=entry.slot.state==ToolRunState::Paused;
        let Some(job)=entry.job.as_mut()else{return Ok(())};
        if job.cancel.is_none(){if let Some((cancel,progress))=semio_framework_job::CancelToken::admit_root(grant).map_err(FaultFrom::into_fault)?{job.cancel=Some(cancel);self.tool_runs.retain_original_receipt(grant,progress)?}return Ok(())}
        if job.authority.is_none(){if let Some((authority,progress))=semio_framework_job::JobPayloadAuthority::admit(job.operation,job.generation,grant).map_err(FaultFrom::into_fault)?{job.authority=Some(authority);self.tool_runs.retain_original_receipt(grant,progress)?}return Ok(())}
        if job.original_failure.is_some(){return Ok(())}
        if let Some(descriptor)=job.outcome.as_mut(){
            if job.outcome_received{if !descriptor.is_acknowledged(){let step=descriptor.acknowledge(grant);self.tool_runs.retain_original_receipt(grant,step.progress())?;return Ok(())}drop(job.outcome.take());job.outcome_received=false;self.tool_runs.retain_original_receipt(grant,RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(())}
            let kind=descriptor.kind();
            let view=job.borrow_outcome().map_err(FaultFrom::into_fault)?;
            match view{Some(semio_framework_job::JobOutcomeView::PreviewReady{..}|semio_framework_job::JobOutcomeView::CheckpointReady{..}|semio_framework_job::JobOutcomeView::Fault{..})=>return Ok(()),Some(semio_framework_job::JobOutcomeView::Complete{state,output,..})if state.is_some()||output.is_some()=>return Ok(()),Some(_)=>{},None=>return Ok(())}
            job.outcome_received=true;let purpose=job.purpose;
            self.tool_runs.retain_original_receipt(grant,RetainedCloneProgress{copied_items:1,..Default::default()})?;
            match kind{semio_framework_job::JobOutcomeKind::Complete=>{selected_entry_mut!(self.tool_runs).unwrap().settle_refold();if purpose==ToolRunJobPurpose::Revalidate{self.complete_tool_run_revalidation()}else{self.complete_tool_run_job(run,generation)}},semio_framework_job::JobOutcomeKind::Cancelled|semio_framework_job::JobOutcomeKind::Fault=>self.fault_tool_run(run,generation),_=>{}}
            return Ok(())
        }
        let fuel=if single{1}else{semio_framework_job::INTERACTIVE_LANE_FUEL};let budget=semio_framework_job::StepBudget::new(fuel,deadline,grant);
        let mut ownership=Default::default();let mut verdict=None;
        let interactive:&mut dyn semio_framework_job::InteractiveJob=match &mut job.job{ToolRunJobHandle::Plain(job)=>job.as_mut(),ToolRunJobHandle::Retargetable(job)=>job.as_mut()};
        let mut context=semio_framework_job::StepContext::with_payload_authority(job.operation,job.generation,budget,job.cancel.as_ref().unwrap(),semio_framework_job::default_now_us,&mut job.preview_sequence,&mut ownership,job.authority.as_ref().unwrap()).map_err(FaultFrom::into_fault)?;
        let stepped=semio_framework_job::drive_step(interactive,&mut context,TOOL_RUN_JOB_SITE,semio_framework_job::InteractiveStage::InteractiveStep,&mut verdict).map(|loan|loan.map(semio_framework_job::JobOutcomeBorrow::into_descriptor));
        drop(context);
        let failed=match stepped{Ok(descriptor)=>{if single&&descriptor.as_ref().is_some_and(|descriptor|!matches!(descriptor.kind(),semio_framework_job::JobOutcomeKind::Yield|semio_framework_job::JobOutcomeKind::CheckpointReady{..})){entry.pending_step=false}job.outcome=descriptor;false},Err(original)=>{*job.original_failure=Some(original);true}};
        self.tool_runs.retain_original_receipt(grant,ownership)?;
        if failed{self.fault_tool_run(run,generation)}
        Ok(())
    }

    /// 🏁️ A finished job's completion — and, for a READ-ONLY run, its finalization too.
    ///
    /// 🪪️ `Complete` means "the job is done and the run awaits the finalize that publishes its
    /// provisional edits". A run that declares `mutating: false` authors no provisional edit at all, so
    /// there is nothing for a finalize to publish and nothing for a user to review: leaving it parked in
    /// `Complete` only holds the tool's single run slot open, and a `toolRunStart` against a non-terminal
    /// run is `ToolRunRejection::Busy`. That is how generation3d's read-only `previewEval` ran exactly
    /// ONCE per session — every later start, whether a poll's or a gesture's own, was refused by the
    /// finished run still sitting in the slot, and the 3d preview stopped re-evaluating
    /// (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`, `📋️tool-run-contract.md` §2.2). Asking a
    /// read-only run's owner to dispatch a finalize it has nothing to finalize is a round trip the host
    /// can always lose: it happens on the host's own refresh cadence, and a run action that publishes
    /// nothing moves no UI that would provoke the next pass.
    fn complete_tool_run_job(&mut self, run: u64, generation: u32) {
        let resident = selected_entry_mut!(self.tool_runs).is_some_and(|entry| entry.definition.mutating && entry.job.as_mut().is_some_and(|job| job.retargetable().is_some()));
        if resident {
            selected_entry_mut!(self.tool_runs).expect("a resident job has a slot").settle_refold();
        } else {
            self.tool_runs.close_current_job();
        }
        self.apply_tool_run_driver_event(ToolRunEvent::JobComplete { run, generation });
        if let Some(entry) = selected_entry_mut!(self.tool_runs) {
            entry.pending_step = false;
        }
        if selected_entry!(self.tool_runs).is_some_and(|entry| !entry.definition.mutating && entry.slot.state == ToolRunState::Complete) {
            let (run, generation) = self.tool_runs.slot().map(|slot| (slot.run, slot.generation)).expect("a complete run has a slot");
            if let Some(ToolRunEffect::BeginFinalize) = self.apply_tool_run_driver_event(ToolRunEvent::Finalize { run, generation }) {
                if let Some(entry) = selected_entry_mut!(self.tool_runs) {
                    entry.finalize = Some(ToolRunFinalize { phase: ToolRunFinalizePhase::Pending, publication: None, ingress:Default::default(),rejected:false,retracted: 0, published: false });
                }
            }
        }
        self.mark_tool_run_document_dirty();
    }

    fn complete_tool_run_revalidation(&mut self) {
        self.tool_runs.close_current_job();
        let Some(entry) = selected_entry_mut!(self.tool_runs) else { return };
        let (run, generation) = (entry.slot.run, entry.slot.generation);
        let retracted = entry.finalize.as_ref().map_or(0, |finalize| finalize.retracted);
        if retracted == 0 {
            if let Some(finalize) = entry.finalize.as_mut() {
                finalize.phase = ToolRunFinalizePhase::Publishing;
            }
            return;
        }
        entry.conflicts = entry.conflicts.saturating_add(retracted);
        entry.framework_step(ToolRunStepKind::Danger, TOOL_RUN_REASON_CONFLICT, &[ToolRunStepArg::Unsigned(u64::from(retracted))]);
        let finalize = entry.finalize.take();
        self.tool_runs.retire_owners(None, Vec::new(), finalize);
        self.apply_tool_run_driver_event(ToolRunEvent::RevalidationConflicts { run, generation });
        self.mark_tool_run_document_dirty();
    }

    /// 🏁️ Finalize (§2.7.4): freshness refold on the head, optional revalidate job, one outbound batched `Edit`.
    async fn finalize_tool_run_turn(&mut self, deadline: u64,grant:RetainedCloneGrant,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let store_generation = self.store.generation();
        let member_generation = self.tool_run_member_generation();
        let entry = selected_entry_mut!(self.tool_runs).expect("finalizing slot");
        let phase = entry.finalize.as_ref().map_or(ToolRunFinalizePhase::Pending, |finalize| finalize.phase);
        let moved = match (entry.member.as_ref(), member_generation) {
            (Some(member), Some(generation)) => member.base_generation != generation,
            (Some(_), None) => false,
            (None, _) => entry.base_generation != store_generation,
        };
        let republish = matches!(phase, ToolRunFinalizePhase::Pending | ToolRunFinalizePhase::Revalidating) || (phase == ToolRunFinalizePhase::Publishing && entry.member.is_some());
        if moved && republish {
            entry.replace_base(self.store.snapshot_owner());
            entry.base_generation = store_generation;
            entry.begin_refold(true);
            if let (Some(member), Some(generation)) = (entry.member.as_mut(), member_generation) {
                member.base_generation = generation;
                member.rebase = true;
                member.retired_emits.extend(member.emit.take());
                member.retired_emission_owners.extend(member.emission_owner.take());
            }
            entry.framework_step(ToolRunStepKind::Warning, TOOL_RUN_REASON_REBASING, &[]);
            if let Some(finalize) = entry.finalize.as_mut() {
                finalize.phase = ToolRunFinalizePhase::Pending;
                finalize.retracted = 0;
            }
            self.tool_runs.close_current_job();
            return Ok(());
        }
        if self.refold_tool_run(deadline) {
            return Ok(());
        }
        match phase {
            ToolRunFinalizePhase::Pending => {
                if selected_entry!(self.tool_runs).is_some_and(|entry| entry.job.as_ref().is_some_and(|job| job.purpose == ToolRunJobPurpose::Run)) {
                    self.tool_runs.close_current_job();
                    return Ok(());
                }
                let entry = selected_entry!(self.tool_runs).expect("finalizing slot");
                let (run, generation) = (entry.slot.run, entry.slot.generation);
                let next = if entry.definition.revalidate_job.is_some() && !(entry.provisional.is_empty() && entry.member.as_ref().is_none_or(|member| member.ops.is_empty())) {
                    match self.build_tool_run_job_slot(entry, ToolRunJobPurpose::Revalidate) {
                        Ok(Some(job)) => {
                            selected_entry_mut!(self.tool_runs).expect("finalizing slot").job = Some(job);
                            ToolRunFinalizePhase::Revalidating
                        }
                        Ok(None) | Err(_) => return self.reject_tool_run_publication(run, generation),
                    }
                } else {
                    ToolRunFinalizePhase::Publishing
                };
                if let Some(finalize) = selected_entry_mut!(self.tool_runs).and_then(|entry| entry.finalize.as_mut()) {
                    finalize.phase = next;
                }
                Ok(())
            }
            ToolRunFinalizePhase::Revalidating => self.step_tool_run_job(deadline,grant),
            ToolRunFinalizePhase::Publishing => self.publish_tool_run(deadline,grant,identity).await,
            ToolRunFinalizePhase::Closing => self.close_tool_run_publication(grant),
        }
    }

    fn reject_tool_run_publication(&mut self, run: u64, generation: u32) -> Result<(), Fault> {
        let Some(entry) = selected_entry_mut!(self.tool_runs) else { return Ok(()) };
        entry.framework_step(ToolRunStepKind::Danger, TOOL_RUN_REASON_CONFLICT, &[ToolRunStepArg::Unsigned(entry.provisional.len() as u64)]);
        if let Some(finalize)=entry.finalize.as_mut(){finalize.phase=ToolRunFinalizePhase::Closing;finalize.rejected=true;}
        self.tool_runs.close_current_job();
        let _=(run,generation);
        self.mark_tool_run_ui_dirty();
        Ok(())
    }

    /// 🏷️ The history row label of a transaction a finalized run published (`<appId>#<toolId>` of a declared tool
    /// run): the tool's own label in every locale, so the row reads the same after a reload.
    pub(crate) fn tool_run_transaction_label(&self, app_id: &str, transaction: &protocol::TransactionRef) -> Option<LocalizedLabel> {
        let tool_id = transaction.tool.strip_prefix(app_id)?.strip_prefix('#')?;
        self.registry.tool_run(tool_id).map(|(label, _)| label.clone())
    }

    /// ♻️ Each folded op leaves the root it was prepared against returned to the Store; reclaim it before the
    /// next fold, or a large finalize keeps one whole document per op (see `reclaim_document_snapshot_read_returns`).
    async fn publish_tool_run(&mut self, deadline: u64,grant:RetainedCloneGrant,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        if selected_entry!(self.tool_runs).is_some_and(|entry| entry.member.is_some()) {
            return self.publish_tool_run_member(deadline,grant).await;
        }
        let entry = selected_entry_mut!(self.tool_runs).expect("finalizing slot");
        let (run, generation) = (entry.slot.run, entry.slot.generation);
        if entry.provisional.is_empty()&&entry.finalize.as_ref().is_none_or(|finalize|finalize.publication.is_none()) {
            entry.finalize.as_mut().expect("finalize owner").phase = ToolRunFinalizePhase::Closing;
            return Ok(());
        }
        if entry.finalize.as_ref().is_some_and(|finalize| finalize.publication.is_none()) {
            if entry.actor.as_str() != self.store.local_actor_id().0.as_str() {
                return self.reject_tool_run_publication(run, generation);
            }
            let finalize=entry.finalize.as_mut().expect("original finalize owner");
            if finalize.ingress.actor.is_none(){
                let progress=finalize.ingress.admit_actor(&entry.actor,grant).map_err(FaultFrom::into_fault)?;
                if let Some(progress)=progress{self.tool_runs.retain_original_receipt(grant,progress)?;}return Ok(());
            }
            if finalize.ingress.transaction.is_none(){
                let clock=semio_framework_tool_machine::authoring_clock(run);
                let progress=finalize.ingress.admit_transaction(&entry.actor,&clock,self.app.instance_id().await,&entry.tool_id,grant).map_err(FaultFrom::into_fault)?;
                if let Some(progress)=progress{self.tool_runs.retain_original_receipt(grant,progress)?;}return Ok(());
            }
            match self.store.admit_apply_batch(semio_framework_job::allocate_operation_id(),entry.base_generation,self.store.content_revision(),&mut finalize.ingress.actor,&mut entry.provisional,HistoryLane::Document,self.artifact_one_item_factory.as_ref(),&mut finalize.ingress.transaction,None,true,grant) {
                Ok(Some((publication,progress)))=>{finalize.publication=Some(publication);self.tool_runs.retain_original_receipt(grant,progress)?;return Ok(());},
                Ok(None)=>return Ok(()),
                Err(_) => return self.reject_tool_run_publication(run, generation),
            }
        }
        {
            if !self.reclaim_document_snapshot_read_returns()? {
                if semio_framework_job::default_now_us().is_none_or(|now| now >= deadline) {
                    return Ok(());
                }
                return Ok(());
            }
            let entry = selected_entry_mut!(self.tool_runs).expect("finalizing slot");
            let publication = entry.finalize.as_mut().and_then(|finalize| finalize.publication.as_mut()).expect("admitted publication");
            match self.store.advance_apply_batch(publication, store::os_store::one_item::ArtifactStoreOneItemGrant{maximum_items:grant.maximum_items.min(1),maximum_copy_bytes:grant.maximum_copy_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes,maximum_release_bytes:grant.maximum_release_bytes,maximum_depth:grant.maximum_depth}) {
                Ok(store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Published(_)) => {
                    let group_id = entry.identity.id.group_id();
                    let tool_id = entry.tool_id.clone();
                    let actor = entry.actor.clone();
                    self.store.stamp_tail_group_id(&group_id).await.map_err(|error| error.into_fault())?;
                    let publication = selected_entry_mut!(self.tool_runs).and_then(|entry| entry.finalize.as_mut()).and_then(|finalize| finalize.publication.as_mut()).expect("published publication");
                    self.store.flush_published_apply_batch(publication).await.map_err(|error| error.into_fault())?;
                    publication.acknowledge();
                    let edit_id = self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone());
                    let label = self.registry.tool_run(&tool_id).map(|(label, _)| label.clone());
                    self.record_command(&tool_id, ActionKind::Mutation, label, edit_id, Vec::new(), None);
                    self.revalidate_interaction_on_document_change(&ActionMeta { actor, instance_id: self.live_runtime_instance_id.unwrap_or(1), view_state: None },identity).await?;
                    let entry = selected_entry_mut!(self.tool_runs).expect("finalizing slot");
                    let finalize = entry.finalize.as_mut().expect("finalize owner");
                    finalize.published = true;
                    finalize.phase = ToolRunFinalizePhase::Closing;
                    return Ok(());
                }
                Ok(store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Blocked) => return Ok(()),
                Ok(store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::PreparationProgress(_,progress))=>{self.tool_runs.retain_original_receipt(grant,progress)?;return Ok(());},
                Ok(_) => {}
                Err(_) => return self.reject_tool_run_publication(run, generation),
            }
            return Ok(());
        }
    }

    /// 🎨️ The finalize SUCCEEDED, so the provisional ops are released — but the run's payload is
    /// its result, not provisional work, and its declared windows keep rendering it once the run
    /// is `Finalized`. See `ToolRunLedger::release_provisional`.
    fn close_tool_run_finalize_owner(&mut self,grant:RetainedCloneGrant)->Result<bool,Fault>{
        if self.tool_runs.has_original_receipt(){return Ok(false);}
        let entry = selected_entry_mut!(self.tool_runs).expect("finalizing slot");
        let Some(finalize)=entry.finalize.as_mut()else{return Ok(true)};
        if let Some(publication) = finalize.publication.as_mut() {
            if publication.terminal_is_empty(){
                let copy=size_of::<Option<store::os_store::one_item_publication::ArtifactStoreBatchPublication<A::Snapshot,A::Mutation>>>();
                if !tool_grant_funds(grant,RetirementDemand{copy_bytes:copy,depth:1,..Default::default()}){return Ok(false);}
                drop(finalize.publication.take());self.tool_runs.retain_original_receipt(grant,RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;return Ok(false);
            }
            let mut demand=publication.retirement_demands(grant.maximum_copy_bytes).map_err(FaultFrom::into_fault)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original ToolRun final publication depth overflow").into_fault())?;
            if !tool_grant_funds(grant,demand){return Ok(false);}
            publication.begin_close();
            let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let step=match publication.close_step(child){Ok(step)=>step,Err(error)=>{self.tool_runs.retain_original_receipt(grant,error.retained_progress())?;return Err(error.into_fault());}};
            let step=match admit_retained_clone_close(child,step,publication.terminal_is_empty(),"tool final publication"){Ok(step)=>step,Err(error)=>{self.tool_runs.retain_original_receipt(grant,error.retained_progress())?;return Err(error.into_fault());}};
            self.tool_runs.retain_original_receipt(grant,step.progress())?;return Ok(false);
        }
        if !finalize.ingress.terminal_is_empty(){let step=match finalize.ingress.close_step(grant){Ok(step)=>step,Err(error)=>{self.tool_runs.retain_original_receipt(grant,error.retained_progress())?;return Err(error.into_fault());}};self.tool_runs.retain_original_receipt(grant,step.progress())?;return Ok(false);}
        Ok(true)
    }
    fn close_tool_run_publication(&mut self,grant:RetainedCloneGrant) -> Result<(), Fault> {
        if !self.close_tool_run_finalize_owner(grant)?{return Ok(());}
        let copy=size_of::<Option<ToolRunFinalize<A>>>();if !tool_grant_funds(grant,RetirementDemand{copy_bytes:copy,depth:1,..Default::default()}){return Ok(());}
        let entry=selected_entry_mut!(self.tool_runs).expect("finalizing original slot");let(run,generation)=(entry.slot.run,entry.slot.generation);let rejected=entry.finalize.as_ref().is_some_and(|finalize|finalize.rejected);drop(entry.finalize.take());self.tool_runs.retain_original_receipt(grant,RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;
        if rejected{self.apply_tool_run_driver_event(ToolRunEvent::StoreRejected{run,generation});self.mark_tool_run_ui_dirty();return Ok(());}
        if self.apply_tool_run_driver_event(ToolRunEvent::PublicationComplete { run, generation }) == Some(ToolRunEffect::ReleaseProvisional) {
            let store_generation = self.store.generation();
            let member_generation = self.tool_run_member_generation();
            let head = self.store.snapshot_owner();
            self.tool_runs.release_provisional(true);
            let entry = selected_entry_mut!(self.tool_runs).expect("finalized slot");
            entry.replace_base(head);
            let overlay = Arc::clone(&entry.base);
            entry.replace_overlay(overlay);
            entry.base_generation = store_generation;
            if let (Some(member), Some(generation)) = (entry.member.as_mut(), member_generation) {
                member.base_generation = generation;
            }
        }
        self.mark_tool_run_document_dirty();
        Ok(())
    }

    /// 🧩️ The member a run declaring `ToolRunDefinition::member` edits: the one live owned child composed under `slot`, with
    /// the member store generation the run rests on. No such child, or several, refuse the start by name.
    fn tool_run_member_target(&self, slot: &str) -> Result<ToolRunMemberRun, Fault> {
        let mut candidates = self.children.keyed_entries().filter(|(key, _)| key.owner.is_empty() && key.slot == slot).map(|(_, entry)| entry);
        let (Some(entry), None) = (candidates.next(), candidates.next()) else {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.member-unavailable"), format!("a member tool run needs exactly one live owned child in slot '{slot}'")));
        };
        Ok(ToolRunMemberRun {
            slot: slot.to_string(),
            child_id: entry.owner.child_id.clone(),
            dialect: entry.reference.dialect.clone(),
            base_generation: entry.member.visit_member(ToolRunMemberGeneration),
            ops: Vec::new(),
            shown: 0,
            rebase: false,
            owners: None,
            children: None,
            stale: Vec::new(),
            emit:None,emission_owner:None,
            retired_emits:std::collections::VecDeque::new(),retired_emission_owners:std::collections::VecDeque::new(),retired_ops:std::collections::VecDeque::new(),
        })
    }

    /// 🔢️ The live generation of the member the selected run edits; `None` for a document run or a member that is gone.
    fn tool_run_member_generation(&self) -> Option<u64> {
        let member = selected_entry!(self.tool_runs)?.member.as_ref()?;
        self.children.get(&(member.slot.clone(), member.child_id.clone())).map(|child| child.member.visit_member(ToolRunMemberGeneration))
    }

    /// 🔁️ Brings the selected member run's composed read up to its provisional ops within the turn wall budget: retires the
    /// reads it replaced, folds the ops the overlay does not show yet (all of them after a rebase) and leases the overlay in
    /// the member's place once every op is shown. `true` while the fold still owns this turn — also while the retirement
    /// of the reads it replaced is saturated (retryable), so the job waits instead of leasing more; a fold that cannot
    /// apply its ops (they do not decode as the member's vocabulary) faults the run.
    fn refresh_tool_run_member(&mut self, deadline: u64) -> Result<bool, Fault> {
        let Some(mut member) = selected_entry_mut!(self.tool_runs).and_then(|entry| entry.member.take()) else { return Ok(false) };
        let outcome = self.refresh_member_overlay(&mut member, deadline);
        let entry = selected_entry_mut!(self.tool_runs).expect("a member run keeps its slot");
        entry.member = Some(member);
        let (run, generation) = (entry.slot.run, entry.slot.generation);
        match outcome {
            Ok(Some((pending, conflicts, rebased))) => {
                entry.conflicts = entry.conflicts.saturating_add(conflicts);
                if !pending {
                    if rebased {
                        self.mark_tool_run_document_dirty();
                    } else {
                        self.mark_tool_run_ui_dirty();
                    }
                }
                Ok(pending)
            }
            Ok(None) => Ok(false),
            Err(fault) if fault.retryable => Ok(true),
            Err(_) => {
                self.fault_tool_run(run, generation);
                Ok(false)
            }
        }
    }

    /// 🔁️ [`Self::refresh_tool_run_member`] on a member taken out of its entry: `(fold pending, conflicts, rebased)`, or
    /// `None` when the overlay already shows every op.
    fn refresh_member_overlay(&mut self, member: &mut ToolRunMemberRun, deadline: u64) -> Result<Option<(bool, u32, bool)>, Fault> {
        let stale = std::mem::take(&mut member.stale);
        self.retire_tool_run_views(stale, &mut member.stale)?;
        if !member.rebase && member.shown == member.ops.len() {
            return Ok(None);
        }
        let child = self.children.get(&(member.slot.clone(), member.child_id.clone())).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.member-gone"), format!("the member {}/{} a tool run edits is gone", member.slot, member.child_id)))?;
        let rebased = member.rebase;
        let folded = child.member.visit_member(ToolRunMemberFold { owners: &mut member.owners, ops: &member.ops, shown: member.shown, rebase: member.rebase, deadline_us: deadline })?;
        member.rebase = false;
        member.shown = folded.shown;
        let pending = folded.shown < member.ops.len();
        if !pending {
            let next = match folded.read {
                Some((read, revision)) => Some(self.child_content_root.with_member_read(MemberKeyRef::root(&member.slot, &member.child_id), &member.dialect, revision, read)?),
                None => None,
            };
            if let Some(previous) = std::mem::replace(&mut member.children, next) {
                match self.retire_tool_run_views(vec![previous], &mut member.stale) {
                    Err(fault) if !fault.retryable => return Err(fault),
                    _ => {}
                }
            }
        }
        Ok(Some((pending, folded.conflicts, rebased)))
    }

    /// 🪦️ A terminal member run holds no provisional op and shows none: the composed reads it replaced retire and its
    /// overlay rests on the member head again, so the run owes nothing. A member that is gone has no head to rest on; a
    /// saturated retirement ring (retryable) is retried on the next turn.
    fn settle_tool_run_member(&mut self, deadline: u64) -> Result<(), Fault> {
        let Some(mut member) = selected_entry_mut!(self.tool_runs).and_then(|entry| entry.member.take()) else { return Ok(()) };
        let outcome = if self.children.get(&(member.slot.clone(), member.child_id.clone())).is_some() {
            self.refresh_member_overlay(&mut member, deadline).map(|_| ())
        } else {
            member.rebase = false;
            let stale = std::mem::take(&mut member.stale);
            self.retire_tool_run_views(stale, &mut member.stale)
        };
        selected_entry_mut!(self.tool_runs).expect("a member run keeps its slot").member = Some(member);
        match outcome {
            Err(fault) if !fault.retryable => Err(fault),
            _ => Ok(()),
        }
    }

    /// 🧹️ Retires a member run's composed reads in order through the child-content retirements, one admitted generation
    /// each; a refused admission hands the view it could not retire and every later one back to `unretired`.
    fn retire_tool_run_views(&mut self, views: Vec<ChildContentView>, unretired: &mut Vec<ChildContentView>) -> Result<(), Fault> {
        let mut views = views.into_iter();
        while let Some(view) = views.next() {
            match self.admit_child_content_publication() {
                Ok(generation) => {
                    self.child_content_retirements.insert_admitted(generation, ChildContentRetirement::new(view, false));
                    self.child_content_generation = generation;
                }
                Err(fault) => {
                    unretired.push(view);
                    unretired.extend(views);
                    return Err(fault);
                }
            }
        }
        Ok(())
    }

    /// 🏁️ A member run's finalize (§12, §20.14): its provisional ops — at most `TOOL_RUN_MEMBER_OPS_MAX` — decoded through
    /// the member's own vocabulary within the turn wall budget, then published as ONE composite group whose member edit
    /// carries the run's `TransactionRef` and group id: one history row whose mutations are the member's leaves, editable on
    /// that member store. A member that is gone or ops that do not decode reject the publication like a conflicting parent
    /// batch.
    async fn publish_tool_run_member(&mut self, deadline: u64,grant:RetainedCloneGrant) -> Result<(), Fault> {
        let app_instance_id = self.app.instance_id().await;
        let entry = selected_entry_mut!(self.tool_runs).expect("finalizing slot");
        let (run, generation) = (entry.slot.run, entry.slot.generation);
        let member = entry.member.as_mut().expect("a member run");
        if member.ops.is_empty() {
            entry.finalize.as_mut().expect("finalize owner").phase = ToolRunFinalizePhase::Closing;
            return Ok(());
        }
        if member.emit.is_none(){
            let decoded = self.children.get(&(member.slot.clone(), member.child_id.clone())).map(|child| child.member.visit_member(ToolRunMemberEmit { slot: &member.slot, child_id: &member.child_id, ops: &member.ops, emit: &mut member.emit, owner:&mut member.emission_owner, grant }));
            match decoded {
                Some(Ok((_,progress)))=>{self.tool_runs.retain_original_receipt(grant,progress)?;return Ok(());},
                _ => return self.reject_tool_run_publication(run, generation),
            }
        }
        let finalize=entry.finalize.as_mut().expect("original member finalizer");
        if finalize.ingress.transaction.is_none(){let clock=semio_framework_tool_machine::authoring_clock(run);let progress=finalize.ingress.admit_transaction(&entry.actor,&clock,app_instance_id,&entry.tool_id,grant).map_err(FaultFrom::into_fault)?;if let Some(progress)=progress{self.tool_runs.retain_original_receipt(grant,progress)?;}return Ok(());}
        let _=deadline;
        Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"ToolRun member retains its original emission and transaction until the mounted group issuer admits their unchanged full grant").into_fault())
    }

    /// ♻️ One bounded unit of member-run retirement: one alias of a member run's typed owners handed to its member store's
    /// alias retirement (displaced aliases of live runs, every alias of a run that left its slot), else the composed reads of
    /// a fully retired member run. `None` when nothing member-owned is retiring or an alias retirement is still stepping.
    fn tool_run_member_retire_step(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,Fault>{
        if self.tool_runs.snapshot_retirement.is_some(){return Ok(None);}
        if grant.maximum_items==0{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        if let Some(index)=self.tool_runs.retired_members.iter().position(|member|!member.stale.is_empty()||member.children.is_some()){
            let copy=size_of::<ChildContentView>();
            if !tool_grant_funds(grant,RetirementDemand{copy_bytes:copy,depth:1,..Default::default()}){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
            let mut member=self.tool_runs.retired_members.remove(index);
            let mut views=std::mem::take(&mut member.stale);
            views.extend(member.children.take());
            let outcome=self.retire_tool_run_views(views,&mut member.stale);
            self.tool_runs.retired_members.insert(index,member);
            return match outcome{
                Ok(())=>Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))),
                Err(fault) if fault.retryable=>Ok(Some(RetainedCloneStep::Progress(Default::default()))),
                Err(fault)=>Err(fault),
            };
        }
        let VcsArtifactApp{tool_runs,children,..}=self;
        let live=tool_runs.entries.iter_mut().filter_map(|entry|entry.member.as_mut()).map(|member|(member,false));
        let retired=tool_runs.retired_members.iter_mut().map(|member|(member,true));
        for(member,everything)in live.chain(retired){
            let queued_owner=!member.retired_emission_owners.is_empty();
            let owner=if queued_owner{member.retired_emission_owners.front_mut()}else if everything{member.emission_owner.as_mut()}else{None};
            if let Some(owner)=owner{
                if owner.is_empty(){
                    let demand=RetirementDemand{copy_bytes:size_of::<Option<Box<dyn ToolRunMemberEmissionOwner>>>(),release_bytes:size_of_val(owner.as_ref()),depth:1,..Default::default()};
                    if !tool_grant_funds(grant,demand){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
                    if queued_owner{drop(member.retired_emission_owners.pop_front());}else{drop(member.emission_owner.take());}
                    return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})));
                }
                let step=match children.get(&(member.slot.clone(),member.child_id.clone())){
                    Some(child)=>child.member.visit_member(ToolRunMemberEmissionRetire{owner:owner.as_mut(),grant})?,
                    None=>owner.close_step(grant)?,
                };return Ok(Some(step));
            }
            if !member.retired_emits.is_empty()||everything&&member.emit.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"tool member metadata retains its granted native emission retirement authority").into_fault());}
            let queued_ops=!member.retired_ops.is_empty();
            let ops=if queued_ops{member.retired_ops.front_mut()}else if everything&&(!member.ops.is_empty()||member.ops.capacity()!=0){Some(&mut member.ops)}else{None};
            if let Some(ops)=ops{
                let copy=2*size_of::<Vec<u8>>();let release=ops.last().map_or_else(||ops.capacity().checked_mul(size_of::<Vec<u8>>()),|op|Some(op.capacity())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original member operation backing overflow").into_fault())?;
                if !tool_grant_funds(grant,RetirementDemand{copy_bytes:copy,release_bytes:release,depth:1,..Default::default()}){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
                if !ops.is_empty(){drop(ops.pop());}else{drop(std::mem::take(ops));if queued_ops{drop(member.retired_ops.pop_front());}}
                return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,released_bytes:release,..Default::default()})));
            }
            macro_rules! close_member_backing{
                ($field:ident,$item:ty)=>{
                    if member.$field.is_empty()&&member.$field.capacity()!=0{
                        let release=member.$field.capacity().checked_mul(std::mem::size_of::<$item>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original member queue backing overflow").into_fault())?;
                        let copy=2*std::mem::size_of::<std::collections::VecDeque<$item>>();
                        if !tool_grant_funds(grant,RetirementDemand{copy_bytes:copy,release_bytes:release,depth:1,..Default::default()}){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
                        drop(std::mem::take(&mut member.$field));return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,released_bytes:release,..Default::default()})));
                    }
                };
            }
            close_member_backing!(retired_emission_owners,Box<dyn ToolRunMemberEmissionOwner>);
            close_member_backing!(retired_emits,ChildEmit);
            close_member_backing!(retired_ops,Vec<Vec<u8>>);
            let Some(owners)=member.owners.as_deref_mut().filter(|owners|everything||owners.has_displaced())else{continue};
            let child=children.get(&(member.slot.clone(),member.child_id.clone())).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"tool member original snapshot retains its missing exact Store issuer").into_fault())?;
            let(owner,progress)=child.member.visit_member(ToolRunMemberRetire{owners,everything,grant})?;
            if let Some(owner)=owner{tool_runs.snapshot_retirement=Some(owner);}
            if progress!=RetainedCloneProgress::default(){return Ok(Some(RetainedCloneStep::Progress(progress)));}
        }
        if let Some(index)=tool_runs.retired_members.iter().position(ToolRunMemberRun::is_retired){
            let release=tool_runs.retired_members[index].address_backing_bytes();
            let demand=RetirementDemand{copy_bytes:size_of::<ToolRunMemberRun>(),release_bytes:release,depth:1,..Default::default()};
            if !tool_grant_funds(grant,demand){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
            drop(tool_runs.retired_members.remove(index));
            return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})));
        }
        Ok(None)
    }
    pub(crate) fn tool_run_retire_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<RetainedCloneStep>,semio_framework_job::OriginalTurnFailure>{
        let grant=cx.retained_grant();if cx.should_yield(){return Ok(Some(RetainedCloneStep::Progress(Default::default())))}
        for entry in [false,true]{
            let step=if entry{self.tool_run_entry_close_step(grant)}else{self.tool_run_member_retire_step(grant)}.map_err(|fault|semio_framework_job::OriginalTurnFailure::debit_original(fault,cx))?;
            if let Some(step)=step{cx.consume_fuel(step.progress().copied_items as u64);cx.consume_retained(step.progress())?;return Ok(Some(step))}
        }
        self.tool_runs.retire_step(&mut self.store,cx)
    }
    pub(crate) fn tool_run_retirement_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.tool_runs.entries.iter().any(|entry|entry.member.as_ref().is_some_and(ToolRunMemberRun::is_work))||!self.tool_runs.retired_members.is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()});}
        if let Some(demand)=self.tool_run_entry_close_demand(body)?{return Ok(demand);}
        self.tool_runs.retirement_demands(&self.store,body)
    }
    /// 🚪️ Quotes the next close unit of the newest live entry of a closing ledger: the final publication and ingress owners of its finalize, then the entry itself.
    fn tool_run_entry_close_demand(&self,body:usize)->Result<Option<RetirementDemand>,ValueError>{
        if !self.tool_runs.closing{return Ok(None);}
        let Some(entry)=self.tool_runs.entries.last()else{return Ok(None)};
        if let Some(finalize)=entry.finalize.as_ref(){
            if let Some(publication)=finalize.publication.as_ref(){
                if publication.terminal_is_empty(){return Ok(Some(RetirementDemand{copy_bytes:size_of::<Option<store::os_store::one_item_publication::ArtifactStoreBatchPublication<A::Snapshot,A::Mutation>>>(),depth:1,..Default::default()}));}
                let mut demand=publication.retirement_demands(body)?;
                demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"tool entry final publication depth overflow"))?;
                return Ok(Some(demand));
            }
            if !finalize.ingress.terminal_is_empty(){return finalize.ingress.close_demands(body).map(Some);}
        }
        Ok(Some(RetirementDemand{copy_bytes:size_of::<ToolRunEntry<A>>(),release_bytes:size_of::<ToolRunEntry<A>>(),depth:1,..Default::default()}))
    }
    /// 🚪️ Closes the newest live entry of a closing ledger one funded unit at a time; its job, member and provisional owners move to the ledger's retiring lists.
    fn tool_run_entry_close_step(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,Fault>{
        let Some(demand)=self.tool_run_entry_close_demand(grant.maximum_copy_bytes).map_err(FaultFrom::into_fault)?else{return Ok(None)};
        if self.tool_runs.has_original_receipt()||grant.maximum_items==0||!tool_grant_funds(grant,demand){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        let entry=self.tool_runs.entries.last_mut().expect("a quoted close unit has its entry");
        if let Some(finalize)=entry.finalize.as_mut(){
            if let Some(publication)=finalize.publication.as_mut(){
                if publication.terminal_is_empty(){drop(finalize.publication.take());return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})));}
                publication.begin_close();
                let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
                let step=publication.close_step(child).map_err(FaultFrom::into_fault)?;
                let step=admit_retained_clone_close(child,step,publication.terminal_is_empty(),"tool entry final publication").map_err(FaultFrom::into_fault)?;
                return Ok(Some(RetainedCloneStep::Progress(step.progress())));
            }
            if !finalize.ingress.terminal_is_empty(){return finalize.ingress.close_step(grant).map(|step|Some(RetainedCloneStep::Progress(step.progress()))).map_err(FaultFrom::into_fault);}
        }
        let entry=self.tool_runs.entries.pop().expect("a quoted close unit has its entry");
        self.tool_runs.selected=None;
        self.tool_runs.retire_entry(*entry);
        Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})))
    }
    /// 🎟️ Forwards the original full grant and retains every unsupported cleanup frontier in place.
    pub(crate) fn tool_run_close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<RetainedCloneStep,semio_framework_job::OriginalTurnFailure>{
        if !self.tool_runs.closing{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"ToolRun close requires genuine granted entry cancellation admission").into())}
        Ok(self.tool_run_retire_step(cx)?.unwrap_or_else(||if self.tool_runs.terminal_is_empty(){RetainedCloneStep::Complete(Default::default())}else{RetainedCloneStep::Progress(Default::default())}))
    }

}
