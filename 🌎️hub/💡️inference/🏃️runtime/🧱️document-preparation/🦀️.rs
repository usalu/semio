//! 🧱️ Retained document preparation: mounting the three-store assembly and restoring the stores.

use std::sync::Arc;
use semio_hub_inference::command::CanonicalInferenceCommandV1;
use semio_hub_inference_binding::{ComposedMapCommitBindingV1, MapBindingRefusalV1};
use directory::os_directory::DocumentScope;
use semio_hub_inference_runtime_contract::{GisMapApprovalCommitErrorV1, GisMapApprovalRequestTokenV1, GisMapCommitIdentityV1, GisMapCommitOperationV1, GisMapDocumentStoresV1, GisMapDocumentWriteAuthorityV1, GisMapDocumentWriteLeaseV1, GisMapDrawingStoreV1, GisMapParentSnapshotV1, GisMapParentStoreV1, GisMapValueStoreV1, InferenceMapBaseV1, approval_mutation_id, document_key, gis_map_storage_refusal, record_prepare_conflict};
use semio_hub_inference::wal::InferenceDocumentFenceV1;
use crate::{RetainedGisMapApprovalCommitterV1, RetainedGisMapDocumentStateV1};
use semio_hub_inference::sha256;

impl<B: ComposedMapCommitBindingV1> RetainedGisMapApprovalCommitterV1<B> {
    pub(super) async fn prepare_retained_document(&self, scope: &DocumentScope, base: &InferenceMapBaseV1, gate: Arc<tokio::sync::Mutex<()>>, request: Arc<GisMapApprovalRequestTokenV1>, actor: &str) -> Result<(), GisMapApprovalCommitErrorV1> {
        let key = document_key(scope);
        let mut state_changes = self.state_epoch.subscribe();
        loop {
            enum PrepareTurn {
                Recover,
                PublishRecovered,
                Acquire,
                Ready,
                Wait,
            }
            let turn = {
                let documents = self.documents.lock().await;
                match documents.get(&key) {
                    Some(RetainedGisMapDocumentStateV1::Recovery { scope: stored, document_write, .. }) => {
                        if stored != scope || !Arc::ptr_eq(&document_write.gate, &gate) {
                            record_prepare_conflict("Recovery/scope-or-gate");
                            return Err(GisMapApprovalCommitErrorV1::Conflict);
                        }
                        if Self::request_matches(document_write, &request) {
                            PrepareTurn::Recover
                        } else {
                            PrepareTurn::Wait
                        }
                    }
                    Some(RetainedGisMapDocumentStateV1::Ready { owners, document_write, .. }) => {
                        let snapshot = B::decode_parent(base.pack.as_slice()).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?;
                        if owners.scope != *scope {
                            record_prepare_conflict("Ready/scope");
                            return Err(GisMapApprovalCommitErrorV1::Conflict);
                        }
                        if !Arc::ptr_eq(&owners.document_write, &gate) {
                            record_prepare_conflict("Ready/document_write");
                            return Err(GisMapApprovalCommitErrorV1::Conflict);
                        }
                        if !Self::stores_match(owners, &snapshot) {
                            record_prepare_conflict("Ready/stores_match");
                            return Err(GisMapApprovalCommitErrorV1::Conflict);
                        }
                        match document_write {
                            Some(document_write) if Self::request_matches(document_write, &request) => PrepareTurn::Ready,
                            Some(_) => PrepareTurn::Wait,
                            None => PrepareTurn::Acquire,
                        }
                    }
                    Some(RetainedGisMapDocumentStateV1::Published { owners, identity, document_write, .. }) => {
                        if owners.scope != *scope {
                            record_prepare_conflict("Published/scope");
                            return Err(GisMapApprovalCommitErrorV1::Conflict);
                        }
                        if !Arc::ptr_eq(&owners.document_write, &gate) {
                            record_prepare_conflict("Published/document_write");
                            return Err(GisMapApprovalCommitErrorV1::Conflict);
                        }
                        if identity.base_frontier != base.frontier || identity.base_digest != base.digest() {
                            let advanced = Self::publication_result_base(identity, &base.frontier, &base.digest(), &base.descriptor_digest) && Self::stores_consistent(owners);
                            if !advanced {
                                let snapshot = B::decode_parent(base.pack.as_slice()).map_err(|_| GisMapApprovalCommitErrorV1::Rejected)?;
                                if !Self::stores_match(owners, &snapshot) {
                                    record_prepare_conflict(&format!(
                                        "Published/stores_match base_frontier={} base_digest={} published_result={} stores_consistent={}",
                                        identity.base_frontier == base.frontier,
                                        identity.base_digest == base.digest(),
                                        Self::publication_result_base(identity, &base.frontier, &base.digest(), &base.descriptor_digest),
                                        Self::stores_consistent(owners),
                                    ));
                                    return Err(GisMapApprovalCommitErrorV1::Conflict);
                                }
                            }
                        }
                        match document_write {
                            Some(document_write) if Self::request_matches(document_write, &request) => PrepareTurn::Ready,
                            Some(_) => PrepareTurn::Wait,
                            None => PrepareTurn::Acquire,
                        }
                    }
                    Some(RetainedGisMapDocumentStateV1::Recovered { owners, document_write, .. }) => {
                        if owners.scope != *scope || !Arc::ptr_eq(&owners.document_write, &gate) || !Arc::ptr_eq(&document_write.gate, &gate) {
                            record_prepare_conflict("Recovered/scope-or-gate");
                            return Err(GisMapApprovalCommitErrorV1::Conflict);
                        }
                        if Self::request_matches(document_write, &request) {
                            PrepareTurn::PublishRecovered
                        } else {
                            PrepareTurn::Wait
                        }
                    }
                    Some(RetainedGisMapDocumentStateV1::Verification { owners, identity, document_write, .. }) | Some(RetainedGisMapDocumentStateV1::Publishing { owners, identity, document_write, .. }) => {
                        if owners.scope != *scope || !Arc::ptr_eq(&owners.document_write, &gate) || !Arc::ptr_eq(&document_write.gate, &gate) || identity.base_frontier != base.frontier || identity.base_digest != base.digest() {
                            record_prepare_conflict(&format!(
                                "Verification|Publishing/scope={} owners_gate={} lease_gate={} base_frontier={} base_digest={}",
                                owners.scope == *scope,
                                Arc::ptr_eq(&owners.document_write, &gate),
                                Arc::ptr_eq(&document_write.gate, &gate),
                                identity.base_frontier == base.frontier,
                                identity.base_digest == base.digest(),
                            ));
                            return Err(GisMapApprovalCommitErrorV1::Conflict);
                        }
                        if Self::request_matches(document_write, &request) {
                            PrepareTurn::Ready
                        } else {
                            PrepareTurn::Wait
                        }
                    }
                    Some(RetainedGisMapDocumentStateV1::Assembly { scope: stored, identity, document_write, .. }) | Some(RetainedGisMapDocumentStateV1::Journal { scope: stored, identity, document_write, .. }) => {
                        if stored != scope || !Arc::ptr_eq(&identity.document_write, &gate) || identity.base_frontier != base.frontier || identity.base_digest != base.digest() {
                            record_prepare_conflict(&format!(
                                "Assembly|Journal/scope={} identity_gate={} base_frontier={} base_digest={}",
                                stored == scope,
                                Arc::ptr_eq(&identity.document_write, &gate),
                                identity.base_frontier == base.frontier,
                                identity.base_digest == base.digest(),
                            ));
                            return Err(GisMapApprovalCommitErrorV1::Conflict);
                        }
                        if Self::request_matches(document_write, &request) {
                            PrepareTurn::Ready
                        } else {
                            PrepareTurn::Wait
                        }
                    }
                    Some(RetainedGisMapDocumentStateV1::Closing(_)) => PrepareTurn::Wait,
                    None => PrepareTurn::Acquire,
                }
            };
            match turn {
                PrepareTurn::Ready => return Ok(()),
                PrepareTurn::Wait => {
                    state_changes.changed().await.map_err(|_| GisMapApprovalCommitErrorV1::Unavailable)?;
                    continue;
                }
                PrepareTurn::Recover => {
                    self.finish_document_recovery(&key).await?;
                    self.announce_state_change();
                    continue;
                }
                PrepareTurn::PublishRecovered => {
                    self.resume_recovered_checkpoint(&key).await?;
                    self.announce_state_change();
                    return Ok(());
                }
                PrepareTurn::Acquire => {}
            }
            let document_write = Arc::new(GisMapDocumentWriteAuthorityV1::acquire(gate.clone(), request.clone()).await);
            let existing = {
                let mut documents = self.documents.lock().await;
                match documents.remove(&key) {
                    Some(RetainedGisMapDocumentStateV1::Ready { owners, pending, document_write: None }) => {
                        documents.insert(key.clone(), RetainedGisMapDocumentStateV1::Ready { owners, pending, document_write: Some(document_write) });
                        true
                    }
                    Some(RetainedGisMapDocumentStateV1::Published { owners, identity, receipt, document_write: None }) => {
                        documents.insert(key.clone(), RetainedGisMapDocumentStateV1::Published { owners, identity, receipt, document_write: Some(document_write) });
                        true
                    }
                    Some(state) => {
                        documents.insert(key.clone(), state);
                        drop(document_write);
                        false
                    }
                    None => {
                        drop(documents);
                        let handle = self.database.document(&protocol::ArtifactId(key.clone())).await.map_err(|error| gis_map_storage_refusal(&error))?;
                        self.mount_document(scope.clone(), handle, base, document_write, actor).await?;
                        self.finish_document_recovery(&key).await?;
                        return Ok(());
                    }
                }
            };
            if existing {
                return Ok(());
            }
        }
    }

