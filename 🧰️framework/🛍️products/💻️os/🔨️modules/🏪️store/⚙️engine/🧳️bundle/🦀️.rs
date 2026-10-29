//! 🧳️ Use cases of the store composer that need the whole `StoreParts` bundle: the batched one-item publication advance, the remote-merge ingest and the pure edit-id resolution.
//!
//! 🧭️ Spill valve D3 of `PLAN-OS-KERNEL-CRATES.md` (sections 5.3, 7.5 and 13): the bodies are the composer's, over the six component borrows; `ArtifactStore` in `os-store-engine` keeps delegates with the old signatures.

use semio_framework_os_spr::ActorId;
use semio_framework_os_spr::ArtifactId;
use semio_framework_os_spr::Edit;
use semio_framework_os_spr::HybridLogicalTimestamp;
use semio_framework_os_spr::Mutation;
use semio_framework_os_spr::MutationId;
use semio_framework_os_spr::OpBinary;
use semio_framework_os_spr::OpText;
use semio_framework_os_spr::SchemaId;
use semio_framework_os_spr::conflict::ProtocolConflictRetirement as ArtifactStoreConflictRetirement;
use semio_framework_os_store_engine_core::lifted_document_materialize::now_iso;
use semio_framework_os_store_engine_core::lifted_store::fold_history;
use semio_framework_os_store_engine_live::lifted_document_materialize::stamp_primary_operation_identity;
use semio_framework_os_store_engine_live::lifted_store::StoreParts;
use semio_framework_os_store_engine_live::lifted_store::StoreReplay;
use semio_framework_os_store_ledger::ArtifactStoreHistoryCommitReservation;
use semio_framework_os_store_ledger::CursorRevisionAccumulator;
use semio_framework_os_store_ledger::CursorRevisionRecord;
use semio_framework_os_store_ledger::PendingCommandReport;
use semio_framework_os_store_ledger::edit_from_operation_envelope_cold;
use semio_framework_os_store_ledger::retire_scratch_edits;
use semio_framework_os_store_ledger::retire_scratch_operations;
use semio_framework_os_store_ledger::stamp_edit_semantics;
use semio_framework_os_store_one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES;
use semio_framework_os_store_one_item::ArtifactStoreOneItemFootprint;
use semio_framework_os_store_one_item::ArtifactStoreOneItemGrant;
use semio_framework_os_store_one_item::ArtifactStoreOneItemPreparationStep;
use semio_framework_os_store_one_item::LaneItemReceipt;
use semio_framework_os_store_one_item::validate_original_preparation_step;
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchPublication;
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchStage;
use semio_framework_os_store_one_item_publication::ArtifactStoreBatchItemRequest;
use semio_framework_os_store_one_item_publication::ArtifactStoreOneItemAdvance;
use semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparation;
use semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPublicationPhase;
use semio_framework_os_store_read::ArtifactStoreMutationDagRetirement;
use semio_framework_os_store_read::ArtifactStoreRevisionAccumulatorRetirement;
use semio_framework_os_store_read::ArtifactStoreStringVectorRetirement;
use semio_framework_os_store_read::SnapshotRead;
use semio_framework_os_store_read::artifact_retirement_box_close_step;
use semio_framework_os_store_replay::EditReplayResult;
use semio_framework_os_store_replay::retire_shared_projection;
use semio_framework_os_store_space_schema::ArtifactProjectionCause;
use semio_framework_os_store_space_schema::HistoryLane;
use semio_framework_os_store_space_schema::OpenToolTransaction;
use semio_framework_os_store_text_io::text_format::canonical_conflict_actors;
use semio_framework_os_store_text_io::text_format::conflict_messages_for_edits;
use semio_framework_os_vcs::ArtifactHistoryKey;
use semio_framework_os_vcs::ArtifactHistoryLedger;
use semio_framework_os_vcs::VcsError;
use semio_framework_value::ArtifactOwnedValueRetirementFactory;
use semio_framework_value::FromValue;
use semio_framework_value::SnapshotRetirementFactory;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use std::collections::HashMap;
use std::sync::Arc;

use semio_framework_os_store_replay::ReplayEdits;
use semio_framework_os_store_one_item_publication::diagnostics::ArtifactStoreDiagnosticStage;
use semio_framework_os_spr::command::MutationMessageLedgerRetirement as ArtifactStoreMessageLedgerRetirement;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use std::mem::size_of;
use semio_framework_os_store_replay::original_ingress::{OriginalReplayEdits, original_edits_for_ids};
pub use semio_framework_os_store_engine_live::lifted_store::publish_prepared_diagnostics;
use semio_framework_os_store_one_item_publication::ArtifactStoreOneItemLiveAuthority;
#[path="📤️message/🦀️.rs"]
mod original_message;
#[path="📚️metadata/🦀️.rs"]
mod commit_metadata;
#[path="📬️commit/🦀️.rs"]
mod original_commit;
pub use commit_metadata::{advance_batch_metadata,BatchCommitMetadataStep};

