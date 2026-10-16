//! 🔎️ Checkpoint verification: preflight completion, publication frontiers, checkpoint publication and verification.

use semio_hub_inference_runtime_contract::{APPROVAL_MAX_RECORDS, GisMapApprovalCheckpointRequestV1, GisMapApprovalCommitErrorV1, GisMapApprovalReceiptV1, GisMapCommitConflictArmV1, GisMapCommitIdentityV1, GisMapCommitOperationV1, GisMapJournalReceiptV1, GisMapParentSnapshotV1, gis_map_commit_conflict, gis_map_storage_refusal, record_prepare_conflict};
use std::sync::Arc;
use directory::os_directory::{ArtifactFrontier, DocumentScope};
use semio_hub_inference_binding::ComposedMapCommitBindingV1;
use semio_hub_inference::{InferenceErrorV1, InferenceOperationControlV1, sha256};
use semio_hub_inference::wal::InferenceWalTargetV1;
use crate::{RetainedGisMapApprovalCommitterV1, RetainedGisMapDocumentStateV1};

impl<B: ComposedMapCommitBindingV1> RetainedGisMapApprovalCommitterV1<B> {
    pub(super) async fn finish_preflight(&self, key: &str, candidate: &GisMapCommitIdentityV1, snapshot: &GisMapParentSnapshotV1<B>, observed: &db::CheckpointPublicationSnapshot) -> Result<(), GisMapApprovalCommitErrorV1> {
        if !Self::checkpoint_matches_frontier(observed, observed.authority_generation, &candidate.base_frontier, &candidate.scope) {
            return Err(gis_map_commit_conflict(GisMapCommitConflictArmV1::PreflightCheckpointDiffers));
        }
        let mut documents = self.documents.lock().await;
        let Some(state) = documents.remove(key) else { return Err(GisMapApprovalCommitErrorV1::Unavailable) };
        let RetainedGisMapDocumentStateV1::Ready { owners, pending: Some(identity), document_write } = state else {
            documents.insert(key.to_owned(), state);
            return Ok(());
        };
        if !Self::identity_matches(&identity, candidate) || owners.generation != observed.authority_generation || !(Self::stores_match(&owners, snapshot) || Self::stores_consistent(&owners)) {
            record_prepare_conflict(&format!(
                "preflight/identity={} generation={} stores_match={} stores_consistent={}",
                Self::identity_matches(&identity, candidate),
                owners.generation == observed.authority_generation,
                Self::stores_match(&owners, snapshot),
                Self::stores_consistent(&owners),
            ));
            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: Some(identity), document_write });
            return Err(gis_map_commit_conflict(GisMapCommitConflictArmV1::PreflightIdentityOrStoresDiffer));
        }
        let Some(document_write) = document_write else {
            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: Some(identity), document_write: None });
            return Err(gis_map_commit_conflict(GisMapCommitConflictArmV1::PreflightWriterMissing));
        };
        match self.build_assembly(owners, identity, snapshot, document_write) {
            Ok(state) => {
                documents.insert(key.to_owned(), state);
                Ok(())
            }
            Err((error, owners)) => {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: None, document_write: None });
                Err(error)
            }
        }
    }

    pub fn publication_frontier(scope: &DocumentScope, snapshot: &db::CheckpointPublicationSnapshot) -> Option<ArtifactFrontier> {
        Some(ArtifactFrontier {
            document_id: scope.document_id.clone(),
            head_edit_ordinal: snapshot.frontier.head_seq,
            head_edit_id: snapshot.head_edit_id.as_ref()?.0.clone(),
            last_commit_seq: snapshot.frontier.commit_seq,
            chain_hash: directory::os_directory::ArtifactHash(snapshot.frontier.chain_hash),
        })
    }

    pub(super) fn publication_wire_frontier(frontier: &ArtifactFrontier) -> directory::os_directory::EditedArtifactFrontierV1 {
        directory::os_directory::EditedArtifactFrontierV1 {
            document_id: frontier.document_id.clone(),
            head_edit_ordinal: frontier.head_edit_ordinal,
            head_edit_id: frontier.head_edit_id.clone(),
            last_commit_seq: frontier.last_commit_seq,
            chain_sha256: directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&frontier.chain_hash),
        }
    }

    async fn publish_checkpoint(
        &self,
        key: &str,
        identity: &GisMapCommitIdentityV1,
        generation: u64,
        receipt: &GisMapJournalReceiptV1,
        attempt_lifetime_ms: u64,
        decision_now_ms: u64,
    ) -> Result<directory::os_directory::PublishedArtifactCheckpoint, GisMapApprovalCommitErrorV1> {
        let (handle, scope) = {
            let documents = self.documents.lock().await;
            let Some(RetainedGisMapDocumentStateV1::Publishing { owners, identity: stored, receipt: stored_receipt, document_write: _ }) = documents.get(key) else {
                return Err(gis_map_commit_conflict(GisMapCommitConflictArmV1::PublishStateMissing));
            };
            if !Self::identity_matches(stored, identity) || stored_receipt != receipt || owners.generation != generation {
                return Err(gis_map_commit_conflict(GisMapCommitConflictArmV1::PublishIdentityDiffers));
            }
            (owners.handle.clone(), owners.scope.clone())
        };
        let actor_snapshot = handle.checkpoint_publication_snapshot().await.map_err(|error| gis_map_storage_refusal(&error))?;
        let (request, document_write) = {
            let documents = self.documents.lock().await;
            let Some(RetainedGisMapDocumentStateV1::Publishing { owners, identity: stored, receipt: stored_receipt, document_write }) = documents.get(key) else {
                return Err(gis_map_commit_conflict(GisMapCommitConflictArmV1::PublishStateMissingAfterSnapshot));
            };
            if !Self::identity_matches(stored, identity) || stored_receipt != receipt || owners.generation != generation || owners.scope != scope {
                return Err(gis_map_commit_conflict(GisMapCommitConflictArmV1::PublishIdentityDiffersAfterSnapshot));
            }
            let projected = Self::publication_frontier(&owners.scope, &actor_snapshot).ok_or_else(|| gis_map_storage_refusal(&"absent"))?;
            if actor_snapshot.authority_generation != generation
                || projected.head_edit_ordinal != identity.base_frontier.head_edit_ordinal.checked_add(1).ok_or(GisMapApprovalCommitErrorV1::Capacity)?
                || projected.last_commit_seq != identity.base_frontier.last_commit_seq.checked_add(1).ok_or(GisMapApprovalCommitErrorV1::Capacity)?
                || projected.head_edit_id != identity.mutation_id
            {
                return Err(gis_map_commit_conflict(GisMapCommitConflictArmV1::PublishProjectedFrontierDiffers));
            }
            let files = ::semio_framework_async::poll::resolve_ready(owners.parent.as_ref().ok_or_else(|| gis_map_storage_refusal(&"absent"))?.snapshot_pack()).map_err(|error| gis_map_storage_refusal(&error))?;
            if files.pack.is_empty() || files.spr.is_empty() {
                return Err(gis_map_storage_refusal(&"refused"));
            }
            (
                GisMapApprovalCheckpointRequestV1 {
                    scope: owners.scope.clone(),
                    descriptor_digest: identity.descriptor_digest.clone(),
                    base_frontier: identity.base_frontier.clone(),
                    handle: owners.handle.clone(),
                    actor_snapshot,
                    pair: semio_hub_artifact_authority::ArtifactPair { pack: files.pack, spr: files.spr },
                    journal_receipt: receipt.clone(),
                },
                document_write.clone(),
            )
        };
        let expected_scope = request.scope.clone();
        let expected_descriptor = identity.descriptor_digest.clone();
        let expected_frontier = Self::publication_frontier(&expected_scope, &request.actor_snapshot).ok_or_else(|| gis_map_storage_refusal(&"absent"))?;
        let expected_pack_hash = sha256(&request.pair.pack);
        let expected_spr_hash = sha256(&request.pair.spr);
        let published = self.publisher.publish(request, document_write, attempt_lifetime_ms, decision_now_ms).await?;
        if published.scope != expected_scope
            || directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&published.descriptor_digest_v1) != expected_descriptor
            || published.baseline_frontier != expected_frontier
            || directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&published.pack.sha256) != expected_pack_hash
            || directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&published.spr.sha256) != expected_spr_hash
        {
            return Err(gis_map_commit_conflict(GisMapCommitConflictArmV1::PublishResponseDiffers));
        }
        Ok(published)
    }

    pub(super) async fn verify(&self, key: &str, identity: &GisMapCommitIdentityV1, generation: u64, receipt: GisMapJournalReceiptV1, attempt_lifetime_ms: u64, decision_now_ms: u64) -> Result<GisMapApprovalReceiptV1, GisMapApprovalCommitErrorV1> {
        let control = Arc::new(InferenceOperationControlV1::new(attempt_lifetime_ms, 65_536).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?);
        let fence = {
            let documents = self.documents.lock().await;
            match documents.get(key) {
                Some(RetainedGisMapDocumentStateV1::Verification { owners, identity: stored, receipt: stored_receipt, .. })
                | Some(RetainedGisMapDocumentStateV1::Publishing { owners, identity: stored, receipt: stored_receipt, .. })
                | Some(RetainedGisMapDocumentStateV1::Published { owners, identity: stored, receipt: stored_receipt, .. })
                    if owners.generation == generation && Self::identity_matches(stored, identity) && stored_receipt == &receipt =>
                {
                    owners.fence.clone()
                }
                _ => return Err(gis_map_commit_conflict(GisMapCommitConflictArmV1::VerifyFenceStateDiffers)),
            }
        };
        let target = InferenceWalTargetV1 {
            scope: identity.scope.clone(),
            generation,
            job_id: identity.job_id.clone(),
            proposal_hash: identity.proposal_hash.clone(),
            mutation_id: identity.mutation_id.clone(),
            command_hash: identity.command_hash.clone(),
            actor: identity.actor.clone(),
            maximum_records: APPROVAL_MAX_RECORDS,
            receipt: receipt.clone(),
        };
        let witness = self
            .verifier
            .verify(target, fence, control)
            .await
            .map_err(|error| match error {
                InferenceErrorV1::Conflict | InferenceErrorV1::Invalid | InferenceErrorV1::Denied => gis_map_commit_conflict(GisMapCommitConflictArmV1::VerifyWalVerifier),
                other => gis_map_storage_refusal(&other),
            })?
            .ok_or_else(|| gis_map_storage_refusal(&"absent"))?;
        let already_published = {
            let mut documents = self.documents.lock().await;
            let Some(state) = documents.remove(key) else { return Err(GisMapApprovalCommitErrorV1::Unavailable) };
            match state {
                RetainedGisMapDocumentStateV1::Verification { owners, identity: stored, receipt: stored_receipt, document_write } if Self::identity_matches(&stored, identity) && stored_receipt == receipt => {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Publishing { owners, identity: stored, receipt: stored_receipt, document_write });
                    false
                }
                RetainedGisMapDocumentStateV1::Publishing { owners, identity: stored, receipt: stored_receipt, document_write } if Self::identity_matches(&stored, identity) && stored_receipt == receipt => {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Publishing { owners, identity: stored, receipt: stored_receipt, document_write });
                    false
                }
                RetainedGisMapDocumentStateV1::Published { owners, identity: stored, receipt: stored_receipt, document_write } if Self::identity_matches(&stored, identity) && stored_receipt == receipt => {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Published { owners, identity: stored, receipt: stored_receipt, document_write });
                    true
                }
                state => {
                    documents.insert(key.to_owned(), state);
                    return Err(gis_map_commit_conflict(GisMapCommitConflictArmV1::VerifyStateDiffersAfterWitness));
                }
            }
        };
        let published = if already_published { None } else { Some(self.publish_checkpoint(key, identity, generation, &receipt, attempt_lifetime_ms, decision_now_ms).await?) };
        let (after_frontier, after_base_digest) = if let Some(checkpoint) = published.as_ref() {
            (Self::publication_wire_frontier(&checkpoint.baseline_frontier), directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&checkpoint.pack.sha256))
        } else {
            let (handle, after_base_digest) = {
                let documents = self.documents.lock().await;
                match documents.get(key) {
                    Some(RetainedGisMapDocumentStateV1::Published { owners, .. }) => {
                        let files = ::semio_framework_async::poll::resolve_ready(owners.parent.as_ref().ok_or_else(|| gis_map_storage_refusal(&"absent"))?.snapshot_pack()).map_err(|error| gis_map_storage_refusal(&error))?;
                        (owners.handle.clone(), sha256(&files.pack))
                    }
                    _ => return Err(gis_map_commit_conflict(GisMapCommitConflictArmV1::VerifyPublishedStateDiffers)),
                }
            };
            let snapshot = handle.checkpoint_publication_snapshot().await.map_err(|error| gis_map_storage_refusal(&error))?;
            (Self::publication_wire_frontier(&Self::publication_frontier(&identity.scope, &snapshot).ok_or_else(|| gis_map_commit_conflict(GisMapCommitConflictArmV1::VerifyPublicationFrontierMissing))?), after_base_digest)
        };
        let (_newly_reconciled, undo) = match &identity.operation {
            GisMapCommitOperationV1::Approval => {
                let reconciliation = self.ledger.reconcile_committed_approval(&identity.job_id, &witness, generation, &after_frontier, &identity.descriptor_digest, &after_base_digest, identity.journal_now_ms).map_err(|error| match error {
                    InferenceErrorV1::Conflict | InferenceErrorV1::Invalid | InferenceErrorV1::Denied => gis_map_commit_conflict(GisMapCommitConflictArmV1::VerifyUndoVerifier),
                    other => gis_map_storage_refusal(&other),
                })?;
                (reconciliation.applied, reconciliation.undo)
            }
            GisMapCommitOperationV1::Undo { target_id, .. } => {
                let applied = self.ledger.reconcile_committed_gis_map_approval_undo(target_id, &witness, generation, &after_frontier).map_err(|error| match error {
                    InferenceErrorV1::Conflict | InferenceErrorV1::Invalid | InferenceErrorV1::Denied => gis_map_commit_conflict(GisMapCommitConflictArmV1::VerifyApplyVerifier),
                    other => gis_map_storage_refusal(&other),
                })?;
                (applied, semio_hub_inference::schema::GisMapApprovalUndoHandleV1 { target_id: target_id.clone(), expected_current: after_frontier.clone() })
            }
        };
        let mut documents = self.documents.lock().await;
        let Some(state) = documents.remove(key) else { return Err(GisMapApprovalCommitErrorV1::Unavailable) };
        match state {
            RetainedGisMapDocumentStateV1::Publishing { owners, identity: stored, receipt: stored_receipt, document_write } if Self::identity_matches(&stored, identity) && stored_receipt == receipt => {
                let notification = published.as_ref().map_or_else(|| Err(gis_map_commit_conflict(GisMapCommitConflictArmV1::VerifyCheckpointApplyMissing)), |checkpoint| match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.publisher.checkpoint_applied(checkpoint))) {
                    Ok(result) => result,
                    Err(_) => Err(gis_map_storage_refusal(&"checkpoint notification panicked")),
                });
                if let Err(error) = notification {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Publishing { owners, identity: stored, receipt: stored_receipt, document_write });
                    return Err(error);
                }
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Published { owners, identity: stored, receipt: stored_receipt, document_write: None });
                drop(document_write);
            }
            RetainedGisMapDocumentStateV1::Published { owners, identity: stored, receipt: stored_receipt, document_write } if Self::identity_matches(&stored, identity) && stored_receipt == receipt => {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Published { owners, identity: stored, receipt: stored_receipt, document_write: None });
                drop(document_write);
            }
            state => {
                documents.insert(key.to_owned(), state);
                return Err(gis_map_commit_conflict(GisMapCommitConflictArmV1::VerifyReceiptStateDiffers));
            }
        }
        Ok(GisMapApprovalReceiptV1 { witness, document_generation: generation, applied: true, undo, frontier: after_frontier })
    }
}
