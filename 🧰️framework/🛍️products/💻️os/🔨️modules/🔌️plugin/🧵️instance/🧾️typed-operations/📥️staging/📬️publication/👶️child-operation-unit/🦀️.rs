//! 👶️ Child operation unit of the typed operation publication: the publication of one mounted child operation. See [`super`].

use super::*;

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> TypedCtx<'ctx, A, M, X> {
    /// 🧩️ The composed lane is PUBLISHED, not discarded: `dispatch_emit_group` has just
    /// built the parent's mutations, one per touched child under the child's own
    /// content-addressed handle, and the single `UndoGroup` that names them all. The
    /// three scalars of `receipt` are what the host ACKs; this is what the host READS.
    pub(super) async fn publish_mounted_typed_child_operation_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        if mounted.pending_child_publication.as_ref().is_some_and(|owner|matches!(owner.phase,PendingChildGroupPublicationPhase::Acknowledged|PendingChildGroupPublicationPhase::Closing)){return Ok(());}
        let Some(mut pending) = mounted.pending_child_publication.take() else {
            return Err(plugin_sdk_fault("typed-operation child publication lost its retained owner"));
        };
        match pending.phase {
            PendingChildGroupPublicationPhase::Acknowledged | PendingChildGroupPublicationPhase::Closing => {
                mounted.pending_child_publication=Some(pending);
                return Err(plugin_sdk_fault("original mounted child publication escaped its full-grant close frontier"));
            }
            PendingChildGroupPublicationPhase::Committed => {
                let receipt = pending.receipt.as_ref().cloned().ok_or_else(|| plugin_sdk_fault("committed child publication lost its exact result receipt"))?;
                mounted.published_artifact = true;
                self.as_dispatch().record_typed_operation_lane(mounted);
                mounted.pending_child_publication = Some(pending);
                return mounted.queue_page(TypedOperationResultPage::try_serialize_cold(mounted.next_token(), TypedOperationResultLane::Child, &receipt)?);
            }
            PendingChildGroupPublicationPhase::Faulted => {
                let fault = pending.fault().unwrap_or_else(|| plugin_sdk_fault("typed-operation child publication fault lost its bounded detail"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            }
            PendingChildGroupPublicationPhase::Ready => {}
            PendingChildGroupPublicationPhase::Dispatching => {
                mounted.pending_child_publication = Some(pending);
                return Err(plugin_sdk_fault("typed-operation child publication re-entered a non-dispatchable phase"));
            }
        }
        match self.child_composition.reclaim_child_retirements(1, 0) {
            Ok(true) => {}
            Ok(false) => {
                mounted.pending_child_publication = Some(pending);
                return Ok(());
            }
            Err(error) => {
                let fault = pending.reject_and_fault(&error);
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            }
        }
        let Some(_permit) = mounted.cancellation_lease.as_ref().and_then(ToolCancellationLease::try_claim_publication) else {
            let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication claim is cancelled or occupied"));
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        };
        let live_revision = self.stores.store.content_revision_now();
        let live_generation = self.stores.store.generation_now();
        let Some(captured) = mounted.captured_child_content.as_ref() else {
            let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication lost its captured immutable root"));
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        };
        let parent_fresh = typed_operation_document_is_fresh(&mounted.operation, mounted.canonical_revision, live_revision, live_generation);
        let child_root_fresh = mounted.captured_child_content_generation == self.child_composition.child_content_generation && captured.identity_digest() == self.child_composition.child_content_root.identity_digest();
        if !parent_fresh || !child_root_fresh {
            let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication rejected stale parent or child immutable roots"));
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        }
        let mut seen = HashSet::with_capacity(pending.child_emits.len());
        for child in pending.child_emits.iter() {
            if !seen.insert((child.slot.clone(), child.child_id.clone())) {
                let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication contains a duplicate exact target"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            }
            let expected_revision = match captured.revision(&child.slot, &child.child_id) {
                Ok(revision) => revision,
                Err(error) => {
                    let fault = pending.reject_and_fault(&error);
                    mounted.pending_child_publication = Some(pending);
                    return Err(fault);
                }
            };
            let Some(expected_dialect) = captured.dialect(&child.slot, &child.child_id) else {
                let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication target is absent from its captured root"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            };
            let Some(entry) = self.child_composition.children.get(&(child.slot.clone(), child.child_id.clone())) else {
                let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication target is absent from the live member registry"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            };
            if entry.reference.dialect != expected_dialect || entry.member.content_revision().await != expected_revision {
                let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication rejected stale member identity, dialect, or revision"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            }
        }
        if mounted.cancellation_lease.as_ref().expect("claimed child publication retains its cancellation lease").is_cancelled().await {
            let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication was cancelled before group linearization"));
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        }
        if let Err(error) = pending.begin_dispatch() {
            let fault = pending.reject_and_fault(&error);
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        }
        let result = self.as_dispatch().dispatch_emit_group(&mounted.verb, &pending.artifact_mutations, &pending.child_emits, Vec::new(), Vec::new(), UiDirtyScope::None, &mounted.meta, None, pending.transaction.as_ref().cloned(), identity).await;
        match result {
            Ok(result) => {
                mounted.artifact_generation = self.stores.store.generation_now();
                mounted.canonical_revision = self.stores.store.content_revision_now();
                mounted.operation.base_revision = semio_framework_job::RevisionId(u64::from_be_bytes(mounted.canonical_revision[..8].try_into().expect("revision lane width")));
                mounted.operation.generation = semio_framework_job::Generation(mounted.artifact_generation);
                let receipt = ChildPublicationResultV1 { invocation_id: result.inverse_group.invocation_id.0.clone(), committed_members: result.inverse_group.member_edits.len(), child_content_generation: self.child_composition.child_content_generation };
                if self.tools.typed_composed_outbox.push(ComposedGestureResult { operation: mounted.operation.operation.0, mutations: result.mutations, inverse_group: result.inverse_group }).is_err() {
                    let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation composed gesture receiver is saturated"));
                    mounted.pending_child_publication = Some(pending);
                    return Err(fault);
                }
                if let Err(error) = pending.commit(receipt.clone()) {
                    let fault = pending.reject_and_fault(&error);
                    mounted.pending_child_publication = Some(pending);
                    return Err(fault);
                }
                mounted.pending_child_publication = Some(pending);
                mounted.queue_page(TypedOperationResultPage::try_serialize_cold(mounted.next_token(), TypedOperationResultLane::Child, &receipt)?)
            }
            Err(fault) => {
                let fault = pending.reject_and_fault(&fault);
                mounted.pending_child_publication = Some(pending);
                Err(fault)
            }
        }
    }

}