//#region 📬️OneItemPublication
/// ⏭️ Advances at most one preparation, fold, cursor, preflight, or atomic move-publication
/// unit. Exactly one admitted mutation is folded into the staged edit per fold turn; the
/// post-snapshot each item produced is an already-built `Arc` and is moved into the stage.
pub fn advance_apply_batch<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>, publication: &mut ArtifactStoreBatchPublication<P, Mutation>, body: usize, grant: ArtifactStoreOneItemGrant, receiving: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemAdvance, VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{
    if !publication.original_registry.as_ref().is_some_and(|registry|registry.ptr_eq(&store.owners.snapshot_read_leases)){return Err(VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"batched publication requires its original Store issuer")));}
    ensure_durable_group_idle(store)?;
    if publication.group_preparation.is_some() { return Err(VcsError::ValidationFailed("group-preparing batch requires its exact common publication decision".into())); }
    if publication.phase == ArtifactStoreOneItemPublicationPhase::Complete {
        return Ok(ArtifactStoreOneItemAdvance::Complete);
    }
    if publication.phase == ArtifactStoreOneItemPublicationPhase::Closing {
        {return Ok(ArtifactStoreOneItemAdvance::Blocked)};
    }
    if publication.phase == ArtifactStoreOneItemPublicationPhase::AwaitingAck {
        return Ok(ArtifactStoreOneItemAdvance::AwaitingAck(publication.receipt.expect("awaiting-ACK publication retains its receipt")));
    }
    if publication.cancel_requested {
        publication.begin_close();
        {return Ok(ArtifactStoreOneItemAdvance::Blocked)};
    }
    if store.cursor.generation != publication.expected_generation || store.cursor.content_revision != publication.expected_revision {
        publication.fault = Some(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"batched publication became stale before commit"));
        publication.phase = ArtifactStoreOneItemPublicationPhase::Fault;
        publication.begin_close();
        return Err(VcsError::ValidationFailed("batched publication became stale before commit".into()));
    }
    if !grant.permits_one() {
        {return Ok(ArtifactStoreOneItemAdvance::Blocked)};
    }
    let item_grant = ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), ..grant };
    match publication.phase {
        ArtifactStoreOneItemPublicationPhase::Preparing => {
            if !publication.source_census_finished {
                let copy=size_of::<ArtifactStoreOneItemFootprint>()+size_of::<usize>();
                if item_grant.maximum_copy_bytes<copy||item_grant.maximum_depth==0{{return Ok(ArtifactStoreOneItemAdvance::Blocked)};}
                match publication.source.as_mut().ok_or_else(|| VcsError::ValidationFailed("batched publication lost its retained census source".into()))?.census_one(publication.lane).map_err(VcsError::ValidationFailed)? {
                    Some(footprint) => { publication.footprint = footprint; publication.source_census_finished = true; },
                    None => { publication.source_census_completed = publication.source_census_completed.saturating_add(1); },
                }
                return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}));
            }
            if publication.item_closing {
                if let Some(stage)=publication.stage.as_mut(){if stage.retiring.is_some(){let step=artifact_retirement_box_close_step(&mut stage.retiring,item_grant.retained_grant()).map_err(|error|VcsError::ValidationFailed(error.into_message()))?;return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),step.progress()));}}
                if publication.preparation.is_none(){return Err(VcsError::ValidationFailed("staged batch item lost its original preparation before closure".into()));}
                let step=spend_item_close_grant(&mut publication.preparation,item_grant,receiving).map_err(VcsError::InverseRefused)?;
                if matches!(step,RetainedCloneStep::Complete(_)){publication.item_closing=false;}
                return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),step.progress()));
            }
            if publication.stage.as_ref().is_some_and(|stage|stage.folding.is_some()){return fold_batch_item(store, publication,item_grant);}
            if publication.preparation.is_none() {
                let staged_root = publication.stage.as_ref().and_then(|stage| stage.post.as_ref());
                let remaining = publication.source.as_ref().ok_or_else(|| VcsError::ValidationFailed("batched publication lost its admitted item source".into()))?.remaining();
                if remaining == 0 {
                    if publication.stage.is_none() {
                        return Err(VcsError::ValidationFailed("batched publication staged no edit before its cursor turn".into()));
                    }
                    let copy=size_of::<ArtifactStoreOneItemPublicationPhase>();if item_grant.maximum_copy_bytes<copy||item_grant.maximum_depth==0{{return Ok(ArtifactStoreOneItemAdvance::Blocked)};}
                    publication.phase = ArtifactStoreOneItemPublicationPhase::PreparingCursor;
                    return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}));
                }
                let refused = |error: ValueError| VcsError::ValidationFailed(error.into_message());
                let demand = publication.source.as_ref().expect("validated batched publication item source").next_begin_demand(publication.lane).map_err(refused)?;
                if demand.admit(item_grant.retained_grant()).is_err() { return Ok(ArtifactStoreOneItemAdvance::Blocked); }
                let original_issuer_copy=size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>()+size_of::<Arc<dyn SnapshotRetirementFactory<P>>>()+size_of::<Arc<ArtifactStoreOneItemLiveAuthority>>()+size_of::<SnapshotRead<P>>() + size_of::<Arc<P>>();
                let copy=publication.source.as_ref().expect("validated original preparation copy source").next_begin_copy_byte_demand(publication.lane).map_err(refused)?.checked_add(original_issuer_copy).ok_or_else(||VcsError::ValidationFailed("batched preparation original header extent overflow".into()))?;
                if item_grant.maximum_copy_bytes<copy{{return Ok(ArtifactStoreOneItemAdvance::Blocked)};}
                let mutation_retirement=store.owners.mutation_retirement_factory.as_ref().ok_or_else(||VcsError::ValidationFailed("batched preparation lost its installed mutation issuer".into()))?;
                let snapshot_retirement=store.owners.snapshot_retirement_factory.as_ref().ok_or_else(||VcsError::ValidationFailed("batched preparation lost its installed snapshot issuer".into()))?;
                let authority = Arc::clone(publication.authority.as_ref().ok_or_else(|| VcsError::ValidationFailed("batched publication lost its live authority".into()))?);
                let base = batch_item_base(store, staged_root)?;
                let request = ArtifactStoreBatchItemRequest { operation: publication.operation, generation: authority.generation(), base_revision: authority.base_revision(), lane: publication.lane, authority, base, mutation_retirement:Arc::clone(mutation_retirement),snapshot_retirement:Arc::clone(snapshot_retirement) };
                let source = publication.source.as_mut().expect("validated batched publication item source");
                match source.begin_next(request, ArtifactStoreOneItemGrant{maximum_copy_bytes:item_grant.maximum_copy_bytes-original_issuer_copy,..item_grant}) {
                    Ok((preparation, progress)) => {
                        publication.preparation = Some(preparation);
                        let progress=RetainedCloneProgress{copied_bytes:progress.copied_bytes.checked_add(original_issuer_copy).ok_or_else(||VcsError::ValidationFailed("batched preparation original issuer copy receipt overflow".into()))?,..progress};
                        if let Err(error)=semio_framework_value::retained_clone::admit_retained_clone_progress(item_grant.retained_grant(), progress, "batched preparation birth") { publication.fault=Some(error.with_retained_progress(progress));publication.begin_close();return Ok(ArtifactStoreOneItemAdvance::PreparationRefused); }
                        return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),progress));
                    }
                    Err((error, request)) => {
                        let ArtifactStoreBatchItemRequest { authority, base, mutation_retirement, snapshot_retirement, .. } = request;
                        publication.refused_base = Some(base);
                        publication.refused_authority = Some(authority);
                        let mutation: Arc<dyn semio_framework_value::FactoryRetirement> = mutation_retirement;
                        let snapshot: Arc<dyn semio_framework_value::FactoryRetirement> = snapshot_retirement;
                        publication.refused_issuers = [Some(semio_framework_value::FactoryAuthority::new(mutation)), Some(semio_framework_value::FactoryAuthority::new(snapshot))];
                        let original_progress = error.retained_progress();
                        let copied_bytes = match original_progress.copied_bytes.checked_add(original_issuer_copy) {
                            Some(bytes) => bytes,
                            None => { publication.fault = Some(error); publication.begin_close(); return Ok(ArtifactStoreOneItemAdvance::PreparationRefused); }
                        };
                        let progress = RetainedCloneProgress { copied_bytes, ..original_progress };
                        publication.fault = Some(error.with_retained_progress(progress));
                        publication.begin_close();
                        return Ok(ArtifactStoreOneItemAdvance::PreparationRefused);
                    }
                }
                return Ok(ArtifactStoreOneItemAdvance::Progress(publication.progress()));
            }
            let owner = publication.preparation.as_mut().expect("validated batched publication item preparation");
            if owner.prepared().is_none() {
                let result=owner.advance(body,item_grant,receiving).and_then(|step|validate_original_preparation_step(step,item_grant,owner.prepared().is_some()));
                let step=match result {Ok(step)=>step,Err(reason)=>{publication.fault=Some(reason);publication.begin_close();return Ok(ArtifactStoreOneItemAdvance::PreparationRefused)}};
                return match step {
                    ArtifactStoreOneItemPreparationStep::Progress(checkpoint, ownership) => Ok(ArtifactStoreOneItemAdvance::PreparationProgress(checkpoint, ownership)),
                    ArtifactStoreOneItemPreparationStep::Prepared(checkpoint, ownership) => {
                        Ok(ArtifactStoreOneItemAdvance::PreparationProgress(checkpoint,ownership))
                    }
                    ArtifactStoreOneItemPreparationStep::Blocked => Ok(ArtifactStoreOneItemAdvance::Blocked),
                };
            }
            fold_batch_item(store, publication, item_grant)
        }
        ArtifactStoreOneItemPublicationPhase::PreparingCursor => {
            let stage = publication.stage.as_ref().ok_or_else(|| VcsError::ValidationFailed("batched cursor preparation lost its staged edit".into()))?;
            let authority = publication.authority.as_ref().ok_or_else(|| VcsError::ValidationFailed("batched cursor preparation lost its live authority".into()))?;
            if stage.edit.as_ref().expect("live staged edit owner").id.is_empty() || stage.edit.as_ref().expect("live staged edit owner").id.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES || authority.base_applied_edit_count() != store.log.applied_edit_ids.len() {
                return Err(VcsError::ValidationFailed("domain-prepared batched candidate failed its live cursor authority".into()));
            }
            let envelope=&mut **store.log.envelope;
            if let Some(edit_id) = envelope.open_transaction.as_ref().filter(|open|publication.transaction.as_ref().is_some_and(|transaction|open.transaction.id==transaction.id)&&store.log.applied_edit_ids.last()==Some(&open.edit_id)).map(|open|open.edit_id.as_str()) {
                let existing = envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id).ok_or_else(|| VcsError::ValidationFailed("batched cursor preparation lost its open edit".into()))?;
                let capacity = existing.inverse.len().checked_add(stage.edit.as_ref().expect("live staged edit owner").inverse.len()).ok_or_else(|| VcsError::ValidationFailed("batched inverse capacity overflow".into()))?;
                if let Some(bytes) = existing.inverse.next_capacity_allocation_bytes(capacity).map_err(|error| VcsError::ValidationFailed(error.reason.into()))? {
                    let depth=existing.inverse.next_reserve_depth_demand().map_err(|error|VcsError::ValidationFailed(error.reason.into()))?;
                    if bytes>grant.maximum_capacity_bytes||grant.maximum_depth<depth{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
                    let step = existing.inverse.reserve_capacity_one(capacity, grant.maximum_capacity_bytes).map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
                    if !step.progressed || step.allocated_bytes > grant.maximum_capacity_bytes {
                        return Err(VcsError::ValidationFailed("batched inverse backing failed its exact allocation admission".into()));
                    }
                    return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,retained_capacity_bytes:step.allocated_bytes,..Default::default()}));
                }
            }
            let copy=size_of::<ArtifactStoreOneItemPublicationPhase>();if grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
            publication.phase = ArtifactStoreOneItemPublicationPhase::PreflightingCommit;
            Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))
        }
        ArtifactStoreOneItemPublicationPhase::PreflightingCommit => {
            let stage = publication.stage.as_mut().ok_or_else(|| VcsError::ValidationFailed("batched commit preflight lost its staged edit".into()))?;
            let authority = publication.authority.as_ref().ok_or_else(|| VcsError::ValidationFailed("batched commit preflight lost its live authority".into()))?;
            let refusal=|error:ValueError|VcsError::ValidationFailed(error.into_message());
            if let Some(owner)=stage.digest_owner.as_mut(){
                if owner.ready(){let Some((edit,digest,ownership))=owner.take_edit(grant.retained_grant()).map_err(refusal)?else{return Ok(ArtifactStoreOneItemAdvance::Blocked);};stage.edit=Some(edit);stage.digest=digest;return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),ownership));}
                if owner.terminal_is_empty(){let released=size_of_val(owner.as_ref());if grant.maximum_release_bytes<released||grant.maximum_depth==0{return Ok(ArtifactStoreOneItemAdvance::Blocked);}stage.digest_owner=None;stage.digest_complete=true;return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,released_bytes:released,..Default::default()}));}
                let step=owner.advance_digest(grant.retained_grant()).map_err(refusal)?;let ownership=semio_framework_value::retained_clone::admit_retained_clone_progress(grant.retained_grant(),step.progress(),"original batch digest normal step").map_err(refusal)?;return Ok(if ownership==Default::default(){ArtifactStoreOneItemAdvance::Blocked}else{ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),ownership)});
            }

            let meta = stage.edit.as_ref().expect("live staged edit owner").mutation_meta.first().ok_or_else(|| VcsError::ValidationFailed("batched edit lacks its exact authority metadata".into()))?;
            if  stage.folded != publication.admitted_items
                || stage.edit.as_ref().expect("live staged edit owner").sequence_number != authority.next_sequence_number()
                || stage.edit.as_ref().expect("live staged edit owner").forwards.len() != publication.admitted_items
                || stage.edit.as_ref().expect("live staged edit owner").mutation_meta.len() != publication.admitted_items
                || stage.edit.as_ref().expect("live staged edit owner").id != stage.applied_edit_id
                || stage.edit.as_ref().expect("live staged edit owner").id != stage.tail_edit_id
                || stage.edit.as_ref().expect("live staged edit owner").actor.as_deref() != Some(authority.actor())
                || !stage.local_actor.as_ref().is_some_and(|actor|actor.eq_str(&authority.actor()))
                || stage.next_clock != authority.next_clock()
                || meta.author_id.as_ref().map(|actor| actor.0.as_str()) != Some(authority.actor())
                || meta.timestamp != authority.next_clock()
                || stage.post.is_none()
                || (store.wire.backbone.is_some() && !publication.outbound)
            {
                return Err(VcsError::ValidationFailed("batched prepared candidate failed its exact fixed commit contract".into()));
            }
            let rows = stage.edit.as_ref().expect("live staged edit owner").inverse.len().saturating_add(stage.edit.as_ref().expect("live staged edit owner").forwards.len());
            if rows > publication.footprint.work_items {
                return Err(VcsError::TooLarge { rows, capacity: publication.footprint.work_items });
            }
            let amending = batch_amend_target_ref(store, publication.transaction.as_ref()).is_some();
            if !amending && (!store.log.applied_edit_ids.admits_one() || !store.cursor.revision_accumulator.applied.admits_one() || store.log.envelope.cursor.as_ref().is_none_or(|cursor| !cursor.applied_edit_ids.admits_one())) {
                return Err(VcsError::HistoryFull { capacity: store.log.applied_edit_ids.len() });
            }
            if store.cursor.generation == u64::MAX {
                return Err(VcsError::ValidationFailed("batched store generation is exhausted".into()));
            }
            if !stage.digest_complete{
                let admitted=match store.owners.one_item_preparation_factory.as_ref(){Some(factory)=>factory.begin_batch_digest(&mut stage.edit,grant.retained_grant()),None=>store.log.operation_wire.as_ref().ok_or_else(||VcsError::ValidationFailed("batch original canonical digest authority is absent".into()))?.begin_batch_digest(&mut stage.edit,grant.retained_grant())};
                let Some((owner,ownership))=admitted.map_err(refusal)?else{return Ok(ArtifactStoreOneItemAdvance::Blocked);};stage.digest_owner=Some(owner);return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),ownership));
            }

            if stage.diagnostics.row_count()==0 { if !stage.diagnostics.terminal_is_empty(){stage.diagnostics.begin_close();let step=stage.diagnostics.close_step(grant.retained_grant()).map_err(refusal)?;return Ok(if step.progress()==RetainedCloneProgress::default(){ArtifactStoreOneItemAdvance::Blocked}else{ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),step.progress())});} }
            else if !stage.diagnostics.joined(){let step=stage.diagnostics.advance_join(grant.retained_grant()).map_err(refusal)?;return Ok(if step.progress()==RetainedCloneProgress::default(){ArtifactStoreOneItemAdvance::Blocked}else{ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),step.progress())});}
            let copy=size_of::<ArtifactStoreOneItemPublicationPhase>();if grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
            store.owners.displaced_retirements.reserve(12)?;
            if store.owners.snapshot_retirement_factory.is_none() || store.owners.mutation_retirement_factory.is_none() {
                return Err(VcsError::ValidationFailed("batched commit lacks exact snapshot or mutation retirement authority".into()));
            }
            publication.phase = ArtifactStoreOneItemPublicationPhase::Publishing;
            Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))
        }
        ArtifactStoreOneItemPublicationPhase::Publishing => {
            if publication.outbound&&!publication.transaction_open&&batch_amend_target_ref(store,publication.transaction.as_ref()).is_none()&&store.log.operation_wire.as_ref().is_some_and(|wire|wire.original_message_admission.is_some()){return original_commit::advance_original_commit(store,publication,body,grant)}

            let refusal=|error:ValueError|VcsError::ValidationFailed(error.into_message());
            let staged=publication.stage.as_mut().ok_or_else(||VcsError::ValidationFailed("diagnostic publication lost its original batch stage".into()))?;
            if staged.diagnostics.row_count()!=0 {
                let identity=batch_amend_target_ref(store, publication.transaction.as_ref()).unwrap_or_else(||staged.edit.as_ref().expect("staged original diagnostic edit").id.as_str());
                if !staged.diagnostics.prepared(){let step=staged.diagnostics.advance(identity,grant.retained_grant()).map_err(refusal)?;return Ok(if step.progress()==RetainedCloneProgress::default(){ArtifactStoreOneItemAdvance::Blocked}else{ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),step.progress())});}
                let bytes=staged.diagnostics.measured_bytes(identity).map_err(refusal)?;
                if !store.log.envelope.edit_messages.admits_measured(identity,bytes){return Err(VcsError::ValidationFailed("prepared diagnostic entry exceeds its fixed ledger authority".into()));}
                if store.log.envelope.edit_messages.get_by_id(identity).is_some()&&staged.diagnostics.displaced.is_none(){let receipt=staged.diagnostics.admit_displaced(grant.retained_grant()).map_err(refusal)?;return Ok(if receipt==RetainedCloneProgress::default(){ArtifactStoreOneItemAdvance::Blocked}else{ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),receipt)});}
                let copy=ArtifactStoreDiagnosticStage::transfer_copy_demand();if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
            }
            if let Some(edit_id) = batch_amend_target(store, publication.transaction.as_ref()) {
                let stage = publication.take_stage().ok_or_else(|| VcsError::ValidationFailed("batched publication lost its staged edit at atomic transfer".into()))?;
                let ArtifactStoreBatchStage { edit, post, next_clock, local_actor: _, unit_flags, mut diagnostics, .. } = *stage;
                let edit = edit.expect("live staged edit owner");
                let post = post.ok_or_else(|| VcsError::ValidationFailed("batched publication lost its staged post root at atomic transfer".into()))?;
                let generation_before = store.cursor.generation;
                let revision_before = store.cursor.content_revision;
                let appended = edit.forwards.len();
                let closes = publication.transaction.is_some() && !publication.transaction_open;
                let mut offset = 0;
                if let Some(existing) = store.log.envelope.vcs.edits.iter_mut().find(|candidate| candidate.id == edit_id) {
                    offset = existing.forwards.len();
                    publication.announce_from = if closes { 0 } else { existing.forwards.len() };
                    existing.forwards.extend(edit.forwards);
                    existing.inverse.extend(edit.inverse);
                    existing.mutation_meta.extend(edit.mutation_meta);
                    existing.finished_at = if closes { Some(now_iso()) } else { edit.finished_at };
                    if closes {
                        stamp_primary_operation_identity(existing);
                    }
                    publication.announce_items = existing.forwards.len() - publication.announce_from;
                } else {
                    return Err(VcsError::ValidationFailed("batched transaction append lost its open edit".into()));
                }
                if closes {
                    store.log.envelope.open_transaction = None;
                } else if publication.transaction_open {
                    publication.outbound = false;
                }
                replace_pending_report_retained(store, PendingCommandReport::default())?;


                let diagnostic_receipt=publish_prepared_diagnostics(store, &mut diagnostics,grant.retained_grant())?;

                replace_redo_edit_ids_retained(store, Vec::new())?;
                replace_current_retained(store, post)?;
                store.log.clock = next_clock;
                let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit_id.as_bytes()]);
                store.cursor.revision_accumulator.unit_flags.extend(unit_flags.into_iter().map(|(index, flag)| ((digest, offset + index), flag)));
                bump(store)?;
                continue_pending_replay_across_append(store, revision_before, &edit_id, appended);
                store.cursor.last_projection_cause = Some(ArtifactProjectionCause::Apply);
                let receipt = LaneItemReceipt { generation_before, generation_after: store.cursor.generation };
                publication.expected_generation = store.cursor.generation;
                publication.expected_revision = store.cursor.content_revision;
                publication.receipt = Some(receipt);
                publication.published = true;
                publication.phase = ArtifactStoreOneItemPublicationPhase::AwaitingAck;
                return Ok(ArtifactStoreOneItemAdvance::Published(receipt,diagnostic_receipt));
            }

            let lane_diagnostic_copy=publication.stage.as_ref().filter(|stage|stage.diagnostics.row_count()!=0).map_or(0,|_|ArtifactStoreDiagnosticStage::transfer_copy_demand());
            let lane_grant=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(lane_diagnostic_copy),..grant.retained_grant()};
            if let Some(owner)=publication.lane_insert.as_ref(){if !owner.commit_ready(&store.log.envelope.lanes,lane_grant).map_err(VcsError::InverseRefused)?{return Ok(ArtifactStoreOneItemAdvance::Blocked)}}
            let mut lane_progress=RetainedCloneProgress::default();
            let reservation = reserve_edit_history_slot(store)?;
            let stage = publication.take_stage().ok_or_else(|| VcsError::ValidationFailed("batched publication lost its staged edit at atomic transfer".into()))?;
            let ArtifactStoreBatchStage { edit, post, next_clock, local_actor: _, applied_edit_id, tail_edit_id, digest, unit_flags, mut diagnostics, .. } = *stage;
            let edit = edit.expect("live staged edit owner");
            let digest = if edit.forwards.len() > 1 { CursorRevisionAccumulator::effective_edit_digest::<P, Mutation>(&edit, CursorRevisionAccumulator::edit_digest(&*edit), &store.live.supersessions) } else { digest };
            publication.announce_from = 0;
            publication.announce_items = publication.admitted_items;
            let post = post.ok_or_else(|| VcsError::ValidationFailed("batched publication lost its staged post root at atomic transfer".into()))?;
            let generation_before = store.cursor.generation;
            let pre_snapshot = Arc::clone(&store.live.current);
            let cursor = store.log.envelope.cursor.as_mut().ok_or_else(|| VcsError::ValidationFailed("batched publication lost its live cursor authority".into()))?;
            let retired_cursor_redo = std::mem::take(&mut cursor.redo_edit_ids);
            cursor.applied_edit_ids.push(edit.id.clone());
            if !retired_cursor_redo.is_empty() || retired_cursor_redo.capacity() != 0 {
                store.owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreStringVectorRetirement::new(retired_cursor_redo)));
            }
            replace_pending_report_retained(store, PendingCommandReport::default())?;


            let diagnostic_receipt=publish_prepared_diagnostics(store, &mut diagnostics,grant.retained_grant())?;

            replace_tail_undo_cache_retained(store, Some((tail_edit_id, Arc::clone(&pre_snapshot))))?;
            store.log.applied_edit_ids.push(applied_edit_id);
            replace_redo_edit_ids_retained(store, Vec::new())?;
            replace_current_retained(store, post)?;
            store.log.edit_sequence = edit.sequence_number;
            store.log.clock = next_clock;
            let edit_id_digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit.id.as_bytes()]);
            let previous = store.cursor.revision_accumulator.applied.last().map_or(store.cursor.revision_accumulator.identity_digest, |record| record.prefix_digest);
            let prefix_digest = CursorRevisionAccumulator::hash_record(b"applied", &[&previous, &digest]);
            let opened = publication.transaction.as_ref().cloned().filter(|_| publication.transaction_open).map(|transaction| OpenToolTransaction { transaction, edit_id: edit.id.clone() });
            store.cursor.revision_accumulator.index_applied_edit::<P, Mutation>(store.log.applied_edit_ids.len() - 1, &edit);
            store.cursor.revision_accumulator.unit_flags.extend(unit_flags.into_iter().map(|(index, flag)| ((edit_id_digest, index), flag)));
            let ledger_key = insert_reserved_edit_history(store, reservation, *edit)?;
            if opened.is_some() {
                publication.outbound = false;
                store.log.envelope.open_transaction = opened;
            }
            store.cursor.revision_accumulator.applied.push(CursorRevisionRecord::new(Some(ledger_key), edit_id_digest, digest, prefix_digest));
            let retired_redo = std::mem::take(&mut store.cursor.revision_accumulator.redo);
            if !retired_redo.is_empty() || retired_redo.capacity() != 0 {
                store.owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreRevisionAccumulatorRetirement::new(CursorRevisionAccumulator {
                    identity_digest: store.cursor.revision_accumulator.identity_digest,
                    applied: semio_framework_os_vcs::HistoryPageStack::new(),
                    redo: retired_redo,
                    applied_tail_chains: None,
                    mutation_positions: protocol::HistoryFoldIndex::new(),
                    indexed_edits: protocol::HistoryFoldIndex::new(),
                    unit_flags: protocol::HistoryFoldIndex::new(),
                })));
            }

            if let Some(owner)=publication.lane_insert.as_mut(){let(slot,progress)=owner.commit_controlled(&mut store.log.envelope.lanes,lane_grant,|lane|lane).expect("preflight validated the unchanged original lane map");assert!(slot.is_some(),"preflight owns the exact unchanged vacant lane rank");lane_progress=progress;}
            store.cursor.generation += 1;
            store.cursor.content_revision = store.cursor.revision_accumulator.revision(store.log.current_checkpoint_id.as_deref());
            assert!(store.owners.snapshot_read_leases.publish_authority(store.cursor.generation, store.cursor.content_revision), "single-owner batched publication must advance its exact snapshot authority");
            store.cursor.last_projection_cause = Some(ArtifactProjectionCause::Apply);
            let receipt = LaneItemReceipt { generation_before, generation_after: store.cursor.generation };
            publication.expected_generation = store.cursor.generation;
            publication.expected_revision = store.cursor.content_revision;
            publication.receipt = Some(receipt);
            publication.published = true;
            publication.phase = ArtifactStoreOneItemPublicationPhase::AwaitingAck;
            Ok(ArtifactStoreOneItemAdvance::Published(receipt,RetainedCloneProgress{copied_items:diagnostic_receipt.copied_items.max(lane_progress.copied_items),copied_bytes:diagnostic_receipt.copied_bytes+lane_progress.copied_bytes,retained_capacity_bytes:diagnostic_receipt.retained_capacity_bytes+lane_progress.retained_capacity_bytes,released_bytes:diagnostic_receipt.released_bytes+lane_progress.released_bytes}))
        }
        ArtifactStoreOneItemPublicationPhase::Fault => Ok(ArtifactStoreOneItemAdvance::PreparationRefused),
        ArtifactStoreOneItemPublicationPhase::AwaitingAck | ArtifactStoreOneItemPublicationPhase::Closing | ArtifactStoreOneItemPublicationPhase::Complete => unreachable!("handled batched terminal control phase"),
    }
}

