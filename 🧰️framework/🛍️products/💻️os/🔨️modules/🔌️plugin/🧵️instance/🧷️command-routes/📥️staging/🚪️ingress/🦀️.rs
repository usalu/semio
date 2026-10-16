//! 📌️ Staged slice of the `ingress` band of the mounted artifact instance: its `instance-commands` members (WP6 step S0 of the instance family).
//!
//! 🚪️ Ingress of the mounted artifact instance: framework-reserved job driving, constructors, store-constructor units, envelope ingress and decode, document archive genesis and merge, child members and packs.
use super::*;
use crate::tool_jobs::time_travel;
use super::docs_maintenance_ladder::{item};

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🎟️ Seals one genuine publication slot and keyed lease before a reserved producer receives source owners.
    pub(super) fn admit_reserved_emit_producer(&mut self,verb:&str,meta:&ActionMeta,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<FrameworkReservedCommitPermit>,semio_framework_job::OriginalTurnFailure>{
        let proof=self.tools.qualified_tool_proof(&self.stores, verb)?;
        if !matches!(proof,QualifiedToolProof::FrameworkOwned(_)|QualifiedToolProof::AppOwned(_)){return Err(plugin_sdk_fault("reserved producer lacks its exact registered publication proof").into())}
        let Some((operation,lease))=self.admit_original_tool_lease(verb,meta,true,cx)?else{return Ok(None)};
        FrameworkReservedCommitPermit::new(operation,lease,proof.contract().max_output_bytes).map(Some).map_err(Into::into)
    }

    /// 🔬️ Exercises real reserved slot admission without exporting private producer authority.
    #[cfg(test)]
    pub(crate) fn test_reserved_emit_admission_case(&mut self, occupied: usize, meta: &ActionMeta) -> bool {
        let before = self.tools.typed_operation_reservations;
        for slot in 0..occupied { self.tools.typed_operation_reservations[slot] = Some((slot + 128) as u64); }
        let original = self.tools.typed_operation_reservations;
        let admitted = match self.admit_reserved_emit_producer("cut", meta) {
            Ok(permit) => {
                let id = permit.operation.operation.0;
                let slot = id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS;
                assert_eq!(slot, occupied);
                assert_eq!(self.tools.typed_operation_reservations[slot], Some(id));
                assert!(permit.lease.keyed);
                assert_eq!(permit.lease.key.operation_id, permit.operation.operation);
                assert_eq!(&self.tools.typed_operation_reservations[..occupied], &original[..occupied]);
                self.tools.release_reserved_emit_slot(id + ARTIFACT_LIVE_OUTPUT_SLOTS as u64);
                assert_eq!(self.tools.typed_operation_reservations[slot], Some(id));
                self.tools.finish_reserved_emit_producer(permit);
                assert_eq!(self.tools.typed_operation_reservations, original);
                true
            }
            Err(_) => { assert_eq!(self.tools.typed_operation_reservations, original); false }
        };
        self.tools.typed_operation_reservations = before;
        admitted
    }

    /// 🧪️ Keeps cancelled worker completion tied to the actual original permit through handoff.
    #[cfg(test)]
    pub(crate) async fn test_reserved_emit_cancelled_worker_handoff(&mut self, meta: &ActionMeta) -> bool {
        let permit = self.admit_reserved_emit_producer("cut", meta).expect("original cut producer admission");
        let id = permit.operation.operation;
        permit.lease.cancel();
        let job = ArtifactReservedToolJob::new(FrameworkCutJob::new(Vec::new(), 0));
        match self.tools.run_framework_reserved_job(&self.mounted_policy, &mut self.stores, "cut", &[], 0, job, meta, None, Some(permit)).await {
            Ok(permit) => { assert_eq!(permit.operation.operation, id); assert_eq!(self.tools.typed_operation_reservations[id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS], Some(id.0)); self.tools.finish_reserved_emit_producer(permit); true }
            Err(_) => false,
        }
    }

    /// 🧫️ Observes busy original completion retention without exposing producer authority.
    #[cfg(test)]
    pub(crate) async fn test_reserved_emit_busy_completion_owner(&mut self, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> bool {
        let permit=self.admit_reserved_emit_producer("cut",meta).expect("fixture cut admission");
        let id=permit.operation.operation.0;
        let completion=ArtifactToolCompletion::<A>::new();
        let mut emit=Emit::default();emit.artifact_mutations=Vec::with_capacity(3);
        completion.complete(Ok(emit),EphemeralEmit::default()).expect("fixture original completion");
        let original=completion.inner.clone();
        let guard=original.lock().expect("fixture original lock");
        let result=self.commit_framework_clipboard_completion("cut",Some(completion),meta,permit, cx).await;
        let retained=result.is_ok()&&self.tools.tool_operations.get(id).is_some_and(|owner|owner.completion.as_ref().is_some_and(|cell|cell.inner.same_cell(&original))&&owner.terminal_fault.is_some()&&owner.cancellation_lease.as_ref().is_some_and(|lease|lease.token.is_cancelled_now()));
        drop(guard);
        let mut original=semio_framework_value::retirement::controlled::ControlledRetirement::new(ArtifactToolCompletion::<A>{inner:original}).unwrap_or_else(|(_,original)|{let _original=std::mem::ManuallyDrop::new(original);panic!("original completion observation alias retirement was refused");});
        for _ in 0..100000 {
            if original.terminal_is_empty(){break;}
            let copy=original.next_copy_byte_demand().expect("original completion alias copy quote").max(4096);
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:original.next_capacity_byte_demand(copy).expect("original completion alias capacity quote"),maximum_release_bytes:original.next_release_byte_demand().expect("original completion alias release quote"),maximum_depth:original.next_depth_demand().expect("original completion alias depth quote")};
            assert!(original.step(grant).expect("funded original completion alias closure").progress().fits(grant));
        }
        assert!(original.terminal_is_empty(),"original completion observation alias remains in custody");
        if !retained {self.tools.release_reserved_emit_slot(id);}
        retained
    }

    /// 🔀️ Merges a decoded read-back into the live document (design §22.22) in the host's own poll turn and requests the
    /// operation's terminal: `Ready` with what it took (`completed == total`) and is ahead by, or the fault that
    /// refused it. Nothing is replaced.
    pub(super) async fn merge_document_archive(&mut self, operation: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        let merged = self.merge_decoded_document_archive(operation, identity, cx).await;
        let active = self.ingest.document_archive_loads.get_mut(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "recursive document archive merge authority changed during its merge"))?;
        match merged {
            Ok((taken, ahead)) => {
                active.completed = taken;
                active.total = taken;
                active.ahead = ahead;
                active.request_terminal(ActiveDocumentArchiveLoadState::Ready);
            }
            Err(fault) => active.request_fault(&fault),
        }
        Ok(())
    }

    /// 🧬️ The merge itself, answering the events taken and the events this program holds that the archive lacks: the
    /// archive's members must be the live ones (a member merge is the member's own, not yet available), its decoded
    /// log joins the store's in one store call — an archive of another document is refused — and an open history edit
    /// hears the base move like after any remote edit.
    async fn merge_decoded_document_archive(&mut self, operation: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(u64, u64), Fault> {
        let live = self.child_composition.archive_member_entries().await?;
        let active = self.ingest.document_archive_loads.get_mut(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "recursive document archive merge authority changed before its merge"))?;
        let archive = active.archive.as_mut().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive merge input owner is absent"))?;
        if !live.iter().eq(archive.members.iter().rev()) {
            return Err(document_load_fault(DOCUMENT_LOAD_MEMBERS_DIFFER_CODE, "the archive's members differ from this document's live members; a member merge is not available"));
        }
        let pack = std::mem::take(&mut archive.parent_pack);
        let history = active.decoded_history.take().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive decoded history owner is absent"))?;
        let generation = self.stores.store.generation();
        let merge = match self.stores.store.merge_persisted_history(&pack, history).await {
            Ok(merge) => merge,
            Err(store::VcsError::ValidationFailed(detail)) => return Err(document_load_fault(DOCUMENT_LOAD_OTHER_DOCUMENT_CODE, detail)),
            Err(error) => return Err(error.into_fault()),
        };
        self.history.cache = None;
        if self.stores.store.generation() != generation {
            self.deliver_base_moved(identity, cx).await?;
        }
        self.child_composition.follow_derivable_children(&mut self.stores, identity).await?;
        Ok((u64::try_from(merge.merged).unwrap_or(u64::MAX), u64::try_from(merge.ahead).unwrap_or(u64::MAX)))
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
    pub(super) fn record_typed_operation_lane(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>) {
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
        let kind = self.tools.typed_operation_command_kind(&self.host_events, &mounted.verb, true);
        let verb = mounted.verb.clone();
        let label = self.tools.verb_label(&verb);
        self.tools.record_command(&mut self.history, &verb, kind, label, Some(edit_id), Vec::new(), None);
    }

    /// 📨️ Ends the event's window's open gesture with the fact's reason (the runtime's ONE mapping, design §22.10), then
    /// delivers `event` to the app: its typed answer ([`ArtifactApp::host_event`]) dispatches to the event's window —
    /// addressed through `meta`'s view when it holds that window, else through the last roster — as an ordinary typed
    /// operation that is never a history row of its own. An event the app ignores, or for a window no view holds,
    /// dispatches nothing.
    async fn deliver_host_event(&mut self, event: HostEvent, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        self.end_window_gesture(&event);
        let Some(command) = A::host_event(&event) else { return Ok(()) };
        let window_id = event.window_id();
        let view = meta.view_state.as_ref().and_then(|view| view.for_window_instance(window_id)).or_else(|| ViewModel { window_instances: self.host_events.host_event_roster.clone(), ..meta.view_state.as_ref()?.clone() }.for_window_instance(window_id));
        let Some(view) = view else { return Ok(()) };
        let window_meta = ActionMeta { view_state: Some(view), ..meta.clone() };
        let verb = A::command_id(&command).await;
        let payload = ::protocol::OpBinary::encode_op(&command).map_err(|error| error.into_fault())?;
        let admission = self.tools.admit_command_wire(&self.stores, verb, &payload, 1).await?;
        self.host_events.delivering_host_event = true;
        let result = Box::pin(self.dispatch_typed_command_inner(Box::new(command), admission, &window_meta, identity, cx)).await;
        self.host_events.delivering_host_event = false;
        result.map(|_| ())
    }

    /// 🧰️ Delivers [`HostEvent::UtilityChanged`] for every window whose utility `meta`'s view moved since a dispatch
    /// last showed it — the targeted window reads the per-call overlay, every other one the view's map, and a window
    /// the view names no utility for is unknown, never a switch — and remembers the view's window roster.
    pub(super) async fn deliver_utility_host_events(&mut self, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        let Some(view) = meta.view_state.as_ref().filter(|_| !self.host_events.delivering_host_event) else { return Ok(()) };
        if !view.window_instances.is_empty() && self.host_events.host_event_roster != view.window_instances {
            self.host_events.host_event_roster = view.window_instances.clone();
            self.host_events.host_event_utilities.retain(|window_id, _| view.window_instances.iter().any(|window| window.id == *window_id));
        }
        let mut switched = Vec::new();
        for window in &view.window_instances {
            let utility = if view.window_id.as_deref() == Some(window.id.as_str()) { Some(view.active_utility_id.as_deref()) } else { view.active_utility_by_window_id.get(&window.id).map(|utility| Some(utility.as_str())) };
            let Some(utility) = utility else { continue };
            match self.host_events.host_event_utilities.get_mut(&window.id) {
                Some(previous) if previous.as_deref() == utility => {}
                Some(previous) => {
                    let from = std::mem::replace(previous, utility.map(str::to_string));
                    switched.push(HostEvent::UtilityChanged { window_id: window.id.clone(), from, to: utility.map(str::to_string) });
                }
                None => {
                    self.host_events.host_event_utilities.insert(window.id.clone(), utility.map(str::to_string));
                }
            }
        }
        for event in switched {
            self.deliver_host_event(event, meta, identity, cx).await?;
        }
        Ok(())
    }

    /// 🌐️ Delivers one instance-wide event, built per window, to every window of `meta`'s view, else of the last roster.
    pub(crate) async fn deliver_host_event_to_every_window(&mut self, event: impl Fn(String) -> HostEvent, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        self.end_every_gesture(&event(String::new()));
        let roster = meta.view_state.as_ref().map(|view| view.window_instances.as_slice()).filter(|roster| !roster.is_empty()).unwrap_or(&self.host_events.host_event_roster);
        let windows: Vec<String> = roster.iter().map(|window| window.id.clone()).collect();
        for window_id in windows {
            self.deliver_host_event(event(window_id), meta, identity, cx).await?;
        }
        Ok(())
    }

    /// 🌐️ Delivers [`HostEvent::BaseMoved`] to every window once a remote edit moved the base, as the local actor.
    pub(crate) async fn deliver_base_moved(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        let Some(instance_id) = self.tools.live_runtime_instance_id else { return Ok(()) };
        let meta = ActionMeta { actor: self.stores.store.local_actor_id().0.clone(), instance_id, view_state: None };
        self.deliver_host_event_to_every_window(|window_id| HostEvent::BaseMoved { window_id }, &meta, identity, cx).await
    }

    /// 📨️ `hostEvent{windowId, kind}`: the host-forwarded window fact, delivered at once; an unknown kind or a
    /// missing window is refused.
    pub(super) async fn dispatch_host_event_action(&mut self, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let text = |key: &str| args.and_then(|value| value.get(key)).and_then(DslValue::as_str);
        let event = text(semio_framework::HOST_EVENT_ARG_WINDOW_ID).zip(text(semio_framework::HOST_EVENT_ARG_KIND)).and_then(|(window_id, kind)| HostEvent::forwarded(window_id, kind));
        let Some(event) = event else {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("hostEvent.invalid"), "hostEvent requires a windowId and a kind of blur, captureLost or retiring"));
        };
        self.deliver_host_event(event, meta, identity, cx).await?;
        let ui_scope = if self.take_gesture_ended() { UiDirtyScope::Full } else { UiDirtyScope::None };
        Ok(ToolOperations::<A>::empty_result(semio_framework::HOST_EVENT_ACTION_ID, meta, Vec::new(), Vec::new(), ui_scope).await)
    }

    #[inline(never)]
    pub(crate) fn retire_typed_operation_unit(&mut self,operation_id:u64,cx:&mut semio_framework_job::StepContext<'_>)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{
        let grant=cx.retained_grant();let demand=self.tools.typed_operation_retirement_demands(&self.child_composition, operation_id, grant.maximum_copy_bytes)?;
        semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,actual|self.retire_typed_operation_granted(operation_id,same,actual).map(|step|(step,false)))
    }

    fn retire_typed_operation_granted(&mut self,operation_id:u64,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{
        self.request_gesture_cancellation(operation_id);
        let item = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        if self.tools.tool_operations.get(operation_id).is_some_and(MountedTypedCommandFullOperation::close_fault_pending){return self.tools.tool_operations.get_mut(operation_id).unwrap().close_fault_step(grant).map(|step|PluginLifecycleStep::Progress(step.progress())).map_err(Into::into);}
        if let Some(group) = self.tools.tool_operations.get(operation_id).and_then(|operation| operation.owned_child_group) {
            if self.child_composition.private_child_groups.get(group).is_some() {
                return Ok(match self.child_composition.close_private_child_group_operation_step(&mut self.stores, group, cx, grant)? {
                    semio_framework_job::InteractiveJobCloseStep::Pending { progress } | semio_framework_job::InteractiveJobCloseStep::Complete { progress } => PluginLifecycleStep::Progress(progress),
                    semio_framework_job::InteractiveJobCloseStep::Blocked => PluginLifecycleStep::Blocked { reason: "private child group close awaits its owner" },
                });
            }
            if grant.maximum_items == 0 { return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress::default())); }
            self.tools.tool_operations.get_mut(operation_id).unwrap().owned_child_group = None;
            return Ok(PluginLifecycleStep::Progress(item));
        }
        if self.tools.tool_operations.get(operation_id).is_some_and(MountedTypedCommandFullOperation::lease_retirement_ready){let step=self.tools.tool_operations.get_mut(operation_id).unwrap().lease_retirement_step(&self.tools.tool_cancellations,grant).map_err(plugin_retirement_fault)?;return Ok(PluginLifecycleStep::Progress(step.progress()));}
        self.tools.record_settled_typed_operation_command(&mut self.history, &mut self.host_events, operation_id);
        let operation = self
            .tools.tool_operations
            .get_mut(operation_id)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-retirement-authority"), "typed operation owner changed during one bounded retirement unit"))?;
        let step = operation.retirement_step(cx)?;
        let PluginLifecycleStep::Complete(progress) = step else { return Ok(step) };
        if !self.tools.tool_operations.get(operation_id).is_some_and(MountedTypedCommandFullOperation::terminal_is_empty) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-terminal"), "typed operation reported Complete without exact terminal emptiness").into());
        }
        let operation =
            self.tools.tool_operations.remove(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-retirement-authority"), "terminal typed operation changed before exact removal"))?;
        self.tools.operation_progress_retired |= operation.progress.is_some();
        drop(operation);
        self.tools.typed_inline_interaction_verbs.retain(|(operation, _)| *operation != operation_id);
        Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress }))
    }

    /// ♻️ Drives ONE retiring typed operation for a bounded slice of this turn, the exact twin of the
    /// slice [`Self::drive_typed_operation_worker`] gives a Worker-stage operation — same ladder, same
    /// owner, same `INTERACTIVE_TURN_WORKER_PUMPS`/`INTERACTIVE_TURN_WORKER_WALL_US` bound. Without it a
    /// `Retiring` operation answered `has_runnable_typed_operations` forever while the turn driver advanced nothing
    /// for it: the slot was released only by `maintenance_step` stage 0 of [`MAINTENANCE_STAGES`], one
    /// unit per rotation on the shared live-cleanup job, so a sustained action rate filled all
    /// [`ARTIFACT_LIVE_OUTPUT_SLOTS`] classes and every later action was refused
    /// `interactive-job.typed-operation-capacity`.
    pub(super) fn retire_typed_operation_run(&mut self,operation_id:u64,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),crate::PluginLifecycleFailure>{
        let started_us=cx.now_us();
        for _ in 0..INTERACTIVE_TURN_WORKER_PUMPS {
            if !self.tools.tool_operations.get(operation_id).is_some_and(|operation| operation.stage == MountedTypedCommandFullOperationStage::Retiring) {
                return Ok(());
            }
            if self.tools.tool_operations.get(operation_id).is_some_and(|operation|operation.granted_retirement_ready()&&operation.granted_retirement_pending()){
                let grant=crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.close,cx.retained_grant());
                let demand=self.tools.tool_operations.get(operation_id).unwrap().granted_retirement_demands(grant.maximum_copy_bytes)?;let step=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,actual|{let step=self.tools.tool_operations.get_mut(operation_id).unwrap().granted_retirement_step(same,actual)?;Ok((PluginLifecycleStep::Progress(step.progress()),false))})?;let step=RetainedCloneStep::Progress(step.progress().unwrap_or_default());
                if !step.progress().fits(grant){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original typed operation retirement exceeded its issuer grant").with_retained_progress(step.progress()).into());}
                let progress=step.progress();if progress==RetainedCloneProgress::default(){return Ok(());}
                let owner=self.tools.tool_operations.get_mut(operation_id).unwrap();assert!(owner.original_retirement_receipt.is_none(),"original mounted Close receipt cannot be overwritten before acknowledgement");owner.original_retirement_receipt=Some((grant,progress));return Ok(());

            }
            let grant = crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.close, cx.retained_grant());
            let demand = self.tools.typed_operation_retirement_demands(&self.child_composition, operation_id, grant.maximum_copy_bytes)?;
            if !plugin_grant_funds(demand, grant) {
                return Ok(());
            }
            match cx.with_retained_loan(grant,|same|self.retire_typed_operation_unit(operation_id,same).map_err(Into::into))? {
                PluginLifecycleStep::Blocked { .. } | PluginLifecycleStep::AwaitingInput { .. } => return Ok(()),
                PluginLifecycleStep::Progress(progress) | PluginLifecycleStep::Complete(progress) => {
                    if !progress.fits(grant) {
                        return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"typed operation retirement exceeded its issuer grant").with_retained_progress(progress).into());
                    }
                    if progress == RetainedCloneProgress::default() {
                        return Ok(());
                    }
                }
            }
            if cx.should_yield()||started_us.is_some_and(|started_us|cx.now_us().is_some_and(|now_us|now_us.saturating_sub(started_us)>=INTERACTIVE_TURN_WORKER_WALL_US)) {
                return Ok(());
            }
        }
        Ok(())
    }

    /// 🎟️ Advances one child-root retirement under the caller's unchanged grant.
    pub(super) fn child_root_retirement_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, Fault> {
        self.child_root_retirement_unit(grant, self.close.close_child_root_detached).map(|step| step.progress())
    }
}
