//! 🚀️ Action dispatch and start of the instance tool run turns: the dispatched and applied tool run actions, the selected action, the run start, driver events, the job port drain, the dirty marks and the trace lane injection. See [`super`].

use super::*;
use semio_framework_os_kernel::DslValue;
use semio_framework::kernel::UiDirtyScope;
use semio_framework::Fault;
use semio_framework::FaultOrigin;
use semio_framework::FaultCode;
use std::collections::BTreeSet;
use semio_framework::InvocationResult;

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> ToolRunCtx<'ctx, A, M, X> {
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
        let mut result = ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), ui_scope).await;
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
                    self.stores.store.cancel_apply_batch(publication);
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
            .or_else(|| view_state.and_then(|view| view.active_utility_id.clone().filter(|id| self.tools.registry.tool_run(id).is_some()).or_else(|| view.active_tool_id.clone())))
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.tool-id"), "toolRunStart needs a toolId argument or an active tool"))?;
        let definition =
            self.tools.registry.tool_run(&tool_id).map(|(_, definition)| definition.clone()).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.unknown-tool"), format!("tool '{tool_id}' declares no ToolRunDefinition")))?;
        let window_id = args.and_then(|args| args.get(TOOL_RUN_ARG_WINDOW_ID)).and_then(DslValue::as_str).map(str::to_string).or_else(|| view_state.and_then(|view| view.window_id.clone().or_else(|| view.focused_window_id.clone())));
        if definition.mutating && self.time_travel.freezes_local_emits() {
            return Ok(ToolRunActionOutcome::Rejected(ToolRunRejection::Busy));
        }
        let member = definition.member.as_deref().map(|slot| self.child_composition.tool_run_member_target(slot)).transpose()?;
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
        self.as_render().refresh_cache().await?;
        let (base, _, _) = self.history.command_cache_inputs();
        let identity = ToolRunIdentity::new(ToolRunId { app_instance_id: self.tools.live_runtime_instance_id.unwrap_or(meta.instance_id), run }, self.stores.store.content_revision());
        let settings_generation = self.shared().tool_run_settings_generation();
        let window = window_id.and_then(|window_id| view_state?.window_instances.iter().find(|window| window.id == window_id).map(|window| (window.id.clone(), window.window_kind_id.clone())));
        let settings_values = self.stores.tool_run_settings_values(&definition.settings, window.as_ref());
        let window_bodies = definition.windows.iter().filter_map(|window_kind_id| self.tools.registry.window_body_key(window_kind_id).map(str::to_string)).collect();
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
            base_generation: self.stores.store.generation(),
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

    pub(super) fn apply_tool_run_driver_event(&mut self, event: ToolRunEvent) -> Option<ToolRunEffect> {
        self.tool_runs.apply_event(event).ok()
    }

    /// 📮️ Hands the host every effect jobs queued on the port, in order, while the typed effect outbox has room.
    pub(super) fn drain_tool_run_port(&mut self) {
        for index in 0..self.tool_runs.entries.len() {
            while let Some(effect) = self.tool_runs.entries[index].port.pop_effect() {
                if let Err(effect) = self.tools.typed_effect_outbox.push(effect) {
                    self.tool_runs.entries[index].port.restore_effect(effect);
                    return;
                }
            }
        }
    }

    /// 🎯️ A tick-sized change: progress, steps, trace and provisional appends reach the panel and the scene
    /// windows only (`ToolRunLedger::dirty_scope`).
    pub(super) fn mark_tool_run_ui_dirty(&mut self) {
        self.tool_runs.ui_dirty = true;
        self.flush_tool_run_ui_dirty();
    }

    /// 📄️ The document every window reads was swapped wholesale (overlay refolded, released or discarded), so
    /// every body is dirty once.
    pub(super) fn mark_tool_run_document_dirty(&mut self) {
        self.tool_runs.document_dirty = true;
        self.flush_tool_run_ui_dirty();
    }

    /// 📮️ Hands the owed scope to the typed UI outbox once it has room; until then the flags stay set, so a
    /// queued unrelated scope never swallows a run change.
    pub fn flush_tool_run_ui_dirty(&mut self) {
        if !self.tool_runs.is_ui_dirty() || self.tools.typed_ui_outbox.len() != 0 {
            return;
        }
        let scope = if self.tool_runs.document_dirty { UiDirtyScope::Full } else { self.tool_runs.dirty_scope() };
        if self.tools.typed_ui_outbox.push(scope).is_ok() {
            self.tool_runs.ui_dirty = false;
            self.tool_runs.document_dirty = false;
        }
    }

    /// 🎬️ Scene lane injection (§3.2, §4.1 layer 2): the first World3d, Canvas2d or Board2d scene surface a window body
    /// renders carries the `toolRunTrace` lane answered for the cursor that window's renderer echoed.
    /// Panel bodies, override renders and bodies without a scene surface carry nothing.
    pub(crate) fn inject_tool_run_trace_lane(&mut self, body_key: &str, view_state: &ViewModel) -> Result<(), Fault> {
        let Some(tree) = self.pending.pending_render_tree.as_mut() else { return Err(semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault("trace injection requires its stationary render source")); };
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

}
