//! 🎬️ Action dispatch and effects of the instance time-travel turns: the dispatched and applied time-travel actions, the event reduction and the performed effects. See [`super`].

use super::*;

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> TimeTravelCtx<'ctx, A, M, X> {
    /// ⏪️ Host-driven routing of the reserved history-edit verbs (design §7): applied to the session now, never queued
    /// behind guest work. A refusal is a silent `{rejected}` result that still repaints the history body, so a stale
    /// button never keeps its stale identity. The reply carries the owed UI scope; the session status the verb moved to
    /// (its stage and generation) is prepared at once as the history patch of the unsolicited UI progress frame of this
    /// same turn, so every dispatch route — a command's reply, a retained-surface intent's reply that carries no patch —
    /// publishes the new generation with the stage flip and a verb stamped with it is never stale.
    pub(crate) async fn dispatch_time_travel_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let mut effects = Vec::new();
        let outcome = self.apply_time_travel_action(action, args, meta, &mut effects, identity, cx).await?;
        let output = match outcome {
            TimeTravelActionOutcome::Pending => DslValue::Object(vec![("timeTravel".into(),DslValue::String("pending".into()))]),
            TimeTravelActionOutcome::Applied(stage) => DslValue::Object(vec![("timeTravel".into(), DslValue::String(stage.as_str().into()))]),
            TimeTravelActionOutcome::Rejected(refusal) => DslValue::Object(vec![("rejected".into(), DslValue::String(refusal.code().into()))]),
        };
        let ui_scope = self.time_travel.dirty_scope();
        self.time_travel.ui_dirty = false;
        self.time_travel.document_dirty = false;
        self.prepare_time_travel_patch().await?;
        let mut result = ToolOperations::<A>::empty_result(action, meta, effects, Vec::new(), ui_scope).await;
        result.output = output;
        Ok(result)
    }

    /// ⚖️ Applies one history-edit verb: delivers a pending base change first, then the verb's event with its effects.
    pub async fn apply_time_travel_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, effects: &mut Vec<Effect>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<TimeTravelActionOutcome, Fault> {
        if self.time_travel.owners().any(TimeTravelOwners::command_pending)||self.time_travel.command_receipt.is_some()||self.time_travel.pending_command.is_some()||self.time_travel.discard_cursor.is_some()||!self.time_travel.command_retirements.terminal_is_empty(){return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Busy))}
        self.watch_time_travel_base(identity).await?;
        if !self.time_travel.is_active() {
            match action {
                HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID => return Ok(self.cancel_reprojection_turns()),
                HISTORY_EDIT_RERUN_ACTION_ID => return Ok(self.resume_reprojection_turns()),
                _ => {}
            }
        }
        let generation = time_travel_arg_generation(args).unwrap_or(self.time_travel.session.generation);
        let event = match action {
            HISTORY_EDIT_BEGIN_ACTION_ID => return self.begin_time_travel(args, false, meta, effects, identity, cx).await,
            HISTORY_EDIT_INPUT_ACTION_ID => {
                let value = args.and_then(|args| args.get(HISTORY_EDIT_ARG_VALUE)).cloned();
                return self.draft_time_travel_input(time_travel_arg_text(args, HISTORY_EDIT_ARG_PATH), value, time_travel_arg_text(args, HISTORY_EDIT_ARG_EDIT), generation, meta, effects, identity).await;
            }
            HISTORY_EDIT_USE_SELECTION_ACTION_ID => return self.draft_time_travel_selection(time_travel_arg_text(args, HISTORY_EDIT_ARG_PATH), generation, meta, effects, identity).await,
            HISTORY_EDIT_WITHDRAW_ACTION_ID if time_travel_arg_text(args, HISTORY_EDIT_ARG_MUTATION_ID).is_some() => return self.begin_time_travel(args, true, meta, effects, identity, cx).await,
            HISTORY_EDIT_WITHDRAW_ACTION_ID => TimeTravelEvent::Withdraw { generation },
            HISTORY_EDIT_ACCEPT_ACTION_ID => TimeTravelEvent::Accept { generation },
            HISTORY_EDIT_DISCARD_ACTION_ID => TimeTravelEvent::Discard { generation },
            HISTORY_EDIT_RESTORE_ACTION_ID => {
                let Some(mutation) = time_travel_arg_text(args, HISTORY_EDIT_ARG_MUTATION_ID) else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownMutation)) };
                if self.time_travel.member_store().as_deref() != time_travel_arg_text(args, HISTORY_EDIT_ARG_STORE).filter(|store| !store.is_empty()) {
                    return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal)));
                }
                TimeTravelEvent::Restore { generation, target: MutationId(mutation.to_string()) }
            }
            HISTORY_EDIT_FINALIZE_ACTION_ID => TimeTravelEvent::RequestFinalize { generation },
            HISTORY_EDIT_COMMIT_ACTION_ID => {
                let choice = match time_travel_arg_text(args, DIALOG_CHOICE_ARG) {
                    Some(HISTORY_EDIT_CHOICE_OVERWRITE) => TimeTravelChoice::Overwrite,
                    _ => match time_travel_arg_text(args, HISTORY_EDIT_ARG_NAME).filter(|name| !name.trim().is_empty()) {
                        Some(name) if is_time_travel_alternative_name(name) => TimeTravelChoice::Alternative { name: name.to_string() },
                        Some(_) => return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::NameInvalid)),
                        None => match meta.view_state.as_ref() {
                            Some(view) => TimeTravelChoice::Alternative { name: TimeTravelLabel::AlternativeNameDefault.localized(LocalizedLabel::native).resolve(Terminology::Native, view.locale).to_string() },
                            None => return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::NameRequired)),
                        },
                    },
                };
                TimeTravelEvent::Choose { generation, choice }
            }
            HISTORY_EDIT_BACK_ACTION_ID => TimeTravelEvent::Back { generation },
            HISTORY_EDIT_EXIT_ACTION_ID => TimeTravelEvent::Exit,
            HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID => TimeTravelEvent::ReplayCancelled { generation },
            HISTORY_EDIT_RERUN_ACTION_ID => TimeTravelEvent::Rerun { generation },
            _ => return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("timeTravel.unknown-action"), format!("'{action}' is not a history-edit verb"))),
        };
        self.apply_time_travel_event(event, Some(meta), effects, identity).await
    }

    /// ⚖️ Applies one event to the session and performs its effects; a refusal changes nothing. Every window body is
    /// re-published only when the event or its effects swapped the document a window shows (the committed document, the
    /// draft preview, the replayed head); any other edge — the finalize prompt opening or closing, an accepted draft
    /// starting its replay, a replay run again — re-publishes the history body and the chips alone.
    async fn reduce_unprepared_time_travel_event(&mut self, event: TimeTravelEvent, meta: Option<&ActionMeta>, effects: &mut Vec<Effect>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TimeTravelActionOutcome, Fault> {
        let shown = self.shared().time_travel_shown_identity();
        match self.time_travel.session.apply(event) {
            Ok(session_effects) => {
                self.note_time_travel_changed(false, true);
                self.perform_time_travel_effects(session_effects, meta, effects, identity).await?;
                self.time_travel.document_dirty |= shown != self.shared().time_travel_shown_identity();
                Ok(TimeTravelActionOutcome::Applied(self.time_travel.session.stage))
            }
            Err(refusal) => {
                self.time_travel.ui_dirty = true;
                Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(refusal)))
            }
        }
    }

    pub(super) async fn apply_time_travel_event(&mut self,event:TimeTravelEvent,meta:Option<&ActionMeta>,effects:&mut Vec<Effect>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<TimeTravelActionOutcome,Fault>{
        if self.time_travel.owners().any(TimeTravelOwners::command_pending)||self.time_travel.command_receipt.is_some()||self.time_travel.pending_command.is_some()||self.time_travel.discard_cursor.is_some()||!self.time_travel.command_retirements.terminal_is_empty(){return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Busy))}
        if let TimeTravelEvent::Discard{generation}=&event{
            if *generation!=self.time_travel.session.generation{return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Stale)))}
            if self.time_travel.session.stage!=TimeTravelStage::Editing||self.time_travel.session.pending.is_none(){return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal)))}
        }
        if matches!(event,TimeTravelEvent::Discard{..}){let grant=self.mounted_policy.maintenance;let mut original=Some(event);let receipt=self.time_travel.stage_time_travel_command(&mut original,grant).map_err(ValueError::into_fault)?;if let Some(receipt)=receipt{assert!(receipt.fits(grant));self.time_travel.command_receipt=Some((grant,receipt));self.time_travel.ui_dirty=true;return Ok(TimeTravelActionOutcome::Pending)}return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Busy))}
        self.reduce_unprepared_time_travel_event(event,meta,effects, identity).await
    }

    /// 🛠️ Performs the session's effects in order; a commit's own `Finalized`/`FinalizeFaulted` effects follow it.
    pub(super) async fn perform_time_travel_effects(&mut self, effects: Vec<TimeTravelEffect>, meta: Option<&ActionMeta>, kernel: &mut Vec<Effect>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let mut queue: VecDeque<TimeTravelEffect> = effects.into();
        while let Some(effect) = queue.pop_front() {
            match effect {
                TimeTravelEffect::ShowPreview { target, replacement } => self.show_time_travel_preview(&target, &replacement).await?,
                TimeTravelEffect::StartReplay { drafts, from } => queue.extend(self.start_time_travel_replay(drafts, &from).await?),
                TimeTravelEffect::CancelReplay => {
                    self.time_travel_run(TimeTravelStoreCommand::CancelReplay,None).await?;
                }
                TimeTravelEffect::OpenFinalizePrompt => {
                    let args = meta
                        .and_then(|meta| meta.view_state.as_ref())
                        .map(|view| DslValue::object([(HISTORY_EDIT_ARG_NAME.to_string(), DslValue::String(TimeTravelLabel::AlternativeNameDefault.localized(LocalizedLabel::native).resolve(Terminology::Native, view.locale).to_string()))]));
                    kernel.push(Effect::OpenDialog { req: TIME_TRAVEL_FINALIZE_REQUEST, dialog_id: HISTORY_EDIT_FINALIZE_DIALOG_ID.to_string(), args });
                }
                TimeTravelEffect::CommitOverwrite { .. } => queue.extend(self.commit_time_travel(store::HistoryFinalization::Overwrite, meta, identity).await?),
                TimeTravelEffect::CommitAlternative { name, .. } => queue.extend(self.commit_time_travel(store::HistoryFinalization::Alternative { name }, meta, identity).await?),
                TimeTravelEffect::Close => self.time_travel.document_dirty = true,
            }
        }
        self.settle_time_travel_owners()?;
        self.refresh_time_travel_children().await
    }

}
