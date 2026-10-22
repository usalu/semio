//! 👀️ Generation watch and turn driver of the instance tool run turns: the store and settings generation watch, the per-turn driver, the retirement loop, the selected run drive and the refold. See [`super`].

use super::*;
use semio_framework::Fault;

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> ToolRunCtx<'ctx, A, M, X> {
    /// 👀️ Store and settings watch (§3.3): emits `baseChanged` / `settingsChanged` with their policies.
    ///
    /// - A base change rebinds a retargetable job to the new identity in place; any other job is closed and
    ///   rebuilt under the new identity (`restart` also discards the provisional ops), so no tick of the old
    ///   identity is ever produced.
    /// - A config or window-config publication is only a settings change when a value behind one of the run's
    ///   declared `settings` pointers changed; `resume` then retargets a retargetable job in place and rebuilds
    ///   any other from its checkpoint.
    fn watch_tool_run_generations(&mut self) {
        let store_generation = self.stores.store.generation();
        let settings_generation = self.shared().tool_run_settings_generation();
        let member_generation = self.shared().tool_run_member_generation();
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
            let head = self.stores.store.snapshot_owner();
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
            let head = self.stores.store.snapshot_owner();
            let restart = rebase == ToolRunRebasePolicy::Restart;
            if restart {
                self.tool_runs.close_current_job();
                self.tool_runs.discard_provisional();
            }
            let entry = selected_entry_mut!(self.tool_runs).expect("refold keeps the slot");
            entry.replace_base(head);
            entry.base_generation = store_generation;
            entry.identity.base_revision = self.stores.store.content_revision();
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
        let values = self.stores.tool_run_settings_values(&entry.definition.settings, entry.window.as_ref());
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
                let config = self.stores.config_store.snapshot_owner();
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
        let grant=cx.retained_grant();
        let outcome=if self.tool_runs.select_run(run){match cx.with_native_receiving_ready(async |receiving| self.drive_selected_tool_run(deadline,grant,identity,receiving).await){Ok(driven)=>driven.map_err(semio_framework_job::OriginalTurnFailure::from),Err(error)=>Err(error.into())}}else{Ok(())};
        self.tool_runs.select_primary();
        outcome
    }

    /// 🚰️ Retirement units until nothing retires, a retirement blocks or the turn deadline. `true` while owners still
    /// retire or a unit released nothing: the runs wait for the next turn instead of displacing more.
    fn retire_tool_runs_until(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let grant=cx.retained_grant();let Some(step)=self.tool_run_retire_step(cx)?else{return Ok(false)};self.tool_runs.retain_original_receipt(grant,step.progress())?;Ok(true)
    }

    /// ⏯️ The selected run's share of one driver turn.
    async fn drive_selected_tool_run(&mut self, deadline: u64,grant:RetainedCloneGrant,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<(), Fault> {
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
                if self.refold_tool_run(deadline) || self.refresh_tool_run_member(deadline,grant,receiving)? {
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
                if self.refresh_tool_run_member(deadline,grant,receiving)? {
                    return Ok(());
                }
                self.finalize_tool_run_turn(deadline,grant,identity,receiving).await
            }
            ToolRunState::Finalized | ToolRunState::Aborted | ToolRunState::Faulted => self.settle_tool_run_member(deadline,grant,receiving),
        }
    }

    /// 🔁️ `true` while a refold still owns this turn; a refold waiting only for its job's boundary lets the job step.
    pub(super) fn refold_tool_run(&mut self, deadline: u64) -> bool {
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

}