/// 🧹️ Preserves the original preparation receipt before admitting its final owned Box frame.
fn spend_item_close_grant<P, Mutation>(owner: &mut Option<Box<dyn ArtifactStoreOneItemPreparation<P, Mutation>>>, grant: ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<RetainedCloneStep, ValueError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{
    use semio_framework_value::retained_clone::{RetainedCloneStep, RetainedCloneProgress};
    let Some(current) = owner.as_mut() else { return Ok(RetainedCloneStep::Complete(Default::default())); };
    if !current.terminal_is_empty() {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let depth = current.next_close_depth_demand()?.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "batch preparation Box depth overflow"))?;
        if grant.maximum_depth < depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "batch preparation exceeds admitted Box depth")); }
        let child_grant = ArtifactStoreOneItemGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        let step=receiving.with_retirement(child_grant.retained_grant(),|retirement|retirement.with_original_close(|original|current.close_original_step(original,child_grant)))?;
        let step = semio_framework_value::retained_clone::admit_retained_clone_close(child_grant.retained_grant(), step, current.terminal_is_empty(), "batch original item preparation")?;
        return Ok(RetainedCloneStep::Progress(step.progress()));
    }
    if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
    if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "batch preparation Box requires admitted depth")); }
    let bytes = size_of_val(current.as_ref());
    if bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
    drop(owner.take());
    Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }))
}
//#endregion 📬️OneItemPublication

