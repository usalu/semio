//! 📬️ Typed operation unit of the typed operation publication: the publication of one mounted typed operation with its store, history, receipt and result page steps. See [`super`].

use super::*;

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> TypedCtx<'ctx, A, M, X> {
    pub fn publish_mounted_typed_operation_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, retained:RetainedCloneGrant, receiving: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<(), Fault> {
        if mounted.reject_cancelled_publication()? {
            self.stores.handoff_mounted_durable_publication(mounted)?;
            return Ok(());
        }
        let _permit = mounted.cancellation_lease.as_ref().and_then(ToolCancellationLease::try_claim_publication).ok_or_else(|| plugin_sdk_fault("typed-operation publication claim is cancelled or occupied"))?;
        if mounted.publication.is_none() {
            let Some(completion) = mounted.completion.as_ref() else { return Err(plugin_sdk_fault("typed-operation completion owner was retired before publication")) };
            let Some(mut publication) = completion.take()? else { return Ok(()) };
            mounted.ui_pending = matches!(publication, ArtifactToolCompletionValue::Emit(Ok(_), _));
            self.as_render().settle_tool_operation(mounted, &mut publication)?;
            if let ArtifactToolCompletionValue::Emit(Ok(emit), _) = &mut publication {
                let verbs = take_inline_interaction_verbs(&mut emit.effects);
                if !verbs.is_empty() {
                    self.tools.typed_inline_interaction_verbs.retain(|(operation, _)| *operation != mounted.operation.operation.0);
                    self.tools.typed_inline_interaction_verbs.push((mounted.operation.operation.0, verbs));
                }
            }
            mounted.publication = Some(publication);
            if mounted_typed_interaction_writes_are_next::<A>(mounted) || self.tools.mounted_typed_inline_interaction_verbs_are_next(mounted) {
                return Ok(());
            }
        }
        let live_revision = self.stores.store.content_revision_now();
        let live_generation = self.stores.store.generation_now();
        if !typed_operation_document_is_fresh(&mounted.operation, mounted.canonical_revision, live_revision, live_generation) {
            if self.stores.handoff_mounted_durable_publication(mounted)? {
                return Err(plugin_sdk_fault("typed-operation pending publication rejected a stale immutable document root"));
            }
            if let Some(pending)=mounted.pending_artifact_publication.as_mut(){pending.begin_close();}
            return Err(plugin_sdk_fault("typed-operation pending publication rejected a stale immutable document root"));
        }
        if mounted.pending_artifact_publication.is_none() && matches!(mounted.publication.as_ref(),Some(ArtifactToolCompletionValue::Emit(Ok(emit),_))if !emit.artifact_mutations.is_empty()||!emit.config_mutations.is_empty()||!emit.draft_mutations.is_empty()||!emit.window_config_mutations.is_empty()) && mounted.admit_actor_capture(retained)? {return Ok(());}
        if let Some(pending) = mounted.pending_artifact_publication.as_mut() {
            let grant=store::os_store::one_item::ArtifactStoreOneItemGrant::from_retained(retained);
            let (pending_lane, _) = pending.lane();
            if pending.is_closing(){return Ok(());}
            if mounted.pending_window_config_receipt.is_some() { return self.advance_mounted_window_config_refresh(mounted, retained); }
            let pending = mounted.pending_artifact_publication.as_mut().expect("pending publication checked above");
            if matches!(pending, PendingArtifactStorePublication::Artifact(_))
                && self.stores.store.returned_snapshot_read_count() > PUBLICATION_RETURNED_ROOT_ALLOWANCE
                && !self.ingest.reclaim_document_snapshot_read_returns(&*self.stores)?
            {
                return Ok(());
            }
            let pending = mounted.pending_artifact_publication.as_mut().expect("pending publication checked above");
            let advance = match pending {
                PendingArtifactStorePublication::Artifact(publication) => self.stores.store.advance_apply_batch(publication, grant.maximum_copy_bytes, grant, receiving).map_err(store_publication_fault)?,
                PendingArtifactStorePublication::Config(publication) => self.stores.config_store.advance_apply_batch(publication, grant.maximum_copy_bytes, grant, receiving).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::Draft(publication) => self.stores.draft_store.advance_apply_batch(publication, grant.maximum_copy_bytes, grant, receiving).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::Presence(publication) => self.stores.presence_store.advance_publish_one(publication, grant, receiving).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::Transient(publication) => self.stores.transient_store.advance_publish_one(publication, grant, receiving).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::WindowConfig(publication) => self.stores.window_config_store.advance(publication.as_mut(), grant)?,
                PendingArtifactStorePublication::WindowTransient(publication) => self.stores.window_transient_store.advance(publication.as_mut(), grant)?,
            };
            return match advance {
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Published(receipt, ownership) => {
                    if ownership!=RetainedCloneProgress::default(){
                        assert!(mounted.publication_ownership_progress.is_none(),"typed publication cannot overwrite an unreturned original receipt");
                        mounted.publication_ownership_progress=Some((retained,ownership));
                    }
                    match pending_lane {
                        TypedOperationResultLane::Artifact => {
                            mounted.published_artifact = true;
                            self.as_dispatch().record_typed_operation_lane(mounted);
                            mounted.artifact_generation = receipt.generation_after;
                            mounted.canonical_revision = self.stores.store.content_revision_now();
                            mounted.operation.base_revision = semio_framework_job::RevisionId(u64::from_be_bytes(mounted.canonical_revision[..8].try_into().expect("revision lane width")));
                            mounted.operation.generation = semio_framework_job::Generation(receipt.generation_after);
                        }
                        TypedOperationResultLane::Config => {
                            mounted.published_config = true;
                            mounted.config_generation = receipt.generation_after;
                        }
                        TypedOperationResultLane::Draft => mounted.draft_generation = receipt.generation_after,
                        TypedOperationResultLane::Presence => mounted.presence_generation = receipt.generation_after,
                        TypedOperationResultLane::Transient => mounted.transient_generation = receipt.generation_after,
                        TypedOperationResultLane::WindowConfig => {
                            mounted.published_window_config = true;
                            mounted.pending_window_config_receipt = Some(receipt);
                            return self.advance_mounted_window_config_refresh(mounted, retained);
                        }
                        TypedOperationResultLane::WindowTransient => {
                            let authority = mounted.window_transient_authority.as_mut().ok_or_else(|| plugin_sdk_fault("window transient receipt lost its captured window authority"))?;
                            authority.generation = receipt.generation_after;
                        }
                        _ => unreachable!("pending artifact publication has one durable store lane"),
                    }
                    let token = mounted.next_token();
                    mounted.queue_page(TypedOperationResultPage::try_serialize_cold(token, pending_lane, &receipt)?)
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Progress(checkpoint) => {
                    mounted.note_publication_checkpoint(checkpoint);
                    Ok(())
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::PreparationProgress(checkpoint, ownership) => {
                    if !ownership.fits(grant.retained_grant())||grant.maximum_depth==0&&ownership!=RetainedCloneProgress::default(){return Err(plugin_sdk_fault("typed preparation violated its original full grant"));}
                    assert!(mounted.publication_ownership_progress.is_none(),"typed preparation cannot overwrite an unreturned original receipt");
                    mounted.publication_ownership_progress=Some((retained,ownership));
                    mounted.note_publication_checkpoint(checkpoint);
                    Ok(())
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::PreparationRefused => {
                    let reason=pending.preparation_refusal().ok_or_else(||plugin_sdk_fault("typed preparation refused without its original retained failure"))?;
                    let ownership=reason.retained_progress();
                    assert!(mounted.publication_ownership_progress.is_none(),"typed refusal cannot overwrite its unreturned actual receipt");
                    mounted.publication_ownership_progress=Some((retained,ownership));
                    Ok(())
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Blocked => Ok(()),
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::AwaitingAck(_) => Err(plugin_sdk_fault("artifact-store publication awaits an ACK without its retained host page")),
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Complete => Err(plugin_sdk_fault("artifact-store publication completed before result ACK")),
            };
        }
        let token = mounted.next_token();
        let publication_lanes = mounted.publication_lanes;
        let page = match mounted.publication.as_mut().expect("publication owner was installed") {
            ArtifactToolCompletionValue::Emit(Err(fault), _) | ArtifactToolCompletionValue::Download(Err(fault), _) => {
                let mut framed = [0; TYPED_OPERATION_FAULT_PAGE_BYTES];
                let len = fault.framed_page_bytes(&mut framed);
                TypedOperationResultPage::try_new(token, TypedOperationResultLane::Fault, &framed[..len])?
            }
            ArtifactToolCompletionValue::Download(Ok(download), ephemeral) => {
                if (!ephemeral.presence.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Presence))
                    || (!ephemeral.transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Transient))
                    || (!ephemeral.window_transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::WindowTransient))
                {
                    return Err(plugin_sdk_fault("typed-operation emitted an undeclared ephemeral publication lane"));
                }
                if let Some(mutation) = ephemeral.presence.pop() {
                    match self.stores.presence_store.begin_publish_one(mounted.operation.operation, mounted.presence_generation, mutation, self.stores.presence_one_item_factory.as_deref(), self.stores.presence_local_root_retirement_factory.clone()) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Presence(publication));
                            return Ok(());
                        }
                        Err(rejected) => {
                            let (reason, mutation) = rejected.into_owners();
                            ephemeral.presence.push(mutation);
                            return Err(plugin_sdk_fault(reason));
                        }
                    }
                } else if let Some(mutation) = ephemeral.transient.pop() {
                    match self.stores.transient_store.begin_publish_one(mounted.operation.operation, mounted.transient_generation, mutation, self.stores.transient_one_item_factory.as_deref(), self.stores.transient_local_root_retirement_factory.clone()) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Transient(publication));
                            return Ok(());
                        }
                        Err(rejected) => {
                            let (reason, mutation) = rejected.into_owners();
                            ephemeral.transient.push(mutation);
                            return Err(plugin_sdk_fault(reason));
                        }
                    }
                } else if let Some(mutation) = ephemeral.window_transient.pop() {
                    let authority = match mounted.window_transient_authority.as_mut() {
                        Some(authority) => authority,
                        None => {
                            ephemeral.window_transient.push(mutation);
                            return Err(plugin_sdk_fault("window transient emission requires one exact captured ViewModel window authority"));
                        }
                    };
                    let grant=self.mounted_policy.maintenance;
                    let refresh=match self.stores.window_transient_store.refresh_demands(authority,grant.maximum_copy_bytes){Ok(_)=>self.stores.window_transient_store.refresh(authority,grant),Err(error)=>Err(plugin_retirement_fault(error))};
                    match refresh{Ok(semio_framework_plugin_window_config::window_mutation::refresh::WindowAuthorityRefreshStep::Ready(_))=>{},Ok(semio_framework_plugin_window_config::window_mutation::refresh::WindowAuthorityRefreshStep::Pending(_))=>{ephemeral.window_transient.push(mutation);return Ok(());},Err(fault)=>{ephemeral.window_transient.push(mutation);return Err(fault);}}
                    match self.stores.window_transient_store.begin(mounted.operation.operation, authority, mutation) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::WindowTransient(publication));
                            return Ok(());
                        }
                        Err(semio_framework_plugin_window_transient::window_transient::RejectedWindowTransientEmission { mutation, fault }) => {
                            ephemeral.window_transient.push(mutation);
                            return Err(fault);
                        }
                    }
                } else {
                    TypedOperationResultPage::try_serialize_cold(
                        token,
                        TypedOperationResultLane::Download,
                        &DownloadResultPayload { filename: download.filename.as_str(), mime_type: download.mime_type.as_str(), encoding: download.encoding.as_deref(), bytes: download.chunks.bytes() },
                    )?
                }
            }
            ArtifactToolCompletionValue::Emit(Ok(emit), ephemeral) => {
                let preparation=match emit.prepare_child_one(retained){Ok(step)=>step,Err(fault)=>{
                    let progress=fault.retained_progress();if !progress.fits(retained){return Err(plugin_sdk_fault("original child preparation error exceeded mounted caller policy"));}
                    if progress!=RetainedCloneProgress::default(){assert!(mounted.publication_ownership_progress.is_none(),"child preparation error cannot overwrite an original receipt");mounted.publication_ownership_progress=Some((retained,progress));}return Err(fault);
                }};
                let progress=preparation.progress();
                if !progress.fits(retained){return Err(plugin_sdk_fault("original emitted child preparation exceeded mounted caller policy"));}
                if progress!=RetainedCloneProgress::default(){
                    assert!(mounted.publication_ownership_progress.is_none(),"child preparation cannot overwrite an original receipt");mounted.publication_ownership_progress=Some((retained,progress));return Ok(());
                }
                match preparation{ChildEmitPreparationStep::Ready(_)=>{},ChildEmitPreparationStep::Pending(_)=>return Ok(()),ChildEmitPreparationStep::Refused(fault,_)=>return Err(fault)}
                if (!emit.artifact_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Artifact))
                    || (!emit.config_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Config))
                    || (!emit.window_config_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::WindowConfig))
                    || (!emit.draft_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Draft))
                    || (!ephemeral.presence.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Presence))
                    || (!ephemeral.transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Transient))
                    || (!ephemeral.window_transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::WindowTransient))
                    || ((!emit.child_emits.is_empty() || !emit.owned_child_emits.is_empty()) && !publication_lanes.contains(&ArtifactToolPublicationLane::Child))
                    || (!emit.interaction_writes.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Interaction))
                {
                    return Err(plugin_sdk_fault("typed-operation emitted a store lane absent from its exact factory publication contract"));
                }
                if !emit.artifact_mutations.is_empty() || !emit.owned_child_emits.is_empty() {
                    self.media.require_operation_emitting_kind(&mounted.verb, self.tools.declared_dispatch_kind(&*self.host_events, &mounted.verb))?;
                    if A::ROLE == AppRole::Viewer {
                        return Err(viewer_read_only_fault(&mounted.verb));
                    }
                    if let Some(fault) = X::mutation_refusal(&*self.time_travel, &*self.tool_runs, &mounted.verb) {
                        return Err(fault);
                    }
                    if let Some(pending_txn_id) = self.txn.pending_transaction.as_ref().map(|pending| pending.txn_id.clone()) {
                        return Err(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.instance-busy", format!("verb {:?} would emit artifact mutations while transaction {pending_txn_id:?} is pending on this instance", mounted.verb)));
                    }
                    if emit.artifact_mutations.iter().any(Mutation::may_emit_foreign_steps) {
                        let mut running = self.stores.store.snapshot().map_err(|error| error.into_fault())?;
                        let foreign = Vec::new();
                        for op in emit.artifact_mutations.iter() {
                            if ColdMutation::foreign_step_source(op, &running, 0).map_err(|error| error.into_fault())?.is_some() { return Err(plugin_sdk_fault("foreign transaction output awaits its original staged preparation recipient")); }
                            let outcome = ColdMutation::diff(op, &running);
                            if !outcome.is_applicable(protocol::MergePolicy::default()) {
                                return Err(MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", format!("mutation outcome was rejected: {:?}", outcome.messages())));
                            }
                            running = protocol::apply_diff_cold(outcome.diff(), &running).map_err(|error| MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string()))?;
                        }
                        if !foreign.is_empty() {
                            let mut local_ops = Vec::with_capacity(emit.artifact_mutations.len());
                            for op in emit.artifact_mutations.iter() {
                                local_ops.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
                            }
                            self.txn.pending_transaction_proposal = Some(TransactionProposalDraft { local_ops, foreign });
                            emit.artifact_mutations.clear();
                            return Ok(());
                        }
                    }
                }
                if let Some(fault) = tool_transaction_shape_fault(
                    &mounted.verb,
                    emit.transaction.as_ref(),
                    emit.transaction_phase,
                    !emit.child_emits.is_empty() || !emit.owned_child_emits.is_empty(),
                    !emit.artifact_mutations.is_empty(),
                    emit.artifact_mutations.iter().any(Mutation::may_emit_foreign_steps),
                ) {
                    return Err(fault);
                }
                if !emit.artifact_mutations.is_empty() || !emit.child_emits.is_empty() || !emit.owned_child_emits.is_empty() {
                    if let Some(fault) = X::document_refusal(&*self.time_travel, &*self.tool_runs, &mounted.verb) {
                        return Err(fault);
                    }
                }
                self.history.close_streamed_transaction_unit(&mut *self.stores, emit)?;
                if !emit.owned_child_emits.is_empty() {
                    if !emit.child_emits.is_empty() || emit.owned_child_emits.len() > PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN { return Err(plugin_sdk_fault("mounted owned child group requires its one exact admitted typed source family")); }
                    let operation = mounted.operation.operation.0;
                    let captured_millis = i64::try_from(store::os_store::document_materialize::now_ms()).map_err(|_|plugin_sdk_fault("mounted child publication requires its original UTC clock capture"))?;
                    let grant=retained;
                    let step=self.child_composition.admit_private_child_group_frame(operation, captured_millis, u64::MAX, grant)?;
                    let progress=step.progress();
                    if !progress.fits(grant){return Err(plugin_sdk_fault("mounted child frame exceeded its original preparation grant"));}
                    if progress!=RetainedCloneProgress::default(){
                        assert!(mounted.publication_ownership_progress.is_none(),"mounted child frame cannot overwrite its original unreturned receipt");
                        mounted.publication_ownership_progress=Some((retained,progress));
                    }
                    if self.child_composition.private_child_groups.get(operation).is_some(){mounted.owned_child_group=Some(operation);}
                    return Ok(());
                }
                if emit.child_emits.is_empty() && emit.owned_child_emits.is_empty() && !emit.artifact_mutations.is_empty() {
                    if let Some(fault) = self.stores.open_transaction_admission_fault(emit.transaction.as_ref()) {
                        return Err(fault);
                    }
                    if let Err(error) = store::os_store::one_item::ArtifactStoreOneItemFootprint::for_gesture::<A::Snapshot, A::Mutation>(&emit.artifact_mutations) {
                        return Err(error.into_fault());
                    }
                    let streams=emit.transaction_phase==TransactionPhase::Stream;
                    let outbound=self.stores.store.backbone_ref().is_some();
                    if let Some((mut publication,ownership))=self.stores.store.admit_apply_batch(mounted.operation.operation,mounted.artifact_generation,mounted.canonical_revision,&mut mounted.actor_capture,&mut emit.artifact_mutations,HistoryLane::Document,self.stores.artifact_one_item_factory.as_ref(),&mut emit.transaction,Some(&mounted.verb),outbound,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {
                        publication.set_transaction_open(streams);
                        emit.transaction_phase=TransactionPhase::Commit;
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::Artifact(publication));
                    }
                    return Ok(());
                }
                if !emit.config_mutations.is_empty() {
                    let revision=self.stores.config_store.content_revision_now();
                    let mut transaction=None;
                    if let Some((publication,ownership))=self.stores.config_store.admit_apply_batch(mounted.operation.operation,mounted.config_generation,revision,&mut mounted.actor_capture,&mut emit.config_mutations,HistoryLane::Document,self.stores.config_one_item_factory.as_ref(),&mut transaction,Some(&mounted.verb),false,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::Config(publication));
                    }
                    return Ok(());
                } else if !emit.window_config_mutations.is_empty() {
                    let authority = mounted.window_config_authority.as_ref().ok_or_else(|| plugin_sdk_fault("window config emission requires one exact captured ViewModel window authority"))?;
                    if let Some((publication,ownership))=self.stores.window_config_store.admit_begin(mounted.operation.operation,&mut mounted.actor_capture,&mut emit.window_config_mutations,authority,retained).map_err(|error|error.into_fault())? {
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::WindowConfig(publication));
                    }
                    return Ok(());
                } else if !emit.draft_mutations.is_empty() {
                    let revision=self.stores.draft_store.content_revision_now();
                    let mut transaction=None;
                    if let Some((publication,ownership))=self.stores.draft_store.admit_apply_batch(mounted.operation.operation,mounted.draft_generation,revision,&mut mounted.actor_capture,&mut emit.draft_mutations,HistoryLane::Document,self.stores.draft_one_item_factory.as_ref(),&mut transaction,Some(&mounted.verb),false,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::Draft(publication));
                    }
                    return Ok(());
                } else if let Some(mutation) = ephemeral.presence.pop() {
                    match self.stores.presence_store.begin_publish_one(mounted.operation.operation, mounted.presence_generation, mutation, self.stores.presence_one_item_factory.as_deref(), self.stores.presence_local_root_retirement_factory.clone()) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Presence(publication));
                            return Ok(());
                        }
                        Err(rejected) => {
                            let (reason, mutation) = rejected.into_owners();
                            ephemeral.presence.push(mutation);
                            return Err(plugin_sdk_fault(reason));
                        }
                    }
                } else if let Some(mutation) = ephemeral.transient.pop() {
                    match self.stores.transient_store.begin_publish_one(mounted.operation.operation, mounted.transient_generation, mutation, self.stores.transient_one_item_factory.as_deref(), self.stores.transient_local_root_retirement_factory.clone()) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Transient(publication));
                            return Ok(());
                        }
                        Err(rejected) => {
                            let (reason, mutation) = rejected.into_owners();
                            ephemeral.transient.push(mutation);
                            return Err(plugin_sdk_fault(reason));
                        }
                    }
                } else if let Some(mutation) = ephemeral.window_transient.pop() {
                    let authority = match mounted.window_transient_authority.as_mut() {
                        Some(authority) => authority,
                        None => {
                            ephemeral.window_transient.push(mutation);
                            return Err(plugin_sdk_fault("window transient emission requires one exact captured ViewModel window authority"));
                        }
                    };
                    let grant=self.mounted_policy.maintenance;
                    let refresh=match self.stores.window_transient_store.refresh_demands(authority,grant.maximum_copy_bytes){Ok(_)=>self.stores.window_transient_store.refresh(authority,grant),Err(error)=>Err(plugin_retirement_fault(error))};
                    match refresh{Ok(semio_framework_plugin_window_config::window_mutation::refresh::WindowAuthorityRefreshStep::Ready(_))=>{},Ok(semio_framework_plugin_window_config::window_mutation::refresh::WindowAuthorityRefreshStep::Pending(_))=>{ephemeral.window_transient.push(mutation);return Ok(());},Err(fault)=>{ephemeral.window_transient.push(mutation);return Err(fault);}}
                    match self.stores.window_transient_store.begin(mounted.operation.operation, authority, mutation) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::WindowTransient(publication));
                            return Ok(());
                        }
                        Err(semio_framework_plugin_window_transient::window_transient::RejectedWindowTransientEmission { mutation, fault }) => {
                            ephemeral.window_transient.push(mutation);
                            return Err(fault);
                        }
                    }
                } else if !emit.child_emits.is_empty() {
                    if mounted.pending_child_publication.is_some() {
                        return Err(plugin_sdk_fault("typed-operation retained more than one child-group publication owner"));
                    }
                    mounted.pending_child_publication = Some(PendingChildGroupPublication::new(std::mem::take(&mut emit.artifact_mutations), std::mem::take(&mut emit.child_emits), emit.transaction.take())?);
                    return Ok(());
                } else {
                    MediaTransfers::<A>::mint_extension_invocations(mounted.meta.instance_id, emit)?;
                    if let Some(effect) = emit.effects.pop() {
                        let effect = self.stores.stamp_load_document_identity(effect)?;
                        if let Err(effect) = self.tools.typed_effect_outbox.push(effect) {
                            emit.effects.push(effect);
                            return Err(plugin_sdk_fault("typed-operation effect receiver is saturated"));
                        }
                        TypedOperationResultPage::try_serialize_cold(token, TypedOperationResultLane::Effect, &("accepted", self.tools.typed_effect_outbox.len()))?
                    } else if let Some(event) = emit.events.pop() {
                        if let Err(event) = self.tools.typed_event_outbox.push(event) {
                            emit.events.push(event);
                            return Err(plugin_sdk_fault("typed-operation event receiver is saturated"));
                        }
                        TypedOperationResultPage::try_serialize_cold(token, TypedOperationResultLane::Event, &("accepted", self.tools.typed_event_outbox.len()))?
                    } else {
                        match () {
                            () if !emit.tasks.is_empty() => {
                                let task = emit.tasks.remove(0);
                                semio_framework_plugin_reactor_tasks::tasks_task_lane::spawn_task(mounted.meta.instance_id, &mounted.meta, task)?;
                                return Ok(());
                            }
                            () if mounted.ui_pending => {
                                if let Err(_scope) = self.tools.typed_ui_outbox.push(emit.ui_scope.clone()) {
                                    return Err(plugin_sdk_fault("typed-operation UI receiver is saturated"));
                                }
                                mounted.ui_pending = false;
                                TypedOperationResultPage::try_serialize_cold(token, TypedOperationResultLane::Ui, &emit.ui_scope)?
                            }
                            () => {
                                if self.tools.typed_completion_outbox.push(TypedOperationCompletionWitness { operation: token.operation, ui_scope: emit.ui_scope.clone() }).is_err() {
                                    return Err(plugin_sdk_fault("typed-operation completion receiver is saturated"));
                                }
                                TypedOperationResultPage::try_new(token, TypedOperationResultLane::Terminal, b"typed-operation-complete")?
                            }
                        }
                    }
                }
            }
        };
        mounted.queue_page(page)
    }
}
