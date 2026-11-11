//! 📬️ Typed operation unit of the typed operation publication: the publication of one mounted typed operation with its store, history, receipt and result page steps. See [`super`].

use super::*;
use semio_plugin_retained_command::typed_operation::pending_result_page::{PublicationPageIntent,PublicationPagePlan,PublicationRawReason};

enum OriginalPublicationAdvanceFailure{Primary(semio_plugin_ownership::bounded_ownership::PluginLifecyclePrimary),Domain(store::VcsError)}

/// 🪙️ Runs one effect-taking store step inside the turn's own performed-work effect and settles its actual work once; `None` means the Work quote was not admitted and nothing ran.
fn granted_effect<R>(cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant,run:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->R)->Result<Option<R>,semio_framework_job::OriginalTurnFailure>{
    let Some(((value,work),_))=cx.with_original_effect(grant,|_,effect|{let value=run(effect);(value,effect.performed_work())})?else{return Ok(None)};
    semio_framework_job::receive_original_retained_work(cx,semio_framework_value::RetainedWorkOutcome{result:Ok::<_,semio_framework_value::ValueError>(semio_framework_value::RetainedCloneStep::Progress(Default::default())),work})?;
    Ok(Some(value))
}

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> TypedCtx<'ctx, A, M, X> {
    pub fn publish_mounted_typed_operation_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, retained:RetainedCloneGrant, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        if !mounted.publication_failure.terminal_is_empty(){let custody=&mut mounted.publication_failure;if !custody.reservation.as_ref().is_some_and(|place|place.ready()){custody.reservation.get_or_insert_with(Default::default).prepare(cx)?;return Ok(())}let place=custody.reservation.as_mut().expect("funded publication failure reservation");if !place.pending(){place.with_body(cx,|_,slot|{*slot=custody.original.take();Ok(())})?;}if let Some(mut failure)=semio_framework_job::admit_original_domain_failure(place,cx)?{if place.terminal_is_empty(){custody.reservation=None}failure.effects=Default::default();return Err(failure.into())}return Ok(())}
        if mounted.result_page_retirement_pending() {
            let step = cx.with_retained_loan(retained, |same| mounted.advance_acknowledged_result_page(same))?;
            let progress = match step {
                semio_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress) | semio_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(progress) => progress,
                _ => Default::default(),
            };
            mounted.publication_ownership_progress = Some((retained, progress));
            return Ok(());
        }
        if mounted.advance_pending_result_page(cx)?{return Ok(())}
        if let Some((lane,receipt))=mounted.pending_artifact_publication.as_ref().and_then(|owner|owner.accepted_receipt().map(|receipt|(owner.lane().0,receipt))){
            if lane==TypedOperationResultLane::WindowConfig{if mounted.pending_window_config_receipt.is_none(){let d=semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<Option<store::os_store::presence::LaneItemReceipt>>(),depth:1,..Default::default()};cx.with_original_retirement(|cx|{semio_framework_job::close_original_retained_step(cx,d,|_|{mounted.pending_window_config_receipt=Some(receipt);semio_framework_value::RetainedWorkOutcome::<_,semio_framework_value::ValueError>{work:1,result:Ok(semio_framework_value::RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}})?;Ok(())})?;return Ok(())}return self.advance_mounted_window_config_refresh(mounted,retained,cx);}
            let revision=match mounted.pending_artifact_publication.as_ref(){Some(PendingArtifactStorePublication::Artifact(owner))=>{if owner.expected_generation!=receipt.generation_after{return Err(plugin_sdk_fault("accepted artifact receipt changed its retained generation").into())}Some(owner.expected_revision)},_=>None};
            mounted.reserve_store_result(lane,receipt,revision,cx)?;return Ok(())
        }
        if mounted.reject_cancelled_publication()? {
            mounted.reserve_cancelled_result(cx)?;
            self.stores.handoff_mounted_durable_publication(mounted)?;
            return Ok(());
        }
        let _permit = mounted.cancellation_lease.as_ref().and_then(ToolCancellationLease::try_claim_publication).ok_or_else(|| plugin_sdk_fault("typed-operation publication claim is cancelled or occupied"))?;
        if mounted.publication.is_none(){
            let bytes=2*std::mem::size_of::<ArtifactToolCompletionValue<A>>()+std::mem::size_of::<bool>();let grant=cx.retained_grant();
            if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(())}
            let completion=mounted.completion.as_ref().ok_or_else(||plugin_sdk_fault("typed-operation completion owner was retired before publication"))?;
            mounted.publication=completion.take()?;
            if mounted.publication.is_none(){return Ok(())}
            mounted.ui_pending=matches!(mounted.publication,Some(ArtifactToolCompletionValue::Emit(Ok(_),_)));
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);
            return Ok(())
        }
        if mounted.tool_stage!=semio_plugin_retained_command::typed_operation::OriginalToolSettlementStage::Complete{
            let complete=self.as_render().settle_tool_operation(mounted.operation.operation.0,&mounted.canonical_revision,&mounted.meta.actor,mounted.publication.as_mut().expect("mounted publication retains its accepted emit"),&mut mounted.tool_stage,&mut mounted.tool_press_result,&mut mounted.command_logged,cx)?;
            if !complete{return Ok(())}
            let bytes=std::mem::size_of::<semio_plugin_retained_command::typed_operation::OriginalToolSettlementStage>()+std::mem::size_of::<bool>();let grant=cx.retained_grant();
            if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(())}
            if mounted.tool_stage==semio_plugin_retained_command::typed_operation::OriginalToolSettlementStage::Scrub{if let Some(ArtifactToolCompletionValue::Emit(Ok(emit),_))=mounted.publication.as_ref(){if emit.transaction.is_none(){mounted.command_logged=true}}}
            mounted.tool_stage=semio_plugin_retained_command::typed_operation::OriginalToolSettlementStage::Complete;
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);
            if let Some(ArtifactToolCompletionValue::Emit(Ok(emit),_))=mounted.publication.as_mut(){let verbs=take_inline_interaction_verbs(&mut emit.effects);if !verbs.is_empty(){self.tools.typed_inline_interaction_verbs.retain(|(operation,_)|*operation!=mounted.operation.operation.0);self.tools.typed_inline_interaction_verbs.push((mounted.operation.operation.0,verbs));}}
            return Ok(())
        }
        if let Some((generation,revision))=mounted.pending_stream_abort.as_ref().and_then(|owner|owner.published_frontier()){
            if generation!=self.stores.store.generation_now()||revision!=self.stores.store.content_revision_now(){return Err(plugin_sdk_fault("stream abort original published frontier changed before mounted acknowledgement").into())}
            if mounted.artifact_generation!=generation{let bytes=2*std::mem::size_of::<u64>()+2*std::mem::size_of::<[u8;32]>();let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(())}mounted.artifact_generation=generation;mounted.canonical_revision=revision;mounted.operation.base_revision=semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap()));mounted.operation.generation=semio_framework_job::Generation(generation);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(())}
        }
        let live_revision = self.stores.store.content_revision_now();
        let live_generation = self.stores.store.generation_now();
        if !typed_operation_document_is_fresh(&mounted.operation, mounted.canonical_revision, live_revision, live_generation) {
            if self.stores.handoff_mounted_durable_publication(mounted)? {
                return Err(plugin_sdk_fault("typed-operation pending publication rejected a stale immutable document root").into());
            }
            if let Some(pending)=mounted.pending_artifact_publication.as_mut(){pending.begin_close();}
            return Err(plugin_sdk_fault("typed-operation pending publication rejected a stale immutable document root").into());
        }
        if mounted.pending_artifact_publication.is_none() && matches!(mounted.publication.as_ref(),Some(ArtifactToolCompletionValue::Emit(Ok(emit),_))if !emit.artifact_mutations.is_empty()||!emit.config_mutations.is_empty()||!emit.draft_mutations.is_empty()||!emit.window_config_mutations.is_empty()) && mounted.admit_actor_capture(retained,cx)? {return Ok(());}
        if let Some(pending) = mounted.pending_artifact_publication.as_mut() {
            let capture=if matches!(pending,PendingArtifactStorePublication::Artifact(_)|PendingArtifactStorePublication::Config(_)|PendingArtifactStorePublication::Draft(_)){2*std::mem::size_of::<Option<semio_plugin_retained_command::typed_operation::OriginalPublicationFailure>>()}else{0};
            if retained.maximum_items==0||retained.maximum_depth==0||retained.maximum_copy_bytes<capture{return Ok(())}
            let body_grant=RetainedCloneGrant{maximum_copy_bytes:retained.maximum_copy_bytes-capture,..retained};let grant=store::os_store::one_item::ArtifactStoreOneItemGrant::from_retained(body_grant);
            let (pending_lane, _) = pending.lane();
            if pending.is_closing(){return Ok(());}
            if mounted.pending_window_config_receipt.is_some() { return self.advance_mounted_window_config_refresh(mounted, retained,cx); }
            let pending = mounted.pending_artifact_publication.as_mut().expect("pending publication checked above");
            if matches!(pending, PendingArtifactStorePublication::Artifact(_))
                && self.stores.store.returned_snapshot_read_count() > PUBLICATION_RETURNED_ROOT_ALLOWANCE
                && !self.ingest.reclaim_document_snapshot_read_returns(&*self.stores)?
            {
                return Ok(());
            }
            let pending = mounted.pending_artifact_publication.as_mut().expect("pending publication checked above");
            let mut scope=None;let mut stepped=None;
            if !cx.with_native_receiving(&mut scope,|receiving,_|{stepped=Some(match pending {
                PendingArtifactStorePublication::Artifact(publication) => self.stores.store.advance_apply_batch(publication, body_grant.maximum_copy_bytes, grant,receiving).map_err(OriginalPublicationAdvanceFailure::Domain),
                PendingArtifactStorePublication::Config(publication) => self.stores.config_store.advance_apply_batch(publication, body_grant.maximum_copy_bytes, grant,receiving).map_err(OriginalPublicationAdvanceFailure::Domain),
                PendingArtifactStorePublication::Draft(publication) => self.stores.draft_store.advance_apply_batch(publication, body_grant.maximum_copy_bytes, grant,receiving).map_err(OriginalPublicationAdvanceFailure::Domain),
                PendingArtifactStorePublication::Presence(publication) => self.stores.presence_store.advance_publish_one(publication, grant,receiving).map_err(|error|OriginalPublicationAdvanceFailure::Primary(error.into())),
                PendingArtifactStorePublication::Transient(publication) => self.stores.transient_store.advance_publish_one(publication, grant,receiving).map_err(|error|OriginalPublicationAdvanceFailure::Primary(error.into())),
                PendingArtifactStorePublication::WindowConfig(publication) => self.stores.window_config_store.advance(publication.as_mut(),body_grant.maximum_copy_bytes, grant,receiving).map_err(|error|OriginalPublicationAdvanceFailure::Primary(error.into())),
                PendingArtifactStorePublication::WindowTransient(publication) => self.stores.window_transient_store.advance(publication.as_mut(), grant,receiving).map_err(OriginalPublicationAdvanceFailure::Primary),
            });Ok(())})?{return Ok(())}
            let scope=scope.ok_or_else(||semio_framework_value::ValueError::invariant("original receiving scope vanished"))?;let result=stepped.ok_or_else(||semio_framework_value::ValueError::invariant("original receiving step did not run"))?;
            let work=match &*pending{PendingArtifactStorePublication::Artifact(publication)=>publication.last_turn_work(),PendingArtifactStorePublication::Config(publication)=>publication.last_turn_work(),PendingArtifactStorePublication::Draft(publication)=>publication.last_turn_work(),_=>0};
            cx.consume_fuel(work);
            let ownership=match &result{Ok(store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::PreparationProgress(_,receipt))=>*receipt,Err(OriginalPublicationAdvanceFailure::Primary(error))=>error.retained_progress(),Err(OriginalPublicationAdvanceFailure::Domain(error))=>{let mut progress=match error{store::VcsError::InverseRefused(error)|store::VcsError::NativeEncoding(error)=>error.retained_progress(),_=>Default::default()};progress.copied_items=progress.copied_items.max(1);progress.copied_bytes=progress.copied_bytes.checked_add(capture).ok_or_else(||semio_framework_value::ValueError::invariant("publication failure capture exceeds original receipt"))?;progress},_=>Default::default()};
            let receipt=cx.consume_receiving_retained(retained,ownership);
            if scope.source_refusal.is_some()||scope.call.is_err(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"original publication receiving scope refused").with_retained_progress(scope.retained_progress()).into())}
            let advance=match result{Err(OriginalPublicationAdvanceFailure::Primary(primary))=>return Err(primary.with_receiving_receipt(receipt)),Err(OriginalPublicationAdvanceFailure::Domain(source))=>{assert!(mounted.publication_failure.original.is_none());mounted.publication_failure.original=Some(semio_plugin_retained_command::typed_operation::OriginalPublicationFailure::new(source,ownership,receipt.err()));return Ok(())},Ok(advance)=>{receipt?;advance}};
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
                            return Ok(());
                        }
                        TypedOperationResultLane::WindowTransient => {
                            let authority = mounted.window_transient_authority.as_mut().ok_or_else(|| plugin_sdk_fault("window transient receipt lost its captured window authority"))?;
                            authority.generation = receipt.generation_after;
                        }
                        _ => unreachable!("pending artifact publication has one durable store lane"),
                    }
                    Ok(())
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Progress(checkpoint) => {
                    mounted.note_publication_checkpoint(checkpoint);
                    Ok(())
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::PreparationProgress(checkpoint, ownership) => {
                    if !ownership.fits(grant.retained_grant())||grant.maximum_depth==0&&ownership!=RetainedCloneProgress::default(){return Err(plugin_sdk_fault("typed preparation violated its original full grant").into());}
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
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::AwaitingAck(_) => Ok(()),
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Complete => Err(plugin_sdk_fault("artifact-store publication completed before result ACK").into()),
            };
        }
        let token = mounted.next_token();
        let publication_lanes = mounted.publication_lanes;
        let intent = match mounted.publication.as_mut().expect("publication owner was installed") {
            ArtifactToolCompletionValue::Emit(Err(fault), _) | ArtifactToolCompletionValue::Download(Err(fault), _) => {
                let _=fault;
                PublicationPageIntent::Raw{lane:TypedOperationResultLane::Fault,reason:PublicationRawReason::Fault}
            }
            ArtifactToolCompletionValue::Download(Ok(download), ephemeral) => {
                if (!ephemeral.presence.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Presence))
                    || (!ephemeral.transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Transient))
                    || (!ephemeral.window_transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::WindowTransient))
                {
                    return Err(plugin_sdk_fault("typed-operation emitted an undeclared ephemeral publication lane").into());
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
                            return Err(plugin_sdk_fault(reason).into());
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
                            return Err(plugin_sdk_fault(reason).into());
                        }
                    }
                } else if let Some(mutation) = ephemeral.window_transient.pop() {
                    let authority = match mounted.window_transient_authority.as_mut() {
                        Some(authority) => authority,
                        None => {
                            ephemeral.window_transient.push(mutation);
                            return Err(plugin_sdk_fault("window transient emission requires one exact captured ViewModel window authority").into());
                        }
                    };
                    let grant=self.mounted_policy.maintenance;
                    let refresh=match self.stores.window_transient_store.refresh_demands(authority,grant.maximum_copy_bytes){Ok(_)=>match granted_effect(cx,grant,|effect|self.stores.window_transient_store.refresh(authority,effect)){Ok(Some(step))=>step,Ok(None)=>Ok(semio_plugin_window_config_mutation::refresh::WindowAuthorityRefreshStep::Pending(Default::default())),Err(failure)=>{ephemeral.window_transient.push(mutation);return Err(failure.into());}},Err(error)=>Err(plugin_retirement_fault(error))};
                    match refresh{Ok(semio_plugin_window_config_mutation::refresh::WindowAuthorityRefreshStep::Ready(_))=>{},Ok(semio_plugin_window_config_mutation::refresh::WindowAuthorityRefreshStep::Pending(_))=>{ephemeral.window_transient.push(mutation);return Ok(());},Err(fault)=>{ephemeral.window_transient.push(mutation);return Err(fault.into());}}
                    match self.stores.window_transient_store.begin(mounted.operation.operation, authority, mutation) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::WindowTransient(publication));
                            return Ok(());
                        }
                        Err(semio_plugin_window_transient::window_transient::RejectedWindowTransientEmission { mutation, fault }) => {
                            ephemeral.window_transient.push(mutation);
                            return Err(fault.into());
                        }
                    }
                } else {
                    let _=download;PublicationPageIntent::Download
                }
            }
            ArtifactToolCompletionValue::Emit(Ok(emit), ephemeral) => {
                let outcome=emit.prepare_child_original(cx);let preparation=match outcome{Ok(step)=>step,Err(fault)=>{
                    let progress=fault.retained_progress();if !progress.fits(retained){return Err(plugin_sdk_fault("original child preparation error exceeded mounted caller policy").into());}
                    if progress!=RetainedCloneProgress::default(){assert!(mounted.publication_ownership_progress.is_none(),"child preparation error cannot overwrite an original receipt");mounted.publication_ownership_progress=Some((retained,progress));}return Err(fault.into());
                }};
                let progress=preparation.progress();
                if !progress.fits(retained){return Err(plugin_sdk_fault("original emitted child preparation exceeded mounted caller policy").into());}
                if progress!=RetainedCloneProgress::default(){
                    assert!(mounted.publication_ownership_progress.is_none(),"child preparation cannot overwrite an original receipt");mounted.publication_ownership_progress=Some((retained,progress));return Ok(());
                }
                match preparation{ChildEmitPreparationStep::Ready(_)=>{},ChildEmitPreparationStep::Pending(_)=>return Ok(()),ChildEmitPreparationStep::Refused(fault,_)=>return Err(fault.into())}
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
                    return Err(plugin_sdk_fault("typed-operation emitted a store lane absent from its exact factory publication contract").into());
                }
                if !emit.artifact_mutations.is_empty() || !emit.owned_child_emits.is_empty() {
                    self.media.require_operation_emitting_kind(&mounted.verb, self.tools.declared_dispatch_kind(&*self.host_events, &mounted.verb))?;
                    if A::ROLE == AppRole::Viewer {
                        return Err(viewer_read_only_fault(&mounted.verb).into());
                    }
                    if let Some(fault) = X::mutation_refusal(&*self.time_travel, &*self.tool_runs, &mounted.verb) {
                        return Err(fault.into());
                    }
                    if let Some(pending_txn_id) = self.txn.pending_transaction.as_ref().map(|pending| pending.txn_id.clone()) {
                        return Err(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.instance-busy", format!("verb {:?} would emit artifact mutations while transaction {pending_txn_id:?} is pending on this instance", mounted.verb)).into());
                    }
                    if emit.artifact_mutations.iter().any(Mutation::may_emit_foreign_steps) {
                        let mut running = self.stores.store.snapshot().map_err(|error| error.into_fault())?;
                        let foreign = Vec::new();
                        for op in emit.artifact_mutations.iter() {
                            if store::ColdMutation::foreign_step_source(op, &running, 0).map_err(|error| error.into_fault())?.is_some() { return Err(plugin_sdk_fault("foreign transaction output awaits its original staged preparation recipient").into()); }
                            let outcome = op.diff(&running);
                            if !outcome.is_applicable(protocol::MergePolicy::default()) {
                                return Err(MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", format!("mutation outcome was rejected: {:?}", outcome.messages())).into());
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
                    return Err(fault.into());
                }
                if !emit.artifact_mutations.is_empty() || !emit.child_emits.is_empty() || !emit.owned_child_emits.is_empty() {
                    if let Some(fault) = X::document_refusal(&*self.time_travel, &*self.tool_runs, &mounted.verb) {
                        return Err(fault.into());
                    }
                }
                if !self.history.close_streamed_transaction_unit(&mut *self.stores,emit,&mut mounted.pending_stream_abort,&mut mounted.stream_abort_resident,&mut mounted.stream_abort_completed,cx)?{return Ok(())}
                if !emit.owned_child_emits.is_empty() {
                    if !emit.child_emits.is_empty() || emit.owned_child_emits.len() > PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN { return Err(plugin_sdk_fault("mounted owned child group requires its one exact admitted typed source family").into()); }
                    let operation = mounted.operation.operation.0;
                    let captured_millis = i64::try_from(store::os_store::document_materialize::now_ms()).map_err(|_|plugin_sdk_fault("mounted child publication requires its original UTC clock capture"))?;
                    let grant=retained;
                    let step=self.child_composition.admit_private_child_group_frame(operation, captured_millis, u64::MAX, grant)?;
                    let progress=step.progress();
                    if !progress.fits(grant){return Err(plugin_sdk_fault("mounted child frame exceeded its original preparation grant").into());}
                    if progress!=RetainedCloneProgress::default(){
                        assert!(mounted.publication_ownership_progress.is_none(),"mounted child frame cannot overwrite its original unreturned receipt");
                        mounted.publication_ownership_progress=Some((retained,progress));
                    }
                    if self.child_composition.private_child_groups.get(operation).is_some(){mounted.owned_child_group=Some(operation);}
                    return Ok(());
                }
                if emit.child_emits.is_empty() && emit.owned_child_emits.is_empty() && !emit.artifact_mutations.is_empty() {
                    if let Some(fault) = self.stores.open_transaction_admission_fault(emit.transaction.as_ref()) {
                        return Err(fault.into());
                    }
                    if let Err(error) = store::os_store::one_item::ArtifactStoreOneItemFootprint::for_gesture::<A::Snapshot, A::Mutation>(&emit.artifact_mutations) {
                        return Err(error.into_fault().into());
                    }
                    let streams=emit.transaction_phase==TransactionPhase::Stream;
                    let outbound=self.stores.store.backbone_ref().is_some();
                    if let Some((mut publication,ownership))=self.stores.store.admit_apply_batch(mounted.operation.operation,mounted.artifact_generation,mounted.canonical_revision,&mut mounted.actor_capture,&mut emit.artifact_mutations,HistoryLane::Document,self.stores.artifact_one_item_factory.as_ref(),&mut emit.transaction,Some(&mounted.verb),None,outbound,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {
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
                    if let Some((publication,ownership))=self.stores.config_store.admit_apply_batch(mounted.operation.operation,mounted.config_generation,revision,&mut mounted.actor_capture,&mut emit.config_mutations,HistoryLane::Document,self.stores.config_one_item_factory.as_ref(),&mut transaction,Some(&mounted.verb),None,false,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {
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
                    if let Some((publication,ownership))=self.stores.draft_store.admit_apply_batch(mounted.operation.operation,mounted.draft_generation,revision,&mut mounted.actor_capture,&mut emit.draft_mutations,HistoryLane::Document,self.stores.draft_one_item_factory.as_ref(),&mut transaction,Some(&mounted.verb),None,false,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {
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
                            return Err(plugin_sdk_fault(reason).into());
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
                            return Err(plugin_sdk_fault(reason).into());
                        }
                    }
                } else if let Some(mutation) = ephemeral.window_transient.pop() {
                    let authority = match mounted.window_transient_authority.as_mut() {
                        Some(authority) => authority,
                        None => {
                            ephemeral.window_transient.push(mutation);
                            return Err(plugin_sdk_fault("window transient emission requires one exact captured ViewModel window authority").into());
                        }
                    };
                    let grant=self.mounted_policy.maintenance;
                    let refresh=match self.stores.window_transient_store.refresh_demands(authority,grant.maximum_copy_bytes){Ok(_)=>match granted_effect(cx,grant,|effect|self.stores.window_transient_store.refresh(authority,effect)){Ok(Some(step))=>step,Ok(None)=>Ok(semio_plugin_window_config_mutation::refresh::WindowAuthorityRefreshStep::Pending(Default::default())),Err(failure)=>{ephemeral.window_transient.push(mutation);return Err(failure.into());}},Err(error)=>Err(plugin_retirement_fault(error))};
                    match refresh{Ok(semio_plugin_window_config_mutation::refresh::WindowAuthorityRefreshStep::Ready(_))=>{},Ok(semio_plugin_window_config_mutation::refresh::WindowAuthorityRefreshStep::Pending(_))=>{ephemeral.window_transient.push(mutation);return Ok(());},Err(fault)=>{ephemeral.window_transient.push(mutation);return Err(fault.into());}}
                    match self.stores.window_transient_store.begin(mounted.operation.operation, authority, mutation) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::WindowTransient(publication));
                            return Ok(());
                        }
                        Err(semio_plugin_window_transient::window_transient::RejectedWindowTransientEmission { mutation, fault }) => {
                            ephemeral.window_transient.push(mutation);
                            return Err(fault.into());
                        }
                    }
                } else if !emit.child_emits.is_empty() {
                    if mounted.pending_child_publication.is_some() {
                        return Err(plugin_sdk_fault("typed-operation retained more than one child-group publication owner").into());
                    }
                    mounted.pending_child_publication = Some(PendingChildGroupPublication::new(std::mem::take(&mut emit.artifact_mutations), std::mem::take(&mut emit.child_emits), emit.transaction.take())?);
                    return Ok(());
                } else {
                    MediaTransfers::<A>::mint_extension_invocations(mounted.meta.instance_id, emit)?;
                    if !emit.effects.is_empty(){
                        let d=semio_framework_value::RetirementDemand{copy_bytes:2*std::mem::size_of_val(emit.effects.last().unwrap())+std::mem::size_of::<Option<PublicationPagePlan>>(),depth:1,..Default::default()};
                        if !cx.retained_work_can_enter(d){return Ok(())}
                        let effect=emit.effects.pop().unwrap();let effect=self.stores.stamp_load_document_identity(effect)?;
                        if let Err(effect)=self.tools.typed_effect_outbox.push(effect){emit.effects.push(effect);return Err(plugin_sdk_fault("typed-operation effect receiver is saturated").into())}
                        mounted.page_publication.plan=Some(PublicationPagePlan{token,intent:PublicationPageIntent::Accepted{lane:TypedOperationResultLane::Effect,count:self.tools.typed_effect_outbox.len()as u64}});
                        cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(())
                    }else if !emit.events.is_empty(){
                        let d=semio_framework_value::RetirementDemand{copy_bytes:2*std::mem::size_of_val(emit.events.last().unwrap())+std::mem::size_of::<Option<PublicationPagePlan>>(),depth:1,..Default::default()};
                        if !cx.retained_work_can_enter(d){return Ok(())}
                        let event=emit.events.pop().unwrap();if let Err(event)=self.tools.typed_event_outbox.push(event){emit.events.push(event);return Err(plugin_sdk_fault("typed-operation event receiver is saturated").into())}
                        mounted.page_publication.plan=Some(PublicationPagePlan{token,intent:PublicationPageIntent::Accepted{lane:TypedOperationResultLane::Event,count:self.tools.typed_event_outbox.len()as u64}});
                        cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(())
                    }else if !emit.tasks.is_empty(){
                        let task=emit.tasks.remove(0);semio_plugin_reactor_tasks::tasks_task_lane::spawn_task(mounted.meta.instance_id,&mounted.meta,task)?;return Ok(())
                    }else{
                        let copy=&mut mounted.page_publication.ui_copy;
                        if !copy.ready(){let d=copy.demands(&emit.ui_scope,retained.maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,d,|g|copy.advance(&emit.ui_scope,g))?;return Ok(())}
                        let mut d=semio_plugin_retained_command::typed_operation::MountedUiCopy::transfer_demands();d.copy_bytes+=std::mem::size_of::<Option<PublicationPagePlan>>()+std::mem::size_of::<bool>();
                        if !cx.retained_work_can_enter(d){return Ok(())}
                        let scope=copy.take();
                        let intent=if mounted.ui_pending{match self.tools.typed_ui_outbox.push(scope){Ok(())=>{},Err(scope)=>{copy.restore(scope);return Err(plugin_sdk_fault("typed-operation UI receiver is saturated").into())}}mounted.ui_pending=false;PublicationPageIntent::Ui}
                        else{match self.tools.typed_completion_outbox.push(TypedOperationCompletionWitness{operation:token.operation,ui_scope:scope}){Ok(())=>{},Err(witness)=>{copy.restore(witness.ui_scope);return Err(plugin_sdk_fault("typed-operation completion receiver is saturated").into())}}PublicationPageIntent::Raw{lane:TypedOperationResultLane::Terminal,reason:PublicationRawReason::Complete}};
                        mounted.page_publication.plan=Some(PublicationPagePlan{token,intent});cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(())
                    }
                }
            }
        };
        mounted.reserve_result_intent(intent,cx)?;Ok(())
    }
}