/// 🕸️ Feeds a remote {@link MutationEnvelope} through the causal DAG, applying it (and any
/// now-unblocked dependents) into the edit timeline. Closes the sync gap between
/// `framework/sync`'s `MutationDag` and the vcs edit history. Sole public remote write gate —
/// parallel to `dispatch` for causal envelopes. Implements the ticket's 9-step algorithm
/// (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6): buffer in the
/// dag, drain whatever it (and anything it unblocks) makes ready, HLC-sort that batch into
/// `applied_edit_ids`, replay only the divergent suffix, then either commit atomically (dag,
/// history, ledger, `applied_edit_ids` all move together) or quarantine the whole batch as an
/// `Open` `Conflict` — state and the dag's own applied-set never move on rejection.
///
/// 1
///
/// 2
///
/// 3 — an edit's HLC is its first forward op's stamped meta timestamp.
///
/// 🌀️ `HybridLogicalTimestamp::cmp_key` is async (📡️replication); `sort_by_key`/
/// `partition_point`/`position` all need sync predicates (R10 shape 1), so every key below
/// is resolved via an explicit `.await` first, then compared as a plain `(u64, u64, u64)`.
///
/// Binary search replicating `partition_point`'s algorithm (assumes `applied_edit_ids` is
/// already HLC-sorted, exactly as `partition_point` itself would have assumed).
///
/// 4 — stable HLC merge of `batch` into `applied_edit_ids[k..]`.
///
/// 5
///
/// 6 — walk order[k..] one edit at a time (H1 determinism fix): an already-committed edit
/// that this rewind proves invalid gets the SAME accept/quarantine verdict a fresh arrival
/// in true HLC order would have given it, instead of the whole suffix being judged
/// atomically. See `replay_suffix_partitioned`'s doc for why this makes the merge a pure
/// function of the envelope SET and `self.merge_policy`, never of arrival order.
///
/// 7
///
/// 🎯️ MEDIUM-3: `degraded_ids` only needs `committed_ids`/`replayed` (both already computed
/// above, before any mutation) — hoisted from below `# 9` so the open-conflict capacity check
/// can see BOTH conflicts this call might mint before touching any store field, keeping the
/// whole call atomic on a refusal exactly like a policy rejection already is.
/// 🌀️ `worst_level` is async; `Iterator::filter`'s closure is sync (R10 shape 1), so it's
/// hoisted into an explicit loop.
///
/// 8 — quarantine: every edit whose OWN outcome the policy rejects — whether newly-arrived
/// this call or retroactively invalidated by this rewind — is pulled out of history. Its
/// forward ops/`MutationMeta` are left untouched wherever they already live in `vcs.edits`
/// (never rewritten, exactly like `Undo`); only its id drops out of `applied_edit_ids`, so
/// a future `resolve_conflict`/redelivery still finds its established payload.
///
/// 🎯️ HIGH-2: `edits_for_ids` errors loudly instead of silently filtering — see its doc.
///
/// 🌀️ The unawaited future would borrow `envelopes`, which is moved into `kind` below —
/// awaited immediately instead of deferred to avoid a move-while-borrowed (E0505).
///
/// 🌀️ `mutation_ids_for_edit` is async (📡️replication); `flat_map`'s closure is sync
/// (R10 shape 1), so it's hoisted into an explicit loop.
///
/// 🎯️ H1: `hlc`/`timestamp` come from the quarantined edits' OWN stamped HLCs, never
/// `self.clock` — `self.clock.merge` ticks its `logical` counter on every call, so its
/// running value depends on how many merges happened and in what order, not just on
/// this envelope set; hashing that into `ConflictId` reintroduced arrival-order
/// dependence even after state/`applied_edit_ids` converged.
/// 🌀️ `HybridLogicalTimestamp::cmp_key` is async; `max_by_key` needs a sync key extractor
/// (R10 shape 1), so the max is found by an explicit fold over resolved keys instead.
///
/// `>=` (not `>`): `Iterator::max_by_key`'s documented tie-break keeps the LAST
/// equally-maximum element, so ties must still overwrite.
///
/// 9 — commit the survivors: history, ledger, `applied_edit_ids`, `current` together. The
/// dag (decided right after the capacity check above) only advanced when nothing in this
/// suffix was quarantined, so a quarantined-but-not-yet-committed envelope can still be
/// retried on redelivery.
///
/// 🎯️ HIGH-2: `edits_for_ids` errors loudly instead of silently filtering — see its doc.
/// Reads from `edits_by_id` (not `self.envelope.vcs.edits`): both already contain every
/// `degraded_ids` entry at this point, but `edits_by_id` doesn't depend on the `vcs.edits`
/// push above having already run, so this block's own correctness never depends on
/// sequencing against that mutation.
///
/// 🌀️ `mutation_ids_for_edit` is async (📡️replication); `flat_map`'s closure is sync
/// (R10 shape 1), so it's hoisted into an explicit loop.
///
/// 🎯️ H1: same reasoning as the quarantine conflict above — derive from the degraded
/// edits' own HLCs, not the arrival-order-dependent `self.clock`.
/// 🌀️ `HybridLogicalTimestamp::cmp_key` is async; `max_by_key` needs a sync key extractor
/// (R10 shape 1), so the max is found by an explicit fold over resolved keys instead.
///
/// `>=`: `Iterator::max_by_key` keeps the LAST equally-maximum element on a tie.
pub async fn ingest_remote_merge<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>, envelope: semio_framework_os_spr::MutationEnvelope) -> Result<semio_framework_os_spr::MergeReport, VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{
    ensure_durable_group_idle(store)?;
    let no_op_report = |policy: semio_framework_os_spr::MergePolicy, insertion_index: usize| semio_framework_os_spr::MergeReport { policy, accepted: true, insertion_index: insertion_index as u32, replayed: Vec::new(), worst: None, conflict: None };
    store.owners.displaced_retirements.reserve(2)?;
    let mut candidate_dag = (*store.log.dag).clone();
    let insertion = match candidate_dag.insert(envelope.clone()) {
        Ok(insertion) => insertion,
        Err(rejected) => {
            store.owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreConflictRetirement::causal_envelope(rejected.envelope)));
            store.owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreMutationDagRetirement::new(candidate_dag)));
            return Err(VcsError::Backbone(rejected.error.to_string()));
        }
    };
    if let semio_framework_os_spr::InsertResult::AlreadyApplied(rejected) = insertion {
        let equivalent = assert_equivalent_remote_mutation(store, &envelope);
        store.owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreConflictRetirement::causal_envelope(rejected)));
        store.owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreMutationDagRetirement::new(candidate_dag)));
        equivalent?;
        return Ok(no_op_report(store.log.merge_policy, store.log.applied_edit_ids.len()));
    }
    let mut batch: Vec<Edit<Mutation>> = Vec::new();
    let mut ready_transitions: Vec<semio_framework_os_spr::MutationEnvelope> = Vec::new();
    loop {
        let ready_envelope = match candidate_dag.take_next_applied() {
            semio_framework_os_spr::MutationDagAppliedStep::Envelope(envelope) => envelope,
            semio_framework_os_spr::MutationDagAppliedStep::SeededIdentity => continue,
            semio_framework_os_spr::MutationDagAppliedStep::Complete => break,
        };
        if semio_framework_os_spr::is_history_transition(&ready_envelope) {
            ready_transitions.push(ready_envelope);
            continue;
        }
        let admitted = match edit_from_operation_envelope_cold::<Mutation>(&ready_envelope).await {
            Ok(mut edit) => {
                edit.actor = Some(ready_envelope.actor.0.clone());
                stamp_edit_semantics::<P, Mutation>(&mut edit, &store.log.envelope.schema);
                match store.log.envelope.vcs.edits.find_near(|existing| existing.id == edit.id) {
                    Some(existing) => {
                        let verdict = assert_equivalent_remote_envelope(store, existing, &ready_envelope).map(|()| None);
                        retire_scratch_edits::<P, Mutation>([edit]);
                        verdict
                    }
                    None if batch.iter().any(|existing: &Edit<Mutation>| existing.id == edit.id) => {
                        let error = VcsError::ValidationFailed(format!("remote ingest repeats authoritative edit {}", edit.id));
                        retire_scratch_edits::<P, Mutation>([edit]);
                        Err(error)
                    }
                    None => Ok(Some(edit)),
                }
            }
            Err(error) => Err(error),
        };
        let edit = match admitted {
            Ok(Some(edit)) => edit,
            Ok(None) => continue,
            Err(error) => {
                retire_scratch_edits::<P, Mutation>(batch);
                return refuse_with_candidate_dag(store, candidate_dag, error);
            }
        };
        store.log.clock.merge(&ready_envelope.timestamp);
        batch.push(edit);
    }
    if batch.is_empty() {
        if ready_transitions.is_empty() {
            replace_causal_dag_retained(store, candidate_dag)?;
            return Ok(no_op_report(store.log.merge_policy, store.log.applied_edit_ids.len()));
        }
        store.cursor.last_projection_cause = None;
        let replayed = match admit_remote_transitions(store, ready_transitions).await {
            Ok(replayed) => replayed,
            Err(error) => return refuse_with_candidate_dag(store, candidate_dag, error),
        };
        replace_causal_dag_retained(store, candidate_dag)?;
        store.replay.pending_report.declare_edit_ids(Vec::new());
        bump(store)?;
        store.cursor.last_projection_cause = Some(store.cursor.last_projection_cause.filter(|cause| *cause == ArtifactProjectionCause::Replay).unwrap_or(ArtifactProjectionCause::RemoteIngest));
        let worst = replayed.iter().flat_map(|entry| entry.messages.iter()).map(|message| message.level).max();
        return Ok(semio_framework_os_spr::MergeReport { policy: store.log.merge_policy, accepted: true, insertion_index: store.log.applied_edit_ids.len() as u32, replayed, worst, conflict: None });
    }
    let edit_hlc = |edit: &Edit<Mutation>| edit.mutation_meta.first().map_or_else(|| HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 }, |meta| meta.timestamp);
    batch.sort_by_key(|edit| edit_hlc(edit).cmp_key());
    let batch_keys: Vec<(u64, u64, u64)> = batch.iter().map(|edit| edit_hlc(edit).cmp_key()).collect();
    let known_hlc = |edit_id: &str, edits: &ArtifactHistoryLedger<Edit<Mutation>>| edits.find_near(|edit| edit.id == *edit_id).map(edit_hlc);
    let min_batch_key = batch_keys[0];
    let mut lo = 0usize;
    let mut hi = store.log.applied_edit_ids.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let candidate_key = known_hlc(&store.log.applied_edit_ids[mid], &store.log.envelope.vcs.edits).map(|hlc| hlc.cmp_key());
        let still_before = candidate_key.is_none_or(|key| key < min_batch_key);
        if still_before {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    let mut k = lo;
    for edit in &batch {
        for dependency in edit.mutation_meta.first().map(|meta| meta.dependencies.clone()).unwrap_or_default() {
            if let Some(position) = store.log.applied_edit_ids.iter().position(|id| *id == dependency.0) {
                k = k.max(position + 1);
            }
        }
    }
    let mut order: Vec<String> = store.log.applied_edit_ids.to_vec();
    for (edit, &hlc_key) in batch.iter().zip(batch_keys.iter()) {
        let mut insert_at = order.len();
        for offset in k..order.len() {
            let existing_key = known_hlc(&order[offset], &store.log.envelope.vcs.edits).map(|existing| existing.cmp_key());
            if existing_key.is_some_and(|key| key > hlc_key) {
                insert_at = offset;
                break;
            }
        }
        order.insert(insert_at, edit.id.clone());
    }
    let base = if k == store.log.applied_edit_ids.len() {
        Arc::clone(&*store.live.current)
    } else {
        match fold_history(&store.log.envelope, &order[..k], &store.live.supersessions).await {
            Ok(base) => Arc::new(base),
            Err(error) => {
                retire_scratch_edits::<P, Mutation>(batch);
                return refuse_with_candidate_dag(store, candidate_dag, error);
            }
        }
    };
    let replay = StoreReplay::<P, Mutation>::replay_suffix_partitioned(base, &order, k, &OriginalReplayEdits::new(&store.log.envelope.vcs.edits,&batch), &store.log.envelope.schema, &store.live.supersessions, store.log.merge_policy);
    let (state, committed_ids, quarantined_ids, rebased_inverse, replayed) = match replay {
        Ok(mut replay) => {
            let taken = (replay.take_state().expect("a finished merge replay holds its projection"), replay.take_committed(), replay.take_quarantined(), replay.take_rebased_undo_rows(), replay.take_replayed());
            let _ = retire_replay_result(store, replay);
            taken
        }
        Err(error) => {
            retire_scratch_edits::<P, Mutation>(batch);
            return refuse_with_candidate_dag(store, candidate_dag, error);
        }
    };
    let document_id = ArtifactId(store.log.envelope.id.clone());
    let schema = SchemaId(store.log.envelope.schema.clone());
    let mut degraded_ids: Vec<String> = Vec::new();
    for id in &committed_ids {
        let is_degraded = match replayed.iter().find(|entry| &entry.edit_id == id) {
            Some(entry) => semio_framework_os_spr::worst_level(&entry.messages).is_some_and(|level| level >= semio_framework_diagnostic::Severity::Warning),
            None => false,
        };
        if is_degraded {
            degraded_ids.push(id.clone());
        }
    }
    if let Err(error) = ensure_open_conflict_capacity(store, usize::from(!quarantined_ids.is_empty()) + usize::from(!degraded_ids.is_empty())) {
        retire_shared_projection::<P, Mutation>(state);
        retire_scratch_edits::<P, Mutation>(batch);

        for (_, inverse) in rebased_inverse {
            retire_scratch_operations::<P, Mutation>(inverse);
        }
        return refuse_with_candidate_dag(store, candidate_dag, error);
    }
    if quarantined_ids.is_empty() {
        replace_causal_dag_retained(store, candidate_dag)?;
    } else {
        store.owners.displaced_retirements.reserve(1)?;
        store.owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreMutationDagRetirement::new(candidate_dag)));
    }
    let mut quarantine_conflict_id = None;
    if !quarantined_ids.is_empty() {
        let  original_edits=OriginalReplayEdits::new(&store.log.envelope.vcs.edits,&batch);
        let quarantined_edits = original_edits_for_ids(&quarantined_ids, &original_edits)?;
        let mut envelopes: Vec<semio_framework_os_spr::MutationEnvelope> = Vec::new();
        for edit in &quarantined_edits {
            envelopes.extend(semio_framework_os_spr::mutation_envelope_from_edit::<P, Mutation>(edit, &document_id, &schema).map_err(|error| VcsError::Serialize(error.to_string()))?);
        }
        let conflict_actors = canonical_conflict_actors(envelopes.iter().map(|envelope| envelope.actor.clone())).await;
        let mut quarantine_mutation_ids: Vec<MutationId> = Vec::new();
        for edit in &quarantined_edits {
            quarantine_mutation_ids.extend(semio_framework_os_spr::mutation_ids_for_edit(edit));
        }
        let quarantine_messages = conflict_messages_for_edits(&quarantined_edits, &replayed).await?;
        let kind = semio_framework_os_spr::ConflictKind::Quarantined { envelopes };
        let mut conflict_hlc = store.log.clock;
        let mut conflict_hlc_key: Option<(u64, u64, u64)> = None;
        for edit in &quarantined_edits {
            let hlc = edit_hlc(edit);
            let key = hlc.cmp_key();
            if conflict_hlc_key.is_none_or(|current| key >= current) {
                conflict_hlc_key = Some(key);
                conflict_hlc = hlc;
            }
        }
        let id = semio_framework_os_spr::ConflictId::new(&kind, &ArtifactId(store.log.envelope.id.clone()), &quarantine_mutation_ids, &conflict_hlc).await;
        store.log.envelope.conflicts.push(semio_framework_os_spr::Conflict { id: id.clone(), kind, status: semio_framework_os_spr::ConflictStatus::Open, messages: quarantine_messages, actors: conflict_actors, timestamp: conflict_hlc });
        prune_resolved_conflicts(store)?;
        for quarantined_id in &quarantined_ids {
            replace_edit_messages(store, quarantined_id, Vec::new())?;
        }

        quarantine_conflict_id = Some(id);
    }

    let mut degraded_conflict_id = None;
    if !degraded_ids.is_empty() {
        let original_edits=OriginalReplayEdits::new(&store.log.envelope.vcs.edits,&batch);
        let degraded_edits = original_edits_for_ids(&degraded_ids, &original_edits)?;
        let degraded_messages = conflict_messages_for_edits(&degraded_edits, &replayed).await?;
        let degraded_actors = canonical_conflict_actors(degraded_edits.iter().filter_map(|edit| edit.actor.clone()).map(ActorId)).await;
        let mut degraded_mutation_ids: Vec<MutationId> = Vec::new();
        for edit in &degraded_edits {
            degraded_mutation_ids.extend(semio_framework_os_spr::mutation_ids_for_edit(edit));
        }
        let kind = semio_framework_os_spr::ConflictKind::Degraded { edit_ids: degraded_ids.clone() };
        let mut conflict_hlc = store.log.clock;
        let mut conflict_hlc_key: Option<(u64, u64, u64)> = None;
        for edit in &degraded_edits {
            let hlc = edit_hlc(edit);
            let key = hlc.cmp_key();
            if conflict_hlc_key.is_none_or(|current| key >= current) {
                conflict_hlc_key = Some(key);
                conflict_hlc = hlc;
            }
        }
        let id = semio_framework_os_spr::ConflictId::new(&kind, &ArtifactId(store.log.envelope.id.clone()), &degraded_mutation_ids, &conflict_hlc).await;
        store.log.envelope.conflicts.push(semio_framework_os_spr::Conflict { id: id.clone(), kind, status: semio_framework_os_spr::ConflictStatus::Open, messages: degraded_messages, actors: degraded_actors, timestamp: conflict_hlc });
        prune_resolved_conflicts(store)?;
        degraded_conflict_id = Some(id);
    }
    let batch_ids:Vec<String>=batch.iter().map(|edit|edit.id.clone()).collect();
    for edit in batch {
        if committed_ids.contains(&edit.id) && !store.log.envelope.vcs.edits.iter().any(|existing| existing.id == edit.id) {
            let reservation = reserve_edit_history_slot(store)?;
            insert_reserved_edit_history(store, reservation, edit)?;
        }else{retire_scratch_edits::<P,Mutation>([edit]);
        }
    }
    for (edit_id, inverse) in rebased_inverse {
        match store.log.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id) {
            Some(edit) => retire_scratch_operations::<P, Mutation>(std::mem::replace(&mut *edit.inverse, inverse)),
            None => retire_scratch_operations::<P, Mutation>(inverse),
        }
    }
    replace_applied_edit_ids_retained(store, store.log.applied_edit_ids.iter().take(k).cloned().chain(committed_ids.iter().cloned()).collect())?;
    for edit_messages in &replayed {
        if committed_ids.contains(&edit_messages.edit_id) {
            replace_edit_messages(store, &edit_messages.edit_id, edit_messages.messages.clone())?;
        }
    }
    let positions: HashMap<&str, usize> = store.log.applied_edit_ids.iter().enumerate().map(|(index, edit_id)| (edit_id.as_str(), index)).collect();
    for edit in store.log.envelope.vcs.edits.iter_mut() {
        if let Some(index) = positions.get(edit.id.as_str()) {
            edit.sequence_number = *index as i32 + 1;
        }
    }
    drop(positions);
    store.log.edit_sequence = store.log.applied_edit_ids.len() as i32;
    replace_tail_undo_cache_retained(store, None)?;
    replace_current_retained(store, state)?;
    store.cursor.revision_dirty_from = Some(store.cursor.revision_dirty_from.map_or(k, |dirty| dirty.min(k)));
    store.cursor.last_projection_cause = None;
    let mut replayed = replayed;
    if !ready_transitions.is_empty() || !store.log.envelope.transitions.is_empty() {
        replayed.extend(admit_remote_transitions(store, ready_transitions).await?);
    }
    let worst = replayed.iter().flat_map(|edit_messages| edit_messages.messages.iter()).map(|message| message.level).max();
    store.replay.pending_report.declare_edit_ids(batch_ids.iter().filter(|id| committed_ids.contains(id)).cloned().collect());
    bump(store)?;
    store.cursor.last_projection_cause = Some(store.cursor.last_projection_cause.filter(|cause| *cause == ArtifactProjectionCause::Replay).unwrap_or(ArtifactProjectionCause::RemoteIngest));
    let accepted = batch_ids.iter().all(|id| committed_ids.contains(id));
    Ok(semio_framework_os_spr::MergeReport { policy: store.log.merge_policy, accepted, insertion_index: k as u32, replayed, worst, conflict: quarantine_conflict_id.or(degraded_conflict_id) })
}