    pub(super) fn build_assembly(
        &self,
        mut owners: GisMapDocumentStoresV1<B>,
        identity: GisMapCommitIdentityV1,
        snapshot: &GisMapParentSnapshotV1<B>,
        document_write: GisMapDocumentWriteLeaseV1,
    ) -> Result<RetainedGisMapDocumentStateV1<B>, (GisMapApprovalCommitErrorV1, GisMapDocumentStoresV1<B>)> {
        use directory::os_store::durable_group::{DurableOwnedMapMemberAdmissionV1, DurableOwnedThreeStoreMapAssemblyV1};
        let work = match &identity.operation {
            GisMapCommitOperationV1::Approval => B::approval_work(snapshot, &identity.job_id),
            GisMapCommitOperationV1::Undo { original_job_id, original_command, .. } => {
                let command = match CanonicalInferenceCommandV1::decode(original_command.as_slice()) {
                    Ok(command) => command,
                    Err(_) => return Err((GisMapApprovalCommitErrorV1::Rejected, owners)),
                };
                let work = match B::undo_work(snapshot, command.inverse_payload(), original_job_id) {
                    Ok(undo) => undo.work,
                    Err(MapBindingRefusalV1::Undecodable | MapBindingRefusalV1::Arity) => return Err((GisMapApprovalCommitErrorV1::Rejected, owners)),
                    Err(_) => return Err((GisMapApprovalCommitErrorV1::Conflict, owners)),
                };
                if !command.matches_fixed_three_parent(&approval_mutation_id(original_job_id, &sha256(&work.parent_json)), &document_key(&identity.scope), &identity.actor, &work.parent_json, &work.parent_inverse_json) {
                    return Err((GisMapApprovalCommitErrorV1::Conflict, owners));
                }
                Ok(work)
            }
        };
        let work = match work {
            Ok(work) => work,
            Err(_) => return Err((GisMapApprovalCommitErrorV1::Rejected, owners)),
        };
        let operations = match self.reserve_operations() {
            Ok(operations) => operations,
            Err(error) => return Err((error, owners)),
        };
        let parent = owners.parent.take().expect("mounted Map owner retains parent Store");
        let drawing = owners.drawing.take().expect("mounted Map owner retains drawing Store");
        let value = owners.value.take().expect("mounted Map owner retains value Store");
        let (parent_mutation, drawing_mutation, value_mutation) = match &identity.operation {
            GisMapCommitOperationV1::Approval => (work.parent, work.drawing, work.value),
            GisMapCommitOperationV1::Undo { .. } => {
                if work.parent_inverse.len() != 1 || work.drawing_inverse.len() != 1 || work.value_inverse.len() != 1 {
                    return Err((GisMapApprovalCommitErrorV1::Rejected, owners));
                }
                (work.parent_inverse.into_iter().next().expect("one parent inverse"), work.drawing_inverse.into_iter().next().expect("one drawing inverse"), work.value_inverse.into_iter().next().expect("one value inverse"))
            }
        };
        let parent_admission = DurableOwnedMapMemberAdmissionV1::new(operations[0], parent.generation_now(), parent.content_revision_now(), identity.actor.clone(), parent_mutation);
        let drawing_admission = DurableOwnedMapMemberAdmissionV1::new(operations[1], drawing.generation_now(), drawing.content_revision_now(), identity.actor.clone(), drawing_mutation);
        let value_admission = DurableOwnedMapMemberAdmissionV1::new(operations[2], value.generation_now(), value.content_revision_now(), identity.actor.clone(), value_mutation);
        let sink = owners.handle.durable_group_journal_sink(identity.journal_now_ms);
        let handle = owners.handle;
        let scope = owners.scope;
        let generation = owners.generation;
        let fence = owners.fence;
        Ok(RetainedGisMapDocumentStateV1::Assembly {
            owner: DurableOwnedThreeStoreMapAssemblyV1::new(
                parent,
                drawing,
                value,
                parent_admission,
                drawing_admission,
                value_admission,
                B::parent_preparation_factory(identity.mutation_id.clone(), identity.timestamp),
                B::drawing_preparation_factory(format!("{}:drawing", identity.mutation_id), identity.timestamp),
                B::value_preparation_factory(format!("{}:value", identity.mutation_id), identity.timestamp),
                sink,
            ),
            handle,
            scope,
            generation,
            identity,
            document_write,
            fence,
        })
    }

    pub(super) fn restored_stores(
        parent: GisMapParentStoreV1<B>,
        drawing: GisMapDrawingStoreV1<B>,
        value: GisMapValueStoreV1<B>,
        handle: db::ArtifactHandle,
        scope: DocumentScope,
        generation: u64,
        document_write: Arc<tokio::sync::Mutex<()>>,
        fence: Arc<InferenceDocumentFenceV1>,
    ) -> GisMapDocumentStoresV1<B> {
        GisMapDocumentStoresV1 { parent: Some(parent), drawing: Some(drawing), value: Some(value), handle, scope, generation, document_write, fence }
    }
}
