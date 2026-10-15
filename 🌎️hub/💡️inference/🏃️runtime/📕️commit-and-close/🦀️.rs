//! 📕️ The retained commit, the undo commit and the closing sequence.

use super::*;

impl RetainedGisMapApprovalCommitterV1 {
    pub(super) async fn commit_retained(&self, request: GisMapApprovalCommitRequestV1<'_>, mut owner: GisMapApprovalRequestOwnerV1, preflight: (GisMapCommitIdentityV1, GisMapParentSnapshotV1)) -> Result<GisMapApprovalReceiptV1, GisMapApprovalCommitErrorV1> {
        self.resume_parked_requests();
        self.resume_parked_prepared();
        let (mut identity, snapshot) = preflight;
        owner.admit(identity.clone());
        let base = request.base;
        let deadline_ms = request.deadline_ms;
        let now_ms = request.now_ms;
        let document_write = request.document_write;
        let mut live_ingress = Some(request.ingress);
        let key = document_key(&identity.scope);
        let request_token = owner.request.clone();
        self.prepare_retained_document(&identity.scope, base, document_write, request_token, &identity.actor).await?;
        let result = loop {
            let turn = self.drive_turn(&key, &identity, &snapshot).await;
            self.announce_state_change();
            match turn {
                GisMapCommitTurnV1::Continue => tokio::task::yield_now().await,
                GisMapCommitTurnV1::Committed => {
                    identity.ingress = None;
                    owner.identity.as_mut().expect("active approval request owner").ingress = None;
                    drop(live_ingress.take());
                    tokio::task::yield_now().await;
                }
                GisMapCommitTurnV1::Preflight { handle, generation } => {
                    tokio::task::yield_now().await;
                    let observed = handle.checkpoint_publication_snapshot().await.map_err(|error| gis_map_storage_refusal(&error))?;
                    if observed.authority_generation != generation {
                        break Err(gis_map_commit_conflict(GisMapCommitConflictArmV1::DriveGenerationDiffers));
                    }
                    self.finish_preflight(&key, &identity, &snapshot, &observed).await?;
                }
                GisMapCommitTurnV1::Verify { generation, receipt } => {
                    let attempt_lifetime_ms = deadline_ms.checked_sub(now_ms).filter(|value| *value != 0).ok_or(GisMapApprovalCommitErrorV1::Rejected)?;
                    let verified = self.verify(&key, &identity, generation, receipt, attempt_lifetime_ms, now_ms).await;
                    self.announce_state_change();
                    break verified;
                }
                GisMapCommitTurnV1::Rejected(error) => break Err(error),
            }
        };
        if result.is_ok() {
            owner.complete();
        }
        result
    }

    pub(super) async fn commit_undo_retained(
        &self,
        request: GisMapApprovalUndoCommitRequestV1<'_>,
        mut owner: GisMapApprovalRequestOwnerV1,
        preflight: (GisMapCommitIdentityV1, GisMapParentSnapshotV1),
    ) -> Result<GisMapApprovalUndoCommitReceiptV1, GisMapApprovalCommitErrorV1> {
        self.resume_parked_requests();
        self.resume_parked_prepared();
        let (mut identity, snapshot) = preflight;
        owner.admit(identity.clone());
        let key = document_key(&identity.scope);
        let request_token = owner.request.clone();
        let mut live_ingress = Some(request.ingress);
        self.prepare_retained_document(&identity.scope, request.base, request.document_write, request_token, &identity.actor).await?;
        let result = loop {
            let turn = self.drive_turn(&key, &identity, &snapshot).await;
            self.announce_state_change();
            match turn {
                GisMapCommitTurnV1::Continue => tokio::task::yield_now().await,
                GisMapCommitTurnV1::Committed => {
                    identity.ingress = None;
                    owner.identity.as_mut().expect("active undo request owner").ingress = None;
                    drop(live_ingress.take());
                    tokio::task::yield_now().await;
                }
                GisMapCommitTurnV1::Preflight { handle, generation } => {
                    tokio::task::yield_now().await;
                    let observed = handle.checkpoint_publication_snapshot().await.map_err(|error| gis_map_storage_refusal(&error))?;
                    if observed.authority_generation != generation {
                        break Err(gis_map_commit_conflict(GisMapCommitConflictArmV1::DriveUndoGenerationDiffers));
                    }
                    self.finish_preflight(&key, &identity, &snapshot, &observed).await?;
                }
                GisMapCommitTurnV1::Verify { generation, receipt } => {
                    let attempt_lifetime_ms = request.deadline_ms.checked_sub(request.now_ms).filter(|value| *value != 0).ok_or(GisMapApprovalCommitErrorV1::Rejected)?;
                    break self.verify(&key, &identity, generation, receipt, attempt_lifetime_ms, request.now_ms).await.map(|receipt| GisMapApprovalUndoCommitReceiptV1 {
                        witness: receipt.witness,
                        document_generation: receipt.document_generation,
                        applied: receipt.applied,
                        frontier: receipt.frontier,
                    });
                }
                GisMapCommitTurnV1::Rejected(error) => break Err(error),
            }
        };
        if result.is_ok() {
            owner.complete();
        }
        result
    }