/// 🎯️ HIGH-2 (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` J1
/// robustness pass): strict counterpart to a `filter_map` over `edits` — a `ConflictId` is
/// content-addressed from a mutation-id set, so silently skipping an id missing from `edits`
/// (the old `filter_map` behaviour) can leave `quarantined_ids`/`degraded_ids` non-empty while
/// the resolved edit list is empty, minting a `ConflictId` from zero mutation ids — a content
/// address that addresses no content, so two unrelated conflicts can collide on it. `edits` is
/// always `edits_by_id` here, and every id passed in already came out of
/// `replay_suffix_partitioned`'s own `committed_ids`/`quarantined_ids` — which, since HIGH-1,
/// only ever contains ids it already found in `edits` — so this should never actually fail in
/// production; it exists so a future refactor that breaks that invariant fails loudly instead of
/// quietly minting a garbage conflict id.
pub fn edits_for_ids<Mutation: Clone>(ids: &[String], edits: &HashMap<String, Edit<Mutation>>) -> Result<Vec<Edit<Mutation>>, VcsError> {
    ids.iter().map(|id| edits.get(id).cloned().ok_or_else(|| VcsError::UnknownEdit(id.clone()))).collect()
}

//#region 🪞️BundleHelpers
#[inline]
async fn admit_remote_transitions<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>, transitions: Vec<semio_framework_os_spr::MutationEnvelope>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,

