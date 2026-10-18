//! 🪄️ Supersede authoring and extension of the instance time-travel turns: the supersede authoring steps and results, the history edit row revert, the frozen action fault and the time-travel extension that registers the turns with the instance. See [`super`].

use super::*;

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> TimeTravelCtx<'ctx, A, M, X> {
    /// ✍️ Authors `inputs` within `scope` (unscoped: every line) resumably (design §16.6) — the restore of an undo or the
    /// re-authoring of a redo: a Report replay of the inputs over the applied history, stepped within this turn's budget and
    /// then per reactor turn, finalized by `commit_finished_replay` without replaying again. Answers whether it landed, is
    /// still replaying, was refused (an input no longer applied, or a report that blocks finalizing; nothing changes) or
    /// waits for an authoring already running.
    async fn author_supersede(&mut self, scope: Option<String>, inputs: Vec<protocol::SupersededInput>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<SupersedeAuthored, Fault> {
        if self.time_travel.authoring.is_some() {
            return Ok(SupersedeAuthored::Busy);
        }
        let drafts: protocol::HistoryInputDrafts = inputs.into_iter().map(|input| (input.target, input.replacement)).collect();
        let finalization = match scope {
            Some(alternative_id) => store::HistoryFinalization::Scope { alternative_id },
            None => store::HistoryFinalization::Overwrite,
        };
        match self.stores.store.begin_derived_report_replay(drafts, None) {
            Ok(replay) => self.time_travel.authoring = Some(SupersedeAuthoring { replay, finalization }),
            Err(vcs::VcsError::ValidationFailed(_) | vcs::VcsError::Rejected { .. }) => return Ok(SupersedeAuthored::Refused),
            Err(error) => return Err(error.into_fault()),
        }
        self.step_supersede_authoring(self.shared().time_travel_turn_deadline(), identity).await
    }

    /// ⏭️ One turn of the undo or redo of a finalize being authored: steps its replay until the turn deadline, then commits
    /// the finished replay in its scope; a store that moved meanwhile replays the same inputs again from the new base.
    pub(super) async fn step_supersede_authoring(&mut self, deadline_us: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<SupersedeAuthored, Fault> {
        let stepped = {
            let TimeTravelCtx { stores, time_travel, .. } = self;
            let store = &mut stores.store;
            let clock = time_travel.turn_clock;
            let Some(authoring) = time_travel.authoring.as_mut() else { return Ok(SupersedeAuthored::Refused) };
            let mut deadline = || clock().is_none_or(|now| now >= deadline_us);
            let grant=history_planning_retirement_grant(authoring.replay.planning_retirement_demands(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).map_err(ValueError::into_fault)?);
            store.step_derived_report_replay(&mut authoring.replay, grant, &mut deadline)
        };
        match stepped {
            Ok(store::os_store::replay::ReplayStep::Pending(_)) => return Ok(SupersedeAuthored::Pending),
            Ok(store::os_store::replay::ReplayStep::Finished(_)) => {}
            Err(_) => {
                if let Some(authoring) = self.time_travel.authoring.take() {
                    let SupersedeAuthoring {replay,finalization}=authoring;let mut replay=Some(replay);
                    let grant=history_retirement_frame_grant(ArtifactStore::<A::Snapshot,A::Mutation>::history_read_retirement_birth_bytes());
                    match self.stores.store.retire_derived_report_replay(&mut replay,grant) {
                        Ok(Some((retirement,progress)))=>{assert!(progress.fits(grant));self.time_travel.document.queue_retirement(retirement);}
                        Ok(None)=>unreachable!("retained supersede authoring is present"),
                        Err(error)=>{self.time_travel.authoring=Some(SupersedeAuthoring {replay:replay.take().unwrap(),finalization});return Err(error.into_fault());}
                    }
                }
                return Ok(SupersedeAuthored::Refused);
            }
        }
        let SupersedeAuthoring { replay, finalization } = self.time_travel.authoring.take().expect("a finished authoring replay was held");
        let Ok((result, head)) = self.stores.store.finish_derived_report_replay(&mut Some(replay)) else { return Ok(SupersedeAuthored::Refused) };
        if let Some(head) = head {
            self.time_travel.document.retire(&self.stores.store, Some(head))?;
        }
        let drafts = result.drafts().clone();
        match self.stores.store.commit_finished_replay(result, finalization.clone(), identity).await {
            Ok(_) => {
                self.history.cache = None;
                Ok(SupersedeAuthored::Landed)
            }
            Err(vcs::VcsError::Stale { .. }) => match self.stores.store.begin_derived_report_replay(drafts, None) {
                Ok(replay) => {
                    self.time_travel.authoring = Some(SupersedeAuthoring { replay, finalization });
                    Ok(SupersedeAuthored::Pending)
                }
                Err(_) => Ok(SupersedeAuthored::Refused),
            },
            Err(vcs::VcsError::ValidationFailed(_) | vcs::VcsError::Rejected { .. } | vcs::VcsError::UnknownAlternative(_)) => Ok(SupersedeAuthored::Refused),
            Err(error) => Err(error.into_fault()),
        }
    }

    /// ⏪️⏩️ A plain `undo` or `redo` whose newest own target is a history edit (design §2: the undo of a finalize is a new
    /// `Supersede` carrying the previous effective input, its redo re-authors the edit's own inputs); `None` leaves the
    /// verb to the document, configuration, child and shell lanes.
    pub async fn dispatch_supersede_history_action(&mut self, action: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Option<InvocationResult>, Fault> {
        self.as_render().refresh_cache().await?;
        let (child_applied_tails, child_has_redo_tail) = self.child_composition.child_history_tails().await;
        let authored = match action {
            "undo" => self.history.supersede_undo_target(&*self.stores, &child_applied_tails).map(|entry| self.history.supersedes.restore(entry)),
            "redo" if !child_has_redo_tail => self.history.supersede_redo_target(&*self.stores).map(|(entry, undo)| self.history.supersedes.redo(entry, undo)),
            _ => None,
        };
        let Some((scope, inputs)) = authored.filter(|(_, inputs)| !inputs.is_empty()) else { return Ok(None) };
        let authored = self.author_supersede(scope, inputs, identity).await?;
        Ok(Some(self.supersede_authored_result(action, meta, authored).await))
    }

    /// 🧾️ The result of a verb that authored the undo or redo of a finalize: a landed one changed the history (every body
    /// refreshes), a pending one lands in a later reactor turn, an authoring already running answers `timeTravel.busy`.
    async fn supersede_authored_result(&mut self, action: &str, meta: &ActionMeta, authored: SupersedeAuthored) -> InvocationResult {
        let (events, scope) = match authored {
            SupersedeAuthored::Landed => (vec![history_changed_event().await], UiDirtyScope::Full),
            SupersedeAuthored::Pending | SupersedeAuthored::Refused | SupersedeAuthored::Busy => (Vec::new(), UiDirtyScope::None),
        };
        let mut result = ToolOperations::<A>::empty_result(action, meta, Vec::new(), events, scope).await;
        if authored == SupersedeAuthored::Busy {
            result.output = DslValue::Object(vec![("rejected".into(), DslValue::String(TIME_TRAVEL_BUSY_CODE.into()))]);
        }
        result
    }

    /// ⏪️ `revertToCommand` on a history-edit row: takes back the history edit the row acts for, when it is this replica's
    /// and still in effect.
    pub async fn revert_history_edit_row(&mut self, transition_id: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        let local = self.stores.supersede_author().to_string();
        let applied_entries = self.history.supersedes.applied_entries(self.stores.store.supersessions().iter());
        let target = self.history.supersedes.record(transition_id).filter(|(_, record)| record.role != SupersedeRole::Undo && record.actor.as_str() == local).map(|(_, record)| record.entry).filter(|entry| self.history.supersedes.undoable(&local, *entry, &applied_entries));
        let authored = match target.map(|entry| self.history.supersedes.restore(entry)).filter(|(_, inputs)| !inputs.is_empty()) {
            Some((scope, inputs)) => self.author_supersede(scope, inputs, identity).await?,
            None => SupersedeAuthored::Refused,
        };
        Ok(self.supersede_authored_result(REVERT_TO_COMMAND_ACTION_ID, meta, authored).await)
    }
}

/// 🧊️ The framework-reserved verbs a history-edit session freezes: the history-lane verbs (they move the applied
/// stack or the head and would invalidate the session base silently) and the clipboard verbs that edit the document.
fn time_travel_freezes_reserved_action(action: &str) -> bool {
    HISTORY_ACTION_IDS.contains(&action) || matches!(action, REVERT_TO_COMMAND_ACTION_ID | "cut" | "paste")
}

/// 🥶️ The `timeTravel.frozen` refusal of `verb` while a history edit is open on this instance.
fn time_travel_frozen_fault(verb: &str) -> Fault {
    Fault::new(FaultOrigin::Framework, FaultCode::new(semio_framework_time_travel::TIME_TRAVEL_FROZEN_CODE), format!("verb {verb:?} would change the document while a history edit is open on this instance"))
}

/// 🕰️ Non-destructive history editing as an instance extension: the `historyEdit*` verbs, the document freeze an open
/// session holds, and the session's maintenance turn.
#[doc(hidden)]
pub struct TimeTravelExtension;

impl InstanceExtension for TimeTravelExtension {
    fn claims_action(action: &str) -> bool {
        is_time_travel_action_id(action)
    }

    fn reads_history(action: &str) -> bool {
        is_time_travel_action_id(action)
    }

    fn admitted_while_loading(action: &str) -> bool {
        action == HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID
    }

    async fn dispatch_action<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &mut ExtensionCtx<'_, A, M, X>, action: &str, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let mut ctx = TimeTravelCtx::of_extension(ctx);
        ctx.as_render().freeze_tool_machines();
        ctx.dispatch_time_travel_action(action, args, meta, identity, cx).await
    }

    fn reserved_action_refusal<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>, action: &str) -> Option<Fault> {
        (time_travel.freezes_local_emits() && time_travel_freezes_reserved_action(action)).then(|| time_travel_frozen_fault(action))
    }

    fn freezes_document<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>) -> bool {
        time_travel.freezes_local_emits()
    }

    fn document_refusal<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>, verb: &str) -> Option<Fault> {
        time_travel.freezes_local_emits().then(|| time_travel_frozen_fault(verb))
    }

    fn transaction_refusal<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>, txn_id: &str) -> Option<Fault> {
        time_travel.freezes_local_emits().then(|| MediaTransfers::<A>::transaction_fault(FaultOrigin::Framework, semio_framework_time_travel::TIME_TRAVEL_FROZEN_CODE, format!("transaction {txn_id:?} would change the document while a history edit is open on this instance")))
    }

    fn has_pending_work<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &ExtensionRef<'_, A, M, X>) -> bool {
        TimeTravelRef::of_extension_ref(ctx).time_travel_has_pending_work()
    }

    fn holds_receipt<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>) -> bool {
        time_travel.has_command_receipt()
    }

    fn terminal_is_empty<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>) -> bool {
        time_travel.terminal_is_empty()
    }

    async fn drive_turn<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &mut ExtensionCtx<'_, A, M, X>, retained: &mut RetainedCloneGrant, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let mut ctx = TimeTravelCtx::of_extension(ctx);
        if !ctx.shared().time_travel_has_pending_work() {
            return Ok(false);
        }
        let grant = semio_framework_plugin_reactor_app::original_plugin_turn_grant(ctx.mounted_policy.maintenance, *retained);
        ctx.drive_time_travel_turn(grant, identity, cx).await?;
        Ok(ctx.time_travel.has_command_receipt())
    }
}
