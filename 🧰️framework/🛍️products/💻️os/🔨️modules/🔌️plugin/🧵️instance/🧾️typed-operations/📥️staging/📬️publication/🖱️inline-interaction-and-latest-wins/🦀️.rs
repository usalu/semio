//! 🖱️ Inline interaction and latest wins of the typed operation publication: the inline interaction unit and the latest wins command advance. See [`super`].

use super::*;

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> TypedCtx<'ctx, A, M, X> {
    /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — the typed-operation twin of
    /// `dispatch_emit`'s inline interaction fold, for the ladder every app command
    /// (`dispatch_typed_command_inner`) and retained tool publishes through and that never enters
    /// `dispatch_emit`. Peels every interaction verb out of the retained emit's effect list as ONE
    /// unit (the same one-transition-per-domain reasoning as the `InteractionWrite` unit above),
    /// applies each through `apply_interaction_verb`, and widens the emit's `ui_scope` with the
    /// verbs' app-declared refresh scopes so the `Ui` page and the completion witness — published
    /// later in this same ladder — already carry them. No result page is queued: the host has
    /// nothing to acknowledge, because the verb it used to be asked to replay never leaves the
    /// reactor, and the ladder simply continues with the remaining effects next pump.
    ///
    /// 🩹 A verb that could not be applied is reported on the runtime's `[TRACE]` diagnostics line
    /// and dropped — the typed ladder has no `InvocationResult.diagnostics` lane (its only fault
    /// lane is terminal), and failing the carrying command for a pick that already happened is
    /// exactly what `fold_inline_interaction_verbs` exists to prevent.
    pub(super) async fn publish_mounted_typed_inline_interaction_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() else {
            return Err(plugin_sdk_fault("typed-operation inline interaction publication lost its emit owner"));
        };
        let mut folded: Vec<(String, Option<DslValue>)> = Vec::new();
        if let Some(index) = self.tools.typed_inline_interaction_verbs.iter().position(|(operation, _)| *operation == mounted.operation.operation.0) {
            folded = self.tools.typed_inline_interaction_verbs.swap_remove(index).1;
        }
        folded.extend(take_inline_interaction_verbs(&mut emit.effects));
        let scope = emit.ui_scope.clone();
        let meta = mounted.meta.clone();
        let fold = self.as_render().fold_inline_interaction_verbs(&mounted.verb, folded, &meta, scope, identity).await;
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() else {
            return Err(plugin_sdk_fault("typed-operation inline interaction publication lost its emit owner after the fold"));
        };
        emit.ui_scope = fold.scope;
        if let Some(view) = fold.leftover {
            self.tools.typed_inline_interaction_leftover = Some(TypedOperationLeftover { operation: mounted.operation.operation.0, view });
        }
        for diagnostic in fold.diagnostics {
            semio_framework_plugin_reactor_app::debug_runtime_line(format_args!("[TRACE] {} {}", diagnostic.code.0, diagnostic.message));
        }
        Ok(())
    }

    pub async fn advance_latest_wins_command_one(&mut self,retained:RetainedCloneGrant, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        if retained.maximum_items==0||retained.maximum_depth==0{return Ok(());}
        let Some(operation_id) = self.tools.latest_wins_order.items.front().copied() else { return Ok(()) };
        let mut pending = self.tools.latest_wins_commands.remove(operation_id).expect("exact pending keyed command");
        if pending.receipt.is_some(){self.tools.latest_wins_commands.insert_admitted(operation_id,pending);return Ok(());}
        if !pending.closing&&pending.lease.is_none(){
            let result=pending.admit_original_lease(&mut self.tools.tool_cancellations,cx);
            self.tools.latest_wins_commands.insert_admitted(operation_id,pending);
            result?;
            return Ok(());
        }
        let ready = match self.tools.advance_latest_wins_admission_unit(&mut *self.stores, &mut pending, retained) {
            Ok(ready) => ready,
            Err(fault) => {
                let result=if pending.receipt.is_some(){Ok(())}else{pending.record_original_receipt(retained,fault.retained_progress())};
                *pending.fault=Some(fault);
                if let Err(error)=result{self.tools.latest_wins_commands.insert_admitted(operation_id,pending);return Err(error.into());}
                pending.closing = true;
                if let Some(lease) = pending.lease.as_ref() {
                    lease.cancel();
                }
                false
            }
        };
        let reserved = self.tools.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] == Some(operation_id);
        if !reserved || !self.tools.tool_operations.can_insert(operation_id) {
            pending.closing = true;
            pending.lease.as_ref().expect("pending slot rejection retains its cancellation lease").cancel();
            if !self.tools.tool_operations.can_insert(operation_id) || self.tools.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS].is_some_and(|owner| owner != operation_id) {
                self.tools.latest_wins_commands.insert_admitted(operation_id, pending);
                return Ok(());
            }
        }
        if ready && !pending.closing {
            let command = pending.command.take().expect("accepted latest-wins command owner");
            let admission = pending.admission.take().expect("accepted latest-wins exact raw admission");
            let lease = pending.lease.take().expect("accepted latest-wins cancellation lease");
            assert_eq!(self.tools.latest_wins_order.pop(), Some(operation_id));
            let started = self.as_dispatch().start_typed_command_operation(command, admission, &pending.meta, pending.operation.operation, lease, cx).await;
            if started.is_err() {
                self.tools.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
            }
            started?;
            return Ok(());
        }
        if pending.closing && pending.command_owners_are_empty() {
            let mut mounted = MountedTypedCommandFullOperation::<A> {
                verb: "latest-wins-admission".into(),
                meta: pending.meta.clone(),
                operation: pending.operation,
                source_identity:pending.source_identity,
                canonical_revision: pending.revision,
                artifact_generation: pending.operation.generation.0,
                config_generation: 0,
                draft_generation: 0,
                presence_generation: 0,
                transient_generation: 0,
                window_config_authority: None,
                window_transient_authority: None,
                publication_lanes: &[], contract: Some(semio_framework::ToolExecutionContract::bounded_first_step(1 << 20, 1 << 20, 1 << 20, 1 << 20, 7_999)),
                session: None,
                session_rejected: None,
                reserved_producer: None,
                completion: None,
                completion_retirement:None,
                publication_retirement:None,
                output_retirement:None,raw_input: None,
                output_chunks: None,
                cancellation_lease: pending.lease.take(),
                identity: None,
                identity_progress: semio_framework_value::native_encoding::NativeEncodeProgress { completed: 0, total: 0, owned_bytes: 0 },
                worker_semantic_pending: false,
                worker_outcome_pending: false,
                worker_fault_capture: None,
                terminal_seen: true,
                worker_resume_pending: false,
                publication: None,
                pending_artifact_publication: None, pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,
                pending_child_publication: None,
                owned_child_group: None,
                owned_child_committed: false,
                owned_child_result_pending: false,
                captured_child_content: Some(std::sync::Arc::new(ChildContentView::EMPTY)),
                captured_child_content_generation: 0,
                result_page: None,
                result_page_presented: false,
                result_sequence: 0,
                publication_progress: 0,
                publication_checkpoint: None,
                publication_ownership_progress: None,
                original_retirement_receipt:None,
                actor_capture: None,
                publication_attempt: 0,
                ui_pending: false,
                progress: None,
                progress_pending: false,
                user_cancel_requested: false,
                published_artifact: false,
                published_config: false,
                published_window_config: false,
                command_logged: false,
                interaction_revalidated: false,
                terminal_fault: None,
                retained_close_fault: None, retained_close_fault_retirement:None,retained_close_fault_refusal:None,
                stage: MountedTypedCommandFullOperationStage::Publishing,
            };
            mounted.queue_page(TypedOperationResultPage::try_new(mounted.next_token(), TypedOperationResultLane::Fault, b"latest-wins command admission cancelled or rejected before worker publication")?)?;
            self.tools.tool_operations.insert_admitted(operation_id, mounted);
            self.tools.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
            assert_eq!(self.tools.latest_wins_order.pop(), Some(operation_id));
        } else {
            self.tools.latest_wins_commands.insert_admitted(operation_id, pending);
        }
        Ok(())
    }

}