{ store.replay.admit_remote_transitions(&mut *store.log, &mut *store.owners, &mut *store.wire, &mut *store.cursor, &mut *store.live, transitions).await }

#[inline]
fn assert_equivalent_remote_envelope<P, Mutation>(store: &StoreParts<'_, P, Mutation>, existing: &Edit<Mutation>, incoming: &semio_framework_os_spr::MutationEnvelope) -> Result<(), VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.log.assert_equivalent_remote_envelope(existing, incoming) }

#[inline]
fn assert_equivalent_remote_mutation<P, Mutation>(store: &StoreParts<'_, P, Mutation>, incoming: &semio_framework_os_spr::MutationEnvelope) -> Result<(), VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.log.assert_equivalent_remote_mutation(incoming) }

#[inline]
fn batch_amend_target<P, Mutation>(store: &StoreParts<'_, P, Mutation>, transaction: Option<&protocol::TransactionRef>) -> Option<String>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.log.batch_amend_target(transaction) }

#[inline]
fn batch_amend_target_ref<'s, P, Mutation>(store: &'s StoreParts<'_, P, Mutation>, transaction: Option<&protocol::TransactionRef>) -> Option<&'s str>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.log.batch_amend_target_ref(transaction) }

#[inline]
fn batch_item_base<P, Mutation>(store: &StoreParts<'_, P, Mutation>, staged_root: Option<&Arc<P>>) -> Result<SnapshotRead<P>, VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.live.batch_item_base(&*store.owners, &*store.wire, &*store.cursor, staged_root) }

#[inline]
fn bump<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>) -> Result<(), VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.replay.bump(&mut *store.log, &mut *store.owners, &mut *store.cursor, &mut *store.live) }

#[inline]
fn continue_pending_replay_across_append<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>, before: [u8; 32], edit_id: &str, appended: usize)
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.replay.continue_pending_replay_across_append(&mut *store.cursor, before, edit_id, appended) }

