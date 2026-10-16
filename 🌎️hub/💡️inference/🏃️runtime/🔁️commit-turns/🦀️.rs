//! 🔁️ Commit turns: driving one request or one abandoned turn to its terminal state.

use std::sync::Arc;
use semio_hub_inference_binding::ComposedMapCommitBindingV1;
use crate::{GisMapAbandonedTurnV1, GisMapCommitTurnV1, RetainedGisMapApprovalCommitterV1, RetainedGisMapDocumentStateV1};
use semio_hub_inference_runtime_contract::{GisMapApprovalCommitErrorV1, GisMapApprovalRequestTokenV1, GisMapCommitConflictArmV1, GisMapCommitIdentityV1, GisMapCommitOperationV1, GisMapDocumentWriteLeaseV1, GisMapParentSnapshotV1, gis_map_commit_conflict, gis_map_storage_refusal, record_abandoned_assembly_turn, record_prepare_conflict, recorded_abandoned_assembly_turn};
use semio_hub_inference_ledger::InferenceReaderV1;

impl<B: ComposedMapCommitBindingV1> RetainedGisMapApprovalCommitterV1<B> {
    pub(super) async fn drive_turn(&self, key: &str, candidate: &GisMapCommitIdentityV1, snapshot: &GisMapParentSnapshotV1<B>) -> GisMapCommitTurnV1 {
        use directory::os_store::durable_group::{DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedThreeStoreMapAssemblyAdvanceV1};
        let mut documents = self.documents.lock().await;
        let Some(state) = documents.remove(key) else { return GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Unavailable) };
        match state {
            RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence } => {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence });
                GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Unavailable)
            }
            RetainedGisMapDocumentStateV1::Recovered { owners, checkpoint, document_write } => {
                let generation = owners.generation;
                let receipt = checkpoint.receipt().clone();
                let mut identity = candidate.clone();
                identity.ingress = None;
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt: receipt.clone(), document_write });
                GisMapCommitTurnV1::Verify { generation, receipt }
            }
            RetainedGisMapDocumentStateV1::Ready { owners, pending, document_write } => {
                if !Self::stores_match(&owners, snapshot) || pending.as_ref().is_some_and(|identity| !Self::identity_matches(identity, candidate)) {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending, document_write });
                    return GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Conflict);
                }
                let handle = owners.handle.clone();
                let generation = owners.generation;
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: Some(candidate.clone()), document_write });
                GisMapCommitTurnV1::Preflight { handle, generation }
            }
            RetainedGisMapDocumentStateV1::Assembly { mut owner, handle, scope, generation, identity, document_write, fence } => {
                if !Self::identity_matches(&identity, candidate) {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Assembly { owner, handle, scope, generation, identity, document_write, fence });
                    return GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Conflict);
                }
                let result = owner.advance(directory::os_store::one_item::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: directory::os_store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES });
                match result {
                    Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Mounted) => {
                        let host = owner.take_mounted_host().expect("mounted assembly returns its exact host");
                        drop(owner);
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner: host, handle, scope, generation, identity, receipt: None, document_write, fence });
                        GisMapCommitTurnV1::Continue
                    }
                    Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Terminal) => {
                        let terminal = owner.take_terminal_owners().expect("terminal assembly returns every owner");
                        drop(owner);
                        eprintln!("[WARN] inference runtime: fixed-three assembly refused before the journal for {key} — {:?}", terminal.failure);
                        let owners = Self::restored_stores(terminal.parent, terminal.drawing, terminal.value, handle, scope, generation, identity.document_write.clone(), fence);
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: None, document_write: None });
                        GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Rejected)
                    }
                    Ok(_) => {
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Assembly { owner, handle, scope, generation, identity, document_write, fence });
                        GisMapCommitTurnV1::Continue
                    }
                    Err(_) => {
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Assembly { owner, handle, scope, generation, identity, document_write, fence });
                        GisMapCommitTurnV1::Rejected(gis_map_storage_refusal(&"turn refused"))
                    }
                }
            }
            RetainedGisMapDocumentStateV1::Journal { mut owner, handle, scope, generation, mut identity, mut receipt, document_write, fence } => {
                if !Self::identity_matches(&identity, candidate) {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                    return GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Conflict);
                }
                match owner.advance(directory::os_store::one_item::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: directory::os_store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES }) {
                    Ok(DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(exact_receipt)) => {
                        if receipt.as_ref().is_some_and(|stored| stored != &exact_receipt) {
                            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                            return GisMapCommitTurnV1::Rejected(gis_map_storage_refusal(&"turn refused"));
                        }
                        receipt = Some(exact_receipt);
                        identity.ingress = None;
                        if !owner.acknowledge() {
                            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                            return GisMapCommitTurnV1::Rejected(gis_map_storage_refusal(&"turn refused"));
                        }
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                        GisMapCommitTurnV1::Committed
                    }
                    Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete) => {
                        let Some(receipt) = receipt else {
                            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt: None, document_write, fence });
                            return GisMapCommitTurnV1::Rejected(gis_map_storage_refusal(&"turn refused"));
                        };
                        let terminal = owner.take_terminal_owners().expect("acknowledged journal host returns exact Stores and sink");
                        drop(owner);
                        drop(terminal.sink);
                        let owners = Self::restored_stores(terminal.parent, terminal.drawing, terminal.value, handle, scope, generation, identity.document_write.clone(), fence);
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt: receipt.clone(), document_write });
                        GisMapCommitTurnV1::Verify { generation, receipt }
                    }
                    Ok(_) => {
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                        GisMapCommitTurnV1::Continue
                    }
                    Err(_) => {
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                        GisMapCommitTurnV1::Rejected(gis_map_storage_refusal(&"turn refused"))
                    }
                }
            }
            RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt, document_write } => {
                let matches = Self::identity_matches(&identity, candidate);
                let generation = owners.generation;
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt: receipt.clone(), document_write });
                if matches {
                    GisMapCommitTurnV1::Verify { generation, receipt }
                } else {
                    GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Conflict)
                }
            }
            RetainedGisMapDocumentStateV1::Publishing { owners, identity, receipt, document_write } => {
                let matches = Self::identity_matches(&identity, candidate);
                let generation = owners.generation;
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Publishing { owners, identity, receipt: receipt.clone(), document_write });
                if matches {
                    GisMapCommitTurnV1::Verify { generation, receipt }
                } else {
                    GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Conflict)
                }
            }
            RetainedGisMapDocumentStateV1::Published { owners, identity, receipt, document_write } => {
                if Self::identity_matches(&identity, candidate) {
                    let generation = owners.generation;
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Published { owners, identity, receipt: receipt.clone(), document_write });
                    GisMapCommitTurnV1::Verify { generation, receipt }
                } else if (Self::stores_match(&owners, snapshot) || (Self::publication_result_base(&identity, &candidate.base_frontier, &candidate.base_digest, &candidate.descriptor_digest) && Self::stores_consistent(&owners))) && document_write.is_some() {
                    let handle = owners.handle.clone();
                    let generation = owners.generation;
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: Some(candidate.clone()), document_write });
                    GisMapCommitTurnV1::Preflight { handle, generation }
                } else {
                    record_prepare_conflict(&format!(
                        "drive Published/stores_match={} published_result={} stores_consistent={} writer={}",
                        Self::stores_match(&owners, snapshot),
                        Self::publication_result_base(&identity, &candidate.base_frontier, &candidate.base_digest, &candidate.descriptor_digest),
                        Self::stores_consistent(&owners),
                        document_write.is_some(),
                    ));
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Published { owners, identity, receipt, document_write });
                    GisMapCommitTurnV1::Rejected(gis_map_commit_conflict(GisMapCommitConflictArmV1::DrivePublishedIdentityAndStoresDiffer))
                }
            }
            closing @ RetainedGisMapDocumentStateV1::Closing(_) => {
                documents.insert(key.to_owned(), closing);
                record_prepare_conflict("drive Closing");
                GisMapCommitTurnV1::Rejected(GisMapApprovalCommitErrorV1::Unavailable)
            }
        }
    }

    pub(super) fn request_matches(document_write: &GisMapDocumentWriteLeaseV1, request: &Arc<GisMapApprovalRequestTokenV1>) -> bool {
        Arc::ptr_eq(&document_write.request, request)
    }

    pub(super) fn state_has_active_request(state: &RetainedGisMapDocumentStateV1<B>) -> bool {
        let document_write = match state {
            RetainedGisMapDocumentStateV1::Recovery { document_write, .. }
            | RetainedGisMapDocumentStateV1::Recovered { document_write, .. }
            | RetainedGisMapDocumentStateV1::Assembly { document_write, .. }
            | RetainedGisMapDocumentStateV1::Journal { document_write, .. }
            | RetainedGisMapDocumentStateV1::Verification { document_write, .. }
            | RetainedGisMapDocumentStateV1::Publishing { document_write, .. } => Some(document_write),
            RetainedGisMapDocumentStateV1::Ready { document_write, .. } | RetainedGisMapDocumentStateV1::Published { document_write, .. } => document_write.as_ref(),
            RetainedGisMapDocumentStateV1::Closing(closing) => closing.document_write.as_ref(),
        };
        document_write.is_some_and(|document_write| Arc::strong_count(&document_write.request) > 1)
    }

    async fn drive_abandoned_turn(&self, key: &str, candidate: &GisMapCommitIdentityV1, request: &Arc<GisMapApprovalRequestTokenV1>) -> GisMapAbandonedTurnV1 {
        use directory::os_store::durable_group::{DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedThreeStoreMapAssemblyAdvanceV1};
        let mut documents = self.documents.lock().await;
        let Some(state) = documents.remove(key) else {
            record_abandoned_assembly_turn("Complete on an absent document");
            return GisMapAbandonedTurnV1::Complete;
        };
        match state {
            RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence } => {
                let matches = Self::request_matches(&document_write, request);
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence });
                if matches {
                    GisMapAbandonedTurnV1::Recover
                } else {
                    record_abandoned_assembly_turn("Complete on Recovery/request");
                    GisMapAbandonedTurnV1::Complete
                }
            }
            RetainedGisMapDocumentStateV1::Recovered { owners, checkpoint, document_write } => {
                let matches = Self::request_matches(&document_write, request);
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovered { owners, checkpoint, document_write });
                if matches {
                    GisMapAbandonedTurnV1::Recovered
                } else {
                    record_abandoned_assembly_turn("Complete on Recovered/request");
                    GisMapAbandonedTurnV1::Complete
                }
            }
            RetainedGisMapDocumentStateV1::Ready { owners, pending, document_write } => {
                let lease_matches = document_write.as_ref().is_some_and(|lease| Self::request_matches(lease, request));
                let pending_matches = pending.as_ref().is_none_or(|identity| Self::identity_matches(identity, candidate));
                let matches = lease_matches && pending_matches;
                if matches {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: None, document_write: None });
                    drop(document_write);
                    GisMapAbandonedTurnV1::Aborted
                } else {
                    record_abandoned_assembly_turn(&format!("Complete on Ready/lease={lease_matches} pending={pending_matches} writer={}", document_write.is_some()));
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending, document_write });
                    GisMapAbandonedTurnV1::Complete
                }
            }
            RetainedGisMapDocumentStateV1::Assembly { mut owner, handle, scope, generation, identity, document_write, fence } => {
                if !Self::request_matches(&document_write, request) || !Self::identity_matches(&identity, candidate) {
                    record_abandoned_assembly_turn(&format!("Complete on Assembly/request={} identity={}", Self::request_matches(&document_write, request), Self::identity_matches(&identity, candidate)));
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Assembly { owner, handle, scope, generation, identity, document_write, fence });
                    return GisMapAbandonedTurnV1::Complete;
                }
                owner.cancel();
                match owner.advance(directory::os_store::one_item::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: directory::os_store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES }) {
                    Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Mounted) => {
                        let host = owner.take_mounted_host().expect("mounted abandoned assembly returns its exact host");
                        drop(owner);
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner: host, handle, scope, generation, identity, receipt: None, document_write, fence });
                        GisMapAbandonedTurnV1::Continue
                    }
                    Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Terminal) => {
                        let terminal = owner.take_terminal_owners().expect("abandoned assembly returns every owner");
                        drop(owner);
                        drop(terminal.sink);
                        let owners = Self::restored_stores(terminal.parent, terminal.drawing, terminal.value, handle, scope, generation, identity.document_write.clone(), fence);
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: None, document_write: None });
                        drop(document_write);
                        GisMapAbandonedTurnV1::Aborted
                    }
                    Ok(progress) => {
                        record_abandoned_assembly_turn(&format!("Continue on {progress:?} [{}]", owner.closing_witness()));
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Assembly { owner, handle, scope, generation, identity, document_write, fence });
                        GisMapAbandonedTurnV1::Continue
                    }
                    Err(error) => {
                        record_abandoned_assembly_turn(&format!("Retry on {error:?} [{}]", owner.closing_witness()));
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Assembly { owner, handle, scope, generation, identity, document_write, fence });
                        GisMapAbandonedTurnV1::Retry
                    }
                }
            }
            RetainedGisMapDocumentStateV1::Journal { mut owner, handle, scope, generation, mut identity, mut receipt, document_write, fence } => {
                if !Self::request_matches(&document_write, request) || !Self::identity_matches(&identity, candidate) {
                    record_abandoned_assembly_turn(&format!(
                        "Complete on Journal/request={} identity={} receipt={}",
                        Self::request_matches(&document_write, request),
                        Self::identity_matches(&identity, candidate),
                        receipt.is_some()
                    ));
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                    return GisMapAbandonedTurnV1::Complete;
                }
                if receipt.is_none() {
                    owner.cancel();
                }
                match owner.advance(directory::os_store::one_item::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: directory::os_store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES }) {
                    Ok(DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(exact_receipt)) => {
                        if receipt.as_ref().is_some_and(|stored| stored != &exact_receipt) {
                            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                            return GisMapAbandonedTurnV1::Retry;
                        }
                        receipt = Some(exact_receipt);
                        identity.ingress = None;
                        if !owner.acknowledge() {
                            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                            return GisMapAbandonedTurnV1::Retry;
                        }
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                        GisMapAbandonedTurnV1::Committed
                    }
                    Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete) => {
                        let terminal = owner.take_terminal_owners().expect("abandoned journal returns every owner");
                        drop(owner);
                        drop(terminal.sink);
                        let owners = Self::restored_stores(terminal.parent, terminal.drawing, terminal.value, handle, scope, generation, identity.document_write.clone(), fence);
                        if let Some(receipt) = receipt {
                            let verify_identity = identity.clone();
                            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt: receipt.clone(), document_write });
                            GisMapAbandonedTurnV1::Verify { identity: verify_identity, generation, receipt }
                        } else {
                            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Ready { owners, pending: None, document_write: None });
                            drop(document_write);
                            GisMapAbandonedTurnV1::Aborted
                        }
                    }
                    Ok(progress) => {
                        record_abandoned_assembly_turn(&format!("Journal Continue on {progress:?} receipt={}", receipt.is_some()));
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                        GisMapAbandonedTurnV1::Continue
                    }
                    Err(error) => {
                        record_abandoned_assembly_turn(&format!("Journal Retry on {error:?} receipt={}", receipt.is_some()));
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                        GisMapAbandonedTurnV1::Retry
                    }
                }
            }
            RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt, document_write } => {
                let matches = Self::request_matches(&document_write, request) && Self::identity_matches(&identity, candidate);
                let generation = owners.generation;
                let verify_identity = identity.clone();
                let verify_receipt = receipt.clone();
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt, document_write });
                if matches {
                    GisMapAbandonedTurnV1::Verify { identity: verify_identity, generation, receipt: verify_receipt }
                } else {
                    record_abandoned_assembly_turn("Complete on Verification/request-or-identity");
                    GisMapAbandonedTurnV1::Complete
                }
            }
            RetainedGisMapDocumentStateV1::Publishing { owners, identity, receipt, document_write } => {
                let matches = Self::request_matches(&document_write, request) && Self::identity_matches(&identity, candidate);
                let generation = owners.generation;
                let verify_identity = identity.clone();
                let verify_receipt = receipt.clone();
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Publishing { owners, identity, receipt, document_write });
                if matches {
                    GisMapAbandonedTurnV1::Verify { identity: verify_identity, generation, receipt: verify_receipt }
                } else {
                    record_abandoned_assembly_turn("Complete on Publishing/request-or-identity");
                    GisMapAbandonedTurnV1::Complete
                }
            }
            RetainedGisMapDocumentStateV1::Published { owners, identity, receipt, document_write } => {
                let matches = document_write.as_ref().is_some_and(|lease| Self::request_matches(lease, request)) && Self::identity_matches(&identity, candidate);
                record_abandoned_assembly_turn(&format!("Complete on Published/matches={matches} writer={}", document_write.is_some()));
                if matches {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Published { owners, identity, receipt, document_write: None });
                    drop(document_write);
                } else {
                    documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Published { owners, identity, receipt, document_write });
                }
                GisMapAbandonedTurnV1::Complete
            }
            closing @ RetainedGisMapDocumentStateV1::Closing(_) => {
                record_abandoned_assembly_turn("Complete on Closing");
                documents.insert(key.to_owned(), closing);
                GisMapAbandonedTurnV1::Complete
            }
        }
    }

    pub(super) async fn drive_abandoned_request(&self, key: &str, mut identity: GisMapCommitIdentityV1, request: Arc<GisMapApprovalRequestTokenV1>) {
        let mut retry_delay = std::time::Duration::from_millis(1);
        loop {
            let turn = self.drive_abandoned_turn(key, &identity, &request).await;
            self.announce_state_change();
            let retry = match turn {
                GisMapAbandonedTurnV1::Complete | GisMapAbandonedTurnV1::Aborted => {
                    let exit = if matches!(turn, GisMapAbandonedTurnV1::Aborted) { "Aborted" } else { "Complete" };
                    let reason = recorded_abandoned_assembly_turn();
                    if matches!(&identity.operation, GisMapCommitOperationV1::Undo { .. }) {
                        record_abandoned_assembly_turn(&format!("driver leaving on {exit} [{reason}] — Undo, no outbox"));
                        return;
                    }
                    let Some(ingress) = identity.ingress.as_ref() else {
                        record_abandoned_assembly_turn(&format!("driver leaving on {exit} [{reason}] — ingress already surrendered"));
                        return;
                    };
                    let reader =
                        InferenceReaderV1 { user_id: ingress.user_id(), session_id: ingress.session_id(), authorization_generation: ingress.authorization_generation(), space_id: &identity.scope.space_id, document_id: &identity.scope.document_id };
                    match self.ledger.abandon_prepared_approval(&reader, &identity.job_id, &identity.mutation_id, &identity.command_hash, &identity.proposal_hash) {
                        Ok(_) => {
                            record_abandoned_assembly_turn(&format!("driver leaving on {exit} [{reason}] — outbox abandoned"));
                            return;
                        }
                        Err(error) => {
                            record_abandoned_assembly_turn(&format!("driver retrying after {exit} [{reason}] — abandon refused {error:?}"));
                            true
                        }
                    }
                }
                GisMapAbandonedTurnV1::Continue => false,
                GisMapAbandonedTurnV1::Committed => {
                    identity.ingress = None;
                    false
                }
                GisMapAbandonedTurnV1::Recover => self.finish_document_recovery(key).await.is_err(),
                GisMapAbandonedTurnV1::Recovered => match self.resume_recovered_checkpoint(key).await {
                    Ok((identity, generation, receipt)) => self.verify(key, &identity, generation, receipt, semio_hub_inference::schema::JOB_MAX_LIFETIME_MS, identity.journal_now_ms).await.is_err(),
                    Err(_) => true,
                },
                GisMapAbandonedTurnV1::Verify { identity, generation, receipt } => self.verify(key, &identity, generation, receipt, semio_hub_inference::schema::JOB_MAX_LIFETIME_MS, identity.journal_now_ms).await.is_err(),
                GisMapAbandonedTurnV1::Retry => true,
            };
            if retry {
                tokio::time::sleep(retry_delay).await;
                retry_delay = retry_delay.saturating_mul(2).min(std::time::Duration::from_secs(1));
            } else {
                retry_delay = std::time::Duration::from_millis(1);
                tokio::task::yield_now().await;
            }
        }
    }
}