    pub(super) fn closing(owners: GisMapDocumentStoresV1, document_write: Option<GisMapDocumentWriteLeaseV1>) -> RetainedGisMapDocumentStateV1 {
        owners.fence.invalidate();
        RetainedGisMapDocumentStateV1::Closing(GisMapClosingStoresV1 { owners, phase: 0, document_write })
    }

    fn close_store<P, M>(slot: &'static str, store: &mut Option<directory::os_store::ArtifactStore<P, M>>) -> Result<bool, GisMapApprovalCommitErrorV1>
    where
        P: pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + Clone + directory::ToValue + directory::FromValue + Send + Sync + 'static,
        M: directory::Mutation<P> + directory::os_spr::OpBinary + directory::os_spr::OpText + Clone + directory::ToValue + directory::FromValue + Send + 'static,
    {
        use directory::os_store::{SnapshotRetirementStep, SpaceMember};
        let Some(owner) = store.as_mut() else { return Ok(true) };
        match owner.close_owned_step(1, directory::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES).map_err(|error| gis_map_storage_refusal(&error))? {
            SnapshotRetirementStep::Complete => {
                let terminal = owner.close_owned_terminal_is_empty();
                record_close_store_step(&format!("{slot} Complete terminal_is_empty={terminal}"));
                if !terminal {
                    return Err(gis_map_storage_refusal(&"refused"));
                }
                drop(store.take());
                Ok(true)
            }
            SnapshotRetirementStep::Pending { .. } => {
                record_close_store_step(&format!("{slot} Pending"));
                Ok(false)
            }
            SnapshotRetirementStep::Blocked => {
                let outstanding = owner.outstanding_snapshot_read_count();
                let returned = owner.returned_snapshot_read_count();
                let terminal = owner.close_owned_terminal_is_empty();
                let phase = owner.close_owned_phase_witness();
                let displaced = owner.close_displaced_witness();
                record_close_store_step(&format!("{slot} Blocked outstanding_reads={outstanding} returned_reads={returned} terminal_is_empty={terminal} phase={phase} displaced={displaced}"));
                Err(gis_map_storage_refusal(&"refused"))
            }
        }
    }

    async fn close_turn(&self, key: &str) -> Result<bool, GisMapApprovalCommitErrorV1> {
        use directory::os_store::durable_group::{DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedThreeStoreMapAssemblyAdvanceV1};
        let mut documents = self.documents.lock().await;
        let Some(state) = documents.remove(key) else { return Ok(true) };
        let state = match state {
            RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence } => {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence });
                return Err(gis_map_storage_refusal(&"refused"));
            }
            RetainedGisMapDocumentStateV1::Ready { owners, document_write, .. } => Self::closing(owners, document_write),
            RetainedGisMapDocumentStateV1::Published { owners, document_write, .. } => Self::closing(owners, document_write),
            RetainedGisMapDocumentStateV1::Recovered { owners, checkpoint, document_write } => {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovered { owners, checkpoint, document_write });
                return Err(gis_map_storage_refusal(&"refused"));
            }
            RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt, document_write } => {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt, document_write });
                return Err(gis_map_storage_refusal(&"refused"));
            }
            RetainedGisMapDocumentStateV1::Publishing { owners, identity, receipt, document_write } => {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Publishing { owners, identity, receipt, document_write });
                return Err(gis_map_storage_refusal(&"refused"));
            }
            RetainedGisMapDocumentStateV1::Assembly { mut owner, handle, scope, generation, identity, document_write, fence } => {
                owner.cancel();
                match owner.advance(directory::os_store::one_item::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: directory::os_store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES }) {
                    Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Terminal) => {
                        let terminal = owner.take_terminal_owners().expect("closed assembly returns exact owners");
                        drop(owner);
                        drop(terminal.sink);
                        Self::closing(Self::restored_stores(terminal.parent, terminal.drawing, terminal.value, handle, scope, generation, identity.document_write.clone(), fence), Some(document_write))
                    }
                    Ok(DurableOwnedThreeStoreMapAssemblyAdvanceV1::Mounted) => {
                        let host = owner.take_mounted_host().expect("mounted closing assembly returns its exact host");
                        drop(owner);
                        RetainedGisMapDocumentStateV1::Journal { owner: host, handle, scope, generation, identity, receipt: None, document_write, fence }
                    }
                    Ok(_) => RetainedGisMapDocumentStateV1::Assembly { owner, handle, scope, generation, identity, document_write, fence },
                    Err(_) => {
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Assembly { owner, handle, scope, generation, identity, document_write, fence });
                        return Err(gis_map_storage_refusal(&"refused"));
                    }
                }
            }
            RetainedGisMapDocumentStateV1::Journal { mut owner, handle, scope, generation, identity, mut receipt, document_write, fence } => {
                if receipt.is_none() {
                    owner.cancel();
                }
                match owner.advance(directory::os_store::one_item::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: directory::os_store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES }) {
                    Ok(DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(exact_receipt)) => {
                        if receipt.as_ref().is_some_and(|stored| stored != &exact_receipt) {
                            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                            return Err(gis_map_storage_refusal(&"refused"));
                        }
                        receipt = Some(exact_receipt);
                        if !owner.acknowledge() {
                            documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                            return Err(gis_map_storage_refusal(&"refused"));
                        }
                        RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence }
                    }
                    Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete) => {
                        let terminal = owner.take_terminal_owners().expect("closed journal host returns exact owners");
                        drop(owner);
                        drop(terminal.sink);
                        let owners = Self::restored_stores(terminal.parent, terminal.drawing, terminal.value, handle, scope, generation, identity.document_write.clone(), fence);
                        match receipt {
                            Some(receipt) => RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt, document_write },
                            None => Self::closing(owners, Some(document_write)),
                        }
                    }
                    Ok(_) => RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence },
                    Err(_) => {
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Journal { owner, handle, scope, generation, identity, receipt, document_write, fence });
                        return Err(gis_map_storage_refusal(&"refused"));
                    }
                }
            }
            RetainedGisMapDocumentStateV1::Closing(mut closing) => {
                let closed = match closing.phase {
                    0 => Self::close_store("value", &mut closing.owners.value),
                    1 => Self::close_store("drawing", &mut closing.owners.drawing),
                    2 => Self::close_store("parent", &mut closing.owners.parent),
                    3 => Ok(true),
                    phase => Err(gis_map_storage_refusal(&phase)),
                };
                let closed = match closed {
                    Ok(closed) => closed,
                    Err(error) => {
                        documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Closing(closing));
                        return Err(error);
                    }
                };
                let complete = match closing.phase {
                    0 if closed => {
                        closing.phase = 1;
                        false
                    }
                    1 if closed => {
                        closing.phase = 2;
                        false
                    }
                    2 if closed => {
                        closing.phase = 3;
                        false
                    }
                    3 => true,
                    _ => false,
                };
                if complete {
                    return Ok(true);
                }
                RetainedGisMapDocumentStateV1::Closing(closing)
            }
        };
        documents.insert(key.to_owned(), state);
        Ok(false)
    }

    /// 🧹 Drives every retained Store, assembly and journal owner to one explicit terminal handoff.
    pub async fn close(&self) -> Result<(), GisMapApprovalCommitErrorV1> {
        let mut closing = {
            let _jobs = self.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            self.closing.store(true, Ordering::Release);
            GisMapClosingCursorV1 { closing: self.closing.clone(), state_epoch: self.state_epoch.clone(), retained: true }
        };
        self.announce_state_change();
        let mut state_changes = self.state_epoch.subscribe();
        loop {
            self.resume_parked_requests();
            self.resume_parked_prepared();
            let active_request = {
                let documents = self.documents.lock().await;
                documents.values().any(Self::state_has_active_request)
            };
            if active_request {
                state_changes.changed().await.map_err(|_| GisMapApprovalCommitErrorV1::Unavailable)?;
                continue;
            }
            let next = {
                let documents = self.documents.lock().await;
                documents
                    .iter()
                    .next()
                    .map(|(key, state)| {
                        let gate = match state {
                            RetainedGisMapDocumentStateV1::Ready { owners, document_write, .. } => {
                                if document_write.is_some() {
                                    return Some((key.clone(), None));
                                }
                                owners.document_write.clone()
                            }
                            RetainedGisMapDocumentStateV1::Published { owners, document_write, .. } => {
                                if document_write.is_some() {
                                    return Some((key.clone(), None));
                                }
                                owners.document_write.clone()
                            }
                            RetainedGisMapDocumentStateV1::Recovery { .. } | RetainedGisMapDocumentStateV1::Recovered { .. } => return Some((key.clone(), None)),
                            RetainedGisMapDocumentStateV1::Assembly { .. } | RetainedGisMapDocumentStateV1::Journal { .. } | RetainedGisMapDocumentStateV1::Verification { .. } | RetainedGisMapDocumentStateV1::Publishing { .. } => {
                                return Some((key.clone(), None));
                            }
                            RetainedGisMapDocumentStateV1::Closing(closing) => {
                                if closing.document_write.is_some() {
                                    return Some((key.clone(), None));
                                }
                                closing.owners.document_write.clone()
                            }
                        };
                        Some((key.clone(), Some(gate)))
                    })
                    .flatten()
            };
            let Some((key, gate)) = next else {
                let maintenance_is_empty = self.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty();
                let cleanup_is_empty = self.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty();
                if maintenance_is_empty && cleanup_is_empty {
                    closing.surrender();
                    return Ok(());
                }
                state_changes.changed().await.map_err(|_| GisMapApprovalCommitErrorV1::Unavailable)?;
                continue;
            };
            let maintenance_owns_document = self.maintenance_owns_document(&key);
            if maintenance_owns_document {
                state_changes.changed().await.map_err(|_| GisMapApprovalCommitErrorV1::Unavailable)?;
                continue;
            }
            let _document_write = match gate {
                Some(gate) => Some(Arc::new(GisMapDocumentWriteAuthorityV1::acquire(gate, Arc::new(GisMapApprovalRequestTokenV1)).await)),
                None => None,
            };
            let recovering = {
                let documents = self.documents.lock().await;
                matches!(documents.get(&key), Some(RetainedGisMapDocumentStateV1::Recovery { .. }))
            };
            if recovering {
                self.finish_document_recovery(&key).await?;
                self.announce_state_change();
                continue;
            }
            let recovered = {
                let documents = self.documents.lock().await;
                matches!(documents.get(&key), Some(RetainedGisMapDocumentStateV1::Recovered { .. }))
            };
            if recovered {
                self.resume_recovered_checkpoint(&key).await?;
                self.announce_state_change();
                continue;
            }
            let verification = {
                let documents = self.documents.lock().await;
                match documents.get(&key) {
                    Some(RetainedGisMapDocumentStateV1::Verification { owners, identity, receipt, .. }) | Some(RetainedGisMapDocumentStateV1::Publishing { owners, identity, receipt, .. }) => {
                        Some((identity.clone(), owners.generation, receipt.clone()))
                    }
                    _ => None,
                }
            };
            if let Some((identity, generation, receipt)) = verification {
                self.verify(&key, &identity, generation, receipt, super::super::schema::JOB_MAX_LIFETIME_MS, identity.journal_now_ms).await?;
                self.announce_state_change();
                continue;
            }
            let closed = self.close_turn(&key).await?;
            self.announce_state_change();
            let _ = closed;
        }
    }
}