#[inline]
fn ensure_durable_group_idle<P, Mutation>(store: &StoreParts<'_, P, Mutation>) -> Result<(), VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.wire.ensure_durable_group_idle() }

#[inline]
fn ensure_open_conflict_capacity<P, Mutation>(store: &StoreParts<'_, P, Mutation>, additional_open: usize) -> Result<(), VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.log.ensure_open_conflict_capacity(additional_open) }

#[inline]
fn fold_batch_item<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>, publication: &mut ArtifactStoreBatchPublication<P, Mutation>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemAdvance, VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.owners.fold_batch_item(&mut *store.log, publication, grant) }

#[inline]
fn insert_reserved_edit_history<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>, reservation: ArtifactStoreHistoryCommitReservation, edit: Edit<Mutation>) -> Result<ArtifactHistoryKey, VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.live.insert_reserved_edit_history(&mut *store.log, &mut *store.owners, reservation, edit) }

#[inline]
fn prune_resolved_conflicts<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>) -> Result<(), VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.owners.prune_resolved_conflicts(&mut *store.log) }

#[inline]
fn refuse_with_candidate_dag<T, P, Mutation>(store: &mut StoreParts<'_, P, Mutation>, candidate_dag: semio_framework_os_spr::MutationDag, error: VcsError) -> Result<T, VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.owners.refuse_with_candidate_dag::<T>(candidate_dag, error) }

#[inline]
fn replace_applied_edit_ids_retained<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>, next: Vec<String>) -> Result<(), VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.live.replace_applied_edit_ids_retained(&mut *store.log, &mut *store.owners, &mut *store.cursor, next) }

#[inline]
fn replace_causal_dag_retained<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>, next: semio_framework_os_spr::MutationDag) -> Result<(), VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.owners.replace_causal_dag_retained(&mut *store.log, next) }

#[inline]
fn replace_current_retained<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>, next: Arc<P>) -> Result<(), VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.live.replace_current_retained(&mut *store.log, &mut *store.owners, next) }

#[inline]
fn replace_edit_messages<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>, edit_id: &str, messages: Vec<semio_framework_os_spr::MutationMessage>) -> Result<(), VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.owners.replace_edit_messages(&mut *store.log, edit_id, messages) }

#[inline]
fn replace_pending_report_retained<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>, next: PendingCommandReport) -> Result<(), VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.replay.replace_pending_report_retained(&mut *store.owners, next) }

#[inline]
fn replace_redo_edit_ids_retained<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>, next: Vec<String>) -> Result<(), VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.live.replace_redo_edit_ids_retained(&mut *store.log, &mut *store.owners, &mut *store.cursor, next) }

#[inline]
fn replace_tail_undo_cache_retained<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>, next: Option<(String, Arc<P>)>) -> Result<(), VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.live.replace_tail_undo_cache_retained(&mut *store.log, &mut *store.owners, next) }

#[inline]
fn reserve_edit_history_slot<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>) -> Result<ArtifactStoreHistoryCommitReservation, VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.owners.reserve_edit_history_slot(&mut *store.log) }

#[inline]
fn retire_replay_result<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>, result: EditReplayResult<P, Mutation>) -> Result<(), VcsError>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{ store.owners.retire_replay_result(result) }
//#endregion 🪞️BundleHelpers

